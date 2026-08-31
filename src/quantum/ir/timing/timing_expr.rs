//! Zamani Quantum IR — Canonical Timing Expressions
//!
//! Hardware-independent, deterministic and resource-safe symbolic timing
//! expressions for the Zamani Quantum Intermediate Representation.
//!
//! # Architectural role
//!
//! `quantum::ir::timing::timing_expr` defines the semantic meaning of
//! expressions whose value is a time quantity.
//!
//! It answers:
//!
//! > "How is this timing value mathematically defined?"
//!
//! It does NOT answer:
//!
//! - when an operation is actually scheduled;
//! - which hardware clock realizes the value;
//! - what a backend's `dt` means;
//! - which physical channel is used;
//! - which qubit is targeted;
//! - how a pulse is calibrated;
//! - how a waveform is sampled;
//! - how routing is performed;
//! - how an operation is optimized;
//! - how a QPU is contacted.
//!
//! Those responsibilities belong to downstream IR consumers.
//!
//! # Design principles
//!
//! The implementation is:
//!
//! - deterministic;
//! - hardware-independent;
//! - exact for concrete timing;
//! - symbolic when timing cannot yet be resolved;
//! - checked for arithmetic overflow;
//! - checked for division by zero;
//! - independent of machine size;
//! - independent of qubit count;
//! - independent of hardware topology;
//! - independent of backend timing units;
//! - safe on Rust 1.97 / 1.97.1;
//! - free of `unsafe` code.
//!
//! # Universal-program principle
//!
//! A Zamani program may contain:
//!
//! ```text
//! 20ns
//! 2 * gate_duration
//! gate_duration + readout_latency
//! 4 * dt
//! start + 20ns
//! duration / 2
//! stretch + 10ns
//! ```
//!
//! The canonical IR must preserve the semantic expression until enough
//! information exists to resolve it.
//!
//! It must NOT prematurely replace symbolic timing with a hardware-specific
//! value.
//!
//! # Important distinction
//!
//! ```text
//! Duration
//!     concrete non-negative elapsed time
//!
//! TimeOffset
//!     concrete signed relative displacement
//!
//! TimingExpression
//!     symbolic or concrete expression describing a timing quantity
//!
//! TimingBinding
//!     explicit mapping from timing symbols to concrete values
//!
//! TimingEvaluation
//!     result of resolving an expression
//! ```
//!
//! # Dependency boundary
//!
//! ```text
//! quantum::ir::parameter
//!          │
//!          ▼
//! timing::timing_expr
//!          │
//!    ┌─────┼───────────────┐
//!    ▼     ▼               ▼
//! timing  pulse         operation
//!    │     │               │
//!    └─────┼───────────────┘
//!          ▼
//!     scheduling
//!          │
//!          ▼
//!       hardware
//! ```
//!
//! `timing_expr.rs` must never depend on scheduling or hardware.
//!
//! # Qubit dependency
//!
//! This module intentionally does NOT import:
//!
//! ```text
//! quantum::ir::qubit
//! ```
//!
//! A timing expression can describe:
//!
//! - qubit operations;
//! - classical operations;
//! - pulses;
//! - frames;
//! - synchronization;
//! - distributed operations;
//! - analog evolution;
//! - measurements;
//! - communication.
//!
//! Qubit identity therefore does not belong in this abstraction.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no `unsafe`.
//!
//! # Integration contract
//!
//! `timing.rs` owns concrete time semantics such as [`Duration`] and
//! [`TimeOffset`].
//!
//! `parameter.rs` owns generic symbolic scalar parameters.
//!
//! This file owns the semantic expression tree that combines those values
//! into timing expressions.
//!
//! `operation.rs` may attach a `TimingExpression` to an operation.
//!
//! `pulse.rs` may use it for symbolic pulse durations.
//!
//! `schedule.rs` may resolve expressions before producing concrete schedules.
//!
//! `validation.rs` validates expression structure and optional policies.
//!
//! `serialization.rs` may serialize the explicit expression tree.
//!
//! `hash.rs` may hash the canonical expression representation.
//!
//! `hardware` resolves symbolic timing against target-specific information.
//!
//! No backend implementation belongs here.

#![forbid(unsafe_code)]

use std::collections::BTreeMap;
use std::fmt;

use crate::quantum::ir::parameter::Parameter;

use super::{Duration, TimeOffset, TimingError, TimingResult};

// =============================================================================
// Policy
// =============================================================================

/// Explicit validation/resource policy for timing expressions.
///
/// This is a compiler/security policy, not an architectural limit on Zamani
/// quantum computers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TimingExpressionPolicy {
    /// Maximum number of expression nodes permitted during validation.
    pub max_nodes: usize,

    /// Maximum expression depth permitted during validation.
    pub max_depth: usize,

    /// Maximum number of symbolic bindings accepted by an evaluation context.
    pub max_bindings: usize,
}

impl TimingExpressionPolicy {
    /// Creates an explicit timing-expression policy.
    #[must_use]
    pub const fn new(
        max_nodes: usize,
        max_depth: usize,
        max_bindings: usize,
    ) -> Self {
        Self {
            max_nodes,
            max_depth,
            max_bindings,
        }
    }

    /// Creates a deliberately permissive policy.
    ///
    /// Resource/security boundaries can be supplied by the compiler or
    /// service layer instead of becoming semantic IR limits.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self {
            max_nodes: usize::MAX,
            max_depth: usize::MAX,
            max_bindings: usize::MAX,
        }
    }

    /// Validates this policy.
    pub const fn validate(self) -> TimingResult<()> {
        if self.max_nodes == 0 {
            return Err(TimingError::InvalidValue {
                message: "timing expression node limit cannot be zero"
                    .to_owned(),
            });
        }

        if self.max_depth == 0 {
            return Err(TimingError::InvalidValue {
                message: "timing expression depth limit cannot be zero"
                    .to_owned(),
            });
        }

        if self.max_bindings == 0 {
            return Err(TimingError::InvalidValue {
                message: "timing expression binding limit cannot be zero"
                    .to_owned(),
            });
        }

        Ok(())
    }
}

impl Default for TimingExpressionPolicy {
    fn default() -> Self {
        Self {
            max_nodes: 1_048_576,
            max_depth: 4_096,
            max_bindings: 1_048_576,
        }
    }
}

// =============================================================================
// Symbol identity
// =============================================================================

/// Stable symbolic identifier used by timing expressions.
///
/// The identifier is a UTF-8 name rather than a machine-specific integer so
/// that serialized IR remains portable across compiler processes and hosts.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TimingSymbol(String);

impl TimingSymbol {
    /// Creates a validated timing symbol.
    pub fn new<S: Into<String>>(name: S) -> TimingResult<Self> {
        let name = name.into();

        if name.is_empty() {
            return Err(TimingError::InvalidValue {
                message: "timing symbol cannot be empty".to_owned(),
            });
        }

        if name.as_bytes().contains(&0) {
            return Err(TimingError::InvalidValue {
                message: "timing symbol cannot contain NUL".to_owned(),
            });
        }

        Ok(Self(name))
    }

    /// Returns the symbol name.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the symbol and returns its name.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for TimingSymbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

// =============================================================================
// Timing value domain
// =============================================================================

/// Semantic dimension of a timing expression.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TimingDimension {
    /// A non-negative elapsed duration.
    Duration,

    /// A signed relative temporal displacement.
    Offset,
}

impl fmt::Display for TimingDimension {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Duration => f.write_str("duration"),
            Self::Offset => f.write_str("offset"),
        }
    }
}

// =============================================================================
// Expression
// =============================================================================

/// Canonical symbolic timing expression.
///
/// The expression tree deliberately contains timing-specific values instead
/// of treating timing as an untyped `f64`.
///
/// This preserves:
///
/// - exact concrete timing;
/// - symbolic timing;
/// - deterministic serialization;
/// - dimensional correctness;
/// - checked arithmetic.
///
/// The tree is an IR semantic structure. It is not an evaluator-specific AST.
#[derive(Debug, Clone, PartialEq)]
pub enum TimingExpression {
    /// Exact non-negative duration.
    Duration(Duration),

    /// Exact signed offset.
    Offset(TimeOffset),

    /// A symbolic timing variable.
    Symbol(TimingSymbol),

    /// A generic canonical parameter used as a scalar timing variable.
///
/// This permits integration with the existing parameter subsystem without
/// duplicating `Parameter`.
    Parameter(Parameter),

    /// Addition.
    Add(
        Box<TimingExpression>,
        Box<TimingExpression>,
    ),

    /// Subtraction.
    Subtract(
        Box<TimingExpression>,
        Box<TimingExpression>,
    ),

    /// Multiplication by a scalar parameter.
///
/// The scalar must be dimensionless. The timing expression supplies the time
/// dimension.
    Multiply(
        Box<TimingExpression>,
        Box<Parameter>,
    ),

    /// Division by a scalar parameter.
///
/// The divisor must be dimensionless and non-zero after resolution.
    Divide(
        Box<TimingExpression>,
        Box<Parameter>,
    ),

    /// Unary negation.
///
/// Negation is valid for offsets. It is rejected when the resulting value
/// would violate a duration-only semantic context.
    Negate(Box<TimingExpression>),

    /// Minimum of two timing expressions.
    Min(
        Box<TimingExpression>,
        Box<TimingExpression>,
    ),

    /// Maximum of two timing expressions.
    Max(
        Box<TimingExpression>,
        Box<TimingExpression>,
    ),
}

impl TimingExpression {
    /// Creates a concrete duration expression.
    #[must_use]
    pub const fn duration(value: Duration) -> Self {
        Self::Duration(value)
    }

    /// Creates a concrete offset expression.
    #[must_use]
    pub const fn offset(value: TimeOffset) -> Self {
        Self::Offset(value)
    }

    /// Creates a symbolic timing expression.
    pub fn symbol<S: Into<String>>(name: S) -> TimingResult<Self> {
        Ok(Self::Symbol(TimingSymbol::new(name)?))
    }

    /// Wraps a canonical Zamani parameter.
    #[must_use]
    pub fn parameter(parameter: Parameter) -> Self {
        Self::Parameter(parameter)
    }

    /// Creates an addition expression.
    #[must_use]
    pub fn add(
        left: Self,
        right: Self,
    ) -> Self {
        Self::Add(Box::new(left), Box::new(right))
    }

    /// Creates a subtraction expression.
    #[must_use]
    pub fn subtract(
        left: Self,
        right: Self,
    ) -> Self {
        Self::Subtract(Box::new(left), Box::new(right))
    }

    /// Creates multiplication by a scalar parameter.
    #[must_use]
    pub fn multiply(
        value: Self,
        scalar: Parameter,
    ) -> Self {
        Self::Multiply(
            Box::new(value),
            Box::new(scalar),
        )
    }

    /// Creates division by a scalar parameter.
    #[must_use]
    pub fn divide(
        value: Self,
        scalar: Parameter,
    ) -> Self {
        Self::Divide(
            Box::new(value),
            Box::new(scalar),
        )
    }

    /// Creates unary negation.
    #[must_use]
    pub fn negate(value: Self) -> Self {
        Self::Negate(Box::new(value))
    }

    /// Creates a minimum expression.
    #[must_use]
    pub fn min(
        left: Self,
        right: Self,
    ) -> Self {
        Self::Min(Box::new(left), Box::new(right))
    }

    /// Creates a maximum expression.
    #[must_use]
    pub fn max(
        left: Self,
        right: Self,
    ) -> Self {
        Self::Max(Box::new(left), Box::new(right))
    }

    /// Returns the semantic dimension if it can be determined statically.
    ///
    /// `None` means that the dimension depends on symbolic information.
    #[must_use]
    pub fn dimension(&self) -> Option<TimingDimension> {
        match self {
            Self::Duration(_) => Some(TimingDimension::Duration),

            Self::Offset(_) => Some(TimingDimension::Offset),

            Self::Symbol(_) | Self::Parameter(_) => None,

            Self::Add(left, right)
            | Self::Subtract(left, right)
            | Self::Min(left, right)
            | Self::Max(left, right) => {
                match (left.dimension(), right.dimension()) {
                    (Some(left), Some(right)) if left == right => {
                        Some(left)
                    }
                    _ => None,
                }
            }

            Self::Multiply(value, _)
            | Self::Divide(value, _) => value.dimension(),

            Self::Negate(value) => value.dimension(),
        }
    }

    /// Returns whether the expression is definitely symbolic.
    #[must_use]
    pub fn is_symbolic(&self) -> bool {
        self.contains_symbolic_value()
    }

    /// Returns whether this expression contains symbolic values.
    pub fn contains_symbolic_value(&self) -> bool {
        let mut stack = vec![self];

        while let Some(node) = stack.pop() {
            match node {
                Self::Duration(_) | Self::Offset(_) => {}

                Self::Symbol(_) | Self::Parameter(_) => {
                    return true;
                }

                Self::Add(left, right)
                | Self::Subtract(left, right)
                | Self::Min(left, right)
                | Self::Max(left, right) => {
                    stack.push(right);
                    stack.push(left);
                }

                Self::Multiply(value, scalar)
                | Self::Divide(value, scalar) => {
                    if parameter_is_symbolic(scalar) {
                        return true;
                    }

                    stack.push(value);
                }

                Self::Negate(value) => {
                    stack.push(value);
                }
            }
        }

        false
    }

    /// Counts nodes iteratively.
    pub fn node_count(&self) -> usize {
        let mut count = 0usize;
        let mut stack = vec![self];

        while let Some(node) = stack.pop() {
            count = count.saturating_add(1);

            match node {
                Self::Duration(_)
                | Self::Offset(_)
                | Self::Symbol(_)
                | Self::Parameter(_) => {}

                Self::Add(left, right)
                | Self::Subtract(left, right)
                | Self::Min(left, right)
                | Self::Max(left, right) => {
                    stack.push(right);
                    stack.push(left);
                }

                Self::Multiply(value, _)
                | Self::Divide(value, _) => {
                    stack.push(value);
                }

                Self::Negate(value) => {
                    stack.push(value);
                }
            }
        }

        count
    }

    /// Computes expression depth iteratively.
    pub fn depth(&self) -> usize {
        let mut maximum = 0usize;
        let mut stack = vec![(self, 0usize)];

        while let Some((node, depth)) = stack.pop() {
            maximum = maximum.max(depth);

            match node {
                Self::Duration(_)
                | Self::Offset(_)
                | Self::Symbol(_)
                | Self::Parameter(_) => {}

                Self::Add(left, right)
                | Self::Subtract(left, right)
                | Self::Min(left, right)
                | Self::Max(left, right) => {
                    stack.push((left, depth.saturating_add(1)));
                    stack.push((right, depth.saturating_add(1)));
                }

                Self::Multiply(value, _)
                | Self::Divide(value, _) => {
                    stack.push((value, depth.saturating_add(1)));
                }

                Self::Negate(value) => {
                    stack.push((value, depth.saturating_add(1)));
                }
            }
        }

        maximum
    }

    /// Validates the expression using the default policy.
    pub fn validate(&self) -> TimingResult<()> {
        self.validate_with_policy(TimingExpressionPolicy::default())
    }

    /// Validates the expression using an explicit policy.
    pub fn validate_with_policy(
        &self,
        policy: TimingExpressionPolicy,
    ) -> TimingResult<()> {
        policy.validate()?;

        let nodes = self.node_count();

        if nodes > policy.max_nodes {
            return Err(TimingError::InvalidValue {
                message: format!(
                    "timing expression contains {nodes} nodes; \
                     policy permits {}",
                    policy.max_nodes
                ),
            });
        }

        let depth = self.depth();

        if depth > policy.max_depth {
            return Err(TimingError::InvalidValue {
                message: format!(
                    "timing expression depth is {depth}; \
                     policy permits {}",
                    policy.max_depth
                ),
            });
        }

        let mut stack = vec![self];

        while let Some(node) = stack.pop() {
            match node {
                Self::Duration(_) | Self::Offset(_) => {}

                Self::Symbol(symbol) => {
                    if symbol.as_str().is_empty() {
                        return Err(TimingError::InvalidValue {
                            message:
                                "timing symbol cannot be empty".to_owned(),
                        });
                    }
                }

                Self::Parameter(parameter) => {
                    parameter
                        .validate()
                        .map_err(|error| {
                            TimingError::InvalidValue {
                                message: error.to_string(),
                            }
                        })?;
                }

                Self::Add(left, right)
                | Self::Subtract(left, right)
                | Self::Min(left, right)
                | Self::Max(left, right) => {
                    stack.push(right);
                    stack.push(left);
                }

                Self::Multiply(value, scalar)
                | Self::Divide(value, scalar) => {
                    scalar
                        .validate()
                        .map_err(|error| {
                            TimingError::InvalidValue {
                                message: error.to_string(),
                            }
                        })?;

                    stack.push(value);
                }

                Self::Negate(value) => {
                    stack.push(value);
                }
            }
        }

        Ok(())
    }

    /// Resolves the expression using a concrete timing environment.
    pub fn evaluate(
        &self,
        environment: &TimingEnvironment,
    ) -> TimingResult<TimingValue> {
        self.evaluate_with_policy(
            environment,
            TimingExpressionPolicy::default(),
        )
    }

    /// Resolves the expression with an explicit evaluation policy.
    ///
    /// Evaluation is iterative and therefore does not recurse through the
    /// Rust call stack for deeply nested expression trees.
    pub fn evaluate_with_policy(
        &self,
        environment: &TimingEnvironment,
        policy: TimingExpressionPolicy,
    ) -> TimingResult<TimingValue> {
        environment.validate(policy)?;

        self.validate_with_policy(policy)?;

        evaluate_expression(self, environment)
    }

    /// Attempts to resolve the expression without a binding environment.
    ///
    /// Returns `Ok(Some(value))` when the expression is fully concrete and
    /// `Ok(None)` when unresolved symbols remain.
    pub fn resolve_if_concrete(
        &self,
    ) -> TimingResult<Option<TimingValue>> {
        let environment = TimingEnvironment::new();

        match evaluate_expression(self, &environment) {
            Ok(value) => Ok(Some(value)),
            Err(TimingError::InvalidValue { message })
                if message.contains("unbound timing symbol")
                    || message.contains("unbound parameter") =>
            {
                Ok(None)
            }
            Err(error) => Err(error),
        }
    }

    /// Collects timing symbols deterministically.
    ///
    /// `BTreeMap`/`BTreeSet`-style ordering is intentionally used by the
    /// environment and collection APIs so canonical serialization and hashing
    /// do not depend on hash-map iteration order.
    pub fn collect_symbols(
        &self,
    ) -> Vec<TimingSymbol> {
        let mut symbols = BTreeMap::<String, TimingSymbol>::new();
        let mut stack = vec![self];

        while let Some(node) = stack.pop() {
            match node {
                Self::Duration(_) | Self::Offset(_) => {}

                Self::Symbol(symbol) => {
                    symbols
                        .entry(symbol.as_str().to_owned())
                        .or_insert_with(|| symbol.clone());
                }

                Self::Parameter(parameter) => {
                    collect_parameter_symbols(
                        parameter,
                        &mut symbols,
                    );
                }

                Self::Add(left, right)
                | Self::Subtract(left, right)
                | Self::Min(left, right)
                | Self::Max(left, right) => {
                    stack.push(right);
                    stack.push(left);
                }

                Self::Multiply(value, scalar)
                | Self::Divide(value, scalar) => {
                    collect_parameter_symbols(
                        scalar,
                        &mut symbols,
                    );
                    stack.push(value);
                }

                Self::Negate(value) => {
                    stack.push(value);
                }
            }
        }

        symbols.into_values().collect()
    }

    /// Returns a deterministic canonical textual representation.
    #[must_use]
    pub fn canonical_string(&self) -> String {
        let mut output = String::new();
        write_canonical(self, &mut output);
        output
    }
}

impl fmt::Display for TimingExpression {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        f.write_str(&self.canonical_string())
    }
}

// =============================================================================
// Resolved value
// =============================================================================

/// Concrete value produced by evaluating a timing expression.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TimingValue {
    /// Non-negative duration.
    Duration(Duration),

    /// Signed offset.
    Offset(TimeOffset),
}

impl TimingValue {
    /// Returns the semantic dimension.
    #[must_use]
    pub const fn dimension(self) -> TimingDimension {
        match self {
            Self::Duration(_) => TimingDimension::Duration,
            Self::Offset(_) => TimingDimension::Offset,
        }
    }

    /// Returns the duration when this is a duration.
    #[must_use]
    pub const fn as_duration(self) -> Option<Duration> {
        match self {
            Self::Duration(value) => Some(value),
            Self::Offset(_) => None,
        }
    }

    /// Returns the offset when this is an offset.
    #[must_use]
    pub const fn as_offset(self) -> Option<TimeOffset> {
        match self {
            Self::Offset(value) => Some(value),
            Self::Duration(_) => None,
        }
    }
}

// =============================================================================
// Timing environment
// =============================================================================

/// Explicit immutable binding environment for timing expressions.
///
/// There is no global timing symbol table.
///
/// A compiler can therefore safely evaluate separate programs concurrently.
#[derive(Debug, Clone, Default)]
pub struct TimingEnvironment {
    bindings: BTreeMap<String, TimingValue>,
    parameters: BTreeMap<String, f64>,
}

impl TimingEnvironment {
    /// Creates an empty environment.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a timing-symbol binding.
    pub fn bind_symbol(
        &mut self,
        symbol: TimingSymbol,
        value: TimingValue,
    ) -> TimingResult<()> {
        self.bindings
            .insert(symbol.into_string(), value);

        Ok(())
    }

    /// Adds a timing-symbol binding by name.
    pub fn bind<S: Into<String>>(
        &mut self,
        name: S,
        value: TimingValue,
    ) -> TimingResult<()> {
        let symbol = TimingSymbol::new(name)?;
        self.bind_symbol(symbol, value)
    }

    /// Adds a scalar parameter binding.
    pub fn bind_parameter<S: Into<String>>(
        &mut self,
        name: S,
        value: f64,
    ) -> TimingResult<()> {
        if !value.is_finite() {
            return Err(TimingError::InvalidValue {
                message:
                    "timing parameter binding must be finite"
                        .to_owned(),
            });
        }

        let name = name.into();

        if name.is_empty() {
            return Err(TimingError::InvalidValue {
                message:
                    "timing parameter binding name cannot be empty"
                        .to_owned(),
            });
        }

        self.parameters.insert(name, value);

        Ok(())
    }

    /// Returns a timing-symbol binding.
    #[must_use]
    pub fn get(
        &self,
        symbol: &TimingSymbol,
    ) -> Option<TimingValue> {
        self.bindings.get(symbol.as_str()).copied()
    }

    /// Returns a scalar parameter binding.
    #[must_use]
    pub fn get_parameter(
        &self,
        name: &str,
    ) -> Option<f64> {
        self.parameters.get(name).copied()
    }

    /// Number of timing bindings.
    #[must_use]
    pub fn binding_count(&self) -> usize {
        self.bindings.len()
    }

    /// Number of scalar parameter bindings.
    #[must_use]
    pub fn parameter_count(&self) -> usize {
        self.parameters.len()
    }

    /// Validates the environment.
    pub fn validate(
        &self,
        policy: TimingExpressionPolicy,
    ) -> TimingResult<()> {
        policy.validate()?;

        if self.bindings.len() > policy.max_bindings {
            return Err(TimingError::InvalidValue {
                message: format!(
                    "timing environment contains {} bindings; \
                     policy permits {}",
                    self.bindings.len(),
                    policy.max_bindings
                ),
            });
        }

        if self.parameters.len() > policy.max_bindings {
            return Err(TimingError::InvalidValue {
                message: format!(
                    "timing environment contains {} parameter bindings; \
                     policy permits {}",
                    self.parameters.len(),
                    policy.max_bindings
                ),
            });
        }

        for (name, value) in &self.parameters {
            if name.is_empty() || !value.is_finite() {
                return Err(TimingError::InvalidValue {
                    message:
                        "timing environment contains invalid parameter"
                            .to_owned(),
                });
            }
        }

        Ok(())
    }
}

// =============================================================================
// Evaluation
// =============================================================================

#[derive(Debug, Clone, Copy)]
enum EvalFrame<'a> {
    Visit(&'a TimingExpression),
    ApplyBinary(BinaryOperator),
    ApplyUnary,
    ApplyScalar(ScalarOperator, f64),
}

#[derive(Debug, Clone, Copy)]
enum BinaryOperator {
    Add,
    Subtract,
    Min,
    Max,
}

#[derive(Debug, Clone, Copy)]
enum ScalarOperator {
    Multiply,
    Divide,
}

fn evaluate_expression(
    root: &TimingExpression,
    environment: &TimingEnvironment,
) -> TimingResult<TimingValue> {
    let mut frames = vec![EvalFrame::Visit(root)];
    let mut values = Vec::<TimingValue>::new();

    while let Some(frame) = frames.pop() {
        match frame {
            EvalFrame::Visit(node) => {
                match node {
                    TimingExpression::Duration(value) => {
                        values.push(TimingValue::Duration(*value));
                    }

                    TimingExpression::Offset(value) => {
                        values.push(TimingValue::Offset(*value));
                    }

                    TimingExpression::Symbol(symbol) => {
                        let value = environment
                            .get(symbol)
                            .ok_or_else(|| {
                                TimingError::InvalidValue {
                                    message: format!(
                                        "unbound timing symbol `{symbol}`"
                                    ),
                                }
                            })?;

                        values.push(value);
                    }

                    TimingExpression::Parameter(parameter) => {
                        let value =
                            evaluate_parameter(parameter, environment)?;

                        values.push(TimingValue::Duration(
                            parameter_to_duration(value)?,
                        ));
                    }

                    TimingExpression::Add(left, right) => {
                        frames.push(EvalFrame::ApplyBinary(
                            BinaryOperator::Add,
                        ));
                        frames.push(EvalFrame::Visit(right));
                        frames.push(EvalFrame::Visit(left));
                    }

                    TimingExpression::Subtract(left, right) => {
                        frames.push(EvalFrame::ApplyBinary(
                            BinaryOperator::Subtract,
                        ));
                        frames.push(EvalFrame::Visit(right));
                        frames.push(EvalFrame::Visit(left));
                    }

                    TimingExpression::Min(left, right) => {
                        frames.push(EvalFrame::ApplyBinary(
                            BinaryOperator::Min,
                        ));
                        frames.push(EvalFrame::Visit(right));
                        frames.push(EvalFrame::Visit(left));
                    }

                    TimingExpression::Max(left, right) => {
                        frames.push(EvalFrame::ApplyBinary(
                            BinaryOperator::Max,
                        ));
                        frames.push(EvalFrame::Visit(right));
                        frames.push(EvalFrame::Visit(left));
                    }

                    TimingExpression::Multiply(value, scalar) => {
                        let factor =
                            evaluate_parameter(scalar, environment)?;

                        frames.push(EvalFrame::ApplyScalar(
                            ScalarOperator::Multiply,
                            factor,
                        ));
                        frames.push(EvalFrame::Visit(value));
                    }

                    TimingExpression::Divide(value, scalar) => {
                        let divisor =
                            evaluate_parameter(scalar, environment)?;

                        if divisor == 0.0 {
                            return Err(
                                TimingError::DivisionByZero,
                            );
                        }

                        frames.push(EvalFrame::ApplyScalar(
                            ScalarOperator::Divide,
                            divisor,
                        ));
                        frames.push(EvalFrame::Visit(value));
                    }

                    TimingExpression::Negate(value) => {
                        frames.push(EvalFrame::ApplyUnary);
                        frames.push(EvalFrame::Visit(value));
                    }
                }
            }

            EvalFrame::ApplyBinary(operator) => {
                let right = values.pop().ok_or_else(|| {
                    TimingError::InvalidValue {
                        message:
                            "malformed timing expression stack"
                                .to_owned(),
                    }
                })?;

                let left = values.pop().ok_or_else(|| {
                    TimingError::InvalidValue {
                        message:
                            "malformed timing expression stack"
                                .to_owned(),
                    }
                })?;

                values.push(apply_binary(operator, left, right)?);
            }

            EvalFrame::ApplyUnary => {
                let value = values.pop().ok_or_else(|| {
                    TimingError::InvalidValue {
                        message:
                            "malformed timing expression stack"
                                .to_owned(),
                    }
                })?;

                values.push(apply_unary(value)?);
            }

            EvalFrame::ApplyScalar(operator, scalar) => {
                let value = values.pop().ok_or_else(|| {
                    TimingError::InvalidValue {
                        message:
                            "malformed timing expression stack"
                                .to_owned(),
                    }
                })?;

                values.push(apply_scalar(
                    operator,
                    value,
                    scalar,
                )?);
            }
        }
    }

    values.pop().ok_or_else(|| TimingError::InvalidValue {
        message: "timing expression produced no value".to_owned(),
    })
}

fn apply_binary(
    operator: BinaryOperator,
    left: TimingValue,
    right: TimingValue,
) -> TimingResult<TimingValue> {
    match operator {
        BinaryOperator::Add => add_values(left, right),

        BinaryOperator::Subtract => {
            subtract_values(left, right)
        }

        BinaryOperator::Min => {
            if left.dimension() != right.dimension() {
                return Err(TimingError::InvalidValue {
                    message:
                        "min requires operands with matching timing dimensions"
                            .to_owned(),
                });
            }

            Ok(if timing_value_cmp(left, right)
                == Ordering::Greater
            {
                right
            } else {
                left
            })
        }

        BinaryOperator::Max => {
            if left.dimension() != right.dimension() {
                return Err(TimingError::InvalidValue {
                    message:
                        "max requires operands with matching timing dimensions"
                            .to_owned(),
                });
            }

            Ok(if timing_value_cmp(left, right)
                == Ordering::Less
            {
                right
            } else {
                left
            })
        }
    }
}

fn add_values(
    left: TimingValue,
    right: TimingValue,
) -> TimingResult<TimingValue> {
    match (left, right) {
        (
            TimingValue::Duration(left),
            TimingValue::Duration(right),
        ) => Ok(TimingValue::Duration(
            left.checked_add(right)?,
        )),

        (
            TimingValue::Offset(left),
            TimingValue::Offset(right),
        ) => Ok(TimingValue::Offset(
            left.checked_add(right)?,
        )),

        (
            TimingValue::Duration(duration),
            TimingValue::Offset(offset),
        )
        | (
            TimingValue::Offset(offset),
            TimingValue::Duration(duration),
        ) => {
            let duration_offset =
                TimeOffset::positive(duration)?;

            let result =
                duration_offset.checked_add(offset)?;

            Ok(TimingValue::Offset(result))
        }
    }
}

fn subtract_values(
    left: TimingValue,
    right: TimingValue,
) -> TimingResult<TimingValue> {
    match (left, right) {
        (
            TimingValue::Duration(left),
            TimingValue::Duration(right),
        ) => match left.checked_sub(right) {
            Ok(value) => Ok(TimingValue::Duration(value)),

            Err(TimingError::NegativeDuration) => {
                let magnitude = right.checked_sub(left)?;

                Ok(TimingValue::Offset(
                    TimeOffset::negative(magnitude)?,
                ))
            }

            Err(error) => Err(error),
        },

        (
            TimingValue::Offset(left),
            TimingValue::Offset(right),
        ) => Ok(TimingValue::Offset(
            left.checked_sub(right)?,
        )),

        (
            TimingValue::Offset(offset),
            TimingValue::Duration(duration),
        ) => {
            let duration_offset =
                TimeOffset::negative(duration)?;

            Ok(TimingValue::Offset(
                offset.checked_add(duration_offset)?,
            ))
        }

        (
            TimingValue::Duration(duration),
            TimingValue::Offset(offset),
        ) => {
            let duration_offset =
                TimeOffset::positive(duration)?;

            Ok(TimingValue::Offset(
                duration_offset.checked_sub(offset)?,
            ))
        }
    }
}

fn apply_unary(
    value: TimingValue,
) -> TimingResult<TimingValue> {
    match value {
        TimingValue::Duration(duration) => {
            if duration.is_zero() {
                Ok(TimingValue::Duration(duration))
            } else {
                let offset =
                    TimeOffset::negative(duration)?;

                Ok(TimingValue::Offset(offset))
            }
        }

        TimingValue::Offset(offset) => {
            let magnitude = offset.magnitude()?;

            if offset.is_negative() {
                Ok(TimingValue::Offset(
                    TimeOffset::positive(magnitude)?,
                ))
            } else {
                Ok(TimingValue::Offset(
                    TimeOffset::negative(magnitude)?,
                ))
            }
        }
    }
}

fn apply_scalar(
    operator: ScalarOperator,
    value: TimingValue,
    scalar: f64,
) -> TimingResult<TimingValue> {
    if !scalar.is_finite() {
        return Err(TimingError::InvalidValue {
            message:
                "timing scalar must be finite".to_owned(),
        });
    }

    if matches!(operator, ScalarOperator::Divide)
        && scalar == 0.0
    {
        return Err(TimingError::DivisionByZero);
    }

    match value {
        TimingValue::Duration(duration) => {
            let result =
                scale_unsigned_duration(duration, scalar)?;

            if scalar >= 0.0 {
                Ok(TimingValue::Duration(result))
            } else {
                Ok(TimingValue::Offset(
                    TimeOffset::negative(result)?,
                ))
            }
        }

        TimingValue::Offset(offset) => {
            let result = scale_offset(offset, scalar)?;

            Ok(TimingValue::Offset(result))
        }
    }
}

// =============================================================================
// Scalar evaluation
// =============================================================================

fn evaluate_parameter(
    parameter: &Parameter,
    environment: &TimingEnvironment,
) -> TimingResult<f64> {
    match parameter {
        Parameter::Constant(value) => {
            if value.is_finite() {
                Ok(*value)
            } else {
                Err(TimingError::InvalidValue {
                    message:
                        "timing parameter must be finite".to_owned(),
                })
            }
        }

        Parameter::Symbol(name) => environment
            .get_parameter(name)
            .ok_or_else(|| TimingError::InvalidValue {
                message: format!(
                    "unbound parameter `{name}`"
                ),
            }),

        Parameter::Expression(expression) => {
            evaluate_parameter_expression(
                expression,
                environment,
            )
        }
    }
}

/// Evaluates a Zamani parameter expression iteratively.
///
/// This intentionally does not depend on the parameter evaluator internals,
/// keeping timing expressions independent of optimization implementation.
fn evaluate_parameter_expression(
    expression: &crate::quantum::ir::parameter::ParameterExpression,
    environment: &TimingEnvironment,
) -> TimingResult<f64> {
    use crate::quantum::ir::parameter::ParameterExpression;

    enum Frame<'a> {
        Visit(&'a ParameterExpression),
        ApplyBinary(ParameterBinary),
        ApplyNegate,
    }

    #[derive(Clone, Copy)]
    enum ParameterBinary {
        Add,
        Subtract,
        Multiply,
        Divide,
    }

    let mut frames = vec![Frame::Visit(expression)];
    let mut values = Vec::<f64>::new();

    while let Some(frame) = frames.pop() {
        match frame {
            Frame::Visit(node) => match node {
                ParameterExpression::Constant(value) => {
                    values.push(*value);
                }

                ParameterExpression::Symbol(name) => {
                    let value = environment
                        .get_parameter(name)
                        .ok_or_else(|| {
                            TimingError::InvalidValue {
                                message: format!(
                                    "unbound parameter `{name}`"
                                ),
                            }
                        })?;

                    values.push(value);
                }

                ParameterExpression::Add(left, right) => {
                    frames.push(Frame::ApplyBinary(
                        ParameterBinary::Add,
                    ));
                    frames.push(Frame::Visit(right));
                    frames.push(Frame::Visit(left));
                }

                ParameterExpression::Subtract(left, right) => {
                    frames.push(Frame::ApplyBinary(
                        ParameterBinary::Subtract,
                    ));
                    frames.push(Frame::Visit(right));
                    frames.push(Frame::Visit(left));
                }

                ParameterExpression::Multiply(left, right) => {
                    frames.push(Frame::ApplyBinary(
                        ParameterBinary::Multiply,
                    ));
                    frames.push(Frame::Visit(right));
                    frames.push(Frame::Visit(left));
                }

                ParameterExpression::Divide(left, right) => {
                    frames.push(Frame::ApplyBinary(
                        ParameterBinary::Divide,
                    ));
                    frames.push(Frame::Visit(right));
                    frames.push(Frame::Visit(left));
                }

                ParameterExpression::Negate(value) => {
                    frames.push(Frame::ApplyNegate);
                    frames.push(Frame::Visit(value));
                }
            },

            Frame::ApplyBinary(operator) => {
                let right = values.pop().ok_or_else(|| {
                    TimingError::InvalidValue {
                        message:
                            "malformed parameter expression"
                                .to_owned(),
                    }
                })?;

                let left = values.pop().ok_or_else(|| {
                    TimingError::InvalidValue {
                        message:
                            "malformed parameter expression"
                                .to_owned(),
                    }
                })?;

                let result = match operator {
                    ParameterBinary::Add => {
                        left.checked_add(right)
                    }

                    ParameterBinary::Subtract => {
                        left.checked_sub(right)
                    }

                    ParameterBinary::Multiply => {
                        left.checked_mul(right)
                    }

                    ParameterBinary::Divide => {
                        if right == 0.0 {
                            return Err(
                                TimingError::DivisionByZero,
                            );
                        }

                        left.checked_div(right)
                    }
                };

                let result =
                    result.ok_or_else(|| {
                        TimingError::InvalidValue {
                            message:
                                "non-finite parameter expression result"
                                    .to_owned(),
                        }
                    })?;

                if !result.is_finite() {
                    return Err(TimingError::InvalidValue {
                        message:
                            "parameter expression produced a non-finite value"
                                .to_owned(),
                    });
                }

                values.push(result);
            }

            Frame::ApplyNegate => {
                let value = values.pop().ok_or_else(|| {
                    TimingError::InvalidValue {
                        message:
                            "malformed parameter expression"
                                .to_owned(),
                    }
                })?;

                let result =
                    value.checked_neg().ok_or_else(|| {
                        TimingError::InvalidValue {
                            message:
                                "parameter negation overflow"
                                    .to_owned(),
                        }
                    })?;

                values.push(result);
            }
        }
    }

    values.pop().ok_or_else(|| TimingError::InvalidValue {
        message:
            "parameter expression produced no value".to_owned(),
    })
}

fn parameter_to_duration(
    value: f64,
) -> TimingResult<Duration> {
    if !value.is_finite() || value < 0.0 {
        return Err(TimingError::InvalidValue {
            message:
                "timing parameter used as a duration must be finite and non-negative"
                    .to_owned(),
        });
    }

    let nanos =
        value * super::ATTOSECONDS_PER_SECOND as f64;

    if !nanos.is_finite()
        || nanos > u128::MAX as f64
    {
        return Err(TimingError::ArithmeticOverflow);
    }

    let attoseconds = nanos.round() as u128;

    Ok(Duration::from_attoseconds(attoseconds))
}

// =============================================================================
// Scaling
// =============================================================================

fn scale_unsigned_duration(
    duration: Duration,
    scalar: f64,
) -> TimingResult<Duration> {
    if !scalar.is_finite() || scalar < 0.0 {
        return Err(TimingError::InvalidValue {
            message:
                "duration scaling requires a finite non-negative scalar"
                    .to_owned(),
        });
    }

    let value =
        duration.attoseconds() as f64 * scalar;

    if !value.is_finite()
        || value > u128::MAX as f64
    {
        return Err(TimingError::ArithmeticOverflow);
    }

    Ok(Duration::from_attoseconds(
        value.round() as u128,
    ))
}

fn scale_offset(
    offset: TimeOffset,
    scalar: f64,
) -> TimingResult<TimeOffset> {
    let value =
        offset.attoseconds() as f64 * scalar;

    if !value.is_finite()
        || value < i128::MIN as f64
        || value > i128::MAX as f64
    {
        return Err(TimingError::ArithmeticOverflow);
    }

    Ok(TimeOffset::from_attoseconds(
        value.round() as i128,
    ))
}

// =============================================================================
// Symbol helpers
// =============================================================================

fn parameter_is_symbolic(
    parameter: &Parameter,
) -> bool {
    match parameter {
        Parameter::Constant(_) => false,

        Parameter::Symbol(_) => true,

        Parameter::Expression(expression) => {
            expression_is_symbolic(expression)
        }
    }
}

fn expression_is_symbolic(
    expression: &crate::quantum::ir::parameter::ParameterExpression,
) -> bool {
    use crate::quantum::ir::parameter::ParameterExpression;

    let mut stack = vec![expression];

    while let Some(node) = stack.pop() {
        match node {
            ParameterExpression::Constant(_) => {}

            ParameterExpression::Symbol(_) => {
                return true;
            }

            ParameterExpression::Add(left, right)
            | ParameterExpression::Subtract(left, right)
            | ParameterExpression::Multiply(left, right)
            | ParameterExpression::Divide(left, right) => {
                stack.push(right);
                stack.push(left);
            }

            ParameterExpression::Negate(value) => {
                stack.push(value);
            }
        }
    }

    false
}

fn collect_parameter_symbols(
    parameter: &Parameter,
    output: &mut BTreeMap<String, TimingSymbol>,
) {
    match parameter {
        Parameter::Constant(_) => {}

        Parameter::Symbol(name) => {
            if let Ok(symbol) =
                TimingSymbol::new(name.clone())
            {
                output
                    .entry(name.clone())
                    .or_insert(symbol);
            }
        }

        Parameter::Expression(expression) => {
            collect_parameter_expression_symbols(
                expression,
                output,
            );
        }
    }
}

fn collect_parameter_expression_symbols(
    expression: &crate::quantum::ir::parameter::ParameterExpression,
    output: &mut BTreeMap<String, TimingSymbol>,
) {
    use crate::quantum::ir::parameter::ParameterExpression;

    let mut stack = vec![expression];

    while let Some(node) = stack.pop() {
        match node {
            ParameterExpression::Constant(_) => {}

            ParameterExpression::Symbol(name) => {
                if let Ok(symbol) =
                    TimingSymbol::new(name.clone())
                {
                    output
                        .entry(name.clone())
                        .or_insert(symbol);
                }
            }

            ParameterExpression::Add(left, right)
            | ParameterExpression::Subtract(left, right)
            | ParameterExpression::Multiply(left, right)
            | ParameterExpression::Divide(left, right) => {
                stack.push(right);
                stack.push(left);
            }

            ParameterExpression::Negate(value) => {
                stack.push(value);
            }
        }
    }
}

// =============================================================================
// Comparison
// =============================================================================

fn timing_value_cmp(
    left: TimingValue,
    right: TimingValue,
) -> Ordering {
    match (left, right) {
        (
            TimingValue::Duration(left),
            TimingValue::Duration(right),
        ) => left.cmp(&right),

        (
            TimingValue::Offset(left),
            TimingValue::Offset(right),
        ) => left.attoseconds().cmp(&right.attoseconds()),

        _ => Ordering::Equal,
    }
}

// =============================================================================
// Canonical formatting
// =============================================================================

fn write_canonical(
    expression: &TimingExpression,
    output: &mut String,
) {
    match expression {
        TimingExpression::Duration(value) => {
            output.push_str(&value.canonical_string());
        }

        TimingExpression::Offset(value) => {
            output.push_str("offset(");

            if value.attoseconds() < 0 {
                output.push('-');
            }

            let magnitude =
                value.magnitude().unwrap_or(Duration::MAX);

            output.push_str(
                &magnitude.canonical_string(),
            );

            output.push(')');
        }

        TimingExpression::Symbol(symbol) => {
            output.push_str("symbol(");
            append_escaped(output, symbol.as_str());
            output.push(')');
        }

        TimingExpression::Parameter(parameter) => {
            output.push_str("parameter(");
            write_parameter_canonical(
                parameter,
                output,
            );
            output.push(')');
        }

        TimingExpression::Add(left, right) => {
            output.push_str("add(");
            write_canonical(left, output);
            output.push(',');
            write_canonical(right, output);
            output.push(')');
        }

        TimingExpression::Subtract(left, right) => {
            output.push_str("sub(");
            write_canonical(left, output);
            output.push(',');
            write_canonical(right, output);
            output.push(')');
        }

        TimingExpression::Multiply(value, scalar) => {
            output.push_str("mul(");
            write_canonical(value, output);
            output.push(',');
            write_parameter_canonical(
                scalar,
                output,
            );
            output.push(')');
        }

        TimingExpression::Divide(value, scalar) => {
            output.push_str("div(");
            write_canonical(value, output);
            output.push(',');
            write_parameter_canonical(
                scalar,
                output,
            );
            output.push(')');
        }

        TimingExpression::Negate(value) => {
            output.push_str("neg(");
            write_canonical(value, output);
            output.push(')');
        }

        TimingExpression::Min(left, right) => {
            output.push_str("min(");
            write_canonical(left, output);
            output.push(',');
            write_canonical(right, output);
            output.push(')');
        }

        TimingExpression::Max(left, right) => {
            output.push_str("max(");
            write_canonical(left, output);
            output.push(',');
            write_canonical(right, output);
            output.push(')');
        }
    }
}

fn write_parameter_canonical(
    parameter: &Parameter,
    output: &mut String,
) {
    match parameter {
        Parameter::Constant(value) => {
            output.push_str(&format!("{value:.17}"));
        }

        Parameter::Symbol(name) => {
            output.push_str("symbol(");
            append_escaped(output, name);
            output.push(')');
        }

        Parameter::Expression(expression) => {
            output.push_str("expr(");

            use crate::quantum::ir::parameter::ParameterExpression;

            match expression.as_ref() {
                ParameterExpression::Constant(value) => {
                    output.push_str("const(");
                    output.push_str(&format!("{value:.17}"));
                    output.push(')');
                }

                ParameterExpression::Symbol(name) => {
                    output.push_str("symbol(");
                    append_escaped(output, name);
                    output.push(')');
                }

                ParameterExpression::Add(left, right) => {
                    output.push_str("add(");
                    write_parameter_canonical(
                        &Parameter::Expression(left.clone()),
                        output,
                    );
                    output.push(',');
                    write_parameter_canonical(
                        &Parameter::Expression(right.clone()),
                        output,
                    );
                    output.push(')');
                }

                ParameterExpression::Subtract(left, right) => {
                    output.push_str("sub(");
                    write_parameter_canonical(
                        &Parameter::Expression(left.clone()),
                        output,
                    );
                    output.push(',');
                    write_parameter_canonical(
                        &Parameter::Expression(right.clone()),
                        output,
                    );
                    output.push(')');
                }

                ParameterExpression::Multiply(left, right) => {
                    output.push_str("mul(");
                    write_parameter_canonical(
                        &Parameter::Expression(left.clone()),
                        output,
                    );
                    output.push(',');
                    write_parameter_canonical(
                        &Parameter::Expression(right.clone()),
                        output,
                    );
                    output.push(')');
                }

                ParameterExpression::Divide(left, right) => {
                    output.push_str("div(");
                    write_parameter_canonical(
                        &Parameter::Expression(left.clone()),
                        output,
                    );
                    output.push(',');
                    write_parameter_canonical(
                        &Parameter::Expression(right.clone()),
                        output,
                    );
                    output.push(')');
                }

                ParameterExpression::Negate(value) => {
                    output.push_str("neg(");
                    write_parameter_canonical(
                        &Parameter::Expression(value.clone()),
                        output,
                    );
                    output.push(')');
                }
            }

            output.push(')');
        }
    }
}

fn append_escaped(
    output: &mut String,
    value: &str,
) {
    for character in value.chars() {
        match character {
            '\\' => output.push_str("\\\\"),
            '"' => output.push_str("\\\""),
            '\n' => output.push_str("\\n"),
            '\r' => output.push_str("\\r"),
            '\t' => output.push_str("\\t"),
            character => output.push(character),
        }
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn ns(value: u128) -> Duration {
        Duration::nanoseconds(value)
            .expect("nanoseconds must fit")
    }

    #[test]
    fn concrete_duration_evaluates_exactly() {
        let expression =
            TimingExpression::duration(ns(20));

        let value = expression
            .evaluate(&TimingEnvironment::new())
            .expect("evaluation must succeed");

        assert_eq!(
            value,
            TimingValue::Duration(ns(20))
        );
    }

    #[test]
    fn timing_symbol_resolves_explicitly() {
        let expression =
            TimingExpression::symbol("gate_duration")
                .expect("symbol must be valid");

        let mut environment =
            TimingEnvironment::new();

        environment
            .bind(
                "gate_duration",
                TimingValue::Duration(ns(20)),
            )
            .expect("binding must succeed");

        assert_eq!(
            expression
                .evaluate(&environment)
                .expect("evaluation must succeed"),
            TimingValue::Duration(ns(20))
        );
    }

    #[test]
    fn addition_is_exact() {
        let expression =
            TimingExpression::add(
                TimingExpression::duration(ns(20)),
                TimingExpression::duration(ns(30)),
            );

        assert_eq!(
            expression
                .evaluate(&TimingEnvironment::new())
                .expect("evaluation must succeed"),
            TimingValue::Duration(ns(50))
        );
    }

    #[test]
    fn subtraction_can_produce_offset() {
        let expression =
            TimingExpression::subtract(
                TimingExpression::duration(ns(20)),
                TimingExpression::duration(ns(30)),
            );

        let value = expression
            .evaluate(&TimingEnvironment::new())
            .expect("evaluation must succeed");

        assert_eq!(
            value,
            TimingValue::Offset(
                TimeOffset::negative(ns(10))
                    .expect("negative offset must fit")
            )
        );
    }

    #[test]
    fn min_and_max_are_dimension_safe() {
        let min =
            TimingExpression::min(
                TimingExpression::duration(ns(20)),
                TimingExpression::duration(ns(30)),
            );

        let max =
            TimingExpression::max(
                TimingExpression::duration(ns(20)),
                TimingExpression::duration(ns(30)),
            );

        assert_eq!(
            min.evaluate(&TimingEnvironment::new())
                .expect("min must succeed"),
            TimingValue::Duration(ns(20))
        );

        assert_eq!(
            max.evaluate(&TimingEnvironment::new())
                .expect("max must succeed"),
            TimingValue::Duration(ns(30))
        );
    }

    #[test]
    fn division_by_zero_is_rejected() {
        let expression =
            TimingExpression::divide(
                TimingExpression::duration(ns(20)),
                Parameter::constant(0.0)
                    .expect("zero is a valid scalar"),
            );

        assert_eq!(
            expression
                .evaluate(&TimingEnvironment::new())
                .expect_err("division by zero must fail"),
            TimingError::DivisionByZero
        );
    }

    #[test]
    fn canonical_representation_is_deterministic() {
        let expression =
            TimingExpression::add(
                TimingExpression::duration(ns(20)),
                TimingExpression::duration(ns(30)),
            );

        assert_eq!(
            expression.canonical_string(),
            "add(20ns,30ns)"
        );

        assert_eq!(
            expression.canonical_string(),
            expression.canonical_string()
        );
    }

    #[test]
    fn deep_expression_does_not_require_recursive_evaluation() {
        let mut expression =
            TimingExpression::duration(ns(1));

        for _ in 0..10_000 {
            expression =
                TimingExpression::add(
                    expression,
                    TimingExpression::duration(ns(1)),
                );
        }

        assert_eq!(
            expression.node_count(),
            20_001
        );

        let policy =
            TimingExpressionPolicy::unlimited();

        assert!(
            expression
                .evaluate_with_policy(
                    &TimingEnvironment::new(),
                    policy,
                )
                .is_ok()
        );
    }

    #[test]
    fn symbols_are_collected_deterministically() {
        let a =
            TimingExpression::symbol("a")
                .expect("symbol");
        let b =
            TimingExpression::symbol("b")
                .expect("symbol");

        let expression =
            TimingExpression::add(
                a.clone(),
                TimingExpression::add(
                    b,
                    a,
                ),
            );

        let symbols =
            expression.collect_symbols();

        let names: Vec<&str> = symbols
            .iter()
            .map(TimingSymbol::as_str)
            .collect();

        assert_eq!(
            names,
            vec!["a", "b"]
        );
    }

    #[test]
    fn environment_is_not_global() {
        let expression =
            TimingExpression::symbol("duration")
                .expect("symbol");

        let mut first =
            TimingEnvironment::new();

        first
            .bind(
                "duration",
                TimingValue::Duration(ns(10)),
            )
            .expect("binding");

        let mut second =
            TimingEnvironment::new();

        second
            .bind(
                "duration",
                TimingValue::Duration(ns(20)),
            )
            .expect("binding");

        assert_ne!(
            expression.evaluate(&first)
                .expect("first evaluation"),
            expression.evaluate(&second)
                .expect("second evaluation")
        );
    }

    #[test]
    fn no_qubit_dependency_exists() {
        // This test is intentionally documentation-level:
        // timing expressions are independent of qubit identity.
        //
        // QubitId belongs to the enclosing operation and must not become a
        // dependency of this module.
        let expression =
            TimingExpression::duration(ns(20));

        assert_eq!(
            expression.dimension(),
            Some(TimingDimension::Duration)
        );
    }
}