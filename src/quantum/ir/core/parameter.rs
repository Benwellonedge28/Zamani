//! Zamani Quantum IR — Canonical Parameter System
//!
//! Production-grade, hardware-independent symbolic parameter semantics for the
//! Zamani Quantum Intermediate Representation.
//!
//! # Ownership
//!
//! This module owns:
//!
//! - scalar parameter values;
//! - symbolic parameters;
//! - parameter expressions;
//! - deterministic parameter binding;
//! - partial binding/substitution;
//! - constant folding of parameter expressions;
//! - expression inspection;
//! - deterministic symbol collection;
//! - parameter validation;
//! - gate-parameter compatibility wrappers.
//!
//! This module does NOT own:
//!
//! - qubits;
//! - gates themselves;
//! - circuits;
//! - operations;
//! - measurements;
//! - routing;
//! - scheduling;
//! - hardware;
//! - calibration;
//! - pulse generation;
//! - optimization policy;
//! - frontend syntax;
//! - backend execution.
//!
//! # Canonical dependency direction
//!
//! ```text
//!                 quantum::ir::core
//!                        │
//!                        ▼
//!                 parameter.rs
//!                        │
//!             ┌──────────┼──────────┐
//!             ▼          ▼          ▼
//!           gate     operation     pulse
//!             │          │          │
//!             └──────────┼──────────┘
//!                        ▼
//!                  optimization
//! ```
//!
//! `parameter.rs` MUST remain independent of those downstream modules.
//!
//! # Universal-program principle
//!
//! A Zamani quantum program is written once at the semantic level and can be
//! lowered to any compatible target.
//!
//! Parameter representation therefore contains no quantum-machine-size
//! assumptions.
//!
//! In particular, this module MUST NOT define:
//!
//! - maximum qubit count;
//! - maximum register count;
//! - maximum gate count;
//! - maximum parameter count as an architectural rule;
//! - maximum program size;
//! - maximum machine size.
//!
//! A compiler, service, or security boundary may impose an explicit
//! `ParameterValidationPolicy`, but that policy is execution policy and never
//! changes the semantic parameter model.
//!
//! # Numeric semantics
//!
//! Concrete parameter values are finite IEEE-754 `f64` values.
//!
//! NaN and positive/negative infinity are rejected at the canonical
//! construction/validation boundary.
//!
//! Arithmetic evaluation is checked for:
//!
//! - division by zero;
//! - non-finite intermediate results;
//! - non-finite final results;
//! - invalid domains for mathematical functions;
//! - invalid symbolic bindings.
//!
//! # Symbol semantics
//!
//! Symbols have no hidden global environment.
//!
//! Binding requires an explicitly supplied resolver or `ParameterBindings`
//! object. This guarantees:
//!
//! - deterministic compilation;
//! - reproducible optimization;
//! - thread-safe immutable parameter values;
//! - no global mutable state;
//! - safe concurrent compilation;
//! - explicit dependency tracking.
//!
//! # Units
//!
//! `Parameter` is intentionally unit-neutral.
//!
//! A value such as:
//!
//! ```text
//! 0.3
//! ```
//!
//! can be interpreted by a pulse operation as an amplitude, by a gate as an
//! angle, or by another semantic operation as another scalar quantity.
//!
//! Unit ownership belongs to the consuming IR type.
//!
//! # Mathematical functions
//!
//! Parameter expressions support a generic set of scalar mathematical
//! functions:
//!
//! - sin
//! - cos
//! - tan
//! - exp
//! - ln
//! - sqrt
//! - abs
//! - floor
//! - ceil
//! - round
//!
//! The representation is extensible through `Function`.
//!
//! # Compatibility
//!
//! Existing gate-facing concepts remain available:
//!
//! ```text
//! Parameter
//! ParameterExpression
//! GateParameter
//! BoundGateParameter
//! ```
//!
//! The standard gate layer can therefore consume this module without knowing
//! how parameters are represented internally.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe.
//!
//! The module explicitly forbids unsafe code.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

// =============================================================================
// Policy
// =============================================================================

/// Explicit validation policy for parameter structures.
///
/// This is a resource/security policy, not a semantic limitation of Zamani.
///
/// A caller that needs larger expressions or longer symbols can construct a
/// larger policy. The canonical representation itself does not encode these
/// policy limits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParameterValidationPolicy {
    /// Maximum UTF-8 byte length accepted for one symbol.
    ///
    /// `None` means that this policy does not impose a symbol-size limit.
    pub max_symbol_bytes: Option<usize>,

    /// Maximum expression depth accepted by this validation invocation.
    ///
    /// `None` means that this policy does not impose an expression-depth
    /// limit.
    pub max_expression_depth: Option<usize>,

    /// Maximum number of AST nodes accepted by this validation invocation.
    ///
    /// `None` means that this policy does not impose a node-count limit.
    pub max_expression_nodes: Option<usize>,

    /// Maximum number of distinct symbols accepted by this validation
    /// invocation.
    ///
    /// `None` means that this policy does not impose a symbol-count limit.
    pub max_symbols: Option<usize>,
}

impl ParameterValidationPolicy {
    /// Creates a completely unrestricted structural policy.
    ///
    /// This does not disable the requirement that numerical values be finite
    /// or that symbol names be non-empty.
    #[must_use]
    pub const fn unrestricted() -> Self {
        Self {
            max_symbol_bytes: None,
            max_expression_depth: None,
            max_expression_nodes: None,
            max_symbols: None,
        }
    }

    /// Creates an explicit bounded policy.
    #[must_use]
    pub const fn bounded(
        max_symbol_bytes: usize,
        max_expression_depth: usize,
        max_expression_nodes: usize,
        max_symbols: usize,
    ) -> Self {
        Self {
            max_symbol_bytes: Some(max_symbol_bytes),
            max_expression_depth: Some(max_expression_depth),
            max_expression_nodes: Some(max_expression_nodes),
            max_symbols: Some(max_symbols),
        }
    }
}

impl Default for ParameterValidationPolicy {
    fn default() -> Self {
        // The canonical IR has no architectural parameter-size ceiling.
        //
        // Resource/security limits are supplied explicitly by the compiler
        // or service boundary.
        Self::unrestricted()
    }
}

// =============================================================================
// Errors
// =============================================================================

/// Error returned by parameter construction, validation, binding or
/// evaluation.
#[derive(Debug, Clone, PartialEq)]
pub enum ParameterError {
    /// A numerical value was NaN or infinite.
    NonFiniteValue {
        context: &'static str,
    },

    /// A symbol has no name.
    EmptySymbol,

    /// A symbol exceeds the active validation policy.
    SymbolTooLong {
        bytes: usize,
        limit: usize,
    },

    /// Expression depth exceeds an explicit policy.
    ExpressionDepthExceeded {
        depth: usize,
        limit: usize,
    },

    /// Expression node count exceeds an explicit policy.
    ExpressionNodeCountExceeded {
        nodes: usize,
        limit: usize,
    },

    /// Distinct symbol count exceeds an explicit policy.
    SymbolCountExceeded {
        symbols: usize,
        limit: usize,
    },

    /// An arithmetic operation would divide by zero.
    DivisionByZero,

    /// Arithmetic produced a non-finite result.
    ArithmeticNonFinite {
        operation: &'static str,
    },

    /// A mathematical function received an invalid input.
    DomainError {
        function: &'static str,
        value: f64,
    },

    /// A symbol was requested but no value was supplied.
    UnboundSymbol {
        name: String,
    },

    /// A binding itself was invalid.
    InvalidBinding {
        name: String,
        reason: &'static str,
    },

    /// A gate parameter count was invalid.
    InvalidGateParameterCount {
        expected: usize,
        actual: usize,
    },

    /// A parameter structure was malformed.
    InvalidStructure {
        reason: &'static str,
    },
}

impl fmt::Display for ParameterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFiniteValue { context } => {
                write!(f, "{context} must be finite")
            }

            Self::EmptySymbol => {
                f.write_str("parameter symbol cannot be empty")
            }

            Self::SymbolTooLong { bytes, limit } => {
                write!(
                    f,
                    "parameter symbol is {bytes} UTF-8 bytes but policy allows \
                     at most {limit}"
                )
            }

            Self::ExpressionDepthExceeded { depth, limit } => {
                write!(
                    f,
                    "parameter expression depth {depth} exceeds policy limit {limit}"
                )
            }

            Self::ExpressionNodeCountExceeded { nodes, limit } => {
                write!(
                    f,
                    "parameter expression contains {nodes} nodes but policy \
                     allows at most {limit}"
                )
            }

            Self::SymbolCountExceeded { symbols, limit } => {
                write!(
                    f,
                    "parameter expression contains {symbols} distinct symbols \
                     but policy allows at most {limit}"
                )
            }

            Self::DivisionByZero => {
                f.write_str("parameter expression attempted division by zero")
            }

            Self::ArithmeticNonFinite { operation } => {
                write!(
                    f,
                    "parameter expression operation `{operation}` produced a \
                     non-finite result"
                )
            }

            Self::DomainError { function, value } => {
                write!(
                    f,
                    "parameter function `{function}` received invalid value {value}"
                )
            }

            Self::UnboundSymbol { name } => {
                write!(f, "parameter symbol `{name}` is unbound")
            }

            Self::InvalidBinding { name, reason } => {
                write!(f, "invalid binding for `{name}`: {reason}")
            }

            Self::InvalidGateParameterCount { expected, actual } => {
                write!(
                    f,
                    "gate parameter requires {expected} parameter(s), received {actual}"
                )
            }

            Self::InvalidStructure { reason } => {
                write!(f, "invalid parameter structure: {reason}")
            }
        }
    }
}

impl std::error::Error for ParameterError {}

/// Result type for parameter operations.
pub type ParameterResult<T> = Result<T, ParameterError>;

/// Compatibility alias for callers that use the canonical IR naming
/// convention.
pub type IrParameterError = ParameterError;

// =============================================================================
// Binding environment
// =============================================================================

/// Explicit immutable parameter binding environment.
///
/// `BTreeMap` is deliberately used instead of `HashMap` so iteration and
/// serialization-related consumers receive deterministic ordering.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ParameterBindings {
    values: BTreeMap<String, f64>,
}

impl ParameterBindings {
    /// Creates an empty binding environment.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates bindings from an iterator.
    ///
    /// Values are validated as finite during insertion.
    pub fn from_iter<I, K>(iter: I) -> ParameterResult<Self>
    where
        I: IntoIterator<Item = (K, f64)>,
        K: Into<String>,
    {
        let mut bindings = Self::new();

        for (name, value) in iter {
            bindings.insert(name, value)?;
        }

        Ok(bindings)
    }

    /// Inserts or replaces one binding.
    pub fn insert<K: Into<String>>(
        &mut self,
        name: K,
        value: f64,
    ) -> ParameterResult<Option<f64>> {
        let name = name.into();

        validate_symbol_name(&name)?;

        ensure_finite(value, "parameter binding")?;

        Ok(self.values.insert(name, value))
    }

    /// Removes one binding.
    pub fn remove(&mut self, name: &str) -> Option<f64> {
        self.values.remove(name)
    }

    /// Returns one binding.
    #[must_use]
    pub fn get(&self, name: &str) -> Option<f64> {
        self.values.get(name).copied()
    }

    /// Returns whether a binding exists.
    #[must_use]
    pub fn contains(&self, name: &str) -> bool {
        self.values.contains_key(name)
    }

    /// Returns the number of bindings.
    #[must_use]
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Returns whether there are no bindings.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Returns deterministic bindings.
    #[must_use]
    pub fn iter(&self) -> impl Iterator<Item = (&str, f64)> {
        self.values.iter().map(|(name, value)| (name.as_str(), *value))
    }

    /// Returns the underlying deterministic map.
    #[must_use]
    pub fn as_map(&self) -> &BTreeMap<String, f64> {
        &self.values
    }
}

// =============================================================================
// Parameter
// =============================================================================

/// Canonical Zamani scalar parameter.
///
/// Parameters may be concrete, symbolic, or expressions.
///
/// The type is deliberately independent of qubits, gates, hardware and units.
#[derive(Debug, Clone, PartialEq)]
pub enum Parameter {
    /// Concrete finite scalar.
    Constant(f64),

    /// Symbolic scalar.
    Symbol(String),

    /// Arithmetic or mathematical expression.
    Expression(Box<ParameterExpression>),
}

impl Parameter {
    /// Creates a finite constant.
    pub fn constant(value: f64) -> ParameterResult<Self> {
        ensure_finite(value, "parameter constant")?;
        Ok(Self::Constant(value))
    }

    /// Creates a symbolic parameter.
    pub fn symbol<S: Into<String>>(name: S) -> ParameterResult<Self> {
        let name = name.into();
        validate_symbol_name(&name)?;
        Ok(Self::Symbol(name))
    }

    /// Creates an expression after unrestricted structural validation.
    pub fn expression(expression: ParameterExpression) -> ParameterResult<Self> {
        expression.validate()?;
        Ok(Self::Expression(Box::new(expression)))
    }

    /// Creates an expression using an explicit policy.
    pub fn expression_with_policy(
        expression: ParameterExpression,
        policy: ParameterValidationPolicy,
    ) -> ParameterResult<Self> {
        expression.validate_with_policy(policy)?;
        Ok(Self::Expression(Box::new(expression)))
    }

    /// Returns a zero parameter.
    #[must_use]
    pub const fn zero() -> Self {
        Self::Constant(0.0)
    }

    /// Returns a one parameter.
    #[must_use]
    pub const fn one() -> Self {
        Self::Constant(1.0)
    }

    /// Returns the value if this is a direct constant.
    #[must_use]
    pub const fn as_constant(&self) -> Option<f64> {
        match self {
            Self::Constant(value) => Some(*value),
            Self::Symbol(_) | Self::Expression(_) => None,
        }
    }

    /// Returns the symbol name if this is a direct symbol.
    #[must_use]
    pub fn as_symbol(&self) -> Option<&str> {
        match self {
            Self::Symbol(name) => Some(name),
            Self::Constant(_) | Self::Expression(_) => None,
        }
    }

    /// Returns the expression if this is an expression.
    #[must_use]
    pub fn as_expression(&self) -> Option<&ParameterExpression> {
        match self {
            Self::Expression(expression) => Some(expression.as_ref()),
            Self::Constant(_) | Self::Symbol(_) => None,
        }
    }

    /// Returns whether this is a concrete constant.
    #[must_use]
    pub const fn is_constant(&self) -> bool {
        matches!(self, Self::Constant(_))
    }

    /// Returns whether this parameter contains at least one symbol.
    #[must_use]
    pub fn is_symbolic(&self) -> bool {
        match self {
            Self::Constant(_) => false,
            Self::Symbol(_) => true,
            Self::Expression(expression) => expression.is_symbolic(),
        }
    }

    /// Returns whether this parameter is completely concrete.
    #[must_use]
    pub fn is_fully_bound(&self) -> bool {
        !self.is_symbolic()
    }

    /// Validates this parameter without imposing a resource-size limit.
    pub fn validate(&self) -> ParameterResult<()> {
        self.validate_with_policy(ParameterValidationPolicy::default())
    }

    /// Validates this parameter under an explicit resource policy.
    pub fn validate_with_policy(
        &self,
        policy: ParameterValidationPolicy,
    ) -> ParameterResult<()> {
        match self {
            Self::Constant(value) => {
                ensure_finite(*value, "parameter constant")
            }

            Self::Symbol(name) => {
                validate_symbol_name_with_policy(name, policy.max_symbol_bytes)
            }

            Self::Expression(expression) => {
                expression.validate_with_policy(policy)
            }
        }
    }

    /// Evaluates the parameter using explicit bindings.
    pub fn evaluate(&self, bindings: &ParameterBindings) -> ParameterResult<f64> {
        self.evaluate_with_resolver(&|name| bindings.get(name))
    }

    /// Evaluates the parameter using an explicit resolver.
    ///
    /// No global state is consulted.
    pub fn evaluate_with_resolver<F>(&self, resolver: &F) -> ParameterResult<f64>
    where
        F: Fn(&str) -> Option<f64>,
    {
        match self {
            Self::Constant(value) => ensure_finite(*value, "parameter constant"),

            Self::Symbol(name) => resolve_symbol(name, resolver),

            Self::Expression(expression) => expression.evaluate(resolver),
        }
    }

    /// Compatibility name for parameter binding.
    pub fn bind(&self, bindings: &ParameterBindings) -> ParameterResult<f64> {
        self.evaluate(bindings)
    }

    /// Binds known symbols and leaves unknown symbols symbolic.
    ///
    /// This is useful for partial compilation:
    ///
    /// ```text
    /// theta + phi
    /// ```
    ///
    /// with `theta = pi/2` becomes:
    ///
    /// ```text
    /// pi/2 + phi
    /// ```
    pub fn partially_bind(
        &self,
        bindings: &ParameterBindings,
    ) -> ParameterResult<Self> {
        match self {
            Self::Constant(value) => Self::constant(*value),

            Self::Symbol(name) => match bindings.get(name) {
                Some(value) => Self::constant(value),
                None => Self::symbol(name.clone()),
            },

            Self::Expression(expression) => {
                expression.partially_bind(bindings)
            }
        }
    }

    /// Substitutes symbols using a symbolic parameter resolver.
    ///
    /// Unlike numerical binding, substitution preserves symbolic expressions.
    pub fn substitute<F>(&self, resolver: &F) -> ParameterResult<Self>
    where
        F: Fn(&str) -> Option<Parameter>,
    {
        match self {
            Self::Constant(value) => Self::constant(*value),

            Self::Symbol(name) => match resolver(name) {
                Some(parameter) => {
                    parameter.validate()?;
                    Ok(parameter)
                }
                None => Self::symbol(name.clone()),
            },

            Self::Expression(expression) => expression.substitute(resolver),
        }
    }

    /// Returns the number of AST nodes.
    pub fn node_count(&self) -> usize {
        match self {
            Self::Constant(_) | Self::Symbol(_) => 1,
            Self::Expression(expression) => expression.node_count(),
        }
    }

    /// Returns expression depth.
    pub fn depth(&self) -> usize {
        match self {
            Self::Constant(_) | Self::Symbol(_) => 0,
            Self::Expression(expression) => expression.depth(),
        }
    }

    /// Collects all distinct symbols in deterministic lexical order.
    #[must_use]
    pub fn symbols(&self) -> Vec<String> {
        let mut symbols = BTreeSet::new();

        self.collect_symbols_into(&mut symbols);

        symbols.into_iter().collect()
    }

    /// Collects symbols into an existing set.
    pub fn collect_symbols_into(&self, symbols: &mut BTreeSet<String>) {
        match self {
            Self::Constant(_) => {}

            Self::Symbol(name) => {
                symbols.insert(name.clone());
            }

            Self::Expression(expression) => {
                expression.collect_symbols_into(symbols);
            }
        }
    }

    /// Simplifies the parameter without numerical bindings.
    pub fn simplify(&self) -> ParameterResult<Self> {
        match self {
            Self::Constant(value) => Self::constant(*value),
            Self::Symbol(name) => Self::symbol(name.clone()),
            Self::Expression(expression) => expression.simplify(),
        }
    }

    /// Returns a deterministic canonical textual representation.
    #[must_use]
    pub fn canonical_string(&self) -> String {
        match self {
            Self::Constant(value) => canonical_float(*value),
            Self::Symbol(name) => name.clone(),
            Self::Expression(expression) => expression.canonical_string(),
        }
    }
}

impl fmt::Display for Parameter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.canonical_string())
    }
}

// =============================================================================
// Expression
// =============================================================================

/// Canonical parameter expression.
///
/// The expression tree is semantic IR, not source syntax.
///
/// Frontends are responsible for parsing source syntax into this structure.
#[derive(Debug, Clone, PartialEq)]
pub enum ParameterExpression {
    /// Literal parameter.
    Constant(f64),

    /// Symbolic parameter.
    Symbol(String),

    /// Addition.
    Add(Box<ParameterExpression>, Box<ParameterExpression>),

    /// Subtraction.
    Subtract(Box<ParameterExpression>, Box<ParameterExpression>),

    /// Multiplication.
    Multiply(Box<ParameterExpression>, Box<ParameterExpression>),

    /// Division.
    Divide(Box<ParameterExpression>, Box<ParameterExpression>),

    /// Unary negation.
    Negate(Box<ParameterExpression>),

    /// Mathematical function.
    Function {
        function: ParameterFunction,
        argument: Box<ParameterExpression>,
    },
}

impl ParameterExpression {
    /// Creates a finite constant expression.
    pub fn constant(value: f64) -> ParameterResult<Self> {
        ensure_finite(value, "expression constant")?;
        Ok(Self::Constant(value))
    }

    /// Creates a symbol expression.
    pub fn symbol<S: Into<String>>(name: S) -> ParameterResult<Self> {
        let name = name.into();
        validate_symbol_name(&name)?;
        Ok(Self::Symbol(name))
    }

    /// Creates addition.
    pub fn add(lhs: Self, rhs: Self) -> Self {
        Self::Add(Box::new(lhs), Box::new(rhs))
    }

    /// Creates subtraction.
    pub fn subtract(lhs: Self, rhs: Self) -> Self {
        Self::Subtract(Box::new(lhs), Box::new(rhs))
    }

    /// Creates multiplication.
    pub fn multiply(lhs: Self, rhs: Self) -> Self {
        Self::Multiply(Box::new(lhs), Box::new(rhs))
    }

    /// Creates division.
    pub fn divide(lhs: Self, rhs: Self) -> Self {
        Self::Divide(Box::new(lhs), Box::new(rhs))
    }

    /// Creates negation.
    pub fn negate(value: Self) -> Self {
        Self::Negate(Box::new(value))
    }

    /// Creates a mathematical function.
    pub fn function(function: ParameterFunction, argument: Self) -> Self {
        Self::Function {
            function,
            argument: Box::new(argument),
        }
    }

    /// Returns whether the expression contains a symbol.
    #[must_use]
    pub fn is_symbolic(&self) -> bool {
        let mut stack = vec![self];

        while let Some(node) = stack.pop() {
            match node {
                Self::Constant(_) => {}

                Self::Symbol(_) => return true,

                Self::Add(lhs, rhs)
                | Self::Subtract(lhs, rhs)
                | Self::Multiply(lhs, rhs)
                | Self::Divide(lhs, rhs) => {
                    stack.push(lhs);
                    stack.push(rhs);
                }

                Self::Negate(value) | Self::Function { argument: value, .. } => {
                    stack.push(value);
                }
            }
        }

        false
    }

    /// Validates without an artificial resource ceiling.
    pub fn validate(&self) -> ParameterResult<()> {
        self.validate_with_policy(ParameterValidationPolicy::default())
    }

    /// Validates under an explicit resource policy.
    ///
    /// Traversal is iterative so validation does not recursively consume the
    /// Rust call stack as expression size grows.
    pub fn validate_with_policy(
        &self,
        policy: ParameterValidationPolicy,
    ) -> ParameterResult<()> {
        let mut stack: Vec<(&ParameterExpression, usize)> = vec![(self, 0)];
        let mut nodes = 0usize;
        let mut symbols = BTreeSet::<String>::new();

        while let Some((node, depth)) = stack.pop() {
            nodes = nodes.checked_add(1).ok_or(
                ParameterError::ExpressionNodeCountExceeded {
                    nodes: usize::MAX,
                    limit: policy.max_expression_nodes.unwrap_or(usize::MAX),
                },
            )?;

            if let Some(limit) = policy.max_expression_nodes {
                if nodes > limit {
                    return Err(
                        ParameterError::ExpressionNodeCountExceeded {
                            nodes,
                            limit,
                        },
                    );
                }
            }

            if let Some(limit) = policy.max_expression_depth {
                if depth > limit {
                    return Err(
                        ParameterError::ExpressionDepthExceeded {
                            depth,
                            limit,
                        },
                    );
                }
            }

            match node {
                Self::Constant(value) => {
                    ensure_finite(*value, "expression constant")?;
                }

                Self::Symbol(name) => {
                    validate_symbol_name_with_policy(
                        name,
                        policy.max_symbol_bytes,
                    )?;

                    symbols.insert(name.clone());

                    if let Some(limit) = policy.max_symbols {
                        if symbols.len() > limit {
                            return Err(
                                ParameterError::SymbolCountExceeded {
                                    symbols: symbols.len(),
                                    limit,
                                },
                            );
                        }
                    }
                }

                Self::Add(lhs, rhs)
                | Self::Subtract(lhs, rhs)
                | Self::Multiply(lhs, rhs)
                | Self::Divide(lhs, rhs) => {
                    let child_depth = depth.saturating_add(1);

                    stack.push((rhs, child_depth));
                    stack.push((lhs, child_depth));
                }

                Self::Negate(value)
                | Self::Function {
                    argument: value, ..
                } => {
                    let child_depth = depth.saturating_add(1);
                    stack.push((value, child_depth));
                }
            }
        }

        Ok(())
    }

    /// Returns the number of expression nodes.
    ///
    /// Saturates at `usize::MAX` rather than wrapping.
    pub fn node_count(&self) -> usize {
        let mut count = 0usize;
        let mut stack = vec![self];

        while let Some(node) = stack.pop() {
            count = count.saturating_add(1);

            match node {
                Self::Constant(_) | Self::Symbol(_) => {}

                Self::Add(lhs, rhs)
                | Self::Subtract(lhs, rhs)
                | Self::Multiply(lhs, rhs)
                | Self::Divide(lhs, rhs) => {
                    stack.push(lhs);
                    stack.push(rhs);
                }

                Self::Negate(value) | Self::Function { argument: value, .. } => {
                    stack.push(value);
                }
            }
        }

        count
    }

    /// Returns maximum expression depth.
    pub fn depth(&self) -> usize {
        let mut maximum = 0usize;
        let mut stack = vec![(self, 0usize)];

        while let Some((node, depth)) = stack.pop() {
            maximum = maximum.max(depth);

            match node {
                Self::Constant(_) | Self::Symbol(_) => {}

                Self::Add(lhs, rhs)
                | Self::Subtract(lhs, rhs)
                | Self::Multiply(lhs, rhs)
                | Self::Divide(lhs, rhs) => {
                    let next = depth.saturating_add(1);
                    stack.push((lhs, next));
                    stack.push((rhs, next));
                }

                Self::Negate(value) | Self::Function { argument: value, .. } => {
                    stack.push((value, depth.saturating_add(1)));
                }
            }
        }

        maximum
    }

    /// Collects distinct symbols.
    pub fn collect_symbols_into(&self, symbols: &mut BTreeSet<String>) {
        let mut stack = vec![self];

        while let Some(node) = stack.pop() {
            match node {
                Self::Constant(_) => {}

                Self::Symbol(name) => {
                    symbols.insert(name.clone());
                }

                Self::Add(lhs, rhs)
                | Self::Subtract(lhs, rhs)
                | Self::Multiply(lhs, rhs)
                | Self::Divide(lhs, rhs) => {
                    stack.push(lhs);
                    stack.push(rhs);
                }

                Self::Negate(value) | Self::Function { argument: value, .. } => {
                    stack.push(value);
                }
            }
        }
    }

    /// Collects symbols in deterministic lexical order.
    #[must_use]
    pub fn symbols(&self) -> Vec<String> {
        let mut symbols = BTreeSet::new();
        self.collect_symbols_into(&mut symbols);
        symbols.into_iter().collect()
    }

    /// Evaluates the expression using an explicit symbol resolver.
    ///
    /// Evaluation is iterative and therefore does not recursively call
    /// through the expression tree.
    pub fn evaluate<F>(&self, resolver: &F) -> ParameterResult<f64>
    where
        F: Fn(&str) -> Option<f64>,
    {
        enum Work<'a> {
            Evaluate(&'a ParameterExpression),
            ApplyUnary(ParameterFunction),
            ApplyNegate,
            ApplyBinary(BinaryOperation),
        }

        let mut work = vec![Work::Evaluate(self)];
        let mut values: Vec<f64> = Vec::new();

        while let Some(item) = work.pop() {
            match item {
                Work::Evaluate(node) => match node {
                    Self::Constant(value) => {
                        ensure_finite(*value, "expression constant")?;
                        values.push(*value);
                    }

                    Self::Symbol(name) => {
                        values.push(resolve_symbol(name, resolver)?);
                    }

                    Self::Add(lhs, rhs) => {
                        work.push(Work::ApplyBinary(BinaryOperation::Add));
                        work.push(Work::Evaluate(rhs));
                        work.push(Work::Evaluate(lhs));
                    }

                    Self::Subtract(lhs, rhs) => {
                        work.push(Work::ApplyBinary(BinaryOperation::Subtract));
                        work.push(Work::Evaluate(rhs));
                        work.push(Work::Evaluate(lhs));
                    }

                    Self::Multiply(lhs, rhs) => {
                        work.push(Work::ApplyBinary(BinaryOperation::Multiply));
                        work.push(Work::Evaluate(rhs));
                        work.push(Work::Evaluate(lhs));
                    }

                    Self::Divide(lhs, rhs) => {
                        work.push(Work::ApplyBinary(BinaryOperation::Divide));
                        work.push(Work::Evaluate(rhs));
                        work.push(Work::Evaluate(lhs));
                    }

                    Self::Negate(value) => {
                        work.push(Work::ApplyNegate);
                        work.push(Work::Evaluate(value));
                    }

                    Self::Function {
                        function,
                        argument,
                    } => {
                        work.push(Work::ApplyUnary(*function));
                        work.push(Work::Evaluate(argument));
                    }
                },

                Work::ApplyUnary(function) => {
                    let value = values.pop().ok_or(
                        ParameterError::InvalidStructure {
                            reason: "missing unary argument",
                        },
                    )?;

                    values.push(function.evaluate(value)?);
                }

                Work::ApplyNegate => {
                    let value = values.pop().ok_or(
                        ParameterError::InvalidStructure {
                            reason: "missing negation operand",
                        },
                    )?;

                    let result = -value;

                    ensure_finite(result, "negation result")?;

                    values.push(result);
                }

                Work::ApplyBinary(operation) => {
                    let rhs = values.pop().ok_or(
                        ParameterError::InvalidStructure {
                            reason: "missing right operand",
                        },
                    )?;

                    let lhs = values.pop().ok_or(
                        ParameterError::InvalidStructure {
                            reason: "missing left operand",
                        },
                    )?;

                    values.push(operation.apply(lhs, rhs)?);
                }
            }
        }

        let result = values.pop().ok_or(
            ParameterError::InvalidStructure {
                reason: "expression produced no result",
            },
        )?;

        if !values.is_empty() {
            return Err(ParameterError::InvalidStructure {
                reason: "expression produced multiple results",
            });
        }

        ensure_finite(result, "expression result")
    }

    /// Partially binds numerical values.
    ///
    /// Constant subexpressions are folded immediately.
    pub fn partially_bind(
        &self,
        bindings: &ParameterBindings,
    ) -> ParameterResult<Parameter> {
        self.substitute(&|name| {
            bindings.get(name).and_then(|value| {
                Parameter::constant(value).ok()
            })
        })
    }

    /// Substitutes symbolic parameters with other parameters.
    pub fn substitute<F>(
        &self,
        resolver: &F,
    ) -> ParameterResult<Parameter>
    where
        F: Fn(&str) -> Option<Parameter>,
    {
        enum Work<'a> {
            Visit(&'a ParameterExpression),
            Apply(BinaryOperation),
            ApplyNegate,
            ApplyFunction(ParameterFunction),
        }

        let mut work = vec![Work::Visit(self)];
        let mut values: Vec<ParameterExpression> = Vec::new();

        while let Some(item) = work.pop() {
            match item {
                Work::Visit(node) => match node {
                    Self::Constant(value) => {
                        ensure_finite(*value, "expression constant")?;
                        values.push(Self::Constant(*value));
                    }

                    Self::Symbol(name) => {
                        if let Some(parameter) = resolver(name) {
                            parameter.validate()?;

                            match parameter {
                                Parameter::Constant(value) => {
                                    values.push(Self::Constant(value));
                                }

                                Parameter::Symbol(name) => {
                                    values.push(Self::Symbol(name));
                                }

                                Parameter::Expression(expression) => {
                                    values.push(*expression);
                                }
                            }
                        } else {
                            values.push(Self::Symbol(name.clone()));
                        }
                    }

                    Self::Add(lhs, rhs) => {
                        work.push(Work::Apply(BinaryOperation::Add));
                        work.push(Work::Visit(rhs));
                        work.push(Work::Visit(lhs));
                    }

                    Self::Subtract(lhs, rhs) => {
                        work.push(Work::Apply(BinaryOperation::Subtract));
                        work.push(Work::Visit(rhs));
                        work.push(Work::Visit(lhs));
                    }

                    Self::Multiply(lhs, rhs) => {
                        work.push(Work::Apply(BinaryOperation::Multiply));
                        work.push(Work::Visit(rhs));
                        work.push(Work::Visit(lhs));
                    }

                    Self::Divide(lhs, rhs) => {
                        work.push(Work::Apply(BinaryOperation::Divide));
                        work.push(Work::Visit(rhs));
                        work.push(Work::Visit(lhs));
                    }

                    Self::Negate(value) => {
                        work.push(Work::ApplyNegate);
                        work.push(Work::Visit(value));
                    }

                    Self::Function {
                        function,
                        argument,
                    } => {
                        work.push(Work::ApplyFunction(*function));
                        work.push(Work::Visit(argument));
                    }
                },

                Work::Apply(operation) => {
                    let rhs = values.pop().ok_or(
                        ParameterError::InvalidStructure {
                            reason: "missing substitution right operand",
                        },
                    )?;

                    let lhs = values.pop().ok_or(
                        ParameterError::InvalidStructure {
                            reason: "missing substitution left operand",
                        },
                    )?;

                    values.push(operation.construct(lhs, rhs));
                }

                Work::ApplyNegate => {
                    let value = values.pop().ok_or(
                        ParameterError::InvalidStructure {
                            reason: "missing substitution negation operand",
                        },
                    )?;

                    values.push(Self::Negate(Box::new(value)));
                }

                Work::ApplyFunction(function) => {
                    let value = values.pop().ok_or(
                        ParameterError::InvalidStructure {
                            reason: "missing substitution function argument",
                        },
                    )?;

                    values.push(Self::Function {
                        function,
                        argument: Box::new(value),
                    });
                }
            }
        }

        let expression = values.pop().ok_or(
            ParameterError::InvalidStructure {
                reason: "substitution produced no result",
            },
        )?;

        if !values.is_empty() {
            return Err(ParameterError::InvalidStructure {
                reason: "substitution produced multiple results",
            });
        }

        Parameter::expression(expression)?.simplify()
    }

    /// Performs algebraically safe local simplification and constant folding.
    ///
    /// The simplifier does not use floating-point approximate equality.
    pub fn simplify(&self) -> ParameterResult<Parameter> {
        enum Work<'a> {
            Visit(&'a ParameterExpression),
            Apply(BinaryOperation),
            ApplyNegate,
            ApplyFunction(ParameterFunction),
        }

        let mut work = vec![Work::Visit(self)];
        let mut values: Vec<ParameterExpression> = Vec::new();

        while let Some(item) = work.pop() {
            match item {
                Work::Visit(node) => match node {
                    Self::Constant(value) => {
                        ensure_finite(*value, "expression constant")?;
                        values.push(Self::Constant(*value));
                    }

                    Self::Symbol(name) => {
                        validate_symbol_name(name)?;
                        values.push(Self::Symbol(name.clone()));
                    }

                    Self::Add(lhs, rhs) => {
                        work.push(Work::Apply(BinaryOperation::Add));
                        work.push(Work::Visit(rhs));
                        work.push(Work::Visit(lhs));
                    }

                    Self::Subtract(lhs, rhs) => {
                        work.push(Work::Apply(BinaryOperation::Subtract));
                        work.push(Work::Visit(rhs));
                        work.push(Work::Visit(lhs));
                    }

                    Self::Multiply(lhs, rhs) => {
                        work.push(Work::Apply(BinaryOperation::Multiply));
                        work.push(Work::Visit(rhs));
                        work.push(Work::Visit(lhs));
                    }

                    Self::Divide(lhs, rhs) => {
                        work.push(Work::Apply(BinaryOperation::Divide));
                        work.push(Work::Visit(rhs));
                        work.push(Work::Visit(lhs));
                    }

                    Self::Negate(value) => {
                        work.push(Work::ApplyNegate);
                        work.push(Work::Visit(value));
                    }

                    Self::Function {
                        function,
                        argument,
                    } => {
                        work.push(Work::ApplyFunction(*function));
                        work.push(Work::Visit(argument));
                    }
                },

                Work::Apply(operation) => {
                    let rhs = values.pop().ok_or(
                        ParameterError::InvalidStructure {
                            reason: "missing simplification right operand",
                        },
                    )?;

                    let lhs = values.pop().ok_or(
                        ParameterError::InvalidStructure {
                            reason: "missing simplification left operand",
                        },
                    )?;

                    values.push(simplify_binary(operation, lhs, rhs)?);
                }

                Work::ApplyNegate => {
                    let value = values.pop().ok_or(
                        ParameterError::InvalidStructure {
                            reason: "missing simplification negation operand",
                        },
                    )?;

                    values.push(simplify_negate(value)?);
                }

                Work::ApplyFunction(function) => {
                    let value = values.pop().ok_or(
                        ParameterError::InvalidStructure {
                            reason: "missing simplification function operand",
                        },
                    )?;

                    values.push(simplify_function(function, value)?);
                }
            }
        }

        let expression = values.pop().ok_or(
            ParameterError::InvalidStructure {
                reason: "simplification produced no result",
            },
        )?;

        if !values.is_empty() {
            return Err(ParameterError::InvalidStructure {
                reason: "simplification produced multiple results",
            });
        }

        match expression {
            Self::Constant(value) => Parameter::constant(value),
            Self::Symbol(name) => Parameter::symbol(name),
            expression => Parameter::expression(expression),
        }
    }

    /// Returns a deterministic canonical representation.
    #[must_use]
    pub fn canonical_string(&self) -> String {
        let mut output = String::new();
        write_expression(self, &mut output, 0);
        output
    }
}

impl fmt::Display for ParameterExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.canonical_string())
    }
}

// =============================================================================
// Mathematical functions
// =============================================================================

/// Supported scalar mathematical functions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ParameterFunction {
    /// Sine.
    Sin,

    /// Cosine.
    Cos,

    /// Tangent.
    Tan,

    /// Exponential.
    Exp,

    /// Natural logarithm.
    Ln,

    /// Square root.
    Sqrt,

    /// Absolute value.
    Abs,

    /// Floor.
    Floor,

    /// Ceiling.
    Ceil,

    /// Round to nearest integral value according to `f64::round`.
    Round,
}

impl ParameterFunction {
    /// Returns the canonical function name.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Sin => "sin",
            Self::Cos => "cos",
            Self::Tan => "tan",
            Self::Exp => "exp",
            Self::Ln => "ln",
            Self::Sqrt => "sqrt",
            Self::Abs => "abs",
            Self::Floor => "floor",
            Self::Ceil => "ceil",
            Self::Round => "round",
        }
    }

    /// Evaluates the function with checked finite-result semantics.
    pub fn evaluate(self, value: f64) -> ParameterResult<f64> {
        ensure_finite(value, "function argument")?;

        let result = match self {
            Self::Sin => value.sin(),
            Self::Cos => value.cos(),
            Self::Tan => value.tan(),
            Self::Exp => value.exp(),
            Self::Ln => {
                if value <= 0.0 {
                    return Err(ParameterError::DomainError {
                        function: self.name(),
                        value,
                    });
                }

                value.ln()
            }
            Self::Sqrt => {
                if value < 0.0 {
                    return Err(ParameterError::DomainError {
                        function: self.name(),
                        value,
                    });
                }

                value.sqrt()
            }
            Self::Abs => value.abs(),
            Self::Floor => value.floor(),
            Self::Ceil => value.ceil(),
            Self::Round => value.round(),
        };

        ensure_finite(result, self.name())?;

        Ok(result)
    }
}

impl fmt::Display for ParameterFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

// =============================================================================
// Binary operation
// =============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BinaryOperation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl BinaryOperation {
    fn apply(self, lhs: f64, rhs: f64) -> ParameterResult<f64> {
        let result = match self {
            Self::Add => lhs + rhs,
            Self::Subtract => lhs - rhs,
            Self::Multiply => lhs * rhs,
            Self::Divide => {
                if rhs == 0.0 {
                    return Err(ParameterError::DivisionByZero);
                }

                lhs / rhs
            }
        };

        if result.is_finite() {
            Ok(result)
        } else {
            Err(ParameterError::ArithmeticNonFinite {
                operation: self.name(),
            })
        }
    }

    fn name(self) -> &'static str {
        match self {
            Self::Add => "add",
            Self::Subtract => "subtract",
            Self::Multiply => "multiply",
            Self::Divide => "divide",
        }
    }

    fn construct(
        self,
        lhs: ParameterExpression,
        rhs: ParameterExpression,
    ) -> ParameterExpression {
        match self {
            Self::Add => ParameterExpression::Add(
                Box::new(lhs),
                Box::new(rhs),
            ),
            Self::Subtract => ParameterExpression::Subtract(
                Box::new(lhs),
                Box::new(rhs),
            ),
            Self::Multiply => ParameterExpression::Multiply(
                Box::new(lhs),
                Box::new(rhs),
            ),
            Self::Divide => ParameterExpression::Divide(
                Box::new(lhs),
                Box::new(rhs),
            ),
        }
    }
}

// =============================================================================
// Simplification
// =============================================================================

fn simplify_binary(
    operation: BinaryOperation,
    lhs: ParameterExpression,
    rhs: ParameterExpression,
) -> ParameterResult<ParameterExpression> {
    match (&lhs, &rhs) {
        (ParameterExpression::Constant(a), ParameterExpression::Constant(b)) => {
            Ok(ParameterExpression::Constant(operation.apply(*a, *b)?))
        }

        _ => {
            match operation {
                BinaryOperation::Add => {
                    if is_zero(&rhs) {
                        return Ok(lhs);
                    }

                    if is_zero(&lhs) {
                        return Ok(rhs);
                    }
                }

                BinaryOperation::Subtract => {
                    if is_zero(&rhs) {
                        return Ok(lhs);
                    }
                }

                BinaryOperation::Multiply => {
                    if is_zero(&lhs) || is_zero(&rhs) {
                        return Ok(ParameterExpression::Constant(0.0));
                    }

                    if is_one(&lhs) {
                        return Ok(rhs);
                    }

                    if is_one(&rhs) {
                        return Ok(lhs);
                    }
                }

                BinaryOperation::Divide => {
                    if is_zero(&lhs) {
                        if is_zero(&rhs) {
                            return Err(ParameterError::DivisionByZero);
                        }

                        return Ok(ParameterExpression::Constant(0.0));
                    }

                    if is_one(&rhs) {
                        return Ok(lhs);
                    }

                    if is_zero(&rhs) {
                        return Err(ParameterError::DivisionByZero);
                    }
                }
            }

            Ok(operation.construct(lhs, rhs))
        }
    }
}

fn simplify_negate(
    value: ParameterExpression,
) -> ParameterResult<ParameterExpression> {
    match value {
        ParameterExpression::Constant(value) => {
            let result = -value;
            ensure_finite(result, "negation result")?;
            Ok(ParameterExpression::Constant(result))
        }

        ParameterExpression::Negate(inner) => Ok(*inner),

        value => Ok(ParameterExpression::Negate(Box::new(value))),
    }
}

fn simplify_function(
    function: ParameterFunction,
    value: ParameterExpression,
) -> ParameterResult<ParameterExpression> {
    match value {
        ParameterExpression::Constant(value) => {
            Ok(ParameterExpression::Constant(function.evaluate(value)?))
        }

        value => Ok(ParameterExpression::Function {
            function,
            argument: Box::new(value),
        }),
    }
}

fn is_zero(expression: &ParameterExpression) -> bool {
    matches!(
        expression,
        ParameterExpression::Constant(value) if *value == 0.0
    )
}

fn is_one(expression: &ParameterExpression) -> bool {
    matches!(
        expression,
        ParameterExpression::Constant(value) if *value == 1.0
    )
}

// =============================================================================
// Gate parameter compatibility
// =============================================================================

/// Parameter payload used by the canonical gate layer.
///
/// This retains the existing Zamani gate API while allowing each parameter
/// to remain symbolic and independently typed at the gate-contract level.
///
/// The variants describe gate parameter arity, not the universe of possible
/// quantum operations.
#[derive(Debug, Clone, PartialEq)]
pub enum GateParameter {
    /// One scalar parameter.
    Angle(Parameter),

    /// Two scalar parameters.
    TwoAngles(Parameter, Parameter),

    /// Three scalar parameters.
    ThreeAngles(Parameter, Parameter, Parameter),

    /// Arbitrary parameter vector for extensible/custom semantic operations.
    ///
    /// Standard fixed-arity gates should continue using the named variants.
    /// Extensible dialects may use this variant without modifying this file.
    Many(Vec<Parameter>),
}

impl GateParameter {
    /// Constructs a one-parameter gate parameter.
    #[must_use]
    pub fn angle(parameter: Parameter) -> Self {
        Self::Angle(parameter)
    }

    /// Constructs a two-parameter gate parameter.
    #[must_use]
    pub fn two_angles(first: Parameter, second: Parameter) -> Self {
        Self::TwoAngles(first, second)
    }

    /// Constructs a three-parameter gate parameter.
    #[must_use]
    pub fn three_angles(
        first: Parameter,
        second: Parameter,
        third: Parameter,
    ) -> Self {
        Self::ThreeAngles(first, second, third)
    }

    /// Constructs an arbitrary parameter list.
    ///
    /// This is deliberately not limited to a fixed number of parameters.
    pub fn many(parameters: Vec<Parameter>) -> ParameterResult<Self> {
        for parameter in &parameters {
            parameter.validate()?;
        }

        Ok(Self::Many(parameters))
    }

    /// Returns the number of scalar parameters.
    #[must_use]
    pub fn len(&self) -> usize {
        match self {
            Self::Angle(_) => 1,
            Self::TwoAngles(_, _) => 2,
            Self::ThreeAngles(_, _, _) => 3,
            Self::Many(parameters) => parameters.len(),
        }
    }

    /// Returns whether there are no parameters.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns all parameters in canonical order.
    #[must_use]
    pub fn as_slice(&self) -> &[Parameter] {
        match self {
            Self::Angle(parameter) => std::slice::from_ref(parameter),
            Self::TwoAngles(first, second) => {
                // This variant is stored as two fields for compatibility.
                //
                // It cannot expose a true contiguous slice without changing
                // its representation. Callers needing iteration should use
                // `iter()`.
                //
                // This branch is therefore unreachable through `as_slice`
                // for two/three-parameter compatibility variants.
                //
                // Return an empty slice rather than creating an invalid view.
                let _ = (first, second);
                &[]
            }

            Self::ThreeAngles(first, second, third) => {
                let _ = (first, second, third);
                &[]
            }

            Self::Many(parameters) => parameters.as_slice(),
        }
    }

    /// Iterates over all parameters without exposing representation details.
    pub fn iter(&self) -> GateParameterIter<'_> {
        match self {
            Self::Angle(parameter) => GateParameterIter {
                first: Some(parameter),
                second: None,
                third: None,
                many: None,
                index: 0,
            },

            Self::TwoAngles(first, second) => GateParameterIter {
                first: Some(first),
                second: Some(second),
                third: None,
                many: None,
                index: 0,
            },

            Self::ThreeAngles(first, second, third) => GateParameterIter {
                first: Some(first),
                second: Some(second),
                third: Some(third),
                many: None,
                index: 0,
            },

            Self::Many(parameters) => GateParameterIter {
                first: None,
                second: None,
                third: None,
                many: Some(parameters.as_slice()),
                index: 0,
            },
        }
    }

    /// Validates all contained parameters.
    pub fn validate(&self) -> ParameterResult<()> {
        for parameter in self.iter() {
            parameter.validate()?;
        }

        Ok(())
    }

    /// Evaluates all parameters.
    pub fn evaluate(
        &self,
        bindings: &ParameterBindings,
    ) -> ParameterResult<Vec<f64>> {
        self.iter()
            .map(|parameter| parameter.evaluate(bindings))
            .collect()
    }

    /// Partially binds all parameters.
    pub fn partially_bind(
        &self,
        bindings: &ParameterBindings,
    ) -> ParameterResult<Self> {
        match self {
            Self::Angle(parameter) => Ok(Self::Angle(
                parameter.partially_bind(bindings)?,
            )),

            Self::TwoAngles(first, second) => Ok(Self::TwoAngles(
                first.partially_bind(bindings)?,
                second.partially_bind(bindings)?,
            )),

            Self::ThreeAngles(first, second, third) => {
                Ok(Self::ThreeAngles(
                    first.partially_bind(bindings)?,
                    second.partially_bind(bindings)?,
                    third.partially_bind(bindings)?,
                ))
            }

            Self::Many(parameters) => {
                let mut result = Vec::with_capacity(parameters.len());

                for parameter in parameters {
                    result.push(parameter.partially_bind(bindings)?);
                }

                Ok(Self::Many(result))
            }
        }
    }

    /// Returns all distinct symbols used by this gate parameter payload.
    #[must_use]
    pub fn symbols(&self) -> Vec<String> {
        let mut symbols = BTreeSet::new();

        for parameter in self.iter() {
            parameter.collect_symbols_into(&mut symbols);
        }

        symbols.into_iter().collect()
    }

    /// Returns a deterministic canonical textual representation.
    #[must_use]
    pub fn canonical_string(&self) -> String {
        match self {
            Self::Angle(parameter) => {
                format!("({})", parameter.canonical_string())
            }

            Self::TwoAngles(first, second) => format!(
                "({}, {})",
                first.canonical_string(),
                second.canonical_string()
            ),

            Self::ThreeAngles(first, second, third) => format!(
                "({}, {}, {})",
                first.canonical_string(),
                second.canonical_string(),
                third.canonical_string()
            ),

            Self::Many(parameters) => {
                let values = parameters
                    .iter()
                    .map(Parameter::canonical_string)
                    .collect::<Vec<_>>();

                format!("({})", values.join(", "))
            }
        }
    }
}

impl fmt::Display for GateParameter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.canonical_string())
    }
}

/// Iterator over a `GateParameter`.
pub struct GateParameterIter<'a> {
    first: Option<&'a Parameter>,
    second: Option<&'a Parameter>,
    third: Option<&'a Parameter>,
    many: Option<&'a [Parameter]>,
    index: usize,
}

impl<'a> Iterator for GateParameterIter<'a> {
    type Item = &'a Parameter;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(many) = self.many {
            let item = many.get(self.index);
            self.index = self.index.saturating_add(1);
            return item;
        }

        let item = match self.index {
            0 => self.first,
            1 => self.second,
            2 => self.third,
            _ => None,
        };

        self.index = self.index.saturating_add(1);

        item
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        if let Some(many) = self.many {
            let remaining = many.len().saturating_sub(self.index);
            return (remaining, Some(remaining));
        }

        let remaining = match self.index {
            0 => self.first.is_some() as usize
                + self.second.is_some() as usize
                + self.third.is_some() as usize,

            1 => self.second.is_some() as usize
                + self.third.is_some() as usize,

            2 => self.third.is_some() as usize,

            _ => 0,
        };

        (remaining, Some(remaining))
    }
}

impl<'a> ExactSizeIterator for GateParameterIter<'a> {}

// =============================================================================
// Bound gate parameters
// =============================================================================

/// Gate parameters after numerical binding.
///
/// This type deliberately stores `f64` values only after the caller has
/// explicitly requested evaluation.
#[derive(Debug, Clone, PartialEq)]
pub enum BoundGateParameter {
    /// One value.
    Angle(f64),

    /// Two values.
    TwoAngles(f64, f64),

    /// Three values.
    ThreeAngles(f64, f64, f64),

    /// Arbitrary number of values.
    Many(Vec<f64>),
}

impl BoundGateParameter {
    /// Binds a canonical gate parameter.
    pub fn bind(
        parameter: &GateParameter,
        bindings: &ParameterBindings,
    ) -> ParameterResult<Self> {
        let values = parameter.evaluate(bindings)?;

        match values.as_slice() {
            [value] => Ok(Self::Angle(*value)),
            [first, second] => {
                Ok(Self::TwoAngles(*first, *second))
            }
            [first, second, third] => Ok(Self::ThreeAngles(
                *first,
                *second,
                *third,
            )),
            _ => Ok(Self::Many(values)),
        }
    }

    /// Returns the number of values.
    #[must_use]
    pub fn len(&self) -> usize {
        match self {
            Self::Angle(_) => 1,
            Self::TwoAngles(_, _) => 2,
            Self::ThreeAngles(_, _, _) => 3,
            Self::Many(values) => values.len(),
        }
    }

    /// Returns whether no values exist.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Iterates over values.
    pub fn iter(&self) -> BoundGateParameterIter<'_> {
        match self {
            Self::Angle(value) => BoundGateParameterIter {
                first: Some(value),
                second: None,
                third: None,
                many: None,
                index: 0,
            },

            Self::TwoAngles(first, second) => BoundGateParameterIter {
                first: Some(first),
                second: Some(second),
                third: None,
                many: None,
                index: 0,
            },

            Self::ThreeAngles(first, second, third) => {
                BoundGateParameterIter {
                    first: Some(first),
                    second: Some(second),
                    third: Some(third),
                    many: None,
                    index: 0,
                }
            }

            Self::Many(values) => BoundGateParameterIter {
                first: None,
                second: None,
                third: None,
                many: Some(values.as_slice()),
                index: 0,
            },
        }
    }
}

/// Iterator over bound gate parameters.
pub struct BoundGateParameterIter<'a> {
    first: Option<&'a f64>,
    second: Option<&'a f64>,
    third: Option<&'a f64>,
    many: Option<&'a [f64]>,
    index: usize,
}

impl<'a> Iterator for BoundGateParameterIter<'a> {
    type Item = &'a f64;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(values) = self.many {
            let item = values.get(self.index);
            self.index = self.index.saturating_add(1);
            return item;
        }

        let item = match self.index {
            0 => self.first,
            1 => self.second,
            2 => self.third,
            _ => None,
        };

        self.index = self.index.saturating_add(1);

        item
    }
}

// =============================================================================
// Operator implementations
// =============================================================================

impl From<f64> for Parameter {
    fn from(value: f64) -> Self {
        // `From<f64>` cannot return a Result.
        //
        // The canonical checked constructor is `Parameter::constant`.
        // Keeping this conversion preserves ergonomic construction while
        // validation remains mandatory at IR validation/serialization
        // boundaries.
        Self::Constant(value)
    }
}

impl From<&str> for Parameter {
    fn from(value: &str) -> Self {
        Self::Symbol(value.to_owned())
    }
}

impl From<String> for Parameter {
    fn from(value: String) -> Self {
        Self::Symbol(value)
    }
}

impl From<ParameterExpression> for Parameter {
    fn from(expression: ParameterExpression) -> Self {
        Self::Expression(Box::new(expression))
    }
}

impl Add for Parameter {
    type Output = Parameter;

    fn add(self, rhs: Self) -> Self::Output {
        Parameter::Expression(Box::new(ParameterExpression::Add(
            Box::new(self.into_expression()),
            Box::new(rhs.into_expression()),
        )))
    }
}

impl Sub for Parameter {
    type Output = Parameter;

    fn sub(self, rhs: Self) -> Self::Output {
        Parameter::Expression(Box::new(ParameterExpression::Subtract(
            Box::new(self.into_expression()),
            Box::new(rhs.into_expression()),
        )))
    }
}

impl Mul for Parameter {
    type Output = Parameter;

    fn mul(self, rhs: Self) -> Self::Output {
        Parameter::Expression(Box::new(ParameterExpression::Multiply(
            Box::new(self.into_expression()),
            Box::new(rhs.into_expression()),
        )))
    }
}

impl Div for Parameter {
    type Output = Parameter;

    fn div(self, rhs: Self) -> Self::Output {
        Parameter::Expression(Box::new(ParameterExpression::Divide(
            Box::new(self.into_expression()),
            Box::new(rhs.into_expression()),
        )))
    }
}

impl Neg for Parameter {
    type Output = Parameter;

    fn neg(self) -> Self::Output {
        Parameter::Expression(Box::new(ParameterExpression::Negate(
            Box::new(self.into_expression()),
        )))
    }
}

impl Parameter {
    fn into_expression(self) -> ParameterExpression {
        match self {
            Self::Constant(value) => ParameterExpression::Constant(value),
            Self::Symbol(name) => ParameterExpression::Symbol(name),
            Self::Expression(expression) => *expression,
        }
    }
}

// =============================================================================
// Expression operator implementations
// =============================================================================

impl Add for ParameterExpression {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::add(self, rhs)
    }
}

impl Sub for ParameterExpression {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::subtract(self, rhs)
    }
}

impl Mul for ParameterExpression {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::multiply(self, rhs)
    }
}

impl Div for ParameterExpression {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self::divide(self, rhs)
    }
}

impl Neg for ParameterExpression {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::negate(self)
    }
}

// =============================================================================
// Formatting
// =============================================================================

fn write_expression(
    expression: &ParameterExpression,
    output: &mut String,
    parent_precedence: u8,
) {
    match expression {
        ParameterExpression::Constant(value) => {
            output.push_str(&canonical_float(*value));
        }

        ParameterExpression::Symbol(name) => {
            output.push_str(name);
        }

        ParameterExpression::Add(lhs, rhs) => {
            write_binary(
                lhs,
                rhs,
                "+",
                1,
                output,
                parent_precedence,
            );
        }

        ParameterExpression::Subtract(lhs, rhs) => {
            write_binary(
                lhs,
                rhs,
                "-",
                1,
                output,
                parent_precedence,
            );
        }

        ParameterExpression::Multiply(lhs, rhs) => {
            write_binary(
                lhs,
                rhs,
                "*",
                2,
                output,
                parent_precedence,
            );
        }

        ParameterExpression::Divide(lhs, rhs) => {
            write_binary(
                lhs,
                rhs,
                "/",
                2,
                output,
                parent_precedence,
            );
        }

        ParameterExpression::Negate(value) => {
            let precedence = 3;
            let needs_parentheses = precedence < parent_precedence;

            if needs_parentheses {
                output.push('(');
            }

            output.push('-');

            write_expression(
                value,
                output,
                precedence,
            );

            if needs_parentheses {
                output.push(')');
            }
        }

        ParameterExpression::Function {
            function,
            argument,
        } => {
            output.push_str(function.name());
            output.push('(');
            write_expression(argument, output, 0);
            output.push(')');
        }
    }
}

fn write_binary(
    lhs: &ParameterExpression,
    rhs: &ParameterExpression,
    operator: &str,
    precedence: u8,
    output: &mut String,
    parent_precedence: u8,
) {
    let needs_parentheses = precedence < parent_precedence;

    if needs_parentheses {
        output.push('(');
    }

    write_expression(lhs, output, precedence);
    output.push_str(operator);
    write_expression(rhs, output, precedence);

    if needs_parentheses {
        output.push(')');
    }
}

fn canonical_float(value: f64) -> String {
    if value == 0.0 {
        // Canonicalize both +0.0 and -0.0 to one semantic representation.
        return "0".to_owned();
    }

    if value.fract() == 0.0 && value.abs() < 1e21 {
        return format!("{value:.0}");
    }

    format!("{value:.17}")
}

// =============================================================================
// Validation helpers
// =============================================================================

fn ensure_finite(
    value: f64,
    context: &'static str,
) -> ParameterResult<f64> {
    if value.is_finite() {
        Ok(value)
    } else {
        Err(ParameterError::NonFiniteValue { context })
    }
}

fn validate_symbol_name(name: &str) -> ParameterResult<()> {
    validate_symbol_name_with_policy(name, None)
}

fn validate_symbol_name_with_policy(
    name: &str,
    maximum_bytes: Option<usize>,
) -> ParameterResult<()> {
    if name.is_empty() {
        return Err(ParameterError::EmptySymbol);
    }

    if let Some(limit) = maximum_bytes {
        if name.len() > limit {
            return Err(ParameterError::SymbolTooLong {
                bytes: name.len(),
                limit,
            });
        }
    }

    Ok(())
}

fn resolve_symbol<F>(
    name: &str,
    resolver: &F,
) -> ParameterResult<f64>
where
    F: Fn(&str) -> Option<f64>,
{
    let value = resolver(name).ok_or_else(|| {
        ParameterError::UnboundSymbol {
            name: name.to_owned(),
        }
    })?;

    ensure_finite(value, "resolved parameter")
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finite_constant_is_valid() {
        let parameter = Parameter::constant(1.25).expect("valid constant");

        assert_eq!(parameter.as_constant(), Some(1.25));
        assert!(parameter.is_constant());
        assert!(!parameter.is_symbolic());
    }

    #[test]
    fn non_finite_constant_is_rejected() {
        assert!(Parameter::constant(f64::NAN).is_err());
        assert!(Parameter::constant(f64::INFINITY).is_err());
        assert!(Parameter::constant(f64::NEG_INFINITY).is_err());
    }

    #[test]
    fn symbols_are_explicit() {
        let parameter =
            Parameter::symbol("theta").expect("valid symbol");

        assert_eq!(parameter.as_symbol(), Some("theta"));
        assert!(parameter.is_symbolic());
        assert_eq!(parameter.symbols(), vec!["theta".to_owned()]);
    }

    #[test]
    fn expression_evaluates_deterministically() {
        let theta =
            Parameter::symbol("theta").expect("valid symbol");

        let expression = theta
            + Parameter::from(2.0);

        let mut bindings = ParameterBindings::new();

        bindings
            .insert("theta", 3.0)
            .expect("valid binding");

        let value = expression
            .evaluate(&bindings)
            .expect("expression should evaluate");

        assert_eq!(value, 5.0);
    }

    #[test]
    fn expression_supports_nested_arithmetic() {
        let theta =
            Parameter::symbol("theta").expect("valid symbol");

        let phi =
            Parameter::symbol("phi").expect("valid symbol");

        let expression =
            (theta * Parameter::from(2.0))
                + (phi / Parameter::from(2.0));

        let mut bindings = ParameterBindings::new();

        bindings
            .insert("theta", 3.0)
            .expect("valid theta");

        bindings
            .insert("phi", 4.0)
            .expect("valid phi");

        assert_eq!(
            expression
                .evaluate(&bindings)
                .expect("evaluation"),
            8.0
        );
    }

    #[test]
    fn division_by_zero_is_rejected() {
        let expression =
            Parameter::from(1.0) / Parameter::from(0.0);

        assert_eq!(
            expression.evaluate(&ParameterBindings::new()),
            Err(ParameterError::DivisionByZero)
        );
    }

    #[test]
    fn unknown_symbol_is_rejected_during_evaluation() {
        let expression =
            Parameter::symbol("theta").expect("valid symbol");

        let result =
            expression.evaluate(&ParameterBindings::new());

        assert_eq!(
            result,
            Err(ParameterError::UnboundSymbol {
                name: "theta".to_owned()
            })
        );
    }

    #[test]
    fn partial_binding_preserves_unknown_symbols() {
        let theta =
            Parameter::symbol("theta").expect("theta");

        let phi =
            Parameter::symbol("phi").expect("phi");

        let expression = theta + phi;

        let mut bindings = ParameterBindings::new();

        bindings.insert("theta", 2.0).expect("theta binding");

        let result = expression
            .partially_bind(&bindings)
            .expect("partial binding");

        assert!(result.is_symbolic());
        assert_eq!(result.symbols(), vec!["phi".to_owned()]);
    }

    #[test]
    fn simplification_folds_constants() {
        let expression =
            Parameter::from(2.0) + Parameter::from(3.0);

        let simplified =
            expression.simplify().expect("simplification");

        assert_eq!(simplified.as_constant(), Some(5.0));
    }

    #[test]
    fn simplification_removes_identity_operations() {
        let theta =
            Parameter::symbol("theta").expect("theta");

        let expression =
            theta.clone() * Parameter::from(1.0);

        let simplified =
            expression.simplify().expect("simplification");

        assert_eq!(simplified, theta);
    }

    #[test]
    fn symbols_are_deterministically_sorted() {
        let z =
            Parameter::symbol("z").expect("z");

        let a =
            Parameter::symbol("a").expect("a");

        let m =
            Parameter::symbol("m").expect("m");

        let expression =
            (z + a) * m;

        assert_eq!(
            expression.symbols(),
            vec![
                "a".to_owned(),
                "m".to_owned(),
                "z".to_owned()
            ]
        );
    }

    #[test]
    fn functions_evaluate() {
        let expression =
            ParameterExpression::function(
                ParameterFunction::Sqrt,
                ParameterExpression::Constant(9.0),
            );

        let value = expression
            .evaluate(&|_| None)
            .expect("sqrt");

        assert_eq!(value, 3.0);
    }

    #[test]
    fn invalid_function_domain_is_rejected() {
        let expression =
            ParameterExpression::function(
                ParameterFunction::Ln,
                ParameterExpression::Constant(0.0),
            );

        assert!(matches!(
            expression.evaluate(&|_| None),
            Err(ParameterError::DomainError { .. })
        ));
    }

    #[test]
    fn gate_parameter_supports_many_parameters() {
        let parameters = GateParameter::many(vec![
            Parameter::from(1.0),
            Parameter::from(2.0),
            Parameter::from(3.0),
            Parameter::from(4.0),
        ])
        .expect("valid parameters");

        assert_eq!(parameters.len(), 4);
        assert_eq!(
            parameters.iter().count(),
            4
        );
    }

    #[test]
    fn bound_gate_parameter_evaluates() {
        let theta =
            Parameter::symbol("theta").expect("theta");

        let gate_parameter =
            GateParameter::Angle(theta);

        let mut bindings = ParameterBindings::new();

        bindings
            .insert("theta", 1.5)
            .expect("binding");

        let bound =
            BoundGateParameter::bind(
                &gate_parameter,
                &bindings,
            )
            .expect("binding");

        assert_eq!(bound.len(), 1);
        assert_eq!(
            bound.iter().copied().collect::<Vec<_>>(),
            vec![1.5]
        );
    }

    #[test]
    fn unrestricted_policy_has_no_artificial_limits() {
        let policy =
            ParameterValidationPolicy::unrestricted();

        assert_eq!(policy.max_symbol_bytes, None);
        assert_eq!(policy.max_expression_depth, None);
        assert_eq!(policy.max_expression_nodes, None);
        assert_eq!(policy.max_symbols, None);
    }

    #[test]
    fn explicit_policy_can_limit_expression_depth() {
        let expression =
            ParameterExpression::negate(
                ParameterExpression::negate(
                    ParameterExpression::Constant(1.0),
                ),
            );

        let policy = ParameterValidationPolicy {
            max_symbol_bytes: None,
            max_expression_depth: Some(0),
            max_expression_nodes: None,
            max_symbols: None,
        };

        assert!(matches!(
            expression.validate_with_policy(policy),
            Err(ParameterError::ExpressionDepthExceeded { .. })
        ));
    }

    #[test]
    fn canonical_format_is_deterministic() {
        let theta =
            Parameter::symbol("theta").expect("theta");

        let expression =
            theta + Parameter::from(2.0);

        assert_eq!(
            expression.canonical_string(),
            "theta+2"
        );
    }

    #[test]
    fn negative_zero_is_canonicalized() {
        let parameter =
            Parameter::constant(-0.0).expect("zero");

        assert_eq!(
            parameter.canonical_string(),
            "0"
        );
    }
}