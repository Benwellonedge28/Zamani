//! Zamani Quantum Algorithms — Grover Search.
//!
//! Production-grade, backend-independent Grover-search orchestration.
//!
//! # Responsibility
//!
//! This module owns:
//!
//! - Grover problem validation;
//! - search-space and solution-count validation;
//! - iteration-policy calculation;
//! - Grover circuit-construction boundary;
//! - execution-policy validation;
//! - final measurement execution;
//! - deterministic result selection;
//! - resource accounting;
//! - algorithm metadata and result validation.
//!
//! This module deliberately does NOT own:
//!
//! - quantum gates;
//! - circuit storage;
//! - IR semantics;
//! - oracle gate implementation;
//! - diffuser gate implementation;
//! - physical qubit mapping;
//! - routing;
//! - transpilation;
//! - hardware;
//! - simulator implementation;
//! - QPU/vendor APIs;
//! - error correction;
//! - persistence.
//!
//! # Architecture
//!
//! ```text
//! GroverProblem
//!      │
//!      ├── search space
//!      ├── solution count
//!      ├── logical qubit count
//!      └── GroverCircuitBuilder
//!              │
//!              ▼
//!      iteration calculation
//!              │
//!              ▼
//!        QuantumCircuit
//!              │
//!              ▼
//!       ExecutionRequest
//!              │
//!              ▼
//!       QuantumExecutor
//!              │
//!              ▼
//!       ExecutionResult
//!              │
//!              ▼
//!       GroverResult
//! ```
//!
//! # IR boundary
//!
//! Grover never constructs gates directly.
//!
//! A [`GroverCircuitBuilder`] produces the repository's canonical
//! `quantum::ir::QuantumCircuit`.
//!
//! This keeps circuit semantics in `quantum::ir` and prevents Grover from
//! becoming coupled to a particular gate representation.
//!
//! # Determinism
//!
//! Grover does not create randomness itself.
//!
//! Explicit execution seeds are supplied through [`ExecutionConfig`].
//! Deterministic execution is enforced by `execution.rs`.
//!
//! When equal-probability states are returned, the deterministic
//! `BTreeMap` ordering is used as the tie-breaking contract.
//!
//! # Resource safety
//!
//! The calculated iteration count is checked against the configured maximum
//! before circuit construction. This prevents an accidentally enormous Grover
//! invocation from crossing the execution boundary.
//!
//! # Rust compatibility
//!
//! Rust 1.97.1.
//!
//! No nightly features.
//!
//! # Safety
//!
//! This module contains no unsafe code.

#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::cmp::Ordering;
use std::fmt;

use crate::quantum::ir::QuantumCircuit;

use super::error::{
    AlgorithmError,
    Result,
};
use super::execution::{
    execute,
    ExecutionConfig,
    ExecutionRequest,
    ExecutionResult,
    QuantumExecutor,
};
use super::types::{
    AlgorithmId,
    AlgorithmMetadata,
    AlgorithmVersion,
    ParameterVector,
    Probability,
    QubitCount,
    DEFAULT_MAX_ITERATIONS,
};

// =============================================================================
// Version
// =============================================================================

/// Stable Grover algorithm contract version.
pub const GROVER_VERSION: AlgorithmVersion =
    AlgorithmVersion::new(1, 0, 0);

// =============================================================================
// Iteration policy
// =============================================================================

/// Grover iteration policy.
///
/// The optimal iteration count depends on the search-space size and the
/// number of marked solutions. Callers may also supply an explicit count.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GroverIterationPolicy {
    /// Calculate the standard near-optimal iteration count.
    Optimal,

    /// Execute exactly the supplied number of Grover iterations.
    Explicit(u64),
}

impl Default for GroverIterationPolicy {
    fn default() -> Self {
        Self::Optimal
    }
}

impl GroverIterationPolicy {
    /// Returns the policy's stable identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Optimal => "optimal",
            Self::Explicit(_) => "explicit",
        }
    }
}

// =============================================================================
// Circuit construction contract
// =============================================================================

/// Backend-independent Grover circuit construction boundary.
///
/// The builder owns the mathematical construction of:
///
/// - state preparation;
/// - oracle;
/// - diffusion operator;
/// - repeated Grover iterations;
/// - any required measurement-independent finalization.
///
/// It returns the canonical logical `QuantumCircuit`.
///
/// Grover orchestration itself never manipulates gates.
pub trait GroverCircuitBuilder {
    /// Returns the number of logical qubits used by the search circuit.
    fn qubit_count(&self) -> Result<QubitCount>;

    /// Returns a stable identifier for the circuit construction.
    ///
    /// This must be an identifier, not a backend handle or credential.
    fn builder_id(&self) -> Result<String>;

    /// Validates the builder independently of a concrete backend.
    fn validate(&self) -> Result<()> {
        let _ = self.qubit_count()?;
        let _ = self.builder_id()?;
        Ok(())
    }

    /// Builds a complete logical Grover circuit.
    ///
    /// `iterations` is guaranteed by the caller to be within the configured
    /// algorithm-level resource limit.
    fn build(&self, iterations: u64) -> Result<QuantumCircuit>;
}

// =============================================================================
// Problem definition
// =============================================================================

/// Immutable Grover problem definition.
///
/// `search_space_size` represents the number of candidate states actually
/// being searched. It does not have to equal `2^qubits`, although it cannot
/// exceed the representable state space of the supplied logical qubit count.
///
/// `solution_count` is the number of marked/valid states.
pub struct GroverProblem<B> {
    search_space_size: u64,
    solution_count: u64,
    builder: B,
    qubit_count: QubitCount,
}

impl<B> fmt::Debug for GroverProblem<B>
where
    B: fmt::Debug,
{
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_struct("GroverProblem")
            .field(
                "search_space_size",
                &self.search_space_size,
            )
            .field(
                "solution_count",
                &self.solution_count,
            )
            .field("builder", &self.builder)
            .field("qubit_count", &self.qubit_count)
            .finish()
    }
}

impl<B> GroverProblem<B>
where
    B: GroverCircuitBuilder,
{
    /// Creates and validates a Grover problem.
    pub fn new(
        search_space_size: u64,
        solution_count: u64,
        builder: B,
    ) -> Result<Self> {
        if search_space_size == 0 {
            return Err(
                AlgorithmError::invalid_input(
                    "search_space_size",
                    "Grover search space must contain at least one state",
                ),
            );
        }

        if solution_count == 0 {
            return Err(
                AlgorithmError::invalid_input(
                    "solution_count",
                    "Grover requires at least one marked solution",
                ),
            );
        }

        if solution_count > search_space_size {
            return Err(
                AlgorithmError::dimension_mismatch(
                    "search_space_size",
                    search_space_size as usize,
                    "solution_count",
                    solution_count as usize,
                    "number of marked solutions cannot exceed the search space",
                ),
            );
        }

        builder.validate()?;

        let qubit_count = builder.qubit_count()?;

        let representable_states = representable_state_count(
            qubit_count.get(),
        );

        if search_space_size > representable_states {
            return Err(
                AlgorithmError::invalid_configuration(
                    "search_space_size",
                    format!(
                        "search space {} exceeds the {}-qubit logical state space",
                        search_space_size,
                        representable_states
                    ),
                ),
            );
        }

        Ok(Self {
            search_space_size,
            solution_count,
            builder,
            qubit_count,
        })
    }

    /// Returns the number of candidate states.
    #[must_use]
    pub const fn search_space_size(&self) -> u64 {
        self.search_space_size
    }

    /// Returns the number of marked states.
    #[must_use]
    pub const fn solution_count(&self) -> u64 {
        self.solution_count
    }

    /// Returns the logical qubit count.
    #[must_use]
    pub const fn qubit_count(&self) -> QubitCount {
        self.qubit_count
    }

    /// Returns the circuit builder.
    pub fn builder(&self) -> &B {
        &self.builder
    }

    /// Calculates the standard near-optimal Grover iteration count.
    ///
    /// For `M` marked states in a search space of `N`, the standard estimate
    /// is:
    ///
    /// `floor((pi / 4) * sqrt(N / M))`
    ///
    /// At least one iteration is returned for a non-trivial search where the
    /// mathematical estimate would otherwise round to zero.
    pub fn optimal_iterations(&self) -> Result<u64> {
        let n = self.search_space_size as f64;
        let m = self.solution_count as f64;

        let ratio = n / m;

        if !ratio.is_finite() || ratio <= 0.0 {
            return Err(
                AlgorithmError::NumericalInstability {
                    operation:
                        "grover_iteration_calculation"
                            .to_string(),
                    message:
                        "invalid search-space/solution ratio"
                            .to_string(),
                },
            );
        }

        let estimate =
            (std::f64::consts::PI / 4.0)
                * ratio.sqrt();

        if !estimate.is_finite() {
            return Err(
                AlgorithmError::NumericalInstability {
                    operation:
                        "grover_iteration_calculation"
                            .to_string(),
                    message:
                        "Grover iteration estimate is non-finite"
                            .to_string(),
                },
            );
        }

        let iterations = estimate.floor() as u64;

        Ok(iterations.max(1))
    }
}

// =============================================================================
// Configuration
// =============================================================================

/// Complete Grover execution configuration.
#[derive(Debug, Clone, PartialEq)]
pub struct GroverConfig {
    /// Quantum execution policy.
    pub execution: ExecutionConfig,

    /// Iteration-count policy.
    pub iteration_policy: GroverIterationPolicy,

    /// Maximum allowed Grover iterations.
    pub max_iterations: u64,

    /// Whether measurement of the final Grover state is required.
    ///
    /// Grover's production result is normally based on measurement, so this
    /// defaults to `true`.
    pub measure_solution: bool,
}

impl Default for GroverConfig {
    fn default() -> Self {
        Self {
            execution:
                ExecutionConfig::default(),
            iteration_policy:
                GroverIterationPolicy::Optimal,
            max_iterations:
                DEFAULT_MAX_ITERATIONS,
            measure_solution: true,
        }
    }
}

impl GroverConfig {
    /// Validates the complete execution configuration.
    pub fn validate(&self) -> Result<()> {
        self.execution.validate()?;

        if self.max_iterations == 0 {
            return Err(
                AlgorithmError::invalid_configuration(
                    "max_iterations",
                    "maximum Grover iterations must be greater than zero",
                ),
            );
        }

        if self.max_iterations
            > DEFAULT_MAX_ITERATIONS
        {
            return Err(
                AlgorithmError::resource_limit_exceeded(
                    "iterations",
                    self.max_iterations,
                    DEFAULT_MAX_ITERATIONS,
                    "Grover maximum iterations exceeds the global algorithm limit",
                ),
            );
        }

        if let GroverIterationPolicy::Explicit(
            iterations,
        ) = self.iteration_policy
        {
            if iterations == 0 {
                return Err(
                    AlgorithmError::invalid_configuration(
                        "iteration_policy",
                        "explicit Grover iteration count must be greater than zero",
                    ),
                );
            }

            if iterations > self.max_iterations {
                return Err(
                    AlgorithmError::resource_limit_exceeded(
                        "iterations",
                        iterations,
                        self.max_iterations,
                        "explicit Grover iteration count exceeds configured maximum",
                    ),
                );
            }
        }

        if self.measure_solution
            && self.execution.shots.is_none()
        {
            return Err(
                AlgorithmError::invalid_configuration(
                    "execution.shots",
                    "Grover solution measurement requires a positive shot count",
                ),
            );
        }

        if self.execution.deterministic
            && self.execution.seed.is_none()
        {
            return Err(
                AlgorithmError::DeterminismViolation {
                    contract:
                        "deterministic Grover execution"
                            .to_string(),
                    message:
                        "deterministic Grover execution requires an explicit seed"
                            .to_string(),
                },
            );
        }

        Ok(())
    }
}

// =============================================================================
// Statistics
// =============================================================================

/// Immutable accounting for one Grover invocation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct GroverStatistics {
    /// Number of Grover iterations requested.
    pub iterations: u64,

    /// Number of logical circuit executions.
    pub circuit_executions: u64,

    /// Number of measurement shots represented by the result.
    pub shots: u64,
}

// =============================================================================
// Result
// =============================================================================

/// Complete production Grover result.
#[derive(Debug, Clone, PartialEq)]
pub struct GroverResult {
    /// Stable algorithm metadata.
    pub metadata: AlgorithmMetadata,

    /// Number of candidate states.
    pub search_space_size: u64,

    /// Number of marked states.
    pub solution_count: u64,

    /// Logical qubit count.
    pub qubit_count: QubitCount,

    /// Number of Grover iterations executed.
    pub iterations: u64,

    /// Most probable measured candidate.
    pub best_bitstring: Option<String>,

    /// Probability of the selected candidate.
    pub best_probability: Option<f64>,

    /// Complete execution statistics.
    pub statistics: GroverStatistics,
}

impl GroverResult {
    /// Returns whether a measured solution is available.
    #[must_use]
    pub fn has_solution(&self) -> bool {
        self.best_bitstring.is_some()
    }

    /// Validates the complete result contract.
    pub fn validate(&self) -> Result<()> {
        if self.metadata.algorithm
            != AlgorithmId::Grover
        {
            return Err(
                AlgorithmError::InternalInvariantViolation {
                    invariant:
                        "grover_result_algorithm_identity"
                            .to_string(),
                    message:
                        "Grover result metadata must identify Grover"
                            .to_string(),
                },
            );
        }

        if self.metadata.version
            != GROVER_VERSION
        {
            return Err(
                AlgorithmError::InternalInvariantViolation {
                    invariant:
                        "grover_result_algorithm_version"
                            .to_string(),
                    message:
                        "Grover result contains an unexpected algorithm version"
                            .to_string(),
                },
            );
        }

        if self.search_space_size == 0 {
            return Err(
                AlgorithmError::InternalInvariantViolation {
                    invariant:
                        "grover_non_empty_search_space"
                            .to_string(),
                    message:
                        "Grover result cannot contain an empty search space"
                            .to_string(),
                },
            );
        }

        if self.solution_count == 0
            || self.solution_count
                > self.search_space_size
        {
            return Err(
                AlgorithmError::InternalInvariantViolation {
                    invariant:
                        "grover_solution_count"
                            .to_string(),
                    message:
                        "Grover result contains an invalid solution count"
                            .to_string(),
                },
            );
        }

        if self.iterations == 0 {
            return Err(
                AlgorithmError::InternalInvariantViolation {
                    invariant:
                        "grover_iterations"
                            .to_string(),
                    message:
                        "Grover result must contain at least one iteration"
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
                        value: probability,
                    },
                );
            }
        }

        if let Some(bitstring) =
            &self.best_bitstring
        {
            if bitstring.is_empty() {
                return Err(
                    AlgorithmError::InternalInvariantViolation {
                        invariant:
                            "grover_non_empty_bitstring"
                                .to_string(),
                        message:
                            "Grover result contains an empty bitstring"
                                .to_string(),
                    },
                );
            }

            if bitstring.len()
                != self.qubit_count.get() as usize
            {
                return Err(
                    AlgorithmError::dimension_mismatch(
                        "qubit_count",
                        self.qubit_count.get() as usize,
                        "bitstring_length",
                        bitstring.len(),
                        "measured Grover bitstring must contain exactly one bit per logical qubit",
                    ),
                );
            }

            if !bitstring
                .bytes()
                .all(|byte| byte == b'0' || byte == b'1')
            {
                return Err(
                    AlgorithmError::invalid_input(
                        "best_bitstring",
                        "Grover measurement result must contain only binary characters",
                    ),
                );
            }
        }

        Ok(())
    }
}

// =============================================================================
// Grover engine
// =============================================================================

/// Backend-independent Grover-search engine.
#[derive(Debug, Clone, Copy, Default)]
pub struct Grover;

impl Grover {
    /// Creates a Grover engine.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Returns stable Grover metadata.
    #[must_use]
    pub const fn metadata()
        -> AlgorithmMetadata
    {
        AlgorithmMetadata::new(
            AlgorithmId::Grover,
            GROVER_VERSION,
        )
    }

    /// Resolves the requested iteration policy.
    fn resolve_iterations<B>(
        problem: &GroverProblem<B>,
        config: &GroverConfig,
    ) -> Result<u64>
    where
        B: GroverCircuitBuilder,
    {
        let iterations =
            match config.iteration_policy {
                GroverIterationPolicy::Optimal =>
                    problem.optimal_iterations()?,
                GroverIterationPolicy::Explicit(
                    iterations,
                ) => iterations,
            };

        if iterations == 0 {
            return Err(
                AlgorithmError::invalid_configuration(
                    "iterations",
                    "Grover requires at least one iteration",
                ),
            );
        }

        if iterations
            > config.max_iterations
        {
            return Err(
                AlgorithmError::resource_limit_exceeded(
                    "iterations",
                    iterations,
                    config.max_iterations,
                    "calculated Grover iteration count exceeds configured maximum",
                ),
            );
        }

        if iterations
            > DEFAULT_MAX_ITERATIONS
        {
            return Err(
                AlgorithmError::resource_limit_exceeded(
                    "iterations",
                    iterations,
                    DEFAULT_MAX_ITERATIONS,
                    "Grover iteration count exceeds the global algorithm limit",
                ),
            );
        }

        Ok(iterations)
    }

    /// Executes Grover search and optionally measures the final state.
    pub fn search<B, E>(
        &self,
        problem: &GroverProblem<B>,
        executor: &mut E,
        config: &GroverConfig,
    ) -> Result<GroverResult>
    where
        B: GroverCircuitBuilder,
        E: QuantumExecutor,
    {
        config.validate()?;

        let iterations =
            Self::resolve_iterations(
                problem,
                config,
            )?;

        problem.builder.validate()?;

        let actual_qubits =
            problem.builder.qubit_count()?;

        if actual_qubits
            != problem.qubit_count
        {
            return Err(
                AlgorithmError::InternalInvariantViolation {
                    invariant:
                        "grover_problem_qubit_count"
                            .to_string(),
                    message:
                        "Grover builder qubit count changed after problem validation"
                            .to_string(),
                },
            );
        }

        let circuit =
            problem.builder.build(iterations)?;

        if config.measure_solution {
            let request =
                ExecutionRequest::measurement(
                    circuit,
                    config.execution.clone(),
                )?
                .with_algorithm("grover")?
                .with_operation(
                    "search_measurement",
                )?;

            let execution =
                execute(
                    executor,
                    &request,
                )?;

            Self::build_result(
                problem,
                iterations,
                execution,
            )
        } else {
            /*
             * State-preparation mode deliberately does not invent a
             * backend-specific state representation.
             */
            let request =
                ExecutionRequest::state_preparation(
                    circuit,
                    config.execution.clone(),
                )?
                .with_algorithm("grover")?
                .with_operation(
                    "search_state_preparation",
                )?;

            let execution =
                execute(
                    executor,
                    &request,
                )?;

            let result =
                GroverResult {
                    metadata:
                        Self::metadata(),
                    search_space_size:
                        problem
                            .search_space_size,
                    solution_count:
                        problem.solution_count,
                    qubit_count:
                        problem.qubit_count,
                    iterations,
                    best_bitstring: None,
                    best_probability: None,
                    statistics:
                        GroverStatistics {
                            iterations,
                            circuit_executions:
                                execution
                                    .circuit_executions(),
                            shots:
                                execution
                                    .shots_executed()
                                    .map(
                                        |shots| {
                                            shots
                                                .get()
                                        },
                                    )
                                    .unwrap_or(0),
                        },
                };

            result.validate()?;

            Ok(result)
        }
    }

    /// Converts an execution result into a validated Grover result.
    fn build_result<B>(
        problem: &GroverProblem<B>,
        iterations: u64,
        execution: ExecutionResult,
    ) -> Result<GroverResult>
    where
        B: GroverCircuitBuilder,
    {
        let selected =
            Self::best_measurement(
                execution.probabilities(),
            );

        let result =
            GroverResult {
                metadata:
                    Self::metadata(),
                search_space_size:
                    problem.search_space_size,
                solution_count:
                    problem.solution_count,
                qubit_count:
                    problem.qubit_count,
                iterations,
                best_bitstring:
                    selected
                        .as_ref()
                        .map(
                            |(state, _)| {
                                state.clone()
                            },
                        ),
                best_probability:
                    selected
                        .map(
                            |(_, probability)| {
                                probability
                            },
                        ),
                statistics:
                    GroverStatistics {
                        iterations,
                        circuit_executions:
                            execution
                                .circuit_executions(),
                        shots:
                            execution
                                .shots_executed()
                                .map(
                                    |shots| {
                                        shots.get()
                                    },
                                )
                                .unwrap_or(0),
                    },
            };

        result.validate()?;

        Ok(result)
    }

    /// Selects the highest-probability measured state.
    ///
    /// `ExecutionResult::probabilities()` is a `BTreeMap`, giving deterministic
    /// key ordering. When two states have exactly equal probability, the
    /// lexicographically smaller bitstring wins.
    fn best_measurement(
        probabilities:
            &std::collections::BTreeMap<
                String,
                Probability,
            >,
    ) -> Option<(String, f64)> {
        probabilities
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
                            Ordering::Equal,
                        )
                        .then_with(|| {
                            /*
                             * Reverse the lexical comparison because `max_by`
                             * otherwise selects the lexicographically larger
                             * key on an exact probability tie.
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
// Mathematical helpers
// =============================================================================

/// Returns the number of states representable by a logical qubit count.
///
/// For 64 or more qubits, `u64::MAX` is used as the conservative representable
/// bound because the algorithm-level search-space type itself is `u64`.
fn representable_state_count(
    qubits: u64,
) -> u64 {
    if qubits >= 64 {
        u64::MAX
    } else {
        1u64 << qubits
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct TestBuilder {
        qubits: QubitCount,
        id: &'static str,
    }

    impl GroverCircuitBuilder for TestBuilder {
        fn qubit_count(
            &self,
        ) -> Result<QubitCount> {
            Ok(self.qubits)
        }

        fn builder_id(
            &self,
        ) -> Result<String> {
            Ok(self.id.to_string())
        }

        fn build(
            &self,
            _iterations: u64,
        ) -> Result<QuantumCircuit> {
            Err(
                AlgorithmError::unsupported_operation(
                    "test_builder",
                    "circuit construction is not required by this validation test",
                ),
            )
        }
    }

    #[test]
    fn optimal_iteration_policy_is_default() {
        assert_eq!(
            GroverIterationPolicy::default(),
            GroverIterationPolicy::Optimal
        );
    }

    #[test]
    fn problem_rejects_empty_search_space() {
        let builder =
            TestBuilder {
                qubits:
                    QubitCount::new(3)
                        .expect("valid qubit count"),
                id: "test",
            };

        assert!(
            GroverProblem::new(
                0,
                1,
                builder,
            )
            .is_err()
        );
    }

    #[test]
    fn problem_rejects_zero_solutions() {
        let builder =
            TestBuilder {
                qubits:
                    QubitCount::new(3)
                        .expect("valid qubit count"),
                id: "test",
            };

        assert!(
            GroverProblem::new(
                8,
                0,
                builder,
            )
            .is_err()
        );
    }

    #[test]
    fn problem_rejects_too_many_solutions() {
        let builder =
            TestBuilder {
                qubits:
                    QubitCount::new(3)
                        .expect("valid qubit count"),
                id: "test",
            };

        assert!(
            GroverProblem::new(
                8,
                9,
                builder,
            )
            .is_err()
        );
    }

    #[test]
    fn problem_rejects_search_space_larger_than_qubit_space() {
        let builder =
            TestBuilder {
                qubits:
                    QubitCount::new(3)
                        .expect("valid qubit count"),
                id: "test",
            };

        assert!(
            GroverProblem::new(
                9,
                1,
                builder,
            )
            .is_err()
        );
    }

    #[test]
    fn optimal_iterations_for_single_solution() {
        let builder =
            TestBuilder {
                qubits:
                    QubitCount::new(2)
                        .expect("valid qubit count"),
                id: "test",
            };

        let problem =
            GroverProblem::new(
                4,
                1,
                builder,
            )
            .expect("valid Grover problem");

        assert_eq!(
            problem
                .optimal_iterations()
                .expect("valid iteration count"),
            1
        );
    }

    #[test]
    fn explicit_zero_iterations_are_rejected() {
        let config =
            GroverConfig {
                iteration_policy:
                    GroverIterationPolicy::Explicit(
                        0,
                    ),
                ..GroverConfig::default()
            };

        assert!(config.validate().is_err());
    }

    #[test]
    fn explicit_iterations_cannot_exceed_limit() {
        let config =
            GroverConfig {
                iteration_policy:
                    GroverIterationPolicy::Explicit(
                        100,
                    ),
                max_iterations: 10,
                ..GroverConfig::default()
            };

        assert!(config.validate().is_err());
    }

    #[test]
    fn deterministic_mode_requires_seed() {
        let config =
            GroverConfig {
                execution:
                    ExecutionConfig {
                        deterministic: true,
                        seed: None,
                        ..ExecutionConfig::default()
                    },
                ..GroverConfig::default()
            };

        assert!(config.validate().is_err());
    }

    #[test]
    fn measurement_requires_shots() {
        let config =
            GroverConfig {
                execution:
                    ExecutionConfig {
                        shots: None,
                        ..ExecutionConfig::default()
                    },
                measure_solution: true,
                ..GroverConfig::default()
            };

        assert!(config.validate().is_err());
    }

    #[test]
    fn representable_state_count_is_correct() {
        assert_eq!(
            representable_state_count(1),
            2
        );

        assert_eq!(
            representable_state_count(3),
            8
        );

        assert_eq!(
            representable_state_count(64),
            u64::MAX
        );
    }
}