//! Zamani Quantum Error Correction — Decoder Contract.
//!
//! Production decoder boundary for the QEC subsystem.
//!
//! # Ownership
//!
//! This module owns:
//!
//! - the common decoder interface;
//! - stable decoder identity;
//! - decoder execution context;
//! - correction representation;
//! - canonical decoder result at the current API layer;
//! - decoder-local termination classification;
//! - decoder-local statistics;
//! - stabilizer-backed decoder validation helpers;
//! - syndrome/correction compatibility validation;
//! - deterministic decoder registration;
//! - conversion of decoder-local failures into `QecError`.
//!
//! This module does NOT own:
//!
//! - Pauli/stabilizer mathematics (`stabilizer.rs`);
//! - decoding-graph construction (`decoding_graph.rs`);
//! - MWPM mathematics (`mwpm.rs`);
//! - Union-Find mathematics (`union_find.rs`);
//! - logical-equivalence mathematics (`logical.rs`);
//! - resource policy (`limits.rs`);
//! - runtime resource accounting (`resources.rs`);
//! - memory allocation (`memory.rs`);
//! - capability authority (`capabilities.rs`);
//! - configuration composition (`configuration.rs`);
//! - QPU access;
//! - scheduling;
//! - telemetry transport.
//!
//! # Integration contract
//!
//! ```text
//!                         QecConfig
//!                             |
//!                             v
//!                    DecodeContext
//!             +---------------+---------------+
//!             |               |               |
//!             v               v               v
//!       Cancellation     Determinism      Capabilities
//!             |               |               |
//!             +---------------+---------------+
//!                             |
//!                             v
//!                       Decoder::decode
//!                             |
//!              +--------------+--------------+
//!              |                             |
//!              v                             v
//!          Correction                  DecodeResult
//!              |                             |
//!              +--------------+--------------+
//!                             v
//!                       PauliFrame
//!                             |
//!                             v
//!                   Logical classification
//! ```
//!
//! `decoder.rs` intentionally provides the contract that future
//! `decoder_result.rs` can re-export or extend without requiring a rewrite of
//! the decoder trait.
//!
//! # Resource contract
//!
//! Resource policy belongs exclusively to `QecLimits`.
//!
//! ```text
//! limits.rs       = permitted workload
//! resources.rs    = runtime accounting
//! memory.rs       = allocation enforcement
//! decoder.rs      = decoder admission/preflight
//! ```
//!
//! A decoder must never invent a second production resource ceiling.
//! Algorithm-specific implementation ceilings, if required, belong to the
//! concrete decoder and must never silently override `QecLimits`.
//!
//! # Security contract
//!
//! Decoder execution requires the `Capability::Decode` capability when using
//! `decode_with_context`.
//!
//! A decoder receives no QPU credentials and no physical-hardware authority.
//!
//! # Cancellation contract
//!
//! `decode_with_context` checks cancellation before execution and after the
//! decoder returns. Concrete decoders performing expensive work must poll the
//! supplied `CancellationToken` during their own loops.
//!
//! # Determinism contract
//!
//! Concrete decoders must use the supplied `DeterministicContext` whenever
//! execution can involve ordering, randomized choices, parallel reductions,
//! or tie-breaking.
//!
//! # Compatibility
//!
//! Rust 1.97.1.
//!
//! No unstable language features are used.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;
use std::collections::BTreeSet;

use super::cancellation::CancellationToken;
use super::capabilities::{
    Capability,
    CapabilitySet,
};
use super::configuration::QecConfig;
use super::deterministic::DeterministicContext;
use super::errors::{
    DecoderKind,
    QecError,
    QecResult,
};
use super::limits::QecLimits;
use super::resources::ResourceManager;
use super::stabilizer::{
    Pauli,
    PauliString,
    StabilizerError,
    StabilizerGroup,
    Syndrome,
};

/* ========================================================================== */
/* Decoder identity                                                           */
/* ========================================================================== */

/// Stable identity for a decoder instance.
///
/// This identifier is an execution-registry identity, not a metrics identity.
/// `metrics.rs` intentionally has its own decoder identity model.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
pub struct DecoderId(pub usize);

impl DecoderId {
    /// Creates a decoder identity.
    #[must_use]
    pub const fn new(id: usize) -> Self {
        Self(id)
    }

    /// Returns the numeric registry identity.
    #[must_use]
    pub const fn index(self) -> usize {
        self.0
    }
}

impl fmt::Display for DecoderId {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            formatter,
            "decoder-{}",
            self.0
        )
    }
}

/* ========================================================================== */
/* Decoder termination                                                        */
/* ========================================================================== */

/// Reason a decoder operation terminated.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
)]
pub enum DecodeTermination {
    /// Decoder completed normally.
    Completed,

    /// The input syndrome was already trivial.
    TrivialInput,

    /// Decoder stopped because cancellation was requested.
    Cancelled,

    /// Decoder stopped because a configured resource/time boundary was hit.
    ResourceLimited,

    /// Decoder could not produce a valid correction.
    Failed,
}

impl DecodeTermination {
    /// Returns whether the operation completed successfully.
    #[must_use]
    pub const fn is_success(self) -> bool {
        matches!(
            self,
            Self::Completed | Self::TrivialInput
        )
    }

    /// Returns whether the operation was cancelled.
    #[must_use]
    pub const fn is_cancelled(self) -> bool {
        matches!(
            self,
            Self::Cancelled
        )
    }
}

/* ========================================================================== */
/* Correction                                                                */
/* ========================================================================== */

/// A proposed physical Pauli correction.
///
/// A correction never mutates a quantum state. It is an immutable description
/// of the operation selected by a decoder.
///
/// The underlying representation is the canonical binary-symplectic
/// `PauliString` owned by `stabilizer.rs`.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub struct Correction {
    operator: PauliString,
}

impl Correction {
    /// Creates a correction from a validated Pauli string.
    #[must_use]
    pub fn new(
        operator: PauliString,
    ) -> Self {
        Self { operator }
    }

    /// Creates the identity correction.
    #[must_use]
    pub fn identity(
        num_qubits: usize,
    ) -> Self {
        Self {
            operator:
                PauliString::identity(
                    num_qubits,
                ),
        }
    }

    /// Returns the underlying Pauli operator.
    #[must_use]
    pub fn operator(
        &self,
    ) -> &PauliString {
        &self.operator
    }

    /// Returns the number of physical qubits represented.
    #[must_use]
    pub fn num_qubits(
        &self,
    ) -> usize {
        self.operator.num_qubits()
    }

    /// Returns physical Pauli weight.
    #[must_use]
    pub fn weight(
        &self,
    ) -> usize {
        self.operator.weight()
    }

    /// Returns whether this is identity.
    #[must_use]
    pub fn is_identity(
        &self,
    ) -> bool {
        self.operator.is_identity()
    }

    /// Consumes the wrapper and returns the Pauli operator.
    #[must_use]
    pub fn into_operator(
        self,
    ) -> PauliString {
        self.operator
    }
}

/* ========================================================================== */
/* Decoder result                                                             */
/* ========================================================================== */

/// Canonical decoder result at the decoder-contract layer.
///
/// Later integration modules may enrich this result with logical-equivalence
/// witnesses, metrics and resource snapshots. They should wrap or re-export
/// this representation rather than creating incompatible decoder-specific
/// result types.
///
/// This result deliberately contains no raw QPU data or credentials.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub struct DecodeResult {
    decoder: DecoderId,
    syndrome: Syndrome,
    correction: Correction,
    termination: DecodeTermination,
    iterations: u64,
}

impl DecodeResult {
    /// Creates a completed decoder result.
    #[must_use]
    pub fn new(
        decoder: DecoderId,
        syndrome: Syndrome,
        correction: Correction,
    ) -> Self {
        let termination =
            if syndrome.is_trivial() {
                DecodeTermination::TrivialInput
            } else {
                DecodeTermination::Completed
            };

        Self {
            decoder,
            syndrome,
            correction,
            termination,
            iterations: 0,
        }
    }

    /// Creates a result with explicit termination and iteration count.
    #[must_use]
    pub fn with_execution(
        decoder: DecoderId,
        syndrome: Syndrome,
        correction: Correction,
        termination: DecodeTermination,
        iterations: u64,
    ) -> Self {
        Self {
            decoder,
            syndrome,
            correction,
            termination,
            iterations,
        }
    }

    /// Returns the decoder registry identity.
    #[must_use]
    pub const fn decoder(
        &self,
    ) -> DecoderId {
        self.decoder
    }

    /// Returns the input syndrome.
    #[must_use]
    pub fn syndrome(
        &self,
    ) -> &Syndrome {
        &self.syndrome
    }

    /// Returns the selected correction.
    #[must_use]
    pub fn correction(
        &self,
    ) -> &Correction {
        &self.correction
    }

    /// Returns the correction weight.
    #[must_use]
    pub fn correction_weight(
        &self,
    ) -> usize {
        self.correction.weight()
    }

    /// Returns the termination reason.
    #[must_use]
    pub const fn termination(
        &self,
    ) -> DecodeTermination {
        self.termination
    }

    /// Returns decoder iteration count.
    #[must_use]
    pub const fn iterations(
        &self,
    ) -> u64 {
        self.iterations
    }

    /// Returns whether the original syndrome was trivial.
    #[must_use]
    pub fn is_trivial(
        &self,
    ) -> bool {
        self.syndrome.is_trivial()
    }

    /// Returns whether this result completed successfully.
    #[must_use]
    pub const fn is_success(
        &self,
    ) -> bool {
        self.termination.is_success()
    }

    /// Returns whether the selected correction is identity.
    #[must_use]
    pub fn is_identity_correction(
        &self,
    ) -> bool {
        self.correction.is_identity()
    }

    /// Replaces the termination state.
    #[must_use]
    pub fn with_termination(
        mut self,
        termination: DecodeTermination,
    ) -> Self {
        self.termination = termination;
        self
    }

    /// Replaces the iteration count.
    #[must_use]
    pub fn with_iterations(
        mut self,
        iterations: u64,
    ) -> Self {
        self.iterations = iterations;
        self
    }
}

/* ========================================================================== */
/* Decoder execution context                                                  */
/* ========================================================================== */

/// Immutable execution context supplied to production decoder execution.
///
/// The context is intentionally composed from existing subsystem contracts:
///
/// - `QecConfig` = validated execution policy;
/// - `ResourceManager` = runtime accounting;
/// - `CancellationToken` = cooperative cancellation;
/// - `DeterministicContext` = deterministic execution;
/// - `CapabilitySet` = authorization input.
///
/// The decoder does not own any of these objects.
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

    /// Returns the QEC configuration.
    #[must_use]
    pub const fn config(
        &self,
    ) -> &QecConfig {
        self.config
    }

    /// Returns runtime resource accounting.
    #[must_use]
    pub const fn resources(
        &self,
    ) -> &ResourceManager {
        self.resources
    }

    /// Returns the cancellation token.
    #[must_use]
    pub const fn cancellation(
        &self,
    ) -> &CancellationToken {
        self.cancellation
    }

    /// Returns deterministic execution context.
    #[must_use]
    pub const fn deterministic(
        &self,
    ) -> &DeterministicContext {
        self.deterministic
    }

    /// Returns the effective capability set.
    #[must_use]
    pub const fn capabilities(
        &self,
    ) -> &CapabilitySet {
        self.capabilities
    }

    /// Performs decoder admission/preflight before algorithm execution.
    ///
    /// This method does not allocate memory, spawn workers, access hardware,
    /// or mutate resource counters.
    pub fn preflight(
        &self,
        syndrome: &Syndrome,
    ) -> QecResult<()> {
        self.cancellation.check()?;

        self.config
            .validate()
            .map_err(|error| QecError::InvalidInput {
                message: format!(
                    "invalid QEC decoder configuration: {error}"
                ),
            })?;

        if !self.capabilities.contains(
            Capability::Decode,
        ) {
            return Err(
                QecError::CapabilityDenied {
                    capability:
                        Capability::Decode
                            .name()
                            .to_owned(),
                    operation:
                        "decode".to_owned(),
                    message:
                        "decoder execution requires qec.decode"
                            .to_owned(),
                },
            );
        }

        self.config
            .limits
            .validate_syndrome(
                syndrome.len(),
                1,
            )
            .map_err(|error| {
                QecError::ResourceLimitExceeded {
                    resource:
                        super::errors::ResourceKind::SyndromeEvents,
                    requested:
                        syndrome.len() as u128,
                    current: 0,
                    limit:
                        self.config
                            .limits
                            .max_syndrome_events
                            as u128,
                    message:
                        error.to_string(),
                }
            })?;

        Ok(())
    }
}

/* ========================================================================== */
/* Decoder trait                                                              */
/* ========================================================================== */

/// Common interface implemented by all QEC decoders.
///
/// The one-argument `decode` method is retained as the lightweight,
/// hardware-independent compatibility contract.
///
/// Production execution should call `decode_with_context`, which adds:
///
/// - configuration validation;
/// - capability authorization;
/// - resource preflight;
/// - cancellation boundaries;
/// - canonical error conversion.
///
/// Concrete decoders should implement `decode` and should poll the
/// `CancellationToken` supplied through `DecodeContext` whenever performing
/// expensive work.
pub trait Decoder {
    /// Returns the decoder's stable registry identity.
    fn id(
        &self,
    ) -> DecoderId;

    /// Performs decoder mathematics without external execution policy.
    ///
    /// This method must remain deterministic for deterministic input.
    fn decode(
        &self,
        syndrome: &Syndrome,
    ) -> Result<DecodeResult, DecoderError>;

    /// Production execution boundary.
    ///
    /// This method must be used by execution infrastructure rather than
    /// bypassing QEC policy.
    fn decode_with_context(
        &self,
        syndrome: &Syndrome,
        context: &DecodeContext<'_>,
    ) -> QecResult<DecodeResult> {
        context.preflight(
            syndrome,
        )?;

        context.cancellation.check()?;

        let result = self
            .decode(syndrome)
            .map_err(
                DecoderError::into_qec_error,
            )?;

        validate_result(
            &result,
        )
        .map_err(
            DecoderError::into_qec_error,
        )?;

        context.cancellation.check()?;

        Ok(result)
    }
}

/* ========================================================================== */
/* Stabilizer-backed decoder                                                  */
/* ========================================================================== */

/// Validated stabilizer model shared by decoder implementations.
///
/// Concrete decoders such as MWPM and Union-Find should use this helper
/// instead of repeating stabilizer validation.
#[derive(
    Debug,
    Clone,
)]
pub struct StabilizerDecoder {
    id: DecoderId,
    stabilizers: StabilizerGroup,
}

impl StabilizerDecoder {
    /// Creates a decoder model from a validated stabilizer group.
    pub fn new(
        id: DecoderId,
        stabilizers: StabilizerGroup,
    ) -> Result<Self, DecoderError> {
        stabilizers
            .validate()
            .map_err(
                DecoderError::Stabilizer,
            )?;

        Ok(Self {
            id,
            stabilizers,
        })
    }

    /// Returns decoder identity.
    #[must_use]
    pub const fn id(
        &self,
    ) -> DecoderId {
        self.id
    }

    /// Returns the stabilizer group.
    #[must_use]
    pub fn stabilizers(
        &self,
    ) -> &StabilizerGroup {
        &self.stabilizers
    }

    /// Returns physical qubit count.
    #[must_use]
    pub fn num_qubits(
        &self,
    ) -> usize {
        self.stabilizers.num_qubits()
    }

    /// Returns stabilizer-generator count.
    #[must_use]
    pub fn generator_count(
        &self,
    ) -> usize {
        self.stabilizers.len()
    }

    /// Recomputes the syndrome produced by a candidate error/correction.
    pub fn syndrome_for_error(
        &self,
        error: &PauliString,
    ) -> Result<Syndrome, DecoderError> {
        self.stabilizers
            .syndrome(error)
            .map_err(
                DecoderError::Stabilizer,
            )
    }

    /// Checks whether a correction reproduces the requested syndrome.
    pub fn correction_matches_syndrome(
        &self,
        correction: &Correction,
        expected: &Syndrome,
    ) -> Result<bool, DecoderError> {
        validate_syndrome(
            expected,
            &self.stabilizers,
        )?;

        validate_correction(
            correction,
            self.num_qubits(),
        )?;

        let actual =
            self.syndrome_for_error(
                correction.operator(),
            )?;

        Ok(actual == *expected)
    }
}

/* ========================================================================== */
/* Identity decoder                                                           */
/* ========================================================================== */

/// Decoder that accepts only a trivial syndrome.
///
/// The old implementation returned identity even for a non-trivial
/// syndrome. That was mathematically unsafe because it silently reported a
/// correction that did not explain the measured syndrome.
///
/// This implementation fails closed.
#[derive(
    Debug,
    Clone,
    Copy,
)]
pub struct IdentityDecoder {
    id: DecoderId,
    num_qubits: usize,
}

impl IdentityDecoder {
    /// Creates an identity decoder.
    pub fn new(
        id: DecoderId,
        num_qubits: usize,
    ) -> Result<Self, DecoderError> {
        if num_qubits == 0 {
            return Err(
                DecoderError::InvalidQubitCount {
                    count: 0,
                },
            );
        }

        Ok(Self {
            id,
            num_qubits,
        })
    }

    /// Returns decoder identity.
    #[must_use]
    pub const fn id(
        &self,
    ) -> DecoderId {
        self.id
    }

    /// Returns physical qubit count.
    #[must_use]
    pub const fn num_qubits(
        &self,
    ) -> usize {
        self.num_qubits
    }
}

impl Decoder for IdentityDecoder {
    fn id(
        &self,
    ) -> DecoderId {
        self.id
    }

    fn decode(
        &self,
        syndrome: &Syndrome,
    ) -> Result<DecodeResult, DecoderError> {
        if !syndrome.is_trivial() {
            return Err(
                DecoderError::NonTrivialSyndrome {
                    triggered:
                        syndrome.triggered_count(),
                },
            );
        }

        Ok(
            DecodeResult::with_execution(
                self.id,
                syndrome.clone(),
                Correction::identity(
                    self.num_qubits,
                ),
                DecodeTermination::TrivialInput,
                0,
            ),
        )
    }
}

/* ========================================================================== */
/* Validation helpers                                                         */
/* ========================================================================== */

/// Validates a syndrome against a stabilizer group.
///
/// A primitive `stabilizer::Syndrome` represents one deterministic
/// stabilizer-generator measurement vector. Multi-round/timestamped syndrome
/// streams belong to `syndrome.rs`.
pub fn validate_syndrome(
    syndrome: &Syndrome,
    stabilizers: &StabilizerGroup,
) -> Result<(), DecoderError> {
    if syndrome.len()
        != stabilizers.len()
    {
        return Err(
            DecoderError::SyndromeLengthMismatch {
                expected:
                    stabilizers.len(),
                actual:
                    syndrome.len(),
            },
        );
    }

    Ok(())
}

/// Validates that a correction belongs to the physical system.
pub fn validate_correction(
    correction: &Correction,
    num_qubits: usize,
) -> Result<(), DecoderError> {
    if correction.num_qubits()
        != num_qubits
    {
        return Err(
            DecoderError::CorrectionQubitCountMismatch {
                expected: num_qubits,
                actual:
                    correction.num_qubits(),
            },
        );
    }

    Ok(())
}

/// Validates that a correction reproduces the requested syndrome.
pub fn validate_correction_for_syndrome(
    correction: &Correction,
    syndrome: &Syndrome,
    stabilizers: &StabilizerGroup,
) -> Result<bool, DecoderError> {
    validate_syndrome(
        syndrome,
        stabilizers,
    )?;

    validate_correction(
        correction,
        stabilizers.num_qubits(),
    )?;

    let produced =
        stabilizers
            .syndrome(
                correction.operator(),
            )
            .map_err(
                DecoderError::Stabilizer,
            )?;

    Ok(produced == *syndrome)
}

/// Validates the structural invariants of a decoder result.
pub fn validate_result(
    result: &DecodeResult,
) -> Result<(), DecoderError> {
    if result.syndrome.len()
        != result.correction.num_qubits()
        && !result.syndrome.is_empty()
    {
        // A primitive syndrome normally contains one bit per stabilizer, not
        // one bit per physical qubit. Therefore this check intentionally only
        // rejects impossible zero-qubit correction/result combinations below.
    }

    if result.correction.num_qubits()
        == 0
    {
        return Err(
            DecoderError::InvalidQubitCount {
                count: 0,
            },
        );
    }

    if result.termination.is_success()
        && matches!(
            result.termination,
            DecodeTermination::Completed
        )
        && result.syndrome.is_trivial()
        && !result.correction.is_identity()
    {
        return Err(
            DecoderError::InvalidResult {
                reason:
                    "trivial syndrome cannot require a non-identity correction",
            },
        );
    }

    Ok(())
}

/* ========================================================================== */
/* Syndrome classification                                                    */
/* ========================================================================== */

/// Classification of a primitive syndrome.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
)]
pub enum SyndromeClass {
    /// No stabilizer generator was triggered.
    Trivial,

    /// At least one stabilizer generator was triggered.
    NonTrivial,
}

impl SyndromeClass {
    /// Classifies a syndrome.
    #[must_use]
    pub const fn classify(
        syndrome: &Syndrome,
    ) -> Self {
        if syndrome.is_trivial() {
            Self::Trivial
        } else {
            Self::NonTrivial
        }
    }
}

/* ========================================================================== */
/* Decoder statistics                                                         */
/* ========================================================================== */

/// Allocation-free aggregate decoder statistics.
#[derive(
    Debug,
    Clone,
    Copy,
    Default,
    PartialEq,
    Eq,
)]
pub struct DecoderStatistics {
    decoded: u64,
    trivial: u64,
    nontrivial: u64,
    failed: u64,
    cancelled: u64,
}

impl DecoderStatistics {
    /// Creates empty statistics.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            decoded: 0,
            trivial: 0,
            nontrivial: 0,
            failed: 0,
            cancelled: 0,
        }
    }

    /// Returns total successful decodes.
    #[must_use]
    pub const fn decoded(
        &self,
    ) -> u64 {
        self.decoded
    }

    /// Returns trivial-input count.
    #[must_use]
    pub const fn trivial(
        &self,
    ) -> u64 {
        self.trivial
    }

    /// Returns non-trivial-input count.
    #[must_use]
    pub const fn nontrivial(
        &self,
    ) -> u64 {
        self.nontrivial
    }

    /// Returns failed-operation count.
    #[must_use]
    pub const fn failed(
        &self,
    ) -> u64 {
        self.failed
    }

    /// Returns cancelled-operation count.
    #[must_use]
    pub const fn cancelled(
        &self,
    ) -> u64 {
        self.cancelled
    }

    /// Records a decoder result.
    pub fn record(
        &mut self,
        result: Result<
            &DecodeResult,
            &DecoderError,
        >,
    ) {
        match result {
            Ok(result) => {
                self.decoded =
                    self.decoded
                        .saturating_add(1);

                if result.is_trivial() {
                    self.trivial =
                        self.trivial
                            .saturating_add(1);
                } else {
                    self.nontrivial =
                        self.nontrivial
                            .saturating_add(1);
                }

                if result
                    .termination()
                    .is_cancelled()
                {
                    self.cancelled =
                        self.cancelled
                            .saturating_add(1);
                }
            }

            Err(error) => {
                if matches!(
                    error,
                    DecoderError::Cancelled
                ) {
                    self.cancelled =
                        self.cancelled
                            .saturating_add(1);
                } else {
                    self.failed =
                        self.failed
                            .saturating_add(1);
                }
            }
        }
    }
}

/* ========================================================================== */
/* Pauli helpers                                                              */
/* ========================================================================== */

/// Creates a single-qubit Pauli error.
pub fn single_qubit_error(
    num_qubits: usize,
    qubit: usize,
    pauli: Pauli,
) -> Result<PauliString, DecoderError> {
    if num_qubits == 0 {
        return Err(
            DecoderError::InvalidQubitCount {
                count: 0,
            },
        );
    }

    if qubit >= num_qubits {
        return Err(
            DecoderError::QubitOutOfRange {
                qubit,
                num_qubits,
            },
        );
    }

    let mut error =
        PauliString::identity(
            num_qubits,
        );

    error
        .set_pauli(
            super::stabilizer::QubitIndex::new(
                qubit,
            ),
            pauli,
        )
        .map_err(
            DecoderError::Stabilizer,
        )?;

    Ok(error)
}

/// Creates a single-qubit X error.
pub fn x_error(
    num_qubits: usize,
    qubit: usize,
) -> Result<PauliString, DecoderError> {
    single_qubit_error(
        num_qubits,
        qubit,
        Pauli::X,
    )
}

/// Creates a single-qubit Y error.
pub fn y_error(
    num_qubits: usize,
    qubit: usize,
) -> Result<PauliString, DecoderError> {
    single_qubit_error(
        num_qubits,
        qubit,
        Pauli::Y,
    )
}

/// Creates a single-qubit Z error.
pub fn z_error(
    num_qubits: usize,
    qubit: usize,
) -> Result<PauliString, DecoderError> {
    single_qubit_error(
        num_qubits,
        qubit,
        Pauli::Z,
    )
}

/* ========================================================================== */
/* Decoder registry                                                           */
/* ========================================================================== */

/// Deterministic registry of decoder identities.
///
/// The registry intentionally stores identity rather than decoder objects.
/// Object ownership belongs to the execution layer.
#[derive(
    Debug,
    Default,
)]
pub struct DecoderRegistry {
    ids: BTreeSet<DecoderId>,
}

impl DecoderRegistry {
    /// Creates an empty registry.
    #[must_use]
    pub fn new() -> Self {
        Self {
            ids: BTreeSet::new(),
        }
    }

    /// Registers a decoder identity.
    pub fn register<D>(
        &mut self,
        decoder: &D,
    ) -> Result<(), DecoderError>
    where
        D: Decoder + ?Sized,
    {
        let id = decoder.id();

        if !self.ids.insert(id) {
            return Err(
                DecoderError::DuplicateDecoder {
                    id,
                },
            );
        }

        Ok(())
    }

    /// Returns whether an identity is registered.
    #[must_use]
    pub fn contains(
        &self,
        id: DecoderId,
    ) -> bool {
        self.ids.contains(&id)
    }

    /// Returns registered decoder count.
    #[must_use]
    pub fn len(
        &self,
    ) -> usize {
        self.ids.len()
    }

    /// Returns whether no decoder is registered.
    #[must_use]
    pub fn is_empty(
        &self,
    ) -> bool {
        self.ids.is_empty()
    }

    /// Returns deterministic decoder identities.
    #[must_use]
    pub fn ids(
        &self,
    ) -> Vec<DecoderId> {
        self.ids.iter().copied().collect()
    }
}

/* ========================================================================== */
/* Decoder errors                                                             */
/* ========================================================================== */

/// Local decoder-contract error.
///
/// Public execution APIs convert this type into the canonical `QecError`.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub enum DecoderError {
    /// Stabilizer mathematics failed.
    Stabilizer(
        StabilizerError,
    ),

    /// Decoder was constructed for zero physical qubits.
    InvalidQubitCount {
        count: usize,
    },

    /// Requested qubit does not exist.
    QubitOutOfRange {
        qubit: usize,
        num_qubits: usize,
    },

    /// Syndrome length does not match the stabilizer system.
    SyndromeLengthMismatch {
        expected: usize,
        actual: usize,
    },

    /// Correction belongs to another physical system.
    CorrectionQubitCountMismatch {
        expected: usize,
        actual: usize,
    },

    /// Identity decoder received a non-trivial syndrome.
    NonTrivialSyndrome {
        triggered: usize,
    },

    /// Two decoders attempted to use the same registry identity.
    DuplicateDecoder {
        id: DecoderId,
    },

    /// Decoder result violates a structural invariant.
    InvalidResult {
        reason: &'static str,
    },

    /// Execution was cancelled.
    Cancelled,
}

impl DecoderError {
    /// Converts the local error to the canonical QEC error boundary.
    #[must_use]
    pub fn into_qec_error(
        self,
    ) -> QecError {
        match self {
            Self::Stabilizer(error) => {
                QecError::InvalidStabilizer {
                    message:
                        error.to_string(),
                }
            }

            Self::InvalidQubitCount {
                count,
            } => {
                QecError::InvalidInput {
                    message: format!(
                        "decoder requires at least one physical qubit; got {count}"
                    ),
                }
            }

            Self::QubitOutOfRange {
                qubit,
                num_qubits,
            } => {
                QecError::InvalidInput {
                    message: format!(
                        "qubit {qubit} is outside {num_qubits}-qubit decoder"
                    ),
                }
            }

            Self::SyndromeLengthMismatch {
                expected,
                actual,
            } => {
                QecError::InvalidSyndrome {
                    message: format!(
                        "syndrome length mismatch: expected {expected}, got {actual}"
                    ),
                }
            }

            Self::CorrectionQubitCountMismatch {
                expected,
                actual,
            } => {
                QecError::DecoderFailure {
                    decoder:
                        DecoderKind::Custom,
                    message: format!(
                        "correction qubit count mismatch: expected {expected}, got {actual}"
                    ),
                }
            }

            Self::NonTrivialSyndrome {
                triggered,
            } => {
                QecError::DecoderFailure {
                    decoder:
                        DecoderKind::Identity,
                    message: format!(
                        "identity decoder cannot decode a non-trivial syndrome with {triggered} triggered generators"
                    ),
                }
            }

            Self::DuplicateDecoder {
                id,
            } => {
                QecError::InvalidInput {
                    message: format!(
                        "decoder registry already contains {id}"
                    ),
                }
            }

            Self::InvalidResult {
                reason,
            } => {
                QecError::InternalInvariantViolation {
                    invariant:
                        "decoder result invariant",
                    message:
                        reason.to_owned(),
                }
            }

            Self::Cancelled => {
                QecError::CancellationRequested {
                    message:
                        "decoder execution cancelled"
                            .to_owned(),
                }
            }
        }
    }
}

impl fmt::Display for DecoderError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::Stabilizer(error) => {
                write!(
                    formatter,
                    "stabilizer error: {error}"
                )
            }

            Self::InvalidQubitCount {
                count,
            } => {
                write!(
                    formatter,
                    "decoder requires at least one qubit; got {count}"
                )
            }

            Self::QubitOutOfRange {
                qubit,
                num_qubits,
            } => {
                write!(
                    formatter,
                    "qubit {qubit} is outside a {num_qubits}-qubit system"
                )
            }

            Self::SyndromeLengthMismatch {
                expected,
                actual,
            } => {
                write!(
                    formatter,
                    "syndrome length mismatch: expected {expected}, got {actual}"
                )
            }

            Self::CorrectionQubitCountMismatch {
                expected,
                actual,
            } => {
                write!(
                    formatter,
                    "correction qubit count mismatch: expected {expected}, got {actual}"
                )
            }

            Self::NonTrivialSyndrome {
                triggered,
            } => {
                write!(
                    formatter,
                    "identity decoder cannot accept {triggered} triggered syndrome generators"
                )
            }

            Self::DuplicateDecoder {
                id,
            } => {
                write!(
                    formatter,
                    "decoder {id} is already registered"
                )
            }

            Self::InvalidResult {
                reason,
            } => {
                write!(
                    formatter,
                    "invalid decoder result: {reason}"
                )
            }

            Self::Cancelled => {
                formatter.write_str(
                    "decoder execution cancelled"
                )
            }
        }
    }
}

impl std::error::Error for DecoderError {}

/* ========================================================================== */
/* Compile-time integration assertions                                        */
/* ========================================================================== */

/// Forces the compiler to type-check the important cross-module contracts
/// whenever this module is compiled.
///
/// These are intentionally zero-cost.
#[allow(dead_code)]
fn assert_integration_contracts(
    config: &QecConfig,
    limits: &QecLimits,
    resources: &ResourceManager,
    cancellation: &CancellationToken,
    deterministic: &DeterministicContext,
    capabilities: &CapabilitySet,
) {
    let context =
        DecodeContext::new(
            config,
            resources,
            cancellation,
            deterministic,
            capabilities,
        );

    let _ = context.config();
    let _ = context.resources();
    let _ = context.cancellation();
    let _ = context.deterministic();
    let _ = context.capabilities();

    let _ = limits.max_decoder_iterations;
}

/* ========================================================================== */
/* Tests                                                                      */
/* ========================================================================== */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correction_identity_is_safe() {
        let correction =
            Correction::identity(3);

        assert!(correction.is_identity());
        assert_eq!(
            correction.num_qubits(),
            3
        );
        assert_eq!(
            correction.weight(),
            0
        );
    }

    #[test]
    fn identity_decoder_rejects_nontrivial_syndrome() {
        let decoder =
            IdentityDecoder::new(
                DecoderId::new(1),
                3,
            )
            .expect("valid decoder");

        let syndrome =
            Syndrome::new(vec![
                true,
            ]);

        let result =
            decoder.decode(
                &syndrome,
            );

        assert!(matches!(
            result,
            Err(
                DecoderError::NonTrivialSyndrome {
                    triggered: 1
                }
            )
        ));
    }

    #[test]
    fn identity_decoder_accepts_trivial_syndrome() {
        let decoder =
            IdentityDecoder::new(
                DecoderId::new(1),
                3,
            )
            .expect("valid decoder");

        let syndrome =
            Syndrome::new(vec![
                false,
                false,
            ]);

        let result =
            decoder
                .decode(
                    &syndrome,
                )
                .expect("trivial syndrome");

        assert_eq!(
            result.decoder(),
            DecoderId::new(1)
        );
        assert!(
            result.is_trivial()
        );
        assert!(
            result
                .correction()
                .is_identity()
        );
        assert_eq!(
            result.termination(),
            DecodeTermination::TrivialInput
        );
    }

    #[test]
    fn syndrome_validation_rejects_wrong_length() {
        let stabilizers =
            StabilizerGroup::new(3)
                .expect("valid group");

        let syndrome =
            Syndrome::new(vec![
                true,
                false,
            ]);

        let result =
            validate_syndrome(
                &syndrome,
                &stabilizers,
            );

        assert!(matches!(
            result,
            Err(
                DecoderError::SyndromeLengthMismatch {
                    expected: 3,
                    actual: 2
                }
            )
        ));
    }

    #[test]
    fn registry_is_deterministic() {
        let first =
            IdentityDecoder::new(
                DecoderId::new(20),
                2,
            )
            .expect("valid decoder");

        let second =
            IdentityDecoder::new(
                DecoderId::new(10),
                2,
            )
            .expect("valid decoder");

        let mut registry =
            DecoderRegistry::new();

        registry
            .register(&first)
            .expect("register first");

        registry
            .register(&second)
            .expect("register second");

        assert_eq!(
            registry.ids(),
            vec![
                DecoderId::new(10),
                DecoderId::new(20),
            ]
        );
    }

    #[test]
    fn duplicate_registry_identity_is_rejected() {
        let first =
            IdentityDecoder::new(
                DecoderId::new(7),
                2,
            )
            .expect("valid decoder");

        let second =
            IdentityDecoder::new(
                DecoderId::new(7),
                2,
            )
            .expect("valid decoder");

        let mut registry =
            DecoderRegistry::new();

        registry
            .register(&first)
            .expect("first registration");

        assert!(matches!(
            registry.register(
                &second
            ),
            Err(
                DecoderError::DuplicateDecoder {
                    id: DecoderId(7)
                }
            )
        ));
    }

    #[test]
    fn single_qubit_helpers_validate_bounds() {
        assert!(
            x_error(3, 2)
                .is_ok()
        );

        assert!(
            x_error(3, 3)
                .is_err()
        );

        assert!(
            y_error(0, 0)
                .is_err()
        );

        assert!(
            z_error(2, 0)
                .is_ok()
        );
    }

    #[test]
    fn syndrome_classification_is_deterministic() {
        let trivial =
            Syndrome::new(vec![
                false,
                false,
            ]);

        let nontrivial =
            Syndrome::new(vec![
                false,
                true,
            ]);

        assert_eq!(
            SyndromeClass::classify(
                &trivial
            ),
            SyndromeClass::Trivial
        );

        assert_eq!(
            SyndromeClass::classify(
                &nontrivial
            ),
            SyndromeClass::NonTrivial
        );
    }

    #[test]
    fn statistics_are_saturating_and_deterministic() {
        let decoder =
            IdentityDecoder::new(
                DecoderId::new(1),
                2,
            )
            .expect("valid decoder");

        let syndrome =
            Syndrome::new(vec![
                false,
            ]);

        let result =
            decoder
                .decode(
                    &syndrome,
                )
                .expect("decode");

        let mut statistics =
            DecoderStatistics::new();

        statistics.record(
            Ok(&result)
        );

        assert_eq!(
            statistics.decoded(),
            1
        );

        assert_eq!(
            statistics.trivial(),
            1
        );

        assert_eq!(
            statistics.nontrivial(),
            0
        );

        assert_eq!(
            statistics.failed(),
            0
        );
    }
}