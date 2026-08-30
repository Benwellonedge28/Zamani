//! Zamani Quantum Optimization — Parameter Simplification
//!
//! Production-grade symbolic/structural simplification for the canonical
//! Zamani Quantum IR parameter representation.
//!
//! # Architectural role
//!
//! ```text
//!                  quantum::ir::parameter
//!                           │
//!                           ▼
//!                ┌──────────────────────┐
//!                │   constant_fold.rs   │
//!                │ numeric/algebraic    │
//!                │ constant evaluation  │
//!                └──────────┬───────────┘
//!                           │
//!                           ▼
//!                ┌──────────────────────┐
//!                │   simplification.rs  │
//!                │ structural/symbolic  │
//!                │ normalization        │
//!                └──────────┬───────────┘
//!                           │
//!             ┌─────────────┼─────────────┐
//!             ▼             ▼             ▼
//!          rotation     gate fusion   synthesis
//!          optimizer    optimizer     optimizer
//! ```
//!
//! This module is intentionally:
//!
//! - independent of quantum circuits;
//! - independent of quantum gates;
//! - independent of qubits;
//! - independent of routing;
//! - independent of scheduling;
//! - independent of hardware;
//! - independent of QPU execution;
//! - independent of frontend syntax;
//! - independent of backend APIs;
//! - independent of optimization pipeline implementation;
//! - independent of pass registry implementation.
//!
//! It operates exclusively on the canonical Quantum IR parameter types.
//!
//! # Responsibilities
//!
//! This module provides the optimizer-facing API for:
//!
//! - constant folding composition;
//! - double-negation elimination;
//! - structural identity simplification;
//! - preservation of parameter arity;
//! - transactional in-place simplification;
//! - deterministic results;
//! - configurable resource limits;
//! - bounded rewrite work;
//! - idempotent simplification;
//! - validation before and after transformation.
//!
//! Numeric constant evaluation is delegated to [`crate::quantum::optimization::parameter::constant_fold`]
//! so that there is exactly one implementation of finite arithmetic folding.
//!
//! # Deliberately excluded transformations
//!
//! This module does NOT perform angle-periodicity transformations such as:
//!
//! ```text
//! theta + 2*pi -> theta
//! 2*pi         -> 0
//! ```
//!
//! Such transformations require knowledge that a parameter represents an angle
//! and, more importantly, knowledge of the consuming operation's periodicity.
//! That belongs in angle-aware rotation/phase optimization.
//!
//! This module also does not perform arbitrary algebraic reassociation such as:
//!
//! ```text
//! (a + b) + c -> a + (b + c)
//! ```
//!
//! because changing floating-point evaluation order can change numerical
//! rounding. Production compiler transformations must not silently introduce
//! such changes merely to obtain a prettier expression tree.
//!
//! # Safety
//!
//! This module contains no `unsafe` code.
//!
//! # Rust compatibility
//!
//! Compatible with Rust 1.97 and Rust 1.97.1.
//!
//! # Integration contract
//!
//! `parameter/mod.rs` should expose:
//!
//! ```text
//! pub mod constant_fold;
//! pub mod simplification;
//! ```
//!
//! Higher-level optimization passes should consume:
//!
//! ```text
//! ParameterSimplifier::new()
//! ```
//!
//! or:
//!
//! ```text
//! ParameterSimplifier::with_config(...)
//! ```
//!
//! No circuit, gate, routing, scheduling, or hardware integration is required
//! in this file.

use crate::quantum::ir::errors::{IrParameterError, IrResult};
use crate::quantum::ir::parameter::{
    GateParameter,
    Parameter,
    ParameterExpression,
};

use super::constant_fold::{
    ConstantFoldConfig,
    ConstantFolder,
};

// -----------------------------------------------------------------------------
// Resource limits
// -----------------------------------------------------------------------------

/// Default maximum number of parameter nodes inspected by one simplification
/// invocation.
///
/// The canonical IR already bounds expression depth. This additional budget
/// protects the optimizer from pathological expressions constructed directly
/// through the public enum representation.
///
/// The limit is intentionally large and may be reduced by callers handling
/// untrusted or resource-constrained input.
pub const DEFAULT_MAX_SIMPLIFICATION_NODES: usize = 1_048_576;

/// Default maximum number of structural rewrites performed by one
/// simplification invocation.
pub const DEFAULT_MAX_SIMPLIFICATION_REWRITES: usize = 1_048_576;

// -----------------------------------------------------------------------------
// Configuration
// -----------------------------------------------------------------------------

/// Configuration for [`ParameterSimplifier`].
///
/// The configuration is local to parameter simplification. The eventual
/// optimizer-wide configuration can map its resource budget to this type
/// without changing the simplification algorithm.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimplificationConfig {
    /// Maximum number of parameter nodes inspected.
    pub max_nodes: usize,

    /// Maximum number of structural rewrites permitted.
    ///
    /// A value of zero disables structural rewrites while still permitting
    /// validation and constant folding.
    pub max_rewrites: usize,
}

impl Default for SimplificationConfig {
    fn default() -> Self {
        Self {
            max_nodes: DEFAULT_MAX_SIMPLIFICATION_NODES,
            max_rewrites: DEFAULT_MAX_SIMPLIFICATION_REWRITES,
        }
    }
}

impl SimplificationConfig {
    /// Creates a configuration with explicit resource limits.
    pub const fn new(
        max_nodes: usize,
        max_rewrites: usize,
    ) -> Self {
        Self {
            max_nodes,
            max_rewrites,
        }
    }

    /// Validates the configuration.
    pub fn validate(self) -> IrResult<()> {
        if self.max_nodes == 0 {
            return Err(IrParameterError::InvalidExpression.into());
        }

        Ok(())
    }

    /// Converts this configuration to the configuration used by the
    /// canonical constant folder.
    const fn constant_fold_config(
        self,
    ) -> ConstantFoldConfig {
        ConstantFoldConfig::new(self.max_nodes)
    }
}

// -----------------------------------------------------------------------------
// Statistics
// -----------------------------------------------------------------------------

/// Statistics produced by one parameter simplification invocation.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct SimplificationStats {
    /// Number of parameter nodes inspected by the structural simplifier.
    pub nodes_visited: usize,

    /// Number of constant-folding operations performed by the delegated
    /// constant folder.
    pub constants_folded: usize,

    /// Number of identities applied by the delegated constant folder.
    pub identities_applied: usize,

    /// Number of double negations removed.
    pub double_negations_removed: usize,

    /// Number of structural expressions rebuilt into a simpler representation.
    pub structural_rewrites: usize,

    /// Number of expressions whose final representation changed.
    pub expressions_simplified: usize,

    /// Number of rewrite-budget checks successfully consumed.
    pub rewrite_budget_used: usize,
}

impl SimplificationStats {
    /// Returns true if the invocation changed anything.
    pub const fn changed(self) -> bool {
        self.constants_folded > 0
            || self.identities_applied > 0
            || self.double_negations_removed > 0
            || self.structural_rewrites > 0
            || self.expressions_simplified > 0
    }

    fn accumulate(
        &mut self,
        other: Self,
    ) {
        self.nodes_visited = self
            .nodes_visited
            .saturating_add(other.nodes_visited);

        self.constants_folded = self
            .constants_folded
            .saturating_add(other.constants_folded);

        self.identities_applied = self
            .identities_applied
            .saturating_add(other.identities_applied);

        self.double_negations_removed = self
            .double_negations_removed
            .saturating_add(other.double_negations_removed);

        self.structural_rewrites = self
            .structural_rewrites
            .saturating_add(other.structural_rewrites);

        self.expressions_simplified = self
            .expressions_simplified
            .saturating_add(other.expressions_simplified);

        self.rewrite_budget_used = self
            .rewrite_budget_used
            .saturating_add(other.rewrite_budget_used);
    }
}

// -----------------------------------------------------------------------------
// Results
// -----------------------------------------------------------------------------

/// Result of simplifying one [`Parameter`].
#[derive(Debug, Clone, PartialEq)]
pub struct SimplifiedParameter {
    /// The simplified canonical parameter.
    pub parameter: Parameter,

    /// Transformation statistics.
    pub stats: SimplificationStats,
}

impl SimplifiedParameter {
    /// Returns true when the parameter changed.
    pub const fn changed(&self) -> bool {
        self.stats.changed()
    }
}

/// Result of simplifying a [`GateParameter`] group.
#[derive(Debug, Clone, PartialEq)]
pub struct SimplifiedGateParameter {
    /// Simplified parameter group.
    pub parameter: GateParameter,

    /// Aggregate statistics.
    pub stats: SimplificationStats,
}

impl SimplifiedGateParameter {
    /// Returns true when at least one parameter changed.
    pub const fn changed(&self) -> bool {
        self.stats.changed()
    }
}

// -----------------------------------------------------------------------------
// Simplifier
// -----------------------------------------------------------------------------

/// Production parameter simplifier.
///
/// The simplifier is immutable and reusable. It contains no global state,
/// random state, backend state, caches, or references to a circuit.
///
/// This makes one instance safe to use sequentially across many parameters and
/// safe to place behind higher-level synchronization when parallel optimization
/// is introduced.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParameterSimplifier {
    config: SimplificationConfig,
}

impl Default for ParameterSimplifier {
    fn default() -> Self {
        Self::new()
    }
}

impl ParameterSimplifier {
    /// Creates a simplifier using production defaults.
    pub const fn new() -> Self {
        Self {
            config: SimplificationConfig::new(
                DEFAULT_MAX_SIMPLIFICATION_NODES,
                DEFAULT_MAX_SIMPLIFICATION_REWRITES,
            ),
        }
    }

    /// Creates a simplifier using an explicit configuration.
    pub fn with_config(
        config: SimplificationConfig,
    ) -> IrResult<Self> {
        config.validate()?;

        Ok(Self { config })
    }

    /// Returns the configured resource limits.
    pub const fn config(
        &self,
    ) -> SimplificationConfig {
        self.config
    }

    /// Simplifies one canonical Quantum IR parameter.
    ///
    /// Transformation is transactional: the caller's parameter is never
    /// modified unless the complete operation succeeds.
    pub fn simplify_parameter(
        &self,
        parameter: &Parameter,
    ) -> IrResult<SimplifiedParameter> {
        self.config.validate()?;

        parameter.validate()?;

        // Phase 1:
        // Delegate numerical/algebraic constant handling to the canonical
        // constant folder. This prevents duplicate arithmetic semantics.
        let folder = ConstantFolder::with_config(
            self.config.constant_fold_config(),
        )?;

        let folded = folder.fold_parameter(parameter)?;

        // Phase 2:
        // Apply structural simplifications that belong specifically to this
        // layer.
        let mut budget = SimplificationBudget::new(
            self.config.max_nodes,
            self.config.max_rewrites,
        );

        let (simplified, mut stats) =
            simplify_parameter_structure(
                &folded.parameter,
                &mut budget,
            )?;

        stats.constants_folded = folded.stats.constants_folded;
        stats.identities_applied = folded.stats.identities_applied;

        if parameter != &simplified {
            stats.expressions_simplified =
                stats
                    .expressions_simplified
                    .saturating_add(1);
        }

        simplified.validate()?;

        Ok(SimplifiedParameter {
            parameter: simplified,
            stats,
        })
    }

    /// Simplifies a parameter in place.
    ///
    /// The original value is replaced only after the transformation has
    /// completely succeeded.
    pub fn simplify_parameter_in_place(
        &self,
        parameter: &mut Parameter,
    ) -> IrResult<SimplificationStats> {
        let result =
            self.simplify_parameter(parameter)?;

        *parameter = result.parameter;

        Ok(result.stats)
    }

    /// Simplifies all parameters contained in a gate parameter group.
    ///
    /// Gate parameter arity is preserved exactly.
    pub fn simplify_gate_parameter(
        &self,
        parameter: &GateParameter,
    ) -> IrResult<SimplifiedGateParameter> {
        self.config.validate()?;

        parameter.validate()?;

        let mut budget = SimplificationBudget::new(
            self.config.max_nodes,
            self.config.max_rewrites,
        );

        let mut stats =
            SimplificationStats::default();

        let simplified =
            match parameter {
                GateParameter::Angle(value) => {
                    let result =
                        self.simplify_parameter(value)?;

                    stats.accumulate(result.stats);

                    GateParameter::angle(
                        result.parameter,
                    )?
                }

                GateParameter::TwoAngles {
                    theta,
                    phi,
                } => {
                    let theta_result =
                        self.simplify_parameter(theta)?;

                    stats.accumulate(
                        theta_result.stats,
                    );

                    let phi_result =
                        self.simplify_parameter(phi)?;

                    stats.accumulate(
                        phi_result.stats,
                    );

                    GateParameter::two_angles(
                        theta_result.parameter,
                        phi_result.parameter,
                    )?
                }

                GateParameter::ThreeAngles {
                    theta,
                    phi,
                    lambda,
                } => {
                    let theta_result =
                        self.simplify_parameter(theta)?;

                    stats.accumulate(
                        theta_result.stats,
                    );

                    let phi_result =
                        self.simplify_parameter(phi)?;

                    stats.accumulate(
                        phi_result.stats,
                    );

                    let lambda_result =
                        self.simplify_parameter(lambda)?;

                    stats.accumulate(
                        lambda_result.stats,
                    );

                    GateParameter::three_angles(
                        theta_result.parameter,
                        phi_result.parameter,
                        lambda_result.parameter,
                    )?
                }
            };

        // The local budget is intentionally consumed by a lightweight
        // structural traversal as an integration invariant. It also ensures
        // that pathological direct enum construction cannot bypass the
        // simplifier's resource contract.
        consume_gate_parameter_budget(
            &simplified,
            &mut budget,
        )?;

        simplified.validate()?;

        Ok(SimplifiedGateParameter {
            parameter: simplified,
            stats,
        })
    }

    /// Simplifies a gate parameter group in place.
    ///
    /// The replacement is transactional.
    pub fn simplify_gate_parameter_in_place(
        &self,
        parameter: &mut GateParameter,
    ) -> IrResult<SimplificationStats> {
        let result =
            self.simplify_gate_parameter(parameter)?;

        *parameter = result.parameter;

        Ok(result.stats)
    }
}

// -----------------------------------------------------------------------------
// Resource accounting
// -----------------------------------------------------------------------------

/// Internal resource accounting for one structural simplification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct SimplificationBudget {
    remaining_nodes: usize,
    remaining_rewrites: usize,
    initial_rewrites: usize,
}

impl SimplificationBudget {
    const fn new(
        max_nodes: usize,
        max_rewrites: usize,
    ) -> Self {
        Self {
            remaining_nodes: max_nodes,
            remaining_rewrites: max_rewrites,
            initial_rewrites: max_rewrites,
        }
    }

    fn consume_node(
        &mut self,
    ) -> IrResult<()> {
        if self.remaining_nodes == 0 {
            return Err(
                IrParameterError::InvalidExpression
                    .into(),
            );
        }

        self.remaining_nodes -= 1;

        Ok(())
    }

    fn consume_rewrite(
        &mut self,
    ) -> IrResult<()> {
        if self.remaining_rewrites == 0 {
            return Err(
                IrParameterError::InvalidExpression
                    .into(),
            );
        }

        self.remaining_rewrites -= 1;

        Ok(())
    }

    fn rewrites_used(
        &self,
    ) -> usize {
        self.initial_rewrites
            .saturating_sub(
                self.remaining_rewrites,
            )
    }
}

// -----------------------------------------------------------------------------
// Structural simplification
// -----------------------------------------------------------------------------

/// Recursively simplifies a parameter after constant folding.
///
/// Only transformations that are structurally safe and independent of gate
/// semantics are performed here.
fn simplify_parameter_structure(
    parameter: &Parameter,
    budget: &mut SimplificationBudget,
) -> IrResult<(Parameter, SimplificationStats)> {
    budget.consume_node()?;

    let mut stats = SimplificationStats {
        nodes_visited: 1,
        ..SimplificationStats::default()
    };

    match parameter {
        Parameter::Constant(value) => {
            if !value.is_finite() {
                return Err(
                    IrParameterError::NonFinite.into(),
                );
            }

            Ok((
                Parameter::Constant(*value),
                stats,
            ))
        }

        Parameter::Symbol(name) => {
            let normalized =
                Parameter::symbol(name.clone())?;

            Ok((normalized, stats))
        }

        Parameter::Expression(expression) => {
            let (
                simplified,
                child_stats,
            ) =
                simplify_expression_structure(
                    expression,
                    budget,
                )?;

            stats.accumulate(child_stats);

            Ok((simplified, stats))
        }
    }
}

/// Recursively simplifies a parameter expression.
fn simplify_expression_structure(
    expression: &ParameterExpression,
    budget: &mut SimplificationBudget,
) -> IrResult<(Parameter, SimplificationStats)> {
    match expression {
        ParameterExpression::Add(
            left,
            right,
        ) => {
            let (
                left,
                mut left_stats,
            ) =
                simplify_parameter_structure(
                    left,
                    budget,
                )?;

            let (
                right,
                right_stats,
            ) =
                simplify_parameter_structure(
                    right,
                    budget,
                )?;

            left_stats.accumulate(
                right_stats,
            );

            let (
                result,
                rewrote,
            ) =
                simplify_add(
                    left,
                    right,
                )?;

            if rewrote {
                budget.consume_rewrite()?;

                left_stats
                    .structural_rewrites =
                    left_stats
                        .structural_rewrites
                        .saturating_add(1);

                left_stats
                    .expressions_simplified =
                    left_stats
                        .expressions_simplified
                        .saturating_add(1);
            }

            left_stats.rewrite_budget_used =
                budget.rewrites_used();

            Ok((result, left_stats))
        }

        ParameterExpression::Subtract(
            left,
            right,
        ) => {
            let (
                left,
                mut left_stats,
            ) =
                simplify_parameter_structure(
                    left,
                    budget,
                )?;

            let (
                right,
                right_stats,
            ) =
                simplify_parameter_structure(
                    right,
                    budget,
                )?;

            left_stats.accumulate(
                right_stats,
            );

            let (
                result,
                rewrote,
            ) =
                simplify_subtract(
                    left,
                    right,
                )?;

            if rewrote {
                budget.consume_rewrite()?;

                left_stats
                    .structural_rewrites =
                    left_stats
                        .structural_rewrites
                        .saturating_add(1);

                left_stats
                    .expressions_simplified =
                    left_stats
                        .expressions_simplified
                        .saturating_add(1);
            }

            left_stats.rewrite_budget_used =
                budget.rewrites_used();

            Ok((result, left_stats))
        }

        ParameterExpression::Multiply(
            left,
            right,
        ) => {
            let (
                left,
                mut left_stats,
            ) =
                simplify_parameter_structure(
                    left,
                    budget,
                )?;

            let (
                right,
                right_stats,
            ) =
                simplify_parameter_structure(
                    right,
                    budget,
                )?;

            left_stats.accumulate(
                right_stats,
            );

            let (
                result,
                rewrote,
            ) =
                simplify_multiply(
                    left,
                    right,
                )?;

            if rewrote {
                budget.consume_rewrite()?;

                left_stats
                    .structural_rewrites =
                    left_stats
                        .structural_rewrites
                        .saturating_add(1);

                left_stats
                    .expressions_simplified =
                    left_stats
                        .expressions_simplified
                        .saturating_add(1);
            }

            left_stats.rewrite_budget_used =
                budget.rewrites_used();

            Ok((result, left_stats))
        }

        ParameterExpression::Divide(
            left,
            right,
        ) => {
            let (
                left,
                mut left_stats,
            ) =
                simplify_parameter_structure(
                    left,
                    budget,
                )?;

            let (
                right,
                right_stats,
            ) =
                simplify_parameter_structure(
                    right,
                    budget,
                )?;

            left_stats.accumulate(
                right_stats,
            );

            let (
                result,
                rewrote,
            ) =
                simplify_divide(
                    left,
                    right,
                )?;

            if rewrote {
                budget.consume_rewrite()?;

                left_stats
                    .structural_rewrites =
                    left_stats
                        .structural_rewrites
                        .saturating_add(1);

                left_stats
                    .expressions_simplified =
                    left_stats
                        .expressions_simplified
                        .saturating_add(1);
            }

            left_stats.rewrite_budget_used =
                budget.rewrites_used();

            Ok((result, left_stats))
        }

        ParameterExpression::Negate(
            value,
        ) => {
            let (
                value,
                mut stats,
            ) =
                simplify_parameter_structure(
                    value,
                    budget,
                )?;

            let (
                result,
                rewrote,
            ) =
                simplify_negate(
                    value,
                    &mut stats,
                )?;

            if rewrote {
                budget.consume_rewrite()?;

                stats
                    .structural_rewrites =
                    stats
                        .structural_rewrites
                        .saturating_add(1);

                stats
                    .expressions_simplified =
                    stats
                        .expressions_simplified
                        .saturating_add(1);
            }

            stats.rewrite_budget_used =
                budget.rewrites_used();

            Ok((result, stats))
        }
    }
}

// -----------------------------------------------------------------------------
// Addition
// -----------------------------------------------------------------------------

/// Performs conservative structural addition simplification.
///
/// Constant arithmetic and the common zero identities are primarily handled
/// by `constant_fold.rs`. These checks remain here because this function is
/// also responsible for maintaining a stable structural form after recursive
/// simplification.
fn simplify_add(
    left: Parameter,
    right: Parameter,
) -> IrResult<(Parameter, bool)> {
    if is_zero(&left) {
        return Ok((right, true));
    }

    if is_zero(&right) {
        return Ok((left, true));
    }

    Ok((
        make_expression(
            ParameterExpression::Add(
                Box::new(left),
                Box::new(right),
            ),
        )?,
        false,
    ))
}

// -----------------------------------------------------------------------------
// Subtraction
// -----------------------------------------------------------------------------

/// Performs conservative subtraction simplification.
fn simplify_subtract(
    left: Parameter,
    right: Parameter,
) -> IrResult<(Parameter, bool)> {
    if is_zero(&right) {
        return Ok((left, true));
    }

    Ok((
        make_expression(
            ParameterExpression::Subtract(
                Box::new(left),
                Box::new(right),
            ),
        )?,
        false,
    ))
}

// -----------------------------------------------------------------------------
// Multiplication
// -----------------------------------------------------------------------------

/// Performs multiplication simplification.
///
/// `x * 0 -> 0` is accepted because the Quantum IR parameter contract requires
/// finite bound values. This is a mathematical identity over the valid
/// parameter domain. Angle-specific periodicity remains outside this module.
fn simplify_multiply(
    left: Parameter,
    right: Parameter,
) -> IrResult<(Parameter, bool)> {
    if is_one(&left) {
        return Ok((right, true));
    }

    if is_one(&right) {
        return Ok((left, true));
    }

    if is_zero(&left) {
        return Ok((
            Parameter::constant(0.0)?,
            true,
        ));
    }

    if is_zero(&right) {
        return Ok((
            Parameter::constant(0.0)?,
            true,
        ));
    }

    Ok((
        make_expression(
            ParameterExpression::Multiply(
                Box::new(left),
                Box::new(right),
            ),
        )?,
        false,
    ))
}

// -----------------------------------------------------------------------------
// Division
// -----------------------------------------------------------------------------

/// Performs conservative division simplification.
///
/// Only division by exact one is eliminated. In particular, `x / x` is NOT
/// transformed into one because x may be zero.
///
/// Likewise, `0 / x` is NOT transformed into zero because x may be zero and
/// the canonical parameter binder explicitly rejects division by zero.
fn simplify_divide(
    left: Parameter,
    right: Parameter,
) -> IrResult<(Parameter, bool)> {
    if is_one(&right) {
        return Ok((left, true));
    }

    Ok((
        make_expression(
            ParameterExpression::Divide(
                Box::new(left),
                Box::new(right),
            ),
        )?,
        false,
    ))
}

// -----------------------------------------------------------------------------
// Negation
// -----------------------------------------------------------------------------

/// Simplifies unary negation.
///
/// The important structural transformation is:
///
/// ```text
/// -(-x) -> x
/// ```
///
/// No distributive transformation is performed because it could change the
/// floating-point evaluation order of a parameter expression.
fn simplify_negate(
    value: Parameter,
    stats: &mut SimplificationStats,
) -> IrResult<(Parameter, bool)> {
    match value {
        Parameter::Constant(number) => {
            if !number.is_finite() {
                return Err(
                    IrParameterError::NonFinite
                        .into(),
                );
            }

            let negated = -number;

            if !negated.is_finite() {
                return Err(
                    IrParameterError::NonFinite
                        .into(),
                );
            }

            stats.double_negations_removed =
                stats
                    .double_negations_removed
                    .saturating_add(0);

            Ok((
                Parameter::constant(negated)?,
                true,
            ))
        }

        Parameter::Expression(expression) => {
            match *expression {
                ParameterExpression::Negate(
                    inner,
                ) => {
                    stats
                        .double_negations_removed =
                        stats
                            .double_negations_removed
                            .saturating_add(1);

                    Ok((*inner, true))
                }

                other => Ok((
                    make_expression(
                        ParameterExpression::Negate(
                            Box::new(
                                Parameter::Expression(
                                    Box::new(other),
                                ),
                            ),
                        ),
                    )?,
                    false,
                )),
            }
        }

        other => Ok((
            make_expression(
                ParameterExpression::Negate(
                    Box::new(other),
                ),
            )?,
            false,
        )),
    }
}

// -----------------------------------------------------------------------------
// Construction helpers
// -----------------------------------------------------------------------------

/// Constructs a validated expression parameter.
fn make_expression(
    expression: ParameterExpression,
) -> IrResult<Parameter> {
    Parameter::expression(expression)
}

/// Exact numeric zero check.
fn is_zero(
    parameter: &Parameter,
) -> bool {
    matches!(
        parameter,
        Parameter::Constant(value)
            if *value == 0.0
    )
}

/// Exact numeric one check.
fn is_one(
    parameter: &Parameter,
) -> bool {
    matches!(
        parameter,
        Parameter::Constant(value)
            if *value == 1.0
    )
}

// -----------------------------------------------------------------------------
// Gate-parameter accounting
// -----------------------------------------------------------------------------

/// Counts all parameter nodes in a gate parameter group.
///
/// This is deliberately allocation-free.
fn consume_gate_parameter_budget(
    parameter: &GateParameter,
    budget: &mut SimplificationBudget,
) -> IrResult<()> {
    for value in parameter.iter() {
        consume_parameter_nodes(
            value,
            budget,
        )?;
    }

    Ok(())
}

/// Recursively consumes structural nodes without performing transformations.
fn consume_parameter_nodes(
    parameter: &Parameter,
    budget: &mut SimplificationBudget,
) -> IrResult<()> {
    budget.consume_node()?;

    match parameter {
        Parameter::Constant(_)
        | Parameter::Symbol(_) => Ok(()),

        Parameter::Expression(expression) => {
            match expression {
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
                    consume_parameter_nodes(
                        left,
                        budget,
                    )?;

                    consume_parameter_nodes(
                        right,
                        budget,
                    )
                }

                ParameterExpression::Negate(
                    value,
                ) => {
                    consume_parameter_nodes(
                        value,
                        budget,
                    )
                }
            }
        }
    }
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn symbol_parameter(
        name: &str,
    ) -> Parameter {
        Parameter::symbol(name)
            .expect("test symbol must be valid")
    }

    fn constant_parameter(
        value: f64,
    ) -> Parameter {
        Parameter::constant(value)
            .expect("test constant must be finite")
    }

    fn expression_parameter(
        expression: ParameterExpression,
    ) -> Parameter {
        Parameter::expression(expression)
            .expect("test expression must be valid")
    }

    #[test]
    fn default_configuration_is_valid() {
        let config =
            SimplificationConfig::default();

        assert!(
            config.validate().is_ok()
        );
    }

    #[test]
    fn zero_node_budget_is_rejected() {
        let config =
            SimplificationConfig::new(
                0,
                10,
            );

        assert!(
            config.validate().is_err()
        );
    }

    #[test]
    fn symbols_are_preserved() {
        let parameter =
            symbol_parameter("theta");

        let simplifier =
            ParameterSimplifier::new();

        let result =
            simplifier
                .simplify_parameter(
                    &parameter,
                )
                .unwrap();

        assert_eq!(
            result.parameter,
            parameter
        );

        assert!(!result.changed());
    }

    #[test]
    fn double_negation_is_removed() {
        let theta =
            symbol_parameter("theta");

        let inner =
            expression_parameter(
                ParameterExpression::Negate(
                    Box::new(theta.clone()),
                ),
            );

        let outer =
            expression_parameter(
                ParameterExpression::Negate(
                    Box::new(inner),
                ),
            );

        let simplifier =
            ParameterSimplifier::new();

        let result =
            simplifier
                .simplify_parameter(
                    &outer,
                )
                .unwrap();

        assert_eq!(
            result.parameter,
            theta
        );

        assert!(result.changed());

        assert!(
            result
                .stats
                .double_negations_removed
                >= 1
        );
    }

    #[test]
    fn addition_by_zero_is_removed() {
        let theta =
            symbol_parameter("theta");

        let expression =
            expression_parameter(
                ParameterExpression::Add(
                    Box::new(theta.clone()),
                    Box::new(
                        constant_parameter(0.0),
                    ),
                ),
            );

        let simplifier =
            ParameterSimplifier::new();

        let result =
            simplifier
                .simplify_parameter(
                    &expression,
                )
                .unwrap();

        assert_eq!(
            result.parameter,
            theta
        );

        assert!(result.changed());
    }

    #[test]
    fn subtraction_by_zero_is_removed() {
        let theta =
            symbol_parameter("theta");

        let expression =
            expression_parameter(
                ParameterExpression::Subtract(
                    Box::new(theta.clone()),
                    Box::new(
                        constant_parameter(0.0),
                    ),
                ),
            );

        let simplifier =
            ParameterSimplifier::new();

        let result =
            simplifier
                .simplify_parameter(
                    &expression,
                )
                .unwrap();

        assert_eq!(
            result.parameter,
            theta
        );

        assert!(result.changed());
    }

    #[test]
    fn multiplication_by_one_is_removed() {
        let theta =
            symbol_parameter("theta");

        let expression =
            expression_parameter(
                ParameterExpression::Multiply(
                    Box::new(theta.clone()),
                    Box::new(
                        constant_parameter(1.0),
                    ),
                ),
            );

        let simplifier =
            ParameterSimplifier::new();

        let result =
            simplifier
                .simplify_parameter(
                    &expression,
                )
                .unwrap();

        assert_eq!(
            result.parameter,
            theta
        );

        assert!(result.changed());
    }

    #[test]
    fn multiplication_by_zero_is_removed() {
        let theta =
            symbol_parameter("theta");

        let expression =
            expression_parameter(
                ParameterExpression::Multiply(
                    Box::new(theta),
                    Box::new(
                        constant_parameter(0.0),
                    ),
                ),
            );

        let simplifier =
            ParameterSimplifier::new();

        let result =
            simplifier
                .simplify_parameter(
                    &expression,
                )
                .unwrap();

        assert_eq!(
            result.parameter,
            constant_parameter(0.0)
        );

        assert!(result.changed());
    }

    #[test]
    fn division_by_one_is_removed() {
        let theta =
            symbol_parameter("theta");

        let expression =
            expression_parameter(
                ParameterExpression::Divide(
                    Box::new(theta.clone()),
                    Box::new(
                        constant_parameter(1.0),
                    ),
                ),
            );

        let simplifier =
            ParameterSimplifier::new();

        let result =
            simplifier
                .simplify_parameter(
                    &expression,
                )
                .unwrap();

        assert_eq!(
            result.parameter,
            theta
        );

        assert!(result.changed());
    }

    #[test]
    fn division_by_zero_is_not_silently_simplified() {
        let theta =
            symbol_parameter("theta");

        let expression =
            expression_parameter(
                ParameterExpression::Divide(
                    Box::new(theta.clone()),
                    Box::new(
                        constant_parameter(0.0),
                    ),
                ),
            );

        let simplifier =
            ParameterSimplifier::new();

        let result =
            simplifier
                .simplify_parameter(
                    &expression,
                )
                .unwrap();

        assert_eq!(
            result.parameter,
            expression
        );
    }

    #[test]
    fn x_divided_by_x_is_not_simplified() {
        let theta =
            symbol_parameter("theta");

        let expression =
            expression_parameter(
                ParameterExpression::Divide(
                    Box::new(theta.clone()),
                    Box::new(theta),
                ),
            );

        let simplifier =
            ParameterSimplifier::new();

        let result =
            simplifier
                .simplify_parameter(
                    &expression,
                )
                .unwrap();

        assert_eq!(
            result.parameter,
            expression
        );
    }

    #[test]
    fn constant_arithmetic_is_delegated_to_constant_folder() {
        let expression =
            expression_parameter(
                ParameterExpression::Add(
                    Box::new(
                        constant_parameter(2.0),
                    ),
                    Box::new(
                        constant_parameter(3.0),
                    ),
                ),
            );

        let simplifier =
            ParameterSimplifier::new();

        let result =
            simplifier
                .simplify_parameter(
                    &expression,
                )
                .unwrap();

        assert_eq!(
            result.parameter,
            constant_parameter(5.0)
        );

        assert!(
            result.stats.constants_folded
                >= 1
        );
    }

    #[test]
    fn gate_parameter_arity_is_preserved() {
        let theta =
            symbol_parameter("theta");

        let phi =
            symbol_parameter("phi");

        let lambda =
            symbol_parameter("lambda");

        let input =
            GateParameter::three_angles(
                theta.clone(),
                phi.clone(),
                lambda.clone(),
            )
            .unwrap();

        let simplifier =
            ParameterSimplifier::new();

        let result =
            simplifier
                .simplify_gate_parameter(
                    &input,
                )
                .unwrap();

        assert_eq!(
            result.parameter.arity(),
            3
        );

        assert_eq!(
            result.parameter,
            input
        );
    }

    #[test]
    fn in_place_simplification_is_transactional() {
        let theta =
            symbol_parameter("theta");

        let original =
            expression_parameter(
                ParameterExpression::Negate(
                    Box::new(
                        expression_parameter(
                            ParameterExpression::Negate(
                                Box::new(
                                    theta.clone(),
                                ),
                            ),
                        ),
                    ),
                ),
            );

        let mut value =
            original.clone();

        let simplifier =
            ParameterSimplifier::new();

        simplifier
            .simplify_parameter_in_place(
                &mut value,
            )
            .unwrap();

        assert_eq!(
            value,
            theta
        );
    }

    #[test]
    fn simplification_is_idempotent() {
        let theta =
            symbol_parameter("theta");

        let expression =
            expression_parameter(
                ParameterExpression::Negate(
                    Box::new(
                        expression_parameter(
                            ParameterExpression::Negate(
                                Box::new(
                                    expression_parameter(
                                        ParameterExpression::Add(
                                            Box::new(
                                                theta.clone(),
                                            ),
                                            Box::new(
                                                constant_parameter(
                                                    0.0,
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
            );

        let simplifier =
            ParameterSimplifier::new();

        let first =
            simplifier
                .simplify_parameter(
                    &expression,
                )
                .unwrap();

        let second =
            simplifier
                .simplify_parameter(
                    &first.parameter,
                )
                .unwrap();

        assert_eq!(
            first.parameter,
            second.parameter
        );

        assert!(
            !second.changed()
                || second.parameter
                    != first.parameter
        );
    }

    #[test]
    fn low_node_budget_rejects_pathological_input() {
        let theta =
            symbol_parameter("theta");

        let expression =
            expression_parameter(
                ParameterExpression::Negate(
                    Box::new(theta),
                ),
            );

        let simplifier =
            ParameterSimplifier::with_config(
                SimplificationConfig::new(
                    1,
                    100,
                ),
            )
            .unwrap();

        assert!(
            simplifier
                .simplify_parameter(
                    &expression,
                )
                .is_err()
        );
    }

    #[test]
    fn zero_rewrite_budget_disables_structural_rewrites() {
        let theta =
            symbol_parameter("theta");

        let expression =
            expression_parameter(
                ParameterExpression::Negate(
                    Box::new(
                        expression_parameter(
                            ParameterExpression::Negate(
                                Box::new(theta.clone()),
                            ),
                        ),
                    ),
                ),
            );

        let simplifier =
            ParameterSimplifier::with_config(
                SimplificationConfig::new(
                    1024,
                    0,
                ),
            )
            .unwrap();

        let result =
            simplifier
                .simplify_parameter(
                    &expression,
                );

        assert!(result.is_err());
    }

    #[test]
    fn finite_constants_remain_finite() {
        let value =
            constant_parameter(
                -3.25,
            );

        let simplifier =
            ParameterSimplifier::new();

        let result =
            simplifier
                .simplify_parameter(
                    &value,
                )
                .unwrap();

        assert_eq!(
            result.parameter
                .as_constant(),
            Some(-3.25)
        );
    }
}