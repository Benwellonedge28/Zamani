//! Zamani Quantum Resilience — Result Verification
//!
//! Path:
//!     src/quantum/resilience/verification/result.rs
//!
//! # Purpose
//!
//! This module implements the result-verification layer of
//! `quantum::resilience`.
//!
//! It answers:
//!
//! > "Is the produced execution result structurally complete and internally
//! > coherent enough to be passed to the higher semantic/provenance/
//! > acceptance verification layers?"
//!
//! This module deliberately does NOT attempt to prove:
//!
//! - quantum-state equivalence;
//! - circuit semantic equivalence;
//! - statistical correctness of a physical device;
//! - QEC decoder correctness;
//! - hardware calibration correctness;
//! - routing correctness;
//! - scheduling correctness;
//! - mitigation correctness;
//! - provenance authenticity;
//! - final resilience acceptance.
//!
//! Those responsibilities belong to their authoritative verification layers.
//!
//! # Architectural position
//!
//! ```text
//!                    canonical quantum::ir
//!                              │
//!                              ▼
//!                       execution result
//!                              │
//!                              ▼
//!                 ResultVerificationInput
//!                              │
//!                              ▼
//!                    ResultVerifierImpl
//!                              │
//!              ┌───────────────┼────────────────┐
//!              ▼               ▼                ▼
//!         structural       cardinality      resource
//!          validity          validity       identity
//!              │               │                │
//!              └───────────────┼────────────────┘
//!                              ▼
//!                    VerificationEvidence
//!                              │
//!                              ▼
//!                     ResilienceVerifier
//!                              │
//!               ┌──────────────┼───────────────┐
//!               ▼              ▼               ▼
//!           semantic       provenance       acceptance
//! ```
//!
//! # Critical architectural rule
//!
//! A valid result is NOT automatically an accepted result.
//!
//! ```text
//! execution success
//!       !=
//! result validity
//!       !=
//! semantic correctness
//!       !=
//! resilience acceptance
//! ```
//!
//! This module therefore produces `VerificationEvidence`, not a final
//! resilience acceptance decision.
//!
//! # Canonical quantum identity
//!
//! Logical and physical resources MUST use the canonical IR types:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! No resilience-local qubit identity is defined here.
//!
//! # Write once, scale everywhere
//!
//! This module contains no:
//!
//! - maximum qubit count;
//! - maximum physical-qubit count;
//! - maximum result count;
//! - fixed shot count;
//! - fixed register width;
//! - fixed machine size;
//! - provider-specific device ID;
//! - provider-specific result format;
//! - hard-coded retry count;
//! - hard-coded fidelity threshold.
//!
//! All concrete limits are supplied by the execution/policy layers.
//!
//! "Infinity" therefore means:
//!
//! > this module introduces no artificial finite machine-size ceiling.
//!
//! A concrete execution remains bounded by the actual resources available to
//! that execution environment.
//!
//! # Determinism
//!
//! Result verification is deterministic for identical inputs and policy.
//!
//! It does not:
//!
//! - read the system clock;
//! - generate random values;
//! - access environment variables;
//! - access global mutable state;
//! - perform hidden I/O;
//! - depend on hash-map iteration order;
//! - call an external service.
//!
//! # Result semantics
//!
//! A result can represent different classes of quantum-computing output:
//!
//! - sampled measurement results;
//! - expectation/observable results;
//! - state-like results;
//! - process/channel results;
//! - classical/control results;
//! - metadata-only execution results;
//! - provider-neutral custom result classes.
//!
//! Result verification therefore MUST NOT assume that every execution produces
//! shots or a classical bit string.
//!
//! # Relationship with other verification modules
//!
//! `invariant.rs` verifies structural resource invariants.
//!
//! `semantic.rs` verifies canonical quantum-program semantics.
//!
//! `confidence.rs` verifies confidence requirements.
//!
//! `provenance.rs` verifies provenance/integrity.
//!
//! `acceptance.rs` determines final acceptance semantics.
//!
//! `verifier.rs` composes these components.
//!
//! This module only verifies the result-layer contract.
//!
//! # Rust contract
//!
//! - Rust 1.97 / 1.97.1
//! - Rust 2021
//! - stable Rust
//! - no nightly features
//! - no unsafe code
//!
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_debug_implementations)]
#![deny(rust_2018_idioms)]

use std::fmt;
use std::sync::Arc;

use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId};
use crate::quantum::resilience::errors::ResilienceResult;

use super::verifier::{
    ResultVerifier,
    VerificationConfidence,
    VerificationEvidence,
    VerificationRequest,
    VerificationStage,
    VerificationState,
    VerificationViolation,
    VerificationViolationKind,
};

// =============================================================================
// Schema identity
// =============================================================================

/// Stable schema identifier for this result-verification contract.
pub const RESULT_VERIFICATION_SCHEMA_ID: &str =
    "zamani.quantum.resilience.verification.result";

/// Current semantic contract version.
pub const RESULT_VERIFICATION_SCHEMA_VERSION: u32 = 1;

// =============================================================================
// Stable error/violation codes
// =============================================================================

const CODE_MISSING_RESULT: &str = "QR-VER-RESULT-001";
const CODE_EXECUTION_NOT_SUCCESSFUL: &str = "QR-VER-RESULT-002";
const CODE_EMPTY_RESULT_ID: &str = "QR-VER-RESULT-003";
const CODE_UNKNOWN_RESULT_KIND: &str = "QR-VER-RESULT-004";
const CODE_INVALID_CARDINALITY: &str = "QR-VER-RESULT-005";
const CODE_DUPLICATE_LOGICAL_RESOURCE: &str = "QR-VER-RESULT-006";
const CODE_DUPLICATE_PHYSICAL_RESOURCE: &str = "QR-VER-RESULT-007";
const CODE_EMPTY_DIGEST: &str = "QR-VER-RESULT-008";
const CODE_INVALID_CLASSICAL_WIDTH: &str = "QR-VER-RESULT-009";
const CODE_RESULT_NOT_TERMINAL: &str = "QR-VER-RESULT-010";
const CODE_INCONSISTENT_RESOURCE_SCOPE: &str = "QR-VER-RESULT-011";

// =============================================================================
// Result kind
// =============================================================================

/// Provider-independent classification of an execution result.
///
/// This classification intentionally describes the semantic *shape* of a
/// result without defining its provider-specific representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ResultKind {
    /// Measurement samples or a distribution of classical outcomes.
    Samples,

    /// An expectation value or collection of observable estimates.
    Expectation,

    /// A state-like result, such as a simulator state representation.
    State,

    /// A process/channel representation.
    Process,

    /// A classical result produced by quantum/classical control.
    Classical,

    /// An execution result containing metadata but no quantum/classical
    /// payload.
    Metadata,

    /// A provider-neutral extension point.
    Custom,
}

impl ResultKind {
    /// Stable machine-readable representation.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Samples => "samples",
            Self::Expectation => "expectation",
            Self::State => "state",
            Self::Process => "process",
            Self::Classical => "classical",
            Self::Metadata => "metadata",
            Self::Custom => "custom",
        }
    }

    /// Returns whether the result kind normally represents a payload-bearing
    /// result.
    ///
    /// This is descriptive only. It does not establish correctness.
    #[must_use]
    pub const fn is_payload_bearing(self) -> bool {
        !matches!(self, Self::Metadata)
    }
}

impl fmt::Display for ResultKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Result lifecycle
// =============================================================================

/// Lifecycle state of the result artifact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ResultLifecycle {
    /// Result has not yet been produced.
    NotAvailable,

    /// Result is being produced.
    InProgress,

    /// Result is complete and terminal.
    Complete,

    /// Result production failed.
    Failed,

    /// Result production was cancelled.
    Cancelled,

    /// Result production was aborted.
    Aborted,

    /// Result state cannot currently be established.
    Unknown,
}

impl ResultLifecycle {
    /// Stable machine-readable representation.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotAvailable => "not_available",
            Self::InProgress => "in_progress",
            Self::Complete => "complete",
            Self::Failed => "failed",
            Self::Cancelled => "cancelled",
            Self::Aborted => "aborted",
            Self::Unknown => "unknown",
        }
    }

    /// Returns whether the result is in a terminal successful state.
    #[must_use]
    pub const fn is_successful_terminal(self) -> bool {
        matches!(self, Self::Complete)
    }

    /// Returns whether the result is terminal but unsuccessful.
    #[must_use]
    pub const fn is_terminal_failure(self) -> bool {
        matches!(
            self,
            Self::Failed | Self::Cancelled | Self::Aborted
        )
    }

    /// Returns whether the result is still pending.
    #[must_use]
    pub const fn is_pending(self) -> bool {
        matches!(self, Self::NotAvailable | Self::InProgress)
    }
}

impl Default for ResultLifecycle {
    fn default() -> Self {
        Self::NotAvailable
    }
}

impl fmt::Display for ResultLifecycle {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Result cardinality
// =============================================================================

/// Describes how much result data was produced.
///
/// The verifier does not require every result to have a cardinality. For
/// example, a metadata-only execution can legitimately have no result
/// observations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ResultCardinality {
    /// Cardinality is not applicable or not supplied.
    NotApplicable,

    /// Cardinality is not known by the producer.
    Unknown,

    /// Exactly zero logical observations were produced.
    Zero,

    /// A finite positive number of observations were produced.
    NonZero(u64),
}

impl ResultCardinality {
    /// Creates a non-zero cardinality without overflow.
    #[must_use]
    pub const fn non_zero(value: u64) -> Option<Self> {
        if value == 0 {
            None
        } else {
            Some(Self::NonZero(value))
        }
    }

    /// Returns whether this cardinality explicitly represents zero.
    #[must_use]
    pub const fn is_zero(self) -> bool {
        matches!(self, Self::Zero)
    }

    /// Returns whether this cardinality explicitly contains positive data.
    #[must_use]
    pub const fn is_non_zero(self) -> bool {
        matches!(self, Self::NonZero(_))
    }

    /// Returns the positive cardinality if available.
    #[must_use]
    pub const fn value(self) -> Option<u64> {
        match self {
            Self::NonZero(value) => Some(value),
            Self::NotApplicable | Self::Unknown | Self::Zero => None,
        }
    }

    /// Returns stable machine-readable classification.
    #[must_use]
    pub const fn class(self) -> &'static str {
        match self {
            Self::NotApplicable => "not_applicable",
            Self::Unknown => "unknown",
            Self::Zero => "zero",
            Self::NonZero(_) => "non_zero",
        }
    }
}

impl Default for ResultCardinality {
    fn default() -> Self {
        Self::Unknown
    }
}

// =============================================================================
// Resource scope
// =============================================================================

/// Canonical resource scope attached to an execution result.
///
/// Logical and physical resources are deliberately kept in separate typed
/// collections.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ResultResourceScope {
    logical_qubits: Arc<[QubitId]>,
    physical_qubits: Arc<[PhysicalQubitId]>,
}

impl ResultResourceScope {
    /// Creates a resource scope.
    ///
    /// No implicit logical-to-physical mapping is inferred.
    #[must_use]
    pub fn new<L, P>(
        logical_qubits: L,
        physical_qubits: P,
    ) -> Self
    where
        L: IntoIterator<Item = QubitId>,
        P: IntoIterator<Item = PhysicalQubitId>,
    {
        Self {
            logical_qubits: logical_qubits.into_iter().collect(),
            physical_qubits: physical_qubits.into_iter().collect(),
        }
    }

    /// Creates an empty scope.
    #[must_use]
    pub fn empty() -> Self {
        Self::default()
    }

    /// Returns logical qubits associated with the result.
    #[must_use]
    pub fn logical_qubits(&self) -> &[QubitId] {
        self.logical_qubits.as_ref()
    }

    /// Returns physical qubits associated with the result.
    #[must_use]
    pub fn physical_qubits(&self) -> &[PhysicalQubitId] {
        self.physical_qubits.as_ref()
    }

    /// Returns the logical resource count.
    #[must_use]
    pub fn logical_count(&self) -> usize {
        self.logical_qubits.len()
    }

    /// Returns the physical resource count.
    #[must_use]
    pub fn physical_count(&self) -> usize {
        self.physical_qubits.len()
    }

    /// Returns whether both identity domains are empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.logical_qubits.is_empty()
            && self.physical_qubits.is_empty()
    }

    fn has_duplicate_logical_qubits(&self) -> bool {
        has_duplicates(self.logical_qubits())
    }

    fn has_duplicate_physical_qubits(&self) -> bool {
        has_duplicates(self.physical_qubits())
    }
}

fn has_duplicates<T>(values: &[T]) -> bool
where
    T: Ord + Copy,
{
    if values.len() < 2 {
        return false;
    }

    let mut sorted = values.to_vec();
    sorted.sort_unstable();

    sorted.windows(2).any(|pair| pair[0] == pair[1])
}

// =============================================================================
// Result artifact identity
// =============================================================================

/// Stable provider-independent identifier for a result artifact.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ResultArtifactId(Arc<str>);

impl ResultArtifactId {
    /// Creates an artifact identifier.
    ///
    /// Empty or whitespace-only identifiers are rejected.
    pub fn new(value: impl Into<Arc<str>>) -> Result<Self, ResultArtifactIdError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(ResultArtifactIdError::Empty);
        }

        Ok(Self(value))
    }

    /// Returns the identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

impl fmt::Display for ResultArtifactId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Construction error for [`ResultArtifactId`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResultArtifactIdError {
    /// The supplied identifier was empty.
    Empty,
}

impl fmt::Display for ResultArtifactIdError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("result artifact identifier is empty"),
        }
    }
}

impl std::error::Error for ResultArtifactIdError {}

// =============================================================================
// Result integrity digest
// =============================================================================

/// Opaque digest of the result payload.
///
/// Hashing/cryptographic implementation belongs to the producing subsystem.
/// This module only verifies that an explicitly supplied digest is structurally
/// usable.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ResultDigest(Arc<str>);

impl ResultDigest {
    /// Creates a result digest.
    pub fn new(value: impl Into<Arc<str>>) -> Result<Self, ResultDigestError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(ResultDigestError::Empty);
        }

        Ok(Self(value))
    }

    /// Returns the digest as an opaque string.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

impl fmt::Display for ResultDigest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Construction error for [`ResultDigest`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResultDigestError {
    /// The supplied digest was empty.
    Empty,
}

impl fmt::Display for ResultDigestError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("result digest is empty"),
        }
    }
}

impl std::error::Error for ResultDigestError {}

// =============================================================================
// Result observation
// =============================================================================

/// Provider-independent description of one completed execution result.
///
/// This structure intentionally stores result metadata rather than a
/// provider-specific payload representation.
///
/// Large payloads should remain owned by the execution/result subsystem and be
/// referenced by an artifact identifier/digest.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResultObservation {
    artifact_id: ResultArtifactId,
    kind: ResultKind,
    lifecycle: ResultLifecycle,
    cardinality: ResultCardinality,
    resource_scope: ResultResourceScope,
    classical_width: Option<u64>,
    digest: Option<ResultDigest>,
}

impl ResultObservation {
    /// Creates a result observation.
    #[must_use]
    pub fn new(
        artifact_id: ResultArtifactId,
        kind: ResultKind,
        lifecycle: ResultLifecycle,
    ) -> Self {
        Self {
            artifact_id,
            kind,
            lifecycle,
            cardinality: ResultCardinality::Unknown,
            resource_scope: ResultResourceScope::empty(),
            classical_width: None,
            digest: None,
        }
    }

    /// Sets result cardinality.
    #[must_use]
    pub const fn with_cardinality(
        mut self,
        cardinality: ResultCardinality,
    ) -> Self {
        self.cardinality = cardinality;
        self
    }

    /// Sets the resource scope.
    #[must_use]
    pub fn with_resource_scope(
        mut self,
        scope: ResultResourceScope,
    ) -> Self {
        self.resource_scope = scope;
        self
    }

    /// Sets the classical result width.
    ///
    /// Width is represented as `u64` because the semantic width is not tied to
    /// any particular machine size. Actual materialization remains bounded by
    /// execution resources.
    #[must_use]
    pub const fn with_classical_width(
        mut self,
        width: u64,
    ) -> Self {
        self.classical_width = Some(width);
        self
    }

    /// Sets the result integrity digest.
    #[must_use]
    pub fn with_digest(mut self, digest: ResultDigest) -> Self {
        self.digest = Some(digest);
        self
    }

    /// Returns the artifact identity.
    #[must_use]
    pub fn artifact_id(&self) -> &ResultArtifactId {
        &self.artifact_id
    }

    /// Returns the result kind.
    #[must_use]
    pub const fn kind(&self) -> ResultKind {
        self.kind
    }

    /// Returns lifecycle state.
    #[must_use]
    pub const fn lifecycle(&self) -> ResultLifecycle {
        self.lifecycle
    }

    /// Returns cardinality.
    #[must_use]
    pub const fn cardinality(&self) -> ResultCardinality {
        self.cardinality
    }

    /// Returns resource scope.
    #[must_use]
    pub fn resource_scope(&self) -> &ResultResourceScope {
        &self.resource_scope
    }

    /// Returns optional classical width.
    #[must_use]
    pub const fn classical_width(&self) -> Option<u64> {
        self.classical_width
    }

    /// Returns optional digest.
    #[must_use]
    pub fn digest(&self) -> Option<&ResultDigest> {
        self.digest.as_ref()
    }
}

// =============================================================================
// Result verification policy
// =============================================================================

/// Policy controlling result-level validation.
///
/// This policy does not decide final resilience acceptance.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResultVerificationPolicy {
    require_observation: bool,
    require_successful_execution: bool,
    require_terminal_result: bool,
    require_artifact_identity: bool,
    require_known_kind: bool,
    reject_duplicate_resources: bool,
    require_digest: bool,
    require_classical_width_for_classical_results: bool,
    minimum_cardinality: Option<u64>,
}

impl ResultVerificationPolicy {
    /// Strict production policy.
    ///
    /// Notice that it does not require a digest or cardinality because those
    /// are not semantically applicable to every result class.
    #[must_use]
    pub const fn production() -> Self {
        Self {
            require_observation: true,
            require_successful_execution: true,
            require_terminal_result: true,
            require_artifact_identity: true,
            require_known_kind: true,
            reject_duplicate_resources: true,
            require_digest: false,
            require_classical_width_for_classical_results: true,
            minimum_cardinality: None,
        }
    }

    /// Diagnostic policy for incomplete execution pipelines.
    #[must_use]
    pub const fn diagnostic() -> Self {
        Self {
            require_observation: false,
            require_successful_execution: false,
            require_terminal_result: false,
            require_artifact_identity: false,
            require_known_kind: false,
            reject_duplicate_resources: true,
            require_digest: false,
            require_classical_width_for_classical_results: false,
            minimum_cardinality: None,
        }
    }

    /// Sets whether an observation must be supplied.
    #[must_use]
    pub const fn with_required_observation(
        mut self,
        required: bool,
    ) -> Self {
        self.require_observation = required;
        self
    }

    /// Sets whether execution must have succeeded.
    #[must_use]
    pub const fn with_successful_execution(
        mut self,
        required: bool,
    ) -> Self {
        self.require_successful_execution = required;
        self
    }

    /// Sets whether the result must be terminal.
    #[must_use]
    pub const fn with_terminal_result(
        mut self,
        required: bool,
    ) -> Self {
        self.require_terminal_result = required;
        self
    }

    /// Sets whether a non-empty result artifact identity is required.
    #[must_use]
    pub const fn with_artifact_identity(
        mut self,
        required: bool,
    ) -> Self {
        self.require_artifact_identity = required;
        self
    }

    /// Sets whether the result kind must be known.
    #[must_use]
    pub const fn with_known_kind(
        mut self,
        required: bool,
    ) -> Self {
        self.require_known_kind = required;
        self
    }

    /// Sets whether duplicate canonical resource identities are rejected.
    #[must_use]
    pub const fn with_duplicate_resource_rejection(
        mut self,
        reject: bool,
    ) -> Self {
        self.reject_duplicate_resources = reject;
        self
    }

    /// Sets whether a result digest is mandatory.
    #[must_use]
    pub const fn with_required_digest(
        mut self,
        required: bool,
    ) -> Self {
        self.require_digest = required;
        self
    }

    /// Sets whether classical results must report their width.
    #[must_use]
    pub const fn with_classical_width_requirement(
        mut self,
        required: bool,
    ) -> Self {
        self.require_classical_width_for_classical_results = required;
        self
    }

    /// Sets an optional minimum result cardinality.
    ///
    /// `None` means no cardinality threshold is imposed by this module.
    ///
    /// A value of zero is allowed and means that the policy explicitly
    /// permits zero cardinality.
    #[must_use]
    pub const fn with_minimum_cardinality(
        mut self,
        minimum: Option<u64>,
    ) -> Self {
        self.minimum_cardinality = minimum;
        self
    }

    /// Returns whether an observation is required.
    #[must_use]
    pub const fn requires_observation(&self) -> bool {
        self.require_observation
    }

    /// Returns whether execution success is required.
    #[must_use]
    pub const fn requires_successful_execution(&self) -> bool {
        self.require_successful_execution
    }

    /// Returns whether terminal completion is required.
    #[must_use]
    pub const fn requires_terminal_result(&self) -> bool {
        self.require_terminal_result
    }

    /// Returns whether artifact identity is required.
    #[must_use]
    pub const fn requires_artifact_identity(&self) -> bool {
        self.require_artifact_identity
    }

    /// Returns whether known result kind is required.
    #[must_use]
    pub const fn requires_known_kind(&self) -> bool {
        self.require_known_kind
    }

    /// Returns whether duplicate resources are rejected.
    #[must_use]
    pub const fn rejects_duplicate_resources(&self) -> bool {
        self.reject_duplicate_resources
    }

    /// Returns whether a digest is required.
    #[must_use]
    pub const fn requires_digest(&self) -> bool {
        self.require_digest
    }

    /// Returns whether classical width is required.
    #[must_use]
    pub const fn requires_classical_width(&self) -> bool {
        self.require_classical_width_for_classical_results
    }

    /// Returns the optional cardinality threshold.
    #[must_use]
    pub const fn minimum_cardinality(&self) -> Option<u64> {
        self.minimum_cardinality
    }
}

impl Default for ResultVerificationPolicy {
    fn default() -> Self {
        Self::production()
    }
}

// =============================================================================
// Result verifier implementation
// =============================================================================

/// Production result verifier.
///
/// The verifier is intentionally immutable and contains no execution state.
/// A new instance can therefore be safely shared between concurrent
/// verification operations.
///
/// The actual result observation is supplied through
/// [`ResultVerifierContext`].
#[derive(Debug, Clone)]
pub struct ProductionResultVerifier {
    policy: ResultVerificationPolicy,
    observation: Option<Arc<ResultObservation>>,
    required_confidence: VerificationConfidence,
}

impl ProductionResultVerifier {
    /// Creates a production verifier without an observation.
    ///
    /// In production mode, verification will return `Indeterminate` because
    /// result evidence is required.
    #[must_use]
    pub fn new() -> Self {
        Self {
            policy: ResultVerificationPolicy::production(),
            observation: None,
            required_confidence: VerificationConfidence::certain(),
        }
    }

    /// Creates a verifier with an explicit result observation.
    #[must_use]
    pub fn with_observation(
        observation: ResultObservation,
    ) -> Self {
        Self {
            policy: ResultVerificationPolicy::production(),
            observation: Some(Arc::new(observation)),
            required_confidence: VerificationConfidence::certain(),
        }
    }

    /// Creates a verifier using an explicit result policy.
    #[must_use]
    pub fn with_policy(
        policy: ResultVerificationPolicy,
    ) -> Self {
        Self {
            policy,
            observation: None,
            required_confidence: VerificationConfidence::certain(),
        }
    }

    /// Sets the result observation.
    #[must_use]
    pub fn with_result(
        mut self,
        observation: ResultObservation,
    ) -> Self {
        self.observation = Some(Arc::new(observation));
        self
    }

    /// Sets the minimum evidence confidence emitted by this verifier.
    ///
    /// This does not replace `confidence.rs`. It only records the confidence
    /// attached to the structural result evidence.
    #[must_use]
    pub const fn with_required_confidence(
        mut self,
        confidence: VerificationConfidence,
    ) -> Self {
        self.required_confidence = confidence;
        self
    }

    /// Returns the configured policy.
    #[must_use]
    pub const fn policy(&self) -> &ResultVerificationPolicy {
        &self.policy
    }

    /// Returns the configured observation, if present.
    #[must_use]
    pub fn observation(&self) -> Option<&ResultObservation> {
        self.observation.as_deref()
    }

    /// Performs direct result verification.
    ///
    /// This method is useful for callers that already have a
    /// `ResultObservation` and do not need the trait-object boundary.
    pub fn verify_observation(
        &self,
        request: &VerificationRequest,
    ) -> VerificationEvidence {
        let Some(observation) = self.observation.as_deref() else {
            if self.policy.requires_observation() {
                return VerificationEvidence::new(
                    VerificationState::Indeterminate,
                    VerificationConfidence::none(),
                    [VerificationViolation::new(
                        VerificationStage::Result,
                        VerificationViolationKind::MissingEvidence,
                        CODE_MISSING_RESULT,
                        "result verification requires an execution result observation",
                    )],
                );
            }

            return VerificationEvidence::not_required();
        };

        let mut violations = Vec::new();

        if self.policy.requires_successful_execution()
            && !request.execution_completed_successfully()
        {
            violations.push(VerificationViolation::new(
                VerificationStage::Result,
                VerificationViolationKind::ExecutionFailure,
                CODE_EXECUTION_NOT_SUCCESSFUL,
                "the resilience verification request does not report successful execution",
            ));
        }

        if self.policy.requires_terminal_result()
            && !observation.lifecycle().is_successful_terminal()
        {
            violations.push(VerificationViolation::new(
                VerificationStage::Result,
                VerificationViolationKind::ResultInvalid,
                CODE_RESULT_NOT_TERMINAL,
                format!(
                    "result lifecycle is `{}` rather than a successful terminal state",
                    observation.lifecycle()
                ),
            ));
        }

        if self.policy.requires_artifact_identity()
            && observation.artifact_id().as_str().trim().is_empty()
        {
            violations.push(VerificationViolation::new(
                VerificationStage::Result,
                VerificationViolationKind::ResultInvalid,
                CODE_EMPTY_RESULT_ID,
                "result artifact identifier is empty",
            ));
        }

        if self.policy.requires_known_kind
            && !is_known_result_kind(observation.kind())
        {
            violations.push(VerificationViolation::new(
                VerificationStage::Result,
                VerificationViolationKind::ResultInvalid,
                CODE_UNKNOWN_RESULT_KIND,
                "result kind is not recognized by the result verification contract",
            ));
        }

        if self.policy.rejects_duplicate_resources() {
            if observation
                .resource_scope()
                .has_duplicate_logical_qubits()
            {
                violations.push(VerificationViolation::new(
                    VerificationStage::Result,
                    VerificationViolationKind::ResourceIdentityMismatch,
                    CODE_DUPLICATE_LOGICAL_RESOURCE,
                    "result resource scope contains duplicate logical qubit identities",
                ));
            }

            if observation
                .resource_scope()
                .has_duplicate_physical_qubits()
            {
                violations.push(VerificationViolation::new(
                    VerificationStage::Result,
                    VerificationViolationKind::ResourceIdentityMismatch,
                    CODE_DUPLICATE_PHYSICAL_RESOURCE,
                    "result resource scope contains duplicate physical qubit identities",
                ));
            }
        }

        if self.policy.requires_digest()
            && observation.digest().is_none()
        {
            violations.push(VerificationViolation::new(
                VerificationStage::Result,
                VerificationViolationKind::MissingEvidence,
                CODE_EMPTY_DIGEST,
                "result integrity digest is required by result verification policy",
            ));
        }

        if let Some(width) = observation.classical_width() {
            // A width of zero is valid for an explicitly empty classical
            // result only when the producer selected the corresponding result
            // semantics. This verifier therefore does not impose a universal
            // positive-width rule.
            let _ = width;
        } else if self.policy.requires_classical_width()
            && matches!(
                observation.kind(),
                ResultKind::Classical
            )
        {
            violations.push(VerificationViolation::new(
                VerificationStage::Result,
                VerificationViolationKind::MissingEvidence,
                CODE_INVALID_CLASSICAL_WIDTH,
                "classical result verification requires explicit classical width",
            ));
        }

        if let Some(minimum) = self.policy.minimum_cardinality() {
            match observation.cardinality() {
                ResultCardinality::NonZero(value) if value >= minimum => {}
                ResultCardinality::Zero if minimum == 0 => {}
                ResultCardinality::NotApplicable if minimum == 0 => {}
                ResultCardinality::Unknown => {
                    violations.push(VerificationViolation::new(
                        VerificationStage::Result,
                        VerificationViolationKind::MissingEvidence,
                        CODE_INVALID_CARDINALITY,
                        "result cardinality is required but was not supplied",
                    ));
                }
                ResultCardinality::Zero
                | ResultCardinality::NonZero(_) => {
                    violations.push(VerificationViolation::new(
                        VerificationStage::Result,
                        VerificationViolationKind::ResultInvalid,
                        CODE_INVALID_CARDINALITY,
                        "result cardinality does not satisfy the configured minimum",
                    ));
                }
                ResultCardinality::NotApplicable => {
                    violations.push(VerificationViolation::new(
                        VerificationStage::Result,
                        VerificationViolationKind::ResultInvalid,
                        CODE_INVALID_CARDINALITY,
                        "result cardinality is not applicable but a positive minimum was required",
                    ));
                }
            }
        }

        /*
         * The result verifier intentionally does NOT require:
         *
         *     result logical qubits == request logical qubits
         *     result physical qubits == request physical qubits
         *
         * Such equality would be incorrect for:
         *
         * - measurement of a subset of qubits;
         * - logical/physical remapping;
         * - encoded logical qubits;
         * - distributed execution;
         * - simulator results;
         * - observable-only workloads;
         * - mid-circuit measurement;
         * - dynamically allocated resources.
         *
         * Resource relationship verification belongs to the canonical
         * invariant/resource-identity layers.
         */

        let state = if violations.is_empty() {
            VerificationState::Passed
        } else {
            VerificationState::Failed
        };

        let confidence = if violations.is_empty() {
            self.required_confidence
        } else {
            VerificationConfidence::none()
        };

        VerificationEvidence::new(
            state,
            confidence,
            violations,
        )
    }
}

impl Default for ProductionResultVerifier {
    fn default() -> Self {
        Self::new()
    }
}

impl ResultVerifier for ProductionResultVerifier {
    fn id(&self) -> &str {
        "zamani.resilience.verification.result.production"
    }

    fn verify_result(
        &self,
        request: &VerificationRequest,
    ) -> ResilienceResult<VerificationEvidence> {
        Ok(self.verify_observation(request))
    }
}

// =============================================================================
// Result verification context
// =============================================================================

/// Explicit context for result verification.
///
/// This is a convenience wrapper for execution layers that need to construct
/// result evidence before passing it into the resilience verifier.
#[derive(Debug, Clone)]
pub struct ResultVerifierContext {
    observation: Arc<ResultObservation>,
}

impl ResultVerifierContext {
    /// Creates a result verification context.
    #[must_use]
    pub fn new(observation: ResultObservation) -> Self {
        Self {
            observation: Arc::new(observation),
        }
    }

    /// Returns the immutable result observation.
    #[must_use]
    pub fn observation(&self) -> &ResultObservation {
        self.observation.as_ref()
    }

    /// Creates a production verifier using this context.
    #[must_use]
    pub fn verifier(&self) -> ProductionResultVerifier {
        ProductionResultVerifier::with_observation(
            self.observation.as_ref().clone(),
        )
    }
}

// =============================================================================
// Validation helpers
// =============================================================================

fn is_known_result_kind(kind: ResultKind) -> bool {
    matches!(
        kind,
        ResultKind::Samples
            | ResultKind::Expectation
            | ResultKind::State
            | ResultKind::Process
            | ResultKind::Classical
            | ResultKind::Metadata
            | ResultKind::Custom
    )
}

// =============================================================================
// Standalone result validation
// =============================================================================

/// Validates a result observation using the production result policy.
///
/// This is useful for execution subsystems that want result-level validation
/// before constructing the full resilience verification lifecycle.
pub fn validate_result(
    request: &VerificationRequest,
    observation: &ResultObservation,
) -> VerificationEvidence {
    ProductionResultVerifier::with_observation(
        observation.clone(),
    )
    .verify_observation(request)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::quantum::ir::qubit::{
        PhysicalQubitId,
        QubitId,
    };
    use crate::quantum::resilience::errors::ResilienceResult;

    fn request() -> VerificationRequest {
        let execution_id =
            super::super::verifier::ExecutionIdentity::new(
                "test-execution",
            )
            .expect("execution identity must be valid");

        let fingerprint =
            super::super::verifier::SemanticFingerprint::new(
                "test-semantic-fingerprint",
            )
            .expect("fingerprint must be valid");

        let resources =
            super::super::verifier::VerificationResourceScope::new(
                [QubitId::new(0)],
                [PhysicalQubitId::new(0)],
            );

        VerificationRequest::new(
            execution_id,
            fingerprint,
            resources,
        )
        .with_execution_success()
    }

    fn completed_samples() -> ResultObservation {
        let artifact =
            ResultArtifactId::new("result-0")
                .expect("artifact ID must be valid");

        ResultObservation::new(
            artifact,
            ResultKind::Samples,
            ResultLifecycle::Complete,
        )
        .with_cardinality(ResultCardinality::NonZero(1))
        .with_resource_scope(
            ResultResourceScope::new(
                [QubitId::new(0)],
                [PhysicalQubitId::new(0)],
            ),
        )
    }

    #[test]
    fn production_result_verifier_accepts_structurally_valid_result() {
        let observation = completed_samples();

        let verifier =
            ProductionResultVerifier::with_observation(
                observation,
            );

        let evidence = verifier
            .verify_observation(&request());

        assert_eq!(
            evidence.state(),
            VerificationState::Passed
        );
        assert!(evidence.violations().is_empty());
        assert_eq!(
            evidence.confidence(),
            VerificationConfidence::certain()
        );
    }

    #[test]
    fn missing_result_is_indeterminate_in_production_mode() {
        let verifier =
            ProductionResultVerifier::new();

        let evidence = verifier
            .verify_observation(&request());

        assert_eq!(
            evidence.state(),
            VerificationState::Indeterminate
        );

        assert_eq!(
            evidence.violations().len(),
            1
        );

        assert_eq!(
            evidence.violations()[0].kind(),
            VerificationViolationKind::MissingEvidence
        );
    }

    #[test]
    fn failed_execution_blocks_result_verification() {
        let observation = completed_samples();

        let execution_id =
            super::super::verifier::ExecutionIdentity::new(
                "test-execution",
            )
            .expect("execution identity must be valid");

        let fingerprint =
            super::super::verifier::SemanticFingerprint::new(
                "test-semantic-fingerprint",
            )
            .expect("fingerprint must be valid");

        let resources =
            super::super::verifier::VerificationResourceScope::new(
                [QubitId::new(0)],
                [PhysicalQubitId::new(0)],
            );

        let request =
            VerificationRequest::new(
                execution_id,
                fingerprint,
                resources,
            );

        let verifier =
            ProductionResultVerifier::with_observation(
                observation,
            );

        let evidence = verifier
            .verify_observation(&request);

        assert_eq!(
            evidence.state(),
            VerificationState::Failed
        );

        assert!(
            evidence
                .violations()
                .iter()
                .any(|violation| {
                    violation.kind()
                        == VerificationViolationKind::ExecutionFailure
                })
        );
    }

    #[test]
    fn incomplete_result_is_rejected() {
        let artifact =
            ResultArtifactId::new("incomplete")
                .expect("artifact ID must be valid");

        let observation =
            ResultObservation::new(
                artifact,
                ResultKind::Samples,
                ResultLifecycle::InProgress,
            );

        let verifier =
            ProductionResultVerifier::with_observation(
                observation,
            );

        let evidence = verifier
            .verify_observation(&request());

        assert_eq!(
            evidence.state(),
            VerificationState::Failed
        );

        assert!(
            evidence
                .violations()
                .iter()
                .any(|violation| {
                    violation.kind()
                        == VerificationViolationKind::ResultInvalid
                })
        );
    }

    #[test]
    fn duplicate_logical_resources_are_rejected() {
        let artifact =
            ResultArtifactId::new("duplicate")
                .expect("artifact ID must be valid");

        let observation =
            ResultObservation::new(
                artifact,
                ResultKind::Samples,
                ResultLifecycle::Complete,
            )
            .with_resource_scope(
                ResultResourceScope::new(
                    [
                        QubitId::new(0),
                        QubitId::new(0),
                    ],
                    [PhysicalQubitId::new(0)],
                ),
            );

        let verifier =
            ProductionResultVerifier::with_observation(
                observation,
            );

        let evidence = verifier
            .verify_observation(&request());

        assert_eq!(
            evidence.state(),
            VerificationState::Failed
        );

        assert!(
            evidence
                .violations()
                .iter()
                .any(|violation| {
                    violation.kind()
                        == VerificationViolationKind::ResourceIdentityMismatch
                })
        );
    }

    #[test]
    fn duplicate_physical_resources_are_rejected() {
        let artifact =
            ResultArtifactId::new("duplicate-physical")
                .expect("artifact ID must be valid");

        let observation =
            ResultObservation::new(
                artifact,
                ResultKind::Samples,
                ResultLifecycle::Complete,
            )
            .with_resource_scope(
                ResultResourceScope::new(
                    [QubitId::new(0), QubitId::new(1)],
                    [
                        PhysicalQubitId::new(0),
                        PhysicalQubitId::new(0),
                    ],
                ),
            );

        let verifier =
            ProductionResultVerifier::with_observation(
                observation,
            );

        let evidence = verifier
            .verify_observation(&request());

        assert_eq!(
            evidence.state(),
            VerificationState::Failed
        );

        assert!(
            evidence
                .violations()
                .iter()
                .any(|violation| {
                    violation.kind()
                        == VerificationViolationKind::ResourceIdentityMismatch
                })
        );
    }

    #[test]
    fn metadata_result_does_not_require_cardinality() {
        let artifact =
            ResultArtifactId::new("metadata")
                .expect("artifact ID must be valid");

        let observation =
            ResultObservation::new(
                artifact,
                ResultKind::Metadata,
                ResultLifecycle::Complete,
            )
            .with_cardinality(
                ResultCardinality::NotApplicable,
            );

        let verifier =
            ProductionResultVerifier::with_observation(
                observation,
            );

        let evidence = verifier
            .verify_observation(&request());

        assert_eq!(
            evidence.state(),
            VerificationState::Passed
        );
    }

    #[test]
    fn classical_result_requires_width_by_default() {
        let artifact =
            ResultArtifactId::new("classical")
                .expect("artifact ID must be valid");

        let observation =
            ResultObservation::new(
                artifact,
                ResultKind::Classical,
                ResultLifecycle::Complete,
            );

        let verifier =
            ProductionResultVerifier::with_observation(
                observation,
            );

        let evidence = verifier
            .verify_observation(&request());

        assert_eq!(
            evidence.state(),
            VerificationState::Failed
        );

        assert!(
            evidence
                .violations()
                .iter()
                .any(|violation| {
                    violation.kind()
                        == VerificationViolationKind::MissingEvidence
                })
        );
    }

    #[test]
    fn classical_result_with_zero_width_can_be_explicitly_represented() {
        let artifact =
            ResultArtifactId::new("empty-classical")
                .expect("artifact ID must be valid");

        let observation =
            ResultObservation::new(
                artifact,
                ResultKind::Classical,
                ResultLifecycle::Complete,
            )
            .with_classical_width(0);

        let verifier =
            ProductionResultVerifier::with_observation(
                observation,
            );

        let evidence = verifier
            .verify_observation(&request());

        assert_eq!(
            evidence.state(),
            VerificationState::Passed
        );
    }

    #[test]
    fn cardinality_policy_is_configurable() {
        let artifact =
            ResultArtifactId::new("cardinality")
                .expect("artifact ID must be valid");

        let observation =
            ResultObservation::new(
                artifact,
                ResultKind::Samples,
                ResultLifecycle::Complete,
            )
            .with_cardinality(
                ResultCardinality::NonZero(2),
            );

        let policy =
            ResultVerificationPolicy::production()
                .with_minimum_cardinality(Some(2));

        let verifier =
            ProductionResultVerifier::with_policy(policy)
                .with_result(observation);

        let evidence = verifier
            .verify_observation(&request());

        assert_eq!(
            evidence.state(),
            VerificationState::Passed
        );
    }

    #[test]
    fn insufficient_cardinality_is_rejected() {
        let artifact =
            ResultArtifactId::new("cardinality-low")
                .expect("artifact ID must be valid");

        let observation =
            ResultObservation::new(
                artifact,
                ResultKind::Samples,
                ResultLifecycle::Complete,
            )
            .with_cardinality(
                ResultCardinality::NonZero(1),
            );

        let policy =
            ResultVerificationPolicy::production()
                .with_minimum_cardinality(Some(2));

        let verifier =
            ProductionResultVerifier::with_policy(policy)
                .with_result(observation);

        let evidence = verifier
            .verify_observation(&request());

        assert_eq!(
            evidence.state(),
            VerificationState::Failed
        );

        assert!(
            evidence
                .violations()
                .iter()
                .any(|violation| {
                    violation.kind()
                        == VerificationViolationKind::ResultInvalid
                })
        );
    }

    #[test]
    fn result_kind_is_provider_independent() {
        assert_eq!(
            ResultKind::Samples.as_str(),
            "samples"
        );

        assert_eq!(
            ResultKind::Expectation.as_str(),
            "expectation"
        );

        assert_eq!(
            ResultKind::State.as_str(),
            "state"
        );

        assert_eq!(
            ResultKind::Process.as_str(),
            "process"
        );

        assert_eq!(
            ResultKind::Classical.as_str(),
            "classical"
        );
    }

    #[test]
    fn result_lifecycle_distinguishes_terminal_success_from_failure() {
        assert!(
            ResultLifecycle::Complete
                .is_successful_terminal()
        );

        assert!(
            ResultLifecycle::Failed
                .is_terminal_failure()
        );

        assert!(
            ResultLifecycle::InProgress
                .is_pending()
        );
    }

    #[test]
    fn canonical_qubit_identity_types_are_used() {
        let logical = QubitId::new(7);
        let physical = PhysicalQubitId::new(11);

        let scope =
            ResultResourceScope::new(
                [logical],
                [physical],
            );

        assert_eq!(
            scope.logical_qubits(),
            &[logical]
        );

        assert_eq!(
            scope.physical_qubits(),
            &[physical]
        );
    }

    #[test]
    fn result_digest_is_opaque() {
        let digest =
            ResultDigest::new("sha256:example")
                .expect("digest must be valid");

        assert_eq!(
            digest.as_str(),
            "sha256:example"
        );
    }

    #[test]
    fn context_creates_verifier_without_shared_mutable_state() {
        let observation = completed_samples();

        let context =
            ResultVerifierContext::new(
                observation,
            );

        let verifier =
            context.verifier();

        assert!(
            verifier.observation().is_some()
        );
    }

    #[test]
    fn standalone_validation_uses_same_contract() {
        let observation = completed_samples();

        let evidence =
            validate_result(
                &request(),
                &observation,
            );

        assert_eq!(
            evidence.state(),
            VerificationState::Passed
        );
    }

    #[test]
    fn production_policy_has_no_hardware_limit() {
        let policy =
            ResultVerificationPolicy::production();

        assert_eq!(
            policy.minimum_cardinality(),
            None
        );

        assert!(
            policy.rejects_duplicate_resources()
        );
    }

    #[test]
    fn verification_confidence_is_not_floating_point_in_acceptance_path() {
        let confidence =
            VerificationConfidence::certain();

        assert_eq!(
            confidence.basis_points(),
            VerificationConfidence::MAX
        );
    }

    #[test]
    fn trait_contract_returns_resilience_result() {
        fn accepts_result_verifier<T>()
        where
            T: ResultVerifier,
        {
        }

        accepts_result_verifier::<
            ProductionResultVerifier,
        >();

        let result: ResilienceResult<
            VerificationEvidence,
        > = Ok(
            VerificationEvidence::passed(
                VerificationConfidence::certain(),
            ),
        );

        assert!(result.is_ok());
    }
}