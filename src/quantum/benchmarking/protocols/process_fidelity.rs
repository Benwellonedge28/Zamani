//! Zamani Quantum Benchmarking — Process Fidelity Protocol
//!
//! Production-grade process-fidelity protocol boundary.
//!
//! # Responsibility
//!
//! This module converts already-available process characterization data into
//! canonical Zamani benchmarking metrics.
//!
//! It deliberately does NOT:
//!
//! - generate tomography circuits;
//! - execute circuits;
//! - communicate with quantum hardware;
//! - select a backend;
//! - perform transpilation;
//! - perform routing;
//! - perform scheduling;
//! - reconstruct a process from raw measurements;
//! - perform state tomography;
//! - perform process tomography;
//! - fit randomized-benchmarking curves;
//! - own Quantum IR;
//! - negotiate backend capabilities;
//! - mutate global state;
//! - print diagnostics;
//! - silently clamp invalid scientific input.
//!
//! Those responsibilities belong to the surrounding benchmarking system.
//!
//! # Scientific convention
//!
//! For normalized Choi/process density matrices, Zamani uses the squared
//! Uhlmann-Jozsa fidelity:
//!
//! ```text
//! F_process(rho, sigma)
//!     = [Tr sqrt(sqrt(rho) sigma sqrt(rho))]^2
//! ```
//!
//! For a target unitary channel U and an implemented unitary channel V,
//! the process fidelity reduces to:
//!
//! ```text
//! F_process(U, V)
//!     = |Tr(U† V)|² / d²
//! ```
//!
//! where `d` is the Hilbert-space dimension.
//!
//! For a d-dimensional channel, when the process fidelity is the normalized
//! entanglement/process fidelity associated with the channel comparison:
//!
//! ```text
//! F_avg = (d * F_process + 1) / (d + 1)
//! ```
//!
//! This conversion is only valid when the supplied process fidelity has the
//! normalization and channel interpretation required by that relationship.
//!
//! # Important distinction
//!
//! This module distinguishes three situations:
//!
//! 1. Exact unitary comparison.
//!
//!    The caller supplies two validated unitary matrices. The result is an
//!    exact mathematical channel comparison. It is not a hardware
//!    measurement.
//!
//! 2. Reconstructed process comparison.
//!
//!    The caller supplies normalized Choi/process density matrices produced by
//!    process tomography or another validated characterization procedure.
//!    The fidelity is calculated by the common fidelity metric layer.
//!
//! 3. Derived average gate fidelity.
//!
//!    The caller supplies a process fidelity and Hilbert-space dimension.
//!    Average gate fidelity is derived using the standard channel relation.
//!
//! The source is recorded in metric metadata so downstream reporting cannot
//! accidentally present an exact mathematical comparison as an experimental
//! measurement.
//!
//! # Architecture
//!
//! ```text
//! execution / tomography / simulator / backend
//!                  │
//!                  ▼
//!          process characterization
//!                  │
//!        ┌─────────┴──────────┐
//!        ▼                    ▼
//! normalized Choi        exact unitaries
//! process matrices       U_target / U_actual
//!        │                    │
//!        └─────────┬──────────┘
//!                  ▼
//!       protocols::process_fidelity
//!                  │
//!        ┌─────────┴──────────┐
//!        ▼                    ▼
//! process fidelity      average gate fidelity
//! process infidelity    average gate infidelity
//!        │                    │
//!        └─────────┬──────────┘
//!                  ▼
//!             core::Metric
//!                  │
//!                  ▼
//!          BenchmarkResult
//! ```
//!
//! # Integration contract
//!
//! Direct dependencies are intentionally limited to:
//!
//! ```text
//! core::metric
//! metrics::fidelity
//! metrics::gate_error
//! serde
//! std
//! ```
//!
//! The intended future integration is:
//!
//! ```text
//! protocols::tomography
//!        │
//!        └──> normalized Choi/process matrix
//!                    │
//!                    ▼
//!             process_fidelity.rs
//!
//! protocols::gate_fidelity
//!        │
//!        └──> process-fidelity result
//!
//! protocols::cycle_benchmarking
//!        │
//!        └──> cycle/process fidelity
//!
//! reporting::*
//!        │
//!        └──> Metric
//!
//! analysis::*
//!        │
//!        └──> Metric
//! ```
//!
//! No modification to `metrics::fidelity.rs` or `core::metric.rs` is required
//! for the API in this file.
//!
//! # Rust compatibility
//!
//! - Rust 1.97
//! - Rust 1.97.1
//! - Rust 2021
//!
//! No nightly features are required.
//!
//! # References
//!
//! Process tomography and randomized benchmarking measure related but
//! distinct notions of gate/process quality. Process tomography provides a
//! reconstructed process representation but is sensitive to state-preparation
//! and measurement errors, while randomized benchmarking estimates
//! computationally relevant average errors under model assumptions.
//!
//! The canonical numerical fidelity implementation lives in
//! `benchmarking::metrics::fidelity` and must remain the single owner of the
//! underlying Uhlmann/process-fidelity mathematics.

use serde::{Deserialize, Serialize};

use crate::quantum::benchmarking::core::metric::{
    ConfidenceMethod,
    FiniteF64,
    Metric,
    MetricConfidence,
    MetricDirection,
    MetricError,
    MetricKind,
    MetricMetadata,
    MetricQuality,
    MetricResult,
    MetricUnit,
    ProvenanceRef,
};

use crate::quantum::benchmarking::metrics::fidelity::{
    process_fidelity as calculate_process_fidelity,
    Complex64,
    ComplexMatrix,
    FidelityError,
    FidelityTolerance,
};

use crate::quantum::benchmarking::metrics::gate_error::{
    average_gate_fidelity_from_entanglement_fidelity,
    process_infidelity_from_process_fidelity,
};

// =============================================================================
// Public protocol identity
// =============================================================================

/// Stable benchmark identifier.
pub const PROCESS_FIDELITY_BENCHMARK_ID: &str = "process_fidelity";

/// Stable protocol version.
///
/// Increment this when the semantic interpretation of the protocol result
/// changes.
pub const PROCESS_FIDELITY_PROTOCOL_VERSION: &str = "1";

/// Default numerical tolerance.
pub const DEFAULT_PROCESS_FIDELITY_TOLERANCE: f64 = 1.0e-10;

/// Maximum integer exactly representable by IEEE-754 f64.
///
/// This protects dimension conversions from silently losing integer
/// information.
const MAX_EXACT_F64_INTEGER: usize = 9_007_199_254_740_992;

/// Tolerance used when validating exact unitary comparison results.
const DEFAULT_UNITARY_TOLERANCE: f64 = 1.0e-10;

// =============================================================================
// Error
// =============================================================================

/// Errors produced by the process-fidelity protocol.
#[derive(Debug, Clone, PartialEq)]
pub enum ProcessFidelityError {
    /// A numerical input was NaN or infinite.
    NonFiniteValue {
        /// Context identifying the invalid value.
        context: &'static str,
    },

    /// A supplied dimension is zero.
    InvalidDimension {
        /// Invalid dimension.
        dimension: usize,
    },

    /// A dimension cannot safely participate in floating-point formulas.
    DimensionNotRepresentable {
        /// Invalid/too-large dimension.
        dimension: usize,
    },

    /// Two process representations have different dimensions.
    DimensionMismatch {
        /// Left dimension.
        left: usize,

        /// Right dimension.
        right: usize,
    },

    /// A unitary matrix is not square.
    MatrixNotSquare {
        /// Matrix row count.
        rows: usize,

        /// Matrix column count.
        columns: usize,
    },

    /// A unitary matrix contains invalid data.
    MatrixContainsNonFinite {
        /// Linear element index.
        index: usize,
    },

    /// A supplied matrix is not unitary within tolerance.
    MatrixNotUnitary {
        /// Maximum deviation from identity.
        maximum_deviation: f64,

        /// Allowed tolerance.
        tolerance: f64,
    },

    /// Invalid numerical tolerance.
    InvalidTolerance {
        /// Supplied tolerance.
        value: f64,
    },

    /// Invalid process fidelity.
    FidelityOutOfRange {
        /// Supplied fidelity.
        value: f64,
    },

    /// Invalid uncertainty.
    InvalidUncertainty {
        /// Supplied uncertainty.
        value: f64,
    },

    /// Invalid confidence level.
    InvalidConfidenceLevel {
        /// Supplied confidence level.
        level: f64,
    },

    /// Invalid confidence interval.
    InvalidConfidenceInterval {
        /// Lower bound.
        lower: f64,

        /// Upper bound.
        upper: f64,
    },

    /// Zero observations are not meaningful for an experimental result.
    ZeroObservationCount,

    /// Zero circuits are not meaningful for an experimental result.
    ZeroCircuitCount,

    /// Zero shots are not meaningful for an experimental result.
    ZeroShotCount,

    /// The underlying fidelity mathematics rejected the input.
    Fidelity(FidelityError),

    /// The canonical metric layer rejected the constructed metric.
    Metric(MetricError),
}

impl From<FidelityError> for ProcessFidelityError {
    fn from(error: FidelityError) -> Self {
        Self::Fidelity(error)
    }
}

impl From<MetricError> for ProcessFidelityError {
    fn from(error: MetricError) -> Self {
        Self::Metric(error)
    }
}

impl std::fmt::Display for ProcessFidelityError {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::NonFiniteValue { context } => {
                write!(
                    formatter,
                    "process-fidelity input contains a non-finite value: {context}"
                )
            }

            Self::InvalidDimension { dimension } => {
                write!(
                    formatter,
                    "process-fidelity dimension must be greater than zero: {dimension}"
                )
            }

            Self::DimensionNotRepresentable { dimension } => {
                write!(
                    formatter,
                    "process-fidelity dimension {dimension} is too large \
                     for exact floating-point representation"
                )
            }

            Self::DimensionMismatch { left, right } => {
                write!(
                    formatter,
                    "process-fidelity dimensions do not match: \
                     left={left}, right={right}"
                )
            }

            Self::MatrixNotSquare { rows, columns } => {
                write!(
                    formatter,
                    "process-fidelity matrix must be square: {rows}x{columns}"
                )
            }

            Self::MatrixContainsNonFinite { index } => {
                write!(
                    formatter,
                    "process-fidelity matrix contains a non-finite \
                     value at index {index}"
                )
            }

            Self::MatrixNotUnitary {
                maximum_deviation,
                tolerance,
            } => {
                write!(
                    formatter,
                    "process-fidelity matrix is not unitary: \
                     maximum deviation={maximum_deviation}, tolerance={tolerance}"
                )
            }

            Self::InvalidTolerance { value } => {
                write!(
                    formatter,
                    "process-fidelity tolerance must be finite and >= 0: {value}"
                )
            }

            Self::FidelityOutOfRange { value } => {
                write!(
                    formatter,
                    "process fidelity must be in [0, 1], got {value}"
                )
            }

            Self::InvalidUncertainty { value } => {
                write!(
                    formatter,
                    "process-fidelity uncertainty must be finite and >= 0: {value}"
                )
            }

            Self::InvalidConfidenceLevel { level } => {
                write!(
                    formatter,
                    "process-fidelity confidence level must be in (0, 1): {level}"
                )
            }

            Self::InvalidConfidenceInterval { lower, upper } => {
                write!(
                    formatter,
                    "invalid process-fidelity confidence interval: \
                     lower={lower}, upper={upper}"
                )
            }

            Self::ZeroObservationCount => {
                write!(
                    formatter,
                    "process-fidelity observation count must be greater than zero"
                )
            }

            Self::ZeroCircuitCount => {
                write!(
                    formatter,
                    "process-fidelity circuit count must be greater than zero"
                )
            }

            Self::ZeroShotCount => {
                write!(
                    formatter,
                    "process-fidelity shot count must be greater than zero"
                )
            }

            Self::Fidelity(error) => error.fmt(formatter),

            Self::Metric(error) => error.fmt(formatter),
        }
    }
}

impl std::error::Error for ProcessFidelityError {}

// =============================================================================
// Source/method identity
// =============================================================================

/// Identifies the source of a process-fidelity result.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProcessFidelitySource {
    /// Exact comparison of two supplied unitary channels.
    ExactUnitaryComparison,

    /// Comparison of normalized Choi/process density matrices.
    NormalizedChoiComparison,

    /// Result produced by process tomography.
    ProcessTomography,

    /// Result produced by gate-set/process characterization.
    GateSetTomography,

    /// Result produced by another benchmark protocol.
    CycleBenchmarking,

    /// Result supplied by an external validated characterization system.
    ExternalCharacterization,

    /// User-defined source.
    Custom(String),
}

impl ProcessFidelitySource {
    /// Returns the stable identifier.
    pub fn id(&self) -> String {
        match self {
            Self::ExactUnitaryComparison => {
                "exact_unitary_comparison".to_string()
            }

            Self::NormalizedChoiComparison => {
                "normalized_choi_comparison".to_string()
            }

            Self::ProcessTomography => {
                "process_tomography".to_string()
            }

            Self::GateSetTomography => {
                "gate_set_tomography".to_string()
            }

            Self::CycleBenchmarking => {
                "cycle_benchmarking".to_string()
            }

            Self::ExternalCharacterization => {
                "external_characterization".to_string()
            }

            Self::Custom(value) => value.clone(),
        }
    }
}

// =============================================================================
// Options
// =============================================================================

/// Optional experimental metadata accepted by process-fidelity analysis.
///
/// This structure deliberately contains no backend or execution object.
/// Execution belongs to `benchmarking::execution`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProcessFidelityOptions {
    /// Numerical validation tolerances.
    pub tolerances: FidelityTolerance,

    /// Optional one-standard-deviation uncertainty.
    pub uncertainty: Option<f64>,

    /// Optional confidence interval.
    pub confidence: Option<MetricConfidence>,

    /// Number of observations contributing to the result.
    pub observation_count: Option<u64>,

    /// Number of shots contributing to the result.
    pub shot_count: Option<u64>,

    /// Number of circuits contributing to the result.
    pub circuit_count: Option<u64>,

    /// Optional provenance reference.
    pub provenance: Option<ProvenanceRef>,
}

impl Default for ProcessFidelityOptions {
    fn default() -> Self {
        Self {
            tolerances: FidelityTolerance::default(),
            uncertainty: None,
            confidence: None,
            observation_count: None,
            shot_count: None,
            circuit_count: None,
            provenance: None,
        }
    }
}

impl ProcessFidelityOptions {
    /// Validates all supplied options.
    pub fn validate(&self) -> Result<(), ProcessFidelityError> {
        self.tolerances.validate()?;

        if let Some(value) = self.uncertainty {
            validate_uncertainty(value)?;
        }

        if let Some(confidence) = &self.confidence {
            validate_confidence(confidence)?;
        }

        if let Some(count) = self.observation_count {
            if count == 0 {
                return Err(ProcessFidelityError::ZeroObservationCount);
            }
        }

        if let Some(count) = self.shot_count {
            if count == 0 {
                return Err(ProcessFidelityError::ZeroShotCount);
            }
        }

        if let Some(count) = self.circuit_count {
            if count == 0 {
                return Err(ProcessFidelityError::ZeroCircuitCount);
            }
        }

        Ok(())
    }
}

// =============================================================================
// Result bundle
// =============================================================================

/// Canonical process-fidelity result bundle.
///
/// `process_fidelity` is always present.
///
/// `process_infidelity` is always derived from it.
///
/// `average_gate_fidelity` is present only when the caller supplies a valid
/// Hilbert-space dimension.
///
/// `average_gate_infidelity` is derived from the average gate fidelity.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProcessFidelityResult {
    /// Process/Choi fidelity.
    pub process_fidelity: Metric,

    /// Process infidelity.
    pub process_infidelity: Metric,

    /// Average gate fidelity, if a Hilbert-space dimension was supplied.
    pub average_gate_fidelity: Option<Metric>,

    /// Average gate infidelity, if average gate fidelity is available.
    pub average_gate_infidelity: Option<Metric>,
}

impl ProcessFidelityResult {
    /// Returns the process fidelity as a finite floating-point value.
    #[inline]
    pub fn value(&self) -> f64 {
        self.process_fidelity.value.get()
    }

    /// Returns the process infidelity as a finite floating-point value.
    #[inline]
    pub fn infidelity(&self) -> f64 {
        self.process_infidelity.value.get()
    }

    /// Returns the average gate fidelity if available.
    #[inline]
    pub fn average_gate_fidelity(&self) -> Option<f64> {
        self.average_gate_fidelity
            .as_ref()
            .map(|metric| metric.value.get())
    }
}

// =============================================================================
// Public API — normalized Choi/process matrices
// =============================================================================

/// Calculates process fidelity from normalized Choi/process density matrices.
///
/// Both inputs must:
///
/// - be square;
/// - contain finite values;
/// - be Hermitian;
/// - have unit trace;
/// - be positive semidefinite;
/// - have identical dimensions.
///
/// The underlying Uhlmann fidelity implementation is delegated to
/// `metrics::fidelity`, keeping all density-matrix mathematics centralized.
pub fn analyze_normalized_choi(
    ideal_choi: &ComplexMatrix,
    actual_choi: &ComplexMatrix,
) -> Result<ProcessFidelityResult, ProcessFidelityError> {
    analyze_normalized_choi_with_options(
        ideal_choi,
        actual_choi,
        ProcessFidelityOptions::default(),
        None,
    )
}

/// Calculates normalized Choi/process fidelity with experimental metadata.
pub fn analyze_normalized_choi_with_options(
    ideal_choi: &ComplexMatrix,
    actual_choi: &ComplexMatrix,
    options: ProcessFidelityOptions,
    hilbert_dimension: Option<usize>,
) -> Result<ProcessFidelityResult, ProcessFidelityError> {
    options.validate()?;

    let fidelity = calculate_process_fidelity(
        ideal_choi,
        actual_choi,
    )?;

    let process_metric = build_process_metric(
        fidelity.value,
        ProcessFidelitySource::NormalizedChoiComparison,
        &options,
    )?;

    build_result(
        process_metric,
        hilbert_dimension,
        &options,
    )
}

/// Calculates process fidelity specifically for a process-tomography result.
///
/// The input matrices must already be reconstructed and normalized by the
/// tomography subsystem. This function does not perform tomography.
pub fn analyze_process_tomography(
    ideal_process: &ComplexMatrix,
    reconstructed_process: &ComplexMatrix,
    options: ProcessFidelityOptions,
    hilbert_dimension: usize,
) -> Result<ProcessFidelityResult, ProcessFidelityError> {
    options.validate()?;
    validate_hilbert_dimension(hilbert_dimension)?;

    let fidelity = calculate_process_fidelity(
        ideal_process,
        reconstructed_process,
    )?;

    let process_metric = build_process_metric(
        fidelity.value,
        ProcessFidelitySource::ProcessTomography,
        &options,
    )?;

    build_result(
        process_metric,
        Some(hilbert_dimension),
        &options,
    )
}

// =============================================================================
// Public API — exact unitary channels
// =============================================================================

/// Calculates exact process fidelity between two unitary channels.
///
/// For d-dimensional unitary matrices U and V:
///
/// ```text
/// F_process = |Tr(U†V)|² / d²
/// ```
///
/// This is an exact mathematical channel comparison. It must not be reported
/// as an experimental hardware measurement.
pub fn analyze_unitary_channels(
    ideal: &ComplexMatrix,
    actual: &ComplexMatrix,
) -> Result<ProcessFidelityResult, ProcessFidelityError> {
    analyze_unitary_channels_with_options(
        ideal,
        actual,
        ProcessFidelityOptions::default(),
    )
}

/// Calculates exact unitary-channel process fidelity with metadata.
pub fn analyze_unitary_channels_with_options(
    ideal: &ComplexMatrix,
    actual: &ComplexMatrix,
    options: ProcessFidelityOptions,
) -> Result<ProcessFidelityResult, ProcessFidelityError> {
    options.validate()?;

    let dimension = validate_matching_unitaries(
        ideal,
        actual,
        options.tolerances.result.max(DEFAULT_UNITARY_TOLERANCE),
    )?;

    let dagger = ideal.dagger();

    let relative = dagger.multiply(actual)?;

    let trace = relative.trace()?;

    if !trace.is_finite() {
        return Err(ProcessFidelityError::NonFiniteValue {
            context: "unitary process trace",
        });
    }

    let d = checked_dimension_as_f64(dimension)?;

    let fidelity = trace.norm_squared() / (d * d);

    validate_fidelity(fidelity)?;

    let process_metric = build_process_metric(
        fidelity,
        ProcessFidelitySource::ExactUnitaryComparison,
        &options,
    )?;

    build_result(
        process_metric,
        Some(dimension),
        &options,
    )
}

// =============================================================================
// Public API — scalar conversions
// =============================================================================

/// Builds a canonical process-fidelity result from an already-established
/// process fidelity.
///
/// This is useful for RB, IRB, cycle benchmarking, or external
/// characterization modules that have already performed their own statistical
/// analysis.
pub fn analyze_scalar_process_fidelity(
    process_fidelity: f64,
    options: ProcessFidelityOptions,
    hilbert_dimension: Option<usize>,
    source: ProcessFidelitySource,
) -> Result<ProcessFidelityResult, ProcessFidelityError> {
    options.validate()?;

    validate_fidelity(process_fidelity)?;

    if let Some(dimension) = hilbert_dimension {
        validate_hilbert_dimension(dimension)?;
    }

    let process_metric = build_process_metric(
        process_fidelity,
        source,
        &options,
    )?;

    build_result(
        process_metric,
        hilbert_dimension,
        &options,
    )
}

/// Converts process fidelity to average gate fidelity.
///
/// ```text
/// F_avg = (d F_process + 1) / (d + 1)
/// ```
pub fn average_gate_fidelity_from_process(
    process_fidelity: f64,
    hilbert_dimension: usize,
) -> Result<Metric, ProcessFidelityError> {
    validate_fidelity(process_fidelity)?;
    validate_hilbert_dimension(hilbert_dimension)?;

    Ok(
        average_gate_fidelity_from_entanglement_fidelity(
            process_fidelity,
            checked_dimension_as_u64(hilbert_dimension)?,
        )?
        .with_metadata(MetricMetadata::new(
            "source_protocol",
            PROCESS_FIDELITY_BENCHMARK_ID,
        )?),
    )
}

/// Converts process fidelity to process infidelity.
///
/// ```text
/// r_process = 1 - F_process
/// ```
pub fn process_infidelity(
    process_fidelity: f64,
) -> Result<Metric, ProcessFidelityError> {
    validate_fidelity(process_fidelity)?;

    Ok(
        process_infidelity_from_process_fidelity(
            process_fidelity,
        )?
        .with_metadata(MetricMetadata::new(
            "source_protocol",
            PROCESS_FIDELITY_BENCHMARK_ID,
        )?),
    )
}

/// Converts average gate fidelity to process fidelity.
///
/// ```text
/// F_process = ((d + 1) F_avg - 1) / d
/// ```
///
/// The conversion is valid only for a d-dimensional channel where the
/// supplied average gate fidelity has the channel interpretation required by
/// the standard relation.
pub fn process_from_average_gate_fidelity(
    average_gate_fidelity: f64,
    hilbert_dimension: usize,
) -> Result<Metric, ProcessFidelityError> {
    validate_fidelity(average_gate_fidelity)?;
    validate_hilbert_dimension(hilbert_dimension)?;

    let d = checked_dimension_as_f64(hilbert_dimension)?;

    let value =
        ((d + 1.0) * average_gate_fidelity - 1.0) / d;

    validate_fidelity(value)?;

    let metric = Metric::new(
        MetricKind::ProcessFidelity,
        MetricUnit::Probability,
        value,
    )?
    .with_quality(MetricQuality::Derived)
    .with_direction(MetricDirection::HigherIsBetter)
    .with_description(
        "Process fidelity derived from average gate fidelity \
         using the standard d-dimensional channel relation.",
    )?
    .with_metadata(MetricMetadata::new(
        "source_protocol",
        PROCESS_FIDELITY_BENCHMARK_ID,
    )?)
    .with_metadata(MetricMetadata::new(
        "derived_from",
        "average_gate_fidelity",
    )?)
    .with_metadata(MetricMetadata::new(
        "hilbert_dimension",
        hilbert_dimension.to_string(),
    )?)
    .with_metadata(MetricMetadata::new(
        "conversion_model",
        "F_process=((d+1)*F_avg-1)/d",
    )?)
    .with_metadata(MetricMetadata::new(
        "protocol_version",
        PROCESS_FIDELITY_PROTOCOL_VERSION,
    )?) ;

    Ok(metric)
}

/// Converts a qubit count into Hilbert-space dimension.
///
/// ```text
/// d = 2^n
/// ```
///
/// The calculation is checked for integer overflow.
pub fn qubit_count_to_hilbert_dimension(
    qubits: usize,
) -> Result<usize, ProcessFidelityError> {
    if qubits >= usize::BITS as usize {
        return Err(ProcessFidelityError::DimensionNotRepresentable {
            dimension: qubits,
        });
    }

    let dimension = 1usize
        .checked_shl(qubits as u32)
        .ok_or(
            ProcessFidelityError::DimensionNotRepresentable {
                dimension: qubits,
            },
        )?;

    validate_hilbert_dimension(dimension)?;

    Ok(dimension)
}

// =============================================================================
// Public API — metric construction with metadata
// =============================================================================

/// Builds a process-fidelity metric from an already calculated value.
///
/// This function is useful to sibling protocols such as:
///
/// - randomized benchmarking;
/// - interleaved randomized benchmarking;
/// - cycle benchmarking;
/// - gate-set tomography;
/// - external characterization.
pub fn build_process_metric(
    value: f64,
    source: ProcessFidelitySource,
    options: &ProcessFidelityOptions,
) -> Result<Metric, ProcessFidelityError> {
    options.validate()?;
    validate_fidelity(value)?;

    let mut metric = Metric::new(
        MetricKind::ProcessFidelity,
        MetricUnit::Probability,
        value,
    )?
    .with_quality(match source {
        ProcessFidelitySource::ExactUnitaryComparison => {
            MetricQuality::Derived
        }

        ProcessFidelitySource::NormalizedChoiComparison => {
            MetricQuality::Derived
        }

        ProcessFidelitySource::ProcessTomography => {
            MetricQuality::Observed
        }

        ProcessFidelitySource::GateSetTomography => {
            MetricQuality::Estimated
        }

        ProcessFidelitySource::CycleBenchmarking => {
            MetricQuality::Estimated
        }

        ProcessFidelitySource::ExternalCharacterization => {
            MetricQuality::Estimated
        }

        ProcessFidelitySource::Custom(_) => {
            MetricQuality::Estimated
        }
    })
    .with_direction(MetricDirection::HigherIsBetter)
    .with_description(
        "Normalized quantum-process fidelity. The exact scientific \
         interpretation is recorded in the source and metadata fields.",
    )?
    .with_metadata(MetricMetadata::new(
        "benchmark_id",
        PROCESS_FIDELITY_BENCHMARK_ID,
    )?)
    .with_metadata(MetricMetadata::new(
        "protocol_version",
        PROCESS_FIDELITY_PROTOCOL_VERSION,
    )?)
    .with_metadata(MetricMetadata::new(
        "source",
        source.id(),
    )?)
    .with_metadata(MetricMetadata::new(
        "fidelity_definition",
        "uhlmann_squared_normalized_process",
    )?);

    if let Some(value) = options.uncertainty {
        metric = metric.with_uncertainty(value)?;
    }

    if let Some(confidence) = options.confidence.clone() {
        metric = metric.with_confidence(confidence)?;
    }

    if let Some(count) = options.observation_count {
        metric = metric.with_sample_count(count)?;
    }

    if let Some(count) = options.shot_count {
        metric = metric.with_shot_count(count)?;
    }

    if let Some(count) = options.circuit_count {
        metric = metric.with_circuit_count(count)?;
    }

    if let Some(provenance) = options.provenance.clone() {
        metric = metric.with_provenance(provenance);
    }

    Ok(metric)
}

// =============================================================================
// Internal result construction
// =============================================================================

fn build_result(
    process_metric: Metric,
    hilbert_dimension: Option<usize>,
    options: &ProcessFidelityOptions,
) -> Result<ProcessFidelityResult, ProcessFidelityError> {
    let process_value = process_metric.value.get();

    let mut process_infidelity =
        process_infidelity_from_process_fidelity(
            process_value,
        )?
        .with_metadata(MetricMetadata::new(
            "benchmark_id",
            PROCESS_FIDELITY_BENCHMARK_ID,
        )?)
        .with_metadata(MetricMetadata::new(
            "protocol_version",
            PROCESS_FIDELITY_PROTOCOL_VERSION,
        )?)
        .with_metadata(MetricMetadata::new(
            "derived_from",
            "process_fidelity",
        )?);

    process_infidelity =
        process_infidelity.with_description(
            "Process infidelity derived as 1 - process fidelity.",
        )?;

    if let Some(count) = options.observation_count {
        process_infidelity =
            process_infidelity.with_sample_count(count)?;
    }

    if let Some(count) = options.shot_count {
        process_infidelity =
            process_infidelity.with_shot_count(count)?;
    }

    if let Some(count) = options.circuit_count {
        process_infidelity =
            process_infidelity.with_circuit_count(count)?;
    }

    if let Some(provenance) = options.provenance.clone() {
        process_infidelity =
            process_infidelity.with_provenance(provenance);
    }

    let average_gate_fidelity =
        if let Some(dimension) = hilbert_dimension {
            let mut metric =
                average_gate_fidelity_from_entanglement_fidelity(
                    process_value,
                    checked_dimension_as_u64(dimension)?,
                )?;

            metric = metric
                .with_metadata(MetricMetadata::new(
                    "benchmark_id",
                    PROCESS_FIDELITY_BENCHMARK_ID,
                )?)
                .with_metadata(MetricMetadata::new(
                    "protocol_version",
                    PROCESS_FIDELITY_PROTOCOL_VERSION,
                )?)
                .with_metadata(MetricMetadata::new(
                    "derived_from",
                    "process_fidelity",
                )?)
                .with_metadata(MetricMetadata::new(
                    "hilbert_dimension",
                    dimension.to_string(),
                )?)
                .with_description(
                    "Average gate fidelity derived from normalized \
                     process fidelity using the standard d-dimensional \
                     channel relation.",
                )?;

            if let Some(count) = options.observation_count {
                metric = metric.with_sample_count(count)?;
            }

            if let Some(count) = options.shot_count {
                metric = metric.with_shot_count(count)?;
            }

            if let Some(count) = options.circuit_count {
                metric = metric.with_circuit_count(count)?;
            }

            if let Some(provenance) = options.provenance.clone() {
                metric = metric.with_provenance(provenance);
            }

            Some(metric)
        } else {
            None
        };

    let average_gate_infidelity =
        if let Some(ref average_gate_fidelity) =
            average_gate_fidelity
        {
            let value =
                1.0 - average_gate_fidelity.value.get();

            let mut metric = Metric::new(
                MetricKind::GateInfidelity,
                MetricUnit::Probability,
                value,
            )?
            .with_quality(MetricQuality::Derived)
            .with_direction(MetricDirection::LowerIsBetter)
            .with_description(
                "Average gate infidelity derived as \
                 1 - average gate fidelity.",
            )?
            .with_metadata(MetricMetadata::new(
                "benchmark_id",
                PROCESS_FIDELITY_BENCHMARK_ID,
            )?)
            .with_metadata(MetricMetadata::new(
                "protocol_version",
                PROCESS_FIDELITY_PROTOCOL_VERSION,
            )?)
            .with_metadata(MetricMetadata::new(
                "derived_from",
                "average_gate_fidelity",
            )?);

            if let Some(count) = options.observation_count {
                metric = metric.with_sample_count(count)?;
            }

            if let Some(count) = options.shot_count {
                metric = metric.with_shot_count(count)?;
            }

            if let Some(count) = options.circuit_count {
                metric = metric.with_circuit_count(count)?;
            }

            if let Some(provenance) = options.provenance.clone() {
                metric = metric.with_provenance(provenance);
            }

            Some(metric)
        } else {
            None
        };

    Ok(ProcessFidelityResult {
        process_fidelity: process_metric,
        process_infidelity,
        average_gate_fidelity,
        average_gate_infidelity,
    })
}

// =============================================================================
// Validation
// =============================================================================

fn validate_hilbert_dimension(
    dimension: usize,
) -> Result<(), ProcessFidelityError> {
    if dimension == 0 {
        return Err(ProcessFidelityError::InvalidDimension {
            dimension,
        });
    }

    if dimension > MAX_EXACT_F64_INTEGER {
        return Err(
            ProcessFidelityError::DimensionNotRepresentable {
                dimension,
            },
        );
    }

    Ok(())
}

fn checked_dimension_as_f64(
    dimension: usize,
) -> Result<f64, ProcessFidelityError> {
    validate_hilbert_dimension(dimension)?;

    let value = dimension as f64;

    if !value.is_finite() {
        return Err(ProcessFidelityError::NonFiniteValue {
            context: "Hilbert-space dimension",
        });
    }

    Ok(value)
}

fn checked_dimension_as_u64(
    dimension: usize,
) -> Result<u64, ProcessFidelityError> {
    validate_hilbert_dimension(dimension)?;

    u64::try_from(dimension).map_err(
        |_| ProcessFidelityError::DimensionNotRepresentable {
            dimension,
        },
    )
}

fn validate_fidelity(
    value: f64,
) -> Result<(), ProcessFidelityError> {
    if !value.is_finite() {
        return Err(ProcessFidelityError::NonFiniteValue {
            context: "process fidelity",
        });
    }

    if value < 0.0 || value > 1.0 {
        return Err(ProcessFidelityError::FidelityOutOfRange {
            value,
        });
    }

    Ok(())
}

fn validate_uncertainty(
    value: f64,
) -> Result<(), ProcessFidelityError> {
    if !value.is_finite() || value < 0.0 {
        return Err(ProcessFidelityError::InvalidUncertainty {
            value,
        });
    }

    Ok(())
}

fn validate_confidence(
    confidence: &MetricConfidence,
) -> Result<(), ProcessFidelityError> {
    let level = confidence.level.get();
    let lower = confidence.lower.get();
    let upper = confidence.upper.get();

    if !level.is_finite() {
        return Err(ProcessFidelityError::InvalidConfidenceLevel {
            level,
        });
    }

    if !(0.0 < level && level < 1.0) {
        return Err(ProcessFidelityError::InvalidConfidenceLevel {
            level,
        });
    }

    if !lower.is_finite() || !upper.is_finite() {
        return Err(
            ProcessFidelityError::NonFiniteValue {
                context: "process-fidelity confidence interval",
            },
        );
    }

    if lower > upper {
        return Err(
            ProcessFidelityError::InvalidConfidenceInterval {
                lower,
                upper,
            },
        );
    }

    if lower < 0.0 || upper > 1.0 {
        return Err(
            ProcessFidelityError::InvalidConfidenceInterval {
                lower,
                upper,
            },
        );
    }

    Ok(())
}

fn validate_matching_unitaries(
    ideal: &ComplexMatrix,
    actual: &ComplexMatrix,
    tolerance: f64,
) -> Result<usize, ProcessFidelityError> {
    if !tolerance.is_finite() || tolerance < 0.0 {
        return Err(ProcessFidelityError::InvalidTolerance {
            value: tolerance,
        });
    }

    if ideal.rows() != ideal.columns() {
        return Err(ProcessFidelityError::MatrixNotSquare {
            rows: ideal.rows(),
            columns: ideal.columns(),
        });
    }

    if actual.rows() != actual.columns() {
        return Err(ProcessFidelityError::MatrixNotSquare {
            rows: actual.rows(),
            columns: actual.columns(),
        });
    }

    if ideal.rows() == 0 {
        return Err(ProcessFidelityError::InvalidDimension {
            dimension: 0,
        });
    }

    if ideal.shape() != actual.shape() {
        return Err(ProcessFidelityError::DimensionMismatch {
            left: ideal.rows(),
            right: actual.rows(),
        });
    }

    for (index, value) in ideal.data().iter().enumerate() {
        if !value.is_finite() {
            return Err(
                ProcessFidelityError::MatrixContainsNonFinite {
                    index,
                },
            );
        }
    }

    for (index, value) in actual.data().iter().enumerate() {
        if !value.is_finite() {
            return Err(
                ProcessFidelityError::MatrixContainsNonFinite {
                    index,
                },
            );
        }
    }

    validate_unitary(ideal, tolerance)?;
    validate_unitary(actual, tolerance)?;

    validate_hilbert_dimension(ideal.rows())?;

    Ok(ideal.rows())
}

fn validate_unitary(
    matrix: &ComplexMatrix,
    tolerance: f64,
) -> Result<(), ProcessFidelityError> {
    if matrix.rows() != matrix.columns() {
        return Err(ProcessFidelityError::MatrixNotSquare {
            rows: matrix.rows(),
            columns: matrix.columns(),
        });
    }

    let dagger = matrix.dagger();

    let product = dagger
        .multiply(matrix)
        .map_err(ProcessFidelityError::Fidelity)?;

    let identity =
        ComplexMatrix::identity(matrix.rows())
            .map_err(ProcessFidelityError::Fidelity)?;

    let deviation =
        product
            .max_difference(&identity)
            .map_err(ProcessFidelityError::Fidelity)?;

    if !deviation.is_finite() {
        return Err(ProcessFidelityError::NonFiniteValue {
            context: "unitary validation deviation",
        });
    }

    if deviation > tolerance {
        return Err(ProcessFidelityError::MatrixNotUnitary {
            maximum_deviation: deviation,
            tolerance,
        });
    }

    Ok(())
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn complex_identity(size: usize) -> ComplexMatrix {
        ComplexMatrix::identity(size)
            .expect("test identity matrix must be constructible")
    }

    #[test]
    fn identical_unitaries_have_unit_process_fidelity() {
        let identity = complex_identity(2);

        let result =
            analyze_unitary_channels(
                &identity,
                &identity,
            )
            .expect("identical unitary channels must succeed");

        assert!(
            (result.process_fidelity.value.get() - 1.0)
                .abs()
                < 1.0e-12
        );

        assert!(
            (result.process_infidelity.value.get())
                .abs()
                < 1.0e-12
        );

        let average =
            result
                .average_gate_fidelity
                .expect("Hilbert dimension is known");

        assert!(
            (average.value.get() - 1.0).abs()
                < 1.0e-12
        );
    }

    #[test]
    fn identical_two_qubit_unitaries_have_unit_fidelity() {
        let identity = complex_identity(4);

        let result =
            analyze_unitary_channels(
                &identity,
                &identity,
            )
            .expect("identical unitary channels must succeed");

        assert!(
            (result.process_fidelity.value.get() - 1.0)
                .abs()
                < 1.0e-12
        );

        assert!(
            (result
                .average_gate_fidelity
                .expect("dimension is known")
                .value
                .get()
                - 1.0)
                .abs()
                < 1.0e-12
        );
    }

    #[test]
    fn qubit_dimension_conversion_is_checked() {
        assert_eq!(
            qubit_count_to_hilbert_dimension(0)
                .expect("zero qubits has dimension one"),
            1
        );

        assert_eq!(
            qubit_count_to_hilbert_dimension(1)
                .expect("one qubit has dimension two"),
            2
        );

        assert_eq!(
            qubit_count_to_hilbert_dimension(2)
                .expect("two qubits have dimension four"),
            4
        );

        assert_eq!(
            qubit_count_to_hilbert_dimension(3)
                .expect("three qubits have dimension eight"),
            8
        );
    }

    #[test]
    fn invalid_fidelity_is_rejected() {
        let error =
            process_infidelity(1.1)
                .expect_err("fidelity above one must fail");

        assert!(matches!(
            error,
            ProcessFidelityError::FidelityOutOfRange {
                value: 1.1
            }
        ));
    }

    #[test]
    fn negative_fidelity_is_rejected() {
        let error =
            process_infidelity(-0.1)
                .expect_err("negative fidelity must fail");

        assert!(matches!(
            error,
            ProcessFidelityError::FidelityOutOfRange {
                value: -0.1
            }
        ));
    }

    #[test]
    fn zero_dimension_is_rejected() {
        let error =
            average_gate_fidelity_from_process(
                0.9,
                0,
            )
            .expect_err(
                "zero Hilbert dimension must fail",
            );

        assert!(matches!(
            error,
            ProcessFidelityError::InvalidDimension {
                dimension: 0
            }
        ));
    }

    #[test]
    fn unitary_matrix_mismatch_is_rejected() {
        let one = complex_identity(2);
        let two = complex_identity(4);

        let error =
            analyze_unitary_channels(&one, &two)
                .expect_err(
                    "different dimensions must fail",
                );

        assert!(matches!(
            error,
            ProcessFidelityError::DimensionMismatch {
                left: 2,
                right: 4
            }
        ));
    }

    #[test]
    fn non_unitary_matrix_is_rejected() {
        let matrix =
            ComplexMatrix::new(
                2,
                2,
                vec![
                    Complex64::new(1.0, 0.0),
                    Complex64::new(0.0, 0.0),
                    Complex64::new(0.0, 0.0),
                    Complex64::new(0.5, 0.0),
                ],
            )
            .expect("test matrix construction must succeed");

        let error =
            analyze_unitary_channels(
                &matrix,
                &matrix,
            )
            .expect_err(
                "non-unitary matrix must fail",
            );

        assert!(matches!(
            error,
            ProcessFidelityError::MatrixNotUnitary {
                ..
            }
        ));
    }

    #[test]
    fn scalar_process_result_preserves_source() {
        let result =
            analyze_scalar_process_fidelity(
                0.99,
                ProcessFidelityOptions::default(),
                Some(2),
                ProcessFidelitySource::ProcessTomography,
            )
            .expect(
                "valid scalar process fidelity must succeed",
            );

        assert!(
            result
                .process_fidelity
                .metadata
                .iter()
                .any(|metadata| {
                    metadata.key == "source"
                        && metadata.value
                            == "process_tomography"
                })
        );
    }

    #[test]
    fn average_gate_conversion_is_dimension_sensitive() {
        let process = 0.9;

        let one_qubit =
            average_gate_fidelity_from_process(
                process,
                2,
            )
            .expect("one-qubit conversion must succeed");

        let two_qubit =
            average_gate_fidelity_from_process(
                process,
                4,
            )
            .expect("two-qubit conversion must succeed");

        assert!(
            two_qubit.value.get()
                > one_qubit.value.get()
        );
    }

    #[test]
    fn process_metric_rejects_zero_shots() {
        let mut options =
            ProcessFidelityOptions::default();

        options.shot_count = Some(0);

        let error =
            build_process_metric(
                0.99,
                ProcessFidelitySource::ExternalCharacterization,
                &options,
            )
            .expect_err(
                "zero shots must be rejected",
            );

        assert!(matches!(
            error,
            ProcessFidelityError::ZeroShotCount
        ));
    }

    #[test]
    fn confidence_interval_is_preserved() {
        let confidence =
            MetricConfidence::new(
                0.95,
                0.90,
                0.99,
                ConfidenceMethod::Wilson,
            )
            .expect(
                "valid confidence interval must construct",
            );

        let mut options =
            ProcessFidelityOptions::default();

        options.confidence = Some(confidence);

        let metric =
            build_process_metric(
                0.95,
                ProcessFidelitySource::ProcessTomography,
                &options,
            )
            .expect(
                "metric with confidence must succeed",
            );

        assert!(
            metric.confidence.is_some()
        );
    }

    #[test]
    fn provenance_is_preserved() {
        let provenance =
            ProvenanceRef::new("process-test")
                .expect(
                    "provenance identifier must be valid",
                );

        let mut options =
            ProcessFidelityOptions::default();

        options.provenance = Some(provenance);

        let metric =
            build_process_metric(
                0.98,
                ProcessFidelitySource::ExternalCharacterization,
                &options,
            )
            .expect(
                "metric with provenance must succeed",
            );

        assert_eq!(
            metric
                .provenance
                .as_ref()
                .expect("provenance must exist")
                .source_id,
            "process-test"
        );
    }
}