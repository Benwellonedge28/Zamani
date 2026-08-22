//! Zamani Quantum Error Correction — Decoder Contract.
//!
//! Production decoder boundary shared by MWPM, Union-Find, future decoders,
//! streaming execution, distributed execution, QPU execution, replay,
//! checkpointing, verification, and logical-equivalence analysis.
//!
//! # Ownership
//!
//! This module owns:
//!
//! - the common [`Decoder`] trait;
//! - decoder execution context;
//! - decoder admission/preflight;
//! - decoder input validation at the execution boundary;
//! - decoder identity requirements;
//! - decoder registration/dispatch contracts;
//! - conversion of decoder-local failures into [`QecError`];
//! - common decoder execution lifecycle.
//!
//! This module does NOT own:
//!
//! - canonical decoder results (`decoder_result.rs`);
//! - Pauli/stabilizer mathematics (`stabilizer.rs`);
//! - logical-equivalence mathematics (`logical_equivalence.rs`);
//! - logical outcome definitions (`logical.rs`);
//! - decoding-graph construction (`decoding_graph.rs`);
//! - MWPM mathematics (`mwpm.rs`);
//! - Union-Find mathematics (`union_find.rs`);
//! - resource policy (`limits.rs`);
//! - runtime resource accounting (`resources.rs`);
//! - memory allocation (`memory.rs`);
//! - capability authority (`capabilities.rs`);
//! - QPU access (`qpu_adapter.rs`);
//! - syndrome extraction (`syndrome_extractor.rs`);
//! - scheduling (`scheduler.rs`);
//! - telemetry transport (`telemetry.rs`);
//!
//! # Canonical execution architecture
//!
//! ```text
//! Input
//!   │
//!   ▼
//! validation
//!   │
//!   ▼
//! DecodeContext
//!   │
//!   ├── QecConfig
//!   ├── QecLimits
//!   ├── ResourceManager
//!   ├── CancellationToken
//!   ├── DeterministicContext
//!   └── CapabilitySet
//!   │
//!   ▼
//! Decoder::decode_with_context
//!   │
//!   ├───────────────┬──────────────────┐
//!   ▼               ▼                  ▼
//! MWPM          Union-Find       Future Decoder
//!   │               │                  │
//!   └───────────────┴──────────────────┘
//!                   │
//!                   ▼
//!            decoder_result.rs
//!                   │
//!          ┌────────┼────────┐
//!          ▼        ▼        ▼
//!       correction metrics logical
//!          │                 │
//!          ▼                 ▼
//!     pauli_frame     logical_equivalence
//! ```
//!
//! # Important architectural rule
//!
//! `decoder.rs` defines the execution contract.
//!
//! `decoder_result.rs` defines the result contract.
//!
//! A decoder implementation must never create its own incompatible result
//! structure.
//!
//! # Resource model
//!
//! ```text
//! limits.rs
//!     = permitted workload
//!
//! memory.rs
//!     = allocation enforcement
//!
//! resources.rs
//!     = runtime accounting
//!
//! decoder.rs
//!     = execution admission and contract enforcement
//! ```
//!
//! No decoder is permitted to introduce an independent production-wide
//! resource ceiling.
//!
//! # Security model
//!
//! Decoder execution requires [`Capability::Decode`].
//!
//! A decoder receives no QPU credentials, private keys, unrestricted backend
//! handles, or calibration authority.
//!
//! Capability authorization remains owned by `capabilities.rs`.
//!
//! # Cancellation
//!
//! `decode_with_context` checks cancellation before admission and after the
//! decoder returns.
//!
//! Concrete decoders MUST poll the supplied token during expensive loops.
//!
//! # Determinism
//!
//! Concrete decoders MUST use [`DeterministicContext`] for:
//!
//! - stable ordering;
//! - tie-breaking;
//! - randomized algorithms;
//! - parallel reductions;
//! - reproducible execution.
//!
//! # Rust compatibility
//!
//! Rust 1.97.1.
//!
//! No unstable language features are used.
//!
//! `unsafe` is forbidden.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;

use super::cancellation::CancellationToken;
use super::capabilities::{Capability, CapabilitySet};
use super::configuration::QecConfig;
use super::decoder_result::{
    Correction,
    DecodeResult,
    DecodeTermination,
    DecoderId,
};
use super::deterministic::DeterministicContext;
use super::errors::{DecoderKind, QecError, QecResult};
use super::resources::ResourceManager;
use super::stabilizer::{PauliString, Syndrome};

/* ========================================================================== */
/* Decoder input                                                              */
/* ========================================================================== */

/// Canonical decoder input.
///
/// The decoder receives a validated syndrome representation and optional
/// contextual information required by the decoding algorithm.
///
/// The input deliberately does not contain:
///
/// - QPU credentials;
/// - backend secrets;
/// - mutable hardware handles;
/// - capability authority.
///
/// Those belong to other architectural layers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecodeInput {
    syndrome: Syndrome,
}

impl DecodeInput {
    /// Creates a decoder input from a syndrome.
    ///
    /// This constructor does not perform policy validation. Full admission is
    /// performed by [`DecodeContext::preflight`].
    #[must_use]
    pub fn new(syndrome: Syndrome) -> Self {
        Self { syndrome }
    }

    /// Returns the syndrome.
    #[must_use]
    pub fn syndrome(&self) -> &Syndrome {
        &self.syndrome
    }

    /// Consumes the input and returns its syndrome.
    #[must_use]
    pub fn into_syndrome(self) -> Syndrome {
        self.syndrome
    }

    /// Returns whether the syndrome contains no active events.
    #[must_use]
    pub fn is_trivial(&self) -> bool {
        self.syndrome.is_trivial()
    }

    /// Returns the number of syndrome events represented by this input.
    #[must_use]
    pub fn len(&self) -> usize {
        self.syndrome.len()
    }

    /// Returns whether the input contains no syndrome events.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.syndrome.is_empty()
    }
}

/* ========================================================================== */
/* Decoder metadata                                                           */
/* ========================================================================== */

/// Immutable metadata describing a decoder implementation.
///
/// This metadata is intentionally small and deterministic so it can safely be
/// included in checkpoints, replay records, cache keys, metrics and audit
/// records.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DecoderMetadata {
    id: DecoderId,
    name: String,
    kind: DecoderKind,
    algorithm_version: String,
}

impl DecoderMetadata {
    /// Creates decoder metadata.
    pub fn new(
        id: DecoderId,
        name: impl Into<String>,
        kind: DecoderKind,
        algorithm_version: impl Into<String>,
    ) -> Self {
        Self {
            id,
            name: name.into(),
            kind,
            algorithm_version: algorithm_version.into(),
        }
    }

    /// Returns the stable decoder ID.
    #[must_use]
    pub const fn id(&self) -> DecoderId {
        self.id
    }

    /// Returns the human-readable decoder name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the decoder category.
    #[must_use]
    pub const fn kind(&self) -> DecoderKind {
        self.kind
    }

    /// Returns the algorithm version.
    #[must_use]
    pub fn algorithm_version(&self) -> &str {
        &self.algorithm_version
    }
}

impl fmt::Display for DecoderMetadata {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}@{} ({})",
            self.name,
            self.algorithm_version,
            self.kind
        )
    }
}

/* ========================================================================== */
/* Decoder execution context                                                  */
/* ========================================================================== */

/// Immutable context supplied to a decoder during execution.
///
/// The context borrows all policy and runtime infrastructure. Ownership
/// remains with the caller/execution layer.
pub struct DecodeContext<'a> {
    config: &'a QecConfig,
    resources: &'a ResourceManager,
    cancellation: &'a CancellationToken,
    deterministic: &'a DeterministicContext,
    capabilities: &'a CapabilitySet,
}

impl<'a> DecodeContext<'a> {
    /// Creates a decoder execution context.
    #[must_use]
    pub fn new(
        config: &'a QecConfig,
        resources: &'a ResourceManager,
        cancellation: &'a CancellationToken,
        deterministic: &'a DeterministicContext,
        capabilities: &'a CapabilitySet,
    ) -> Self {
        Self {
            config,
            resources,
            cancellation,
            deterministic,
            capabilities,
        }
    }

    /// Returns the validated QEC configuration.
    #[must_use]
    pub const fn config(&self) -> &QecConfig {
        self.config
    }

    /// Returns runtime resource accounting.
    #[must_use]
    pub const fn resources(&self) -> &ResourceManager {
        self.resources
    }

    /// Returns the cancellation token.
    #[must_use]
    pub const fn cancellation(&self) -> &CancellationToken {
        self.cancellation
    }

    /// Returns deterministic execution context.
    #[must_use]
    pub const fn deterministic(&self) -> &DeterministicContext {
        self.deterministic
    }

    /// Returns the effective capabilities.
    #[must_use]
    pub const fn capabilities(&self) -> &CapabilitySet {
        self.capabilities
    }

    /// Performs common decoder admission checks.
    ///
    /// This function:
    ///
    /// 1. checks cancellation;
    /// 2. validates the configuration;
    /// 3. verifies `Capability::Decode`;
    /// 4. validates syndrome workload against canonical limits.
    ///
    /// It does not:
    ///
    /// - allocate decoder state;
    /// - access a QPU;
    /// - spawn workers;
    /// - mutate decoder state.
    pub fn preflight(&self, input: &DecodeInput) -> QecResult<()> {
        self.cancellation.check()?;

        self.config
            .validate()
            .map_err(|error| QecError::InvalidInput {
                message: format!(
                    "invalid QEC configuration for decoder execution: {error}"
                ),
            })?;

        if !self.capabilities.contains(Capability::Decode) {
            return Err(QecError::CapabilityDenied {
                capability: Capability::Decode.name().to_owned(),
                operation: "decode".to_owned(),
                message: "decoder execution requires the Decode capability"
                    .to_owned(),
            });
        }

        self.config
            .limits
            .validate_syndrome(input.len(), 1)
            .map_err(|error| QecError::InvalidInput {
                message: format!(
                    "decoder syndrome rejected by QEC limits: {error}"
                ),
            })?;

        Ok(())
    }
}

/* ========================================================================== */
/* Decoder trait                                                              */
/* ========================================================================== */

/// Common contract implemented by every QEC decoder.
///
/// Concrete implementations include:
///
/// - MWPM;
/// - Union-Find;
/// - exact small-instance decoders;
/// - sparse decoders;
/// - streaming decoders;
/// - future hardware-aware classical decoders.
///
/// The trait intentionally does not prescribe the internal decoding
/// algorithm.
///
/// # Required invariants
///
/// Implementations MUST:
///
/// - be deterministic when supplied deterministic input/context;
/// - honor cancellation;
/// - honor the canonical resource policy;
/// - return a canonical [`DecodeResult`];
/// - never fabricate a logical outcome;
/// - never access QPU credentials;
/// - never bypass capability checks performed by the execution boundary;
/// - never silently truncate input;
/// - never silently saturate resource usage.
pub trait Decoder: Send + Sync {
    /// Returns immutable decoder metadata.
    fn metadata(&self) -> &DecoderMetadata;

    /// Performs the decoder's actual mathematical work.
    ///
    /// The common execution boundary is already responsible for:
    ///
    /// - capability checks;
    /// - configuration validation;
    /// - initial cancellation;
    /// - syndrome admission.
    ///
    /// The implementation remains responsible for:
    ///
    /// - algorithm-specific resource requests;
    /// - periodic cancellation polling;
    /// - deterministic ordering;
    /// - producing a canonical result.
    fn decode(
        &self,
        input: &DecodeInput,
        context: &DecodeContext<'_>,
    ) -> QecResult<DecodeResult>;

    /// Executes the decoder through the canonical admission boundary.
    ///
    /// This is the preferred production entry point.
    fn decode_with_context(
        &self,
        input: &DecodeInput,
        context: &DecodeContext<'_>,
    ) -> QecResult<DecodeResult> {
        context.preflight(input)?;

        context.cancellation.check()?;

        let result = self.decode(input, context)?;

        context.cancellation.check()?;

        validate_decoder_result(
            self.metadata(),
            input,
            &result,
        )?;

        Ok(result)
    }

    /// Returns the stable decoder ID.
    #[must_use]
    fn id(&self) -> DecoderId {
        self.metadata().id()
    }

    /// Returns the decoder category.
    #[must_use]
    fn kind(&self) -> DecoderKind {
        self.metadata().kind()
    }

    /// Returns the stable decoder name.
    #[must_use]
    fn name(&self) -> &str {
        self.metadata().name()
    }

    /// Returns the algorithm version.
    #[must_use]
    fn algorithm_version(&self) -> &str {
        self.metadata().algorithm_version()
    }
}

/* ========================================================================== */
/* Decoder result validation                                                  */
/* ========================================================================== */

/// Validates that a decoder returned a result compatible with its input.
///
/// This function intentionally validates only the decoder/result contract.
///
/// It does NOT prove logical correctness.
///
/// Logical correctness belongs to `logical_equivalence.rs`.
pub fn validate_decoder_result(
    metadata: &DecoderMetadata,
    input: &DecodeInput,
    result: &DecodeResult,
) -> QecResult<()> {
    if result.decoder() != metadata.id() {
        return Err(QecError::DecoderFailure {
            decoder: metadata.name().to_owned(),
            message: format!(
                "decoder result identity mismatch: expected {}, received {}",
                metadata.id(),
                result.decoder()
            ),
        });
    }

    if result.syndrome() != input.syndrome() {
        return Err(QecError::DecoderFailure {
            decoder: metadata.name().to_owned(),
            message:
                "decoder result does not correspond to the supplied syndrome"
                    .to_owned(),
        });
    }

    if result.correction().num_qubits()
        != correction_qubit_count(input)
    {
        return Err(QecError::DecoderFailure {
            decoder: metadata.name().to_owned(),
            message:
                "decoder correction qubit count is incompatible with input"
                    .to_owned(),
        });
    }

    match result.termination() {
        DecodeTermination::Completed
        | DecodeTermination::TrivialInput => {}

        DecodeTermination::Cancelled
        | DecodeTermination::ResourceLimited
        | DecodeTermination::Failed => {
            return Err(QecError::DecoderFailure {
                decoder: metadata.name().to_owned(),
                message: format!(
                    "decoder returned non-success termination {:?} through \
                     the successful result channel",
                    result.termination()
                ),
            });
        }
    }

    Ok(())
}

/// Determines the physical qubit count represented by decoder input.
///
/// Syndrome is the canonical source of decoder workload, but its representation
/// intentionally does not expose a direct "number of physical qubits" field.
/// A decoder correction must therefore be validated by the concrete decoder
/// against its code topology before producing the result.
///
/// This helper provides the conservative contract used by this module:
/// a correction must represent at least the syndrome's referenced physical
/// support. A zero-event syndrome is validated separately.
fn correction_qubit_count(input: &DecodeInput) -> usize {
    input
        .syndrome()
        .num_qubits()
}

/* ========================================================================== */
/* Decoder registry                                                           */
/* ========================================================================== */

/// Deterministic collection of decoder implementations.
///
/// The registry owns decoder instances but not their execution resources.
///
/// Registration is explicit and duplicate IDs are rejected.
pub struct DecoderRegistry {
    decoders: Vec<Box<dyn Decoder>>,
}

impl DecoderRegistry {
    /// Creates an empty decoder registry.
    #[must_use]
    pub fn new() -> Self {
        Self {
            decoders: Vec::new(),
        }
    }

    /// Creates a registry with an initial decoder set.
    ///
    /// Registration order is normalized by decoder ID.
    pub fn from_decoders(
        decoders: Vec<Box<dyn Decoder>>,
    ) -> QecResult<Self> {
        let mut registry = Self::new();

        for decoder in decoders {
            registry.register(decoder)?;
        }

        Ok(registry)
    }

    /// Registers a decoder.
    ///
    /// Decoder IDs must be unique.
    pub fn register(
        &mut self,
        decoder: Box<dyn Decoder>,
    ) -> QecResult<()> {
        let id = decoder.id();

        if self.decoders.iter().any(|entry| entry.id() == id) {
            return Err(QecError::DecoderFailure {
                decoder: decoder.name().to_owned(),
                message: format!(
                    "duplicate decoder identity {}",
                    id
                ),
            });
        }

        self.decoders.push(decoder);

        self.decoders.sort_by_key(|entry| entry.id());

        Ok(())
    }

    /// Returns a decoder by stable ID.
    #[must_use]
    pub fn get(
        &self,
        id: DecoderId,
    ) -> Option<&dyn Decoder> {
        self.decoders
            .iter()
            .find(|decoder| decoder.id() == id)
            .map(|decoder| decoder.as_ref())
    }

    /// Returns all registered decoder IDs in deterministic order.
    #[must_use]
    pub fn ids(&self) -> Vec<DecoderId> {
        self.decoders
            .iter()
            .map(|decoder| decoder.id())
            .collect()
    }

    /// Returns the number of registered decoders.
    #[must_use]
    pub fn len(&self) -> usize {
        self.decoders.len()
    }

    /// Returns whether the registry contains no decoders.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.decoders.is_empty()
    }

    /// Executes a decoder selected by stable ID.
    pub fn decode(
        &self,
        id: DecoderId,
        input: &DecodeInput,
        context: &DecodeContext<'_>,
    ) -> QecResult<DecodeResult> {
        let decoder = self.get(id).ok_or_else(|| {
            QecError::DecoderFailure {
                decoder: id.to_string(),
                message: "decoder is not registered".to_owned(),
            }
        })?;

        decoder.decode_with_context(
            input,
            context,
        )
    }
}

impl Default for DecoderRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/* ========================================================================== */
/* Decoder helper functions                                                   */
/* ========================================================================== */

/// Creates an identity correction compatible with a physical qubit count.
///
/// This helper is useful for:
///
/// - trivial syndrome handling;
/// - exact decoder fallbacks;
/// - test implementations;
/// - no-error fast paths.
///
/// It does not claim logical correctness.
#[must_use]
pub fn identity_correction(
    num_qubits: usize,
) -> Correction {
    Correction::new(
        PauliString::identity(num_qubits)
    )
}

/// Creates a trivial decoder result for a syndrome containing no events.
///
/// The decoder ID must still be supplied so the result remains attributable
/// to the correct decoder implementation.
#[must_use]
pub fn trivial_result(
    decoder: DecoderId,
    syndrome: Syndrome,
) -> DecodeResult {
    let correction =
        identity_correction(
            syndrome.num_qubits(),
        );

    DecodeResult::with_execution(
        decoder,
        syndrome,
        correction,
        DecodeTermination::TrivialInput,
        0,
    )
}

/// Returns whether an input is eligible for the decoder's trivial-input
/// fast path.
#[must_use]
pub fn is_trivial_input(
    input: &DecodeInput,
) -> bool {
    input.is_trivial()
}

/// Performs the canonical trivial-input handling.
///
/// Concrete decoders should use this before expensive graph construction or
/// matching work.
pub fn decode_trivial_or_continue(
    decoder: DecoderId,
    input: &DecodeInput,
) -> Option<DecodeResult> {
    if input.is_trivial() {
        Some(
            trivial_result(
                decoder,
                input.syndrome().clone(),
            ),
        )
    } else {
        None
    }
}

/* ========================================================================== */
/* Decoder execution helpers                                                  */
/* ========================================================================== */

/// Polls cancellation at a decoder loop boundary.
///
/// Concrete decoders should call this from:
///
/// - matching loops;
/// - union/cluster growth loops;
/// - graph traversals;
/// - sparse matrix operations;
/// - streaming batches;
/// - distributed worker loops.
pub fn check_cancellation(
    context: &DecodeContext<'_>,
) -> QecResult<()> {
    context.cancellation().check()
}

/// Returns the canonical runtime resource manager.
///
/// This helper exists to make resource accounting explicit in concrete
/// decoders without exposing implementation details of `ResourceManager`.
#[must_use]
pub const fn resource_manager(
    context: &DecodeContext<'_>,
) -> &ResourceManager {
    context.resources()
}

/// Returns the deterministic execution context.
///
/// Concrete algorithms must use this context rather than constructing
/// independent random seeds or ordering policies.
#[must_use]
pub const fn deterministic_context(
    context: &DecodeContext<'_>,
) -> &DeterministicContext {
    context.deterministic()
}

/// Returns the canonical QEC configuration.
#[must_use]
pub const fn qec_config(
    context: &DecodeContext<'_>,
) -> &QecConfig {
    context.config()
}

/* ========================================================================== */
/* Error adaptation                                                           */
/* ========================================================================== */

/// Converts an arbitrary decoder failure into the canonical decoder error
/// boundary.
///
/// Concrete decoders should prefer returning the original `QecError` directly.
/// This helper exists for local algorithm errors that do not already have a
/// canonical QEC representation.
pub fn decoder_failure(
    decoder: &DecoderMetadata,
    message: impl Into<String>,
) -> QecError {
    QecError::DecoderFailure {
        decoder: decoder.name().to_owned(),
        message: message.into(),
    }
}

/// Converts an algorithm-specific error into a decoder failure.
///
/// This function deliberately does not expose implementation-specific error
/// types through the public decoder boundary.
pub fn decoder_error<T: fmt::Display>(
    decoder: &DecoderMetadata,
    error: T,
) -> QecError {
    decoder_failure(
        decoder,
        error.to_string(),
    )
}

/* ========================================================================== */
/* Contract assertions                                                        */
/* ========================================================================== */

/// Performs cheap structural assertions for a decoder result.
///
/// This is intended for tests, debug validation and integration self-checks.
///
/// It does not perform a full logical-equivalence proof.
pub fn assert_result_contract(
    metadata: &DecoderMetadata,
    input: &DecodeInput,
    result: &DecodeResult,
) -> QecResult<()> {
    validate_decoder_result(
        metadata,
        input,
        result,
    )
}

/// Verifies that a decoder's correction has the same physical width as the
/// decoder input.
pub fn validate_correction_width(
    input: &DecodeInput,
    correction: &Correction,
) -> QecResult<()> {
    if correction.num_qubits()
        != input.syndrome().num_qubits()
    {
        return Err(QecError::DecoderFailure {
            decoder: "unknown".to_owned(),
            message: format!(
                "correction width mismatch: syndrome has {} qubits, \
                 correction has {}",
                input.syndrome().num_qubits(),
                correction.num_qubits()
            ),
        });
    }

    Ok(())
}

/* ========================================================================== */
/* Tests                                                                      */
/* ========================================================================== */

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct TestDecoder {
        metadata: DecoderMetadata,
    }

    impl TestDecoder {
        fn new() -> Self {
            Self {
                metadata: DecoderMetadata::new(
                    DecoderId::new(1),
                    "test",
                    DecoderKind::Other,
                    "1.0.0",
                ),
            }
        }
    }

    impl Decoder for TestDecoder {
        fn metadata(&self) -> &DecoderMetadata {
            &self.metadata
        }

        fn decode(
            &self,
            input: &DecodeInput,
            _context: &DecodeContext<'_>,
        ) -> QecResult<DecodeResult> {
            if let Some(result) =
                decode_trivial_or_continue(
                    self.id(),
                    input,
                )
            {
                return Ok(result);
            }

            let correction =
                identity_correction(
                    input.syndrome().num_qubits(),
                );

            Ok(
                DecodeResult::with_execution(
                    self.id(),
                    input.syndrome().clone(),
                    correction,
                    DecodeTermination::Completed,
                    1,
                ),
            )
        }
    }

    #[test]
    fn decoder_id_is_stable() {
        let id = DecoderId::new(42);

        assert_eq!(id.index(), 42);
        assert_eq!(id.to_string(), "decoder-42");
    }

    #[test]
    fn identity_correction_has_requested_width() {
        let correction =
            identity_correction(7);

        assert_eq!(
            correction.num_qubits(),
            7
        );

        assert!(correction.is_identity());
        assert_eq!(
            correction.weight(),
            0
        );
    }

    #[test]
    fn trivial_input_produces_trivial_result() {
        let syndrome =
            Syndrome::new(
                5,
                Vec::new(),
            )
            .expect("empty syndrome should be valid");

        let input =
            DecodeInput::new(
                syndrome.clone(),
            );

        let result =
            trivial_result(
                DecoderId::new(1),
                syndrome,
            );

        assert!(
            result.is_success()
        );

        assert_eq!(
            result.termination(),
            DecodeTermination::TrivialInput
        );

        assert!(
            result.is_identity_correction()
        );

        assert!(
            input.is_trivial()
        );
    }

    #[test]
    fn registry_orders_decoders_by_id() {
        let first =
            Box::new(
                TestDecoder::new()
            );

        let second_metadata =
            DecoderMetadata::new(
                DecoderId::new(2),
                "second",
                DecoderKind::Other,
                "1.0.0",
            );

        struct SecondDecoder {
            metadata: DecoderMetadata,
        }

        impl Decoder for SecondDecoder {
            fn metadata(
                &self,
            ) -> &DecoderMetadata {
                &self.metadata
            }

            fn decode(
                &self,
                input: &DecodeInput,
                _context: &DecodeContext<'_>,
            ) -> QecResult<DecodeResult> {
                Ok(
                    trivial_result(
                        self.id(),
                        input.syndrome().clone(),
                    )
                )
            }
        }

        let second =
            Box::new(
                SecondDecoder {
                    metadata:
                        second_metadata,
                }
            );

        let registry =
            DecoderRegistry::from_decoders(
                vec![
                    second,
                    first,
                ],
            )
            .expect(
                "unique decoder IDs should register",
            );

        assert_eq!(
            registry.ids(),
            vec![
                DecoderId::new(1),
                DecoderId::new(2),
            ]
        );
    }

    #[test]
    fn duplicate_decoder_ids_are_rejected() {
        let first =
            Box::new(
                TestDecoder::new()
            );

        let duplicate =
            Box::new(
                TestDecoder::new()
            );

        let result =
            DecoderRegistry::from_decoders(
                vec![
                    first,
                    duplicate,
                ],
            );

        assert!(
            result.is_err()
        );
    }

    #[test]
    fn result_validation_rejects_wrong_decoder() {
        let syndrome =
            Syndrome::new(
                3,
                Vec::new(),
            )
            .expect("empty syndrome should be valid");

        let input =
            DecodeInput::new(
                syndrome.clone(),
            );

        let metadata =
            DecoderMetadata::new(
                DecoderId::new(1),
                "test",
                DecoderKind::Other,
                "1.0.0",
            );

        let result =
            trivial_result(
                DecoderId::new(2),
                syndrome,
            );

        assert!(
            validate_decoder_result(
                &metadata,
                &input,
                &result,
            )
            .is_err()
        );
    }

    #[test]
    fn correction_width_is_checked() {
        let syndrome =
            Syndrome::new(
                4,
                Vec::new(),
            )
            .expect("empty syndrome should be valid");

        let input =
            DecodeInput::new(
                syndrome,
            );

        let correction =
            identity_correction(4);

        assert!(
            validate_correction_width(
                &input,
                &correction,
            )
            .is_ok()
        );

        let wrong =
            identity_correction(3);

        assert!(
            validate_correction_width(
                &input,
                &wrong,
            )
            .is_err()
        );
    }
}