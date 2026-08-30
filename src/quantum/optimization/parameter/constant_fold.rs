//! Zamani Quantum Optimization — Parameter Constant Folding
//!
//! Production-grade compile-time simplification of [`crate::quantum::ir::parameter::Parameter`]
//! and [`crate::quantum::ir::parameter::ParameterExpression`] values.
//!
//! # Architectural role
//!
//! This module belongs to:
//!
//! ```text
//! quantum::ir::parameter
//!          │
//!          ▼
//! optimization::parameter::constant_fold
//!          │
//!          ├── parameter simplification
//!          ├── rotation optimization
//!          ├── gate fusion
//!          ├── phase-polynomial optimization
//!          └── synthesis
//! ```
//!
//! This module is intentionally independent from:
//!
//! - quantum circuits;
//! - quantum gates;
//! - hardware targets;
//! - routing;
//! - scheduling;
//! - QPU execution;
//! - optimization pipelines;
//! - optimization pass registries;
//! - backend-specific cost models.
//!
//! The optimizer operates on the canonical Quantum IR parameter types.
//!
//! # What this pass does
//!
//! The pass performs exact structural constant folding and algebraic
//! simplification of the arithmetic vocabulary currently defined by the
//! Quantum IR:
//!
//! - addition;
//! - subtraction;
//! - multiplication;
//! - division;
//! - unary negation.
//!
//! Examples:
//!
//! ```text
//! 2 + 3        -> 5
//! 7 - 4        -> 3
//! 2 * 3        -> 6
//! 8 / 2        -> 4
//! -(3)         -> -3
//!
//! x + 0        -> x
//! 0 + x        -> x
//! x - 0        -> x
//! x * 1        -> x
//! 1 * x        -> x
//! x * 0        -> 0
//! 0 * x        -> 0
//! x / 1        -> x
//! -(-x)        -> x
//!
//! (2 + 3) * x  -> 5 * x
//! (2 * 3) + 4  -> 10
//! ```
//!
//! The pass deliberately does NOT perform angle-specific reductions such as:
//!
//! ```text
//! theta + 2*pi -> theta
//! 2*pi         -> 0
//! ```
//!
//! because `Parameter` is intentionally a generic scalar parameter
//! representation. Such transformations belong to an angle/phase-aware
//! optimization layer where the consuming gate's periodicity is known.
//!
//! # Numerical safety
//!
//! Constant folding never intentionally creates NaN or infinity.
//!
//! Arithmetic overflow, invalid division, and non-finite results are rejected
//! rather than silently entering the Quantum IR.
//!
//! # Complexity
//!
//! For an expression containing `N` nodes, folding is O(N) time and O(D) stack
//! space where `D` is the expression depth. The existing Quantum IR limits
//! expression depth, preventing unbounded recursive descent.
//!
//! The implementation does not repeatedly clone or stringify the expression
//! during folding. Subtrees are consumed and rebuilt only when necessary.
//!
//! # Integration contract
//!
//! Future `optimization::parameter::mod.rs` should expose this module with:
//!
//! ```text
//! pub mod constant_fold;
//! ```
//!
//! Future optimization passes can use:
//!
//! ```text
//! ConstantFolder::new().fold_parameter(&parameter)
//! ```
//!
//! or, when a mutable parameter already exists:
//!
//! ```text
//! ConstantFolder::new().fold_parameter_in_place(&mut parameter)
//! ```
//!
//! No change to this file should be required when the optimizer pipeline,
//! pass registry, planner, cost model, or verification subsystem is added.
//!
//! Rust compatibility: Rust 1.97 / Rust 1.97.1.
//!
//! Safety: no `unsafe` code.

use crate::quantum::ir::errors::{IrError, IrParameterError, IrResult};
use crate::quantum::ir::parameter::{
    GateParameter,
    Parameter,
    ParameterExpression,
};

/// Default maximum number of expression nodes a single folding invocation
/// will inspect.
///
/// The canonical IR already limits expression depth. This additional budget
/// protects callers that construct the public expression enum directly rather
/// than through the validating constructors.
///
/// The value is intentionally large. Production callers can configure a
/// smaller budget when processing untrusted or resource-constrained input.
pub const DEFAULT_MAX_FOLD_NODES: usize = 1_048_576;

/// Configuration for constant folding.
///
/// The configuration is independent from the optimizer-wide `OptimizationConfig`
/// so this file can be completed and tested before the broader optimization
/// configuration infrastructure exists.
///
/// The future optimizer configuration can map its resource budget directly to
/// this type without changing the folding algorithm.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConstantFoldConfig {
    /// Maximum number of parameter-expression nodes visited by one operation.
    ///
    /// A value of zero is rejected by [`ConstantFoldConfig::validate`].
    pub max_nodes: usize,
}

impl Default for ConstantFoldConfig {
    fn default() -> Self {
        Self {
            max_nodes: DEFAULT_MAX_FOLD_NODES,
        }
    }
}

impl ConstantFoldConfig {
    /// Creates a configuration with the supplied node budget.
    pub const fn new(max_nodes: usize) -> Self {
        Self { max_nodes }
    }

    /// Validates the configuration.
    pub fn validate(self) -> IrResult<()> {
        if self.max_nodes == 0 {
            return Err(IrParameterError::InvalidExpression.into());
        }

        Ok(())
    }
}

/// Result statistics for one constant-folding invocation.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ConstantFoldStats {
    /// Number of expression nodes inspected.
    pub nodes_visited: usize,

    /// Number of arithmetic expressions whose children were constant-folded.
    pub constants_folded: usize,

    /// Number of algebraic identities applied.
    pub identities_applied: usize,

    /// Number of unary negations simplified.
    pub negations_simplified: usize,

    /// Number of divisions simplified.
    pub divisions_simplified: usize,

    /// Number of expressions whose structural representation became simpler.
    pub expressions_simplified: usize,
}

impl ConstantFoldStats {
    /// Returns whether the folding operation changed the parameter.
    pub const fn changed(self) -> bool {
        self.constants_folded > 0
            || self.identities_applied > 0
            || self.negations_simplified > 0
            || self.divisions_simplified > 0
            || self.expressions_simplified > 0
    }

    /// Adds another statistics record to this record.
    fn accumulate(&mut self, other: Self) {
        self.nodes_visited = self
            .nodes_visited
            .saturating_add(other.nodes_visited);

        self.constants_folded = self
            .constants_folded
            .saturating_add(other.constants_folded);

        self.identities_applied = self
            .identities_applied
            .saturating_add(other.identities_applied);

        self.negations_simplified = self
            .negations_simplified
            .saturating_add(other.negations_simplified);

        self.divisions_simplified = self
            .divisions_simplified
            .saturating_add(other.divisions_simplified);

        self.expressions_simplified = self
            .expressions_simplified
            .saturating_add(other.expressions_simplified);
    }
}

/// Output of folding a single parameter.
#[derive(Debug, Clone, PartialEq)]
pub struct FoldedParameter {
    /// Simplified parameter.
    pub parameter: Parameter,

    /// Statistics describing the transformation.
    pub stats: ConstantFoldStats,
}

impl FoldedParameter {
    /// Returns whether the parameter changed.
    pub const fn changed(&self) -> bool {
        self.stats.changed()
    }
}

/// Output of folding a gate parameter group.
#[derive(Debug, Clone, PartialEq)]
pub struct FoldedGateParameter {
    /// Simplified gate parameter group.
    pub parameter: GateParameter,

    /// Aggregate folding statistics.
    pub stats: ConstantFoldStats,
}

impl FoldedGateParameter {
    /// Returns whether any parameter changed.
    pub const fn changed(&self) -> bool {
        self.stats.changed()
    }
}

/// Production constant-folding engine.
///
/// The folder is stateless apart from its resource configuration, making it
/// safe to reuse for many parameters and circuits. There is no global mutable
/// state, cache, random state, or backend dependency.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConstantFolder {
    config: ConstantFoldConfig,
}

impl Default for ConstantFolder {
    fn default() -> Self {
        Self::new()
    }
}

impl ConstantFolder {
    /// Creates a folder with production defaults.
    pub const fn new() -> Self {
        Self {
            config: ConstantFoldConfig::new(DEFAULT_MAX_FOLD_NODES),
        }
    }

    /// Creates a folder with an explicit configuration.
    ///
    /// The configuration is validated before any folding operation.
    pub fn with_config(config: ConstantFoldConfig) -> IrResult<Self> {
        config.validate()?;

        Ok(Self { config })
    }

    /// Returns the current configuration.
    pub const fn config(&self) -> ConstantFoldConfig {
        self.config
    }

    /// Folds one canonical Quantum IR parameter.
    ///
    /// This is the primary integration API for later optimization passes.
    pub fn fold_parameter(
        &self,
        parameter: &Parameter,
    ) -> IrResult<FoldedParameter> {
        self.config.validate()?;
        parameter.validate()?;

        let mut budget = NodeBudget::new(self.config.max_nodes);

        let (parameter, stats) =
            fold_parameter_recursive(parameter, &mut budget)?;

        parameter.validate()?;

        Ok(FoldedParameter {
            parameter,
            stats,
        })
    }

    /// Folds one parameter in place.
    ///
    /// The original value is replaced only after the entire transformation
    /// succeeds. Therefore a failed fold leaves the caller's parameter
    /// untouched.
    pub fn fold_parameter_in_place(
        &self,
        parameter: &mut Parameter,
    ) -> IrResult<ConstantFoldStats> {
        let folded = self.fold_parameter(parameter)?;

        *parameter = folded.parameter;

        Ok(folded.stats)
    }

    /// Folds every parameter in a [`GateParameter`].
    ///
    /// Gate arity and representation are preserved exactly.
    pub fn fold_gate_parameter(
        &self,
        parameter: &GateParameter,
    ) -> IrResult<FoldedGateParameter> {
        self.config.validate()?;
        parameter.validate()?;

        let mut budget = NodeBudget::new(self.config.max_nodes);

        let (parameter, stats) =
            match parameter {
                GateParameter::Angle(value) => {
                    let (value, stats) =
                        fold_parameter_recursive(value, &mut budget)?;

                    (
                        GateParameter::angle(value)?,
                        stats,
                    )
                }

                GateParameter::TwoAngles {
                    theta,
                    phi,
                } => {
                    let (theta, mut theta_stats) =
                        fold_parameter_recursive(theta, &mut budget)?;

                    let (phi, phi_stats) =
                        fold_parameter_recursive(phi, &mut budget)?;

                    theta_stats.accumulate(phi_stats);

                    (
                        GateParameter::two_angles(
                            theta,
                            phi,
                        )?,
                        theta_stats,
                    )
                }

                GateParameter::ThreeAngles {
                    theta,
                    phi,
                    lambda,
                } => {
                    let (theta, mut stats) =
                        fold_parameter_recursive(theta, &mut budget)?;

                    let (phi, phi_stats) =
                        fold_parameter_recursive(phi, &mut budget)?;

                    stats.accumulate(phi_stats);

                    let (lambda, lambda_stats) =
                        fold_parameter_recursive(
                            lambda,
                            &mut budget,
                        )?;

                    stats.accumulate(lambda_stats);

                    (
                        GateParameter::three_angles(
                            theta,
                            phi,
                            lambda,
                        )?,
                        stats,
                    )
                }
            };

        parameter.validate()?;

        Ok(FoldedGateParameter {
            parameter,
            stats,
        })
    }

    /// Folds a gate parameter group in place.
    pub fn fold_gate_parameter_in_place(
        &self,
        parameter: &mut GateParameter,
    ) -> IrResult<ConstantFoldStats> {
        let folded = self.fold_gate_parameter(parameter)?;

        *parameter = folded.parameter;

        Ok(folded.stats)
    }
}

/// Tracks the amount of expression work performed by the folder.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct NodeBudget {
    remaining: usize,
}

impl NodeBudget {
    const fn new(limit: usize) -> Self {
        Self { remaining: limit }
    }

    fn consume(&mut self) -> IrResult<()> {
        if self.remaining == 0 {
            return Err(
                IrError::from(IrParameterError::InvalidExpression)
            );
        }

        self.remaining -= 1;

        Ok(())
    }
}

/// Recursively folds one parameter.
///
/// The function consumes no caller-owned data and returns a new parameter,
/// which makes transactional in-place integration possible.
fn fold_parameter_recursive(
    parameter: &Parameter,
    budget: &mut NodeBudget,
) -> IrResult<(Parameter, ConstantFoldStats)> {
    budget.consume()?;

    let mut stats = ConstantFoldStats {
        nodes_visited: 1,
        ..ConstantFoldStats::default()
    };

    match parameter {
        Parameter::Constant(value) => {
            if !value.is_finite() {
                return Err(IrParameterError::NonFinite.into());
            }

            let normalized = normalize_zero(*value);

            Ok((
                Parameter::Constant(normalized),
                stats,
            ))
        }

        Parameter::Symbol(name) => {
            let normalized =
                Parameter::symbol(name.clone())?;

            Ok((normalized, stats))
        }

        Parameter::Expression(expression) => {
            let (folded, expression_stats) =
                fold_expression(expression, budget)?;

            stats.accumulate(expression_stats);

            Ok((folded, stats))
        }
    }
}

/// Recursively folds one parameter expression.
///
/// Child expressions are folded before the parent expression is evaluated.
/// This gives the pass one-pass cascading behavior:
///
/// ```text
/// ((2 + 3) * (4 - 1))
///          │
///          ▼
/// (5 * 3)
///          │
///          ▼
/// 15
/// ```
fn fold_expression(
    expression: &ParameterExpression,
    budget: &mut NodeBudget,
) -> IrResult<(Parameter, ConstantFoldStats)> {
    let mut stats = ConstantFoldStats::default();

    match expression {
        ParameterExpression::Add(left, right) => {
            let (left, left_stats) =
                fold_parameter_recursive(left, budget)?;

            let (right, right_stats) =
                fold_parameter_recursive(right, budget)?;

            stats.accumulate(left_stats);
            stats.accumulate(right_stats);

            fold_add(left, right, &mut stats)
        }

        ParameterExpression::Subtract(left, right) => {
            let (left, left_stats) =
                fold_parameter_recursive(left, budget)?;

            let (right, right_stats) =
                fold_parameter_recursive(right, budget)?;

            stats.accumulate(left_stats);
            stats.accumulate(right_stats);

            fold_subtract(left, right, &mut stats)
        }

        ParameterExpression::Multiply(left, right) => {
            let (left, left_stats) =
                fold_parameter_recursive(left, budget)?;

            let (right, right_stats) =
                fold_parameter_recursive(right, budget)?;

            stats.accumulate(left_stats);
            stats.accumulate(right_stats);

            fold_multiply(left, right, &mut stats)
        }

        ParameterExpression::Divide(left, right) => {
            let (left, left_stats) =
                fold_parameter_recursive(left, budget)?;

            let (right, right_stats) =
                fold_parameter_recursive(right, budget)?;

            stats.accumulate(left_stats);
            stats.accumulate(right_stats);

            fold_divide(left, right, &mut stats)
        }

        ParameterExpression::Negate(value) => {
            let (value, value_stats) =
                fold_parameter_recursive(value, budget)?;

            stats.accumulate(value_stats);

            fold_negate(value, &mut stats)
        }
    }
}

/// Folds addition.
fn fold_add(
    left: Parameter,
    right: Parameter,
    stats: &mut ConstantFoldStats,
) -> IrResult<(Parameter, ConstantFoldStats)> {
    if let (Some(left), Some(right)) =
        (constant_value(&left), constant_value(&right))
    {
        let value = checked_add(left, right)?;

        stats.constants_folded += 1;

        return Ok((
            Parameter::Constant(value),
            *stats,
        ));
    }

    if is_exact_zero(&left) {
        stats.identities_applied += 1;
        stats.expressions_simplified += 1;

        return Ok((right, *stats));
    }

    if is_exact_zero(&right) {
        stats.identities_applied += 1;
        stats.expressions_simplified += 1;

        return Ok((left, *stats));
    }

    Ok((
        expression_parameter(
            ParameterExpression::Add(
                Box::new(left),
                Box::new(right),
            ),
        )?,
        *stats,
    ))
}

/// Folds subtraction.
fn fold_subtract(
    left: Parameter,
    right: Parameter,
    stats: &mut ConstantFoldStats,
) -> IrResult<(Parameter, ConstantFoldStats)> {
    if let (Some(left), Some(right)) =
        (constant_value(&left), constant_value(&right))
    {
        let value = checked_sub(left, right)?;

        stats.constants_folded += 1;

        return Ok((
            Parameter::Constant(value),
            *stats,
        ));
    }

    if is_exact_zero(&right) {
        stats.identities_applied += 1;
        stats.expressions_simplified += 1;

        return Ok((left, *stats));
    }

    if parameters_exactly_equal(&left, &right) {
        stats.identities_applied += 1;
        stats.expressions_simplified += 1;

        return Ok((
            Parameter::Constant(0.0),
            *stats,
        ));
    }

    Ok((
        expression_parameter(
            ParameterExpression::Subtract(
                Box::new(left),
                Box::new(right),
            ),
        )?,
        *stats,
    ))
}

/// Folds multiplication.
fn fold_multiply(
    left: Parameter,
    right: Parameter,
    stats: &mut ConstantFoldStats,
) -> IrResult<(Parameter, ConstantFoldStats)> {
    if let (Some(left), Some(right)) =
        (constant_value(&left), constant_value(&right))
    {
        let value = checked_mul(left, right)?;

        stats.constants_folded += 1;

        return Ok((
            Parameter::Constant(value),
            *stats,
        ));
    }

    if is_exact_zero(&left) || is_exact_zero(&right) {
        stats.identities_applied += 1;
        stats.expressions_simplified += 1;

        return Ok((
            Parameter::Constant(0.0),
            *stats,
        ));
    }

    if is_exact_one(&left) {
        stats.identities_applied += 1;
        stats.expressions_simplified += 1;

        return Ok((right, *stats));
    }

    if is_exact_one(&right) {
        stats.identities_applied += 1;
        stats.expressions_simplified += 1;

        return Ok((left, *stats));
    }

    Ok((
        expression_parameter(
            ParameterExpression::Multiply(
                Box::new(left),
                Box::new(right),
            ),
        )?,
        *stats,
    ))
}

/// Folds division.
fn fold_divide(
    left: Parameter,
    right: Parameter,
    stats: &mut ConstantFoldStats,
) -> IrResult<(Parameter, ConstantFoldStats)> {
    if let Some(denominator) =
        constant_value(&right)
    {
        if denominator == 0.0 {
            return Err(
                IrParameterError::InvalidExpression.into()
            );
        }
    }

    if let (Some(left), Some(right)) =
        (constant_value(&left), constant_value(&right))
    {
        let value = checked_div(left, right)?;

        stats.constants_folded += 1;
        stats.divisions_simplified += 1;

        return Ok((
            Parameter::Constant(value),
            *stats,
        ));
    }

    if is_exact_one(&right) {
        stats.identities_applied += 1;
        stats.divisions_simplified += 1;
        stats.expressions_simplified += 1;

        return Ok((left, *stats));
    }

    Ok((
        expression_parameter(
            ParameterExpression::Divide(
                Box::new(left),
                Box::new(right),
            ),
        )?,
        *stats,
    ))
}

/// Folds unary negation.
fn fold_negate(
    value: Parameter,
    stats: &mut ConstantFoldStats,
) -> IrResult<(Parameter, ConstantFoldStats)> {
    if let Some(value) = constant_value(&value) {
        let value = checked_neg(value)?;

        stats.constants_folded += 1;
        stats.negations_simplified += 1;

        return Ok((
            Parameter::Constant(value),
            *stats,
        ));
    }

    if let Parameter::Expression(expression) = &value {
        if let ParameterExpression::Negate(inner) =
            expression.as_ref()
        {
            stats.identities_applied += 1;
            stats.negations_simplified += 1;
            stats.expressions_simplified += 1;

            return Ok((
                inner.as_ref().clone(),
                *stats,
            ));
        }
    }

    Ok((
        expression_parameter(
            ParameterExpression::Negate(
                Box::new(value),
            ),
        )?,
        *stats,
    ))
}

/// Creates a validated expression parameter.
fn expression_parameter(
    expression: ParameterExpression,
) -> IrResult<Parameter> {
    Parameter::expression(expression)
}

/// Returns the concrete value of a constant parameter.
fn constant_value(
    parameter: &Parameter,
) -> Option<f64> {
    match parameter {
        Parameter::Constant(value) => Some(*value),

        Parameter::Symbol(_)
        | Parameter::Expression(_) => None,
    }
}

/// Returns true for exactly +0.0 or -0.0.
///
/// IEEE-754 treats both as equal numerically. Constant folding canonicalizes
/// both to +0.0.
fn is_exact_zero(parameter: &Parameter) -> bool {
    matches!(
        parameter,
        Parameter::Constant(value)
            if *value == 0.0
    )
}

/// Returns true for exactly +1.0.
fn is_exact_one(parameter: &Parameter) -> bool {
    matches!(
        parameter,
        Parameter::Constant(value)
            if *value == 1.0
    )
}

/// Structural parameter equality.
///
/// This intentionally uses exact structural equality rather than floating
/// point tolerance. Constant folding is an exact compiler transformation;
// approximate equivalence belongs to a separate approximation-aware pass.
fn parameters_exactly_equal(
    left: &Parameter,
    right: &Parameter,
) -> bool {
    left == right
}

/// Canonicalizes both +0.0 and -0.0 to +0.0.
fn normalize_zero(value: f64) -> f64 {
    if value == 0.0 {
        0.0
    } else {
        value
    }
}

/// Performs checked finite floating-point addition.
fn checked_add(left: f64, right: f64) -> IrResult<f64> {
    let value = left + right;

    if value.is_finite() {
        Ok(normalize_zero(value))
    } else {
        Err(IrParameterError::NonFinite.into())
    }
}

/// Performs checked finite floating-point subtraction.
fn checked_sub(left: f64, right: f64) -> IrResult<f64> {
    let value = left - right;

    if value.is_finite() {
        Ok(normalize_zero(value))
    } else {
        Err(IrParameterError::NonFinite.into())
    }
}

/// Performs checked finite floating-point multiplication.
fn checked_mul(left: f64, right: f64) -> IrResult<f64> {
    let value = left * right;

    if value.is_finite() {
        Ok(normalize_zero(value))
    } else {
        Err(IrParameterError::NonFinite.into())
    }
}

/// Performs checked finite floating-point division.
fn checked_div(left: f64, right: f64) -> IrResult<f64> {
    if right == 0.0 {
        return Err(
            IrParameterError::InvalidExpression.into()
        );
    }

    let value = left / right;

    if value.is_finite() {
        Ok(normalize_zero(value))
    } else {
        Err(IrParameterError::NonFinite.into())
    }
}

/// Performs checked finite unary negation.
fn checked_neg(value: f64) -> IrResult<f64> {
    let result = -value;

    if result.is_finite() {
        Ok(normalize_zero(result))
    } else {
        Err(IrParameterError::NonFinite.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn constant(value: f64) -> Parameter {
        Parameter::constant(value)
            .expect("test constant must be finite")
    }

    fn symbol(name: &str) -> Parameter {
        Parameter::symbol(name)
            .expect("test symbol must be valid")
    }

    fn expression(
        expression: ParameterExpression,
    ) -> Parameter {
        Parameter::expression(expression)
            .expect("test expression must be valid")
    }

    fn add(
        left: Parameter,
        right: Parameter,
    ) -> Parameter {
        expression(
            ParameterExpression::Add(
                Box::new(left),
                Box::new(right),
            ),
        )
    }

    fn subtract(
        left: Parameter,
        right: Parameter,
    ) -> Parameter {
        expression(
            ParameterExpression::Subtract(
                Box::new(left),
                Box::new(right),
            ),
        )
    }

    fn multiply(
        left: Parameter,
        right: Parameter,
    ) -> Parameter {
        expression(
            ParameterExpression::Multiply(
                Box::new(left),
                Box::new(right),
            ),
        )
    }

    fn divide(
        left: Parameter,
        right: Parameter,
    ) -> Parameter {
        expression(
            ParameterExpression::Divide(
                Box::new(left),
                Box::new(right),
            ),
        )
    }

    fn negate(
        value: Parameter,
    ) -> Parameter {
        expression(
            ParameterExpression::Negate(
                Box::new(value),
            ),
        )
    }

    #[test]
    fn folds_addition() {
        let parameter =
            add(constant(2.0), constant(3.0));

        let result =
            ConstantFolder::new()
                .fold_parameter(&parameter)
                .expect("folding should succeed");

        assert_eq!(
            result.parameter,
            constant(5.0)
        );

        assert!(result.changed());
    }

    #[test]
    fn folds_subtraction() {
        let parameter =
            subtract(constant(7.0), constant(4.0));

        let result =
            ConstantFolder::new()
                .fold_parameter(&parameter)
                .expect("folding should succeed");

        assert_eq!(
            result.parameter,
            constant(3.0)
        );
    }

    #[test]
    fn folds_multiplication() {
        let parameter =
            multiply(constant(2.0), constant(3.0));

        let result =
            ConstantFolder::new()
                .fold_parameter(&parameter)
                .expect("folding should succeed");

        assert_eq!(
            result.parameter,
            constant(6.0)
        );
    }

    #[test]
    fn folds_division() {
        let parameter =
            divide(constant(8.0), constant(2.0));

        let result =
            ConstantFolder::new()
                .fold_parameter(&parameter)
                .expect("folding should succeed");

        assert_eq!(
            result.parameter,
            constant(4.0)
        );
    }

    #[test]
    fn folds_negation() {
        let parameter =
            negate(constant(3.0));

        let result =
            ConstantFolder::new()
                .fold_parameter(&parameter)
                .expect("folding should succeed");

        assert_eq!(
            result.parameter,
            constant(-3.0)
        );
    }

    #[test]
    fn folds_deep_constant_expression_in_one_invocation() {
        let parameter = multiply(
            add(constant(2.0), constant(3.0)),
            subtract(constant(10.0), constant(4.0)),
        );

        let result =
            ConstantFolder::new()
                .fold_parameter(&parameter)
                .expect("folding should succeed");

        assert_eq!(
            result.parameter,
            constant(30.0)
        );
    }

    #[test]
    fn removes_additive_zero() {
        let parameter =
            add(symbol("theta"), constant(0.0));

        let result =
            ConstantFolder::new()
                .fold_parameter(&parameter)
                .expect("folding should succeed");

        assert_eq!(
            result.parameter,
            symbol("theta")
        );
    }

    #[test]
    fn removes_zero_plus_symbol() {
        let parameter =
            add(constant(0.0), symbol("theta"));

        let result =
            ConstantFolder::new()
                .fold_parameter(&parameter)
                .expect("folding should succeed");

        assert_eq!(
            result.parameter,
            symbol("theta")
        );
    }

    #[test]
    fn removes_subtractive_zero() {
        let parameter =
            subtract(symbol("theta"), constant(0.0));

        let result =
            ConstantFolder::new()
                .fold_parameter(&parameter)
                .expect("folding should succeed");

        assert_eq!(
            result.parameter,
            symbol("theta")
        );
    }

    #[test]
    fn removes_multiplicative_one() {
        let parameter =
            multiply(symbol("theta"), constant(1.0));

        let result =
            ConstantFolder::new()
                .fold_parameter(&parameter)
                .expect("folding should succeed");

        assert_eq!(
            result.parameter,
            symbol("theta")
        );
    }

    #[test]
    fn removes_left_multiplicative_one() {
        let parameter =
            multiply(constant(1.0), symbol("theta"));

        let result =
            ConstantFolder::new()
                .fold_parameter(&parameter)
                .expect("folding should succeed");

        assert_eq!(
            result.parameter,
            symbol("theta")
        );
    }

    #[test]
    fn folds_symbol_times_zero() {
        let parameter =
            multiply(symbol("theta"), constant(0.0));

        let result =
            ConstantFolder::new()
                .fold_parameter(&parameter)
                .expect("folding should succeed");

        assert_eq!(
            result.parameter,
            constant(0.0)
        );
    }

    #[test]
    fn folds_zero_times_symbol() {
        let parameter =
            multiply(constant(0.0), symbol("theta"));

        let result =
            ConstantFolder::new()
                .fold_parameter(&parameter)
                .expect("folding should succeed");

        assert_eq!(
            result.parameter,
            constant(0.0)
        );
    }

    #[test]
    fn removes_division_by_one() {
        let parameter =
            divide(symbol("theta"), constant(1.0));

        let result =
            ConstantFolder::new()
                .fold_parameter(&parameter)
                .expect("folding should succeed");

        assert_eq!(
            result.parameter,
            symbol("theta")
        );
    }

    #[test]
    fn folds_x_minus_x() {
        let theta = symbol("theta");

        let parameter =
            subtract(theta.clone(), theta);

        let result =
            ConstantFolder::new()
                .fold_parameter(&parameter)
                .expect("folding should succeed");

        assert_eq!(
            result.parameter,
            constant(0.0)
        );
    }

    #[test]
    fn folds_double_negation() {
        let theta = symbol("theta");

        let parameter =
            negate(negate(theta.clone()));

        let result =
            ConstantFolder::new()
                .fold_parameter(&parameter)
                .expect("folding should succeed");

        assert_eq!(
            result.parameter,
            theta
        );
    }

    #[test]
    fn preserves_symbolic_expression() {
        let parameter =
            add(symbol("theta"), symbol("phi"));

        let result =
            ConstantFolder::new()
                .fold_parameter(&parameter)
                .expect("folding should succeed");

        assert_eq!(
            result.parameter,
            parameter
        );
    }

    #[test]
    fn partially_folds_symbolic_expression() {
        let parameter =
            add(
                multiply(
                    constant(2.0),
                    constant(3.0),
                ),
                symbol("theta"),
            );

        let expected =
            add(constant(6.0), symbol("theta"));

        let result =
            ConstantFolder::new()
                .fold_parameter(&parameter)
                .expect("folding should succeed");

        assert_eq!(
            result.parameter,
            expected
        );
    }

    #[test]
    fn folds_all_parameters_in_two_angle_gate_parameter() {
        let parameter =
            GateParameter::two_angles(
                add(constant(2.0), constant(3.0)),
                multiply(
                    constant(4.0),
                    constant(5.0),
                ),
            )
            .expect("test gate parameter must be valid");

        let result =
            ConstantFolder::new()
                .fold_gate_parameter(&parameter)
                .expect("folding should succeed");

        let expected =
            GateParameter::two_angles(
                constant(5.0),
                constant(20.0),
            )
            .expect("expected gate parameter must be valid");

        assert_eq!(
            result.parameter,
            expected
        );
    }

    #[test]
    fn preserves_gate_parameter_arity() {
        let parameter =
            GateParameter::three_angles(
                add(constant(1.0), constant(2.0)),
                symbol("phi"),
                negate(constant(3.0)),
            )
            .expect("test gate parameter must be valid");

        let result =
            ConstantFolder::new()
                .fold_gate_parameter(&parameter)
                .expect("folding should succeed");

        assert_eq!(
            result.parameter.arity(),
            3
        );
    }

    #[test]
    fn rejects_division_by_zero() {
        let parameter =
            divide(constant(1.0), constant(0.0));

        let result =
            ConstantFolder::new()
                .fold_parameter(&parameter);

        assert!(result.is_err());
    }

    #[test]
    fn rejects_non_finite_constant_constructed_directly() {
        let parameter =
            Parameter::Constant(f64::INFINITY);

        let result =
            ConstantFolder::new()
                .fold_parameter(&parameter);

        assert!(matches!(
            result,
            Err(IrError::Parameter(
                IrParameterError::NonFinite
            ))
        ));
    }

    #[test]
    fn rejects_constant_overflow() {
        let parameter =
            multiply(
                constant(f64::MAX),
                constant(2.0),
            );

        let result =
            ConstantFolder::new()
                .fold_parameter(&parameter);

        assert!(matches!(
            result,
            Err(IrError::Parameter(
                IrParameterError::NonFinite
            ))
        ));
    }

    #[test]
    fn canonicalizes_negative_zero() {
        let parameter =
            constant(-0.0);

        let result =
            ConstantFolder::new()
                .fold_parameter(&parameter)
                .expect("folding should succeed");

        assert_eq!(
            result.parameter,
            constant(0.0)
        );
    }

    #[test]
    fn in_place_folding_is_transactional_on_error() {
        let original =
            multiply(
                constant(f64::MAX),
                constant(2.0),
            );

        let mut parameter =
            original.clone();

        let result =
            ConstantFolder::new()
                .fold_parameter_in_place(&mut parameter);

        assert!(result.is_err());
        assert_eq!(parameter, original);
    }

    #[test]
    fn node_budget_is_enforced() {
        let parameter =
            add(
                constant(1.0),
                constant(2.0),
            );

        let folder =
            ConstantFolder::with_config(
                ConstantFoldConfig::new(1),
            )
            .expect("configuration should be valid");

        let result =
            folder.fold_parameter(&parameter);

        assert!(result.is_err());
    }

    #[test]
    fn zero_node_budget_is_rejected() {
        let result =
            ConstantFolder::with_config(
                ConstantFoldConfig::new(0),
            );

        assert!(result.is_err());
    }

    #[test]
    fn folding_is_idempotent() {
        let parameter =
            multiply(
                add(constant(2.0), constant(3.0)),
                symbol("theta"),
            );

        let folder =
            ConstantFolder::new();

        let first =
            folder
                .fold_parameter(&parameter)
                .expect("first fold should succeed")
                .parameter;

        let second =
            folder
                .fold_parameter(&first)
                .expect("second fold should succeed")
                .parameter;

        assert_eq!(first, second);
    }

    #[test]
    fn does_not_apply_approximate_equality() {
        let left =
            constant(1.0);

        let right =
            constant(
                1.0 + f64::EPSILON,
            );

        let parameter =
            subtract(left, right);

        let result =
            ConstantFolder::new()
                .fold_parameter(&parameter)
                .expect("folding should succeed");

        assert_eq!(
            result.parameter,
            constant(-f64::EPSILON)
        );
    }
}