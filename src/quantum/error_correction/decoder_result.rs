//! Zamani Quantum Error Correction — Canonical Decoder Results.
//!
//! This module owns the canonical result contract shared by every decoder,
//! execution layer, verification layer, Pauli-frame layer, streaming layer,
//! distributed layer, checkpoint layer, and QPU integration layer.
//!
//! # Ownership
//!
//! This module owns:
//!
//! - `DecoderId`;
//! - `DecodeTermination`;
//! - `Correction`;
//! - `DecodeResult`;
//! - immutable decoder-result metadata;
//! - result validation;
//! - result composition;
//! - deterministic result identity;
//! - result classification helpers;
//! - logical-outcome attachment;
//! - execution/resource summary attachment;
//! - result witnesses.
//!
//! This module does NOT own:
//!
//! - decoder algorithms;
//! - stabilizer mathematics;
//! - logical-equivalence mathematics;
//! - Pauli-frame mutation;
//! - resource policy;
//! - runtime resource accounting;
//! - cancellation state;
//! - capability authorization;
//! - QPU credentials;
//! - telemetry transport;
//! - checkpoint persistence.
//!
//! Those responsibilities remain in their respective modules.
//!
//! # Canonical architecture
//!
//! ```text
//!                         Decoder
//!                            │
//!                            ▼
//!                    ┌───────────────┐
//!                    │ DecodeResult  │
//!                    └───────┬───────┘
//!                            │
//!              ┌─────────────┼─────────────┐
//!              ▼             ▼             ▼
//!        PauliFrame     Logical         Verification
//!              │        Equivalence          │
//!              │             │               │
//!              └─────────────┼───────────────┘
//!                            ▼
//!                    LogicalOutcome
//!
//! QPU ──► Syndrome ──► Decoder ──► DecodeResult
//!
//! Stream ──► Decoder ──► DecodeResult
//!
//! Distributed worker ──► Decoder ──► DecodeResult
//!
//! Checkpoint ──► DecodeResult ──► Resume
//! ```
//!
//! # Important architectural rule
//!
//! A decoder result describes what a decoder produced.
//!
//! It does NOT by itself prove that the correction succeeded logically.
//!
//! Logical correctness must be established by:
//!
//! ```text
//! physical error
//!       +
//! decoder correction
//!       │
//!       ▼
//! residual error
//!       │
//!       ▼
//! logical_equivalence.rs
//!       │
//!       ▼
//! LogicalOutcome
//! ```
//!
//! Consequently `DecodeResult` may carry an optional logical classification,
//! but the decoder must never fabricate one without an actual equivalence
//! analysis.
//!
//! # Resource semantics
//!
//! `limits.rs` owns permitted workload.
//!
//! `resources.rs` owns runtime accounting.
//!
//! `memory.rs` owns allocation enforcement.
//!
//! This module only records an immutable summary of resource consumption
//! supplied by the execution layer.
//!
//! # Security
//!
//! A decoder result must never contain:
//!
//! - QPU credentials;
//! - authentication tokens;
//! - private keys;
//! - raw backend secrets;
//! - unrestricted hardware handles.
//!
//! Backend identity and public execution metadata may be recorded.
//!
//! # Determinism
//!
//! For deterministic execution, equivalent executions must produce identical
//! semantic result fields:
//!
//! - decoder identity;
//! - input identity;
//! - correction;
//! - termination;
//! - iteration count;
//! - logical outcome;
//! - witness;
//! - public execution metadata.
//!
//! Wall-clock timestamps and other inherently nondeterministic telemetry do
//! not belong in the semantic identity.
//!
//! # Rust compatibility
//!
//! Rust 1.97.1.
//!
//! No unstable language features are required.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;
use std::collections::BTreeMap;
use std::hash::{Hash, Hasher};

use super::errors::{DecoderKind, QecError, QecResult};
use super::logical::LogicalOutcome;
use super::stabilizer::{PauliString, Syndrome};

/// Stable semantic version of the decoder-result contract.
pub const DECODER_RESULT_FORMAT_VERSION: u32 = 1;

/// Canonical identity of a decoder implementation.
///
/// This identity is stable within a QEC execution environment and is suitable
/// for checkpoints, replay records, cache keys, metrics, and result validation.
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
    /// Creates a decoder identifier.
    #[must_use]
    pub const fn new(id: usize) -> Self {
        Self(id)
    }

    /// Returns the numeric registry identifier.
    #[must_use]
    pub const fn index(self) -> usize {
        self.0
    }
}

impl fmt::Display for DecoderId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "decoder-{}", self.0)
    }
}

/// Reason a decoder execution terminated.
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
pub enum DecodeTermination {
    /// Decoder completed successfully.
    Completed,

    /// Input contained no triggered syndrome.
    TrivialInput,

    /// Execution was explicitly cancelled.
    Cancelled,

    /// Execution stopped because a resource boundary was reached.
    ResourceLimited,

    /// Decoder failed to produce a valid correction.
    Failed,
}

impl DecodeTermination {
    /// Returns whether this termination represents successful execution.
    #[must_use]
    pub const fn is_success(self) -> bool {
        matches!(
            self,
            Self::Completed | Self::TrivialInput
        )
    }

    /// Returns whether execution was cancelled.
    #[must_use]
    pub const fn is_cancelled(self) -> bool {
        matches!(self, Self::Cancelled)
    }

    /// Returns whether execution stopped because of resource policy.
    #[must_use]
    pub const fn is_resource_limited(self) -> bool {
        matches!(self, Self::ResourceLimited)
    }

    /// Returns whether execution failed.
    #[must_use]
    pub const fn is_failure(self) -> bool {
        matches!(self, Self::Failed)
    }
}

/// Immutable physical Pauli correction selected by a decoder.
///
/// This is deliberately a value object. It does not mutate quantum state or
/// Pauli-frame state.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
)]
pub struct Correction {
    operator: PauliString,
}

impl Correction {
    /// Creates a correction from a canonical Pauli operator.
    #[must_use]
    pub fn new(operator: PauliString) -> Self {
        Self { operator }
    }

    /// Creates an identity correction.
    #[must_use]
    pub fn identity(num_qubits: usize) -> Self {
        Self {
            operator: PauliString::identity(num_qubits),
        }
    }

    /// Returns the underlying Pauli operator.
    #[must_use]
    pub fn operator(&self) -> &PauliString {
        &self.operator
    }

    /// Returns the physical-qubit count.
    #[must_use]
    pub fn num_qubits(&self) -> usize {
        self.operator.num_qubits()
    }

    /// Returns the physical Pauli weight.
    #[must_use]
    pub fn weight(&self) -> usize {
        self.operator.weight()
    }

    /// Returns whether this correction is identity.
    #[must_use]
    pub fn is_identity(&self) -> bool {
        self.operator.is_identity()
    }

    /// Consumes the wrapper and returns the underlying operator.
    #[must_use]
    pub fn into_operator(self) -> PauliString {
        self.operator
    }
}

/// Immutable summary of runtime resources consumed by one decode.
///
/// This is intentionally separate from `ResourceManager`.
///
/// `ResourceManager` owns live accounting.
/// `DecodeResourceUsage` owns the result snapshot.
#[derive(
    Debug,
    Clone,
    Copy,
    Default,
    PartialEq,
    Eq,
    Hash,
)]
pub struct DecodeResourceUsage {
    /// Peak memory observed during decoding.
    pub peak_memory_bytes: u64,

    /// Decoder iterations consumed.
    pub decoder_iterations: u64,

    /// Graph nodes processed.
    pub graph_nodes: u64,

    /// Graph edges processed.
    pub graph_edges: u64,

    /// Syndrome events processed.
    pub syndrome_events: u64,

    /// Worker slots consumed.
    pub workers: u64,

    /// Verification operations performed.
    pub verification_operations: u64,

    /// QPU shots associated with this result, if applicable.
    pub qpu_shots: u64,
}

impl DecodeResourceUsage {
    /// Creates an empty usage snapshot.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            peak_memory_bytes: 0,
            decoder_iterations: 0,
            graph_nodes: 0,
            graph_edges: 0,
            syndrome_events: 0,
            workers: 0,
            verification_operations: 0,
            qpu_shots: 0,
        }
    }

    /// Returns whether no tracked resource was consumed.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.peak_memory_bytes == 0
            && self.decoder_iterations == 0
            && self.graph_nodes == 0
            && self.graph_edges == 0
            && self.syndrome_events == 0
            && self.workers == 0
            && self.verification_operations == 0
            && self.qpu_shots == 0
    }
}

/// Public, non-secret execution metadata attached to a result.
///
/// Values are deterministic and suitable for checkpoint/replay identity.
///
/// Secrets must never be inserted into this structure.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Default,
)]
pub struct DecodeMetadata {
    entries: BTreeMap<String, String>,
}

impl DecodeMetadata {
    /// Creates empty metadata.
    #[must_use]
    pub fn new() -> Self {
        Self {
            entries: BTreeMap::new(),
        }
    }

    /// Inserts public metadata.
    ///
    /// Keys and values must not contain credentials or other secrets.
    pub fn insert(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) {
        self.entries.insert(key.into(), value.into());
    }

    /// Returns a metadata value.
    #[must_use]
    pub fn get(&self, key: &str) -> Option<&str> {
        self.entries.get(key).map(String::as_str)
    }

    /// Returns deterministic metadata entries.
    #[must_use]
    pub fn entries(&self) -> &BTreeMap<String, String> {
        &self.entries
    }

    /// Returns whether no metadata is attached.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

/// Optional witness describing how a result was established.
///
/// The witness is intentionally opaque to the decoder-result layer.
///
/// `logical_equivalence.rs`, `verification.rs`, and future decoder
/// implementations may attach deterministic proof information here.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub enum DecodeWitness {
    /// No witness was generated.
    None,

    /// Decoder produced an internal matching/correction witness.
    CorrectionValidated {
        /// Number of syndrome constraints checked.
        constraints_checked: u64,
    },

    /// Logical equivalence was explicitly verified.
    LogicalEquivalenceVerified {
        /// Canonical logical outcome.
        outcome: LogicalOutcome,
    },

    /// A deterministic replay/checkpoint witness.
    ReplayValidated {
        /// Stable replay identifier.
        replay_id: String,
    },
}

impl Default for DecodeWitness {
    fn default() -> Self {
        Self::None
    }
}

/// Canonical result returned by every decoder.
///
/// This is now the **single production result type**.
///
/// MWPM, Union-Find, Identity, future hardware-aware decoders, streaming
/// decoders, and distributed decoders must all return this representation.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub struct DecodeResult {
    /// Decoder-result schema version.
    format_version: u32,

    /// Decoder identity.
    decoder: DecoderId,

    /// Input syndrome used for decoding.
    syndrome: Syndrome,

    /// Physical correction proposed by the decoder.
    correction: Correction,

    /// Execution termination classification.
    termination: DecodeTermination,

    /// Number of decoder iterations.
    iterations: u64,

    /// Optional logical classification.
    ///
    /// `None` means logical equivalence has not yet been evaluated.
    ///
    /// `Some(Unknown)` means logical evaluation was attempted but could not
    /// safely classify the result.
    logical_outcome: Option<LogicalOutcome>,

    /// Runtime resource snapshot.
    resources: DecodeResourceUsage,

    /// Public execution metadata.
    metadata: DecodeMetadata,

    /// Optional deterministic verification witness.
    witness: DecodeWitness,
}

impl DecodeResult {
    /// Creates a basic decoder result.
    #[must_use]
    pub fn new(
        decoder: DecoderId,
        syndrome: Syndrome,
        correction: Correction,
    ) -> Self {
        let termination = if syndrome.is_trivial() {
            DecodeTermination::TrivialInput
        } else {
            DecodeTermination::Completed
        };

        Self {
            format_version: DECODER_RESULT_FORMAT_VERSION,
            decoder,
            syndrome,
            correction,
            termination,
            iterations: 0,
            logical_outcome: None,
            resources: DecodeResourceUsage::new(),
            metadata: DecodeMetadata::new(),
            witness: DecodeWitness::None,
        }
    }

    /// Creates a result with explicit execution information.
    #[must_use]
    pub fn with_execution(
        decoder: DecoderId,
        syndrome: Syndrome,
        correction: Correction,
        termination: DecodeTermination,
        iterations: u64,
    ) -> Self {
        Self {
            format_version: DECODER_RESULT_FORMAT_VERSION,
            decoder,
            syndrome,
            correction,
            termination,
            iterations,
            logical_outcome: None,
            resources: DecodeResourceUsage::new(),
            metadata: DecodeMetadata::new(),
            witness: DecodeWitness::None,
        }
    }

    /// Returns the result schema version.
    #[must_use]
    pub const fn format_version(&self) -> u32 {
        self.format_version
    }

    /// Returns the decoder identity.
    #[must_use]
    pub const fn decoder(&self) -> DecoderId {
        self.decoder
    }

    /// Returns the input syndrome.
    #[must_use]
    pub fn syndrome(&self) -> &Syndrome {
        &self.syndrome
    }

    /// Returns the correction.
    #[must_use]
    pub fn correction(&self) -> &Correction {
        &self.correction
    }

    /// Returns correction weight.
    #[must_use]
    pub fn correction_weight(&self) -> usize {
        self.correction.weight()
    }

    /// Returns termination reason.
    #[must_use]
    pub const fn termination(&self) -> DecodeTermination {
        self.termination
    }

    /// Returns decoder iteration count.
    #[must_use]
    pub const fn iterations(&self) -> u64 {
        self.iterations
    }

    /// Returns whether the input syndrome was trivial.
    #[must_use]
    pub fn is_trivial(&self) -> bool {
        self.syndrome.is_trivial()
    }

    /// Returns whether decoding completed successfully.
    #[must_use]
    pub const fn is_success(&self) -> bool {
        self.termination.is_success()
    }

    /// Returns whether decoding was cancelled.
    #[must_use]
    pub const fn is_cancelled(&self) -> bool {
        self.termination.is_cancelled()
    }

    /// Returns whether the operation was resource limited.
    #[must_use]
    pub const fn is_resource_limited(&self) -> bool {
        self.termination.is_resource_limited()
    }

    /// Returns whether the correction is identity.
    #[must_use]
    pub fn is_identity_correction(&self) -> bool {
        self.correction.is_identity()
    }

    /// Returns the optional logical outcome.
    #[must_use]
    pub const fn logical_outcome(&self) -> Option<LogicalOutcome> {
        self.logical_outcome
    }

    /// Returns true when logical classification has been performed.
    #[must_use]
    pub const fn has_logical_outcome(&self) -> bool {
        self.logical_outcome.is_some()
    }

    /// Returns true when a logical failure was explicitly established.
    #[must_use]
    pub const fn has_logical_failure(&self) -> bool {
        matches!(
            self.logical_outcome,
            Some(
                LogicalOutcome::LogicalX
                    | LogicalOutcome::LogicalY
                    | LogicalOutcome::LogicalZ
            )
        )
    }

    /// Returns the resource snapshot.
    #[must_use]
    pub const fn resources(&self) -> DecodeResourceUsage {
        self.resources
    }

    /// Returns public metadata.
    #[must_use]
    pub const fn metadata(&self) -> &DecodeMetadata {
        &self.metadata
    }

    /// Returns the verification witness.
    #[must_use]
    pub const fn witness(&self) -> &DecodeWitness {
        &self.witness
    }

    /// Attaches a logical outcome produced by
    /// `logical_equivalence.rs`.
    ///
    /// This is intentionally the only semantic path for attaching a logical
    /// outcome after construction.
    #[must_use]
    pub fn with_logical_outcome(
        mut self,
        outcome: LogicalOutcome,
    ) -> Self {
        self.logical_outcome = Some(outcome);
        self
    }

    /// Mutably attaches a logical outcome.
    pub fn set_logical_outcome(
        &mut self,
        outcome: LogicalOutcome,
    ) {
        self.logical_outcome = Some(outcome);
    }

    /// Attaches resource usage.
    #[must_use]
    pub fn with_resources(
        mut self,
        resources: DecodeResourceUsage,
    ) -> Self {
        self.resources = resources;
        self
    }

    /// Attaches public metadata.
    #[must_use]
    pub fn with_metadata(
        mut self,
        metadata: DecodeMetadata,
    ) -> Self {
        self.metadata = metadata;
        self
    }

    /// Attaches a verification witness.
    #[must_use]
    pub fn with_witness(
        mut self,
        witness: DecodeWitness,
    ) -> Self {
        self.witness = witness;
        self
    }

    /// Changes the termination classification.
    #[must_use]
    pub fn with_termination(
        mut self,
        termination: DecodeTermination,
    ) -> Self {
        self.termination = termination;
        self
    }

    /// Changes the iteration count.
    #[must_use]
    pub fn with_iterations(
        mut self,
        iterations: u64,
    ) -> Self {
        self.iterations = iterations;
        self
    }

    /// Validates all structural invariants that can be checked without
    /// knowing the stabilizer code.
    pub fn validate(&self) -> QecResult<()> {
        if self.format_version
            != DECODER_RESULT_FORMAT_VERSION
        {
            return Err(QecError::VersionMismatch {
                component:
                    "decoder_result".to_owned(),
                expected:
                    DECODER_RESULT_FORMAT_VERSION
                        .to_string(),
                actual:
                    self.format_version
                        .to_string(),
                message:
                    "unsupported decoder-result schema version"
                        .to_owned(),
            });
        }

        if self.correction.num_qubits() == 0 {
            return Err(QecError::DecoderFailure {
                decoder: DecoderKind::Custom,
                message:
                    "decoder result contains a zero-qubit correction"
                        .to_owned(),
            });
        }

        if self.is_trivial()
            && !self.correction.is_identity()
            && self.termination.is_success()
        {
            return Err(QecError::DecoderFailure {
                decoder: DecoderKind::Custom,
                message:
                    "successful trivial-syndrome result must use identity correction"
                        .to_owned(),
            });
        }

        if self.logical_outcome
            == Some(LogicalOutcome::Identity)
            && !self.termination.is_success()
        {
            return Err(QecError::DecoderFailure {
                decoder: DecoderKind::Custom,
                message:
                    "logical identity cannot be attached to an unsuccessful decoder result"
                        .to_owned(),
            });
        }

        Ok(())
    }

    /// Validates the correction against a known physical qubit count.
    pub fn validate_qubit_count(
        &self,
        expected: usize,
    ) -> QecResult<()> {
        if self.correction.num_qubits()
            != expected
        {
            return Err(QecError::DecoderFailure {
                decoder: DecoderKind::Custom,
                message: format!(
                    "decoder result correction has {} qubits; expected {expected}",
                    self.correction.num_qubits()
                ),
            });
        }

        Ok(())
    }

    /// Returns a stable semantic fingerprint.
    ///
    /// The fingerprint intentionally excludes:
    ///
    /// - metadata;
    /// - resource usage;
    /// - anything time-dependent.
    ///
    /// This makes it appropriate for deterministic replay and result
    /// equivalence.
    #[must_use]
    pub fn semantic_fingerprint(&self) -> u64 {
        let mut hasher =
            std::collections::hash_map::DefaultHasher::new();

        self.format_version.hash(&mut hasher);
        self.decoder.hash(&mut hasher);
        self.syndrome.hash(&mut hasher);
        self.correction.hash(&mut hasher);
        self.termination.hash(&mut hasher);
        self.iterations.hash(&mut hasher);
        self.logical_outcome.hash(&mut hasher);
        self.witness.hash(&mut hasher);

        hasher.finish()
    }

    /// Determines whether two results have the same semantic result.
    #[must_use]
    pub fn semantically_equal(
        &self,
        other: &Self,
    ) -> bool {
        self.semantic_fingerprint()
            == other.semantic_fingerprint()
    }

    /// Converts a failed result into the canonical QEC error boundary.
    ///
    /// Successful results return `Ok(())`.
    pub fn into_execution_result(
        &self,
    ) -> QecResult<()> {
        self.validate()?;

        match self.termination {
            DecodeTermination::Completed
            | DecodeTermination::TrivialInput => Ok(()),

            DecodeTermination::Cancelled => {
                Err(
                    QecError::CancellationRequested {
                        message:
                            "decoder execution cancelled"
                                .to_owned(),
                    },
                )
            }

            DecodeTermination::ResourceLimited => {
                Err(
                    QecError::ResourceLimitExceeded {
                        resource:
                            super::errors::ResourceKind::Operations,
                        requested: 0,
                        current: 0,
                        limit: 0,
                        message:
                            "decoder execution stopped at a configured resource boundary"
                                .to_owned(),
                    },
                )
            }

            DecodeTermination::Failed => {
                Err(
                    QecError::DecoderFailure {
                        decoder:
                            DecoderKind::Custom,
                        message:
                            "decoder failed to produce a successful result"
                                .to_owned(),
                    },
                )
            }
        }
    }
}

/// Lightweight result builder.
///
/// This is useful for concrete decoders while keeping construction of the
/// canonical result centralized.
#[derive(Debug)]
pub struct DecodeResultBuilder {
    decoder: DecoderId,
    syndrome: Syndrome,
    correction: Correction,
    termination: DecodeTermination,
    iterations: u64,
    logical_outcome: Option<LogicalOutcome>,
    resources: DecodeResourceUsage,
    metadata: DecodeMetadata,
    witness: DecodeWitness,
}

impl DecodeResultBuilder {
    /// Starts a result builder.
    #[must_use]
    pub fn new(
        decoder: DecoderId,
        syndrome: Syndrome,
        correction: Correction,
    ) -> Self {
        Self {
            decoder,
            syndrome,
            correction,
            termination:
                DecodeTermination::Completed,
            iterations: 0,
            logical_outcome: None,
            resources:
                DecodeResourceUsage::new(),
            metadata:
                DecodeMetadata::new(),
            witness:
                DecodeWitness::None,
        }
    }

    /// Sets termination.
    #[must_use]
    pub fn termination(
        mut self,
        value: DecodeTermination,
    ) -> Self {
        self.termination = value;
        self
    }

    /// Sets iteration count.
    #[must_use]
    pub fn iterations(
        mut self,
        value: u64,
    ) -> Self {
        self.iterations = value;
        self
    }

    /// Sets logical outcome.
    #[must_use]
    pub fn logical_outcome(
        mut self,
        value: LogicalOutcome,
    ) -> Self {
        self.logical_outcome = Some(value);
        self
    }

    /// Sets resource usage.
    #[must_use]
    pub fn resources(
        mut self,
        value: DecodeResourceUsage,
    ) -> Self {
        self.resources = value;
        self
    }

    /// Sets metadata.
    #[must_use]
    pub fn metadata(
        mut self,
        value: DecodeMetadata,
    ) -> Self {
        self.metadata = value;
        self
    }

    /// Sets witness.
    #[must_use]
    pub fn witness(
        mut self,
        value: DecodeWitness,
    ) -> Self {
        self.witness = value;
        self
    }

    /// Finalizes and validates the result.
    pub fn build(self) -> QecResult<DecodeResult> {
        let result = DecodeResult {
            format_version:
                DECODER_RESULT_FORMAT_VERSION,
            decoder: self.decoder,
            syndrome: self.syndrome,
            correction: self.correction,
            termination:
                self.termination,
            iterations:
                self.iterations,
            logical_outcome:
                self.logical_outcome,
            resources:
                self.resources,
            metadata:
                self.metadata,
            witness:
                self.witness,
        };

        result.validate()?;

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::stabilizer::Pauli;

    fn syndrome(values: &[bool]) -> Syndrome {
        Syndrome::from_bits(values.to_vec())
            .expect("valid syndrome")
    }

    #[test]
    fn identity_result_is_trivial() {
        let result = DecodeResult::new(
            DecoderId::new(1),
            syndrome(&[false, false]),
            Correction::identity(2),
        );

        assert_eq!(
            result.termination(),
            DecodeTermination::TrivialInput
        );

        assert!(result.is_success());
        assert!(result.is_trivial());
        assert!(result.is_identity_correction());
    }

    #[test]
    fn nontrivial_result_defaults_to_completed() {
        let result = DecodeResult::new(
            DecoderId::new(1),
            syndrome(&[true]),
            Correction::new(
                PauliString::from_paulis(
                    &[Pauli::X],
                ),
            ),
        );

        assert_eq!(
            result.termination(),
            DecodeTermination::Completed
        );

        assert!(result.is_success());
        assert!(!result.is_trivial());
    }

    #[test]
    fn result_rejects_zero_qubit_correction() {
        let result = DecodeResult::new(
            DecoderId::new(1),
            syndrome(&[]),
            Correction::identity(0),
        );

        assert!(result.validate().is_err());
    }

    #[test]
    fn trivial_success_requires_identity_correction() {
        let result = DecodeResult::new(
            DecoderId::new(1),
            syndrome(&[false]),
            Correction::new(
                PauliString::from_paulis(
                    &[Pauli::X],
                ),
            ),
        );

        assert!(result.validate().is_err());
    }

    #[test]
    fn logical_outcome_is_optional_until_verified() {
        let result = DecodeResult::new(
            DecoderId::new(1),
            syndrome(&[true]),
            Correction::new(
                PauliString::from_paulis(
                    &[Pauli::X],
                ),
            ),
        );

        assert_eq!(
            result.logical_outcome(),
            None
        );
    }

    #[test]
    fn logical_outcome_can_be_attached() {
        let result = DecodeResult::new(
            DecoderId::new(1),
            syndrome(&[true]),
            Correction::new(
                PauliString::from_paulis(
                    &[Pauli::X],
                ),
            ),
        )
        .with_logical_outcome(
            LogicalOutcome::LogicalX,
        );

        assert_eq!(
            result.logical_outcome(),
            Some(
                LogicalOutcome::LogicalX
            )
        );

        assert!(
            result.has_logical_failure()
        );
    }

    #[test]
    fn resource_usage_defaults_to_zero() {
        let result = DecodeResult::new(
            DecoderId::new(1),
            syndrome(&[true]),
            Correction::new(
                PauliString::from_paulis(
                    &[Pauli::X],
                ),
            ),
        );

        assert!(
            result.resources().is_empty()
        );
    }

    #[test]
    fn semantic_fingerprint_ignores_resource_usage() {
        let first = DecodeResult::new(
            DecoderId::new(1),
            syndrome(&[true]),
            Correction::new(
                PauliString::from_paulis(
                    &[Pauli::X],
                ),
            ),
        );

        let second = first
            .clone()
            .with_resources(
                DecodeResourceUsage {
                    decoder_iterations: 100,
                    ..DecodeResourceUsage::new()
                },
            );

        assert_eq!(
            first.semantic_fingerprint(),
            second.semantic_fingerprint()
        );
    }

    #[test]
    fn semantic_fingerprint_changes_for_correction() {
        let first = DecodeResult::new(
            DecoderId::new(1),
            syndrome(&[true]),
            Correction::new(
                PauliString::from_paulis(
                    &[Pauli::X],
                ),
            ),
        );

        let second = DecodeResult::new(
            DecoderId::new(1),
            syndrome(&[true]),
            Correction::new(
                PauliString::from_paulis(
                    &[Pauli::Z],
                ),
            ),
        );

        assert_ne!(
            first.semantic_fingerprint(),
            second.semantic_fingerprint()
        );
    }

    #[test]
    fn metadata_is_deterministically_ordered() {
        let mut metadata =
            DecodeMetadata::new();

        metadata.insert(
            "decoder",
            "mwpm",
        );

        metadata.insert(
            "backend",
            "cpu",
        );

        let entries =
            metadata.entries();

        let keys: Vec<&String> =
            entries.keys().collect();

        assert_eq!(
            keys,
            vec![
                &"backend".to_owned(),
                &"decoder".to_owned()
            ]
        );
    }

    #[test]
    fn builder_produces_valid_result() {
        let result =
            DecodeResultBuilder::new(
                DecoderId::new(7),
                syndrome(&[true]),
                Correction::new(
                    PauliString::from_paulis(
                        &[Pauli::X],
                    ),
                ),
            )
            .iterations(3)
            .build()
            .expect("builder should produce valid result");

        assert_eq!(
            result.decoder(),
            DecoderId::new(7)
        );

        assert_eq!(
            result.iterations(),
            3
        );
    }

    #[test]
    fn cancellation_is_not_success() {
        let result =
            DecodeResult::with_execution(
                DecoderId::new(1),
                syndrome(&[true]),
                Correction::identity(1),
                DecodeTermination::Cancelled,
                1,
            );

        assert!(!result.is_success());
        assert!(result.is_cancelled());
        assert!(
            result
                .into_execution_result()
                .is_err()
        );
    }

    #[test]
    fn resource_limitation_is_not_success() {
        let result =
            DecodeResult::with_execution(
                DecoderId::new(1),
                syndrome(&[true]),
                Correction::identity(1),
                DecodeTermination::ResourceLimited,
                1,
            );

        assert!(
            result.is_resource_limited()
        );

        assert!(
            result
                .into_execution_result()
                .is_err()
        );
    }
}