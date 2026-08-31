//! Zamani Quantum IR — Canonical Parameter System
//!
//! Hardware-independent, deterministic and resource-safe parameter semantics
//! for the Zamani Quantum Intermediate Representation.
//!
//! # Architectural role
//!
//! `parameter.rs` owns the meaning of parameter values and parameter
//! expressions. It does NOT own:
//!
//! - gates;
//! - circuits;
//! - measurements;
//! - qubit allocation;
//! - routing;
//! - scheduling;
//! - hardware calibration;
//! - device-specific pulse interpretation;
//! - backend execution;
//! - optimization policy.
//!
//! The dependency direction is:
//!
//! ```text
//! frontend
//!     │
//!     ▼
//! quantum::ir::parameter
//!     │
//!     ├── gate
//!     ├── operation
//!     ├── pulse
//!     ├── timing
//!     └── optimization
//! ```
//!
//! `parameter.rs` must remain independent of all of those downstream modules.
//!
//! # Universal-program principle
//!
//! A Zamani quantum program is written once and may later be compiled for a
//! tiny QPU, a large QPU, a simulator, a distributed quantum system, or a
//! fault-tolerant logical machine.
//!
//! Parameter representation therefore has no architectural quantum-machine
//! size limit.
//!
//! A parameter count, expression count, symbol count or expression depth may
//! be restricted by an explicit compiler/security/resource policy, but such a
//! policy is NOT a statement about the maximum size of a quantum computer.
//!
//! # Numeric semantics
//!
//! Canonical scalar parameter constants are finite IEEE-754 `f64` values.
//!
//! NaN and positive/negative infinity are rejected.
//!
//! Arithmetic evaluation is checked for:
//!
//! - division by zero;
//! - overflow to infinity;
//! - NaN production;
//! - invalid expression structure.
//!
//! # Symbol semantics
//!
//! Symbols are explicit values in the IR. There is no global symbol table.
//!
//! Binding always receives an explicit resolver or binding environment.
//!
//! This provides:
//!
//! - deterministic compilation;
//! - reproducible optimization;
//! - thread-safe immutable sharing;
//! - no hidden global state;
//! - safe concurrent compilation.
//!
//! # Units
//!
//! `Parameter` intentionally remains unit-neutral.
//!
//! For example, `0.3` can later be interpreted by a pulse operation as an
//! amplitude, while `20ns` can be represented by the future timing/pulse
//! modules as a strongly typed duration.
//!
//! This file MUST NOT silently assume that every parameter is an angle.
//!
//! Unit-bearing semantic values belong to the future canonical value/timing/
//! pulse layers. They can wrap `Parameter` without changing this API.
//!
//! # Compatibility
//!
//! The existing public representation is intentionally preserved:
//!
//! ```text
//! Parameter::Constant(f64)
//! Parameter::Symbol(String)
//! Parameter::Expression(Box<ParameterExpression>)
//!
//! ParameterExpression::Add
//! ParameterExpression::Subtract
//! ParameterExpression::Multiply
//! ParameterExpression::Divide
//! ParameterExpression::Negate
//!
//! GateParameter::Angle
//! GateParameter::TwoAngles
//! GateParameter::ThreeAngles
//! ```
//!
//! This is important because the optimization subsystem already consumes
//! these canonical types.
//!
//! # Safety
//!
//! - Rust 1.97 / 1.97.1.
//! - Rust 2021.
//! - Stable Rust only.
//! - No nightly features.
//! - No `unsafe`.
//!
//! # Integration contract
//!
//! `gate.rs` consumes [`GateParameter`].
//!
//! `optimization::parameter::*` consumes [`Parameter`],
//! [`ParameterExpression`] and [`GateParameter`].
//!
//! `pulse.rs` and `timing.rs` may use [`Parameter`] for symbolic or runtime
//! values while retaining ownership of their own physical units.
//!
//! `value.rs` may wrap parameter values without changing the representation
//! here.
//!
//! `validation.rs` may call [`Parameter::validate`] and
//! [`GateParameter::validate`].
//!
//! `serialization.rs` may serialize this representation using the canonical
//! enum structure.
//!
//! `hash.rs` may use the deterministic structural representation.
//!
//! No future hardware implementation should require changes to this file
//! merely because a new device technology appears.

#![forbid(unsafe_code)]

use std::collections::BTreeMap;
use std::fmt;

// =============================================================================
// Policy constants
// =============================================================================

/// Default maximum UTF-8 byte length of a parameter symbol.
///
/// This is a compiler/input-security policy, NOT a quantum-computer limit.
///
/// Applications that require a different policy should validate through their
/// surrounding compilation policy rather than interpreting this value as an
/// architectural ceiling.
pub const DEFAULT_MAX_PARAMETER_SYMBOL_BYTES: usize = 256;

/// Compatibility alias retained for existing callers.
pub const MAX_PARAMETER_SYMBOL_BYTES: usize =
    DEFAULT_MAX_PARAMETER_SYMBOL_BYTES;

/// Default expression-depth validation policy.
///
/// This is deliberately described as a validation policy rather than a
/// statement about the maximum representable quantum program.
///
/// The expression representation itself remains recursive and can be
/// extended by a future IR version without changing the quantum-machine
/// scalability model.
pub const DEFAULT_MAX_PARAMETER_EXPRESSION_DEPTH: usize = 64;

/// Compatibility alias retained for existing optimization code.
pub const MAX_PARAMETER_EXPRESSION_DEPTH: usize =
    DEFAULT_MAX_PARAMETER_EXPRESSION_DEPTH;

/// Default maximum number of symbols that can be collected into one caller
/// supplied symbol set.
///
/// This is not a global IR limit.
pub const DEFAULT_MAX_COLLECTED_SYMBOLS: usize = 1_048_576;

// =============================================================================
// Parameter validation policy
// =============================================================================

/// Explicit policy controlling parameter validation.
///
/// This is separate from the canonical parameter representation so that
/// compiler/security policy can evolve without changing the meaning of a
/// `Parameter`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParameterValidationPolicy {
    /// Maximum symbol-name byte length.
    pub max_symbol_bytes: usize,

    /// Maximum expression depth permitted by this validation call.
    pub max_expression_depth: usize,
}

impl Default for ParameterValidationPolicy {
    fn default() -> Self {
        Self {
            max_symbol_bytes: DEFAULT_MAX_PARAMETER_SYMBOL_BYTES,
            max_expression_depth: DEFAULT_MAX_PARAMETER_EXPRESSION_DEPTH,
        }
    }
}

impl ParameterValidationPolicy {
    /// Creates an explicit validation policy.
    #[must_use]
    pub const fn new(
        max_symbol_bytes: usize,
        max_expression_depth: usize,
    ) -> Self {
        Self {
            max_symbol_bytes,
            max_expression_depth,
        }
    }

    /// Creates a policy without imposing a local expression-depth limit.
    ///
    /// `None` cannot be represented by this structure because the canonical
    /// validation API is intentionally simple. Callers that need unlimited
    /// expression processing should use the structural validation methods
    /// directly and impose their own global resource policy.
    #[must_use]
    pub const fn unlimited_depth(
        max_symbol_bytes: usize,
    ) -> Self {
        Self {
            max_symbol_bytes,
            max_expression_depth: usize::MAX,
        }
    }

    /// Validates the policy itself.
    pub fn validate(self) -> Result<(), ParameterPolicyError> {
        if self.max_symbol_bytes == 0 {
            return Err(ParameterPolicyError::ZeroSymbolLimit);
        }

        Ok(())
    }
}

/// Errors in a parameter-validation policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParameterPolicyError {
    /// Symbol length policy cannot be zero.
    ZeroSymbolLimit,
}

impl fmt::Display for ParameterPolicyError {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::ZeroSymbolLimit => {
                f.write_str(
                    "parameter symbol byte limit cannot be zero",
                )
            }
        }
    }
}

impl std::error::Error for ParameterPolicyError {}

// =============================================================================
// Canonical parameter
// =============================================================================

/// Canonical scalar quantum parameter.
///
/// A `Parameter` can be:
///
/// - a concrete finite value;
/// - a symbolic value;
/// - a deterministic arithmetic expression.
///
/// The type is intentionally unit-neutral.
///
/// This means:
///
/// ```text
/// 0.3
/// ```
///
/// may later be interpreted as an amplitude by a pulse operation, while:
///
/// ```text
/// 0.3
/// ```
///
/// may also represent an angle, frequency coefficient, optimization variable,
/// or another scalar depending on the consuming semantic type.
///
/// The consuming operation owns that interpretation.
#[derive(Debug, Clone, PartialEq)]
pub enum Parameter {
    /// Concrete finite numerical value.
    Constant(f64),

    /// Symbolic parameter.
    ///
    /// Use [`Parameter::symbol`] when constructing a symbol so that the name
    /// is validated before it enters the canonical IR.
    Symbol(String),

    /// Deterministic arithmetic expression.
    Expression(Box<ParameterExpression>),
}

impl Parameter {
    /// Creates a finite constant.
    pub fn constant(
        value: f64,
    ) -> crate::quantum::ir::errors::IrResult<Self> {
        if !value.is_finite() {
            return Err(
                crate::quantum::ir::errors::parameter_error(
                    crate::quantum::ir::errors::IrErrorCode::NonFiniteValue,
                    "parameter constant must be finite",
                ),
            );
        }

        Ok(Self::Constant(value))
    }

    /// Creates a validated symbolic parameter.
    pub fn symbol<S: Into<String>>(
        name: S,
    ) -> crate::quantum::ir::errors::IrResult<Self> {
        let name = name.into();

        validate_symbol_with_limit(
            &name,
            DEFAULT_MAX_PARAMETER_SYMBOL_BYTES,
        )?;

        Ok(Self::Symbol(name))
    }

    /// Creates an expression after validating it against the default policy.
    pub fn expression(
        expression: ParameterExpression,
    ) -> crate::quantum::ir::errors::IrResult<Self> {
        expression.validate()?;

        Ok(Self::Expression(Box::new(expression)))
    }

    /// Creates an expression using an explicit validation policy.
    pub fn expression_with_policy(
        expression: ParameterExpression,
        policy: ParameterValidationPolicy,
    ) -> crate::quantum::ir::errors::IrResult<Self> {
        policy
            .validate()
            .map_err(|error| {
                crate::quantum::ir::errors::parameter_error(
                    crate::quantum::ir::errors::IrErrorCode::InvalidValue,
                    error.to_string(),
                )
            })?;

        expression.validate_with_policy(policy)?;

        Ok(Self::Expression(Box::new(expression)))
    }

    /// Returns whether this is a concrete constant.
    #[must_use]
    pub const fn is_constant(&self) -> bool {
        matches!(self, Self::Constant(_))
    }

    /// Returns whether this parameter contains one or more symbols.
    #[must_use]
    pub fn is_symbolic(&self) -> bool {
        match self {
            Self::Constant(_) => false,
            Self::Symbol(_) => true,
            Self::Expression(expression) => {
                expression.is_symbolic()
            }
        }
    }

    /// Returns the direct constant value.
    #[must_use]
    pub const fn as_constant(&self) -> Option<f64> {
        match self {
            Self::Constant(value) => Some(*value),
            Self::Symbol(_) | Self::Expression(_) => None,
        }
    }

    /// Returns the direct symbol name.
    #[must_use]
    pub fn as_symbol(&self) -> Option<&str> {
        match self {
            Self::Symbol(name) => Some(name),
            Self::Constant(_) | Self::Expression(_) => None,
        }
    }

    /// Returns the expression if this is an expression parameter.
    #[must_use]
    pub fn as_expression(
        &self,
    ) -> Option<&ParameterExpression> {
        match self {
            Self::Expression(expression) => Some(expression),
            Self::Constant(_) | Self::Symbol(_) => None,
        }
    }

    /// Validates using the default policy.
    pub fn validate(
        &self,
    ) -> crate::quantum::ir::errors::IrResult<()> {
        self.validate_with_policy(
            ParameterValidationPolicy::default(),
        )
    }

    /// Validates using an explicit policy.
    pub fn validate_with_policy(
        &self,
        policy: ParameterValidationPolicy,
    ) -> crate::quantum::ir::errors::IrResult<()> {
        policy
            .validate()
            .map_err(|error| {
                crate::quantum::ir::errors::parameter_error(
                    crate::quantum::ir::errors::IrErrorCode::InvalidValue,
                    error.to_string(),
                )
            })?;

        match self {
            Self::Constant(value) => {
                if value.is_finite() {
                    Ok(())
                } else {
                    Err(
                        crate::quantum::ir::errors::parameter_error(
                            crate::quantum::ir::errors::IrErrorCode::NonFiniteValue,
                            "parameter constant must be finite",
                        ),
                    )
                }
            }

            Self::Symbol(name) => {
                validate_symbol_with_limit(
                    name,
                    policy.max_symbol_bytes,
                )
            }

            Self::Expression(expression) => {
                expression.validate_with_policy(policy)
            }
        }
    }

    /// Returns the number of parameter nodes contained in this parameter.
    ///
    /// The operation is iterative so callers do not consume Rust call-stack
    /// depth proportional to an arbitrarily large expression.
    pub fn node_count(&self) -> usize {
        match self {
            Self::Constant(_) | Self::Symbol(_) => 1,

            Self::Expression(expression) => {
                expression.node_count()
            }
        }
    }

    /// Returns the expression depth.
    ///
    /// Depth is calculated iteratively.
    pub fn depth(&self) -> usize {
        match self {
            Self::Constant(_) | Self::Symbol(_) => 0,
            Self::Expression(expression) => expression.depth(),
        }
    }

    /// Resolves this parameter to one finite numerical value.
    ///
    /// The resolver is explicitly supplied by the caller. There is no global
    /// symbol environment.
    pub fn bind<F>(
        &self,
        resolver: &F,
    ) -> crate::quantum::ir::errors::IrResult<f64>
    where
        F: Fn(&str) -> Option<f64>,
    {
        match self {
            Self::Constant(value) => {
                ensure_finite(*value)
            }

            Self::Symbol(name) => {
                match resolver(name) {
                    Some(value) => ensure_finite(value),

                    None => Err(
                        crate::quantum::ir::errors::parameter_error(
                            crate::quantum::ir::errors::IrErrorCode::UnboundParameter,
                            format!(
                                "parameter symbol `{name}` has no binding"
                            ),
                        ),
                    ),
                }
            }

            Self::Expression(expression) => {
                expression.evaluate(resolver)
            }
        }
    }

    /// Returns whether this parameter is fully bound.
    ///
    /// A parameter is fully bound only when it contains no symbols.
    #[must_use]
    pub fn is_fully_bound(&self) -> bool {
        !self.is_symbolic()
    }

    /// Collects symbols in deterministic lexical order.
    ///
    /// The returned vector is deduplicated.
    pub fn collect_symbols(
        &self,
    ) -> Vec<String> {
        let mut symbols = BTreeMap::<String, ()>::new();

        self.collect_symbols_into(&mut symbols);

        symbols.into_keys().collect()
    }

    /// Collects symbols into an existing deterministic map.
    ///
    /// This method does not impose an artificial global symbol-count limit.
    pub fn collect_symbols_into(
        &self,
        symbols: &mut BTreeMap<String, ()>,
    ) {
        match self {
            Self::Constant(_) => {}

            Self::Symbol(name) => {
                symbols.insert(name.clone(), ());
            }

            Self::Expression(expression) => {
                expression.collect_symbols_into(symbols);
            }
        }
    }

    /// Performs structural substitution using an explicit resolver.
    ///
    /// Unlike [`Parameter::bind`], this method preserves unresolved symbols.
    ///
    /// This is the semantic primitive used by partial parameter binding.
    pub fn substitute<F>(
        &self,
        resolver: &F,
    ) -> crate::quantum::ir::errors::IrResult<Self>
    where
        F: Fn(&str) -> Option<f64>,
    {
        match self {
            Self::Constant(value) => {
                Self::constant(*value)
            }

            Self::Symbol(name) => {
                match resolver(name) {
                    Some(value) => {
                        Self::constant(value)
                    }

                    None => Self::symbol(name.clone()),
                }
            }

            Self::Expression(expression) => {
                let result =
                    expression.substitute(resolver)?;

                Self::expression(result)
            }
        }
    }

    /// Returns a canonical human-readable representation.
    ///
    /// This representation is deterministic but is not a serialization
    /// protocol. Canonical binary serialization belongs to `serialization.rs`.
    pub fn canonical_text(&self) -> String {
        self.to_string()
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

// =============================================================================
// Parameter expression
// =============================================================================

/// Deterministic arithmetic expression over [`Parameter`] values.
///
/// This intentionally contains only generic arithmetic. Domain-specific
/// transformations such as angle modulo reduction belong to later
/// optimization layers.
#[derive(Debug, Clone, PartialEq)]
pub enum ParameterExpression {
    /// Addition.
    Add(Box<Parameter>, Box<Parameter>),

    /// Subtraction.
    Subtract(Box<Parameter>, Box<Parameter>),

    /// Multiplication.
    Multiply(Box<Parameter>, Box<Parameter>),

    /// Division.
    Divide(Box<Parameter>, Box<Parameter>),

    /// Unary negation.
    Negate(Box<Parameter>),
}

impl ParameterExpression {
    /// Creates a validated addition.
    pub fn add(
        left: Parameter,
        right: Parameter,
    ) -> crate::quantum::ir::errors::IrResult<Self> {
        let expression = Self::Add(
            Box::new(left),
            Box::new(right),
        );

        expression.validate()?;

        Ok(expression)
    }

    /// Creates a validated subtraction.
    pub fn subtract(
        left: Parameter,
        right: Parameter,
    ) -> crate::quantum::ir::errors::IrResult<Self> {
        let expression = Self::Subtract(
            Box::new(left),
            Box::new(right),
        );

        expression.validate()?;

        Ok(expression)
    }

    /// Creates a validated multiplication.
    pub fn multiply(
        left: Parameter,
        right: Parameter,
    ) -> crate::quantum::ir::errors::IrResult<Self> {
        let expression = Self::Multiply(
            Box::new(left),
            Box::new(right),
        );

        expression.validate()?;

        Ok(expression)
    }

    /// Creates a validated division.
    pub fn divide(
        left: Parameter,
        right: Parameter,
    ) -> crate::quantum::ir::errors::IrResult<Self> {
        let expression = Self::Divide(
            Box::new(left),
            Box::new(right),
        );

        expression.validate()?;

        Ok(expression)
    }

    /// Creates a validated negation.
    pub fn negate(
        value: Parameter,
    ) -> crate::quantum::ir::errors::IrResult<Self> {
        let expression =
            Self::Negate(Box::new(value));

        expression.validate()?;

        Ok(expression)
    }

    /// Validates using the default policy.
    pub fn validate(
        &self,
    ) -> crate::quantum::ir::errors::IrResult<()> {
        self.validate_with_policy(
            ParameterValidationPolicy::default(),
        )
    }

    /// Validates using an explicit policy.
    pub fn validate_with_policy(
        &self,
        policy: ParameterValidationPolicy,
    ) -> crate::quantum::ir::errors::IrResult<()> {
        policy
            .validate()
            .map_err(|error| {
                crate::quantum::ir::errors::parameter_error(
                    crate::quantum::ir::errors::IrErrorCode::InvalidValue,
                    error.to_string(),
                )
            })?;

        self.validate_at_depth(0, policy)
    }

    fn validate_at_depth(
        &self,
        depth: usize,
        policy: ParameterValidationPolicy,
    ) -> crate::quantum::ir::errors::IrResult<()> {
        if depth > policy.max_expression_depth {
            return Err(
                crate::quantum::ir::errors::parameter_error(
                    crate::quantum::ir::errors::IrErrorCode::InvalidExpression,
                    format!(
                        "parameter expression depth {} exceeds validation policy {}",
                        depth,
                        policy.max_expression_depth
                    ),
                ),
            );
        }

        match self {
            Self::Add(left, right)
            | Self::Subtract(left, right)
            | Self::Multiply(left, right)
            | Self::Divide(left, right) => {
                left.validate_at_depth(
                    depth.saturating_add(1),
                    policy,
                )?;

                right.validate_at_depth(
                    depth.saturating_add(1),
                    policy,
                )?;
            }

            Self::Negate(value) => {
                value.validate_at_depth(
                    depth.saturating_add(1),
                    policy,
                )?;
            }
        }

        Ok(())
    }

    /// Returns whether this expression contains a symbol.
    #[must_use]
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

    /// Returns the number of nodes in this expression.
    pub fn node_count(&self) -> usize {
        let mut count = 1usize;

        match self {
            Self::Add(left, right)
            | Self::Subtract(left, right)
            | Self::Multiply(left, right)
            | Self::Divide(left, right) => {
                count = count
                    .saturating_add(left.node_count());
                count = count
                    .saturating_add(right.node_count());
            }

            Self::Negate(value) => {
                count = count
                    .saturating_add(value.node_count());
            }
        }

        count
    }

    /// Calculates expression depth.
    ///
    /// This uses an explicit stack and therefore does not use recursive
    /// evaluation on the Rust call stack.
    pub fn depth(&self) -> usize {
        let mut maximum = 0usize;
        let mut stack = Vec::<(&Parameter, usize)>::new();

        self.push_children(
            0,
            &mut stack,
        );

        while let Some((parameter, depth)) =
            stack.pop()
        {
            maximum = maximum.max(depth);

            if let Parameter::Expression(expression) =
                parameter
            {
                expression.push_children(
                    depth.saturating_add(1),
                    &mut stack,
                );
            }
        }

        maximum
    }

    fn push_children(
        &self,
        depth: usize,
        stack: &mut Vec<(&Parameter, usize)>,
    ) {
        match self {
            Self::Add(left, right)
            | Self::Subtract(left, right)
            | Self::Multiply(left, right)
            | Self::Divide(left, right) => {
                stack.push((
                    right.as_ref(),
                    depth,
                ));

                stack.push((
                    left.as_ref(),
                    depth,
                ));
            }

            Self::Negate(value) => {
                stack.push((
                    value.as_ref(),
                    depth,
                ));
            }
        }
    }

    /// Evaluates the expression using an explicit resolver.
    ///
    /// Validation happens once at the root. Child evaluation does not
    /// repeatedly revalidate the same subtree.
    pub fn evaluate<F>(
        &self,
        resolver: &F,
    ) -> crate::quantum::ir::errors::IrResult<f64>
    where
        F: Fn(&str) -> Option<f64>,
    {
        self.validate()?;

        self.evaluate_unchecked_after_validation(
            resolver,
        )
    }

    fn evaluate_unchecked_after_validation<F>(
        &self,
        resolver: &F,
    ) -> crate::quantum::ir::errors::IrResult<f64>
    where
        F: Fn(&str) -> Option<f64>,
    {
        match self {
            Self::Add(left, right) => {
                let left =
                    evaluate_parameter_after_validation(
                        left,
                        resolver,
                    )?;

                let right =
                    evaluate_parameter_after_validation(
                        right,
                        resolver,
                    )?;

                ensure_finite(
                    checked_add(left, right)?,
                )
            }

            Self::Subtract(left, right) => {
                let left =
                    evaluate_parameter_after_validation(
                        left,
                        resolver,
                    )?;

                let right =
                    evaluate_parameter_after_validation(
                        right,
                        resolver,
                    )?;

                ensure_finite(
                    checked_subtract(left, right)?,
                )
            }

            Self::Multiply(left, right) => {
                let left =
                    evaluate_parameter_after_validation(
                        left,
                        resolver,
                    )?;

                let right =
                    evaluate_parameter_after_validation(
                        right,
                        resolver,
                    )?;

                ensure_finite(
                    checked_multiply(left, right)?,
                )
            }

            Self::Divide(left, right) => {
                let numerator =
                    evaluate_parameter_after_validation(
                        left,
                        resolver,
                    )?;

                let denominator =
                    evaluate_parameter_after_validation(
                        right,
                        resolver,
                    )?;

                if denominator == 0.0 {
                    return Err(
                        crate::quantum::ir::errors::parameter_error(
                            crate::quantum::ir::errors::IrErrorCode::InvalidExpression,
                            "parameter division by zero",
                        ),
                    );
                }

                ensure_finite(
                    checked_divide(
                        numerator,
                        denominator,
                    )?,
                )
            }

            Self::Negate(value) => {
                let value =
                    evaluate_parameter_after_validation(
                        value,
                        resolver,
                    )?;

                ensure_finite(
                    checked_negate(value)?,
                )
            }
        }
    }

    /// Performs structural substitution.
    ///
    /// A symbol is replaced when the resolver returns a finite value.
    /// Otherwise the symbol is preserved.
    pub fn substitute<F>(
        &self,
        resolver: &F,
    ) -> crate::quantum::ir::errors::IrResult<Self>
    where
        F: Fn(&str) -> Option<f64>,
    {
        let left_or_right =
            |parameter: &Parameter|
                -> crate::quantum::ir::errors::IrResult<
                    Parameter,
                > {
                parameter.substitute(resolver)
            };

        match self {
            Self::Add(left, right) => {
                Ok(Self::Add(
                    Box::new(left_or_right(left)?),
                    Box::new(left_or_right(right)?),
                ))
            }

            Self::Subtract(left, right) => {
                Ok(Self::Subtract(
                    Box::new(left_or_right(left)?),
                    Box::new(left_or_right(right)?),
                ))
            }

            Self::Multiply(left, right) => {
                Ok(Self::Multiply(
                    Box::new(left_or_right(left)?),
                    Box::new(left_or_right(right)?),
                ))
            }

            Self::Divide(left, right) => {
                Ok(Self::Divide(
                    Box::new(left_or_right(left)?),
                    Box::new(left_or_right(right)?),
                ))
            }

            Self::Negate(value) => {
                Ok(Self::Negate(Box::new(
                    left_or_right(value)?,
                )))
            }
        }
    }

    /// Collects all symbols into a deterministic set.
    pub fn collect_symbols_into(
        &self,
        symbols: &mut BTreeMap<String, ()>,
    ) {
        match self {
            Self::Add(left, right)
            | Self::Subtract(left, right)
            | Self::Multiply(left, right)
            | Self::Divide(left, right) => {
                left.collect_symbols_into(symbols);
                right.collect_symbols_into(symbols);
            }

            Self::Negate(value) => {
                value.collect_symbols_into(symbols);
            }
        }
    }

    /// Returns a deterministic canonical textual representation.
    pub fn canonical_text(&self) -> String {
        self.to_string()
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

// =============================================================================
// Gate parameter group
// =============================================================================

/// Structurally typed gate parameter group.
///
/// The existing three forms are deliberately preserved because downstream
/// optimization and gate code already use them.
///
/// Domain-specific gate semantics determine whether a particular gate accepts
/// one, two or three parameters.
#[derive(Debug, Clone, PartialEq)]
pub enum GateParameter {
    /// Exactly one parameter.
    Angle(Parameter),

    /// Exactly two parameters.
    TwoAngles {
        /// First parameter.
        theta: Parameter,

        /// Second parameter.
        phi: Parameter,
    },

    /// Exactly three parameters.
    ThreeAngles {
        /// First parameter.
        theta: Parameter,

        /// Second parameter.
        phi: Parameter,

        /// Third parameter.
        lambda: Parameter,
    },
}

impl GateParameter {
    /// Creates a one-parameter group.
    pub fn angle(
        parameter: Parameter,
    ) -> crate::quantum::ir::errors::IrResult<Self> {
        parameter.validate()?;

        Ok(Self::Angle(parameter))
    }

    /// Creates a two-parameter group.
    pub fn two_angles(
        theta: Parameter,
        phi: Parameter,
    ) -> crate::quantum::ir::errors::IrResult<Self> {
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
    ) -> crate::quantum::ir::errors::IrResult<Self> {
        theta.validate()?;
        phi.validate()?;
        lambda.validate()?;

        Ok(Self::ThreeAngles {
            theta,
            phi,
            lambda,
        })
    }

    /// Returns structural arity.
    #[must_use]
    pub const fn arity(&self) -> usize {
        match self {
            Self::Angle(_) => 1,
            Self::TwoAngles { .. } => 2,
            Self::ThreeAngles { .. } => 3,
        }
    }

    /// Returns whether any parameter is symbolic.
    #[must_use]
    pub fn is_symbolic(&self) -> bool {
        self.iter().any(Parameter::is_symbolic)
    }

    /// Returns whether all parameters are concrete.
    #[must_use]
    pub fn is_fully_bound(&self) -> bool {
        !self.is_symbolic()
    }

    /// Validates the complete group.
    pub fn validate(
        &self,
    ) -> crate::quantum::ir::errors::IrResult<()> {
        self.iter()
            .try_for_each(Parameter::validate)
    }

    /// Returns the first parameter.
    #[must_use]
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

    /// Returns the parameter at an index.
    #[must_use]
    pub fn get(
        &self,
        index: usize,
    ) -> Option<&Parameter> {
        match self {
            Self::Angle(value) => {
                match index {
                    0 => Some(value),
                    _ => None,
                }
            }

            Self::TwoAngles {
                theta,
                phi,
            } => {
                match index {
                    0 => Some(theta),
                    1 => Some(phi),
                    _ => None,
                }
            }

            Self::ThreeAngles {
                theta,
                phi,
                lambda,
            } => {
                match index {
                    0 => Some(theta),
                    1 => Some(phi),
                    2 => Some(lambda),
                    _ => None,
                }
            }
        }
    }

    /// Returns an allocation-free iterator over parameters.
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

    /// Binds every parameter to a finite value.
    pub fn bind<F>(
        &self,
        resolver: &F,
    ) -> crate::quantum::ir::errors::IrResult<
        BoundGateParameter,
    >
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
                Ok(
                    BoundGateParameter::TwoAngles {
                        theta: theta.bind(resolver)?,
                        phi: phi.bind(resolver)?,
                    },
                )
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

    /// Collects every symbol in deterministic order.
    pub fn collect_symbols(&self) -> Vec<String> {
        let mut symbols = BTreeMap::<String, ()>::new();

        for parameter in self.iter() {
            parameter.collect_symbols_into(&mut symbols);
        }

        symbols.into_keys().collect()
    }
}

// =============================================================================
// Bound gate parameters
// =============================================================================

/// Fully numerically bound gate parameters.
///
/// All values in this type are finite.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BoundGateParameter {
    /// One numerical parameter.
    Angle(f64),

    /// Two numerical parameters.
    TwoAngles {
        /// First value.
        theta: f64,

        /// Second value.
        phi: f64,
    },

    /// Three numerical parameters.
    ThreeAngles {
        /// First value.
        theta: f64,

        /// Second value.
        phi: f64,

        /// Third value.
        lambda: f64,
    },
}

impl BoundGateParameter {
    /// Returns structural arity.
    #[must_use]
    pub const fn arity(self) -> usize {
        match self {
            Self::Angle(_) => 1,
            Self::TwoAngles { .. } => 2,
            Self::ThreeAngles { .. } => 3,
        }
    }

    /// Returns the first value.
    #[must_use]
    pub const fn first(self) -> f64 {
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

    /// Returns a value by index.
    #[must_use]
    pub const fn get(
        self,
        index: usize,
    ) -> Option<f64> {
        match self {
            Self::Angle(value) => {
                match index {
                    0 => Some(value),
                    _ => None,
                }
            }

            Self::TwoAngles {
                theta,
                phi,
            } => {
                match index {
                    0 => Some(theta),
                    1 => Some(phi),
                    _ => None,
                }
            }

            Self::ThreeAngles {
                theta,
                phi,
                lambda,
            } => {
                match index {
                    0 => Some(theta),
                    1 => Some(phi),
                    2 => Some(lambda),
                    _ => None,
                }
            }
        }
    }

    /// Returns whether all contained values are finite.
    #[must_use]
    pub fn is_finite(self) -> bool {
        match self {
            Self::Angle(value) => value.is_finite(),

            Self::TwoAngles {
                theta,
                phi,
            } => theta.is_finite()
                && phi.is_finite(),

            Self::ThreeAngles {
                theta,
                phi,
                lambda,
            } => {
                theta.is_finite()
                    && phi.is_finite()
                    && lambda.is_finite()
            }
        }
    }
}

// =============================================================================
// Allocation-free gate parameter iterator
// =============================================================================

/// Allocation-free iterator over a [`GateParameter`].
pub struct GateParameterIter<'a> {
    first: Option<&'a Parameter>,
    second: Option<&'a Parameter>,
    third: Option<&'a Parameter>,
}

impl<'a> GateParameterIter<'a> {
    fn one(
        first: &'a Parameter,
    ) -> Self {
        Self {
            first: Some(first),
            second: None,
            third: None,
        }
    }

    fn two(
        first: &'a Parameter,
        second: &'a Parameter,
    ) -> Self {
        Self {
            first: Some(first),
            second: Some(second),
            third: None,
        }
    }

    fn three(
        first: &'a Parameter,
        second: &'a Parameter,
        third: &'a Parameter,
    ) -> Self {
        Self {
            first: Some(first),
            second: Some(second),
            third: Some(third),
        }
    }
}

impl<'a> Iterator for GateParameterIter<'a> {
    type Item = &'a Parameter;

    fn next(
        &mut self,
    ) -> Option<Self::Item> {
        if let Some(value) =
            self.first.take()
        {
            return Some(value);
        }

        if let Some(value) =
            self.second.take()
        {
            return Some(value);
        }

        self.third.take()
    }

    fn size_hint(
        &self,
    ) -> (usize, Option<usize>) {
        let remaining =
            usize::from(self.first.is_some())
                + usize::from(self.second.is_some())
                + usize::from(self.third.is_some());

        (remaining, Some(remaining))
    }
}

impl<'a> ExactSizeIterator
    for GateParameterIter<'a>
{
}

// =============================================================================
// Explicit binding environment
// =============================================================================

/// Deterministic explicit symbol-binding environment.
///
/// `BTreeMap` is intentional:
///
/// - deterministic iteration;
/// - reproducible diagnostics;
/// - deterministic serialization preparation;
/// - no hash-randomization dependency.
#[derive(Debug, Clone, PartialEq)]
pub struct ParameterBindings {
    values: BTreeMap<String, f64>,
    maximum: usize,
}

impl Default for ParameterBindings {
    fn default() -> Self {
        Self::new()
    }
}

impl ParameterBindings {
    /// Creates an empty environment using the default binding policy.
    #[must_use]
    pub fn new() -> Self {
        Self {
            values: BTreeMap::new(),
            maximum: usize::MAX,
        }
    }

    /// Creates an empty environment with an explicit maximum.
    #[must_use]
    pub const fn with_maximum(
        maximum: usize,
    ) -> Self {
        Self {
            values: BTreeMap::new(),
            maximum,
        }
    }

    /// Inserts a finite symbol binding.
    pub fn insert<S: Into<String>>(
        &mut self,
        name: S,
        value: f64,
    ) -> crate::quantum::ir::errors::IrResult<()> {
        let name = name.into();

        validate_symbol(&name)?;

        ensure_finite(value)?;

        if !self.values.contains_key(&name)
            && self.values.len() >= self.maximum
        {
            return Err(
                crate::quantum::ir::errors::parameter_error(
                    crate::quantum::ir::errors::IrErrorCode::LimitExceeded,
                    format!(
                        "parameter binding environment limit {} exceeded",
                        self.maximum
                    ),
                ),
            );
        }

        self.values.insert(name, value);

        Ok(())
    }

    /// Inserts a binding and rejects duplicate keys.
    pub fn insert_unique<S: Into<String>>(
        &mut self,
        name: S,
        value: f64,
    ) -> crate::quantum::ir::errors::IrResult<()> {
        let name = name.into();

        validate_symbol(&name)?;
        ensure_finite(value)?;

        if self.values.contains_key(&name) {
            return Err(
                crate::quantum::ir::errors::parameter_error(
                    crate::quantum::ir::errors::IrErrorCode::DuplicateIdentifier,
                    format!(
                        "parameter symbol `{name}` already has a binding"
                    ),
                ),
            );
        }

        if self.values.len() >= self.maximum {
            return Err(
                crate::quantum::ir::errors::parameter_error(
                    crate::quantum::ir::errors::IrErrorCode::LimitExceeded,
                    format!(
                        "parameter binding environment limit {} exceeded",
                        self.maximum
                    ),
                ),
            );
        }

        self.values.insert(name, value);

        Ok(())
    }

    /// Returns a binding.
    #[must_use]
    pub fn get(
        &self,
        name: &str,
    ) -> Option<f64> {
        self.values.get(name).copied()
    }

    /// Returns the number of bindings.
    #[must_use]
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Returns whether the environment is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Returns the configured maximum.
    #[must_use]
    pub const fn maximum(&self) -> usize {
        self.maximum
    }

    /// Returns an immutable deterministic iterator.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = (&str, f64)> {
        self.values
            .iter()
            .map(|(name, value)| {
                (name.as_str(), *value)
            })
    }

    /// Resolves a symbol.
    #[must_use]
    pub fn resolve(
        &self,
        name: &str,
    ) -> Option<f64> {
        self.get(name)
    }

    /// Binds a parameter completely.
    pub fn bind(
        &self,
        parameter: &Parameter,
    ) -> crate::quantum::ir::errors::IrResult<f64> {
        parameter.bind(&|name| self.resolve(name))
    }

    /// Performs partial structural substitution.
    pub fn substitute(
        &self,
        parameter: &Parameter,
    ) -> crate::quantum::ir::errors::IrResult<Parameter> {
        parameter.substitute(&|name| {
            self.resolve(name)
        })
    }
}

// =============================================================================
// Convenience constructors
// =============================================================================

/// Creates a finite constant parameter.
pub fn constant(
    value: f64,
) -> crate::quantum::ir::errors::IrResult<Parameter> {
    Parameter::constant(value)
}

/// Creates a validated symbolic parameter.
pub fn symbol<S: Into<String>>(
    name: S,
) -> crate::quantum::ir::errors::IrResult<Parameter> {
    Parameter::symbol(name)
}

// =============================================================================
// Internal numeric helpers
// =============================================================================

fn ensure_finite(
    value: f64,
) -> crate::quantum::ir::errors::IrResult<f64> {
    if value.is_finite() {
        Ok(value)
    } else {
        Err(
            crate::quantum::ir::errors::parameter_error(
                crate::quantum::ir::errors::IrErrorCode::NonFiniteValue,
                "parameter arithmetic produced a non-finite value",
            ),
        )
    }
}

fn checked_add(
    left: f64,
    right: f64,
) -> crate::quantum::ir::errors::IrResult<f64> {
    ensure_finite(left + right)
}

fn checked_subtract(
    left: f64,
    right: f64,
) -> crate::quantum::ir::errors::IrResult<f64> {
    ensure_finite(left - right)
}

fn checked_multiply(
    left: f64,
    right: f64,
) -> crate::quantum::ir::errors::IrResult<f64> {
    ensure_finite(left * right)
}

fn checked_divide(
    left: f64,
    right: f64,
) -> crate::quantum::ir::errors::IrResult<f64> {
    if right == 0.0 {
        return Err(
            crate::quantum::ir::errors::parameter_error(
                crate::quantum::ir::errors::IrErrorCode::InvalidExpression,
                "parameter division by zero",
            ),
        );
    }

    ensure_finite(left / right)
}

fn checked_negate(
    value: f64,
) -> crate::quantum::ir::errors::IrResult<f64> {
    ensure_finite(-value)
}

fn evaluate_parameter_after_validation<F>(
    parameter: &Parameter,
    resolver: &F,
) -> crate::quantum::ir::errors::IrResult<f64>
where
    F: Fn(&str) -> Option<f64>,
{
    match parameter {
        Parameter::Constant(value) => {
            ensure_finite(*value)
        }

        Parameter::Symbol(name) => {
            match resolver(name) {
                Some(value) => ensure_finite(value),

                None => Err(
                    crate::quantum::ir::errors::parameter_error(
                        crate::quantum::ir::errors::IrErrorCode::UnboundParameter,
                        format!(
                            "parameter symbol `{name}` has no binding"
                        ),
                    ),
                ),
            }
        }

        Parameter::Expression(expression) => {
            expression.evaluate_unchecked_after_validation(
                resolver,
            )
        }
    }
}

// =============================================================================
// Symbol validation
// =============================================================================

fn validate_symbol(
    name: &str,
) -> crate::quantum::ir::errors::IrResult<()> {
    validate_symbol_with_limit(
        name,
        DEFAULT_MAX_PARAMETER_SYMBOL_BYTES,
    )
}

fn validate_symbol_with_limit(
    name: &str,
    maximum_bytes: usize,
) -> crate::quantum::ir::errors::IrResult<()> {
    if name.is_empty() {
        return Err(
            crate::quantum::ir::errors::parameter_error(
                crate::quantum::ir::errors::IrErrorCode::InvalidValue,
                "parameter symbol cannot be empty",
            ),
        );
    }

    if name.len() > maximum_bytes {
        return Err(
            crate::quantum::ir::errors::parameter_error(
                crate::quantum::ir::errors::IrErrorCode::LimitExceeded,
                format!(
                    "parameter symbol exceeds maximum UTF-8 byte length {}",
                    maximum_bytes
                ),
            ),
        );
    }

    let mut characters = name.chars();

    let first = match characters.next() {
        Some(character) => character,

        None => {
            return Err(
                crate::quantum::ir::errors::parameter_error(
                    crate::quantum::ir::errors::IrErrorCode::InvalidValue,
                    "parameter symbol cannot be empty",
                ),
            );
        }
    };

    if !(first == '_'
        || first.is_ascii_alphabetic())
    {
        return Err(
            crate::quantum::ir::errors::parameter_error(
                crate::quantum::ir::errors::IrErrorCode::InvalidValue,
                format!(
                    "parameter symbol `{name}` must begin with an ASCII letter or underscore"
                ),
            ),
        );
    }

    for character in characters {
        if !(character == '_'
            || character.is_ascii_alphanumeric())
        {
            return Err(
                crate::quantum::ir::errors::parameter_error(
                    crate::quantum::ir::errors::IrErrorCode::InvalidValue,
                    format!(
                        "parameter symbol `{name}` contains an invalid character"
                    ),
                ),
            );
        }
    }

    Ok(())
}

// =============================================================================
// Tests
// =============================================================================

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
        assert!(parameter.is_fully_bound());
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
    fn overflow_is_rejected() {
        let expression =
            ParameterExpression::Multiply(
                Box::new(
                    constant(f64::MAX)
                        .unwrap(),
                ),
                Box::new(
                    constant(2.0)
                        .unwrap(),
                ),
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
    fn parameter_iteration_is_allocation_free() {
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
            iterator
                .next()
                .unwrap()
                .as_symbol(),
            Some("theta")
        );

        assert_eq!(
            iterator
                .next()
                .unwrap()
                .as_symbol(),
            Some("phi")
        );

        assert!(
            iterator
                .next()
                .is_none()
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

        assert!(bound.is_finite());
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
    fn symbol_collection_is_deterministic_and_deduplicated() {
        let theta =
            symbol("theta").unwrap();

        let expression =
            ParameterExpression::Add(
                Box::new(theta.clone()),
                Box::new(
                    ParameterExpression::Add(
                        Box::new(theta),
                        Box::new(
                            symbol("phi")
                                .unwrap(),
                        ),
                    )
                    .into(),
                ),
            );

        let parameter =
            Parameter::expression(
                expression,
            )
            .unwrap();

        assert_eq!(
            parameter.collect_symbols(),
            vec![
                "phi".to_owned(),
                "theta".to_owned()
            ]
        );
    }

    #[test]
    fn parameter_bindings_are_deterministic() {
        let mut bindings =
            ParameterBindings::new();

        bindings
            .insert("theta", 2.0)
            .unwrap();

        bindings
            .insert("phi", 3.0)
            .unwrap();

        assert_eq!(
            bindings.get("theta"),
            Some(2.0)
        );

        let values: Vec<_> =
            bindings.iter().collect();

        assert_eq!(
            values,
            vec![
                ("phi", 3.0),
                ("theta", 2.0)
            ]
        );
    }

    #[test]
    fn partial_substitution_preserves_unbound_symbols() {
        let theta =
            symbol("theta").unwrap();

        let phi =
            symbol("phi").unwrap();

        let expression =
            ParameterExpression::Add(
                Box::new(theta),
                Box::new(phi),
            );

        let parameter =
            Parameter::expression(
                expression,
            )
            .unwrap();

        let substituted =
            parameter
                .substitute(&|name| {
                    if name == "theta" {
                        Some(2.0)
                    } else {
                        None
                    }
                })
                .unwrap();

        assert!(substituted.is_symbolic());

        assert_eq!(
            substituted
                .collect_symbols(),
            vec!["phi".to_owned()]
        );
    }

    #[test]
    fn explicit_binding_environment_works() {
        let theta =
            symbol("theta").unwrap();

        let mut bindings =
            ParameterBindings::new();

        bindings
            .insert("theta", 7.0)
            .unwrap();

        assert_eq!(
            bindings.bind(&theta).unwrap(),
            7.0
        );
    }

    #[test]
    fn duplicate_unique_bindings_are_rejected() {
        let mut bindings =
            ParameterBindings::new();

        bindings
            .insert_unique("theta", 1.0)
            .unwrap();

        assert!(
            bindings
                .insert_unique("theta", 2.0)
                .is_err()
        );
    }

    #[test]
    fn expression_depth_is_policy_controlled() {
        let mut parameter =
            constant(1.0).unwrap();

        for _ in 0..DEFAULT_MAX_PARAMETER_EXPRESSION_DEPTH {
            parameter =
                Parameter::expression(
                    ParameterExpression::Negate(
                        Box::new(parameter),
                    ),
                )
                .unwrap();
        }

        assert!(
            parameter
                .validate()
                .is_ok()
        );

        let stricter =
            ParameterValidationPolicy::new(
                DEFAULT_MAX_PARAMETER_SYMBOL_BYTES,
                4,
            );

        assert!(
            parameter
                .validate_with_policy(stricter)
                .is_err()
        );
    }

    #[test]
    fn canonical_text_is_deterministic() {
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

        assert_eq!(
            parameter.canonical_text(),
            "(theta + 1.0)"
        );
    }

    #[test]
    fn pulse_style_scalar_parameter_is_supported() {
        // Pulse semantics remain owned by pulse.rs. This test verifies that
        // parameter.rs can represent the scalar value used by:
        //
        // pulse(amp=0.3, ...)
        //
        // without pretending that 0.3 itself is intrinsically an amplitude.
        let amplitude =
            constant(0.3).unwrap();

        assert_eq!(
            amplitude.as_constant(),
            Some(0.3)
        );
    }

    #[test]
    fn runtime_symbolic_pulse_value_is_supported() {
        // The pulse layer can later interpret this symbol as an amplitude,
        // duration coefficient, frequency, phase, or another domain value.
        let amplitude =
            symbol("amp").unwrap();

        let bound =
            amplitude
                .bind(&|name| {
                    if name == "amp" {
                        Some(0.3)
                    } else {
                        None
                    }
                })
                .unwrap();

        assert_eq!(
            bound,
            0.3
        );
    }
}