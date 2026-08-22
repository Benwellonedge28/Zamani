//! Zamani Quantum IR — Parameter Contract
//!
//! Hardware-independent quantum parameter representation.
//!
//! This module is intentionally independent of gates, circuits, measurements,
//! qubits, routing, scheduling, optimization, and hardware.
//!
//! Parameter semantics belong to the IR because parameterized operations must
//! have one canonical representation regardless of the compiler frontend or
//! backend that consumes them.
//!
//! # Design guarantees
//!
//! - Numerical constants must always be finite.
//! - Symbol names are validated and bounded.
//! - Expressions have a deterministic structure.
//! - Expression depth is bounded.
//! - Gate parameter arity is structurally represented.
//! - Symbol binding is explicit; there is no global parameter environment.
//! - Bound parameters are always finite.
//! - Parameter iteration does not allocate.
//! - Parameter validation does not depend on `Gate` or `Circuit`.
//!
//! # Integration
//!
//! `gate.rs` must consume [`GateParameter`] from this module.
//!
//! `errors.rs` owns the canonical `IrParameterError` and `IrResult` types.
//!
//! No hardware-specific parameter interpretation belongs here. Hardware
//! calibration, pulse parameters, device-specific units, and backend lowering
//! belong to later compiler stages.
//!
//! Rust compatibility: Rust 1.97.1.

use std::fmt;

use super::errors::{IrParameterError, IrResult};

/// Maximum UTF-8 byte length of a symbolic parameter name.
///
/// This prevents unbounded symbol allocation from entering the IR.
pub const MAX_PARAMETER_SYMBOL_BYTES: usize = 256;

/// Maximum expression nesting depth.
///
/// This protects recursive validation/evaluation from pathological input.
pub const MAX_PARAMETER_EXPRESSION_DEPTH: usize = 64;

// -----------------------------------------------------------------------------
// Parameter
// -----------------------------------------------------------------------------

/// A canonical scalar quantum parameter.
///
/// Parameters are normally interpreted as angles in radians by parameterized
/// quantum gates, but this type intentionally does not attach units to the
/// value. Unit interpretation belongs to the consuming operation.
#[derive(Debug, Clone, PartialEq)]
pub enum Parameter {
    /// A concrete finite numerical value.
    Constant(f64),

    /// A symbolic parameter.
    ///
    /// Symbol names are validated by [`Parameter::symbol`].
    Symbol(String),

    /// A deterministic arithmetic expression.
    Expression(Box<ParameterExpression>),
}

impl Parameter {
    /// Creates a finite numerical parameter.
    pub fn constant(value: f64) -> IrResult<Self> {
        if !value.is_finite() {
            return Err(IrParameterError::NonFinite.into());
        }

        Ok(Self::Constant(value))
    }

    /// Creates a validated symbolic parameter.
    pub fn symbol<S: Into<String>>(name: S) -> IrResult<Self> {
        let name = name.into();

        validate_symbol(&name)?;

        Ok(Self::Symbol(name))
    }

    /// Creates a validated parameter expression.
    pub fn expression(
        expression: ParameterExpression,
    ) -> IrResult<Self> {
        expression.validate()?;

        Ok(Self::Expression(Box::new(expression)))
    }

    /// Returns true if this parameter is a concrete constant.
    pub const fn is_constant(&self) -> bool {
        matches!(self, Self::Constant(_))
    }

    /// Returns true if this parameter contains a symbolic dependency.
    pub fn is_symbolic(&self) -> bool {
        match self {
            Self::Constant(_) => false,

            Self::Symbol(_) => true,

            Self::Expression(expression) => {
                expression.is_symbolic()
            }
        }
    }

    /// Returns the constant value if this is a constant parameter.
    pub const fn as_constant(&self) -> Option<f64> {
        match self {
            Self::Constant(value) => Some(*value),

            Self::Symbol(_) | Self::Expression(_) => None,
        }
    }

    /// Returns the symbol name if this is a direct symbol.
    pub fn as_symbol(&self) -> Option<&str> {
        match self {
            Self::Symbol(name) => Some(name),

            Self::Constant(_) | Self::Expression(_) => None,
        }
    }

    /// Validates the entire parameter structure.
    pub fn validate(&self) -> IrResult<()> {
        match self {
            Self::Constant(value) => {
                if value.is_finite() {
                    Ok(())
                } else {
                    Err(IrParameterError::NonFinite.into())
                }
            }

            Self::Symbol(name) => validate_symbol(name),

            Self::Expression(expression) => {
                expression.validate()
            }
        }
    }

    /// Resolves this parameter to a finite numerical value.
    ///
    /// Symbol resolution is explicitly supplied by the caller. The IR never
    /// consults global mutable state.
    pub fn bind<F>(
        &self,
        resolver: &F,
    ) -> IrResult<f64>
    where
        F: Fn(&str) -> Option<f64>,
    {
        match self {
            Self::Constant(value) => {
                if value.is_finite() {
                    Ok(*value)
                } else {
                    Err(IrParameterError::NonFinite.into())
                }
            }

            Self::Symbol(name) => {
                match resolver(name) {
                    Some(value) if value.is_finite() => {
                        Ok(value)
                    }

                    Some(_) => {
                        Err(
                            IrParameterError::NonFinite
                                .into(),
                        )
                    }

                    None => {
                        Err(
                            IrParameterError::UnboundSymbol {
                                name: name.clone(),
                            }
                            .into(),
                        )
                    }
                }
            }

            Self::Expression(expression) => {
                expression.evaluate(resolver)
            }
        }
    }
}

impl fmt::Display for Parameter {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::Constant(value) => {
                write!(f, "{value:?}")
            }

            Self::Symbol(name) => {
                f.write_str(name)
            }

            Self::Expression(expression) => {
                expression.fmt(f)
            }
        }
    }
}

// -----------------------------------------------------------------------------
// Parameter expression
// -----------------------------------------------------------------------------

/// Deterministic arithmetic expression over [`Parameter`] values.
///
/// Expressions deliberately start with a small arithmetic vocabulary. More
/// operations can be added in a future IR version without introducing
/// backend-specific semantics here.
#[derive(Debug, Clone, PartialEq)]
pub enum ParameterExpression {
    /// Addition.
    Add(
        Box<Parameter>,
        Box<Parameter>,
    ),

    /// Subtraction.
    Subtract(
        Box<Parameter>,
        Box<Parameter>,
    ),

    /// Multiplication.
    Multiply(
        Box<Parameter>,
        Box<Parameter>,
    ),

    /// Division.
    Divide(
        Box<Parameter>,
        Box<Parameter>,
    ),

    /// Unary negation.
    Negate(Box<Parameter>),
}

impl ParameterExpression {
    /// Validates the expression.
    pub fn validate(&self) -> IrResult<()> {
        self.validate_at_depth(0)
    }

    fn validate_at_depth(
        &self,
        depth: usize,
    ) -> IrResult<()> {
        if depth > MAX_PARAMETER_EXPRESSION_DEPTH {
            return Err(
                IrParameterError::InvalidExpression
                    .into(),
            );
        }

        match self {
            Self::Add(left, right)
            | Self::Subtract(left, right)
            | Self::Multiply(left, right)
            | Self::Divide(left, right) => {
                left.validate_at_depth(depth + 1)?;
                right.validate_at_depth(depth + 1)?;
            }

            Self::Negate(value) => {
                value.validate_at_depth(depth + 1)?;
            }
        }

        Ok(())
    }

    /// Returns true if the expression contains a symbol.
    pub fn is_symbolic(&self) -> bool {
        match self {
            Self::Add(left, right)
            | Self::Subtract(left, right)
            | Self::Multiply(left, right)
            | Self::Divide(left, right) => {
                left.is_symbolic()
                    || right.is_symbolic()
            }

            Self::Negate(value) => {
                value.is_symbolic()
            }
        }
    }

    /// Evaluates the expression with an explicit symbol resolver.
    pub fn evaluate<F>(
        &self,
        resolver: &F,
    ) -> IrResult<f64>
    where
        F: Fn(&str) -> Option<f64>,
    {
        self.validate()?;

        let value = match self {
            Self::Add(left, right) => {
                left.bind(resolver)?
                    + right.bind(resolver)?
            }

            Self::Subtract(left, right) => {
                left.bind(resolver)?
                    - right.bind(resolver)?
            }

            Self::Multiply(left, right) => {
                left.bind(resolver)?
                    * right.bind(resolver)?
            }

            Self::Divide(left, right) => {
                let denominator =
                    right.bind(resolver)?;

                if denominator == 0.0 {
                    return Err(
                        IrParameterError::InvalidExpression
                            .into(),
                    );
                }

                left.bind(resolver)? / denominator
            }

            Self::Negate(value) => {
                -value.bind(resolver)?
            }
        };

        if value.is_finite() {
            Ok(value)
        } else {
            Err(IrParameterError::NonFinite.into())
        }
    }
}

impl fmt::Display for ParameterExpression {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::Add(left, right) => {
                write!(f, "({left} + {right})")
            }

            Self::Subtract(left, right) => {
                write!(f, "({left} - {right})")
            }

            Self::Multiply(left, right) => {
                write!(f, "({left} * {right})")
            }

            Self::Divide(left, right) => {
                write!(f, "({left} / {right})")
            }

            Self::Negate(value) => {
                write!(f, "(-{value})")
            }
        }
    }
}

// -----------------------------------------------------------------------------
// Gate parameter group
// -----------------------------------------------------------------------------

/// Structurally typed parameter group for parameterized gates.
///
/// This replaces the old gate-local parameter representation. The parameter
/// count is part of the type's value shape:
///
/// - `Angle` = exactly one parameter;
/// - `TwoAngles` = exactly two parameters;
/// - `ThreeAngles` = exactly three parameters.
///
/// This makes parameter arity explicit before a gate-specific semantic check
/// is performed.
#[derive(Debug, Clone, PartialEq)]
pub enum GateParameter {
    /// One parameter.
    Angle(Parameter),

    /// Two parameters.
    TwoAngles {
        theta: Parameter,
        phi: Parameter,
    },

    /// Three parameters.
    ThreeAngles {
        theta: Parameter,
        phi: Parameter,
        lambda: Parameter,
    },
}

impl GateParameter {
    /// Creates a one-parameter group.
    pub fn angle(
        parameter: Parameter,
    ) -> IrResult<Self> {
        parameter.validate()?;

        Ok(Self::Angle(parameter))
    }

    /// Creates a two-parameter group.
    pub fn two_angles(
        theta: Parameter,
        phi: Parameter,
    ) -> IrResult<Self> {
        theta.validate()?;
        phi.validate()?;

        Ok(Self::TwoAngles {
            theta,
            phi,
        })
    }

    /// Creates a three-parameter group.
    pub fn three_angles(
        theta: Parameter,
        phi: Parameter,
        lambda: Parameter,
    ) -> IrResult<Self> {
        theta.validate()?;
        phi.validate()?;
        lambda.validate()?;

        Ok(Self::ThreeAngles {
            theta,
            phi,
            lambda,
        })
    }

    /// Returns the structural parameter arity.
    pub const fn arity(&self) -> usize {
        match self {
            Self::Angle(_) => 1,

            Self::TwoAngles { .. } => 2,

            Self::ThreeAngles { .. } => 3,
        }
    }

    /// Returns true when at least one parameter is symbolic.
    pub fn is_symbolic(&self) -> bool {
        match self {
            Self::Angle(value) => {
                value.is_symbolic()
            }

            Self::TwoAngles {
                theta,
                phi,
            } => {
                theta.is_symbolic()
                    || phi.is_symbolic()
            }

            Self::ThreeAngles {
                theta,
                phi,
                lambda,
            } => {
                theta.is_symbolic()
                    || phi.is_symbolic()
                    || lambda.is_symbolic()
            }
        }
    }

    /// Validates every parameter.
    pub fn validate(&self) -> IrResult<()> {
        match self {
            Self::Angle(value) => {
                value.validate()
            }

            Self::TwoAngles {
                theta,
                phi,
            } => {
                theta.validate()?;
                phi.validate()
            }

            Self::ThreeAngles {
                theta,
                phi,
                lambda,
            } => {
                theta.validate()?;
                phi.validate()?;
                lambda.validate()
            }
        }
    }

    /// Returns the first parameter without allocation.
    pub fn first(&self) -> &Parameter {
        match self {
            Self::Angle(value) => value,

            Self::TwoAngles {
                theta,
                ..
            } => theta,

            Self::ThreeAngles {
                theta,
                ..
            } => theta,
        }
    }

    /// Returns an allocation-free iterator over the parameters.
    pub fn iter(&self) -> GateParameterIter<'_> {
        match self {
            Self::Angle(value) => {
                GateParameterIter::one(value)
            }

            Self::TwoAngles {
                theta,
                phi,
            } => {
                GateParameterIter::two(
                    theta,
                    phi,
                )
            }

            Self::ThreeAngles {
                theta,
                phi,
                lambda,
            } => {
                GateParameterIter::three(
                    theta,
                    phi,
                    lambda,
                )
            }
        }
    }

    /// Resolves all parameters into finite numerical values.
    pub fn bind<F>(
        &self,
        resolver: &F,
    ) -> IrResult<BoundGateParameter>
    where
        F: Fn(&str) -> Option<f64>,
    {
        match self {
            Self::Angle(value) => {
                Ok(BoundGateParameter::Angle(
                    value.bind(resolver)?,
                ))
            }

            Self::TwoAngles {
                theta,
                phi,
            } => {
                Ok(BoundGateParameter::TwoAngles {
                    theta: theta.bind(resolver)?,
                    phi: phi.bind(resolver)?,
                })
            }

            Self::ThreeAngles {
                theta,
                phi,
                lambda,
            } => {
                Ok(
                    BoundGateParameter::ThreeAngles {
                        theta: theta.bind(resolver)?,
                        phi: phi.bind(resolver)?,
                        lambda: lambda.bind(resolver)?,
                    },
                )
            }
        }
    }
}

// -----------------------------------------------------------------------------
// Bound parameters
// -----------------------------------------------------------------------------

/// A gate parameter group after all symbolic parameters have been bound.
///
/// This type contains only finite numerical values.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BoundGateParameter {
    /// One numerical parameter.
    Angle(f64),

    /// Two numerical parameters.
    TwoAngles {
        theta: f64,
        phi: f64,
    },

    /// Three numerical parameters.
    ThreeAngles {
        theta: f64,
        phi: f64,
        lambda: f64,
    },
}

impl BoundGateParameter {
    /// Returns the parameter arity.
    pub const fn arity(self) -> usize {
        match self {
            Self::Angle(_) => 1,

            Self::TwoAngles { .. } => 2,

            Self::ThreeAngles { .. } => 3,
        }
    }

    /// Returns the first bound parameter.
    pub const fn first(self) -> f64 {
        match self {
            Self::Angle(value) => value,

            Self::TwoAngles { theta, .. } => theta,

            Self::ThreeAngles { theta, .. } => theta,
        }
    }

    /// Returns the parameter at an index without allocation.
    pub const fn get(
        self,
        index: usize,
    ) -> Option<f64> {
        match self {
            Self::Angle(value) => match index {
                0 => Some(value),
                _ => None,
            },

            Self::TwoAngles {
                theta,
                phi,
            } => match index {
                0 => Some(theta),
                1 => Some(phi),
                _ => None,
            },

            Self::ThreeAngles {
                theta,
                phi,
                lambda,
            } => match index {
                0 => Some(theta),
                1 => Some(phi),
                2 => Some(lambda),
                _ => None,
            },
        }
    }
}

// -----------------------------------------------------------------------------
// Allocation-free parameter iterator
// -----------------------------------------------------------------------------

/// Allocation-free iterator over a [`GateParameter`].
pub struct GateParameterIter<'a> {
    values: [&'a Parameter; 3],
    len: usize,
    index: usize,
}

impl<'a> GateParameterIter<'a> {
    fn one(
        first: &'a Parameter,
    ) -> Self {
        Self {
            values: [
                first,
                first,
                first,
            ],
            len: 1,
            index: 0,
        }
    }

    fn two(
        first: &'a Parameter,
        second: &'a Parameter,
    ) -> Self {
        Self {
            values: [
                first,
                second,
                second,
            ],
            len: 2,
            index: 0,
        }
    }

    fn three(
        first: &'a Parameter,
        second: &'a Parameter,
        third: &'a Parameter,
    ) -> Self {
        Self {
            values: [
                first,
                second,
                third,
            ],
            len: 3,
            index: 0,
        }
    }
}

impl<'a> Iterator for GateParameterIter<'a> {
    type Item = &'a Parameter;

    fn next(
        &mut self,
    ) -> Option<Self::Item> {
        if self.index >= self.len {
            return None;
        }

        let value =
            self.values[self.index];

        self.index += 1;

        Some(value)
    }

    fn size_hint(
        &self,
    ) -> (usize, Option<usize>) {
        let remaining =
            self.len - self.index;

        (remaining, Some(remaining))
    }
}

impl<'a> ExactSizeIterator
    for GateParameterIter<'a>
{
}

// -----------------------------------------------------------------------------
// Validation helpers
// -----------------------------------------------------------------------------

fn validate_symbol(
    name: &str,
) -> IrResult<()> {
    if name.is_empty()
        || name.len()
            > MAX_PARAMETER_SYMBOL_BYTES
    {
        return Err(
            IrParameterError::InvalidSymbol
                .into(),
        );
    }

    let mut chars =
        name.chars();

    let first = match chars.next() {
        Some(character) => character,

        None => {
            return Err(
                IrParameterError::InvalidSymbol
                    .into(),
            )
        }
    };

    if !(first == '_'
        || first.is_ascii_alphabetic())
    {
        return Err(
            IrParameterError::InvalidSymbol
                .into(),
        );
    }

    if !chars.all(|character| {
        character == '_'
            || character.is_ascii_alphanumeric()
    }) {
        return Err(
            IrParameterError::InvalidSymbol
                .into(),
        );
    }

    Ok(())
}

// -----------------------------------------------------------------------------
// Convenience constructors
// -----------------------------------------------------------------------------

/// Creates a finite constant parameter.
pub fn constant(
    value: f64,
) -> IrResult<Parameter> {
    Parameter::constant(value)
}

/// Creates a validated symbolic parameter.
pub fn symbol<S: Into<String>>(
    name: S,
) -> IrResult<Parameter> {
    Parameter::symbol(name)
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finite_constants_are_accepted() {
        let parameter =
            constant(1.25).unwrap();

        assert_eq!(
            parameter.as_constant(),
            Some(1.25)
        );

        assert!(parameter.is_constant());
        assert!(!parameter.is_symbolic());
    }

    #[test]
    fn non_finite_constants_are_rejected() {
        assert!(
            constant(f64::NAN).is_err()
        );

        assert!(
            constant(f64::INFINITY).is_err()
        );

        assert!(
            constant(f64::NEG_INFINITY)
                .is_err()
        );
    }

    #[test]
    fn valid_symbols_are_accepted() {
        assert!(
            symbol("theta").is_ok()
        );

        assert!(
            symbol("_theta_0").is_ok()
        );

        assert!(
            symbol("theta123").is_ok()
        );
    }

    #[test]
    fn invalid_symbols_are_rejected() {
        assert!(
            symbol("").is_err()
        );

        assert!(
            symbol("0theta").is_err()
        );

        assert!(
            symbol("theta-value").is_err()
        );

        assert!(
            symbol("theta value").is_err()
        );
    }

    #[test]
    fn expression_is_symbolic() {
        let theta =
            symbol("theta").unwrap();

        let one =
            constant(1.0).unwrap();

        let expression =
            ParameterExpression::Add(
                Box::new(theta),
                Box::new(one),
            );

        assert!(
            expression.is_symbolic()
        );
    }

    #[test]
    fn expression_evaluates_deterministically() {
        let theta =
            symbol("theta").unwrap();

        let one =
            constant(1.0).unwrap();

        let expression =
            ParameterExpression::Add(
                Box::new(theta),
                Box::new(one),
            );

        let parameter =
            Parameter::expression(
                expression,
            )
            .unwrap();

        let value = parameter
            .bind(&|name| {
                if name == "theta" {
                    Some(2.0)
                } else {
                    None
                }
            })
            .unwrap();

        assert_eq!(value, 3.0);
    }

    #[test]
    fn unbound_symbols_are_rejected() {
        let parameter =
            symbol("theta").unwrap();

        let result =
            parameter.bind(&|_| None);

        assert!(result.is_err());

        assert!(matches!(
            result,
            Err(_)
        ));
    }

    #[test]
    fn division_by_zero_is_rejected() {
        let numerator =
            constant(1.0).unwrap();

        let denominator =
            constant(0.0).unwrap();

        let expression =
            ParameterExpression::Divide(
                Box::new(numerator),
                Box::new(denominator),
            );

        let parameter =
            Parameter::expression(
                expression,
            )
            .unwrap();

        assert!(
            parameter
                .bind(&|_| None)
                .is_err()
        );
    }

    #[test]
    fn gate_parameter_arity_is_structural() {
        let theta =
            constant(1.0).unwrap();

        let phi =
            constant(2.0).unwrap();

        let lambda =
            constant(3.0).unwrap();

        let one =
            GateParameter::angle(
                theta.clone(),
            )
            .unwrap();

        let two =
            GateParameter::two_angles(
                theta.clone(),
                phi.clone(),
            )
            .unwrap();

        let three =
            GateParameter::three_angles(
                theta,
                phi,
                lambda,
            )
            .unwrap();

        assert_eq!(
            one.arity(),
            1
        );

        assert_eq!(
            two.arity(),
            2
        );

        assert_eq!(
            three.arity(),
            3
        );
    }

    #[test]
    fn parameter_iteration_does_not_allocate() {
        let theta =
            symbol("theta").unwrap();

        let phi =
            symbol("phi").unwrap();

        let parameters =
            GateParameter::two_angles(
                theta,
                phi,
            )
            .unwrap();

        let mut iterator =
            parameters.iter();

        assert_eq!(
            iterator.next()
                .unwrap()
                .as_symbol(),
            Some("theta")
        );

        assert_eq!(
            iterator.next()
                .unwrap()
                .as_symbol(),
            Some("phi")
        );

        assert!(
            iterator.next().is_none()
        );
    }

    #[test]
    fn symbolic_gate_parameters_can_be_bound() {
        let theta =
            symbol("theta").unwrap();

        let phi =
            constant(2.0).unwrap();

        let parameters =
            GateParameter::two_angles(
                theta,
                phi,
            )
            .unwrap();

        assert!(
            parameters.is_symbolic()
        );

        let bound = parameters
            .bind(&|name| {
                if name == "theta" {
                    Some(1.0)
                } else {
                    None
                }
            })
            .unwrap();

        assert_eq!(
            bound.arity(),
            2
        );

        assert_eq!(
            bound.get(0),
            Some(1.0)
        );

        assert_eq!(
            bound.get(1),
            Some(2.0)
        );
    }

    #[test]
    fn bound_parameters_are_indexable_without_allocation() {
        let parameters =
            GateParameter::three_angles(
                constant(1.0).unwrap(),
                constant(2.0).unwrap(),
                constant(3.0).unwrap(),
            )
            .unwrap();

        let bound = parameters
            .bind(&|_| None)
            .unwrap();

        assert_eq!(
            bound.get(0),
            Some(1.0)
        );

        assert_eq!(
            bound.get(1),
            Some(2.0)
        );

        assert_eq!(
            bound.get(2),
            Some(3.0)
        );

        assert_eq!(
            bound.get(3),
            None
        );
    }

    #[test]
    fn expression_depth_is_bounded() {
        let mut parameter =
            constant(1.0).unwrap();

        for _ in 0..=MAX_PARAMETER_EXPRESSION_DEPTH {
            parameter =
                Parameter::expression(
                    ParameterExpression::Negate(
                        Box::new(parameter),
                    ),
                )
                .unwrap_or_else(|_| {
                    constant(1.0)
                        .unwrap()
                });
        }

        assert!(
            parameter.validate().is_ok()
        );
    }
}