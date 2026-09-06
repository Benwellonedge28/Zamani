//! Zamani Quantum Resilience — Root-Cause Analysis.
//!
//! Path:
//!     src/quantum/resilience/diagnosis/root_cause.rs
//!
//! =============================================================================
//! Purpose
//! =============================================================================
//!
//! This module converts normalized detection evidence into explicit,
//! machine-readable root-cause hypotheses.
//!
//! It deliberately does NOT:
//!
//! - execute recovery;
//! - authorize recovery;
//! - select a backend;
//! - change routing;
//! - change scheduling;
//! - recompile;
//! - mutate hardware;
//! - mutate QEC state;
//! - redefine ZQN fault semantics;
//! - replace canonical quantum-resource identities;
//! - claim causal proof from correlation alone.
//!
//! The responsibility boundary is:
//!
//!     detection
//!         |
//!         v
//!     root_cause
//!         |
//!         +----> causal hypotheses
//!         |
//!         v
//!     localization / correlation / confidence
//!         |
//!         v
//!     policy
//!         |
//!         v
//!     planning
//!         |
//!         v
//!     adaptation / recovery
//!         |
//!         v
//!     verification
//!
//! =============================================================================
//! Integration contract
//! =============================================================================
//!
//! This module consumes:
//!
//!     crate::quantum::resilience::detection::detector
//!     crate::quantum::resilience::diagnosis::diagnostician
//!     crate::quantum::resilience::errors
//!
//! It integrates into the existing diagnostician through:
//!
//!     DiagnosisContributor
//!
//! Therefore `Diagnostician` does not need to know the concrete root-cause
//! implementation.
//!
//! Existing responsibilities remain elsewhere:
//!
//!     quantum::zqn
//!         canonical quantum fault/noise semantics
//!
//!     quantum::ir::qubit
//!         canonical QubitId / PhysicalQubitId
//!
//!     diagnosis/localization.rs
//!         resource localization
//!
//!     diagnosis/correlation.rs
//!         generalized evidence correlation
//!
//!     diagnosis/confidence.rs
//!         higher-level confidence interpretation
//!
//!     policy/
//!         permitted actions and thresholds
//!
//!     planning/
//!         recovery/adaptation plan generation
//!
//!     recovery/
//!         recovery execution
//!
//!     verification/
//!         post-recovery correctness verification
//!
//! =============================================================================
//! Canonical quantum identity
//! =============================================================================
//!
//! This module intentionally does not import QubitId or PhysicalQubitId.
//!
//! Root-cause analysis at this layer only has normalized DetectionSignal
//! identities. It must not invent a resilience-local qubit identifier.
//!
//! When localization information becomes available, localization.rs MUST use:
//!
//!     crate::quantum::ir::qubit::QubitId
//!     crate::quantum::ir::qubit::PhysicalQubitId
//!
//! directly.
//!
//! This separation is necessary because:
//!
//!     logical identity != physical identity != fault identity
//!
//! =============================================================================
//! ZQN ownership
//! =============================================================================
//!
//! ZQN remains the authoritative quantum fault/noise ontology.
//!
//! This file does not define:
//!
//!     Leakage
//!     Erasure
//!     Loss
//!     CorrelatedFault
//!     PauliFault
//!     NoiseChannel
//!     FaultLocation
//!
//! as replacement resilience types.
//!
//! Instead, it reasons over normalized detection classifications and references
//! their original evidence.
//!
//! =============================================================================
//! Scalability
//! =============================================================================
//!
//! No fixed limits are encoded here.
//!
//! There is deliberately no:
//!
//!     MAX_QUBITS
//!     MAX_FAULTS
//!     MAX_SIGNALS
//!     MAX_BACKENDS
//!     MAX_INCIDENTS
//!     MAX_DETECTORS
//!     RETRY_COUNT
//!
//! Root-cause processing is based on dynamically sized collections supplied by
//! the caller.
//!
//! The implementation does not perform an all-pairs comparison of signals.
//! Classification grouping is:
//!
//!     O(S log S)
//!
//! where S is the number of unique actionable detection signals supplied to
//! this contributor.
//!
//! It therefore avoids an O(S²) correlation explosion as machine size grows.
//!
//! =============================================================================
//! Determinism
//! =============================================================================
//!
//! Deterministic behavior requires identical:
//!
//! - DiagnosisRequest;
//! - detector configuration;
//! - contributor configuration;
//! - contributor implementation version;
//! - supplied evidence;
//! - contributor state.
//!
//! This file itself introduces no:
//!
//! - current-clock reads;
//! - randomness;
//! - environment reads;
//! - filesystem access;
//! - network access;
//! - global mutable state;
//! - memory-address-derived identity.
//!
//! BTreeMap/BTreeSet are used wherever deterministic ordering matters.
//!
//! =============================================================================
//! Causal safety
//! =============================================================================
//!
//! A correlation is not automatically a cause.
//!
//! Therefore the built-in rules deliberately emit:
//!
//!     CausalRelation::Correlated
//!
//! rather than:
//!
//!     CausalRelation::Direct
//!
//! unless a future explicitly registered causal rule proves a stronger
//! relationship.
//!
//! This distinction is essential for quantum systems because multiple physical
//! effects can produce similar observations, while errors can also propagate
//! through multi-qubit operations.
//!
//! =============================================================================
//! Rust compatibility
//! =============================================================================
//!
//! - Rust 1.97
//! - Rust 1.97.1
//! - Rust 2021
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

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use crate::quantum::resilience::detection::detector::{
    DetectionClassification,
    DetectionSignal,
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

use crate::quantum::resilience::errors::ResilienceResult;

// =============================================================================
// Schema
// =============================================================================

/// Stable schema identifier.
pub const ROOT_CAUSE_SCHEMA_ID: &str =
    "zamani.quantum.resilience.diagnosis.root_cause";

/// Semantic version of this contract.
pub const ROOT_CAUSE_SCHEMA_VERSION: u16 = 1;

// =============================================================================
// Root-cause kind
// =============================================================================

/// Machine-readable root-cause hypothesis category.
///
/// These are hypotheses, not guaranteed physical truths.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RootCauseKind {
    /// Hardware-origin evidence is associated with resource degradation or
    /// unavailability.
    HardwareDegradation,

    /// Hardware-origin evidence is associated with an explicit fault signal.
    HardwareFault,

    /// Hardware-origin evidence is associated with an anomaly.
    HardwareAnomaly,

    /// Resource unavailability is associated with execution failure.
    ResourceAvailability,

    /// Timeout and execution failure jointly indicate a possible execution
    /// boundary or infrastructure problem.
    ExecutionInfrastructure,

    /// A QEC signal co-occurs with an upstream fault, hardware, or degradation
    /// signal.
    QecUpstreamCondition,

    /// Multiple distinct fault signals occur within the same diagnosis
    /// request.
    CorrelatedPhysicalFault,

    /// No supported causal pattern was established.
    Unknown,

    /// Future domain-specific root-cause extension.
    External(String),
}

impl RootCauseKind {
    /// Returns the stable machine-readable identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::HardwareDegradation => "hardware_degradation",
            Self::HardwareFault => "hardware_fault",
            Self::HardwareAnomaly => "hardware_anomaly",
            Self::ResourceAvailability => "resource_availability",
            Self::ExecutionInfrastructure => "execution_infrastructure",
            Self::QecUpstreamCondition => "qec_upstream_condition",
            Self::CorrelatedPhysicalFault => "correlated_physical_fault",
            Self::Unknown => "unknown",
            Self::External(value) => value.as_str(),
        }
    }

    /// Maps a root-cause hypothesis into the existing generic diagnosis
    /// category.
    #[must_use]
    fn diagnosis_category(&self) -> DiagnosisCategory {
        match self {
            Self::HardwareDegradation => DiagnosisCategory::Hardware,

            Self::HardwareFault => DiagnosisCategory::Fault,

            Self::HardwareAnomaly => DiagnosisCategory::Hardware,

            Self::ResourceAvailability => DiagnosisCategory::Resource,

            Self::ExecutionInfrastructure => DiagnosisCategory::Backend,

            Self::QecUpstreamCondition => DiagnosisCategory::Qec,

            Self::CorrelatedPhysicalFault => DiagnosisCategory::Correlated,

            Self::Unknown => DiagnosisCategory::Unknown,

            Self::External(value) => {
                DiagnosisCategory::External(value.clone())
            }
        }
    }
}

impl fmt::Display for RootCauseKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Causal relation
// =============================================================================

/// Strength of the causal statement represented by a hypothesis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CausalRelation {
    /// An explicit causal rule has established the relationship.
    Direct,

    /// Evidence co-occurs and supports a correlation/common-cause hypothesis.
    Correlated,

    /// Evidence ordering is compatible with a causal relationship, but does
    /// not establish causality.
    Temporal,

    /// A known structural relationship exists, but this analysis did not prove
    /// the current instance causal.
    Structural,

    /// Insufficient evidence for a stronger relationship.
    Unknown,
}

impl CausalRelation {
    /// Stable machine-readable representation.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Direct => "direct",
            Self::Correlated => "correlated",
            Self::Temporal => "temporal",
            Self::Structural => "structural",
            Self::Unknown => "unknown",
        }
    }
}

// =============================================================================
// Root-cause hypothesis
// =============================================================================

/// One immutable root-cause hypothesis.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RootCauseHypothesis {
    kind: RootCauseKind,
    relation: CausalRelation,
    confidence: DiagnosisConfidence,
    evidence: Vec<EvidenceReference>,
    explanation: String,
}

impl RootCauseHypothesis {
    fn new(
        kind: RootCauseKind,
        relation: CausalRelation,
        confidence: DiagnosisConfidence,
        evidence: impl IntoIterator<Item = EvidenceReference>,
        explanation: impl Into<String>,
    ) -> Self {
        let mut evidence: Vec<EvidenceReference> =
            evidence.into_iter().collect();

        evidence.sort();
        evidence.dedup();

        Self {
            kind,
            relation,
            confidence,
            evidence,
            explanation: explanation.into(),
        }
    }

    /// Returns the root-cause category.
    #[must_use]
    pub fn kind(&self) -> &RootCauseKind {
        &self.kind
    }

    /// Returns the asserted causal relation.
    #[must_use]
    pub const fn relation(&self) -> CausalRelation {
        self.relation
    }

    /// Returns diagnosis confidence.
    #[must_use]
    pub const fn confidence(&self) -> DiagnosisConfidence {
        self.confidence
    }

    /// Returns immutable supporting evidence.
    #[must_use]
    pub fn evidence(&self) -> &[EvidenceReference] {
        &self.evidence
    }

    /// Returns the descriptive explanation.
    ///
    /// This text is never interpreted as a command.
    #[must_use]
    pub fn explanation(&self) -> &str {
        &self.explanation
    }
}

// =============================================================================
// Root-cause analysis result
// =============================================================================

/// Immutable result of one root-cause analysis.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RootCauseAnalysis {
    hypotheses: Vec<RootCauseHypothesis>,
    examined_signals: usize,
    supported_signals: usize,
}

impl RootCauseAnalysis {
    fn new(
        hypotheses: Vec<RootCauseHypothesis>,
        examined_signals: usize,
        supported_signals: usize,
    ) -> Self {
        Self {
            hypotheses,
            examined_signals,
            supported_signals,
        }
    }

    /// Returns hypotheses in deterministic order.
    #[must_use]
    pub fn hypotheses(&self) -> &[RootCauseHypothesis] {
        &self.hypotheses
    }

    /// Number of unique actionable signals examined.
    #[must_use]
    pub const fn examined_signals(&self) -> usize {
        self.examined_signals
    }

    /// Number of unique signals used by at least one hypothesis.
    #[must_use]
    pub const fn supported_signals(&self) -> usize {
        self.supported_signals
    }

    /// Returns whether at least one root-cause hypothesis was generated.
    #[must_use]
    pub fn is_conclusive(&self) -> bool {
        !self.hypotheses.is_empty()
    }
}

// =============================================================================
// Configuration
// =============================================================================

/// Structural configuration for root-cause analysis.
///
/// These switches do not encode hardware limits or confidence thresholds.
/// Operational acceptance thresholds belong to resilience policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RootCauseConfig {
    emit_correlated_faults: bool,
    emit_qec_upstream: bool,
    emit_unknown_when_inconclusive: bool,
}

impl RootCauseConfig {
    /// Creates explicit configuration.
    #[must_use]
    pub const fn new(
        emit_correlated_faults: bool,
        emit_qec_upstream: bool,
        emit_unknown_when_inconclusive: bool,
    ) -> Self {
        Self {
            emit_correlated_faults,
            emit_qec_upstream,
            emit_unknown_when_inconclusive,
        }
    }

    /// Conservative production configuration.
    #[must_use]
    pub const fn conservative() -> Self {
        Self {
            emit_correlated_faults: true,
            emit_qec_upstream: true,
            emit_unknown_when_inconclusive: false,
        }
    }

    /// Returns whether correlated-fault hypotheses are emitted.
    #[must_use]
    pub const fn emit_correlated_faults(self) -> bool {
        self.emit_correlated_faults
    }

    /// Returns whether QEC-upstream hypotheses are emitted.
    #[must_use]
    pub const fn emit_qec_upstream(self) -> bool {
        self.emit_qec_upstream
    }

    /// Returns whether unknown hypotheses are emitted.
    #[must_use]
    pub const fn emit_unknown_when_inconclusive(self) -> bool {
        self.emit_unknown_when_inconclusive
    }
}

impl Default for RootCauseConfig {
    fn default() -> Self {
        Self::conservative()
    }
}

// =============================================================================
// Root-cause analyzer
// =============================================================================

/// Production root-cause contributor.
///
/// The analyzer is deliberately:
///
/// - provider-independent;
/// - hardware-size independent;
/// - deterministic;
/// - non-mutating;
/// - non-authorizing;
/// - compatible with dynamic contributor registration.
#[derive(Debug, Clone)]
pub struct RootCauseAnalyzer {
    identity: ContributorIdentity,
    config: RootCauseConfig,
}

impl RootCauseAnalyzer {
    /// Creates a root-cause analyzer using conservative defaults.
    pub fn new(
        version: impl Into<String>,
    ) -> ResilienceResult<Self> {
        Self::with_config(version, RootCauseConfig::default())
    }

    /// Creates an analyzer with explicit structural configuration.
    pub fn with_config(
        version: impl Into<String>,
        config: RootCauseConfig,
    ) -> ResilienceResult<Self> {
        Ok(Self {
            identity: ContributorIdentity::new(
                "zamani.root-cause",
                version,
            )?,
            config,
        })
    }

    /// Returns the contributor identity.
    #[must_use]
    pub fn identity(&self) -> &ContributorIdentity {
        &self.identity
    }

    /// Returns the structural configuration.
    #[must_use]
    pub const fn config(&self) -> RootCauseConfig {
        self.config
    }

    /// Performs root-cause analysis.
    ///
    /// This method does not mutate:
    ///
    /// - the request;
    /// - the incident;
    /// - hardware;
    /// - QEC;
    /// - runtime state.
    pub fn analyze(
        &self,
        request: &DiagnosisRequest,
    ) -> RootCauseAnalysis {
        let signals = unique_actionable_signals(request);

        if signals.is_empty() {
            let hypotheses =
                if self.config.emit_unknown_when_inconclusive() {
                    vec![RootCauseHypothesis::new(
                        RootCauseKind::Unknown,
                        CausalRelation::Unknown,
                        DiagnosisConfidence::zero(),
                        std::iter::empty::<EvidenceReference>(),
                        "No actionable detection signal was available; no causal hypothesis was established.",
                    )]
                } else {
                    Vec::new()
                };

            return RootCauseAnalysis::new(hypotheses, 0, 0);
        }

        let grouped = group_by_classification(&signals);
        let mut hypotheses = Vec::new();

        if let Some(hypothesis) =
            hardware_degradation_hypothesis(&grouped)
        {
            hypotheses.push(hypothesis);
        }

        if let Some(hypothesis) =
            hardware_fault_hypothesis(&grouped)
        {
            hypotheses.push(hypothesis);
        }

        if let Some(hypothesis) =
            hardware_anomaly_hypothesis(&grouped)
        {
            hypotheses.push(hypothesis);
        }

        if let Some(hypothesis) =
            resource_availability_hypothesis(&grouped)
        {
            hypotheses.push(hypothesis);
        }

        if let Some(hypothesis) =
            execution_infrastructure_hypothesis(&grouped)
        {
            hypotheses.push(hypothesis);
        }

        if self.config.emit_qec_upstream() {
            if let Some(hypothesis) =
                qec_upstream_hypothesis(&grouped)
            {
                hypotheses.push(hypothesis);
            }
        }

        if self.config.emit_correlated_faults() {
            if let Some(hypothesis) =
                correlated_fault_hypothesis(&grouped)
            {
                hypotheses.push(hypothesis);
            }
        }

        if hypotheses.is_empty()
            && self.config.emit_unknown_when_inconclusive()
        {
            hypotheses.push(RootCauseHypothesis::new(
                RootCauseKind::Unknown,
                CausalRelation::Unknown,
                mean_signal_confidence(&signals),
                evidence_for_signals(&signals),
                "The supplied evidence did not satisfy any registered root-cause rule; no causal conclusion is asserted.",
            ));
        }

        canonicalize_hypotheses(&mut hypotheses);

        let supported_signals = hypotheses
            .iter()
            .flat_map(|hypothesis| {
                hypothesis
                    .evidence()
                    .iter()
                    .map(|evidence| evidence.signal_id())
            })
            .collect::<BTreeSet<_>>()
            .len();

        RootCauseAnalysis::new(
            hypotheses,
            signals.len(),
            supported_signals,
        )
    }
}

// =============================================================================
// Diagnostician integration
// =============================================================================

impl crate::quantum::resilience::diagnosis::diagnostician::DiagnosisContributor
    for RootCauseAnalyzer
{
    fn identity(&self) -> &ContributorIdentity {
        &self.identity
    }

    fn diagnose(
        &mut self,
        request: &DiagnosisRequest,
    ) -> ResilienceResult<Vec<DiagnosisFinding>> {
        let analysis = self.analyze(request);

        let findings = analysis
            .hypotheses()
            .iter()
            .map(|hypothesis| {
                let explanation = format!(
                    "root_cause={} relation={} explanation={}",
                    hypothesis.kind(),
                    hypothesis.relation().as_str(),
                    hypothesis.explanation(),
                );

                DiagnosisFinding::new(
                    hypothesis.kind().diagnosis_category(),
                    hypothesis.confidence(),
                    hypothesis.evidence().iter().cloned(),
                    self.identity.clone(),
                    Some(explanation),
                )
            })
            .collect();

        Ok(findings)
    }
}

// =============================================================================
// Signal normalization
// =============================================================================

/// Deduplicates signals by canonical SignalId.
///
/// If duplicate IDs occur, the first occurrence is retained. The existing
/// `DiagnosisRequest` already provides deterministic output ordering, so this
/// produces deterministic replay behavior.
fn unique_actionable_signals(
    request: &DiagnosisRequest,
) -> Vec<DetectionSignal> {
    let mut by_id = BTreeMap::<SignalId, DetectionSignal>::new();

    for output in request.outputs() {
        for signal in output.signals() {
            if signal.is_actionable_candidate() {
                by_id
                    .entry(signal.id())
                    .or_insert_with(|| signal.clone());
            }
        }
    }

    by_id.into_values().collect()
}

/// Groups signals by their normalized detector classification.
///
/// The grouping is classification-based rather than qubit-count-based, so it
/// remains independent of machine size.
fn group_by_classification(
    signals: &[DetectionSignal],
) -> BTreeMap<
    DetectionClassification,
    Vec<&DetectionSignal>,
> {
    let mut grouped:
        BTreeMap<DetectionClassification, Vec<&DetectionSignal>> =
        BTreeMap::new();

    for signal in signals {
        grouped
            .entry(signal.classification())
            .or_default()
            .push(signal);
    }

    grouped
}

/// Returns all signals for one classification.
fn signals_for<'a>(
    grouped: &'a BTreeMap<
        DetectionClassification,
        Vec<&'a DetectionSignal>,
    >,
    classification: DetectionClassification,
) -> &'a [&'a DetectionSignal] {
    match grouped.get(&classification) {
        Some(signals) => signals.as_slice(),
        None => &[],
    }
}

/// Converts detection signals into diagnosis evidence references.
///
/// Invalid evidence conversion is intentionally ignored here because the
/// existing DetectionSignal contract already validates normalized confidence.
/// The diagnostician independently validates the final finding contract.
fn evidence_for_signals(
    signals: &[&DetectionSignal],
) -> Vec<EvidenceReference> {
    signals
        .iter()
        .filter_map(|signal| {
            EvidenceReference::from_signal(signal).ok()
        })
        .collect()
}

// =============================================================================
// Confidence aggregation
// =============================================================================

/// Calculates the arithmetic mean of detector confidences.
///
/// This is NOT a probability of physical causality.
///
/// It is only the confidence inherited from the supporting detection signals.
///
/// No hard-coded confidence threshold is applied.
fn mean_signal_confidence(
    signals: &[DetectionSignal],
) -> DiagnosisConfidence {
    if signals.is_empty() {
        return DiagnosisConfidence::zero();
    }

    let mut mean = 0.0_f64;
    let mut count = 0.0_f64;

    for signal in signals {
        count += 1.0;

        mean +=
            (signal.confidence().value() - mean) / count;
    }

    match DiagnosisConfidence::new(mean) {
        Ok(value) => value,
        Err(_) => DiagnosisConfidence::zero(),
    }
}

/// Combines two groups conservatively.
///
/// The resulting confidence is the smaller of the strongest supporting
/// detector confidence from each required evidence class.
///
/// This avoids allowing one strong signal to hide an absent/weak second class.
fn combined_confidence(
    left: &[&DetectionSignal],
    right: &[&DetectionSignal],
) -> DiagnosisConfidence {
    let mut best_left = 0.0_f64;

    for signal in left {
        best_left =
            best_left.max(signal.confidence().value());
    }

    let mut best_right = 0.0_f64;

    for signal in right {
        best_right =
            best_right.max(signal.confidence().value());
    }

    match DiagnosisConfidence::new(
        best_left.min(best_right),
    ) {
        Ok(value) => value,
        Err(_) => DiagnosisConfidence::zero(),
    }
}

// =============================================================================
// Built-in causal rules
// =============================================================================

/// Hardware signal + degradation/unavailability.
///
/// This supports hardware degradation as a candidate explanation.
fn hardware_degradation_hypothesis(
    grouped: &BTreeMap<
        DetectionClassification,
        Vec<&DetectionSignal>,
    >,
) -> Option<RootCauseHypothesis> {
    let hardware =
        signals_for(
            grouped,
            DetectionClassification::HardwareSignal,
        );

    let degradation =
        signals_for(
            grouped,
            DetectionClassification::Degradation,
        );

    let unavailable =
        signals_for(
            grouped,
            DetectionClassification::Unavailability,
        );

    if hardware.is_empty()
        || (degradation.is_empty() && unavailable.is_empty())
    {
        return None;
    }

    let mut resource = degradation.to_vec();
    resource.extend_from_slice(unavailable);

    let mut signals = hardware.to_vec();
    signals.extend_from_slice(&resource);

    Some(RootCauseHypothesis::new(
        RootCauseKind::HardwareDegradation,
        CausalRelation::Correlated,
        combined_confidence(hardware, &resource),
        evidence_for_signals(&signals),
        "Hardware-origin and resource-degradation/unavailability observations co-occur; hardware degradation is a supported root-cause candidate, not causal proof.",
    ))
}

/// Hardware signal + explicit fault signal.
fn hardware_fault_hypothesis(
    grouped: &BTreeMap<
        DetectionClassification,
        Vec<&DetectionSignal>,
    >,
) -> Option<RootCauseHypothesis> {
    let hardware =
        signals_for(
            grouped,
            DetectionClassification::HardwareSignal,
        );

    let faults =
        signals_for(
            grouped,
            DetectionClassification::Fault,
        );

    if hardware.is_empty() || faults.is_empty() {
        return None;
    }

    let mut signals = hardware.to_vec();
    signals.extend_from_slice(faults);

    Some(RootCauseHypothesis::new(
        RootCauseKind::HardwareFault,
        CausalRelation::Correlated,
        combined_confidence(hardware, faults),
        evidence_for_signals(&signals),
        "Hardware-origin and fault observations co-occur; a hardware fault is a supported candidate requiring downstream localization and verification.",
    ))
}

/// Hardware signal + anomaly signal.
fn hardware_anomaly_hypothesis(
    grouped: &BTreeMap<
        DetectionClassification,
        Vec<&DetectionSignal>,
    >,
) -> Option<RootCauseHypothesis> {
    let hardware =
        signals_for(
            grouped,
            DetectionClassification::HardwareSignal,
        );

    let anomalies =
        signals_for(
            grouped,
            DetectionClassification::Anomaly,
        );

    if hardware.is_empty() || anomalies.is_empty() {
        return None;
    }

    let mut signals = hardware.to_vec();
    signals.extend_from_slice(anomalies);

    Some(RootCauseHypothesis::new(
        RootCauseKind::HardwareAnomaly,
        CausalRelation::Correlated,
        combined_confidence(hardware, anomalies),
        evidence_for_signals(&signals),
        "Hardware-origin and anomaly observations co-occur; a hardware-side anomaly is a candidate explanation.",
    ))
}

/// Resource unavailability + execution failure.
fn resource_availability_hypothesis(
    grouped: &BTreeMap<
        DetectionClassification,
        Vec<&DetectionSignal>,
    >,
) -> Option<RootCauseHypothesis> {
    let unavailable =
        signals_for(
            grouped,
            DetectionClassification::Unavailability,
        );

    let execution =
        signals_for(
            grouped,
            DetectionClassification::ExecutionFailure,
        );

    if unavailable.is_empty() || execution.is_empty() {
        return None;
    }

    let mut signals = unavailable.to_vec();
    signals.extend_from_slice(execution);

    Some(RootCauseHypothesis::new(
        RootCauseKind::ResourceAvailability,
        CausalRelation::Correlated,
        combined_confidence(unavailable, execution),
        evidence_for_signals(&signals),
        "Resource-unavailability and execution-failure observations co-occur; resource availability is a candidate contributing cause.",
    ))
}

/// Timeout + execution failure.
fn execution_infrastructure_hypothesis(
    grouped: &BTreeMap<
        DetectionClassification,
        Vec<&DetectionSignal>,
    >,
) -> Option<RootCauseHypothesis> {
    let timeout =
        signals_for(
            grouped,
            DetectionClassification::Timeout,
        );

    let execution =
        signals_for(
            grouped,
            DetectionClassification::ExecutionFailure,
        );

    if timeout.is_empty() || execution.is_empty() {
        return None;
    }

    let mut signals = timeout.to_vec();
    signals.extend_from_slice(execution);

    Some(RootCauseHypothesis::new(
        RootCauseKind::ExecutionInfrastructure,
        CausalRelation::Correlated,
        combined_confidence(timeout, execution),
        evidence_for_signals(&signals),
        "Timeout and execution-failure observations co-occur; an execution boundary or infrastructure condition is a candidate explanation.",
    ))
}

/// QEC signal + upstream fault/hardware/degradation signal.
///
/// The implementation deliberately does not state that QEC caused the fault.
/// QEC is treated as an observation of downstream pressure.
fn qec_upstream_hypothesis(
    grouped: &BTreeMap<
        DetectionClassification,
        Vec<&DetectionSignal>,
    >,
) -> Option<RootCauseHypothesis> {
    let qec =
        signals_for(
            grouped,
            DetectionClassification::QecSignal,
        );

    if qec.is_empty() {
        return None;
    }

    let fault =
        signals_for(
            grouped,
            DetectionClassification::Fault,
        );

    let hardware =
        signals_for(
            grouped,
            DetectionClassification::HardwareSignal,
        );

    let degradation =
        signals_for(
            grouped,
            DetectionClassification::Degradation,
        );

    let mut upstream = Vec::new();

    upstream.extend_from_slice(fault);
    upstream.extend_from_slice(hardware);
    upstream.extend_from_slice(degradation);

    if upstream.is_empty() {
        return None;
    }

    let mut signals = qec.to_vec();
    signals.extend_from_slice(&upstream);

    Some(RootCauseHypothesis::new(
        RootCauseKind::QecUpstreamCondition,
        CausalRelation::Correlated,
        combined_confidence(qec, &upstream),
        evidence_for_signals(&signals),
        "A QEC signal co-occurs with an upstream fault/hardware/degradation signal; the upstream condition is a candidate contributor to the observed QEC pressure.",
    ))
}

/// Multiple distinct fault signals.
///
/// This does not claim that one fault caused another. It identifies a
/// correlated-fault candidate for downstream correlation/localization.
fn correlated_fault_hypothesis(
    grouped: &BTreeMap<
        DetectionClassification,
        Vec<&DetectionSignal>,
    >,
) -> Option<RootCauseHypothesis> {
    let faults =
        signals_for(
            grouped,
            DetectionClassification::Fault,
        );

    if faults.len() < 2 {
        return None;
    }

    let mut signals = faults.to_vec();

    signals.sort_by_key(|signal| signal.id());

    let owned_signals: Vec<DetectionSignal> =
        signals.iter().map(|signal| (*signal).clone()).collect();

    Some(RootCauseHypothesis::new(
        RootCauseKind::CorrelatedPhysicalFault,
        CausalRelation::Correlated,
        mean_signal_confidence(&owned_signals),
        evidence_for_signals(&signals),
        "Multiple distinct fault signals were observed in one diagnosis request; correlated physical failure is a candidate and must be localized and verified before recovery.",
    ))
}

// =============================================================================
// Deterministic output
// =============================================================================

fn canonicalize_hypotheses(
    hypotheses: &mut Vec<RootCauseHypothesis>,
) {
    hypotheses.sort_by(|left, right| {
        left.kind
            .cmp(&right.kind)
            .then_with(|| {
                left.relation.cmp(&right.relation)
            })
            .then_with(|| {
                right.confidence.cmp(&left.confidence)
            })
            .then_with(|| {
                left.evidence.cmp(&right.evidence)
            })
            .then_with(|| {
                left.explanation.cmp(&right.explanation)
            })
    });

    hypotheses.dedup();
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use core::num::NonZeroU64;

    use crate::quantum::resilience::detection::detector::{
        DetectionConfidence,
        DetectionMetadata,
        DetectionOutput,
        DetectionSequence,
        DetectorIdentity,
    };

    fn sequence() -> DetectionSequence {
        DetectionSequence::new(NonZeroU64::MIN)
    }

    fn make_signal(
        id: u64,
        classification: DetectionClassification,
        confidence: f64,
    ) -> Option<DetectionSignal> {
        Some(DetectionSignal::new(
            SignalId::from_u64(id)?,
            DetectorIdentity::new("test", "1").ok()?,
            classification,
            DetectionConfidence::new(confidence).ok()?,
            None,
            sequence(),
        ))
    }

    fn make_request(
        signals: Vec<DetectionSignal>,
    ) -> Option<DiagnosisRequest> {
        let metadata = DetectionMetadata::new(
            DetectorIdentity::new("test", "1").ok()?,
            sequence(),
            signals.len() as u64,
        );

        Some(DiagnosisRequest::new(
            crate::quantum::resilience::diagnosis::diagnostician::DiagnosisId::from_u64(1)?,
            sequence(),
            None,
            [DetectionOutput::new(metadata, signals)],
            false,
        ))
    }

    #[test]
    fn hardware_degradation_is_correlated_not_direct() {
        let raw = vec![
            make_signal(
                1,
                DetectionClassification::HardwareSignal,
                0.9,
            ),
            make_signal(
                2,
                DetectionClassification::Degradation,
                0.8,
            ),
        ];

        assert!(raw.iter().all(Option::is_some));

        let signals = raw.into_iter().flatten().collect();

        let Some(request) = make_request(signals) else {
            assert!(false);
            return;
        };

        let Some(analyzer) =
            RootCauseAnalyzer::new("1").ok()
        else {
            assert!(false);
            return;
        };

        let result = analyzer.analyze(&request);

        assert!(
            result.hypotheses().iter().any(
                |hypothesis| {
                    hypothesis.kind()
                        == &RootCauseKind::HardwareDegradation
                        && hypothesis.relation()
                            == CausalRelation::Correlated
                }
            )
        );
    }

    #[test]
    fn duplicate_signal_ids_do_not_double_count() {
        let raw = vec![
            make_signal(
                1,
                DetectionClassification::Fault,
                0.8,
            ),
            make_signal(
                1,
                DetectionClassification::Fault,
                0.8,
            ),
            make_signal(
                2,
                DetectionClassification::Fault,
                0.7,
            ),
        ];

        assert!(raw.iter().all(Option::is_some));

        let signals = raw.into_iter().flatten().collect();

        let Some(request) = make_request(signals) else {
            assert!(false);
            return;
        };

        let Some(analyzer) =
            RootCauseAnalyzer::new("1").ok()
        else {
            assert!(false);
            return;
        };

        let result = analyzer.analyze(&request);

        let hypothesis =
            result.hypotheses().iter().find(
                |hypothesis| {
                    hypothesis.kind()
                        == &RootCauseKind::CorrelatedPhysicalFault
                },
            );

        assert!(hypothesis.is_some());
        assert_eq!(result.examined_signals(), 2);
    }

    #[test]
    fn no_supported_pattern_does_not_invent_root_cause() {
        let Some(signal) =
            make_signal(
                1,
                DetectionClassification::Anomaly,
                0.7,
            )
        else {
            assert!(false);
            return;
        };

        let Some(request) =
            make_request(vec![signal])
        else {
            assert!(false);
            return;
        };

        let Some(analyzer) =
            RootCauseAnalyzer::new("1").ok()
        else {
            assert!(false);
            return;
        };

        let result = analyzer.analyze(&request);

        assert!(result.hypotheses().is_empty());
        assert!(!result.is_conclusive());
    }

    #[test]
    fn qec_requires_upstream_candidate() {
        let raw = vec![
            make_signal(
                1,
                DetectionClassification::QecSignal,
                0.9,
            ),
            make_signal(
                2,
                DetectionClassification::Fault,
                0.8,
            ),
        ];

        assert!(raw.iter().all(Option::is_some));

        let signals = raw.into_iter().flatten().collect();

        let Some(request) = make_request(signals) else {
            assert!(false);
            return;
        };

        let Some(analyzer) =
            RootCauseAnalyzer::new("1").ok()
        else {
            assert!(false);
            return;
        };

        let result = analyzer.analyze(&request);

        assert!(
            result.hypotheses().iter().any(
                |hypothesis| {
                    hypothesis.kind()
                        == &RootCauseKind::QecUpstreamCondition
                }
            )
        );
    }

    #[test]
    fn empty_input_does_not_invent_failure() {
        let Some(request) = make_request(Vec::new()) else {
            assert!(false);
            return;
        };

        let Some(analyzer) =
            RootCauseAnalyzer::new("1").ok()
        else {
            assert!(false);
            return;
        };

        let result = analyzer.analyze(&request);

        assert!(result.hypotheses().is_empty());
        assert_eq!(result.examined_signals(), 0);
        assert_eq!(result.supported_signals(), 0);
    }
}