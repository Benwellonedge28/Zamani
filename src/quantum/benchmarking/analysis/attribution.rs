//! Zamani Quantum Benchmarking — Attribution Analysis
//!
//! # Purpose
//!
//! This module provides deterministic, backend-independent attribution analysis
//! for quantum benchmark performance changes.
//!
//! Attribution answers:
//!
//! ```text
//! "What measurable factors are associated with the observed performance
//!  change, and how much of the observed change can be accounted for by
//!  those factors?"
//! ```
//!
//! It deliberately does NOT claim causality merely because two metrics changed
//! together. Production attribution must distinguish:
//!
//! - observed change;
//! - normalized contribution;
//! - evidence strength;
//! - association;
//! - experimentally established causation.
//!
//! The default attribution produced by this module is therefore an
//! `Association` claim. A caller may explicitly provide controlled-experiment
//! evidence to establish a `Causal` claim.
//!
//! # Architectural boundary
//!
//! This module is a pure analysis layer.
//!
//! It does NOT:
//!
//! - execute circuits;
//! - access hardware;
//! - access a simulator;
//! - compile circuits;
//! - perform routing;
//! - perform scheduling;
//! - perform calibration;
//! - mutate benchmark results;
//! - read process-global state;
//! - perform network I/O;
//! - print diagnostics;
//! - depend on Quantum IR implementation details;
//! - depend on a particular hardware provider;
//! - assume that all quantum systems are gate-model systems.
//!
//! The intended dependency direction is:
//!
//! ```text
//! benchmark result / baseline
//!          │
//!          ▼
//!   analysis::attribution
//!          │
//!          ├── observed metric changes
//!          ├── evidence
//!          ├── contribution model
//!          └── attribution confidence
//!          │
//!          ▼
//! analysis::diagnosis / reporting / regression
//! ```
//!
//! # Important scientific limitation
//!
//! Attribution is not automatically causal inference.
//!
//! For example:
//!
//! ```text
//! two-qubit error increased
//! benchmark fidelity decreased
//! ```
//!
//! does not, by itself, prove that the increase in two-qubit error caused the
//! fidelity decrease. Both could have been caused by calibration drift.
//!
//! Therefore this module exposes explicit evidence and claim-strength types.
//!
//! # Supported attribution sources
//!
//! The framework can attribute benchmark changes to:
//!
//! - compilation;
//! - routing;
//! - scheduling;
//! - gate error;
//! - two-qubit gate error;
//! - readout error;
//! - coherence;
//! - crosstalk;
//! - calibration drift;
//! - queue latency;
//! - execution latency;
//! - classical processing;
//! - resource growth;
//! - circuit depth;
//! - circuit width;
//! - leakage;
//! - error correction;
//! - decoding;
//! - user-defined factors.
//!
//! The list is intentionally extensible.
//!
//! # Determinism
//!
//! Attribution is deterministic given:
//!
//! - the baseline snapshot;
//! - the current snapshot;
//! - factor specifications;
//! - evidence supplied by the caller.
//!
//! No random number generator is used.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97
//! - Rust 1.97.1
//! - Rust 2021
//!
//! No nightly features are required.
//!
//! # Integration contract
//!
//! Future modules can consume this file without changing it:
//!
//! ```text
//! analysis::baseline
//!       │
//!       ▼
//! MetricSnapshot
//!       │
//!       ├───────────────┐
//!       ▼               ▼
//! current result     baseline result
//!       │               │
//!       └──────┬────────┘
//!              ▼
//!      AttributionAnalyzer
//!              │
//!              ▼
//!      AttributionReport
//!              │
//!       ┌──────┴────────┐
//!       ▼               ▼
//! diagnosis          reporting
//! ```
//!
//! `analysis::diagnosis.rs` may interpret the resulting findings.
//!
//! `analysis::baseline.rs` may provide the baseline snapshot.
//!
//! `analysis::compare.rs` may construct the metric deltas.
//!
//! `reporting::*` may serialize the attribution report.
//!
//! None of those modules are required to compile this file.

use std::cmp::Ordering;
use std::error::Error;
use std::fmt;

// ============================================================================
// Public constants
// ============================================================================

/// Stable module identifier.
pub const ATTRIBUTION_ANALYSIS_ID: &str = "quantum_benchmark_attribution";

/// Version of this attribution contract.
pub const ATTRIBUTION_SCHEMA_VERSION: u32 = 1;

/// Default minimum absolute metric change required before a factor is treated
/// as materially changed.
///
/// This is deliberately small because individual metrics can be normalized
/// differently. The value is only a default and must not be interpreted as a
/// universal scientific threshold.
pub const DEFAULT_MIN_MATERIAL_CHANGE: f64 = 1.0e-12;

/// Default minimum contribution magnitude below which a finding is considered
/// negligible.
pub const DEFAULT_MIN_CONTRIBUTION: f64 = 1.0e-12;

/// Default confidence assigned to a direct observed association when the caller
/// has supplied valid before/after measurements but no controlled experiment.
pub const DEFAULT_OBSERVATIONAL_CONFIDENCE: f64 = 0.5;

/// Maximum supported confidence value.
pub const MAX_CONFIDENCE: f64 = 1.0;

/// Minimum supported confidence value.
pub const MIN_CONFIDENCE: f64 = 0.0;

// ============================================================================
// Error handling
// ============================================================================

/// Errors returned by attribution analysis.
#[derive(Debug, Clone, PartialEq)]
pub enum AttributionError {
    /// A metric name is empty.
    EmptyMetricName,

    /// A factor identifier is empty.
    EmptyFactorId,

    /// A factor description is empty.
    EmptyFactorDescription,

    /// A numeric value was not finite.
    NonFiniteValue {
        field: &'static str,
        value: f64,
    },

    /// A normalized value is invalid.
    InvalidNormalizedValue {
        field: &'static str,
        value: f64,
    },

    /// A confidence value is outside [0, 1].
    InvalidConfidence {
        value: f64,
    },

    /// A contribution weight is invalid.
    InvalidWeight {
        value: f64,
    },

    /// No metrics were supplied.
    EmptyMetricSnapshot,

    /// No attribution factors were supplied.
    EmptyFactorSet,

    /// A factor refers to a metric that does not exist.
    MissingMetric {
        metric: String,
        factor_id: String,
    },

    /// A duplicate metric was supplied.
    DuplicateMetric {
        metric: String,
    },

    /// A duplicate factor ID was supplied.
    DuplicateFactor {
        factor_id: String,
    },

    /// The requested attribution target does not exist.
    MissingTargetMetric {
        metric: String,
    },

    /// A target metric cannot be attributed because its baseline value is zero
    /// and relative change was requested.
    ZeroBaselineForRelativeChange {
        metric: String,
    },

    /// A metric's direction is inconsistent with its specification.
    InvalidDirection,

    /// An attribution model cannot be computed.
    InvalidModel {
        message: String,
    },

    /// A result contains an impossible value.
    InvalidResult {
        message: String,
    },
}

impl fmt::Display for AttributionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyMetricName => {
                write!(formatter, "metric name cannot be empty")
            }

            Self::EmptyFactorId => {
                write!(formatter, "attribution factor ID cannot be empty")
            }

            Self::EmptyFactorDescription => {
                write!(formatter, "attribution factor description cannot be empty")
            }

            Self::NonFiniteValue { field, value } => {
                write!(
                    formatter,
                    "{} must be finite, got {}",
                    field,
                    value
                )
            }

            Self::InvalidNormalizedValue { field, value } => {
                write!(
                    formatter,
                    "{} must be finite and non-negative, got {}",
                    field,
                    value
                )
            }

            Self::InvalidConfidence { value } => {
                write!(
                    formatter,
                    "confidence must be in [0, 1], got {}",
                    value
                )
            }

            Self::InvalidWeight { value } => {
                write!(
                    formatter,
                    "attribution weight must be finite and non-negative, got {}",
                    value
                )
            }

            Self::EmptyMetricSnapshot => {
                write!(formatter, "metric snapshot cannot be empty")
            }

            Self::EmptyFactorSet => {
                write!(formatter, "attribution factor set cannot be empty")
            }

            Self::MissingMetric { metric, factor_id } => {
                write!(
                    formatter,
                    "factor '{}' references missing metric '{}'",
                    factor_id,
                    metric
                )
            }

            Self::DuplicateMetric { metric } => {
                write!(formatter, "duplicate metric '{}'", metric)
            }

            Self::DuplicateFactor { factor_id } => {
                write!(formatter, "duplicate attribution factor '{}'", factor_id)
            }

            Self::MissingTargetMetric { metric } => {
                write!(
                    formatter,
                    "target metric '{}' is missing",
                    metric
                )
            }

            Self::ZeroBaselineForRelativeChange { metric } => {
                write!(
                    formatter,
                    "cannot calculate relative change for '{}' because baseline is zero",
                    metric
                )
            }

            Self::InvalidDirection => {
                write!(formatter, "invalid metric direction")
            }

            Self::InvalidModel { message } => {
                write!(formatter, "invalid attribution model: {}", message)
            }

            Self::InvalidResult { message } => {
                write!(formatter, "invalid attribution result: {}", message)
            }
        }
    }
}

impl Error for AttributionError {}

// ============================================================================
// Metric identity
// ============================================================================

/// Stable identifier for a benchmark metric.
///
/// The string is intentionally not restricted to a fixed enumeration so that
/// future benchmark families and user-defined Zamani metrics do not require
/// changes to this file.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MetricId(String);

impl MetricId {
    /// Create a metric identifier.
    pub fn new(value: impl Into<String>) -> Result<Self, AttributionError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(AttributionError::EmptyMetricName);
        }

        Ok(Self(value))
    }

    /// Return the identifier as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consume the identifier and return its string.
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for MetricId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// ============================================================================
// Metric direction
// ============================================================================

/// Indicates whether larger or smaller values are better for a metric.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetricDirection {
    /// Larger values represent better performance.
    HigherIsBetter,

    /// Smaller values represent better performance.
    LowerIsBetter,

    /// Neither direction is intrinsically better.
    Neutral,
}

impl MetricDirection {
    /// Return whether an increase represents degradation.
    pub fn increase_is_degradation(self) -> bool {
        matches!(self, Self::LowerIsBetter)
    }

    /// Return whether a decrease represents degradation.
    pub fn decrease_is_degradation(self) -> bool {
        matches!(self, Self::HigherIsBetter)
    }
}

// ============================================================================
// Metric value
// ============================================================================

/// A single measured benchmark metric.
#[derive(Debug, Clone, PartialEq)]
pub struct MetricValue {
    /// Stable metric identifier.
    pub id: MetricId,

    /// Numeric value.
    pub value: f64,

    /// Optional unit.
    ///
    /// Examples:
    ///
    /// - `"probability"`
    /// - `"seconds"`
    /// - `"nanoseconds"`
    /// - `"qubits"`
    /// - `"gates"`
    /// - `"percent"`
    pub unit: Option<String>,

    /// Interpretation direction.
    pub direction: MetricDirection,
}

impl MetricValue {
    /// Construct a metric value.
    pub fn new(
        id: MetricId,
        value: f64,
        direction: MetricDirection,
    ) -> Result<Self, AttributionError> {
        if !value.is_finite() {
            return Err(AttributionError::NonFiniteValue {
                field: "metric value",
                value,
            });
        }

        Ok(Self {
            id,
            value,
            unit: None,
            direction,
        })
    }

    /// Set a unit.
    pub fn with_unit(mut self, unit: impl Into<String>) -> Self {
        self.unit = Some(unit.into());
        self
    }
}

// ============================================================================
// Metric snapshot
// ============================================================================

/// Immutable collection of metrics at one point in a benchmark comparison.
#[derive(Debug, Clone, PartialEq)]
pub struct MetricSnapshot {
    /// Human-readable snapshot identifier.
    pub id: String,

    /// Metric values.
    pub metrics: Vec<MetricValue>,
}

impl MetricSnapshot {
    /// Create an empty snapshot.
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            metrics: Vec::new(),
        }
    }

    /// Create a snapshot from a collection of metrics.
    pub fn from_metrics<I>(
        id: impl Into<String>,
        metrics: I,
    ) -> Result<Self, AttributionError>
    where
        I: IntoIterator<Item = MetricValue>,
    {
        let snapshot = Self {
            id: id.into(),
            metrics: metrics.into_iter().collect(),
        };

        snapshot.validate()?;

        Ok(snapshot)
    }

    /// Add one metric.
    ///
    /// Validation is performed immediately.
    pub fn push(&mut self, metric: MetricValue) -> Result<(), AttributionError> {
        if self.metrics.iter().any(|existing| existing.id == metric.id) {
            return Err(AttributionError::DuplicateMetric {
                metric: metric.id.to_string(),
            });
        }

        self.metrics.push(metric);

        Ok(())
    }

    /// Validate the snapshot.
    pub fn validate(&self) -> Result<(), AttributionError> {
        if self.metrics.is_empty() {
            return Err(AttributionError::EmptyMetricSnapshot);
        }

        for metric in &self.metrics {
            if !metric.value.is_finite() {
                return Err(AttributionError::NonFiniteValue {
                    field: "metric value",
                    value: metric.value,
                });
            }
        }

        for left in 0..self.metrics.len() {
            for right in (left + 1)..self.metrics.len() {
                if self.metrics[left].id == self.metrics[right].id {
                    return Err(AttributionError::DuplicateMetric {
                        metric: self.metrics[left].id.to_string(),
                    });
                }
            }
        }

        Ok(())
    }

    /// Look up a metric.
    pub fn get(&self, id: &MetricId) -> Option<&MetricValue> {
        self.metrics.iter().find(|metric| &metric.id == id)
    }

    /// Look up a metric by string identifier.
    pub fn get_by_name(&self, name: &str) -> Option<&MetricValue> {
        self.metrics
            .iter()
            .find(|metric| metric.id.as_str() == name)
    }
}

// ============================================================================
// Metric delta
// ============================================================================

/// Difference between a baseline and current metric.
#[derive(Debug, Clone, PartialEq)]
pub struct MetricDelta {
    /// Metric identifier.
    pub metric: MetricId,

    /// Baseline value.
    pub baseline: f64,

    /// Current value.
    pub current: f64,

    /// Absolute change: `current - baseline`.
    pub absolute_change: f64,

    /// Relative change when defined.
    ///
    /// ```text
    /// (current - baseline) / abs(baseline)
    /// ```
    ///
    /// `None` is used when the baseline is zero.
    pub relative_change: Option<f64>,

    /// Whether the observed change is a degradation according to the metric
    /// direction.
    pub is_degradation: bool,

    /// Metric direction.
    pub direction: MetricDirection,
}

impl MetricDelta {
    /// Calculate a delta between two metric values.
    pub fn between(
        baseline: &MetricValue,
        current: &MetricValue,
    ) -> Result<Self, AttributionError> {
        if baseline.id != current.id {
            return Err(AttributionError::InvalidModel {
                message: format!(
                    "cannot compare different metrics '{}' and '{}'",
                    baseline.id,
                    current.id
                ),
            });
        }

        if !baseline.value.is_finite() {
            return Err(AttributionError::NonFiniteValue {
                field: "baseline metric value",
                value: baseline.value,
            });
        }

        if !current.value.is_finite() {
            return Err(AttributionError::NonFiniteValue {
                field: "current metric value",
                value: current.value,
            });
        }

        let absolute_change = current.value - baseline.value;

        let relative_change = if baseline.value == 0.0 {
            None
        } else {
            Some(absolute_change / baseline.value.abs())
        };

        let is_degradation = match baseline.direction {
            MetricDirection::HigherIsBetter => absolute_change < 0.0,
            MetricDirection::LowerIsBetter => absolute_change > 0.0,
            MetricDirection::Neutral => false,
        };

        Ok(Self {
            metric: baseline.id.clone(),
            baseline: baseline.value,
            current: current.value,
            absolute_change,
            relative_change,
            is_degradation,
            direction: baseline.direction,
        })
    }

    /// Return the magnitude of the relative change.
    ///
    /// If the baseline is zero, the absolute magnitude is returned instead.
    pub fn change_magnitude(&self) -> f64 {
        self.relative_change
            .map(f64::abs)
            .unwrap_or_else(|| self.absolute_change.abs())
    }

    /// Return whether the change is materially different from zero.
    pub fn is_material(&self, threshold: f64) -> bool {
        self.change_magnitude() > threshold
    }
}

// ============================================================================
// Attribution source
// ============================================================================

/// Known attribution domains.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum AttributionSource {
    /// Compiler/lowering overhead.
    Compilation,

    /// Circuit routing/transpilation.
    Routing,

    /// Instruction scheduling.
    Scheduling,

    /// Single-qubit gate quality.
    GateError,

    /// Two-qubit gate quality.
    TwoQubitGateError,

    /// Measurement/readout quality.
    ReadoutError,

    /// State-preparation and measurement effects.
    Spam,

    /// Coherence limitation.
    Coherence,

    /// Crosstalk.
    Crosstalk,

    /// Hardware calibration drift.
    CalibrationDrift,

    /// Queue latency.
    QueueLatency,

    /// Circuit execution latency.
    ExecutionLatency,

    /// Classical post-processing.
    ClassicalProcessing,

    /// Circuit width growth.
    CircuitWidth,

    /// Circuit depth growth.
    CircuitDepth,

    /// Leakage from the computational subspace.
    Leakage,

    /// Error-correction overhead or failure.
    ErrorCorrection,

    /// Decoder behavior.
    Decoding,

    /// Physical-to-logical resource overhead.
    LogicalOverhead,

    /// Generic resource consumption.
    ResourceGrowth,

    /// User-defined attribution domain.
    Custom(String),
}

impl AttributionSource {
    /// Stable identifier.
    pub fn as_str(&self) -> &str {
        match self {
            Self::Compilation => "compilation",
            Self::Routing => "routing",
            Self::Scheduling => "scheduling",
            Self::GateError => "gate_error",
            Self::TwoQubitGateError => "two_qubit_gate_error",
            Self::ReadoutError => "readout_error",
            Self::Spam => "spam",
            Self::Coherence => "coherence",
            Self::Crosstalk => "crosstalk",
            Self::CalibrationDrift => "calibration_drift",
            Self::QueueLatency => "queue_latency",
            Self::ExecutionLatency => "execution_latency",
            Self::ClassicalProcessing => "classical_processing",
            Self::CircuitWidth => "circuit_width",
            Self::CircuitDepth => "circuit_depth",
            Self::Leakage => "leakage",
            Self::ErrorCorrection => "error_correction",
            Self::Decoding => "decoding",
            Self::LogicalOverhead => "logical_overhead",
            Self::ResourceGrowth => "resource_growth",
            Self::Custom(value) => value.as_str(),
        }
    }
}

impl fmt::Display for AttributionSource {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// Evidence semantics
// ============================================================================

/// Strength/type of evidence supporting an attribution.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvidenceType {
    /// The factor and target changed together in observational data.
    Observational,

    /// The factor is supported by a controlled comparison.
    ControlledComparison,

    /// The factor was isolated through a controlled experiment.
    ControlledExperiment,

    /// A mechanistic model predicts the observed change and has been
    /// experimentally validated.
    Mechanistic,

    /// Evidence is insufficient to support an attribution.
    Insufficient,
}

impl EvidenceType {
    /// Whether this evidence is sufficient to permit a causal claim.
    ///
    /// Even controlled evidence should only be interpreted as causal when the
    /// caller explicitly marks the evidence as causal through
    /// `EvidenceAssessment`.
    pub fn permits_causal_claim(self) -> bool {
        matches!(
            self,
            Self::ControlledExperiment | Self::Mechanistic
        )
    }
}

// ============================================================================
// Evidence assessment
// ============================================================================

/// Explicit evidence assessment.
///
/// The caller is responsible for supplying this information because the
/// attribution engine cannot infer experimental design from metric values.
#[derive(Debug, Clone, PartialEq)]
pub struct EvidenceAssessment {
    /// Evidence category.
    pub evidence_type: EvidenceType,

    /// Statistical/experimental confidence in the supplied evidence.
    pub confidence: f64,

    /// Number of independent observations supporting the evidence.
    pub sample_count: usize,

    /// Whether the experiment explicitly controlled relevant confounders.
    pub confounders_controlled: bool,

    /// Human/machine-readable evidence note.
    pub note: Option<String>,
}

impl EvidenceAssessment {
    /// Construct an observational evidence assessment.
    pub fn observational(
        confidence: f64,
        sample_count: usize,
    ) -> Result<Self, AttributionError> {
        Self::new(
            EvidenceType::Observational,
            confidence,
            sample_count,
            false,
        )
    }

    /// Construct a controlled-experiment assessment.
    pub fn controlled_experiment(
        confidence: f64,
        sample_count: usize,
        confounders_controlled: bool,
    ) -> Result<Self, AttributionError> {
        Self::new(
            EvidenceType::ControlledExperiment,
            confidence,
            sample_count,
            confounders_controlled,
        )
    }

    /// Construct an assessment.
    pub fn new(
        evidence_type: EvidenceType,
        confidence: f64,
        sample_count: usize,
        confounders_controlled: bool,
    ) -> Result<Self, AttributionError> {
        validate_confidence(confidence)?;

        Ok(Self {
            evidence_type,
            confidence,
            sample_count,
            confounders_controlled,
            note: None,
        })
    }

    /// Add an explanatory note.
    pub fn with_note(mut self, note: impl Into<String>) -> Self {
        self.note = Some(note.into());
        self
    }

    /// Determine the effective evidence strength.
    pub fn effective_confidence(&self) -> f64 {
        let mut confidence = self.confidence;

        if self.sample_count == 0 {
            confidence *= 0.25;
        }

        if !self.confounders_controlled {
            confidence *= 0.75;
        }

        if self.evidence_type == EvidenceType::Insufficient {
            confidence = 0.0;
        }

        confidence.clamp(MIN_CONFIDENCE, MAX_CONFIDENCE)
    }
}

// ============================================================================
// Factor specification
// ============================================================================

/// Specifies how a measurable factor is associated with a target benchmark
/// metric.
#[derive(Debug, Clone, PartialEq)]
pub struct AttributionFactor {
    /// Stable factor identifier.
    pub id: String,

    /// Attribution domain.
    pub source: AttributionSource,

    /// Human-readable explanation.
    pub description: String,

    /// Metric representing this factor.
    pub metric: MetricId,

    /// Relative importance assigned by the caller.
    ///
    /// Weights are normalized internally and therefore need not sum to one.
    pub weight: f64,

    /// Expected relationship between factor degradation and target degradation.
    pub relationship: FactorRelationship,

    /// Evidence supporting this factor.
    pub evidence: EvidenceAssessment,

    /// Optional causal-control metadata.
    pub controlled: bool,
}

impl AttributionFactor {
    /// Construct a factor.
    pub fn new(
        id: impl Into<String>,
        source: AttributionSource,
        description: impl Into<String>,
        metric: MetricId,
        weight: f64,
        relationship: FactorRelationship,
        evidence: EvidenceAssessment,
    ) -> Result<Self, AttributionError> {
        let id = id.into();
        let description = description.into();

        if id.trim().is_empty() {
            return Err(AttributionError::EmptyFactorId);
        }

        if description.trim().is_empty() {
            return Err(AttributionError::EmptyFactorDescription);
        }

        if !weight.is_finite() || weight < 0.0 {
            return Err(AttributionError::InvalidWeight { value: weight });
        }

        Ok(Self {
            id,
            source,
            description,
            metric,
            weight,
            relationship,
            controlled: false,
            evidence,
        })
    }

    /// Mark this factor as controlled by the supplied experiment.
    pub fn controlled(mut self, controlled: bool) -> Self {
        self.controlled = controlled;
        self
    }
}

// ============================================================================
// Factor relationship
// ============================================================================

/// Expected relationship between a factor metric and the target metric.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FactorRelationship {
    /// Factor degradation and target degradation move in the same direction.
    SameDirection,

    /// Factor degradation and target degradation move in opposite numerical
    /// directions.
    OppositeDirection,

    /// Factor has no expected monotonic direction.
    Neutral,
}

// ============================================================================
// Attribution model
// ============================================================================

/// Mathematical model used to distribute observed target degradation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttributionModel {
    /// Allocate the target degradation according to normalized factor
    /// magnitudes, weights, and evidence.
    WeightedObservedChange,

    /// Same as weighted observed change but only count factors whose observed
    /// direction agrees with the configured relationship.
    DirectionalWeightedChange,
}

impl Default for AttributionModel {
    fn default() -> Self {
        Self::DirectionalWeightedChange
    }
}

// ============================================================================
// Attribution finding
// ============================================================================

/// One attribution finding.
#[derive(Debug, Clone, PartialEq)]
pub struct AttributionFinding {
    /// Stable factor identifier.
    pub factor_id: String,

    /// Attribution source.
    pub source: AttributionSource,

    /// Factor metric.
    pub metric: MetricId,

    /// Description.
    pub description: String,

    /// Observed factor delta.
    pub factor_delta: MetricDelta,

    /// Raw factor magnitude used by the attribution model.
    pub raw_factor_magnitude: f64,

    /// Normalized share of the attributed change.
    ///
    /// The sum of all included finding shares is normally approximately one.
    pub normalized_share: f64,

    /// Estimated amount of target degradation attributed to this factor.
    ///
    /// This is a model allocation, not an independently measured causal effect.
    pub attributed_target_change: f64,

    /// Effective evidence confidence.
    pub evidence_confidence: f64,

    /// Evidence classification.
    pub evidence_type: EvidenceType,

    /// Whether the finding is allowed to be described as causal.
    pub causal_claim_supported: bool,

    /// Whether the factor was materially changed.
    pub materially_changed: bool,
}

impl AttributionFinding {
    /// Whether this finding has a meaningful attribution.
    pub fn is_material(&self, threshold: f64) -> bool {
        self.attributed_target_change.abs() > threshold
    }
}

// ============================================================================
// Attribution claim
// ============================================================================

/// Scientific interpretation of the overall attribution.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttributionClaim {
    /// Factors are associated with the observed change.
    Association,

    /// At least one factor has controlled evidence supporting causality.
    Causal,

    /// Evidence is insufficient to make a useful attribution.
    InsufficientEvidence,
}

// ============================================================================
// Attribution report
// ============================================================================

/// Complete attribution report.
#[derive(Debug, Clone, PartialEq)]
pub struct AttributionReport {
    /// Stable schema version.
    pub schema_version: u32,

    /// Analysis identifier.
    pub analysis_id: &'static str,

    /// Baseline snapshot identifier.
    pub baseline_id: String,

    /// Current snapshot identifier.
    pub current_id: String,

    /// Target metric.
    pub target_metric: MetricId,

    /// Target metric delta.
    pub target_delta: MetricDelta,

    /// Attribution model.
    pub model: AttributionModel,

    /// Attribution findings.
    pub findings: Vec<AttributionFinding>,

    /// Unattributed fraction.
    ///
    /// This is important because a production system must not force 100% of a
    /// change into the supplied factors when the evidence does not support it.
    pub unattributed_share: f64,

    /// Overall scientific claim.
    pub claim: AttributionClaim,

    /// Overall confidence.
    pub confidence: f64,

    /// Human/machine-readable warnings.
    pub warnings: Vec<String>,
}

impl AttributionReport {
    /// Return findings sorted by descending attribution magnitude.
    ///
    /// The returned collection is a clone so callers cannot accidentally
    /// mutate the canonical report ordering.
    pub fn ranked_findings(&self) -> Vec<AttributionFinding> {
        let mut findings = self.findings.clone();

        findings.sort_by(|left, right| {
            right
                .attributed_target_change
                .abs()
                .partial_cmp(&left.attributed_target_change.abs())
                .unwrap_or(Ordering::Equal)
                .then_with(|| left.factor_id.cmp(&right.factor_id))
        });

        findings
    }

    /// Return the strongest finding.
    pub fn top_finding(&self) -> Option<&AttributionFinding> {
        self.findings.iter().max_by(|left, right| {
            left.attributed_target_change
                .abs()
                .partial_cmp(&right.attributed_target_change.abs())
                .unwrap_or(Ordering::Equal)
        })
    }

    /// Return only materially attributed findings.
    pub fn material_findings(
        &self,
        threshold: f64,
    ) -> Vec<&AttributionFinding> {
        self.findings
            .iter()
            .filter(|finding| finding.is_material(threshold))
            .collect()
    }

    /// Validate report invariants.
    pub fn validate(&self) -> Result<(), AttributionError> {
        if !self.unattributed_share.is_finite()
            || self.unattributed_share < 0.0
            || self.unattributed_share > 1.0 + DEFAULT_MIN_CONTRIBUTION
        {
            return Err(AttributionError::InvalidResult {
                message: format!(
                    "unattributed share must be approximately in [0, 1], got {}",
                    self.unattributed_share
                ),
            });
        }

        if !self.confidence.is_finite()
            || !(MIN_CONFIDENCE..=MAX_CONFIDENCE).contains(&self.confidence)
        {
            return Err(AttributionError::InvalidResult {
                message: format!(
                    "report confidence must be in [0, 1], got {}",
                    self.confidence
                ),
            });
        }

        for finding in &self.findings {
            if !finding.normalized_share.is_finite()
                || finding.normalized_share < 0.0
            {
                return Err(AttributionError::InvalidResult {
                    message: format!(
                        "invalid normalized share for factor '{}'",
                        finding.factor_id
                    ),
                });
            }

            if !finding.attributed_target_change.is_finite() {
                return Err(AttributionError::InvalidResult {
                    message: format!(
                        "non-finite attributed change for factor '{}'",
                        finding.factor_id
                    ),
                });
            }
        }

        Ok(())
    }
}

// ============================================================================
// Analyzer configuration
// ============================================================================

/// Configuration for attribution analysis.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AttributionConfig {
    /// Attribution model.
    pub model: AttributionModel,

    /// Minimum material factor change.
    pub minimum_material_change: f64,

    /// Minimum contribution retained in the report.
    pub minimum_contribution: f64,

    /// Whether zero-change factors should be included.
    pub include_zero_change_factors: bool,

    /// Whether factors with insufficient evidence should be included.
    ///
    /// Keeping them is normally preferable because it preserves transparency.
    pub include_insufficient_evidence: bool,
}

impl Default for AttributionConfig {
    fn default() -> Self {
        Self {
            model: AttributionModel::default(),
            minimum_material_change: DEFAULT_MIN_MATERIAL_CHANGE,
            minimum_contribution: DEFAULT_MIN_CONTRIBUTION,
            include_zero_change_factors: false,
            include_insufficient_evidence: true,
        }
    }
}

impl AttributionConfig {
    /// Validate configuration.
    pub fn validate(&self) -> Result<(), AttributionError> {
        validate_non_negative(
            self.minimum_material_change,
            "minimum_material_change",
        )?;

        validate_non_negative(
            self.minimum_contribution,
            "minimum_contribution",
        )?;

        Ok(())
    }
}

// ============================================================================
// Analyzer
// ============================================================================

/// Production attribution analyzer.
///
/// This type owns no mutable global state and is safe to construct per
/// benchmark analysis.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AttributionAnalyzer {
    /// Analysis configuration.
    pub config: AttributionConfig,
}

impl AttributionAnalyzer {
    /// Construct an analyzer using production defaults.
    pub fn new() -> Self {
        Self {
            config: AttributionConfig::default(),
        }
    }

    /// Construct an analyzer from explicit configuration.
    pub fn with_config(
        config: AttributionConfig,
    ) -> Result<Self, AttributionError> {
        config.validate()?;

        Ok(Self { config })
    }

    /// Analyze a target metric using explicit attribution factors.
    ///
    /// This is the primary public API.
    pub fn analyze(
        &self,
        baseline: &MetricSnapshot,
        current: &MetricSnapshot,
        target_metric: &MetricId,
        factors: &[AttributionFactor],
    ) -> Result<AttributionReport, AttributionError> {
        self.validate_inputs(
            baseline,
            current,
            target_metric,
            factors,
        )?;

        let baseline_target = baseline
            .get(target_metric)
            .ok_or_else(|| AttributionError::MissingTargetMetric {
                metric: target_metric.to_string(),
            })?;

        let current_target = current
            .get(target_metric)
            .ok_or_else(|| AttributionError::MissingTargetMetric {
                metric: target_metric.to_string(),
            })?;

        let target_delta =
            MetricDelta::between(baseline_target, current_target)?;

        let mut candidate_findings = Vec::new();

        for factor in factors {
            let baseline_metric =
                baseline
                    .get(&factor.metric)
                    .ok_or_else(|| AttributionError::MissingMetric {
                        metric: factor.metric.to_string(),
                        factor_id: factor.id.clone(),
                    })?;

            let current_metric =
                current
                    .get(&factor.metric)
                    .ok_or_else(|| AttributionError::MissingMetric {
                        metric: factor.metric.to_string(),
                        factor_id: factor.id.clone(),
                    })?;

            let factor_delta =
                MetricDelta::between(baseline_metric, current_metric)?;

            let materially_changed =
                factor_delta.is_material(self.config.minimum_material_change);

            if !materially_changed
                && !self.config.include_zero_change_factors
            {
                continue;
            }

            if factor.evidence.evidence_type == EvidenceType::Insufficient
                && !self.config.include_insufficient_evidence
            {
                continue;
            }

            let raw_magnitude = self.raw_factor_magnitude(
                factor,
                &factor_delta,
                target_delta.is_degradation,
            );

            candidate_findings.push((
                factor,
                factor_delta,
                materially_changed,
                raw_magnitude,
            ));
        }

        let total_weighted_magnitude = candidate_findings
            .iter()
            .map(|(_, _, _, magnitude)| *magnitude)
            .sum::<f64>();

        let findings = self.build_findings(
            candidate_findings,
            total_weighted_magnitude,
            target_delta.change_magnitude(),
        );

        let attributed_share = findings
            .iter()
            .map(|finding| finding.normalized_share)
            .sum::<f64>();

        let unattributed_share =
            (1.0 - attributed_share).clamp(0.0, 1.0);

        let claim = determine_claim(&findings);

        let confidence = determine_report_confidence(
            &findings,
            attributed_share,
        );

        let mut warnings = Vec::new();

        if findings.is_empty() {
            warnings.push(
                "No materially changed attribution factor was available."
                    .to_string(),
            );
        }

        if unattributed_share
            > DEFAULT_MIN_CONTRIBUTION
        {
            warnings.push(format!(
                "{:.2}% of the target change remains unattributed.",
                unattributed_share * 100.0
            ));
        }

        if findings
            .iter()
            .any(|finding| !finding.causal_claim_supported)
        {
            warnings.push(
                "At least one attribution is observational; \
                 association must not be reported as causation."
                    .to_string(),
            );
        }

        let report = AttributionReport {
            schema_version: ATTRIBUTION_SCHEMA_VERSION,
            analysis_id: ATTRIBUTION_ANALYSIS_ID,
            baseline_id: baseline.id.clone(),
            current_id: current.id.clone(),
            target_metric: target_metric.clone(),
            target_delta,
            model: self.config.model,
            findings,
            unattributed_share,
            claim,
            confidence,
            warnings,
        };

        report.validate()?;

        Ok(report)
    }

    /// Validate analyzer inputs.
    pub fn validate_inputs(
        &self,
        baseline: &MetricSnapshot,
        current: &MetricSnapshot,
        target_metric: &MetricId,
        factors: &[AttributionFactor],
    ) -> Result<(), AttributionError> {
        self.config.validate()?;

        baseline.validate()?;
        current.validate()?;

        if factors.is_empty() {
            return Err(AttributionError::EmptyFactorSet);
        }

        if baseline.get(target_metric).is_none()
            || current.get(target_metric).is_none()
        {
            return Err(AttributionError::MissingTargetMetric {
                metric: target_metric.to_string(),
            });
        }

        let mut factor_ids: Vec<&str> = Vec::with_capacity(factors.len());

        for factor in factors {
            if factor.id.trim().is_empty() {
                return Err(AttributionError::EmptyFactorId);
            }

            if factor.description.trim().is_empty() {
                return Err(AttributionError::EmptyFactorDescription);
            }

            if !factor.weight.is_finite() || factor.weight < 0.0 {
                return Err(AttributionError::InvalidWeight {
                    value: factor.weight,
                });
            }

            if baseline.get(&factor.metric).is_none() {
                return Err(AttributionError::MissingMetric {
                    metric: factor.metric.to_string(),
                    factor_id: factor.id.clone(),
                });
            }

            if current.get(&factor.metric).is_none() {
                return Err(AttributionError::MissingMetric {
                    metric: factor.metric.to_string(),
                    factor_id: factor.id.clone(),
                });
            }

            factor_ids.push(factor.id.as_str());
        }

        for left in 0..factor_ids.len() {
            for right in (left + 1)..factor_ids.len() {
                if factor_ids[left] == factor_ids[right] {
                    return Err(AttributionError::DuplicateFactor {
                        factor_id: factor_ids[left].to_string(),
                    });
                }
            }
        }

        Ok(())
    }

    /// Calculate the raw contribution magnitude of one factor.
    fn raw_factor_magnitude(
        &self,
        factor: &AttributionFactor,
        delta: &MetricDelta,
        target_is_degradation: bool,
    ) -> f64 {
        let magnitude = delta.change_magnitude();

        if magnitude == 0.0 {
            return 0.0;
        }

        let directionally_consistent = relationship_matches(
            factor.relationship,
            delta,
            target_is_degradation,
        );

        let evidence = factor.evidence.effective_confidence();

        let weight = factor.weight;

        match self.config.model {
            AttributionModel::WeightedObservedChange => {
                magnitude * weight * evidence
            }

            AttributionModel::DirectionalWeightedChange => {
                if directionally_consistent {
                    magnitude * weight * evidence
                } else {
                    0.0
                }
            }
        }
    }

    /// Construct findings from raw factor magnitudes.
    fn build_findings<'a>(
        &self,
        candidates: Vec<(
            &'a AttributionFactor,
            MetricDelta,
            bool,
            f64,
        )>,
        total_magnitude: f64,
        target_change_magnitude: f64,
    ) -> Vec<AttributionFinding> {
        if total_magnitude <= 0.0
            || !total_magnitude.is_finite()
        {
            return Vec::new();
        }

        let mut findings = candidates
            .into_iter()
            .filter_map(
                |(
                    factor,
                    factor_delta,
                    materially_changed,
                    raw_magnitude,
                )| {
                    if raw_magnitude <= 0.0
                        || !raw_magnitude.is_finite()
                    {
                        return None;
                    }

                    let normalized_share =
                        (raw_magnitude / total_magnitude)
                            .clamp(0.0, 1.0);

                    if normalized_share
                        < self.config.minimum_contribution
                    {
                        return None;
                    }

                    let attributed_target_change =
                        target_change_magnitude
                            * normalized_share;

                    Some(AttributionFinding {
                        factor_id: factor.id.clone(),
                        source: factor.source.clone(),
                        metric: factor.metric.clone(),
                        description: factor.description.clone(),
                        factor_delta,
                        raw_factor_magnitude: raw_magnitude,
                        normalized_share,
                        attributed_target_change,
                        evidence_confidence:
                            factor.evidence.effective_confidence(),
                        evidence_type:
                            factor.evidence.evidence_type,
                        causal_claim_supported:
                            factor.controlled
                                && factor.evidence
                                    .evidence_type
                                    .permits_causal_claim()
                                && factor.evidence
                                    .confounders_controlled,
                        materially_changed,
                    })
                },
            )
            .collect::<Vec<_>>();

        findings.sort_by(|left, right| {
            right
                .attributed_target_change
                .abs()
                .partial_cmp(
                    &left.attributed_target_change.abs(),
                )
                .unwrap_or(Ordering::Equal)
                .then_with(|| {
                    left.factor_id.cmp(&right.factor_id)
                })
        });

        findings
    }
}

impl Default for AttributionAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Convenience functions
// ============================================================================

/// Analyze attribution using production defaults.
pub fn attribute(
    baseline: &MetricSnapshot,
    current: &MetricSnapshot,
    target_metric: &MetricId,
    factors: &[AttributionFactor],
) -> Result<AttributionReport, AttributionError> {
    AttributionAnalyzer::new().analyze(
        baseline,
        current,
        target_metric,
        factors,
    )
}

/// Calculate a metric delta.
pub fn calculate_delta(
    baseline: &MetricValue,
    current: &MetricValue,
) -> Result<MetricDelta, AttributionError> {
    MetricDelta::between(baseline, current)
}

// ============================================================================
// Internal relationship logic
// ============================================================================

fn relationship_matches(
    relationship: FactorRelationship,
    delta: &MetricDelta,
    target_is_degradation: bool,
) -> bool {
    match relationship {
        FactorRelationship::Neutral => true,

        FactorRelationship::SameDirection => {
            if target_is_degradation {
                delta.is_degradation
            } else {
                !delta.is_degradation
            }
        }

        FactorRelationship::OppositeDirection => {
            if target_is_degradation {
                !delta.is_degradation
            } else {
                delta.is_degradation
            }
        }
    }
}

fn determine_claim(
    findings: &[AttributionFinding],
) -> AttributionClaim {
    if findings.is_empty() {
        return AttributionClaim::InsufficientEvidence;
    }

    let has_causal = findings.iter().any(|finding| {
        finding.causal_claim_supported
            && finding.evidence_confidence > 0.0
    });

    if has_causal {
        AttributionClaim::Causal
    } else {
        AttributionClaim::Association
    }
}

fn determine_report_confidence(
    findings: &[AttributionFinding],
    attributed_share: f64,
) -> f64 {
    if findings.is_empty() {
        return 0.0;
    }

    let weighted_confidence = findings
        .iter()
        .map(|finding| {
            finding.normalized_share
                * finding.evidence_confidence
        })
        .sum::<f64>();

    (weighted_confidence * attributed_share)
        .clamp(MIN_CONFIDENCE, MAX_CONFIDENCE)
}

fn validate_confidence(
    value: f64,
) -> Result<(), AttributionError> {
    if !value.is_finite()
        || !(MIN_CONFIDENCE..=MAX_CONFIDENCE).contains(&value)
    {
        return Err(AttributionError::InvalidConfidence {
            value,
        });
    }

    Ok(())
}

fn validate_non_negative(
    value: f64,
    field: &'static str,
) -> Result<(), AttributionError> {
    if !value.is_finite() || value < 0.0 {
        return Err(AttributionError::NonFiniteValue {
            field,
            value,
        });
    }

    Ok(())
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn metric(
        name: &str,
        value: f64,
        direction: MetricDirection,
    ) -> MetricValue {
        MetricValue::new(
            MetricId::new(name).expect("valid metric"),
            value,
            direction,
        )
        .expect("valid metric value")
    }

    fn factor(
        id: &str,
        metric_name: &str,
        weight: f64,
    ) -> AttributionFactor {
        AttributionFactor::new(
            id,
            AttributionSource::TwoQubitGateError,
            "Two-qubit gate error contribution.",
            MetricId::new(metric_name).expect("valid metric"),
            weight,
            FactorRelationship::SameDirection,
            EvidenceAssessment::observational(
                DEFAULT_OBSERVATIONAL_CONFIDENCE,
                100,
            )
            .expect("valid evidence"),
        )
        .expect("valid factor")
    }

    #[test]
    fn metric_id_rejects_empty_name() {
        assert!(MetricId::new("   ").is_err());
    }

    #[test]
    fn metric_snapshot_rejects_duplicates() {
        let mut snapshot = MetricSnapshot::new("baseline");

        snapshot
            .push(metric(
                "fidelity",
                0.99,
                MetricDirection::HigherIsBetter,
            ))
            .expect("first metric");

        assert!(snapshot
            .push(metric(
                "fidelity",
                0.98,
                MetricDirection::HigherIsBetter,
            ))
            .is_err());
    }

    #[test]
    fn delta_detects_fidelity_degradation() {
        let baseline = metric(
            "fidelity",
            0.99,
            MetricDirection::HigherIsBetter,
        );

        let current = metric(
            "fidelity",
            0.90,
            MetricDirection::HigherIsBetter,
        );

        let delta =
            MetricDelta::between(&baseline, &current)
                .expect("delta");

        assert!(delta.is_degradation);
        assert!(delta.absolute_change < 0.0);
        assert_eq!(
            delta.relative_change,
            Some(-0.09 / 0.99)
        );
    }

    #[test]
    fn delta_detects_latency_degradation() {
        let baseline =
            metric("latency", 10.0, MetricDirection::LowerIsBetter);

        let current =
            metric("latency", 12.0, MetricDirection::LowerIsBetter);

        let delta =
            MetricDelta::between(&baseline, &current)
                .expect("delta");

        assert!(delta.is_degradation);
        assert_eq!(delta.absolute_change, 2.0);
    }

    #[test]
    fn zero_baseline_uses_absolute_change() {
        let baseline =
            metric("error", 0.0, MetricDirection::LowerIsBetter);

        let current =
            metric("error", 0.01, MetricDirection::LowerIsBetter);

        let delta =
            MetricDelta::between(&baseline, &current)
                .expect("delta");

        assert_eq!(delta.relative_change, None);
        assert_eq!(delta.change_magnitude(), 0.01);
        assert!(delta.is_degradation);
    }

    #[test]
    fn observational_attribution_is_not_causal() {
        let mut baseline = MetricSnapshot::new("baseline");

        baseline
            .push(metric(
                "fidelity",
                0.99,
                MetricDirection::HigherIsBetter,
            ))
            .expect("fidelity");

        baseline
            .push(metric(
                "two_qubit_error",
                0.01,
                MetricDirection::LowerIsBetter,
            ))
            .expect("error");

        let mut current = MetricSnapshot::new("current");

        current
            .push(metric(
                "fidelity",
                0.90,
                MetricDirection::HigherIsBetter,
            ))
            .expect("fidelity");

        current
            .push(metric(
                "two_qubit_error",
                0.03,
                MetricDirection::LowerIsBetter,
            ))
            .expect("error");

        let report = attribute(
            &baseline,
            &current,
            &MetricId::new("fidelity").expect("metric"),
            &[factor(
                "two_qubit_error_factor",
                "two_qubit_error",
                1.0,
            )],
        )
        .expect("attribution");

        assert_eq!(
            report.claim,
            AttributionClaim::Association
        );

        assert_eq!(report.findings.len(), 1);

        assert!(!report.findings[0].causal_claim_supported);
    }

    #[test]
    fn controlled_experiment_can_support_causal_claim() {
        let mut baseline = MetricSnapshot::new("baseline");

        baseline
            .push(metric(
                "fidelity",
                0.99,
                MetricDirection::HigherIsBetter,
            ))
            .expect("fidelity");

        baseline
            .push(metric(
                "gate_error",
                0.01,
                MetricDirection::LowerIsBetter,
            ))
            .expect("gate error");

        let mut current = MetricSnapshot::new("current");

        current
            .push(metric(
                "fidelity",
                0.90,
                MetricDirection::HigherIsBetter,
            ))
            .expect("fidelity");

        current
            .push(metric(
                "gate_error",
                0.03,
                MetricDirection::LowerIsBetter,
            ))
            .expect("gate error");

        let evidence =
            EvidenceAssessment::controlled_experiment(
                0.95,
                1000,
                true,
            )
            .expect("evidence");

        let factor = AttributionFactor::new(
            "gate_error_factor",
            AttributionSource::GateError,
            "Controlled gate-error experiment.",
            MetricId::new("gate_error").expect("metric"),
            1.0,
            FactorRelationship::SameDirection,
            evidence,
        )
        .expect("factor")
        .controlled(true);

        let report = attribute(
            &baseline,
            &current,
            &MetricId::new("fidelity").expect("metric"),
            &[factor],
        )
        .expect("report");

        assert_eq!(
            report.claim,
            AttributionClaim::Causal
        );

        assert!(
            report.findings[0]
                .causal_claim_supported
        );
    }

    #[test]
    fn directional_model_rejects_inconsistent_factor() {
        let mut baseline = MetricSnapshot::new("baseline");

        baseline
            .push(metric(
                "fidelity",
                0.90,
                MetricDirection::HigherIsBetter,
            ))
            .expect("fidelity");

        baseline
            .push(metric(
                "throughput",
                100.0,
                MetricDirection::HigherIsBetter,
            ))
            .expect("throughput");

        let mut current = MetricSnapshot::new("current");

        current
            .push(metric(
                "fidelity",
                0.80,
                MetricDirection::HigherIsBetter,
            ))
            .expect("fidelity");

        current
            .push(metric(
                "throughput",
                110.0,
                MetricDirection::HigherIsBetter,
            ))
            .expect("throughput");

        let factor = AttributionFactor::new(
            "throughput_factor",
            AttributionSource::ExecutionLatency,
            "Throughput changed.",
            MetricId::new("throughput").expect("metric"),
            1.0,
            FactorRelationship::SameDirection,
            EvidenceAssessment::observational(0.8, 100)
                .expect("evidence"),
        )
        .expect("factor");

        let report = attribute(
            &baseline,
            &current,
            &MetricId::new("fidelity").expect("metric"),
            &[factor],
        )
        .expect("report");

        assert!(
            report.findings.is_empty()
                || report.unattributed_share > 0.0
        );
    }

    #[test]
    fn multiple_factors_are_normalized() {
        let mut baseline = MetricSnapshot::new("baseline");

        baseline
            .push(metric(
                "fidelity",
                0.99,
                MetricDirection::HigherIsBetter,
            ))
            .expect("fidelity");

        baseline
            .push(metric(
                "gate_error",
                0.01,
                MetricDirection::LowerIsBetter,
            ))
            .expect("gate");

        baseline
            .push(metric(
                "readout_error",
                0.01,
                MetricDirection::LowerIsBetter,
            ))
            .expect("readout");

        let mut current = MetricSnapshot::new("current");

        current
            .push(metric(
                "fidelity",
                0.90,
                MetricDirection::HigherIsBetter,
            ))
            .expect("fidelity");

        current
            .push(metric(
                "gate_error",
                0.02,
                MetricDirection::LowerIsBetter,
            ))
            .expect("gate");

        current
            .push(metric(
                "readout_error",
                0.02,
                MetricDirection::LowerIsBetter,
            ))
            .expect("readout");

        let factors = vec![
            factor("gate", "gate_error", 1.0),
            AttributionFactor::new(
                "readout",
                AttributionSource::ReadoutError,
                "Readout error.",
                MetricId::new("readout_error")
                    .expect("metric"),
                1.0,
                FactorRelationship::SameDirection,
                EvidenceAssessment::observational(
                    DEFAULT_OBSERVATIONAL_CONFIDENCE,
                    100,
                )
                .expect("evidence"),
            )
            .expect("factor"),
        ];

        let report = attribute(
            &baseline,
            &current,
            &MetricId::new("fidelity").expect("metric"),
            &factors,
        )
        .expect("report");

        let share_sum = report
            .findings
            .iter()
            .map(|finding| finding.normalized_share)
            .sum::<f64>();

        assert!(
            (share_sum - 1.0).abs()
                < 1.0e-12
        );

        assert!(
            report.unattributed_share.abs()
                < 1.0e-12
        );
    }

    #[test]
    fn report_ranking_is_deterministic() {
        let findings = vec![
            AttributionFinding {
                factor_id: "b".to_string(),
                source: AttributionSource::GateError,
                metric: MetricId::new("b").expect("metric"),
                description: "b".to_string(),
                factor_delta: MetricDelta {
                    metric: MetricId::new("b").expect("metric"),
                    baseline: 1.0,
                    current: 2.0,
                    absolute_change: 1.0,
                    relative_change: Some(1.0),
                    is_degradation: true,
                    direction: MetricDirection::LowerIsBetter,
                },
                raw_factor_magnitude: 0.2,
                normalized_share: 0.2,
                attributed_target_change: 0.02,
                evidence_confidence: 0.8,
                evidence_type: EvidenceType::Observational,
                causal_claim_supported: false,
                materially_changed: true,
            },
            AttributionFinding {
                factor_id: "a".to_string(),
                source: AttributionSource::ReadoutError,
                metric: MetricId::new("a").expect("metric"),
                description: "a".to_string(),
                factor_delta: MetricDelta {
                    metric: MetricId::new("a").expect("metric"),
                    baseline: 1.0,
                    current: 2.0,
                    absolute_change: 1.0,
                    relative_change: Some(1.0),
                    is_degradation: true,
                    direction: MetricDirection::LowerIsBetter,
                },
                raw_factor_magnitude: 0.8,
                normalized_share: 0.8,
                attributed_target_change: 0.08,
                evidence_confidence: 0.8,
                evidence_type: EvidenceType::Observational,
                causal_claim_supported: false,
                materially_changed: true,
            },
        ];

        let report = AttributionReport {
            schema_version: ATTRIBUTION_SCHEMA_VERSION,
            analysis_id: ATTRIBUTION_ANALYSIS_ID,
            baseline_id: "baseline".to_string(),
            current_id: "current".to_string(),
            target_metric: MetricId::new("fidelity").expect("metric"),
            target_delta: MetricDelta {
                metric: MetricId::new("fidelity").expect("metric"),
                baseline: 1.0,
                current: 0.9,
                absolute_change: -0.1,
                relative_change: Some(-0.1),
                is_degradation: true,
                direction: MetricDirection::HigherIsBetter,
            },
            model: AttributionModel::DirectionalWeightedChange,
            findings,
            unattributed_share: 0.0,
            claim: AttributionClaim::Association,
            confidence: 0.5,
            warnings: Vec::new(),
        };

        let ranked = report.ranked_findings();

        assert_eq!(ranked[0].factor_id, "a");
        assert_eq!(ranked[1].factor_id, "b");
    }

    #[test]
    fn insufficient_evidence_has_zero_effective_confidence() {
        let evidence = EvidenceAssessment::new(
            EvidenceType::Insufficient,
            1.0,
            100,
            true,
        )
        .expect("evidence");

        assert_eq!(
            evidence.effective_confidence(),
            0.0
        );
    }

    #[test]
    fn evidence_confidence_is_bounded() {
        let evidence = EvidenceAssessment::observational(
            1.0,
            0,
        )
        .expect("evidence");

        assert!(
            evidence.effective_confidence()
                <= 1.0
        );

        assert!(
            evidence.effective_confidence()
                >= 0.0
        );
    }

    #[test]
    fn empty_factors_are_rejected() {
        let mut baseline = MetricSnapshot::new("baseline");

        baseline
            .push(metric(
                "fidelity",
                0.99,
                MetricDirection::HigherIsBetter,
            ))
            .expect("metric");

        let current = baseline.clone();

        let result = attribute(
            &baseline,
            &current,
            &MetricId::new("fidelity").expect("metric"),
            &[],
        );

        assert!(matches!(
            result,
            Err(AttributionError::EmptyFactorSet)
        ));
    }

    #[test]
    fn missing_factor_metric_is_rejected() {
        let mut baseline = MetricSnapshot::new("baseline");

        baseline
            .push(metric(
                "fidelity",
                0.99,
                MetricDirection::HigherIsBetter,
            ))
            .expect("metric");

        let current = baseline.clone();

        let factor = factor(
            "missing",
            "does_not_exist",
            1.0,
        );

        let result = attribute(
            &baseline,
            &current,
            &MetricId::new("fidelity").expect("metric"),
            &[factor],
        );

        assert!(matches!(
            result,
            Err(AttributionError::MissingMetric { .. })
        ));
    }

    #[test]
    fn non_finite_metric_is_rejected() {
        let result = MetricValue::new(
            MetricId::new("fidelity").expect("metric"),
            f64::NAN,
            MetricDirection::HigherIsBetter,
        );

        assert!(result.is_err());
    }

    #[test]
    fn custom_sources_are_supported() {
        let source =
            AttributionSource::Custom(
                "my_custom_factor".to_string(),
            );

        assert_eq!(
            source.as_str(),
            "my_custom_factor"
        );
    }

    #[test]
    fn analyzer_is_default_constructible() {
        let analyzer = AttributionAnalyzer::default();

        assert_eq!(
            analyzer.config.model,
            AttributionModel::DirectionalWeightedChange
        );
    }

    #[test]
    fn report_validation_accepts_valid_report() {
        let report = AttributionReport {
            schema_version: ATTRIBUTION_SCHEMA_VERSION,
            analysis_id: ATTRIBUTION_ANALYSIS_ID,
            baseline_id: "baseline".to_string(),
            current_id: "current".to_string(),
            target_metric: MetricId::new("fidelity")
                .expect("metric"),
            target_delta: MetricDelta {
                metric: MetricId::new("fidelity")
                    .expect("metric"),
                baseline: 0.99,
                current: 0.90,
                absolute_change: -0.09,
                relative_change: Some(-0.09 / 0.99),
                is_degradation: true,
                direction: MetricDirection::HigherIsBetter,
            },
            model: AttributionModel::DirectionalWeightedChange,
            findings: Vec::new(),
            unattributed_share: 1.0,
            claim: AttributionClaim::InsufficientEvidence,
            confidence: 0.0,
            warnings: vec![
                "No attribution available.".to_string()
            ],
        };

        assert!(report.validate().is_ok());
    }
}