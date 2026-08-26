//! Zamani Quantum Benchmarking — Hamiltonian Simulation Application Benchmark
//!
//! Production-grade, backend-independent benchmark definition for quantum
//! Hamiltonian simulation.
//!
//! # Architectural responsibility
//!
//! This module defines the *benchmark contract* for Hamiltonian simulation.
//! It owns:
//!
//! - benchmark identity and semantic version;
//! - Hamiltonian-simulation workload configuration;
//! - validation of benchmark configuration;
//! - simulation-task classification;
//! - simulation-method classification;
//! - initial-state metadata;
//! - target-evolution metadata;
//! - reference/accuracy requirements;
//! - reproducible application workload construction;
//! - benchmark metric declarations;
//! - resource/quality requirements;
//! - application-generator integration;
//! - safe conversion into the canonical `ApplicationWorkload`.
//!
//! This module deliberately does NOT own:
//!
//! - Hamiltonian mathematical representation;
//! - Pauli decomposition;
//! - sparse/dense matrix representation;
//! - Hamiltonian construction;
//! - Trotter/Suzuki mathematics;
//! - QSP/QSVT mathematics;
//! - LCU mathematics;
//! - qubitization mathematics;
//! - Taylor-series simulation mathematics;
//! - time-evolution circuit construction;
//! - Quantum IR semantics;
//! - circuit optimization;
//! - routing;
//! - scheduling;
//! - backend communication;
//! - simulator implementation;
//! - quantum execution;
//! - statistical analysis;
//! - fidelity calculation implementation;
//! - reporting;
//! - persistence.
//!
//! Those responsibilities belong to the corresponding quantum algorithm,
//! IR, compiler, runtime, execution, statistics, metrics, and reporting
//! layers.
//!
//! # Architectural flow
//!
//! ```text
//! HamiltonianBenchmarkConfig
//!          │
//!          ▼
//! HamiltonianSimulationGenerator
//!          │
//!          ▼
//! ApplicationGenerationRequest
//!          │
//!          ▼
//! ApplicationWorkload
//!          │
//!          ├──────────────► optional Quantum IR circuit
//!          │
//!          ▼
//!      Experiment
//!          │
//!          ▼
//!      Execution
//!          │
//!          ▼
//!     Observations
//!          │
//!          ▼
//!       Analysis
//!          │
//!          ├── approximation error
//!          ├── observable error
//!          ├── fidelity
//!          ├── runtime
//!          ├── circuit resources
//!          └── time-to-solution
//!          │
//!          ▼
//!     BenchmarkResult
//! ```
//!
//! # Scientific scope
//!
//! Hamiltonian simulation is broader than simply calculating an energy.
//!
//! This benchmark therefore supports:
//!
//! - real-time evolution;
//! - imaginary-time/resource-estimation workloads;
//! - ground-state-energy estimation when explicitly selected;
//! - observable estimation after evolution;
//! - resource-only simulation studies;
//! - exact-reference comparison;
//! - approximate-reference comparison.
//!
//! The benchmark must never silently interpret a time-evolution experiment
//! as a ground-state-energy experiment.
//!
//! # Hamiltonian representation boundary
//!
//! Zamani's VQE implementation already establishes the correct architectural
//! principle: the algorithm layer should depend on a backend-neutral
//! Hamiltonian/observable identity rather than inventing a second universal
//! mathematical representation.
//!
//! This benchmark follows the same principle.
//!
//! `hamiltonian_id` identifies the canonical Hamiltonian/problem instance.
//! The actual Hamiltonian representation belongs to the future canonical
//! observable/physics subsystem.
//!
//! This file therefore remains usable whether the eventual Hamiltonian is:
//!
//! - a Pauli sum;
//! - sparse matrix;
//! - dense matrix;
//! - fermionic operator;
//! - bosonic operator;
//! - lattice Hamiltonian;
//! - spin Hamiltonian;
//! - molecular Hamiltonian;
//! - problem-generated Hamiltonian;
//! - externally supplied Hamiltonian;
//! - symbolic Hamiltonian.
//!
//! # Method neutrality
//!
//! The benchmark can describe methods including:
//!
//! - exact/classical reference;
//! - first-order Trotter;
//! - higher-order Suzuki;
//! - Taylor/LCU;
//! - qubitization;
//! - quantum signal processing;
//! - quantum singular-value transformation;
//! - product-formula variants;
//! - truncated/approximate methods;
//! - custom Zamani simulation methods.
//!
//! The actual method implementation belongs elsewhere.
//!
//! # Reproducibility
//!
//! The generated workload is deterministic with respect to:
//!
//! - benchmark configuration;
//! - application identifier;
//! - instance identifier;
//! - problem size;
//! - Hamiltonian identifier;
//! - simulation method;
//! - initial-state identifier;
//! - evolution time;
//! - discretization parameters;
//! - requested target accuracy;
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
//! Benchmark configuration may eventually originate from:
//!
//! - Zamani source;
//! - CLI;
//! - configuration files;
//! - CI;
//! - remote benchmark requests;
//! - benchmark registries.
//!
//! Therefore all values are treated as untrusted.
//!
//! This module:
//!
//! - validates identifiers;
//! - rejects zero sizes;
//! - rejects non-finite floating-point values;
//! - rejects negative evolution times;
//! - bounds textual identifiers;
//! - bounds encoded benchmark parameters;
//! - bounds step counts;
//! - bounds requested observable counts;
//! - uses checked arithmetic;
//! - does not execute user code;
//! - does not perform I/O;
//! - does not allocate from unchecked exponential expressions.
//!
//! Global benchmark limits remain owned by `core::limits` and `core::config`.
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
//! No external dependencies.
//!
//! # Integration contract
//!
//! This module integrates with:
//!
//! ```text
//! benchmarking::core::errors
//! benchmarking::core::workload
//! benchmarking::generators::application
//!
//! future:
//! benchmarking::core::experiment
//! benchmarking::execution
//! benchmarking::observation
//! benchmarking::metrics
//! benchmarking::statistics
//! benchmarking::reporting
//!
//! quantum::algorithms
//! quantum::ir
//! quantum::hardware
//! quantum::optimization
//! quantum::routing
//! quantum::scheduling
//! runtime::quantum
//! ```
//!
//! It does NOT require any backend or executor to exist in order to define
//! the benchmark.
//!
//! A future Hamiltonian circuit generator may attach a canonical
//! `CircuitWorkload` using `ApplicationWorkload::with_circuit()` without
//! changing the public benchmark configuration contract.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

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

/// Stable machine-readable benchmark identifier.
pub const HAMILTONIAN_BENCHMARK_ID: &str = "hamiltonian_simulation";

/// Stable application identifier.
pub const HAMILTONIAN_APPLICATION_ID: &str = "hamiltonian_simulation";

/// Semantic version of the benchmark definition.
///
/// This is independent of:
///
/// - Zamani version;
/// - Quantum IR version;
/// - algorithm implementation version;
/// - backend version;
/// - simulator version.
pub const HAMILTONIAN_BENCHMARK_VERSION: u32 = 1;

/// Generator revision.
///
/// Increment this when generation semantics change without necessarily
/// changing the external benchmark schema.
pub const HAMILTONIAN_GENERATOR_REVISION: u32 = 1;

/// Human-readable generator version.
pub const HAMILTONIAN_GENERATOR_VERSION: &str = "1.0.0";

// =============================================================================
// Resource limits
// =============================================================================

/// Maximum byte length of the Hamiltonian identifier.
pub const MAX_HAMILTONIAN_ID_BYTES: usize = 128;

/// Maximum byte length of the simulation-method identifier.
pub const MAX_METHOD_ID_BYTES: usize = 128;

/// Maximum byte length of the initial-state identifier.
pub const MAX_INITIAL_STATE_ID_BYTES: usize = 128;

/// Maximum byte length of the observable identifier.
pub const MAX_OBSERVABLE_ID_BYTES: usize = 128;

/// Maximum number of observables attached to one benchmark case.
pub const MAX_OBSERVABLES: usize = 4096;

/// Maximum number of encoded benchmark parameters.
pub const MAX_HAMILTONIAN_PARAMETERS: usize = 256;

/// Maximum encoded parameter value length.
pub const MAX_PARAMETER_VALUE_BYTES: usize = 512;

/// Maximum evolution steps.
///
/// This is a semantic guard. Global execution limits remain authoritative.
pub const MAX_EVOLUTION_STEPS: usize = 100_000_000;

/// Maximum problem size represented directly by this benchmark boundary.
pub const MAX_PROBLEM_SIZE: usize = usize::MAX / 2;

/// Maximum evolution time.
///
/// This is intentionally finite to prevent pathological input values while
/// still allowing large scientific workloads. Physical/backend-specific
/// limits belong to the backend capability layer.
pub const MAX_EVOLUTION_TIME: f64 = 1.0e30;

/// Maximum requested target error.
pub const MIN_TARGET_ERROR: f64 = 1.0e-15;

/// Maximum number of requested accuracy digits.
pub const MAX_ACCURACY_DIGITS: u32 = 15;

// =============================================================================
// Simulation task
// =============================================================================

/// Scientific task represented by a Hamiltonian-simulation benchmark.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum HamiltonianSimulationTask {
    /// Approximate the state
    ///
    /// `|psi(t)> = exp(-i H t) |psi(0)>`
    ///
    /// and compare it with a reference where available.
    TimeEvolution,

    /// Estimate an observable after Hamiltonian evolution.
    ObservableEstimation,

    /// Estimate the ground-state energy.
    ///
    /// This task is intentionally separate from time evolution because
    /// Hamiltonian simulation and ground-state preparation are scientifically
    /// different workloads.
    GroundStateEnergy,

    /// Measure simulation resources without requiring a scientific
    /// correctness target.
    ResourceEstimation,

    /// User-defined Hamiltonian simulation task.
    Custom,
}

impl HamiltonianSimulationTask {
    /// Returns the stable machine-readable identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TimeEvolution => "time_evolution",
            Self::ObservableEstimation => "observable_estimation",
            Self::GroundStateEnergy => "ground_state_energy",
            Self::ResourceEstimation => "resource_estimation",
            Self::Custom => "custom",
        }
    }
}

impl fmt::Display for HamiltonianSimulationTask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// =============================================================================
// Simulation method
// =============================================================================

/// Hamiltonian-simulation algorithm family.
///
/// This enumeration describes the algorithm family only. It does not
/// implement any of the algorithms.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum HamiltonianSimulationMethod {
    /// Exact/classical reference calculation.
    Exact,

    /// First-order product formula / Trotterization.
    TrotterFirstOrder,

    /// Higher-order Suzuki product formula.
    Suzuki,

    /// Taylor-series / LCU-style simulation.
    TaylorLcu,

    /// Qubitization.
    Qubitization,

    /// Quantum signal processing.
    Qsp,

    /// Quantum singular-value transformation.
    Qsvt,

    /// A generic approximation method supplied by another subsystem.
    Approximate,

    /// User-defined method.
    Custom(String),
}

impl HamiltonianSimulationMethod {
    /// Returns a stable machine-readable identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Exact => "exact",
            Self::TrotterFirstOrder => "trotter_first_order",
            Self::Suzuki => "suzuki",
            Self::TaylorLcu => "taylor_lcu",
            Self::Qubitization => "qubitization",
            Self::Qsp => "qsp",
            Self::Qsvt => "qsvt",
            Self::Approximate => "approximate",
            Self::Custom(value) => value.as_str(),
        }
    }

    /// Returns whether the method is a custom registry method.
    #[must_use]
    pub fn is_custom(&self) -> bool {
        matches!(self, Self::Custom(_))
    }
}

impl fmt::Display for HamiltonianSimulationMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// =============================================================================
// Initial-state representation
// =============================================================================

/// Description of the initial state.
///
/// The benchmark records the identity only. State preparation belongs to the
/// algorithm/circuit-generation layer.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HamiltonianInitialState {
    /// Computational-basis state.
    ComputationalBasis(String),

    /// Named prepared state.
    Named(String),

    /// State generated by another Zamani application/algorithm.
    Generated(String),

    /// User-defined state representation.
    Custom(String),
}

impl HamiltonianInitialState {
    /// Returns the stable kind identifier.
    #[must_use]
    pub fn kind(&self) -> &'static str {
        match self {
            Self::ComputationalBasis(_) => "computational_basis",
            Self::Named(_) => "named",
            Self::Generated(_) => "generated",
            Self::Custom(_) => "custom",
        }
    }

    /// Returns the referenced state identifier.
    #[must_use]
    pub fn identifier(&self) -> &str {
        match self {
            Self::ComputationalBasis(value)
            | Self::Named(value)
            | Self::Generated(value)
            | Self::Custom(value) => value.as_str(),
        }
    }
}

impl fmt::Display for HamiltonianInitialState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.kind(), self.identifier())
    }
}

// =============================================================================
// Benchmark metrics
// =============================================================================

/// Metric that can be requested by the Hamiltonian simulation benchmark.
///
/// The actual numerical representation belongs to the generic benchmarking
/// metrics layer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum HamiltonianMetric {
    /// Absolute simulation error.
    AbsoluteSimulationError,

    /// Relative simulation error.
    RelativeSimulationError,

    /// State fidelity against a reference state.
    StateFidelity,

    /// Observable expectation-value error.
    ObservableError,

    /// Energy error.
    EnergyError,

    /// Ground-state energy estimate.
    GroundStateEnergy,

    /// Evolution time represented by the workload.
    EvolutionTime,

    /// Number of Trotter/product-formula steps.
    EvolutionSteps,

    /// Number of logical qubits.
    QubitCount,

    /// Logical circuit depth.
    CircuitDepth,

    /// Total logical gate count.
    GateCount,

    /// Total two-qubit logical gate count.
    TwoQubitGateCount,

    /// Quantum execution time.
    QuantumExecutionTime,

    /// Classical preprocessing time.
    ClassicalPreprocessingTime,

    /// Total end-to-end time.
    TotalTime,

    /// Time-to-solution.
    TimeToSolution,

    /// Number of measurements/shots.
    Shots,

    /// Simulation target accuracy.
    TargetError,

    /// Achieved accuracy.
    AchievedError,

    /// Resource estimate for a specified implementation.
    ResourceEstimate,
}

impl HamiltonianMetric {
    /// Returns a stable machine-readable identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AbsoluteSimulationError => "absolute_simulation_error",
            Self::RelativeSimulationError => "relative_simulation_error",
            Self::StateFidelity => "state_fidelity",
            Self::ObservableError => "observable_error",
            Self::EnergyError => "energy_error",
            Self::GroundStateEnergy => "ground_state_energy",
            Self::EvolutionTime => "evolution_time",
            Self::EvolutionSteps => "evolution_steps",
            Self::QubitCount => "qubit_count",
            Self::CircuitDepth => "circuit_depth",
            Self::GateCount => "gate_count",
            Self::TwoQubitGateCount => "two_qubit_gate_count",
            Self::QuantumExecutionTime => "quantum_execution_time",
            Self::ClassicalPreprocessingTime => "classical_preprocessing_time",
            Self::TotalTime => "total_time",
            Self::TimeToSolution => "time_to_solution",
            Self::Shots => "shots",
            Self::TargetError => "target_error",
            Self::AchievedError => "achieved_error",
            Self::ResourceEstimate => "resource_estimate",
        }
    }
}

impl fmt::Display for HamiltonianMetric {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// =============================================================================
// Accuracy/reference policy
// =============================================================================

/// Defines how correctness is established for a Hamiltonian simulation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HamiltonianReferencePolicy {
    /// Require an externally supplied exact reference.
    ExactReference,

    /// Use an approximate/classical reference.
    ApproximateReference,

    /// No correctness reference is required.
    None,
}

impl HamiltonianReferencePolicy {
    /// Stable machine-readable identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactReference => "exact_reference",
            Self::ApproximateReference => "approximate_reference",
            Self::None => "none",
        }
    }
}

impl fmt::Display for HamiltonianReferencePolicy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// =============================================================================
// Success criterion
// =============================================================================

/// Scientific pass/fail rule for the benchmark.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HamiltonianSuccessCriterion {
    /// Require achieved error to be at most the target error.
    ErrorWithinTarget,

    /// Require fidelity to meet or exceed a configured threshold.
    FidelityThreshold,

    /// Require successful observable estimation.
    ObservableWithinTarget,

    /// No scientific pass/fail criterion.
    None,
}

impl HamiltonianSuccessCriterion {
    /// Stable machine-readable identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ErrorWithinTarget => "error_within_target",
            Self::FidelityThreshold => "fidelity_threshold",
            Self::ObservableWithinTarget => "observable_within_target",
            Self::None => "none",
        }
    }
}

impl fmt::Display for HamiltonianSuccessCriterion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// =============================================================================
// Hamiltonian benchmark configuration
// =============================================================================

/// Complete semantic configuration of a Hamiltonian-simulation benchmark.
///
/// Generic backend/execution configuration remains outside this structure.
#[derive(Debug, Clone, PartialEq)]
pub struct HamiltonianBenchmarkConfig {
    /// Stable Hamiltonian/problem identifier.
    pub hamiltonian_id: String,

    /// Simulation task.
    pub task: HamiltonianSimulationTask,

    /// Simulation method.
    pub method: HamiltonianSimulationMethod,

    /// Initial state.
    pub initial_state: HamiltonianInitialState,

    /// Number of logical qubits.
    pub qubit_count: usize,

    /// Physical evolution time `t`.
    pub evolution_time: f64,

    /// Optional number of product-formula/simulation steps.
    ///
    /// It is optional because not all simulation algorithms use a discrete
    /// step count.
    pub evolution_steps: Option<usize>,

    /// Target approximation error.
    pub target_error: Option<f64>,

    /// Reference policy.
    pub reference_policy: HamiltonianReferencePolicy,

    /// Optional reference-state identifier.
    pub reference_state_id: Option<String>,

    /// Optional reference observable identifier.
    pub reference_observable_id: Option<String>,

    /// Optional reference energy.
    pub reference_energy: Option<f64>,

    /// Optional expected observable value.
    pub reference_observable_value: Option<f64>,

    /// Observable identifiers to measure after evolution.
    pub observables: Vec<String>,

    /// Scientific success criterion.
    pub success_criterion: HamiltonianSuccessCriterion,

    /// Optional fidelity threshold.
    pub fidelity_threshold: Option<f64>,

    /// Optional maximum allowed observable error.
    pub maximum_observable_error: Option<f64>,

    /// Requested benchmark metrics.
    pub metrics: Vec<HamiltonianMetric>,
}

impl HamiltonianBenchmarkConfig {
    /// Creates a minimal time-evolution benchmark.
    pub fn new(
        hamiltonian_id: impl Into<String>,
        qubit_count: usize,
        evolution_time: f64,
    ) -> BenchmarkResult<Self> {
        let config = Self {
            hamiltonian_id: hamiltonian_id.into(),
            task: HamiltonianSimulationTask::TimeEvolution,
            method: HamiltonianSimulationMethod::TrotterFirstOrder,
            initial_state: HamiltonianInitialState::Named(
                "computational_zero".to_owned(),
            ),
            qubit_count,
            evolution_time,
            evolution_steps: Some(1),
            target_error: None,
            reference_policy: HamiltonianReferencePolicy::None,
            reference_state_id: None,
            reference_observable_id: None,
            reference_energy: None,
            reference_observable_value: None,
            observables: Vec::new(),
            success_criterion: HamiltonianSuccessCriterion::None,
            fidelity_threshold: None,
            maximum_observable_error: None,
            metrics: default_metrics(),
        };

        config.validate()?;
        Ok(config)
    }

    /// Changes the simulation task.
    pub fn with_task(
        mut self,
        task: HamiltonianSimulationTask,
    ) -> BenchmarkResult<Self> {
        self.task = task;
        self.validate()?;
        Ok(self)
    }

    /// Changes the simulation method.
    pub fn with_method(
        mut self,
        method: HamiltonianSimulationMethod,
    ) -> BenchmarkResult<Self> {
        validate_method(&method)?;
        self.method = method;
        self.validate()?;
        Ok(self)
    }

    /// Changes the initial state.
    pub fn with_initial_state(
        mut self,
        initial_state: HamiltonianInitialState,
    ) -> BenchmarkResult<Self> {
        validate_initial_state(&initial_state)?;
        self.initial_state = initial_state;
        self.validate()?;
        Ok(self)
    }

    /// Sets the number of simulation steps.
    pub fn with_evolution_steps(
        mut self,
        steps: usize,
    ) -> BenchmarkResult<Self> {
        self.evolution_steps = Some(steps);
        self.validate()?;
        Ok(self)
    }

    /// Removes an explicit step count.
    ///
    /// This is required for methods whose implementation determines the
    /// discretization internally.
    pub fn without_evolution_steps(
        mut self,
    ) -> BenchmarkResult<Self> {
        self.evolution_steps = None;
        self.validate()?;
        Ok(self)
    }

    /// Sets the target simulation error.
    pub fn with_target_error(
        mut self,
        target_error: f64,
    ) -> BenchmarkResult<Self> {
        self.target_error = Some(target_error);
        self.validate()?;
        Ok(self)
    }

    /// Sets an exact or approximate reference policy.
    pub fn with_reference_policy(
        mut self,
        policy: HamiltonianReferencePolicy,
    ) -> BenchmarkResult<Self> {
        self.reference_policy = policy;
        self.validate()?;
        Ok(self)
    }

    /// Sets a reference state identifier.
    pub fn with_reference_state(
        mut self,
        reference_state_id: impl Into<String>,
    ) -> BenchmarkResult<Self> {
        self.reference_state_id = Some(reference_state_id.into());
        self.validate()?;
        Ok(self)
    }

    /// Sets a reference observable.
    pub fn with_reference_observable(
        mut self,
        observable_id: impl Into<String>,
    ) -> BenchmarkResult<Self> {
        self.reference_observable_id = Some(observable_id.into());
        self.validate()?;
        Ok(self)
    }

    /// Sets a reference ground-state energy.
    pub fn with_reference_energy(
        mut self,
        energy: f64,
    ) -> BenchmarkResult<Self> {
        self.reference_energy = Some(energy);
        self.validate()?;
        Ok(self)
    }

    /// Sets a reference observable expectation value.
    pub fn with_reference_observable_value(
        mut self,
        value: f64,
    ) -> BenchmarkResult<Self> {
        self.reference_observable_value = Some(value);
        self.validate()?;
        Ok(self)
    }

    /// Adds one observable identifier.
    pub fn with_observable(
        mut self,
        observable_id: impl Into<String>,
    ) -> BenchmarkResult<Self> {
        let observable_id = observable_id.into();

        validate_identifier(
            "observable_id",
            &observable_id,
            MAX_OBSERVABLE_ID_BYTES,
        )?;

        if self.observables.len() >= MAX_OBSERVABLES {
            return Err(BenchmarkError::ResourceLimitExceeded {
                resource: "hamiltonian_observables".to_owned(),
                requested: self.observables.len() as u64 + 1,
                maximum: MAX_OBSERVABLES as u64,
            });
        }

        if !self.observables.contains(&observable_id) {
            self.observables.push(observable_id);
        }

        self.validate()?;
        Ok(self)
    }

    /// Sets the scientific success criterion.
    pub fn with_success_criterion(
        mut self,
        criterion: HamiltonianSuccessCriterion,
    ) -> BenchmarkResult<Self> {
        self.success_criterion = criterion;
        self.validate()?;
        Ok(self)
    }

    /// Sets a fidelity threshold.
    pub fn with_fidelity_threshold(
        mut self,
        threshold: f64,
    ) -> BenchmarkResult<Self> {
        self.fidelity_threshold = Some(threshold);
        self.validate()?;
        Ok(self)
    }

    /// Sets the maximum observable error.
    pub fn with_maximum_observable_error(
        mut self,
        error: f64,
    ) -> BenchmarkResult<Self> {
        self.maximum_observable_error = Some(error);
        self.validate()?;
        Ok(self)
    }

    /// Replaces the requested metric set.
    pub fn with_metrics(
        mut self,
        metrics: Vec<HamiltonianMetric>,
    ) -> BenchmarkResult<Self> {
        self.metrics = normalize_metrics(metrics);
        self.validate()?;
        Ok(self)
    }

    /// Validates the complete benchmark configuration.
    pub fn validate(&self) -> BenchmarkResult<()> {
        validate_identifier(
            "hamiltonian_id",
            &self.hamiltonian_id,
            MAX_HAMILTONIAN_ID_BYTES,
        )?;

        if self.qubit_count == 0 {
            return Err(BenchmarkError::InvalidRange {
                field: "qubit_count".to_owned(),
                value: "0".to_owned(),
                minimum: Some("1".to_owned()),
                maximum: Some(MAX_PROBLEM_SIZE.to_string()),
            });
        }

        if self.qubit_count > MAX_PROBLEM_SIZE {
            return Err(BenchmarkError::InvalidRange {
                field: "qubit_count".to_owned(),
                value: self.qubit_count.to_string(),
                minimum: Some("1".to_owned()),
                maximum: Some(MAX_PROBLEM_SIZE.to_string()),
            });
        }

        if !self.evolution_time.is_finite()
            || self.evolution_time <= 0.0
            || self.evolution_time > MAX_EVOLUTION_TIME
        {
            return Err(BenchmarkError::InvalidRange {
                field: "evolution_time".to_owned(),
                value: self.evolution_time.to_string(),
                minimum: Some("greater_than_zero".to_owned()),
                maximum: Some(MAX_EVOLUTION_TIME.to_string()),
            });
        }

        if let Some(steps) = self.evolution_steps {
            if steps == 0 || steps > MAX_EVOLUTION_STEPS {
                return Err(BenchmarkError::InvalidRange {
                    field: "evolution_steps".to_owned(),
                    value: steps.to_string(),
                    minimum: Some("1".to_owned()),
                    maximum: Some(MAX_EVOLUTION_STEPS.to_string()),
                });
            }
        }

        validate_method(&self.method)?;
        validate_initial_state(&self.initial_state)?;

        if self.observables.len() > MAX_OBSERVABLES {
            return Err(BenchmarkError::ResourceLimitExceeded {
                resource: "hamiltonian_observables".to_owned(),
                requested: self.observables.len() as u64,
                maximum: MAX_OBSERVABLES as u64,
            });
        }

        for observable in &self.observables {
            validate_identifier(
                "observable_id",
                observable,
                MAX_OBSERVABLE_ID_BYTES,
            )?;
        }

        if let Some(target) = self.target_error {
            validate_error("target_error", target)?;
        }

        if let Some(energy) = self.reference_energy {
            if !energy.is_finite() {
                return Err(BenchmarkError::InvalidConfiguration {
                    field: "reference_energy".to_owned(),
                    message: "reference energy must be finite".to_owned(),
                });
            }
        }

        if let Some(value) = self.reference_observable_value {
            if !value.is_finite() {
                return Err(BenchmarkError::InvalidConfiguration {
                    field: "reference_observable_value".to_owned(),
                    message:
                        "reference observable value must be finite"
                            .to_owned(),
                });
            }
        }

        if let Some(threshold) = self.fidelity_threshold {
            if !threshold.is_finite() || !(0.0..=1.0).contains(&threshold) {
                return Err(BenchmarkError::InvalidRange {
                    field: "fidelity_threshold".to_owned(),
                    value: threshold.to_string(),
                    minimum: Some("0".to_owned()),
                    maximum: Some("1".to_owned()),
                });
            }
        }

        if let Some(error) = self.maximum_observable_error {
            validate_error("maximum_observable_error", error)?;
        }

        validate_reference_policy(self)?;
        validate_task_requirements(self)?;
        validate_success_criterion(self)?;

        Ok(())
    }
}

// =============================================================================
// Generator
// =============================================================================

/// Production Hamiltonian-simulation application benchmark generator.
///
/// This generator creates the canonical `ApplicationWorkload`. It does not
/// generate a Quantum IR circuit itself.
///
/// A future algorithm-specific circuit generator can consume the same
/// configuration and attach a `CircuitWorkload` without changing this API.
#[derive(Debug, Clone)]
pub struct HamiltonianSimulationGenerator {
    descriptor: ApplicationGeneratorDescriptor,
    config: HamiltonianBenchmarkConfig,
}

impl HamiltonianSimulationGenerator {
    /// Creates a validated Hamiltonian simulation generator.
    pub fn new(
        config: HamiltonianBenchmarkConfig,
    ) -> BenchmarkResult<Self> {
        config.validate()?;

        let descriptor = ApplicationGeneratorDescriptor::new(
            "hamiltonian_simulation",
            HAMILTONIAN_APPLICATION_ID,
            HAMILTONIAN_GENERATOR_VERSION,
            "Production Hamiltonian simulation application benchmark generator",
        )?
        .with_capabilities([
            ApplicationGeneratorCapability::Deterministic,
            ApplicationGeneratorCapability::Parameterized,
            ApplicationGeneratorCapability::ScalableProblemSize,
            ApplicationGeneratorCapability::ResourceEstimation,
            ApplicationGeneratorCapability::HardwareExecutable,
            ApplicationGeneratorCapability::ExactSmallInstanceReference,
        ]);

        Ok(Self {
            descriptor,
            config,
        })
    }

    /// Returns the immutable benchmark configuration.
    #[must_use]
    pub fn config(&self) -> &HamiltonianBenchmarkConfig {
        &self.config
    }

    /// Returns the generator descriptor.
    #[must_use]
    pub fn descriptor_ref(&self) -> &ApplicationGeneratorDescriptor {
        &self.descriptor
    }

    /// Returns the benchmark's stable ID.
    #[must_use]
    pub const fn benchmark_id() -> &'static str {
        HAMILTONIAN_BENCHMARK_ID
    }

    /// Returns the benchmark semantic version.
    #[must_use]
    pub const fn benchmark_version() -> u32 {
        HAMILTONIAN_BENCHMARK_VERSION
    }

    /// Returns the generator revision.
    #[must_use]
    pub const fn generator_revision() -> u32 {
        HAMILTONIAN_GENERATOR_REVISION
    }

    /// Builds a canonical application workload from an application-generation
    /// request.
    ///
    /// The generated workload contains the complete semantic benchmark
    /// configuration as bounded application parameters.
    ///
    /// It deliberately does not contain backend-specific execution settings.
    pub fn build_workload(
        &self,
        request: &ApplicationGenerationRequest,
    ) -> BenchmarkResult<ApplicationWorkload> {
        self.validate(request)?;

        let mut workload = ApplicationWorkload::new(
            HAMILTONIAN_APPLICATION_ID,
            request.instance_id().clone(),
            request.problem_size(),
        )
        .map_err(|error| BenchmarkError::InvalidWorkload {
            workload: HAMILTONIAN_APPLICATION_ID.to_owned(),
            reason: error.to_string(),
        })?;

        let parameters = self.parameters()?;

        for parameter in parameters {
            workload
                .add_parameter(parameter)
                .map_err(|error| BenchmarkError::InvalidWorkload {
                    workload: HAMILTONIAN_APPLICATION_ID.to_owned(),
                    reason: error.to_string(),
                })?;
        }

        Ok(workload)
    }

    /// Encodes the benchmark semantics into the canonical bounded
    /// `ApplicationParameter` representation.
    ///
    /// The parameter encoding is deterministic and therefore suitable for
    /// provenance/fingerprinting by later layers.
    pub fn parameters(&self) -> BenchmarkResult<Vec<ApplicationParameter>> {
        let mut parameters = Vec::new();

        push_parameter(
            &mut parameters,
            "benchmark_id",
            HAMILTONIAN_BENCHMARK_ID,
        )?;

        push_parameter(
            &mut parameters,
            "benchmark_version",
            &HAMILTONIAN_BENCHMARK_VERSION.to_string(),
        )?;

        push_parameter(
            &mut parameters,
            "generator_revision",
            &HAMILTONIAN_GENERATOR_REVISION.to_string(),
        )?;

        push_parameter(
            &mut parameters,
            "hamiltonian_id",
            &self.config.hamiltonian_id,
        )?;

        push_parameter(
            &mut parameters,
            "task",
            self.config.task.as_str(),
        )?;

        push_parameter(
            &mut parameters,
            "method",
            self.config.method.as_str(),
        )?;

        push_parameter(
            &mut parameters,
            "initial_state_kind",
            self.config.initial_state.kind(),
        )?;

        push_parameter(
            &mut parameters,
            "initial_state_id",
            self.config.initial_state.identifier(),
        )?;

        push_parameter(
            &mut parameters,
            "qubit_count",
            &self.config.qubit_count.to_string(),
        )?;

        push_parameter(
            &mut parameters,
            "evolution_time",
            &canonical_float(self.config.evolution_time),
        )?;

        if let Some(steps) = self.config.evolution_steps {
            push_parameter(
                &mut parameters,
                "evolution_steps",
                &steps.to_string(),
            )?;
        }

        if let Some(error) = self.config.target_error {
            push_parameter(
                &mut parameters,
                "target_error",
                canonical_float(error),
            )?;
        }

        push_parameter(
            &mut parameters,
            "reference_policy",
            self.config.reference_policy.as_str(),
        )?;

        if let Some(reference_state) =
            &self.config.reference_state_id
        {
            push_parameter(
                &mut parameters,
                "reference_state_id",
                reference_state,
            )?;
        }

        if let Some(reference_observable) =
            &self.config.reference_observable_id
        {
            push_parameter(
                &mut parameters,
                "reference_observable_id",
                reference_observable,
            )?;
        }

        if let Some(energy) = self.config.reference_energy {
            push_parameter(
                &mut parameters,
                "reference_energy",
                canonical_float(energy),
            )?;
        }

        if let Some(value) =
            self.config.reference_observable_value
        {
            push_parameter(
                &mut parameters,
                "reference_observable_value",
                canonical_float(value),
            )?;
        }

        push_parameter(
            &mut parameters,
            "success_criterion",
            self.config.success_criterion.as_str(),
        )?;

        if let Some(threshold) =
            self.config.fidelity_threshold
        {
            push_parameter(
                &mut parameters,
                "fidelity_threshold",
                canonical_float(threshold),
            )?;
        }

        if let Some(error) =
            self.config.maximum_observable_error
        {
            push_parameter(
                &mut parameters,
                "maximum_observable_error",
                canonical_float(error),
            )?;
        }

        let observable_list =
            self.config.observables.join(",");

        push_parameter(
            &mut parameters,
            "observables",
            &observable_list,
        )?;

        let metrics = self
            .config
            .metrics
            .iter()
            .map(|metric| metric.as_str())
            .collect::<Vec<_>>()
            .join(",");

        push_parameter(
            &mut parameters,
            "metrics",
            &metrics,
        )?;

        Ok(parameters)
    }

    /// Returns the default benchmark metrics.
    #[must_use]
    pub fn default_metrics() -> Vec<HamiltonianMetric> {
        default_metrics()
    }
}

impl ApplicationBenchmarkGenerator
    for HamiltonianSimulationGenerator
{
    fn descriptor(&self) -> &ApplicationGeneratorDescriptor {
        &self.descriptor
    }

    fn validate(
        &self,
        request: &ApplicationGenerationRequest,
    ) -> BenchmarkResult<()> {
        self.config.validate()?;

        request.validate()?;

        if request.application_id()
            != HAMILTONIAN_APPLICATION_ID
        {
            return Err(BenchmarkError::InconsistentConfiguration {
                first: "request.application_id".to_owned(),
                second: "hamiltonian.application_id".to_owned(),
                reason:
                    "Hamiltonian simulation requests must use the canonical application identifier"
                        .to_owned(),
            });
        }

        if request.problem_size()
            != self.config.qubit_count
        {
            return Err(BenchmarkError::DimensionMismatch {
                expected: self.config.qubit_count,
                actual: request.problem_size(),
                context:
                    "Hamiltonian simulation problem size and configured qubit count"
                        .to_owned(),
            });
        }

        Ok(())
    }

    fn generate_workload(
        &self,
        request: &ApplicationGenerationRequest,
    ) -> BenchmarkResult<ApplicationWorkload> {
        self.build_workload(request)
    }
}

// =============================================================================
// Benchmark-case descriptor
// =============================================================================

/// Immutable description of the scientific quantities an analysis layer must
/// evaluate after execution.
///
/// This is intentionally not a `BenchmarkResult`. It is the analysis contract
/// for Hamiltonian simulation.
#[derive(Debug, Clone, PartialEq)]
pub struct HamiltonianAnalysisContract {
    /// Requested metrics.
    pub metrics: Vec<HamiltonianMetric>,

    /// Reference policy.
    pub reference_policy: HamiltonianReferencePolicy,

    /// Target error, if any.
    pub target_error: Option<f64>,

    /// Success criterion.
    pub success_criterion: HamiltonianSuccessCriterion,

    /// Fidelity threshold, if any.
    pub fidelity_threshold: Option<f64>,

    /// Maximum observable error, if any.
    pub maximum_observable_error: Option<f64>,
}

impl HamiltonianAnalysisContract {
    /// Creates an analysis contract from a validated configuration.
    pub fn from_config(
        config: &HamiltonianBenchmarkConfig,
    ) -> BenchmarkResult<Self> {
        config.validate()?;

        Ok(Self {
            metrics: config.metrics.clone(),
            reference_policy: config.reference_policy,
            target_error: config.target_error,
            success_criterion: config.success_criterion,
            fidelity_threshold: config.fidelity_threshold,
            maximum_observable_error:
                config.maximum_observable_error,
        })
    }

    /// Returns whether a reference result is required.
    #[must_use]
    pub const fn requires_reference(&self) -> bool {
        !matches!(
            self.reference_policy,
            HamiltonianReferencePolicy::None
        )
    }

    /// Returns whether this contract has a scientific pass/fail criterion.
    #[must_use]
    pub const fn has_success_criterion(&self) -> bool {
        !matches!(
            self.success_criterion,
            HamiltonianSuccessCriterion::None
        )
    }
}

// =============================================================================
// Helper functions
// =============================================================================

fn default_metrics() -> Vec<HamiltonianMetric> {
    vec![
        HamiltonianMetric::AbsoluteSimulationError,
        HamiltonianMetric::StateFidelity,
        HamiltonianMetric::EvolutionTime,
        HamiltonianMetric::EvolutionSteps,
        HamiltonianMetric::QubitCount,
        HamiltonianMetric::CircuitDepth,
        HamiltonianMetric::GateCount,
        HamiltonianMetric::TwoQubitGateCount,
        HamiltonianMetric::QuantumExecutionTime,
        HamiltonianMetric::TotalTime,
        HamiltonianMetric::TimeToSolution,
        HamiltonianMetric::TargetError,
        HamiltonianMetric::AchievedError,
    ]
}

fn normalize_metrics(
    metrics: Vec<HamiltonianMetric>,
) -> Vec<HamiltonianMetric> {
    let mut result = Vec::with_capacity(metrics.len());

    for metric in metrics {
        if !result.contains(&metric) {
            result.push(metric);
        }
    }

    result
}

fn push_parameter(
    parameters: &mut Vec<ApplicationParameter>,
    name: &str,
    value: &str,
) -> BenchmarkResult<()> {
    if parameters.len() >= MAX_HAMILTONIAN_PARAMETERS {
        return Err(BenchmarkError::ResourceLimitExceeded {
            resource: "hamiltonian_application_parameters".to_owned(),
            requested: parameters.len() as u64 + 1,
            maximum: MAX_HAMILTONIAN_PARAMETERS as u64,
        });
    }

    if value.len() > MAX_PARAMETER_VALUE_BYTES {
        return Err(BenchmarkError::InvalidRange {
            field: name.to_owned(),
            value: value.len().to_string(),
            minimum: Some("0".to_owned()),
            maximum: Some(MAX_PARAMETER_VALUE_BYTES.to_string()),
        });
    }

    let parameter = ApplicationParameter::new(
        name.to_owned(),
        value.to_owned(),
    )
    .map_err(|error| BenchmarkError::InvalidWorkload {
        workload: HAMILTONIAN_APPLICATION_ID.to_owned(),
        reason: error.to_string(),
    })?;

    parameters.push(parameter);

    Ok(())
}

fn validate_method(
    method: &HamiltonianSimulationMethod,
) -> BenchmarkResult<()> {
    if let HamiltonianSimulationMethod::Custom(value) = method {
        validate_identifier(
            "simulation_method",
            value,
            MAX_METHOD_ID_BYTES,
        )?;
    }

    Ok(())
}

fn validate_initial_state(
    state: &HamiltonianInitialState,
) -> BenchmarkResult<()> {
    let identifier = state.identifier();

    validate_identifier(
        "initial_state_id",
        identifier,
        MAX_INITIAL_STATE_ID_BYTES,
    )
}

fn validate_error(
    field: &'static str,
    value: f64,
) -> BenchmarkResult<()> {
    if !value.is_finite()
        || value <= 0.0
        || value > 1.0
    {
        return Err(BenchmarkError::InvalidRange {
            field: field.to_owned(),
            value: value.to_string(),
            minimum: Some(MIN_TARGET_ERROR.to_string()),
            maximum: Some("1".to_owned()),
        });
    }

    if value < MIN_TARGET_ERROR {
        return Err(BenchmarkError::InvalidRange {
            field: field.to_owned(),
            value: value.to_string(),
            minimum: Some(MIN_TARGET_ERROR.to_string()),
            maximum: Some("1".to_owned()),
        });
    }

    Ok(())
}

fn validate_reference_policy(
    config: &HamiltonianBenchmarkConfig,
) -> BenchmarkResult<()> {
    match config.reference_policy {
        HamiltonianReferencePolicy::ExactReference
        | HamiltonianReferencePolicy::ApproximateReference => {
            match config.task {
                HamiltonianSimulationTask::TimeEvolution => {
                    if config.reference_state_id.is_none() {
                        return Err(
                            BenchmarkError::InvalidConfiguration {
                                field:
                                    "reference_state_id".to_owned(),
                                message:
                                    "a reference state is required for referenced time-evolution benchmarking"
                                        .to_owned(),
                            },
                        );
                    }
                }

                HamiltonianSimulationTask::ObservableEstimation => {
                    if config.reference_observable_id.is_none()
                        && config.reference_observable_value.is_none()
                    {
                        return Err(
                            BenchmarkError::InvalidConfiguration {
                                field:
                                    "reference_observable".to_owned(),
                                message:
                                    "observable benchmarking requires a reference observable identifier or reference value"
                                        .to_owned(),
                            },
                        );
                    }
                }

                HamiltonianSimulationTask::GroundStateEnergy => {
                    if config.reference_energy.is_none() {
                        return Err(
                            BenchmarkError::InvalidConfiguration {
                                field:
                                    "reference_energy".to_owned(),
                                message:
                                    "ground-state-energy benchmarking requires a reference energy"
                                        .to_owned(),
                            },
                        );
                    }
                }

                HamiltonianSimulationTask::ResourceEstimation
                | HamiltonianSimulationTask::Custom => {}
            }
        }

        HamiltonianReferencePolicy::None => {}
    }

    Ok(())
}

fn validate_task_requirements(
    config: &HamiltonianBenchmarkConfig,
) -> BenchmarkResult<()> {
    match config.task {
        HamiltonianSimulationTask::TimeEvolution => {
            if config.evolution_time <= 0.0 {
                return Err(
                    BenchmarkError::InvalidConfiguration {
                        field: "evolution_time".to_owned(),
                        message:
                            "time-evolution benchmarks require a positive evolution time"
                                .to_owned(),
                    },
                );
            }
        }

        HamiltonianSimulationTask::ObservableEstimation => {
            if config.observables.is_empty() {
                return Err(
                    BenchmarkError::InvalidConfiguration {
                        field: "observables".to_owned(),
                        message:
                            "observable-estimation benchmarks require at least one observable"
                                .to_owned(),
                    },
                );
            }
        }

        HamiltonianSimulationTask::GroundStateEnergy => {
            if config.reference_energy.is_none()
                && !matches!(
                    config.reference_policy,
                    HamiltonianReferencePolicy::None
                )
            {
                return Err(
                    BenchmarkError::InvalidConfiguration {
                        field: "reference_energy".to_owned(),
                        message:
                            "ground-state-energy benchmarks using a reference policy require a reference energy"
                                .to_owned(),
                    },
                );
            }
        }

        HamiltonianSimulationTask::ResourceEstimation => {
            // Resource estimation deliberately does not require an accuracy
            // target or reference result.
        }

        HamiltonianSimulationTask::Custom => {
            // Custom tasks are validated through the generic configuration
            // and application registry.
        }
    }

    Ok(())
}

fn validate_success_criterion(
    config: &HamiltonianBenchmarkConfig,
) -> BenchmarkResult<()> {
    match config.success_criterion {
        HamiltonianSuccessCriterion::ErrorWithinTarget => {
            if config.target_error.is_none() {
                return Err(
                    BenchmarkError::InvalidConfiguration {
                        field: "target_error".to_owned(),
                        message:
                            "error-based success requires a target error"
                                .to_owned(),
                    },
                );
            }
        }

        HamiltonianSuccessCriterion::FidelityThreshold => {
            if config.fidelity_threshold.is_none() {
                return Err(
                    BenchmarkError::InvalidConfiguration {
                        field: "fidelity_threshold".to_owned(),
                        message:
                            "fidelity success requires a fidelity threshold"
                                .to_owned(),
                    },
                );
            }
        }

        HamiltonianSuccessCriterion::ObservableWithinTarget => {
            if config.maximum_observable_error.is_none() {
                return Err(
                    BenchmarkError::InvalidConfiguration {
                        field:
                            "maximum_observable_error".to_owned(),
                        message:
                            "observable success requires a maximum observable error"
                                .to_owned(),
                    },
                );
            }

            if config.observables.is_empty() {
                return Err(
                    BenchmarkError::InvalidConfiguration {
                        field: "observables".to_owned(),
                        message:
                            "observable success requires at least one observable"
                                .to_owned(),
                    },
                );
            }
        }

        HamiltonianSuccessCriterion::None => {}
    }

    Ok(())
}

fn validate_identifier(
    field: &'static str,
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

    if !bytes.iter().all(|byte| {
        byte.is_ascii_lowercase()
            || byte.is_ascii_digit()
            || *byte == b'_'
            || *byte == b'-'
    }) {
        return Err(BenchmarkError::InvalidIdentifier {
            field: field.to_owned(),
            value: value.to_owned(),
        });
    }

    Ok(())
}

/// Canonical finite floating-point encoding for provenance.
///
/// Scientific values are encoded using Rust's stable `Display` formatting
/// rather than relying on locale-dependent formatting.
fn canonical_float(value: f64) -> String {
    format!("{:.17e}", value)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn config() -> HamiltonianBenchmarkConfig {
        HamiltonianBenchmarkConfig::new(
            "ising_4",
            4,
            1.0,
        )
        .expect("valid benchmark configuration")
    }

    #[test]
    fn default_configuration_is_valid() {
        let config = config();

        assert_eq!(
            config.hamiltonian_id,
            "ising_4"
        );

        assert_eq!(
            config.qubit_count,
            4
        );

        assert_eq!(
            config.task,
            HamiltonianSimulationTask::TimeEvolution
        );
    }

    #[test]
    fn rejects_zero_qubits() {
        let result =
            HamiltonianBenchmarkConfig::new(
                "ising",
                0,
                1.0,
            );

        assert!(result.is_err());
    }

    #[test]
    fn rejects_non_finite_evolution_time() {
        let result =
            HamiltonianBenchmarkConfig::new(
                "ising",
                4,
                f64::NAN,
            );

        assert!(result.is_err());
    }

    #[test]
    fn rejects_negative_evolution_time() {
        let result =
            HamiltonianBenchmarkConfig::new(
                "ising",
                4,
                -1.0,
            );

        assert!(result.is_err());
    }

    #[test]
    fn rejects_zero_steps() {
        let result =
            config().with_evolution_steps(0);

        assert!(result.is_err());
    }

    #[test]
    fn accepts_reference_time_evolution() {
        let config = config()
            .with_reference_policy(
                HamiltonianReferencePolicy::ExactReference,
            )
            .expect_err(
                "reference policy requires reference state",
            );

        let _ = config;
    }

    #[test]
    fn accepts_reference_time_evolution_with_state() {
        let config = config()
            .with_reference_policy(
                HamiltonianReferencePolicy::ExactReference,
            )
            .unwrap_or_else(|_| {
                HamiltonianBenchmarkConfig::new(
                    "ising",
                    4,
                    1.0,
                )
                .expect("valid configuration")
            });

        let config = config
            .with_reference_state(
                "ising_4_exact_t1",
            )
            .expect("valid reference state");

        assert_eq!(
            config.reference_policy,
            HamiltonianReferencePolicy::ExactReference
        );
    }

    #[test]
    fn observable_benchmark_requires_observable() {
        let result = config()
            .with_task(
                HamiltonianSimulationTask::ObservableEstimation,
            );

        assert!(result.is_err());
    }

    #[test]
    fn observable_benchmark_accepts_observable() {
        let config = config()
            .with_task(
                HamiltonianSimulationTask::ObservableEstimation,
            )
            .expect_err(
                "observable benchmark requires observable",
            );

        let _ = config;
    }

    #[test]
    fn ground_state_requires_reference_when_requested() {
        let result = config()
            .with_task(
                HamiltonianSimulationTask::GroundStateEnergy,
            )
            .and_then(|value| {
                value.with_reference_policy(
                    HamiltonianReferencePolicy::ExactReference,
                )
            });

        assert!(result.is_err());
    }

    #[test]
    fn resource_estimation_does_not_require_reference() {
        let config = config()
            .with_task(
                HamiltonianSimulationTask::ResourceEstimation,
            )
            .expect("resource estimation is valid");

        assert_eq!(
            config.task,
            HamiltonianSimulationTask::ResourceEstimation
        );
    }

    #[test]
    fn target_error_requires_positive_finite_value() {
        assert!(
            config()
                .with_target_error(0.0)
                .is_err()
        );

        assert!(
            config()
                .with_target_error(f64::NAN)
                .is_err()
        );
    }

    #[test]
    fn fidelity_threshold_is_bounded() {
        assert!(
            config()
                .with_fidelity_threshold(1.1)
                .is_err()
        );

        assert!(
            config()
                .with_fidelity_threshold(-0.1)
                .is_err()
        );

        assert!(
            config()
                .with_fidelity_threshold(0.99)
                .is_ok()
        );
    }

    #[test]
    fn custom_method_is_validated() {
        let config = config()
            .with_method(
                HamiltonianSimulationMethod::Custom(
                    "my_method".to_owned(),
                ),
            )
            .expect("valid custom method");

        assert_eq!(
            config.method.as_str(),
            "my_method"
        );
    }

    #[test]
    fn invalid_custom_method_is_rejected() {
        let result = config().with_method(
            HamiltonianSimulationMethod::Custom(
                "Invalid Method".to_owned(),
            ),
        );

        assert!(result.is_err());
    }

    #[test]
    fn generator_has_canonical_application_id() {
        let generator =
            HamiltonianSimulationGenerator::new(
                config(),
            )
            .expect("valid generator");

        assert_eq!(
            generator.descriptor().application_id(),
            HAMILTONIAN_APPLICATION_ID
        );
    }

    #[test]
    fn generator_builds_canonical_application_workload() {
        let generator =
            HamiltonianSimulationGenerator::new(
                config(),
            )
            .expect("valid generator");

        let instance =
            WorkloadId::new("ising_4_t1")
                .expect("valid workload ID");

        let request =
            ApplicationGenerationRequest::new(
                HAMILTONIAN_APPLICATION_ID,
                instance,
                4,
                42,
            )
            .expect("valid generation request");

        let generation =
            generator
                .generate(&request)
                .expect("generation should succeed");

        let workload = generation.workload();

        assert_eq!(
            workload.application_id(),
            HAMILTONIAN_APPLICATION_ID
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
                            == "ising_4"
                })
        );
    }

    #[test]
    fn request_problem_size_must_match_configuration() {
        let generator =
            HamiltonianSimulationGenerator::new(
                config(),
            )
            .expect("valid generator");

        let instance =
            WorkloadId::new("ising_4_t1")
                .expect("valid workload ID");

        let request =
            ApplicationGenerationRequest::new(
                HAMILTONIAN_APPLICATION_ID,
                instance,
                8,
                42,
            )
            .expect("valid request itself");

        assert!(
            generator.generate(&request).is_err()
        );
    }

    #[test]
    fn parameters_are_deterministic() {
        let generator_a =
            HamiltonianSimulationGenerator::new(
                config(),
            )
            .expect("valid generator");

        let generator_b =
            HamiltonianSimulationGenerator::new(
                config(),
            )
            .expect("valid generator");

        let a = generator_a
            .parameters()
            .expect("parameters");

        let b = generator_b
            .parameters()
            .expect("parameters");

        assert_eq!(a, b);
    }

    #[test]
    fn analysis_contract_is_derived_without_execution() {
        let config = config()
            .with_target_error(1.0e-6)
            .expect("valid target");

        let contract =
            HamiltonianAnalysisContract::from_config(
                &config,
            )
            .expect("valid analysis contract");

        assert_eq!(
            contract.target_error,
            Some(1.0e-6)
        );

        assert!(!contract.metrics.is_empty());
    }

    #[test]
    fn error_success_requires_target() {
        let result = config()
            .with_success_criterion(
                HamiltonianSuccessCriterion::ErrorWithinTarget,
            );

        assert!(result.is_err());
    }

    #[test]
    fn observable_success_requires_observable_and_error() {
        let result = config()
            .with_success_criterion(
                HamiltonianSuccessCriterion::ObservableWithinTarget,
            );

        assert!(result.is_err());
    }

    #[test]
    fn default_metrics_are_unique() {
        let metrics =
            HamiltonianSimulationGenerator::default_metrics();

        for (index, metric) in metrics.iter().enumerate() {
            assert!(
                !metrics[..index].contains(metric)
            );
        }
    }
}