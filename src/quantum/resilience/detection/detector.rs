//! Zamani Quantum Resilience — Detection Contract.
//!
//! Path:
//!     src/quantum/resilience/detection/detector.rs
//!
//! # Purpose
//!
//! This module defines the provider-neutral contract between resilience
//! observations and the rest of the Zamani resilience subsystem.
//!
//! Detection answers:
//!
//! > "Has an observable condition occurred that may require resilience
//! > interpretation?"
//!
//! Detection does NOT answer:
//!
//! - what the root cause is;
//! - how severe the incident ultimately is;
//! - which recovery action should be performed;
//! - which backend should be selected;
//! - how routing should change;
//! - how scheduling should change;
//! - how QEC should be configured;
//! - whether a result is semantically correct.
//!
//! Those responsibilities belong to:
//!
//! ```text
//! diagnosis/
//! model/
//! policy/
//! planning/
//! adaptation/
//! recovery/
//! verification/
//! ```
//!
//! # Architectural position
//!
//! ```text
//! hardware / runtime / QEC / ZQN / telemetry
//!                    │
//!                    ▼
//!             DetectionInput
//!                    │
//!                    ▼
//!              Detector(s)
//!                    │
//!                    ▼
//!             DetectionOutput
//!                    │
//!             ┌──────┴──────┐
//!             │             │
//!             ▼             ▼
//!       normalized       diagnostics
//!        signals          / evidence
//!             │
//!             ▼
//!          diagnosis
//!             │
//!             ▼
//!           policy
//!             │
//!             ▼
//!          planning
//! ```
//!
//! The detector contract is intentionally below diagnosis.
//!
//! # Write once, scale everywhere
//!
//! This module imposes no architectural limit on:
//!
//! - qubits;
//! - logical qubits;
//! - physical qubits;
//! - operations;
//! - circuits;
//! - detector count;
//! - observation count;
//! - resource count;
//! - backend count;
//! - device count;
//! - distributed execution domains;
//! - execution attempts;
//! - machine size.
//!
//! There are no constants such as:
//!
//! ```text
//! MAX_QUBITS
//! MAX_DETECTORS
//! MAX_OBSERVATIONS
//! MAX_SIGNALS
//! DEFAULT_RETRY_COUNT
//! ```
//!
//! Actual resource limits belong to explicit runtime, execution, security,
//! memory, and resilience policies.
//!
//! "Infinity" means that this semantic contract does not introduce an
//! artificial finite machine-size ceiling. A concrete invocation remains
//! bounded by resources actually available to it.
//!
//! # Canonical quantum identity
//!
//! This file does not define `QubitId` or `PhysicalQubitId`.
//!
//! When a detector needs to identify quantum resources, implementations MUST
//! use the canonical types:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! This contract deliberately does not duplicate those identities.
//!
//! A detector may operate on non-qubit resources as well, because resilience
//! applies to:
//!
//! - logical qubits;
//! - physical qubits;
//! - gates;
//! - couplings;
//! - execution stages;
//! - control resources;
//! - devices;
//! - backends;
//! - classical execution resources;
//! - distributed execution resources.
//!
//! Resource identity remains owned by the appropriate canonical subsystem.
//!
//! # Fault ownership
//!
//! The canonical quantum fault ontology remains owned by ZQN.
//!
//! A detector may observe or reference a canonical fault, but MUST NOT create a
//! competing fault taxonomy.
//!
//! The relationship is:
//!
//! ```text
//! ZQN
//!   │
//!   │ canonical physical/noise/fault semantics
//!   ▼
//! detector
//!   │
//!   │ resilience observation
//!   ▼
//! detection signal
//!   │
//!   ▼
//! diagnosis
//! ```
//!
//! # Detection versus diagnosis
//!
//! Detection and diagnosis must remain separate.
//!
//! Detection may say:
//!
//! ```text
//! "anomaly observed"
//! ```
//!
//! Diagnosis may later say:
//!
//! ```text
//! "likely calibration drift affecting a coupling"
//! ```
//!
//! The detector MUST NOT silently turn an observation into a causal claim.
//!
//! # Determinism
//!
//! The detector contract supports deterministic operation.
//!
//! A detector implementation MUST NOT silently obtain:
//!
//! - current time;
//! - random numbers;
//! - environment variables;
//! - process identifiers;
//! - memory addresses;
//! - mutable global state;
//! - hidden configuration.
//!
//! If temporal or stochastic information is required, it MUST be supplied as
//! part of `DetectionInput` or another explicit dependency.
//!
//! A detector may therefore be deterministic when its input, configuration,
//! state, and dependencies are deterministic.
//!
//! # Streaming
//!
//! Detection must work for both:
//!
//! ```text
//! one observation
//! ```
//!
//! and:
//!
//! ```text
//! arbitrarily large streams of observations
//! ```
//!
//! The contract therefore does not require materializing an entire execution
//! history in memory.
//!
//! Stateful detectors should keep only the state required by their detection
//! algorithm and must expose that state through explicit detector state rather
//! than hidden global storage.
//!
//! # Batch processing
//!
//! A batch is represented as an iterator rather than a fixed-size collection.
//!
//! This permits callers to provide:
//!
//! - slices;
//! - vectors;
//! - streaming iterators;
//! - database-backed iterators;
//! - telemetry streams;
//! - distributed streams.
//!
//! The detector contract therefore does not impose a collection representation
//! or maximum batch size.
//!
//! # Concurrency
//!
//! The trait does not require `Send` or `Sync` on detector implementations.
//!
//! This is deliberate.
//!
//! A caller may require:
//!
//! - single-threaded deterministic execution;
//! - independent detector instances per worker;
//! - shared immutable detector configuration;
//! - externally synchronized state;
//! - parallel detector execution.
//!
//! Implementations should add `Send`/`Sync` bounds only where required by their
//! actual execution environment.
//!
//! # Security
//!
//! Detection is a data-processing boundary, not an authorization boundary.
//!
//! A detection signal MUST NOT itself grant:
//!
//! - QPU access;
//! - credentials;
//! - recovery authority;
//! - migration authority;
//! - backend access;
//! - filesystem access;
//! - network access.
//!
//! Observation sources may be untrusted.
//!
//! Therefore implementations should preserve source identity, trust metadata,
//! provenance, and integrity information when those are supplied by the
//! surrounding telemetry/security subsystem.
//!
//! Detection MUST NOT accept an observation merely because it requests a
//! particular recovery action.
//!
//! # Error ownership
//!
//! Fallible detector operations use the canonical resilience error contract:
//!
//! ```text
//! crate::quantum::resilience::errors
//! ```
//!
//! Detector implementations should use the existing detection error codes:
//!
//! ```text
//! DetectionFailed
//! InvalidDetectionInput
//! DetectionInconsistent
//! DetectionDataUnavailable
//! DetectionDataStale
//! DetectionInconclusive
//! ```
//!
//! This file does not redefine those errors.
//!
//! # Integration contract
//!
//! This file is intentionally designed to be implemented before:
//!
//! ```text
//! detection/threshold.rs
//! detection/anomaly.rs
//! detection/statistical.rs
//! detection/drift.rs
//! detection/timeout.rs
//! detection/execution_failure.rs
//! detection/qec_signal.rs
//! detection/hardware_signal.rs
//! ```
//!
//! Those modules should depend on this contract.
//!
//! This contract must not depend on any of those concrete detector
//! implementations.
//!
//! The dependency direction is:
//!
//! ```text
//! errors ──────────────┐
//!                      │
//! model/fault ─────────┤
//!                      ▼
//!                detector.rs
//!                      │
//!          ┌───────────┼────────────┐
//!          ▼           ▼            ▼
//!      threshold    anomaly     statistical
//!          │           │            │
//!          └───────────┼────────────┘
//!                      ▼
//!                  diagnosis
//! ```
//!
//! # Public API stability
//!
//! This module intentionally owns the stable detector contract.
//!
//! Concrete detector algorithms must not require changes to this file merely
//! because a new algorithm is introduced.
//!
//! New detector implementations should normally implement `Detector` and,
//! where useful, reuse:
//!
//! - `DetectionInput`;
//! - `DetectionObservation`;
//! - `DetectionSignal`;
//! - `DetectionOutput`;
//! - `DetectionMetadata`;
//! - `DetectorIdentity`.
//!
//! # No unsafe Rust
//!
//! This module explicitly forbids unsafe Rust.
//!
//! Compatible with:
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
use core::num::NonZeroU64;

use crate::quantum::resilience::errors::ResilienceResult;

// ============================================================================
// Stable schema identifiers
// ============================================================================

/// Stable schema identifier for the detector contract.
pub const DETECTOR_SCHEMA_ID: &str = "zamani.quantum.resilience.detection.detector";

/// Semantic version of the detector contract.
pub const DETECTOR_SCHEMA_VERSION: u16 = 1;

// ============================================================================
// Detector identity
// ============================================================================

/// Stable identity of a detector implementation.
///
/// A detector identity is descriptive metadata. It is not an authorization
/// capability and does not identify a hardware provider unless the
/// implementation explicitly chooses to describe itself that way.
///
/// The identity must be supplied by the implementation rather than generated
/// from memory addresses, process IDs, timestamps, or random values.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DetectorIdentity {
    name: String,
    version: String,
}

impl DetectorIdentity {
    /// Creates a detector identity.
    ///
    /// Empty names or versions are rejected because they cannot provide a
    /// stable machine-readable identity.
    pub fn new(
        name: impl Into<String>,
        version: impl Into<String>,
    ) -> ResilienceResult<Self> {
        let name = name.into();
        let version = version.into();

        if name.trim().is_empty() || version.trim().is_empty() {
            return Err(crate::quantum::resilience::errors::ResilienceError::new(
                crate::quantum::resilience::errors::ResilienceErrorCode::InvalidArgument,
            ));
        }

        Ok(Self { name, version })
    }

    /// Returns the detector name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the detector implementation version.
    #[must_use]
    pub fn version(&self) -> &str {
        &self.version
    }
}

impl fmt::Display for DetectorIdentity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}@{}", self.name, self.version)
    }
}

// ============================================================================
// Observation identity
// ============================================================================

/// Stable identifier for an observation.
///
/// Observation identifiers are caller-owned. The detector does not generate
/// them implicitly.
///
/// A caller may derive an ID from a deterministic execution/provenance system,
/// a telemetry sequence, or another stable identity mechanism.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ObservationId(NonZeroU64);

impl ObservationId {
    /// Creates an observation ID.
    ///
    /// Zero is rejected because it is reserved as the absence/default value
    /// and therefore cannot be a valid observation identity.
    pub const fn new(value: NonZeroU64) -> Self {
        Self(value)
    }

    /// Creates an observation ID from a raw non-zero value.
    pub const fn from_u64(value: u64) -> Option<Self> {
        match NonZeroU64::new(value) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }

    /// Returns the underlying value.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0.get()
    }
}

impl fmt::Display for ObservationId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.value())
    }
}

// ============================================================================
// Detection sequence identity
// ============================================================================

/// Identifies one logically ordered detection evaluation.
///
/// The value is supplied by the execution/provenance layer.
///
/// It MUST NOT be interpreted as a retry count, machine size, or detector
/// limit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DetectionSequence(NonZeroU64);

impl DetectionSequence {
    /// Creates a sequence identity.
    pub const fn new(value: NonZeroU64) -> Self {
        Self(value)
    }

    /// Creates a sequence identity from a raw non-zero value.
    pub const fn from_u64(value: u64) -> Option<Self> {
        match NonZeroU64::new(value) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }

    /// Returns the sequence value.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0.get()
    }
}

// ============================================================================
// Observation source
// ============================================================================

/// Describes where an observation originated.
///
/// This is deliberately generic and provider-neutral.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ObservationSource {
    /// Observation came from the execution runtime.
    Runtime,

    /// Observation came from a hardware abstraction boundary.
    Hardware,

    /// Observation came from QEC processing.
    Qec,

    /// Observation came from ZQN/noise semantics.
    Zqn,

    /// Observation came from benchmarking/characterization.
    Benchmarking,

    /// Observation came from simulation/emulation.
    Simulation,

    /// Observation came from another resilience component.
    Resilience,

    /// Observation came from an externally supplied integration source.
    External(String),
}

impl ObservationSource {
    /// Returns a stable machine-readable representation.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Runtime => "runtime",
            Self::Hardware => "hardware",
            Self::Qec => "qec",
            Self::Zqn => "zqn",
            Self::Benchmarking => "benchmarking",
            Self::Simulation => "simulation",
            Self::Resilience => "resilience",
            Self::External(value) => value.as_str(),
        }
    }
}

// ============================================================================
// Observation trust
// ============================================================================

/// Trust classification supplied by the observation producer/security layer.
///
/// Trust does not authorize recovery. It only describes how the observation
/// should be treated by downstream policy and diagnosis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ObservationTrust {
    /// Source trust is unknown.
    Unknown,

    /// Source is known but the observation has not been integrity-verified.
    Unverified,

    /// Observation integrity has been verified.
    Verified,

    /// Observation source has been explicitly trusted by the surrounding
    /// security boundary.
    Trusted,
}

impl ObservationTrust {
    /// Returns whether the observation has been integrity verified.
    #[must_use]
    pub const fn is_verified(self) -> bool {
        matches!(self, Self::Verified | Self::Trusted)
    }

    /// Returns whether the observation is explicitly trusted.
    #[must_use]
    pub const fn is_trusted(self) -> bool {
        matches!(self, Self::Trusted)
    }
}

// ============================================================================
// Observation freshness
// ============================================================================

/// Describes freshness supplied by the producer.
///
/// Detection does not read the system clock. Freshness must therefore be
/// established by an upstream component or expressed as explicit execution
/// metadata.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ObservationFreshness {
    /// Freshness is unknown.
    Unknown,

    /// Observation is known to be fresh for the consuming operation.
    Fresh,

    /// Observation may still be useful but is older than the preferred
    /// freshness window.
    Stale,

    /// Observation is explicitly expired and should normally not influence
    /// detection.
    Expired,
}

impl ObservationFreshness {
    /// Returns whether the observation is usable without an explicit
    /// stale-data override.
    #[must_use]
    pub const fn is_current(self) -> bool {
        matches!(self, Self::Fresh)
    }

    /// Returns whether the observation is stale or expired.
    #[must_use]
    pub const fn is_stale(self) -> bool {
        matches!(self, Self::Stale | Self::Expired)
    }
}

// ============================================================================
// Observation payload
// ============================================================================

/// Provider-neutral observation payload.
///
/// Detection deliberately does not prescribe a single numerical metric type.
/// Quantum systems produce heterogeneous observations:
///
/// - fault records;
/// - health state;
/// - timing information;
/// - calibration information;
/// - execution failures;
/// - syndrome outcomes;
/// - measurement statistics;
/// - backend status;
/// - resource availability;
/// - anomaly scores.
///
/// Concrete detector modules should wrap their domain-specific values in an
/// implementation-specific type and feed them through the detector contract.
///
/// This enum provides only the minimal semantic categories needed by the
/// detection boundary.
#[derive(Debug, Clone, PartialEq)]
pub enum ObservationPayload {
    /// A boolean condition was observed.
    Boolean(bool),

    /// A signed integer observation.
    Integer(i128),

    /// An unsigned integer observation.
    Unsigned(u128),

    /// A finite floating-point observation.
    Float(f64),

    /// An opaque, provider-neutral textual observation.
    ///
    /// Text is descriptive evidence only. It MUST NOT be interpreted as a
    /// capability or command by the detector contract.
    Text(String),

    /// An empty observation carrying only metadata.
    Marker,
}

impl ObservationPayload {
    /// Returns whether this payload contains a finite floating-point value.
    #[must_use]
    pub fn is_finite(&self) -> bool {
        match self {
            Self::Float(value) => value.is_finite(),
            _ => true,
        }
    }

    /// Returns a stable category name.
    #[must_use]
    pub const fn kind(&self) -> &'static str {
        match self {
            Self::Boolean(_) => "boolean",
            Self::Integer(_) => "integer",
            Self::Unsigned(_) => "unsigned",
            Self::Float(_) => "float",
            Self::Text(_) => "text",
            Self::Marker => "marker",
        }
    }
}

// ============================================================================
// Detection observation
// ============================================================================

/// One immutable observation presented to a detector.
#[derive(Debug, Clone, PartialEq)]
pub struct DetectionObservation {
    id: ObservationId,
    sequence: DetectionSequence,
    source: ObservationSource,
    trust: ObservationTrust,
    freshness: ObservationFreshness,
    payload: ObservationPayload,
}

impl DetectionObservation {
    /// Creates an observation.
    ///
    /// Validation is intentionally limited to structural properties owned by
    /// this contract. Domain-specific validation remains with the producer and
    /// concrete detector.
    pub fn new(
        id: ObservationId,
        sequence: DetectionSequence,
        source: ObservationSource,
        trust: ObservationTrust,
        freshness: ObservationFreshness,
        payload: ObservationPayload,
    ) -> ResilienceResult<Self> {
        if !payload.is_finite() {
            return Err(crate::quantum::resilience::errors::ResilienceError::new(
                crate::quantum::resilience::errors::ResilienceErrorCode::InvalidDetectionInput,
            ));
        }

        Ok(Self {
            id,
            sequence,
            source,
            trust,
            freshness,
            payload,
        })
    }

    /// Returns the observation identity.
    #[must_use]
    pub const fn id(&self) -> ObservationId {
        self.id
    }

    /// Returns the detection sequence.
    #[must_use]
    pub const fn sequence(&self) -> DetectionSequence {
        self.sequence
    }

    /// Returns the observation source.
    #[must_use]
    pub const fn source(&self) -> &ObservationSource {
        &self.source
    }

    /// Returns the observation trust classification.
    #[must_use]
    pub const fn trust(&self) -> ObservationTrust {
        self.trust
    }

    /// Returns the observation freshness.
    #[must_use]
    pub const fn freshness(&self) -> ObservationFreshness {
        self.freshness
    }

    /// Returns the observation payload.
    #[must_use]
    pub const fn payload(&self) -> &ObservationPayload {
        &self.payload
    }
}

// ============================================================================
// Detection context
// ============================================================================

/// Explicit context supplied to a detector evaluation.
///
/// The context contains no hidden time source, random source, or global state.
///
/// Concrete detectors may use the context to ensure that the same detector
/// configuration produces deterministic output for the same execution state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DetectionContext {
    sequence: DetectionSequence,
    allow_stale_observations: bool,
    require_verified_observations: bool,
}

impl DetectionContext {
    /// Creates a detection context.
    ///
    /// The caller explicitly controls stale-data and trust behavior.
    #[must_use]
    pub const fn new(
        sequence: DetectionSequence,
        allow_stale_observations: bool,
        require_verified_observations: bool,
    ) -> Self {
        Self {
            sequence,
            allow_stale_observations,
            require_verified_observations,
        }
    }

    /// Returns the evaluation sequence.
    #[must_use]
    pub const fn sequence(&self) -> DetectionSequence {
        self.sequence
    }

    /// Returns whether stale observations may be considered.
    #[must_use]
    pub const fn allow_stale_observations(&self) -> bool {
        self.allow_stale_observations
    }

    /// Returns whether observations must be integrity verified.
    #[must_use]
    pub const fn require_verified_observations(&self) -> bool {
        self.require_verified_observations
    }
}

// ============================================================================
// Detection input
// ============================================================================

/// Input to one detector evaluation.
///
/// The observation iterator is borrowed and therefore does not require the
/// detector contract to allocate or materialize an entire stream.
///
/// The detector may consume the iterator exactly once.
pub struct DetectionInput<'a, I>
where
    I: Iterator<Item = &'a DetectionObservation>,
{
    context: &'a DetectionContext,
    observations: I,
}

impl<'a, I> DetectionInput<'a, I>
where
    I: Iterator<Item = &'a DetectionObservation>,
{
    /// Creates detector input from an explicit context and observation stream.
    #[must_use]
    pub fn new(context: &'a DetectionContext, observations: I) -> Self {
        Self {
            context,
            observations,
        }
    }

    /// Returns the explicit detection context.
    #[must_use]
    pub const fn context(&self) -> &'a DetectionContext {
        self.context
    }

    /// Returns the underlying observation iterator.
    ///
    /// This consumes the input wrapper, allowing detectors to process the
    /// stream without forcing a collection.
    #[must_use]
    pub fn observations(self) -> I {
        self.observations
    }
}

// ============================================================================
// Detection classification
// ============================================================================

/// Provider-neutral classification of a detector result.
///
/// This is deliberately not a root-cause taxonomy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum DetectionClassification {
    /// No actionable condition was detected.
    NoCondition,

    /// An observation indicates a potentially abnormal condition.
    Anomaly,

    /// A known fault signal was detected.
    Fault,

    /// A resource may be degraded.
    Degradation,

    /// A resource may have become unavailable.
    Unavailability,

    /// A timeout or deadline condition was observed.
    Timeout,

    /// Execution itself reported a failure.
    ExecutionFailure,

    /// QEC produced a resilience-relevant signal.
    QecSignal,

    /// Hardware produced a resilience-relevant signal.
    HardwareSignal,

    /// Detection could not establish a sufficiently strong conclusion.
    Inconclusive,
}

impl DetectionClassification {
    /// Returns a stable machine-readable name.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoCondition => "no_condition",
            Self::Anomaly => "anomaly",
            Self::Fault => "fault",
            Self::Degradation => "degradation",
            Self::Unavailability => "unavailability",
            Self::Timeout => "timeout",
            Self::ExecutionFailure => "execution_failure",
            Self::QecSignal => "qec_signal",
            Self::HardwareSignal => "hardware_signal",
            Self::Inconclusive => "inconclusive",
        }
    }

    /// Returns whether this classification indicates a possible condition
    /// requiring downstream interpretation.
    #[must_use]
    pub const fn is_actionable_candidate(self) -> bool {
        !matches!(self, Self::NoCondition)
    }
}

// ============================================================================
// Detection confidence
// ============================================================================

/// Detector confidence represented as a normalized finite value.
///
/// The value is constrained to the closed interval `[0, 1]`.
///
/// This is confidence in the detector's conclusion, not:
///
/// - probability of a physical fault;
/// - fidelity;
/// - logical error rate;
/// - severity;
/// - recovery success probability.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct DetectionConfidence(f64);

impl DetectionConfidence {
    /// Creates confidence after validating that it is finite and in `[0, 1]`.
    pub fn new(value: f64) -> ResilienceResult<Self> {
        if !value.is_finite() || !(0.0..=1.0).contains(&value) {
            return Err(crate::quantum::resilience::errors::ResilienceError::new(
                crate::quantum::resilience::errors::ResilienceErrorCode::InvalidArgument,
            ));
        }

        Ok(Self(value))
    }

    /// Returns the confidence value.
    #[must_use]
    pub const fn value(self) -> f64 {
        self.0
    }

    /// Returns zero confidence.
    #[must_use]
    pub const fn zero() -> Self {
        Self(0.0)
    }

    /// Returns full confidence.
    #[must_use]
    pub const fn full() -> Self {
        Self(1.0)
    }
}

// ============================================================================
// Detection signal identity
// ============================================================================

/// Stable identity of a normalized detection signal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SignalId(NonZeroU64);

impl SignalId {
    /// Creates a signal ID from a non-zero value.
    pub const fn new(value: NonZeroU64) -> Self {
        Self(value)
    }

    /// Creates a signal ID from a raw value.
    pub const fn from_u64(value: u64) -> Option<Self> {
        match NonZeroU64::new(value) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }

    /// Returns the underlying value.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0.get()
    }
}

// ============================================================================
// Detection signal
// ============================================================================

/// A normalized resilience detection signal.
///
/// A signal is an observation-derived fact or candidate condition. It is NOT
/// a diagnosis and NOT a recovery command.
#[derive(Debug, Clone, PartialEq)]
pub struct DetectionSignal {
    id: SignalId,
    detector: DetectorIdentity,
    classification: DetectionClassification,
    confidence: DetectionConfidence,
    observation_id: Option<ObservationId>,
    sequence: DetectionSequence,
}

impl DetectionSignal {
    /// Creates a normalized detection signal.
    pub fn new(
        id: SignalId,
        detector: DetectorIdentity,
        classification: DetectionClassification,
        confidence: DetectionConfidence,
        observation_id: Option<ObservationId>,
        sequence: DetectionSequence,
    ) -> Self {
        Self {
            id,
            detector,
            classification,
            confidence,
            observation_id,
            sequence,
        }
    }

    /// Returns the signal identity.
    #[must_use]
    pub const fn id(&self) -> SignalId {
        self.id
    }

    /// Returns the detector identity.
    #[must_use]
    pub const fn detector(&self) -> &DetectorIdentity {
        &self.detector
    }

    /// Returns the classification.
    #[must_use]
    pub const fn classification(&self) -> DetectionClassification {
        self.classification
    }

    /// Returns detector confidence.
    #[must_use]
    pub const fn confidence(&self) -> DetectionConfidence {
        self.confidence
    }

    /// Returns the originating observation ID, if one exists.
    #[must_use]
    pub const fn observation_id(&self) -> Option<ObservationId> {
        self.observation_id
    }

    /// Returns the detection sequence.
    #[must_use]
    pub const fn sequence(&self) -> DetectionSequence {
        self.sequence
    }

    /// Returns whether this signal represents a no-condition result.
    #[must_use]
    pub const fn is_no_condition(&self) -> bool {
        matches!(
            self.classification,
            DetectionClassification::NoCondition
        )
    }

    /// Returns whether this signal is an actionable candidate.
    #[must_use]
    pub const fn is_actionable_candidate(&self) -> bool {
        self.classification.is_actionable_candidate()
    }
}

// ============================================================================
// Detection metadata
// ============================================================================

/// Metadata describing one detector evaluation.
///
/// Metadata is intentionally small and immutable so it can be retained in
/// telemetry/provenance without retaining detector implementation state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DetectionMetadata {
    detector: DetectorIdentity,
    sequence: DetectionSequence,
    observations_examined: u64,
}

impl DetectionMetadata {
    /// Creates detection metadata.
    #[must_use]
    pub const fn new(
        detector: DetectorIdentity,
        sequence: DetectionSequence,
        observations_examined: u64,
    ) -> Self {
        Self {
            detector,
            sequence,
            observations_examined,
        }
    }

    /// Returns detector identity.
    #[must_use]
    pub const fn detector(&self) -> &DetectorIdentity {
        &self.detector
    }

    /// Returns evaluation sequence.
    #[must_use]
    pub const fn sequence(&self) -> DetectionSequence {
        self.sequence
    }

    /// Returns how many observations were examined.
    #[must_use]
    pub const fn observations_examined(&self) -> u64 {
        self.observations_examined
    }
}

// ============================================================================
// Detection output
// ============================================================================

/// Result of one detector evaluation.
///
/// The output contains normalized signals and evaluation metadata.
///
/// Concrete detectors may emit zero, one, or many signals. No fixed output
/// cardinality is imposed.
#[derive(Debug, Clone, PartialEq)]
pub struct DetectionOutput {
    metadata: DetectionMetadata,
    signals: Vec<DetectionSignal>,
}

impl DetectionOutput {
    /// Creates an output from an explicit signal collection.
    ///
    /// The caller owns the collection size and therefore controls resource
    /// consumption according to its execution policy.
    #[must_use]
    pub fn new(metadata: DetectionMetadata, signals: Vec<DetectionSignal>) -> Self {
        Self { metadata, signals }
    }

    /// Returns evaluation metadata.
    #[must_use]
    pub const fn metadata(&self) -> &DetectionMetadata {
        &self.metadata
    }

    /// Returns all normalized signals.
    #[must_use]
    pub fn signals(&self) -> &[DetectionSignal] {
        &self.signals
    }

    /// Returns whether no signals were produced.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.signals.is_empty()
    }

    /// Returns the number of normalized signals.
    #[must_use]
    pub fn len(&self) -> usize {
        self.signals.len()
    }

    /// Consumes the output and returns its signals.
    #[must_use]
    pub fn into_signals(self) -> Vec<DetectionSignal> {
        self.signals
    }
}

// ============================================================================
// Detector trait
// ============================================================================

/// Core provider-neutral detector contract.
///
/// A detector consumes explicit observations and produces normalized detection
/// output.
///
/// Implementations should be small and domain-specific:
///
/// ```text
/// ThresholdDetector
/// StatisticalDetector
/// DriftDetector
/// TimeoutDetector
/// ExecutionFailureDetector
/// QecSignalDetector
/// HardwareSignalDetector
/// ```
///
/// The detector MUST NOT:
///
/// - execute recovery;
/// - mutate hardware;
/// - select a backend;
/// - change routing;
/// - change scheduling;
/// - recompile programs;
/// - authorize actions;
/// - perform hidden I/O;
/// - access hidden global state.
///
/// Those operations belong to downstream orchestration.
pub trait Detector {
    /// Returns the stable detector identity.
    fn identity(&self) -> &DetectorIdentity;

    /// Evaluates observations and returns normalized detection output.
    ///
    /// The input iterator is single-pass. Implementations that require
    /// multiple passes must explicitly buffer or summarize the observations
    /// according to their own resource policy.
    fn detect<'a, I>(
        &mut self,
        input: DetectionInput<'a, I>,
    ) -> ResilienceResult<DetectionOutput>
    where
        I: Iterator<Item = &'a DetectionObservation>;

    /// Returns whether this detector is currently able to evaluate input.
    ///
    /// This is a capability query only. Returning `false` does not itself
    /// indicate a fault in the monitored quantum system.
    fn is_available(&self) -> bool {
        true
    }

    /// Resets detector-local state.
    ///
    /// The default implementation is stateless.
    ///
    /// Stateful detectors should override this method and clear only their
    /// detector-local state. They must not modify global resilience state.
    fn reset(&mut self) {}
}

// ============================================================================
// Detector reference
// ============================================================================

/// Object-safe detector collection boundary.
///
/// This trait is useful for registries and orchestration code that needs to
/// store heterogeneous detector implementations without prescribing their
/// concrete types.
pub trait DetectorObject {
    /// Returns the detector identity.
    fn identity(&self) -> &DetectorIdentity;

    /// Runs detection using a borrowed slice of observations.
    ///
    /// This adapter intentionally exists at the object boundary. It avoids
    /// exposing generic associated iterator types through registry APIs.
    fn detect_slice(
        &mut self,
        context: &DetectionContext,
        observations: &[DetectionObservation],
    ) -> ResilienceResult<DetectionOutput>;

    /// Returns whether the detector is available.
    fn is_available(&self) -> bool {
        true
    }

    /// Resets detector-local state.
    fn reset(&mut self) {}
}

/// Blanket object-safe adapter for ordinary [`Detector`] implementations.
impl<T> DetectorObject for T
where
    T: Detector,
{
    fn identity(&self) -> &DetectorIdentity {
        Detector::identity(self)
    }

    fn detect_slice(
        &mut self,
        context: &DetectionContext,
        observations: &[DetectionObservation],
    ) -> ResilienceResult<DetectionOutput> {
        let input = DetectionInput::new(context, observations.iter());
        Detector::detect(self, input)
    }

    fn is_available(&self) -> bool {
        Detector::is_available(self)
    }

    fn reset(&mut self) {
        Detector::reset(self);
    }
}

// ============================================================================
// Detector composition
// ============================================================================

/// Executes multiple heterogeneous detectors against the same observation
/// slice.
///
/// This helper intentionally does not decide how detector results should be
/// diagnosed, correlated, ranked, or recovered. It merely provides a
/// deterministic fan-out/fan-in boundary.
///
/// Detector ordering is the ordering supplied by the caller.
///
/// No detector count is hard-coded.
pub fn detect_with_all(
    detectors: &mut [Box<dyn DetectorObject>],
    context: &DetectionContext,
    observations: &[DetectionObservation],
) -> ResilienceResult<Vec<DetectionOutput>> {
    let mut outputs = Vec::new();

    for detector in detectors {
        if !detector.is_available() {
            continue;
        }

        outputs.push(detector.detect_slice(context, observations)?);
    }

    Ok(outputs)
}

// ============================================================================
// Validation helpers
// ============================================================================

/// Validates an observation stream's structural invariants.
///
/// This helper does not perform domain-specific validation and does not infer
/// a fault.
///
/// It checks:
///
/// - observation IDs are unique within the supplied stream;
/// - all observations belong to the requested sequence;
/// - floating-point observations are finite;
/// - required trust/freshness constraints are satisfied.
///
/// The implementation uses a caller-owned `BTreeSet`, avoiding a hidden global
/// registry and keeping memory proportional to the number of observations
/// actually validated.
pub fn validate_observations<'a, I>(
    context: &DetectionContext,
    observations: I,
) -> ResilienceResult<Vec<&'a DetectionObservation>>
where
    I: IntoIterator<Item = &'a DetectionObservation>,
{
    use std::collections::BTreeSet;

    let mut ids = BTreeSet::new();
    let mut validated = Vec::new();

    for observation in observations {
        if observation.sequence() != context.sequence() {
            return Err(crate::quantum::resilience::errors::ResilienceError::new(
                crate::quantum::resilience::errors::ResilienceErrorCode::DetectionInconsistent,
            ));
        }

        if !ids.insert(observation.id()) {
            return Err(crate::quantum::resilience::errors::ResilienceError::new(
                crate::quantum::resilience::errors::ResilienceErrorCode::DetectionInconsistent,
            ));
        }

        if !observation.payload().is_finite() {
            return Err(crate::quantum::resilience::errors::ResilienceError::new(
                crate::quantum::resilience::errors::ResilienceErrorCode::InvalidDetectionInput,
            ));
        }

        if context.require_verified_observations()
            && !observation.trust().is_verified()
        {
            return Err(crate::quantum::resilience::errors::ResilienceError::new(
                crate::quantum::resilience::errors::ResilienceErrorCode::UntrustedObservation,
            ));
        }

        if !context.allow_stale_observations()
            && observation.freshness().is_stale()
        {
            return Err(crate::quantum::resilience::errors::ResilienceError::new(
                crate::quantum::resilience::errors::ResilienceErrorCode::DetectionDataStale,
            ));
        }

        validated.push(observation);
    }

    Ok(validated)
}

// ============================================================================
// Deterministic signal ordering
// ============================================================================

/// Deterministically orders detection signals.
///
/// Ordering is based only on explicit semantic values:
///
/// 1. detection sequence;
/// 2. signal ID;
/// 3. detector identity.
///
/// No timestamp, pointer address, thread scheduling order, or hash iteration
/// order is used.
///
/// This helper is useful before serialization, provenance recording, or
/// deterministic diagnosis.
pub fn sort_signals_deterministically(signals: &mut [DetectionSignal]) {
    signals.sort_by(|left, right| {
        left.sequence()
            .cmp(&right.sequence())
            .then_with(|| left.id().cmp(&right.id()))
            .then_with(|| left.detector().cmp(right.detector()))
    });
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use core::num::NonZeroU64;

    fn observation_id(value: u64) -> ObservationId {
        ObservationId::new(
            NonZeroU64::new(value).expect("test observation ID must be non-zero"),
        )
    }

    fn sequence(value: u64) -> DetectionSequence {
        DetectionSequence::new(
            NonZeroU64::new(value).expect("test sequence must be non-zero"),
        )
    }

    fn signal_id(value: u64) -> SignalId {
        SignalId::new(
            NonZeroU64::new(value).expect("test signal ID must be non-zero"),
        )
    }

    fn detector_identity() -> DetectorIdentity {
        DetectorIdentity::new("test-detector", "1.0.0")
            .expect("test detector identity must be valid")
    }

    fn observation(
        id: u64,
        sequence_value: u64,
        payload: ObservationPayload,
    ) -> DetectionObservation {
        DetectionObservation::new(
            observation_id(id),
            sequence(sequence_value),
            ObservationSource::Runtime,
            ObservationTrust::Verified,
            ObservationFreshness::Fresh,
            payload,
        )
        .expect("test observation must be valid")
    }

    #[test]
    fn detector_identity_is_stable() {
        let identity =
            DetectorIdentity::new("statistical", "1.0.0").expect("identity should be valid");

        assert_eq!(identity.name(), "statistical");
        assert_eq!(identity.version(), "1.0.0");
        assert_eq!(identity.to_string(), "statistical@1.0.0");
    }

    #[test]
    fn zero_observation_id_is_rejected() {
        assert!(ObservationId::from_u64(0).is_none());
    }

    #[test]
    fn zero_sequence_is_rejected() {
        assert!(DetectionSequence::from_u64(0).is_none());
    }

    #[test]
    fn zero_signal_id_is_rejected() {
        assert!(SignalId::from_u64(0).is_none());
    }

    #[test]
    fn invalid_float_observation_is_rejected() {
        let result = DetectionObservation::new(
            observation_id(1),
            sequence(1),
            ObservationSource::Runtime,
            ObservationTrust::Verified,
            ObservationFreshness::Fresh,
            ObservationPayload::Float(f64::NAN),
        );

        assert!(result.is_err());
    }

    #[test]
    fn infinite_float_observation_is_rejected() {
        let result = DetectionObservation::new(
            observation_id(1),
            sequence(1),
            ObservationSource::Runtime,
            ObservationTrust::Verified,
            ObservationFreshness::Fresh,
            ObservationPayload::Float(f64::INFINITY),
        );

        assert!(result.is_err());
    }

    #[test]
    fn confidence_accepts_closed_interval() {
        assert!(DetectionConfidence::new(0.0).is_ok());
        assert!(DetectionConfidence::new(0.5).is_ok());
        assert!(DetectionConfidence::new(1.0).is_ok());
    }

    #[test]
    fn confidence_rejects_out_of_range_values() {
        assert!(DetectionConfidence::new(-0.1).is_err());
        assert!(DetectionConfidence::new(1.1).is_err());
        assert!(DetectionConfidence::new(f64::NAN).is_err());
        assert!(DetectionConfidence::new(f64::INFINITY).is_err());
    }

    #[test]
    fn context_controls_stale_observation_policy() {
        let context = DetectionContext::new(sequence(1), false, false);

        let stale = DetectionObservation::new(
            observation_id(1),
            sequence(1),
            ObservationSource::Runtime,
            ObservationTrust::Verified,
            ObservationFreshness::Stale,
            ObservationPayload::Marker,
        )
        .expect("observation should be structurally valid");

        assert!(validate_observations(&context, [stale].iter()).is_err());
    }

    #[test]
    fn context_can_allow_stale_observations() {
        let context = DetectionContext::new(sequence(1), true, false);

        let stale = DetectionObservation::new(
            observation_id(1),
            sequence(1),
            ObservationSource::Runtime,
            ObservationTrust::Verified,
            ObservationFreshness::Stale,
            ObservationPayload::Marker,
        )
        .expect("observation should be structurally valid");

        assert!(validate_observations(&context, [stale].iter()).is_ok());
    }

    #[test]
    fn context_can_require_verified_observations() {
        let context = DetectionContext::new(sequence(1), false, true);

        let unverified = DetectionObservation::new(
            observation_id(1),
            sequence(1),
            ObservationSource::Runtime,
            ObservationTrust::Unverified,
            ObservationFreshness::Fresh,
            ObservationPayload::Marker,
        )
        .expect("observation should be structurally valid");

        assert!(validate_observations(&context, [unverified].iter()).is_err());
    }

    #[test]
    fn duplicate_observation_ids_are_rejected() {
        let context = DetectionContext::new(sequence(1), false, false);

        let first = observation(1, 1, ObservationPayload::Marker);
        let second = observation(1, 1, ObservationPayload::Marker);

        assert!(
            validate_observations(&context, [first, second].iter()).is_err()
        );
    }

    #[test]
    fn mismatched_sequences_are_rejected() {
        let context = DetectionContext::new(sequence(1), false, false);

        let observation = observation(1, 2, ObservationPayload::Marker);

        assert!(validate_observations(&context, [observation].iter()).is_err());
    }

    #[test]
    fn matching_sequences_are_accepted() {
        let context = DetectionContext::new(sequence(1), false, false);

        let observation = observation(1, 1, ObservationPayload::Unsigned(42));

        assert!(validate_observations(&context, [observation].iter()).is_ok());
    }

    #[test]
    fn signal_classification_is_provider_neutral() {
        assert_eq!(
            DetectionClassification::Fault.as_str(),
            "fault"
        );

        assert!(DetectionClassification::Fault.is_actionable_candidate());
        assert!(!DetectionClassification::NoCondition.is_actionable_candidate());
    }

    #[test]
    fn signal_can_reference_observation() {
        let signal = DetectionSignal::new(
            signal_id(1),
            detector_identity(),
            DetectionClassification::Anomaly,
            DetectionConfidence::full(),
            Some(observation_id(7)),
            sequence(3),
        );

        assert_eq!(signal.id().value(), 1);
        assert_eq!(signal.observation_id().map(ObservationId::value), Some(7));
        assert_eq!(signal.sequence().value(), 3);
        assert!(signal.is_actionable_candidate());
    }

    #[test]
    fn signal_without_observation_is_supported() {
        let signal = DetectionSignal::new(
            signal_id(1),
            detector_identity(),
            DetectionClassification::HardwareSignal,
            DetectionConfidence::zero(),
            None,
            sequence(1),
        );

        assert_eq!(signal.observation_id(), None);
    }

    #[test]
    fn deterministic_signal_sorting_does_not_use_arrival_order() {
        let identity = detector_identity();

        let mut signals = vec![
            DetectionSignal::new(
                signal_id(3),
                identity.clone(),
                DetectionClassification::Anomaly,
                DetectionConfidence::full(),
                None,
                sequence(2),
            ),
            DetectionSignal::new(
                signal_id(1),
                identity.clone(),
                DetectionClassification::Fault,
                DetectionConfidence::full(),
                None,
                sequence(1),
            ),
            DetectionSignal::new(
                signal_id(2),
                identity,
                DetectionClassification::Degradation,
                DetectionConfidence::full(),
                None,
                sequence(1),
            ),
        ];

        sort_signals_deterministically(&mut signals);

        assert_eq!(signals[0].id().value(), 1);
        assert_eq!(signals[1].id().value(), 2);
        assert_eq!(signals[2].id().value(), 3);
    }

    #[test]
    fn output_can_contain_arbitrary_number_of_signals() {
        let identity = detector_identity();
        let mut signals = Vec::new();

        for value in 1..=128_u64 {
            signals.push(DetectionSignal::new(
                signal_id(value),
                identity.clone(),
                DetectionClassification::Anomaly,
                DetectionConfidence::full(),
                None,
                sequence(1),
            ));
        }

        let metadata = DetectionMetadata::new(identity, sequence(1), 128);
        let output = DetectionOutput::new(metadata, signals);

        assert_eq!(output.len(), 128);
        assert!(!output.is_empty());
    }
}