//! Zamani Quantum Resilience — Diagnosis Orchestrator Contract.
//!
//! Path:
//!     src/quantum/resilience/diagnosis/diagnostician.rs
//!
//! =============================================================================
//! Purpose
//! =============================================================================
//!
//! This module owns the stable, provider-neutral composition boundary between
//! detection and the rest of the resilience system.
//!
//! Diagnosis answers:
//!
//!     "What does the available evidence most strongly indicate?"
//!
//! Diagnosis does NOT:
//!
//! - execute recovery;
//! - authorize recovery;
//! - select a backend;
//! - change routing;
//! - change scheduling;
//! - recompile a program;
//! - mutate hardware;
//! - modify QEC state;
//! - modify an Incident;
//! - redefine ZQN fault semantics;
//! - claim causal proof when only observational evidence exists.
//!
//! Those responsibilities belong to:
//!
//!     policy/
//!     planning/
//!     adaptation/
//!     recovery/
//!     verification/
//!     quantum::zqn
//!     quantum::hardware
//!     quantum::routing
//!     quantum::scheduling
//!     quantum::optimization
//!     quantum::qec
//!
//! =============================================================================
//! Architectural position
//! =============================================================================
//!
//!     hardware / runtime / QEC / ZQN / benchmarking / telemetry
//!                              |
//!                              v
//!                         detection
//!                              |
//!                              v
//!                      DetectionOutput
//!                              |
//!                              v
//!                    +-------------------+
//!                    |   diagnostician   |
//!                    +-------------------+
//!                       |      |       |
//!                       v      v       v
//!                   classifier correlation root-cause
//!                       |      |       |
//!                       +------+------+
//!                              |
//!                              v
//!                          Diagnosis
//!                              |
//!                              v
//!                            policy
//!                              |
//!                              v
//!                           planning
//!                              |
//!                              v
//!                           recovery
//!                              |
//!                              v
//!                         verification
//!
//! The diagnostician is therefore an orchestration/composition boundary.
//!
//! =============================================================================
//! Design principles
//! =============================================================================
//!
//! 1. Write once, scale everywhere.
//!
//! No qubit count, detector count, incident count, backend count, or machine
//! size is encoded here.
//!
//! "Infinity" means that this semantic layer imposes no artificial finite
//! machine-size ceiling. Concrete executions remain bounded by the resources
//! supplied by the caller and by explicit runtime/policy limits.
//!
//! 2. Evidence is not truth.
//!
//! Detection signals are observations. A diagnosis is an interpretation of
//! those observations. The implementation must preserve that distinction.
//!
//! 3. Diagnosis is not recovery.
//!
//! A diagnosis may recommend that downstream policy/planning consider an
//! action, but this module never executes one.
//!
//! 4. Confidence is evidence strength.
//!
//! Detection confidence is converted only through an explicit caller-supplied
//! interpretation boundary. No hidden confidence threshold exists here.
//!
//! 5. Determinism.
//!
//! Given identical request data, contributor configuration, contributor state,
//! and contributor ordering, deterministic contributors must produce the same
//! diagnosis.
//!
//! 6. No hidden I/O.
//!
//! This module does not read:
//!
//! - clocks;
//! - environment variables;
//! - filesystem;
//! - network;
//! - process state;
//! - hardware;
//! - random generators;
//! - global mutable state.
//!
//! 7. Canonical identity.
//!
//! This file does not define QubitId or PhysicalQubitId.
//!
//! Quantum identities remain owned by:
//!
//!     crate::quantum::ir::qubit
//!
//! When a future diagnosis contributor needs a qubit identity, it MUST use the
//! canonical IR types rather than introducing a resilience-local replacement.
//!
//! 8. Canonical fault semantics.
//!
//! ZQN remains authoritative for physical/noise/fault semantics.
//!
//! The diagnostician consumes resilience-normalized evidence and does not
//! construct a competing quantum-fault ontology.
//!
//! 9. Immutable results.
//!
//! A completed Diagnosis must not change underneath planning, recovery,
//! verification, telemetry, history, or audit consumers.
//!
//! 10. Explicit scalability.
//!
//! Collections are caller-owned and dynamically sized. There is no fixed
//! capacity, array size, or hard-coded machine limit.
//!
//! =============================================================================
//! Integration contract
//! =============================================================================
//!
//! This file is intended to be implemented before:
//!
//!     diagnosis/classifier.rs
//!     diagnosis/correlation.rs
//!     diagnosis/localization.rs
//!     diagnosis/root_cause.rs
//!     diagnosis/confidence.rs
//!
//! Those modules implement diagnosis contributors and consume the stable
//! structures defined here.
//!
//! Dependency direction:
//!
//!     errors ---------------------+
//!                                  |
//!     model::incident ------------>|
//!                                  |
//!     detection::detector -------->| diagnostician.rs
//!                                  |
//!                                  v
//!                         diagnosis contributors
//!                                  |
//!                                  v
//!                              policy
//!                                  |
//!                                  v
//!                              planning
//!
//! The diagnostician must not depend on concrete classifier/correlation/root
//! cause implementations. This prevents circular dependencies and allows new
//! diagnosis strategies to be added without modifying the stable orchestrator.
//!
//! =============================================================================
//! Rust compatibility
//! =============================================================================
//!
//! Target:
//!
//! - Rust 1.97
//! - Rust 1.97.1
//! - Rust 2021 edition
//! - stable Rust
//! - no nightly features
//! - no unsafe code
//!
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::todo)]
#![deny(clippy::unimplemented)]

use core::cmp::Ordering;
use core::fmt;
use core::num::NonZeroU64;
use std::collections::{BTreeMap, BTreeSet};

use crate::quantum::resilience::detection::detector::{
    DetectionClassification,
    DetectionConfidence,
    DetectionOutput,
    DetectionSequence,
    DetectionSignal,
    DetectorIdentity,
    ObservationId,
    SignalId,
};
use crate::quantum::resilience::errors::{
    ResilienceError,
    ResilienceErrorCode,
    ResilienceResult,
};
use crate::quantum::resilience::model::incident::Incident;

// =============================================================================
// Stable schema
// =============================================================================

/// Stable schema identifier for the diagnosis contract.
pub const DIAGNOSIS_SCHEMA_ID: &str =
    "zamani.quantum.resilience.diagnosis.diagnostician";

/// Semantic version of the diagnosis contract.
pub const DIAGNOSIS_SCHEMA_VERSION: u16 = 1;

// =============================================================================
// Diagnosis identity
// =============================================================================

/// Stable identity for one completed diagnosis.
///
/// This identifier is supplied by the caller. The diagnostician never derives
/// an identity from:
///
/// - memory addresses;
/// - process IDs;
/// - current time;
/// - random numbers;
/// - thread IDs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DiagnosisId(NonZeroU64);

impl DiagnosisId {
    /// Creates a diagnosis ID from a non-zero value.
    #[must_use]
    pub const fn new(value: NonZeroU64) -> Self {
        Self(value)
    }

    /// Creates a diagnosis ID from a raw integer.
    ///
    /// Returns `None` for zero.
    #[must_use]
    pub const fn from_u64(value: u64) -> Option<Self> {
        NonZeroU64::new(value).map(Self)
    }

    /// Returns the underlying opaque identifier.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0.get()
    }
}

impl fmt::Display for DiagnosisId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "diagnosis-{}", self.value())
    }
}

// =============================================================================
// Diagnosis contributor identity
// =============================================================================

/// Stable identity of one diagnosis contributor.
///
/// A contributor can later be implemented by:
///
/// - classifier.rs;
/// - correlation.rs;
/// - localization.rs;
/// - root_cause.rs;
/// - confidence.rs;
/// - future domain-specific diagnosis modules.
///
/// The diagnostician treats contributor identity as descriptive metadata only.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ContributorIdentity {
    name: String,
    version: String,
}

impl ContributorIdentity {
    /// Creates a contributor identity.
    ///
    /// Empty names and versions are rejected because they cannot form a stable
    /// machine-readable identity.
    pub fn new(
        name: impl Into<String>,
        version: impl Into<String>,
    ) -> ResilienceResult<Self> {
        let name = name.into();
        let version = version.into();

        if name.trim().is_empty() || version.trim().is_empty() {
            return Err(ResilienceError::new(
                ResilienceErrorCode::InvalidArgument,
            ));
        }

        Ok(Self { name, version })
    }

    /// Returns the contributor name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the contributor version.
    #[must_use]
    pub fn version(&self) -> &str {
        &self.version
    }
}

impl fmt::Display for ContributorIdentity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}@{}", self.name, self.version)
    }
}

// =============================================================================
// Diagnosis category
// =============================================================================

/// Provider-neutral semantic category for a diagnosis finding.
///
/// This is deliberately broader than a hardware-fault taxonomy.
///
/// Concrete diagnosis modules may map domain-specific observations into these
/// categories. New external providers must not require provider-specific
/// variants in the core orchestration logic.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum DiagnosisCategory {
    /// No sufficiently supported condition was established.
    NoCondition,

    /// General anomaly with no stronger interpretation.
    Anomaly,

    /// Physical/noise fault interpretation.
    Fault,

    /// Hardware degradation.
    Hardware,

    /// Calibration or device-parameter drift.
    CalibrationDrift,

    /// Noise-characterization drift.
    NoiseDrift,

    /// Resource degradation or exhaustion.
    Resource,

    /// Backend or execution service failure.
    Backend,

    /// Routing-related condition.
    Routing,

    /// Scheduling/timing-related condition.
    Scheduling,

    /// QEC-related condition.
    Qec,

    /// Timeout/deadline condition.
    Timeout,

    /// Execution failure.
    ExecutionFailure,

    /// Security/integrity concern.
    Security,

    /// Observation/data-quality problem.
    DataQuality,

    /// Multiple interacting categories.
    Correlated,

    /// Software/compiler/runtime condition.
    Software,

    /// Semantic correctness concern.
    Semantic,

    /// Unknown/insufficiently classified condition.
    Unknown,

    /// Extension category supplied by a future module.
    External(String),
}

impl DiagnosisCategory {
    /// Returns a stable machine-readable category string.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::NoCondition => "no_condition",
            Self::Anomaly => "anomaly",
            Self::Fault => "fault",
            Self::Hardware => "hardware",
            Self::CalibrationDrift => "calibration_drift",
            Self::NoiseDrift => "noise_drift",
            Self::Resource => "resource",
            Self::Backend => "backend",
            Self::Routing => "routing",
            Self::Scheduling => "scheduling",
            Self::Qec => "qec",
            Self::Timeout => "timeout",
            Self::ExecutionFailure => "execution_failure",
            Self::Security => "security",
            Self::DataQuality => "data_quality",
            Self::Correlated => "correlated",
            Self::Software => "software",
            Self::Semantic => "semantic",
            Self::Unknown => "unknown",
            Self::External(value) => value.as_str(),
        }
    }

    /// Returns whether this category represents an actionable candidate.
    #[must_use]
    pub const fn is_actionable(&self) -> bool {
        !matches!(self, Self::NoCondition)
    }
}

// =============================================================================
// Diagnosis confidence
// =============================================================================

/// Normalized confidence attached to a diagnosis finding.
///
/// This is intentionally separate from physical fault probability and from
/// recovery success probability.
///
/// The value is finite and lies in `[0, 1]`.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct DiagnosisConfidence(f64);

impl DiagnosisConfidence {
    /// Creates validated diagnosis confidence.
    pub fn new(value: f64) -> ResilienceResult<Self> {
        if !value.is_finite() || !(0.0..=1.0).contains(&value) {
            return Err(ResilienceError::new(
                ResilienceErrorCode::InvalidArgument,
            ));
        }

        Ok(Self(value))
    }

    /// Returns the confidence value.
    #[must_use]
    pub const fn value(self) -> f64 {
        self.0
    }

    /// Zero confidence.
    #[must_use]
    pub const fn zero() -> Self {
        Self(0.0)
    }

    /// Maximum confidence representable by this model.
    #[must_use]
    pub const fn maximum() -> Self {
        Self(1.0)
    }

    /// Returns whether this confidence meets an explicit caller-supplied
    /// requirement.
    #[must_use]
    pub fn meets(self, required: Self) -> bool {
        self.0 >= required.0
    }

    /// Returns the more conservative of two confidence values.
    #[must_use]
    pub fn minimum(self, other: Self) -> Self {
        if self.0 <= other.0 {
            self
        } else {
            other
        }
    }

    /// Returns the stronger of two confidence values.
    #[must_use]
    pub fn maximum_of(self, other: Self) -> Self {
        if self.0 >= other.0 {
            self
        } else {
            other
        }
    }
}

impl Eq for DiagnosisConfidence {}

impl Ord for DiagnosisConfidence {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other)
            .unwrap_or(Ordering::Equal)
    }
}

impl fmt::Display for DiagnosisConfidence {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

// =============================================================================
// Evidence reference
// =============================================================================

/// Immutable reference to one detection signal used as diagnosis evidence.
///
/// The complete raw observation remains owned by the detection/telemetry
/// subsystem. Diagnosis retains only stable identifiers and interpretation
/// metadata.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EvidenceReference {
    signal_id: SignalId,
    observation_id: Option<ObservationId>,
    detector: ContributorIdentity,
    classification: DetectionClassification,
    confidence_bits: u64,
    sequence: DetectionSequence,
}

impl EvidenceReference {
    /// Creates an evidence reference from a detection signal.
    ///
    /// The floating-point confidence is represented by its exact bit pattern
    /// for deterministic equality and ordering. No arithmetic interpretation
    /// is performed here.
    pub fn from_signal(signal: &DetectionSignal) -> ResilienceResult<Self> {
        let detector = ContributorIdentity::new(
            signal.detector().name(),
            signal.detector().version(),
        )?;

        Ok(Self {
            signal_id: signal.id(),
            observation_id: signal.observation_id(),
            detector,
            classification: signal.classification(),
            confidence_bits: signal.confidence().value().to_bits(),
            sequence: signal.sequence(),
        })
    }

    /// Returns the signal identity.
    #[must_use]
    pub const fn signal_id(&self) -> SignalId {
        self.signal_id
    }

    /// Returns the originating observation identity.
    #[must_use]
    pub const fn observation_id(&self) -> Option<ObservationId> {
        self.observation_id
    }

    /// Returns the detector identity.
    #[must_use]
    pub fn detector(&self) -> &ContributorIdentity {
        &self.detector
    }

    /// Returns the original detection classification.
    #[must_use]
    pub const fn classification(&self) -> DetectionClassification {
        self.classification
    }

    /// Returns the original detection confidence.
    ///
    /// The original detection contract guarantees a normalized finite value,
    /// therefore reconstruction is expected to succeed.
    pub fn confidence(&self) -> ResilienceResult<DetectionConfidence> {
        let value = f64::from_bits(self.confidence_bits);

        DetectionConfidence::new(value)
    }

    /// Returns the detection sequence.
    #[must_use]
    pub const fn sequence(&self) -> DetectionSequence {
        self.sequence
    }
}

// =============================================================================
// Diagnosis finding
// =============================================================================

/// One immutable interpretation produced by a diagnosis contributor.
///
/// A finding is not a recovery command and not causal proof.
///
/// The `category` describes the interpretation. The evidence references explain
/// why the interpretation was produced.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiagnosisFinding {
    category: DiagnosisCategory,
    confidence: DiagnosisConfidence,
    evidence: Vec<EvidenceReference>,
    contributor: ContributorIdentity,
    explanation: Option<String>,
}

impl DiagnosisFinding {
    /// Creates a finding.
    ///
    /// Evidence is deterministically sorted and duplicate signal references
    /// are removed.
    pub fn new(
        category: DiagnosisCategory,
        confidence: DiagnosisConfidence,
        evidence: impl IntoIterator<Item = EvidenceReference>,
        contributor: ContributorIdentity,
        explanation: Option<String>,
    ) -> Self {
        let mut evidence: Vec<EvidenceReference> = evidence.into_iter().collect();

        evidence.sort();
        evidence.dedup();

        let explanation = explanation.and_then(|value| {
            let trimmed = value.trim();

            if trimmed.is_empty() {
                None
            } else {
                Some(trimmed.to_owned())
            }
        });

        Self {
            category,
            confidence,
            evidence,
            contributor,
            explanation,
        }
    }

    /// Returns the diagnosis category.
    #[must_use]
    pub const fn category(&self) -> &DiagnosisCategory {
        &self.category
    }

    /// Returns diagnosis confidence.
    #[must_use]
    pub const fn confidence(&self) -> DiagnosisConfidence {
        self.confidence
    }

    /// Returns immutable evidence references.
    #[must_use]
    pub fn evidence(&self) -> &[EvidenceReference] {
        &self.evidence
    }

    /// Returns the contributor that produced the finding.
    #[must_use]
    pub fn contributor(&self) -> &ContributorIdentity {
        &self.contributor
    }

    /// Returns the optional human-readable explanation.
    ///
    /// The explanation is descriptive only and must never be parsed as a
    /// machine-readable command.
    #[must_use]
    pub fn explanation(&self) -> Option<&str> {
        self.explanation.as_deref()
    }

    /// Returns whether this finding has actionable semantic content.
    #[must_use]
    pub fn is_actionable(&self) -> bool {
        self.category.is_actionable()
    }

    /// Returns the number of distinct evidence references.
    #[must_use]
    pub fn evidence_count(&self) -> usize {
        self.evidence.len()
    }
}

// =============================================================================
// Diagnosis request
// =============================================================================

/// Immutable input to one diagnosis evaluation.
///
/// The caller owns collection size and therefore controls memory consumption.
///
/// A large distributed system can provide many detector outputs; a small
/// system can provide one. No fixed cardinality is assumed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiagnosisRequest {
    diagnosis_id: DiagnosisId,
    sequence: DetectionSequence,
    incident: Option<Incident>,
    outputs: Vec<DetectionOutput>,
    require_verified_evidence: bool,
}

impl DiagnosisRequest {
    /// Creates a diagnosis request.
    ///
    /// Detector outputs are deterministically ordered by:
    ///
    /// 1. detection sequence;
    /// 2. detector identity;
    ///
    /// The ordering is semantic-neutral. It exists only to make replay and
    /// contributor processing deterministic.
    pub fn new(
        diagnosis_id: DiagnosisId,
        sequence: DetectionSequence,
        incident: Option<Incident>,
        outputs: impl IntoIterator<Item = DetectionOutput>,
        require_verified_evidence: bool,
    ) -> Self {
        let mut outputs: Vec<DetectionOutput> = outputs.into_iter().collect();

        outputs.sort_by(|left, right| {
            left.metadata()
                .sequence()
                .cmp(&right.metadata().sequence())
                .then_with(|| {
                    left.metadata()
                        .detector()
                        .cmp(right.metadata().detector())
                })
        });

        Self {
            diagnosis_id,
            sequence,
            incident,
            outputs,
            require_verified_evidence,
        }
    }

    /// Returns diagnosis identity.
    #[must_use]
    pub const fn diagnosis_id(&self) -> DiagnosisId {
        self.diagnosis_id
    }

    /// Returns diagnosis sequence.
    #[must_use]
    pub const fn sequence(&self) -> DetectionSequence {
        self.sequence
    }

    /// Returns the associated incident, if one exists.
    #[must_use]
    pub fn incident(&self) -> Option<&Incident> {
        self.incident.as_ref()
    }

    /// Returns detector outputs.
    #[must_use]
    pub fn outputs(&self) -> &[DetectionOutput] {
        &self.outputs
    }

    /// Returns whether verified evidence is required.
    #[must_use]
    pub const fn require_verified_evidence(&self) -> bool {
        self.require_verified_evidence
    }

    /// Returns whether the request contains any detector output.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.outputs.is_empty()
    }

    /// Returns the number of detector outputs.
    #[must_use]
    pub fn output_count(&self) -> usize {
        self.outputs.len()
    }
}

// =============================================================================
// Diagnosis result
// =============================================================================

/// Immutable completed diagnosis.
///
/// The result deliberately preserves:
///
/// - diagnosis identity;
/// - evaluation sequence;
/// - findings;
/// - evidence;
/// - contributors;
/// - signal count;
/// - actionable condition state.
///
/// It does not contain a recovery command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnosis {
    id: DiagnosisId,
    sequence: DetectionSequence,
    findings: Vec<DiagnosisFinding>,
    evidence: Vec<EvidenceReference>,
    contributors: Vec<ContributorIdentity>,
    examined_signal_count: usize,
    actionable_signal_count: usize,
    conflict: bool,
}

impl Diagnosis {
    fn new(
        id: DiagnosisId,
        sequence: DetectionSequence,
        findings: Vec<DiagnosisFinding>,
        evidence: Vec<EvidenceReference>,
        contributors: Vec<ContributorIdentity>,
        examined_signal_count: usize,
        actionable_signal_count: usize,
        conflict: bool,
    ) -> Self {
        Self {
            id,
            sequence,
            findings,
            evidence,
            contributors,
            examined_signal_count,
            actionable_signal_count,
            conflict,
        }
    }

    /// Returns diagnosis identity.
    #[must_use]
    pub const fn id(&self) -> DiagnosisId {
        self.id
    }

    /// Returns detection sequence.
    #[must_use]
    pub const fn sequence(&self) -> DetectionSequence {
        self.sequence
    }

    /// Returns diagnosis findings in deterministic order.
    #[must_use]
    pub fn findings(&self) -> &[DiagnosisFinding] {
        &self.findings
    }

    /// Returns all unique evidence references.
    #[must_use]
    pub fn evidence(&self) -> &[EvidenceReference] {
        &self.evidence
    }

    /// Returns contributors that participated in the diagnosis.
    #[must_use]
    pub fn contributors(&self) -> &[ContributorIdentity] {
        &self.contributors
    }

    /// Returns the number of examined signals.
    #[must_use]
    pub const fn examined_signal_count(&self) -> usize {
        self.examined_signal_count
    }

    /// Returns the number of actionable signals.
    #[must_use]
    pub const fn actionable_signal_count(&self) -> usize {
        self.actionable_signal_count
    }

    /// Returns whether at least one actionable diagnosis exists.
    #[must_use]
    pub fn is_actionable(&self) -> bool {
        self.findings.iter().any(DiagnosisFinding::is_actionable)
    }

    /// Returns whether no diagnosis finding was produced.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.findings.is_empty()
    }

    /// Returns whether the contributors produced incompatible findings.
    ///
    /// A conflict does not automatically mean that the quantum system is
    /// faulty. It means that diagnosis could not establish one compatible
    /// interpretation from the supplied evidence.
    #[must_use]
    pub const fn has_conflict(&self) -> bool {
        self.conflict
    }

    /// Returns the strongest finding by confidence, with deterministic
    /// tie-breaking.
    #[must_use]
    pub fn strongest_finding(&self) -> Option<&DiagnosisFinding> {
        self.findings.iter().max_by(|left, right| {
            left.confidence()
                .cmp(&right.confidence())
                .then_with(|| right.category().cmp(left.category()))
                .then_with(|| right.contributor().cmp(left.contributor()))
        })
    }

    /// Returns all findings belonging to one category.
    ///
    /// This allocates a result collection and is therefore intended for callers
    /// that explicitly need materialized filtering.
    #[must_use]
    pub fn findings_for(
        &self,
        category: &DiagnosisCategory,
    ) -> Vec<&DiagnosisFinding> {
        self.findings
            .iter()
            .filter(|finding| finding.category() == category)
            .collect()
    }
}

// =============================================================================
// Diagnosis contributor
// =============================================================================

/// Stable extension point for diagnosis algorithms.
///
/// Future modules should normally implement this trait rather than modify the
/// diagnostician itself.
///
/// Examples:
///
/// - `classifier.rs`
/// - `correlation.rs`
/// - `localization.rs`
/// - `root_cause.rs`
/// - statistical diagnosis contributors
/// - hardware-specific adapters
/// - QEC diagnosis contributors
/// - learned diagnosis contributors
///
/// Contributors:
///
/// - receive immutable evidence;
/// - may emit zero, one, or many findings;
/// - never execute recovery;
/// - never mutate hardware;
/// - never mutate the incident;
/// - never authorize actions.
pub trait DiagnosisContributor {
    /// Returns stable contributor identity.
    fn identity(&self) -> &ContributorIdentity;

    /// Produces diagnosis findings from the supplied request.
    fn diagnose(
        &mut self,
        request: &DiagnosisRequest,
    ) -> ResilienceResult<Vec<DiagnosisFinding>>;

    /// Returns whether the contributor is currently available.
    ///
    /// Availability is not a health diagnosis.
    fn is_available(&self) -> bool {
        true
    }

    /// Resets contributor-local state.
    ///
    /// Stateless contributors can use the default implementation.
    fn reset(&mut self) {}
}

// =============================================================================
// Object-safe contributor boundary
// =============================================================================

/// Object-safe contributor interface for dynamic registries.
///
/// This permits heterogeneous contributors without forcing callers to know
/// their concrete types.
pub trait DiagnosisContributorObject {
    /// Returns contributor identity.
    fn identity(&self) -> &ContributorIdentity;

    /// Runs diagnosis.
    fn diagnose(
        &mut self,
        request: &DiagnosisRequest,
    ) -> ResilienceResult<Vec<DiagnosisFinding>>;

    /// Returns availability.
    fn is_available(&self) -> bool {
        true
    }

    /// Resets contributor-local state.
    fn reset(&mut self) {}
}

impl<T> DiagnosisContributorObject for T
where
    T: DiagnosisContributor,
{
    fn identity(&self) -> &ContributorIdentity {
        DiagnosisContributor::identity(self)
    }

    fn diagnose(
        &mut self,
        request: &DiagnosisRequest,
    ) -> ResilienceResult<Vec<DiagnosisFinding>> {
        DiagnosisContributor::diagnose(self, request)
    }

    fn is_available(&self) -> bool {
        DiagnosisContributor::is_available(self)
    }

    fn reset(&mut self) {
        DiagnosisContributor::reset(self);
    }
}

// =============================================================================
// Diagnostician configuration
// =============================================================================

/// Explicit configuration for diagnosis composition.
///
/// There are deliberately no hidden thresholds.
///
/// In particular, this configuration does NOT contain:
///
/// - a default confidence threshold;
/// - a fixed maximum detector count;
/// - a fixed maximum qubit count;
/// - a fixed maximum incident size;
/// - a fixed retry count.
///
/// Policy owns operational thresholds.
///
/// The diagnostician only controls structural behavior of the composition
/// operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiagnosticianConfig {
    reject_conflicting_findings: bool,
    reject_empty_input: bool,
    ignore_unavailable_contributors: bool,
}

impl Default for DiagnosticianConfig {
    fn default() -> Self {
        Self {
            reject_conflicting_findings: false,
            reject_empty_input: false,
            ignore_unavailable_contributors: true,
        }
    }
}

impl DiagnosticianConfig {
    /// Creates configuration with explicit values.
    #[must_use]
    pub const fn new(
        reject_conflicting_findings: bool,
        reject_empty_input: bool,
        ignore_unavailable_contributors: bool,
    ) -> Self {
        Self {
            reject_conflicting_findings,
            reject_empty_input,
            ignore_unavailable_contributors,
        }
    }

    /// Returns whether conflicting findings are fatal.
    #[must_use]
    pub const fn reject_conflicting_findings(&self) -> bool {
        self.reject_conflicting_findings
    }

    /// Returns whether empty input is rejected.
    #[must_use]
    pub const fn reject_empty_input(&self) -> bool {
        self.reject_empty_input
    }

    /// Returns whether unavailable contributors are skipped.
    #[must_use]
    pub const fn ignore_unavailable_contributors(&self) -> bool {
        self.ignore_unavailable_contributors
    }
}

// =============================================================================
// Diagnostician
// =============================================================================

/// Production diagnosis orchestrator.
///
/// `Diagnostician` composes independently implemented diagnosis contributors.
///
/// It intentionally contains no quantum-machine-specific knowledge.
///
/// A single instance can therefore be used for:
///
/// - one qubit;
/// - many qubits;
/// - logical qubits;
/// - physical QPUs;
/// - simulators;
/// - heterogeneous systems;
/// - distributed quantum execution;
///
/// subject only to the resources supplied by the caller.
///
/// The contributor collection is owned by the diagnostician. No global
/// registry is required.
pub struct Diagnostician {
    config: DiagnosticianConfig,
    contributors: Vec<Box<dyn DiagnosisContributorObject>>,
}

impl Diagnostician {
    /// Creates a diagnostician with explicit configuration and contributors.
    ///
    /// Contributor order is preserved because it can be part of deterministic
    /// replay semantics.
    #[must_use]
    pub fn new(
        config: DiagnosticianConfig,
        contributors: Vec<Box<dyn DiagnosisContributorObject>>,
    ) -> Self {
        Self {
            config,
            contributors,
        }
    }

    /// Creates an empty diagnostician.
    ///
    /// This is useful when contributors are installed dynamically through the
    /// explicit `register` method.
    #[must_use]
    pub fn empty() -> Self {
        Self::new(DiagnosticianConfig::default(), Vec::new())
    }

    /// Returns the immutable configuration.
    #[must_use]
    pub const fn config(&self) -> &DiagnosticianConfig {
        &self.config
    }

    /// Returns the number of registered contributors.
    ///
    /// This is a runtime property, not a system limit.
    #[must_use]
    pub fn contributor_count(&self) -> usize {
        self.contributors.len()
    }

    /// Registers one contributor.
    ///
    /// No fixed contributor count is imposed.
    pub fn register(
        &mut self,
        contributor: Box<dyn DiagnosisContributorObject>,
    ) {
        self.contributors.push(contributor);
    }

    /// Returns the identities of all registered contributors.
    #[must_use]
    pub fn contributor_identities(&self) -> Vec<&ContributorIdentity> {
        self.contributors
            .iter()
            .map(|contributor| contributor.identity())
            .collect()
    }

    /// Resets all contributor-local state.
    ///
    /// This does not modify diagnosis history, incidents, telemetry, or
    /// hardware.
    pub fn reset_contributors(&mut self) {
        for contributor in &mut self.contributors {
            contributor.reset();
        }
    }

    /// Diagnoses one explicit request.
    ///
    /// Processing is:
    ///
    ///     validate request
    ///         |
    ///         v
    ///     normalize evidence
    ///         |
    ///         v
    ///     invoke contributors
    ///         |
    ///         v
    ///     validate findings
    ///         |
    ///         v
    ///     deterministically compose findings
    ///         |
    ///         v
    ///     return immutable Diagnosis
    ///
    /// No recovery or adaptation occurs here.
    pub fn diagnose(
        &mut self,
        request: &DiagnosisRequest,
    ) -> ResilienceResult<Diagnosis> {
        self.validate_request(request)?;

        let evidence = collect_evidence(request)?;
        let (examined_signal_count, actionable_signal_count) =
            signal_counts(request);

        let mut findings = Vec::new();
        let mut contributors = BTreeSet::new();

        for contributor in &mut self.contributors {
            if !contributor.is_available() {
                if self.config.ignore_unavailable_contributors() {
                    continue;
                }

                return Err(ResilienceError::new(
                    ResilienceErrorCode::DiagnosisFailed,
                ));
            }

            let contributor_identity = contributor.identity().clone();

            let contributor_findings = contributor.diagnose(request)?;

            for finding in contributor_findings {
                validate_finding(&finding)?;

                contributors.insert(contributor_identity.clone());
                findings.push(finding);
            }
        }

        canonicalize_findings(&mut findings);

        let conflict = detect_conflict(&findings);

        if conflict && self.config.reject_conflicting_findings() {
            return Err(ResilienceError::new(
                ResilienceErrorCode::DiagnosisConflict,
            ));
        }

        let contributors = contributors.into_iter().collect();

        Ok(Diagnosis::new(
            request.diagnosis_id(),
            request.sequence(),
            findings,
            evidence,
            contributors,
            examined_signal_count,
            actionable_signal_count,
            conflict,
        ))
    }

    fn validate_request(
        &self,
        request: &DiagnosisRequest,
    ) -> ResilienceResult<()> {
        if self.config.reject_empty_input() && request.is_empty() {
            return Err(ResilienceError::new(
                ResilienceErrorCode::InsufficientEvidence,
            ));
        }

        for output in request.outputs() {
            if output.metadata().sequence() != request.sequence() {
                return Err(ResilienceError::new(
                    ResilienceErrorCode::DetectionInconsistent,
                ));
            }

            for signal in output.signals() {
                if signal.sequence() != request.sequence() {
                    return Err(ResilienceError::new(
                        ResilienceErrorCode::DetectionInconsistent,
                    ));
                }
            }
        }

        if let Some(incident) = request.incident() {
            if !incident.is_structurally_valid() {
                return Err(ResilienceError::new(
                    ResilienceErrorCode::InvariantViolation,
                ));
            }
        }

        Ok(())
    }
}

// =============================================================================
// Evidence collection
// =============================================================================

fn collect_evidence(
    request: &DiagnosisRequest,
) -> ResilienceResult<Vec<EvidenceReference>> {
    let mut evidence_by_signal = BTreeMap::<SignalId, EvidenceReference>::new();

    for output in request.outputs() {
        for signal in output.signals() {
            if !signal.is_actionable_candidate() {
                continue;
            }

            let evidence = EvidenceReference::from_signal(signal)?;

            evidence_by_signal
                .entry(signal.id())
                .or_insert(evidence);
        }
    }

    Ok(evidence_by_signal.into_values().collect())
}

// =============================================================================
// Signal statistics
// =============================================================================

fn signal_counts(request: &DiagnosisRequest) -> (usize, usize) {
    let mut seen = BTreeSet::<SignalId>::new();
    let mut actionable = 0usize;

    for output in request.outputs() {
        for signal in output.signals() {
            if !seen.insert(signal.id()) {
                continue;
            }

            if signal.is_actionable_candidate() {
                actionable += 1;
            }
        }
    }

    (seen.len(), actionable)
}

// =============================================================================
// Finding validation
// =============================================================================

fn validate_finding(
    finding: &DiagnosisFinding,
) -> ResilienceResult<()> {
    if finding.contributor().name().trim().is_empty()
        || finding.contributor().version().trim().is_empty()
    {
        return Err(ResilienceError::new(
            ResilienceErrorCode::InvariantViolation,
        ));
    }

    let mut previous: Option<&EvidenceReference> = None;

    for evidence in finding.evidence() {
        if let Some(previous) = previous {
            if previous >= evidence {
                return Err(ResilienceError::new(
                    ResilienceErrorCode::InvariantViolation,
                ));
            }
        }

        previous = Some(evidence);
    }

    Ok(())
}

// =============================================================================
// Finding canonicalization
// =============================================================================

fn canonicalize_findings(findings: &mut Vec<DiagnosisFinding>) {
    findings.sort_by(|left, right| {
        right
            .confidence()
            .cmp(&left.confidence())
            .then_with(|| left.category().cmp(right.category()))
            .then_with(|| left.contributor().cmp(right.contributor()))
            .then_with(|| left.evidence().cmp(right.evidence()))
    });

    findings.dedup();
}

// =============================================================================
// Conflict detection
// =============================================================================

/// Determines whether the diagnosis contains mutually incompatible high-level
/// interpretations.
///
/// This function deliberately does not use a confidence threshold. Policy is
/// responsible for deciding whether a conflict is acceptable.
///
/// The conflict detector is conservative:
///
/// - different categories alone are NOT automatically a conflict;
/// - `NoCondition` combined with an actionable finding is a conflict;
/// - `Security` combined with another actionable category is a conflict
///   because security interpretation requires explicit downstream handling;
/// - multiple actionable findings are otherwise allowed because correlated
///   quantum failures may legitimately have several causes.
fn detect_conflict(findings: &[DiagnosisFinding]) -> bool {
    let mut has_no_condition = false;
    let mut has_actionable = false;
    let mut has_security = false;

    for finding in findings {
        match finding.category() {
            DiagnosisCategory::NoCondition => {
                has_no_condition = true;
            }
            DiagnosisCategory::Security => {
                has_security = true;
                has_actionable = true;
            }
            category if category.is_actionable() => {
                has_actionable = true;
            }
            _ => {}
        }
    }

    (has_no_condition && has_actionable)
        || (has_security
            && findings
                .iter()
                .any(|finding| {
                    !matches!(
                        finding.category(),
                        DiagnosisCategory::Security
                    ) && finding.category().is_actionable()
                }))
}

// =============================================================================
// Convenience composition function
// =============================================================================

/// Runs a set of contributors without requiring callers to construct a
/// persistent `Diagnostician`.
///
/// This function is useful for one-shot diagnosis and tests.
///
/// For long-running systems, prefer [`Diagnostician`] so contributor state and
/// configuration remain explicit.
pub fn diagnose_with_contributors(
    request: &DiagnosisRequest,
    config: DiagnosticianConfig,
    contributors: &mut [Box<dyn DiagnosisContributorObject>],
) -> ResilienceResult<Diagnosis> {
    let mut diagnostician = Diagnostician::new(
        config,
        std::mem::take(&mut contributors.to_vec()),
    );

    diagnostician.diagnose(request)
}

// =============================================================================
// Built-in observation-only contributor
// =============================================================================

/// A minimal contributor that preserves actionable detection classifications
/// as diagnosis findings without claiming a stronger root cause.
///
/// This is intentionally conservative.
///
/// It provides a safe baseline when no specialized classifier has been
/// installed yet.
///
/// Specialized `classifier.rs` should normally supersede or complement this
/// contributor.
#[derive(Debug, Clone)]
pub struct DetectionClassificationContributor {
    identity: ContributorIdentity,
}

impl DetectionClassificationContributor {
    /// Creates the baseline contributor.
    pub fn new(
        version: impl Into<String>,
    ) -> ResilienceResult<Self> {
        Ok(Self {
            identity: ContributorIdentity::new(
                "zamani.detection-classification",
                version,
            )?,
        })
    }

    fn category(
        classification: DetectionClassification,
    ) -> DiagnosisCategory {
        match classification {
            DetectionClassification::NoCondition => {
                DiagnosisCategory::NoCondition
            }
            DetectionClassification::Anomaly => DiagnosisCategory::Anomaly,
            DetectionClassification::Fault => DiagnosisCategory::Fault,
            DetectionClassification::Degradation => {
                DiagnosisCategory::Resource
            }
            DetectionClassification::Unavailability => {
                DiagnosisCategory::Resource
            }
            DetectionClassification::Timeout => {
                DiagnosisCategory::Timeout
            }
            DetectionClassification::ExecutionFailure => {
                DiagnosisCategory::ExecutionFailure
            }
            DetectionClassification::QecSignal => DiagnosisCategory::Qec,
            DetectionClassification::HardwareSignal => {
                DiagnosisCategory::Hardware
            }
            DetectionClassification::Inconclusive => {
                DiagnosisCategory::Unknown
            }
        }
    }
}

impl DiagnosisContributor for DetectionClassificationContributor {
    fn identity(&self) -> &ContributorIdentity {
        &self.identity
    }

    fn diagnose(
        &mut self,
        request: &DiagnosisRequest,
    ) -> ResilienceResult<Vec<DiagnosisFinding>> {
        let mut findings = Vec::new();
        let mut seen = BTreeSet::<SignalId>::new();

        for output in request.outputs() {
            for signal in output.signals() {
                if !seen.insert(signal.id()) {
                    continue;
                }

                if !signal.is_actionable_candidate() {
                    continue;
                }

                let evidence =
                    EvidenceReference::from_signal(signal)?;

                let confidence =
                    DiagnosisConfidence::new(signal.confidence().value())?;

                findings.push(DiagnosisFinding::new(
                    Self::category(signal.classification()),
                    confidence,
                    [evidence],
                    self.identity.clone(),
                    None,
                ));
            }
        }

        Ok(findings)
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use core::num::NonZeroU64;

    struct StaticContributor {
        identity: ContributorIdentity,
        category: DiagnosisCategory,
        confidence: DiagnosisConfidence,
    }

    impl StaticContributor {
        fn new(
            name: &str,
            category: DiagnosisCategory,
            confidence: DiagnosisConfidence,
        ) -> Self {
            Self {
                identity: ContributorIdentity::new(
                    name,
                    "1",
                )
                .expect("test contributor identity must be valid"),
                category,
                confidence,
            }
        }
    }

    impl DiagnosisContributor for StaticContributor {
        fn identity(&self) -> &ContributorIdentity {
            &self.identity
        }

        fn diagnose(
            &mut self,
            request: &DiagnosisRequest,
        ) -> ResilienceResult<Vec<DiagnosisFinding>> {
            let evidence = collect_evidence(request)?;

            Ok(vec![DiagnosisFinding::new(
                self.category.clone(),
                self.confidence,
                evidence,
                self.identity.clone(),
                Some("deterministic test finding".to_owned()),
            )])
        }
    }

    fn diagnosis_id(value: u64) -> DiagnosisId {
        DiagnosisId::new(
            NonZeroU64::new(value)
                .expect("test diagnosis ID must be non-zero"),
        )
    }

    fn sequence(value: u64) -> DetectionSequence {
        DetectionSequence::new(
            NonZeroU64::new(value)
                .expect("test sequence must be non-zero"),
        )
    }

    #[test]
    fn diagnosis_id_rejects_zero() {
        assert!(DiagnosisId::from_u64(0).is_none());
        assert!(DiagnosisId::from_u64(1).is_some());
    }

    #[test]
    fn confidence_rejects_non_finite_values() {
        assert!(DiagnosisConfidence::new(f64::NAN).is_err());
        assert!(DiagnosisConfidence::new(f64::INFINITY).is_err());
        assert!(DiagnosisConfidence::new(f64::NEG_INFINITY).is_err());
    }

    #[test]
    fn confidence_rejects_out_of_range_values() {
        assert!(DiagnosisConfidence::new(-0.1).is_err());
        assert!(DiagnosisConfidence::new(1.1).is_err());
    }

    #[test]
    fn confidence_accepts_closed_interval() {
        assert!(DiagnosisConfidence::new(0.0).is_ok());
        assert!(DiagnosisConfidence::new(0.5).is_ok());
        assert!(DiagnosisConfidence::new(1.0).is_ok());
    }

    #[test]
    fn contributor_identity_rejects_empty_name() {
        assert!(
            ContributorIdentity::new("", "1").is_err()
        );
    }

    #[test]
    fn contributor_identity_rejects_empty_version() {
        assert!(
            ContributorIdentity::new("classifier", "").is_err()
        );
    }

    #[test]
    fn category_names_are_stable() {
        assert_eq!(
            DiagnosisCategory::CalibrationDrift.as_str(),
            "calibration_drift"
        );

        assert_eq!(
            DiagnosisCategory::ExecutionFailure.as_str(),
            "execution_failure"
        );
    }

    #[test]
    fn configuration_is_explicit() {
        let config = DiagnosticianConfig::new(
            true,
            true,
            false,
        );

        assert!(config.reject_conflicting_findings());
        assert!(config.reject_empty_input());
        assert!(!config.ignore_unavailable_contributors());
    }

    #[test]
    fn empty_diagnostician_has_no_contributors() {
        let diagnostician = Diagnostician::empty();

        assert_eq!(diagnostician.contributor_count(), 0);
    }

    #[test]
    fn diagnosis_request_preserves_identity() {
        let request = DiagnosisRequest::new(
            diagnosis_id(7),
            sequence(11),
            None,
            Vec::<DetectionOutput>::new(),
            false,
        );

        assert_eq!(request.diagnosis_id().value(), 7);
        assert_eq!(request.sequence().value(), 11);
        assert!(request.incident().is_none());
        assert!(request.is_empty());
    }

    #[test]
    fn finding_normalizes_evidence_order() {
        let contributor =
            ContributorIdentity::new("test", "1")
                .expect("valid test contributor");

        let finding = DiagnosisFinding::new(
            DiagnosisCategory::Anomaly,
            DiagnosisConfidence::maximum(),
            Vec::<EvidenceReference>::new(),
            contributor,
            Some("  explanation  ".to_owned()),
        );

        assert_eq!(
            finding.explanation(),
            Some("explanation")
        );

        assert!(finding.evidence().is_empty());
    }

    #[test]
    fn strongest_finding_is_deterministic() {
        let contributor =
            ContributorIdentity::new("test", "1")
                .expect("valid test contributor");

        let low = DiagnosisFinding::new(
            DiagnosisCategory::Anomaly,
            DiagnosisConfidence::new(0.2)
                .expect("valid confidence"),
            Vec::<EvidenceReference>::new(),
            contributor.clone(),
            None,
        );

        let high = DiagnosisFinding::new(
            DiagnosisCategory::Hardware,
            DiagnosisConfidence::new(0.9)
                .expect("valid confidence"),
            Vec::<EvidenceReference>::new(),
            contributor,
            None,
        );

        let diagnosis = Diagnosis::new(
            diagnosis_id(1),
            sequence(1),
            vec![high.clone(), low],
            Vec::new(),
            Vec::new(),
            0,
            0,
            false,
        );

        assert_eq!(
            diagnosis
                .strongest_finding()
                .map(DiagnosisFinding::category),
            Some(high.category())
        );
    }

    #[test]
    fn conflict_detection_is_conservative() {
        let contributor =
            ContributorIdentity::new("test", "1")
                .expect("valid test contributor");

        let no_condition = DiagnosisFinding::new(
            DiagnosisCategory::NoCondition,
            DiagnosisConfidence::maximum(),
            Vec::<EvidenceReference>::new(),
            contributor.clone(),
            None,
        );

        let anomaly = DiagnosisFinding::new(
            DiagnosisCategory::Anomaly,
            DiagnosisConfidence::maximum(),
            Vec::<EvidenceReference>::new(),
            contributor,
            None,
        );

        assert!(detect_conflict(
            &[no_condition, anomaly]
        ));
    }

    #[test]
    fn multiple_actionable_categories_are_not_automatically_conflicts() {
        let contributor =
            ContributorIdentity::new("test", "1")
                .expect("valid test contributor");

        let hardware = DiagnosisFinding::new(
            DiagnosisCategory::Hardware,
            DiagnosisConfidence::maximum(),
            Vec::<EvidenceReference>::new(),
            contributor.clone(),
            None,
        );

        let qec = DiagnosisFinding::new(
            DiagnosisCategory::Qec,
            DiagnosisConfidence::maximum(),
            Vec::<EvidenceReference>::new(),
            contributor,
            None,
        );

        assert!(!detect_conflict(&[hardware, qec]));
    }

    #[test]
    fn security_plus_other_actionable_category_is_conflict() {
        let contributor =
            ContributorIdentity::new("test", "1")
                .expect("valid test contributor");

        let security = DiagnosisFinding::new(
            DiagnosisCategory::Security,
            DiagnosisConfidence::maximum(),
            Vec::<EvidenceReference>::new(),
            contributor.clone(),
            None,
        );

        let hardware = DiagnosisFinding::new(
            DiagnosisCategory::Hardware,
            DiagnosisConfidence::maximum(),
            Vec::<EvidenceReference>::new(),
            contributor,
            None,
        );

        assert!(detect_conflict(&[security, hardware]));
    }
}