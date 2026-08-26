//! Zamani Quantum Benchmarking — Gate Fidelity Protocol
//!
//! Production-grade gate-fidelity protocol boundary.
//!
//! # Responsibility
//!
//! This module converts already-available gate-characterization data into a
//! canonical gate-fidelity benchmark result.
//!
//! It deliberately does NOT:
//!
//! - generate quantum circuits;
//! - execute circuits;
//! - communicate with hardware;
//! - select a backend;
//! - perform transpilation;
//! - perform routing;
//! - perform scheduling;
//! - perform randomized-benchmarking sequence generation;
//! - fit randomized-benchmarking decay curves;
//! - perform process tomography;
//! - perform state tomography;
//! - own Quantum IR;
//! - own backend capability negotiation;
//! - mutate process-global state;
//! - print diagnostics;
//! - silently clamp scientifically invalid input.
//!
//! Those responsibilities belong to the surrounding benchmarking
//! architecture.
//!
//! # Architecture
//!
//! ```text
//! Quantum IR / backend / simulator
//!             │
//!             ▼
//!      execution / protocol data
//!             │
//!       ┌─────┴─────────┐
//!       │               │
//!       ▼               ▼
//! exact unitary       measured / fitted
//! comparison          fidelity data
//!       │               │
//!       └──────┬────────┘
//!              ▼
//! protocols::gate_fidelity
//!              │
//!       ┌──────┴────────┐
//!       ▼               ▼
//! average fidelity   gate infidelity
//!       │               │
//!       └──────┬────────┘
//!              ▼
//!      core::Metric / result
//! ```
//!
//! # Scientific conventions
//!
//! Zamani distinguishes:
//!
//! * average gate fidelity:
//!     F_avg
//!
//! * entanglement/process fidelity:
//!     F_e
//!
//! * average gate infidelity:
//!     r_avg = 1 - F_avg
//!
//! * process/entanglement infidelity:
//!     r_e = 1 - F_e
//!
//! For a d-dimensional channel:
//!
//! ```text
//! F_avg = (d F_e + 1) / (d + 1)
//! ```
//!
//! For two d-dimensional unitary operators U and V:
//!
//! ```text
//! F_e = |Tr(U† V)|² / d²
//!
//! F_avg = (d F_e + 1) / (d + 1)
//! ```
//!
//! The unitary formula is an exact mathematical comparison of the supplied
//! matrices. It is NOT a statement that a hardware experiment has measured
//! that fidelity.
//!
//! # Randomized benchmarking
//!
//! This module accepts an already-fitted RB decay parameter `p`.
//!
//! Under the standard depolarizing RB model:
//!
//! ```text
//! r_avg = (d - 1) / d * (1 - p)
//! ```
//!
//! The RB fit itself belongs to `statistics::regression` and/or
//! `protocols::randomized_benchmarking`.
//!
//! This module records the model assumption explicitly instead of presenting
//! an RB-derived estimate as a direct measurement.
//!
//! # Interleaved RB
//!
//! Interleaved RB is intentionally NOT implemented here.
//!
//! `protocols/interleaved_rb.rs` owns the reference/interleaved decay analysis.
//! Once it produces a target-gate fidelity/error estimate, it may feed that
//! result into this module's canonical result model.
//!
//! # Cycle benchmarking
//!
//! Cycle benchmarking is also intentionally NOT implemented here.
//!
//! `protocols/cycle_benchmarking.rs` owns cycle-level process fidelity.
//!
//! This module can normalize a cycle-derived gate/cycle fidelity when the
//! caller explicitly identifies it as such.
//!
//! # Process tomography
//!
//! Process tomography belongs to `protocols/process_fidelity.rs` and/or
//! `protocols/tomography.rs`.
//!
//! This module can consume the resulting process fidelity, but it does not
//! reconstruct the process itself.
//!
//! # Production guarantees
//!
//! This module:
//!
//! - rejects NaN and infinity;
//! - rejects zero-dimensional inputs;
//! - rejects non-square unitary matrices;
//! - rejects dimension mismatches;
//! - validates supplied unitary matrices;
//! - validates probability/fidelity values;
//! - validates confidence intervals;
//! - validates uncertainty values;
//! - avoids integer overflow in dimension arithmetic;
//! - avoids unsafe code;
//! - avoids hidden global state;
//! - records the source method;
//! - records mathematical assumptions;
//! - preserves confidence information;
//! - preserves sample/circuit/shot counts where supplied;
//! - distinguishes observed, derived, estimated and fitted metrics;
//! - does not silently turn process fidelity into average gate fidelity;
//! - does not silently turn RB estimates into exact measurements;
//! - is deterministic for identical inputs;
//! - has no hardware dependency.
//!
//! # Integration
//!
//! Direct dependencies:
//!
//! ```text
//! protocols::gate_fidelity
//!        │
//!        ├── core::metric
//!        ├── metrics::fidelity
//!        └── metrics::gate_error
//! ```
//!
//! It does NOT depend on:
//!
//! ```text
//! execution
//! hardware
//! frontend
//! IR
//! algorithms
//! runtime
//! ```
//!
//! The intended future integration is:
//!
//! ```text
//! protocols::gate_fidelity
//!       │
//!       ├── simulator exact-unitary result
//!       ├── process tomography result
//!       ├── RB result
//!       ├── IRB result
//!       └── cycle result
//!              │
//!              ▼
//!       core::Metric
//!              │
//!              ▼
//!       core::BenchmarkResult
//! ```
//!
//! # Rust compatibility
//!
//! Designed for:
//!
//! - Rust 1.97
//! - Rust 1.97.1
//! - Rust 2021
//!
//! No nightly features are required.

use serde::{Deserialize, Serialize};

use crate::quantum::benchmarking::core::metric::{
    ConfidenceMethod,
    Metric,
    MetricConfidence,
    MetricError,
    MetricKind,
    MetricMetadata,
    MetricResult,
    MetricUnit,
    ProvenanceRef,
};

use crate::quantum::benchmarking::metrics::fidelity::{
    Complex64,
    ComplexMatrix,
};

use crate::quantum::benchmarking::metrics::gate_error::{
    average_gate_fidelity_from_entanglement_fidelity,
    entanglement_fidelity_from_average_gate_fidelity,
};

// =============================================================================
// Public constants
// =============================================================================

/// Stable benchmark identifier.
pub const GATE_FIDELITY_BENCHMARK_ID: &str = "gate_fidelity";

/// Version of the protocol result contract.
pub const GATE_FIDELITY_PROTOCOL_VERSION: &str = "1";

/// Default numerical tolerance used for unitary comparisons.
pub const DEFAULT_UNITARY_TOLERANCE: f64 = 1.0e-10;

/// Maximum integer that can be represented exactly by an IEEE-754 f64.
///
/// Dimensions above this value are rejected because the fidelity formulas use
/// floating-point arithmetic and an exact integer dimension is required for
/// scientifically meaningful conversion.
const MAX_EXACT_F64_INTEGER: usize = 9_007_199_254_740_992;

/// Small tolerance for accepting a result that differs from [0, 1] only by
/// floating-point roundoff.
const RESULT_BOUND_TOLERANCE: f64 = 1.0e-12;

// =============================================================================
// Error type
// =============================================================================

/// Errors produced by the gate-fidelity protocol.
#[derive(Debug, Clone, PartialEq)]
pub enum GateFidelityError {
    /// A numerical input was NaN or infinite.
    NonFiniteValue {
        /// Location of the invalid value.
        context: &'static str,
    },

    /// A supplied dimension was zero.
    InvalidDimension {
        /// Supplied dimension.
        dimension: usize,
    },

    /// A dimension is too large for exact floating-point representation.
    DimensionNotRepresentable {
        /// Supplied dimension.
        dimension: usize,
    },

    /// Two matrices have different dimensions.
    DimensionMismatch {
        /// First matrix dimension.
        left: usize,

        /// Second matrix dimension.
        right: usize,
    },

    /// A matrix is not square.
    MatrixNotSquare {
        /// Number of rows.
        rows: usize,

        /// Number of columns.
        columns: usize,
    },

    /// A matrix contains non-finite values.
    MatrixContainsNonFinite {
        /// Linear matrix index.
        index: usize,
    },

    /// A supplied matrix is not unitary within tolerance.
    MatrixNotUnitary {
        /// Maximum deviation from identity.
        maximum_deviation: f64,

        /// Allowed tolerance.
        tolerance: f64,
    },

    /// The numerical tolerance is invalid.
    InvalidTolerance {
        /// Supplied tolerance.
        value: f64,
    },

    /// A fidelity was outside [0, 1].
    FidelityOutOfRange {
        /// Supplied value.
        value: f64,
    },

    /// An uncertainty was negative or non-finite.
    InvalidUncertainty {
        /// Supplied uncertainty.
        value: f64,
    },

    /// A confidence interval was invalid.
    InvalidConfidenceInterval {
        /// Lower bound.
        lower: f64,

        /// Upper bound.
        upper: f64,
    },

    /// A confidence level was invalid.
    InvalidConfidenceLevel {
        /// Confidence level.
        level: f64,
    },

    /// A confidence interval method is incompatible with the supplied data.
    InvalidConfidenceMethod,

    /// An observation count was zero.
    ZeroObservationCount,

    /// A circuit count was zero.
    ZeroCircuitCount,

    /// A shot count was zero.
    ZeroShotCount,

    /// An RB decay parameter was invalid.
    InvalidDecayParameter {
        /// Supplied decay parameter.
        value: f64,
    },

    /// RB requires a Hilbert-space dimension greater than one.
    InvalidRbDimension {
        /// Supplied dimension.
        dimension: u64,
    },

    /// A derived metric became non-finite.
    NonFiniteDerivedValue {
        /// Name of the quantity.
        quantity: &'static str,
    },

    /// The underlying canonical metric subsystem rejected the result.
    Metric(MetricError),
}

impl From<MetricError> for GateFidelityError {
    fn from(error: MetricError) -> Self {
        Self::Metric(error)
    }
}

impl std::fmt::Display for GateFidelityError {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::NonFiniteValue { context } => {
                write!(
                    formatter,
                    "gate-fidelity input contains a non-finite value: {context}"
                )
            }

            Self::InvalidDimension { dimension } => {
                write!(
                    formatter,
                    "gate-fidelity dimension must be greater than zero: {dimension}"
                )
            }

            Self::DimensionNotRepresentable { dimension } => {
                write!(
                    formatter,
                    "gate-fidelity dimension {dimension} is too large \
                     for exact floating-point representation"
                )
            }

            Self::DimensionMismatch { left, right } => {
                write!(
                    formatter,
                    "gate-fidelity dimensions do not match: \
                     left={left}, right={right}"
                )
            }

            Self::MatrixNotSquare { rows, columns } => {
                write!(
                    formatter,
                    "gate-fidelity matrix must be square: {rows}x{columns}"
                )
            }

            Self::MatrixContainsNonFinite { index } => {
                write!(
                    formatter,
                    "gate-fidelity matrix contains a non-finite value \
                     at index {index}"
                )
            }

            Self::MatrixNotUnitary {
                maximum_deviation,
                tolerance,
            } => {
                write!(
                    formatter,
                    "gate-fidelity matrix is not unitary: \
                     maximum deviation={maximum_deviation}, \
                     tolerance={tolerance}"
                )
            }

            Self::InvalidTolerance { value } => {
                write!(
                    formatter,
                    "gate-fidelity tolerance must be finite and >= 0: {value}"
                )
            }

            Self::FidelityOutOfRange { value } => {
                write!(
                    formatter,
                    "gate fidelity must be in [0, 1], got {value}"
                )
            }

            Self::InvalidUncertainty { value } => {
                write!(
                    formatter,
                    "gate-fidelity uncertainty must be finite and >= 0: {value}"
                )
            }

            Self::InvalidConfidenceInterval {
                lower,
                upper,
            } => {
                write!(
                    formatter,
                    "invalid gate-fidelity confidence interval: \
                     lower={lower}, upper={upper}"
                )
            }

            Self::InvalidConfidenceLevel { level } => {
                write!(
                    formatter,
                    "invalid gate-fidelity confidence level: {level}"
                )
            }

            Self::InvalidConfidenceMethod => {
                write!(
                    formatter,
                    "invalid confidence method for gate-fidelity result"
                )
            }

            Self::ZeroObservationCount => {
                write!(
                    formatter,
                    "gate-fidelity observation count must be greater than zero"
                )
            }

            Self::ZeroCircuitCount => {
                write!(
                    formatter,
                    "gate-fidelity circuit count must be greater than zero"
                )
            }

            Self::ZeroShotCount => {
                write!(
                    formatter,
                    "gate-fidelity shot count must be greater than zero"
                )
            }

            Self::InvalidDecayParameter { value } => {
                write!(
                    formatter,
                    "RB decay parameter must be finite and in [0, 1]: {value}"
                )
            }

            Self::InvalidRbDimension { dimension } => {
                write!(
                    formatter,
                    "RB dimension must be at least 2: {dimension}"
                )
            }

            Self::NonFiniteDerivedValue { quantity } => {
                write!(
                    formatter,
                    "derived gate-fidelity quantity is non-finite: {quantity}"
                )
            }

            Self::Metric(error) => error.fmt(formatter),
        }
    }
}

impl std::error::Error for GateFidelityError {}

// =============================================================================
// Fidelity source
// =============================================================================

/// Identifies how the gate-fidelity value was obtained.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GateFidelitySource {
    /// Exact mathematical comparison of two supplied unitary matrices.
    ExactUnitaryComparison,

    /// Process/entanglement fidelity supplied by another characterization
    /// protocol.
    ProcessFidelity,

    /// Average gate fidelity supplied directly by another protocol.
    AverageGateFidelity,

    /// Average gate fidelity derived from an already-fitted RB decay
    /// parameter.
    RandomizedBenchmarking,

    /// Gate fidelity supplied by an interleaved-RB analysis.
    InterleavedRandomizedBenchmarking,

    /// Gate/cycle fidelity supplied by cycle benchmarking.
    CycleBenchmarking,

    /// Gate fidelity obtained from process tomography.
    ProcessTomography,

    /// Custom externally-defined source.
    Custom(String),
}

impl GateFidelitySource {
    /// Stable machine-readable identifier.
    pub fn id(&self) -> String {
        match self {
            Self::ExactUnitaryComparison => {
                "exact_unitary_comparison".to_string()
            }

            Self::ProcessFidelity => {
                "process_fidelity".to_string()
            }

            Self::AverageGateFidelity => {
                "average_gate_fidelity".to_string()
            }

            Self::RandomizedBenchmarking => {
                "randomized_benchmarking".to_string()
            }

            Self::InterleavedRandomizedBenchmarking => {
                "interleaved_randomized_benchmarking".to_string()
            }

            Self::CycleBenchmarking => {
                "cycle_benchmarking".to_string()
            }

            Self::ProcessTomography => {
                "process_tomography".to_string()
            }

            Self::Custom(value) => value.clone(),
        }
    }
}

// =============================================================================
// Fidelity convention
// =============================================================================

/// Identifies which fidelity quantity is the primary source quantity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GateFidelityQuantity {
    /// Average gate fidelity.
    AverageGate,

    /// Entanglement/process fidelity.
    Process,
}

impl GateFidelityQuantity {
    /// Stable machine-readable identifier.
    pub const fn id(self) -> &'static str {
        match self {
            Self::AverageGate => "average_gate_fidelity",
            Self::Process => "process_fidelity",
        }
    }
}

// =============================================================================
// Statistical assumptions
// =============================================================================

/// Assumptions attached to the gate-fidelity estimate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GateFidelityAssumptions {
    /// Whether the value came from a statistical model.
    pub model_based: bool,

    /// Whether a depolarizing RB model was used.
    pub depolarizing_rb_model: bool,

    /// Whether the supplied value was measured directly.
    pub directly_observed: bool,

    /// Human-readable assumptions.
    pub statements: Vec<String>,
}

impl GateFidelityAssumptions {
    /// Creates assumptions for an exact unitary comparison.
    pub fn exact_unitary() -> Self {
        Self {
            model_based: false,
            depolarizing_rb_model: false,
            directly_observed: false,
            statements: vec![
                "Fidelity is calculated exactly from the supplied unitary \
                 matrices within floating-point precision."
                    .to_string(),
                "The supplied matrices are assumed to represent the \
                 implemented and ideal unitary channels."
                    .to_string(),
            ],
        }
    }

    /// Creates assumptions for a direct average-gate-fidelity result.
    pub fn direct_average_gate() -> Self {
        Self {
            model_based: false,
            depolarizing_rb_model: false,
            directly_observed: true,
            statements: vec![
                "The supplied value is treated as an already-established \
                 average gate fidelity."
                    .to_string(),
                "No protocol-specific statistical inference is performed \
                 by this module."
                    .to_string(),
            ],
        }
    }

    /// Creates assumptions for a process-fidelity result.
    pub fn direct_process() -> Self {
        Self {
            model_based: false,
            depolarizing_rb_model: false,
            directly_observed: true,
            statements: vec![
                "The supplied value is treated as an already-established \
                 entanglement/process fidelity."
                    .to_string(),
                "Conversion to average gate fidelity uses the standard \
                 d-dimensional channel relation."
                    .to_string(),
            ],
        }
    }

    /// Creates assumptions for an RB-derived estimate.
    pub fn randomized_benchmarking() -> Self {
        Self {
            model_based: true,
            depolarizing_rb_model: true,
            directly_observed: false,
            statements: vec![
                "The RB decay parameter is assumed to have already been \
                 fitted by the randomized-benchmarking protocol."
                    .to_string(),
                "The conversion uses the standard depolarizing RB model."
                    .to_string(),
                "Noise assumptions and RB fit quality belong to the \
                 originating RB protocol and must be retained there."
                    .to_string(),
            ],
        }
    }
}

// =============================================================================
// Protocol metadata
// =============================================================================

/// Configuration controlling the gate-fidelity protocol boundary.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GateFidelityConfig {
    /// Numerical tolerance for exact unitary validation.
    pub unitary_tolerance: f64,

    /// Optional provenance reference.
    pub provenance: Option<ProvenanceRef>,
}

impl Default for GateFidelityConfig {
    fn default() -> Self {
        Self {
            unitary_tolerance: DEFAULT_UNITARY_TOLERANCE,
            provenance: None,
        }
    }
}

impl GateFidelityConfig {
    /// Validates the configuration.
    pub fn validate(&self) -> Result<(), GateFidelityError> {
        validate_tolerance(self.unitary_tolerance)
    }

    /// Attaches provenance.
    pub fn with_provenance(
        mut self,
        provenance: ProvenanceRef,
    ) -> Self {
        self.provenance = Some(provenance);
        self
    }
}

// =============================================================================
// Canonical gate-fidelity result
// =============================================================================

/// Canonical gate-fidelity protocol result.
///
/// The primary fidelity and the derived infidelity are both represented as
/// universal Zamani `Metric` values.
///
/// The result deliberately retains the source method and assumptions because
/// equal numerical fidelities obtained through different protocols are not
/// necessarily scientifically interchangeable.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GateFidelityResult {
    /// Stable benchmark identifier.
    pub benchmark_id: String,

    /// Protocol version.
    pub protocol_version: String,

    /// Source protocol/method.
    pub source: GateFidelitySource,

    /// Primary fidelity quantity.
    pub primary_quantity: GateFidelityQuantity,

    /// Primary fidelity metric.
    pub fidelity: Metric,

    /// Derived average gate fidelity, when available.
    pub average_gate_fidelity: Metric,

    /// Derived process/entanglement fidelity, when available.
    pub process_fidelity: Metric,

    /// Derived average gate infidelity.
    pub gate_infidelity: Metric,

    /// Assumptions attached to the result.
    pub assumptions: GateFidelityAssumptions,

    /// Hilbert-space dimension used for process/average conversion.
    pub hilbert_dimension: u64,
}

impl GateFidelityResult {
    /// Returns the average gate fidelity as a scalar.
    pub fn average_gate_fidelity_value(&self) -> f64 {
        self.average_gate_fidelity.value.get()
    }

    /// Returns the process/entanglement fidelity as a scalar.
    pub fn process_fidelity_value(&self) -> f64 {
        self.process_fidelity.value.get()
    }

    /// Returns average gate infidelity as a scalar.
    pub fn gate_infidelity_value(&self) -> f64 {
        self.gate_infidelity.value.get()
    }
}

// =============================================================================
// Direct average gate fidelity
// =============================================================================

/// Creates a gate-fidelity result from an already-established average gate
/// fidelity.
///
/// This function does not infer how the value was obtained.
///
/// Use this when the caller has already completed:
///
/// - process tomography;
/// - direct fidelity estimation;
/// - RB analysis;
/// - IRB analysis;
/// - another validated characterization protocol.
pub fn from_average_gate_fidelity(
    average_gate_fidelity: f64,
    dimension: u64,
) -> Result<GateFidelityResult, GateFidelityError> {
    from_average_gate_fidelity_with_metadata(
        average_gate_fidelity,
        dimension,
        GateFidelitySource::AverageGateFidelity,
        None,
        None,
        None,
        None,
    )
}

/// Creates a gate-fidelity result from an average gate fidelity while
/// preserving statistical metadata.
pub fn from_average_gate_fidelity_with_metadata(
    average_gate_fidelity: f64,
    dimension: u64,
    source: GateFidelitySource,
    uncertainty: Option<f64>,
    confidence: Option<MetricConfidence>,
    sample_count: Option<u64>,
    circuit_count: Option<u64>,
) -> Result<GateFidelityResult, GateFidelityError> {
    validate_dimension(dimension)?;
    validate_fidelity(average_gate_fidelity)?;
    validate_uncertainty(uncertainty)?;
    validate_confidence(confidence.as_ref())?;
    validate_optional_count(sample_count, CountKind::Observation)?;
    validate_optional_count(circuit_count, CountKind::Circuit)?;

    let mut average_metric = Metric::observed(
        MetricKind::AverageGateFidelity,
        MetricUnit::Probability,
        average_gate_fidelity,
    )?;

    if let Some(value) = uncertainty {
        average_metric = average_metric.with_uncertainty(value)?;
    }

    if let Some(value) = confidence.clone() {
        average_metric = average_metric.with_confidence(value)?;
    }

    if let Some(value) = sample_count {
        average_metric = average_metric.with_sample_count(value)?;
    }

    if let Some(value) = circuit_count {
        average_metric = average_metric.with_circuit_count(value)?;
    }

    average_metric = average_metric
        .with_metadata(MetricMetadata::new(
            "benchmark",
            GATE_FIDELITY_BENCHMARK_ID,
        )?)
        .with_metadata(MetricMetadata::new(
            "protocol_version",
            GATE_FIDELITY_PROTOCOL_VERSION,
        )?)
        .with_metadata(MetricMetadata::new(
            "source",
            source.id(),
        )?)
        .with_metadata(MetricMetadata::new(
            "primary_quantity",
            GateFidelityQuantity::AverageGate.id(),
        )?);

    let process_value =
        process_fidelity_from_average(average_gate_fidelity, dimension)?;

    let mut process_metric = Metric::derived(
        MetricKind::ProcessFidelity,
        MetricUnit::Probability,
        process_value,
    )?;

    process_metric = process_metric
        .with_metadata(MetricMetadata::new(
            "derived_from",
            "average_gate_fidelity",
        )?)
        .with_metadata(MetricMetadata::new(
            "dimension",
            dimension.to_string(),
        )?)
        .with_metadata(MetricMetadata::new(
            "conversion",
            "F_e=((d+1)F_avg-1)/d",
        )?);

    let mut gate_infidelity = Metric::derived(
        MetricKind::GateInfidelity,
        MetricUnit::Probability,
        1.0 - average_gate_fidelity,
    )?;

    gate_infidelity = gate_infidelity
        .with_metadata(MetricMetadata::new(
            "derived_from",
            "average_gate_fidelity",
        )?)
        .with_metadata(MetricMetadata::new(
            "conversion",
            "1-F_avg",
        )?);

    if let Some(value) = uncertainty {
        gate_infidelity = gate_infidelity.with_uncertainty(value)?;
    }

    if let Some(value) = confidence {
        gate_infidelity = gate_infidelity
            .with_confidence(
                transform_fidelity_confidence_to_infidelity(&value)?,
            )?;
    }

    if let Some(value) = sample_count {
        gate_infidelity = gate_infidelity.with_sample_count(value)?;
    }

    if let Some(value) = circuit_count {
        gate_infidelity = gate_infidelity.with_circuit_count(value)?;
    }

    Ok(GateFidelityResult {
        benchmark_id: GATE_FIDELITY_BENCHMARK_ID.to_string(),
        protocol_version: GATE_FIDELITY_PROTOCOL_VERSION.to_string(),
        source,
        primary_quantity: GateFidelityQuantity::AverageGate,
        fidelity: average_metric.clone(),
        average_gate_fidelity: average_metric,
        process_fidelity: process_metric,
        gate_infidelity,
        assumptions: GateFidelityAssumptions::direct_average_gate(),
        hilbert_dimension: dimension,
    })
}

// =============================================================================
// Direct process / entanglement fidelity
// =============================================================================

/// Creates a gate-fidelity result from process/entanglement fidelity.
///
/// The standard d-dimensional relation is used to derive average gate
/// fidelity:
///
/// ```text
/// F_avg = (d F_e + 1) / (d + 1)
/// ```
pub fn from_process_fidelity(
    process_fidelity: f64,
    dimension: u64,
) -> Result<GateFidelityResult, GateFidelityError> {
    from_process_fidelity_with_metadata(
        process_fidelity,
        dimension,
        GateFidelitySource::ProcessFidelity,
        None,
        None,
        None,
        None,
    )
}

/// Process-fidelity constructor with statistical metadata.
pub fn from_process_fidelity_with_metadata(
    process_fidelity: f64,
    dimension: u64,
    source: GateFidelitySource,
    uncertainty: Option<f64>,
    confidence: Option<MetricConfidence>,
    sample_count: Option<u64>,
    circuit_count: Option<u64>,
) -> Result<GateFidelityResult, GateFidelityError> {
    validate_dimension(dimension)?;
    validate_fidelity(process_fidelity)?;
    validate_uncertainty(uncertainty)?;
    validate_confidence(confidence.as_ref())?;
    validate_optional_count(sample_count, CountKind::Observation)?;
    validate_optional_count(circuit_count, CountKind::Circuit)?;

    let average = average_gate_fidelity_from_entanglement_fidelity(
        process_fidelity,
        dimension,
    )?;

    from_average_gate_fidelity_with_metadata(
        average.value.get(),
        dimension,
        source,
        uncertainty.map(|value| {
            propagate_process_uncertainty_to_average(
                value,
                dimension,
            )
        }),
        confidence.map(|value| {
            transform_process_confidence_to_average(
                &value,
                dimension,
            )
        })
        .transpose()?,
        sample_count,
        circuit_count,
    )
}

// =============================================================================
// Exact unitary comparison
// =============================================================================

/// Calculates exact average gate fidelity between two unitary matrices.
///
/// The supplied matrices represent:
///
/// * `ideal` — intended gate;
/// * `actual` — implemented gate.
///
/// For dimension `d`:
///
/// ```text
/// F_e = |Tr(U†V)|² / d²
///
/// F_avg = (d F_e + 1) / (d + 1)
/// ```
///
/// This function validates both matrices as unitary before calculating the
/// result.
pub fn from_unitary_matrices(
    ideal: &ComplexMatrix,
    actual: &ComplexMatrix,
) -> Result<GateFidelityResult, GateFidelityError> {
    from_unitary_matrices_with_tolerance(
        ideal,
        actual,
        DEFAULT_UNITARY_TOLERANCE,
    )
}

/// Exact unitary comparison with explicit validation tolerance.
pub fn from_unitary_matrices_with_tolerance(
    ideal: &ComplexMatrix,
    actual: &ComplexMatrix,
    tolerance: f64,
) -> Result<GateFidelityResult, GateFidelityError> {
    validate_tolerance(tolerance)?;

    validate_unitary_matrix(ideal, tolerance)?;
    validate_unitary_matrix(actual, tolerance)?;

    if ideal.rows() != actual.rows() {
        return Err(GateFidelityError::DimensionMismatch {
            left: ideal.rows(),
            right: actual.rows(),
        });
    }

    let dimension = ideal.rows();

    validate_dimension(dimension)?;

    let process_fidelity =
        unitary_process_fidelity(ideal, actual, tolerance)?;

    let dimension_u64 =
        u64::try_from(dimension).map_err(|_| {
            GateFidelityError::DimensionNotRepresentable {
                dimension,
            }
        })?;

    from_process_fidelity(
        process_fidelity,
        dimension_u64,
    )
    .map(|mut result| {
        result.source =
            GateFidelitySource::ExactUnitaryComparison;
        result.assumptions =
            GateFidelityAssumptions::exact_unitary();

        result
            .fidelity
            .metadata
            .push(MetricMetadata {
                key: "source".to_string(),
                value: GateFidelitySource::ExactUnitaryComparison.id(),
            });

        result
    })
}

// =============================================================================
// Randomized benchmarking
// =============================================================================

/// Converts an already-fitted RB decay parameter into average gate fidelity.
///
/// The standard depolarizing RB relationship is:
///
/// ```text
/// r_avg = (d - 1) / d * (1 - p)
/// F_avg = 1 - r_avg
/// ```
///
/// where:
///
/// * `p` is the fitted exponential decay parameter;
/// * `d` is the Hilbert-space dimension.
///
/// This function does NOT perform RB fitting.
pub fn from_rb_decay(
    decay_parameter: f64,
    dimension: u64,
) -> Result<GateFidelityResult, GateFidelityError> {
    from_rb_decay_with_metadata(
        decay_parameter,
        dimension,
        None,
        None,
        None,
    )
}

/// RB-derived gate fidelity with optional fit metadata.
pub fn from_rb_decay_with_metadata(
    decay_parameter: f64,
    dimension: u64,
    uncertainty: Option<f64>,
    confidence: Option<MetricConfidence>,
    circuit_count: Option<u64>,
) -> Result<GateFidelityResult, GateFidelityError> {
    validate_rb_decay(decay_parameter)?;
    validate_rb_dimension(dimension)?;
    validate_uncertainty(uncertainty)?;
    validate_confidence(confidence.as_ref())?;
    validate_optional_count(circuit_count, CountKind::Circuit)?;

    let d = dimension as f64;

    let error_rate =
        ((d - 1.0) / d) * (1.0 - decay_parameter);

    validate_fidelity(error_rate)?;

    let average_fidelity = 1.0 - error_rate;

    let transformed_uncertainty =
        uncertainty.map(|value| {
            validate_uncertainty(value)
                .map(|_| ((d - 1.0) / d) * value)
                .unwrap_or(value)
        });

    let transformed_confidence =
        confidence
            .as_ref()
            .map(|interval| {
                rb_confidence_to_average_fidelity(
                    interval,
                    dimension,
                )
            })
            .transpose()?;

    let mut result =
        from_average_gate_fidelity_with_metadata(
            average_fidelity,
            dimension,
            GateFidelitySource::RandomizedBenchmarking,
            transformed_uncertainty,
            transformed_confidence,
            None,
            circuit_count,
        )?;

    result.assumptions =
        GateFidelityAssumptions::randomized_benchmarking();

    result.fidelity =
        result
            .fidelity
            .with_quality(
                crate::quantum::benchmarking::core::metric::MetricQuality::Fitted,
            )
            .with_metadata(MetricMetadata::new(
                "rb_decay_parameter",
                decay_parameter.to_string(),
            )?);

    result.average_gate_fidelity =
        result
            .average_gate_fidelity
            .with_quality(
                crate::quantum::benchmarking::core::metric::MetricQuality::Fitted,
            )
            .with_metadata(MetricMetadata::new(
                "rb_decay_parameter",
                decay_parameter.to_string(),
            )?);

    result.gate_infidelity =
        result
            .gate_infidelity
            .with_quality(
                crate::quantum::benchmarking::core::metric::MetricQuality::Fitted,
            )
            .with_metadata(MetricMetadata::new(
                "rb_decay_parameter",
                decay_parameter.to_string(),
            )?)
            .with_metadata(MetricMetadata::new(
                "model",
                "depolarizing_rb",
            )?);

    Ok(result)
}

// =============================================================================
// Confidence transformation
// =============================================================================

/// Transforms a fidelity confidence interval into an infidelity interval.
///
/// ```text
/// [F_low, F_high]
/// ->
/// [1-F_high, 1-F_low]
/// ```
pub fn transform_fidelity_confidence_to_infidelity(
    confidence: &MetricConfidence,
) -> Result<MetricConfidence, GateFidelityError> {
    validate_confidence(Some(confidence))?;

    MetricConfidence::new(
        confidence.level.get(),
        1.0 - confidence.upper.get(),
        1.0 - confidence.lower.get(),
        confidence.method.clone(),
    )
    .map_err(GateFidelityError::from)
}

/// Converts process-fidelity confidence into average-gate-fidelity
/// confidence.
///
/// ```text
/// F_avg = (d F_e + 1) / (d + 1)
/// ```
pub fn transform_process_confidence_to_average(
    confidence: &MetricConfidence,
    dimension: u64,
) -> Result<MetricConfidence, GateFidelityError> {
    validate_dimension(dimension)?;
    validate_confidence(Some(confidence))?;

    let d = dimension as f64;

    let lower =
        (d * confidence.lower.get() + 1.0) /
        (d + 1.0);

    let upper =
        (d * confidence.upper.get() + 1.0) /
        (d + 1.0);

    MetricConfidence::new(
        confidence.level.get(),
        lower,
        upper,
        confidence.method.clone(),
    )
    .map_err(GateFidelityError::from)
}

/// Converts an RB confidence interval for the decay parameter into an
/// average-gate-fidelity confidence interval.
///
/// Since:
///
/// ```text
/// F_avg = 1 - (d - 1)/d * (1 - p)
///        = 1/d + (d - 1)/d * p
/// ```
///
/// the transformation is monotonic increasing.
pub fn rb_confidence_to_average_fidelity(
    confidence: &MetricConfidence,
    dimension: u64,
) -> Result<MetricConfidence, GateFidelityError> {
    validate_rb_dimension(dimension)?;
    validate_confidence(Some(confidence))?;

    let d = dimension as f64;

    let lower =
        (1.0 / d) +
        ((d - 1.0) / d) * confidence.lower.get();

    let upper =
        (1.0 / d) +
        ((d - 1.0) / d) * confidence.upper.get();

    MetricConfidence::new(
        confidence.level.get(),
        lower,
        upper,
        confidence.method.clone(),
    )
    .map_err(GateFidelityError::from)
}

// =============================================================================
// Mathematical helpers
// =============================================================================

/// Converts average gate fidelity into process fidelity.
///
/// ```text
/// F_e = ((d + 1)F_avg - 1) / d
/// ```
fn process_fidelity_from_average(
    average_gate_fidelity: f64,
    dimension: u64,
) -> Result<f64, GateFidelityError> {
    validate_dimension(dimension)?;
    validate_fidelity(average_gate_fidelity)?;

    let d = dimension as f64;

    let value =
        ((d + 1.0) * average_gate_fidelity - 1.0) / d;

    validate_derived_fidelity(
        value,
        "process_fidelity",
    )?;

    Ok(value)
}

/// Propagates a process-fidelity uncertainty through the linear conversion.
///
/// For:
///
/// ```text
/// F_avg = (d F_e + 1) / (d + 1)
/// ```
///
/// the local absolute uncertainty transformation is:
///
/// ```text
/// u(F_avg) = d/(d+1) * u(F_e)
/// ```
fn propagate_process_uncertainty_to_average(
    uncertainty: f64,
    dimension: u64,
) -> f64 {
    let d = dimension as f64;
    (d / (d + 1.0)) * uncertainty
}

/// Calculates process fidelity between two unitary matrices.
///
/// ```text
/// F_e = |Tr(U†V)|² / d²
/// ```
fn unitary_process_fidelity(
    ideal: &ComplexMatrix,
    actual: &ComplexMatrix,
    tolerance: f64,
) -> Result<f64, GateFidelityError> {
    let dimension = ideal.rows();

    if dimension == 0 {
        return Err(GateFidelityError::InvalidDimension {
            dimension,
        });
    }

    let mut trace = Complex64::zero();

    for row in 0..dimension {
        let mut diagonal = Complex64::zero();

        for column in 0..dimension {
            let ideal_value =
                ideal
                    .get(row, column)
                    .ok_or(
                        GateFidelityError::MatrixContainsNonFinite {
                            index: row * dimension + column,
                        },
                    )?;

            let actual_value =
                actual
                    .get(row, column)
                    .ok_or(
                        GateFidelityError::MatrixContainsNonFinite {
                            index: row * dimension + column,
                        },
                    )?;

            diagonal = diagonal
                + ideal_value.conjugate() * actual_value;
        }

        trace = trace + diagonal;
    }

    let trace_squared = trace.norm_squared();

    let d = dimension as f64;

    let denominator = d * d;

    if !denominator.is_finite() || denominator <= 0.0 {
        return Err(
            GateFidelityError::NonFiniteDerivedValue {
                quantity: "unitary_process_fidelity_denominator",
            },
        );
    }

    let value = trace_squared / denominator;

    let value = validate_and_round_fidelity(
        value,
        tolerance,
    )?;

    Ok(value)
}

/// Validates a unitary matrix.
///
/// The condition checked is:
///
/// ```text
/// U†U = I
/// ```
fn validate_unitary_matrix(
    matrix: &ComplexMatrix,
    tolerance: f64,
) -> Result<(), GateFidelityError> {
    validate_tolerance(tolerance)?;

    let rows = matrix.rows();
    let columns = matrix.columns();

    if rows == 0 || columns == 0 {
        return Err(GateFidelityError::InvalidDimension {
            dimension: rows.max(columns),
        });
    }

    if rows != columns {
        return Err(GateFidelityError::MatrixNotSquare {
            rows,
            columns,
        });
    }

    validate_dimension(rows)?;

    for (index, value) in matrix.data().iter().enumerate() {
        if !value.is_finite() {
            return Err(
                GateFidelityError::MatrixContainsNonFinite {
                    index,
                },
            );
        }
    }

    let dagger = matrix.dagger();

    let product =
        dagger
            .multiply(matrix)
            .map_err(|error| {
                GateFidelityError::MatrixNotUnitary {
                    maximum_deviation: f64::INFINITY,
                    tolerance: match error {
                        _ => tolerance,
                    },
                }
            })?;

    let identity =
        ComplexMatrix::identity(rows)
            .map_err(|_| {
                GateFidelityError::InvalidDimension {
                    dimension: rows,
                }
            })?;

    let maximum_deviation =
        product
            .max_difference(&identity)
            .map_err(|_| {
                GateFidelityError::MatrixNotUnitary {
                    maximum_deviation: f64::INFINITY,
                    tolerance,
                }
            })?;

    if !maximum_deviation.is_finite() {
        return Err(
            GateFidelityError::NonFiniteValue {
                context: "unitary validation",
            },
        );
    }

    if maximum_deviation > tolerance {
        return Err(
            GateFidelityError::MatrixNotUnitary {
                maximum_deviation,
                tolerance,
            },
        );
    }

    Ok(())
}

// =============================================================================
// Validation helpers
// =============================================================================

/// Validates a fidelity value.
fn validate_fidelity(
    value: f64,
) -> Result<(), GateFidelityError> {
    if !value.is_finite() {
        return Err(GateFidelityError::NonFiniteValue {
            context: "fidelity",
        });
    }

    if value < -RESULT_BOUND_TOLERANCE
        || value > 1.0 + RESULT_BOUND_TOLERANCE
    {
        return Err(GateFidelityError::FidelityOutOfRange {
            value,
        });
    }

    Ok(())
}

/// Validates and removes insignificant floating-point boundary error.
fn validate_and_round_fidelity(
    value: f64,
    tolerance: f64,
) -> Result<f64, GateFidelityError> {
    if !value.is_finite() {
        return Err(
            GateFidelityError::NonFiniteDerivedValue {
                quantity: "fidelity",
            },
        );
    }

    if value < -tolerance
        || value > 1.0 + tolerance
    {
        return Err(GateFidelityError::FidelityOutOfRange {
            value,
        });
    }

    if value < 0.0 {
        return Ok(0.0);
    }

    if value > 1.0 {
        return Ok(1.0);
    }

    Ok(value)
}

/// Validates a positive Hilbert-space dimension.
fn validate_dimension(
    dimension: u64,
) -> Result<(), GateFidelityError> {
    if dimension == 0 {
        return Err(GateFidelityError::InvalidDimension {
            dimension: 0,
        });
    }

    if dimension > MAX_EXACT_F64_INTEGER as u64 {
        return Err(
            GateFidelityError::DimensionNotRepresentable {
                dimension: dimension as usize,
            },
        );
    }

    Ok(())
}

/// Validates a matrix dimension.
fn validate_dimension_usize(
    dimension: usize,
) -> Result<(), GateFidelityError> {
    if dimension == 0 {
        return Err(GateFidelityError::InvalidDimension {
            dimension,
        });
    }

    if dimension > MAX_EXACT_F64_INTEGER {
        return Err(
            GateFidelityError::DimensionNotRepresentable {
                dimension,
            },
        );
    }

    Ok(())
}

/// Validates a numerical tolerance.
fn validate_tolerance(
    tolerance: f64,
) -> Result<(), GateFidelityError> {
    if !tolerance.is_finite()
        || tolerance < 0.0
    {
        return Err(
            GateFidelityError::InvalidTolerance {
                value: tolerance,
            },
        );
    }

    Ok(())
}

/// Validates uncertainty.
fn validate_uncertainty(
    uncertainty: Option<f64>,
) -> Result<(), GateFidelityError> {
    if let Some(value) = uncertainty {
        if !value.is_finite() || value < 0.0 {
            return Err(
                GateFidelityError::InvalidUncertainty {
                    value,
                },
            );
        }
    }

    Ok(())
}

/// Validates an optional confidence interval.
fn validate_confidence(
    confidence: Option<&MetricConfidence>,
) -> Result<(), GateFidelityError> {
    let Some(confidence) = confidence else {
        return Ok(());
    };

    let level = confidence.level.get();
    let lower = confidence.lower.get();
    let upper = confidence.upper.get();

    if !level.is_finite()
        || !(0.0 < level && level < 1.0)
    {
        return Err(
            GateFidelityError::InvalidConfidenceLevel {
                level,
            },
        );
    }

    if !lower.is_finite()
        || !upper.is_finite()
        || lower < 0.0
        || upper > 1.0
        || lower > upper
    {
        return Err(
            GateFidelityError::InvalidConfidenceInterval {
                lower,
                upper,
            },
        );
    }

    if !confidence.contains(
        lower.max(0.0).min(1.0),
    ) {
        return Err(
            GateFidelityError::InvalidConfidenceInterval {
                lower,
                upper,
            },
        );
    }

    Ok(())
}

/// Validates derived fidelity.
fn validate_derived_fidelity(
    value: f64,
    quantity: &'static str,
) -> Result<(), GateFidelityError> {
    if !value.is_finite() {
        return Err(
            GateFidelityError::NonFiniteDerivedValue {
                quantity,
            },
        );
    }

    if value < -RESULT_BOUND_TOLERANCE
        || value > 1.0 + RESULT_BOUND_TOLERANCE
    {
        return Err(GateFidelityError::FidelityOutOfRange {
            value,
        });
    }

    Ok(())
}

/// Validates RB decay.
fn validate_rb_decay(
    value: f64,
) -> Result<(), GateFidelityError> {
    if !value.is_finite()
        || value < 0.0
        || value > 1.0
    {
        return Err(
            GateFidelityError::InvalidDecayParameter {
                value,
            },
        );
    }

    Ok(())
}

/// Validates an RB Hilbert-space dimension.
fn validate_rb_dimension(
    dimension: u64,
) -> Result<(), GateFidelityError> {
    if dimension < 2 {
        return Err(
            GateFidelityError::InvalidRbDimension {
                dimension,
            },
        );
    }

    validate_dimension(dimension)
}

#[derive(Debug, Clone, Copy)]
enum CountKind {
    Observation,
    Circuit,
    Shot,
}

fn validate_optional_count(
    value: Option<u64>,
    kind: CountKind,
) -> Result<(), GateFidelityError> {
    let Some(value) = value else {
        return Ok(());
    };

    if value != 0 {
        return Ok(());
    }

    match kind {
        CountKind::Observation => {
            Err(GateFidelityError::ZeroObservationCount)
        }

        CountKind::Circuit => {
            Err(GateFidelityError::ZeroCircuitCount)
        }

        CountKind::Shot => {
            Err(GateFidelityError::ZeroShotCount)
        }
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn identity_matrix(
        dimension: usize,
    ) -> ComplexMatrix {
        ComplexMatrix::identity(dimension)
            .expect("test identity matrix must be constructible")
    }

    fn x_gate() -> ComplexMatrix {
        ComplexMatrix::new(
            2,
            2,
            vec![
                Complex64::new(0.0, 0.0),
                Complex64::new(1.0, 0.0),
                Complex64::new(1.0, 0.0),
                Complex64::new(0.0, 0.0),
            ],
        )
        .expect("test X gate must be constructible")
    }

    fn z_gate() -> ComplexMatrix {
        ComplexMatrix::new(
            2,
            2,
            vec![
                Complex64::new(1.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(-1.0, 0.0),
            ],
        )
        .expect("test Z gate must be constructible")
    }

    #[test]
    fn identical_unitaries_have_perfect_fidelity() {
        let identity = identity_matrix(2);

        let result =
            from_unitary_matrices(
                &identity,
                &identity,
            )
            .expect("identity must have perfect fidelity");

        assert!(
            (result.average_gate_fidelity_value() - 1.0)
                .abs()
                < 1.0e-12
        );

        assert!(
            result.gate_infidelity_value()
                .abs()
                < 1.0e-12
        );

        assert!(
            (result.process_fidelity_value() - 1.0)
                .abs()
                < 1.0e-12
        );
    }

    #[test]
    fn_orthogonal_single_qubit_unitaries_have_zero_process_fidelity() {
        let identity = identity_matrix(2);
        let x = x_gate();

        let result =
            from_unitary_matrices(
                &identity,
                &x,
            )
            .expect("X must be unitary");

        assert!(
            result.process_fidelity_value()
                .abs()
                < 1.0e-12
        );

        assert!(
            (result.average_gate_fidelity_value()
                - 1.0 / 3.0)
                .abs()
                < 1.0e-12
        );
    }

    #[test]
    fn_global_phase_does_not_change_unitary_fidelity() {
        let identity = identity_matrix(2);

        let phase_identity =
            ComplexMatrix::new(
                2,
                2,
                vec![
                    Complex64::new(0.0, 1.0),
                    Complex64::zero(),
                    Complex64::zero(),
                    Complex64::new(0.0, 1.0),
                ],
            )
            .expect("phase identity must be constructible");

        let result =
            from_unitary_matrices(
                &identity,
                &phase_identity,
            )
            .expect("global phase matrix must be unitary");

        assert!(
            (result.average_gate_fidelity_value()
                - 1.0)
                .abs()
                < 1.0e-12
        );
    }

    #[test]
    fn process_to_average_conversion_is_correct() {
        let result =
            from_process_fidelity(
                1.0,
                2,
            )
            .expect("perfect process fidelity is valid");

        assert!(
            (result.average_gate_fidelity_value()
                - 1.0)
                .abs()
                < 1.0e-12
        );
    }

    #[test]
    fn average_to_process_conversion_is_correct() {
        let result =
            from_average_gate_fidelity(
                1.0,
                2,
            )
            .expect("perfect average fidelity is valid");

        assert!(
            (result.process_fidelity_value()
                - 1.0)
                .abs()
                < 1.0e-12
        );
    }

    #[test]
    fn rb_perfect_decay_has_perfect_fidelity() {
        let result =
            from_rb_decay(
                1.0,
                2,
            )
            .expect("perfect RB decay is valid");

        assert!(
            (result.average_gate_fidelity_value()
                - 1.0)
                .abs()
                < 1.0e-12
        );
    }

    #[test]
    fn rb_single_qubit_decay_matches_standard_formula() {
        let decay = 0.99;

        let result =
            from_rb_decay(
                decay,
                2,
            )
            .expect("valid RB decay must succeed");

        let expected =
            0.5 + 0.5 * decay;

        assert!(
            (result.average_gate_fidelity_value()
                - expected)
                .abs()
                < 1.0e-12
        );
    }

    #[test]
    fn invalid_fidelity_is_rejected() {
        assert!(
            from_average_gate_fidelity(
                1.1,
                2,
            )
            .is_err()
        );

        assert!(
            from_average_gate_fidelity(
                -0.1,
                2,
            )
            .is_err()
        );

        assert!(
            from_average_gate_fidelity(
                f64::NAN,
                2,
            )
            .is_err()
        );
    }

    #[test]
    fn invalid_rb_decay_is_rejected() {
        assert!(
            from_rb_decay(
                -0.01,
                2,
            )
            .is_err()
        );

        assert!(
            from_rb_decay(
                1.01,
                2,
            )
            .is_err()
        );

        assert!(
            from_rb_decay(
                0.9,
                1,
            )
            .is_err()
        );
    }

    #[test]
    fn non_unitary_matrix_is_rejected() {
        let invalid =
            ComplexMatrix::new(
                2,
                2,
                vec![
                    Complex64::new(2.0, 0.0),
                    Complex64::zero(),
                    Complex64::zero(),
                    Complex64::new(1.0, 0.0),
                ],
            )
            .expect("matrix itself is structurally valid");

        let identity = identity_matrix(2);

        assert!(
            from_unitary_matrices(
                &identity,
                &invalid,
            )
            .is_err()
        );
    }

    #[test]
    fn mismatched_dimensions_are_rejected() {
        let one =
            identity_matrix(2);

        let two =
            identity_matrix(4);

        assert!(
            from_unitary_matrices(
                &one,
                &two,
            )
            .is_err()
        );
    }

    #[test]
    fn confidence_interval_transformation_is_monotonic() {
        let confidence =
            MetricConfidence::new(
                0.95,
                0.90,
                0.99,
                ConfidenceMethod::Wilson,
            )
            .expect("test confidence interval must be valid");

        let transformed =
            transform_fidelity_confidence_to_infidelity(
                &confidence,
            )
            .expect("transformation must succeed");

        assert!(
            (transformed.lower.get() - 0.01)
                .abs()
                < 1.0e-12
        );

        assert!(
            (transformed.upper.get() - 0.10)
                .abs()
                < 1.0e-12
        );
    }

    #[test]
    fn process_confidence_transformation_is_correct() {
        let confidence =
            MetricConfidence::new(
                0.95,
                0.90,
                1.0,
                ConfidenceMethod::Wilson,
            )
            .expect("test confidence interval must be valid");

        let transformed =
            transform_process_confidence_to_average(
                &confidence,
                2,
            )
            .expect("process confidence conversion must succeed");

        let expected_lower =
            (2.0 * 0.90 + 1.0) / 3.0;

        assert!(
            (transformed.lower.get()
                - expected_lower)
                .abs()
                < 1.0e-12
        );

        assert!(
            (transformed.upper.get() - 1.0)
                .abs()
                < 1.0e-12
        );
    }

    #[test]
    fn result_preserves_source_identity() {
        let result =
            from_rb_decay(
                0.99,
                2,
            )
            .expect("valid RB result");

        assert_eq!(
            result.source,
            GateFidelitySource::RandomizedBenchmarking
        );

        assert!(
            result
                .assumptions
                .depolarizing_rb_model
        );
    }

    #[test]
    fn result_metrics_are_canonical_metrics() {
        let result =
            from_average_gate_fidelity(
                0.99,
                2,
            )
            .expect("valid average fidelity");

        assert_eq!(
            result.average_gate_fidelity.kind,
            MetricKind::AverageGateFidelity
        );

        assert_eq!(
            result.process_fidelity.kind,
            MetricKind::ProcessFidelity
        );

        assert_eq!(
            result.gate_infidelity.kind,
            MetricKind::GateInfidelity
        );

        assert_eq!(
            result.average_gate_fidelity.unit,
            MetricUnit::Probability
        );
    }
}