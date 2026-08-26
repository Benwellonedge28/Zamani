//! Zamani Quantum Benchmarking — VQE Application Benchmark
//!
//! Production-grade VQE benchmark definition and workload generator.
//!
//! # Architectural responsibility
//!
//! This module owns the benchmarking definition of the Variational Quantum
//! Eigensolver (VQE):
//!
//! - VQE benchmark identity and semantic version;
//! - VQE-specific benchmark configuration;
//! - validation of VQE benchmark parameters;
//! - reproducible VQE workload construction;
//! - application workload metadata;
//! - VQE resource/measurement requirements;
//! - explicit success criteria;
//! - deterministic instance identity;
//! - integration metadata for the canonical VQE algorithm;
//! - an optional circuit-construction boundary;
//! - safe conversion into the canonical benchmarking workload model.
//!
//! This module deliberately does NOT own:
//!
//! - the VQE optimizer;
//! - optimizer mathematics;
//! - Hamiltonian mathematics;
//! - ansatz mathematics;
//! - quantum execution;
//! - backend communication;
//! - Quantum IR semantics;
//! - transpilation;
//! - routing;
//! - scheduling;
//! - calibration;
//! - statistical estimation;
//! - reporting;
//! - persistence;
//! - hardware capability negotiation.
//!
//! Those responsibilities belong to:
//!
//! ```text
//! quantum::algorithms::vqe
//! quantum::ir
//! quantum::hardware
//! quantum::routing
//! quantum::scheduling
//! benchmarking::execution
//! benchmarking::statistics
//! benchmarking::metrics
//! benchmarking::reporting
//! ```
//!
//! # Architectural flow
//!
//! ```text
//! VqeBenchmarkConfig
//!        │
//!        ▼
//! VqeBenchmarkGenerator
//!        │
//!        ▼
//! ApplicationGenerationRequest
//!        │
//!        ▼
//! ApplicationWorkload
//!        │
//!        ├──────────────► optional Quantum IR circuit
//!        │
//!        ▼
//! BenchmarkExperiment
//!        │
//!        ▼
//! BenchmarkExecutor
//!        │
//!        ▼
//! BenchmarkObservationSet
//!        │
//!        ▼
//! VQE application analysis
//!        │
//!        ▼
//! BenchmarkResult
//! ```
//!
//! # Critical VQE boundary
//!
//! The repository already has the authoritative VQE algorithm implementation
//! under:
//!
//! `crate::quantum::algorithms::vqe`
//!
//! That implementation owns:
//!
//! ```text
//! VqeHamiltonian
//! VqeAnsatz
//! VqeProblem
//! VqeConfig
//! Vqe::solve()
//! ```
//!
//! This benchmark module MUST NOT recreate any of those types.
//!
//! Instead, the benchmark describes how VQE should be measured.
//!
//! A future execution adapter may use:
//!
//! ```text
//! VqeProblem
//!      │
//!      ▼
//! quantum::algorithms::vqe::Vqe
//!      │
//!      ▼
//! BenchmarkExecutor
//! ```
//!
//! # Hybrid workload model
//!
//! VQE is inherently a hybrid quantum/classical workload:
//!
//! ```text
//! classical parameters
//!        │
//!        ▼
//! parameterized ansatz
//!        │
//!        ▼
//! quantum execution
//!        │
//!        ▼
//! expectation value
//!        │
//!        ▼
//! classical optimizer
//!        │
//!        └───────────────┐
//!                        ▼
//!                 convergence/energy
//! ```
//!
//! Consequently this benchmark must never reduce VQE to only:
//!
//! `circuit -> execution time`.
//!
//! The benchmark must preserve:
//!
//! - quantum execution cost;
//! - number of objective evaluations;
//! - optimizer iterations;
//! - convergence status;
//! - final energy;
//! - best energy;
//! - energy error where a reference is available;
//! - parameter count;
//! - circuit resource requirements;
//! - total wall time;
//! - quantum execution time;
//! - classical optimization time;
//! - time-to-solution;
//! - measurement/shots cost;
//! - reproducibility metadata.
//!
//! # Reproducibility
//!
//! VQE benchmark generation is deterministic with respect to:
//!
//! - benchmark configuration;
//! - application ID;
//! - instance ID;
//! - problem size;
//! - Hamiltonian identifier;
//! - ansatz identifier;
//! - optimizer identifier;
//! - initial parameters;
//! - seed;
//! - generator revision.
//!
//! This module never uses:
//!
//! - system time;
//! - process ID;
//! - pointer addresses;
//! - thread IDs;
//! - hidden global RNG state;
//! - implicit entropy.
//!
//! # Security/resource model
//!
//! VQE benchmark configuration can eventually originate from the Zamani
//! language, CLI, configuration files, CI, or remote benchmark requests.
//!
//! Therefore this module:
//!
//! - validates all identifiers;
//! - validates all floating-point values;
//! - bounds parameter counts;
//! - bounds encoded parameter sizes;
//! - rejects zero qubit counts;
//! - rejects zero optimizer iterations;
//! - rejects zero shots;
//! - rejects invalid tolerances;
//! - rejects non-finite values;
//! - uses checked arithmetic;
//! - does not perform I/O;
//! - does not execute user code;
//! - does not allocate from unchecked sizes.
//!
//! Global benchmark limits remain owned by:
//!
//! `benchmarking::core::limits`
//!
//! while this file enforces VQE-specific semantic limits.
//!
//! # Rust compatibility
//!
//! Rust 1.97 / Rust 1.97.1.
//!
//! Rust 2021.
//!
//! No nightly features.
//! No unsafe code.
//! No additional dependencies.
//!
//! # Integration contract
//!
//! This file integrates with:
//!
//! ```text
//! benchmarking::generators::application
//! benchmarking::core::workload
//! quantum::algorithms::vqe
//! quantum::ir
//! benchmarking::core::config
//! ```
//!
//! It does NOT require modifications to those files.
//!
//! Later integration is expected in:
//!
//! ```text
//! applications/mod.rs
//! registry/builtin.rs
//! protocols/application execution/analysis layer
//! ```
//!
//! Those modules can consume this file without changing its public semantic
//! contract.

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

/// Stable machine-readable identifier for the VQE application benchmark.
pub const VQE_BENCHMARK_ID: &str = "vqe";

/// Stable application identifier.
///
/// This intentionally matches the benchmark identity because VQE is itself
/// the application family being measured.
pub const VQE_APPLICATION_ID: &str = "vqe";

/// Stable semantic version of the VQE benchmark definition.
///
/// This is independent of:
///
/// - Zamani package version;
/// - Quantum IR version;
/// - VQE algorithm version;
/// - backend version;
/// - compiler version.
pub const VQE_BENCHMARK_VERSION: u32 = 1;

/// Stable generator revision.
///
/// Increment this when generation semantics change while retaining the same
/// benchmark schema.
pub const VQE_GENERATOR_REVISION: u32 = 1;

/// Stable benchmark generator implementation version.
pub const VQE_GENERATOR_VERSION: &str = "1.0.0";

// =============================================================================
// Safety limits
// =============================================================================

/// Maximum number of VQE parameters represented directly by this benchmark
/// boundary.
///
/// The global benchmark limits remain authoritative for execution. This is
/// an additional semantic protection against pathological requests.
pub const MAX_VQE_PARAMETERS: usize = 16_384;

/// Maximum UTF-8 byte length of an encoded VQE identifier.
pub const MAX_VQE_IDENTIFIER_BYTES: usize = 128;

/// Maximum UTF-8 byte length of one encoded parameter value.
pub const MAX_VQE_PARAMETER_VALUE_BYTES: usize = 512;

/// Maximum UTF-8 byte length of the encoded Hamiltonian identifier.
pub const MAX_HAMILTONIAN_ID_BYTES: usize = 128;

/// Maximum UTF-8 byte length of the encoded ansatz identifier.
pub const MAX_ANSATZ_ID_BYTES: usize = 128;

/// Maximum UTF-8 byte length of the encoded optimizer identifier.
pub const MAX_OPTIMIZER_ID_BYTES: usize = 128;

/// Maximum allowed optimizer iterations at this application boundary.
pub const MAX_VQE_ITERATIONS: usize = 10_000_000;

/// Maximum allowed shots per objective evaluation.
pub const MAX_VQE_SHOTS: usize = 10_000_000;

// =============================================================================
// Benchmark semantics
// =============================================================================

/// How VQE success is evaluated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VqeSuccessCriterion {
    /// Require convergence according to the optimizer.
    Convergence,

    /// Require convergence and an absolute energy error below a threshold.
    ConvergenceAndEnergyError,

    /// Require only an absolute energy error below a threshold.
    EnergyError,

    /// Do not impose a scientific pass/fail condition.
    ///
    /// This is appropriate when benchmarking performance only.
    None,
}

impl VqeSuccessCriterion {
    /// Stable machine-readable identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Convergence => "convergence",
            Self::ConvergenceAndEnergyError => {
                "convergence_and_energy_error"
            }
            Self::EnergyError => "energy_error",
            Self::None => "none",
        }
    }
}

impl fmt::Display for VqeSuccessCriterion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// =============================================================================
// Benchmark metrics
// =============================================================================

/// Metric that the VQE benchmark may request from its analysis layer.
///
/// This enumeration is deliberately independent from the generic
/// benchmarking metric implementation. The metrics layer owns actual
/// numerical representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VqeMetric {
    /// Final variational energy.
    FinalEnergy,

    /// Best energy observed during the complete optimization.
    BestEnergy,

    /// Absolute difference between final energy and a supplied reference.
    AbsoluteEnergyError,

    /// Relative difference between final energy and a supplied reference.
    RelativeEnergyError,

    /// Whether the optimizer converged.
    Convergence,

    /// Number of objective evaluations.
    ObjectiveEvaluations,

    /// Number of optimizer iterations.
    OptimizerIterations,

    /// Number of optimizer steps accepted by the optimizer.
    OptimizerSteps,

    /// Number of logical quantum circuit executions.
    CircuitExecutions,

    /// Total number of measurement shots.
    Shots,

    /// Total quantum execution time.
    QuantumExecutionTime,

    /// Classical optimization time.
    ClassicalOptimizationTime,

    /// Total end-to-end wall-clock time.
    TotalTime,

    /// Time-to-solution.
    TimeToSolution,

    /// Number of variational parameters.
    ParameterCount,

    /// Logical qubit count.
    QubitCount,

    /// Total circuit depth.
    CircuitDepth,

    /// Total gate count.
    GateCount,

    /// Total two-qubit gate count.
    TwoQubitGateCount,

    /// Solution quality relative to an application-defined reference.
    SolutionQuality,
}

impl VqeMetric {
    /// Stable machine-readable identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FinalEnergy => "final_energy",
            Self::BestEnergy => "best_energy",
            Self::AbsoluteEnergyError => "absolute_energy_error",
            Self::RelativeEnergyError => "relative_energy_error",
            Self::Convergence => "convergence",
            Self::ObjectiveEvaluations => "objective_evaluations",
            Self::OptimizerIterations => "optimizer_iterations",
            Self::OptimizerSteps => "optimizer_steps",
            Self::CircuitExecutions => "circuit_executions",
            Self::Shots => "shots",
            Self::QuantumExecutionTime => "quantum_execution_time",
            Self::ClassicalOptimizationTime => {
                "classical_optimization_time"
            }
            Self::TotalTime => "total_time",
            Self::TimeToSolution => "time_to_solution",
            Self::ParameterCount => "parameter_count",
            Self::QubitCount => "qubit_count",
            Self::CircuitDepth => "circuit_depth",
            Self::GateCount => "gate_count",
            Self::TwoQubitGateCount => "two_qubit_gate_count",
            Self::SolutionQuality => "solution_quality",
        }
    }
}

impl fmt::Display for VqeMetric {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// =============================================================================
// VQE benchmark configuration
// =============================================================================

/// VQE-specific benchmark configuration.
///
/// This is intentionally separate from the generic `BenchmarkConfig`.
///
/// Generic execution concerns such as:
///
/// - backend;
/// - shots;
/// - compiler;
/// - routing;
/// - scheduling;
/// - confidence level;
/// - resource limits;
///
/// remain in `benchmarking::core::config::BenchmarkConfig`.
///
/// This structure contains only VQE semantic configuration.
#[derive(Debug, Clone, PartialEq)]
pub struct VqeBenchmarkConfig {
    /// Stable Hamiltonian/observable identifier.
    pub hamiltonian_id: String,

    /// Stable ansatz identifier.
    pub ansatz_id: String,

    /// Stable classical optimizer identifier.
    pub optimizer_id: String,

    /// Number of logical qubits.
    pub qubit_count: usize,

    /// Initial variational parameters.
    pub initial_parameters: Vec<f64>,

    /// Maximum classical optimizer iterations.
    pub max_iterations: usize,

    /// Energy convergence tolerance.
    pub energy_tolerance: f64,

    /// Parameter convergence tolerance.
    pub parameter_tolerance: f64,

    /// Optional reference ground-state energy.
    ///
    /// If present, the analysis layer can calculate energy error.
    pub reference_energy: Option<f64>,

    /// Scientific success criterion.
    pub success_criterion: VqeSuccessCriterion,

    /// Maximum allowed absolute energy error when the selected criterion
    /// requires one.
    pub maximum_energy_error: Option<f64>,

    /// Metrics requested by the benchmark.
    pub metrics: Vec<VqeMetric>,
}

impl VqeBenchmarkConfig {
    /// Creates a validated VQE benchmark configuration.
    pub fn new(
        hamiltonian_id: impl Into<String>,
        ansatz_id: impl Into<String>,
        optimizer_id: impl Into<String>,
        qubit_count: usize,
        initial_parameters: Vec<f64>,
    ) -> BenchmarkResult<Self> {
        let config = Self {
            hamiltonian_id: hamiltonian_id.into(),
            ansatz_id: ansatz_id.into(),
            optimizer_id: optimizer_id.into(),
            qubit_count,
            initial_parameters,
            max_iterations: 1_000,
            energy_tolerance: 1.0e-8,
            parameter_tolerance: 1.0e-8,
            reference_energy: None,
            success_criterion: VqeSuccessCriterion::Convergence,
            maximum_energy_error: None,
            metrics: default_metrics(),
        };

        config.validate()?;

        Ok(config)
    }

    /// Sets the optimizer iteration limit.
    pub fn with_max_iterations(
        mut self,
        max_iterations: usize,
    ) -> BenchmarkResult<Self> {
        self.max_iterations = max_iterations;
        self.validate()?;
        Ok(self)
    }

    /// Sets the energy convergence tolerance.
    pub fn with_energy_tolerance(
        mut self,
        tolerance: f64,
    ) -> BenchmarkResult<Self> {
        self.energy_tolerance = tolerance;
        self.validate()?;
        Ok(self)
    }

    /// Sets the parameter convergence tolerance.
    pub fn with_parameter_tolerance(
        mut self,
        tolerance: f64,
    ) -> BenchmarkResult<Self> {
        self.parameter_tolerance = tolerance;
        self.validate()?;
        Ok(self)
    }

    /// Supplies a reference ground-state energy.
    pub fn with_reference_energy(
        mut self,
        energy: f64,
    ) -> BenchmarkResult<Self> {
        self.reference_energy = Some(energy);
        self.validate()?;
        Ok(self)
    }

    /// Selects the scientific success criterion.
    pub fn with_success_criterion(
        mut self,
        criterion: VqeSuccessCriterion,
    ) -> BenchmarkResult<Self> {
        self.success_criterion = criterion;
        self.validate()?;
        Ok(self)
    }

    /// Sets the maximum acceptable absolute energy error.
    pub fn with_maximum_energy_error(
        mut self,
        error: f64,
    ) -> BenchmarkResult<Self> {
        self.maximum_energy_error = Some(error);
        self.validate()?;
        Ok(self)
    }

    /// Replaces the requested metric set.
    pub fn with_metrics(
        mut self,
        metrics: Vec<VqeMetric>,
    ) -> BenchmarkResult<Self> {
        self.metrics = canonicalize_metrics(metrics)?;
        self.validate()?;
        Ok(self)
    }

    /// Validates all VQE-specific semantics.
    pub fn validate(&self) -> BenchmarkResult<()> {
        validate_identifier(
            "hamiltonian_id",
            &self.hamiltonian_id,
            MAX_HAMILTONIAN_ID_BYTES,
        )?;

        validate_identifier(
            "ansatz_id",
            &self.ansatz_id,
            MAX_ANSATZ_ID_BYTES,
        )?;

        validate_identifier(
            "optimizer_id",
            &self.optimizer_id,
            MAX_OPTIMIZER_ID_BYTES,
        )?;

        if self.qubit_count == 0 {
            return Err(BenchmarkError::InvalidRange {
                field: "vqe.qubit_count".to_owned(),
                value: "0".to_owned(),
                minimum: Some("1".to_owned()),
                maximum: None,
            });
        }

        if self.initial_parameters.is_empty() {
            return Err(BenchmarkError::InvalidConfiguration {
                field: "vqe.initial_parameters".to_owned(),
                reason:
                    "VQE requires at least one variational parameter"
                        .to_owned(),
            });
        }

        if self.initial_parameters.len() > MAX_VQE_PARAMETERS {
            return Err(BenchmarkError::ResourceLimitExceeded {
                resource: "vqe.parameters".to_owned(),
                requested: self.initial_parameters.len() as u64,
                maximum: MAX_VQE_PARAMETERS as u64,
            });
        }

        for (index, value) in self.initial_parameters.iter().enumerate() {
            if !value.is_finite() {
                return Err(BenchmarkError::InvalidConfiguration {
                    field: format!(
                        "vqe.initial_parameters[{index}]"
                    ),
                    reason: "parameter must be finite".to_owned(),
                });
            }
        }

        if self.max_iterations == 0 {
            return Err(BenchmarkError::InvalidRange {
                field: "vqe.max_iterations".to_owned(),
                value: "0".to_owned(),
                minimum: Some("1".to_owned()),
                maximum: Some(MAX_VQE_ITERATIONS.to_string()),
            });
        }

        if self.max_iterations > MAX_VQE_ITERATIONS {
            return Err(BenchmarkError::ResourceLimitExceeded {
                resource: "vqe.max_iterations".to_owned(),
                requested: self.max_iterations as u64,
                maximum: MAX_VQE_ITERATIONS as u64,
            });
        }

        validate_positive_finite(
            "vqe.energy_tolerance",
            self.energy_tolerance,
        )?;

        validate_positive_finite(
            "vqe.parameter_tolerance",
            self.parameter_tolerance,
        )?;

        if let Some(reference_energy) = self.reference_energy {
            if !reference_energy.is_finite() {
                return Err(BenchmarkError::InvalidConfiguration {
                    field: "vqe.reference_energy".to_owned(),
                    reason: "reference energy must be finite".to_owned(),
                });
            }
        }

        if let Some(maximum_error) = self.maximum_energy_error {
            validate_positive_finite(
                "vqe.maximum_energy_error",
                maximum_error,
            )?;
        }

        match self.success_criterion {
            VqeSuccessCriterion::EnergyError
            | VqeSuccessCriterion::ConvergenceAndEnergyError => {
                if self.reference_energy.is_none() {
                    return Err(BenchmarkError::InvalidConfiguration {
                        field: "vqe.reference_energy".to_owned(),
                        reason:
                            "an energy-error success criterion requires a reference energy"
                                .to_owned(),
                    });
                }

                if self.maximum_energy_error.is_none() {
                    return Err(BenchmarkError::InvalidConfiguration {
                        field:
                            "vqe.maximum_energy_error"
                                .to_owned(),
                        reason:
                            "an energy-error success criterion requires a maximum energy error"
                                .to_owned(),
                    });
                }
            }

            VqeSuccessCriterion::Convergence
            | VqeSuccessCriterion::None => {}
        }

        canonicalize_metrics(self.metrics.clone())?;

        Ok(())
    }

    /// Returns whether a reference energy is available.
    #[must_use]
    pub fn has_reference_energy(&self) -> bool {
        self.reference_energy.is_some()
    }

    /// Returns whether the selected success criterion requires convergence.
    #[must_use]
    pub const fn requires_convergence(&self) -> bool {
        matches!(
            self.success_criterion,
            VqeSuccessCriterion::Convergence
                | VqeSuccessCriterion::ConvergenceAndEnergyError
        )
    }

    /// Returns whether the selected success criterion requires an energy
    /// reference.
    #[must_use]
    pub const fn requires_energy_reference(&self) -> bool {
        matches!(
            self.success_criterion,
            VqeSuccessCriterion::EnergyError
                | VqeSuccessCriterion::ConvergenceAndEnergyError
        )
    }
}

// =============================================================================
// VQE workload descriptor
// =============================================================================

/// Stable semantic description of one VQE benchmark workload.
///
/// This structure is deliberately independent of the execution backend.
///
/// It can therefore describe the same VQE instance before and after:
///
/// - compilation;
/// - routing;
/// - scheduling;
/// - hardware lowering.
///
/// It is also suitable for provenance.
#[derive(Debug, Clone, PartialEq)]
pub struct VqeWorkloadDescriptor {
    /// Hamiltonian identifier.
    pub hamiltonian_id: String,

    /// Ansatz identifier.
    pub ansatz_id: String,

    /// Optimizer identifier.
    pub optimizer_id: String,

    /// Logical qubit count.
    pub qubit_count: usize,

    /// Variational parameter count.
    pub parameter_count: usize,

    /// Maximum optimizer iterations.
    pub max_iterations: usize,

    /// Initial parameters.
    pub initial_parameters: Vec<f64>,

    /// Optional reference energy.
    pub reference_energy: Option<f64>,

    /// Success criterion.
    pub success_criterion: VqeSuccessCriterion,
}

impl VqeWorkloadDescriptor {
    /// Creates a workload descriptor from validated VQE configuration.
    pub fn from_config(
        config: &VqeBenchmarkConfig,
    ) -> BenchmarkResult<Self> {
        config.validate()?;

        Ok(Self {
            hamiltonian_id: config.hamiltonian_id.clone(),
            ansatz_id: config.ansatz_id.clone(),
            optimizer_id: config.optimizer_id.clone(),
            qubit_count: config.qubit_count,
            parameter_count: config.initial_parameters.len(),
            max_iterations: config.max_iterations,
            initial_parameters: config.initial_parameters.clone(),
            reference_energy: config.reference_energy,
            success_criterion: config.success_criterion,
        })
    }
}

// =============================================================================
// Optional circuit construction boundary
// =============================================================================

/// Optional VQE circuit builder.
///
/// This is the only circuit-construction boundary exposed by this benchmark.
///
/// The implementation may adapt the canonical VQE ansatz from
/// `quantum::algorithms::vqe` and return the canonical `QuantumCircuit`.
///
/// The benchmark itself never executes the circuit.
pub trait VqeCircuitBuilder: Send + Sync {
    /// Builds the logical Quantum IR circuit for one VQE parameter vector.
    ///
    /// Implementations must:
    ///
    /// - validate parameter dimensionality;
    /// - validate all parameter values;
    /// - return canonical `QuantumCircuit`;
    /// - avoid backend communication;
    /// - avoid routing;
    /// - avoid scheduling;
    /// - avoid measurement execution.
    fn build(
        &self,
        parameters: &[f64],
    ) -> BenchmarkResult<crate::quantum::ir::QuantumCircuit>;

    /// Returns the logical qubit count.
    fn qubit_count(&self) -> usize;

    /// Returns the number of variational parameters.
    fn parameter_count(&self) -> usize;
}

// =============================================================================
// VQE benchmark generator
// =============================================================================

/// Production VQE application benchmark generator.
///
/// The generator is stateless and therefore safe to share between benchmark
/// registry instances.
///
/// It generates only canonical application workloads. It never executes VQE.
#[derive(Debug, Clone)]
pub struct VqeBenchmarkGenerator {
    descriptor: ApplicationGeneratorDescriptor,
}

impl Default for VqeBenchmarkGenerator {
    fn default() -> Self {
        Self::new()
            .expect("static VQE benchmark generator descriptor must be valid")
    }
}

impl VqeBenchmarkGenerator {
    /// Creates the canonical VQE benchmark generator.
    pub fn new() -> BenchmarkResult<Self> {
        let descriptor =
            ApplicationGeneratorDescriptor::new(
                VQE_BENCHMARK_ID,
                VQE_APPLICATION_ID,
                VQE_GENERATOR_VERSION,
                "Production VQE application benchmark workload generator",
            )?
            .with_capabilities([
                ApplicationGeneratorCapability::Hybrid,
                ApplicationGeneratorCapability::Deterministic,
                ApplicationGeneratorCapability::BatchGeneration,
                ApplicationGeneratorCapability::ScalableProblemSize,
                ApplicationGeneratorCapability::Parameterized,
                ApplicationGeneratorCapability::ExactSmallInstanceReference,
                ApplicationGeneratorCapability::ClassicallyVerifiable,
                ApplicationGeneratorCapability::HardwareExecutable,
            ]);

        Ok(Self { descriptor })
    }

    /// Returns the canonical generator descriptor.
    #[must_use]
    pub fn descriptor_ref(
        &self,
    ) -> &ApplicationGeneratorDescriptor {
        &self.descriptor
    }

    /// Builds a benchmark generation request from a VQE-specific configuration.
    ///
    /// The returned request contains only generation semantics. Execution
    /// parameters such as shots, backend, timeout, compiler, routing and
    /// scheduling are intentionally absent.
    pub fn request(
        config: &VqeBenchmarkConfig,
        instance_id: WorkloadId,
        seed: u64,
    ) -> BenchmarkResult<ApplicationGenerationRequest> {
        config.validate()?;

        let mut request =
            ApplicationGenerationRequest::new(
                VQE_APPLICATION_ID,
                instance_id,
                config.qubit_count,
                seed,
            )?
            .with_generator_revision(
                VQE_GENERATOR_REVISION,
            );

        request =
            request.with_parameters(
                encoded_parameters(config)?,
            )?;

        Ok(request)
    }

    /// Generates a single VQE application workload from the supplied
    /// configuration and instance identity.
    pub fn generate_from_config(
        &self,
        config: &VqeBenchmarkConfig,
        instance_id: WorkloadId,
        seed: u64,
    ) -> BenchmarkResult<ApplicationWorkload> {
        let request =
            Self::request(config, instance_id, seed)?;

        self.generate_workload(&request)
    }

    /// Generates a single VQE application workload with an optional logical
    /// circuit.
    ///
    /// The circuit builder is deliberately supplied by the caller so this
    /// benchmark module does not own a second VQE ansatz abstraction.
    pub fn generate_with_circuit_builder(
        &self,
        config: &VqeBenchmarkConfig,
        instance_id: WorkloadId,
        seed: u64,
        builder: &dyn VqeCircuitBuilder,
    ) -> BenchmarkResult<ApplicationWorkload> {
        config.validate()?;

        if builder.qubit_count() != config.qubit_count {
            return Err(BenchmarkError::InconsistentConfiguration {
                first: "vqe.config.qubit_count".to_owned(),
                second: "vqe.circuit_builder.qubit_count".to_owned(),
                reason:
                    "VQE benchmark and circuit builder must use the same logical qubit count"
                        .to_owned(),
            });
        }

        if builder.parameter_count()
            != config.initial_parameters.len()
        {
            return Err(BenchmarkError::InconsistentConfiguration {
                first:
                    "vqe.config.parameter_count"
                        .to_owned(),
                second:
                    "vqe.circuit_builder.parameter_count"
                        .to_owned(),
                reason:
                    "VQE benchmark and circuit builder must expose the same parameter count"
                        .to_owned(),
            });
        }

        let mut workload =
            self.generate_from_config(
                config,
                instance_id,
                seed,
            )?;

        let circuit =
            builder.build(
                &config.initial_parameters,
            )?;

        let circuit_workload =
            super::super::core::workload::CircuitWorkload::new(
                format!(
                    "vqe_{}_initial",
                    workload.instance_id()
                ),
                circuit,
            )
            .map_err(|error| {
                BenchmarkError::InvalidWorkload {
                    workload:
                        workload
                            .instance_id()
                            .to_string(),
                    reason: error.to_string(),
                }
            })?;

        workload =
            workload.with_circuit(
                circuit_workload,
            );

        Ok(workload)
    }

    /// Returns the stable set of default VQE metrics.
    #[must_use]
    pub fn default_metrics() -> Vec<VqeMetric> {
        default_metrics()
    }

    /// Returns whether a VQE configuration is scientifically valid for the
    /// selected success criterion.
    pub fn validate_success_criterion(
        config: &VqeBenchmarkConfig,
    ) -> BenchmarkResult<()> {
        config.validate()
    }
}

impl ApplicationBenchmarkGenerator for VqeBenchmarkGenerator {
    fn descriptor(
        &self,
    ) -> &ApplicationGeneratorDescriptor {
        &self.descriptor
    }

    fn validate(
        &self,
        request: &ApplicationGenerationRequest,
    ) -> BenchmarkResult<()> {
        request.validate()?;

        if request.application_id()
            != VQE_APPLICATION_ID
        {
            return Err(
                BenchmarkError::InconsistentConfiguration {
                    first:
                        "request.application_id"
                            .to_owned(),
                    second:
                        "vqe.application_id"
                            .to_owned(),
                    reason:
                        "VQE requests must use the canonical vqe application identifier"
                            .to_owned(),
                },
            );
        }

        if request.metadata().generator_revision()
            != VQE_GENERATOR_REVISION
        {
            return Err(
                BenchmarkError::ReproducibilityFailure {
                    component:
                        "vqe.generator_revision"
                            .to_owned(),
                    expected:
                        VQE_GENERATOR_REVISION
                            .to_string(),
                    actual:
                        request
                            .metadata()
                            .generator_revision()
                            .to_string(),
                },
            );
        }

        if request.parameters().is_empty() {
            return Err(
                BenchmarkError::InvalidConfiguration {
                    field:
                        "vqe.parameters"
                            .to_owned(),
                    reason:
                        "VQE workload requests must contain benchmark parameters"
                            .to_owned(),
                },
            );
        }

        Ok(())
    }

    fn generate_workload(
        &self,
        request: &ApplicationGenerationRequest,
    ) -> BenchmarkResult<ApplicationWorkload> {
        self.validate(request)?;

        let parameters =
            decode_and_validate_parameters(
                request.parameters(),
            )?;

        let qubit_count =
            parse_required_usize(
                &parameters,
                "qubit_count",
            )?;

        let parameter_count =
            parse_required_usize(
                &parameters,
                "parameter_count",
            )?;

        let max_iterations =
            parse_required_usize(
                &parameters,
                "max_iterations",
            )?;

        if qubit_count != request.problem_size() {
            return Err(
                BenchmarkError::InconsistentConfiguration {
                    first:
                        "vqe.qubit_count"
                            .to_owned(),
                    second:
                        "request.problem_size"
                            .to_owned(),
                    reason:
                        "VQE qubit count must equal the application workload problem size"
                            .to_owned(),
                },
            );
        }

        if parameter_count == 0 {
            return Err(
                BenchmarkError::InvalidConfiguration {
                    field:
                        "vqe.parameter_count"
                            .to_owned(),
                    reason:
                        "VQE parameter count must be greater than zero"
                            .to_owned(),
                },
            );
        }

        if parameter_count
            > MAX_VQE_PARAMETERS
        {
            return Err(
                BenchmarkError::ResourceLimitExceeded {
                    resource:
                        "vqe.parameters"
                            .to_owned(),
                    requested:
                        parameter_count
                            as u64,
                    maximum:
                        MAX_VQE_PARAMETERS
                            as u64,
                },
            );
        }

        if max_iterations == 0
            || max_iterations
                > MAX_VQE_ITERATIONS
        {
            return Err(
                BenchmarkError::ResourceLimitExceeded {
                    resource:
                        "vqe.max_iterations"
                            .to_owned(),
                    requested:
                        max_iterations
                            as u64,
                    maximum:
                        MAX_VQE_ITERATIONS
                            as u64,
                },
            );
        }

        let instance =
            WorkloadId::new(
                request
                    .instance_id()
                    .as_str()
                    .to_owned(),
            )
            .map_err(|error| {
                BenchmarkError::InvalidWorkload {
                    workload:
                        VQE_APPLICATION_ID
                            .to_owned(),
                    reason:
                        error.to_string(),
                }
            })?;

        let mut workload =
            ApplicationWorkload::new(
                VQE_APPLICATION_ID,
                instance,
                request.problem_size(),
            )
            .map_err(|error| {
                BenchmarkError::InvalidWorkload {
                    workload:
                        VQE_APPLICATION_ID
                            .to_owned(),
                    reason:
                        error.to_string(),
                }
            })?;

        for parameter in
            request.parameters()
        {
            workload
                .add_parameter(
                    parameter.clone(),
                )
                .map_err(|error| {
                    BenchmarkError::InvalidWorkload {
                        workload:
                            VQE_APPLICATION_ID
                                .to_owned(),
                        reason:
                            error.to_string(),
                    }
                })?;
        }

        Ok(workload)
    }
}

// =============================================================================
// Encoded VQE parameters
// =============================================================================

/// Encodes VQE-specific configuration into the canonical bounded application
/// parameter representation.
///
/// The values are textual intentionally because the canonical workload model
/// must not impose a VQE-specific numerical representation.
fn encoded_parameters(
    config: &VqeBenchmarkConfig,
) -> BenchmarkResult<Vec<ApplicationParameter>> {
    config.validate()?;

    let mut parameters =
        Vec::with_capacity(
            config
                .initial_parameters
                .len()
                .saturating_add(8),
        );

    parameters.push(
        ApplicationParameter::new(
            "hamiltonian_id",
            config.hamiltonian_id.clone(),
        )
        .map_err(workload_error)?,
    );

    parameters.push(
        ApplicationParameter::new(
            "ansatz_id",
            config.ansatz_id.clone(),
        )
        .map_err(workload_error)?,
    );

    parameters.push(
        ApplicationParameter::new(
            "optimizer_id",
            config.optimizer_id.clone(),
        )
        .map_err(workload_error)?,
    );

    parameters.push(
        ApplicationParameter::new(
            "qubit_count",
            config.qubit_count.to_string(),
        )
        .map_err(workload_error)?,
    );

    parameters.push(
        ApplicationParameter::new(
            "parameter_count",
            config.initial_parameters.len().to_string(),
        )
        .map_err(workload_error)?,
    );

    parameters.push(
        ApplicationParameter::new(
            "max_iterations",
            config.max_iterations.to_string(),
        )
        .map_err(workload_error)?,
    );

    parameters.push(
        ApplicationParameter::new(
            "energy_tolerance",
            format_finite_float(
                config.energy_tolerance,
            )?,
        )
        .map_err(workload_error)?,
    );

    parameters.push(
        ApplicationParameter::new(
            "parameter_tolerance",
            format_finite_float(
                config.parameter_tolerance,
            )?,
        )
        .map_err(workload_error)?,
    );

    parameters.push(
        ApplicationParameter::new(
            "success_criterion",
            config
                .success_criterion
                .as_str()
                .to_owned(),
        )
        .map_err(workload_error)?,
    );

    if let Some(reference_energy) =
        config.reference_energy
    {
        parameters.push(
            ApplicationParameter::new(
                "reference_energy",
                format_finite_float(
                    reference_energy,
                )?,
            )
            .map_err(workload_error)?,
        );
    }

    if let Some(maximum_energy_error) =
        config.maximum_energy_error
    {
        parameters.push(
            ApplicationParameter::new(
                "maximum_energy_error",
                format_finite_float(
                    maximum_energy_error,
                )?,
            )
            .map_err(workload_error)?,
        );
    }

    for (index, value) in
        config.initial_parameters.iter().enumerate()
    {
        parameters.push(
            ApplicationParameter::new(
                format!(
                    "initial_parameter_{index}"
                ),
                format_finite_float(*value)?,
            )
            .map_err(workload_error)?,
        );
    }

    Ok(parameters)
}

/// Decodes the canonical VQE workload parameters into a bounded lookup.
///
/// This intentionally does not create a general-purpose parameter parser.
fn decode_and_validate_parameters(
    parameters: &[ApplicationParameter],
) -> BenchmarkResult<
    std::collections::BTreeMap<String, String>,
> {
    let mut result =
        std::collections::BTreeMap::new();

    for parameter in parameters {
        if result
            .insert(
                parameter.name().to_owned(),
                parameter.value().to_owned(),
            )
            .is_some()
        {
            return Err(
                BenchmarkError::InvalidWorkload {
                    workload:
                        VQE_APPLICATION_ID
                            .to_owned(),
                    reason:
                        format!(
                            "duplicate VQE parameter '{}'",
                            parameter.name()
                        ),
                },
            );
        }
    }

    Ok(result)
}

/// Parses a required bounded unsigned integer.
fn parse_required_usize(
    parameters: &std::collections::BTreeMap<String, String>,
    name: &str,
) -> BenchmarkResult<usize> {
    let value =
        parameters.get(name).ok_or_else(
            || BenchmarkError::InvalidWorkload {
                workload:
                    VQE_APPLICATION_ID
                        .to_owned(),
                reason:
                    format!(
                        "missing required VQE parameter '{name}'"
                    ),
            },
        )?;

    let parsed =
        value.parse::<usize>().map_err(|_| {
            BenchmarkError::InvalidWorkload {
                workload:
                    VQE_APPLICATION_ID
                        .to_owned(),
                reason:
                    format!(
                        "VQE parameter '{name}' is not a valid unsigned integer"
                    ),
            }
        })?;

    Ok(parsed)
}

// =============================================================================
// Metric helpers
// =============================================================================

fn default_metrics() -> Vec<VqeMetric> {
    vec![
        VqeMetric::FinalEnergy,
        VqeMetric::BestEnergy,
        VqeMetric::Convergence,
        VqeMetric::ObjectiveEvaluations,
        VqeMetric::OptimizerIterations,
        VqeMetric::CircuitExecutions,
        VqeMetric::Shots,
        VqeMetric::QuantumExecutionTime,
        VqeMetric::ClassicalOptimizationTime,
        VqeMetric::TotalTime,
        VqeMetric::TimeToSolution,
        VqeMetric::ParameterCount,
        VqeMetric::QubitCount,
        VqeMetric::CircuitDepth,
        VqeMetric::GateCount,
        VqeMetric::TwoQubitGateCount,
    ]
}

/// Canonicalizes and validates a metric list.
///
/// Duplicate metrics are removed while preserving deterministic enum order.
fn canonicalize_metrics(
    metrics: Vec<VqeMetric>,
) -> BenchmarkResult<Vec<VqeMetric>> {
    if metrics.is_empty() {
        return Err(
            BenchmarkError::InvalidConfiguration {
                field:
                    "vqe.metrics".to_owned(),
                reason:
                    "VQE benchmark must request at least one metric"
                        .to_owned(),
            },
        );
    }

    let mut canonical =
        Vec::with_capacity(metrics.len());

    for metric in metrics {
        if !canonical.contains(&metric) {
            canonical.push(metric);
        }
    }

    canonical.sort_by_key(|metric| metric.as_str());

    Ok(canonical)
}

// =============================================================================
// Numeric validation
// =============================================================================

fn validate_identifier(
    field: &str,
    value: &str,
    maximum_bytes: usize,
) -> BenchmarkResult<()> {
    if value.is_empty() {
        return Err(
            BenchmarkError::InvalidIdentifier {
                field: field.to_owned(),
                value: value.to_owned(),
            },
        );
    }

    if value.len() > maximum_bytes {
        return Err(
            BenchmarkError::InvalidRange {
                field: field.to_owned(),
                value: value.len().to_string(),
                minimum: Some("1".to_owned()),
                maximum:
                    Some(maximum_bytes.to_string()),
            },
        );
    }

    let mut bytes =
        value.bytes();

    match bytes.next() {
        Some(first)
            if first.is_ascii_lowercase() => {}

        _ => {
            return Err(
                BenchmarkError::InvalidIdentifier {
                    field: field.to_owned(),
                    value: value.to_owned(),
                },
            );
        }
    }

    for byte in bytes {
        if !(byte.is_ascii_lowercase()
            || byte.is_ascii_digit()
            || byte == b'_'
            || byte == b'-')
        {
            return Err(
                BenchmarkError::InvalidIdentifier {
                    field: field.to_owned(),
                    value: value.to_owned(),
                },
            );
        }
    }

    Ok(())
}

fn validate_positive_finite(
    field: &str,
    value: f64,
) -> BenchmarkResult<()> {
    if !value.is_finite()
        || value <= 0.0
    {
        return Err(
            BenchmarkError::InvalidConfiguration {
                field: field.to_owned(),
                reason:
                    "value must be finite and greater than zero"
                        .to_owned(),
            },
        );
    }

    Ok(())
}

fn format_finite_float(
    value: f64,
) -> BenchmarkResult<String> {
    if !value.is_finite() {
        return Err(
            BenchmarkError::InvalidConfiguration {
                field:
                    "vqe.floating_point_value"
                        .to_owned(),
                reason:
                    "value must be finite"
                        .to_owned(),
            },
        );
    }

    Ok(value.to_string())
}

fn workload_error(
    error: super::super::core::workload::WorkloadError,
) -> BenchmarkError {
    BenchmarkError::InvalidWorkload {
        workload:
            VQE_APPLICATION_ID.to_owned(),
        reason: error.to_string(),
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config() -> VqeBenchmarkConfig {
        VqeBenchmarkConfig::new(
            "h2_hamiltonian",
            "hardware_efficient",
            "cobyla",
            4,
            vec![0.0, 0.1, 0.2, 0.3],
        )
        .expect("test VQE configuration must be valid")
    }

    #[test]
    fn canonical_generator_descriptor_is_stable() {
        let generator =
            VqeBenchmarkGenerator::new()
                .expect("generator");

        assert_eq!(
            generator
                .descriptor()
                .generator_id(),
            VQE_BENCHMARK_ID
        );

        assert_eq!(
            generator
                .descriptor()
                .application_id(),
            VQE_APPLICATION_ID
        );

        assert!(
            generator
                .descriptor()
                .supports(
                    ApplicationGeneratorCapability::Hybrid
                )
        );

        assert!(
            generator
                .descriptor()
                .supports(
                    ApplicationGeneratorCapability::Deterministic
                )
        );
    }

    #[test]
    fn configuration_rejects_empty_hamiltonian_id() {
        let result =
            VqeBenchmarkConfig::new(
                "",
                "hardware_efficient",
                "cobyla",
                4,
                vec![0.0],
            );

        assert!(result.is_err());
    }

    #[test]
    fn configuration_rejects_zero_qubits() {
        let result =
            VqeBenchmarkConfig::new(
                "h",
                "a",
                "o",
                0,
                vec![0.0],
            );

        assert!(result.is_err());
    }

    #[test]
    fn configuration_rejects_non_finite_parameter() {
        let result =
            VqeBenchmarkConfig::new(
                "h",
                "a",
                "o",
                1,
                vec![f64::NAN],
            );

        assert!(result.is_err());
    }

    #[test]
    fn configuration_rejects_infinite_tolerance() {
        let result =
            VqeBenchmarkConfig::new(
                "h",
                "a",
                "o",
                1,
                vec![0.0],
            )
            .and_then(|config| {
                config.with_energy_tolerance(
                    f64::INFINITY,
                )
            });

        assert!(result.is_err());
    }

    #[test]
    fn energy_error_requires_reference_energy() {
        let result =
            VqeBenchmarkConfig::new(
                "h",
                "a",
                "o",
                1,
                vec![0.0],
            )
            .and_then(|config| {
                config.with_success_criterion(
                    VqeSuccessCriterion::EnergyError,
                )
            });

        assert!(result.is_err());
    }

    #[test]
    fn energy_error_requires_maximum_error() {
        let result =
            VqeBenchmarkConfig::new(
                "h",
                "a",
                "o",
                1,
                vec![0.0],
            )
            .and_then(|config| {
                config
                    .with_reference_energy(-1.137)
            })
            .and_then(|config| {
                config.with_success_criterion(
                    VqeSuccessCriterion::EnergyError,
                )
            });

        assert!(result.is_err());
    }

    #[test]
    fn workload_generation_is_deterministic() {
        let generator =
            VqeBenchmarkGenerator::new()
                .expect("generator");

        let config =
            test_config();

        let first =
            generator
                .generate_from_config(
                    &config,
                    WorkloadId::new(
                        "vqe_h2_instance_0",
                    )
                    .expect("instance"),
                    42,
                )
                .expect("generation");

        let second =
            generator
                .generate_from_config(
                    &config,
                    WorkloadId::new(
                        "vqe_h2_instance_0",
                    )
                    .expect("instance"),
                    42,
                )
                .expect("generation");

        assert_eq!(
            first.application_id(),
            second.application_id()
        );

        assert_eq!(
            first.instance_id(),
            second.instance_id()
        );

        assert_eq!(
            first.problem_size(),
            second.problem_size()
        );

        assert_eq!(
            first.parameters(),
            second.parameters()
        );
    }

    #[test]
    fn workload_generation_preserves_parameter_metadata() {
        let generator =
            VqeBenchmarkGenerator::new()
                .expect("generator");

        let config =
            test_config();

        let workload =
            generator
                .generate_from_config(
                    &config,
                    WorkloadId::new(
                        "vqe_h2_instance_0",
                    )
                    .expect("instance"),
                    42,
                )
                .expect("generation");

        assert_eq!(
            workload.application_id(),
            VQE_APPLICATION_ID
        );

        assert_eq!(
            workload.problem_size(),
            4
        );

        assert!(
            workload
                .parameters()
                .iter()
                .any(|parameter| {
                    parameter.name()
                        == "hamiltonian_id"
                        && parameter.value()
                            == "h2_hamiltonian"
                })
        );

        assert!(
            workload
                .parameters()
                .iter()
                .any(|parameter| {
                    parameter.name()
                        == "ansatz_id"
                        && parameter.value()
                            == "hardware_efficient"
                })
        );

        assert!(
            workload
                .parameters()
                .iter()
                .any(|parameter| {
                    parameter.name()
                        == "parameter_count"
                        && parameter.value()
                            == "4"
                })
        );
    }

    #[test]
    fn metrics_are_non_empty_and_deterministic() {
        let first =
            VqeBenchmarkGenerator::default_metrics();

        let second =
            VqeBenchmarkGenerator::default_metrics();

        assert!(!first.is_empty());
        assert_eq!(first, second);
    }

    #[test]
    fn metric_duplicates_are_removed() {
        let metrics =
            canonicalize_metrics(vec![
                VqeMetric::FinalEnergy,
                VqeMetric::FinalEnergy,
                VqeMetric::Shots,
            ])
            .expect("metrics");

        assert_eq!(
            metrics.len(),
            2
        );
    }

    #[test]
    fn generator_revision_is_encoded_in_request() {
        let generator =
            VqeBenchmarkGenerator::new()
                .expect("generator");

        let request =
            VqeBenchmarkGenerator::request(
                &test_config(),
                WorkloadId::new(
                    "vqe_instance",
                )
                .expect("instance"),
                42,
            )
            .expect("request");

        assert_eq!(
            request
                .metadata()
                .generator_revision(),
            VQE_GENERATOR_REVISION
        );

        assert_eq!(
            request
                .metadata()
                .seed(),
            42
        );

        assert_eq!(
            request.application_id(),
            generator
                .descriptor()
                .application_id()
        );
    }

    #[test]
    fn success_criterion_none_is_valid_without_reference() {
        let config =
            VqeBenchmarkConfig::new(
                "h",
                "a",
                "o",
                1,
                vec![0.0],
            )
            .expect("base config")
            .with_success_criterion(
                VqeSuccessCriterion::None,
            )
            .expect("criterion");

        assert!(
            !config.requires_convergence()
        );

        assert!(
            !config.requires_energy_reference()
        );
    }
}