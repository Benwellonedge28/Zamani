//! Zamani Quantum Resilience — Execution Failure Detector.
//!
//! Path:
//!     src/quantum/resilience/detection/execution_failure.rs
//!
//! # Responsibility
//!
//! This module detects execution failures reported as resilience observations.
//!
//! It answers only:
//!
//! > "Does this observation contain an execution failure signal that should
//! > be passed to diagnosis?"
//!
//! It does NOT:
//!
//! - retry execution;
//! - cancel execution;
//! - restart a job;
//! - migrate a workload;
//! - select a backend;
//! - change routing;
//! - change scheduling;
//! - recompile a program;
//! - perform QEC;
//! - perform error mitigation;
//! - determine root cause;
//! - authorize recovery;
//! - declare a result correct.
//!
//! Those responsibilities belong to the appropriate resilience, hardware,
//! runtime, routing, scheduling, QEC, mitigation, diagnosis, planning,
//! recovery, policy, and verification modules.
//!
//! # Architectural position
//!
//! ```text
//! quantum::hardware::execution
//!            |
//!            v
//! runtime / hardware observation
//!            |
//!            v
//! DetectionObservation
//!            |
//!            v
//! ExecutionFailureDetector
//!            |
//!            v
//! DetectionSignal
//!            |
//!            v
//! diagnosis
//!            |
//!            v
//! policy
//!            |
//!            v
//! planning
//!            |
//!            v
//! recovery
//! ```
//!
//! # Write once, scale everywhere
//!
//! This detector introduces no machine-size assumptions.
//!
//! It does not assume:
//!
//! - a particular number of qubits;
//! - a particular number of physical qubits;
//! - a particular number of logical qubits;
//! - a particular backend;
//! - a particular provider;
//! - a particular number of execution attempts;
//! - a particular number of observations;
//! - a particular number of jobs;
//! - a particular topology;
//! - a particular QEC code.
//!
//! Detection operates over an iterator and therefore does not require the
//! entire execution history to be materialized in memory.
//!
//! "Infinity" means that this module introduces no artificial finite machine
//! size. A concrete execution remains bounded only by the resources and
//! policies supplied by the surrounding system.
//!
//! # Provider neutrality
//!
//! Provider-specific execution layers must translate their native failures
//! into the canonical resilience observation boundary.
//!
//! This detector deliberately does not contain branches such as:
//!
//! ```text
//! if IBM ...
//! if AWS ...
//! if IonQ ...
//! if Rigetti ...
//! ```
//!
//! Provider-specific adapters belong under `quantum::hardware::adapters`.
//!
//! # Observation semantics
//!
//! The current canonical detector contract provides a provider-neutral
//! `ObservationPayload`. Execution failures are normally represented as
//! `ObservationPayload::Text` when the upstream execution integration has not
//! introduced a richer structured execution-failure payload.
//!
//! The detector therefore recognizes stable semantic tokens rather than
//! provider-specific response formats.
//!
//! Recognized categories include:
//!
//! - timeout;
//! - cancellation;
//! - interruption;
//! - backend unavailability;
//! - resource unavailability;
//! - rejected execution;
//! - communication/transport failure;
//! - generic execution failure.
//!
//! The textual observation is treated strictly as evidence. It is never
//! interpreted as an instruction or command.
//!
//! # Important semantic distinction
//!
//! A timeout is not necessarily the same thing as remote execution failure.
//!
//! The hardware execution orchestrator explicitly distinguishes a local wait
//! timeout from proof that a remote provider stopped executing the job.
//!
//! Therefore this detector classifies an observation according to what the
//! observation actually reports. It does not infer remote cancellation from a
//! local timeout unless the upstream observation explicitly says so.
//!
//! # Determinism
//!
//! The detector:
//!
//! - does not read the system clock;
//! - does not generate random numbers;
//! - does not read environment variables;
//! - does not inspect process identifiers;
//! - does not use memory addresses;
//! - does not access global mutable state;
//! - does not perform hidden I/O;
//! - does not depend on provider-specific global configuration.
//!
//! Signal IDs are deterministically derived from explicit observation identity,
//! detection sequence, detector identity, and classification.
//!
//! # Streaming
//!
//! The detector consumes the observation iterator once.
//!
//! Memory usage is O(number of emitted signals), not O(number of observations
//! examined), except for the caller-owned output vector required by the
//! canonical `DetectionOutput` contract.
//!
//! The detector does not retain observations after processing them.
//!
//! # Trust and freshness
//!
//! The detector honors `DetectionContext`:
//!
//! - if verified observations are required, unverified observations are
//!   ignored;
//! - if stale observations are not allowed, stale/expired observations are
//!   ignored.
//!
//! Ignoring an observation is not equivalent to declaring the system healthy.
//! It means that the observation was not admissible evidence for this detector
//! evaluation.
//!
//! # Error handling
//!
//! The detector uses the canonical resilience error taxonomy.
//!
//! It does not define a competing error type.
//!
//! Structural invalidity is reported as:
//!
//! `ResilienceErrorCode::InvalidDetectionInput`
//!
//! Internal arithmetic exhaustion is reported as:
//!
//! `ResilienceErrorCode::ArithmeticOverflow`
//!
//! The detector does not manufacture backend-specific error codes.
//!
//! # Canonical quantum identity
//!
//! This module does not need to identify quantum resources itself.
//!
//! When an execution observation carries a qubit identity, the upstream
//! producer MUST use the canonical:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! This module must never introduce an alternative `QubitId`,
//! `PhysicalQubitId`, or integer convention.
//!
//! # Integration contract
//!
//! This file depends only on:
//!
//! ```text
//! crate::quantum::resilience::detection::detector
//! crate::quantum::resilience::errors
//! Rust standard library
//! ```
//!
//! It does not depend directly on:
//!
//! ```text
//! hardware adapters
//! routing
//! scheduling
//! optimization
//! QEC
//! diagnosis
//! planning
//! recovery
//! mitigation
//! verification
//! checkpointing
//! learning
//! distributed coordination
//! ```
//!
//! Those integrations occur through the detector contract.
//!
//! Therefore adding another backend, execution provider, QEC system, routing
//! algorithm, or recovery strategy does not require changing this file.
//!
//! # Rust compatibility
//!
//! Supported:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! =============================================================================
//! Implementation
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use core::fmt;

use crate::quantum::resilience::detection::detector::{
    DetectionClassification,
    DetectionConfidence,
    DetectionContext,
    DetectionInput,
    DetectionMetadata,
    DetectionObservation,
    DetectionOutput,
    DetectionPayload,
    DetectionSignal,
    Detector,
    DetectorIdentity,
    ObservationFreshness,
    ObservationPayload,
    ObservationTrust,
    SignalId,
};

use crate::quantum::resilience::errors::{
    ResilienceError,
    ResilienceErrorCode,
    ResilienceResult,
};

// ============================================================================
// Schema
// ============================================================================

/// Stable schema identifier for this detector.
pub const EXECUTION_FAILURE_DETECTOR_SCHEMA_ID: &str =
    "zamani.quantum.resilience.detection.execution_failure";

/// Semantic version of the detector contract.
pub const EXECUTION_FAILURE_DETECTOR_SCHEMA_VERSION: u16 = 1;

/// Stable implementation name.
pub const EXECUTION_FAILURE_DETECTOR_NAME: &str =
    "execution-failure-detector";

/// Stable implementation version.
///
/// This is implementation identity, not a provider/backend version.
pub const EXECUTION_FAILURE_DETECTOR_VERSION: &str = "1";

// ============================================================================
// Failure category
// ============================================================================

/// Normalized execution-failure category.
///
/// This is deliberately smaller than the complete backend error taxonomy.
/// The detector identifies observable execution conditions; diagnosis may
/// later determine the precise cause.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ExecutionFailureKind {
    /// A deadline or execution timeout was explicitly observed.
    Timeout,

    /// Execution was explicitly cancelled.
    Cancellation,

    /// Execution was explicitly interrupted.
    Interruption,

    /// The target backend/device was unavailable.
    BackendUnavailable,

    /// A required execution resource was unavailable.
    ResourceUnavailable,

    /// The backend explicitly rejected execution.
    Rejected,

    /// Communication/transport failure prevented execution lifecycle progress.
    CommunicationFailure,

    /// A generic execution failure was explicitly reported.
    ExecutionFailure,
}

impl ExecutionFailureKind {
    /// Returns the stable machine-readable representation.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Timeout => "timeout",
            Self::Cancellation => "cancellation",
            Self::Interruption => "interruption",
            Self::BackendUnavailable => "backend_unavailable",
            Self::ResourceUnavailable => "resource_unavailable",
            Self::Rejected => "rejected",
            Self::CommunicationFailure => "communication_failure",
            Self::ExecutionFailure => "execution_failure",
        }
    }

    /// Converts the normalized execution category to the canonical detector
    /// classification.
    #[must_use]
    pub const fn classification(self) -> DetectionClassification {
        match self {
            Self::Timeout => DetectionClassification::Timeout,

            Self::Cancellation
            | Self::Interruption
            | Self::ExecutionFailure
            | Self::Rejected
            | Self::CommunicationFailure => {
                DetectionClassification::ExecutionFailure
            }

            Self::BackendUnavailable
            | Self::ResourceUnavailable => {
                DetectionClassification::Unavailability
            }
        }
    }

    /// Returns the deterministic base confidence for an explicit category.
    ///
    /// Confidence here means confidence that the observation contains the
    /// indicated execution-failure category. It does not mean:
    ///
    /// - probability of physical failure;
    /// - logical error probability;
    /// - severity;
    /// - recovery success probability.
    #[must_use]
    pub const fn confidence(self) -> f64 {
        match self {
            Self::Timeout => 0.99,
            Self::Cancellation => 0.99,
            Self::Interruption => 0.99,
            Self::BackendUnavailable => 0.98,
            Self::ResourceUnavailable => 0.97,
            Self::Rejected => 0.98,
            Self::CommunicationFailure => 0.97,
            Self::ExecutionFailure => 0.90,
        }
    }
}

impl fmt::Display for ExecutionFailureKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// Configuration
// ============================================================================

/// Controls how execution-failure observations are interpreted.
///
/// The configuration is intentionally provider-neutral.
///
/// It does not contain:
///
/// - backend IDs;
/// - provider names;
/// - qubit counts;
/// - fixed retry counts;
/// - fixed machine sizes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionFailureDetectorConfig {
    /// Whether cancellation observations should produce signals.
    detect_cancellation: bool,

    /// Whether interruption observations should produce signals.
    detect_interruption: bool,

    /// Whether timeout observations should produce signals.
    detect_timeout: bool,

    /// Whether backend-unavailable observations should produce signals.
    detect_backend_unavailable: bool,

    /// Whether resource-unavailable observations should produce signals.
    detect_resource_unavailable: bool,

    /// Whether explicit rejection observations should produce signals.
    detect_rejection: bool,

    /// Whether communication failures should produce signals.
    detect_communication_failure: bool,

    /// Whether generic execution failures should produce signals.
    detect_generic_failure: bool,

    /// Whether observations with unknown textual content should be treated as
    /// inconclusive evidence.
    emit_inconclusive_for_unknown_text: bool,
}

impl Default for ExecutionFailureDetectorConfig {
    fn default() -> Self {
        Self {
            detect_cancellation: true,
            detect_interruption: true,
            detect_timeout: true,
            detect_backend_unavailable: true,
            detect_resource_unavailable: true,
            detect_rejection: true,
            detect_communication_failure: true,
            detect_generic_failure: true,
            emit_inconclusive_for_unknown_text: false,
        }
    }
}

impl ExecutionFailureDetectorConfig {
    /// Creates the default production configuration.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            detect_cancellation: true,
            detect_interruption: true,
            detect_timeout: true,
            detect_backend_unavailable: true,
            detect_resource_unavailable: true,
            detect_rejection: true,
            detect_communication_failure: true,
            detect_generic_failure: true,
            emit_inconclusive_for_unknown_text: false,
        }
    }

    /// Enables or disables cancellation detection.
    #[must_use]
    pub const fn with_cancellation_detection(mut self, enabled: bool) -> Self {
        self.detect_cancellation = enabled;
        self
    }

    /// Enables or disables interruption detection.
    #[must_use]
    pub const fn with_interruption_detection(mut self, enabled: bool) -> Self {
        self.detect_interruption = enabled;
        self
    }

    /// Enables or disables timeout detection.
    #[must_use]
    pub const fn with_timeout_detection(mut self, enabled: bool) -> Self {
        self.detect_timeout = enabled;
        self
    }

    /// Enables or disables backend-unavailable detection.
    #[must_use]
    pub const fn with_backend_unavailable_detection(
        mut self,
        enabled: bool,
    ) -> Self {
        self.detect_backend_unavailable = enabled;
        self
    }

    /// Enables or disables resource-unavailable detection.
    #[must_use]
    pub const fn with_resource_unavailable_detection(
        mut self,
        enabled: bool,
    ) -> Self {
        self.detect_resource_unavailable = enabled;
        self
    }

    /// Enables or disables rejection detection.
    #[must_use]
    pub const fn with_rejection_detection(mut self, enabled: bool) -> Self {
        self.detect_rejection = enabled;
        self
    }

    /// Enables or disables communication-failure detection.
    #[must_use]
    pub const fn with_communication_failure_detection(
        mut self,
        enabled: bool,
    ) -> Self {
        self.detect_communication_failure = enabled;
        self
    }

    /// Enables or disables generic execution-failure detection.
    #[must_use]
    pub const fn with_generic_failure_detection(
        mut self,
        enabled: bool,
    ) -> Self {
        self.detect_generic_failure = enabled;
        self
    }

    /// Enables or disables inconclusive signals for unrecognized textual
    /// observations.
    #[must_use]
    pub const fn with_inconclusive_unknown_text(
        mut self,
        enabled: bool,
    ) -> Self {
        self.emit_inconclusive_for_unknown_text = enabled;
        self
    }

    /// Returns whether cancellation detection is enabled.
    #[must_use]
    pub const fn detect_cancellation(&self) -> bool {
        self.detect_cancellation
    }

    /// Returns whether interruption detection is enabled.
    #[must_use]
    pub const fn detect_interruption(&self) -> bool {
        self.detect_interruption
    }

    /// Returns whether timeout detection is enabled.
    #[must_use]
    pub const fn detect_timeout(&self) -> bool {
        self.detect_timeout
    }

    /// Returns whether backend-unavailable detection is enabled.
    #[must_use]
    pub const fn detect_backend_unavailable(&self) -> bool {
        self.detect_backend_unavailable
    }

    /// Returns whether resource-unavailable detection is enabled.
    #[must_use]
    pub const fn detect_resource_unavailable(&self) -> bool {
        self.detect_resource_unavailable
    }

    /// Returns whether rejection detection is enabled.
    #[must_use]
    pub const fn detect_rejection(&self) -> bool {
        self.detect_rejection
    }

    /// Returns whether communication-failure detection is enabled.
    #[must_use]
    pub const fn detect_communication_failure(&self) -> bool {
        self.detect_communication_failure
    }

    /// Returns whether generic execution-failure detection is enabled.
    #[must_use]
    pub const fn detect_generic_failure(&self) -> bool {
        self.detect_generic_failure
    }

    /// Returns whether unknown text should produce an inconclusive signal.
    #[must_use]
    pub const fn emit_inconclusive_for_unknown_text(&self) -> bool {
        self.emit_inconclusive_for_unknown_text
    }
}

// ============================================================================
// Detector
// ============================================================================

/// Detects execution failures from canonical resilience observations.
///
/// The detector is state-free. This is intentional:
///
/// - execution failures are already explicit observations;
/// - there is no need for a historical baseline;
/// - history belongs to diagnosis/history;
/// - retry policy belongs to policy/recovery;
/// - correlation belongs to diagnosis/correlation.
///
/// Statelessness also permits callers to create independent detector
/// instances for parallel execution domains without shared mutable state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionFailureDetector {
    identity: DetectorIdentity,
    config: ExecutionFailureDetectorConfig,
}

impl ExecutionFailureDetector {
    /// Creates a production execution-failure detector.
    pub fn new() -> ResilienceResult<Self> {
        Self::with_config(ExecutionFailureDetectorConfig::new())
    }

    /// Creates a detector from explicit configuration.
    pub fn with_config(
        config: ExecutionFailureDetectorConfig,
    ) -> ResilienceResult<Self> {
        let identity = DetectorIdentity::new(
            EXECUTION_FAILURE_DETECTOR_NAME,
            EXECUTION_FAILURE_DETECTOR_VERSION,
        )?;

        Ok(Self { identity, config })
    }

    /// Returns the detector configuration.
    #[must_use]
    pub const fn config(&self) -> &ExecutionFailureDetectorConfig {
        &self.config
    }

    /// Returns the detector schema identifier.
    #[must_use]
    pub const fn schema_id() -> &'static str {
        EXECUTION_FAILURE_DETECTOR_SCHEMA_ID
    }

    /// Returns the detector schema version.
    #[must_use]
    pub const fn schema_version() -> u16 {
        EXECUTION_FAILURE_DETECTOR_SCHEMA_VERSION
    }

    /// Classifies one observation.
    ///
    /// Only textual execution-failure evidence is interpreted here because
    /// that is the structured payload currently exposed by the canonical
    /// detector boundary for provider-neutral execution diagnostics.
    ///
    /// Boolean, integer, unsigned, floating-point, and marker observations
    /// are not guessed to be execution failures.
    fn classify_observation(
        &self,
        observation: &DetectionObservation,
    ) -> Option<ExecutionFailureKind> {
        let payload = observation.payload();

        let text = match payload {
            ObservationPayload::Text(value) => value.as_str(),
            _ => return None,
        };

        let normalized = normalize_text(text);

        self.classify_text(&normalized)
    }

    /// Classifies normalized textual evidence.
    ///
    /// Precedence is intentional:
    ///
    /// 1. cancellation;
    /// 2. interruption;
    /// 3. timeout;
    /// 4. backend unavailability;
    /// 5. resource unavailability;
    /// 6. rejection;
    /// 7. communication failure;
    /// 8. generic execution failure.
    ///
    /// More specific conditions therefore win over the generic execution
    /// failure category.
    fn classify_text(&self, normalized: &str) -> Option<ExecutionFailureKind> {
        if self.config.detect_cancellation
            && contains_any(
                normalized,
                &[
                    "cancelled",
                    "canceled",
                    "cancellation",
                    "execution cancelled",
                    "execution canceled",
                    "job cancelled",
                    "job canceled",
                ],
            )
        {
            return Some(ExecutionFailureKind::Cancellation);
        }

        if self.config.detect_interruption
            && contains_any(
                normalized,
                &[
                    "interrupted",
                    "interruption",
                    "execution interrupted",
                    "job interrupted",
                ],
            )
        {
            return Some(ExecutionFailureKind::Interruption);
        }

        if self.config.detect_timeout
            && contains_any(
                normalized,
                &[
                    "timeout",
                    "timed out",
                    "deadline exceeded",
                    "execution deadline exceeded",
                    "backend timeout",
                    "job timeout",
                ],
            )
        {
            return Some(ExecutionFailureKind::Timeout);
        }

        if self.config.detect_backend_unavailable
            && contains_any(
                normalized,
                &[
                    "backend unavailable",
                    "backend unavailable",
                    "backend offline",
                    "device unavailable",
                    "device offline",
                    "qpu unavailable",
                    "qpu offline",
                    "service unavailable",
                    "execution unavailable",
                ],
            )
        {
            return Some(ExecutionFailureKind::BackendUnavailable);
        }

        if self.config.detect_resource_unavailable
            && contains_any(
                normalized,
                &[
                    "resource unavailable",
                    "resource exhausted",
                    "no execution slot",
                    "no capacity",
                    "capacity unavailable",
                    "qubit unavailable",
                    "physical qubit unavailable",
                    "control channel unavailable",
                ],
            )
        {
            return Some(ExecutionFailureKind::ResourceUnavailable);
        }

        if self.config.detect_rejection
            && contains_any(
                normalized,
                &[
                    "execution rejected",
                    "job rejected",
                    "request rejected",
                    "backend rejected",
                    "submission rejected",
                    "operation rejected",
                ],
            )
        {
            return Some(ExecutionFailureKind::Rejected);
        }

        if self.config.detect_communication_failure
            && contains_any(
                normalized,
                &[
                    "communication failure",
                    "communication failed",
                    "connection failed",
                    "connection failure",
                    "transport failure",
                    "transport failed",
                    "network failure",
                    "network error",
                    "provider communication failed",
                    "backend communication failed",
                ],
            )
        {
            return Some(ExecutionFailureKind::CommunicationFailure);
        }

        if self.config.detect_generic_failure
            && contains_any(
                normalized,
                &[
                    "execution failure",
                    "execution failed",
                    "execution error",
                    "job failed",
                    "job failure",
                    "job error",
                    "quantum execution failed",
                ],
            )
        {
            return Some(ExecutionFailureKind::ExecutionFailure);
        }

        None
    }

    /// Builds a deterministic signal ID.
    ///
    /// The detector cannot allocate globally coordinated IDs because that
    /// would introduce hidden mutable state or an external identity service.
    ///
    /// Instead, this method derives a stable 64-bit identity from explicit
    /// detector/observation/sequence/classification data.
    ///
    /// The canonical `SignalId` contract treats the identifier as an identity,
    /// not as a cryptographic digest. Collision resistance is therefore not
    /// used as a security boundary.
    fn signal_id(
        &self,
        observation: &DetectionObservation,
        sequence: crate::quantum::resilience::detection::detector::DetectionSequence,
        kind: ExecutionFailureKind,
    ) -> ResilienceResult<SignalId> {
        let mut hash = FNV_OFFSET_BASIS;

        hash = fnv_bytes(hash, self.identity.name().as_bytes());
        hash = fnv_bytes(hash, self.identity.version().as_bytes());
        hash = fnv_bytes(hash, &observation.id().value().to_le_bytes());
        hash = fnv_bytes(hash, &sequence.value().to_le_bytes());
        hash = fnv_bytes(hash, kind.as_str().as_bytes());

        // FNV-1a can theoretically produce zero only for a zero input state,
        // but the initial state is non-zero and the update operation is
        // multiplicative, so a zero result is not expected. We nevertheless
        // explicitly handle it to preserve the NonZero invariant without
        // relying on an unchecked conversion.
        let value = if hash == 0 { 1 } else { hash };

        SignalId::from_u64(value).ok_or_else(|| {
            ResilienceError::new(ResilienceErrorCode::RepresentationOverflow)
        })
    }

    /// Creates a normalized signal for an observation.
    fn make_signal(
        &self,
        observation: &DetectionObservation,
        context: &DetectionContext,
        kind: ExecutionFailureKind,
    ) -> ResilienceResult<DetectionSignal> {
        let confidence = DetectionConfidence::new(kind.confidence())?;

        let id = self.signal_id(observation, context.sequence(), kind)?;

        Ok(DetectionSignal::new(
            id,
            self.identity.clone(),
            kind.classification(),
            confidence,
            Some(observation.id()),
            context.sequence(),
        ))
    }

    /// Validates whether an observation is admissible under the supplied
    /// detection context.
    fn is_admissible(
        &self,
        context: &DetectionContext,
        observation: &DetectionObservation,
    ) -> bool {
        if context.require_verified_observations()
            && !observation.trust().is_verified()
        {
            return false;
        }

        if !context.allow_stale_observations()
            && observation.freshness().is_stale()
        {
            return false;
        }

        true
    }

    /// Produces an inconclusive signal for an unknown textual observation
    /// when explicitly configured to do so.
    fn make_inconclusive_signal(
        &self,
        observation: &DetectionObservation,
        context: &DetectionContext,
    ) -> ResilienceResult<DetectionSignal> {
        let id = self.inconclusive_signal_id(observation, context);

        Ok(DetectionSignal::new(
            id,
            self.identity.clone(),
            DetectionClassification::Inconclusive,
            DetectionConfidence::zero(),
            Some(observation.id()),
            context.sequence(),
        ))
    }

    /// Builds a deterministic ID for an inconclusive observation.
    fn inconclusive_signal_id(
        &self,
        observation: &DetectionObservation,
        context: &DetectionContext,
    ) -> SignalId {
        let mut hash = FNV_OFFSET_BASIS;

        hash = fnv_bytes(hash, self.identity.name().as_bytes());
        hash = fnv_bytes(hash, self.identity.version().as_bytes());
        hash = fnv_bytes(hash, &observation.id().value().to_le_bytes());
        hash = fnv_bytes(hash, &context.sequence().value().to_le_bytes());
        hash = fnv_bytes(hash, b"inconclusive");

        SignalId::from_u64(if hash == 0 { 1 } else { hash })
            .expect("non-zero fallback guarantees SignalId construction")
    }
}

impl Detector for ExecutionFailureDetector {
    fn identity(&self) -> &DetectorIdentity {
        &self.identity
    }

    fn detect<'a, I>(
        &mut self,
        input: DetectionInput<'a, I>,
    ) -> ResilienceResult<DetectionOutput>
    where
        I: Iterator<Item = &'a DetectionObservation>,
    {
        let context = input.context();

        if context.sequence().value() == 0 {
            return Err(ResilienceError::new(
                ResilienceErrorCode::InvalidDetectionInput,
            ));
        }

        let mut observations_examined = 0_u64;
        let mut signals = Vec::new();

        for observation in input.observations() {
            observations_examined =
                observations_examined.checked_add(1).ok_or_else(|| {
                    ResilienceError::new(
                        ResilienceErrorCode::ArithmeticOverflow,
                    )
                })?;

            if !self.is_admissible(context, observation) {
                continue;
            }

            let kind = match self.classify_observation(observation) {
                Some(kind) => kind,

                None => {
                    if self.config.emit_inconclusive_for_unknown_text {
                        signals.push(self.make_inconclusive_signal(
                            observation,
                            context,
                        )?);
                    }

                    continue;
                }
            };

            signals.push(self.make_signal(observation, context, kind)?);
        }

        let metadata = DetectionMetadata::new(
            self.identity.clone(),
            context.sequence(),
            observations_examined,
        );

        Ok(DetectionOutput::new(metadata, signals))
    }

    fn is_available(&self) -> bool {
        true
    }

    fn reset(&mut self) {
        // The detector is intentionally stateless.
        //
        // Keeping this explicit prevents a future implementation from
        // accidentally introducing hidden history into the detector lifecycle.
    }
}

// ============================================================================
// Text normalization
// ============================================================================

/// Normalizes textual execution evidence for deterministic matching.
///
/// This function:
///
/// - trims leading/trailing whitespace;
/// - converts ASCII uppercase letters to lowercase;
/// - preserves non-ASCII UTF-8 bytes;
/// - does not perform locale-sensitive transformations.
///
/// It deliberately avoids allocation-heavy Unicode normalization because the
/// detector's semantic vocabulary is ASCII-defined and execution providers
/// should normalize their own structured error representations upstream.
fn normalize_text(value: &str) -> String {
    value
        .trim()
        .bytes()
        .map(|byte| {
            if byte.is_ascii_uppercase() {
                byte.to_ascii_lowercase()
            } else {
                byte
            }
        })
        .map(char::from)
        .collect()
}

/// Returns whether any semantic token occurs in the normalized evidence.
///
/// This helper performs literal semantic matching only. It does not execute
/// text, parse commands, interpret URLs, or perform dynamic dispatch.
fn contains_any(value: &str, tokens: &[&str]) -> bool {
    tokens.iter().any(|token| value.contains(token))
}

// ============================================================================
// Deterministic signal hashing
// ============================================================================

/// FNV-1a 64-bit offset basis.
///
/// This is used only to derive deterministic signal identities. It is not a
/// cryptographic hash and must never be used for authentication, integrity,
/// authorization, or security decisions.
const FNV_OFFSET_BASIS: u64 = 0xcbf29ce484222325;

/// FNV-1a 64-bit prime.
const FNV_PRIME: u64 = 0x00000100000001b3;

/// Deterministically incorporates bytes into a signal identity.
fn fnv_bytes(mut hash: u64, bytes: &[u8]) -> u64 {
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(FNV_PRIME);
    }

    hash
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use core::num::NonZeroU64;

    fn observation(
        id: u64,
        sequence: u64,
        payload: ObservationPayload,
    ) -> DetectionObservation {
        DetectionObservation::new(
            ObservationId::from_u64(id).expect("test ID must be non-zero"),
            DetectionSequence::from_u64(sequence)
                .expect("test sequence must be non-zero"),
            ObservationSource::Runtime,
            ObservationTrust::Verified,
            ObservationFreshness::Fresh,
            payload,
        )
        .expect("test observation must be valid")
    }

    fn context(sequence: u64) -> DetectionContext {
        DetectionContext::new(
            DetectionSequence::from_u64(sequence)
                .expect("test sequence must be non-zero"),
            false,
            true,
        )
    }

    #[test]
    fn detects_timeout() {
        let mut detector =
            ExecutionFailureDetector::new().expect("detector construction");

        let item = observation(
            1,
            1,
            ObservationPayload::Text("execution timed out".to_owned()),
        );

        let ctx = context(1);

        let output = detector
            .detect(DetectionInput::new(&ctx, std::iter::once(&item)))
            .expect("detection must succeed");

        assert_eq!(output.len(), 1);
        assert_eq!(
            output.signals()[0].classification(),
            DetectionClassification::Timeout
        );
    }

    #[test]
    fn detects_backend_unavailability() {
        let mut detector =
            ExecutionFailureDetector::new().expect("detector construction");

        let item = observation(
            1,
            1,
            ObservationPayload::Text("backend unavailable".to_owned()),
        );

        let ctx = context(1);

        let output = detector
            .detect(DetectionInput::new(&ctx, std::iter::once(&item)))
            .expect("detection must succeed");

        assert_eq!(output.len(), 1);
        assert_eq!(
            output.signals()[0].classification(),
            DetectionClassification::Unavailability
        );
    }

    #[test]
    fn detects_cancellation() {
        let mut detector =
            ExecutionFailureDetector::new().expect("detector construction");

        let item = observation(
            1,
            1,
            ObservationPayload::Text("job CANCELLED".to_owned()),
        );

        let ctx = context(1);

        let output = detector
            .detect(DetectionInput::new(&ctx, std::iter::once(&item)))
            .expect("detection must succeed");

        assert_eq!(output.len(), 1);
        assert_eq!(
            output.signals()[0].classification(),
            DetectionClassification::ExecutionFailure
        );
    }

    #[test]
    fn detects_communication_failure() {
        let mut detector =
            ExecutionFailureDetector::new().expect("detector construction");

        let item = observation(
            1,
            1,
            ObservationPayload::Text("transport failure".to_owned()),
        );

        let ctx = context(1);

        let output = detector
            .detect(DetectionInput::new(&ctx, std::iter::once(&item)))
            .expect("detection must succeed");

        assert_eq!(output.len(), 1);
        assert_eq!(
            output.signals()[0].classification(),
            DetectionClassification::ExecutionFailure
        );
    }

    #[test]
    fn ignores_non_text_payloads_without_guessing() {
        let mut detector =
            ExecutionFailureDetector::new().expect("detector construction");

        let item = observation(1, 1, ObservationPayload::Boolean(true));

        let ctx = context(1);

        let output = detector
            .detect(DetectionInput::new(&ctx, std::iter::once(&item)))
            .expect("detection must succeed");

        assert!(output.is_empty());
    }

    #[test]
    fn ignores_unverified_observation_when_verification_required() {
        let mut detector =
            ExecutionFailureDetector::new().expect("detector construction");

        let item = DetectionObservation::new(
            ObservationId::from_u64(1).expect("valid ID"),
            DetectionSequence::from_u64(1).expect("valid sequence"),
            ObservationSource::Runtime,
            ObservationTrust::Unverified,
            ObservationFreshness::Fresh,
            ObservationPayload::Text("execution failed".to_owned()),
        )
        .expect("valid observation");

        let ctx = DetectionContext::new(
            DetectionSequence::from_u64(1).expect("valid sequence"),
            false,
            true,
        );

        let output = detector
            .detect(DetectionInput::new(&ctx, std::iter::once(&item)))
            .expect("detection must succeed");

        assert!(output.is_empty());
    }

    #[test]
    fn allows_unverified_observation_when_policy_allows_it() {
        let mut detector =
            ExecutionFailureDetector::new().expect("detector construction");

        let item = DetectionObservation::new(
            ObservationId::from_u64(1).expect("valid ID"),
            DetectionSequence::from_u64(1).expect("valid sequence"),
            ObservationSource::Runtime,
            ObservationTrust::Unverified,
            ObservationFreshness::Fresh,
            ObservationPayload::Text("execution failed".to_owned()),
        )
        .expect("valid observation");

        let ctx = DetectionContext::new(
            DetectionSequence::from_u64(1).expect("valid sequence"),
            false,
            false,
        );

        let output = detector
            .detect(DetectionInput::new(&ctx, std::iter::once(&item)))
            .expect("detection must succeed");

        assert_eq!(output.len(), 1);
    }

    #[test]
    fn ignores_stale_observation_by_default() {
        let mut detector =
            ExecutionFailureDetector::new().expect("detector construction");

        let item = DetectionObservation::new(
            ObservationId::from_u64(1).expect("valid ID"),
            DetectionSequence::from_u64(1).expect("valid sequence"),
            ObservationSource::Hardware,
            ObservationTrust::Verified,
            ObservationFreshness::Stale,
            ObservationPayload::Text("execution failed".to_owned()),
        )
        .expect("valid observation");

        let ctx = DetectionContext::new(
            DetectionSequence::from_u64(1).expect("valid sequence"),
            false,
            true,
        );

        let output = detector
            .detect(DetectionInput::new(&ctx, std::iter::once(&item)))
            .expect("detection must succeed");

        assert!(output.is_empty());
    }

    #[test]
    fn accepts_stale_observation_when_explicitly_allowed() {
        let mut detector =
            ExecutionFailureDetector::new().expect("detector construction");

        let item = DetectionObservation::new(
            ObservationId::from_u64(1).expect("valid ID"),
            DetectionSequence::from_u64(1).expect("valid sequence"),
            ObservationSource::Hardware,
            ObservationTrust::Verified,
            ObservationFreshness::Stale,
            ObservationPayload::Text("execution failed".to_owned()),
        )
        .expect("valid observation");

        let ctx = DetectionContext::new(
            DetectionSequence::from_u64(1).expect("valid sequence"),
            true,
            true,
        );

        let output = detector
            .detect(DetectionInput::new(&ctx, std::iter::once(&item)))
            .expect("detection must succeed");

        assert_eq!(output.len(), 1);
    }

    #[test]
    fn signal_id_is_deterministic() {
        let detector =
            ExecutionFailureDetector::new().expect("detector construction");

        let item = observation(
            42,
            7,
            ObservationPayload::Text("execution failed".to_owned()),
        );

        let ctx = context(7);

        let first = detector
            .signal_id(
                &item,
                &ctx.sequence(),
                ExecutionFailureKind::ExecutionFailure,
            )
            .expect("signal ID");

        let second = detector
            .signal_id(
                &item,
                &ctx.sequence(),
                ExecutionFailureKind::ExecutionFailure,
            )
            .expect("signal ID");

        assert_eq!(first, second);
    }

    #[test]
    fn different_observations_produce_different_expected_inputs() {
        let detector =
            ExecutionFailureDetector::new().expect("detector construction");

        let first = observation(
            1,
            1,
            ObservationPayload::Text("execution failed".to_owned()),
        );

        let second = observation(
            2,
            1,
            ObservationPayload::Text("execution failed".to_owned()),
        );

        let ctx = context(1);

        let first_id = detector
            .signal_id(
                &first,
                &ctx.sequence(),
                ExecutionFailureKind::ExecutionFailure,
            )
            .expect("signal ID");

        let second_id = detector
            .signal_id(
                &second,
                &ctx.sequence(),
                ExecutionFailureKind::ExecutionFailure,
            )
            .expect("signal ID");

        assert_ne!(first_id, second_id);
    }

    #[test]
    fn precedence_prefers_timeout_over_generic_failure() {
        let detector =
            ExecutionFailureDetector::new().expect("detector construction");

        let normalized = "execution failed: deadline exceeded";

        assert_eq!(
            detector.classify_text(normalized),
            Some(ExecutionFailureKind::Timeout)
        );
    }

    #[test]
    fn precedence_prefers_backend_unavailability() {
        let detector =
            ExecutionFailureDetector::new().expect("detector construction");

        let normalized = "execution failed: backend unavailable";

        assert_eq!(
            detector.classify_text(normalized),
            Some(ExecutionFailureKind::BackendUnavailable)
        );
    }

    #[test]
    fn configurable_categories_can_be_disabled() {
        let config = ExecutionFailureDetectorConfig::new()
            .with_timeout_detection(false);

        let detector =
            ExecutionFailureDetector::with_config(config)
                .expect("detector construction");

        assert_eq!(
            detector.classify_text("execution timed out"),
            None
        );
    }

    #[test]
    fn unknown_text_is_not_a_failure_by_default() {
        let detector =
            ExecutionFailureDetector::new().expect("detector construction");

        assert_eq!(
            detector.classify_text("provider returned diagnostic information"),
            None
        );
    }

    #[test]
    fn inconclusive_unknown_text_is_optional() {
        let config = ExecutionFailureDetectorConfig::new()
            .with_inconclusive_unknown_text(true);

        let mut detector =
            ExecutionFailureDetector::with_config(config)
                .expect("detector construction");

        let item = observation(
            1,
            1,
            ObservationPayload::Text(
                "provider returned diagnostic information".to_owned(),
            ),
        );

        let ctx = context(1);

        let output = detector
            .detect(DetectionInput::new(&ctx, std::iter::once(&item)))
            .expect("detection must succeed");

        assert_eq!(output.len(), 1);
        assert_eq!(
            output.signals()[0].classification(),
            DetectionClassification::Inconclusive
        );
        assert_eq!(
            output.signals()[0].confidence(),
            DetectionConfidence::zero()
        );
    }

    #[test]
    fn detector_is_stateless() {
        let mut detector =
            ExecutionFailureDetector::new().expect("detector construction");

        let item = observation(
            1,
            1,
            ObservationPayload::Text("execution failed".to_owned()),
        );

        let ctx = context(1);

        let first = detector
            .detect(DetectionInput::new(&ctx, std::iter::once(&item)))
            .expect("first detection");

        detector.reset();

        let second = detector
            .detect(DetectionInput::new(&ctx, std::iter::once(&item)))
            .expect("second detection");

        assert_eq!(first, second);
    }

    #[test]
    fn large_stream_is_processed_without_history_state() {
        let mut detector =
            ExecutionFailureDetector::new().expect("detector construction");

        let ctx = context(1);

        let observations = (1_u64..=10_000_u64).map(|id| {
            observation(
                id,
                1,
                ObservationPayload::Text(
                    "execution failed".to_owned(),
                ),
            )
        });

        let output = detector
            .detect(DetectionInput::new(&ctx, observations.iter()))
            .expect("large stream detection");

        assert_eq!(output.metadata().observations_examined(), 10_000);
        assert_eq!(output.len(), 10_000);
    }

    #[test]
    fn observation_ids_must_be_non_zero() {
        assert!(ObservationId::from_u64(0).is_none());
        assert!(ObservationId::from_u64(1).is_some());
    }

    #[test]
    fn canonical_non_zero_constructor_is_supported() {
        let value =
            NonZeroU64::new(7).expect("literal seven is non-zero");

        let id = SignalId::new(value);

        assert_eq!(id.value(), 7);
    }
}