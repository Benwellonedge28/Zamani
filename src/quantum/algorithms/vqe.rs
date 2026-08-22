//! Zamani Quantum Algorithms — Variational Quantum Eigensolver.
//!
//! Production-grade, backend-independent orchestration for the Variational
//! Quantum Eigensolver (VQE).
//!
//! # Architectural responsibility
//!
//! This module owns:
//!
//! - VQE problem validation;
//! - Hamiltonian/observable integration at the VQE boundary;
//! - parameterized ansatz integration;
//! - VQE energy-objective construction;
//! - classical optimizer orchestration;
//! - execution accounting;
//! - deterministic-execution enforcement through the algorithm execution
//!   contract;
//! - VQE result validation;
//! - VQE-specific provenance and statistics.
//!
//! This module deliberately does NOT own:
//!
//! - quantum gate definitions;
//! - quantum circuit storage;
//! - IR validation implementation;
//! - physical qubit routing;
//! - transpilation;
//! - hardware topology;
//! - QPU communication;
//! - backend implementation;
//! - error-correction decoding;
//! - optimizer implementation;
//! - generic objective implementation;
//! - persistence.
//!
//! Those responsibilities belong to:
//!
//! ```text
//! quantum::ir
//! quantum::routing
//! quantum::transpiler
//! quantum::hardware
//! quantum::error_correction
//! algorithms::execution
//! algorithms::objective
//! algorithms::optimizer
//! algorithms::types
//! ```
//!
//! # Architectural flow
//!
//! ```text
//! VqeProblem
//!     │
//!     ├── Hamiltonian
//!     │
//!     ├── Ansatz
//!     │       │
//!     │       ▼
//!     │   QuantumCircuit
//!     │       │
//!     │       ▼
//!     │   ExecutionRequest
//!     │       │
//!     │       ▼
//!     │   QuantumExecutor
//!     │       │
//!     │       ▼
//!     │   ExpectationValue
//!     │
//!     └──────────────► VqeEnergyObjective
//!                           │
//!                           ▼
//!                       Optimizer
//!                           │
//!                           ▼
//!                       VqeResult
//! ```
//!
//! # Important contract
//!
//! The VQE layer never directly calls a backend. All execution crosses the
//! canonical `QuantumExecutor` boundary in `execution.rs`.
//!
//! The VQE layer also never defines a second circuit abstraction. Ansätze
//! return the canonical `quantum::ir::QuantumCircuit`.
//!
//! # Hamiltonian boundary
//!
//! The repository's execution contract intentionally does not prescribe a
//! concrete mathematical observable representation. It carries a bounded,
//! backend-neutral observable identifier and receives the resulting
//! expectation value from the executor.
//!
//! Therefore this module defines the minimal VQE Hamiltonian contract:
//!
//! - stable observable identifier;
//! - logical qubit count;
//! - validation.
//!
//! A future canonical observable/Pauli subsystem can implement this trait
//! without requiring changes to VQE orchestration.
//!
//! # Determinism
//!
//! VQE itself does not create randomness.
//!
//! Determinism is controlled by `ExecutionConfig` and enforced by
//! `QuantumExecutor`/`ExecutionResult::validate_against`.
//!
//! # Numerical safety
//!
//! All externally supplied floating-point values must be finite.
//!
//! Non-finite energy, expectation, uncertainty, or optimizer output is
//! rejected through `AlgorithmError`.
//!
//! # Resource safety
//!
//! VQE validates the logical qubit count and relies on the canonical execution
//! and optimizer contracts for circuit, shot, iteration, evaluation, and
//! parameter resource limits.
//!
//! # Rust compatibility
//!
//! Rust 1.97.1.
//!
//! No nightly features.
//! No external dependencies.
//!
//! # Safety
//!
//! This module contains no unsafe code.

#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::fmt;

use crate::quantum::ir::QuantumCircuit;

use super::error::{AlgorithmError, AlgorithmResource, Result};
use super::execution::{
    execute,
    ExecutionConfig,
    ExecutionRequest,
    QuantumExecutor,
};
use super::objective::{
    Objective,
    ObjectiveDirection,
    ObjectiveEvaluation,
    ObjectiveKind,
};
use super::optimizer::{
    OptimizationConfig,
    OptimizationResult,
    Optimizer,
};
use super::types::{
    AlgorithmId,
    AlgorithmMetadata,
    AlgorithmVersion,
    Energy,
    ParameterVector,
    QubitCount,
    ShotCount,
};

// =============================================================================
// VQE version
// =============================================================================

/// Stable VQE algorithm contract version.
///
/// This version identifies the semantics of this VQE implementation, not the
/// version of a backend or the Quantum IR.
pub const VQE_VERSION: AlgorithmVersion =
    AlgorithmVersion::new(1, 0, 0);

// =============================================================================
// Hamiltonian contract
// =============================================================================

/// Backend-independent Hamiltonian/observable contract used by VQE.
///
/// The mathematical representation remains owned by the observable/physics
/// layer that implements this trait.
///
/// The execution subsystem only receives the stable observable identifier.
/// This prevents VQE from duplicating the repository's future canonical
/// observable representation.
pub trait VqeHamiltonian {
    /// Returns the number of logical qubits on which the Hamiltonian acts.
    fn qubit_count(&self) -> Result<QubitCount>;

    /// Returns a stable backend-neutral identifier for this Hamiltonian.
    ///
    /// The identifier is context/provenance data. It must not contain
    /// credentials, backend-specific connection information, or mutable
    /// execution state.
    fn observable_id(&self) -> Result<String>;

    /// Validates the Hamiltonian.
    ///
    /// Implementations must reject malformed mathematical representations,
    /// non-finite coefficients, invalid qubit references, and inconsistent
    /// dimensions.
    fn validate(&self) -> Result<()> {
        let _ = self.qubit_count()?;
        let _ = self.observable_id()?;
        Ok(())
    }
}

// =============================================================================
// Ansatz contract
// =============================================================================

/// Parameterized VQE ansatz.
///
/// The ansatz owns how classical parameters become a canonical logical
/// `QuantumCircuit`.
///
/// It must not perform backend execution.
pub trait VqeAnsatz {
    /// Returns the number of logical qubits required by the ansatz.
    fn qubit_count(&self) -> Result<QubitCount>;

    /// Returns the number of classical parameters accepted by the ansatz.
    fn parameter_count(&self) -> Result<u64>;

    /// Builds a logical Quantum IR circuit for the supplied parameters.
    fn build(&self, parameters: &ParameterVector)
        -> Result<QuantumCircuit>;

    /// Validates the ansatz independently of any concrete parameter vector.
    fn validate(&self) -> Result<()> {
        let _ = self.qubit_count()?;
        let _ = self.parameter_count()?;
        Ok(())
    }
}

// =============================================================================
// VQE problem
// =============================================================================

/// Complete validated VQE problem.
///
/// A problem contains the mathematical Hamiltonian and parameterized ansatz.
/// Execution and optimization policy are intentionally kept outside this
/// immutable problem description.
pub struct VqeProblem<H, A> {
    hamiltonian: H,
    ansatz: A,
    initial_parameters: ParameterVector,
}

impl<H, A> fmt::Debug for VqeProblem<H, A>
where
    H: fmt::Debug,
    A: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("VqeProblem")
            .field("hamiltonian", &self.hamiltonian)
            .field("ansatz", &self.ansatz)
            .field("initial_parameters", &self.initial_parameters)
            .finish()
    }
}

impl<H, A> VqeProblem<H, A>
where
    H: VqeHamiltonian,
    A: VqeAnsatz,
{
    /// Creates and validates a VQE problem.
    pub fn new(
        hamiltonian: H,
        ansatz: A,
        initial_parameters: ParameterVector,
    ) -> Result<Self> {
        hamiltonian.validate()?;
        ansatz.validate()?;

        let hamiltonian_qubits = hamiltonian.qubit_count()?;
        let ansatz_qubits = ansatz.qubit_count()?;

        if hamiltonian_qubits != ansatz_qubits {
            return Err(AlgorithmError::DimensionMismatch {
                expected: hamiltonian_qubits.get() as usize,
                actual: ansatz_qubits.get() as usize,
                context:
                    "VQE Hamiltonian and ansatz qubit counts"
                        .to_string(),
            });
        }

        let parameter_count = ansatz.parameter_count()?;

        if parameter_count == 0 {
            return Err(AlgorithmError::InvalidConfiguration {
                field: "parameter_count".to_string(),
                message:
                    "VQE ansatz must expose at least one parameter"
                        .to_string(),
            });
        }

        let parameter_count_usize =
            usize::try_from(parameter_count).map_err(|_| {
                AlgorithmError::ResourceLimitExceeded {
                    resource: AlgorithmResource::Parameters,
                    requested: parameter_count as u128,
                    limit: usize::MAX as u128,
                    message:
                        "VQE parameter count cannot be represented by usize"
                            .to_string(),
                }
            })?;

        if initial_parameters.len() != parameter_count_usize {
            return Err(AlgorithmError::DimensionMismatch {
                expected: parameter_count_usize,
                actual: initial_parameters.len(),
                context: "VQE initial parameter vector".to_string(),
            });
        }

        Ok(Self {
            hamiltonian,
            ansatz,
            initial_parameters,
        })
    }

    /// Returns the Hamiltonian.
    pub fn hamiltonian(&self) -> &H {
        &self.hamiltonian
    }

    /// Returns the ansatz.
    pub fn ansatz(&self) -> &A {
        &self.ansatz
    }

    /// Returns the initial parameters.
    pub fn initial_parameters(&self) -> &ParameterVector {
        &self.initial_parameters
    }

    /// Returns the logical qubit count.
    pub fn qubit_count(&self) -> Result<QubitCount> {
        self.hamiltonian.qubit_count()
    }

    /// Returns the parameter count.
    pub fn parameter_count(&self) -> Result<u64> {
        self.ansatz.parameter_count()
    }
}

// =============================================================================
// VQE configuration
// =============================================================================

/// Execution and orchestration policy for one VQE invocation.
#[derive(Debug, Clone, PartialEq)]
pub struct VqeConfig {
    /// Quantum execution configuration.
    pub execution: ExecutionConfig,

    /// Classical optimizer configuration.
    pub optimization: OptimizationConfig,

    /// Whether failure to converge is considered an algorithm error.
    ///
    /// `false` is useful for scientific workflows that want the best bounded
    /// result even when the configured iteration limit is reached.
    pub require_convergence: bool,
}

impl Default for VqeConfig {
    fn default() -> Self {
        Self {
            execution: ExecutionConfig::default(),
            optimization: OptimizationConfig::default(),
            require_convergence: false,
        }
    }
}

impl VqeConfig {
    /// Validates the complete VQE configuration.
    pub fn validate(&self) -> Result<()> {
        self.execution.validate()?;
        self.optimization.validate()?;

        if self.execution.deterministic
            && self.execution.seed.is_none()
        {
            return Err(AlgorithmError::DeterminismViolation {
                contract: "VQE deterministic execution".to_string(),
                message:
                    "deterministic VQE execution requires an explicit seed"
                        .to_string(),
            });
        }

        Ok(())
    }
}

// =============================================================================
// VQE statistics
// =============================================================================

/// Immutable execution statistics for a completed VQE invocation.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct VqeStatistics {
    /// Number of objective evaluations.
    pub objective_evaluations: u64,

    /// Number of logical circuit executions.
    pub circuit_executions: u64,

    /// Number of measurement shots.
    pub shots: u64,

    /// Number of optimizer iterations.
    pub optimizer_iterations: u64,

    /// Number of accepted optimizer steps.
    pub optimizer_steps: u64,
}

impl VqeStatistics {
    fn from_optimization(
        optimization: &OptimizationResult,
        objective_evaluations: u64,
        circuit_executions: u64,
        shots: u64,
    ) -> Result<Self> {
        Ok(Self {
            objective_evaluations,
            circuit_executions,
            shots,
            optimizer_iterations:
                optimization.statistics.iterations,
            optimizer_steps:
                optimization.statistics.optimizer_steps,
        })
    }
}

// =============================================================================
// VQE result
// =============================================================================

/// Complete result of one VQE invocation.
#[derive(Debug, Clone, PartialEq)]
pub struct VqeResult {
    /// Stable algorithm metadata.
    pub metadata: AlgorithmMetadata,

    /// Ground-state energy estimate returned by the optimization.
    pub energy: Energy,

    /// Parameters associated with the final optimizer state.
    pub parameters: ParameterVector,

    /// Parameters associated with the best energy observed.
    pub best_parameters: ParameterVector,

    /// Best energy observed during optimization.
    pub best_energy: Energy,

    /// Classical optimization result.
    pub optimization: OptimizationResult,

    /// VQE execution accounting.
    pub statistics: VqeStatistics,
}

impl VqeResult {
    /// Returns whether the optimization converged.
    #[must_use]
    pub fn converged(&self) -> bool {
        self.optimization.converged()
    }

    /// Validates the complete VQE result.
    pub fn validate(&self) -> Result<()> {
        if self.metadata.algorithm != AlgorithmId::Vqe {
            return Err(AlgorithmError::InternalInvariantViolation {
                message:
                    "VQE result metadata must identify the VQE algorithm"
                        .to_string(),
            });
        }

        if self.metadata.version != VQE_VERSION {
            return Err(AlgorithmError::InternalInvariantViolation {
                message:
                    "VQE result metadata contains an unexpected VQE version"
                        .to_string(),
            });
        }

        self.energy.get();
        self.best_energy.get();

        if self.parameters.is_empty()
            || self.best_parameters.is_empty()
        {
            return Err(AlgorithmError::InternalInvariantViolation {
                message:
                    "VQE result parameter vectors cannot be empty"
                        .to_string(),
            });
        }

        if self.parameters.len()
            != self.best_parameters.len()
        {
            return Err(AlgorithmError::InternalInvariantViolation {
                message:
                    "VQE final and best parameter dimensions differ"
                        .to_string(),
            });
        }

        self.optimization.validate()?;

        Ok(())
    }
}

// =============================================================================
// VQE objective
// =============================================================================

/// Quantum objective used internally by VQE.
///
/// This adapter translates:
///
/// `ParameterVector -> Ansatz -> QuantumCircuit -> Executor -> Energy`.
struct VqeEnergyObjective<'a, H, A, E> {
    hamiltonian: &'a H,
    ansatz: &'a A,
    executor: &'a mut E,
    execution_config: ExecutionConfig,
    evaluations: u64,
    circuit_executions: u64,
    shots: u64,
}

impl<'a, H, A, E> VqeEnergyObjective<'a, H, A, E>
where
    H: VqeHamiltonian,
    A: VqeAnsatz,
    E: QuantumExecutor,
{
    fn new(
        hamiltonian: &'a H,
        ansatz: &'a A,
        executor: &'a mut E,
        execution_config: ExecutionConfig,
    ) -> Self {
        Self {
            hamiltonian,
            ansatz,
            executor,
            execution_config,
            evaluations: 0,
            circuit_executions: 0,
            shots: 0,
        }
    }

    fn statistics(&self) -> VqeStatistics {
        VqeStatistics {
            objective_evaluations: self.evaluations,
            circuit_executions: self.circuit_executions,
            shots: self.shots,
            optimizer_iterations: 0,
            optimizer_steps: 0,
        }
    }

    fn record_execution(
        &mut self,
        result: &super::execution::ExecutionResult,
    ) -> Result<()> {
        self.circuit_executions = self
            .circuit_executions
            .checked_add(result.circuit_executions())
            .ok_or_else(|| {
                AlgorithmError::ResourceLimitExceeded {
                    resource: AlgorithmResource::CircuitExecutions,
                    requested: u128::MAX,
                    limit: u128::MAX,
                    message:
                        "VQE circuit-execution counter overflowed"
                            .to_string(),
                }
            })?;

        if let Some(shots) = result.shots_executed() {
            self.shots = self
                .shots
                .checked_add(shots.get())
                .ok_or_else(|| {
                    AlgorithmError::ResourceLimitExceeded {
                        resource: AlgorithmResource::Shots,
                        requested: u128::MAX,
                        limit: u128::MAX,
                        message:
                            "VQE shot counter overflowed"
                                .to_string(),
                    }
                })?;
        }

        Ok(())
    }
}

impl<'a, H, A, E> Objective
    for VqeEnergyObjective<'a, H, A, E>
where
    H: VqeHamiltonian,
    A: VqeAnsatz,
    E: QuantumExecutor,
{
    fn evaluate(
        &mut self,
        parameters: &ParameterVector,
    ) -> Result<ObjectiveEvaluation> {
        self.evaluations = self
            .evaluations
            .checked_add(1)
            .ok_or_else(|| {
                AlgorithmError::ResourceLimitExceeded {
                    resource: AlgorithmResource::ObjectiveEvaluations,
                    requested: u128::MAX,
                    limit: u128::MAX,
                    message:
                        "VQE objective-evaluation counter overflowed"
                            .to_string(),
                }
            })?;

        let circuit = self.ansatz.build(parameters)?;

        let observable = self.hamiltonian.observable_id()?;

        let request = ExecutionRequest::expectation(
            circuit,
            self.execution_config.clone(),
        )?
        .with_algorithm("vqe")?
        .with_operation("energy_expectation")?
        .with_observable(observable)?;

        let result = execute(
            self.executor,
            &request,
        )?;

        self.record_execution(&result)?;

        let expectation =
            result.expectation().ok_or_else(|| {
                AlgorithmError::ExecutionFailed {
                    backend: Some(
                        result.metadata().backend_id.clone(),
                    ),
                    operation:
                        "vqe_energy_expectation".to_string(),
                    message:
                        "executor returned no expectation value"
                            .to_string(),
                }
            })?;

        let energy =
            Energy::new(expectation.get())?;

        ObjectiveEvaluation::quantum(
            super::types::ObjectiveValue::new(
                energy.get(),
            )?,
            self.evaluations,
            result.circuit_executions(),
            result.shots_executed().map(|v| v.get()),
            result.metadata().deterministic,
        )
    }

    fn direction(&self) -> ObjectiveDirection {
        ObjectiveDirection::Minimize
    }

    fn kind(&self) -> ObjectiveKind {
        ObjectiveKind::Energy
    }

    fn statistics(&self)
        -> super::objective::ObjectiveStatistics
    {
        super::objective::ObjectiveStatistics {
            evaluations: self.evaluations,
            circuit_executions:
                self.circuit_executions,
            shots: self.shots,
        }
    }
}

// =============================================================================
// VQE executor
// =============================================================================

/// Production VQE orchestration object.
///
/// `Vqe` owns no backend. The backend is supplied for each execution, keeping
/// VQE reusable across simulators, CPU/GPU executors, QPUs, and remote
/// execution implementations.
#[derive(Debug, Default, Clone, Copy)]
pub struct Vqe;

impl Vqe {
    /// Stable algorithm metadata.
    #[must_use]
    pub const fn metadata() -> AlgorithmMetadata {
        AlgorithmMetadata::new(
            AlgorithmId::Vqe,
            VQE_VERSION,
        )
    }

    /// Solves a VQE problem.
    ///
    /// The problem and configuration are validated before execution.
    ///
    /// The optimizer is supplied by the caller so VQE does not hard-code a
    /// classical optimization strategy.
    pub fn solve<H, A, E, O>(
        &self,
        problem: &VqeProblem<H, A>,
        executor: &mut E,
        optimizer: &mut O,
        config: &VqeConfig,
    ) -> Result<VqeResult>
    where
        H: VqeHamiltonian,
        A: VqeAnsatz,
        E: QuantumExecutor,
        O: Optimizer,
    {
        config.validate()?;

        problem.hamiltonian.validate()?;
        problem.ansatz.validate()?;

        let mut objective =
            VqeEnergyObjective::new(
                &problem.hamiltonian,
                &problem.ansatz,
                executor,
                config.execution.clone(),
            );

        let optimization =
            optimizer.optimize(
                &mut objective,
                problem.initial_parameters.clone(),
                &config.optimization,
            )?;

        optimization.validate()?;

        if config.require_convergence
            && !optimization.converged()
        {
            return Err(AlgorithmError::OptimizationFailed {
                message: format!(
                    "VQE optimizer terminated with status {}",
                    optimization.status
                ),
            });
        }

        let energy =
            Energy::new(
                optimization.objective.get(),
            )?;

        let best_energy =
            Energy::new(
                optimization.best_objective.get(),
            )?;

        let objective_statistics =
            objective.statistics();

        let statistics =
            VqeStatistics::from_optimization(
                &optimization,
                objective_statistics.evaluations,
                objective_statistics.circuit_executions,
                objective_statistics.shots,
            )?;

        let result = VqeResult {
            metadata: Self::metadata(),
            energy,
            parameters:
                optimization.parameters.clone(),
            best_parameters:
                optimization.best_parameters.clone(),
            best_energy,
            optimization,
            statistics,
        };

        result.validate()?;

        Ok(result)
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::quantum::ir::{
        CircuitMetadata,
        QuantumCircuit,
        QuantumIrLimits,
    };

    struct TestHamiltonian {
        qubits: QubitCount,
    }

    impl VqeHamiltonian for TestHamiltonian {
        fn qubit_count(&self) -> Result<QubitCount> {
            Ok(self.qubits)
        }

        fn observable_id(&self) -> Result<String> {
            Ok("test_hamiltonian".to_string())
        }
    }

    struct TestAnsatz;

    impl VqeAnsatz for TestAnsatz {
        fn qubit_count(&self) -> Result<QubitCount> {
            QubitCount::new(1)
        }

        fn parameter_count(&self) -> Result<u64> {
            Ok(1)
        }

        fn build(
            &self,
            _parameters: &ParameterVector,
        ) -> Result<QuantumCircuit> {
            QuantumCircuit::new(
                1,
                0,
                CircuitMetadata::default(),
                QuantumIrLimits::default(),
            )
            .map_err(|error| {
                AlgorithmError::InvalidCircuit {
                    message: error.to_string(),
                }
            })
        }
    }

    #[test]
    fn problem_rejects_dimension_mismatch() {
        struct TwoQubitAnsatz;

        impl VqeAnsatz for TwoQubitAnsatz {
            fn qubit_count(
                &self,
            ) -> Result<QubitCount> {
                QubitCount::new(2)
            }

            fn parameter_count(
                &self,
            ) -> Result<u64> {
                Ok(1)
            }

            fn build(
                &self,
                _parameters: &ParameterVector,
            ) -> Result<QuantumCircuit> {
                Err(
                    AlgorithmError::InvalidInput {
                        message:
                            "test ansatz should not be built"
                                .to_string(),
                    },
                )
            }
        }

        let result =
            VqeProblem::new(
                TestHamiltonian {
                    qubits:
                        QubitCount::new(1)
                            .unwrap(),
                },
                TwoQubitAnsatz,
                ParameterVector::new(
                    vec![0.0],
                )
                .unwrap(),
            );

        assert!(result.is_err());
    }

    #[test]
    fn problem_rejects_parameter_dimension_mismatch() {
        let result =
            VqeProblem::new(
                TestHamiltonian {
                    qubits:
                        QubitCount::new(1)
                            .unwrap(),
                },
                TestAnsatz,
                ParameterVector::new(
                    vec![0.0, 1.0],
                )
                .unwrap(),
            );

        assert!(result.is_err());
    }

    #[test]
    fn deterministic_vqe_requires_seed() {
        let mut config =
            VqeConfig::default();

        config.execution.deterministic = true;
        config.execution.seed = None;

        assert!(config.validate().is_err());
    }

    #[test]
    fn metadata_is_vqe() {
        let metadata = Vqe::metadata();

        assert_eq!(
            metadata.algorithm,
            AlgorithmId::Vqe
        );

        assert_eq!(
            metadata.version,
            VQE_VERSION
        );
    }
}