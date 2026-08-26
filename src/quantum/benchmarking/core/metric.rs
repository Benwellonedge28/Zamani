//! Universal quantum-benchmarking metrics.
//!
//! This module defines the canonical representation of a measured quantum
//! benchmarking metric.
//!
//! Design goals:
//! - No protocol-specific metric representation.
//! - No dependency on benchmark protocols.
//! - No dependency on execution backends.
//! - No dependency on the future `dimension`, `provenance`, or `result` modules.
//! - Explicit units and semantic meaning.
//! - Explicit uncertainty and confidence information.
//! - Rejection of NaN and infinity.
//! - Validation of probabilities, fidelities, error rates, counts, and
//!   confidence intervals.
//! - Stable machine-readable identifiers.
//! - Extensibility for future quantum technologies.
//!
//! Rust compatibility: Rust 1.97 / 1.97.1.
//!
//! Integration:
//! ```text
//! quantum::benchmarking::core::metric
//!              │
//!              ├── core::result
//!              ├── core::dimension
//!              ├── core::provenance
//!              ├── statistics::*
//!              ├── metrics::*
//!              └── protocols::*
//! ```
//!
//! This module is intentionally lower-level than those modules and therefore
//! must not depend on them.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Result type used by metric construction and validation.
pub type MetricResult<T> = Result<T, MetricError>;

/// Canonical metric value.
///
/// `FiniteF64` exists specifically to prevent NaN and infinity from entering
/// the benchmark result model.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(transparent)]
pub struct FiniteF64(f64);

impl FiniteF64 {
    /// Creates a finite value.
    pub fn new(value: f64) -> MetricResult<Self> {
        if value.is_finite() {
            Ok(Self(value))
        } else {
            Err(MetricError::NonFiniteValue { value })
        }
    }

    /// Returns the underlying floating-point value.
    #[inline]
    pub fn get(self) -> f64 {
        self.0
    }

    /// Returns the underlying floating-point value by reference.
    #[inline]
    pub fn as_f64(&self) -> f64 {
        self.0
    }
}

impl From<FiniteF64> for f64 {
    fn from(value: FiniteF64) -> Self {
        value.0
    }
}

impl fmt::Display for FiniteF64 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

/// Machine-readable metric category.
///
/// The enum contains the standard Zamani quantum-benchmarking metric families.
/// `Custom` permits future metrics without changing this foundational module.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MetricKind {
    /// Probability of observing a specified event.
    Probability,

    /// Probability that an algorithm produces an accepted answer.
    SuccessProbability,

    /// Quantum-state fidelity.
    StateFidelity,

    /// Quantum-process fidelity.
    ProcessFidelity,

    /// Average gate fidelity.
    AverageGateFidelity,

    /// Entanglement fidelity.
    EntanglementFidelity,

    /// Hellinger fidelity/distance-derived quality measure.
    HellingerFidelity,

    /// Total variation distance.
    TotalVariationDistance,

    /// Trace distance.
    TraceDistance,

    /// Classical distribution fidelity.
    ClassicalFidelity,

    /// Generic fidelity.
    Fidelity,

    /// Generic error rate.
    ErrorRate,

    /// Error per gate.
    ErrorPerGate,

    /// Error per Clifford.
    ErrorPerClifford,

    /// Cycle error.
    CycleError,

    /// Gate infidelity.
    GateInfidelity,

    /// Process infidelity.
    ProcessInfidelity,

    /// Readout/assignment fidelity.
    ReadoutFidelity,

    /// Readout assignment error.
    ReadoutError,

    /// State-preparation error.
    StatePreparationError,

    /// SPAM error.
    SpamError,

    /// Leakage probability/rate.
    LeakageRate,

    /// Logical error rate.
    LogicalErrorRate,

    /// Logical fidelity.
    LogicalFidelity,

    /// Physical error rate.
    PhysicalErrorRate,

    /// Decoder failure probability.
    DecoderFailureProbability,

    /// Threshold estimate.
    Threshold,

    /// Runtime.
    Runtime,

    /// Compilation/transpilation latency.
    CompilationTime,

    /// Queue latency.
    QueueTime,

    /// Submission latency.
    SubmissionTime,

    /// Execution latency.
    ExecutionTime,

    /// Readout latency.
    ReadoutTime,

    /// Analysis latency.
    AnalysisTime,

    /// End-to-end wall-clock time.
    TotalWallTime,

    /// Generic latency.
    Latency,

    /// Throughput.
    Throughput,

    /// Shots per second.
    ShotsPerSecond,

    /// Circuits per second.
    CircuitsPerSecond,

    /// Gates per second.
    GatesPerSecond,

    /// Two-qubit gates per second.
    TwoQubitGatesPerSecond,

    /// Circuit layers per second.
    LayersPerSecond,

    /// Quantum volume.
    QuantumVolume,

    /// Number of qubits.
    QubitCount,

    /// Number of logical qubits.
    LogicalQubitCount,

    /// Number of physical qubits.
    PhysicalQubitCount,

    /// Circuit depth.
    CircuitDepth,

    /// Two-qubit circuit depth.
    TwoQubitDepth,

    /// Total gate count.
    GateCount,

    /// Two-qubit gate count.
    TwoQubitGateCount,

    /// Measurement count.
    MeasurementCount,

    /// T-gate count.
    TGateCount,

    /// Classical operation count.
    ClassicalOperationCount,

    /// Memory consumption.
    Memory,

    /// Energy consumption.
    Energy,

    /// Time-to-solution.
    TimeToSolution,

    /// Objective-function value.
    ObjectiveValue,

    /// Approximation ratio.
    ApproximationRatio,

    /// Estimation error.
    EstimationError,

    /// Energy error.
    EnergyError,

    /// Observable error.
    ObservableError,

    /// Algorithmic solution quality.
    SolutionQuality,

    /// Coherence time T1.
    T1,

    /// Coherence time T2.
    T2,

    /// Inhomogeneous dephasing time T2*.
    T2Star,

    /// Pure dephasing time Tphi.
    TPhi,

    /// Crosstalk degradation.
    Crosstalk,

    /// Drift.
    Drift,

    /// Generic stability measurement.
    Stability,

    /// Space-time volume.
    SpaceTimeVolume,

    /// Physical-to-logical resource overhead.
    ResourceOverhead,

    /// Generic custom metric.
    Custom(String),
}

impl MetricKind {
    /// Returns a stable machine-readable identifier.
    pub fn id(&self) -> String {
        match self {
            Self::Probability => "probability".into(),
            Self::SuccessProbability => "success_probability".into(),
            Self::StateFidelity => "state_fidelity".into(),
            Self::ProcessFidelity => "process_fidelity".into(),
            Self::AverageGateFidelity => "average_gate_fidelity".into(),
            Self::EntanglementFidelity => "entanglement_fidelity".into(),
            Self::HellingerFidelity => "hellinger_fidelity".into(),
            Self::TotalVariationDistance => "total_variation_distance".into(),
            Self::TraceDistance => "trace_distance".into(),
            Self::ClassicalFidelity => "classical_fidelity".into(),
            Self::Fidelity => "fidelity".into(),
            Self::ErrorRate => "error_rate".into(),
            Self::ErrorPerGate => "error_per_gate".into(),
            Self::ErrorPerClifford => "error_per_clifford".into(),
            Self::CycleError => "cycle_error".into(),
            Self::GateInfidelity => "gate_infidelity".into(),
            Self::ProcessInfidelity => "process_infidelity".into(),
            Self::ReadoutFidelity => "readout_fidelity".into(),
            Self::ReadoutError => "readout_error".into(),
            Self::StatePreparationError => "state_preparation_error".into(),
            Self::SpamError => "spam_error".into(),
            Self::LeakageRate => "leakage_rate".into(),
            Self::LogicalErrorRate => "logical_error_rate".into(),
            Self::LogicalFidelity => "logical_fidelity".into(),
            Self::PhysicalErrorRate => "physical_error_rate".into(),
            Self::DecoderFailureProbability => "decoder_failure_probability".into(),
            Self::Threshold => "threshold".into(),
            Self::Runtime => "runtime".into(),
            Self::CompilationTime => "compilation_time".into(),
            Self::QueueTime => "queue_time".into(),
            Self::SubmissionTime => "submission_time".into(),
            Self::ExecutionTime => "execution_time".into(),
            Self::ReadoutTime => "readout_time".into(),
            Self::AnalysisTime => "analysis_time".into(),
            Self::TotalWallTime => "total_wall_time".into(),
            Self::Latency => "latency".into(),
            Self::Throughput => "throughput".into(),
            Self::ShotsPerSecond => "shots_per_second".into(),
            Self::CircuitsPerSecond => "circuits_per_second".into(),
            Self::GatesPerSecond => "gates_per_second".into(),
            Self::TwoQubitGatesPerSecond => "two_qubit_gates_per_second".into(),
            Self::LayersPerSecond => "layers_per_second".into(),
            Self::QuantumVolume => "quantum_volume".into(),
            Self::QubitCount => "qubit_count".into(),
            Self::LogicalQubitCount => "logical_qubit_count".into(),
            Self::PhysicalQubitCount => "physical_qubit_count".into(),
            Self::CircuitDepth => "circuit_depth".into(),
            Self::TwoQubitDepth => "two_qubit_depth".into(),
            Self::GateCount => "gate_count".into(),
            Self::TwoQubitGateCount => "two_qubit_gate_count".into(),
            Self::MeasurementCount => "measurement_count".into(),
            Self::TGateCount => "t_gate_count".into(),
            Self::ClassicalOperationCount => "classical_operation_count".into(),
            Self::Memory => "memory".into(),
            Self::Energy => "energy".into(),
            Self::TimeToSolution => "time_to_solution".into(),
            Self::ObjectiveValue => "objective_value".into(),
            Self::ApproximationRatio => "approximation_ratio".into(),
            Self::EstimationError => "estimation_error".into(),
            Self::EnergyError => "energy_error".into(),
            Self::ObservableError => "observable_error".into(),
            Self::SolutionQuality => "solution_quality".into(),
            Self::T1 => "t1".into(),
            Self::T2 => "t2".into(),
            Self::T2Star => "t2_star".into(),
            Self::TPhi => "t_phi".into(),
            Self::Crosstalk => "crosstalk".into(),
            Self::Drift => "drift".into(),
            Self::Stability => "stability".into(),
            Self::SpaceTimeVolume => "space_time_volume".into(),
            Self::ResourceOverhead => "resource_overhead".into(),
            Self::Custom(value) => value.clone(),
        }
    }

    /// Returns whether the metric is mathematically expected to lie in [0, 1].
    pub fn requires_unit_interval(&self) -> bool {
        matches!(
            self,
            Self::Probability
                | Self::SuccessProbability
                | Self::StateFidelity
                | Self::ProcessFidelity
                | Self::AverageGateFidelity
                | Self::EntanglementFidelity
                | Self::HellingerFidelity
                | Self::ClassicalFidelity
                | Self::Fidelity
                | Self::ErrorRate
                | Self::ErrorPerGate
                | Self::ErrorPerClifford
                | Self::CycleError
                | Self::GateInfidelity
                | Self::ProcessInfidelity
                | Self::ReadoutFidelity
                | Self::ReadoutError
                | Self::StatePreparationError
                | Self::SpamError
                | Self::LeakageRate
                | Self::LogicalErrorRate
                | Self::LogicalFidelity
                | Self::PhysicalErrorRate
                | Self::DecoderFailureProbability
                | Self::ApproximationRatio
                | Self::SolutionQuality
        )
    }

    /// Returns whether larger values normally indicate better performance.
    ///
    /// This is a semantic default, not a substitute for protocol-specific
    /// interpretation.
    pub fn default_direction(&self) -> MetricDirection {
        match self {
            Self::ErrorRate
            | Self::ErrorPerGate
            | Self::ErrorPerClifford
            | Self::CycleError
            | Self::GateInfidelity
            | Self::ProcessInfidelity
            | Self::ReadoutError
            | Self::StatePreparationError
            | Self::SpamError
            | Self::LeakageRate
            | Self::LogicalErrorRate
            | Self::PhysicalErrorRate
            | Self::DecoderFailureProbability
            | Self::Runtime
            | Self::CompilationTime
            | Self::QueueTime
            | Self::SubmissionTime
            | Self::ExecutionTime
            | Self::ReadoutTime
            | Self::AnalysisTime
            | Self::TotalWallTime
            | Self::Latency
            | Self::Memory
            | Self::Energy
            | Self::EstimationError
            | Self::EnergyError
            | Self::ObservableError
            | Self::Drift
            | Self::Crosstalk
            | Self::ResourceOverhead => MetricDirection::LowerIsBetter,

            _ => MetricDirection::HigherIsBetter,
        }
    }
}

/// Unit associated with a metric.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MetricUnit {
    /// Dimensionless quantity.
    Dimensionless,

    /// Probability/fraction in [0, 1].
    Probability,

    /// Percentage, represented as 0..100.
    Percent,

    /// Nanoseconds.
    Nanoseconds,

    /// Microseconds.
    Microseconds,

    /// Milliseconds.
    Milliseconds,

    /// Seconds.
    Seconds,

    /// Hertz.
    Hertz,

    /// Kilohertz.
    Kilohertz,

    /// Megahertz.
    Megahertz,

    /// Gigahertz.
    Gigahertz,

    /// Bytes.
    Bytes,

    /// Kibibytes.
    Kibibytes,

    /// Mebibytes.
    Mebibytes,

    /// Gibibytes.
    Gibibytes,

    /// Joules.
    Joules,

    /// Watts.
    Watts,

    /// Qubits.
    Qubits,

    /// Logical qubits.
    LogicalQubits,

    /// Physical qubits.
    PhysicalQubits,

    /// Gates.
    Gates,

    /// Two-qubit gates.
    TwoQubitGates,

    /// Layers.
    Layers,

    /// Shots.
    Shots,

    /// Circuits.
    Circuits,

    /// Operations.
    Operations,

    /// T gates.
    TGates,

    /// Space-time volume units.
    SpaceTimeVolume,

    /// Custom unit.
    Custom(String),
}

impl MetricUnit {
    /// Stable machine-readable identifier.
    pub fn id(&self) -> String {
        match self {
            Self::Dimensionless => "dimensionless".into(),
            Self::Probability => "probability".into(),
            Self::Percent => "percent".into(),
            Self::Nanoseconds => "nanoseconds".into(),
            Self::Microseconds => "microseconds".into(),
            Self::Milliseconds => "milliseconds".into(),
            Self::Seconds => "seconds".into(),
            Self::Hertz => "hertz".into(),
            Self::Kilohertz => "kilohertz".into(),
            Self::Megahertz => "megahertz".into(),
            Self::Gigahertz => "gigahertz".into(),
            Self::Bytes => "bytes".into(),
            Self::Kibibytes => "kibibytes".into(),
            Self::Mebibytes => "mebibytes".into(),
            Self::Gibibytes => "gibibytes".into(),
            Self::Joules => "joules".into(),
            Self::Watts => "watts".into(),
            Self::Qubits => "qubits".into(),
            Self::LogicalQubits => "logical_qubits".into(),
            Self::PhysicalQubits => "physical_qubits".into(),
            Self::Gates => "gates".into(),
            Self::TwoQubitGates => "two_qubit_gates".into(),
            Self::Layers => "layers".into(),
            Self::Shots => "shots".into(),
            Self::Circuits => "circuits".into(),
            Self::Operations => "operations".into(),
            Self::TGates => "t_gates".into(),
            Self::SpaceTimeVolume => "space_time_volume".into(),
            Self::Custom(value) => value.clone(),
        }
    }
}

/// Indicates how a metric should normally be optimized.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MetricDirection {
    /// Larger values are normally better.
    HigherIsBetter,

    /// Smaller values are normally better.
    LowerIsBetter,

    /// There is no generic ordering.
    Neutral,
}

/// Indicates the statistical quality of a metric.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MetricQuality {
    /// Directly observed/measured without a statistical fit.
    Observed,

    /// Calculated deterministically from observations.
    Derived,

    /// Estimated through a statistical model.
    Estimated,

    /// Obtained through a fitted model.
    Fitted,

    /// Approximation rather than an exact measurement.
    Approximate,

    /// Result is insufficiently reliable.
    Uncertain,

    /// Result failed validation or cannot be trusted.
    Invalid,
}

/// Confidence information attached to a metric.
///
/// `level` is represented as a fraction, e.g. 0.95 means 95%.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetricConfidence {
    /// Confidence level in [0, 1].
    pub level: FiniteF64,

    /// Lower confidence bound.
    pub lower: FiniteF64,

    /// Upper confidence bound.
    pub upper: FiniteF64,

    /// Method used to calculate the interval.
    pub method: ConfidenceMethod,
}

impl MetricConfidence {
    /// Creates a confidence interval.
    pub fn new(
        level: f64,
        lower: f64,
        upper: f64,
        method: ConfidenceMethod,
    ) -> MetricResult<Self> {
        let level = FiniteF64::new(level)?;
        let lower = FiniteF64::new(lower)?;
        let upper = FiniteF64::new(upper)?;

        if !(0.0 < level.get() && level.get() < 1.0) {
            return Err(MetricError::InvalidConfidenceLevel {
                level: level.get(),
            });
        }

        if lower.get() > upper.get() {
            return Err(MetricError::InvalidConfidenceInterval {
                lower: lower.get(),
                upper: upper.get(),
            });
        }

        Ok(Self {
            level,
            lower,
            upper,
            method,
        })
    }

    /// Width of the confidence interval.
    pub fn width(&self) -> f64 {
        self.upper.get() - self.lower.get()
    }

    /// Whether the supplied value is contained in the interval.
    pub fn contains(&self, value: f64) -> bool {
        self.lower.get() <= value && value <= self.upper.get()
    }
}

/// Statistical confidence-interval method.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfidenceMethod {
    /// Wilson score interval.
    Wilson,

    /// Exact Clopper-Pearson interval.
    ClopperPearson,

    /// Normal approximation.
    NormalApproximation,

    /// Bootstrap interval.
    Bootstrap,

    /// Bayesian credible interval.
    Bayesian,

    /// Backend/provider supplied interval.
    BackendProvided,

    /// Custom method.
    Custom(String),
}

/// Minimal provenance reference.
///
/// The full provenance model belongs in `core::provenance`.
///
/// This lightweight reference keeps `metric.rs` independent while allowing
/// future provenance integration without changing the metric's semantic model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProvenanceRef {
    /// Stable identifier for the source/experiment.
    pub source_id: String,

    /// Optional hash identifying the source data.
    pub data_hash: Option<String>,
}

impl ProvenanceRef {
    /// Creates a provenance reference.
    pub fn new(source_id: impl Into<String>) -> MetricResult<Self> {
        let source_id = source_id.into();

        if source_id.trim().is_empty() {
            return Err(MetricError::EmptyIdentifier {
                field: "provenance.source_id",
            });
        }

        Ok(Self {
            source_id,
            data_hash: None,
        })
    }

    /// Adds a source-data hash.
    pub fn with_data_hash(mut self, hash: impl Into<String>) -> MetricResult<Self> {
        let hash = hash.into();

        if hash.trim().is_empty() {
            return Err(MetricError::EmptyIdentifier {
                field: "provenance.data_hash",
            });
        }

        self.data_hash = Some(hash);
        Ok(self)
    }
}

/// Canonical universal quantum-benchmarking metric.
///
/// A metric is deliberately richer than a naked `f64`.
///
/// ```text
/// Metric
/// ├── kind
/// ├── unit
/// ├── value
/// ├── uncertainty
/// ├── confidence interval
/// ├── sample count
/// ├── direction
/// ├── quality
/// ├── provenance
/// └── metadata
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Metric {
    /// Semantic metric kind.
    pub kind: MetricKind,

    /// Unit associated with the value.
    pub unit: MetricUnit,

    /// Measured/derived value.
    pub value: FiniteF64,

    /// Optional one-sided or symmetric standard uncertainty.
    pub uncertainty: Option<FiniteF64>,

    /// Optional confidence interval.
    pub confidence: Option<MetricConfidence>,

    /// Number of observations used to produce this metric.
    pub sample_count: Option<u64>,

    /// Number of shots contributing to the metric, where meaningful.
    pub shot_count: Option<u64>,

    /// Number of circuits contributing to the metric, where meaningful.
    pub circuit_count: Option<u64>,

    /// Generic optimization direction.
    pub direction: MetricDirection,

    /// Statistical/semantic quality classification.
    pub quality: MetricQuality,

    /// Optional provenance.
    pub provenance: Option<ProvenanceRef>,

    /// Human-readable description.
    pub description: Option<String>,

    /// Optional protocol-specific metadata encoded as key/value strings.
    ///
    /// This deliberately does not use `serde_json::Value`, keeping this
    /// foundational module lightweight and dependency-stable.
    pub metadata: Vec<MetricMetadata>,
}

/// Additional metadata attached to a metric.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MetricMetadata {
    /// Metadata key.
    pub key: String,

    /// Metadata value.
    pub value: String,
}

impl MetricMetadata {
    /// Creates a metadata entry.
    pub fn new(
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> MetricResult<Self> {
        let key = key.into();
        let value = value.into();

        if key.trim().is_empty() {
            return Err(MetricError::EmptyIdentifier {
                field: "metadata.key",
            });
        }

        Ok(Self { key, value })
    }
}

impl Metric {
    /// Creates a metric with the minimum required information.
    pub fn new(
        kind: MetricKind,
        unit: MetricUnit,
        value: f64,
    ) -> MetricResult<Self> {
        let metric = Self {
            direction: kind.default_direction(),
            kind,
            unit,
            value: FiniteF64::new(value)?,
            uncertainty: None,
            confidence: None,
            sample_count: None,
            shot_count: None,
            circuit_count: None,
            quality: MetricQuality::Derived,
            provenance: None,
            description: None,
            metadata: Vec::new(),
        };

        metric.validate()?;
        Ok(metric)
    }

    /// Creates an observed metric.
    pub fn observed(
        kind: MetricKind,
        unit: MetricUnit,
        value: f64,
    ) -> MetricResult<Self> {
        let mut metric = Self::new(kind, unit, value)?;
        metric.quality = MetricQuality::Observed;
        Ok(metric)
    }

    /// Creates an estimated metric.
    pub fn estimated(
        kind: MetricKind,
        unit: MetricUnit,
        value: f64,
    ) -> MetricResult<Self> {
        let mut metric = Self::new(kind, unit, value)?;
        metric.quality = MetricQuality::Estimated;
        Ok(metric)
    }

    /// Sets uncertainty.
    pub fn with_uncertainty(mut self, uncertainty: f64) -> MetricResult<Self> {
        let uncertainty = FiniteF64::new(uncertainty)?;

        if uncertainty.get() < 0.0 {
            return Err(MetricError::NegativeUncertainty {
                value: uncertainty.get(),
            });
        }

        self.uncertainty = Some(uncertainty);
        self.validate()?;
        Ok(self)
    }

    /// Sets a confidence interval.
    pub fn with_confidence(
        mut self,
        confidence: MetricConfidence,
    ) -> MetricResult<Self> {
        if !confidence.contains(self.value.get()) {
            return Err(MetricError::ValueOutsideConfidenceInterval {
                value: self.value.get(),
                lower: confidence.lower.get(),
                upper: confidence.upper.get(),
            });
        }

        self.confidence = Some(confidence);
        self.quality = MetricQuality::Estimated;
        self.validate()?;
        Ok(self)
    }

    /// Sets the number of samples.
    pub fn with_sample_count(mut self, count: u64) -> MetricResult<Self> {
        if count == 0 {
            return Err(MetricError::ZeroSampleCount);
        }

        self.sample_count = Some(count);
        Ok(self)
    }

    /// Sets the number of shots.
    pub fn with_shot_count(mut self, count: u64) -> MetricResult<Self> {
        if count == 0 {
            return Err(MetricError::ZeroShotCount);
        }

        self.shot_count = Some(count);
        Ok(self)
    }

    /// Sets the number of circuits.
    pub fn with_circuit_count(mut self, count: u64) -> MetricResult<Self> {
        if count == 0 {
            return Err(MetricError::ZeroCircuitCount);
        }

        self.circuit_count = Some(count);
        Ok(self)
    }

    /// Overrides the default optimization direction.
    pub fn with_direction(mut self, direction: MetricDirection) -> Self {
        self.direction = direction;
        self
    }

    /// Sets the quality classification.
    pub fn with_quality(mut self, quality: MetricQuality) -> Self {
        self.quality = quality;
        self
    }

    /// Attaches provenance.
    pub fn with_provenance(mut self, provenance: ProvenanceRef) -> Self {
        self.provenance = Some(provenance);
        self
    }

    /// Adds human-readable description.
    pub fn with_description(
        mut self,
        description: impl Into<String>,
    ) -> MetricResult<Self> {
        let description = description.into();

        if description.trim().is_empty() {
            return Err(MetricError::EmptyIdentifier {
                field: "description",
            });
        }

        self.description = Some(description);
        Ok(self)
    }

    /// Adds metadata.
    pub fn with_metadata(
        mut self,
        metadata: MetricMetadata,
    ) -> Self {
        self.metadata.push(metadata);
        self
    }

    /// Returns the stable metric kind identifier.
    pub fn kind_id(&self) -> String {
        self.kind.id()
    }

    /// Returns the stable unit identifier.
    pub fn unit_id(&self) -> String {
        self.unit.id()
    }

    /// Validates the complete metric.
    pub fn validate(&self) -> MetricResult<()> {
        if !self.value.get().is_finite() {
            return Err(MetricError::NonFiniteValue {
                value: self.value.get(),
            });
        }

        if let Some(uncertainty) = self.uncertainty {
            if uncertainty.get() < 0.0 {
                return Err(MetricError::NegativeUncertainty {
                    value: uncertainty.get(),
                });
            }
        }

        if self.kind.requires_unit_interval() {
            let value = self.value.get();

            if !(0.0..=1.0).contains(&value) {
                return Err(MetricError::OutOfRange {
                    metric: self.kind.id(),
                    value,
                    minimum: 0.0,
                    maximum: 1.0,
                });
            }

            if let Some(confidence) = &self.confidence {
                if confidence.lower.get() < 0.0 || confidence.upper.get() > 1.0 {
                    return Err(MetricError::ConfidenceIntervalOutOfRange {
                        metric: self.kind.id(),
                        lower: confidence.lower.get(),
                        upper: confidence.upper.get(),
                    });
                }
            }
        }

        if let Some(confidence) = &self.confidence {
            if !confidence.contains(self.value.get()) {
                return Err(MetricError::ValueOutsideConfidenceInterval {
                    value: self.value.get(),
                    lower: confidence.lower.get(),
                    upper: confidence.upper.get(),
                });
            }
        }

        if let Some(sample_count) = self.sample_count {
            if sample_count == 0 {
                return Err(MetricError::ZeroSampleCount);
            }
        }

        if let Some(shot_count) = self.shot_count {
            if shot_count == 0 {
                return Err(MetricError::ZeroShotCount);
            }
        }

        if let Some(circuit_count) = self.circuit_count {
            if circuit_count == 0 {
                return Err(MetricError::ZeroCircuitCount);
            }
        }

        self.validate_unit_compatibility()?;

        Ok(())
    }

    /// Validates that the metric kind and unit are semantically compatible.
    pub fn validate_unit_compatibility(&self) -> MetricResult<()> {
        let valid = match &self.kind {
            MetricKind::Probability
            | MetricKind::SuccessProbability
            | MetricKind::StateFidelity
            | MetricKind::ProcessFidelity
            | MetricKind::AverageGateFidelity
            | MetricKind::EntanglementFidelity
            | MetricKind::HellingerFidelity
            | MetricKind::ClassicalFidelity
            | MetricKind::Fidelity
            | MetricKind::ErrorRate
            | MetricKind::ErrorPerGate
            | MetricKind::ErrorPerClifford
            | MetricKind::CycleError
            | MetricKind::GateInfidelity
            | MetricKind::ProcessInfidelity
            | MetricKind::ReadoutFidelity
            | MetricKind::ReadoutError
            | MetricKind::StatePreparationError
            | MetricKind::SpamError
            | MetricKind::LeakageRate
            | MetricKind::LogicalErrorRate
            | MetricKind::LogicalFidelity
            | MetricKind::PhysicalErrorRate
            | MetricKind::DecoderFailureProbability
            | MetricKind::ApproximationRatio
            | MetricKind::SolutionQuality => matches!(
                self.unit,
                MetricUnit::Dimensionless
                    | MetricUnit::Probability
                    | MetricUnit::Percent
            ),

            MetricKind::TotalVariationDistance | MetricKind::TraceDistance => {
                matches!(
                    self.unit,
                    MetricUnit::Dimensionless
                        | MetricUnit::Probability
                        | MetricUnit::Percent
                )
            }

            MetricKind::QuantumVolume => {
                matches!(self.unit, MetricUnit::Dimensionless)
            }

            MetricKind::QubitCount => {
                matches!(self.unit, MetricUnit::Qubits)
            }

            MetricKind::LogicalQubitCount => {
                matches!(self.unit, MetricUnit::LogicalQubits)
            }

            MetricKind::PhysicalQubitCount => {
                matches!(self.unit, MetricUnit::PhysicalQubits)
            }

            MetricKind::CircuitDepth | MetricKind::TwoQubitDepth => {
                matches!(self.unit, MetricUnit::Layers | MetricUnit::Dimensionless)
            }

            MetricKind::GateCount
            | MetricKind::TwoQubitGateCount
            | MetricKind::MeasurementCount
            | MetricKind::ClassicalOperationCount => {
                matches!(self.unit, MetricUnit::Gates | MetricUnit::Operations)
            }

            MetricKind::TGateCount => {
                matches!(self.unit, MetricUnit::TGates)
            }

            MetricKind::Runtime
            | MetricKind::CompilationTime
            | MetricKind::QueueTime
            | MetricKind::SubmissionTime
            | MetricKind::ExecutionTime
            | MetricKind::ReadoutTime
            | MetricKind::AnalysisTime
            | MetricKind::TotalWallTime
            | MetricKind::Latency
            | MetricKind::T1
            | MetricKind::T2
            | MetricKind::T2Star
            | MetricKind::TPhi
            | MetricKind::TimeToSolution => matches!(
                self.unit,
                MetricUnit::Nanoseconds
                    | MetricUnit::Microseconds
                    | MetricUnit::Milliseconds
                    | MetricUnit::Seconds
            ),

            MetricKind::ShotsPerSecond
            | MetricKind::CircuitsPerSecond
            | MetricKind::GatesPerSecond
            | MetricKind::TwoQubitGatesPerSecond
            | MetricKind::LayersPerSecond
            | MetricKind::Throughput => matches!(
                self.unit,
                MetricUnit::Hertz
                    | MetricUnit::Kilohertz
                    | MetricUnit::Megahertz
                    | MetricUnit::Gigahertz
            ),

            MetricKind::Memory => matches!(
                self.unit,
                MetricUnit::Bytes
                    | MetricUnit::Kibibytes
                    | MetricUnit::Mebibytes
                    | MetricUnit::Gibibytes
            ),

            MetricKind::Energy => {
                matches!(self.unit, MetricUnit::Joules)
            }

            MetricKind::SpaceTimeVolume => {
                matches!(
                    self.unit,
                    MetricUnit::SpaceTimeVolume | MetricUnit::Dimensionless
                )
            }

            _ => true,
        };

        if valid {
            Ok(())
        } else {
            Err(MetricError::IncompatibleUnit {
                metric: self.kind.id(),
                unit: self.unit.id(),
            })
        }
    }
}

/// Errors generated while constructing or validating metrics.
#[derive(Debug, Clone, PartialEq)]
pub enum MetricError {
    /// Floating-point value is NaN or infinity.
    NonFiniteValue { value: f64 },

    /// Value lies outside an allowed range.
    OutOfRange {
        metric: String,
        value: f64,
        minimum: f64,
        maximum: f64,
    },

    /// Confidence level is invalid.
    InvalidConfidenceLevel { level: f64 },

    /// Confidence interval has an invalid ordering.
    InvalidConfidenceInterval { lower: f64, upper: f64 },

    /// Confidence interval lies outside the metric's valid range.
    ConfidenceIntervalOutOfRange {
        metric: String,
        lower: f64,
        upper: f64,
    },

    /// Metric value is not contained by its confidence interval.
    ValueOutsideConfidenceInterval {
        value: f64,
        lower: f64,
        upper: f64,
    },

    /// Uncertainty cannot be negative.
    NegativeUncertainty { value: f64 },

    /// Zero observations are not valid when a count is supplied.
    ZeroSampleCount,

    /// Zero shots are not valid when a shot count is supplied.
    ZeroShotCount,

    /// Zero circuits are not valid when a circuit count is supplied.
    ZeroCircuitCount,

    /// Empty identifier.
    EmptyIdentifier { field: &'static str },

    /// Metric kind and unit are incompatible.
    IncompatibleUnit { metric: String, unit: String },
}

impl fmt::Display for MetricError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFiniteValue { value } => {
                write!(formatter, "metric value must be finite: {value}")
            }

            Self::OutOfRange {
                metric,
                value,
                minimum,
                maximum,
            } => write!(
                formatter,
                "metric '{metric}' has value {value}, outside [{minimum}, {maximum}]"
            ),

            Self::InvalidConfidenceLevel { level } => {
                write!(
                    formatter,
                    "confidence level must be strictly between 0 and 1: {level}"
                )
            }

            Self::InvalidConfidenceInterval { lower, upper } => {
                write!(
                    formatter,
                    "confidence interval is invalid: lower={lower}, upper={upper}"
                )
            }

            Self::ConfidenceIntervalOutOfRange {
                metric,
                lower,
                upper,
            } => write!(
                formatter,
                "confidence interval for '{metric}' is outside its valid range: \
                 [{lower}, {upper}]"
            ),

            Self::ValueOutsideConfidenceInterval {
                value,
                lower,
                upper,
            } => write!(
                formatter,
                "metric value {value} is outside confidence interval \
                 [{lower}, {upper}]"
            ),

            Self::NegativeUncertainty { value } => {
                write!(formatter, "uncertainty cannot be negative: {value}")
            }

            Self::ZeroSampleCount => {
                write!(formatter, "sample count must be greater than zero")
            }

            Self::ZeroShotCount => {
                write!(formatter, "shot count must be greater than zero")
            }

            Self::ZeroCircuitCount => {
                write!(formatter, "circuit count must be greater than zero")
            }

            Self::EmptyIdentifier { field } => {
                write!(formatter, "{field} must not be empty")
            }

            Self::IncompatibleUnit { metric, unit } => {
                write!(
                    formatter,
                    "unit '{unit}' is incompatible with metric '{metric}'"
                )
            }
        }
    }
}

impl std::error::Error for MetricError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finite_values_are_accepted() {
        assert_eq!(FiniteF64::new(1.25).unwrap().get(), 1.25);
    }

    #[test]
    fn_nan_is_rejected() {
        assert!(matches!(
            FiniteF64::new(f64::NAN),
            Err(MetricError::NonFiniteValue { .. })
        ));
    }

    #[test]
    fn infinity_is_rejected() {
        assert!(matches!(
            FiniteF64::new(f64::INFINITY),
            Err(MetricError::NonFiniteValue { .. })
        ));
    }

    #[test]
    fn probability_must_be_between_zero_and_one() {
        assert!(Metric::new(
            MetricKind::Probability,
            MetricUnit::Probability,
            1.1
        )
        .is_err());

        assert!(Metric::new(
            MetricKind::Probability,
            MetricUnit::Probability,
            -0.1
        )
        .is_err());
    }

    #[test]
    fn probability_boundary_values_are_valid() {
        assert!(Metric::new(
            MetricKind::Probability,
            MetricUnit::Probability,
            0.0
        )
        .is_ok());

        assert!(Metric::new(
            MetricKind::Probability,
            MetricUnit::Probability,
            1.0
        )
        .is_ok());
    }

    #[test]
    fn quantum_volume_is_dimensionless() {
        let metric = Metric::new(
            MetricKind::QuantumVolume,
            MetricUnit::Dimensionless,
            256.0,
        );

        assert!(metric.is_ok());
    }

    #[test]
    fn quantum_volume_cannot_use_probability_unit() {
        let metric = Metric::new(
            MetricKind::QuantumVolume,
            MetricUnit::Probability,
            256.0,
        );

        assert!(metric.is_err());
    }

    #[test]
    fn negative_uncertainty_is_rejected() {
        let metric = Metric::new(
            MetricKind::Fidelity,
            MetricUnit::Probability,
            0.95,
        )
        .unwrap()
        .with_uncertainty(-0.01);

        assert!(matches!(
            metric,
            Err(MetricError::NegativeUncertainty { .. })
        ));
    }

    #[test]
    fn confidence_interval_requires_valid_level() {
        assert!(MetricConfidence::new(
            1.0,
            0.9,
            1.0,
            ConfidenceMethod::Wilson
        )
        .is_err());

        assert!(MetricConfidence::new(
            0.0,
            0.9,
            1.0,
            ConfidenceMethod::Wilson
        )
        .is_err());
    }

    #[test]
    fn confidence_interval_requires_ordered_bounds() {
        assert!(MetricConfidence::new(
            0.95,
            0.99,
            0.90,
            ConfidenceMethod::Wilson
        )
        .is_err());
    }

    #[test]
    fn confidence_interval_must_contain_metric_value() {
        let metric = Metric::new(
            MetricKind::Probability,
            MetricUnit::Probability,
            0.50,
        )
        .unwrap();

        let confidence = MetricConfidence::new(
            0.95,
            0.60,
            0.80,
            ConfidenceMethod::Wilson,
        )
        .unwrap();

        assert!(metric.with_confidence(confidence).is_err());
    }

    #[test]
    fn confidence_interval_accepts_metric_value() {
        let metric = Metric::new(
            MetricKind::Probability,
            MetricUnit::Probability,
            0.70,
        )
        .unwrap();

        let confidence = MetricConfidence::new(
            0.95,
            0.60,
            0.80,
            ConfidenceMethod::Wilson,
        )
        .unwrap();

        assert!(metric.with_confidence(confidence).is_ok());
    }

    #[test]
    fn confidence_interval_width_is_correct() {
        let confidence = MetricConfidence::new(
            0.95,
            0.60,
            0.80,
            ConfidenceMethod::Wilson,
        )
        .unwrap();

        assert!((confidence.width() - 0.20).abs() < 1e-12);
    }

    #[test]
    fn sample_count_must_be_positive() {
        let metric = Metric::new(
            MetricKind::Fidelity,
            MetricUnit::Probability,
            0.99,
        )
        .unwrap();

        assert!(metric.with_sample_count(0).is_err());
        assert!(metric.with_sample_count(100).is_ok());
    }

    #[test]
    fn shot_count_must_be_positive() {
        let metric = Metric::new(
            MetricKind::Probability,
            MetricUnit::Probability,
            0.5,
        )
        .unwrap();

        assert!(metric.with_shot_count(0).is_err());
        assert!(metric.with_shot_count(1_000).is_ok());
    }

    #[test]
    fn circuit_count_must_be_positive() {
        let metric = Metric::new(
            MetricKind::QuantumVolume,
            MetricUnit::Dimensionless,
            64.0,
        )
        .unwrap();

        assert!(metric.with_circuit_count(0).is_err());
        assert!(metric.with_circuit_count(100).is_ok());
    }

    #[test]
    fn time_metric_requires_time_unit() {
        assert!(Metric::new(
            MetricKind::ExecutionTime,
            MetricUnit::Milliseconds,
            12.0
        )
        .is_ok());

        assert!(Metric::new(
            MetricKind::ExecutionTime,
            MetricUnit::Qubits,
            12.0
        )
        .is_err());
    }

    #[test]
    fn memory_metric_requires_memory_unit() {
        assert!(Metric::new(
            MetricKind::Memory,
            MetricUnit::Mebibytes,
            128.0
        )
        .is_ok());

        assert!(Metric::new(
            MetricKind::Memory,
            MetricUnit::Seconds,
            128.0
        )
        .is_err());
    }

    #[test]
    fn qubit_count_requires_qubit_unit() {
        assert!(Metric::new(
            MetricKind::QubitCount,
            MetricUnit::Qubits,
            20.0
        )
        .is_ok());

        assert!(Metric::new(
            MetricKind::QubitCount,
            MetricUnit::Seconds,
            20.0
        )
        .is_err());
    }

    #[test]
    fn throughput_accepts_frequency_units() {
        assert!(Metric::new(
            MetricKind::ShotsPerSecond,
            MetricUnit::Hertz,
            1_000.0
        )
        .is_ok());

        assert!(Metric::new(
            MetricKind::ShotsPerSecond,
            MetricUnit::Megahertz,
            0.001
        )
        .is_ok());
    }

    #[test]
    fn error_metrics_default_to_lower_is_better() {
        assert_eq!(
            MetricKind::ErrorRate.default_direction(),
            MetricDirection::LowerIsBetter
        );
    }

    #[test]
    fn fidelity_metrics_default_to_higher_is_better() {
        assert_eq!(
            MetricKind::Fidelity.default_direction(),
            MetricDirection::HigherIsBetter
        );
    }

    #[test]
    fn provenance_requires_source_id() {
        assert!(ProvenanceRef::new("").is_err());
        assert!(ProvenanceRef::new("experiment-001").is_ok());
    }

    #[test]
    fn metadata_requires_key() {
        assert!(MetricMetadata::new("", "value").is_err());
        assert!(MetricMetadata::new("backend", "simulator").is_ok());
    }

    #[test]
    fn builder_supports_production_metadata() {
        let provenance = ProvenanceRef::new("experiment-001")
            .unwrap()
            .with_data_hash("sha256:abc123")
            .unwrap();

        let confidence = MetricConfidence::new(
            0.95,
            0.94,
            0.99,
            ConfidenceMethod::Wilson,
        )
        .unwrap();

        let metric = Metric::observed(
            MetricKind::ReadoutFidelity,
            MetricUnit::Probability,
            0.97,
        )
        .unwrap()
        .with_uncertainty(0.01)
        .unwrap()
        .with_confidence(confidence)
        .unwrap()
        .with_sample_count(10_000)
        .unwrap()
        .with_shot_count(10_000)
        .unwrap()
        .with_provenance(provenance)
        .with_description("Readout fidelity benchmark")
        .unwrap()
        .with_metadata(
            MetricMetadata::new("backend", "local-simulator").unwrap(),
        );

        assert!(metric.validate().is_ok());
        assert_eq!(metric.kind_id(), "readout_fidelity");
        assert_eq!(metric.unit_id(), "probability");
        assert_eq!(metric.metadata.len(), 1);
    }

    #[test]
    fn serde_round_trip_preserves_metric() {
        let metric = Metric::new(
            MetricKind::QuantumVolume,
            MetricUnit::Dimensionless,
            256.0,
        )
        .unwrap();

        let encoded = serde_json::to_string(&metric).unwrap();
        let decoded: Metric = serde_json::from_str(&encoded).unwrap();

        assert_eq!(metric, decoded);
    }

    #[test]
    fn custom_metric_is_supported() {
        let metric = Metric::new(
            MetricKind::Custom("logical_cycle_success".into()),
            MetricUnit::Probability,
            0.999,
        );

        assert!(metric.is_ok());
        assert_eq!(
            metric.unwrap().kind_id(),
            "logical_cycle_success"
        );
    }

    #[test]
    fn custom_unit_is_supported() {
        let metric = Metric::new(
            MetricKind::Custom("custom_metric".into()),
            MetricUnit::Custom("custom_unit".into()),
            42.0,
        );

        assert!(metric.is_ok());
    }
}