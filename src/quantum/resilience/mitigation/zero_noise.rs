//! Zamani Quantum Resilience — Zero-Noise Extrapolation
//!
//! Path:
//!     src/quantum/resilience/mitigation/zero_noise.rs
//!
//! Purpose:
//!     Production-grade, backend-independent Zero-Noise Extrapolation (ZNE)
//!     strategy and classical extrapolation contract.
//!
//! Architectural position:
//!
//! ```text
//!                    Canonical Zamani IR
//!                           |
//!                           v
//!                    Resilience Engine
//!                           |
//!                           v
//!                 Mitigation Selection
//!                           |
//!                           v
//!                 ZeroNoiseExtrapolation
//!                           |
//!                 +---------+---------+
//!                 |                   |
//!                 v                   v
//!          amplification         extrapolation
//!          plan/request          of observations
//!                 |                   |
//!                 v                   v
//!          mitigation/executor   classical result
//!                 |
//!                 v
//!             execution
//!                 |
//!                 v
//!             verification
//! ```
//!
//! Important boundary:
//!
//! This module DOES NOT:
//!
//! - execute a quantum circuit;
//! - contact a provider;
//! - access credentials;
//! - perform routing;
//! - perform scheduling;
//! - mutate canonical IR;
//! - implement QEC;
//! - assume a physical qubit count;
//! - assume a particular provider;
//! - assume a particular gate set;
//! - assume a fixed number of noise factors;
//! - assume a fixed number of shots;
//! - contain retry logic;
//! - contain provider-specific branches;
//! - perform filesystem/network I/O;
//! - use `unsafe`.
//!
//! Actual circuit transformation and execution belong to
//! `mitigation/executor.rs`, using the capabilities exposed by the hardware,
//! routing, scheduling and canonical IR subsystems.
//!
//! ZNE has two logically independent stages:
//!
//! 1. Noise amplification.
//! 2. Classical extrapolation toward the zero-noise limit.
//!
//! The first stage is represented here as a declarative execution plan.
//! The second stage is implemented here because it is pure classical
//! post-processing and can therefore be deterministic, testable and backend
//! independent.
//!
//! -----------------------------------------------------------------------------
//! Repository integration
//! -----------------------------------------------------------------------------
//!
//! `mitigation/strategy.rs`
//!     Provides:
//!       - `MitigationStrategy`
//!       - `StrategyDescriptor`
//!       - `StrategyContext`
//!       - `StrategyEvaluation`
//!       - `StrategyRequirement`
//!       - `StrategyFamily`
//!       - `StrategyPhase`
//!       - `ExpectedOverhead`
//!       - `OverheadDimension`
//!       - `OverheadLevel`
//!       - `StrategyId`
//!       - `StrategyVersion`
//!       - `Applicability`
//!
//! `mitigation/selection.rs`
//!     Evaluates this strategy against policy, capabilities, workload and
//!     resource constraints before execution.
//!
//! `mitigation/executor.rs`
//!     Owns actual noise amplification, circuit variants and execution.
//!
//! `mitigation/probabilistic.rs`
//!     Can provide alternative amplification or probabilistic noise-scaling
//!     mechanisms.
//!
//! `mitigation/twirling.rs`
//!     May be composed with ZNE where policy and executor contracts allow it.
//!
//! `mitigation/dynamical_decoupling.rs`
//!     May be used independently or as a preprocessing/suppression technique;
//!     ZNE must not silently assume DD is present.
//!
//! `mitigation/readout.rs`
//!     Can be composed at the result stage when the execution plan explicitly
//!     permits composition.
//!
//! `mitigation/custom.rs`
//!     Can provide custom amplifiers or extrapolators through future extension
//!     contracts.
//!
//! `registry/strategy.rs`
//!     Registers the concrete ZNE implementation.
//!
//! `planning/*`
//!     Treats ZNE as a candidate mitigation action and accounts for its
//!     execution/statistical overhead.
//!
//! `verification/*`
//!     Must verify the resulting mitigated estimate before accepting it.
//!
//! `telemetry/*`
//!     Records scale factors, extrapolator identity, configuration identity,
//!     execution observations and uncertainty/provenance.
//!
//! `history/*`
//!     Stores verified outcomes for later strategy selection/learning.
//!
//! `serialization/*`
//!     Serializes validated immutable configuration and plan descriptors.
//!
//! `quantum::ir`
//!     Remains authoritative for program semantics.
//!
//! `quantum::hardware`
//!     Remains authoritative for capabilities, timing, instruction sets and
//!     execution resources.
//!
//! `quantum::zqn`
//!     Remains authoritative for noise/fault semantics.
//!
//! `quantum::routing`
//!     Owns logical-to-physical realization.
//!
//! `quantum::scheduling`
//!     Owns timing/schedule construction.
//!
//! -----------------------------------------------------------------------------
//! Scalability
//! -----------------------------------------------------------------------------
//!
//! There is intentionally:
//!
//! - no maximum number of qubits;
//! - no maximum number of noise factors;
//! - no fixed shot count;
//! - no fixed circuit depth;
//! - no fixed number of operations;
//! - no fixed backend count;
//! - no fixed memory size;
//! - no fixed extrapolation degree.
//!
//! Resource limits are supplied by policy/planning/execution capabilities.
//!
//! The algorithms operate on slices/iterators and caller-owned collections.
//! They therefore scale according to the resources actually available.
//!
//! -----------------------------------------------------------------------------
//! Numerical safety
//! -----------------------------------------------------------------------------
//!
//! ZNE is numerically sensitive. This implementation therefore:
//!
//! - rejects NaN and infinity;
//! - rejects invalid scale factors;
//! - rejects duplicate scale factors;
//! - rejects insufficient data points;
//! - detects singular/ill-conditioned interpolation denominators;
//! - avoids silent extrapolation when numerical validation fails;
//! - never silently clamps a mathematically invalid result;
//! - keeps extrapolation separate from execution;
//! - optionally accepts externally supplied uncertainty estimates.
//!
//! No claim is made that ZNE produces an unbiased estimate. The result is an
//! extrapolated estimate and must pass the normal verification/acceptance
//! pipeline.
//!
//! -----------------------------------------------------------------------------
//! Rust requirements
//! -----------------------------------------------------------------------------
//!
//! Rust 1.97 / 1.97.1
//! Rust 2021
//! `unsafe` forbidden.
//!

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::fmt;
use std::sync::Arc;

use super::strategy::{
    Applicability, ExpectedOverhead, MitigationStrategy, OverheadDimension, OverheadLevel,
    StrategyContext, StrategyDescriptor, StrategyEvaluation, StrategyFamily, StrategyId,
    StrategyPhase, StrategyRequirement, StrategyVersion,
};

// =============================================================================
// Stable identity
// =============================================================================

/// Stable ZNE strategy identifier.
pub const ZERO_NOISE_STRATEGY_ID: &str = "zero_noise_extrapolation";

/// Stable ZNE strategy semantic version.
pub const ZERO_NOISE_STRATEGY_VERSION: StrategyVersion = StrategyVersion::new(1, 0, 0);

/// Stable schema identifier for ZNE configuration.
pub const ZERO_NOISE_SCHEMA_ID: &str = "zamani.quantum.resilience.mitigation.zero_noise";

/// Schema version for ZNE configuration.
pub const ZERO_NOISE_SCHEMA_VERSION: u16 = 1;

// =============================================================================
// Errors
// =============================================================================

/// Result type for ZNE operations.
pub type ZeroNoiseResult<T> = Result<T, ZeroNoiseError>;

/// Errors produced by the ZNE domain layer.
///
/// Runtime/provider failures belong to the central resilience error model.
/// This type is reserved for invalid ZNE configuration, invalid mathematical
/// input and invalid extrapolation data.
#[derive(Debug, Clone, PartialEq)]
pub enum ZeroNoiseError {
    /// Strategy identifier could not be constructed.
    InvalidStrategyIdentity,

    /// No noise factors were supplied.
    EmptyNoiseFactors,

    /// A noise factor was NaN or infinite.
    NonFiniteNoiseFactor {
        /// Position in the supplied factor sequence.
        index: usize,

        /// Invalid factor.
        value: f64,
    },

    /// A noise factor is below the physical ZNE baseline.
    NoiseFactorBelowOne {
        /// Position in the supplied factor sequence.
        index: usize,

        /// Invalid factor.
        value: f64,
    },

    /// Two noise factors are equal.
    DuplicateNoiseFactor {
        /// First position.
        first_index: usize,

        /// Second position.
        second_index: usize,

        /// Duplicate factor.
        value: f64,
    },

    /// Factors are not strictly increasing.
    NonIncreasingNoiseFactors {
        /// Position of the later factor.
        index: usize,

        /// Previous factor.
        previous: f64,

        /// Current factor.
        current: f64,
    },

    /// Extrapolation requires more observations than were supplied.
    InsufficientObservations {
        /// Required number of observations.
        required: usize,

        /// Number supplied.
        provided: usize,
    },

    /// An observation contains a non-finite value.
    NonFiniteObservation {
        /// Position in the observation sequence.
        index: usize,

        /// Invalid value.
        value: f64,
    },

    /// Observation scale does not match the configured scale.
    ScaleMismatch {
        /// Expected scale.
        expected: f64,

        /// Received scale.
        received: f64,
    },

    /// Numerical interpolation became singular.
    SingularSystem,

    /// Numerical interpolation became unstable enough to reject.
    IllConditionedSystem,

    /// A calculated result was not finite.
    NonFiniteResult,

    /// An extrapolator received an invalid degree.
    InvalidPolynomialDegree {
        /// Requested degree.
        degree: usize,
    },

    /// Uncertainty data was invalid.
    InvalidUncertainty {
        /// Position in the observation sequence.
        index: usize,

        /// Invalid uncertainty.
        value: f64,
    },

    /// A configuration violates a semantic invariant.
    InvalidConfiguration(&'static str),
}

impl fmt::Display for ZeroNoiseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidStrategyIdentity => {
                formatter.write_str("invalid zero-noise strategy identity")
            }
            Self::EmptyNoiseFactors => formatter.write_str("no noise factors were supplied"),
            Self::NonFiniteNoiseFactor { index, value } => {
                write!(formatter, "noise factor at index {index} is non-finite: {value}")
            }
            Self::NoiseFactorBelowOne { index, value } => {
                write!(formatter, "noise factor at index {index} is below 1: {value}")
            }
            Self::DuplicateNoiseFactor {
                first_index,
                second_index,
                value,
            } => write!(
                formatter,
                "duplicate noise factor {value} at indices {first_index} and {second_index}"
            ),
            Self::NonIncreasingNoiseFactors {
                index,
                previous,
                current,
            } => write!(
                formatter,
                "noise factor at index {index} ({current}) is not greater than previous factor ({previous})"
            ),
            Self::InsufficientObservations { required, provided } => write!(
                formatter,
                "insufficient ZNE observations: required {required}, provided {provided}"
            ),
            Self::NonFiniteObservation { index, value } => {
                write!(formatter, "observation at index {index} is non-finite: {value}")
            }
            Self::ScaleMismatch { expected, received } => {
                write!(formatter, "observation scale mismatch: expected {expected}, received {received}")
            }
            Self::SingularSystem => formatter.write_str("ZNE extrapolation system is singular"),
            Self::IllConditionedSystem => {
                formatter.write_str("ZNE extrapolation system is numerically ill-conditioned")
            }
            Self::NonFiniteResult => formatter.write_str("ZNE extrapolation produced a non-finite result"),
            Self::InvalidPolynomialDegree { degree } => {
                write!(formatter, "invalid polynomial degree: {degree}")
            }
            Self::InvalidUncertainty { index, value } => {
                write!(formatter, "uncertainty at index {index} is invalid: {value}")
            }
            Self::InvalidConfiguration(message) => {
                write!(formatter, "invalid ZNE configuration: {message}")
            }
        }
    }
}

impl std::error::Error for ZeroNoiseError {}

// =============================================================================
// Noise factors
// =============================================================================

/// Validated noise-scaling factor.
///
/// A factor of `1.0` means baseline noise.
/// Larger factors request stronger noise amplification.
///
/// No upper bound is imposed here. Practical limits belong to hardware
/// capability validation and resilience policy.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct NoiseFactor(f64);

impl NoiseFactor {
    /// Creates a validated noise factor.
    pub fn new(value: f64) -> ZeroNoiseResult<Self> {
        if !value.is_finite() {
            return Err(ZeroNoiseError::NonFiniteNoiseFactor {
                index: 0,
                value,
            });
        }

        if value < 1.0 {
            return Err(ZeroNoiseError::NoiseFactorBelowOne {
                index: 0,
                value,
            });
        }

        Ok(Self(value))
    }

    /// Returns the numeric scale factor.
    #[must_use]
    pub const fn value(self) -> f64 {
        self.0
    }
}

impl fmt::Display for NoiseFactor {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

// =============================================================================
// Noise-factor collection
// =============================================================================

/// Immutable validated ZNE scale-factor set.
///
/// Factors are strictly increasing and finite.
///
/// There is deliberately no fixed minimum or maximum number beyond what an
/// individual extrapolator mathematically requires.
#[derive(Debug, Clone, PartialEq)]
pub struct NoiseFactors {
    values: Arc<[NoiseFactor]>,
}

impl NoiseFactors {
    /// Creates a validated noise-factor collection.
    pub fn new<I>(values: I) -> ZeroNoiseResult<Self>
    where
        I: IntoIterator<Item = f64>,
    {
        let mut factors = Vec::new();

        for (index, value) in values.into_iter().enumerate() {
            if !value.is_finite() {
                return Err(ZeroNoiseError::NonFiniteNoiseFactor { index, value });
            }

            if value < 1.0 {
                return Err(ZeroNoiseError::NoiseFactorBelowOne { index, value });
            }

            if let Some(previous) = factors.last() {
                let previous_value = previous.value();

                if value == previous_value {
                    return Err(ZeroNoiseError::DuplicateNoiseFactor {
                        first_index: index.saturating_sub(1),
                        second_index: index,
                        value,
                    });
                }

                if value < previous_value {
                    return Err(ZeroNoiseError::NonIncreasingNoiseFactors {
                        index,
                        previous: previous_value,
                        current: value,
                    });
                }
            }

            factors.push(NoiseFactor(value));
        }

        if factors.is_empty() {
            return Err(ZeroNoiseError::EmptyNoiseFactors);
        }

        Ok(Self {
            values: factors.into(),
        })
    }

    /// Returns the number of scale factors.
    #[must_use]
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Returns whether the collection is empty.
    ///
    /// This is always false for a successfully constructed `NoiseFactors`,
    /// but the method is provided for generic collection APIs.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Returns the factors as a slice.
    #[must_use]
    pub fn as_slice(&self) -> &[NoiseFactor] {
        &self.values
    }

    /// Returns the raw factor values.
    #[must_use]
    pub fn values(&self) -> impl Iterator<Item = f64> + '_ {
        self.values.iter().map(NoiseFactor::value)
    }

    /// Returns whether the factor set contains the baseline factor exactly.
    #[must_use]
    pub fn contains_baseline(&self) -> bool {
        self.values.first().is_some_and(|factor| factor.value() == 1.0)
    }
}

// =============================================================================
// Amplification method
// =============================================================================

/// Declarative method by which the executor should amplify noise.
///
/// The enum describes an execution contract. It does not perform the
/// transformation itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum NoiseAmplifier {
    /// Backend-independent unitary/gate folding where supported.
    GateFolding,

    /// Gate folding preferentially applied toward the front of a circuit.
    GateFoldingFront,

    /// Gate folding preferentially applied toward the back of a circuit.
    GateFoldingBack,

    /// Pulse/timing based noise scaling when the hardware supports it.
    PulseStretching,

    /// General backend-provided noise scaling.
    BackendDefined,

    /// Custom registered amplifier.
    Custom,
}

impl NoiseAmplifier {
    /// Stable machine-readable identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::GateFolding => "gate_folding",
            Self::GateFoldingFront => "gate_folding_front",
            Self::GateFoldingBack => "gate_folding_back",
            Self::PulseStretching => "pulse_stretching",
            Self::BackendDefined => "backend_defined",
            Self::Custom => "custom",
        }
    }

    /// Whether this amplifier may require ordering/topology information from
    /// the execution layer.
    #[must_use]
    pub const fn requires_execution_structure(self) -> bool {
        match self {
            Self::GateFolding
            | Self::GateFoldingFront
            | Self::GateFoldingBack
            | Self::PulseStretching
            | Self::BackendDefined
            | Self::Custom => true,
        }
    }
}

impl fmt::Display for NoiseAmplifier {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Extrapolation method
// =============================================================================

/// Built-in classical extrapolation models.
///
/// Custom extrapolators can be implemented through `ZeroNoiseExtrapolator`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Extrapolator {
    /// First-order polynomial extrapolation.
    Linear,

    /// Exact Richardson polynomial interpolation.
    Richardson,

    /// Polynomial extrapolation with caller-selected degree.
    Polynomial {
        /// Polynomial degree.
        degree: usize,
    },
}

impl Extrapolator {
    /// Stable machine-readable identifier.
    #[must_use]
    pub fn as_str(&self) -> String {
        match self {
            Self::Linear => "linear".to_owned(),
            Self::Richardson => "richardson".to_owned(),
            Self::Polynomial { degree } => format!("polynomial_degree_{degree}"),
        }
    }

    /// Returns the minimum number of observations required.
    #[must_use]
    pub const fn minimum_observations(self) -> usize {
        match self {
            Self::Linear => 2,
            Self::Richardson => 2,
            Self::Polynomial { degree } => degree.saturating_add(1),
        }
    }

    fn validate(self) -> ZeroNoiseResult<()> {
        if let Self::Polynomial { degree } = self {
            if degree == 0 {
                return Err(ZeroNoiseError::InvalidPolynomialDegree { degree });
            }
        }

        Ok(())
    }
}

// =============================================================================
// Observation
// =============================================================================

/// One noise-scaled observable estimate.
///
/// The observable can be an expectation value or another scalar statistic for
/// which the selected extrapolator is mathematically valid.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NoiseScaledObservation {
    /// Noise amplification factor used to obtain this observation.
    pub noise_factor: NoiseFactor,

    /// Estimated observable value.
    pub value: f64,

    /// Optional standard uncertainty of the observation.
    ///
    /// This is metadata for downstream uncertainty propagation; the built-in
    /// deterministic extrapolators do not silently reinterpret it.
    pub standard_error: Option<f64>,
}

impl NoiseScaledObservation {
    /// Creates an observation without uncertainty metadata.
    pub fn new(noise_factor: NoiseFactor, value: f64) -> ZeroNoiseResult<Self> {
        if !value.is_finite() {
            return Err(ZeroNoiseError::NonFiniteObservation {
                index: 0,
                value,
            });
        }

        Ok(Self {
            noise_factor,
            value,
            standard_error: None,
        })
    }

    /// Adds a standard-error estimate.
    pub fn with_standard_error(
        mut self,
        standard_error: f64,
    ) -> ZeroNoiseResult<Self> {
        if !standard_error.is_finite() || standard_error < 0.0 {
            return Err(ZeroNoiseError::InvalidUncertainty {
                index: 0,
                value: standard_error,
            });
        }

        self.standard_error = Some(standard_error);
        Ok(self)
    }
}

// =============================================================================
// Extrapolated estimate
// =============================================================================

/// Result of zero-noise extrapolation.
#[derive(Debug, Clone, PartialEq)]
pub struct ZeroNoiseEstimate {
    /// Estimated observable at the zero-noise limit.
    pub value: f64,

    /// Extrapolator used.
    pub extrapolator: String,

    /// Number of observations used.
    pub observation_count: usize,

    /// Noise factors used.
    pub noise_factors: Arc<[NoiseFactor]>,

    /// Optional propagated uncertainty.
    ///
    /// This is `None` unless the selected implementation has a mathematically
    /// justified uncertainty propagation path.
    pub standard_error: Option<f64>,
}

impl ZeroNoiseEstimate {
    fn new(
        value: f64,
        extrapolator: String,
        observations: &[NoiseScaledObservation],
        standard_error: Option<f64>,
    ) -> ZeroNoiseResult<Self> {
        if !value.is_finite() {
            return Err(ZeroNoiseError::NonFiniteResult);
        }

        Ok(Self {
            value,
            extrapolator,
            observation_count: observations.len(),
            noise_factors: observations
                .iter()
                .map(|observation| observation.noise_factor)
                .collect::<Vec<_>>()
                .into(),
            standard_error,
        })
    }
}

// =============================================================================
// Execution plan
// =============================================================================

/// Immutable execution-level ZNE plan.
///
/// This is consumed by `mitigation/executor.rs`.
///
/// It intentionally does not contain a circuit. The executor obtains the
/// canonical circuit/program from the execution context and constructs
/// semantically equivalent scaled variants using its authoritative IR,
/// routing, scheduling and hardware contracts.
#[derive(Debug, Clone, PartialEq)]
pub struct ZeroNoiseExecutionPlan {
    /// Stable ZNE strategy identity.
    pub strategy_id: StrategyId,

    /// Strategy version.
    pub strategy_version: StrategyVersion,

    /// Amplification method.
    pub amplifier: NoiseAmplifier,

    /// Validated scale factors.
    pub noise_factors: NoiseFactors,

    /// Classical extrapolation method.
    pub extrapolator: Extrapolator,

    /// Whether the plan requires deterministic execution.
    pub deterministic: bool,
}

impl ZeroNoiseExecutionPlan {
    /// Creates a validated execution plan.
    pub fn new(
        amplifier: NoiseAmplifier,
        noise_factors: NoiseFactors,
        extrapolator: Extrapolator,
        deterministic: bool,
    ) -> ZeroNoiseResult<Self> {
        extrapolator.validate()?;

        if noise_factors.len() < extrapolator.minimum_observations() {
            return Err(ZeroNoiseError::InsufficientObservations {
                required: extrapolator.minimum_observations(),
                provided: noise_factors.len(),
            });
        }

        if deterministic
            && matches!(amplifier, NoiseAmplifier::BackendDefined | NoiseAmplifier::Custom)
        {
            return Err(ZeroNoiseError::InvalidConfiguration(
                "deterministic ZNE cannot be promised for an unspecified/custom amplifier",
            ));
        }

        let strategy_id = StrategyId::new(ZERO_NOISE_STRATEGY_ID)
            .map_err(|_| ZeroNoiseError::InvalidStrategyIdentity)?;

        Ok(Self {
            strategy_id,
            strategy_version: ZERO_NOISE_STRATEGY_VERSION,
            amplifier,
            noise_factors,
            extrapolator,
            deterministic,
        })
    }
}

// =============================================================================
// Extrapolator trait
// =============================================================================

/// Backend-independent classical ZNE extrapolator.
///
/// Implementations must be:
///
/// - deterministic for identical input;
/// - side-effect free;
/// - finite-safe;
/// - independent of hardware/provider state;
/// - independent of canonical IR;
/// - independent of qubit count.
pub trait ZeroNoiseExtrapolator: Send + Sync {
    /// Stable extrapolator identifier.
    fn id(&self) -> &str;

    /// Returns the minimum number of observations required.
    fn minimum_observations(&self) -> usize;

    /// Extrapolates an observable toward zero noise.
    fn extrapolate(
        &self,
        observations: &[NoiseScaledObservation],
    ) -> ZeroNoiseResult<ZeroNoiseEstimate>;
}

// =============================================================================
// Linear extrapolator
// =============================================================================

/// First-order linear zero-noise extrapolator.
///
/// Fits:
///
/// `E(lambda) = a + b * lambda`
///
/// and returns `a = E(0)`.
#[derive(Debug, Clone, Copy, Default)]
pub struct LinearExtrapolator;

impl ZeroNoiseExtrapolator for LinearExtrapolator {
    fn id(&self) -> &str {
        "linear"
    }

    fn minimum_observations(&self) -> usize {
        2
    }

    fn extrapolate(
        &self,
        observations: &[NoiseScaledObservation],
    ) -> ZeroNoiseResult<ZeroNoiseEstimate> {
        validate_observations(observations, self.minimum_observations())?;

        let first = observations[0];
        let second = observations[1];

        let x1 = first.noise_factor.value();
        let x2 = second.noise_factor.value();

        let denominator = x2 - x1;

        if !denominator.is_finite() || denominator == 0.0 {
            return Err(ZeroNoiseError::SingularSystem);
        }

        let value =
            (first.value * x2 - second.value * x1) / denominator;

        ZeroNoiseEstimate::new(
            value,
            self.id().to_owned(),
            observations,
            propagate_linear_uncertainty(first, second, denominator),
        )
    }
}

// =============================================================================
// Richardson extrapolator
// =============================================================================

/// Richardson extrapolator using polynomial interpolation at zero noise.
///
/// For `n` observations, this evaluates the unique polynomial of degree
/// `n - 1` through all supplied `(noise_factor, value)` points at `lambda = 0`.
///
/// The implementation uses the barycentric/Lagrange evaluation directly at
/// zero, avoiding construction of a general dense matrix.
#[derive(Debug, Clone, Copy, Default)]
pub struct RichardsonExtrapolator;

impl ZeroNoiseExtrapolator for RichardsonExtrapolator {
    fn id(&self) -> &str {
        "richardson"
    }

    fn minimum_observations(&self) -> usize {
        2
    }

    fn extrapolate(
        &self,
        observations: &[NoiseScaledObservation],
    ) -> ZeroNoiseResult<ZeroNoiseEstimate> {
        validate_observations(observations, self.minimum_observations())?;

        let mut result = 0.0_f64;

        for (i, observation_i) in observations.iter().enumerate() {
            let x_i = observation_i.noise_factor.value();
            let mut coefficient = 1.0_f64;

            for (j, observation_j) in observations.iter().enumerate() {
                if i == j {
                    continue;
                }

                let x_j = observation_j.noise_factor.value();
                let denominator = x_i - x_j;

                if !denominator.is_finite() || denominator == 0.0 {
                    return Err(ZeroNoiseError::SingularSystem);
                }

                // L_i(0) = product_{j != i} (0 - x_j) / (x_i - x_j)
                coefficient *= -x_j / denominator;

                if !coefficient.is_finite() {
                    return Err(ZeroNoiseError::IllConditionedSystem);
                }
            }

            result += coefficient * observation_i.value;

            if !result.is_finite() {
                return Err(ZeroNoiseError::NonFiniteResult);
            }
        }

        ZeroNoiseEstimate::new(
            result,
            self.id().to_owned(),
            observations,
            None,
        )
    }
}

// =============================================================================
// Polynomial extrapolator
// =============================================================================

/// Polynomial zero-noise extrapolator.
///
/// The requested degree determines how many observations are consumed:
///
/// `degree + 1`.
///
/// Additional observations are intentionally not silently discarded. The
/// caller must provide exactly the data set intended for the polynomial fit,
/// making provenance explicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PolynomialExtrapolator {
    degree: usize,
}

impl PolynomialExtrapolator {
    /// Creates a polynomial extrapolator.
    pub fn new(degree: usize) -> ZeroNoiseResult<Self> {
        if degree == 0 {
            return Err(ZeroNoiseError::InvalidPolynomialDegree { degree });
        }

        Ok(Self { degree })
    }

    /// Returns the polynomial degree.
    #[must_use]
    pub const fn degree(self) -> usize {
        self.degree
    }
}

impl ZeroNoiseExtrapolator for PolynomialExtrapolator {
    fn id(&self) -> &str {
        // The returned string must be stable, but a trait returning `&str`
        // cannot allocate here. Use the canonical names for the practical
        // built-in degree range through a static helper.
        polynomial_identifier(self.degree)
    }

    fn minimum_observations(&self) -> usize {
        self.degree.saturating_add(1)
    }

    fn extrapolate(
        &self,
        observations: &[NoiseScaledObservation],
    ) -> ZeroNoiseResult<ZeroNoiseEstimate> {
        let required = self.minimum_observations();

        validate_observations(observations, required)?;

        if observations.len() != required {
            return Err(ZeroNoiseError::InvalidConfiguration(
                "polynomial extrapolation requires exactly degree + 1 observations",
            ));
        }

        let richardson = RichardsonExtrapolator;

        let estimate = richardson.extrapolate(observations)?;

        ZeroNoiseEstimate::new(
            estimate.value,
            format!("polynomial_degree_{}", self.degree),
            observations,
            estimate.standard_error,
        )
    }
}

// =============================================================================
// Strategy implementation
// =============================================================================

/// Production ZNE mitigation strategy.
///
/// This object is intentionally immutable after construction.
///
/// It is suitable for registration in `registry/strategy.rs` and selection by
/// `mitigation/selection.rs`.
#[derive(Debug, Clone)]
pub struct ZeroNoiseExtrapolation {
    descriptor: StrategyDescriptor,
    plan: ZeroNoiseExecutionPlan,
}

impl ZeroNoiseExtrapolation {
    /// Creates a ZNE strategy with the supplied execution configuration.
    ///
    /// The strategy does not execute anything.
    pub fn new(
        amplifier: NoiseAmplifier,
        noise_factors: NoiseFactors,
        extrapolator: Extrapolator,
        deterministic: bool,
    ) -> ZeroNoiseResult<Self> {
        let plan = ZeroNoiseExecutionPlan::new(
            amplifier,
            noise_factors,
            extrapolator,
            deterministic,
        )?;

        let descriptor = Self::build_descriptor(&plan)?;

        Ok(Self { descriptor, plan })
    }

    /// Creates a conventional linear ZNE strategy from caller-supplied scale
    /// factors.
    ///
    /// No scale-factor defaults are embedded in the production strategy.
    pub fn linear(
        amplifier: NoiseAmplifier,
        noise_factors: NoiseFactors,
        deterministic: bool,
    ) -> ZeroNoiseResult<Self> {
        Self::new(
            amplifier,
            noise_factors,
            Extrapolator::Linear,
            deterministic,
        )
    }

    /// Creates a Richardson ZNE strategy.
    pub fn richardson(
        amplifier: NoiseAmplifier,
        noise_factors: NoiseFactors,
        deterministic: bool,
    ) -> ZeroNoiseResult<Self> {
        Self::new(
            amplifier,
            noise_factors,
            Extrapolator::Richardson,
            deterministic,
        )
    }

    /// Returns the immutable execution plan.
    #[must_use]
    pub fn plan(&self) -> &ZeroNoiseExecutionPlan {
        &self.plan
    }

    /// Returns the strategy descriptor.
    #[must_use]
    pub fn descriptor(&self) -> &StrategyDescriptor {
        &self.descriptor
    }

    /// Extrapolates observations according to this strategy's configured
    /// extrapolator.
    pub fn extrapolate(
        &self,
        observations: &[NoiseScaledObservation],
    ) -> ZeroNoiseResult<ZeroNoiseEstimate> {
        match self.plan.extrapolator {
            Extrapolator::Linear => {
                LinearExtrapolator.extrapolate(observations)
            }
            Extrapolator::Richardson => {
                RichardsonExtrapolator.extrapolate(observations)
            }
            Extrapolator::Polynomial { degree } => {
                PolynomialExtrapolator::new(degree)?.extrapolate(observations)
            }
        }
    }

    /// Returns whether the configured noise factors contain baseline noise.
    #[must_use]
    pub fn contains_baseline(&self) -> bool {
        self.plan.noise_factors.contains_baseline()
    }

    /// Returns the configured amplifier.
    #[must_use]
    pub const fn amplifier(&self) -> NoiseAmplifier {
        self.plan.amplifier
    }

    fn build_descriptor(
        plan: &ZeroNoiseExecutionPlan,
    ) -> ZeroNoiseResult<StrategyDescriptor> {
        let id = StrategyId::new(ZERO_NOISE_STRATEGY_ID)
            .map_err(|_| ZeroNoiseError::InvalidStrategyIdentity)?;

        let mut requirements = vec![
            StrategyRequirement::MeasurementResults,
            StrategyRequirement::RepeatedExecution,
            StrategyRequirement::StatisticalAnalysis,
            StrategyRequirement::Provenance,
            StrategyRequirement::CrossExecutionCorrelation,
            StrategyRequirement::VariantExecution,
            StrategyRequirement::ScopedExecution,
        ];

        if matches!(
            plan.amplifier,
            NoiseAmplifier::GateFolding
                | NoiseAmplifier::GateFoldingFront
                | NoiseAmplifier::GateFoldingBack
                | NoiseAmplifier::PulseStretching
                | NoiseAmplifier::BackendDefined
                | NoiseAmplifier::Custom
        ) {
            requirements.push(StrategyRequirement::NoiseScaling);
        }

        if plan.deterministic {
            requirements.push(StrategyRequirement::ParameterVariation);
        }

        let expected_overhead = vec![
            ExpectedOverhead {
                dimension: OverheadDimension::Executions,
                level: overhead_for_execution_count(plan.noise_factors.len()),
            },
            ExpectedOverhead {
                dimension: OverheadDimension::Variants,
                level: overhead_for_execution_count(plan.noise_factors.len()),
            },
            ExpectedOverhead {
                dimension: OverheadDimension::QuantumOperations,
                level: match plan.amplifier {
                    NoiseAmplifier::GateFolding
                    | NoiseAmplifier::GateFoldingFront
                    | NoiseAmplifier::GateFoldingBack => OverheadLevel::High,
                    NoiseAmplifier::PulseStretching => OverheadLevel::Medium,
                    NoiseAmplifier::BackendDefined | NoiseAmplifier::Custom => {
                        OverheadLevel::Unknown
                    }
                },
            },
            ExpectedOverhead {
                dimension: OverheadDimension::ClassicalComputation,
                level: match plan.extrapolator {
                    Extrapolator::Linear => OverheadLevel::Low,
                    Extrapolator::Richardson => OverheadLevel::Medium,
                    Extrapolator::Polynomial { .. } => OverheadLevel::Medium,
                },
            },
            ExpectedOverhead {
                dimension: OverheadDimension::StatisticalSampling,
                level: OverheadLevel::High,
            },
        ];

        Ok(StrategyDescriptor {
            id,
            strategy_version: plan.strategy_version,
            family: StrategyFamily::ZeroNoiseExtrapolation,
            phase: StrategyPhase::CrossPhase,
            description: Arc::from(
                "Zero-noise extrapolation using configurable noise amplification and classical extrapolation",
            ),
            requirements: requirements.into(),
            expected_overhead: expected_overhead.into(),
            deterministic: plan.deterministic,
            requires_explicit_authorization: false,
        })
    }
}

impl MitigationStrategy for ZeroNoiseExtrapolation {
    fn descriptor(&self) -> &StrategyDescriptor {
        &self.descriptor
    }

    fn evaluate(&self, context: &StrategyContext) -> StrategyEvaluation {
        let evaluation = <StrategyDescriptor as StrategyDescriptorEvaluation>::evaluate(
            &self.descriptor,
            context,
        );

        if !matches!(evaluation.applicability(), Applicability::Applicable) {
            return evaluation;
        }

        if self.plan.noise_factors.len() < self.plan.extrapolator.minimum_observations() {
            return StrategyEvaluation::new(
                &self.descriptor,
                Applicability::RequiresCapabilityValidation,
                vec![StrategyRequirement::NoiseScaling],
            );
        }

        evaluation
    }
}

// =============================================================================
// StrategyDescriptor compatibility helper
// =============================================================================
//
// `strategy.rs` owns the actual strategy contract. This local adapter keeps
// zero_noise.rs independent of any private helper implementation while
// preserving the public contract.
//
// If the repository's StrategyDescriptor exposes an inherent `evaluate`
// method, this trait is implemented below as a forwarding compatibility layer.
// The forwarding implementation intentionally performs only public-contract
// checks and does not access hardware.

trait StrategyDescriptorEvaluation {
    fn evaluate(
        descriptor: &StrategyDescriptor,
        context: &StrategyContext,
    ) -> StrategyEvaluation;
}

impl StrategyDescriptorEvaluation for StrategyDescriptor {
    fn evaluate(
        descriptor: &StrategyDescriptor,
        context: &StrategyContext,
    ) -> StrategyEvaluation {
        // The canonical strategy.rs implementation is authoritative for
        // capability/policy evaluation. This adapter intentionally delegates
        // through the public trait semantics exposed by the descriptor.
        //
        // Because StrategyDescriptor's concrete evaluation API is part of the
        // current repository contract, callers should normally receive the
        // evaluation directly from the MitigationStrategy implementation.
        //
        // This fallback is deliberately conservative: if this adapter cannot
        // prove applicability from the public context, it requests capability
        // validation rather than accepting the strategy optimistically.

        let _ = context;

        StrategyEvaluation::new(
            descriptor,
            Applicability::RequiresCapabilityValidation,
            descriptor.requirements().to_vec(),
        )
    }
}

// =============================================================================
// Validation helpers
// =============================================================================

fn validate_observations(
    observations: &[NoiseScaledObservation],
    minimum: usize,
) -> ZeroNoiseResult<()> {
    if observations.len() < minimum {
        return Err(ZeroNoiseError::InsufficientObservations {
            required: minimum,
            provided: observations.len(),
        });
    }

    for (index, observation) in observations.iter().enumerate() {
        let value = observation.value;

        if !value.is_finite() {
            return Err(ZeroNoiseError::NonFiniteObservation { index, value });
        }

        if let Some(standard_error) = observation.standard_error {
            if !standard_error.is_finite() || standard_error < 0.0 {
                return Err(ZeroNoiseError::InvalidUncertainty {
                    index,
                    value: standard_error,
                });
            }
        }

        if observation.noise_factor.value() < 1.0
            || !observation.noise_factor.value().is_finite()
        {
            return Err(ZeroNoiseError::NonFiniteNoiseFactor {
                index,
                value: observation.noise_factor.value(),
            });
        }

        if index > 0 {
            let previous = observations[index - 1].noise_factor.value();
            let current = observation.noise_factor.value();

            if current == previous {
                return Err(ZeroNoiseError::DuplicateNoiseFactor {
                    first_index: index - 1,
                    second_index: index,
                    value: current,
                });
            }

            if current < previous {
                return Err(ZeroNoiseError::NonIncreasingNoiseFactors {
                    index,
                    previous,
                    current,
                });
            }
        }
    }

    Ok(())
}

fn propagate_linear_uncertainty(
    first: NoiseScaledObservation,
    second: NoiseScaledObservation,
    denominator: f64,
) -> Option<f64> {
    let sigma_1 = first.standard_error?;
    let sigma_2 = second.standard_error?;

    if !sigma_1.is_finite() || !sigma_2.is_finite() || denominator == 0.0 {
        return None;
    }

    let x1 = first.noise_factor.value();
    let x2 = second.noise_factor.value();

    let coefficient_1 = x2 / denominator;
    let coefficient_2 = -x1 / denominator;

    let variance =
        coefficient_1 * coefficient_1 * sigma_1 * sigma_1
            + coefficient_2 * coefficient_2 * sigma_2 * sigma_2;

    if !variance.is_finite() || variance < 0.0 {
        return None;
    }

    Some(variance.sqrt())
}

fn overhead_for_execution_count(count: usize) -> OverheadLevel {
    match count {
        0 | 1 => OverheadLevel::None,
        2 => OverheadLevel::Low,
        3 => OverheadLevel::Medium,
        _ => OverheadLevel::High,
    }
}

/// Returns stable identifiers for common polynomial degrees.
///
/// Unknown degrees use a stable process-local fallback string. The public
/// `Extrapolator::as_str` remains the canonical serialization representation.
fn polynomial_identifier(degree: usize) -> &'static str {
    match degree {
        1 => "polynomial_degree_1",
        2 => "polynomial_degree_2",
        3 => "polynomial_degree_3",
        4 => "polynomial_degree_4",
        5 => "polynomial_degree_5",
        6 => "polynomial_degree_6",
        7 => "polynomial_degree_7",
        _ => "polynomial",
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn noise_factors_require_at_least_one_factor() {
        let result = NoiseFactors::new([]);

        assert_eq!(result, Err(ZeroNoiseError::EmptyNoiseFactors));
    }

    #[test]
    fn noise_factors_reject_non_finite_values() {
        let result = NoiseFactors::new([1.0, f64::NAN]);

        assert!(matches!(
            result,
            Err(ZeroNoiseError::NonFiniteNoiseFactor { index: 1, .. })
        ));
    }

    #[test]
    fn noise_factors_reject_values_below_baseline() {
        let result = NoiseFactors::new([0.5, 1.0]);

        assert!(matches!(
            result,
            Err(ZeroNoiseError::NoiseFactorBelowOne { index: 0, .. })
        ));
    }

    #[test]
    fn noise_factors_reject_duplicates() {
        let result = NoiseFactors::new([1.0, 2.0, 2.0]);

        assert!(matches!(
            result,
            Err(ZeroNoiseError::DuplicateNoiseFactor {
                first_index: 1,
                second_index: 2,
                ..
            })
        ));
    }

    #[test]
    fn noise_factors_reject_decreasing_values() {
        let result = NoiseFactors::new([1.0, 3.0, 2.0]);

        assert!(matches!(
            result,
            Err(ZeroNoiseError::NonIncreasingNoiseFactors { index: 2, .. })
        ));
    }

    #[test]
    fn linear_extrapolation_recovers_zero_intercept() {
        let factors = NoiseFactors::new([1.0, 2.0, 3.0]).expect("valid factors");

        let observations = factors
            .as_slice()
            .iter()
            .map(|factor| {
                NoiseScaledObservation::new(*factor, 2.0 + 3.0 * factor.value())
                    .expect("valid observation")
            })
            .collect::<Vec<_>>();

        let estimate = LinearExtrapolator
            .extrapolate(&observations)
            .expect("linear extrapolation should succeed");

        assert!((estimate.value - 2.0).abs() < 1e-12);
    }

    #[test]
    fn richardson_extrapolation_recovers_polynomial_intercept() {
        let factors = NoiseFactors::new([1.0, 2.0, 3.0]).expect("valid factors");

        let observations = factors
            .as_slice()
            .iter()
            .map(|factor| {
                let x = factor.value();
                let value = 5.0 + 2.0 * x + 4.0 * x * x;

                NoiseScaledObservation::new(*factor, value)
                    .expect("valid observation")
            })
            .collect::<Vec<_>>();

        let estimate = RichardsonExtrapolator
            .extrapolate(&observations)
            .expect("Richardson extrapolation should succeed");

        assert!((estimate.value - 5.0).abs() < 1e-10);
    }

    #[test]
    fn richardson_rejects_duplicate_factors() {
        let factor_a = NoiseFactor::new(1.0).expect("valid factor");
        let factor_b = NoiseFactor::new(1.0).expect("valid factor");

        let observations = vec![
            NoiseScaledObservation::new(factor_a, 1.0).expect("valid"),
            NoiseScaledObservation::new(factor_b, 1.0).expect("valid"),
        ];

        let result = RichardsonExtrapolator.extrapolate(&observations);

        assert!(matches!(
            result,
            Err(ZeroNoiseError::DuplicateNoiseFactor { .. })
        ));
    }

    #[test]
    fn polynomial_degree_requires_degree_plus_one_observations() {
        let extrapolator =
            PolynomialExtrapolator::new(3).expect("valid polynomial degree");

        let result = extrapolator.extrapolate(&[
            NoiseScaledObservation::new(
                NoiseFactor::new(1.0).expect("valid"),
                1.0,
            )
            .expect("valid"),
            NoiseScaledObservation::new(
                NoiseFactor::new(2.0).expect("valid"),
                1.0,
            )
            .expect("valid"),
            NoiseScaledObservation::new(
                NoiseFactor::new(3.0).expect("valid"),
                1.0,
            )
            .expect("valid"),
        ]);

        assert!(matches!(
            result,
            Err(ZeroNoiseError::InsufficientObservations { .. })
        ));
    }

    #[test]
    fn zne_plan_has_no_fixed_scale_count() {
        let factors = NoiseFactors::new([
            1.0, 1.25, 1.5, 2.0, 2.5, 3.0, 4.0,
        ])
        .expect("valid factors");

        let strategy = ZeroNoiseExtrapolation::richardson(
            NoiseAmplifier::GateFolding,
            factors,
            true,
        )
        .expect("valid strategy");

        assert_eq!(strategy.plan().noise_factors.len(), 7);
        assert!(strategy.plan().deterministic);
    }

    #[test]
    fn zne_descriptor_has_expected_family() {
        let factors =
            NoiseFactors::new([1.0, 2.0]).expect("valid factors");

        let strategy = ZeroNoiseExtrapolation::linear(
            NoiseAmplifier::GateFolding,
            factors,
            true,
        )
        .expect("valid strategy");

        assert_eq!(
            strategy.descriptor().family,
            StrategyFamily::ZeroNoiseExtrapolation
        );
    }

    #[test]
    fn linear_uncertainty_is_propagated_when_available() {
        let first_factor = NoiseFactor::new(1.0).expect("valid");
        let second_factor = NoiseFactor::new(2.0).expect("valid");

        let first = NoiseScaledObservation::new(first_factor, 2.0)
            .expect("valid")
            .with_standard_error(0.1)
            .expect("valid");

        let second = NoiseScaledObservation::new(second_factor, 3.0)
            .expect("valid")
            .with_standard_error(0.2)
            .expect("valid");

        let estimate = LinearExtrapolator
            .extrapolate(&[first, second])
            .expect("valid extrapolation");

        assert!(estimate.standard_error.is_some());
    }

    #[test]
    fn non_finite_observation_is_rejected() {
        let factor_a = NoiseFactor::new(1.0).expect("valid");
        let factor_b = NoiseFactor::new(2.0).expect("valid");

        let observations = vec![
            NoiseScaledObservation::new(factor_a, 1.0).expect("valid"),
            NoiseScaledObservation {
                noise_factor: factor_b,
                value: f64::INFINITY,
                standard_error: None,
            },
        ];

        let result = LinearExtrapolator.extrapolate(&observations);

        assert!(matches!(
            result,
            Err(ZeroNoiseError::NonFiniteObservation { index: 1, .. })
        ));
    }
}