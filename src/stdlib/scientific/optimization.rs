
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! Zamani Standard Library — Scientific Optimization.
//!
//! Path: src/stdlib/scientific/optimization.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: unsafe Rust is forbidden.
//!
//! # Purpose
//!
//! Define portable mathematical optimization problems, model evaluation
//! contracts, solver interfaces, execution budgets, cancellation, and
//! optimization results.
//!
//! # Architectural ownership
//!
//! This module owns:
//! - Mathematical optimization problem descriptions.
//! - Decision-variable metadata.
//! - Objective metadata and direction.
//! - Mathematical constraint descriptions.
//! - Model evaluation contracts.
//! - Optional gradient evaluation contracts.
//! - Solver integration contracts.
//! - Solver execution context and accounting.
//! - Optimization result and termination contracts.
//!
//! This module does not own:
//! - Numeric representation.
//! - Matrix, vector, or tensor storage.
//! - Compiler optimization passes.
//! - Compiler optimization planning.
//! - Canonical IR.
//! - Semantic analysis.
//! - Automatic differentiation.
//! - Quantum circuit optimization.
//! - AI model optimization.
//! - Hardware discovery or selection.
//! - Memory allocation policy.
//! - Runtime scheduling.
//! - Distributed execution.
//! - Target-specific numerical kernels.
//!
//! # Canonical dependencies
//!
//! Numeric representation:
//!     crate::quantum::ir::classical::float::ClassicalFloat
//!
//! Compiler optimization planning:
//!     crate::compiler::optimization
//!
//! Classical optimization:
//!     crate::classical::optimization
//!
//! Quantum optimization:
//!     crate::quantum::optimization
//!
//! AI optimization:
//!     crate::ai::optimization
//!
//! Semantic authority:
//!     crate::frontend::semantic
//!
//! # POCO-REAF
//!
//! Optimization problems describe mathematical intent, not a particular
//! processor, accelerator, quantum device, FPGA, ASIC, or deployment.
//!
//! No universal maximum is imposed on:
//! - Decision variables.
//! - Objectives.
//! - Constraints.
//! - Solver iterations.
//! - Objective evaluations.
//! - Gradient evaluations.
//! - Solver implementations.
//! - Optimization domains.
//!
//! Optional budgets are operational policies. They are not semantic
//! restrictions on what a Zamani program can express.
//!
//! Concrete execution remains subject to representational limits,
//! implementation capabilities, and available resources.
//!
//! # Integration
//!
//! 1. Semantic analysis validates source-level optimization intent.
//! 2. Domain lowering produces the appropriate canonical representation.
//! 3. Numerical applications implement `OptimizationModel`.
//! 4. Solver providers implement `OptimizationSolver`.
//! 5. Compiler/runtime infrastructure chooses an appropriate provider.
//! 6. The solver uses `SolverContext` for evaluation, cancellation,
//!    budget enforcement, and accounting.
//! 7. Results are validated before being consumed by downstream code.
//!
//! This module does not parse source code or create an optimization IR.

use core::fmt;
use std::collections::BTreeSet;
use std::num::NonZeroU64;
use std::time::{Duration, Instant};

use crate::quantum::ir::classical::float::ClassicalFloat;

/// Result type for scientific optimization operations.
pub type OptimizationResult<T> =
    Result<T, OptimizationError>;

/// Mathematical objective direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectiveDirection {
    /// Find a solution with a lower objective value.
    Minimize,

    /// Find a solution with a higher objective value.
    Maximize,
}

/// Mathematical relationship used by a constraint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConstraintRelation {
    /// Expression must be less than or equal to its bound.
    LessEqual,

    /// Expression must equal its bound within the configured tolerance.
    Equal,

    /// Expression must be greater than or equal to its bound.
    GreaterEqual,
}

/// Domain of a decision variable.
///
/// `Named` allows application-specific and future domains without
/// modifying this enum for every new optimization problem.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VariableDomain {
    /// A real-valued variable.
    Continuous,

    /// An integer-valued variable.
    Integer,

    /// A binary variable whose valid values are zero and one.
    Binary,

    /// An extensible domain identified by a qualified name.
    Named(String),
}

impl VariableDomain {
    /// Validates the domain descriptor.
    pub fn validate(&self) -> OptimizationResult<()> {
        if let Self::Named(name) = self {
            validate_identifier(name, "variable domain")?;
        }

        Ok(())
    }
}

/// A decision variable in an optimization problem.
#[derive(Debug, Clone, PartialEq)]
pub struct DecisionVariable {
    id: String,
    domain: VariableDomain,
    lower_bound: Option<ClassicalFloat>,
    upper_bound: Option<ClassicalFloat>,
}

impl DecisionVariable {
    /// Creates a decision variable.
    ///
    /// Bounds are optional. When supplied, they must be finite and
    /// the lower bound must not exceed the upper bound.
    pub fn new(
        id: impl Into<String>,
        domain: VariableDomain,
        lower_bound: Option<ClassicalFloat>,
        upper_bound: Option<ClassicalFloat>,
    ) -> OptimizationResult<Self> {
        let id = id.into();

        validate_identifier(&id, "decision variable")?;
        domain.validate()?;

        validate_optional_number(lower_bound, "lower bound")?;
        validate_optional_number(upper_bound, "upper bound")?;

        if let (Some(lower), Some(upper)) =
            (lower_bound, upper_bound)
        {
            if lower.get() > upper.get() {
                return Err(OptimizationError::InvalidBounds {
                    variable: id,
                });
            }
        }

        Ok(Self {
            id,
            domain,
            lower_bound,
            upper_bound,
        })
    }

    /// Returns the variable identifier.
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the variable domain.
    #[must_use]
    pub fn domain(&self) -> &VariableDomain {
        &self.domain
    }

    /// Returns the optional lower bound.
    #[must_use]
    pub const fn lower_bound(&self) -> Option<ClassicalFloat> {
        self.lower_bound
    }

    /// Returns the optional upper bound.
    #[must_use]
    pub const fn upper_bound(&self) -> Option<ClassicalFloat> {
        self.upper_bound
    }

    /// Checks whether a value satisfies this variable's domain and bounds.
    pub fn accepts(&self, value: ClassicalFloat) -> bool {
        let number = value.get();

        if !number.is_finite() {
            return false;
        }

        if let Some(lower) = self.lower_bound {
            if number < lower.get() {
                return false;
            }
        }

        if let Some(upper) = self.upper_bound {
            if number > upper.get() {
                return false;
            }
        }

        match &self.domain {
            VariableDomain::Continuous => true,

            VariableDomain::Integer => number.fract() == 0.0,

            VariableDomain::Binary => {
                number == 0.0 || number == 1.0
            }

            VariableDomain::Named(_) => true,
        }
    }
}

/// Description of one mathematical objective.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OptimizationObjective {
    id: String,
    direction: ObjectiveDirection,
}

impl OptimizationObjective {
    /// Creates an objective descriptor.
    pub fn new(
        id: impl Into<String>,
        direction: ObjectiveDirection,
    ) -> OptimizationResult<Self> {
        let id = id.into();

        validate_identifier(&id, "objective")?;

        Ok(Self { id, direction })
    }

    /// Returns the objective identifier.
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the optimization direction.
    #[must_use]
    pub const fn direction(&self) -> ObjectiveDirection {
        self.direction
    }
}

/// A mathematical constraint on an expression.
///
/// The model evaluates the expression and returns its value in the
/// corresponding position in `ModelEvaluation::constraint_values`.
#[derive(Debug, Clone, PartialEq)]
pub struct OptimizationConstraint {
    id: String,
    relation: ConstraintRelation,
    bound: ClassicalFloat,
}

impl OptimizationConstraint {
    /// Creates a mathematical constraint.
    pub fn new(
        id: impl Into<String>,
        relation: ConstraintRelation,
        bound: ClassicalFloat,
    ) -> OptimizationResult<Self> {
        let id = id.into();

        validate_identifier(&id, "constraint")?;
        validate_number(bound, "constraint bound")?;

        Ok(Self {
            id,
            relation,
            bound,
        })
    }

    /// Returns the constraint identifier.
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the constraint relationship.
    #[must_use]
    pub const fn relation(&self) -> ConstraintRelation {
        self.relation
    }

    /// Returns the constraint bound.
    #[must_use]
    pub const fn bound(&self) -> ClassicalFloat {
        self.bound
    }

    /// Checks a constraint expression value using an absolute tolerance.
    #[must_use]
    pub fn is_satisfied(
        &self,
        value: ClassicalFloat,
        tolerance: f64,
    ) -> bool {
        let actual = value.get();
        let expected = self.bound.get();

        if !actual.is_finite()
            || !tolerance.is_finite()
            || tolerance < 0.0
        {
            return false;
        }

        match self.relation {
            ConstraintRelation::LessEqual => {
                actual <= expected + tolerance
            }

            ConstraintRelation::Equal => {
                (actual - expected).abs() <= tolerance
            }

            ConstraintRelation::GreaterEqual => {
                actual >= expected - tolerance
            }
        }
    }
}

/// A complete mathematical optimization problem.
///
/// The problem is target-independent. It does not identify a CPU,
/// GPU, QPU, FPGA, ASIC, runtime, or execution provider.
#[derive(Debug, Clone, PartialEq)]
pub struct OptimizationProblem {
    id: String,
    variables: Vec<DecisionVariable>,
    objectives: Vec<OptimizationObjective>,
    constraints: Vec<OptimizationConstraint>,
}

impl OptimizationProblem {
    /// Creates and validates an optimization problem.
    ///
    /// A problem must contain at least one decision variable and
    /// one objective. Constraints are optional.
    pub fn new(
        id: impl Into<String>,
        variables: Vec<DecisionVariable>,
        objectives: Vec<OptimizationObjective>,
        constraints: Vec<OptimizationConstraint>,
    ) -> OptimizationResult<Self> {
        let id = id.into();

        validate_identifier(&id, "optimization problem")?;

        if variables.is_empty() {
            return Err(OptimizationError::EmptyProblem {
                component: "decision variables",
            });
        }

        if objectives.is_empty() {
            return Err(OptimizationError::EmptyProblem {
                component: "objectives",
            });
        }

        validate_unique(
            variables.iter().map(|item| item.id()),
            "decision variable",
        )?;

        validate_unique(
            objectives.iter().map(|item| item.id()),
            "objective",
        )?;

        validate_unique(
            constraints.iter().map(|item| item.id()),
            "constraint",
        )?;

        Ok(Self {
            id,
            variables,
            objectives,
            constraints,
        })
    }

    /// Returns the problem identifier.
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns decision variables in their canonical problem order.
    #[must_use]
    pub fn variables(&self) -> &[DecisionVariable] {
        &self.variables
    }

    /// Returns objective descriptors in their canonical problem order.
    #[must_use]
    pub fn objectives(&self) -> &[OptimizationObjective] {
        &self.objectives
    }

    /// Returns mathematical constraints in their canonical problem order.
    #[must_use]
    pub fn constraints(&self) -> &[OptimizationConstraint] {
        &self.constraints
    }

    /// Returns the number of decision variables.
    #[must_use]
    pub fn variable_count(&self) -> usize {
        self.variables.len()
    }

    /// Returns the number of objectives.
    #[must_use]
    pub fn objective_count(&self) -> usize {
        self.objectives.len()
    }

    /// Returns the number of constraints.
    #[must_use]
    pub fn constraint_count(&self) -> usize {
        self.constraints.len()
    }

    /// Validates a candidate variable vector.
    pub fn validate_candidate(
        &self,
        values: &[ClassicalFloat],
    ) -> OptimizationResult<()> {
        if values.len() != self.variables.len() {
            return Err(OptimizationError::DimensionMismatch {
                component: "decision variables",
                expected: self.variables.len(),
                actual: values.len(),
            });
        }

        for (index, (variable, value)) in
            self.variables.iter().zip(values).enumerate()
        {
            if !variable.accepts(*value) {
                return Err(OptimizationError::InvalidVariableValue {
                    variable: variable.id().to_owned(),
                    index,
                });
            }
        }

        Ok(())
    }
}

/// Evaluated objective and constraint values for one candidate.
#[derive(Debug, Clone, PartialEq)]
pub struct ModelEvaluation {
    objective_values: Vec<ClassicalFloat>,
    constraint_values: Vec<ClassicalFloat>,
}

impl ModelEvaluation {
    /// Creates an evaluation result.
    pub fn new(
        objective_values: Vec<ClassicalFloat>,
        constraint_values: Vec<ClassicalFloat>,
    ) -> OptimizationResult<Self> {
        for value in &objective_values {
            validate_number(*value, "objective evaluation")?;
        }

        for value in &constraint_values {
            validate_number(*value, "constraint evaluation")?;
        }

        Ok(Self {
            objective_values,
            constraint_values,
        })
    }

    /// Returns objective values in problem objective order.
    #[must_use]
    pub fn objective_values(&self) -> &[ClassicalFloat] {
        &self.objective_values
    }

    /// Returns constraint expression values in problem constraint order.
    #[must_use]
    pub fn constraint_values(&self) -> &[ClassicalFloat] {
        &self.constraint_values
    }

    /// Validates the dimensions against a problem.
    pub fn validate_for(
        &self,
        problem: &OptimizationProblem,
    ) -> OptimizationResult<()> {
        validate_dimension(
            "objective evaluations",
            problem.objective_count(),
            self.objective_values.len(),
        )?;

        validate_dimension(
            "constraint evaluations",
            problem.constraint_count(),
            self.constraint_values.len(),
        )?;

        for value in &self.objective_values {
            validate_number(*value, "objective evaluation")?;
        }

        for value in &self.constraint_values {
            validate_number(*value, "constraint evaluation")?;
        }

        Ok(())
    }

    /// Returns whether all problem constraints are satisfied.
    #[must_use]
    pub fn is_feasible(
        &self,
        problem: &OptimizationProblem,
        tolerance: f64,
    ) -> bool {
        if self.validate_for(problem).is_err()
            || !tolerance.is_finite()
            || tolerance < 0.0
        {
            return false;
        }

        problem
            .constraints()
            .iter()
            .zip(&self.constraint_values)
            .all(|(constraint, value)| {
                constraint.is_satisfied(*value, tolerance)
            })
    }
}

/// Numerical optimization model.
///
/// Applications implement this trait to connect their mathematical
/// problem to the standard-library solver contracts.
///
/// The model owns its objective and constraint evaluation semantics.
/// This module does not construct an expression tree or IR for it.
pub trait OptimizationModel {
    /// Returns the immutable problem definition.
    fn problem(&self) -> &OptimizationProblem;

    /// Evaluates all objectives and constraint expressions at a candidate.
    fn evaluate(
        &self,
        variables: &[ClassicalFloat],
    ) -> OptimizationResult<ModelEvaluation>;

    /// Optionally evaluates the gradient of one objective.
    ///
    /// The returned vector must have one component per decision variable.
    /// Returning `Ok(None)` means that an analytical gradient is unavailable.
    ///
    /// A solver may then use another permitted gradient strategy.
    fn gradient(
        &self,
        _objective_index: usize,
        _variables: &[ClassicalFloat],
    ) -> OptimizationResult<Option<Vec<ClassicalFloat>>> {
        Ok(None)
    }
}

/// Configurable operational budgets for one solver invocation.
///
/// Every budget is optional. `None` means that this API imposes no
/// budget for that dimension. The execution environment may still
/// impose its own resource and time restrictions.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SolverOptions {
    /// Maximum solver iterations.
    pub max_iterations: Option<NonZeroU64>,

    /// Maximum objective/model evaluations.
    pub max_evaluations: Option<NonZeroU64>,

    /// Maximum gradient evaluations.
    pub max_gradient_evaluations: Option<NonZeroU64>,

    /// Optional elapsed-time budget.
    pub max_duration: Option<Duration>,

    /// Optional absolute feasibility/convergence tolerance.
    pub tolerance: Option<ClassicalFloat>,
}

impl SolverOptions {
    /// Validates solver options.
    pub fn validate(&self) -> OptimizationResult<()> {
        if let Some(tolerance) = self.tolerance {
            validate_number(tolerance, "solver tolerance")?;

            if tolerance.get() < 0.0 {
                return Err(OptimizationError::InvalidOption {
                    name: "tolerance",
                    reason: "tolerance must not be negative".to_owned(),
                });
            }
        }

        if self.max_duration.is_some_and(Duration::is_zero) {
            return Err(OptimizationError::InvalidOption {
                name: "max_duration",
                reason: "duration budget must be greater than zero".to_owned(),
            });
        }

        Ok(())
    }

    /// Returns the configured tolerance, or exact comparison tolerance.
    #[must_use]
    pub fn tolerance_value(&self) -> f64 {
        self.tolerance.map_or(0.0, ClassicalFloat::get)
    }
}

/// Cooperative cancellation contract.
///
/// Runtime and application cancellation tokens can implement this trait.
/// The standard library does not create a competing task runtime.
pub trait Cancellation {
    /// Returns whether the current solver invocation should stop.
    fn is_cancelled(&self) -> bool;
}

/// Cancellation implementation that never requests cancellation.
#[derive(Debug, Clone, Copy, Default)]
pub struct NeverCancel;

impl Cancellation for NeverCancel {
    fn is_cancelled(&self) -> bool {
        false
    }
}

/// Kind of operational budget that was exhausted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BudgetKind {
    /// Solver iteration budget.
    Iterations,

    /// Objective evaluation budget.
    Evaluations,

    /// Gradient evaluation budget.
    GradientEvaluations,

    /// Elapsed-time budget.
    Duration,
}

/// Accounting for one solver invocation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct OptimizationStatistics {
    /// Number of iterations recorded by the solver.
    pub iterations: u64,

    /// Number of objective/model evaluations.
    pub evaluations: u64,

    /// Number of gradient evaluations.
    pub gradient_evaluations: u64,
}

/// Controlled evaluation context supplied to a solver.
///
/// Solvers should use this context instead of calling model evaluation
/// directly. It validates candidates, validates model outputs, enforces
/// configured budgets, checks cancellation, and accounts for evaluations.
pub struct SolverContext<'a> {
    model: &'a dyn OptimizationModel,
    options: &'a SolverOptions,
    cancellation: &'a dyn Cancellation,
    started_at: Instant,
    statistics: OptimizationStatistics,
}

impl<'a> SolverContext<'a> {
    /// Creates a solver context after validating the problem and options.
    pub fn new(
        model: &'a dyn OptimizationModel,
        options: &'a SolverOptions,
        cancellation: &'a dyn Cancellation,
    ) -> OptimizationResult<Self> {
        options.validate()?;

        let problem = model.problem();

        if problem.variable_count() == 0
            || problem.objective_count() == 0
        {
            return Err(OptimizationError::InvalidProblem);
        }

        Ok(Self {
            model,
            options,
            cancellation,
            started_at: Instant::now(),
            statistics: OptimizationStatistics::default(),
        })
    }

    /// Returns the optimization problem.
    #[must_use]
    pub fn problem(&self) -> &OptimizationProblem {
        self.model.problem()
    }

    /// Returns the solver options.
    #[must_use]
    pub const fn options(&self) -> &SolverOptions {
        self.options
    }

    /// Returns current invocation statistics.
    #[must_use]
    pub const fn statistics(&self) -> OptimizationStatistics {
        self.statistics
    }

    /// Checks cancellation and elapsed-time budget.
    pub fn check_continue(&self) -> OptimizationResult<()> {
        if self.cancellation.is_cancelled() {
            return Err(OptimizationError::Cancelled);
        }

        if let Some(limit) = self.options.max_duration {
            if self.started_at.elapsed() >= limit {
                return Err(OptimizationError::BudgetExceeded {
                    kind: BudgetKind::Duration,
                });
            }
        }

        Ok(())
    }

    /// Records one solver iteration.
    pub fn record_iteration(&mut self) -> OptimizationResult<()> {
        self.check_continue()?;

        if self
            .options
            .max_iterations
            .is_some_and(|limit| self.statistics.iterations >= limit.get())
        {
            return Err(OptimizationError::BudgetExceeded {
                kind: BudgetKind::Iterations,
            });
        }

        self.statistics.iterations = self
            .statistics
            .iterations
            .checked_add(1)
            .ok_or(OptimizationError::CounterOverflow)?;

        Ok(())
    }

    /// Evaluates the model under the configured evaluation budget.
    pub fn evaluate(
        &mut self,
        variables: &[ClassicalFloat],
    ) -> OptimizationResult<ModelEvaluation> {
        self.check_continue()?;

        self.problem().validate_candidate(variables)?;

        if self
            .options
            .max_evaluations
            .is_some_and(|limit| self.statistics.evaluations >= limit.get())
        {
            return Err(OptimizationError::BudgetExceeded {
                kind: BudgetKind::Evaluations,
            });
        }

        self.statistics.evaluations = self
            .statistics
            .evaluations
            .checked_add(1)
            .ok_or(OptimizationError::CounterOverflow)?;

        let evaluation = self.model.evaluate(variables)?;

        evaluation.validate_for(self.problem())?;

        Ok(evaluation)
    }

    /// Evaluates an analytical gradient when the model provides one.
    ///
    /// `Ok(None)` means no analytical gradient is available.
    pub fn evaluate_gradient(
        &mut self,
        objective_index: usize,
        variables: &[ClassicalFloat],
    ) -> OptimizationResult<Option<Vec<ClassicalFloat>>> {
        self.check_continue()?;

        self.problem().validate_candidate(variables)?;

        if objective_index >= self.problem().objective_count() {
            return Err(OptimizationError::IndexOutOfRange {
                component: "objective",
                index: objective_index,
            });
        }

        if self.options.max_gradient_evaluations.is_some_and(
            |limit| {
                self.statistics.gradient_evaluations >= limit.get()
            },
        ) {
            return Err(OptimizationError::BudgetExceeded {
                kind: BudgetKind::GradientEvaluations,
            });
        }

        let gradient = self
            .model
            .gradient(objective_index, variables)?;

        if let Some(values) = &gradient {
            validate_dimension(
                "gradient",
                self.problem().variable_count(),
                values.len(),
            )?;

            for value in values {
                validate_number(*value, "gradient component")?;
            }

            self.statistics.gradient_evaluations = self
                .statistics
                .gradient_evaluations
                .checked_add(1)
                .ok_or(OptimizationError::CounterOverflow)?;
        }

        Ok(gradient)
    }
}

/// Terminal state of an optimization invocation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TerminationStatus {
    /// The solver's convergence criteria were satisfied.
    Converged,

    /// A feasible solution was found, but convergence was not established.
    Feasible,

    /// The solver established that no feasible solution was found.
    Infeasible,

    /// The solver stopped because its iteration budget was exhausted.
    IterationBudget,

    /// The solver stopped because its evaluation budget was exhausted.
    EvaluationBudget,

    /// The solver stopped because its gradient evaluation budget was exhausted.
    GradientBudget,

    /// The solver stopped because its time budget was exhausted.
    TimeBudget,

    /// The invocation was cancelled.
    Cancelled,

    /// The solver could not establish a valid solution.
    NoSolution,
}

/// Result of a mathematical optimization invocation.
#[derive(Debug, Clone, PartialEq)]
pub struct OptimizationOutcome {
    /// Solver identity.
    pub solver_id: String,

    /// Final decision-variable values.
    pub variables: Vec<ClassicalFloat>,

    /// Final objective and constraint evaluation.
    pub evaluation: ModelEvaluation,

    /// Whether the final point satisfies the problem constraints.
    pub feasible: bool,

    /// Terminal solver state.
    pub status: TerminationStatus,

    /// Evaluation and iteration accounting.
    pub statistics: OptimizationStatistics,
}

impl OptimizationOutcome {
    /// Validates the result against its original problem.
    pub fn validate(
        &self,
        problem: &OptimizationProblem,
    ) -> OptimizationResult<()> {
        validate_identifier(&self.solver_id, "solver")?;

        problem.validate_candidate(&self.variables)?;
        self.evaluation.validate_for(problem)?;

        let actual_feasibility = self.evaluation.is_feasible(
            problem,
            0.0,
        );

        if self.feasible && !actual_feasibility {
            return Err(OptimizationError::InvalidOutcome {
                reason: "result claims feasibility but violates constraints"
                    .to_owned(),
            });
        }

        Ok(())
    }
}

/// Extensible mathematical optimization solver.
///
/// Implementations may provide gradient-based, derivative-free,
/// stochastic, exact, approximate, Bayesian, evolutionary, or other
/// solver strategies.
///
/// A solver must preserve the problem's declared objective and
/// constraint semantics.
///
/// Hardware selection and execution placement are not solver duties.
pub trait OptimizationSolver {
    /// Returns a stable, qualified solver identifier.
    fn solver_id(&self) -> &str;

    /// Executes the solver using the supplied controlled context.
    fn solve(
        &self,
        context: &mut SolverContext<'_>,
    ) -> OptimizationResult<OptimizationOutcome>;
}

/// Errors returned by scientific optimization contracts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OptimizationError {
    /// An identifier was empty or contained control characters.
    InvalidIdentifier {
        /// Identifier category.
        kind: &'static str,
    },

    /// An optimization problem omitted a required component.
    EmptyProblem {
        /// Missing component.
        component: &'static str,
    },

    /// A problem contains duplicate identifiers.
    DuplicateIdentifier {
        /// Identifier category.
        kind: &'static str,
        /// Duplicated identifier.
        id: String,
    },

    /// A variable's lower bound exceeds its upper bound.
    InvalidBounds {
        /// Variable identifier.
        variable: String,
    },

    /// A numeric value was invalid or non-finite.
    InvalidNumber {
        /// Value description.
        component: &'static str,
    },

    /// A candidate variable value violates its declared domain or bounds.
    InvalidVariableValue {
        /// Variable identifier.
        variable: String,
        /// Zero-based variable index.
        index: usize,
    },

    /// A vector dimension did not match the problem definition.
    DimensionMismatch {
        /// Component being checked.
        component: &'static str,
        /// Required length.
        expected: usize,
        /// Supplied length.
        actual: usize,
    },

    /// A requested objective index does not exist.
    IndexOutOfRange {
        /// Indexed component.
        component: &'static str,
        /// Requested index.
        index: usize,
    },

    /// A solver option is invalid.
    InvalidOption {
        /// Option name.
        name: &'static str,
        /// Reason for rejection.
        reason: String,
    },

    /// The model or problem is invalid.
    InvalidProblem,

    /// The model could not evaluate a candidate.
    ModelEvaluation {
        /// Evaluation failure description.
        reason: String,
    },

    /// A solver operational budget was exhausted.
    BudgetExceeded {
        /// Exhausted budget.
        kind: BudgetKind,
    },

    /// Cancellation was requested.
    Cancelled,

    /// An accounting counter overflowed its representation.
    CounterOverflow,

    /// A solver returned an invalid result.
    InvalidOutcome {
        /// Reason for rejection.
        reason: String,
    },
}

impl fmt::Display for OptimizationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidIdentifier { kind } => {
                write!(formatter, "invalid {kind} identifier")
            }

            Self::EmptyProblem { component } => {
                write!(formatter, "optimization problem has no {component}")
            }

            Self::DuplicateIdentifier { kind, id } => {
                write!(formatter, "duplicate {kind} identifier: {id}")
            }

            Self::InvalidBounds { variable } => {
                write!(formatter, "invalid bounds for variable {variable}")
            }

            Self::InvalidNumber { component } => {
                write!(formatter, "invalid or non-finite {component}")
            }

            Self::InvalidVariableValue { variable, index } => {
                write!(
                    formatter,
                    "candidate value at index {index} violates variable {variable}"
                )
            }

            Self::DimensionMismatch {
                component,
                expected,
                actual,
            } => {
                write!(
                    formatter,
                    "{component} dimension mismatch: expected {expected}, got {actual}"
                )
            }

            Self::IndexOutOfRange { component, index } => {
                write!(formatter, "{component} index {index} is out of range")
            }

            Self::InvalidOption { name, reason } => {
                write!(formatter, "invalid solver option {name}: {reason}")
            }

            Self::InvalidProblem => {
                formatter.write_str("invalid optimization problem")
            }

            Self::ModelEvaluation { reason } => {
                write!(formatter, "optimization model evaluation failed: {reason}")
            }

            Self::BudgetExceeded { kind } => {
                write!(formatter, "optimization budget exceeded: {kind:?}")
            }

            Self::Cancelled => {
                formatter.write_str("optimization was cancelled")
            }

            Self::CounterOverflow => {
                formatter.write_str("optimization accounting counter overflowed")
            }

            Self::InvalidOutcome { reason } => {
                write!(formatter, "invalid optimization outcome: {reason}")
            }
        }
    }
}

impl std::error::Error for OptimizationError {}

fn validate_identifier(
    value: &str,
    kind: &'static str,
) -> OptimizationResult<()> {
    if value.trim().is_empty()
        || value.chars().any(char::is_control)
    {
        return Err(OptimizationError::InvalidIdentifier { kind });
    }

    Ok(())
}

fn validate_unique<'a>(
    values: impl Iterator<Item = &'a str>,
    kind: &'static str,
) -> OptimizationResult<()> {
    let mut seen = BTreeSet::new();

    for value in values {
        if !seen.insert(value) {
            return Err(OptimizationError::DuplicateIdentifier {
                kind,
                id: value.to_owned(),
            });
        }
    }

    Ok(())
}

fn validate_number(
    value: ClassicalFloat,
    component: &'static str,
) -> OptimizationResult<()> {
    if !value.get().is_finite() {
        return Err(OptimizationError::InvalidNumber { component });
    }

    Ok(())
}

fn validate_optional_number(
    value: Option<ClassicalFloat>,
    component: &'static str,
) -> OptimizationResult<()> {
    if let Some(value) = value {
        validate_number(value, component)?;
    }

    Ok(())
}

fn validate_dimension(
    component: &'static str,
    expected: usize,
    actual: usize,
) -> OptimizationResult<()> {
    if expected != actual {
        return Err(OptimizationError::DimensionMismatch {
            component,
            expected,
            actual,
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn number(value: f64) -> ClassicalFloat {
        ClassicalFloat::new(value)
            .expect("test value must be finite")
    }

    fn sample_problem() -> OptimizationProblem {
        OptimizationProblem::new(
            "test.problem",
            vec![
                DecisionVariable::new(
                    "x",
                    VariableDomain::Continuous,
                    Some(number(-10.0)),
                    Some(number(10.0)),
                )
                .expect("valid variable"),
            ],
            vec![
                OptimizationObjective::new(
                    "loss",
                    ObjectiveDirection::Minimize,
                )
                .expect("valid objective"),
            ],
            vec![
                OptimizationConstraint::new(
                    "positive",
                    ConstraintRelation::GreaterEqual,
                    number(0.0),
                )
                .expect("valid constraint"),
            ],
        )
        .expect("valid problem")
    }

    struct TestModel {
        problem: OptimizationProblem,
    }

    impl OptimizationModel for TestModel {
        fn problem(&self) -> &OptimizationProblem {
            &self.problem
        }

        fn evaluate(
            &self,
            variables: &[ClassicalFloat],
        ) -> OptimizationResult<ModelEvaluation> {
            let x = variables[0].get();

            ModelEvaluation::new(
                vec![number(x * x)],
                vec![number(x)],
            )
        }

        fn gradient(
            &self,
            objective_index: usize,
            variables: &[ClassicalFloat],
        ) -> OptimizationResult<Option<Vec<ClassicalFloat>>> {
            if objective_index != 0 {
                return Err(OptimizationError::IndexOutOfRange {
                    component: "objective",
                    index: objective_index,
                });
            }

            Ok(Some(vec![number(2.0 * variables[0].get())]))
        }
    }

    #[test]
    fn rejects_duplicate_variable_ids() {
        let result = OptimizationProblem::new(
            "duplicate.problem",
            vec![
                DecisionVariable::new(
                    "x",
                    VariableDomain::Continuous,
                    None,
                    None,
                )
                .expect("valid variable"),
                DecisionVariable::new(
                    "x",
                    VariableDomain::Continuous,
                    None,
                    None,
                )
                .expect("valid variable"),
            ],
            vec![
                OptimizationObjective::new(
                    "objective",
                    ObjectiveDirection::Minimize,
                )
                .expect("valid objective"),
            ],
            Vec::new(),
        );

        assert!(matches!(
            result,
            Err(OptimizationError::DuplicateIdentifier { .. })
        ));
    }

    #[test]
    fn rejects_reversed_variable_bounds() {
        let result = DecisionVariable::new(
            "x",
            VariableDomain::Continuous,
            Some(number(5.0)),
            Some(number(1.0)),
        );

        assert!(matches!(
            result,
            Err(OptimizationError::InvalidBounds { .. })
        ));
    }

    #[test]
    fn validates_candidate_domain_and_bounds() {
        let problem = sample_problem();

        assert!(problem.validate_candidate(&[number(2.0)]).is_ok());
        assert!(problem.validate_candidate(&[number(20.0)]).is_err());
    }

    #[test]
    fn validates_model_evaluation_dimensions() {
        let problem = sample_problem();

        let evaluation = ModelEvaluation::new(
            vec![number(1.0)],
            Vec::new(),
        )
        .expect("finite evaluation");

        assert!(evaluation.validate_for(&problem).is_err());
    }

    #[test]
    fn evaluates_through_controlled_context() {
        let model = TestModel {
            problem: sample_problem(),
        };

        let options = SolverOptions::default();
        let cancellation = NeverCancel;

        let mut context = SolverContext::new(
            &model,
            &options,
            &cancellation,
        )
        .expect("valid context");

        let evaluation = context
            .evaluate(&[number(2.0)])
            .expect("valid evaluation");

        assert_eq!(evaluation.objective_values()[0].get(), 4.0);
        assert_eq!(context.statistics().evaluations, 1);
    }

    #[test]
    fn evaluates_optional_analytical_gradient() {
        let model = TestModel {
            problem: sample_problem(),
        };

        let options = SolverOptions::default();
        let cancellation = NeverCancel;

        let mut context = SolverContext::new(
            &model,
            &options,
            &cancellation,
        )
        .expect("valid context");

        let gradient = context
            .evaluate_gradient(0, &[number(3.0)])
            .expect("gradient evaluation")
            .expect("analytical gradient");

        assert_eq!(gradient[0].get(), 6.0);
        assert_eq!(context.statistics().gradient_evaluations, 1);
    }

    #[test]
    fn enforces_configured_evaluation_budget() {
        let model = TestModel {
            problem: sample_problem(),
        };

        let options = SolverOptions {
            max_evaluations: NonZeroU64::new(1),
            ..SolverOptions::default()
        };

        let cancellation = NeverCancel;

        let mut context = SolverContext::new(
            &model,
            &options,
            &cancellation,
        )
        .expect("valid context");

        context
            .evaluate(&[number(1.0)])
            .expect("first evaluation");

        assert!(matches!(
            context.evaluate(&[number(2.0)]),
            Err(OptimizationError::BudgetExceeded {
                kind: BudgetKind::Evaluations
            })
        ));
    }

    #[test]
    fn checks_constraint_feasibility() {
        let problem = sample_problem();

        let feasible = ModelEvaluation::new(
            vec![number(4.0)],
            vec![number(2.0)],
        )
        .expect("valid evaluation");

        let infeasible = ModelEvaluation::new(
            vec![number(4.0)],
            vec![number(-2.0)],
        )
        .expect("valid evaluation");

        assert!(feasible.is_feasible(&problem, 0.0));
        assert!(!infeasible.is_feasible(&problem, 0.0));
    }
}
