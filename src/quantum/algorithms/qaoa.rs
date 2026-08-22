//! Zamani Quantum Algorithms — Quantum Approximate Optimization Algorithm.
//!
//! Production-grade, backend-independent QAOA orchestration.
//!
//! # Responsibility
//!
//! This module owns:
//!
//! - QAOA problem validation;
//! - cost/mixer operator contracts;
//! - QAOA ansatz construction boundary;
//! - QAOA parameter layout;
//! - optimization orchestration;
//! - expectation-value objective construction;
//! - final measurement execution;
//! - QAOA result/provenance metadata;
//! - resource and determinism enforcement at the algorithm boundary.
//!
//! This module deliberately does NOT own:
//!
//! - quantum gates;
//! - circuit storage;
//! - IR validation;
//! - physical qubit mapping;
//! - routing;
//! - transpilation;
//! - hardware;
//! - simulator implementation;
//! - QPU/vendor APIs;
//! - optimizer implementation;
//! - error-correction implementation;
//! - persistence.
//!
//! # Architecture
//!
//! ```text
//! QaoaProblem
//!      │
//!      ├── CostOperator
//!      ├── MixerOperator
//!      ├── QaoaAnsatz
//!      └── Initial Parameters
//!             │
//!             ▼
//!       QaoaCostObjective
//!             │
//!             ▼
//!          Optimizer
//!             │
//!             ▼
//!      optimized parameters
//!             │
//!             ▼
//!        QaoaAnsatz
//!             │
//!             ▼
//!       QuantumCircuit
//!             │
//!             ▼
//!       QuantumExecutor
//!             │
//!             ├── expectation
//!             └── measurement
//!
//! ```
//!
//! # IR boundary
//!
//! QAOA never constructs gates directly. A `QaoaAnsatz` produces the
//! repository's canonical `quantum::ir::QuantumCircuit`.
//!
//! This prevents the algorithm layer from duplicating IR semantics.
//!
//! # Determinism
//!
//! QAOA itself does not create randomness. Explicit execution seeds are
//! supplied through `ExecutionConfig`.
//!
//! When deterministic execution is requested, the canonical execution
//! boundary verifies that the executor actually reports deterministic
//! execution.
//!
//! # Optimization
//!
//! QAOA uses the generic `Optimizer` contract. It therefore does not depend
//! upon gradient descent, SPSA, parameter-shift, or any particular classical
//! optimization method.
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

use super::error::{
    AlgorithmError,
    AlgorithmResource,
    Result,
};
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
    ObjectiveValue,
    ParameterVector,
    QubitCount,
    ShotCount,
};

// =============================================================================
// Version
// =============================================================================

/// Stable QAOA algorithm contract version.
pub const QAOA_VERSION: AlgorithmVersion =
    AlgorithmVersion::new(1, 0, 0);

// =============================================================================
// QAOA parameter layout
// =============================================================================

/// Number of parameter layers used by QAOA.
///
/// QAOA with depth `p` contains:
///
/// - `p` cost angles (`gamma`);
/// - `p` mixer angles (`beta`);
///
/// for a total of `2p` classical parameters.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QaoaDepth(u64);

impl QaoaDepth {
    /// Creates a validated QAOA depth.
    pub fn new(depth: u64) -> Result<Self> {
        if depth == 0 {
            return Err(AlgorithmError::InvalidConfiguration {
                field: "depth".to_string(),
                message:
                    "QAOA depth must be greater than zero"
                        .to_string(),
            });
        }

        Ok(Self(depth))
    }

    /// Returns the number of QAOA layers.
    #[must_use]
    pub const fn get(self) -> u64 {
        self.0
    }

    /// Returns the required number of classical parameters.
    pub fn parameter_count(self) -> Result<u64> {
        self.0.checked_mul(2).ok_or_else(|| {
            AlgorithmError::ResourceLimitExceeded {
                resource: AlgorithmResource::Parameters,
                requested: u128::MAX,
                limit: u128::from(
                    super::types::DEFAULT_MAX_PARAMETERS,
                ),
                message:
                    "QAOA parameter count overflowed"
                        .to_string(),
            }
        })
    }
}

// =============================================================================
// Operator contract
// =============================================================================

/// Backend-independent QAOA operator contract.
///
/// A concrete cost or mixer Hamiltonian/observable implementation can satisfy
/// this contract without forcing QAOA to own a particular Pauli or sparse
/// matrix representation.
pub trait QaoaOperator {
    /// Returns the number of logical qubits acted upon.
    fn qubit_count(&self) -> Result<QubitCount>;

    /// Returns a stable backend-neutral identifier.
    ///
    /// This is an identifier only. It is not a backend handle, credential,
    /// network address, or mutable execution state.
    fn operator_id(&self) -> Result<String>;

    /// Validates the mathematical operator.
    fn validate(&self) -> Result<()> {
        let _ = self.qubit_count()?;
        let _ = self.operator_id()?;
        Ok(())
    }
}

// =============================================================================
// QAOA ansatz
// =============================================================================

/// QAOA circuit-construction boundary.
///
/// The ansatz is responsible for converting:
///
/// ```text
/// cost operator
/// mixer operator
/// beta/gamma parameters
/// depth
/// ```
///
/// into the canonical logical Quantum IR.
///
/// QAOA itself does not manipulate gates.
pub trait QaoaAnsatz {
    /// Returns the number of logical qubits.
    fn qubit_count(&self) -> Result<QubitCount>;

    /// Returns the number of parameters expected by this ansatz.
    fn parameter_count(&self) -> Result<u64>;

    /// Builds the logical QAOA circuit.
    fn build(
        &self,
        parameters: &ParameterVector,
    ) -> Result<QuantumCircuit>;

    /// Validates the ansatz.
    fn validate(&self) -> Result<()> {
        let _ = self.qubit_count()?;
        let _ = self.parameter_count()?;
        Ok(())
    }
}

// =============================================================================
// QAOA problem
// =============================================================================

/// Complete immutable QAOA problem definition.
pub struct QaoaProblem<C, M, A> {
    cost_operator: C,
    mixer_operator: M,
    ansatz: A,
    depth: QaoaDepth,
    initial_parameters: ParameterVector,
}

impl<C, M, A> fmt::Debug for QaoaProblem<C, M, A>
where
    C: fmt::Debug,
    M: fmt::Debug,
    A: fmt::Debug,
{
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_struct("QaoaProblem")
            .field(
                "cost_operator",
                &self.cost_operator,
            )
            .field(
                "mixer_operator",
                &self.mixer_operator,
            )
            .field("ansatz", &self.ansatz)
            .field("depth", &self.depth)
            .field(
                "initial_parameters",
                &self.initial_parameters,
            )
            .finish()
    }
}

impl<C, M, A> QaoaProblem<C, M, A>
where
    C: QaoaOperator,
    M: QaoaOperator,
    A: QaoaAnsatz,
{
    /// Creates and validates a QAOA problem.
    pub fn new(
        cost_operator: C,
        mixer_operator: M,
        ansatz: A,
        depth: QaoaDepth,
        initial_parameters: ParameterVector,
    ) -> Result<Self> {
        cost_operator.validate()?;
        mixer_operator.validate()?;
        ansatz.validate()?;

        let cost_qubits =
            cost_operator.qubit_count()?;

        let mixer_qubits =
            mixer_operator.qubit_count()?;

        let ansatz_qubits =
            ansatz.qubit_count()?;

        if cost_qubits != mixer_qubits {
            return Err(
                AlgorithmError::DimensionMismatch {
                    expected:
                        cost_qubits.get() as usize,
                    actual:
                        mixer_qubits.get() as usize,
                    context:
                        "QAOA cost/mixer qubit count"
                            .to_string(),
                },
            );
        }

        if cost_qubits != ansatz_qubits {
            return Err(
                AlgorithmError::DimensionMismatch {
                    expected:
                        cost_qubits.get() as usize,
                    actual:
                        ansatz_qubits.get() as usize,
                    context:
                        "QAOA operator/ansatz qubit count"
                            .to_string(),
                },
            );
        }

        let expected =
            depth.parameter_count()?;

        let actual =
            u64::try_from(
                initial_parameters.len(),
            )
            .map_err(|_| {
                AlgorithmError::ResourceLimitExceeded {
                    resource:
                        AlgorithmResource::Parameters,
                    requested:
                        u128::MAX,
                    limit:
                        u128::from(
                            super::types::DEFAULT_MAX_PARAMETERS,
                        ),
                    message:
                        "QAOA parameter vector is too large"
                            .to_string(),
                }
            })?;

        if expected != actual {
            return Err(
                AlgorithmError::DimensionMismatch {
                    expected:
                        expected as usize,
                    actual:
                        actual as usize,
                    context:
                        "QAOA initial parameter vector"
                            .to_string(),
                },
            );
        }

        if ansatz.parameter_count()? != expected {
            return Err(
                AlgorithmError::DimensionMismatch {
                    expected:
                        expected as usize,
                    actual:
                        ansatz
                            .parameter_count()?
                            as usize,
                    context:
                        "QAOA ansatz parameter count"
                            .to_string(),
                },
            );
        }

        Ok(Self {
            cost_operator,
            mixer_operator,
            ansatz,
            depth,
            initial_parameters,
        })
    }

    /// Returns the cost operator.
    pub fn cost_operator(&self) -> &C {
        &self.cost_operator
    }

    /// Returns the mixer operator.
    pub fn mixer_operator(&self) -> &M {
        &self.mixer_operator
    }

    /// Returns the ansatz.
    pub fn ansatz(&self) -> &A {
        &self.ansatz
    }

    /// Returns QAOA depth.
    #[must_use]
    pub const fn depth(&self) -> QaoaDepth {
        self.depth
    }

    /// Returns the initial parameters.
    pub fn initial_parameters(
        &self,
    ) -> &ParameterVector {
        &self.initial_parameters
    }

    /// Returns the logical qubit count.
    pub fn qubit_count(&self) -> Result<QubitCount> {
        self.cost_operator.qubit_count()
    }

    /// Returns the number of QAOA parameters.
    pub fn parameter_count(&self) -> Result<u64> {
        self.depth.parameter_count()
    }
}

// =============================================================================
// Configuration
// =============================================================================

/// Complete QAOA execution configuration.
#[derive(Debug, Clone, PartialEq)]
pub struct QaoaConfig {
    /// Quantum execution configuration.
    pub execution: ExecutionConfig,

    /// Classical optimization configuration.
    pub optimization: OptimizationConfig,

    /// Whether QAOA must reach the optimizer's convergence state.
    pub require_convergence: bool,

    /// Whether the optimized circuit should also be sampled.
    pub measure_solution: bool,
}

impl Default for QaoaConfig {
    fn default() -> Self {
        Self {
            execution:
                ExecutionConfig::default(),
            optimization:
                OptimizationConfig::default(),
            require_convergence: false,
            measure_solution: true,
        }
    }
}

impl QaoaConfig {
    /// Validates the complete configuration.
    pub fn validate(&self) -> Result<()> {
        self.execution.validate()?;
        self.optimization.validate()?;

        if self.execution.deterministic
            && self.execution.seed.is_none()
        {
            return Err(
                AlgorithmError::DeterminismViolation {
                    contract:
                        "deterministic QAOA execution"
                            .to_string(),
                    message:
                        "deterministic QAOA execution requires an explicit seed"
                            .to_string(),
                },
            );
        }

        if self.measure_solution
            && self.execution.shots.is_none()
        {
            return Err(
                AlgorithmError::InvalidConfiguration {
                    field: "execution.shots"
                        .to_string(),
                    message:
                        "QAOA solution measurement requires a positive shot count"
                            .to_string(),
                },
            );
        }

        Ok(())
    }
}

// =============================================================================
// QAOA statistics
// =============================================================================

/// Immutable accounting for one QAOA invocation.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct QaoaStatistics {
    /// Number of objective evaluations.
    pub objective_evaluations: u64,

    /// Number of logical circuit executions.
    pub circuit_executions: u64,

    /// Number of measurement shots.
    pub shots: u64,

    /// Number of optimizer iterations.
    pub optimizer_iterations: u64,

    /// Number of optimizer steps.
    pub optimizer_steps: u64,

    /// Number of final solution-sampling circuit executions.
    pub solution_sampling_executions: u64,
}

// =============================================================================
// QAOA result
// =============================================================================

/// Complete QAOA result.
#[derive(Debug, Clone, PartialEq)]
pub struct QaoaResult {
    /// Stable algorithm metadata.
    pub metadata: AlgorithmMetadata,

    /// QAOA depth.
    pub depth: QaoaDepth,

    /// Final optimized parameters.
    pub parameters: ParameterVector,

    /// Best parameters observed.
    pub best_parameters: ParameterVector,

    /// Final objective value.
    pub objective: ObjectiveValue,

    /// Best objective value observed.
    pub best_objective: ObjectiveValue,

    /// Classical optimization result.
    pub optimization: OptimizationResult,

    /// Most probable measured solution, when measurement was requested.
    pub best_bitstring: Option<String>,

    /// Probability of the most probable measured solution, when supplied by
    /// the executor.
    pub best_probability: Option<f64>,

    /// QAOA execution accounting.
    pub statistics: QaoaStatistics,
}

impl QaoaResult {
    /// Returns whether optimization converged.
    #[must_use]
    pub fn converged(&self) -> bool {
        self.optimization.converged()
    }

    /// Validates the result.
    pub fn validate(&self) -> Result<()> {
        if self.metadata.algorithm
            != AlgorithmId::Qaoa
        {
            return Err(
                AlgorithmError::InternalInvariantViolation {
                    message:
                        "QAOA result metadata must identify QAOA"
                            .to_string(),
                },
            );
        }

        if self.metadata.version
            != QAOA_VERSION
        {
            return Err(
                AlgorithmError::InternalInvariantViolation {
                    message:
                        "QAOA result contains an unexpected algorithm version"
                            .to_string(),
                },
            );
        }

        if let Some(probability) =
            self.best_probability
        {
            if !probability.is_finite()
                || !(0.0..=1.0)
                    .contains(&probability)
            {
                return Err(
                    AlgorithmError::NonFiniteValue {
                        field:
                            "best_probability"
                                .to_string(),
                        value:
                            probability,
                    },
                );
            }
        }

        if self.parameters.is_empty()
            || self.best_parameters.is_empty()
        {
            return Err(
                AlgorithmError::InternalInvariantViolation {
                    message:
                        "QAOA result parameter vectors cannot be empty"
                            .to_string(),
                },
            );
        }

        if self.parameters.len()
            != self.best_parameters.len()
        {
            return Err(
                AlgorithmError::InternalInvariantViolation {
                    message:
                        "QAOA final and best parameter dimensions differ"
                            .to_string(),
                },
            );
        }

        self.optimization.validate()?;

        Ok(())
    }
}

// =============================================================================
// QAOA objective
// =============================================================================

/// Quantum cost objective used by QAOA.
struct QaoaCostObjective<'a, C, A, E> {
    cost_operator: &'a C,
    ansatz: &'a A,
    executor: &'a mut E,
    execution_config: ExecutionConfig,

    evaluations: u64,
    circuit_executions: u64,
    shots: u64,
}

impl<'a, C, A, E>
    QaoaCostObjective<'a, C, A, E>
where
    C: QaoaOperator,
    A: QaoaAnsatz,
    E: QuantumExecutor,
{
    fn new(
        cost_operator: &'a C,
        ansatz: &'a A,
        executor: &'a mut E,
        execution_config: ExecutionConfig,
    ) -> Self {
        Self {
            cost_operator,
            ansatz,
            executor,
            execution_config,
            evaluations: 0,
            circuit_executions: 0,
            shots: 0,
        }
    }

    fn statistics(&self) -> QaoaStatistics {
        QaoaStatistics {
            objective_evaluations:
                self.evaluations,
            circuit_executions:
                self.circuit_executions,
            shots: self.shots,
            optimizer_iterations: 0,
            optimizer_steps: 0,
            solution_sampling_executions: 0,
        }
    }
}

impl<'a, C, A, E> Objective
    for QaoaCostObjective<'a, C, A, E>
where
    C: QaoaOperator,
    A: QaoaAnsatz,
    E: QuantumExecutor,
{
    fn evaluate(
        &mut self,
        parameters: &ParameterVector,
    ) -> Result<ObjectiveEvaluation> {
        self.evaluations =
            self.evaluations
                .checked_add(1)
                .ok_or_else(|| {
                    AlgorithmError::ResourceLimitExceeded {
                        resource:
                            AlgorithmResource::ObjectiveEvaluations,
                        requested:
                            u128::MAX,
                        limit:
                            u128::from(
                                super::types::
                                    DEFAULT_MAX_OBJECTIVE_EVALUATIONS,
                            ),
                        message:
                            "QAOA objective evaluation counter overflowed"
                                .to_string(),
                    }
                })?;

        let circuit =
            self.ansatz.build(parameters)?;

        let observable =
            self.cost_operator.operator_id()?;

        let request =
            ExecutionRequest::expectation(
                circuit,
                self.execution_config.clone(),
            )?
            .with_algorithm("qaoa")?
            .with_operation(
                "cost_expectation",
            )?
            .with_observable(
                observable,
            )?;

        let result =
            execute(
                self.executor,
                &request,
            )?;

        self.circuit_executions =
            self.circuit_executions
                .checked_add(
                    result
                        .circuit_executions(),
                )
                .ok_or_else(|| {
                    AlgorithmError::ResourceLimitExceeded {
                        resource:
                            AlgorithmResource::CircuitExecutions,
                        requested:
                            u128::MAX,
                        limit:
                            u128::from(
                                super::types::
                                    DEFAULT_MAX_CIRCUIT_EXECUTIONS,
                            ),
                        message:
                            "QAOA circuit execution counter overflowed"
                                .to_string(),
                    }
                })?;

        if let Some(shots) =
            result.shots_executed()
        {
            self.shots =
                self.shots
                    .checked_add(
                        shots.get(),
                    )
                    .ok_or_else(|| {
                        AlgorithmError::ResourceLimitExceeded {
                            resource:
                                AlgorithmResource::Shots,
                            requested:
                                u128::MAX,
                            limit:
                                u128::from(
                                    super::types::
                                        DEFAULT_MAX_SHOTS,
                                ),
                            message:
                                "QAOA shot counter overflowed"
                                    .to_string(),
                        }
                    })?;
        }

        let expectation =
            result.expectation()
                .ok_or_else(|| {
                    AlgorithmError::ExecutionFailed {
                        backend: Some(
                            result
                                .metadata()
                                .backend_id
                                .clone(),
                        ),
                        operation:
                            "qaoa_cost_expectation"
                                .to_string(),
                        message:
                            "executor returned no QAOA cost expectation"
                                .to_string(),
                    }
                })?;

        let value =
            ObjectiveValue::new(
                expectation.get(),
            )?;

        ObjectiveEvaluation::quantum(
            value,
            self.evaluations,
            result.circuit_executions(),
            result
                .shots_executed()
                .map(|shots| shots.get()),
            result
                .metadata()
                .deterministic,
        )
    }

    fn direction(
        &self,
    ) -> ObjectiveDirection {
        ObjectiveDirection::Minimize
    }

    fn kind(
        &self,
    ) -> ObjectiveKind {
        ObjectiveKind::Cost
    }
}

// =============================================================================
// QAOA engine
// =============================================================================

/// Backend-independent QAOA orchestration engine.
#[derive(Debug, Clone, Copy, Default)]
pub struct Qaoa;

impl Qaoa {
    /// Creates a QAOA engine.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Returns stable QAOA metadata.
    #[must_use]
    pub const fn metadata()
        -> AlgorithmMetadata
    {
        AlgorithmMetadata::new(
            AlgorithmId::Qaoa,
            QAOA_VERSION,
        )
    }

    /// Optimizes a QAOA problem.
    ///
    /// This performs the expectation-value optimization stage only.
    pub fn solve<C, M, A, E, O>(
        &self,
        problem: &QaoaProblem<C, M, A>,
        executor: &mut E,
        optimizer: &mut O,
        config: &QaoaConfig,
    ) -> Result<QaoaResult>
    where
        C: QaoaOperator,
        M: QaoaOperator,
        A: QaoaAnsatz,
        E: QuantumExecutor,
        O: Optimizer,
    {
        config.validate()?;

        problem.cost_operator.validate()?;
        problem.mixer_operator.validate()?;
        problem.ansatz.validate()?;

        let mut objective =
            QaoaCostObjective::new(
                &problem.cost_operator,
                &problem.ansatz,
                executor,
                config.execution.clone(),
            );

        let optimization =
            optimizer.optimize(
                &mut objective,
                problem
                    .initial_parameters
                    .clone(),
                &config.optimization,
            )?;

        optimization.validate()?;

        if config.require_convergence
            && !optimization.converged()
        {
            return Err(
                AlgorithmError::OptimizationFailed {
                    message: format!(
                        "QAOA optimizer terminated with status {}",
                        optimization.status
                    ),
                },
            );
        }

        let statistics =
            objective.statistics();

        let mut result =
            QaoaResult {
                metadata:
                    Self::metadata(),
                depth:
                    problem.depth,
                parameters:
                    optimization
                        .parameters
                        .clone(),
                best_parameters:
                    optimization
                        .best_parameters
                        .clone(),
                objective:
                    optimization
                        .objective,
                best_objective:
                    optimization
                        .best_objective,
                optimization,
                best_bitstring:
                    None,
                best_probability:
                    None,
                statistics,
            };

        if config.measure_solution {
            let measurement =
                Self::measure_solution(
                    &problem.ansatz,
                    &result.best_parameters,
                    executor,
                    config.execution.clone(),
                )?;

            let statistics =
                &mut result.statistics;

            statistics
                .solution_sampling_executions =
                statistics
                    .solution_sampling_executions
                    .checked_add(
                        measurement
                            .circuit_executions(),
                    )
                    .ok_or_else(|| {
                        AlgorithmError::ResourceLimitExceeded {
                            resource:
                                AlgorithmResource::CircuitExecutions,
                            requested:
                                u128::MAX,
                            limit:
                                u128::from(
                                    super::types::
                                        DEFAULT_MAX_CIRCUIT_EXECUTIONS,
                                ),
                            message:
                                "QAOA solution execution counter overflowed"
                                    .to_string(),
                        }
                    })?;

            if let Some(shots) =
                measurement.shots_executed()
            {
                statistics.shots =
                    statistics
                        .shots
                        .checked_add(
                            shots.get(),
                        )
                        .ok_or_else(|| {
                            AlgorithmError::ResourceLimitExceeded {
                                resource:
                                    AlgorithmResource::Shots,
                                requested:
                                    u128::MAX,
                                limit:
                                    u128::from(
                                        super::types::
                                            DEFAULT_MAX_SHOTS,
                                    ),
                                message:
                                    "QAOA solution shot counter overflowed"
                                        .to_string(),
                            }
                        })?;
            }

            if let Some((bitstring, probability)) =
                Self::best_measurement(
                    &measurement,
                )
            {
                result.best_bitstring =
                    Some(bitstring);

                result.best_probability =
                    Some(probability);
            }
        }

        result.validate()?;

        Ok(result)
    }

    /// Executes the optimized circuit in measurement mode.
    fn measure_solution<A, E>(
        ansatz: &A,
        parameters: &ParameterVector,
        executor: &mut E,
        mut config: ExecutionConfig,
    ) -> Result<
        super::execution::ExecutionResult,
    >
    where
        A: QaoaAnsatz,
        E: QuantumExecutor,
    {
        let circuit =
            ansatz.build(parameters)?;

        /*
         * Measurement requires shots. QaoaConfig::validate has already
         * guaranteed this when solution measurement is enabled.
         */
        if config.shots.is_none() {
            return Err(
                AlgorithmError::InvalidConfiguration {
                    field:
                        "execution.shots"
                            .to_string(),
                    message:
                        "QAOA solution measurement requires shots"
                            .to_string(),
                },
            );
        }

        /*
         * Preserve every execution-policy field. We only change the
         * operation at the request boundary.
         */
        let request =
            ExecutionRequest::measurement(
                circuit,
                config.clone(),
            )?
            .with_algorithm("qaoa")?
            .with_operation(
                "solution_measurement",
            )?;

        execute(
            executor,
            &request,
        )
    }

    /// Selects the highest-probability measured state.
    ///
    /// `ExecutionResult::probabilities()` is a deterministic `BTreeMap`, so
    /// ties are resolved lexicographically by bitstring.
    fn best_measurement(
        result:
            &super::execution::ExecutionResult,
    ) -> Option<(String, f64)> {
        result
            .probabilities()
            .iter()
            .max_by(
                |(left_key, left_probability),
                 (right_key, right_probability)| {
                    left_probability
                        .get()
                        .partial_cmp(
                            &right_probability.get(),
                        )
                        .unwrap_or(
                            std::cmp::Ordering::Equal,
                        )
                        .then_with(|| {
                            /*
                             * Reverse lexical comparison so the smaller
                             * bitstring wins when probabilities are equal.
                             */
                            right_key
                                .cmp(left_key)
                        })
                },
            )
            .map(
                |(bitstring, probability)| {
                    (
                        bitstring.clone(),
                        probability.get(),
                    )
                },
            )
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct TestOperator {
        qubits: QubitCount,
        id: &'static str,
    }

    impl QaoaOperator for TestOperator {
        fn qubit_count(
            &self,
        ) -> Result<QubitCount> {
            Ok(self.qubits)
        }

        fn operator_id(
            &self,
        ) -> Result<String> {
            Ok(self.id.to_string())
        }
    }

    #[test]
    fn depth_rejects_zero() {
        assert!(
            QaoaDepth::new(0).is_err()
        );
    }

    #[test]
    fn depth_parameter_count_is_two_p() {
        let depth =
            QaoaDepth::new(3)
                .expect("valid depth");

        assert_eq!(
            depth
                .parameter_count()
                .expect("valid parameter count"),
            6
        );
    }

    #[test]
    fn problem_rejects_operator_dimension_mismatch() {
        let cost =
            TestOperator {
                qubits:
                    QubitCount::new(2)
                        .expect("valid qubit count"),
                id: "cost",
            };

        let mixer =
            TestOperator {
                qubits:
                    QubitCount::new(3)
                        .expect("valid qubit count"),
                id: "mixer",
            };

        struct TestAnsatz;

        impl QaoaAnsatz for TestAnsatz {
            fn qubit_count(
                &self,
            ) -> Result<QubitCount> {
                QubitCount::new(2)
            }

            fn parameter_count(
                &self,
            ) -> Result<u64> {
                Ok(2)
            }

            fn build(
                &self,
                _parameters:
                    &ParameterVector,
            ) -> Result<QuantumCircuit> {
                Err(
                    AlgorithmError::UnsupportedOperation {
                        operation:
                            "test".to_string(),
                        message:
                            "not used"
                                .to_string(),
                    },
                )
            }
        }

        let result =
            QaoaProblem::new(
                cost,
                mixer,
                TestAnsatz,
                QaoaDepth::new(1)
                    .expect("valid depth"),
                ParameterVector::new(
                    vec![0.0, 0.0],
                )
                .expect(
                    "valid parameters",
                ),
            );

        assert!(result.is_err());
    }

    #[test]
    fn problem_rejects_parameter_dimension_mismatch() {
        let operator =
            TestOperator {
                qubits:
                    QubitCount::new(2)
                        .expect("valid qubit count"),
                id: "operator",
            };

        struct TestAnsatz;

        impl QaoaAnsatz for TestAnsatz {
            fn qubit_count(
                &self,
            ) -> Result<QubitCount> {
                QubitCount::new(2)
            }

            fn parameter_count(
                &self,
            ) -> Result<u64> {
                Ok(4)
            }

            fn build(
                &self,
                _parameters:
                    &ParameterVector,
            ) -> Result<QuantumCircuit> {
                Err(
                    AlgorithmError::UnsupportedOperation {
                        operation:
                            "test".to_string(),
                        message:
                            "not used"
                                .to_string(),
                    },
                )
            }
        }

        let result =
            QaoaProblem::new(
                operator,
                TestOperator {
                    qubits:
                        QubitCount::new(2)
                            .expect(
                                "valid qubit count",
                            ),
                    id: "mixer",
                },
                TestAnsatz,
                QaoaDepth::new(2)
                    .expect("valid depth"),
                ParameterVector::new(
                    vec![0.0, 0.0],
                )
                .expect(
                    "valid parameters",
                ),
            );

        assert!(result.is_err());
    }

    #[test]
    fn deterministic_config_requires_seed() {
        let mut config =
            QaoaConfig::default();

        config.execution.deterministic =
            true;

        config.execution.seed = None;

        assert!(
            config.validate().is_err()
        );
    }

    #[test]
    fn metadata_identifies_qaoa() {
        let metadata =
            Qaoa::metadata();

        assert_eq!(
            metadata.algorithm,
            AlgorithmId::Qaoa
        );

        assert_eq!(
            metadata.version,
            QAOA_VERSION
        );
    }
}