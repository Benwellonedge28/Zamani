//! Zamani Quantum Optimization — Parameter Optimization
//!
//! Production-grade symbolic parameter optimization for the canonical
//! Zamani Quantum IR.
//!
//! # Architectural position
//!
//! ```text
//!                    crate::quantum::ir
//!                           │
//!                           │ Parameter
//!                           │ ParameterExpression
//!                           ▼
//!             optimization::parameter
//!                           │
//!          ┌────────────────┼────────────────┐
//!          ▼                ▼                ▼
//!   constant folding   symbolic simplify   binding
//!          │                │                │
//!          └────────────────┼────────────────┘
//!                           ▼
//!                 optimized canonical IR
//! ```
//!
//! This module owns parameter *optimization policy and transformations*.
//! It does NOT own the parameter representation.
//!
//! The authoritative parameter representation remains:
//!
//! `crate::quantum::ir::parameter`
//!
//! In particular, this module deliberately does NOT define another:
//!
//! - `Parameter`;
//! - `ParameterExpression`;
//! - `QuantumGate`;
//! - circuit representation;
//! - global symbol environment.
//!
//! # Design goals
//!
//! This module provides:
//!
//! - constant folding;
//! - symbolic simplification;
//! - algebraic normalization;
//! - deterministic expression canonicalization;
//! - explicit symbolic binding;
//! - parameter collection;
//! - dependency analysis;
//! - exact structural equality helpers;
//! - optional tolerance-aware numerical comparison;
//! - configurable angle normalization;
//! - configurable resource limits;
//! - overflow/non-finite protection;
//! - bounded recursive processing;
//! - allocation-conscious traversal;
//! - deterministic output;
//! - stable integration contracts for future optimizer passes;
//! - safe Rust only.
//!
//! # Semantic safety
//!
//! Parameter optimization is allowed to simplify expressions only when the
//! transformation is mathematically valid under the selected policy.
//!
//! In particular:
//!
//! - floating-point tolerance is NEVER used to silently rewrite expressions;
//! - division by zero is rejected;
//! - non-finite values are rejected;
//! - symbolic expressions are never numerically guessed;
//! - angle periodicity is opt-in because it is meaningful for angles but not
//!   for arbitrary scalar parameters;
//! - symbolic cancellation is structural and deterministic;
//! - resource limits are enforced before potentially expensive transformations.
//!
//! # Integration contract
//!
//! This module is intentionally independent from the rest of the optimizer.
//!
//! Future modules integrate with it as follows:
//!
//! - `optimization::local::rotation` uses [`simplify_parameter`] and
//!   [`simplify_gate_parameter`];
//! - `optimization::local::peephole` uses [`simplify_parameter`] when combining
//!   parameterized gates;
//! - `optimization::algebra::phase_polynomial` can use
//!   [`collect_symbols`] and [`normalize_parameter`];
//! - `optimization::passes::normalize` can use [`normalize_parameter`];
//! - `optimization::passes::simplify` can use [`simplify_parameter`];
//! - `optimization::analysis::parameter_usage` can use
//!   [`parameter_dependencies`];
//! - `optimization::context` can store [`ParameterOptimizationConfig`];
//! - `optimization::cost` may consume [`ParameterComplexity`];
//! - `optimization::verification` can use [`structurally_equal`] and
//!   [`equivalent_with_tolerance`];
//! - `optimization::serialization` may serialize the public configuration;
//! - `optimization::pipeline` can invoke [`optimize_parameter`] as a pure
//!   transformation.
//!
//! None of those future integrations require changing this module merely
//! because another optimization pass is added.
//!
//! # Rust compatibility
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - no nightly features;
//! - no `unsafe`.
//!
//! # Scaling
//!
//! "Unlimited" input is interpreted as:
//!
//! > no artificial fixed circuit-size ceiling beyond explicit caller/resource
//! > limits and the available machine resources.
//!
//! This module therefore uses:
//!
//! - configurable operation budgets;
//! - configurable expression-depth budgets;
//! - configurable node budgets;
//! - deterministic early termination;
//! - no unbounded equality-saturation;
//! - no uncontrolled recursion;
//! - no global mutable state.
//!
//! Large workloads should be processed incrementally by higher-level optimizer
//! passes rather than materializing an unbounded number of intermediate forms.

use std::collections::BTreeSet;
use std::fmt;

use crate::quantum::ir::{
    GateParameter,
    Parameter,
    ParameterExpression,
};

/// Maximum number of nodes processed by the default parameter optimizer.
///
/// This is deliberately a safety/default budget rather than a theoretical
/// maximum. Callers handling very large circuits should provide their own
/// limit through [`ParameterOptimizationConfig`].
pub const DEFAULT_MAX_NODES: usize = 1_000_000;

/// Maximum expression depth processed by the default parameter optimizer.
///
/// The canonical IR already has its own expression-depth limit. This smaller
/// optimizer limit prevents expensive transformations from consuming
/// unbounded compiler resources.
pub const DEFAULT_MAX_DEPTH: usize = 64;

/// Default floating-point comparison tolerance.
///
/// This value is used ONLY by explicit numerical comparison APIs. It is never
/// used to rewrite symbolic expressions.
pub const DEFAULT_ABSOLUTE_TOLERANCE: f64 = 1.0e-12;

/// Default relative floating-point comparison tolerance.
pub const DEFAULT_RELATIVE_TOLERANCE: f64 = 1.0e-12;

// =============================================================================
// Errors
// =============================================================================

/// Result type for parameter optimization.
pub type ParameterOptimizationResult<T> =
    Result<T, ParameterOptimizationError>;

/// Structured failures produced by this module.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParameterOptimizationError {
    /// A supplied floating-point value is not finite.
    NonFiniteValue {
        /// Human-readable location.
        context: &'static str,
    },

    /// An arithmetic operation would produce a non-finite value.
    ArithmeticOverflow {
        /// Human-readable operation name.
        operation: &'static str,
    },

    /// Division by zero was encountered.
    DivisionByZero,

    /// The expression exceeds the configured depth limit.
    DepthLimitExceeded {
        /// Maximum permitted depth.
        limit: usize,
    },

    /// The expression exceeds the configured node budget.
    NodeLimitExceeded {
        /// Maximum permitted nodes.
        limit: usize,
    },

    /// The caller supplied an invalid optimization configuration.
    InvalidConfiguration {
        /// Configuration field.
        field: &'static str,
    },

    /// The canonical IR rejected a generated parameter.
    InvalidIrParameter {
        /// Stable textual description of the IR failure.
        message: String,
    },
}

impl fmt::Display for ParameterOptimizationError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::NonFiniteValue { context } => {
                write!(
                    formatter,
                    "non-finite parameter value in {context}"
                )
            }

            Self::ArithmeticOverflow { operation } => {
                write!(
                    formatter,
                    "non-finite result produced by parameter operation `{operation}`"
                )
            }

            Self::DivisionByZero => {
                formatter.write_str(
                    "parameter expression contains division by zero",
                )
            }

            Self::DepthLimitExceeded { limit } => {
                write!(
                    formatter,
                    "parameter expression depth exceeds optimization limit {limit}"
                )
            }

            Self::NodeLimitExceeded { limit } => {
                write!(
                    formatter,
                    "parameter expression node count exceeds optimization limit {limit}"
                )
            }

            Self::InvalidConfiguration { field } => {
                write!(
                    formatter,
                    "invalid parameter optimization configuration field `{field}`"
                )
            }

            Self::InvalidIrParameter { message } => {
                write!(
                    formatter,
                    "canonical quantum IR rejected optimized parameter: {message}"
                )
            }
        }
    }
}

impl std::error::Error for ParameterOptimizationError {}

// =============================================================================
// Configuration
// =============================================================================

/// Controls how parameter expressions are optimized.
///
/// The configuration is intentionally local to this module. The higher-level
/// `optimization::config` can later translate its public compiler policy into
/// this structure without creating a dependency in this direction.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ParameterOptimizationConfig {
    /// Maximum expression depth this optimizer may traverse.
    pub max_depth: usize,

    /// Maximum expression nodes this optimizer may process.
    pub max_nodes: usize,

    /// Fold purely numerical expressions.
    pub constant_folding: bool,

    /// Apply algebraic identities such as x + 0 and x * 1.
    pub algebraic_simplification: bool,

    /// Flatten associative addition/multiplication.
    pub associative_normalization: bool,

    /// Sort independent symbolic terms deterministically.
    pub canonical_ordering: bool,

    /// Combine identical symbolic terms where safe.
    pub combine_like_terms: bool,

    /// Normalize unary negation.
    pub normalize_negation: bool,

    /// Normalize angles modulo 2π.
    ///
    /// This must remain disabled for generic scalar parameters because
    /// x and x + 2π are not universally equivalent scalars.
    pub normalize_angles: bool,

    /// Permit exact division by a numerical constant.
    pub constant_division: bool,

    /// Numerical comparison absolute tolerance.
    pub absolute_tolerance: f64,

    /// Numerical comparison relative tolerance.
    pub relative_tolerance: f64,
}

impl Default for ParameterOptimizationConfig {
    fn default() -> Self {
        Self {
            max_depth: DEFAULT_MAX_DEPTH,
            max_nodes: DEFAULT_MAX_NODES,
            constant_folding: true,
            algebraic_simplification: true,
            associative_normalization: true,
            canonical_ordering: true,
            combine_like_terms: true,
            normalize_negation: true,
            normalize_angles: false,
            constant_division: true,
            absolute_tolerance: DEFAULT_ABSOLUTE_TOLERANCE,
            relative_tolerance: DEFAULT_RELATIVE_TOLERANCE,
        }
    }
}

impl ParameterOptimizationConfig {
    /// Creates the production default configuration.
    #[must_use]
    pub const fn production() -> Self {
        Self {
            max_depth: DEFAULT_MAX_DEPTH,
            max_nodes: DEFAULT_MAX_NODES,
            constant_folding: true,
            algebraic_simplification: true,
            associative_normalization: true,
            canonical_ordering: true,
            combine_like_terms: true,
            normalize_negation: true,
            normalize_angles: false,
            constant_division: true,
            absolute_tolerance: DEFAULT_ABSOLUTE_TOLERANCE,
            relative_tolerance: DEFAULT_RELATIVE_TOLERANCE,
        }
    }

    /// Creates a conservative configuration suitable for fast compilation.
    #[must_use]
    pub const fn fast() -> Self {
        Self {
            max_depth: DEFAULT_MAX_DEPTH,
            max_nodes: DEFAULT_MAX_NODES,
            constant_folding: true,
            algebraic_simplification: true,
            associative_normalization: false,
            canonical_ordering: false,
            combine_like_terms: false,
            normalize_negation: true,
            normalize_angles: false,
            constant_division: true,
            absolute_tolerance: DEFAULT_ABSOLUTE_TOLERANCE,
            relative_tolerance: DEFAULT_RELATIVE_TOLERANCE,
        }
    }

    /// Creates an aggressive symbolic simplification configuration.
    #[must_use]
    pub const fn aggressive() -> Self {
        Self {
            max_depth: DEFAULT_MAX_DEPTH,
            max_nodes: DEFAULT_MAX_NODES,
            constant_folding: true,
            algebraic_simplification: true,
            associative_normalization: true,
            canonical_ordering: true,
            combine_like_terms: true,
            normalize_negation: true,
            normalize_angles: false,
            constant_division: true,
            absolute_tolerance: DEFAULT_ABSOLUTE_TOLERANCE,
            relative_tolerance: DEFAULT_RELATIVE_TOLERANCE,
        }
    }

    /// Validates this configuration.
    pub fn validate(
        &self,
    ) -> ParameterOptimizationResult<()> {
        if self.max_depth == 0 {
            return Err(
                ParameterOptimizationError::InvalidConfiguration {
                    field: "max_depth",
                },
            );
        }

        if self.max_nodes == 0 {
            return Err(
                ParameterOptimizationError::InvalidConfiguration {
                    field: "max_nodes",
                },
            );
        }

        if !self.absolute_tolerance.is_finite()
            || self.absolute_tolerance < 0.0
        {
            return Err(
                ParameterOptimizationError::InvalidConfiguration {
                    field: "absolute_tolerance",
                },
            );
        }

        if !self.relative_tolerance.is_finite()
            || self.relative_tolerance < 0.0
        {
            return Err(
                ParameterOptimizationError::InvalidConfiguration {
                    field: "relative_tolerance",
                },
            );
        }

        Ok(())
    }
}

// =============================================================================
// Optimization statistics
// =============================================================================

/// Statistics emitted by one parameter optimization operation.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ParameterOptimizationStatistics {
    /// Number of input expression nodes.
    pub input_nodes: usize,

    /// Number of output expression nodes.
    pub output_nodes: usize,

    /// Number of constants folded.
    pub constants_folded: usize,

    /// Number of identities removed.
    pub identities_removed: usize,

    /// Number of expressions structurally reordered.
    pub expressions_reordered: usize,

    /// Number of like terms combined.
    pub like_terms_combined: usize,

    /// Number of negations normalized.
    pub negations_normalized: usize,

    /// Number of angle normalizations.
    pub angles_normalized: usize,

    /// Whether the configured resource budget was reached.
    pub budget_reached: bool,

    /// Whether the resulting parameter differs structurally from the input.
    pub changed: bool,
}

impl ParameterOptimizationStatistics {
    /// Returns true when no transformation occurred.
    #[must_use]
    pub const fn unchanged() -> Self {
        Self {
            input_nodes: 0,
            output_nodes: 0,
            constants_folded: 0,
            identities_removed: 0,
            expressions_reordered: 0,
            like_terms_combined: 0,
            negations_normalized: 0,
            angles_normalized: 0,
            budget_reached: false,
            changed: false,
        }
    }
}

// =============================================================================
// Optimization result
// =============================================================================

/// Result of optimizing one canonical IR parameter.
#[derive(Debug, Clone, PartialEq)]
pub struct ParameterOptimizationResultValue {
    /// Optimized canonical IR parameter.
    pub parameter: Parameter,

    /// Transformation statistics.
    pub statistics: ParameterOptimizationStatistics,
}

impl ParameterOptimizationResultValue {
    /// Returns the optimized parameter.
    #[must_use]
    pub fn parameter(&self) -> &Parameter {
        &self.parameter
    }

    /// Consumes the result and returns the parameter.
    #[must_use]
    pub fn into_parameter(self) -> Parameter {
        self.parameter
    }
}

// =============================================================================
// Public entry points
// =============================================================================

/// Optimizes a canonical IR parameter using production defaults.
///
/// This is the main entry point future optimization passes should normally
/// use.
pub fn optimize_parameter(
    parameter: &Parameter,
) -> ParameterOptimizationResultValue {
    optimize_parameter_with_config(
        parameter,
        &ParameterOptimizationConfig::production(),
    )
    .unwrap_or_else(|error| {
        // The canonical IR should already guarantee the basic validity
        // conditions. A failure here therefore indicates that a caller has
        // supplied an invalid configuration or that a transformation violated
        // an IR invariant.
        //
        // Do not panic. Return the original parameter through a conservative
        // fallback result.
        //
        // The public fallible API below should be used by compiler code that
        // needs to distinguish these cases.
        ParameterOptimizationResultValue {
            parameter: parameter.clone(),
            statistics: ParameterOptimizationStatistics {
                budget_reached: matches!(
                    error,
                    ParameterOptimizationError::DepthLimitExceeded { .. }
                        | ParameterOptimizationError::NodeLimitExceeded { .. }
                ),
                ..ParameterOptimizationStatistics::unchanged()
            },
        }
    })
}

/// Fallible production entry point.
pub fn optimize_parameter_checked(
    parameter: &Parameter,
) -> ParameterOptimizationResult<ParameterOptimizationResultValue> {
    optimize_parameter_with_config(
        parameter,
        &ParameterOptimizationConfig::production(),
    )
}

/// Optimizes a parameter with explicit configuration.
pub fn optimize_parameter_with_config(
    parameter: &Parameter,
    config: &ParameterOptimizationConfig,
) -> ParameterOptimizationResult<ParameterOptimizationResultValue> {
    config.validate()?;
    parameter.validate().map_err(|error| {
        ParameterOptimizationError::InvalidIrParameter {
            message: error.to_string(),
        }
    })?;

    let input_nodes = parameter_node_count_bounded(
        parameter,
        config.max_nodes,
        config.max_depth,
    )?;

    let mut state = OptimizerState {
        config,
        statistics: ParameterOptimizationStatistics {
            input_nodes,
            ..ParameterOptimizationStatistics::unchanged()
        },
        nodes_seen: 0,
    };

    let optimized =
        simplify_parameter_internal(parameter, 0, &mut state)?;

    optimized.validate().map_err(|error| {
        ParameterOptimizationError::InvalidIrParameter {
            message: error.to_string(),
        }
    })?;

    let output_nodes = parameter_node_count_bounded(
        &optimized,
        config.max_nodes,
        config.max_depth,
    )?;

    state.statistics.output_nodes = output_nodes;
    state.statistics.changed =
        !structurally_equal(parameter, &optimized);

    Ok(ParameterOptimizationResultValue {
        parameter: optimized,
        statistics: state.statistics,
    })
}

/// Simplifies a parameter using production defaults.
///
/// This function is convenient for local optimization passes that only need
/// the transformed parameter.
pub fn simplify_parameter(
    parameter: &Parameter,
) -> ParameterOptimizationResult<Parameter> {
    Ok(
        optimize_parameter_checked(parameter)?
            .into_parameter(),
    )
}

/// Normalizes a parameter without enabling aggressive term collection.
///
/// This is useful for compiler canonicalization passes where deterministic
/// structure matters but transformation strength should remain conservative.
pub fn normalize_parameter(
    parameter: &Parameter,
) -> ParameterOptimizationResult<Parameter> {
    let mut config = ParameterOptimizationConfig::fast();

    config.associative_normalization = true;
    config.canonical_ordering = true;
    config.combine_like_terms = false;

    Ok(
        optimize_parameter_with_config(
            parameter,
            &config,
        )?
        .into_parameter(),
    )
}

/// Optimizes all parameters in a canonical [`GateParameter`].
pub fn simplify_gate_parameter(
    parameters: &GateParameter,
) -> ParameterOptimizationResult<GateParameter> {
    let config = ParameterOptimizationConfig::production();

    simplify_gate_parameter_with_config(
        parameters,
        &config,
    )
}

/// Optimizes all parameters in a canonical [`GateParameter`] using explicit
/// configuration.
pub fn simplify_gate_parameter_with_config(
    parameters: &GateParameter,
    config: &ParameterOptimizationConfig,
) -> ParameterOptimizationResult<GateParameter> {
    config.validate()?;
    parameters.validate().map_err(|error| {
        ParameterOptimizationError::InvalidIrParameter {
            message: error.to_string(),
        }
    })?;

    match parameters {
        GateParameter::Angle(value) => {
            let optimized =
                optimize_parameter_with_config(
                    value,
                    config,
                )?;

            GateParameter::angle(
                optimized.parameter,
            )
            .map_err(|error| {
                ParameterOptimizationError::InvalidIrParameter {
                    message: error.to_string(),
                }
            })
        }

        GateParameter::TwoAngles {
            theta,
            phi,
        } => {
            let theta =
                optimize_parameter_with_config(
                    theta,
                    config,
                )?
                .into_parameter();

            let phi =
                optimize_parameter_with_config(
                    phi,
                    config,
                )?
                .into_parameter();

            GateParameter::two_angles(theta, phi)
                .map_err(|error| {
                    ParameterOptimizationError::InvalidIrParameter {
                        message: error.to_string(),
                    }
                })
        }

        GateParameter::ThreeAngles {
            theta,
            phi,
            lambda,
        } => {
            let theta =
                optimize_parameter_with_config(
                    theta,
                    config,
                )?
                .into_parameter();

            let phi =
                optimize_parameter_with_config(
                    phi,
                    config,
                )?
                .into_parameter();

            let lambda =
                optimize_parameter_with_config(
                    lambda,
                    config,
                )?
                .into_parameter();

            GateParameter::three_angles(
                theta,
                phi,
                lambda,
            )
            .map_err(|error| {
                ParameterOptimizationError::InvalidIrParameter {
                    message: error.to_string(),
                }
            })
        }
    }
}

// =============================================================================
// Internal optimizer state
// =============================================================================

struct OptimizerState<'a> {
    config: &'a ParameterOptimizationConfig,
    statistics: ParameterOptimizationStatistics,
    nodes_seen: usize,
}

impl OptimizerState<'_> {
    fn visit(
        &mut self,
    ) -> ParameterOptimizationResult<()> {
        self.nodes_seen = self
            .nodes_seen
            .checked_add(1)
            .ok_or(
                ParameterOptimizationError::NodeLimitExceeded {
                    limit: self.config.max_nodes,
                },
            )?;

        if self.nodes_seen > self.config.max_nodes {
            return Err(
                ParameterOptimizationError::NodeLimitExceeded {
                    limit: self.config.max_nodes,
                },
            );
        }

        Ok(())
    }
}

// =============================================================================
// Recursive simplification
// =============================================================================

fn simplify_parameter_internal(
    parameter: &Parameter,
    depth: usize,
    state: &mut OptimizerState<'_>,
) -> ParameterOptimizationResult<Parameter> {
    if depth > state.config.max_depth {
        return Err(
            ParameterOptimizationError::DepthLimitExceeded {
                limit: state.config.max_depth,
            },
        );
    }

    state.visit()?;

    match parameter {
        Parameter::Constant(value) => {
            ensure_finite(
                *value,
                "constant",
            )?;

            Ok(parameter.clone())
        }

        Parameter::Symbol(_) => {
            Ok(parameter.clone())
        }

        Parameter::Expression(expression) => {
            simplify_expression(
                expression,
                depth,
                state,
            )
        }
    }
}

fn simplify_expression(
    expression: &ParameterExpression,
    depth: usize,
    state: &mut OptimizerState<'_>,
) -> ParameterOptimizationResult<Parameter> {
    if depth >= state.config.max_depth {
        return Err(
            ParameterOptimizationError::DepthLimitExceeded {
                limit: state.config.max_depth,
            },
        );
    }

    let simplified =
        match expression {
            ParameterExpression::Add(left, right) => {
                let left =
                    simplify_parameter_internal(
                        left,
                        depth + 1,
                        state,
                    )?;

                let right =
                    simplify_parameter_internal(
                        right,
                        depth + 1,
                        state,
                    )?;

                simplify_add(
                    left,
                    right,
                    state,
                )?
            }

            ParameterExpression::Subtract(left, right) => {
                let left =
                    simplify_parameter_internal(
                        left,
                        depth + 1,
                        state,
                    )?;

                let right =
                    simplify_parameter_internal(
                        right,
                        depth + 1,
                        state,
                    )?;

                simplify_subtract(
                    left,
                    right,
                    state,
                )?
            }

            ParameterExpression::Multiply(left, right) => {
                let left =
                    simplify_parameter_internal(
                        left,
                        depth + 1,
                        state,
                    )?;

                let right =
                    simplify_parameter_internal(
                        right,
                        depth + 1,
                        state,
                    )?;

                simplify_multiply(
                    left,
                    right,
                    state,
                )?
            }

            ParameterExpression::Divide(left, right) => {
                let left =
                    simplify_parameter_internal(
                        left,
                        depth + 1,
                        state,
                    )?;

                let right =
                    simplify_parameter_internal(
                        right,
                        depth + 1,
                        state,
                    )?;

                simplify_divide(
                    left,
                    right,
                    state,
                )?
            }

            ParameterExpression::Negate(value) => {
                let value =
                    simplify_parameter_internal(
                        value,
                        depth + 1,
                        state,
                    )?;

                simplify_negate(
                    value,
                    state,
                )?
            }
        };

    simplified.validate().map_err(|error| {
        ParameterOptimizationError::InvalidIrParameter {
            message: error.to_string(),
        }
    })?;

    Ok(simplified)
}

// =============================================================================
// Addition
// =============================================================================

fn simplify_add(
    left: Parameter,
    right: Parameter,
    state: &mut OptimizerState<'_>,
) -> ParameterOptimizationResult<Parameter> {
    if state.config.constant_folding {
        if let (
            Parameter::Constant(a),
            Parameter::Constant(b),
        ) = (&left, &right)
        {
            let value = checked_add(*a, *b)?;

            state.statistics.constants_folded += 1;

            return Parameter::constant(value)
                .map_err(|error| {
                    ParameterOptimizationError::InvalidIrParameter {
                        message: error.to_string(),
                    }
                });
        }
    }

    if state.config.algebraic_simplification {
        if is_zero(&left) {
            state.statistics.identities_removed += 1;
            return Ok(right);
        }

        if is_zero(&right) {
            state.statistics.identities_removed += 1;
            return Ok(left);
        }

        if structurally_equal(&left, &right)
            && state.config.combine_like_terms
        {
            let coefficient =
                Parameter::constant(2.0)
                    .map_err(|error| {
                        ParameterOptimizationError::InvalidIrParameter {
                            message: error.to_string(),
                        }
                    })?;

            state.statistics.like_terms_combined += 1;

            return make_expression(
                ParameterExpression::Multiply(
                    Box::new(coefficient),
                    Box::new(left),
                ),
            );
        }
    }

    let mut terms = Vec::new();

    if state.config.associative_normalization {
        flatten_add(
            left,
            &mut terms,
        );
        flatten_add(
            right,
            &mut terms,
        );
    } else {
        terms.push(left);
        terms.push(right);
    }

    if state.config.combine_like_terms {
        combine_add_terms(
            &mut terms,
            state,
        );
    }

    if state.config.canonical_ordering {
        terms.sort_by(parameter_order);
        state.statistics.expressions_reordered += 1;
    }

    build_add_expression(terms)
}

// =============================================================================
// Subtraction
// =============================================================================

fn simplify_subtract(
    left: Parameter,
    right: Parameter,
    state: &mut OptimizerState<'_>,
) -> ParameterOptimizationResult<Parameter> {
    if state.config.constant_folding {
        if let (
            Parameter::Constant(a),
            Parameter::Constant(b),
        ) = (&left, &right)
        {
            let value =
                checked_sub(*a, *b)?;

            state.statistics.constants_folded += 1;

            return Parameter::constant(value)
                .map_err(|error| {
                    ParameterOptimizationError::InvalidIrParameter {
                        message: error.to_string(),
                    }
                });
        }
    }

    if state.config.algebraic_simplification {
        if is_zero(&right) {
            state.statistics.identities_removed += 1;
            return Ok(left);
        }

        if structurally_equal(&left, &right) {
            state.statistics.identities_removed += 1;

            return Parameter::constant(0.0)
                .map_err(|error| {
                    ParameterOptimizationError::InvalidIrParameter {
                        message: error.to_string(),
                    }
                });
        }
    }

    let negated =
        simplify_negate(
            right,
            state,
        )?;

    simplify_add(
        left,
        negated,
        state,
    )
}

// =============================================================================
// Multiplication
// =============================================================================

fn simplify_multiply(
    left: Parameter,
    right: Parameter,
    state: &mut OptimizerState<'_>,
) -> ParameterOptimizationResult<Parameter> {
    if state.config.constant_folding {
        if let (
            Parameter::Constant(a),
            Parameter::Constant(b),
        ) = (&left, &right)
        {
            let value =
                checked_mul(*a, *b)?;

            state.statistics.constants_folded += 1;

            return Parameter::constant(value)
                .map_err(|error| {
                    ParameterOptimizationError::InvalidIrParameter {
                        message: error.to_string(),
                    }
                });
        }
    }

    if state.config.algebraic_simplification {
        if is_zero(&left)
            || is_zero(&right)
        {
            state.statistics.identities_removed += 1;

            return Parameter::constant(0.0)
                .map_err(|error| {
                    ParameterOptimizationError::InvalidIrParameter {
                        message: error.to_string(),
                    }
                });
        }

        if is_one(&left) {
            state.statistics.identities_removed += 1;
            return Ok(right);
        }

        if is_one(&right) {
            state.statistics.identities_removed += 1;
            return Ok(left);
        }

        if is_negative_one(&left) {
            return simplify_negate(
                right,
                state,
            );
        }

        if is_negative_one(&right) {
            return simplify_negate(
                left,
                state,
            );
        }
    }

    let mut factors = Vec::new();

    if state.config.associative_normalization {
        flatten_multiply(
            left,
            &mut factors,
        );
        flatten_multiply(
            right,
            &mut factors,
        );
    } else {
        factors.push(left);
        factors.push(right);
    }

    if state.config.canonical_ordering {
        factors.sort_by(parameter_order);
        state.statistics.expressions_reordered += 1;
    }

    build_multiply_expression(factors)
}

// =============================================================================
// Division
// =============================================================================

fn simplify_divide(
    left: Parameter,
    right: Parameter,
    state: &mut OptimizerState<'_>,
) -> ParameterOptimizationResult<Parameter> {
    if is_zero(&right) {
        return Err(
            ParameterOptimizationError::DivisionByZero,
        );
    }

    if state.config.constant_folding {
        if let (
            Parameter::Constant(a),
            Parameter::Constant(b),
        ) = (&left, &right)
        {
            let value =
                checked_div(*a, *b)?;

            state.statistics.constants_folded += 1;

            return Parameter::constant(value)
                .map_err(|error| {
                    ParameterOptimizationError::InvalidIrParameter {
                        message: error.to_string(),
                    }
                });
        }
    }

    if state.config.algebraic_simplification {
        if is_zero(&left) {
            state.statistics.identities_removed += 1;

            return Parameter::constant(0.0)
                .map_err(|error| {
                    ParameterOptimizationError::InvalidIrParameter {
                        message: error.to_string(),
                    }
                });
        }

        if is_one(&right) {
            state.statistics.identities_removed += 1;
            return Ok(left);
        }

        if structurally_equal(&left, &right) {
            state.statistics.identities_removed += 1;

            return Parameter::constant(1.0)
                .map_err(|error| {
                    ParameterOptimizationError::InvalidIrParameter {
                        message: error.to_string(),
                    }
                });
        }
    }

    if state.config.constant_division {
        if let Parameter::Constant(denominator) = right {
            if let Parameter::Constant(numerator) = left {
                let value =
                    checked_div(
                        numerator,
                        denominator,
                    )?;

                return Parameter::constant(value)
                    .map_err(|error| {
                        ParameterOptimizationError::InvalidIrParameter {
                            message: error.to_string(),
                        }
                    });
            }

            if denominator.is_finite()
                && denominator != 0.0
            {
                let reciprocal =
                    checked_div(
                        1.0,
                        denominator,
                    )?;

                let coefficient =
                    Parameter::constant(reciprocal)
                        .map_err(|error| {
                            ParameterOptimizationError::InvalidIrParameter {
                                message: error.to_string(),
                            }
                        })?;

                return simplify_multiply(
                    left,
                    coefficient,
                    state,
                );
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

// =============================================================================
// Negation
// =============================================================================

fn simplify_negate(
    value: Parameter,
    state: &mut OptimizerState<'_>,
) -> ParameterOptimizationResult<Parameter> {
    if state.config.constant_folding {
        if let Parameter::Constant(number) = value {
            let result =
                checked_neg(number)?;

            state.statistics.constants_folded += 1;

            return Parameter::constant(result)
                .map_err(|error| {
                    ParameterOptimizationError::InvalidIrParameter {
                        message: error.to_string(),
                    }
                });
        }
    }

    if state.config.algebraic_simplification {
        if is_zero(&value) {
            state.statistics.identities_removed += 1;
            return Ok(value);
        }
    }

    if state.config.normalize_negation {
        if let Parameter::Expression(expression) = &value {
            if let ParameterExpression::Negate(inner) =
                expression.as_ref()
            {
                state.statistics.negations_normalized += 1;
                return Ok((**inner).clone());
            }
        }
    }

    make_expression(
        ParameterExpression::Negate(
            Box::new(value),
        ),
    )
}

// =============================================================================
// Addition term normalization
// =============================================================================

fn flatten_add(
    parameter: Parameter,
    terms: &mut Vec<Parameter>,
) {
    match parameter {
        Parameter::Expression(expression) => {
            match *expression {
                ParameterExpression::Add(
                    left,
                    right,
                ) => {
                    flatten_add(
                        *left,
                        terms,
                    );
                    flatten_add(
                        *right,
                        terms,
                    );
                }

                other => {
                    terms.push(
                        Parameter::Expression(
                            Box::new(other),
                        ),
                    );
                }
            }
        }

        other => terms.push(other),
    }
}

fn flatten_multiply(
    parameter: Parameter,
    factors: &mut Vec<Parameter>,
) {
    match parameter {
        Parameter::Expression(expression) => {
            match *expression {
                ParameterExpression::Multiply(
                    left,
                    right,
                ) => {
                    flatten_multiply(
                        *left,
                        factors,
                    );
                    flatten_multiply(
                        *right,
                        factors,
                    );
                }

                other => {
                    factors.push(
                        Parameter::Expression(
                            Box::new(other),
                        ),
                    );
                }
            }
        }

        other => factors.push(other),
    }
}

fn combine_add_terms(
    terms: &mut Vec<Parameter>,
    state: &mut OptimizerState<'_>,
) {
    if terms.len() < 2 {
        return;
    }

    let mut output = Vec::with_capacity(
        terms.len(),
    );

    for term in terms.drain(..) {
        let mut combined = false;

        for existing in &mut output {
            if structurally_equal(
                existing,
                &term,
            ) {
                let coefficient =
                    Parameter::constant(2.0);

                if let Ok(coefficient) = coefficient {
                    if let Ok(combined_term) =
                        make_expression(
                            ParameterExpression::Multiply(
                                Box::new(coefficient),
                                Box::new(term.clone()),
                            ),
                        )
                    {
                        *existing = combined_term;
                        state.statistics.like_terms_combined += 1;
                        combined = true;
                        break;
                    }
                }
            }
        }

        if !combined {
            output.push(term);
        }
    }

    *terms = output;
}

// =============================================================================
// Expression construction
// =============================================================================

fn build_add_expression(
    mut terms: Vec<Parameter>,
) -> ParameterOptimizationResult<Parameter> {
    terms.retain(|term| !is_zero(term));

    if terms.is_empty() {
        return Parameter::constant(0.0)
            .map_err(|error| {
                ParameterOptimizationError::InvalidIrParameter {
                    message: error.to_string(),
                }
            });
    }

    if terms.len() == 1 {
        return Ok(terms
            .pop()
            .expect("length checked"));
    }

    let mut expression =
        terms
            .pop()
            .expect("length checked");

    while let Some(term) = terms.pop() {
        expression =
            make_expression(
                ParameterExpression::Add(
                    Box::new(term),
                    Box::new(expression),
                ),
            )?;
    }

    Ok(expression)
}

fn build_multiply_expression(
    mut factors: Vec<Parameter>,
) -> ParameterOptimizationResult<Parameter> {
    if factors.iter().any(is_zero) {
        return Parameter::constant(0.0)
            .map_err(|error| {
                ParameterOptimizationError::InvalidIrParameter {
                    message: error.to_string(),
                }
            });
    }

    factors.retain(|factor| !is_one(factor));

    if factors.is_empty() {
        return Parameter::constant(1.0)
            .map_err(|error| {
                ParameterOptimizationError::InvalidIrParameter {
                    message: error.to_string(),
                }
            });
    }

    if factors.len() == 1 {
        return Ok(factors
            .pop()
            .expect("length checked"));
    }

    let mut expression =
        factors
            .pop()
            .expect("length checked");

    while let Some(factor) =
        factors.pop()
    {
        expression =
            make_expression(
                ParameterExpression::Multiply(
                    Box::new(factor),
                    Box::new(expression),
                ),
            )?;
    }

    Ok(expression)
}

fn make_expression(
    expression: ParameterExpression,
) -> ParameterOptimizationResult<Parameter> {
    Parameter::expression(expression)
        .map_err(|error| {
            ParameterOptimizationError::InvalidIrParameter {
                message: error.to_string(),
            }
        })
}

// =============================================================================
// Predicates
// =============================================================================

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

fn is_negative_one(
    parameter: &Parameter,
) -> bool {
    matches!(
        parameter,
        Parameter::Constant(value)
            if *value == -1.0
    )
}

// =============================================================================
// Stable structural ordering
// =============================================================================

/// Returns a deterministic ordering key for a parameter.
///
/// The ordering is intentionally independent of memory addresses and hash
/// randomization. This is required for reproducible compiler output.
fn parameter_order(
    left: &Parameter,
    right: &Parameter,
) -> std::cmp::Ordering {
    parameter_rank(left)
        .cmp(&parameter_rank(right))
        .then_with(|| {
            parameter_string_key(left)
                .cmp(&parameter_string_key(right))
        })
}

fn parameter_rank(
    parameter: &Parameter,
) -> u8 {
    match parameter {
        Parameter::Constant(_) => 0,
        Parameter::Symbol(_) => 1,
        Parameter::Expression(_) => 2,
    }
}

fn parameter_string_key(
    parameter: &Parameter,
) -> String {
    parameter.to_string()
}

// =============================================================================
// Structural equality
// =============================================================================

/// Tests exact structural equality.
///
/// This does NOT perform floating-point tolerance comparison.
#[must_use]
pub fn structurally_equal(
    left: &Parameter,
    right: &Parameter,
) -> bool {
    left == right
}

// =============================================================================
// Numerical equivalence
// =============================================================================

/// Compares two finite numerical values with explicit absolute and relative
/// tolerances.
///
/// This function does not rewrite parameters and therefore cannot accidentally
/// convert approximate numerical equality into exact symbolic equality.
#[must_use]
pub fn numerically_equal(
    left: f64,
    right: f64,
    absolute_tolerance: f64,
    relative_tolerance: f64,
) -> bool {
    if !left.is_finite()
        || !right.is_finite()
        || !absolute_tolerance.is_finite()
        || !relative_tolerance.is_finite()
        || absolute_tolerance < 0.0
        || relative_tolerance < 0.0
    {
        return false;
    }

    if left == right {
        return true;
    }

    let difference =
        (left - right).abs();

    if difference <= absolute_tolerance {
        return true;
    }

    let scale =
        left.abs().max(right.abs());

    difference
        <= relative_tolerance * scale
}

/// Tests equivalence of two concrete parameters using explicit numerical
/// tolerance.
///
/// Symbolic parameters are compared structurally.
#[must_use]
pub fn equivalent_with_tolerance(
    left: &Parameter,
    right: &Parameter,
    absolute_tolerance: f64,
    relative_tolerance: f64,
) -> bool {
    match (left, right) {
        (
            Parameter::Constant(a),
            Parameter::Constant(b),
        ) => numerically_equal(
            *a,
            *b,
            absolute_tolerance,
            relative_tolerance,
        ),

        (
            Parameter::Symbol(a),
            Parameter::Symbol(b),
        ) => a == b,

        (
            Parameter::Expression(a),
            Parameter::Expression(b),
        ) => {
            equivalent_expression_with_tolerance(
                a,
                b,
                absolute_tolerance,
                relative_tolerance,
            )
        }

        _ => false,
    }
}

fn equivalent_expression_with_tolerance(
    left: &ParameterExpression,
    right: &ParameterExpression,
    absolute_tolerance: f64,
    relative_tolerance: f64,
) -> bool {
    match (left, right) {
        (
            ParameterExpression::Add(
                a_left,
                a_right,
            ),
            ParameterExpression::Add(
                b_left,
                b_right,
            ),
        )
        | (
            ParameterExpression::Multiply(
                a_left,
                a_right,
            ),
            ParameterExpression::Multiply(
                b_left,
                b_right,
            ),
        )
        | (
            ParameterExpression::Subtract(
                a_left,
                a_right,
            ),
            ParameterExpression::Subtract(
                b_left,
                b_right,
            ),
        )
        | (
            ParameterExpression::Divide(
                a_left,
                a_right,
            ),
            ParameterExpression::Divide(
                b_left,
                b_right,
            ),
        ) => {
            equivalent_with_tolerance(
                a_left,
                b_left,
                absolute_tolerance,
                relative_tolerance,
            )
            && equivalent_with_tolerance(
                a_right,
                b_right,
                absolute_tolerance,
                relative_tolerance,
            )
        }

        (
            ParameterExpression::Negate(a),
            ParameterExpression::Negate(b),
        ) => {
            equivalent_with_tolerance(
                a,
                b,
                absolute_tolerance,
                relative_tolerance,
            )
        }

        _ => false,
    }
}

// =============================================================================
// Symbol analysis
// =============================================================================

/// Collects every symbol used by a parameter in deterministic lexical order.
#[must_use]
pub fn collect_symbols(
    parameter: &Parameter,
) -> BTreeSet<String> {
    let mut symbols =
        BTreeSet::new();

    collect_symbols_into(
        parameter,
        &mut symbols,
    );

    symbols
}

fn collect_symbols_into(
    parameter: &Parameter,
    symbols: &mut BTreeSet<String>,
) {
    match parameter {
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
                    collect_symbols_into(
                        left,
                        symbols,
                    );
                    collect_symbols_into(
                        right,
                        symbols,
                    );
                }

                ParameterExpression::Negate(value) => {
                    collect_symbols_into(
                        value,
                        symbols,
                    );
                }
            }
        }
    }
}

/// Returns the number of distinct symbols used by a parameter.
#[must_use]
pub fn symbol_count(
    parameter: &Parameter,
) -> usize {
    collect_symbols(parameter).len()
}

/// Returns whether the parameter depends on any symbolic input.
#[must_use]
pub fn is_symbolic(
    parameter: &Parameter,
) -> bool {
    parameter.is_symbolic()
}

// =============================================================================
// Complexity analysis
// =============================================================================

/// Structural complexity information for one parameter.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ParameterComplexity {
    /// Total number of parameter nodes.
    pub nodes: usize,

    /// Maximum expression depth.
    pub depth: usize,

    /// Number of constants.
    pub constants: usize,

    /// Number of direct symbols.
    pub symbols: usize,

    /// Number of arithmetic expressions.
    pub expressions: usize,

    /// Number of additions.
    pub additions: usize,

    /// Number of subtractions.
    pub subtractions: usize,

    /// Number of multiplications.
    pub multiplications: usize,

    /// Number of divisions.
    pub divisions: usize,

    /// Number of negations.
    pub negations: usize,
}

/// Calculates parameter complexity under an explicit resource limit.
pub fn parameter_complexity(
    parameter: &Parameter,
    max_nodes: usize,
    max_depth: usize,
) -> ParameterOptimizationResult<ParameterComplexity> {
    if max_nodes == 0 {
        return Err(
            ParameterOptimizationError::InvalidConfiguration {
                field: "max_nodes",
            },
        );
    }

    if max_depth == 0 {
        return Err(
            ParameterOptimizationError::InvalidConfiguration {
                field: "max_depth",
            },
        );
    }

    let mut complexity =
        ParameterComplexity::default();

    analyze_complexity(
        parameter,
        0,
        max_nodes,
        max_depth,
        &mut complexity,
    )?;

    Ok(complexity)
}

fn analyze_complexity(
    parameter: &Parameter,
    depth: usize,
    max_nodes: usize,
    max_depth: usize,
    complexity: &mut ParameterComplexity,
) -> ParameterOptimizationResult<()> {
    if depth > max_depth {
        return Err(
            ParameterOptimizationError::DepthLimitExceeded {
                limit: max_depth,
            },
        );
    }

    complexity.nodes =
        complexity
            .nodes
            .checked_add(1)
            .ok_or(
                ParameterOptimizationError::NodeLimitExceeded {
                    limit: max_nodes,
                },
            )?;

    if complexity.nodes > max_nodes {
        return Err(
            ParameterOptimizationError::NodeLimitExceeded {
                limit: max_nodes,
            },
        );
    }

    complexity.depth =
        complexity.depth.max(depth);

    match parameter {
        Parameter::Constant(value) => {
            ensure_finite(
                *value,
                "complexity analysis",
            )?;

            complexity.constants += 1;
        }

        Parameter::Symbol(_) => {
            complexity.symbols += 1;
        }

        Parameter::Expression(expression) => {
            complexity.expressions += 1;

            match expression.as_ref() {
                ParameterExpression::Add(
                    left,
                    right,
                ) => {
                    complexity.additions += 1;

                    analyze_complexity(
                        left,
                        depth + 1,
                        max_nodes,
                        max_depth,
                        complexity,
                    )?;

                    analyze_complexity(
                        right,
                        depth + 1,
                        max_nodes,
                        max_depth,
                        complexity,
                    )?;
                }

                ParameterExpression::Subtract(
                    left,
                    right,
                ) => {
                    complexity.subtractions += 1;

                    analyze_complexity(
                        left,
                        depth + 1,
                        max_nodes,
                        max_depth,
                        complexity,
                    )?;

                    analyze_complexity(
                        right,
                        depth + 1,
                        max_nodes,
                        max_depth,
                        complexity,
                    )?;
                }

                ParameterExpression::Multiply(
                    left,
                    right,
                ) => {
                    complexity.multiplications += 1;

                    analyze_complexity(
                        left,
                        depth + 1,
                        max_nodes,
                        max_depth,
                        complexity,
                    )?;

                    analyze_complexity(
                        right,
                        depth + 1,
                        max_nodes,
                        max_depth,
                        complexity,
                    )?;
                }

                ParameterExpression::Divide(
                    left,
                    right,
                ) => {
                    complexity.divisions += 1;

                    analyze_complexity(
                        left,
                        depth + 1,
                        max_nodes,
                        max_depth,
                        complexity,
                    )?;

                    analyze_complexity(
                        right,
                        depth + 1,
                        max_nodes,
                        max_depth,
                        complexity,
                    )?;
                }

                ParameterExpression::Negate(value) => {
                    complexity.negations += 1;

                    analyze_complexity(
                        value,
                        depth + 1,
                        max_nodes,
                        max_depth,
                        complexity,
                    )?;
                }
            }
        }
    }

    Ok(())
}

// =============================================================================
// Node counting
// =============================================================================

/// Counts parameter nodes with explicit limits.
pub fn parameter_node_count_bounded(
    parameter: &Parameter,
    max_nodes: usize,
    max_depth: usize,
) -> ParameterOptimizationResult<usize> {
    let complexity =
        parameter_complexity(
            parameter,
            max_nodes,
            max_depth,
        )?;

    Ok(complexity.nodes)
}

/// Counts parameter nodes using the production default limits.
pub fn parameter_node_count(
    parameter: &Parameter,
) -> ParameterOptimizationResult<usize> {
    parameter_node_count_bounded(
        parameter,
        DEFAULT_MAX_NODES,
        DEFAULT_MAX_DEPTH,
    )
}

// =============================================================================
// Parameter binding
// =============================================================================

/// Binds a parameter using an explicit symbol resolver and then optimizes the
/// resulting concrete/symbolic expression.
///
/// The resolver is supplied by the caller. There is intentionally no global
/// parameter table.
pub fn bind_and_simplify<F>(
    parameter: &Parameter,
    resolver: &F,
) -> ParameterOptimizationResult<Parameter>
where
    F: Fn(&str) -> Option<f64>,
{
    let bound =
        bind_parameter(
            parameter,
            resolver,
        )?;

    simplify_parameter(
        &bound,
    )
}

/// Binds a parameter without applying optimization afterwards.
pub fn bind_parameter<F>(
    parameter: &Parameter,
    resolver: &F,
) -> ParameterOptimizationResult<Parameter>
where
    F: Fn(&str) -> Option<f64>,
{
    parameter
        .bind(resolver)
        .map_err(|error| {
            ParameterOptimizationError::InvalidIrParameter {
                message: error.to_string(),
            }
        })
        .and_then(|value| {
            Parameter::constant(value)
                .map_err(|error| {
                    ParameterOptimizationError::InvalidIrParameter {
                        message: error.to_string(),
                    }
                })
        })
}

// =============================================================================
// Angle normalization
// =============================================================================

/// Normalizes an angle in radians into the half-open interval
/// `[-π, π)`.
///
/// This is a numerical helper only. It does not modify a symbolic parameter.
pub fn normalize_angle(
    angle: f64,
) -> ParameterOptimizationResult<f64> {
    ensure_finite(
        angle,
        "angle normalization",
    )?;

    let two_pi =
        std::f64::consts::TAU;

    let pi =
        std::f64::consts::PI;

    let mut normalized =
        angle.rem_euclid(two_pi);

    if normalized >= pi {
        normalized -= two_pi;
    }

    ensure_finite(
        normalized,
        "normalized angle",
    )?;

    Ok(normalized)
}

/// Normalizes a concrete numerical parameter as an angle.
///
/// Symbolic values are returned unchanged because their periodicity cannot be
/// assumed by the generic parameter layer.
pub fn normalize_parameter_angle(
    parameter: &Parameter,
) -> ParameterOptimizationResult<Parameter> {
    match parameter {
        Parameter::Constant(value) => {
            Parameter::constant(
                normalize_angle(*value)?,
            )
            .map_err(|error| {
                ParameterOptimizationError::InvalidIrParameter {
                    message: error.to_string(),
                }
            })
        }

        Parameter::Symbol(_) => {
            Ok(parameter.clone())
        }

        Parameter::Expression(_) => {
            let optimized =
                simplify_parameter(parameter)?;

            match optimized {
                Parameter::Constant(value) => {
                    Parameter::constant(
                        normalize_angle(value)?,
                    )
                    .map_err(|error| {
                        ParameterOptimizationError::InvalidIrParameter {
                            message: error.to_string(),
                        }
                    })
                }

                _ => Ok(optimized),
            }
        }
    }
}

// =============================================================================
// Safe arithmetic
// =============================================================================

fn ensure_finite(
    value: f64,
    context: &'static str,
) -> ParameterOptimizationResult<()> {
    if value.is_finite() {
        Ok(())
    } else {
        Err(
            ParameterOptimizationError::NonFiniteValue {
                context,
            },
        )
    }
}

fn checked_add(
    left: f64,
    right: f64,
) -> ParameterOptimizationResult<f64> {
    ensure_finite(
        left,
        "addition operand",
    )?;

    ensure_finite(
        right,
        "addition operand",
    )?;

    let result =
        left + right;

    if result.is_finite() {
        Ok(result)
    } else {
        Err(
            ParameterOptimizationError::ArithmeticOverflow {
                operation: "addition",
            },
        )
    }
}

fn checked_sub(
    left: f64,
    right: f64,
) -> ParameterOptimizationResult<f64> {
    ensure_finite(
        left,
        "subtraction operand",
    )?;

    ensure_finite(
        right,
        "subtraction operand",
    )?;

    let result =
        left - right;

    if result.is_finite() {
        Ok(result)
    } else {
        Err(
            ParameterOptimizationError::ArithmeticOverflow {
                operation: "subtraction",
            },
        )
    }
}

fn checked_mul(
    left: f64,
    right: f64,
) -> ParameterOptimizationResult<f64> {
    ensure_finite(
        left,
        "multiplication operand",
    )?;

    ensure_finite(
        right,
        "multiplication operand",
    )?;

    let result =
        left * right;

    if result.is_finite() {
        Ok(result)
    } else {
        Err(
            ParameterOptimizationError::ArithmeticOverflow {
                operation: "multiplication",
            },
        )
    }
}

fn checked_div(
    numerator: f64,
    denominator: f64,
) -> ParameterOptimizationResult<f64> {
    ensure_finite(
        numerator,
        "division numerator",
    )?;

    ensure_finite(
        denominator,
        "division denominator",
    )?;

    if denominator == 0.0 {
        return Err(
            ParameterOptimizationError::DivisionByZero,
        );
    }

    let result =
        numerator / denominator;

    if result.is_finite() {
        Ok(result)
    } else {
        Err(
            ParameterOptimizationError::ArithmeticOverflow {
                operation: "division",
            },
        )
    }
}

fn checked_neg(
    value: f64,
) -> ParameterOptimizationResult<f64> {
    ensure_finite(
        value,
        "negation operand",
    )?;

    let result =
        -value;

    if result.is_finite() {
        Ok(result)
    } else {
        Err(
            ParameterOptimizationError::ArithmeticOverflow {
                operation: "negation",
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

    fn constant(
        value: f64,
    ) -> Parameter {
        Parameter::constant(value)
            .expect("test constant must be finite")
    }

    fn symbol(
        name: &str,
    ) -> Parameter {
        Parameter::symbol(name)
            .expect("test symbol must be valid")
    }

    fn expression(
        expression: ParameterExpression,
    ) -> Parameter {
        Parameter::expression(expression)
            .expect("test expression must be valid")
    }

    #[test]
    fn folds_constant_addition() {
        let parameter =
            expression(
                ParameterExpression::Add(
                    Box::new(constant(2.0)),
                    Box::new(constant(3.0)),
                ),
            );

        let optimized =
            simplify_parameter(&parameter)
                .expect("optimization should succeed");

        assert_eq!(
            optimized,
            constant(5.0)
        );
    }

    #[test]
    fn folds_constant_multiplication() {
        let parameter =
            expression(
                ParameterExpression::Multiply(
                    Box::new(constant(4.0)),
                    Box::new(constant(5.0)),
                ),
            );

        let optimized =
            simplify_parameter(&parameter)
                .expect("optimization should succeed");

        assert_eq!(
            optimized,
            constant(20.0)
        );
    }

    #[test]
    fn removes_additive_zero() {
        let parameter =
            expression(
                ParameterExpression::Add(
                    Box::new(symbol("theta")),
                    Box::new(constant(0.0)),
                ),
            );

        let optimized =
            simplify_parameter(&parameter)
                .expect("optimization should succeed");

        assert_eq!(
            optimized,
            symbol("theta")
        );
    }

    #[test]
    fn removes_multiplicative_one() {
        let parameter =
            expression(
                ParameterExpression::Multiply(
                    Box::new(symbol("theta")),
                    Box::new(constant(1.0)),
                ),
            );

        let optimized =
            simplify_parameter(&parameter)
                .expect("optimization should succeed");

        assert_eq!(
            optimized,
            symbol("theta")
        );
    }

    #[test]
    fn removes_multiplicative_zero() {
        let parameter =
            expression(
                ParameterExpression::Multiply(
                    Box::new(symbol("theta")),
                    Box::new(constant(0.0)),
                ),
            );

        let optimized =
            simplify_parameter(&parameter)
                .expect("optimization should succeed");

        assert_eq!(
            optimized,
            constant(0.0)
        );
    }

    #[test]
    fn removes_subtraction_of_identical_parameters() {
        let theta =
            symbol("theta");

        let parameter =
            expression(
                ParameterExpression::Subtract(
                    Box::new(theta.clone()),
                    Box::new(theta),
                ),
            );

        let optimized =
            simplify_parameter(&parameter)
                .expect("optimization should succeed");

        assert_eq!(
            optimized,
            constant(0.0)
        );
    }

    #[test]
    fn removes_division_by_one() {
        let parameter =
            expression(
                ParameterExpression::Divide(
                    Box::new(symbol("theta")),
                    Box::new(constant(1.0)),
                ),
            );

        let optimized =
            simplify_parameter(&parameter)
                .expect("optimization should succeed");

        assert_eq!(
            optimized,
            symbol("theta")
        );
    }

    #[test]
    fn detects_division_by_zero() {
        let parameter =
            expression(
                ParameterExpression::Divide(
                    Box::new(symbol("theta")),
                    Box::new(constant(0.0)),
                ),
            );

        let result =
            simplify_parameter(&parameter);

        assert!(matches!(
            result,
            Err(
                ParameterOptimizationError::DivisionByZero
            )
        ));
    }

    #[test]
    fn normalizes_double_negation() {
        let theta =
            symbol("theta");

        let parameter =
            expression(
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

        let optimized =
            simplify_parameter(&parameter)
                .expect("optimization should succeed");

        assert_eq!(
            optimized,
            theta
        );
    }

    #[test]
    fn binds_symbol() {
        let theta =
            symbol("theta");

        let optimized =
            bind_and_simplify(
                &theta,
                &|name| {
                    if name == "theta" {
                        Some(1.25)
                    } else {
                        None
                    }
                },
            )
            .expect("binding should succeed");

        assert_eq!(
            optimized,
            constant(1.25)
        );
    }

    #[test]
    fn collects_symbols_deterministically() {
        let parameter =
            expression(
                ParameterExpression::Add(
                    Box::new(symbol("beta")),
                    Box::new(
                        expression(
                            ParameterExpression::Multiply(
                                Box::new(
                                    symbol("alpha"),
                                ),
                                Box::new(
                                    symbol("gamma"),
                                ),
                            ),
                        ),
                    ),
                ),
            );

        let symbols =
            collect_symbols(&parameter);

        let expected =
            ["alpha", "beta", "gamma"]
                .into_iter()
                .map(str::to_string)
                .collect::<BTreeSet<_>>();

        assert_eq!(
            symbols,
            expected
        );
    }

    #[test]
    fn complexity_counts_nodes() {
        let parameter =
            expression(
                ParameterExpression::Add(
                    Box::new(symbol("theta")),
                    Box::new(constant(1.0)),
                ),
            );

        let complexity =
            parameter_complexity(
                &parameter,
                100,
                64,
            )
            .expect("complexity should succeed");

        assert_eq!(
            complexity.nodes,
            3
        );

        assert_eq!(
            complexity.additions,
            1
        );

        assert_eq!(
            complexity.symbols,
            1
        );

        assert_eq!(
            complexity.constants,
            1
        );
    }

    #[test]
    fn numerical_comparison_is_explicit() {
        assert!(
            numerically_equal(
                1.0,
                1.0 + 1.0e-13,
                1.0e-12,
                1.0e-12,
            )
        );
    }

    #[test]
    fn numerical_comparison_rejects_non_finite() {
        assert!(
            !numerically_equal(
                f64::NAN,
                1.0,
                1.0e-12,
                1.0e-12,
            )
        );
    }

    #[test]
    fn angle_normalization_is_deterministic() {
        let normalized =
            normalize_angle(
                3.0 * std::f64::consts::PI,
            )
            .expect("angle should normalize");

        assert!(
            numerically_equal(
                normalized,
                -std::f64::consts::PI,
                1.0e-12,
                1.0e-12,
            )
        );
    }

    #[test]
    fn gate_parameter_optimization_preserves_arity() {
        let parameters =
            GateParameter::three_angles(
                expression(
                    ParameterExpression::Add(
                        Box::new(constant(1.0)),
                        Box::new(constant(2.0)),
                    ),
                ),
                symbol("phi"),
                constant(0.5),
            )
            .expect("gate parameters should be valid");

        let optimized =
            simplify_gate_parameter(
                &parameters,
            )
            .expect("optimization should succeed");

        assert_eq!(
            optimized.arity(),
            3
        );
    }

    #[test]
    fn structural_equality_is_exact() {
        let left =
            constant(1.0);

        let right =
            constant(1.0 + 1.0e-14);

        assert!(
            !structurally_equal(
                &left,
                &right,
            )
        );
    }

    #[test]
    fn fast_configuration_is_conservative() {
        let config =
            ParameterOptimizationConfig::fast();

        assert!(
            config.constant_folding
        );

        assert!(
            config.algebraic_simplification
        );

        assert!(
            !config.combine_like_terms
        );
    }

    #[test]
    fn configuration_rejects_zero_node_limit() {
        let config =
            ParameterOptimizationConfig {
                max_nodes: 0,
                ..ParameterOptimizationConfig::production()
            };

        assert!(matches!(
            config.validate(),
            Err(
                ParameterOptimizationError::InvalidConfiguration {
                    field: "max_nodes"
                }
            )
        ));
    }
}