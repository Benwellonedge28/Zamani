//! Zamani Quantum Benchmarking — HHL Application Benchmark
//!
//! Production-grade benchmark definition and workload generator for the
//! Harrow–Hassidim–Lloyd (HHL) quantum linear-system algorithm.
//!
//! # Scope
//!
//! This module defines the semantic benchmark boundary for HHL:
//!
//! - benchmark identity and version;
//! - linear-system problem metadata;
//! - matrix structural assumptions;
//! - condition-number metadata;
//! - sparsity metadata;
//! - precision requirements;
//! - input-state metadata;
//! - post-selection/resource metadata;
//! - success criteria;
//! - benchmark metrics;
//! - deterministic workload generation;
//! - bounded application parameters;
//! - optional Quantum IR circuit-construction boundary.
//!
//! This module does NOT:
//!
//! - execute HHL;
//! - implement quantum phase estimation;
//! - implement Hamiltonian simulation;
//! - implement controlled rotations;
//! - implement amplitude amplification;
//! - implement state preparation;
//! - implement matrix loading oracles;
//! - implement a classical linear solver;
//! - communicate with quantum hardware;
//! - perform routing;
//! - perform scheduling;
//! - perform calibration;
//! - perform statistical analysis;
//! - perform reporting;
//! - duplicate Quantum IR;
//! - claim a quantum advantage;
//! - return a complete classical solution vector as though HHL inherently
//!   produced one.
//!
//! Those responsibilities belong to the corresponding algorithm, compiler,
//! runtime, execution, hardware, statistics and reporting layers.
//!
//! # HHL benchmark model
//!
//! HHL addresses a linear system
//!
//! ```text
//! A x = b
//! ```
//!
//! by preparing a quantum state proportional to the solution,
//!
//! ```text
//! |x> ∝ A⁻¹ |b>.
//! ```
//!
//! The algorithm therefore has a fundamentally different output contract from
//! a classical linear solver. A benchmark must distinguish:
//!
//! 1. state-preparation success;
//! 2. HHL flag/post-selection success;
//! 3. solution-state fidelity;
//! 4. residual/error when classically evaluable;
//! 5. observable-estimation accuracy;
//! 6. circuit resources;
//! 7. execution time;
//! 8. repetition/post-selection overhead.
//!
//! A benchmark must never report "HHL solved Ax=b" merely because a circuit
//! executed successfully.
//!
//! # Scientific assumptions
//!
//! Canonical HHL-style benchmarks normally require or explicitly model:
//!
//! - a square matrix;
//! - a Hermitian matrix, or an explicitly documented Hermitian embedding;
//! - a normalized input state |b>;
//! - a non-singular/invertible problem or an explicitly bounded effective
//!   condition number;
//! - efficient state preparation;
//! - efficient access to A / Hamiltonian simulation;
//! - phase estimation or an equivalent spectral transformation;
//! - controlled eigenvalue inversion;
//! - post-selection or an equivalent success-amplification mechanism.
//!
//! The benchmark stores these assumptions as metadata instead of silently
//! assuming that they hold.
//!
//! # Condition number
//!
//! The condition number is a central HHL benchmark dimension:
//!
//! ```text
//! κ = λ_max / λ_min
//! ```
//!
//! for the relevant singular/eigenvalue magnitudes.
//!
//! Increasing κ generally makes the problem harder and increases the cost of
//! accurate eigenvalue inversion and/or post-selection. HHL resource
//! complexity is therefore reported as a parameterized resource model rather
//! than as one universal hard-coded runtime equation.
//!
//! # Post-selection
//!
//! For a normalized |b> and an HHL controlled-rotation scale C, the idealized
//! flag-success probability has the form
//!
//! ```text
//! p_success = C² ||A⁻¹ b||²
//! ```
//!
//! subject to the particular normalization/scaling and implementation.
//!
//! This file therefore does NOT fabricate an exact success probability from
//! κ alone. It can provide a conservative theoretical lower bound when the
//! benchmark's spectral normalization assumptions justify it, while the
//! actual measured probability must come from execution observations.
//!
//! # Benchmarking methodology
//!
//! HHL should be benchmarked over a parameter landscape rather than one
//! hand-picked matrix. Important sweep dimensions include:
//!
//! - system dimension N;
//! - matrix sparsity s;
//! - condition number κ;
//! - target precision ε;
//! - input-state structure;
//! - Hamiltonian-simulation strategy;
//! - phase-estimation precision;
//! - circuit decomposition;
//! - noise level;
//! - hardware topology;
//! - compilation/routing strategy.
//!
//! This is consistent with application-oriented quantum benchmarking work,
//! where workload size and algorithm parameters are varied while recording
//! quality, runtime and quantum resources.
//!
//! # Integration boundary
//!
//! ```text
//! HhlBenchmarkConfig
//!         │
//!         ▼
//! HhlBenchmarkGenerator
//!         │
//!         ▼
//! ApplicationGenerationRequest
//!         │
//!         ▼
//! ApplicationWorkload
//!         │
//!         ├──────────────► optional Quantum IR circuit
//!         │
//!         ▼
//! BenchmarkExperiment
//!         │
//!         ▼
//! BenchmarkExecutor
//!         │
//!         ▼
//! BenchmarkObservation
//!         │
//!         ▼
//! HHL analysis/statistics layer
//!         │
//!         ▼
//! BenchmarkResult
//! ```
//!
//! # Existing Zamani architecture
//!
//! The canonical application workload is:
//!
//! `crate::quantum::benchmarking::core::workload::ApplicationWorkload`
//!
//! The canonical application generator contract is:
//!
//! `crate::quantum::benchmarking::generators::application::ApplicationBenchmarkGenerator`
//!
//! This module uses those contracts rather than defining competing versions.
//!
//! # Quantum IR
//!
//! An optional circuit builder may return:
//!
//! `crate::quantum::ir::QuantumCircuit`
//!
//! The benchmark does not define a second circuit representation.
//!
//! # Rust compatibility
//!
//! - Rust 1.97
//! - Rust 1.97.1
//! - Rust 2021
//! - stable Rust
//! - no nightly features
//! - no unsafe code
//! - no additional dependencies
//!
//! # Security/resource model
//!
//! HHL benchmark declarations may eventually originate from Zamani source,
//! configuration files, CI, external benchmark definitions or APIs.
//!
//! This file therefore:
//!
//! - validates all identifiers;
//! - rejects zero dimensions;
//! - validates power-of-two requirements when requested;
//! - validates finite floating-point values;
//! - validates condition numbers;
//! - validates sparsity;
//! - validates precision;
//! - bounds parameter counts;
//! - bounds encoded metadata;
//! - uses checked arithmetic;
//! - does not allocate matrices;
//! - does not allocate solution vectors;
//! - does not execute user code;
//! - does not perform I/O;
//! - does not communicate with hardware.
//!
//! Large matrices belong to the execution/generator/resource-limit layer.
//! Merely describing a 2^30-dimensional linear system must not allocate 2^30
//! elements in this file.

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

/// Stable machine-readable HHL benchmark identifier.
pub const HHL_BENCHMARK_ID: &str = "hhl";

/// Stable application identifier.
pub const HHL_APPLICATION_ID: &str = "hhl";

/// Semantic version of the HHL benchmark definition.
///
/// This is independent from the Zamani package version, Quantum IR version,
/// backend version and HHL circuit implementation version.
pub const HHL_BENCHMARK_VERSION: u32 = 1;

/// Generator semantic revision.
pub const HHL_GENERATOR_REVISION: u32 = 1;

/// Generator implementation version.
pub const HHL_GENERATOR_VERSION: &str = "1.0.0";

/// Benchmark schema version.
pub const HHL_SCHEMA_VERSION: u16 = 1;

// =============================================================================
// Safety limits
// =============================================================================

/// Maximum UTF-8 byte length of an HHL identifier.
pub const MAX_HHL_IDENTIFIER_BYTES: usize = 128;

/// Maximum UTF-8 byte length of a matrix identifier.
pub const MAX_HHL_MATRIX_ID_BYTES: usize = 128;

/// Maximum UTF-8 byte length of an input-state identifier.
pub const MAX_HHL_STATE_ID_BYTES: usize = 128;

/// Maximum UTF-8 byte length of a simulation strategy identifier.
pub const MAX_HHL_SIMULATION_ID_BYTES: usize = 128;

/// Maximum UTF-8 byte length of a phase-estimation strategy identifier.
pub const MAX_HHL_QPE_ID_BYTES: usize = 128;

/// Maximum UTF-8 byte length of one encoded application parameter.
pub const MAX_HHL_PARAMETER_VALUE_BYTES: usize = 512;

/// Maximum number of application parameters represented directly.
pub const MAX_HHL_PARAMETERS: usize = 256;

/// Maximum logical system dimension representable by this semantic boundary.
///
/// This is deliberately a metadata limit, not an allocation request.
pub const MAX_HHL_SYSTEM_DIMENSION: usize = usize::MAX / 2;

/// Maximum matrix sparsity represented by the benchmark.
pub const MAX_HHL_SPARSITY: usize = usize::MAX / 2;

/// Maximum phase-estimation register width.
pub const MAX_HHL_PHASE_QUBITS: usize = 1024;

/// Maximum requested benchmark repetitions.
pub const MAX_HHL_REPETITIONS: usize = 10_000_000;

// =============================================================================
// Matrix assumptions
// =============================================================================

/// Matrix representation/structural class used by the HHL benchmark.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HhlMatrixClass {
    /// Real symmetric matrix.
    RealSymmetric,

    /// Complex Hermitian matrix.
    ComplexHermitian,

    /// Non-Hermitian matrix transformed through an explicit Hermitian
    /// embedding supplied by the implementation.
    HermitianEmbedded,

    /// General matrix accepted only by a benchmark implementation that
    /// explicitly documents the transformation it performs.
    GeneralWithDocumentedEmbedding,
}

impl HhlMatrixClass {
    /// Stable identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RealSymmetric => "real_symmetric",
            Self::ComplexHermitian => "complex_hermitian",
            Self::HermitianEmbedded => "hermitian_embedded",
            Self::GeneralWithDocumentedEmbedding => {
                "general_with_documented_embedding"
            }
        }
    }

    /// Returns whether the class is intrinsically Hermitian.
    #[must_use]
    pub const fn is_intrinsically_hermitian(self) -> bool {
        matches!(
            self,
            Self::RealSymmetric | Self::ComplexHermitian
        )
    }
}

impl fmt::Display for HhlMatrixClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// =============================================================================
// Input state model
// =============================================================================

/// Structure of the normalized input state |b>.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HhlInputStateKind {
    /// Computational-basis state.
    ComputationalBasis,

    /// Explicitly supplied amplitude-encoded state.
    AmplitudeEncoded,

    /// State generated by a benchmark-defined preparation circuit.
    PreparedCircuit,

    /// Problem-specific state-preparation oracle.
    OraclePrepared,

    /// User-defined state-preparation mechanism.
    Custom,
}

impl HhlInputStateKind {
    /// Stable identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ComputationalBasis => "computational_basis",
            Self::AmplitudeEncoded => "amplitude_encoded",
            Self::PreparedCircuit => "prepared_circuit",
            Self::OraclePrepared => "oracle_prepared",
            Self::Custom => "custom",
        }
    }
}

impl fmt::Display for HhlInputStateKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// =============================================================================
// Simulation model
// =============================================================================

/// Hamiltonian-simulation model used by the eventual HHL implementation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HhlSimulationModel {
    /// Exact simulation, normally suitable only for small reference cases.
    Exact,

    /// Trotter/Suzuki product formula.
    TrotterSuzuki,

    /// Block-encoding based simulation.
    BlockEncoding,

    /// Quantum-signal-processing/quantum-singular-value-transformation style
    /// implementation.
    Qsvt,

    /// Backend-specific simulation with externally supplied semantics.
    Custom,
}

impl HhlSimulationModel {
    /// Stable identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Exact => "exact",
            Self::TrotterSuzuki => "trotter_suzuki",
            Self::BlockEncoding => "block_encoding",
            Self::Qsvt => "qsvt",
            Self::Custom => "custom",
        }
    }
}

impl fmt::Display for HhlSimulationModel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// =============================================================================
// Phase estimation model
// =============================================================================

/// Phase/eigenvalue estimation strategy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HhlPhaseEstimationModel {
    /// Standard inverse-QFT based phase estimation.
    StandardQpe,

    /// Iterative phase estimation.
    Iterative,

    /// Maximum-likelihood / adaptive phase estimation.
    Adaptive,

    /// Backend-specific implementation.
    Custom,
}

impl HhlPhaseEstimationModel {
    /// Stable identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StandardQpe => "standard_qpe",
            Self::Iterative => "iterative",
            Self::Adaptive => "adaptive",
            Self::Custom => "custom",
        }
    }
}

impl fmt::Display for HhlPhaseEstimationModel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// =============================================================================
// Success criterion
// =============================================================================

/// Scientific criterion used to classify an HHL benchmark result.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HhlSuccessCriterion {
    /// Require successful post-selection only.
    PostSelection,

    /// Require solution-state fidelity above a threshold.
    SolutionStateFidelity,

    /// Require residual norm below a threshold.
    ResidualNorm,

    /// Require solution-state fidelity and residual quality.
    FidelityAndResidual,

    /// Require an observable estimate within an allowed error.
    ObservableError,

    /// Do not make a scientific pass/fail decision.
    None,
}

impl HhlSuccessCriterion {
    /// Stable identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PostSelection => "post_selection",
            Self::SolutionStateFidelity => "solution_state_fidelity",
            Self::ResidualNorm => "residual_norm",
            Self::FidelityAndResidual => "fidelity_and_residual",
            Self::ObservableError => "observable_error",
            Self::None => "none",
        }
    }
}

impl fmt::Display for HhlSuccessCriterion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// =============================================================================
// Metrics
// =============================================================================

/// HHL-specific benchmark metric.
///
/// The generic benchmarking metric layer owns units, uncertainty and
/// provenance. This enum only identifies what should be measured.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HhlMetric {
    /// System dimension N.
    SystemDimension,

    /// Number of logical system qubits.
    SystemQubits,

    /// Number of phase-estimation qubits.
    PhaseQubits,

    /// Number of ancilla qubits.
    AncillaQubits,

    /// Total logical qubits.
    TotalLogicalQubits,

    /// Matrix sparsity.
    Sparsity,

    /// Matrix condition number.
    ConditionNumber,

    /// Matrix spectral scale.
    SpectralNorm,

    /// Target precision.
    Precision,

    /// Measured post-selection probability.
    PostSelectionProbability,

    /// Theoretical lower bound on post-selection probability when justified.
    PostSelectionLowerBound,

    /// Expected repetitions due to post-selection.
    ExpectedPostSelectionRepetitions,

    /// Solution-state fidelity.
    SolutionStateFidelity,

    /// Residual norm.
    ResidualNorm,

    /// Relative residual norm.
    RelativeResidualNorm,

    /// Observable estimation error.
    ObservableError,

    /// Circuit depth.
    CircuitDepth,

    /// Total gate count.
    GateCount,

    /// Two-qubit gate count.
    TwoQubitGateCount,

    /// Hamiltonian-simulation cost.
    SimulationCost,

    /// Phase-estimation cost.
    PhaseEstimationCost,

    /// State-preparation cost.
    StatePreparationCost,

    /// Total logical circuit volume.
    CircuitVolume,

    /// Number of executions.
    CircuitExecutions,

    /// Number of shots.
    Shots,

    /// Quantum execution time.
    QuantumExecutionTime,

    /// Classical preprocessing time.
    ClassicalPreprocessingTime,

    /// Total end-to-end time.
    TotalTime,

    /// Time-to-solution.
    TimeToSolution,

    /// Whether the scientific success criterion passed.
    Success,
}

impl HhlMetric {
    /// Stable machine-readable metric identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SystemDimension => "system_dimension",
            Self::SystemQubits => "system_qubits",
            Self::PhaseQubits => "phase_qubits",
            Self::AncillaQubits => "ancilla_qubits",
            Self::TotalLogicalQubits => "total_logical_qubits",
            Self::Sparsity => "sparsity",
            Self::ConditionNumber => "condition_number",
            Self::SpectralNorm => "spectral_norm",
            Self::Precision => "precision",
            Self::PostSelectionProbability => {
                "post_selection_probability"
            }
            Self::PostSelectionLowerBound => {
                "post_selection_lower_bound"
            }
            Self::ExpectedPostSelectionRepetitions => {
                "expected_post_selection_repetitions"
            }
            Self::SolutionStateFidelity => {
                "solution_state_fidelity"
            }
            Self::ResidualNorm => "residual_norm",
            Self::RelativeResidualNorm => {
                "relative_residual_norm"
            }
            Self::ObservableError => "observable_error",
            Self::CircuitDepth => "circuit_depth",
            Self::GateCount => "gate_count",
            Self::TwoQubitGateCount => {
                "two_qubit_gate_count"
            }
            Self::SimulationCost => "simulation_cost",
            Self::PhaseEstimationCost => {
                "phase_estimation_cost"
            }
            Self::StatePreparationCost => {
                "state_preparation_cost"
            }
            Self::CircuitVolume => "circuit_volume",
            Self::CircuitExecutions => {
                "circuit_executions"
            }
            Self::Shots => "shots",
            Self::QuantumExecutionTime => {
                "quantum_execution_time"
            }
            Self::ClassicalPreprocessingTime => {
                "classical_preprocessing_time"
            }
            Self::TotalTime => "total_time",
            Self::TimeToSolution => "time_to_solution",
            Self::Success => "success",
        }
    }
}

impl fmt::Display for HhlMetric {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// =============================================================================
// HHL benchmark configuration
// =============================================================================

/// Semantic configuration for one HHL benchmark instance.
///
/// Execution concerns such as backend, shots, compiler, routing and timeout
/// remain owned by the generic benchmarking configuration.
#[derive(Debug, Clone, PartialEq)]
pub struct HhlBenchmarkConfig {
    /// Stable matrix/problem identifier.
    pub matrix_id: String,

    /// Mathematical dimension N of A.
    pub system_dimension: usize,

    /// Matrix structural class.
    pub matrix_class: HhlMatrixClass,

    /// Maximum number of non-zero entries per row/column, when known.
    pub sparsity: usize,

    /// Spectral condition number κ.
    pub condition_number: f64,

    /// Spectral normalization ||A|| or an explicitly documented equivalent.
    pub spectral_norm: f64,

    /// Target algorithmic precision ε.
    pub precision: f64,

    /// Input-state preparation class.
    pub input_state_kind: HhlInputStateKind,

    /// Stable input-state identifier.
    pub input_state_id: String,

    /// Hamiltonian simulation strategy.
    pub simulation_model: HhlSimulationModel,

    /// Phase-estimation strategy.
    pub phase_estimation_model: HhlPhaseEstimationModel,

    /// Number of phase-estimation qubits.
    pub phase_qubits: usize,

    /// Number of additional HHL ancilla qubits.
    pub ancilla_qubits: usize,

    /// Whether the benchmark assumes eigenvalues have been scaled so their
    /// relevant magnitudes lie in a documented normalized interval.
    pub normalized_spectrum: bool,

    /// Optional known norm of the exact solution x = A⁻¹b.
    ///
    /// This enables an ideal post-selection probability calculation when the
    /// scaling constant is explicitly supplied.
    pub exact_solution_norm: Option<f64>,

    /// Controlled-rotation scaling constant C.
    ///
    /// If absent, the benchmark does not fabricate an exact post-selection
    /// probability.
    pub rotation_scale: Option<f64>,

    /// Success criterion.
    pub success_criterion: HhlSuccessCriterion,

    /// Required minimum solution-state fidelity.
    pub minimum_solution_fidelity: Option<f64>,

    /// Maximum permitted residual norm.
    pub maximum_residual_norm: Option<f64>,

    /// Maximum permitted observable error.
    pub maximum_observable_error: Option<f64>,

    /// Requested metrics.
    pub metrics: Vec<HhlMetric>,
}

impl HhlBenchmarkConfig {
    /// Creates a validated HHL benchmark configuration.
    pub fn new(
        matrix_id: impl Into<String>,
        system_dimension: usize,
        sparsity: usize,
        condition_number: f64,
        precision: f64,
    ) -> BenchmarkResult<Self> {
        let config = Self {
            matrix_id: matrix_id.into(),
            system_dimension,
            matrix_class: HhlMatrixClass::RealSymmetric,
            sparsity,
            condition_number,
            spectral_norm: 1.0,
            precision,
            input_state_kind: HhlInputStateKind::ComputationalBasis,
            input_state_id: "computational_basis".to_owned(),
            simulation_model: HhlSimulationModel::Exact,
            phase_estimation_model: HhlPhaseEstimationModel::StandardQpe,
            phase_qubits: 4,
            ancilla_qubits: 1,
            normalized_spectrum: true,
            exact_solution_norm: None,
            rotation_scale: None,
            success_criterion: HhlSuccessCriterion::None,
            minimum_solution_fidelity: None,
            maximum_residual_norm: None,
            maximum_observable_error: None,
            metrics: default_metrics(),
        };

        let mut config = config;
        config.validate()?;

        Ok(config)
    }

    /// Changes the matrix class.
    pub fn with_matrix_class(
        mut self,
        matrix_class: HhlMatrixClass,
    ) -> BenchmarkResult<Self> {
        self.matrix_class = matrix_class;
        self.validate()?;
        Ok(self)
    }

    /// Sets the input-state representation.
    pub fn with_input_state(
        mut self,
        kind: HhlInputStateKind,
        id: impl Into<String>,
    ) -> BenchmarkResult<Self> {
        self.input_state_kind = kind;
        self.input_state_id = id.into();
        self.validate()?;
        Ok(self)
    }

    /// Selects the Hamiltonian-simulation model.
    pub fn with_simulation_model(
        mut self,
        model: HhlSimulationModel,
    ) -> BenchmarkResult<Self> {
        self.simulation_model = model;
        self.validate()?;
        Ok(self)
    }

    /// Selects the phase-estimation model and register width.
    pub fn with_phase_estimation(
        mut self,
        model: HhlPhaseEstimationModel,
        phase_qubits: usize,
    ) -> BenchmarkResult<Self> {
        self.phase_estimation_model = model;
        self.phase_qubits = phase_qubits;
        self.validate()?;
        Ok(self)
    }

    /// Supplies an exact solution norm.
    pub fn with_exact_solution_norm(
        mut self,
        norm: f64,
    ) -> BenchmarkResult<Self> {
        self.exact_solution_norm = Some(norm);
        self.validate()?;
        Ok(self)
    }

    /// Supplies the controlled-rotation scale C.
    pub fn with_rotation_scale(
        mut self,
        scale: f64,
    ) -> BenchmarkResult<Self> {
        self.rotation_scale = Some(scale);
        self.validate()?;
        Ok(self)
    }

    /// Selects the scientific success criterion.
    pub fn with_success_criterion(
        mut self,
        criterion: HhlSuccessCriterion,
    ) -> BenchmarkResult<Self> {
        self.success_criterion = criterion;
        self.validate()?;
        Ok(self)
    }

    /// Sets the minimum solution-state fidelity.
    pub fn with_minimum_solution_fidelity(
        mut self,
        fidelity: f64,
    ) -> BenchmarkResult<Self> {
        self.minimum_solution_fidelity = Some(fidelity);
        self.validate()?;
        Ok(self)
    }

    /// Sets the maximum residual norm.
    pub fn with_maximum_residual_norm(
        mut self,
        residual: f64,
    ) -> BenchmarkResult<Self> {
        self.maximum_residual_norm = Some(residual);
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
    pub fn with_metrics<I>(
        mut self,
        metrics: I,
    ) -> BenchmarkResult<Self>
    where
        I: IntoIterator<Item = HhlMetric>,
    {
        self.metrics = metrics.into_iter().collect();
        self.validate()?;
        Ok(self)
    }

    /// Returns whether the success criterion requires fidelity.
    #[must_use]
    pub const fn requires_fidelity(&self) -> bool {
        matches!(
            self.success_criterion,
            HhlSuccessCriterion::SolutionStateFidelity
                | HhlSuccessCriterion::FidelityAndResidual
        )
    }

    /// Returns whether the success criterion requires residual quality.
    #[must_use]
    pub const fn requires_residual(&self) -> bool {
        matches!(
            self.success_criterion,
            HhlSuccessCriterion::ResidualNorm
                | HhlSuccessCriterion::FidelityAndResidual
        )
    }

    /// Returns whether the success criterion requires observable error.
    #[must_use]
    pub const fn requires_observable_error(&self) -> bool {
        matches!(
            self.success_criterion,
            HhlSuccessCriterion::ObservableError
        )
    }

    /// Validates all HHL-specific semantics.
    pub fn validate(&self) -> BenchmarkResult<()> {
        validate_identifier(
            "hhl.matrix_id",
            &self.matrix_id,
            MAX_HHL_MATRIX_ID_BYTES,
        )?;

        validate_identifier(
            "hhl.input_state_id",
            &self.input_state_id,
            MAX_HHL_STATE_ID_BYTES,
        )?;

        if self.system_dimension == 0 {
            return Err(BenchmarkError::InvalidRange {
                field: "hhl.system_dimension".to_owned(),
                value: "0".to_owned(),
                minimum: Some("1".to_owned()),
                maximum: Some(
                    MAX_HHL_SYSTEM_DIMENSION.to_string(),
                ),
            });
        }

        if self.system_dimension > MAX_HHL_SYSTEM_DIMENSION {
            return Err(BenchmarkError::ResourceLimitExceeded {
                resource: "hhl.system_dimension".to_owned(),
                requested: self.system_dimension as u64,
                maximum: MAX_HHL_SYSTEM_DIMENSION as u64,
            });
        }

        if !self.system_dimension.is_power_of_two() {
            return Err(BenchmarkError::InvalidConfiguration {
                field: "hhl.system_dimension".to_owned(),
                reason:
                    "the canonical HHL circuit boundary requires a power-of-two system dimension; use an explicit embedding/padding layer before generation"
                        .to_owned(),
            });
        }

        if self.sparsity == 0 {
            return Err(BenchmarkError::InvalidRange {
                field: "hhl.sparsity".to_owned(),
                value: "0".to_owned(),
                minimum: Some("1".to_owned()),
                maximum: Some(
                    MAX_HHL_SPARSITY.to_string(),
                ),
            });
        }

        if self.sparsity > self.system_dimension {
            return Err(BenchmarkError::InvalidConfiguration {
                field: "hhl.sparsity".to_owned(),
                reason:
                    "row/column sparsity cannot exceed the matrix dimension"
                        .to_owned(),
            });
        }

        validate_positive_finite(
            "hhl.condition_number",
            self.condition_number,
        )?;

        if self.condition_number < 1.0 {
            return Err(BenchmarkError::InvalidConfiguration {
                field: "hhl.condition_number".to_owned(),
                reason:
                    "the spectral condition number must be at least one"
                        .to_owned(),
            });
        }

        validate_positive_finite(
            "hhl.spectral_norm",
            self.spectral_norm,
        )?;

        validate_positive_finite(
            "hhl.precision",
            self.precision,
        )?;

        if self.precision >= 1.0 {
            return Err(BenchmarkError::InvalidConfiguration {
                field: "hhl.precision".to_owned(),
                reason:
                    "HHL precision must be strictly less than one"
                        .to_owned(),
            });
        }

        validate_identifier(
            "hhl.simulation_model",
            self.simulation_model.as_str(),
            MAX_HHL_SIMULATION_ID_BYTES,
        )?;

        validate_identifier(
            "hhl.phase_estimation_model",
            self.phase_estimation_model.as_str(),
            MAX_HHL_QPE_ID_BYTES,
        )?;

        if self.phase_qubits == 0 {
            return Err(BenchmarkError::InvalidRange {
                field: "hhl.phase_qubits".to_owned(),
                value: "0".to_owned(),
                minimum: Some("1".to_owned()),
                maximum: Some(
                    MAX_HHL_PHASE_QUBITS.to_string(),
                ),
            });
        }

        if self.phase_qubits > MAX_HHL_PHASE_QUBITS {
            return Err(BenchmarkError::ResourceLimitExceeded {
                resource: "hhl.phase_qubits".to_owned(),
                requested: self.phase_qubits as u64,
                maximum: MAX_HHL_PHASE_QUBITS as u64,
            });
        }

        if self.ancilla_qubits == 0 {
            return Err(BenchmarkError::InvalidConfiguration {
                field: "hhl.ancilla_qubits".to_owned(),
                reason:
                    "HHL requires at least one flag/rotation ancilla in the canonical formulation"
                        .to_owned(),
            });
        }

        if let Some(norm) = self.exact_solution_norm {
            validate_positive_finite(
                "hhl.exact_solution_norm",
                norm,
            )?;
        }

        if let Some(scale) = self.rotation_scale {
            validate_positive_finite(
                "hhl.rotation_scale",
                scale,
            )?;

            if scale > 1.0 {
                return Err(BenchmarkError::InvalidConfiguration {
                    field: "hhl.rotation_scale".to_owned(),
                    reason:
                        "the controlled-rotation scale must not exceed one"
                            .to_owned(),
                });
            }
        }

        if self.rotation_scale.is_some()
            && self.exact_solution_norm.is_none()
        {
            return Err(BenchmarkError::InvalidConfiguration {
                field: "hhl.exact_solution_norm".to_owned(),
                reason:
                    "an exact rotation scale requires the corresponding solution norm to be supplied if an exact ideal post-selection probability is to be derived"
                        .to_owned(),
            });
        }

        if let Some(fidelity) = self.minimum_solution_fidelity {
            if !fidelity.is_finite()
                || !(0.0..=1.0).contains(&fidelity)
            {
                return Err(BenchmarkError::InvalidConfiguration {
                    field: "hhl.minimum_solution_fidelity"
                        .to_owned(),
                    reason:
                        "solution fidelity must be finite and in [0, 1]"
                            .to_owned(),
                });
            }
        }

        if let Some(residual) = self.maximum_residual_norm {
            validate_non_negative_finite(
                "hhl.maximum_residual_norm",
                residual,
            )?;
        }

        if let Some(error) = self.maximum_observable_error {
            validate_non_negative_finite(
                "hhl.maximum_observable_error",
                error,
            )?;
        }

        match self.success_criterion {
            HhlSuccessCriterion::SolutionStateFidelity
            | HhlSuccessCriterion::FidelityAndResidual => {
                if self.minimum_solution_fidelity.is_none() {
                    return Err(BenchmarkError::InvalidConfiguration {
                        field:
                            "hhl.minimum_solution_fidelity"
                                .to_owned(),
                        reason:
                            "a fidelity success criterion requires a fidelity threshold"
                                .to_owned(),
                    });
                }
            }

            HhlSuccessCriterion::ResidualNorm
            | HhlSuccessCriterion::FidelityAndResidual => {
                if self.maximum_residual_norm.is_none() {
                    return Err(BenchmarkError::InvalidConfiguration {
                        field:
                            "hhl.maximum_residual_norm"
                                .to_owned(),
                        reason:
                            "a residual success criterion requires a residual threshold"
                                .to_owned(),
                    });
                }
            }

            HhlSuccessCriterion::ObservableError => {
                if self.maximum_observable_error.is_none() {
                    return Err(BenchmarkError::InvalidConfiguration {
                        field:
                            "hhl.maximum_observable_error"
                                .to_owned(),
                        reason:
                            "an observable-error success criterion requires an error threshold"
                                .to_owned(),
                    });
                }
            }

            HhlSuccessCriterion::PostSelection
            | HhlSuccessCriterion::None => {}
        }

        canonicalize_metrics(self.metrics.clone())?;

        Ok(())
    }
}

// =============================================================================
// Workload descriptor
// =============================================================================

/// Stable semantic description of an HHL workload.
///
/// This descriptor is deliberately independent of execution backend and
/// circuit representation.
#[derive(Debug, Clone, PartialEq)]
pub struct HhlWorkloadDescriptor {
    /// Matrix identifier.
    pub matrix_id: String,

    /// Matrix dimension.
    pub system_dimension: usize,

    /// Logical system-qubit count.
    pub system_qubits: usize,

    /// Matrix class.
    pub matrix_class: HhlMatrixClass,

    /// Sparsity.
    pub sparsity: usize,

    /// Condition number.
    pub condition_number: f64,

    /// Spectral norm.
    pub spectral_norm: f64,

    /// Target precision.
    pub precision: f64,

    /// Input-state kind.
    pub input_state_kind: HhlInputStateKind,

    /// Input-state identifier.
    pub input_state_id: String,

    /// Simulation model.
    pub simulation_model: HhlSimulationModel,

    /// Phase-estimation model.
    pub phase_estimation_model: HhlPhaseEstimationModel,

    /// Phase-register width.
    pub phase_qubits: usize,

    /// Ancilla count.
    pub ancilla_qubits: usize,

    /// Total logical qubits under the benchmark's logical decomposition.
    pub total_logical_qubits: usize,

    /// Success criterion.
    pub success_criterion: HhlSuccessCriterion,
}

impl HhlWorkloadDescriptor {
    /// Constructs a descriptor from validated configuration.
    pub fn from_config(
        config: &HhlBenchmarkConfig,
    ) -> BenchmarkResult<Self> {
        config.validate()?;

        let system_qubits =
            log2_exact_power_of_two(
                config.system_dimension,
            )?;

        let total_logical_qubits =
            checked_add(
                system_qubits,
                config.phase_qubits,
                "HHL logical qubit count",
            )?
            .checked_add(config.ancilla_qubits)
            .ok_or_else(|| {
                BenchmarkError::InvalidConfiguration {
                    field:
                        "hhl.total_logical_qubits"
                            .to_owned(),
                    reason:
                        "logical qubit count overflow"
                            .to_owned(),
                }
            })?;

        Ok(Self {
            matrix_id: config.matrix_id.clone(),
            system_dimension: config.system_dimension,
            system_qubits,
            matrix_class: config.matrix_class,
            sparsity: config.sparsity,
            condition_number: config.condition_number,
            spectral_norm: config.spectral_norm,
            precision: config.precision,
            input_state_kind: config.input_state_kind,
            input_state_id: config.input_state_id.clone(),
            simulation_model: config.simulation_model,
            phase_estimation_model:
                config.phase_estimation_model,
            phase_qubits: config.phase_qubits,
            ancilla_qubits: config.ancilla_qubits,
            total_logical_qubits,
            success_criterion: config.success_criterion,
        })
    }
}

// =============================================================================
// Resource model
// =============================================================================

/// Resource estimate returned by the HHL benchmark's semantic layer.
///
/// These are benchmark-model estimates, not measured hardware results.
///
/// The benchmark intentionally keeps assumptions visible.
#[derive(Debug, Clone, PartialEq)]
pub struct HhlResourceEstimate {
    /// System qubits.
    pub system_qubits: usize,

    /// Phase-estimation qubits.
    pub phase_qubits: usize,

    /// Ancilla qubits.
    pub ancilla_qubits: usize,

    /// Total logical qubits.
    pub total_logical_qubits: usize,

    /// Sparsity.
    pub sparsity: usize,

    /// Condition number.
    pub condition_number: f64,

    /// Target precision.
    pub precision: f64,

    /// Approximate phase-estimation precision contribution.
    pub phase_precision_scale: f64,

    /// Theoretical post-selection lower bound when justified.
    pub post_selection_lower_bound: Option<f64>,

    /// Expected repetitions corresponding to the supplied exact probability
    /// model or conservative lower bound.
    pub expected_post_selection_repetitions: Option<f64>,

    /// Whether the resource estimate assumes normalized spectrum.
    pub normalized_spectrum_assumption: bool,
}

impl HhlResourceEstimate {
    /// Builds a resource estimate from a validated configuration.
    pub fn from_config(
        config: &HhlBenchmarkConfig,
    ) -> BenchmarkResult<Self> {
        config.validate()?;

        let descriptor =
            HhlWorkloadDescriptor::from_config(config)?;

        let phase_precision_scale =
            2.0_f64.powi(-(config.phase_qubits as i32));

        let lower_bound =
            theoretical_post_selection_lower_bound(
                config,
            )?;

        let probability_for_repetition =
            if let Some(scale) = config.rotation_scale {
                let norm = config
                    .exact_solution_norm
                    .expect(
                        "validated rotation scale requires solution norm",
                    );

                let probability =
                    scale * scale * norm * norm;

                if probability.is_finite()
                    && probability > 0.0
                    && probability <= 1.0
                {
                    Some(probability)
                } else {
                    lower_bound
                }
            } else {
                lower_bound
            };

        let expected_repetitions =
            probability_for_repetition
                .map(|probability| 1.0 / probability);

        Ok(Self {
            system_qubits: descriptor.system_qubits,
            phase_qubits: descriptor.phase_qubits,
            ancilla_qubits: descriptor.ancilla_qubits,
            total_logical_qubits:
                descriptor.total_logical_qubits,
            sparsity: descriptor.sparsity,
            condition_number:
                descriptor.condition_number,
            precision: descriptor.precision,
            phase_precision_scale,
            post_selection_lower_bound:
                lower_bound,
            expected_post_selection_repetitions:
                expected_repetitions,
            normalized_spectrum_assumption:
                config.normalized_spectrum,
        })
    }
}

// =============================================================================
// Post-selection analysis
// =============================================================================

/// Returns the idealized post-selection probability when the caller supplies
/// enough information to calculate it exactly.
///
/// Formula:
///
/// ```text
/// p = C² ||A⁻¹b||²
/// ```
///
/// The function does not infer C or ||A⁻¹b|| from κ alone.
pub fn ideal_post_selection_probability(
    config: &HhlBenchmarkConfig,
) -> BenchmarkResult<Option<f64>> {
    config.validate()?;

    let scale = match config.rotation_scale {
        Some(value) => value,
        None => return Ok(None),
    };

    let norm = match config.exact_solution_norm {
        Some(value) => value,
        None => return Ok(None),
    };

    let probability = scale * scale * norm * norm;

    if !probability.is_finite()
        || !(0.0..=1.0).contains(&probability)
    {
        return Err(BenchmarkError::InvalidConfiguration {
            field:
                "hhl.ideal_post_selection_probability"
                    .to_owned(),
            reason:
                "derived probability is outside [0, 1]; check matrix scaling, solution norm and rotation scale"
                    .to_owned(),
        });
    }

    Ok(Some(probability))
}

/// Returns a conservative theoretical post-selection lower bound when the
/// standard normalized-spectrum assumptions justify it.
///
/// Under the simplified normalized HHL model with:
///
/// - ||A|| <= 1;
/// - κ >= 1;
/// - normalized |b>;
/// - C chosen on the order of 1/κ;
///
/// the benchmark can use an O(1/κ²) lower-bound model.
///
/// The exact constant depends on the implementation/scaling convention, so
/// Zamani deliberately exposes this as a benchmark-model lower bound rather
/// than a universal physical law.
pub fn theoretical_post_selection_lower_bound(
    config: &HhlBenchmarkConfig,
) -> BenchmarkResult<Option<f64>> {
    config.validate()?;

    if !config.normalized_spectrum {
        return Ok(None);
    }

    let kappa_squared =
        config.condition_number
            * config.condition_number;

    if !kappa_squared.is_finite()
        || kappa_squared <= 0.0
    {
        return Err(BenchmarkError::InvalidConfiguration {
            field:
                "hhl.condition_number".to_owned(),
            reason:
                "condition number cannot produce a finite squared value"
                    .to_owned(),
        });
    }

    let lower_bound = 1.0 / kappa_squared;

    if !lower_bound.is_finite()
        || lower_bound <= 0.0
        || lower_bound > 1.0
    {
        return Err(BenchmarkError::InvalidConfiguration {
            field:
                "hhl.post_selection_lower_bound"
                    .to_owned(),
            reason:
                "derived post-selection lower bound is invalid"
                    .to_owned(),
        });
    }

    Ok(Some(lower_bound))
}

/// Calculates the expected number of independent HHL executions required for
/// one successful post-selection event.
///
/// This is:
///
/// ```text
/// E[R] = 1 / p_success
/// ```
///
/// where p_success is either the exact supplied ideal probability or the
/// conservative theoretical lower bound.
pub fn expected_post_selection_repetitions(
    config: &HhlBenchmarkConfig,
) -> BenchmarkResult<Option<f64>> {
    config.validate()?;

    let probability =
        match ideal_post_selection_probability(config)? {
            Some(value) => Some(value),
            None => theoretical_post_selection_lower_bound(config)?,
        };

    Ok(probability.map(|value| 1.0 / value))
}

// =============================================================================
// Optional circuit-construction boundary
// =============================================================================

/// Optional HHL logical circuit builder.
///
/// The implementation is intentionally supplied by the caller. This prevents
/// the benchmark from defining a second HHL circuit implementation or a second
/// Quantum IR.
///
/// A builder may use:
///
/// - the canonical HHL algorithm implementation;
/// - a future quantum linear-system module;
/// - a reference circuit generator;
/// - a hardware-aware logical circuit generator.
///
/// It must not communicate with hardware.
pub trait HhlCircuitBuilder: Send + Sync {
    /// Builds the logical Quantum IR circuit for the configured HHL instance.
    fn build(
        &self,
        config: &HhlBenchmarkConfig,
    ) -> BenchmarkResult<crate::quantum::ir::QuantumCircuit>;

    /// Returns the system-qubit count represented by the builder.
    fn system_qubits(&self) -> usize;

    /// Returns the phase-register width represented by the builder.
    fn phase_qubits(&self) -> usize;

    /// Returns the ancilla count represented by the builder.
    fn ancilla_qubits(&self) -> usize;
}

// =============================================================================
// HHL benchmark generator
// =============================================================================

/// Production HHL application benchmark generator.
///
/// The generator is stateless and safe to share between registry instances.
#[derive(Debug, Clone)]
pub struct HhlBenchmarkGenerator {
    descriptor: ApplicationGeneratorDescriptor,
}

impl Default for HhlBenchmarkGenerator {
    fn default() -> Self {
        Self::new()
            .expect(
                "static HHL benchmark generator descriptor must be valid",
            )
    }
}

impl HhlBenchmarkGenerator {
    /// Creates the canonical HHL benchmark generator.
    pub fn new() -> BenchmarkResult<Self> {
        let descriptor =
            ApplicationGeneratorDescriptor::new(
                HHL_BENCHMARK_ID,
                HHL_APPLICATION_ID,
                HHL_GENERATOR_VERSION,
                "Production HHL quantum linear-system application benchmark generator",
            )?
            .with_capabilities([
                ApplicationGeneratorCapability::GeneratesCircuit,
                ApplicationGeneratorCapability::Deterministic,
                ApplicationGeneratorCapability::BatchGeneration,
                ApplicationGeneratorCapability::ScalableProblemSize,
                ApplicationGeneratorCapability::Parameterized,
                ApplicationGeneratorCapability::ExactSmallInstanceReference,
                ApplicationGeneratorCapability::ClassicallyVerifiable,
                ApplicationGeneratorCapability::ResourceEstimation,
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

    /// Builds an application-generation request from HHL configuration.
    ///
    /// Execution properties such as shots, backend, timeout, compiler,
    /// routing and scheduling are intentionally absent.
    pub fn request(
        config: &HhlBenchmarkConfig,
        instance_id: WorkloadId,
        seed: u64,
    ) -> BenchmarkResult<ApplicationGenerationRequest> {
        config.validate()?;

        let mut request =
            ApplicationGenerationRequest::new(
                HHL_APPLICATION_ID,
                instance_id,
                config.system_dimension,
                seed,
            )?
            .with_generator_revision(
                HHL_GENERATOR_REVISION,
            );

        request = request.with_parameters(
            encoded_parameters(config)?,
        )?;

        Ok(request)
    }

    /// Generates one canonical HHL application workload.
    pub fn generate_from_config(
        &self,
        config: &HhlBenchmarkConfig,
        instance_id: WorkloadId,
        seed: u64,
    ) -> BenchmarkResult<ApplicationWorkload> {
        let request =
            Self::request(config, instance_id, seed)?;

        self.generate_workload(&request)
    }

    /// Generates one HHL workload and attaches an optional logical Quantum IR
    /// circuit.
    pub fn generate_with_circuit_builder(
        &self,
        config: &HhlBenchmarkConfig,
        instance_id: WorkloadId,
        seed: u64,
        builder: &dyn HhlCircuitBuilder,
    ) -> BenchmarkResult<ApplicationWorkload> {
        config.validate()?;

        let descriptor =
            HhlWorkloadDescriptor::from_config(
                config,
            )?;

        if builder.system_qubits()
            != descriptor.system_qubits
        {
            return Err(
                BenchmarkError::InconsistentConfiguration {
                    first:
                        "hhl.config.system_qubits"
                            .to_owned(),
                    second:
                        "hhl.circuit_builder.system_qubits"
                            .to_owned(),
                    reason:
                        "HHL benchmark and circuit builder must expose the same system-register width"
                            .to_owned(),
                },
            );
        }

        if builder.phase_qubits()
            != config.phase_qubits
        {
            return Err(
                BenchmarkError::InconsistentConfiguration {
                    first:
                        "hhl.config.phase_qubits"
                            .to_owned(),
                    second:
                        "hhl.circuit_builder.phase_qubits"
                            .to_owned(),
                    reason:
                        "HHL benchmark and circuit builder must expose the same phase-register width"
                            .to_owned(),
                },
            );
        }

        if builder.ancilla_qubits()
            != config.ancilla_qubits
        {
            return Err(
                BenchmarkError::InconsistentConfiguration {
                    first:
                        "hhl.config.ancilla_qubits"
                            .to_owned(),
                    second:
                        "hhl.circuit_builder.ancilla_qubits"
                            .to_owned(),
                    reason:
                        "HHL benchmark and circuit builder must expose the same ancilla count"
                            .to_owned(),
                },
            );
        }

        let mut workload =
            self.generate_from_config(
                config,
                instance_id,
                seed,
            )?;

        let circuit =
            builder.build(config)?;

        let circuit_workload =
            super::super::core::workload::CircuitWorkload::new(
                format!(
                    "hhl_{}_logical",
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

    /// Returns the default HHL metric set.
    #[must_use]
    pub fn default_metrics() -> Vec<HhlMetric> {
        default_metrics()
    }

    /// Returns an HHL semantic resource estimate.
    pub fn resource_estimate(
        config: &HhlBenchmarkConfig,
    ) -> BenchmarkResult<HhlResourceEstimate> {
        HhlResourceEstimate::from_config(config)
    }
}

impl ApplicationBenchmarkGenerator
    for HhlBenchmarkGenerator
{
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
            != HHL_APPLICATION_ID
        {
            return Err(
                BenchmarkError::InconsistentConfiguration {
                    first:
                        "request.application_id"
                            .to_owned(),
                    second:
                        "hhl.application_id"
                            .to_owned(),
                    reason:
                        "HHL requests must use the canonical hhl application identifier"
                            .to_owned(),
                },
            );
        }

        if request.metadata()
            .generator_revision()
            != HHL_GENERATOR_REVISION
        {
            return Err(
                BenchmarkError::ReproducibilityFailure {
                    component:
                        "hhl.generator_revision"
                            .to_owned(),
                    expected:
                        HHL_GENERATOR_REVISION
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
                        "hhl.parameters".to_owned(),
                    reason:
                        "HHL generation requests must contain benchmark parameters"
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
            decode_parameters(
                request.parameters(),
            )?;

        let dimension =
            parse_required_usize(
                &parameters,
                "system_dimension",
            )?;

        let system_qubits =
            parse_required_usize(
                &parameters,
                "system_qubits",
            )?;

        let phase_qubits =
            parse_required_usize(
                &parameters,
                "phase_qubits",
            )?;

        let ancilla_qubits =
            parse_required_usize(
                &parameters,
                "ancilla_qubits",
            )?;

        if dimension
            != request.problem_size()
        {
            return Err(
                BenchmarkError::InconsistentConfiguration {
                    first:
                        "hhl.system_dimension"
                            .to_owned(),
                    second:
                        "request.problem_size"
                            .to_owned(),
                    reason:
                        "HHL system dimension must equal the application workload problem size"
                            .to_owned(),
                },
            );
        }

        let expected_system_qubits =
            log2_exact_power_of_two(
                dimension,
            )?;

        if system_qubits
            != expected_system_qubits
        {
            return Err(
                BenchmarkError::InconsistentConfiguration {
                    first:
                        "hhl.system_qubits"
                            .to_owned(),
                    second:
                        "hhl.system_dimension"
                            .to_owned(),
                    reason:
                        "HHL system-qubit count does not match the power-of-two system dimension"
                            .to_owned(),
                },
            );
        }

        if phase_qubits == 0
            || ancilla_qubits == 0
        {
            return Err(
                BenchmarkError::InvalidConfiguration {
                    field:
                        "hhl.registers"
                            .to_owned(),
                    reason:
                        "HHL phase and ancilla registers must both be non-empty"
                            .to_owned(),
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
                        HHL_APPLICATION_ID
                            .to_owned(),
                    reason:
                        error.to_string(),
                }
            })?;

        let mut workload =
            ApplicationWorkload::new(
                HHL_APPLICATION_ID,
                instance,
                request.problem_size(),
            )
            .map_err(|error| {
                BenchmarkError::InvalidWorkload {
                    workload:
                        HHL_APPLICATION_ID
                            .to_owned(),
                    reason:
                        error.to_string(),
                }
            })?;

        for parameter
            in request.parameters()
        {
            workload
                .add_parameter(
                    parameter.clone(),
                )
                .map_err(|error| {
                    BenchmarkError::InvalidWorkload {
                        workload:
                            HHL_APPLICATION_ID
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
// Parameter encoding
// =============================================================================

/// Encodes the semantic HHL configuration into the canonical bounded
/// ApplicationParameter representation.
///
/// Text encoding is intentional: the canonical workload model should not
/// become coupled to HHL-specific numerical types.
fn encoded_parameters(
    config: &HhlBenchmarkConfig,
) -> BenchmarkResult<Vec<ApplicationParameter>> {
    config.validate()?;

    let mut parameters =
        Vec::with_capacity(24);

    push_parameter(
        &mut parameters,
        "matrix_id",
        config.matrix_id.clone(),
    )?;

    push_parameter(
        &mut parameters,
        "system_dimension",
        config.system_dimension.to_string(),
    )?;

    push_parameter(
        &mut parameters,
        "system_qubits",
        log2_exact_power_of_two(
            config.system_dimension,
        )?
        .to_string(),
    )?;

    push_parameter(
        &mut parameters,
        "matrix_class",
        config.matrix_class.as_str().to_owned(),
    )?;

    push_parameter(
        &mut parameters,
        "sparsity",
        config.sparsity.to_string(),
    )?;

    push_parameter(
        &mut parameters,
        "condition_number",
        format_finite_float(
            config.condition_number,
        )?,
    )?;

    push_parameter(
        &mut parameters,
        "spectral_norm",
        format_finite_float(
            config.spectral_norm,
        )?,
    )?;

    push_parameter(
        &mut parameters,
        "precision",
        format_finite_float(
            config.precision,
        )?,
    )?;

    push_parameter(
        &mut parameters,
        "input_state_kind",
        config.input_state_kind.as_str().to_owned(),
    )?;

    push_parameter(
        &mut parameters,
        "input_state_id",
        config.input_state_id.clone(),
    )?;

    push_parameter(
        &mut parameters,
        "simulation_model",
        config.simulation_model.as_str().to_owned(),
    )?;

    push_parameter(
        &mut parameters,
        "phase_estimation_model",
        config
            .phase_estimation_model
            .as_str()
            .to_owned(),
    )?;

    push_parameter(
        &mut parameters,
        "phase_qubits",
        config.phase_qubits.to_string(),
    )?;

    push_parameter(
        &mut parameters,
        "ancilla_qubits",
        config.ancilla_qubits.to_string(),
    )?;

    push_parameter(
        &mut parameters,
        "normalized_spectrum",
        config.normalized_spectrum.to_string(),
    )?;

    push_parameter(
        &mut parameters,
        "success_criterion",
        config.success_criterion.as_str().to_owned(),
    )?;

    if let Some(norm) =
        config.exact_solution_norm
    {
        push_parameter(
            &mut parameters,
            "exact_solution_norm",
            format_finite_float(norm)?,
        )?;
    }

    if let Some(scale) =
        config.rotation_scale
    {
        push_parameter(
            &mut parameters,
            "rotation_scale",
            format_finite_float(scale)?,
        )?;
    }

    if let Some(fidelity) =
        config.minimum_solution_fidelity
    {
        push_parameter(
            &mut parameters,
            "minimum_solution_fidelity",
            format_finite_float(fidelity)?,
        )?;
    }

    if let Some(residual) =
        config.maximum_residual_norm
    {
        push_parameter(
            &mut parameters,
            "maximum_residual_norm",
            format_finite_float(residual)?,
        )?;
    }

    if let Some(error) =
        config.maximum_observable_error
    {
        push_parameter(
            &mut parameters,
            "maximum_observable_error",
            format_finite_float(error)?,
        )?;
    }

    if parameters.len()
        > MAX_HHL_PARAMETERS
    {
        return Err(
            BenchmarkError::ResourceLimitExceeded {
                resource:
                    "hhl.parameters"
                        .to_owned(),
                requested:
                    parameters.len()
                        as u64,
                maximum:
                    MAX_HHL_PARAMETERS
                        as u64,
            },
        );
    }

    Ok(parameters)
}

/// Adds one bounded application parameter.
fn push_parameter(
    parameters: &mut Vec<ApplicationParameter>,
    name: &str,
    value: String,
) -> BenchmarkResult<()> {
    if value.len()
        > MAX_HHL_PARAMETER_VALUE_BYTES
    {
        return Err(
            BenchmarkError::ResourceLimitExceeded {
                resource:
                    "hhl.parameter_value"
                        .to_owned(),
                requested:
                    value.len()
                        as u64,
                maximum:
                    MAX_HHL_PARAMETER_VALUE_BYTES
                        as u64,
            },
        );
    }

    let parameter =
        ApplicationParameter::new(
            name,
            value,
        )
        .map_err(|error| {
            BenchmarkError::InvalidWorkload {
                workload:
                    HHL_APPLICATION_ID
                        .to_owned(),
                reason:
                    error.to_string(),
            }
        })?;

    parameters.push(parameter);

    Ok(())
}

// =============================================================================
// Parameter decoding
// =============================================================================

/// Decodes the bounded application parameter representation into a lookup
/// table.
///
/// Duplicate names are rejected rather than silently overwritten.
fn decode_parameters(
    parameters: &[ApplicationParameter],
) -> BenchmarkResult<
    std::collections::BTreeMap<String, String>,
> {
    let mut result =
        std::collections::BTreeMap::new();

    for parameter
        in parameters
    {
        if parameter.value().len()
            > MAX_HHL_PARAMETER_VALUE_BYTES
        {
            return Err(
                BenchmarkError::ResourceLimitExceeded {
                    resource:
                        "hhl.parameter_value"
                            .to_owned(),
                    requested:
                        parameter
                            .value()
                            .len()
                            as u64,
                    maximum:
                        MAX_HHL_PARAMETER_VALUE_BYTES
                            as u64,
                },
            );
        }

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
                        HHL_APPLICATION_ID
                            .to_owned(),
                    reason:
                        format!(
                            "duplicate HHL parameter '{}'",
                            parameter.name()
                        ),
                },
            );
        }
    }

    Ok(result)
}

/// Parses a required unsigned integer parameter.
fn parse_required_usize(
    parameters: &std::collections::BTreeMap<
        String,
        String,
    >,
    name: &str,
) -> BenchmarkResult<usize> {
    let value =
        parameters
            .get(name)
            .ok_or_else(|| {
                BenchmarkError::InvalidWorkload {
                    workload:
                        HHL_APPLICATION_ID
                            .to_owned(),
                    reason:
                        format!(
                            "missing required HHL parameter '{name}'"
                        ),
                }
            })?;

    value.parse::<usize>().map_err(|_| {
        BenchmarkError::InvalidWorkload {
            workload:
                HHL_APPLICATION_ID
                    .to_owned(),
            reason:
                format!(
                    "invalid unsigned integer for HHL parameter '{name}'"
                ),
        }
    })
}

// =============================================================================
// Default metrics
// =============================================================================

fn default_metrics() -> Vec<HhlMetric> {
    vec![
        HhlMetric::SystemDimension,
        HhlMetric::SystemQubits,
        HhlMetric::PhaseQubits,
        HhlMetric::AncillaQubits,
        HhlMetric::TotalLogicalQubits,
        HhlMetric::Sparsity,
        HhlMetric::ConditionNumber,
        HhlMetric::Precision,
        HhlMetric::PostSelectionProbability,
        HhlMetric::PostSelectionLowerBound,
        HhlMetric::ExpectedPostSelectionRepetitions,
        HhlMetric::SolutionStateFidelity,
        HhlMetric::ResidualNorm,
        HhlMetric::CircuitDepth,
        HhlMetric::GateCount,
        HhlMetric::TwoQubitGateCount,
        HhlMetric::CircuitVolume,
        HhlMetric::CircuitExecutions,
        HhlMetric::Shots,
        HhlMetric::QuantumExecutionTime,
        HhlMetric::TotalTime,
        HhlMetric::TimeToSolution,
        HhlMetric::Success,
    ]
}

/// Validates and canonicalizes an HHL metric set.
///
/// Duplicate metrics are rejected because silently accepting duplicates would
/// make result schemas ambiguous.
fn canonicalize_metrics(
    metrics: Vec<HhlMetric>,
) -> BenchmarkResult<Vec<HhlMetric>> {
    let mut result = Vec::with_capacity(
        metrics.len(),
    );

    for metric in metrics {
        if result.contains(&metric) {
            return Err(
                BenchmarkError::InvalidConfiguration {
                    field:
                        "hhl.metrics".to_owned(),
                    reason:
                        format!(
                            "duplicate HHL metric '{}'",
                            metric.as_str()
                        ),
                },
            );
        }

        result.push(metric);
    }

    if result.len()
        > MAX_HHL_PARAMETERS
    {
        return Err(
            BenchmarkError::ResourceLimitExceeded {
                resource:
                    "hhl.metrics"
                        .to_owned(),
                requested:
                    result.len()
                        as u64,
                maximum:
                    MAX_HHL_PARAMETERS
                        as u64,
            },
        );
    }

    Ok(result)
}

// =============================================================================
// Numerical helpers
// =============================================================================

/// Calculates log2(N) only when N is an exact power of two.
fn log2_exact_power_of_two(
    value: usize,
) -> BenchmarkResult<usize> {
    if value == 0
        || !value.is_power_of_two()
    {
        return Err(
            BenchmarkError::InvalidConfiguration {
                field:
                    "hhl.system_dimension"
                        .to_owned(),
                reason:
                    "system dimension must be a non-zero power of two"
                        .to_owned(),
            },
        );
    }

    Ok(value.trailing_zeros() as usize)
}

/// Checked addition with a benchmark-specific error.
fn checked_add(
    left: usize,
    right: usize,
    calculation: &'static str,
) -> BenchmarkResult<usize> {
    left.checked_add(right).ok_or_else(|| {
        BenchmarkError::InvalidConfiguration {
            field:
                "hhl.resource_estimate"
                    .to_owned(),
            reason:
                calculation.to_owned()
                    + " overflowed",
        }
    })
}

/// Validates a bounded identifier.
///
/// Accepted syntax:
///
/// ```text
/// [a-z][a-z0-9_-]*
/// ```
///
/// This keeps machine identifiers stable and suitable for registries,
/// serialization and benchmark-result schemas.
fn validate_identifier(
    field: &str,
    value: &str,
    maximum: usize,
) -> BenchmarkResult<()> {
    if value.is_empty() {
        return Err(
            BenchmarkError::InvalidIdentifier {
                field: field.to_owned(),
                value: value.to_owned(),
            },
        );
    }

    if value.len() > maximum {
        return Err(
            BenchmarkError::ResourceLimitExceeded {
                resource: field.to_owned(),
                requested:
                    value.len() as u64,
                maximum:
                    maximum as u64,
            },
        );
    }

    let bytes =
        value.as_bytes();

    if !bytes[0].is_ascii_lowercase() {
        return Err(
            BenchmarkError::InvalidIdentifier {
                field: field.to_owned(),
                value: value.to_owned(),
            },
        );
    }

    if !bytes.iter().all(|byte| {
        byte.is_ascii_lowercase()
            || byte.is_ascii_digit()
            || *byte == b'_'
            || *byte == b'-'
    }) {
        return Err(
            BenchmarkError::InvalidIdentifier {
                field: field.to_owned(),
                value: value.to_owned(),
            },
        );
    }

    Ok(())
}

/// Validates a positive finite floating-point quantity.
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

/// Validates a non-negative finite floating-point quantity.
fn validate_non_negative_finite(
    field: &str,
    value: f64,
) -> BenchmarkResult<()> {
    if !value.is_finite()
        || value < 0.0
    {
        return Err(
            BenchmarkError::InvalidConfiguration {
                field: field.to_owned(),
                reason:
                    "value must be finite and non-negative"
                        .to_owned(),
            },
        );
    }

    Ok(())
}

/// Formats a finite floating-point number deterministically.
fn format_finite_float(
    value: f64,
) -> BenchmarkResult<String> {
    if !value.is_finite() {
        return Err(
            BenchmarkError::InvalidConfiguration {
                field:
                    "hhl.numeric_value"
                        .to_owned(),
                reason:
                    "floating-point values must be finite"
                        .to_owned(),
            },
        );
    }

    Ok(format!("{value:.17e}"))
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_small_hhl_configuration() {
        let config =
            HhlBenchmarkConfig::new(
                "poisson_4",
                4,
                2,
                4.0,
                1.0e-3,
            )
            .expect(
                "valid HHL configuration",
            );

        assert_eq!(
            config.system_dimension,
            4
        );

        assert_eq!(
            log2_exact_power_of_two(4)
                .expect("4 is a power of two"),
            2
        );
    }

    #[test]
    fn rejects_non_power_of_two_dimension() {
        let result =
            HhlBenchmarkConfig::new(
                "invalid",
                3,
                2,
                2.0,
                1.0e-3,
            );

        assert!(result.is_err());
    }

    #[test]
    fn rejects_condition_number_below_one() {
        let result =
            HhlBenchmarkConfig::new(
                "invalid",
                4,
                2,
                0.5,
                1.0e-3,
            );

        assert!(result.is_err());
    }

    #[test]
    fn rejects_zero_sparsity() {
        let result =
            HhlBenchmarkConfig::new(
                "invalid",
                4,
                0,
                2.0,
                1.0e-3,
            );

        assert!(result.is_err());
    }

    #[test]
    fn rejects_sparsity_larger_than_dimension() {
        let result =
            HhlBenchmarkConfig::new(
                "invalid",
                4,
                5,
                2.0,
                1.0e-3,
            );

        assert!(result.is_err());
    }

    #[test]
    fn rejects_non_finite_condition_number() {
        let result =
            HhlBenchmarkConfig::new(
                "invalid",
                4,
                2,
                f64::NAN,
                1.0e-3,
            );

        assert!(result.is_err());
    }

    #[test]
    fn rejects_precision_equal_to_one() {
        let result =
            HhlBenchmarkConfig::new(
                "invalid",
                4,
                2,
                2.0,
                1.0,
            );

        assert!(result.is_err());
    }

    #[test]
    fn calculates_system_qubits() {
        assert_eq!(
            log2_exact_power_of_two(1)
                .expect("1 is a power of two"),
            0
        );

        assert_eq!(
            log2_exact_power_of_two(2)
                .expect("2 is a power of two"),
            1
        );

        assert_eq!(
            log2_exact_power_of_two(8)
                .expect("8 is a power of two"),
            3
        );

        assert_eq!(
            log2_exact_power_of_two(16)
                .expect("16 is a power of two"),
            4
        );
    }

    #[test]
    fn resource_estimate_has_correct_logical_qubit_count() {
        let config =
            HhlBenchmarkConfig::new(
                "poisson_4",
                16,
                3,
                8.0,
                1.0e-3,
            )
            .expect(
                "valid HHL configuration",
            )
            .with_phase_estimation(
                HhlPhaseEstimationModel::StandardQpe,
                6,
            )
            .expect(
                "valid phase-estimation configuration",
            );

        let estimate =
            HhlResourceEstimate::from_config(
                &config,
            )
            .expect(
                "resource estimate should succeed",
            );

        assert_eq!(
            estimate.system_qubits,
            4
        );

        assert_eq!(
            estimate.phase_qubits,
            6
        );

        assert_eq!(
            estimate.ancilla_qubits,
            1
        );

        assert_eq!(
            estimate.total_logical_qubits,
            11
        );
    }

    #[test]
    fn normalized_post_selection_lower_bound_scales_as_inverse_kappa_squared()
    {
        let config =
            HhlBenchmarkConfig::new(
                "test",
                4,
                2,
                10.0,
                1.0e-3,
            )
            .expect(
                "valid HHL configuration",
            );

        let bound =
            theoretical_post_selection_lower_bound(
                &config,
            )
            .expect(
                "bound calculation",
            )
            .expect(
                "normalized spectrum provides bound",
            );

        assert!(
            (bound - 0.01).abs()
                < 1.0e-15
        );
    }

    #[test]
    fn exact_post_selection_probability_requires_explicit_scale_and_norm()
    {
        let config =
            HhlBenchmarkConfig::new(
                "test",
                4,
                2,
                2.0,
                1.0e-3,
            )
            .expect(
                "valid HHL configuration",
            )
            .with_exact_solution_norm(
                1.5,
            )
            .expect(
                "valid solution norm",
            )
            .with_rotation_scale(
                0.25,
            )
            .expect(
                "valid rotation scale",
            );

        let probability =
            ideal_post_selection_probability(
                &config,
            )
            .expect(
                "probability calculation",
            )
            .expect(
                "explicit probability inputs",
            );

        assert!(
            (probability - 0.140625).abs()
                < 1.0e-15
        );
    }

    #[test]
    fn expected_repetitions_are_inverse_probability() {
        let config =
            HhlBenchmarkConfig::new(
                "test",
                4,
                2,
                10.0,
                1.0e-3,
            )
            .expect(
                "valid HHL configuration",
            );

        let repetitions =
            expected_post_selection_repetitions(
                &config,
            )
            .expect(
                "repetition calculation",
            )
            .expect(
                "normalized spectrum gives lower-bound repetitions",
            );

        assert!(
            (repetitions - 100.0).abs()
                < 1.0e-12
        );
    }

    #[test]
    fn fidelity_success_requires_threshold() {
        let result =
            HhlBenchmarkConfig::new(
                "test",
                4,
                2,
                2.0,
                1.0e-3,
            )
            .expect(
                "valid configuration",
            )
            .with_success_criterion(
                HhlSuccessCriterion::SolutionStateFidelity,
            );

        assert!(result.is_err());
    }

    #[test]
    fn residual_success_requires_threshold() {
        let result =
            HhlBenchmarkConfig::new(
                "test",
                4,
                2,
                2.0,
                1.0e-3,
            )
            .expect(
                "valid configuration",
            )
            .with_success_criterion(
                HhlSuccessCriterion::ResidualNorm,
            );

        assert!(result.is_err());
    }

    #[test]
    fn observable_success_requires_threshold() {
        let result =
            HhlBenchmarkConfig::new(
                "test",
                4,
                2,
                2.0,
                1.0e-3,
            )
            .expect(
                "valid configuration",
            )
            .with_success_criterion(
                HhlSuccessCriterion::ObservableError,
            );

        assert!(result.is_err());
    }

    #[test]
    fn workload_request_is_reproducible_in_structure() {
        let config =
            HhlBenchmarkConfig::new(
                "poisson_4",
                4,
                2,
                4.0,
                1.0e-3,
            )
            .expect(
                "valid HHL configuration",
            );

        let id1 =
            WorkloadId::new(
                "poisson_4_instance",
            )
            .expect(
                "valid workload identifier",
            );

        let id2 =
            WorkloadId::new(
                "poisson_4_instance",
            )
            .expect(
                "valid workload identifier",
            );

        let request1 =
            HhlBenchmarkGenerator::request(
                &config,
                id1,
                42,
            )
            .expect(
                "request generation",
            );

        let request2 =
            HhlBenchmarkGenerator::request(
                &config,
                id2,
                42,
            )
            .expect(
                "request generation",
            );

        assert_eq!(
            request1.application_id(),
            request2.application_id()
        );

        assert_eq!(
            request1.problem_size(),
            request2.problem_size()
        );

        assert_eq!(
            request1.parameters(),
            request2.parameters()
        );
    }

    #[test]
    fn generator_produces_canonical_application_workload() {
        let config =
            HhlBenchmarkConfig::new(
                "poisson_4",
                4,
                2,
                4.0,
                1.0e-3,
            )
            .expect(
                "valid HHL configuration",
            );

        let generator =
            HhlBenchmarkGenerator::new()
                .expect(
                    "static generator descriptor",
                );

        let instance =
            WorkloadId::new(
                "poisson_4_instance",
            )
            .expect(
                "valid workload identifier",
            );

        let workload =
            generator
                .generate_from_config(
                    &config,
                    instance,
                    123,
                )
                .expect(
                    "workload generation",
                );

        assert_eq!(
            workload.application_id(),
            HHL_APPLICATION_ID
        );

        assert_eq!(
            workload.problem_size(),
            4
        );
    }

    #[test]
    fn generator_descriptor_is_stable() {
        let generator =
            HhlBenchmarkGenerator::new()
                .expect(
                    "generator descriptor",
                );

        assert_eq!(
            generator
                .descriptor_ref()
                .generator_id(),
            HHL_BENCHMARK_ID
        );

        assert_eq!(
            generator
                .descriptor_ref()
                .application_id(),
            HHL_APPLICATION_ID
        );

        assert!(
            generator
                .descriptor_ref()
                .supports(
                    ApplicationGeneratorCapability::ResourceEstimation
                )
        );
    }

    #[test]
    fn default_metrics_are_unique() {
        let metrics =
            default_metrics();

        for i in 0..metrics.len() {
            for j in (i + 1)..metrics.len() {
                assert_ne!(
                    metrics[i],
                    metrics[j]
                );
            }
        }
    }
}