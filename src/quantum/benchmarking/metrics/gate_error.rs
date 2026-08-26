//! Zamani Quantum Benchmarking — Gate Error Metrics
//!
//! Production implementation of gate-error and gate-infidelity metric
//! calculations.
//!
//! # Responsibility
//!
//! This module owns the mathematical conversion and construction of
//! gate-quality metrics. It deliberately does NOT:
//!
//! - generate quantum circuits;
//! - execute circuits;
//! - communicate with hardware;
//! - perform randomized-benchmarking sequence generation;
//! - fit RB decay curves;
//! - perform tomography;
//! - perform calibration discovery;
//! - perform backend capability negotiation;
//! - own the universal benchmark result envelope.
//!
//! Those responsibilities belong to the appropriate sibling subsystems.
//!
//! # Architecture
//!
//! ```text
//! raw observation / protocol analysis
//!              │
//!              ▼
//!      gate_error.rs
//!              │
//!       ┌──────┼────────┐
//!       ▼      ▼        ▼
//!   infidelity  EPG     EPC
//!       │      │        │
//!       └──────┼────────┘
//!              ▼
//!       core::metric::Metric
//!              │
//!              ▼
//!       core::result / reporting / analysis
//! ```
//!
//! # Important statistical boundary
//!
//! This module does not infer a gate error from raw experimental data unless
//! the caller has already performed the required statistical analysis.
//!
//! For example, randomized benchmarking produces a fitted decay parameter.
//! This module may convert that already-established decay parameter into an
//! error-per-Clifford estimate, but it does not perform the exponential fit.
//!
//! # Mathematical conventions
//!
//! For a fidelity-like quantity F in [0, 1]:
//!
//! ```text
//! infidelity = 1 - F
//! ```
//!
//! For a d-dimensional quantum system, the relationship between average gate
//! fidelity and entanglement/process fidelity is:
//!
//! ```text
//! F_avg = (d * F_e + 1) / (d + 1)
//! F_e   = ((d + 1) * F_avg - 1) / d
//! ```
//!
//! Consequently:
//!
//! ```text
//! r_avg = 1 - F_avg
//! r_e   = 1 - F_e
//! ```
//!
//! When converting an error-per-Clifford estimate into an error-per-gate
//! estimate, the conversion assumes an independent identical-error model:
//!
//! ```text
//! EPC = 1 - (1 - EPG)^g
//!
//! EPG = 1 - (1 - EPC)^(1/g)
//! ```
//!
//! where `g` is the effective number of gates represented by a Clifford.
//!
//! This assumption is NEVER hidden. The returned metric contains metadata
//! identifying the conversion model.
//!
//! # Production invariants
//!
//! 1. No NaN or infinity may enter this module.
//! 2. Probabilities, fidelities and error rates are constrained to [0, 1].
//! 3. Dimensions must be positive.
//! 4. Gate/Clifford counts must be positive when used as divisors.
//! 5. Confidence intervals must remain mathematically valid.
//! 6. Derived metrics retain sample/circuit/shot metadata where supplied.
//! 7. Statistical assumptions are recorded in metric metadata.
//! 8. No diagnostic printing occurs.
//! 9. No panic-based validation occurs for public inputs.
//! 10. Calculations must not silently clamp invalid scientific data.
//!
//! # Rust compatibility
//!
//! Designed for:
//!
//! - Rust 1.97
//! - Rust 1.97.1
//! - Rust 2021 edition
//!
//! No nightly features are used.
//!
//! # Integration
//!
//! The module depends only on `core::metric` and the Rust standard library.
//! This keeps it usable by:
//!
//! - randomized benchmarking;
//! - interleaved RB;
//! - cycle benchmarking;
//! - gate-fidelity protocols;
//! - process-fidelity protocols;
//! - hardware characterization;
//! - simulator validation;
//! - QEC physical-error analysis;
//! - reporting;
//! - Zamani-language benchmarking APIs.
//!
//! The intended module declaration is:
//!
//! ```ignore
//! // src/quantum/benchmarking/metrics/mod.rs
//! pub mod gate_error;
//! ```
//!
//! No change to `core::metric` is required by this file.

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

/// Mathematical constants used by this module.
const ZERO: f64 = 0.0;
const ONE: f64 = 1.0;

/// Minimum physically meaningful positive dimension.
const MIN_DIMENSION: u64 = 1;

/// Minimum positive number of gates represented by a Clifford/cycle.
const MIN_GATE_COUNT: u64 = 1;

/// Metadata key identifying the mathematical conversion model.
const MODEL_KEY: &str = "conversion_model";

/// Metadata key identifying an assumed effective gate count.
const EFFECTIVE_GATE_COUNT_KEY: &str = "effective_gate_count";

/// Metadata key identifying the Hilbert-space dimension.
const DIMENSION_KEY: &str = "dimension";

/// Metadata key identifying the source metric.
const SOURCE_METRIC_KEY: &str = "source_metric";

/// Metadata key identifying an assumption.
const ASSUMPTION_KEY: &str = "assumption";

/// Metadata key identifying the protocol family.
const PROTOCOL_KEY: &str = "protocol";

/// Metadata key identifying that a quantity is derived.
const DERIVED_FROM_KEY: &str = "derived_from";

/// Metadata key identifying the model version.
const MODEL_VERSION_KEY: &str = "model_version";

/// Version of the mathematical conversion model implemented here.
const MODEL_VERSION: &str = "1";

/// Standard gate-error result bundle.
///
/// This is useful when a caller wants to calculate several related metrics
/// from the same source fidelity without repeatedly reconstructing the
/// associated metadata.
///
/// The individual `Metric` objects remain the authoritative values.
///
/// # Example
///
/// ```ignore
/// let result = GateErrorMetrics::from_average_gate_fidelity(0.99)?;
///
/// assert_eq!(result.average_gate_fidelity.value.get(), 0.99);
/// assert!((result.gate_infidelity.value.get() - 0.01).abs() < 1e-12);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct GateErrorMetrics {
    /// Source average gate fidelity.
    pub average_gate_fidelity: Metric,

    /// Gate infidelity, equal to `1 - average_gate_fidelity`.
    pub gate_infidelity: Metric,
}

impl GateErrorMetrics {
    /// Creates average-gate-fidelity and gate-infidelity metrics.
    pub fn from_average_gate_fidelity(fidelity: f64) -> MetricResult<Self> {
        Self::from_average_gate_fidelity_with_metadata(
            fidelity,
            None,
            None,
            None,
            None,
        )
    }

    /// Creates average-gate-fidelity and gate-infidelity metrics while
    /// preserving experimental context.
    pub fn from_average_gate_fidelity_with_metadata(
        fidelity: f64,
        uncertainty: Option<f64>,
        confidence: Option<MetricConfidence>,
        sample_count: Option<u64>,
        circuit_count: Option<u64>,
    ) -> MetricResult<Self> {
        let validate = ValidatedProbability::new(fidelity)?;

        let average_gate_fidelity = build_metric(
            MetricKind::AverageGateFidelity,
            validate.value(),
            MetricUnit::Probability,
            MetricQuality::Observed,
            uncertainty,
            confidence.clone(),
            sample_count,
            None,
            circuit_count,
            vec![
                metadata(SOURCE_METRIC_KEY, "average_gate_fidelity")?,
            ],
        )?;

        let transformed_confidence =
            transform_complement_confidence(confidence)?;

        let transformed_uncertainty =
            uncertainty.map(|value| validate_uncertainty(value, "uncertainty"));

        let gate_infidelity = build_metric(
            MetricKind::GateInfidelity,
            validate.complement(),
            MetricUnit::Probability,
            MetricQuality::Derived,
            transformed_uncertainty.transpose()?,
            transformed_confidence,
            sample_count,
            None,
            circuit_count,
            vec![
                metadata(DERIVED_FROM_KEY, "average_gate_fidelity")?,
                metadata(MODEL_KEY, "complement")?,
                metadata(MODEL_VERSION_KEY, MODEL_VERSION)?,
            ],
        )?;

        Ok(Self {
            average_gate_fidelity,
            gate_infidelity,
        })
    }
}

/// Calculates gate infidelity from average gate fidelity.
///
/// ```text
/// gate_infidelity = 1 - average_gate_fidelity
/// ```
///
/// This function is purely mathematical. It does not imply a particular
/// experimental protocol.
pub fn gate_infidelity_from_average_gate_fidelity(
    average_gate_fidelity: f64,
) -> MetricResult<Metric> {
    let value = ValidatedProbability::new(average_gate_fidelity)?;

    build_metric(
        MetricKind::GateInfidelity,
        value.complement(),
        MetricUnit::Probability,
        MetricQuality::Derived,
        None,
        None,
        None,
        None,
        None,
        vec![
            metadata(DERIVED_FROM_KEY, "average_gate_fidelity")?,
            metadata(MODEL_KEY, "complement")?,
            metadata(MODEL_VERSION_KEY, MODEL_VERSION)?,
        ],
    )
}

/// Calculates average gate fidelity from gate infidelity.
///
/// ```text
/// average_gate_fidelity = 1 - gate_infidelity
/// ```
pub fn average_gate_fidelity_from_gate_infidelity(
    gate_infidelity: f64,
) -> MetricResult<Metric> {
    let value = ValidatedProbability::new(gate_infidelity)?;

    build_metric(
        MetricKind::AverageGateFidelity,
        value.complement(),
        MetricUnit::Probability,
        MetricQuality::Derived,
        None,
        None,
        None,
        None,
        None,
        vec![
            metadata(DERIVED_FROM_KEY, "gate_infidelity")?,
            metadata(MODEL_KEY, "complement")?,
            metadata(MODEL_VERSION_KEY, MODEL_VERSION)?,
        ],
    )
}

/// Calculates process infidelity from process fidelity.
///
/// ```text
/// process_infidelity = 1 - process_fidelity
/// ```
pub fn process_infidelity_from_process_fidelity(
    process_fidelity: f64,
) -> MetricResult<Metric> {
    let value = ValidatedProbability::new(process_fidelity)?;

    build_metric(
        MetricKind::ProcessInfidelity,
        value.complement(),
        MetricUnit::Probability,
        MetricQuality::Derived,
        None,
        None,
        None,
        None,
        None,
        vec![
            metadata(DERIVED_FROM_KEY, "process_fidelity")?,
            metadata(MODEL_KEY, "complement")?,
            metadata(MODEL_VERSION_KEY, MODEL_VERSION)?,
        ],
    )
}

/// Calculates cycle error from cycle fidelity.
///
/// ```text
/// cycle_error = 1 - cycle_fidelity
/// ```
pub fn cycle_error_from_cycle_fidelity(
    cycle_fidelity: f64,
) -> MetricResult<Metric> {
    let value = ValidatedProbability::new(cycle_fidelity)?;

    build_metric(
        MetricKind::CycleError,
        value.complement(),
        MetricUnit::Probability,
        MetricQuality::Derived,
        None,
        None,
        None,
        None,
        None,
        vec![
            metadata(DERIVED_FROM_KEY, "cycle_fidelity")?,
            metadata(MODEL_KEY, "complement")?,
            metadata(MODEL_VERSION_KEY, MODEL_VERSION)?,
        ],
    )
}

/// Calculates error rate from a fidelity-like quantity.
///
/// This function is intentionally generic and should only be used when the
/// supplied fidelity really represents the quantity whose complement is
/// scientifically defined as the desired error rate.
///
/// For protocol-specific quantities, prefer one of the explicitly named
/// functions in this module.
pub fn error_rate_from_fidelity(
    fidelity: f64,
) -> MetricResult<Metric> {
    let value = ValidatedProbability::new(fidelity)?;

    build_metric(
        MetricKind::ErrorRate,
        value.complement(),
        MetricUnit::Probability,
        MetricQuality::Derived,
        None,
        None,
        None,
        None,
        None,
        vec![
            metadata(DERIVED_FROM_KEY, "fidelity")?,
            metadata(MODEL_KEY, "complement")?,
            metadata(MODEL_VERSION_KEY, MODEL_VERSION)?,
        ],
    )
}

/// Converts entanglement/process fidelity into average gate fidelity.
///
/// For a d-dimensional system:
///
/// ```text
/// F_avg = (d F_e + 1) / (d + 1)
/// ```
///
/// `dimension` is the Hilbert-space dimension, not the number of physical
/// qubits.
///
/// Examples:
///
/// - one qubit: d = 2
/// - two qubits: d = 4
/// - n qubits: d = 2^n
///
/// The caller must provide the actual dimension because this module must not
/// make assumptions about the physical system.
pub fn average_gate_fidelity_from_entanglement_fidelity(
    entanglement_fidelity: f64,
    dimension: u64,
) -> MetricResult<Metric> {
    validate_dimension(dimension)?;

    let fidelity = ValidatedProbability::new(entanglement_fidelity)?;

    let d = dimension as f64;
    let average = (d * fidelity.value() + ONE) / (d + ONE);

    validate_unit_interval(average)?;

    build_metric(
        MetricKind::AverageGateFidelity,
        average,
        MetricUnit::Probability,
        MetricQuality::Derived,
        None,
        None,
        None,
        None,
        None,
        vec![
            metadata(DERIVED_FROM_KEY, "entanglement_fidelity")?,
            metadata(DIMENSION_KEY, dimension.to_string())?,
            metadata(MODEL_KEY, "average_gate_fidelity_from_entanglement_fidelity")?,
            metadata(MODEL_VERSION_KEY, MODEL_VERSION)?,
        ],
    )
}

/// Converts average gate fidelity into entanglement/process fidelity.
///
/// ```text
/// F_e = ((d + 1) F_avg - 1) / d
/// ```
///
/// The conversion is only valid for a d-dimensional channel under the
/// standard fidelity relationship.
pub fn entanglement_fidelity_from_average_gate_fidelity(
    average_gate_fidelity: f64,
    dimension: u64,
) -> MetricResult<Metric> {
    validate_dimension(dimension)?;

    let fidelity = ValidatedProbability::new(average_gate_fidelity)?;

    let d = dimension as f64;
    let entanglement = ((d + ONE) * fidelity.value() - ONE) / d;

    validate_unit_interval(entanglement)?;

    build_metric(
        MetricKind::EntanglementFidelity,
        entanglement,
        MetricUnit::Probability,
        MetricQuality::Derived,
        None,
        None,
        None,
        None,
        None,
        vec![
            metadata(DERIVED_FROM_KEY, "average_gate_fidelity")?,
            metadata(DIMENSION_KEY, dimension.to_string())?,
            metadata(MODEL_KEY, "entanglement_fidelity_from_average_gate_fidelity")?,
            metadata(MODEL_VERSION_KEY, MODEL_VERSION)?,
        ],
    )
}

/// Calculates process/entanglement infidelity from average gate fidelity.
///
/// ```text
/// F_e = ((d + 1) F_avg - 1) / d
/// process_infidelity = 1 - F_e
/// ```
pub fn process_infidelity_from_average_gate_fidelity(
    average_gate_fidelity: f64,
    dimension: u64,
) -> MetricResult<Metric> {
    let fidelity =
        entanglement_fidelity_from_average_gate_fidelity(
            average_gate_fidelity,
            dimension,
        )?;

    let entanglement_fidelity = fidelity.value.get();

    let infidelity = ONE - entanglement_fidelity;

    build_metric(
        MetricKind::ProcessInfidelity,
        infidelity,
        MetricUnit::Probability,
        MetricQuality::Derived,
        None,
        None,
        None,
        None,
        None,
        vec![
            metadata(DERIVED_FROM_KEY, "average_gate_fidelity")?,
            metadata(DIMENSION_KEY, dimension.to_string())?,
            metadata(MODEL_KEY, "process_infidelity_from_average_gate_fidelity")?,
            metadata(MODEL_VERSION_KEY, MODEL_VERSION)?,
        ],
    )
}

/// Calculates average gate infidelity from entanglement/process fidelity.
///
/// ```text
/// F_avg = (d F_e + 1) / (d + 1)
/// r_avg = 1 - F_avg
/// ```
pub fn gate_infidelity_from_entanglement_fidelity(
    entanglement_fidelity: f64,
    dimension: u64,
) -> MetricResult<Metric> {
    let average =
        average_gate_fidelity_from_entanglement_fidelity(
            entanglement_fidelity,
            dimension,
        )?;

    gate_infidelity_from_average_gate_fidelity(
        average.value.get(),
    )
}

/// Calculates error-per-gate from error-per-Clifford.
///
/// The conversion is:
///
/// ```text
/// EPC = 1 - (1 - EPG)^g
/// EPG = 1 - (1 - EPC)^(1/g)
/// ```
///
/// where `g` is the effective number of gates represented by a Clifford.
///
/// # Scientific assumption
///
/// This is a model conversion. It assumes independent, identically
/// distributed gate errors and treats the effective gate count as a useful
/// representation of the Clifford. It is NOT a direct hardware measurement.
///
/// The returned metric explicitly records this assumption.
pub fn error_per_gate_from_error_per_clifford(
    error_per_clifford: f64,
    effective_gate_count: u64,
) -> MetricResult<Metric> {
    validate_gate_count(effective_gate_count)?;

    let epc = ValidatedProbability::new(error_per_clifford)?;

    let gates = effective_gate_count as f64;

    // `1 - EPC` is in [0, 1], so the real-valued positive root is defined.
    let survival = ONE - epc.value();
    let error_per_gate = ONE - survival.powf(ONE / gates);

    validate_unit_interval(error_per_gate)?;

    build_metric(
        MetricKind::ErrorPerGate,
        error_per_gate,
        MetricUnit::Probability,
        MetricQuality::Estimated,
        None,
        None,
        None,
        None,
        None,
        vec![
            metadata(DERIVED_FROM_KEY, "error_per_clifford")?,
            metadata(EFFECTIVE_GATE_COUNT_KEY, effective_gate_count.to_string())?,
            metadata(
                MODEL_KEY,
                "independent_identical_gate_error_conversion",
            )?,
            metadata(
                ASSUMPTION_KEY,
                "independent_identically_distributed_gate_errors",
            )?,
            metadata(MODEL_VERSION_KEY, MODEL_VERSION)?,
        ],
    )
}

/// Calculates error-per-Clifford from error-per-gate.
///
/// ```text
/// EPC = 1 - (1 - EPG)^g
/// ```
///
/// where `g` is the effective number of gates represented by a Clifford.
///
/// This is the inverse of
/// [`error_per_gate_from_error_per_clifford`].
pub fn error_per_clifford_from_error_per_gate(
    error_per_gate: f64,
    effective_gate_count: u64,
) -> MetricResult<Metric> {
    validate_gate_count(effective_gate_count)?;

    let epg = ValidatedProbability::new(error_per_gate)?;

    let gates = effective_gate_count as f64;

    let survival =
        (ONE - epg.value()).powf(gates);

    let error_per_clifford = ONE - survival;

    validate_unit_interval(error_per_clifford)?;

    build_metric(
        MetricKind::ErrorPerClifford,
        error_per_clifford,
        MetricUnit::Probability,
        MetricQuality::Estimated,
        None,
        None,
        None,
        None,
        None,
        vec![
            metadata(DERIVED_FROM_KEY, "error_per_gate")?,
            metadata(EFFECTIVE_GATE_COUNT_KEY, effective_gate_count.to_string())?,
            metadata(
                MODEL_KEY,
                "independent_identical_gate_error_conversion",
            )?,
            metadata(
                ASSUMPTION_KEY,
                "independent_identically_distributed_gate_errors",
            )?,
            metadata(MODEL_VERSION_KEY, MODEL_VERSION)?,
        ],
    )
}

/// Calculates randomized-benchmarking error per Clifford from a fitted decay
/// parameter.
///
/// For a d-dimensional system under the standard depolarizing RB model:
///
/// ```text
/// EPC = (d - 1) / d * (1 - p)
/// ```
///
/// where `p` is the fitted RB decay parameter.
///
/// # Important
///
/// This function accepts an already-fitted decay parameter. It does not fit
/// experimental data.
///
/// The returned metric is marked `Estimated` because it is model-derived.
pub fn error_per_clifford_from_rb_decay(
    decay_parameter: f64,
    dimension: u64,
) -> MetricResult<Metric> {
    validate_dimension(dimension)?;

    let p = ValidatedProbability::new(decay_parameter)?;

    let d = dimension as f64;

    let error_per_clifford =
        ((d - ONE) / d) * (ONE - p.value());

    validate_unit_interval(error_per_clifford)?;

    build_metric(
        MetricKind::ErrorPerClifford,
        error_per_clifford,
        MetricUnit::Probability,
        MetricQuality::Estimated,
        None,
        None,
        None,
        None,
        None,
        vec![
            metadata(DERIVED_FROM_KEY, "rb_decay_parameter")?,
            metadata(DIMENSION_KEY, dimension.to_string())?,
            metadata(PROTOCOL_KEY, "randomized_benchmarking")?,
            metadata(
                MODEL_KEY,
                "depolarizing_rb_error_per_clifford",
            )?,
            metadata(
                ASSUMPTION_KEY,
                "standard_depolarizing_rb_model",
            )?,
            metadata(MODEL_VERSION_KEY, MODEL_VERSION)?,
        ],
    )
}

/// Calculates the RB decay parameter from an error-per-Clifford estimate.
///
/// This is the inverse of
/// [`error_per_clifford_from_rb_decay`]:
///
/// ```text
/// p = 1 - d/(d - 1) * EPC
/// ```
///
/// This function is valid only for dimensions greater than one.
pub fn rb_decay_from_error_per_clifford(
    error_per_clifford: f64,
    dimension: u64,
) -> MetricResult<Metric> {
    if dimension <= 1 {
        return Err(MetricError::OutOfRange {
            metric: DIMENSION_KEY.to_string(),
            value: dimension as f64,
            minimum: 2.0,
            maximum: u64::MAX as f64,
        });
    }

    let epc = ValidatedProbability::new(error_per_clifford)?;

    let d = dimension as f64;

    let decay =
        ONE - (d / (d - ONE)) * epc.value();

    validate_unit_interval(decay)?;

    build_metric(
        MetricKind::Custom("rb_decay_parameter".to_string()),
        decay,
        MetricUnit::Probability,
        MetricQuality::Derived,
        None,
        None,
        None,
        None,
        None,
        vec![
            metadata(DERIVED_FROM_KEY, "error_per_clifford")?,
            metadata(DIMENSION_KEY, dimension.to_string())?,
            metadata(PROTOCOL_KEY, "randomized_benchmarking")?,
            metadata(
                MODEL_KEY,
                "depolarizing_rb_decay_from_error_per_clifford",
            )?,
            metadata(
                ASSUMPTION_KEY,
                "standard_depolarizing_rb_model",
            )?,
            metadata(MODEL_VERSION_KEY, MODEL_VERSION)?,
        ],
    )
}

/// Converts an average-gate-fidelity confidence interval into a
/// gate-infidelity confidence interval.
///
/// Since:
///
/// ```text
/// r = 1 - F
/// ```
///
/// the interval endpoints are reversed:
///
/// ```text
/// [F_low, F_high]
///     ->
/// [1-F_high, 1-F_low]
/// ```
pub fn gate_infidelity_confidence_from_fidelity(
    confidence: &MetricConfidence,
) -> MetricResult<MetricConfidence> {
    if confidence.lower.get() < ZERO
        || confidence.upper.get() > ONE
    {
        return Err(MetricError::ConfidenceIntervalOutOfRange {
            metric: MetricKind::AverageGateFidelity.id(),
            lower: confidence.lower.get(),
            upper: confidence.upper.get(),
        });
    }

    MetricConfidence::new(
        confidence.level.get(),
        ONE - confidence.upper.get(),
        ONE - confidence.lower.get(),
        confidence.method.clone(),
    )
}

/// Converts an error-rate confidence interval back into a
/// fidelity confidence interval.
///
/// ```text
/// [r_low, r_high]
///     ->
/// [1-r_high, 1-r_low]
/// ```
pub fn fidelity_confidence_from_gate_infidelity(
    confidence: &MetricConfidence,
) -> MetricResult<MetricConfidence> {
    if confidence.lower.get() < ZERO
        || confidence.upper.get() > ONE
    {
        return Err(MetricError::ConfidenceIntervalOutOfRange {
            metric: MetricKind::GateInfidelity.id(),
            lower: confidence.lower.get(),
            upper: confidence.upper.get(),
        });
    }

    MetricConfidence::new(
        confidence.level.get(),
        ONE - confidence.upper.get(),
        ONE - confidence.lower.get(),
        confidence.method.clone(),
    )
}

/// Propagates an average-gate-fidelity confidence interval through the
/// standard entanglement-fidelity relationship.
///
/// ```text
/// F_e = ((d + 1)F_avg - 1) / d
/// ```
///
/// The transformation is monotonic for positive `d`.
pub fn entanglement_fidelity_confidence_from_average_gate_fidelity(
    confidence: &MetricConfidence,
    dimension: u64,
) -> MetricResult<MetricConfidence> {
    validate_dimension(dimension)?;

    if confidence.lower.get() < ZERO
        || confidence.upper.get() > ONE
    {
        return Err(MetricError::ConfidenceIntervalOutOfRange {
            metric: MetricKind::AverageGateFidelity.id(),
            lower: confidence.lower.get(),
            upper: confidence.upper.get(),
        });
    }

    let d = dimension as f64;

    let lower =
        ((d + ONE) * confidence.lower.get() - ONE) / d;

    let upper =
        ((d + ONE) * confidence.upper.get() - ONE) / d;

    validate_unit_interval(lower)?;
    validate_unit_interval(upper)?;

    MetricConfidence::new(
        confidence.level.get(),
        lower,
        upper,
        confidence.method.clone(),
    )
}

/// Propagates an average-gate-fidelity confidence interval into process
/// infidelity.
///
/// ```text
/// process_infidelity = 1 - F_e
/// ```
pub fn process_infidelity_confidence_from_average_gate_fidelity(
    confidence: &MetricConfidence,
    dimension: u64,
) -> MetricResult<MetricConfidence> {
    let entanglement =
        entanglement_fidelity_confidence_from_average_gate_fidelity(
            confidence,
            dimension,
        )?;

    fidelity_confidence_from_gate_infidelity(&entanglement)
}

/// Converts an error-per-Clifford confidence interval to an
/// error-per-gate confidence interval under the same independent-error model.
///
/// Because:
///
/// ```text
/// EPG = 1 - (1 - EPC)^(1/g)
/// ```
///
/// is monotonic increasing in EPC, the endpoints are transformed in the same
/// order.
pub fn error_per_gate_confidence_from_error_per_clifford(
    confidence: &MetricConfidence,
    effective_gate_count: u64,
) -> MetricResult<MetricConfidence> {
    validate_gate_count(effective_gate_count)?;

    if confidence.lower.get() < ZERO
        || confidence.upper.get() > ONE
    {
        return Err(MetricError::ConfidenceIntervalOutOfRange {
            metric: MetricKind::ErrorPerClifford.id(),
            lower: confidence.lower.get(),
            upper: confidence.upper.get(),
        });
    }

    let gates = effective_gate_count as f64;

    let lower_survival =
        (ONE - confidence.lower.get()).powf(ONE / gates);

    let upper_survival =
        (ONE - confidence.upper.get()).powf(ONE / gates);

    let lower = ONE - lower_survival;
    let upper = ONE - upper_survival;

    MetricConfidence::new(
        confidence.level.get(),
        lower,
        upper,
        confidence.method.clone(),
    )
}

/// Converts an error-per-gate confidence interval into an
/// error-per-Clifford confidence interval.
pub fn error_per_clifford_confidence_from_error_per_gate(
    confidence: &MetricConfidence,
    effective_gate_count: u64,
) -> MetricResult<MetricConfidence> {
    validate_gate_count(effective_gate_count)?;

    if confidence.lower.get() < ZERO
        || confidence.upper.get() > ONE
    {
        return Err(MetricError::ConfidenceIntervalOutOfRange {
            metric: MetricKind::ErrorPerGate.id(),
            lower: confidence.lower.get(),
            upper: confidence.upper.get(),
        });
    }

    let gates = effective_gate_count as f64;

    let lower =
        ONE - (ONE - confidence.lower.get()).powf(gates);

    let upper =
        ONE - (ONE - confidence.upper.get()).powf(gates);

    MetricConfidence::new(
        confidence.level.get(),
        lower,
        upper,
        confidence.method.clone(),
    )
}

/// Constructs an error-per-gate metric while retaining statistical context.
///
/// This helper is intended for protocol modules that have already estimated
/// EPG and want a canonical Zamani `Metric`.
pub fn error_per_gate_metric(
    value: f64,
    uncertainty: Option<f64>,
    confidence: Option<MetricConfidence>,
    sample_count: Option<u64>,
    shot_count: Option<u64>,
    circuit_count: Option<u64>,
) -> MetricResult<Metric> {
    build_metric(
        MetricKind::ErrorPerGate,
        value,
        MetricUnit::Probability,
        MetricQuality::Estimated,
        uncertainty,
        confidence,
        sample_count,
        shot_count,
        circuit_count,
        Vec::new(),
    )
}

/// Constructs an error-per-Clifford metric while retaining statistical
/// context.
///
/// The caller is responsible for ensuring that the supplied value actually
/// represents EPC obtained from a valid protocol analysis.
pub fn error_per_clifford_metric(
    value: f64,
    uncertainty: Option<f64>,
    confidence: Option<MetricConfidence>,
    sample_count: Option<u64>,
    shot_count: Option<u64>,
    circuit_count: Option<u64>,
) -> MetricResult<Metric> {
    build_metric(
        MetricKind::ErrorPerClifford,
        value,
        MetricUnit::Probability,
        MetricQuality::Estimated,
        uncertainty,
        confidence,
        sample_count,
        shot_count,
        circuit_count,
        Vec::new(),
    )
}

/// Constructs a cycle-error metric.
///
/// This function does not determine cycle fidelity. The caller must provide
/// the already-derived cycle error.
pub fn cycle_error_metric(
    value: f64,
    uncertainty: Option<f64>,
    confidence: Option<MetricConfidence>,
    sample_count: Option<u64>,
    shot_count: Option<u64>,
    circuit_count: Option<u64>,
) -> MetricResult<Metric> {
    build_metric(
        MetricKind::CycleError,
        value,
        MetricUnit::Probability,
        MetricQuality::Estimated,
        uncertainty,
        confidence,
        sample_count,
        shot_count,
        circuit_count,
        Vec::new(),
    )
}

/// Validated probability-like input.
///
/// Keeping validation centralized prevents individual public functions from
/// accidentally implementing different range rules.
#[derive(Debug, Clone, Copy, PartialEq)]
struct ValidatedProbability {
    value: f64,
}

impl ValidatedProbability {
    fn new(value: f64) -> MetricResult<Self> {
        if !value.is_finite() {
            return Err(MetricError::NonFiniteValue { value });
        }

        if !(ZERO..=ONE).contains(&value) {
            return Err(MetricError::OutOfRange {
                metric: "probability_or_fidelity".to_string(),
                value,
                minimum: ZERO,
                maximum: ONE,
            });
        }

        Ok(Self { value })
    }

    #[inline]
    fn value(self) -> f64 {
        self.value
    }

    #[inline]
    fn complement(self) -> f64 {
        ONE - self.value
    }
}

/// Validates a generic unit-interval result.
fn validate_unit_interval(value: f64) -> MetricResult<()> {
    if !value.is_finite() {
        return Err(MetricError::NonFiniteValue { value });
    }

    if !(ZERO..=ONE).contains(&value) {
        return Err(MetricError::OutOfRange {
            metric: "unit_interval".to_string(),
            value,
            minimum: ZERO,
            maximum: ONE,
        });
    }

    Ok(())
}

/// Validates a Hilbert-space dimension.
fn validate_dimension(dimension: u64) -> MetricResult<()> {
    if dimension < MIN_DIMENSION {
        return Err(MetricError::OutOfRange {
            metric: DIMENSION_KEY.to_string(),
            value: dimension as f64,
            minimum: MIN_DIMENSION as f64,
            maximum: u64::MAX as f64,
        });
    }

    Ok(())
}

/// Validates an effective gate count.
fn validate_gate_count(count: u64) -> MetricResult<()> {
    if count < MIN_GATE_COUNT {
        return Err(MetricError::OutOfRange {
            metric: EFFECTIVE_GATE_COUNT_KEY.to_string(),
            value: count as f64,
            minimum: MIN_GATE_COUNT as f64,
            maximum: u64::MAX as f64,
        });
    }

    Ok(())
}

/// Validates an optional uncertainty.
fn validate_uncertainty(
    value: f64,
    field: &'static str,
) -> MetricResult<f64> {
    if !value.is_finite() {
        return Err(MetricError::NonFiniteValue { value });
    }

    if value < ZERO {
        return Err(MetricError::NegativeUncertainty { value });
    }

    if field.is_empty() {
        // This branch is intentionally unreachable for the internal callers,
        // but keeps the validation contract explicit.
        return Err(MetricError::EmptyIdentifier { field });
    }

    Ok(value)
}

/// Transforms a fidelity confidence interval into an infidelity confidence
/// interval when present.
fn transform_complement_confidence(
    confidence: Option<MetricConfidence>,
) -> MetricResult<Option<MetricConfidence>> {
    confidence
        .as_ref()
        .map(gate_infidelity_confidence_from_fidelity)
        .transpose()
}

/// Creates a metadata item and converts its validation error into the
/// canonical metric error.
fn metadata(
    key: &str,
    value: impl Into<String>,
) -> MetricResult<MetricMetadata> {
    MetricMetadata::new(key, value)
}

/// Canonical metric construction helper.
///
/// All public metric constructors in this module eventually pass through this
/// function so the universal `core::metric::Metric` validation rules remain
/// authoritative.
fn build_metric(
    kind: MetricKind,
    value: f64,
    unit: MetricUnit,
    quality: MetricQuality,
    uncertainty: Option<f64>,
    confidence: Option<MetricConfidence>,
    sample_count: Option<u64>,
    shot_count: Option<u64>,
    circuit_count: Option<u64>,
    metadata_entries: Vec<MetricMetadata>,
) -> MetricResult<Metric> {
    let mut metric = Metric::new(kind, unit, value)?
        .with_quality(quality);

    if let Some(value) = uncertainty {
        metric = metric.with_uncertainty(value)?;
    }

    if let Some(confidence) = confidence {
        metric = metric.with_confidence(confidence)?;
    }

    if let Some(count) = sample_count {
        metric = metric.with_sample_count(count)?;
    }

    if let Some(count) = shot_count {
        metric = metric.with_shot_count(count)?;
    }

    if let Some(count) = circuit_count {
        metric = metric.with_circuit_count(count)?;
    }

    for entry in metadata_entries {
        metric = metric.with_metadata(entry);
    }

    metric.validate()?;

    Ok(metric)
}

/// Tests for mathematical correctness and production invariants.
#[cfg(test)]
mod tests {
    use super::*;

    fn assert_close(left: f64, right: f64, tolerance: f64) {
        assert!(
            (left - right).abs() <= tolerance,
            "left={left}, right={right}, tolerance={tolerance}"
        );
    }

    #[test]
    fn gate_infidelity_is_complement_of_fidelity() {
        let metric =
            gate_infidelity_from_average_gate_fidelity(0.99)
                .expect("valid fidelity");

        assert_close(
            metric.value.get(),
            0.01,
            1e-12,
        );

        assert_eq!(
            metric.kind,
            MetricKind::GateInfidelity
        );

        assert_eq!(
            metric.unit,
            MetricUnit::Probability
        );
    }

    #[test]
    fn average_gate_fidelity_is_complement_of_infidelity() {
        let metric =
            average_gate_fidelity_from_gate_infidelity(0.01)
                .expect("valid error");

        assert_close(
            metric.value.get(),
            0.99,
            1e-12,
        );
    }

    #[test]
    fn process_infidelity_is_complement_of_process_fidelity() {
        let metric =
            process_infidelity_from_process_fidelity(0.999)
                .expect("valid fidelity");

        assert_close(
            metric.value.get(),
            0.001,
            1e-12,
        );
    }

    #[test]
    fn cycle_error_is_complement_of_cycle_fidelity() {
        let metric =
            cycle_error_from_cycle_fidelity(0.98)
                .expect("valid fidelity");

        assert_close(
            metric.value.get(),
            0.02,
            1e-12,
        );
    }

    #[test]
    fn one_qubit_entanglement_fidelity_conversion_is_correct() {
        let metric =
            average_gate_fidelity_from_entanglement_fidelity(
                0.99,
                2,
            )
            .expect("valid one-qubit fidelity");

        let expected =
            (2.0 * 0.99 + 1.0) / 3.0;

        assert_close(
            metric.value.get(),
            expected,
            1e-12,
        );
    }

    #[test]
    fn inverse_fidelity_conversion_is_correct() {
        let average =
            average_gate_fidelity_from_entanglement_fidelity(
                0.99,
                2,
            )
            .expect("valid fidelity");

        let entanglement =
            entanglement_fidelity_from_average_gate_fidelity(
                average.value.get(),
                2,
            )
            .expect("valid inverse conversion");

        assert_close(
            entanglement.value.get(),
            0.99,
            1e-12,
        );
    }

    #[test]
    fn process_infidelity_from_average_fidelity_is_correct() {
        let metric =
            process_infidelity_from_average_gate_fidelity(
                0.99,
                2,
            )
            .expect("valid fidelity");

        let expected_entanglement =
            ((2.0 + 1.0) * 0.99 - 1.0) / 2.0;

        let expected_error =
            1.0 - expected_entanglement;

        assert_close(
            metric.value.get(),
            expected_error,
            1e-12,
        );
    }

    #[test]
    fn epc_to_epg_is_inverse_of_epg_to_epc() {
        let epg = 0.001;
        let gates = 5;

        let epc =
            error_per_clifford_from_error_per_gate(
                epg,
                gates,
            )
            .expect("valid EPG");

        let recovered =
            error_per_gate_from_error_per_clifford(
                epc.value.get(),
                gates,
            )
            .expect("valid EPC");

        assert_close(
            recovered.value.get(),
            epg,
            1e-12,
        );
    }

    #[test]
    fn zero_error_is_preserved() {
        let metric =
            error_per_gate_from_error_per_clifford(
                0.0,
                10,
            )
            .expect("zero error is valid");

        assert_eq!(
            metric.value.get(),
            0.0
        );
    }

    #[test]
    fn perfect_fidelity_has_zero_error() {
        let metric =
            gate_infidelity_from_average_gate_fidelity(1.0)
                .expect("perfect fidelity is valid");

        assert_eq!(
            metric.value.get(),
            0.0
        );
    }

    #[test]
    fn zero_fidelity_has_unit_error() {
        let metric =
            gate_infidelity_from_average_gate_fidelity(0.0)
                .expect("zero fidelity is mathematically valid");

        assert_eq!(
            metric.value.get(),
            1.0
        );
    }

    #[test]
    fn rb_decay_conversion_is_correct_for_qubit() {
        let metric =
            error_per_clifford_from_rb_decay(
                0.99,
                2,
            )
            .expect("valid RB decay");

        assert_close(
            metric.value.get(),
            0.005,
            1e-12,
        );
    }

    #[test]
    fn rb_decay_conversion_is_invertible() {
        let original_decay = 0.98;

        let epc =
            error_per_clifford_from_rb_decay(
                original_decay,
                2,
            )
            .expect("valid decay");

        let recovered =
            rb_decay_from_error_per_clifford(
                epc.value.get(),
                2,
            )
            .expect("valid EPC");

        assert_close(
            recovered.value.get(),
            original_decay,
            1e-12,
        );
    }

    #[test]
    fn rb_decay_for_two_level_system_has_expected_formula() {
        let metric =
            error_per_clifford_from_rb_decay(
                0.95,
                2,
            )
            .expect("valid decay");

        // (d - 1) / d * (1 - p)
        // = 1 / 2 * 0.05
        assert_close(
            metric.value.get(),
            0.025,
            1e-12,
        );
    }

    #[test]
    fn invalid_probability_is_rejected() {
        assert!(
            gate_infidelity_from_average_gate_fidelity(
                -0.01
            )
            .is_err()
        );

        assert!(
            gate_infidelity_from_average_gate_fidelity(
                1.01
            )
            .is_err()
        );
    }

    #[test]
    fn nan_is_rejected() {
        assert!(
            gate_infidelity_from_average_gate_fidelity(
                f64::NAN
            )
            .is_err()
        );
    }

    #[test]
    fn infinity_is_rejected() {
        assert!(
            gate_infidelity_from_average_gate_fidelity(
                f64::INFINITY
            )
            .is_err()
        );
    }

    #[test]
    fn zero_dimension_is_rejected() {
        assert!(
            average_gate_fidelity_from_entanglement_fidelity(
                0.99,
                0,
            )
            .is_err()
        );
    }

    #[test]
    fn zero_effective_gate_count_is_rejected() {
        assert!(
            error_per_gate_from_error_per_clifford(
                0.01,
                0,
            )
            .is_err()
        );
    }

    #[test]
    fn dimension_one_is_rejected_for_rb_decay_inverse() {
        assert!(
            rb_decay_from_error_per_clifford(
                0.01,
                1,
            )
            .is_err()
        );
    }

    #[test]
    fn confidence_interval_complement_is_correct() {
        let confidence =
            MetricConfidence::new(
                0.95,
                0.98,
                0.995,
                ConfidenceMethod::Wilson,
            )
            .expect("valid confidence interval");

        let transformed =
            gate_infidelity_confidence_from_fidelity(
                &confidence,
            )
            .expect("valid transformed interval");

        assert_close(
            transformed.lower.get(),
            0.005,
            1e-12,
        );

        assert_close(
            transformed.upper.get(),
            0.02,
            1e-12,
        );
    }

    #[test]
    fn epc_confidence_transforms_monotonically() {
        let confidence =
            MetricConfidence::new(
                0.95,
                0.01,
                0.02,
                ConfidenceMethod::Wilson,
            )
            .expect("valid confidence interval");

        let transformed =
            error_per_gate_confidence_from_error_per_clifford(
                &confidence,
                5,
            )
            .expect("valid transformed interval");

        assert!(
            transformed.lower.get()
                <= transformed.upper.get()
        );

        assert!(
            transformed.lower.get()
                >= 0.0
        );

        assert!(
            transformed.upper.get()
                <= 1.0
        );
    }

    #[test]
    fn confidence_interval_outside_probability_range_is_rejected() {
        let confidence =
            MetricConfidence::new(
                0.95,
                -0.1,
                0.5,
                ConfidenceMethod::Wilson,
            );

        // The core metric API itself may reject this interval depending on
        // its construction. This test intentionally only asserts that the
        // gate-error boundary never accepts it.
        if let Ok(confidence) = confidence {
            assert!(
                gate_infidelity_confidence_from_fidelity(
                    &confidence
                )
                .is_err()
            );
        }
    }

    #[test]
    fn metric_context_is_preserved() {
        let metric =
            error_per_gate_metric(
                0.001,
                Some(0.0001),
                None,
                Some(100),
                Some(100_000),
                Some(100),
            )
            .expect("valid metric");

        assert_eq!(
            metric.sample_count,
            Some(100)
        );

        assert_eq!(
            metric.shot_count,
            Some(100_000)
        );

        assert_eq!(
            metric.circuit_count,
            Some(100)
        );

        assert_eq!(
            metric.quality,
            MetricQuality::Estimated
        );
    }

    #[test]
    fn rb_conversion_records_model_assumption() {
        let metric =
            error_per_clifford_from_rb_decay(
                0.99,
                2,
            )
            .expect("valid RB result");

        assert!(
            metric.metadata.iter().any(|item| {
                item.key == MODEL_KEY
                    && item.value
                        == "depolarizing_rb_error_per_clifford"
            })
        );

        assert!(
            metric.metadata.iter().any(|item| {
                item.key == ASSUMPTION_KEY
            })
        );
    }

    #[test]
    fn epc_to_epg_records_model_assumption() {
        let metric =
            error_per_gate_from_error_per_clifford(
                0.01,
                5,
            )
            .expect("valid EPC");

        assert!(
            metric.metadata.iter().any(|item| {
                item.key == ASSUMPTION_KEY
            })
        );
    }

    #[test]
    fn no_nan_is_produced_at_valid_boundaries() {
        let inputs = [
            0.0,
            0.1,
            0.5,
            0.9,
            0.999999,
            1.0,
        ];

        for fidelity in inputs {
            let metric =
                gate_infidelity_from_average_gate_fidelity(
                    fidelity
                )
                .expect("valid fidelity");

            assert!(
                metric.value.get().is_finite()
            );
        }
    }

    #[test]
    fn bundle_contains_consistent_metrics() {
        let bundle =
            GateErrorMetrics::from_average_gate_fidelity(
                0.997,
            )
            .expect("valid fidelity");

        assert_close(
            bundle.average_gate_fidelity.value.get()
                + bundle.gate_infidelity.value.get(),
            1.0,
            1e-12,
        );
    }

    #[test]
    fn process_infidelity_from_entanglement_fidelity_matches_complement() {
        let entanglement = 0.98;

        let average =
            average_gate_fidelity_from_entanglement_fidelity(
                entanglement,
                4,
            )
            .expect("valid fidelity");

        let process =
            process_infidelity_from_average_gate_fidelity(
                average.value.get(),
                4,
            )
            .expect("valid fidelity");

        assert_close(
            process.value.get(),
            1.0 - entanglement,
            1e-12,
        );
    }

    #[test]
    fn confidence_transform_preserves_confidence_level() {
        let confidence =
            MetricConfidence::new(
                0.99,
                0.97,
                0.995,
                ConfidenceMethod::ClopperPearson,
            )
            .expect("valid confidence");

        let transformed =
            gate_infidelity_confidence_from_fidelity(
                &confidence,
            )
            .expect("valid transformed confidence");

        assert_close(
            transformed.level.get(),
            0.99,
            1e-12,
        );

        assert_eq!(
            transformed.method,
            ConfidenceMethod::ClopperPearson
        );
    }

    #[test]
    fn custom_rb_decay_metric_is_machine_identifiable() {
        let metric =
            rb_decay_from_error_per_clifford(
                0.02,
                2,
            )
            .expect("valid EPC");

        assert_eq!(
            metric.kind_id(),
            "rb_decay_parameter"
        );
    }

    #[test]
    fn metrics_are_validated_by_the_core_metric_contract() {
        let metric =
            error_per_clifford_metric(
                0.01,
                None,
                None,
                Some(10),
                Some(1_000),
                Some(10),
            )
            .expect("valid EPC metric");

        metric
            .validate()
            .expect("metric must satisfy core validation");
    }

    #[test]
    fn provenance_can_be_attached_after_metric_creation() {
        let provenance =
            ProvenanceRef::new("rb-experiment-001")
                .expect("valid provenance");

        let metric =
            error_per_gate_metric(
                0.001,
                None,
                None,
                Some(10),
                None,
                Some(10),
            )
            .expect("valid metric")
            .with_provenance(provenance);

        assert!(
            metric.provenance.is_some()
        );
    }

    #[test]
    fn direction_is_lower_is_better_for_gate_error() {
        let metric =
            error_per_gate_metric(
                0.001,
                None,
                None,
                None,
                None,
                None,
            )
            .expect("valid metric");

        assert_eq!(
            metric.direction,
            MetricDirection::LowerIsBetter
        );
    }

    #[test]
    fn direction_is_lower_is_better_for_epc() {
        let metric =
            error_per_clifford_metric(
                0.001,
                None,
                None,
                None,
                None,
                None,
            )
            .expect("valid metric");

        assert_eq!(
            metric.direction,
            MetricDirection::LowerIsBetter
        );
    }

    #[test]
    fn rb_dimension_four_uses_correct_depolarizing_factor() {
        let metric =
            error_per_clifford_from_rb_decay(
                0.9,
                4,
            )
            .expect("valid decay");

        let expected =
            (3.0 / 4.0) * 0.1;

        assert_close(
            metric.value.get(),
            expected,
            1e-12,
        );
    }

    #[test]
    fn epg_never_exceeds_epc_for_multiple_gates() {
        let metric =
            error_per_gate_from_error_per_clifford(
                0.1,
                10,
            )
            .expect("valid EPC");

        assert!(
            metric.value.get() <= 0.1
        );
    }
}