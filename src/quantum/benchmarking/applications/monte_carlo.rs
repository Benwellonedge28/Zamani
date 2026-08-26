//! Zamani Quantum Benchmarking — Monte Carlo Application Benchmark
//!
//! Production application-benchmark definition for quantum Monte Carlo
//! estimation using amplitude-estimation-style quantum workloads.
//!
//! # Architectural responsibility
//!
//! This module owns:
//!
//! - Monte Carlo benchmark identity;
//! - benchmark semantic versioning;
//! - Monte Carlo problem configuration;
//! - distribution/payoff identifiers;
//! - target precision;
//! - reference-value semantics;
//! - success criteria;
//! - benchmark metric declarations;
//! - deterministic workload construction;
//! - application workload metadata;
//! - resource estimates intrinsic to the benchmark definition;
//! - validation of all Monte Carlo-specific inputs;
//! - reproducibility metadata represented through the common application
//!   generation contract.
//!
//! This module deliberately does NOT:
//!
//! - execute quantum circuits;
//! - select a backend;
//! - select a provider;
//! - communicate with hardware;
//! - perform routing;
//! - perform scheduling;
//! - perform calibration;
//! - transpile;
//! - implement Quantum IR;
//! - duplicate amplitude-estimation algorithms;
//! - implement an amplitude oracle;
//! - implement state preparation;
//! - estimate the final Monte Carlo value from raw observations;
//! - calculate confidence intervals from execution data;
//! - calculate runtime metrics;
//! - perform fidelity calculations;
//! - perform statistical regression;
//! - parse Zamani source code;
//! - perform filesystem I/O;
//! - perform network I/O;
//! - use hidden randomness.
//!
//! Those responsibilities belong to the corresponding algorithm, IR,
//! execution, hardware, statistics, metrics, reporting, and frontend layers.
//!
//! # Architectural position
//!
//! ```text
//! Zamani benchmark declaration
//!             │
//!             ▼
//! MonteCarloBenchmarkConfig
//!             │
//!             ▼
//! MonteCarloBenchmarkGenerator
//!             │
//!             ▼
//! ApplicationGenerationRequest
//!             │
//!             ▼
//! ApplicationWorkload
//!             │
//!             ▼
//! algorithm/execution adapter
//!             │
//!             ▼
//! amplitude-estimation / quantum execution
//!             │
//!             ▼
//! BenchmarkObservation
//!             │
//!             ▼
//! Monte Carlo analysis
//!             │
//!             ▼
//! BenchmarkResult
//! ```
//!
//! # Why this file does not duplicate the amplitude-estimation circuit
//!
//! Zamani already has an amplitude-estimation algorithm boundary under the
//! quantum algorithms subsystem. That algorithm layer defines the
//! backend-independent amplitude-estimation problem and execution contract.
//!
//! The benchmark layer must describe *what is being measured*, not create a
//! second implementation of amplitude estimation.
//!
//! Consequently this file produces an `ApplicationWorkload` containing the
//! complete Monte Carlo semantic configuration. A later algorithm/execution
//! adapter may translate that workload into the canonical Quantum IR.
//!
//! This follows the existing repository boundary:
//!
//! ```text
//! application benchmark
//!       │
//!       ▼
//! ApplicationWorkload
//!       │
//!       ▼
//! quantum::algorithms::amplitude
//!       │
//!       ▼
//! Quantum IR / execution
//! ```
//!
//! This also prevents the benchmark from becoming coupled to one particular
//! amplitude-estimation implementation. Future implementations such as:
//!
//! - canonical QAE;
//! - iterative amplitude estimation;
//! - maximum-likelihood amplitude estimation;
//! - maximum-confidence estimation;
//! - adaptive amplitude estimation;
//! - fault-tolerant amplitude estimation;
//! - hardware-native amplitude estimation;
//!
//! can benchmark the same Monte Carlo problem.
//!
//! # Mathematical problem
//!
//! Quantum Monte Carlo estimates an expectation:
//!
//! ```text
//! μ = E[f(X)]
//! ```
//!
//! where:
//!
//! - `X` is sampled from a configured probability distribution;
//! - `f(X)` is the configured payoff/integrand;
//! - `μ` is the reference quantity being estimated.
//!
//! For quantum amplitude estimation, the bounded payoff is represented as an
//! amplitude:
//!
//! ```text
//! a = E[g(X)]
//! ```
//!
//! where:
//!
//! ```text
//! 0 <= g(X) <= 1
//! ```
//!
//! and the physical/application result may be reconstructed through an
//! explicitly recorded affine scaling:
//!
//! ```text
//! f(X) = offset + scale * g(X)
//! ```
//!
//! Therefore:
//!
//! ```text
//! μ = offset + scale * a
//! ```
//!
//! The benchmark never silently assumes that the application value is itself
//! a probability. The amplitude-space quantity and application-space quantity
//! are recorded separately.
//!
//! # Benchmark philosophy
//!
//! A Monte Carlo benchmark must measure more than a single estimated value.
//!
//! The surrounding benchmarking system should eventually collect:
//!
//! - absolute estimation error;
//! - relative estimation error;
//! - success/failure against the requested precision;
//! - confidence interval;
//! - quantum execution time;
//! - classical processing time;
//! - total time;
//! - time-to-solution;
//! - circuit width;
//! - circuit depth;
//! - gate count;
//! - two-qubit gate count;
//! - oracle calls;
//! - amplitude-amplification calls;
//! - shots;
//! - circuits executed;
//! - qubits;
//! - logical qubits;
//! - physical qubits;
//! - compilation cost;
//! - routing cost;
//! - measurement cost;
//! - reproducibility information.
//!
//! QED-C's application-oriented methodology explicitly treats quality,
//! execution time, and quantum resources as relevant dimensions of application
//! performance rather than reducing an application to one scalar metric.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97
//! - Rust 1.97.1
//! - Rust 2021
//!
//! No nightly features.
//! No unsafe code.
//! No additional dependencies.
//!
//! # Integration contract
//!
//! This file integrates with the repository's already-established contracts:
//!
//! ```text
//! benchmarking::generators::application
//!     ├── ApplicationBenchmarkGenerator
//!     ├── ApplicationGenerationRequest
//!     ├── ApplicationGeneratorDescriptor
//!     └── ApplicationGeneratorCapability
//!
//! benchmarking::core::workload
//!     ├── ApplicationParameter
//!     ├── ApplicationWorkload
//!     └── WorkloadId
//!
//! quantum::algorithms::amplitude
//!     └── amplitude-estimation execution boundary
//!
//! benchmarking::core::experiment
//!     └── future experiment integration
//!
//! benchmarking::execution
//!     └── future execution integration
//!
//! benchmarking::statistics
//!     └── future estimation/error analysis
//!
//! benchmarking::metrics
//!     └── future quality/resource/time metrics
//! ```
//!
//! No modification of those files is required to establish this module's
//! semantic contract.
//!
//! The only namespace addition required for this file is:
//!
//! ```text
//! src/quantum/benchmarking/applications/mod.rs
//!
//! pub mod monte_carlo;
//! ```
//!
//! Registry integration belongs in `registry/builtin.rs` and can consume the
//! stable constants and generator without changing this file.
//!
//! # Security/resource model
//!
//! Monte Carlo benchmark requests may eventually originate from:
//!
//! - Zamani source;
//! - CLI;
//! - CI;
//! - benchmark configuration files;
//! - remote benchmark services;
//! - machine-generated benchmark requests.
//!
//! Therefore every input is treated as untrusted.
//!
//! This module:
//!
//! - validates all identifiers;
//! - validates all floating-point values;
//! - rejects NaN;
//! - rejects infinities;
//! - rejects zero problem sizes;
//! - rejects zero precision;
//! - rejects negative precision;
//! - validates bounded amplitudes;
//! - validates scaling parameters;
//! - bounds parameter counts;
//! - bounds encoded values;
//! - uses checked arithmetic;
//! - bounds derived resource estimates;
//! - never allocates based on an unchecked problem-size multiplication;
//! - never invokes external code;
//! - never performs I/O.
//!
//! Global limits remain owned by `benchmarking::core::limits`.
//!
//! # Reproducibility
//!
//! Monte Carlo workload generation is deterministic with respect to:
//!
//! ```text
//! application_id
//! instance_id
//! problem_size
//! distribution_id
//! payoff_id
//! estimation_method
//! target_precision
//! amplitude
//! offset
//! scale
//! reference_value
//! seed
//! sequence_index
//! generator_revision
//! ```
//!
//! The seed belongs to the common application-generation contract. This file
//! never uses it as hidden entropy.
//!
//! # Important statistical distinction
//!
//! `reference_value` is not an observed result.
//!
//! It is a benchmark reference used by the analysis layer to calculate error.
//! It may come from:
//!
//! - an analytically known value;
//! - a high-precision classical calculation;
//! - a trusted numerical reference;
//! - a small-instance exact computation.
//!
//! The benchmark must record how the reference was obtained. This file
//! therefore exposes a `ReferenceKind` rather than pretending all references
//! have identical scientific status.
//!
//! # QED-C alignment
//!
//! QED-C includes Monte Carlo Sampling as a Level-4 functional benchmark.
//! Its application-oriented benchmark framework varies problem size and
//! collects quality, execution-time, and quantum-resource information.
//!
//! Zamani retains that application-oriented philosophy but makes the semantic
//! problem definition explicit and backend-independent.
//!
//! # Important limitation
//!
//! This file defines the benchmark *problem and workload contract*.
//!
//! It is not itself the final Monte Carlo analyzer.
//!
//! The future analysis layer must consume raw observations and produce:
//!
//! ```text
//! MonteCarloResult
//! ```
//!
//! containing measured estimate, error, uncertainty, success decision,
//! execution resources, and provenance.
//!
//! That separation is intentional and prevents the generator from mixing
//! workload construction with experimental analysis.

use std::fmt;

use super::super::core::errors::{BenchmarkError, BenchmarkResult};
use super::super::core::workload::{
    ApplicationParameter,
    ApplicationWorkload,
    WorkloadId,
};
use super::super::generators::application::{
    ApplicationBenchmarkGenerator,
    ApplicationGenerationRequest,
    ApplicationGeneratorCapability,
    ApplicationGeneratorDescriptor,
};

// =============================================================================
// Stable benchmark identity
// =============================================================================

/// Stable machine-readable Monte Carlo benchmark identifier.
pub const MONTE_CARLO_BENCHMARK_ID: &str = "monte_carlo";

/// Stable application identifier.
pub const MONTE_CARLO_APPLICATION_ID: &str = "monte_carlo";

/// Semantic version of the benchmark definition.
///
/// This is independent of the Zamani version, compiler version, Quantum IR
/// version, and algorithm implementation version.
pub const MONTE_CARLO_BENCHMARK_VERSION: u32 = 1;

/// Generator implementation version.
pub const MONTE_CARLO_GENERATOR_VERSION: &str = "1.0.0";

/// Reproducibility revision.
///
/// Increment this when generation semantics change.
pub const MONTE_CARLO_GENERATOR_REVISION: u32 = 1;

/// Human-readable benchmark name.
pub const MONTE_CARLO_NAME: &str = "Quantum Monte Carlo Sampling";

/// Maximum UTF-8 byte length of a Monte Carlo identifier.
pub const MAX_MONTE_CARLO_IDENTIFIER_BYTES: usize = 128;

/// Maximum number of application parameters generated by this benchmark.
pub const MAX_MONTE_CARLO_PARAMETERS: usize = 64;

/// Maximum problem size represented by the benchmark definition.
///
/// This is a structural guard, not a claim about the maximum capability of
/// quantum hardware.
pub const MAX_MONTE_CARLO_PROBLEM_SIZE: usize = 4096;

/// Maximum requested target precision.
///
/// A precision larger than this would be equivalent to an extremely loose
/// benchmark and is normally a configuration error.
pub const MAX_MONTE_CARLO_PRECISION: f64 = 1.0;

/// Minimum positive precision accepted by the benchmark.
///
/// Extremely small values create impractical resource demands and are rejected
/// at this semantic boundary rather than allowing pathological requests to
/// propagate into the execution system.
pub const MIN_MONTE_CARLO_PRECISION: f64 = 1.0e-12;

/// Maximum absolute scale factor.
pub const MAX_MONTE_CARLO_SCALE: f64 = 1.0e15;

/// Maximum absolute offset.
pub const MAX_MONTE_CARLO_OFFSET: f64 = 1.0e15;

/// Maximum encoded identifier/value length.
pub const MAX_MONTE_CARLO_VALUE_BYTES: usize = 512;

/// Maximum estimated amplitude-estimation precision qubits.
///
/// This is deliberately bounded because phase-register requirements grow with
/// requested precision and must never be allowed to create an accidental
/// enormous workload.
pub const MAX_ESTIMATION_QUBITS: usize = 64;

/// Maximum estimated oracle applications.
///
/// This is a semantic safety bound used only for resource estimation.
pub const MAX_ESTIMATED_ORACLE_APPLICATIONS: u128 = 1u128 << 62;

// =============================================================================
// Estimation method
// =============================================================================

/// Quantum/classical estimation strategy used by the benchmark workload.
///
/// The benchmark definition does not implement the method. It records the
/// requested strategy so the algorithm/execution layer can select the
/// corresponding implementation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MonteCarloEstimationMethod {
    /// Canonical phase-estimation-based quantum amplitude estimation.
    CanonicalAmplitudeEstimation,

    /// Iterative/adaptive amplitude estimation.
    IterativeAmplitudeEstimation,

    /// Maximum-likelihood amplitude estimation.
    MaximumLikelihoodAmplitudeEstimation,

    /// Classical Monte Carlo baseline.
    ///
    /// This is useful for end-to-end quantum-versus-classical comparison but
    /// does not require a quantum backend.
    ClassicalMonteCarlo,
}

impl MonteCarloEstimationMethod {
    /// Stable machine-readable identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CanonicalAmplitudeEstimation => {
                "canonical_amplitude_estimation"
            }
            Self::IterativeAmplitudeEstimation => {
                "iterative_amplitude_estimation"
            }
            Self::MaximumLikelihoodAmplitudeEstimation => {
                "maximum_likelihood_amplitude_estimation"
            }
            Self::ClassicalMonteCarlo => "classical_monte_carlo",
        }
    }

    /// Returns whether the method requires a quantum execution target.
    #[must_use]
    pub const fn requires_quantum_execution(self) -> bool {
        !matches!(self, Self::ClassicalMonteCarlo)
    }

    /// Returns whether the method can produce a circuit workload.
    ///
    /// The concrete circuit is produced by the algorithm/execution adapter,
    /// not by this benchmark definition.
    #[must_use]
    pub const fn supports_quantum_circuit_execution(self) -> bool {
        self.requires_quantum_execution()
    }
}

impl fmt::Display for MonteCarloEstimationMethod {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Distribution semantics
// =============================================================================

/// Stable identifier for the probability distribution used by the Monte Carlo
/// problem.
///
/// The benchmark deliberately uses identifiers rather than function pointers
/// or executable closures so workloads remain serializable and reproducible.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MonteCarloDistribution {
    /// Uniform distribution over the normalized domain.
    Uniform,

    /// Bernoulli distribution.
    Bernoulli,

    /// Normal/Gaussian distribution.
    Normal,

    /// Log-normal distribution.
    LogNormal,

    /// Exponential distribution.
    Exponential,

    /// User-defined registered distribution.
    Custom(String),
}

impl MonteCarloDistribution {
    /// Stable machine-readable identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Uniform => "uniform",
            Self::Bernoulli => "bernoulli",
            Self::Normal => "normal",
            Self::LogNormal => "log_normal",
            Self::Exponential => "exponential",
            Self::Custom(value) => value.as_str(),
        }
    }

    /// Validates a distribution identifier.
    pub fn validate(&self) -> BenchmarkResult<()> {
        match self {
            Self::Custom(value) => validate_identifier(
                "distribution_id",
                value,
                MAX_MONTE_CARLO_IDENTIFIER_BYTES,
            ),
            _ => Ok(()),
        }
    }
}

impl fmt::Display for MonteCarloDistribution {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Payoff/integrand semantics
// =============================================================================

/// Stable identifier for the function being averaged by Monte Carlo.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MonteCarloPayoff {
    /// Identity function.
    Identity,

    /// Indicator function.
    Indicator,

    /// Squared value.
    Square,

    /// Absolute value.
    Absolute,

    /// User-defined registered payoff.
    Custom(String),
}

impl MonteCarloPayoff {
    /// Stable machine-readable identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Identity => "identity",
            Self::Indicator => "indicator",
            Self::Square => "square",
            Self::Absolute => "absolute",
            Self::Custom(value) => value.as_str(),
        }
    }

    /// Validates the payoff identifier.
    pub fn validate(&self) -> BenchmarkResult<()> {
        match self {
            Self::Custom(value) => validate_identifier(
                "payoff_id",
                value,
                MAX_MONTE_CARLO_IDENTIFIER_BYTES,
            ),
            _ => Ok(()),
        }
    }
}

impl fmt::Display for MonteCarloPayoff {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Reference semantics
// =============================================================================

/// Scientific origin of the reference value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReferenceKind {
    /// Analytically derived reference.
    Analytic,

    /// Exact finite-size classical calculation.
    ExactClassical,

    /// High-precision numerical calculation.
    HighPrecisionNumerical,

    /// Trusted external reference.
    ExternalReference,

    /// No reference is supplied.
    Unavailable,
}

impl ReferenceKind {
    /// Stable machine-readable identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Analytic => "analytic",
            Self::ExactClassical => "exact_classical",
            Self::HighPrecisionNumerical => "high_precision_numerical",
            Self::ExternalReference => "external_reference",
            Self::Unavailable => "unavailable",
        }
    }

    /// Whether a numerical reference is required.
    #[must_use]
    pub const fn requires_value(self) -> bool {
        !matches!(self, Self::Unavailable)
    }
}

impl fmt::Display for ReferenceKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Success criterion
// =============================================================================

/// Scientific pass/fail condition for a Monte Carlo experiment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MonteCarloSuccessCriterion {
    /// Require absolute estimation error <= target precision.
    AbsoluteError,

    /// Require relative estimation error <= target precision.
    RelativeError,

    /// Require both absolute and relative error criteria where both are
    /// meaningful.
    AbsoluteAndRelativeError,

    /// Benchmark performance without a scientific pass/fail decision.
    None,
}

impl MonteCarloSuccessCriterion {
    /// Stable machine-readable identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AbsoluteError => "absolute_error",
            Self::RelativeError => "relative_error",
            Self::AbsoluteAndRelativeError => {
                "absolute_and_relative_error"
            }
            Self::None => "none",
        }
    }
}

impl fmt::Display for MonteCarloSuccessCriterion {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Benchmark metrics
// =============================================================================

/// Monte Carlo-specific metric declaration.
///
/// Numerical storage belongs to the generic benchmarking metric subsystem.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MonteCarloMetric {
    /// Estimated application value.
    Estimate,

    /// Known/reference application value.
    ReferenceValue,

    /// Absolute estimation error.
    AbsoluteError,

    /// Relative estimation error.
    RelativeError,

    /// Whether the benchmark passed.
    Success,

    /// Requested target precision.
    TargetPrecision,

    /// Number of quantum evaluation qubits.
    EvaluationQubits,

    /// Number of total logical qubits.
    LogicalQubits,

    /// Number of oracle applications.
    OracleApplications,

    /// Number of amplitude-amplification applications.
    AmplificationApplications,

    /// Number of circuit executions.
    CircuitExecutions,

    /// Total shots.
    Shots,

    /// Circuit depth.
    CircuitDepth,

    /// Gate count.
    GateCount,

    /// Two-qubit gate count.
    TwoQubitGateCount,

    /// Quantum execution time.
    QuantumExecutionTime,

    /// Classical processing time.
    ClassicalProcessingTime,

    /// Total wall-clock time.
    TotalTime,

    /// End-to-end time-to-solution.
    TimeToSolution,

    /// Number of samples in the classical baseline.
    ClassicalSamples,

    /// Quantum speedup comparison when a valid classical baseline exists.
    QuantumSpeedup,
}

impl MonteCarloMetric {
    /// Stable machine-readable identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Estimate => "estimate",
            Self::ReferenceValue => "reference_value",
            Self::AbsoluteError => "absolute_error",
            Self::RelativeError => "relative_error",
            Self::Success => "success",
            Self::TargetPrecision => "target_precision",
            Self::EvaluationQubits => "evaluation_qubits",
            Self::LogicalQubits => "logical_qubits",
            Self::OracleApplications => "oracle_applications",
            Self::AmplificationApplications => {
                "amplification_applications"
            }
            Self::CircuitExecutions => "circuit_executions",
            Self::Shots => "shots",
            Self::CircuitDepth => "circuit_depth",
            Self::GateCount => "gate_count",
            Self::TwoQubitGateCount => "two_qubit_gate_count",
            Self::QuantumExecutionTime => "quantum_execution_time",
            Self::ClassicalProcessingTime => "classical_processing_time",
            Self::TotalTime => "total_time",
            Self::TimeToSolution => "time_to_solution",
            Self::ClassicalSamples => "classical_samples",
            Self::QuantumSpeedup => "quantum_speedup",
        }
    }
}

impl fmt::Display for MonteCarloMetric {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Monte Carlo benchmark configuration
// =============================================================================

/// Complete semantic configuration for one Monte Carlo benchmark workload.
#[derive(Debug, Clone, PartialEq)]
pub struct MonteCarloBenchmarkConfig {
    /// Probability distribution.
    pub distribution: MonteCarloDistribution,

    /// Integrand/payoff function.
    pub payoff: MonteCarloPayoff,

    /// Quantum/classical estimation strategy.
    pub estimation_method: MonteCarloEstimationMethod,

    /// Problem size used by the application benchmark.
    ///
    /// This represents the application/instance size, not necessarily the
    /// number of qubits. The algorithm adapter determines the concrete quantum
    /// representation.
    pub problem_size: usize,

    /// Desired application-space absolute precision.
    pub target_precision: f64,

    /// Amplitude-space probability to be estimated.
    ///
    /// This is optional because a general Monte Carlo workload may require the
    /// algorithm adapter to derive the amplitude from the distribution/payoff
    /// definition.
    pub target_amplitude: Option<f64>,

    /// Affine mapping from amplitude space to application space.
    ///
    /// `application_value = offset + scale * amplitude`.
    pub amplitude_scale: f64,

    /// Affine mapping offset.
    pub amplitude_offset: f64,

    /// Reference value, when known.
    pub reference_value: Option<f64>,

    /// Origin of the reference value.
    pub reference_kind: ReferenceKind,

    /// Scientific success criterion.
    pub success_criterion: MonteCarloSuccessCriterion,

    /// Number of classical Monte Carlo samples used when the classical
    /// baseline is requested.
    pub classical_samples: Option<usize>,

    /// Requested benchmark metrics.
    pub metrics: Vec<MonteCarloMetric>,
}

impl MonteCarloBenchmarkConfig {
    /// Creates a standard quantum Monte Carlo configuration.
    pub fn new(
        distribution: MonteCarloDistribution,
        payoff: MonteCarloPayoff,
        problem_size: usize,
        target_precision: f64,
    ) -> BenchmarkResult<Self> {
        let configuration = Self {
            distribution,
            payoff,
            estimation_method:
                MonteCarloEstimationMethod::CanonicalAmplitudeEstimation,
            problem_size,
            target_precision,
            target_amplitude: None,
            amplitude_scale: 1.0,
            amplitude_offset: 0.0,
            reference_value: None,
            reference_kind: ReferenceKind::Unavailable,
            success_criterion: MonteCarloSuccessCriterion::AbsoluteError,
            classical_samples: None,
            metrics: default_metrics(),
        };

        configuration.validate()?;

        Ok(configuration)
    }

    /// Sets the quantum/classical estimation method.
    pub fn with_estimation_method(
        mut self,
        method: MonteCarloEstimationMethod,
    ) -> Self {
        self.estimation_method = method;
        self
    }

    /// Sets the target amplitude.
    pub fn with_target_amplitude(
        mut self,
        amplitude: f64,
    ) -> BenchmarkResult<Self> {
        self.target_amplitude = Some(amplitude);
        self.validate()?;
        Ok(self)
    }

    /// Sets the affine amplitude-to-application scaling.
    pub fn with_scaling(
        mut self,
        offset: f64,
        scale: f64,
    ) -> BenchmarkResult<Self> {
        self.amplitude_offset = offset;
        self.amplitude_scale = scale;
        self.validate()?;
        Ok(self)
    }

    /// Sets a reference value and its scientific provenance.
    pub fn with_reference(
        mut self,
        value: f64,
        kind: ReferenceKind,
    ) -> BenchmarkResult<Self> {
        self.reference_value = Some(value);
        self.reference_kind = kind;
        self.validate()?;
        Ok(self)
    }

    /// Removes the reference value.
    pub fn without_reference(mut self) -> Self {
        self.reference_value = None;
        self.reference_kind = ReferenceKind::Unavailable;
        self
    }

    /// Sets the success criterion.
    pub fn with_success_criterion(
        mut self,
        criterion: MonteCarloSuccessCriterion,
    ) -> Self {
        self.success_criterion = criterion;
        self
    }

    /// Sets the classical baseline sample count.
    pub fn with_classical_samples(
        mut self,
        samples: usize,
    ) -> BenchmarkResult<Self> {
        if samples == 0 {
            return Err(BenchmarkError::InvalidRange {
                field: "classical_samples".to_owned(),
                value: "0".to_owned(),
                minimum: Some("1".to_owned()),
                maximum: None,
            });
        }

        self.classical_samples = Some(samples);
        self.validate()?;
        Ok(self)
    }

    /// Replaces the requested metric list.
    pub fn with_metrics(
        mut self,
        metrics: Vec<MonteCarloMetric>,
    ) -> BenchmarkResult<Self> {
        validate_metric_count(metrics.len())?;
        self.metrics = deduplicate_metrics(metrics);
        Ok(self)
    }

    /// Adds one metric if it is not already present.
    pub fn with_metric(mut self, metric: MonteCarloMetric) -> BenchmarkResult<Self> {
        if !self.metrics.contains(&metric) {
            validate_metric_count(
                self.metrics
                    .len()
                    .checked_add(1)
                    .ok_or_else(|| numerical_overflow("Monte Carlo metric count"))?,
            )?;

            self.metrics.push(metric);
        }

        Ok(self)
    }

    /// Validates the complete benchmark configuration.
    pub fn validate(&self) -> BenchmarkResult<()> {
        self.distribution.validate()?;
        self.payoff.validate()?;

        if self.problem_size == 0 {
            return Err(BenchmarkError::InvalidRange {
                field: "problem_size".to_owned(),
                value: "0".to_owned(),
                minimum: Some("1".to_owned()),
                maximum: Some(MAX_MONTE_CARLO_PROBLEM_SIZE.to_string()),
            });
        }

        if self.problem_size > MAX_MONTE_CARLO_PROBLEM_SIZE {
            return Err(BenchmarkError::ResourceLimitExceeded {
                resource: "monte_carlo_problem_size".to_owned(),
                requested: self.problem_size as u64,
                maximum: MAX_MONTE_CARLO_PROBLEM_SIZE as u64,
            });
        }

        validate_precision(self.target_precision)?;

        validate_finite_bounded(
            "amplitude_scale",
            self.amplitude_scale,
            MAX_MONTE_CARLO_SCALE,
        )?;

        validate_finite_bounded(
            "amplitude_offset",
            self.amplitude_offset,
            MAX_MONTE_CARLO_OFFSET,
        )?;

        if let Some(amplitude) = self.target_amplitude {
            validate_probability("target_amplitude", amplitude)?;
        }

        match (
            self.reference_kind.requires_value(),
            self.reference_value,
        ) {
            (true, None) => {
                return Err(BenchmarkError::InconsistentConfiguration {
                    first: "reference_kind".to_owned(),
                    second: "reference_value".to_owned(),
                    reason: "a reference kind requiring a value must have a reference value"
                        .to_owned(),
                });
            }

            (false, Some(_)) => {
                return Err(BenchmarkError::InconsistentConfiguration {
                    first: "reference_kind".to_owned(),
                    second: "reference_value".to_owned(),
                    reason: "reference value is present while reference kind is unavailable"
                        .to_owned(),
                });
            }

            _ => {}
        }

        if let Some(reference) = self.reference_value {
            if !reference.is_finite() {
                return Err(BenchmarkError::InvalidValue {
                    field: "reference_value".to_owned(),
                    value: reference.to_string(),
                    reason: "reference value must be finite".to_owned(),
                });
            }
        }

        if let Some(samples) = self.classical_samples {
            if samples == 0 {
                return Err(BenchmarkError::InvalidRange {
                    field: "classical_samples".to_owned(),
                    value: "0".to_owned(),
                    minimum: Some("1".to_owned()),
                    maximum: None,
                });
            }
        }

        validate_metric_count(self.metrics.len())?;

        validate_success_criterion_requirements(self)?;

        let _ = self.estimated_evaluation_qubits()?;
        let _ = self.estimated_oracle_applications()?;

        Ok(())
    }

    /// Returns the estimated number of evaluation/phase qubits required by
    /// canonical phase-estimation-based amplitude estimation.
    ///
    /// This is an estimate only. The concrete algorithm implementation owns
    /// the final circuit construction.
    pub fn estimated_evaluation_qubits(&self) -> BenchmarkResult<usize> {
        let precision = self.target_precision;

        let raw = (-precision.log2()).ceil();

        if !raw.is_finite() || raw < 1.0 {
            return Err(BenchmarkError::NumericalOverflow {
                operation: "Monte Carlo evaluation-qubit estimate".to_owned(),
                value: Some(raw.to_string()),
            });
        }

        let estimate = raw as usize;

        if estimate > MAX_ESTIMATION_QUBITS {
            return Err(BenchmarkError::ResourceLimitExceeded {
                resource: "monte_carlo_evaluation_qubits".to_owned(),
                requested: estimate as u64,
                maximum: MAX_ESTIMATION_QUBITS as u64,
            });
        }

        Ok(estimate)
    }

    /// Returns an estimated number of oracle/amplification applications for
    /// canonical phase-estimation-style QAE.
    ///
    /// The estimate is intentionally conservative and checked.
    pub fn estimated_oracle_applications(&self) -> BenchmarkResult<u128> {
        match self.estimation_method {
            MonteCarloEstimationMethod::ClassicalMonteCarlo => Ok(0),

            MonteCarloEstimationMethod::CanonicalAmplitudeEstimation => {
                let evaluation_qubits =
                    self.estimated_evaluation_qubits()?;

                let exponent =
                    u32::try_from(evaluation_qubits).map_err(|_| {
                        BenchmarkError::NumericalOverflow {
                            operation:
                                "Monte Carlo oracle exponent conversion"
                                    .to_owned(),
                            value: Some(evaluation_qubits.to_string()),
                        }
                    })?;

                let applications = 1u128
                    .checked_shl(exponent)
                    .ok_or_else(|| {
                        numerical_overflow(
                            "Monte Carlo oracle-application estimate",
                        )
                    })?;

                if applications > MAX_ESTIMATED_ORACLE_APPLICATIONS {
                    return Err(BenchmarkError::ResourceLimitExceeded {
                        resource:
                            "monte_carlo_oracle_applications".to_owned(),
                        requested: u64::try_from(applications)
                            .unwrap_or(u64::MAX),
                        maximum: u64::try_from(
                            MAX_ESTIMATED_ORACLE_APPLICATIONS,
                        )
                        .unwrap_or(u64::MAX),
                    });
                }

                Ok(applications)
            }

            MonteCarloEstimationMethod::IterativeAmplitudeEstimation
            | MonteCarloEstimationMethod::MaximumLikelihoodAmplitudeEstimation => {
                // These methods are adaptive. Their final number of oracle
                // applications depends on the execution/analysis policy.
                //
                // The benchmark therefore exposes no false deterministic
                // count here.
                Ok(0)
            }
        }
    }

    /// Returns the application-space value represented by a known amplitude.
    pub fn application_value_from_amplitude(
        &self,
        amplitude: f64,
    ) -> BenchmarkResult<f64> {
        validate_probability("amplitude", amplitude)?;

        let scaled = self
            .amplitude_scale
            .mul_add(amplitude, self.amplitude_offset);

        if !scaled.is_finite() {
            return Err(BenchmarkError::NumericalOverflow {
                operation:
                    "Monte Carlo amplitude-to-application conversion"
                        .to_owned(),
                value: Some(scaled.to_string()),
            });
        }

        Ok(scaled)
    }

    /// Returns the amplitude corresponding to a known application-space value.
    pub fn amplitude_from_application_value(
        &self,
        application_value: f64,
    ) -> BenchmarkResult<f64> {
        if !application_value.is_finite() {
            return Err(BenchmarkError::InvalidValue {
                field: "application_value".to_owned(),
                value: application_value.to_string(),
                reason: "application value must be finite".to_owned(),
            });
        }

        if self.amplitude_scale == 0.0 {
            return Err(BenchmarkError::InconsistentConfiguration {
                first: "amplitude_scale".to_owned(),
                second: "application_value".to_owned(),
                reason: "amplitude scale must be non-zero when converting an application value back to amplitude space"
                    .to_owned(),
            });
        }

        let amplitude =
            (application_value - self.amplitude_offset) / self.amplitude_scale;

        validate_probability("derived_amplitude", amplitude)?;

        Ok(amplitude)
    }
}

// =============================================================================
// Strongly typed workload description
// =============================================================================

/// Fully validated semantic description of one Monte Carlo benchmark case.
///
/// This structure is deliberately separate from `ApplicationWorkload`.
/// `ApplicationWorkload` remains the canonical benchmarking workload model;
/// this type provides a strongly typed Rust-facing API for Monte Carlo.
#[derive(Debug, Clone, PartialEq)]
pub struct MonteCarloWorkloadDescription {
    /// Benchmark configuration.
    pub configuration: MonteCarloBenchmarkConfig,

    /// Stable application instance identifier.
    pub instance_id: WorkloadId,

    /// Generation seed.
    pub seed: u64,

    /// Generation sequence index.
    pub sequence_index: u64,
}

impl MonteCarloWorkloadDescription {
    /// Creates a validated workload description.
    pub fn new(
        configuration: MonteCarloBenchmarkConfig,
        instance_id: WorkloadId,
        seed: u64,
        sequence_index: u64,
    ) -> BenchmarkResult<Self> {
        configuration.validate()?;

        Ok(Self {
            configuration,
            instance_id,
            seed,
            sequence_index,
        })
    }

    /// Converts the semantic description into the canonical Zamani
    /// `ApplicationWorkload`.
    ///
    /// No backend-specific state is introduced.
    pub fn into_application_workload(
        &self,
    ) -> BenchmarkResult<ApplicationWorkload> {
        self.configuration.validate()?;

        let mut workload = ApplicationWorkload::new(
            MONTE_CARLO_APPLICATION_ID,
            self.instance_id.clone(),
            self.configuration.problem_size,
        )
        .map_err(BenchmarkError::from)?;

        add_parameter(
            &mut workload,
            "benchmark_version",
            MONTE_CARLO_BENCHMARK_VERSION.to_string(),
        )?;

        add_parameter(
            &mut workload,
            "generator_version",
            MONTE_CARLO_GENERATOR_VERSION,
        )?;

        add_parameter(
            &mut workload,
            "generator_revision",
            MONTE_CARLO_GENERATOR_REVISION.to_string(),
        )?;

        add_parameter(
            &mut workload,
            "distribution_id",
            self.configuration.distribution.as_str(),
        )?;

        add_parameter(
            &mut workload,
            "payoff_id",
            self.configuration.payoff.as_str(),
        )?;

        add_parameter(
            &mut workload,
            "estimation_method",
            self.configuration.estimation_method.as_str(),
        )?;

        add_parameter(
            &mut workload,
            "target_precision",
            format_float(self.configuration.target_precision),
        )?;

        add_parameter(
            &mut workload,
            "amplitude_scale",
            format_float(self.configuration.amplitude_scale),
        )?;

        add_parameter(
            &mut workload,
            "amplitude_offset",
            format_float(self.configuration.amplitude_offset),
        )?;

        add_parameter(
            &mut workload,
            "success_criterion",
            self.configuration.success_criterion.as_str(),
        )?;

        add_parameter(
            &mut workload,
            "reference_kind",
            self.configuration.reference_kind.as_str(),
        )?;

        add_parameter(
            &mut workload,
            "reference_value",
            self.configuration
                .reference_value
                .map(format_float)
                .unwrap_or_else(|| "unavailable".to_owned()),
        )?;

        add_parameter(
            &mut workload,
            "target_amplitude",
            self.configuration
                .target_amplitude
                .map(format_float)
                .unwrap_or_else(|| "derived_or_unknown".to_owned()),
        )?;

        add_parameter(
            &mut workload,
            "seed",
            self.seed.to_string(),
        )?;

        add_parameter(
            &mut workload,
            "sequence_index",
            self.sequence_index.to_string(),
        )?;

        add_parameter(
            &mut workload,
            "quantum_execution_required",
            self.configuration
                .estimation_method
                .requires_quantum_execution()
                .to_string(),
        )?;

        add_parameter(
            &mut workload,
            "estimated_evaluation_qubits",
            self.configuration
                .estimated_evaluation_qubits()?
                .to_string(),
        )?;

        add_parameter(
            &mut workload,
            "estimated_oracle_applications",
            self.configuration
                .estimated_oracle_applications()?
                .to_string(),
        )?;

        if let Some(samples) = self.configuration.classical_samples {
            add_parameter(
                &mut workload,
                "classical_samples",
                samples.to_string(),
            )?;
        }

        for metric in &self.configuration.metrics {
            add_parameter(
                &mut workload,
                metric_parameter_name(*metric),
                "enabled",
            )?;
        }

        Ok(workload)
    }
}

// =============================================================================
// Generator
// =============================================================================

/// Production Monte Carlo benchmark generator.
///
/// The generator is intentionally zero-sized. All semantic configuration is
/// supplied through `ApplicationGenerationRequest`.
#[derive(Debug, Clone)]
pub struct MonteCarloBenchmarkGenerator {
    descriptor: ApplicationGeneratorDescriptor,
}

impl MonteCarloBenchmarkGenerator {
    /// Creates the canonical Monte Carlo generator.
    pub fn new() -> BenchmarkResult<Self> {
        let descriptor = ApplicationGeneratorDescriptor::new(
            MONTE_CARLO_BENCHMARK_ID,
            MONTE_CARLO_APPLICATION_ID,
            MONTE_CARLO_GENERATOR_VERSION,
            "Production quantum Monte Carlo application benchmark using a \
             backend-independent amplitude-estimation workload contract.",
        )?
        .with_capabilities([
            ApplicationGeneratorCapability::Parameterized,
            ApplicationGeneratorCapability::Deterministic,
            ApplicationGeneratorCapability::ScalableProblemSize,
            ApplicationGeneratorCapability::Hybrid,
            ApplicationGeneratorCapability::ResourceEstimation,
            ApplicationGeneratorCapability::ExactSmallInstanceReference,
        ]);

        Ok(Self { descriptor })
    }

    /// Returns the canonical descriptor without heap allocation.
    ///
    /// This helper is useful for registry code when construction cannot fail
    /// because the constants are compile-time validated literals.
    pub fn canonical_descriptor() -> BenchmarkResult<ApplicationGeneratorDescriptor> {
        ApplicationGeneratorDescriptor::new(
            MONTE_CARLO_BENCHMARK_ID,
            MONTE_CARLO_APPLICATION_ID,
            MONTE_CARLO_GENERATOR_VERSION,
            "Production quantum Monte Carlo application benchmark using a \
             backend-independent amplitude-estimation workload contract.",
        )
        .map(|descriptor| {
            descriptor.with_capabilities([
                ApplicationGeneratorCapability::Parameterized,
                ApplicationGeneratorCapability::Deterministic,
                ApplicationGeneratorCapability::ScalableProblemSize,
                ApplicationGeneratorCapability::Hybrid,
                ApplicationGeneratorCapability::ResourceEstimation,
                ApplicationGeneratorCapability::ExactSmallInstanceReference,
            ])
        })
    }

    /// Parses the common application-generation request into the strongly
    /// typed Monte Carlo configuration.
    ///
    /// The request must use the parameter names defined by this module.
    pub fn configuration_from_request(
        request: &ApplicationGenerationRequest,
    ) -> BenchmarkResult<MonteCarloBenchmarkConfig> {
        request.validate()?;

        if request.application_id() != MONTE_CARLO_APPLICATION_ID {
            return Err(BenchmarkError::InconsistentConfiguration {
                first: "request.application_id".to_owned(),
                second: "monte_carlo.application_id".to_owned(),
                reason: "application identifier does not match the Monte Carlo generator"
                    .to_owned(),
            });
        }

        let mut distribution = MonteCarloDistribution::Uniform;
        let mut payoff = MonteCarloPayoff::Identity;

        let mut method =
            MonteCarloEstimationMethod::CanonicalAmplitudeEstimation;

        let mut target_precision = 0.01_f64;
        let mut target_amplitude = None;
        let mut amplitude_scale = 1.0_f64;
        let mut amplitude_offset = 0.0_f64;

        let mut reference_value = None;
        let mut reference_kind = ReferenceKind::Unavailable;

        let mut success_criterion =
            MonteCarloSuccessCriterion::AbsoluteError;

        let mut classical_samples = None;

        let mut metrics = default_metrics();

        let mut seen = ParameterSet::new();

        for parameter in request.parameters() {
            let name = parameter.name();
            let value = parameter.value();

            if !seen.insert(name) {
                return Err(BenchmarkError::InconsistentConfiguration {
                    first: name.to_owned(),
                    second: name.to_owned(),
                    reason: "duplicate Monte Carlo application parameter"
                        .to_owned(),
                });
            }

            match name {
                "distribution_id" => {
                    distribution = parse_distribution(value)?;
                }

                "payoff_id" => {
                    payoff = parse_payoff(value)?;
                }

                "estimation_method" => {
                    method = parse_estimation_method(value)?;
                }

                "target_precision" => {
                    target_precision = parse_f64_parameter(
                        "target_precision",
                        value,
                    )?;
                }

                "target_amplitude" => {
                    if value != "derived_or_unknown" {
                        target_amplitude = Some(parse_f64_parameter(
                            "target_amplitude",
                            value,
                        )?);
                    }
                }

                "amplitude_scale" => {
                    amplitude_scale = parse_f64_parameter(
                        "amplitude_scale",
                        value,
                    )?;
                }

                "amplitude_offset" => {
                    amplitude_offset = parse_f64_parameter(
                        "amplitude_offset",
                        value,
                    )?;
                }

                "reference_kind" => {
                    reference_kind = parse_reference_kind(value)?;
                }

                "reference_value" => {
                    if value != "unavailable" {
                        reference_value = Some(parse_f64_parameter(
                            "reference_value",
                            value,
                        )?);
                    }
                }

                "success_criterion" => {
                    success_criterion =
                        parse_success_criterion(value)?;
                }

                "classical_samples" => {
                    classical_samples = Some(parse_usize_parameter(
                        "classical_samples",
                        value,
                    )?);
                }

                "metrics" => {
                    metrics = parse_metrics(value)?;
                }

                "benchmark_version"
                | "generator_version"
                | "generator_revision"
                | "seed"
                | "sequence_index"
                | "quantum_execution_required"
                | "estimated_evaluation_qubits"
                | "estimated_oracle_applications" => {
                    // These are generated/provenance parameters and are not
                    // configuration inputs.
                }

                _ => {
                    return Err(BenchmarkError::InvalidConfiguration {
                        field: name.to_owned(),
                        reason:
                            "unknown Monte Carlo application parameter"
                                .to_owned(),
                    });
                }
            }
        }

        let configuration = MonteCarloBenchmarkConfig {
            distribution,
            payoff,
            estimation_method: method,
            problem_size: request.problem_size(),
            target_precision,
            target_amplitude,
            amplitude_scale,
            amplitude_offset,
            reference_value,
            reference_kind,
            success_criterion,
            classical_samples,
            metrics,
        };

        configuration.validate()?;

        Ok(configuration)
    }
}

impl Default for MonteCarloBenchmarkGenerator {
    fn default() -> Self {
        Self::new()
            .expect("canonical Monte Carlo benchmark descriptor must be valid")
    }
}

impl ApplicationBenchmarkGenerator for MonteCarloBenchmarkGenerator {
    fn descriptor(&self) -> &ApplicationGeneratorDescriptor {
        &self.descriptor
    }

    fn validate(
        &self,
        request: &ApplicationGenerationRequest,
    ) -> BenchmarkResult<()> {
        ApplicationBenchmarkGenerator::validate(self, request)?;

        let configuration =
            Self::configuration_from_request(request)?;

        configuration.validate()?;

        Ok(())
    }

    fn generate_workload(
        &self,
        request: &ApplicationGenerationRequest,
    ) -> BenchmarkResult<ApplicationWorkload> {
        let configuration =
            Self::configuration_from_request(request)?;

        let description = MonteCarloWorkloadDescription::new(
            configuration,
            request.instance_id().clone(),
            request.metadata().seed(),
            request.metadata().sequence_index(),
        )?;

        description.into_application_workload()
    }
}

// =============================================================================
// Default metrics
// =============================================================================

fn default_metrics() -> Vec<MonteCarloMetric> {
    vec![
        MonteCarloMetric::Estimate,
        MonteCarloMetric::ReferenceValue,
        MonteCarloMetric::AbsoluteError,
        MonteCarloMetric::RelativeError,
        MonteCarloMetric::Success,
        MonteCarloMetric::TargetPrecision,
        MonteCarloMetric::EvaluationQubits,
        MonteCarloMetric::LogicalQubits,
        MonteCarloMetric::OracleApplications,
        MonteCarloMetric::AmplificationApplications,
        MonteCarloMetric::CircuitExecutions,
        MonteCarloMetric::Shots,
        MonteCarloMetric::CircuitDepth,
        MonteCarloMetric::GateCount,
        MonteCarloMetric::TwoQubitGateCount,
        MonteCarloMetric::QuantumExecutionTime,
        MonteCarloMetric::ClassicalProcessingTime,
        MonteCarloMetric::TotalTime,
        MonteCarloMetric::TimeToSolution,
    ]
}

fn deduplicate_metrics(
    metrics: Vec<MonteCarloMetric>,
) -> Vec<MonteCarloMetric> {
    let mut result = Vec::with_capacity(metrics.len());

    for metric in metrics {
        if !result.contains(&metric) {
            result.push(metric);
        }
    }

    result
}

fn validate_metric_count(count: usize) -> BenchmarkResult<()> {
    if count > MAX_MONTE_CARLO_PARAMETERS {
        return Err(BenchmarkError::ResourceLimitExceeded {
            resource: "monte_carlo_metrics".to_owned(),
            requested: count as u64,
            maximum: MAX_MONTE_CARLO_PARAMETERS as u64,
        });
    }

    Ok(())
}

fn metric_parameter_name(metric: MonteCarloMetric) -> &'static str {
    match metric {
        MonteCarloMetric::Estimate => "metric_estimate",
        MonteCarloMetric::ReferenceValue => "metric_reference_value",
        MonteCarloMetric::AbsoluteError => "metric_absolute_error",
        MonteCarloMetric::RelativeError => "metric_relative_error",
        MonteCarloMetric::Success => "metric_success",
        MonteCarloMetric::TargetPrecision => "metric_target_precision",
        MonteCarloMetric::EvaluationQubits => "metric_evaluation_qubits",
        MonteCarloMetric::LogicalQubits => "metric_logical_qubits",
        MonteCarloMetric::OracleApplications => {
            "metric_oracle_applications"
        }
        MonteCarloMetric::AmplificationApplications => {
            "metric_amplification_applications"
        }
        MonteCarloMetric::CircuitExecutions => {
            "metric_circuit_executions"
        }
        MonteCarloMetric::Shots => "metric_shots",
        MonteCarloMetric::CircuitDepth => "metric_circuit_depth",
        MonteCarloMetric::GateCount => "metric_gate_count",
        MonteCarloMetric::TwoQubitGateCount => {
            "metric_two_qubit_gate_count"
        }
        MonteCarloMetric::QuantumExecutionTime => {
            "metric_quantum_execution_time"
        }
        MonteCarloMetric::ClassicalProcessingTime => {
            "metric_classical_processing_time"
        }
        MonteCarloMetric::TotalTime => "metric_total_time",
        MonteCarloMetric::TimeToSolution => "metric_time_to_solution",
        MonteCarloMetric::ClassicalSamples => "metric_classical_samples",
        MonteCarloMetric::QuantumSpeedup => "metric_quantum_speedup",
    }
}

// =============================================================================
// Parameter parsing
// =============================================================================

#[derive(Debug)]
struct ParameterSet<'a> {
    values: Vec<&'a str>,
}

impl<'a> ParameterSet<'a> {
    fn new() -> Self {
        Self { values: Vec::new() }
    }

    fn insert(&mut self, value: &'a str) -> bool {
        if self.values.contains(&value) {
            false
        } else {
            self.values.push(value);
            true
        }
    }
}

fn parse_distribution(
    value: &str,
) -> BenchmarkResult<MonteCarloDistribution> {
    match value {
        "uniform" => Ok(MonteCarloDistribution::Uniform),
        "bernoulli" => Ok(MonteCarloDistribution::Bernoulli),
        "normal" => Ok(MonteCarloDistribution::Normal),
        "log_normal" => Ok(MonteCarloDistribution::LogNormal),
        "exponential" => Ok(MonteCarloDistribution::Exponential),
        _ => {
            validate_identifier(
                "distribution_id",
                value,
                MAX_MONTE_CARLO_IDENTIFIER_BYTES,
            )?;

            Ok(MonteCarloDistribution::Custom(value.to_owned()))
        }
    }
}

fn parse_payoff(value: &str) -> BenchmarkResult<MonteCarloPayoff> {
    match value {
        "identity" => Ok(MonteCarloPayoff::Identity),
        "indicator" => Ok(MonteCarloPayoff::Indicator),
        "square" => Ok(MonteCarloPayoff::Square),
        "absolute" => Ok(MonteCarloPayoff::Absolute),
        _ => {
            validate_identifier(
                "payoff_id",
                value,
                MAX_MONTE_CARLO_IDENTIFIER_BYTES,
            )?;

            Ok(MonteCarloPayoff::Custom(value.to_owned()))
        }
    }
}

fn parse_estimation_method(
    value: &str,
) -> BenchmarkResult<MonteCarloEstimationMethod> {
    match value {
        "canonical_amplitude_estimation" => Ok(
            MonteCarloEstimationMethod::CanonicalAmplitudeEstimation,
        ),

        "iterative_amplitude_estimation" => Ok(
            MonteCarloEstimationMethod::IterativeAmplitudeEstimation,
        ),

        "maximum_likelihood_amplitude_estimation" => Ok(
            MonteCarloEstimationMethod::MaximumLikelihoodAmplitudeEstimation,
        ),

        "classical_monte_carlo" => {
            Ok(MonteCarloEstimationMethod::ClassicalMonteCarlo)
        }

        _ => Err(BenchmarkError::InvalidConfiguration {
            field: "estimation_method".to_owned(),
            reason: "unsupported Monte Carlo estimation method".to_owned(),
        }),
    }
}

fn parse_reference_kind(value: &str) -> BenchmarkResult<ReferenceKind> {
    match value {
        "analytic" => Ok(ReferenceKind::Analytic),
        "exact_classical" => Ok(ReferenceKind::ExactClassical),
        "high_precision_numerical" => {
            Ok(ReferenceKind::HighPrecisionNumerical)
        }
        "external_reference" => Ok(ReferenceKind::ExternalReference),
        "unavailable" => Ok(ReferenceKind::Unavailable),
        _ => Err(BenchmarkError::InvalidConfiguration {
            field: "reference_kind".to_owned(),
            reason: "unsupported Monte Carlo reference kind".to_owned(),
        }),
    }
}

fn parse_success_criterion(
    value: &str,
) -> BenchmarkResult<MonteCarloSuccessCriterion> {
    match value {
        "absolute_error" => Ok(MonteCarloSuccessCriterion::AbsoluteError),
        "relative_error" => Ok(MonteCarloSuccessCriterion::RelativeError),
        "absolute_and_relative_error" => {
            Ok(MonteCarloSuccessCriterion::AbsoluteAndRelativeError)
        }
        "none" => Ok(MonteCarloSuccessCriterion::None),
        _ => Err(BenchmarkError::InvalidConfiguration {
            field: "success_criterion".to_owned(),
            reason: "unsupported Monte Carlo success criterion".to_owned(),
        }),
    }
}

fn parse_metrics(value: &str) -> BenchmarkResult<Vec<MonteCarloMetric>> {
    if value.is_empty() {
        return Err(BenchmarkError::InvalidConfiguration {
            field: "metrics".to_owned(),
            reason: "metric list cannot be empty".to_owned(),
        });
    }

    let mut metrics = Vec::new();

    for token in value.split(',') {
        let metric = match token {
            "estimate" => MonteCarloMetric::Estimate,
            "reference_value" => MonteCarloMetric::ReferenceValue,
            "absolute_error" => MonteCarloMetric::AbsoluteError,
            "relative_error" => MonteCarloMetric::RelativeError,
            "success" => MonteCarloMetric::Success,
            "target_precision" => MonteCarloMetric::TargetPrecision,
            "evaluation_qubits" => MonteCarloMetric::EvaluationQubits,
            "logical_qubits" => MonteCarloMetric::LogicalQubits,
            "oracle_applications" => MonteCarloMetric::OracleApplications,
            "amplification_applications" => {
                MonteCarloMetric::AmplificationApplications
            }
            "circuit_executions" => MonteCarloMetric::CircuitExecutions,
            "shots" => MonteCarloMetric::Shots,
            "circuit_depth" => MonteCarloMetric::CircuitDepth,
            "gate_count" => MonteCarloMetric::GateCount,
            "two_qubit_gate_count" => {
                MonteCarloMetric::TwoQubitGateCount
            }
            "quantum_execution_time" => {
                MonteCarloMetric::QuantumExecutionTime
            }
            "classical_processing_time" => {
                MonteCarloMetric::ClassicalProcessingTime
            }
            "total_time" => MonteCarloMetric::TotalTime,
            "time_to_solution" => MonteCarloMetric::TimeToSolution,
            "classical_samples" => MonteCarloMetric::ClassicalSamples,
            "quantum_speedup" => MonteCarloMetric::QuantumSpeedup,
            _ => {
                return Err(BenchmarkError::InvalidConfiguration {
                    field: "metrics".to_owned(),
                    reason: "unknown Monte Carlo metric".to_owned(),
                });
            }
        };

        metrics.push(metric);
    }

    validate_metric_count(metrics.len())?;

    Ok(deduplicate_metrics(metrics))
}

fn parse_f64_parameter(
    field: &str,
    value: &str,
) -> BenchmarkResult<f64> {
    if value.len() > MAX_MONTE_CARLO_VALUE_BYTES {
        return Err(BenchmarkError::InvalidRange {
            field: field.to_owned(),
            value: value.len().to_string(),
            minimum: Some("1".to_owned()),
            maximum: Some(MAX_MONTE_CARLO_VALUE_BYTES.to_string()),
        });
    }

    value.parse::<f64>().map_err(|_| {
        BenchmarkError::InvalidConfiguration {
            field: field.to_owned(),
            reason: "expected a finite decimal floating-point value"
                .to_owned(),
        }
    })
}

fn parse_usize_parameter(
    field: &str,
    value: &str,
) -> BenchmarkResult<usize> {
    if value.len() > MAX_MONTE_CARLO_VALUE_BYTES {
        return Err(BenchmarkError::InvalidRange {
            field: field.to_owned(),
            value: value.len().to_string(),
            minimum: Some("1".to_owned()),
            maximum: Some(MAX_MONTE_CARLO_VALUE_BYTES.to_string()),
        });
    }

    value.parse::<usize>().map_err(|_| {
        BenchmarkError::InvalidConfiguration {
            field: field.to_owned(),
            reason: "expected an unsigned integer".to_owned(),
        }
    })
}

// =============================================================================
// Validation
// =============================================================================

fn validate_precision(value: f64) -> BenchmarkResult<()> {
    if !value.is_finite()
        || value <= 0.0
        || value > MAX_MONTE_CARLO_PRECISION
        || value < MIN_MONTE_CARLO_PRECISION
    {
        return Err(BenchmarkError::InvalidRange {
            field: "target_precision".to_owned(),
            value: value.to_string(),
            minimum: Some(MIN_MONTE_CARLO_PRECISION.to_string()),
            maximum: Some(MAX_MONTE_CARLO_PRECISION.to_string()),
        });
    }

    Ok(())
}

fn validate_probability(
    field: &str,
    value: f64,
) -> BenchmarkResult<()> {
    if !value.is_finite() || !(0.0..=1.0).contains(&value) {
        return Err(BenchmarkError::InvalidRange {
            field: field.to_owned(),
            value: value.to_string(),
            minimum: Some("0".to_owned()),
            maximum: Some("1".to_owned()),
        });
    }

    Ok(())
}

fn validate_finite_bounded(
    field: &str,
    value: f64,
    maximum_absolute_value: f64,
) -> BenchmarkResult<()> {
    if !value.is_finite() || value.abs() > maximum_absolute_value {
        return Err(BenchmarkError::InvalidRange {
            field: field.to_owned(),
            value: value.to_string(),
            minimum: Some(
                (-maximum_absolute_value).to_string(),
            ),
            maximum: Some(maximum_absolute_value.to_string()),
        });
    }

    Ok(())
}

fn validate_identifier(
    field: &str,
    value: &str,
    maximum: usize,
) -> BenchmarkResult<()> {
    if value.is_empty() {
        return Err(BenchmarkError::InvalidIdentifier {
            field: field.to_owned(),
            value: value.to_owned(),
        });
    }

    if value.len() > maximum {
        return Err(BenchmarkError::InvalidRange {
            field: field.to_owned(),
            value: value.len().to_string(),
            minimum: Some("1".to_owned()),
            maximum: Some(maximum.to_string()),
        });
    }

    let bytes = value.as_bytes();

    if !bytes[0].is_ascii_lowercase() {
        return Err(BenchmarkError::InvalidIdentifier {
            field: field.to_owned(),
            value: value.to_owned(),
        });
    }

    for byte in bytes.iter().copied() {
        if !(byte.is_ascii_lowercase()
            || byte.is_ascii_digit()
            || byte == b'_'
            || byte == b'-')
        {
            return Err(BenchmarkError::InvalidIdentifier {
                field: field.to_owned(),
                value: value.to_owned(),
            });
        }
    }

    Ok(())
}

fn validate_success_criterion_requirements(
    configuration: &MonteCarloBenchmarkConfig,
) -> BenchmarkResult<()> {
    match configuration.success_criterion {
        MonteCarloSuccessCriterion::None => Ok(()),

        MonteCarloSuccessCriterion::AbsoluteError
        | MonteCarloSuccessCriterion::RelativeError
        | MonteCarloSuccessCriterion::AbsoluteAndRelativeError => {
            if configuration.reference_value.is_none() {
                return Err(BenchmarkError::InconsistentConfiguration {
                    first: "success_criterion".to_owned(),
                    second: "reference_value".to_owned(),
                    reason:
                        "a quantitative success criterion requires a reference value"
                            .to_owned(),
                });
            }

            Ok(())
        }
    }
}

fn add_parameter(
    workload: &mut ApplicationWorkload,
    name: &str,
    value: impl Into<String>,
) -> BenchmarkResult<()> {
    let parameter =
        ApplicationParameter::new(name, value.into())
            .map_err(BenchmarkError::from)?;

    workload
        .add_parameter(parameter)
        .map_err(BenchmarkError::from)
}

fn format_float(value: f64) -> String {
    // `Debug` formatting is deterministic and preserves enough information for
    // provenance without introducing locale-dependent formatting.
    format!("{value:?}")
}

fn numerical_overflow(operation: &str) -> BenchmarkError {
    BenchmarkError::NumericalOverflow {
        operation: operation.to_owned(),
        value: None,
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn default_request() -> ApplicationGenerationRequest {
        let instance =
            WorkloadId::new("monte_carlo_test").expect("valid instance id");

        ApplicationGenerationRequest::new(
            MONTE_CARLO_APPLICATION_ID,
            instance,
            8,
            42,
        )
        .expect("valid generation request")
    }

    #[test]
    fn canonical_generator_descriptor_is_valid() {
        let generator =
            MonteCarloBenchmarkGenerator::new().expect("valid generator");

        assert_eq!(
            generator.descriptor().generator_id(),
            MONTE_CARLO_BENCHMARK_ID
        );

        assert_eq!(
            generator.descriptor().application_id(),
            MONTE_CARLO_APPLICATION_ID
        );

        assert!(
            generator
                .descriptor()
                .supports(ApplicationGeneratorCapability::Deterministic)
        );
    }

    #[test]
    fn default_configuration_is_valid() {
        let configuration = MonteCarloBenchmarkConfig::new(
            MonteCarloDistribution::Uniform,
            MonteCarloPayoff::Identity,
            8,
            0.01,
        )
        .expect("valid configuration");

        assert_eq!(configuration.problem_size, 8);
        assert_eq!(
            configuration.estimation_method,
            MonteCarloEstimationMethod::CanonicalAmplitudeEstimation
        );
    }

    #[test]
    fn invalid_precision_is_rejected() {
        assert!(
            MonteCarloBenchmarkConfig::new(
                MonteCarloDistribution::Uniform,
                MonteCarloPayoff::Identity,
                8,
                0.0,
            )
            .is_err()
        );

        assert!(
            MonteCarloBenchmarkConfig::new(
                MonteCarloDistribution::Uniform,
                MonteCarloPayoff::Identity,
                8,
                f64::NAN,
            )
            .is_err()
        );
    }

    #[test]
    fn invalid_amplitude_is_rejected() {
        let configuration = MonteCarloBenchmarkConfig::new(
            MonteCarloDistribution::Uniform,
            MonteCarloPayoff::Identity,
            8,
            0.01,
        )
        .expect("valid configuration");

        assert!(
            configuration
                .clone()
                .with_target_amplitude(1.1)
                .is_err()
        );

        assert!(
            configuration
                .with_target_amplitude(f64::NAN)
                .is_err()
        );
    }

    #[test]
    fn amplitude_scaling_is_reversible() {
        let configuration = MonteCarloBenchmarkConfig::new(
            MonteCarloDistribution::Uniform,
            MonteCarloPayoff::Identity,
            8,
            0.01,
        )
        .expect("valid configuration")
        .with_scaling(2.0, 4.0)
        .expect("valid scaling");

        let application_value =
            configuration
                .application_value_from_amplitude(0.25)
                .expect("valid application value");

        assert!((application_value - 3.0).abs() < 1.0e-12);

        let recovered =
            configuration
                .amplitude_from_application_value(application_value)
                .expect("valid amplitude");

        assert!((recovered - 0.25).abs() < 1.0e-12);
    }

    #[test]
    fn reference_is_required_for_quantitative_success() {
        let configuration = MonteCarloBenchmarkConfig::new(
            MonteCarloDistribution::Uniform,
            MonteCarloPayoff::Identity,
            8,
            0.01,
        )
        .expect("valid configuration");

        assert!(configuration.validate().is_err());
    }

    #[test]
    fn reference_makes_quantitative_success_valid() {
        let configuration = MonteCarloBenchmarkConfig::new(
            MonteCarloDistribution::Uniform,
            MonteCarloPayoff::Identity,
            8,
            0.01,
        )
        .expect("valid configuration")
        .with_reference(0.5, ReferenceKind::Analytic)
        .expect("valid reference");

        assert!(configuration.validate().is_ok());
    }

    #[test]
    fn classical_mode_requires_no_quantum_execution() {
        let configuration = MonteCarloBenchmarkConfig::new(
            MonteCarloDistribution::Uniform,
            MonteCarloPayoff::Identity,
            8,
            0.01,
        )
        .expect("valid configuration")
        .with_estimation_method(
            MonteCarloEstimationMethod::ClassicalMonteCarlo,
        );

        assert!(
            !configuration
                .estimation_method
                .requires_quantum_execution()
        );
    }

    #[test]
    fn workload_generation_is_deterministic() {
        let generator =
            MonteCarloBenchmarkGenerator::new().expect("valid generator");

        let request = default_request()
            .with_parameter(
                ApplicationParameter::new(
                    "distribution_id",
                    "uniform",
                )
                .expect("valid parameter"),
            )
            .expect("parameter accepted")
            .with_parameter(
                ApplicationParameter::new(
                    "payoff_id",
                    "identity",
                )
                .expect("valid parameter"),
            )
            .expect("parameter accepted")
            .with_parameter(
                ApplicationParameter::new(
                    "reference_kind",
                    "analytic",
                )
                .expect("valid parameter"),
            )
            .expect("parameter accepted")
            .with_parameter(
                ApplicationParameter::new(
                    "reference_value",
                    "0.5",
                )
                .expect("valid parameter"),
            )
            .expect("parameter accepted");

        let first = generator
            .generate(&request)
            .expect("first generation");

        let second = generator
            .generate(&request)
            .expect("second generation");

        assert_eq!(
            first.workload().application_id(),
            second.workload().application_id()
        );

        assert_eq!(
            first.workload().instance_id(),
            second.workload().instance_id()
        );

        assert_eq!(
            first.workload().problem_size(),
            second.workload().problem_size()
        );

        assert_eq!(
            first.metadata(),
            second.metadata()
        );
    }

    #[test]
    fn unknown_parameter_is_rejected() {
        let generator =
            MonteCarloBenchmarkGenerator::new().expect("valid generator");

        let request = default_request()
            .with_parameter(
                ApplicationParameter::new(
                    "not_a_real_monte_carlo_parameter",
                    "1",
                )
                .expect("valid generic parameter"),
            )
            .expect("parameter accepted");

        assert!(generator.generate(&request).is_err());
    }

    #[test]
    fn generated_workload_contains_core_semantics() {
        let generator =
            MonteCarloBenchmarkGenerator::new().expect("valid generator");

        let request = default_request()
            .with_parameter(
                ApplicationParameter::new(
                    "reference_kind",
                    "analytic",
                )
                .expect("valid parameter"),
            )
            .expect("parameter accepted")
            .with_parameter(
                ApplicationParameter::new(
                    "reference_value",
                    "0.5",
                )
                .expect("valid parameter"),
            )
            .expect("parameter accepted");

        let generation =
            generator.generate(&request).expect("generation succeeds");

        assert_eq!(
            generation.workload().application_id(),
            MONTE_CARLO_APPLICATION_ID
        );

        assert_eq!(
            generation.workload().problem_size(),
            request.problem_size()
        );
    }

    #[test]
    fn evaluation_qubit_estimate_is_bounded() {
        let configuration = MonteCarloBenchmarkConfig::new(
            MonteCarloDistribution::Uniform,
            MonteCarloPayoff::Identity,
            8,
            0.01,
        )
        .expect("valid configuration");

        let qubits = configuration
            .estimated_evaluation_qubits()
            .expect("valid estimate");

        assert!(qubits > 0);
        assert!(qubits <= MAX_ESTIMATION_QUBITS);
    }

    #[test]
    fn oracle_estimate_is_checked() {
        let configuration = MonteCarloBenchmarkConfig::new(
            MonteCarloDistribution::Uniform,
            MonteCarloPayoff::Identity,
            8,
            0.01,
        )
        .expect("valid configuration");

        let applications = configuration
            .estimated_oracle_applications()
            .expect("valid estimate");

        assert!(applications > 0);
    }

    #[test]
    fn metric_list_is_deduplicated() {
        let configuration = MonteCarloBenchmarkConfig::new(
            MonteCarloDistribution::Uniform,
            MonteCarloPayoff::Identity,
            8,
            0.01,
        )
        .expect("valid configuration")
        .with_metrics(vec![
            MonteCarloMetric::Estimate,
            MonteCarloMetric::Estimate,
            MonteCarloMetric::Success,
        ])
        .expect("valid metrics");

        assert_eq!(configuration.metrics.len(), 2);
    }
}