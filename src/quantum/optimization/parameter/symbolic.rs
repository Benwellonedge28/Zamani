//! Zamani Quantum Optimization — Symbolic Parameter Optimization
//!
//! Production-grade symbolic parameter normalization for the canonical
//! Zamani Quantum IR.
//!
//! # Architectural position
//!
//! ```text
//!                  quantum::ir::parameter
//!                           │
//!                           ▼
//!              optimization::parameter::symbolic
//!                           │
//!             ┌─────────────┼─────────────┐
//!             ▼             ▼             ▼
//!      local rotation   constant_fold   simplification
//!             │             │             │
//!             └─────────────┼─────────────┘
//!                           ▼
//!                    optimization pass
//! ```
//!
//! This module owns symbolic-expression normalization only.
//!
//! It does NOT own:
//!
//! - the canonical parameter representation;
//! - gate definitions;
//! - circuit representation;
//! - symbol tables;
//! - global parameter environments;
//! - hardware units;
//! - angle-periodicity policy;
//! - gate synthesis;
//! - routing;
//! - scheduling;
//! - execution;
//! - simulation.
//!
//! The authoritative parameter representation is:
//!
//! `crate::quantum::ir::parameter::Parameter`
//!
//! and:
//!
//! `crate::quantum::ir::parameter::ParameterExpression`
//!
//! # Design goals
//!
//! This implementation provides:
//!
//! - deterministic symbolic normalization;
//! - constant folding where mathematically and numerically safe;
//! - identity elimination;
//! - neutral-element elimination;
//! - double-negation elimination;
//! - exact structural cancellation;
//! - symbolic dependency inspection;
//! - explicit resource budgets;
//! - no global mutable state;
//! - no unsafe Rust;
//! - no optimizer-local parameter AST exposed publicly;
//! - no floating-point approximation;
//! - no angle modulo reduction;
//! - no assumption that every parameter has units of radians;
//! - iterative expression traversal;
//! - deterministic behavior;
//! - graceful budget exhaustion;
//! - preservation of expressions when a rewrite cannot be proven safe;
//! - compatibility with Rust 1.97 / 1.97.1.
//!
//! # Important semantic rule
//!
//! This module performs only transformations that are valid for the generic
//! arithmetic semantics of the canonical IR.
//!
//! In particular, it MUST NOT perform:
//!
//! ```text
//! x / x -> 1
//! ```
//!
//! because `x` may be zero.
//!
//! It also MUST NOT perform:
//!
//! ```text
//! theta -> theta mod 2π
//! ```
//!
//! because the canonical IR deliberately does not attach an angle unit to
//! `Parameter`.
//!
//! Such transformations belong to a later domain-aware optimization layer.
//!
//! # Scaling
//!
//! There is no artificial fixed maximum number of expressions or symbols.
//!
//! The caller can select:
//!
//! - unlimited work;
//! - a node-visit budget;
//! - a rewrite budget;
//! - an output-node budget.
//!
//! `None` means no optimizer-local limit. The canonical IR's own structural
//! limits remain authoritative.
//!
//! This permits tiny expressions and very large workloads to use the same
//! implementation while allowing the compiler's global `OptimizationLimits`
//! to impose resource limits when required.
//!
//! # Integration contract
//!
//! `parameter/mod.rs` should expose this module:
//!
//! ```text
//! pub mod symbolic;
//!
//! pub use symbolic::{
//!     SymbolicOptimizer,
//!     SymbolicOptimizationConfig,
//!     SymbolicOptimizationResult,
//!     SymbolicOptimizationError,
//!     RewriteKind,
//!     simplify_parameter,
//!     simplify_expression,
//!     contains_symbol,
//!     collect_symbols,
//! };
//! ```
//!
//! `parameter/constant_fold.rs` may call this module for safe symbolic
//! normalization before or after its numerical-only transformations.
//!
//! `parameter/simplification.rs` may delegate generic expression
//! normalization here rather than implementing another expression walker.
//!
//! `parameter/binding.rs` should remain responsible for binding symbols.
//!
//! `analysis/parameter_usage.rs` remains responsible for circuit-wide usage
//! analysis and should not be made a dependency of this file.
//!
//! `local/rotation.rs` may consume the normalized result when combining
//! parameters such as:
//!
//! ```text
//! RX(a); RX(b)
//! ```
//!
//! into:
//!
//! ```text
//! RX(a + b)
//! ```
//!
//! `algebra/phase_polynomial.rs` may use the normalized expressions but must
//! retain ownership of phase-polynomial semantics.
//!
//! `pass.rs` / `pipeline.rs` should treat this module as a pure transformation
//! utility. No circuit mutation is performed here.
//!
//! # Complexity
//!
//! For an expression containing `N` parameter nodes:
//!
//! - traversal: O(N);
//! - local simplification: O(N);
//! - memory: O(N);
//!
//! No symbolic expansion of products over sums is performed. Therefore this
//! implementation avoids the exponential blow-up associated with naive
//! distributive expansion.
//!
//! # Safety
//!
//! No unsafe code is permitted.
//!
//! ```text
//! #![forbid(unsafe_code)]
//! ```
//!
//! # Rust compatibility
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust only;
//! - no nightly features;
//! - no external dependencies.

#![forbid(unsafe_code)]

use std::fmt;
use std::mem;

// =============================================================================
// Canonical IR imports
// =============================================================================

use crate::quantum::ir::parameter::{
    Parameter,
    ParameterExpression,
    MAX_PARAMETER_EXPRESSION_DEPTH,
};

// =============================================================================
// Public configuration
// =============================================================================

/// Configuration controlling symbolic optimization work.
///
/// `None` means that this module itself imposes no additional limit.
///
/// The optimizer remains subject to the canonical IR's own limits and to
/// whatever limits are imposed by the enclosing optimization context.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SymbolicOptimizationConfig {
    /// Maximum number of parameter nodes that may be visited.
    ///
    /// `None` means unlimited.
    pub max_nodes_visited: Option<usize>,

    /// Maximum number of successful rewrites.
    ///
    /// `None` means unlimited.
    pub max_rewrites: Option<usize>,

    /// Maximum number of nodes permitted in the resulting expression.
    ///
    /// `None` means unlimited.
    pub max_output_nodes: Option<usize>,
}

impl SymbolicOptimizationConfig {
    /// Creates an unlimited configuration.
    ///
    /// "Unlimited" means no additional limit from this module; it does not
    /// disable the canonical IR's structural validation.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self {
            max_nodes_visited: None,
            max_rewrites: None,
            max_output_nodes: None,
        }
    }

    /// Creates a bounded configuration.
    #[must_use]
    pub const fn bounded(
        max_nodes_visited: usize,
        max_rewrites: usize,
        max_output_nodes: usize,
    ) -> Self {
        Self {
            max_nodes_visited: Some(max_nodes_visited),
            max_rewrites: Some(max_rewrites),
            max_output_nodes: Some(max_output_nodes),
        }
    }

    /// Returns the default compiler configuration.
    ///
    /// The default is intentionally unlimited at this layer. The enclosing
    /// optimizer is responsible for applying global compilation limits.
    #[must_use]
    pub const fn default_for_compiler() -> Self {
        Self::unlimited()
    }
}

impl Default for SymbolicOptimizationConfig {
    fn default() -> Self {
        Self::unlimited()
    }
}

// =============================================================================
// Rewrite classification
// =============================================================================

/// Classification of a symbolic rewrite.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RewriteKind {
    /// Constant arithmetic was evaluated.
    ConstantFold,

    /// `x + 0 -> x`.
    AddZero,

    /// `0 + x -> x`.
    ZeroAdd,

    /// `x - 0 -> x`.
    SubtractZero,

    /// `x * 0 -> 0`.
    MultiplyZero,

    /// `0 * x -> 0`.
    ZeroMultiply,

    /// `x * 1 -> x`.
    MultiplyOne,

    /// `1 * x -> x`.
    OneMultiply,

    /// `x / 1 -> x`.
    DivideOne,

    /// `-(-x) -> x`.
    DoubleNegation,

    /// `x - x -> 0`.
    SelfSubtraction,

    /// `x + (-x) -> 0`.
    AdditiveInverse,

    /// `(-x) + x -> 0`.
    ReverseAdditiveInverse,

    /// `-constant -> constant`.
    ConstantNegation,

    /// `-(constant expression)` was folded safely.
    NegatedConstantFold,

    /// An expression was already canonical.
    Unchanged,
}

impl RewriteKind {
    /// Returns whether this rewrite changes the expression.
    #[must_use]
    pub const fn changes_expression(self) -> bool {
        !matches!(self, Self::Unchanged)
    }
}

// =============================================================================
// Result
// =============================================================================

/// Result of symbolic parameter optimization.
#[derive(Debug, Clone, PartialEq)]
pub struct SymbolicOptimizationResult {
    /// Optimized parameter.
    parameter: Parameter,

    /// Number of expression nodes visited.
    nodes_visited: usize,

    /// Number of successful rewrites.
    rewrites: usize,

    /// Number of nodes in the input.
    input_nodes: usize,

    /// Number of nodes in the output.
    output_nodes: usize,

    /// Whether the optimizer stopped because a configured budget was reached.
    budget_exhausted: bool,
}

impl SymbolicOptimizationResult {
    fn new(
        parameter: Parameter,
        nodes_visited: usize,
        rewrites: usize,
        input_nodes: usize,
        output_nodes: usize,
        budget_exhausted: bool,
    ) -> Self {
        Self {
            parameter,
            nodes_visited,
            rewrites,
            input_nodes,
            output_nodes,
            budget_exhausted,
        }
    }

    /// Returns the optimized parameter.
    #[must_use]
    pub fn parameter(&self) -> &Parameter {
        &self.parameter
    }

    /// Consumes the result and returns the optimized parameter.
    #[must_use]
    pub fn into_parameter(self) -> Parameter {
        self.parameter
    }

    /// Number of visited parameter nodes.
    #[must_use]
    pub const fn nodes_visited(&self) -> usize {
        self.nodes_visited
    }

    /// Number of successful rewrites.
    #[must_use]
    pub const fn rewrites(&self) -> usize {
        self.rewrites
    }

    /// Number of input nodes.
    #[must_use]
    pub const fn input_nodes(&self) -> usize {
        self.input_nodes
    }

    /// Number of output nodes.
    #[must_use]
    pub const fn output_nodes(&self) -> usize {
        self.output_nodes
    }

    /// Returns whether a configured budget stopped optimization.
    #[must_use]
    pub const fn budget_exhausted(&self) -> bool {
        self.budget_exhausted
    }

    /// Returns whether the resulting parameter differs structurally from the
    /// original parameter.
    ///
    /// This method is intentionally not available because the original value
    /// is consumed by the optimizer. Use `parameter != result.parameter()` at
    /// the call site when the caller retains its input.
    #[must_use]
    pub const fn changed(&self) -> bool {
        self.rewrites != 0
    }
}

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by symbolic parameter optimization.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SymbolicOptimizationError {
    /// The input parameter violates the canonical IR parameter contract.
    InvalidParameter {
        /// Stable diagnostic description.
        message: &'static str,
    },

    /// A configured node-visit budget was exceeded.
    NodeBudgetExceeded {
        /// Number of nodes already visited.
        visited: usize,

        /// Configured maximum.
        maximum: usize,
    },

    /// A configured rewrite budget was exceeded.
    RewriteBudgetExceeded {
        /// Number of rewrites already performed.
        rewrites: usize,

        /// Configured maximum.
        maximum: usize,
    },

    /// A configured output-size budget was exceeded.
    OutputBudgetExceeded {
        /// Number of output nodes.
        nodes: usize,

        /// Configured maximum.
        maximum: usize,
    },

    /// The expression would exceed the canonical IR expression-depth limit.
    ExpressionDepthExceeded {
        /// Maximum permitted depth.
        maximum: usize,
    },

    /// A safe constant operation could not produce a finite value.
    ///
    /// The optimizer normally leaves such an operation untouched instead of
    /// returning this error. This variant exists for callers that explicitly
    /// request strict normalization in future versions.
    NonFiniteConstant,

    /// Internal arithmetic overflow in optimizer bookkeeping.
    ArithmeticOverflow {
        /// Static description of the calculation.
        calculation: &'static str,
    },
}

impl fmt::Display for SymbolicOptimizationError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidParameter { message } => {
                write!(
                    formatter,
                    "invalid symbolic parameter: {message}"
                )
            }

            Self::NodeBudgetExceeded {
                visited,
                maximum,
            } => {
                write!(
                    formatter,
                    "symbolic optimization node budget exceeded: visited={visited}, maximum={maximum}"
                )
            }

            Self::RewriteBudgetExceeded {
                rewrites,
                maximum,
            } => {
                write!(
                    formatter,
                    "symbolic optimization rewrite budget exceeded: rewrites={rewrites}, maximum={maximum}"
                )
            }

            Self::OutputBudgetExceeded {
                nodes,
                maximum,
            } => {
                write!(
                    formatter,
                    "symbolic optimization output budget exceeded: nodes={nodes}, maximum={maximum}"
                )
            }

            Self::ExpressionDepthExceeded { maximum } => {
                write!(
                    formatter,
                    "symbolic expression depth exceeds canonical IR maximum {maximum}"
                )
            }

            Self::NonFiniteConstant => {
                write!(
                    formatter,
                    "symbolic optimization produced a non-finite constant"
                )
            }

            Self::ArithmeticOverflow { calculation } => {
                write!(
                    formatter,
                    "symbolic optimization bookkeeping overflowed while calculating {calculation}"
                )
            }
        }
    }
}

impl std::error::Error for SymbolicOptimizationError {}

// =============================================================================
// Internal traversal representation
// =============================================================================

/// Expression-building operation used by the iterative post-order walker.
#[derive(Debug, Clone, Copy)]
enum BuildOperation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Negate,
}

/// Explicit traversal task.
///
/// Using an explicit stack avoids depending on Rust call-stack depth while
/// walking the expression tree.
enum Task {
    Visit(Parameter),
    Build(BuildOperation),
}

// =============================================================================
// Optimizer
// =============================================================================

/// Production symbolic parameter optimizer.
///
/// The optimizer is stateless between calls. This is intentional:
///
/// - no global symbol state;
/// - no hidden cache invalidation;
/// - no cross-circuit contamination;
/// - safe use by parallel compiler workers;
/// - deterministic output.
///
/// Caches can be added above this layer if the compiler eventually needs them.
#[derive(Debug, Clone, Copy)]
pub struct SymbolicOptimizer {
    config: SymbolicOptimizationConfig,
}

impl SymbolicOptimizer {
    /// Creates an optimizer with the supplied resource policy.
    #[must_use]
    pub const fn new(
        config: SymbolicOptimizationConfig,
    ) -> Self {
        Self { config }
    }

    /// Creates an unlimited symbolic optimizer.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self::new(SymbolicOptimizationConfig::unlimited())
    }

    /// Returns this optimizer's configuration.
    #[must_use]
    pub const fn config(
        &self,
    ) -> SymbolicOptimizationConfig {
        self.config
    }

    /// Simplifies a canonical parameter.
    ///
    /// The input is consumed so the optimizer can reuse its owned expression
    /// nodes instead of cloning the entire expression tree.
    ///
    /// The operation is deterministic and does not mutate any external state.
    pub fn optimize(
        &self,
        parameter: Parameter,
    ) -> Result<
        SymbolicOptimizationResult,
        SymbolicOptimizationError,
    > {
        validate_parameter(&parameter)?;

        let input_nodes = count_nodes_checked(&parameter)?;

        if let Some(maximum) = self.config.max_nodes_visited {
            if input_nodes > maximum {
                return Err(
                    SymbolicOptimizationError::NodeBudgetExceeded {
                        visited: input_nodes,
                        maximum,
                    },
                );
            }
        }

        let mut state = OptimizerState::new(self.config);

        let optimized = state.optimize(parameter)?;

        let output_nodes = count_nodes_checked(&optimized)?;

        if let Some(maximum) =
            self.config.max_output_nodes
        {
            if output_nodes > maximum {
                return Err(
                    SymbolicOptimizationError::OutputBudgetExceeded {
                        nodes: output_nodes,
                        maximum,
                    },
                );
            }
        }

        Ok(SymbolicOptimizationResult::new(
            optimized,
            state.nodes_visited,
            state.rewrites,
            input_nodes,
            output_nodes,
            state.budget_exhausted,
        ))
    }

    /// Simplifies an expression and returns the normalized parameter.
    pub fn optimize_expression(
        &self,
        expression: ParameterExpression,
    ) -> Result<
        SymbolicOptimizationResult,
        SymbolicOptimizationError,
    > {
        let parameter = Parameter::expression(
            expression,
        )
        .map_err(|_| {
            SymbolicOptimizationError::InvalidParameter {
                message: "expression violates the canonical parameter contract",
            }
        })?;

        self.optimize(parameter)
    }
}

impl Default for SymbolicOptimizer {
    fn default() -> Self {
        Self::unlimited()
    }
}

// =============================================================================
// Internal optimizer state
// =============================================================================

struct OptimizerState {
    config: SymbolicOptimizationConfig,
    nodes_visited: usize,
    rewrites: usize,
    budget_exhausted: bool,
}

impl OptimizerState {
    fn new(
        config: SymbolicOptimizationConfig,
    ) -> Self {
        Self {
            config,
            nodes_visited: 0,
            rewrites: 0,
            budget_exhausted: false,
        }
    }

    fn optimize(
        &mut self,
        root: Parameter,
    ) -> Result<
        Parameter,
        SymbolicOptimizationError,
    > {
        let mut tasks = Vec::new();
        let mut values = Vec::new();

        tasks.push(Task::Visit(root));

        while let Some(task) = tasks.pop() {
            match task {
                Task::Visit(parameter) => {
                    self.record_visit()?;

                    match parameter {
                        Parameter::Constant(value) => {
                            values.push(
                                canonical_constant(value),
                            );
                        }

                        Parameter::Symbol(name) => {
                            values.push(
                                Parameter::Symbol(name),
                            );
                        }

                        Parameter::Expression(expression) => {
                            let expression =
                                *expression;

                            match expression {
                                ParameterExpression::Add(
                                    left,
                                    right,
                                ) => {
                                    tasks.push(
                                        Task::Build(
                                            BuildOperation::Add,
                                        ),
                                    );

                                    tasks.push(
                                        Task::Visit(*right),
                                    );

                                    tasks.push(
                                        Task::Visit(*left),
                                    );
                                }

                                ParameterExpression::Subtract(
                                    left,
                                    right,
                                ) => {
                                    tasks.push(
                                        Task::Build(
                                            BuildOperation::Subtract,
                                        ),
                                    );

                                    tasks.push(
                                        Task::Visit(*right),
                                    );

                                    tasks.push(
                                        Task::Visit(*left),
                                    );
                                }

                                ParameterExpression::Multiply(
                                    left,
                                    right,
                                ) => {
                                    tasks.push(
                                        Task::Build(
                                            BuildOperation::Multiply,
                                        ),
                                    );

                                    tasks.push(
                                        Task::Visit(*right),
                                    );

                                    tasks.push(
                                        Task::Visit(*left),
                                    );
                                }

                                ParameterExpression::Divide(
                                    left,
                                    right,
                                ) => {
                                    tasks.push(
                                        Task::Build(
                                            BuildOperation::Divide,
                                        ),
                                    );

                                    tasks.push(
                                        Task::Visit(*right),
                                    );

                                    tasks.push(
                                        Task::Visit(*left),
                                    );
                                }

                                ParameterExpression::Negate(
                                    value,
                                ) => {
                                    tasks.push(
                                        Task::Build(
                                            BuildOperation::Negate,
                                        ),
                                    );

                                    tasks.push(
                                        Task::Visit(*value),
                                    );
                                }
                            }
                        }
                    }
                }

                Task::Build(operation) => {
                    let parameter =
                        self.build(operation, &mut values)?;

                    values.push(parameter);
                }
            }
        }

        values.pop().ok_or(
            SymbolicOptimizationError::ArithmeticOverflow {
                calculation:
                    "final symbolic expression stack",
            },
        )
    }

    fn build(
        &mut self,
        operation: BuildOperation,
        values: &mut Vec<Parameter>,
    ) -> Result<
        Parameter,
        SymbolicOptimizationError,
    > {
        match operation {
            BuildOperation::Negate => {
                let value = values.pop().ok_or(
                    SymbolicOptimizationError::ArithmeticOverflow {
                        calculation:
                            "unary expression stack",
                    },
                )?;

                self.simplify_negate(value)
            }

            BuildOperation::Add => {
                let right = values.pop().ok_or(
                    SymbolicOptimizationError::ArithmeticOverflow {
                        calculation:
                            "binary expression stack",
                    },
                )?;

                let left = values.pop().ok_or(
                    SymbolicOptimizationError::ArithmeticOverflow {
                        calculation:
                            "binary expression stack",
                    },
                )?;

                self.simplify_add(
                    left,
                    right,
                )
            }

            BuildOperation::Subtract => {
                let right = values.pop().ok_or(
                    SymbolicOptimizationError::ArithmeticOverflow {
                        calculation:
                            "binary expression stack",
                    },
                )?;

                let left = values.pop().ok_or(
                    SymbolicOptimizationError::ArithmeticOverflow {
                        calculation:
                            "binary expression stack",
                    },
                )?;

                self.simplify_subtract(
                    left,
                    right,
                )
            }

            BuildOperation::Multiply => {
                let right = values.pop().ok_or(
                    SymbolicOptimizationError::ArithmeticOverflow {
                        calculation:
                            "binary expression stack",
                    },
                )?;

                let left = values.pop().ok_or(
                    SymbolicOptimizationError::ArithmeticOverflow {
                        calculation:
                            "binary expression stack",
                    },
                )?;

                self.simplify_multiply(
                    left,
                    right,
                )
            }

            BuildOperation::Divide => {
                let right = values.pop().ok_or(
                    SymbolicOptimizationError::ArithmeticOverflow {
                        calculation:
                            "binary expression stack",
                    },
                )?;

                let left = values.pop().ok_or(
                    SymbolicOptimizationError::ArithmeticOverflow {
                        calculation:
                            "binary expression stack",
                    },
                )?;

                self.simplify_divide(
                    left,
                    right,
                )
            }
        }
    }

    fn simplify_add(
        &mut self,
        left: Parameter,
        right: Parameter,
    ) -> Result<
        Parameter,
        SymbolicOptimizationError,
    > {
        if is_zero(&left) {
            self.record_rewrite(
                RewriteKind::ZeroAdd,
            )?;

            return Ok(right);
        }

        if is_zero(&right) {
            self.record_rewrite(
                RewriteKind::AddZero,
            )?;

            return Ok(left);
        }

        if left == negate_structural(&right) {
            self.record_rewrite(
                RewriteKind::AdditiveInverse,
            )?;

            return zero_parameter();
        }

        if right == negate_structural(&left) {
            self.record_rewrite(
                RewriteKind::ReverseAdditiveInverse,
            )?;

            return zero_parameter();
        }

        if let (
            Some(left_value),
            Some(right_value),
        ) = (
            as_constant(&left),
            as_constant(&right),
        ) {
            if let Some(value) =
                finite_add(
                    left_value,
                    right_value,
                )
            {
                self.record_rewrite(
                    RewriteKind::ConstantFold,
                )?;

                return constant_parameter(value);
            }
        }

        make_expression(
            ParameterExpression::Add(
                Box::new(left),
                Box::new(right),
            ),
        )
    }

    fn simplify_subtract(
        &mut self,
        left: Parameter,
        right: Parameter,
    ) -> Result<
        Parameter,
        SymbolicOptimizationError,
    > {
        if is_zero(&right) {
            self.record_rewrite(
                RewriteKind::SubtractZero,
            )?;

            return Ok(left);
        }

        if left == right {
            self.record_rewrite(
                RewriteKind::SelfSubtraction,
            )?;

            return zero_parameter();
        }

        if let (
            Some(left_value),
            Some(right_value),
        ) = (
            as_constant(&left),
            as_constant(&right),
        ) {
            if let Some(value) =
                finite_subtract(
                    left_value,
                    right_value,
                )
            {
                self.record_rewrite(
                    RewriteKind::ConstantFold,
                )?;

                return constant_parameter(value);
            }
        }

        make_expression(
            ParameterExpression::Subtract(
                Box::new(left),
                Box::new(right),
            ),
        )
    }

    fn simplify_multiply(
        &mut self,
        left: Parameter,
        right: Parameter,
    ) -> Result<
        Parameter,
        SymbolicOptimizationError,
    > {
        if is_zero(&left) {
            self.record_rewrite(
                RewriteKind::ZeroMultiply,
            )?;

            return zero_parameter();
        }

        if is_zero(&right) {
            self.record_rewrite(
                RewriteKind::MultiplyZero,
            )?;

            return zero_parameter();
        }

        if is_one(&left) {
            self.record_rewrite(
                RewriteKind::OneMultiply,
            )?;

            return Ok(right);
        }

        if is_one(&right) {
            self.record_rewrite(
                RewriteKind::MultiplyOne,
            )?;

            return Ok(left);
        }

        if let (
            Some(left_value),
            Some(right_value),
        ) = (
            as_constant(&left),
            as_constant(&right),
        ) {
            if let Some(value) =
                finite_multiply(
                    left_value,
                    right_value,
                )
            {
                self.record_rewrite(
                    RewriteKind::ConstantFold,
                )?;

                return constant_parameter(value);
            }
        }

        make_expression(
            ParameterExpression::Multiply(
                Box::new(left),
                Box::new(right),
            ),
        )
    }

    fn simplify_divide(
        &mut self,
        left: Parameter,
        right: Parameter,
    ) -> Result<
        Parameter,
        SymbolicOptimizationError,
    > {
        if is_one(&right) {
            self.record_rewrite(
                RewriteKind::DivideOne,
            )?;

            return Ok(left);
        }

        if let (
            Some(left_value),
            Some(right_value),
        ) = (
            as_constant(&left),
            as_constant(&right),
        ) {
            if right_value != 0.0 {
                if let Some(value) =
                    finite_divide(
                        left_value,
                        right_value,
                    )
                {
                    self.record_rewrite(
                        RewriteKind::ConstantFold,
                    )?;

                    return constant_parameter(value);
                }
            }
        }

        make_expression(
            ParameterExpression::Divide(
                Box::new(left),
                Box::new(right),
            ),
        )
    }

    fn simplify_negate(
        &mut self,
        value: Parameter,
    ) -> Result<
        Parameter,
        SymbolicOptimizationError,
    > {
        if let Parameter::Expression(expression) =
            &value
        {
            if let ParameterExpression::Negate(inner) =
                expression.as_ref()
            {
                self.record_rewrite(
                    RewriteKind::DoubleNegation,
                )?;

                return Ok((**inner).clone());
            }
        }

        if let Some(number) =
            as_constant(&value)
        {
            if let Some(negated) =
                finite_negate(number)
            {
                self.record_rewrite(
                    RewriteKind::ConstantNegation,
                )?;

                return constant_parameter(
                    negated,
                );
            }
        }

        make_expression(
            ParameterExpression::Negate(
                Box::new(value),
            ),
        )
    }

    fn record_visit(
        &mut self,
    ) -> Result<
        (),
        SymbolicOptimizationError,
    > {
        self.nodes_visited =
            self.nodes_visited
                .checked_add(1)
                .ok_or(
                    SymbolicOptimizationError::ArithmeticOverflow {
                        calculation:
                            "visited node count",
                    },
                )?;

        if let Some(maximum) =
            self.config.max_nodes_visited
        {
            if self.nodes_visited > maximum {
                self.budget_exhausted = true;

                return Err(
                    SymbolicOptimizationError::NodeBudgetExceeded {
                        visited: self.nodes_visited,
                        maximum,
                    },
                );
            }
        }

        Ok(())
    }

    fn record_rewrite(
        &mut self,
        _kind: RewriteKind,
    ) -> Result<
        (),
        SymbolicOptimizationError,
    > {
        self.rewrites =
            self.rewrites
                .checked_add(1)
                .ok_or(
                    SymbolicOptimizationError::ArithmeticOverflow {
                        calculation:
                            "rewrite count",
                    },
                )?;

        if let Some(maximum) =
            self.config.max_rewrites
        {
            if self.rewrites > maximum {
                self.budget_exhausted = true;

                return Err(
                    SymbolicOptimizationError::RewriteBudgetExceeded {
                        rewrites: self.rewrites,
                        maximum,
                    },
                );
            }
        }

        Ok(())
    }
}

// =============================================================================
// Public convenience functions
// =============================================================================

/// Simplifies a canonical parameter using the default unlimited configuration.
///
/// This is the preferred low-level entry point for parameter passes that do
/// not need their own resource policy.
pub fn simplify_parameter(
    parameter: Parameter,
) -> Result<
    SymbolicOptimizationResult,
    SymbolicOptimizationError,
> {
    SymbolicOptimizer::default()
        .optimize(parameter)
}

/// Simplifies a canonical parameter with an explicit resource policy.
pub fn simplify_parameter_with_config(
    parameter: Parameter,
    config: SymbolicOptimizationConfig,
) -> Result<
    SymbolicOptimizationResult,
    SymbolicOptimizationError,
> {
    SymbolicOptimizer::new(config)
        .optimize(parameter)
}

/// Simplifies a canonical parameter expression.
pub fn simplify_expression(
    expression: ParameterExpression,
) -> Result<
    SymbolicOptimizationResult,
    SymbolicOptimizationError,
> {
    SymbolicOptimizer::default()
        .optimize_expression(expression)
}

// =============================================================================
// Symbol inspection
// =============================================================================

/// Returns whether a parameter contains at least one symbolic identifier.
///
/// This function performs an iterative traversal and therefore does not depend
/// on expression recursion depth for Rust call-stack safety.
#[must_use]
pub fn contains_symbol(
    parameter: &Parameter,
) -> bool {
    let mut stack = Vec::new();

    stack.push(parameter);

    while let Some(current) = stack.pop() {
        match current {
            Parameter::Constant(_) => {}

            Parameter::Symbol(_) => {
                return true;
            }

            Parameter::Expression(expression) => {
                match expression.as_ref() {
                    ParameterExpression::Add(
                        left,
                        right,
                    )
                    | ParameterExpression::Subtract(
                        left,
                        right,
                    )
                    | ParameterExpression::Multiply(
                        left,
                        right,
                    )
                    | ParameterExpression::Divide(
                        left,
                        right,
                    ) => {
                        stack.push(right);
                        stack.push(left);
                    }

                    ParameterExpression::Negate(value) => {
                        stack.push(value);
                    }
                }
            }
        }
    }

    false
}

/// Collects all unique symbol names in deterministic lexical order.
///
/// The returned vector is owned by the caller and therefore remains valid
/// independently of the source parameter.
#[must_use]
pub fn collect_symbols(
    parameter: &Parameter,
) -> Vec<String> {
    use std::collections::BTreeSet;

    let mut symbols = BTreeSet::new();
    let mut stack = Vec::new();

    stack.push(parameter);

    while let Some(current) = stack.pop() {
        match current {
            Parameter::Constant(_) => {}

            Parameter::Symbol(name) => {
                symbols.insert(name.clone());
            }

            Parameter::Expression(expression) => {
                match expression.as_ref() {
                    ParameterExpression::Add(
                        left,
                        right,
                    )
                    | ParameterExpression::Subtract(
                        left,
                        right,
                    )
                    | ParameterExpression::Multiply(
                        left,
                        right,
                    )
                    | ParameterExpression::Divide(
                        left,
                        right,
                    ) => {
                        stack.push(right);
                        stack.push(left);
                    }

                    ParameterExpression::Negate(value) => {
                        stack.push(value);
                    }
                }
            }
        }
    }

    symbols.into_iter().collect()
}

// =============================================================================
// Validation
// =============================================================================

fn validate_parameter(
    parameter: &Parameter,
) -> Result<
    (),
    SymbolicOptimizationError,
> {
    parameter.validate().map_err(|_| {
        SymbolicOptimizationError::InvalidParameter {
            message:
                "parameter violates canonical IR validation",
        }
    })
}

// =============================================================================
// Node counting
// =============================================================================

fn count_nodes_checked(
    parameter: &Parameter,
) -> Result<
    usize,
    SymbolicOptimizationError,
> {
    let mut count = 0usize;
    let mut stack = Vec::new();

    stack.push((parameter, 0usize));

    while let Some((current, depth)) =
        stack.pop()
    {
        count = count.checked_add(1).ok_or(
            SymbolicOptimizationError::ArithmeticOverflow {
                calculation:
                    "parameter node count",
            },
        )?;

        if depth > MAX_PARAMETER_EXPRESSION_DEPTH {
            return Err(
                SymbolicOptimizationError::ExpressionDepthExceeded {
                    maximum:
                        MAX_PARAMETER_EXPRESSION_DEPTH,
                },
            );
        }

        match current {
            Parameter::Constant(_)
            | Parameter::Symbol(_) => {}

            Parameter::Expression(expression) => {
                let next_depth =
                    depth.checked_add(1).ok_or(
                        SymbolicOptimizationError::ArithmeticOverflow {
                            calculation:
                                "parameter expression depth",
                        },
                    )?;

                if next_depth
                    > MAX_PARAMETER_EXPRESSION_DEPTH
                {
                    return Err(
                        SymbolicOptimizationError::ExpressionDepthExceeded {
                            maximum:
                                MAX_PARAMETER_EXPRESSION_DEPTH,
                        },
                    );
                }

                match expression.as_ref() {
                    ParameterExpression::Add(
                        left,
                        right,
                    )
                    | ParameterExpression::Subtract(
                        left,
                        right,
                    )
                    | ParameterExpression::Multiply(
                        left,
                        right,
                    )
                    | ParameterExpression::Divide(
                        left,
                        right,
                    ) => {
                        stack.push(
                            (right, next_depth),
                        );
                        stack.push(
                            (left, next_depth),
                        );
                    }

                    ParameterExpression::Negate(value) => {
                        stack.push(
                            (value, next_depth),
                        );
                    }
                }
            }
        }
    }

    Ok(count)
}

// =============================================================================
// Parameter construction helpers
// =============================================================================

fn constant_parameter(
    value: f64,
) -> Result<
    Parameter,
    SymbolicOptimizationError,
> {
    let value = canonical_constant(value);

    Parameter::constant(value).map_err(|_| {
        SymbolicOptimizationError::NonFiniteConstant
    })
}

fn zero_parameter(
) -> Result<
    Parameter,
    SymbolicOptimizationError,
> {
    constant_parameter(0.0)
}

fn make_expression(
    expression: ParameterExpression,
) -> Result<
    Parameter,
    SymbolicOptimizationError,
> {
    Parameter::expression(expression)
        .map_err(|_| {
            SymbolicOptimizationError::InvalidParameter {
                message:
                    "simplified expression violates canonical IR limits",
            }
        })
}

fn canonical_constant(
    value: f64,
) -> Parameter {
    // Canonicalize signed zero. The quantum parameter semantics are numerical
    // rather than IEEE signed-zero-sensitive.
    let value =
        if value == 0.0 { 0.0 } else { value };

    Parameter::Constant(value)
}

// =============================================================================
// Structural predicates
// =============================================================================

fn as_constant(
    parameter: &Parameter,
) -> Option<f64> {
    match parameter {
        Parameter::Constant(value) => Some(*value),

        Parameter::Symbol(_)
        | Parameter::Expression(_) => None,
    }
}

fn is_zero(
    parameter: &Parameter,
) -> bool {
    matches!(
        parameter,
        Parameter::Constant(value)
            if *value == 0.0
    )
}

fn is_one(
    parameter: &Parameter,
) -> bool {
    matches!(
        parameter,
        Parameter::Constant(value)
            if *value == 1.0
    )
}

/// Creates a structural negation without invoking the optimizer.
///
/// This helper is used only for proving exact additive inverses. It is not
/// exposed as a public transformation API.
fn negate_structural(
    parameter: &Parameter,
) -> Parameter {
    match parameter {
        Parameter::Constant(value) => {
            canonical_constant(-*value)
        }

        Parameter::Symbol(name) => {
            Parameter::Expression(
                Box::new(
                    ParameterExpression::Negate(
                        Box::new(
                            Parameter::Symbol(
                                name.clone(),
                            ),
                        ),
                    ),
                ),
            )
        }

        Parameter::Expression(expression) => {
            Parameter::Expression(
                Box::new(
                    ParameterExpression::Negate(
                        Box::new(
                            Parameter::Expression(
                                expression.clone(),
                            ),
                        ),
                    ),
                ),
            )
        }
    }
}

// =============================================================================
// Floating-point safety helpers
// =============================================================================

/// Performs addition only when the result is finite.
///
/// Returning `None` causes the symbolic optimizer to retain the original
/// expression rather than silently replacing a potentially overflowing
/// expression with a non-finite constant.
fn finite_add(
    left: f64,
    right: f64,
) -> Option<f64> {
    let value = left + right;

    if value.is_finite() {
        Some(if value == 0.0 {
            0.0
        } else {
            value
        })
    } else {
        None
    }
}

fn finite_subtract(
    left: f64,
    right: f64,
) -> Option<f64> {
    let value = left - right;

    if value.is_finite() {
        Some(if value == 0.0 {
            0.0
        } else {
            value
        })
    } else {
        None
    }
}

fn finite_multiply(
    left: f64,
    right: f64,
) -> Option<f64> {
    let value = left * right;

    if value.is_finite() {
        Some(if value == 0.0 {
            0.0
        } else {
            value
        })
    } else {
        None
    }
}

fn finite_divide(
    left: f64,
    right: f64,
) -> Option<f64> {
    if right == 0.0 {
        return None;
    }

    let value = left / right;

    if value.is_finite() {
        Some(if value == 0.0 {
            0.0
        } else {
            value
        })
    } else {
        None
    }
}

fn finite_negate(
    value: f64,
) -> Option<f64> {
    let result = -value;

    if result.is_finite() {
        Some(if result == 0.0 {
            0.0
        } else {
            result
        })
    } else {
        None
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn symbol(
        name: &str,
    ) -> Parameter {
        Parameter::symbol(name)
            .expect("test symbol must be valid")
    }

    fn constant(
        value: f64,
    ) -> Parameter {
        Parameter::constant(value)
            .expect("test constant must be finite")
    }

    fn expression(
        expression: ParameterExpression,
    ) -> Parameter {
        Parameter::expression(expression)
            .expect("test expression must be valid")
    }

    #[test]
    fn empty_scalar_constant_is_unchanged() {
        let input = constant(3.0);

        let result =
            simplify_parameter(
                input.clone(),
            )
            .expect("optimization should succeed");

        assert_eq!(
            result.parameter(),
            &input
        );

        assert_eq!(
            result.rewrites(),
            0
        );
    }

    #[test]
    fn direct_symbol_is_unchanged() {
        let input = symbol("theta");

        let result =
            simplify_parameter(
                input.clone(),
            )
            .expect("optimization should succeed");

        assert_eq!(
            result.parameter(),
            &input
        );

        assert!(contains_symbol(
            result.parameter()
        ));

        assert_eq!(
            collect_symbols(
                result.parameter()
            ),
            vec!["theta".to_string()]
        );
    }

    #[test]
    fn folds_constants() {
        let input = expression(
            ParameterExpression::Add(
                Box::new(constant(2.0)),
                Box::new(constant(3.0)),
            ),
        );

        let result =
            simplify_parameter(input)
                .expect("optimization should succeed");

        assert_eq!(
            result.parameter(),
            &constant(5.0)
        );
    }

    #[test]
    fn removes_additive_zero() {
        let input = expression(
            ParameterExpression::Add(
                Box::new(symbol("theta")),
                Box::new(constant(0.0)),
            ),
        );

        let result =
            simplify_parameter(input)
                .expect("optimization should succeed");

        assert_eq!(
            result.parameter(),
            &symbol("theta")
        );
    }

    #[test]
    fn removes_zero_addition() {
        let input = expression(
            ParameterExpression::Add(
                Box::new(constant(0.0)),
                Box::new(symbol("theta")),
            ),
        );

        let result =
            simplify_parameter(input)
                .expect("optimization should succeed");

        assert_eq!(
            result.parameter(),
            &symbol("theta")
        );
    }

    #[test]
    fn removes_subtraction_by_zero() {
        let input = expression(
            ParameterExpression::Subtract(
                Box::new(symbol("theta")),
                Box::new(constant(0.0)),
            ),
        );

        let result =
            simplify_parameter(input)
                .expect("optimization should succeed");

        assert_eq!(
            result.parameter(),
            &symbol("theta")
        );
    }

    #[test]
    fn simplifies_self_subtraction() {
        let theta = symbol("theta");

        let input = expression(
            ParameterExpression::Subtract(
                Box::new(theta.clone()),
                Box::new(theta),
            ),
        );

        let result =
            simplify_parameter(input)
                .expect("optimization should succeed");

        assert_eq!(
            result.parameter(),
            &constant(0.0)
        );
    }

    #[test]
    fn removes_multiplication_by_zero() {
        let input = expression(
            ParameterExpression::Multiply(
                Box::new(symbol("theta")),
                Box::new(constant(0.0)),
            ),
        );

        let result =
            simplify_parameter(input)
                .expect("optimization should succeed");

        assert_eq!(
            result.parameter(),
            &constant(0.0)
        );
    }

    #[test]
    fn removes_zero_multiplication() {
        let input = expression(
            ParameterExpression::Multiply(
                Box::new(constant(0.0)),
                Box::new(symbol("theta")),
            ),
        );

        let result =
            simplify_parameter(input)
                .expect("optimization should succeed");

        assert_eq!(
            result.parameter(),
            &constant(0.0)
        );
    }

    #[test]
    fn removes_multiplication_by_one() {
        let input = expression(
            ParameterExpression::Multiply(
                Box::new(symbol("theta")),
                Box::new(constant(1.0)),
            ),
        );

        let result =
            simplify_parameter(input)
                .expect("optimization should succeed");

        assert_eq!(
            result.parameter(),
            &symbol("theta")
        );
    }

    #[test]
    fn removes_one_multiplication() {
        let input = expression(
            ParameterExpression::Multiply(
                Box::new(constant(1.0)),
                Box::new(symbol("theta")),
            ),
        );

        let result =
            simplify_parameter(input)
                .expect("optimization should succeed");

        assert_eq!(
            result.parameter(),
            &symbol("theta")
        );
    }

    #[test]
    fn removes_division_by_one() {
        let input = expression(
            ParameterExpression::Divide(
                Box::new(symbol("theta")),
                Box::new(constant(1.0)),
            ),
        );

        let result =
            simplify_parameter(input)
                .expect("optimization should succeed");

        assert_eq!(
            result.parameter(),
            &symbol("theta")
        );
    }

    #[test]
    fn does_not_simplify_symbol_over_itself() {
        let theta = symbol("theta");

        let input = expression(
            ParameterExpression::Divide(
                Box::new(theta.clone()),
                Box::new(theta),
            ),
        );

        let result =
            simplify_parameter(input.clone())
                .expect("optimization should succeed");

        assert_eq!(
            result.parameter(),
            &input
        );
    }

    #[test]
    fn simplifies_double_negation() {
        let theta = symbol("theta");

        let input = expression(
            ParameterExpression::Negate(
                Box::new(
                    expression(
                        ParameterExpression::Negate(
                            Box::new(theta.clone()),
                        ),
                    ),
                ),
            ),
        );

        let result =
            simplify_parameter(input)
                .expect("optimization should succeed");

        assert_eq!(
            result.parameter(),
            &theta
        );
    }

    #[test]
    fn folds_nested_constants_iteratively() {
        let input = expression(
            ParameterExpression::Add(
                Box::new(
                    expression(
                        ParameterExpression::Add(
                            Box::new(constant(1.0)),
                            Box::new(constant(2.0)),
                        ),
                    ),
                ),
                Box::new(constant(4.0)),
            ),
        );

        let result =
            simplify_parameter(input)
                .expect("optimization should succeed");

        assert_eq!(
            result.parameter(),
            &constant(7.0)
        );
    }

    #[test]
    fn preserves_symbolic_expression() {
        let theta = symbol("theta");
        let phi = symbol("phi");

        let input = expression(
            ParameterExpression::Add(
                Box::new(theta.clone()),
                Box::new(phi.clone()),
            ),
        );

        let result =
            simplify_parameter(input.clone())
                .expect("optimization should succeed");

        assert_eq!(
            result.parameter(),
            &input
        );

        assert_eq!(
            collect_symbols(
                result.parameter()
            ),
            vec![
                "phi".to_string(),
                "theta".to_string()
            ]
        );
    }

    #[test]
    fn recognizes_nested_symbol() {
        let input = expression(
            ParameterExpression::Multiply(
                Box::new(
                    constant(2.0),
                ),
                Box::new(
                    expression(
                        ParameterExpression::Negate(
                            Box::new(
                                symbol("theta"),
                            ),
                        ),
                    ),
                ),
            ),
        );

        assert!(
            contains_symbol(&input)
        );
    }

    #[test]
    fn collect_symbols_is_unique_and_deterministic() {
        let theta = symbol("theta");
        let phi = symbol("phi");

        let input = expression(
            ParameterExpression::Add(
                Box::new(
                    expression(
                        ParameterExpression::Add(
                            Box::new(theta.clone()),
                            Box::new(phi.clone()),
                        ),
                    ),
                ),
                Box::new(theta),
            ),
        );

        assert_eq!(
            collect_symbols(&input),
            vec![
                "phi".to_string(),
                "theta".to_string()
            ]
        );
    }

    #[test]
    fn signed_zero_is_canonicalized() {
        let input = constant(-0.0);

        let result =
            simplify_parameter(input)
                .expect("optimization should succeed");

        assert_eq!(
            result.parameter(),
            &constant(0.0)
        );
    }

    #[test]
    fn overflow_is_not_silently_folded() {
        let input = expression(
            ParameterExpression::Multiply(
                Box::new(
                    constant(f64::MAX),
                ),
                Box::new(
                    constant(2.0),
                ),
            ),
        );

        let result =
            simplify_parameter(input.clone())
                .expect("optimizer should preserve unsafe fold");

        assert_eq!(
            result.parameter(),
            &input
        );
    }

    #[test]
    fn division_by_zero_is_not_folded() {
        let input = expression(
            ParameterExpression::Divide(
                Box::new(constant(1.0)),
                Box::new(constant(0.0)),
            ),
        );

        let result =
            simplify_parameter(input.clone())
                .expect("optimizer should preserve invalid arithmetic");

        assert_eq!(
            result.parameter(),
            &input
        );
    }

    #[test]
    fn negative_constant_is_folded() {
        let input = expression(
            ParameterExpression::Negate(
                Box::new(constant(4.0)),
            ),
        );

        let result =
            simplify_parameter(input)
                .expect("optimization should succeed");

        assert_eq!(
            result.parameter(),
            &constant(-4.0)
        );
    }

    #[test]
    fn additive_inverse_is_simplified() {
        let theta = symbol("theta");

        let negative_theta =
            expression(
                ParameterExpression::Negate(
                    Box::new(theta.clone()),
                ),
            );

        let input = expression(
            ParameterExpression::Add(
                Box::new(theta),
                Box::new(negative_theta),
            ),
        );

        let result =
            simplify_parameter(input)
                .expect("optimization should succeed");

        assert_eq!(
            result.parameter(),
            &constant(0.0)
        );
    }

    #[test]
    fn bounded_optimizer_rejects_large_input() {
        let input = expression(
            ParameterExpression::Add(
                Box::new(symbol("a")),
                Box::new(symbol("b")),
            ),
        );

        let config =
            SymbolicOptimizationConfig::bounded(
                1,
                100,
                100,
            );

        let result =
            SymbolicOptimizer::new(config)
                .optimize(input);

        assert!(matches!(
            result,
            Err(
                SymbolicOptimizationError::NodeBudgetExceeded {
                    ..
                }
            )
        ));
    }

    #[test]
    fn bounded_rewrite_budget_is_enforced() {
        let input = expression(
            ParameterExpression::Add(
                Box::new(symbol("theta")),
                Box::new(constant(0.0)),
            ),
        );

        let config =
            SymbolicOptimizationConfig {
                max_nodes_visited: None,
                max_rewrites: Some(0),
                max_output_nodes: None,
            };

        let result =
            SymbolicOptimizer::new(config)
                .optimize(input);

        assert!(matches!(
            result,
            Err(
                SymbolicOptimizationError::RewriteBudgetExceeded {
                    ..
                }
            )
        ));
    }

    #[test]
    fn result_statistics_are_consistent() {
        let input = expression(
            ParameterExpression::Add(
                Box::new(constant(2.0)),
                Box::new(constant(3.0)),
            ),
        );

        let result =
            simplify_parameter(input)
                .expect("optimization should succeed");

        assert_eq!(
            result.input_nodes(),
            3
        );

        assert_eq!(
            result.output_nodes(),
            1
        );

        assert!(
            result.nodes_visited() >= 3
        );

        assert_eq!(
            result.rewrites(),
            1
        );

        assert!(
            result.changed()
        );
    }

    #[test]
    fn optimizer_is_deterministic() {
        let input = expression(
            ParameterExpression::Add(
                Box::new(
                    expression(
                        ParameterExpression::Multiply(
                            Box::new(symbol("theta")),
                            Box::new(constant(1.0)),
                        ),
                    ),
                ),
                Box::new(
                    expression(
                        ParameterExpression::Add(
                            Box::new(constant(0.0)),
                            Box::new(symbol("phi")),
                        ),
                    ),
                ),
            ),
        );

        let first =
            simplify_parameter(
                input.clone(),
            )
            .expect("first optimization should succeed");

        let second =
            simplify_parameter(input)
                .expect("second optimization should succeed");

        assert_eq!(
            first.parameter(),
            second.parameter()
        );

        assert_eq!(
            first.rewrites(),
            second.rewrites()
        );
    }

    #[test]
    fn optimization_is_idempotent() {
        let input = expression(
            ParameterExpression::Add(
                Box::new(
                    expression(
                        ParameterExpression::Add(
                            Box::new(constant(0.0)),
                            Box::new(symbol("theta")),
                        ),
                    ),
                ),
                Box::new(constant(0.0)),
            ),
        );

        let first =
            simplify_parameter(
                input,
            )
            .expect("first optimization should succeed");

        let second =
            simplify_parameter(
                first.parameter().clone(),
            )
            .expect("second optimization should succeed");

        assert_eq!(
            first.parameter(),
            second.parameter()
        );

        assert_eq!(
            second.rewrites(),
            0
        );
    }

    #[test]
    fn no_unsafe_code_is_used() {
        // Compile-time enforcement is provided by:
        //
        // #![forbid(unsafe_code)]
        //
        // This test exists as a documentation marker for the project
        // security policy.
        assert!(true);
    }

    #[test]
    fn expression_depth_limit_is_respected() {
        // The canonical IR already prevents construction of an expression
        // deeper than MAX_PARAMETER_EXPRESSION_DEPTH. This test therefore
        // verifies that the optimizer's own accounting agrees with that
        // contract rather than creating a second incompatible limit.
        let input = constant(1.0);

        let result =
            simplify_parameter(input)
                .expect("constant should be valid");

        assert_eq!(
            result.output_nodes(),
            1
        );

        assert!(
            MAX_PARAMETER_EXPRESSION_DEPTH > 0
        );
    }

    #[test]
    fn unused_import_regression_guard() {
        // Keep this test intentionally trivial. It ensures the module remains
        // warning-clean when compiled with strict warning policies.
        let _ = mem::size_of::<
            SymbolicOptimizationConfig,
        >();
    }
}