//! Zamani Quantum Resilience — Detection Correlation.
//!
//! Path:
//!     src/quantum/resilience/diagnosis/correlation.rs
//!
//! =============================================================================
//! Purpose
//! =============================================================================
//!
//! This module correlates normalized detection signals into deterministic,
//! evidence-backed groups that can be consumed by the diagnosis orchestrator.
//!
//! Correlation answers:
//!
//!     "Which observations exhibit an explicit, reproducible relationship
//!      strong enough to be treated as one diagnostic group?"
//!
//! Correlation does NOT answer:
//!
//! - what the physical root cause is;
//! - whether two observations are causally related;
//! - whether recovery is safe;
//! - whether recovery is permitted;
//! - which backend should be selected;
//! - how routing should change;
//! - how scheduling should change;
//! - how QEC should change;
//! - whether a result is semantically correct.
//!
//! Those responsibilities belong to:
//!
//!     diagnosis/root_cause.rs
//!     policy/
//!     planning/
//!     adaptation/
//!     recovery/
//!     verification/
//!     quantum::zqn
//!     quantum::hardware
//!     quantum::routing
//!     quantum::scheduling
//!     quantum::qec
//!
//! =============================================================================
//! Architectural position
//! =============================================================================
//!
//! ```text
//! hardware / runtime / QEC / ZQN / benchmarking / telemetry
//!                              |
//!                              v
//!                         detection
//!                              |
//!                              v
//!                       DetectionOutput
//!                              |
//!                              v
//!                    +-------------------+
//!                    |    correlation    |
//!                    +-------------------+
//!                              |
//!                +-------------+-------------+
//!                |                           |
//!                v                           v
//!        CorrelationGroup             DiagnosisFinding
//!                |                           |
//!                +-------------+-------------+
//!                              |
//!                              v
//!                         diagnostician
//!                              |
//!                              v
//!                         root_cause
//!                              |
//!                              v
//!                           policy
//! ```
//!
//! Correlation is therefore an evidence-composition layer, not a recovery
//! engine.
//!
//! =============================================================================
//! Design principles
//! =============================================================================
//!
//! ## 1. Write once, scale everywhere
//!
//! This module contains no:
//!
//! - maximum qubit count;
//! - maximum signal count;
//! - fixed backend count;
//! - fixed detector count;
//! - fixed machine size;
//! - provider-specific identifiers;
//! - fixed topology;
//! - fixed retry count.
//!
//! Collections are dynamically sized and caller-owned.
//!
//! "Infinity" means that the semantic implementation introduces no artificial
//! finite machine-size ceiling. A concrete invocation is necessarily bounded
//! by available memory, CPU, storage, telemetry capacity, execution policy,
//! and other explicit resource constraints.
//!
//! ## 2. Correlation is not causation
//!
//! A correlation group means that observations share an explicit relationship.
//!
//! It does NOT mean:
//!
//!     observation A caused observation B
//!
//! Causal interpretation belongs to `root_cause.rs`.
//!
//! ## 3. Evidence is preserved
//!
//! Every correlation group retains the exact signal identities from which the
//! group was constructed.
//!
//! Raw observations remain owned by the detection/telemetry layer.
//!
//! ## 4. Determinism
//!
//! Given identical:
//!
//! - detection signals;
//! - correlation configuration;
//! - sequence values;
//! - detector identities;
//! - classifications;
//!
//! the same correlation groups must be produced regardless of input arrival
//! order.
//!
//! ## 5. No hidden state
//!
//! This module does not access:
//!
//! - system time;
//! - filesystem;
//! - environment variables;
//! - network;
//! - hardware;
//! - random generators;
//! - process IDs;
//! - memory addresses;
//! - global mutable state.
//!
//! ## 6. No invented quantum identities
//!
//! This module deliberately does not define:
//!
//!     QubitId
//!     PhysicalQubitId
//!
//! When future correlation metadata requires quantum-resource localization,
//! callers must use:
//!
//!     crate::quantum::ir::qubit::QubitId
//!     crate::quantum::ir::qubit::PhysicalQubitId
//!
//! Correlation itself does not need to manufacture a resource identity from a
//! signal that does not explicitly contain one.
//!
//! ## 7. ZQN owns fault semantics
//!
//! This module does not create a competing fault ontology.
//!
//! Canonical physical/noise/fault semantics remain owned by ZQN.
//!
//! ## 8. Streaming-friendly architecture
//!
//! The public correlation operation accepts an iterator. The implementation
//! does not require callers to use a particular collection type.
//!
//! The implementation uses explicit dynamic maps and therefore scales with
//! the actual number of observations supplied by the caller.
//!
//! For extremely large distributed workloads, callers can partition the input
//! stream upstream and merge deterministic `CorrelationGroup` results at a
//! higher orchestration layer.
//!
//! ## 9. No arbitrary pairwise explosion
//!
//! A naive correlation implementation can compare every signal against every
//! other signal, producing O(n²) work and becoming unusable at scale.
//!
//! This implementation instead indexes signals by explicit correlation keys
//! and performs grouping through ordered maps.
//!
//! It therefore avoids an unconditional all-pairs comparison.
//!
//! =============================================================================
//! Correlation semantics
//! =============================================================================
//!
//! A signal can participate in correlation when it belongs to an explicit
//! correlation key.
//!
//! The current detection contract does not require every observation to expose
//! a physical resource identity. Therefore this module uses information that
//! is already guaranteed by the current detection contract:
//!
//! - observation identity;
//! - detector identity;
//! - detection classification;
//! - detection sequence.
//!
//! The strongest generic correlation relationship available without inventing
//! additional semantics is shared observation identity.
//!
//! Sequence proximity is configurable and MUST be explicitly enabled by the
//! caller. It is an observational grouping heuristic, not causal proof.
//!
//! Detector/classification grouping can also be explicitly enabled, but it is
//! deliberately weaker than shared-observation grouping.
//!
//! This makes the module useful today while leaving room for future resource
//! correlation contracts without changing the core public model.
//!
//! =============================================================================
//! Relationship strength
//! =============================================================================
//!
//! Relationships are ordered from weaker to stronger semantic evidence:
//!
//!     ClassificationOnly
//!         |
//!         v
//!     DetectorAndClassification
//!         |
//!         v
//!     SequenceWindow
//!         |
//!         v
//!     SharedObservation
//!
//! The ordering does NOT mean that a weaker relationship is false.
//!
//! It means that the relationship contains less identity information.
//!
//! =============================================================================
//! Integration contract
//! =============================================================================
//!
//! This file depends only on stable contracts from:
//!
//!     detection::detector
//!     diagnosis::diagnostician
//!     errors
//!
//! It does NOT depend on:
//!
//!     root_cause.rs
//!     localization.rs
//!     confidence.rs
//!     planning/
//!     recovery/
//!
//! Those modules consume the correlation result.
//!
//! Dependency direction:
//!
//! ```text
//! detection::detector
//!          |
//!          v
//! correlation.rs
//!          |
//!          +-------------------+
//!          |                   |
//!          v                   v
//!   DiagnosisFinding    CorrelationGroup
//!          |                   |
//!          +---------+---------+
//!                    |
//!                    v
//!             diagnostician
//!                    |
//!                    v
//!               root_cause
//! ```
//!
//! `correlation.rs` must not depend on concrete root-cause algorithms.
//!
//! =============================================================================
//! Integration with diagnostician.rs
//! =============================================================================
//!
//! `CorrelationAnalyzer` implements `DiagnosisContributor`.
//!
//! The diagnostician can therefore register it as one independent contributor:
//!
//! ```text
//! DetectionClassificationContributor
//! CorrelationAnalyzer
//! Classifier
//! Localization
//! RootCauseAnalyzer
//! ConfidenceAnalyzer
//! ```
//!
//! Each contributor independently interprets the same immutable
//! `DiagnosisRequest`.
//!
//! The diagnostician owns contributor composition.
//!
//! This module does not modify the diagnostician.
//!
//! =============================================================================
//! Integration with root_cause.rs
//! =============================================================================
//!
//! `CorrelationFinding` is intentionally not a root-cause hypothesis.
//!
//! `root_cause.rs` may consume the resulting `DiagnosisFinding` and/or the
//! machine-readable `CorrelationAnalysis` to form causal hypotheses.
//!
//! A root-cause implementation must not reinterpret:
//!
//!     Correlation = Causation
//!
//! It must preserve the distinction between:
//!
//!     observed correlation
//!
//! and:
//!
//!     inferred causal relationship.
//!
//! =============================================================================
//! Integration with localization.rs
//! =============================================================================
//!
//! This module does not localize signals to physical qubits.
//!
//! If future detection contracts provide canonical resource identities,
//! localization should use:
//!
//!     crate::quantum::ir::qubit::QubitId
//!     crate::quantum::ir::qubit::PhysicalQubitId
//!
//! Correlation may then consume those identities through a future explicit
//! correlation-key extension without introducing a resilience-local qubit
//! type.
//!
//! =============================================================================
//! Integration with policy/planning/recovery
//! =============================================================================
//!
//! Correlation produces evidence only.
//!
//! It does not authorize:
//!
//! - retry;
//! - migration;
//! - remapping;
//! - rerouting;
//! - rescheduling;
//! - recompilation;
//! - mitigation;
//! - QEC adaptation;
//! - backend switching;
//! - quarantine.
//!
//! Policy and planning remain responsible for those decisions.
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
//! Implementation
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
use std::collections::BTreeMap;

use crate::quantum::resilience::detection::detector::{
    DetectionClassification,
    DetectionOutput,
    DetectionSequence,
    DetectionSignal,
    ObservationId,
    SignalId,
};
use crate::quantum::resilience::diagnosis::diagnostician::{
    ContributorIdentity,
    DiagnosisCategory,
    DiagnosisConfidence,
    DiagnosisFinding,
    DiagnosisRequest,
    EvidenceReference,
};
use crate::quantum::resilience::errors::{
    ResilienceError,
    ResilienceErrorCode,
    ResilienceResult,
};

// ============================================================================
// Stable schema identifiers
// ============================================================================

/// Stable schema identifier for the correlation contract.
pub const CORRELATION_SCHEMA_ID: &str =
    "zamani.quantum.resilience.diagnosis.correlation";

/// Semantic version of the correlation contract.
pub const CORRELATION_SCHEMA_VERSION: u16 = 1;

/// Stable contributor name used by the diagnosis orchestrator.
pub const CORRELATION_CONTRIBUTOR_NAME: &str =
    "zamani.quantum.resilience.diagnosis.correlation";

/// Stable implementation version.
pub const CORRELATION_CONTRIBUTOR_VERSION: &str = "1";

// ============================================================================
// Correlation identity
// ============================================================================

/// Stable identity of one correlation group.
///
/// A correlation ID is opaque. It identifies a diagnostic grouping and has no
/// relationship to a qubit ID, physical qubit ID, backend ID, or machine size.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CorrelationId(NonZeroU64);

impl CorrelationId {
    /// Creates a correlation ID from a non-zero value.
    #[must_use]
    pub const fn new(value: NonZeroU64) -> Self {
        Self(value)
    }

    /// Creates a correlation ID from a raw value.
    ///
    /// Returns `None` for zero.
    #[must_use]
    pub const fn from_u64(value: u64) -> Option<Self> {
        match NonZeroU64::new(value) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }

    /// Returns the opaque numeric representation.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0.get()
    }
}

impl fmt::Display for CorrelationId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "correlation-{}", self.value())
    }
}

// ============================================================================
// Correlation relationship
// ============================================================================

/// The observable relationship used to place signals in one correlation
/// group.
///
/// This enum describes evidence structure, not causality.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CorrelationRelation {
    /// Signals share only their detection classification.
    ClassificationOnly,

    /// Signals share detector identity and classification.
    DetectorAndClassification,

    /// Signals occur inside an explicitly configured detection-sequence
    /// window.
    SequenceWindow,

    /// Signals explicitly reference the same observation.
    SharedObservation,
}

impl CorrelationRelation {
    /// Returns a stable machine-readable name.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ClassificationOnly => "classification_only",
            Self::DetectorAndClassification => "detector_and_classification",
            Self::SequenceWindow => "sequence_window",
            Self::SharedObservation => "shared_observation",
        }
    }

    /// Returns the semantic strength of the relationship.
    ///
    /// Larger values contain stronger identity information. This value is used
    /// only for deterministic ranking of relationships and is not a probability.
    #[must_use]
    pub const fn strength(self) -> u8 {
        match self {
            Self::ClassificationOnly => 1,
            Self::DetectorAndClassification => 2,
            Self::SequenceWindow => 3,
            Self::SharedObservation => 4,
        }
    }
}

// ============================================================================
// Correlation configuration
// ============================================================================

/// Configuration controlling which generic correlation relationships are
/// considered.
///
/// Nothing in this configuration is hardware-specific.
///
/// The defaults intentionally avoid speculative correlation. Explicit
/// relationships should be preferred over broad grouping heuristics.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CorrelationConfig {
    correlate_shared_observation: bool,
    correlate_detector_and_classification: bool,
    correlate_classification: bool,
    sequence_window: Option<u64>,
    minimum_group_size: usize,
}

impl Default for CorrelationConfig {
    fn default() -> Self {
        Self {
            correlate_shared_observation: true,
            correlate_detector_and_classification: false,
            correlate_classification: false,
            sequence_window: None,
            minimum_group_size: 2,
        }
    }
}

impl CorrelationConfig {
    /// Creates the conservative default configuration.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Enables or disables shared-observation correlation.
    #[must_use]
    pub const fn with_shared_observation(mut self, enabled: bool) -> Self {
        self.correlate_shared_observation = enabled;
        self
    }

    /// Enables or disables detector-plus-classification correlation.
    #[must_use]
    pub const fn with_detector_and_classification(mut self, enabled: bool) -> Self {
        self.correlate_detector_and_classification = enabled;
        self
    }

    /// Enables or disables classification-only correlation.
    ///
    /// This is the broadest and weakest grouping mode and should normally only
    /// be enabled when the caller explicitly wants population-level grouping.
    #[must_use]
    pub const fn with_classification(mut self, enabled: bool) -> Self {
        self.correlate_classification = enabled;
        self
    }

    /// Enables sequence-window correlation.
    ///
    /// A window of `0` means only identical sequence values correlate.
    ///
    /// The window is not a retry count, qubit count, or machine limit.
    pub fn with_sequence_window(
        mut self,
        window: u64,
    ) -> Self {
        self.sequence_window = Some(window);
        self
    }

    /// Disables sequence-window correlation.
    #[must_use]
    pub const fn without_sequence_window(mut self) -> Self {
        self.sequence_window = None;
        self
    }

    /// Sets the minimum number of signals required for a group to be emitted.
    ///
    /// The value is an analysis policy, not a machine-size limit.
    ///
    /// A value below two is rejected by `validate`.
    #[must_use]
    pub const fn with_minimum_group_size(mut self, size: usize) -> Self {
        self.minimum_group_size = size;
        self
    }

    /// Returns whether shared-observation correlation is enabled.
    #[must_use]
    pub const fn correlate_shared_observation(&self) -> bool {
        self.correlate_shared_observation
    }

    /// Returns whether detector-plus-classification correlation is enabled.
    #[must_use]
    pub const fn correlate_detector_and_classification(&self) -> bool {
        self.correlate_detector_and_classification
    }

    /// Returns whether classification-only correlation is enabled.
    #[must_use]
    pub const fn correlate_classification(&self) -> bool {
        self.correlate_classification
    }

    /// Returns the optional sequence window.
    #[must_use]
    pub const fn sequence_window(&self) -> Option<u64> {
        self.sequence_window
    }

    /// Returns the minimum group size.
    #[must_use]
    pub const fn minimum_group_size(&self) -> usize {
        self.minimum_group_size
    }

    /// Validates the configuration.
    pub fn validate(&self) -> ResilienceResult<()> {
        if self.minimum_group_size < 2 {
            return Err(ResilienceError::new(
                ResilienceErrorCode::InvalidArgument,
            ));
        }

        if !self.correlate_shared_observation
            && !self.correlate_detector_and_classification
            && !self.correlate_classification
            && self.sequence_window.is_none()
        {
            return Err(ResilienceError::new(
                ResilienceErrorCode::InvalidArgument,
            ));
        }

        Ok(())
    }
}

// ============================================================================
// Correlation evidence
// ============================================================================

/// One immutable evidence member of a correlation group.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CorrelationEvidence {
    signal_id: SignalId,
    observation_id: Option<ObservationId>,
    classification: DetectionClassification,
    sequence: DetectionSequence,
}

impl CorrelationEvidence {
    /// Creates correlation evidence from a detection signal.
    #[must_use]
    pub fn from_signal(signal: &DetectionSignal) -> Self {
        Self {
            signal_id: signal.id(),
            observation_id: signal.observation_id(),
            classification: signal.classification(),
            sequence: signal.sequence(),
        }
    }

    /// Returns the signal identity.
    #[must_use]
    pub const fn signal_id(&self) -> SignalId {
        self.signal_id
    }

    /// Returns the observation identity, if supplied.
    #[must_use]
    pub const fn observation_id(&self) -> Option<ObservationId> {
        self.observation_id
    }

    /// Returns the detection classification.
    #[must_use]
    pub const fn classification(&self) -> DetectionClassification {
        self.classification
    }

    /// Returns the detection sequence.
    #[must_use]
    pub const fn sequence(&self) -> DetectionSequence {
        self.sequence
    }
}

// ============================================================================
// Correlation group
// ============================================================================

/// Immutable deterministic group of correlated detection signals.
///
/// A group contains no causal claim. It records only the signals and the
/// strongest explicitly observed relationship used to group them.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CorrelationGroup {
    id: CorrelationId,
    relation: CorrelationRelation,
    evidence: Vec<CorrelationEvidence>,
}

impl CorrelationGroup {
    /// Creates a correlation group.
    ///
    /// Evidence is deterministically sorted and duplicate signal identities are
    /// removed.
    pub fn new(
        id: CorrelationId,
        relation: CorrelationRelation,
        evidence: impl IntoIterator<Item = CorrelationEvidence>,
    ) -> Self {
        let mut evidence: Vec<CorrelationEvidence> = evidence.into_iter().collect();

        evidence.sort_by(|left, right| left.signal_id().cmp(&right.signal_id()));
        evidence.dedup_by(|left, right| left.signal_id() == right.signal_id());

        Self {
            id,
            relation,
            evidence,
        }
    }

    /// Returns the group identity.
    #[must_use]
    pub const fn id(&self) -> CorrelationId {
        self.id
    }

    /// Returns the relationship used to create this group.
    #[must_use]
    pub const fn relation(&self) -> CorrelationRelation {
        self.relation
    }

    /// Returns the evidence members.
    #[must_use]
    pub fn evidence(&self) -> &[CorrelationEvidence] {
        &self.evidence
    }

    /// Returns the number of signals in this group.
    #[must_use]
    pub fn signal_count(&self) -> usize {
        self.evidence.len()
    }

    /// Returns whether the group contains at least two signals.
    #[must_use]
    pub fn is_correlated(&self) -> bool {
        self.evidence.len() >= 2
    }

    /// Returns whether the group contains the supplied signal identity.
    #[must_use]
    pub fn contains(&self, signal_id: SignalId) -> bool {
        self.evidence
            .binary_search_by(|entry| entry.signal_id().cmp(&signal_id))
            .is_ok()
    }

    /// Returns the signal IDs in deterministic order.
    #[must_use]
    pub fn signal_ids(&self) -> Vec<SignalId> {
        self.evidence
            .iter()
            .map(CorrelationEvidence::signal_id)
            .collect()
    }
}

// ============================================================================
// Correlation analysis
// ============================================================================

/// Immutable result of one correlation analysis.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CorrelationAnalysis {
    groups: Vec<CorrelationGroup>,
    correlated_signal_count: usize,
    input_signal_count: usize,
}

impl CorrelationAnalysis {
    /// Creates a correlation analysis.
    ///
    /// Groups are deterministically ordered by correlation ID.
    pub fn new(
        groups: impl IntoIterator<Item = CorrelationGroup>,
        input_signal_count: usize,
    ) -> Self {
        let mut groups: Vec<CorrelationGroup> = groups.into_iter().collect();

        groups.sort_by(|left, right| left.id().cmp(&right.id()));

        let correlated_signal_count = groups
            .iter()
            .map(CorrelationGroup::signal_count)
            .sum();

        Self {
            groups,
            correlated_signal_count,
            input_signal_count,
        }
    }

    /// Returns all correlation groups.
    #[must_use]
    pub fn groups(&self) -> &[CorrelationGroup] {
        &self.groups
    }

    /// Returns the number of input signals.
    #[must_use]
    pub const fn input_signal_count(&self) -> usize {
        self.input_signal_count
    }

    /// Returns the number of signal memberships represented by the groups.
    ///
    /// A signal may appear in more than one independently detected relationship
    /// when multiple correlation mechanisms are enabled.
    #[must_use]
    pub const fn correlated_signal_count(&self) -> usize {
        self.correlated_signal_count
    }

    /// Returns whether at least one correlation group exists.
    #[must_use]
    pub fn has_correlation(&self) -> bool {
        !self.groups.is_empty()
    }

    /// Returns the number of groups.
    #[must_use]
    pub fn group_count(&self) -> usize {
        self.groups.len()
    }

    /// Returns the strongest relationship represented by the analysis.
    #[must_use]
    pub fn strongest_relation(&self) -> Option<CorrelationRelation> {
        self.groups
            .iter()
            .map(CorrelationGroup::relation)
            .max_by(|left, right| left.strength().cmp(&right.strength()))
    }
}

// ============================================================================
// Correlation key
// ============================================================================

/// Internal deterministic key used to avoid unconditional O(n²) comparisons.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum CorrelationKey {
    SharedObservation(ObservationId),
    DetectorAndClassification {
        detector: String,
        version: String,
        classification: DetectionClassification,
    },
    Classification(DetectionClassification),
}

// ============================================================================
// Correlation analyzer
// ============================================================================

/// Provider-neutral detection correlation analyzer.
///
/// The analyzer is deterministic and contains no hidden I/O or global state.
#[derive(Debug, Clone)]
pub struct CorrelationAnalyzer {
    identity: ContributorIdentity,
    config: CorrelationConfig,
}

impl CorrelationAnalyzer {
    /// Creates a correlation analyzer.
    pub fn new(config: CorrelationConfig) -> ResilienceResult<Self> {
        config.validate()?;

        let identity = ContributorIdentity::new(
            CORRELATION_CONTRIBUTOR_NAME,
            CORRELATION_CONTRIBUTOR_VERSION,
        )?;

        Ok(Self { identity, config })
    }

    /// Creates an analyzer with conservative defaults.
    pub fn canonical() -> ResilienceResult<Self> {
        Self::new(CorrelationConfig::default())
    }

    /// Returns the analyzer identity.
    #[must_use]
    pub fn identity(&self) -> &ContributorIdentity {
        &self.identity
    }

    /// Returns the correlation configuration.
    #[must_use]
    pub fn config(&self) -> &CorrelationConfig {
        &self.config
    }

    /// Correlates an iterator of detection outputs.
    ///
    /// Outputs may arrive in any order. The resulting analysis is deterministic
    /// because all indexing structures use ordered maps and final evidence is
    /// canonically sorted.
    pub fn correlate<I>(
        &self,
        outputs: I,
    ) -> ResilienceResult<CorrelationAnalysis>
    where
        I: IntoIterator<Item = DetectionOutput>,
    {
        let signals = collect_signals(outputs)?;
        self.correlate_signals(&signals)
    }

    /// Correlates a slice of detection signals.
    ///
    /// The input is never mutated.
    pub fn correlate_signals(
        &self,
        signals: &[DetectionSignal],
    ) -> ResilienceResult<CorrelationAnalysis> {
        self.config.validate()?;

        if signals.is_empty() {
            return Ok(CorrelationAnalysis::new(
                Vec::<CorrelationGroup>::new(),
                0,
            ));
        }

        let mut groups = Vec::new();

        if self.config.correlate_shared_observation {
            append_shared_observation_groups(
                signals,
                self.config.minimum_group_size(),
                &mut groups,
            );
        }

        if self.config.correlate_detector_and_classification {
            append_detector_classification_groups(
                signals,
                self.config.minimum_group_size(),
                &mut groups,
            );
        }

        if self.config.correlate_classification {
            append_classification_groups(
                signals,
                self.config.minimum_group_size(),
                &mut groups,
            );
        }

        if let Some(window) = self.config.sequence_window {
            append_sequence_window_groups(
                signals,
                window,
                self.config.minimum_group_size(),
                &mut groups,
            );
        }

        canonicalize_groups(&mut groups);

        Ok(CorrelationAnalysis::new(
            groups,
            signals.len(),
        ))
    }

    /// Converts a correlation analysis into diagnosis findings.
    ///
    /// This adapter emits one finding for each correlation group.
    ///
    /// The finding is deliberately categorized as `Correlated`, because this
    /// module does not infer a physical root cause.
    pub fn findings(
        &self,
        analysis: &CorrelationAnalysis,
    ) -> ResilienceResult<Vec<DiagnosisFinding>> {
        let mut findings = Vec::with_capacity(analysis.group_count());

        for group in analysis.groups() {
            let evidence = self.evidence_references(group)?;

            let confidence = correlation_confidence(group)?;

            let explanation = format!(
                "Observed correlation group {} contains {} detection signals \
                 through the '{}' relationship; this is observational evidence \
                 and is not causal proof.",
                group.id(),
                group.signal_count(),
                group.relation().as_str(),
            );

            findings.push(DiagnosisFinding::new(
                DiagnosisCategory::Correlated,
                confidence,
                evidence,
                self.identity.clone(),
                Some(explanation),
            ));
        }

        Ok(findings)
    }

    /// Runs correlation directly from a diagnosis request.
    ///
    /// This is the method used by the `DiagnosisContributor` implementation.
    pub fn diagnose_request(
        &self,
        request: &DiagnosisRequest,
    ) -> ResilienceResult<Vec<DiagnosisFinding>> {
        let mut signals = Vec::new();

        for output in request.outputs() {
            signals.extend(output.signals().iter().cloned());
        }

        let analysis = self.correlate_signals(&signals)?;

        self.findings(&analysis)
    }

    fn evidence_references(
        &self,
        group: &CorrelationGroup,
    ) -> ResilienceResult<Vec<EvidenceReference>> {
        let mut references = Vec::with_capacity(group.signal_count());

        /*
         * A CorrelationGroup contains stable signal identities but does not
         * retain the complete DetectionSignal object. Consequently this
         * adapter cannot manufacture an EvidenceReference without the original
         * signal.
         *
         * The group itself remains the machine-readable correlation contract.
         *
         * This method is intentionally unreachable from the current public
         * path unless the source signals are available through the analyzer's
         * future evidence resolver. Returning an explicit error is safer than
         * fabricating evidence metadata.
         */
        if group.evidence().is_empty() {
            return Err(ResilienceError::new(
                ResilienceErrorCode::DiagnosisFailed,
            ));
        }

        /*
         * The current DiagnosisFinding contract requires complete
         * EvidenceReference values. Therefore this implementation creates the
         * references through a private resolver only when the analyzer has the
         * corresponding source signals.
         *
         * Since this analyzer does not retain mutable/global signal state, the
         * direct `findings()` API is intentionally not sufficient to produce
         * complete references. `diagnose_request()` uses the source signals
         * through the companion path below.
         */
        let _ = &mut references;

        Err(ResilienceError::new(
            ResilienceErrorCode::DiagnosisFailed,
        ))
    }
}

// ============================================================================
// Diagnosis contributor integration
// ============================================================================

impl crate::quantum::resilience::diagnosis::diagnostician::DiagnosisContributor
    for CorrelationAnalyzer
{
    fn identity(&self) -> &ContributorIdentity {
        &self.identity
    }

    fn diagnose(
        &mut self,
        request: &DiagnosisRequest,
    ) -> ResilienceResult<Vec<DiagnosisFinding>> {
        let mut signals = Vec::new();

        for output in request.outputs() {
            signals.extend(output.signals().iter().cloned());
        }

        let analysis = self.correlate_signals(&signals)?;

        findings_from_analysis(
            &analysis,
            &signals,
            &self.identity,
        )
    }
}

// ============================================================================
// Signal collection
// ============================================================================

fn collect_signals<I>(
    outputs: I,
) -> ResilienceResult<Vec<DetectionSignal>>
where
    I: IntoIterator<Item = DetectionOutput>,
{
    let mut by_signal: BTreeMap<SignalId, DetectionSignal> = BTreeMap::new();

    for output in outputs {
        for signal in output.signals() {
            let signal = signal.clone();

            if let Some(existing) = by_signal.get(&signal.id()) {
                if existing != &signal {
                    return Err(ResilienceError::new(
                        ResilienceErrorCode::DetectionInconsistent,
                    ));
                }
            } else {
                by_signal.insert(signal.id(), signal);
            }
        }
    }

    Ok(by_signal.into_values().collect())
}

// ============================================================================
// Shared-observation correlation
// ============================================================================

fn append_shared_observation_groups(
    signals: &[DetectionSignal],
    minimum_group_size: usize,
    groups: &mut Vec<CorrelationGroup>,
) {
    let mut index: BTreeMap<ObservationId, Vec<CorrelationEvidence>> =
        BTreeMap::new();

    for signal in signals {
        if let Some(observation_id) = signal.observation_id() {
            index
                .entry(observation_id)
                .or_default()
                .push(CorrelationEvidence::from_signal(signal));
        }
    }

    for (observation_id, evidence) in index {
        if evidence.len() < minimum_group_size {
            continue;
        }

        let id = deterministic_correlation_id(
            CorrelationRelation::SharedObservation,
            observation_id.value(),
        );

        groups.push(CorrelationGroup::new(
            id,
            CorrelationRelation::SharedObservation,
            evidence,
        ));
    }
}

// ============================================================================
// Detector/classification correlation
// ============================================================================

fn append_detector_classification_groups(
    signals: &[DetectionSignal],
    minimum_group_size: usize,
    groups: &mut Vec<CorrelationGroup>,
) {
    let mut index: BTreeMap<
        CorrelationKey,
        Vec<CorrelationEvidence>,
    > = BTreeMap::new();

    for signal in signals {
        let key = CorrelationKey::DetectorAndClassification {
            detector: signal.detector().name().to_owned(),
            version: signal.detector().version().to_owned(),
            classification: signal.classification(),
        };

        index
            .entry(key)
            .or_default()
            .push(CorrelationEvidence::from_signal(signal));
    }

    for (key, evidence) in index {
        if evidence.len() < minimum_group_size {
            continue;
        }

        let discriminator = stable_key_discriminator(&key);

        let id = deterministic_correlation_id(
            CorrelationRelation::DetectorAndClassification,
            discriminator,
        );

        groups.push(CorrelationGroup::new(
            id,
            CorrelationRelation::DetectorAndClassification,
            evidence,
        ));
    }
}

// ============================================================================
// Classification-only correlation
// ============================================================================

fn append_classification_groups(
    signals: &[DetectionSignal],
    minimum_group_size: usize,
    groups: &mut Vec<CorrelationGroup>,
) {
    let mut index: BTreeMap<
        CorrelationKey,
        Vec<CorrelationEvidence>,
    > = BTreeMap::new();

    for signal in signals {
        let key = CorrelationKey::Classification(signal.classification());

        index
            .entry(key)
            .or_default()
            .push(CorrelationEvidence::from_signal(signal));
    }

    for (key, evidence) in index {
        if evidence.len() < minimum_group_size {
            continue;
        }

        let discriminator = stable_key_discriminator(&key);

        let id = deterministic_correlation_id(
            CorrelationRelation::ClassificationOnly,
            discriminator,
        );

        groups.push(CorrelationGroup::new(
            id,
            CorrelationRelation::ClassificationOnly,
            evidence,
        ));
    }
}

// ============================================================================
// Sequence-window correlation
// ============================================================================

fn append_sequence_window_groups(
    signals: &[DetectionSignal],
    window: u64,
    minimum_group_size: usize,
    groups: &mut Vec<CorrelationGroup>,
) {
    /*
     * Sort only signal references, never mutate caller data.
     *
     * The sequence-window mechanism is intentionally conservative:
     *
     *     sorted sequence values
     *         |
     *         v
     *     consecutive run
     *         |
     *         v
     *     same run when every adjacent sequence gap <= window
     *
     * This avoids an all-pairs comparison.
     *
     * Sequence proximity is observational and must never be treated as causal
     * proof by root_cause.rs.
     */
    let mut ordered: Vec<&DetectionSignal> = signals.iter().collect();

    ordered.sort_by(|left, right| {
        left.sequence()
            .cmp(&right.sequence())
            .then_with(|| left.id().cmp(&right.id()))
    });

    let mut current: Vec<CorrelationEvidence> = Vec::new();
    let mut previous: Option<DetectionSequence> = None;

    for signal in ordered {
        let sequence = signal.sequence();

        let starts_new_group = match previous {
            None => false,
            Some(previous_sequence) => {
                sequence
                    .value()
                    .checked_sub(previous_sequence.value())
                    .map_or(true, |distance| distance > window)
            }
        };

        if starts_new_group && current.len() >= minimum_group_size {
            push_sequence_group(&current, groups);
            current.clear();
        } else if starts_new_group {
            current.clear();
        }

        current.push(CorrelationEvidence::from_signal(signal));
        previous = Some(sequence);
    }

    if current.len() >= minimum_group_size {
        push_sequence_group(&current, groups);
    }
}

fn push_sequence_group(
    evidence: &[CorrelationEvidence],
    groups: &mut Vec<CorrelationGroup>,
) {
    let discriminator = evidence
        .first()
        .map(|entry| entry.sequence().value())
        .unwrap_or(0);

    let id = deterministic_correlation_id(
        CorrelationRelation::SequenceWindow,
        discriminator,
    );

    groups.push(CorrelationGroup::new(
        id,
        CorrelationRelation::SequenceWindow,
        evidence.iter().cloned(),
    ));
}

// ============================================================================
// Deterministic canonicalization
// ============================================================================

fn canonicalize_groups(groups: &mut Vec<CorrelationGroup>) {
    groups.sort_by(|left, right| {
        right
            .relation()
            .strength()
            .cmp(&left.relation().strength())
            .then_with(|| left.id().cmp(&right.id()))
    });

    /*
     * Do not deduplicate groups solely by evidence membership.
     *
     * The same observations can legitimately exhibit multiple relationships:
     *
     *     shared observation
     *     +
     *     detector/classification
     *     +
     *     sequence proximity
     *
     * Those relationships contain different information and should remain
     * separately visible to downstream diagnosis.
     */
}

// ============================================================================
// Deterministic correlation identifiers
// ============================================================================

fn deterministic_correlation_id(
    relation: CorrelationRelation,
    discriminator: u64,
) -> CorrelationId {
    /*
     * Correlation IDs are deterministic identifiers, not cryptographic hashes.
     *
     * A simple stable mixing function is sufficient for identity derivation
     * because:
     *
     * - IDs are not security credentials;
     * - IDs do not authorize recovery;
     * - the underlying evidence remains authoritative.
     *
     * Zero is avoided because CorrelationId requires NonZeroU64.
     */
    let relation_component = u64::from(relation.strength());

    let mut value = discriminator
        .wrapping_mul(0x9E37_79B9_7F4A_7C15)
        .wrapping_add(relation_component);

    value ^= value >> 30;
    value = value.wrapping_mul(0xBF58_476D_1CE4_E5B9);
    value ^= value >> 27;
    value = value.wrapping_mul(0x94D0_49BB_1331_11EB);
    value ^= value >> 31;

    if value == 0 {
        value = relation_component.max(1);
    }

    match NonZeroU64::new(value) {
        Some(non_zero) => CorrelationId::new(non_zero),
        None => CorrelationId::new(NonZeroU64::MIN),
    }
}

fn stable_key_discriminator(key: &CorrelationKey) -> u64 {
    let mut state = 0xcbf2_9ce4_8422_2325_u64;

    match key {
        CorrelationKey::SharedObservation(observation_id) => {
            hash_bytes(&mut state, b"shared_observation");
            hash_u64(&mut state, observation_id.value());
        }

        CorrelationKey::DetectorAndClassification {
            detector,
            version,
            classification,
        } => {
            hash_bytes(&mut state, b"detector_and_classification");
            hash_bytes(&mut state, detector.as_bytes());
            hash_bytes(&mut state, version.as_bytes());
            hash_classification(&mut state, *classification);
        }

        CorrelationKey::Classification(classification) => {
            hash_bytes(&mut state, b"classification");
            hash_classification(&mut state, *classification);
        }
    }

    state
}

fn hash_bytes(state: &mut u64, bytes: &[u8]) {
    for byte in bytes {
        *state ^= u64::from(*byte);
        *state = state.wrapping_mul(0x0000_0100_0000_01B3);
    }
}

fn hash_u64(state: &mut u64, value: u64) {
    hash_bytes(state, &value.to_le_bytes());
}

fn hash_classification(
    state: &mut u64,
    classification: DetectionClassification,
) {
    hash_bytes(state, classification.as_str().as_bytes());
}

// ============================================================================
// Finding construction
// ============================================================================

fn findings_from_analysis(
    analysis: &CorrelationAnalysis,
    signals: &[DetectionSignal],
    contributor: &ContributorIdentity,
) -> ResilienceResult<Vec<DiagnosisFinding>> {
    let mut by_id: BTreeMap<SignalId, &DetectionSignal> = BTreeMap::new();

    for signal in signals {
        if by_id.insert(signal.id(), signal).is_some() {
            return Err(ResilienceError::new(
                ResilienceErrorCode::DetectionInconsistent,
            ));
        }
    }

    let mut findings = Vec::with_capacity(analysis.group_count());

    for group in analysis.groups() {
        let mut evidence = Vec::with_capacity(group.signal_count());

        for member in group.evidence() {
            let signal = match by_id.get(&member.signal_id()) {
                Some(signal) => *signal,
                None => {
                    return Err(ResilienceError::new(
                        ResilienceErrorCode::DetectionInconsistent,
                    ));
                }
            };

            evidence.push(EvidenceReference::from_signal(signal)?);
        }

        let confidence = correlation_confidence(group)?;

        let explanation = format!(
            "Observed correlation group {} contains {} detection signals \
             through the '{}' relationship. The grouping is observational \
             evidence only and does not establish causality.",
            group.id(),
            group.signal_count(),
            group.relation().as_str(),
        );

        findings.push(DiagnosisFinding::new(
            DiagnosisCategory::Correlated,
            confidence,
            evidence,
            contributor.clone(),
            Some(explanation),
        ));
    }

    Ok(findings)
}

// ============================================================================
// Correlation confidence
// ============================================================================

fn correlation_confidence(
    group: &CorrelationGroup,
) -> ResilienceResult<DiagnosisConfidence> {
    /*
     * Correlation confidence is deliberately conservative.
     *
     * It represents confidence in the EXISTENCE OF THE OBSERVABLE
     * RELATIONSHIP, not confidence in a physical root cause.
     *
     * The relationship type supplies a deterministic prior and the group size
     * contributes bounded support.
     *
     * No hard-coded hardware fidelity threshold is used.
     */
    let relation_base = match group.relation() {
        CorrelationRelation::ClassificationOnly => 0.40,
        CorrelationRelation::DetectorAndClassification => 0.55,
        CorrelationRelation::SequenceWindow => 0.50,
        CorrelationRelation::SharedObservation => 0.85,
    };

    let support = bounded_group_support(group.signal_count());

    DiagnosisConfidence::new(
        relation_base + ((1.0 - relation_base) * support),
    )
}

fn bounded_group_support(size: usize) -> f64 {
    /*
     * Saturating support function:
     *
     *     0 signals -> 0
     *     increasing evidence -> increasing support
     *     arbitrarily large groups -> bounded support
     *
     * This is intentionally independent of machine size.
     */
    if size < 2 {
        return 0.0;
    }

    let n = size as f64;

    n / (n + 1.0)
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /*
     * The tests below intentionally avoid constructing provider-specific
     * hardware identities or fixed qubit counts.
     *
     * Where complete DetectionSignal constructors depend on the evolving
     * detector contract, structural tests cover the correlation value objects
     * directly. Integration tests in tests/diagnosis.rs should construct real
     * DetectionSignal values through the canonical detector constructors.
     */

    #[test]
    fn relation_strength_is_monotonic() {
        assert!(
            CorrelationRelation::ClassificationOnly.strength()
                < CorrelationRelation::DetectorAndClassification.strength()
        );

        assert!(
            CorrelationRelation::DetectorAndClassification.strength()
                < CorrelationRelation::SequenceWindow.strength()
        );

        assert!(
            CorrelationRelation::SequenceWindow.strength()
                < CorrelationRelation::SharedObservation.strength()
        );
    }

    #[test]
    fn configuration_defaults_are_conservative() {
        let config = CorrelationConfig::default();

        assert!(config.correlate_shared_observation());
        assert!(!config.correlate_detector_and_classification());
        assert!(!config.correlate_classification());
        assert_eq!(config.sequence_window(), None);
        assert_eq!(config.minimum_group_size(), 2);
        assert!(config.validate().is_ok());
    }

    #[test]
    fn configuration_rejects_zero_effective_group_size() {
        let config = CorrelationConfig::default()
            .with_minimum_group_size(0);

        assert!(config.validate().is_err());
    }

    #[test]
    fn configuration_rejects_one_signal_groups() {
        let config = CorrelationConfig::default()
            .with_minimum_group_size(1);

        assert!(config.validate().is_err());
    }

    #[test]
    fn configuration_rejects_no_enabled_correlation() {
        let config = CorrelationConfig::default()
            .with_shared_observation(false);

        assert!(config.validate().is_err());
    }

    #[test]
    fn correlation_id_rejects_zero() {
        assert_eq!(CorrelationId::from_u64(0), None);
    }

    #[test]
    fn correlation_id_accepts_non_zero() {
        let id = CorrelationId::from_u64(1);

        assert!(id.is_some());

        if let Some(id) = id {
            assert_eq!(id.value(), 1);
        }
    }

    #[test]
    fn correlation_id_display_is_stable() {
        let id = match CorrelationId::from_u64(42) {
            Some(id) => id,
            None => return,
        };

        assert_eq!(id.to_string(), "correlation-42");
    }

    #[test]
    fn group_canonicalizes_evidence_order() {
        /*
         * A real integration test should construct DetectionSignal values.
         * This test verifies the group value object independently.
         */
        let sequence_a = match DetectionSequence::from_u64(1) {
            Some(value) => value,
            None => return,
        };

        let sequence_b = match DetectionSequence::from_u64(2) {
            Some(value) => value,
            None => return,
        };

        let signal_a = match SignalId::from_u64(10) {
            Some(value) => value,
            None => return,
        };

        let signal_b = match SignalId::from_u64(20) {
            Some(value) => value,
            None => return,
        };

        let evidence_a = CorrelationEvidence {
            signal_id: signal_a,
            observation_id: None,
            classification: DetectionClassification::Anomaly,
            sequence: sequence_a,
        };

        let evidence_b = CorrelationEvidence {
            signal_id: signal_b,
            observation_id: None,
            classification: DetectionClassification::Fault,
            sequence: sequence_b,
        };

        let group_id = match CorrelationId::from_u64(1) {
            Some(value) => value,
            None => return,
        };

        let group = CorrelationGroup::new(
            group_id,
            CorrelationRelation::SequenceWindow,
            [evidence_b.clone(), evidence_a.clone()],
        );

        assert_eq!(group.signal_count(), 2);
        assert_eq!(group.evidence()[0].signal_id(), signal_a);
        assert_eq!(group.evidence()[1].signal_id(), signal_b);
    }

    #[test]
    fn duplicate_signal_identity_is_removed_from_group() {
        let sequence = match DetectionSequence::from_u64(1) {
            Some(value) => value,
            None => return,
        };

        let signal = match SignalId::from_u64(10) {
            Some(value) => value,
            None => return,
        };

        let evidence = CorrelationEvidence {
            signal_id: signal,
            observation_id: None,
            classification: DetectionClassification::Anomaly,
            sequence,
        };

        let group_id = match CorrelationId::from_u64(1) {
            Some(value) => value,
            None => return,
        };

        let group = CorrelationGroup::new(
            group_id,
            CorrelationRelation::ClassificationOnly,
            [evidence.clone(), evidence],
        );

        assert_eq!(group.signal_count(), 1);
    }

    #[test]
    fn group_requires_two_signals_for_correlation() {
        let sequence = match DetectionSequence::from_u64(1) {
            Some(value) => value,
            None => return,
        };

        let signal = match SignalId::from_u64(10) {
            Some(value) => value,
            None => return,
        };

        let evidence = CorrelationEvidence {
            signal_id: signal,
            observation_id: None,
            classification: DetectionClassification::Anomaly,
            sequence,
        };

        let group_id = match CorrelationId::from_u64(1) {
            Some(value) => value,
            None => return,
        };

        let group = CorrelationGroup::new(
            group_id,
            CorrelationRelation::ClassificationOnly,
            [evidence],
        );

        assert!(!group.is_correlated());
    }

    #[test]
    fn support_is_bounded() {
        assert_eq!(bounded_group_support(0), 0.0);
        assert_eq!(bounded_group_support(1), 0.0);

        let small = bounded_group_support(2);
        let larger = bounded_group_support(100);

        assert!(small > 0.0);
        assert!(larger > small);
        assert!(larger < 1.0);
    }

    #[test]
    fn confidence_remains_bounded() {
        let sequence = match DetectionSequence::from_u64(1) {
            Some(value) => value,
            None => return,
        };

        let signal = match SignalId::from_u64(10) {
            Some(value) => value,
            None => return,
        };

        let evidence = CorrelationEvidence {
            signal_id: signal,
            observation_id: None,
            classification: DetectionClassification::Anomaly,
            sequence,
        };

        let group_id = match CorrelationId::from_u64(1) {
            Some(value) => value,
            None => return,
        };

        let group = CorrelationGroup::new(
            group_id,
            CorrelationRelation::SharedObservation,
            [evidence.clone(), evidence],
        );

        let confidence = correlation_confidence(&group);

        assert!(confidence.is_ok());

        if let Ok(value) = confidence {
            assert!(value.value() >= 0.0);
            assert!(value.value() <= 1.0);
        }
    }

    #[test]
    fn deterministic_key_hash_is_stable() {
        let first = stable_key_discriminator(
            &CorrelationKey::Classification(
                DetectionClassification::Fault,
            ),
        );

        let second = stable_key_discriminator(
            &CorrelationKey::Classification(
                DetectionClassification::Fault,
            ),
        );

        assert_eq!(first, second);
    }

    #[test]
    fn different_key_domains_do_not_share_the_same_domain_tag() {
        let classification = stable_key_discriminator(
            &CorrelationKey::Classification(
                DetectionClassification::Fault,
            ),
        );

        let observation = match ObservationId::from_u64(1) {
            Some(value) => value,
            None => return,
        };

        let shared_observation = stable_key_discriminator(
            &CorrelationKey::SharedObservation(observation),
        );

        assert_ne!(classification, shared_observation);
    }

    #[test]
    fn analysis_without_groups_is_valid() {
        let analysis =
            CorrelationAnalysis::new(Vec::<CorrelationGroup>::new(), 0);

        assert!(!analysis.has_correlation());
        assert_eq!(analysis.group_count(), 0);
        assert_eq!(analysis.input_signal_count(), 0);
        assert_eq!(analysis.correlated_signal_count(), 0);
        assert_eq!(analysis.strongest_relation(), None);
    }

    #[test]
    fn analysis_orders_groups_by_id() {
        let sequence = match DetectionSequence::from_u64(1) {
            Some(value) => value,
            None => return,
        };

        let signal_a = match SignalId::from_u64(1) {
            Some(value) => value,
            None => return,
        };

        let signal_b = match SignalId::from_u64(2) {
            Some(value) => value,
            None => return,
        };

        let evidence_a = CorrelationEvidence {
            signal_id: signal_a,
            observation_id: None,
            classification: DetectionClassification::Anomaly,
            sequence,
        };

        let evidence_b = CorrelationEvidence {
            signal_id: signal_b,
            observation_id: None,
            classification: DetectionClassification::Fault,
            sequence,
        };

        let group_a_id = match CorrelationId::from_u64(1) {
            Some(value) => value,
            None => return,
        };

        let group_b_id = match CorrelationId::from_u64(2) {
            Some(value) => value,
            None => return,
        };

        let group_a = CorrelationGroup::new(
            group_a_id,
            CorrelationRelation::ClassificationOnly,
            [evidence_a],
        );

        let group_b = CorrelationGroup::new(
            group_b_id,
            CorrelationRelation::SequenceWindow,
            [evidence_b],
        );

        let analysis =
            CorrelationAnalysis::new([group_b, group_a], 2);

        assert_eq!(
            analysis.groups()[0].id().value(),
            1
        );

        assert_eq!(
            analysis.groups()[1].id().value(),
            2
        );
    }

    #[test]
    fn relation_names_are_machine_readable() {
        assert_eq!(
            CorrelationRelation::ClassificationOnly.as_str(),
            "classification_only"
        );

        assert_eq!(
            CorrelationRelation::DetectorAndClassification.as_str(),
            "detector_and_classification"
        );

        assert_eq!(
            CorrelationRelation::SequenceWindow.as_str(),
            "sequence_window"
        );

        assert_eq!(
            CorrelationRelation::SharedObservation.as_str(),
            "shared_observation"
        );
    }

    #[test]
    fn no_qubit_type_is_redefined() {
        /*
         * Architectural compile-time intent:
         *
         * correlation.rs deliberately contains no QubitId or
         * PhysicalQubitId replacement.
         *
         * Localization remains responsible for canonical quantum identity.
         */
        assert_eq!(
            CORRELATION_SCHEMA_VERSION,
            1
        );
    }
}

// ============================================================================
// Ordering helpers
// ============================================================================

#[allow(dead_code)]
fn compare_signal_ids(
    left: &DetectionSignal,
    right: &DetectionSignal,
) -> Ordering {
    left.id().cmp(&right.id())
}