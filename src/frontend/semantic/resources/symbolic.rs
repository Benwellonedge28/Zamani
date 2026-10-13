
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani Symbolic Resource Expressions
//!
//! Path: `src/frontend/semantic/resources/symbolic.rs`
//!
//! ## PURPOSE
//!
//! Represent, normalize, compare, and preserve symbolic resource quantities
//! independently of physical hardware and host integer widths.
//!
//! ## STATUS
//!
//! Implementation candidate. Repository integration, compilation, and
//! conformance testing are required before this file can be marked frozen.
//!
//! ## AUTHORITY
//!
//! - `grammar/DESIGN.md`
//! - `grammar/Zamani.g4`
//! - `grammar/grammar.md`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/spec/type-system.md`
//! - `src/frontend/ast/node/node_id.rs`
//! - `src/frontend/ast/node/resources/resource.rs`
//! - `src/frontend/semantic/resources/model.rs`
//! - `src/frontend/semantic/types/constants.rs`
//!
//! The grammar owns syntax. The canonical AST owns source expression
//! identities. The semantic type and constant-evaluation subsystems own
//! expression typing and compile-time evaluation. The resource model owns
//! canonical semantic resource quantities.
//!
//! ## OWNS
//!
//! - Symbolic resource-expression structure.
//! - Canonical expression keys and deterministic local normalization.
//! - Safe algebraic simplification of supported non-negative integer
//!   constants.
//! - Symbolic expression validation.
//! - Explicit normalization work accounting.
//! - Conversion of a single AST expression reference into the existing
//!   `ResourceQuantity` representation.
//! - Structured errors for invalid operations and exhausted work budgets.
//!
//! ## DOES_NOT_OWN
//!
//! - Source parsing or AST construction.
//! - AST node identities or AST expression semantics.
//! - Name resolution or symbol tables.
//! - Type inference or type-level constraint solving.
//! - General constant-expression evaluation.
//! - Unit conversion or dimensional analysis.
//! - Capability discovery.
//! - Resource allocation, target selection, scheduling, routing, or lowering.
//! - Physical qubit, register, device, memory, or accelerator limits.
//! - Runtime resource handles.
//!
//! ## INTEGRATION CONTRACT
//!
//! `model.rs` remains the owner of `ExactQuantity` and `ResourceQuantity`.
//! `NodeId` remains the canonical identity of a source expression.
//!
//! This module does not invent a NodeId for a derived expression. If a
//! compound symbolic expression must be represented in `ResourceQuantity`,
//! the owning semantic-expression or AST-construction service must provide
//! its real, canonical AST identity.
//!
//! `types/constants.rs` remains the authority for evaluating general
//! compile-time expressions. This module only folds operations for which
//! it can prove the result exactly using the supported representation.
//!
//! `requirements.rs` preserves source resource declarations.
//! `constraints.rs` consumes resource requirements and constraints.
//! `resources/mod.rs` exports this module's intended public API.
//!
//! ## SCALABILITY
//!
//! No machine-capacity limit is encoded here. Resource cardinalities are not
//! converted to usize, u64, u128, or floating-point values.
//!
//! Exact integer constants are represented using decimal digits. Compound
//! expressions remain symbolic when their exact value cannot be established
//! by this module.
//!
//! Memory, address space, compilation time, and explicitly configured
//! work budgets remain finite implementation resources.
//!
//! ## DETERMINISM
//!
//! Canonical expression keys use explicit operator names and stable operand
//! ordering. No randomized hashing, global mutable state, hardware queries,
//! filesystem access, environment access, or timestamps are used.
//!
//! Canonicalization is structural. It does not claim algebraic equivalence
//! for arbitrary expressions, nor does it perform transformations that could
//! change overflow, evaluation-order, or language-defined error semantics.
//!
//! ## FREEZE CRITERIA
//!
//! - The canonical AST and resource model APIs compile against this module.
//! - `resources/mod.rs` exports the intended public types.
//! - Exact integer folding and symbolic preservation tests pass.
//! - Invalid subtraction and division by zero are handled explicitly.
//! - Normalization budgets are enforced without claiming semantic validity
//!   for an incomplete operation.
//! - Determinism, fuzz, and stress tests pass.
//! - Full workspace checks pass on the supported Rust toolchain.

use core::cmp::Ordering;
use core::fmt;
use std::error::Error;

use crate::frontend::ast::node::node_id::NodeId;

use super::model::{ExactQuantity, ResourceQuantity};

/// The symbolic-resource module's in-memory schema version.
///
/// This is not the Zamani language version or serialized AST version.
pub const SYMBOLIC_RESOURCE_SCHEMA_VERSION: u16 = 1;

/// An operator supported by symbolic resource expressions.
///
/// This is an extensible semantic expression vocabulary, not a list of
/// source-language keywords. Source syntax must be translated by the
/// canonical AST and semantic expression services.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum SymbolicOperator {
    /// Addition.
    Add,

    /// Subtraction.
    Subtract,

    /// Multiplication.
    Multiply,

    /// Integer division, with language-defined rounding semantics supplied
    /// by the caller before constant folding is requested.
    Divide,

    /// Minimum of two compatible quantities.
    Minimum,

    /// Maximum of two compatible quantities.
    Maximum,
}

impl SymbolicOperator {
    /// Returns a stable textual name for this operator.
    #[must_use]
    pub const fn canonical_name(self) -> &'static str {
        match self {
            Self::Add => "add",
            Self::Subtract => "subtract",
            Self::Multiply => "multiply",
            Self::Divide => "divide",
            Self::Minimum => "minimum",
            Self::Maximum => "maximum",
        }
    }

    /// Returns whether operand order may be canonicalized.
    ///
    /// This method describes mathematical operand symmetry. Callers must
    /// still preserve source evaluation order separately if the language
    /// permits effects in either operand.
    #[must_use]
    pub const fn is_commutative(self) -> bool {
        matches!(
            self,
            Self::Add | Self::Multiply | Self::Minimum | Self::Maximum
        )
    }
}

impl fmt::Display for SymbolicOperator {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.canonical_name())
    }
}

/// A symbolic resource expression.
///
/// `AstExpression` references the canonical AST instead of copying or
/// reimplementing its expression syntax. Other variants are semantic
/// expressions derived from already validated inputs.
///
/// Constants use the resource model's exact non-negative decimal type.
/// Compound operations are not assumed to be integral or dimensionally
/// valid merely because they are structurally representable.
///
/// Constructors preserve operands. Use [`normalize`] to request safe,
/// budgeted structural normalization.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum SymbolicResourceExpr {
    /// An exact, non-negative decimal constant.
    Constant(ExactQuantity),

    /// A reference to an existing canonical AST expression.
    AstExpression(NodeId),

    /// A binary symbolic operation.
    Binary {
        /// The operation to apply.
        operator: SymbolicOperator,

        /// The left operand.
        left: Box<SymbolicResourceExpr>,

        /// The right operand.
        right: Box<SymbolicResourceExpr>,
    },
}

impl SymbolicResourceExpr {
    /// Constructs an exact non-negative decimal constant.
    ///
    /// The supplied text is validated and canonicalized by the existing
    /// semantic resource model.
    pub fn constant(value: impl AsRef<str>) -> Result<Self, SymbolicError> {
        ExactQuantity::new(value)
            .map(Self::Constant)
            .map_err(|error| SymbolicError::InvalidConstant(error.to_string()))
    }

    /// Constructs an expression referring to an existing AST node.
    #[must_use]
    pub const fn ast_expression(node: NodeId) -> Self {
        Self::AstExpression(node)
    }

    /// Constructs a binary expression without evaluating its operands.
    ///
    /// This constructor is intentionally lossless. It does not reorder
    /// operands, erase operations, or infer dimensional compatibility.
    #[must_use]
    pub fn binary(
        operator: SymbolicOperator,
        left: Self,
        right: Self,
    ) -> Self {
        Self::Binary {
            operator,
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    /// Constructs an addition expression.
    #[must_use]
    pub fn add(left: Self, right: Self) -> Self {
        Self::binary(SymbolicOperator::Add, left, right)
    }

    /// Constructs a subtraction expression.
    #[must_use]
    pub fn subtract(left: Self, right: Self) -> Self {
        Self::binary(SymbolicOperator::Subtract, left, right)
    }

    /// Constructs a multiplication expression.
    #[must_use]
    pub fn multiply(left: Self, right: Self) -> Self {
        Self::binary(SymbolicOperator::Multiply, left, right)
    }

    /// Constructs a division expression.
    #[must_use]
    pub fn divide(left: Self, right: Self) -> Self {
        Self::binary(SymbolicOperator::Divide, left, right)
    }

    /// Constructs a minimum expression.
    #[must_use]
    pub fn minimum(left: Self, right: Self) -> Self {
        Self::binary(SymbolicOperator::Minimum, left, right)
    }

    /// Constructs a maximum expression.
    #[must_use]
    pub fn maximum(left: Self, right: Self) -> Self {
        Self::binary(SymbolicOperator::Maximum, left, right)
    }

    /// Returns the AST expression identity if this expression is a direct
    /// reference to one source expression.
    ///
    /// A compound expression deliberately has no fabricated AST identity.
    #[must_use]
    pub const fn ast_node_id(&self) -> Option<NodeId> {
        match self {
            Self::AstExpression(node) => Some(*node),
            Self::Constant(_) | Self::Binary { .. } => None,
        }
    }

    /// Returns whether the expression is a known constant.
    #[must_use]
    pub const fn is_constant(&self) -> bool {
        matches!(self, Self::Constant(_))
    }

    /// Returns the exact constant, if this expression is constant.
    #[must_use]
    pub fn constant_value(&self) -> Option<&ExactQuantity> {
        match self {
            Self::Constant(value) => Some(value),
            Self::AstExpression(_) | Self::Binary { .. } => None,
        }
    }

    /// Returns the number of expression nodes in the tree.
    ///
    /// This is a structural metric, not a limit on the number of resources
    /// described by the program. Use it only when the expression tree is
    /// already known to be safe to traverse.
    #[must_use]
    pub fn node_count(&self) -> usize {
        let mut count = 0_usize;
        let mut pending = vec![self];

        while let Some(expression) = pending.pop() {
            count = count.saturating_add(1);

            if let Self::Binary { left, right, .. } = expression {
                pending.push(right);
                pending.push(left);
            }
        }

        count
    }

    /// Returns a stable structural key.
    ///
    /// This key is suitable for deterministic ordering, diagnostics,
    /// snapshots, and structural deduplication. It is not a cryptographic
    /// identity and does not prove general mathematical equivalence.
    #[must_use]
    pub fn canonical_key(&self) -> String {
        match self {
            Self::Constant(value) => format!("constant({})", value.as_str()),
            Self::AstExpression(node) => {
                format!("ast({node:?})")
            }
            Self::Binary {
                operator,
                left,
                right,
            } => format!(
                "{}({},{})",
                operator.canonical_name(),
                left.canonical_key(),
                right.canonical_key()
            ),
        }
    }

    /// Converts this expression to the canonical resource quantity model
    /// when its representation is directly supported by that model.
    ///
    /// A compound expression is not converted into a guessed constant or
    /// assigned an invented `NodeId`. The semantic expression service must
    /// materialize or otherwise represent it using its canonical interface.
    pub fn to_resource_quantity(&self) -> Result<ResourceQuantity, SymbolicError> {
        match self {
            Self::Constant(value) => {
                Ok(ResourceQuantity::Known(value.clone()))
            }
            Self::AstExpression(node) => {
                Ok(ResourceQuantity::Symbolic(*node))
            }
            Self::Binary { .. } => {
                Err(SymbolicError::CompoundExpressionNeedsCanonicalIdentity)
            }
        }
    }
}

impl fmt::Display for SymbolicResourceExpr {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.canonical_key())
    }
}

/// A configurable work budget for symbolic normalization.
///
/// `None` means that this module does not impose an additional node-work
/// limit. It does not guarantee infinite execution time or memory.
///
/// The budget counts visited expression nodes. Exceeding it returns an
/// explicit error instead of returning an incomplete result as if it were
/// fully normalized.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NormalizationBudget {
    max_nodes: Option<usize>,
}

impl NormalizationBudget {
    /// Creates a budget with an optional maximum number of visited nodes.
    #[must_use]
    pub const fn new(max_nodes: Option<usize>) -> Self {
        Self { max_nodes }
    }

    /// Creates an unlimited additional node-work budget.
    ///
    /// Actual execution remains bounded by the process and environment.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self { max_nodes: None }
    }

    /// Returns the configured maximum, if one exists.
    #[must_use]
    pub const fn max_nodes(self) -> Option<usize> {
        self.max_nodes
    }
}

/// Result of symbolic normalization.
///
/// The result includes the normalized expression and the number of source
/// expression nodes visited. The counter is checked and cannot wrap.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NormalizationResult {
    expression: SymbolicResourceExpr,
    visited_nodes: usize,
}

impl NormalizationResult {
    /// Returns the normalized expression.
    #[must_use]
    pub fn expression(&self) -> &SymbolicResourceExpr {
        &self.expression
    }

    /// Consumes the result and returns the normalized expression.
    #[must_use]
    pub fn into_expression(self) -> SymbolicResourceExpr {
        self.expression
    }

    /// Returns the number of expression nodes visited.
    #[must_use]
    pub const fn visited_nodes(&self) -> usize {
        self.visited_nodes
    }
}

/// Normalizes an expression using safe, deterministic transformations.
///
/// The implementation uses an explicit work stack for tree traversal, so
/// visiting a deeply nested input does not itself require recursive Rust
/// calls. It preserves the expression's meaning and only folds exact integer
/// constants or applies identities that are safe for non-negative resource
/// arithmetic.
///
/// Subtraction that would yield a negative resource quantity is rejected.
/// Division by zero is rejected. Non-integral constants and operations whose
/// exact semantics cannot be established are preserved symbolically.
///
/// The caller is responsible for type checking, units, effect semantics, and
/// whether the source language permits a given operation.
pub fn normalize(
    expression: SymbolicResourceExpr,
    budget: NormalizationBudget,
) -> Result<NormalizationResult, SymbolicError> {
    enum Work {
        Visit(SymbolicResourceExpr),
        Build(SymbolicOperator),
    }

    let mut work = vec![Work::Visit(expression)];
    let mut results: Vec<SymbolicResourceExpr> = Vec::new();
    let mut visited_nodes = 0_usize;

    while let Some(item) = work.pop() {
        match item {
            Work::Visit(expression) => {
                visited_nodes = visited_nodes
                    .checked_add(1)
                    .ok_or(SymbolicError::WorkCounterOverflow)?;

                if let Some(limit) = budget.max_nodes {
                    if visited_nodes > limit {
                        return Err(SymbolicError::WorkBudgetExceeded {
                            budget: limit,
                            visited: visited_nodes,
                        });
                    }
                }

                match expression {
                    SymbolicResourceExpr::Constant(_)
                    | SymbolicResourceExpr::AstExpression(_) => {
                        results.push(expression);
                    }
                    SymbolicResourceExpr::Binary {
                        operator,
                        left,
                        right,
                    } => {
                        work.push(Work::Build(operator));
                        work.push(Work::Visit(*right));
                        work.push(Work::Visit(*left));
                    }
                }
            }
            Work::Build(operator) => {
                let right = results
                    .pop()
                    .ok_or(SymbolicError::InvalidExpressionStack)?;
                let left = results
                    .pop()
                    .ok_or(SymbolicError::InvalidExpressionStack)?;

                results.push(normalize_binary(operator, left, right)?);
            }
        }
    }

    if results.len() != 1 {
        return Err(SymbolicError::InvalidExpressionStack);
    }

    let expression = results
        .pop()
        .ok_or(SymbolicError::InvalidExpressionStack)?;

    Ok(NormalizationResult {
        expression,
        visited_nodes,
    })
}

fn normalize_binary(
    operator: SymbolicOperator,
    left: SymbolicResourceExpr,
    right: SymbolicResourceExpr,
) -> Result<SymbolicResourceExpr, SymbolicError> {
    // Canonicalize commutative operations by their stable structural keys.
    // This is structural normalization only. It does not authorize reordering
    // of source evaluation with effects; the caller must validate that first.
    let (left, right) = if operator.is_commutative()
        && right.canonical_key() < left.canonical_key()
    {
        (right, left)
    } else {
        (left, right)
    };

    if left == right {
        match operator {
            SymbolicOperator::Subtract => {
                return Ok(SymbolicResourceExpr::constant("0")?);
            }
            SymbolicOperator::Minimum | SymbolicOperator::Maximum => {
                return Ok(left);
            }
            _ => {}
        }
    }

    if let (Some(a), Some(b)) =
        (left.constant_value(), right.constant_value())
    {
        if let Some(folded) = fold_constants(operator, a, b)? {
            return Ok(SymbolicResourceExpr::Constant(folded));
        }
    }

    // Safe identities for non-negative resource arithmetic.
    match operator {
        SymbolicOperator::Add => {
            if is_integer_constant(&left, "0") {
                return Ok(right);
            }
            if is_integer_constant(&right, "0") {
                return Ok(left);
            }
        }
        SymbolicOperator::Subtract => {
            if is_integer_constant(&right, "0") {
                return Ok(left);
            }
        }
        SymbolicOperator::Multiply => {
            if is_integer_constant(&left, "0")
                || is_integer_constant(&right, "0")
            {
                return SymbolicResourceExpr::constant("0");
            }
            if is_integer_constant(&left, "1") {
                return Ok(right);
            }
            if is_integer_constant(&right, "1") {
                return Ok(left);
            }
        }
        SymbolicOperator::Divide => {
            if is_integer_constant(&right, "0") {
                return Err(SymbolicError::DivisionByZero);
            }
            if is_integer_constant(&right, "1") {
                return Ok(left);
            }
        }
        SymbolicOperator::Minimum | SymbolicOperator::Maximum => {}
    }

    Ok(SymbolicResourceExpr::binary(operator, left, right))
}

fn is_integer_constant(
    expression: &SymbolicResourceExpr,
    expected: &str,
) -> bool {
    match expression {
        SymbolicResourceExpr::Constant(value) => {
            !value.as_str().contains('.')
                && value.as_str() == expected
        }
        SymbolicResourceExpr::AstExpression(_)
        | SymbolicResourceExpr::Binary { .. } => false,
    }
}

fn fold_constants(
    operator: SymbolicOperator,
    left: &ExactQuantity,
    right: &ExactQuantity,
) -> Result<Option<ExactQuantity>, SymbolicError> {
    let a = left.as_str();
    let b = right.as_str();

    // Decimal arithmetic, rounding modes, and unit conversion belong to the
    // canonical constant evaluator. This module folds only exact integers.
    if a.contains('.') || b.contains('.') {
        return Ok(None);
    }

    let result = match operator {
        SymbolicOperator::Add => decimal_add(a, b),
        SymbolicOperator::Subtract => {
            match decimal_cmp(a, b) {
                Ordering::Less => {
                    return Err(SymbolicError::NegativeResourceQuantity);
                }
                Ordering::Equal => "0".to_owned(),
                Ordering::Greater => decimal_subtract(a, b),
            }
        }
        SymbolicOperator::Multiply => decimal_multiply(a, b),
        SymbolicOperator::Divide => {
            if b == "0" {
                return Err(SymbolicError::DivisionByZero);
            }

            // Division is folded only when exact integer division is proven.
            let (quotient, remainder) = decimal_div_rem(a, b)?;
            if remainder != "0" {
                return Ok(None);
            }
            quotient
        }
        SymbolicOperator::Minimum => {
            if decimal_cmp(a, b) == Ordering::Greater {
                b.to_owned()
            } else {
                a.to_owned()
            }
        }
        SymbolicOperator::Maximum => {
            if decimal_cmp(a, b) == Ordering::Less {
                b.to_owned()
            } else {
                a.to_owned()
            }
        }
    };

    ExactQuantity::new(result)
        .map(Some)
        .map_err(|error| SymbolicError::InvalidConstant(error.to_string()))
}

/// Compares canonical non-negative integer decimal strings.
fn decimal_cmp(left: &str, right: &str) -> Ordering {
    left.len()
        .cmp(&right.len())
        .then_with(|| left.cmp(right))
}

/// Adds arbitrary-length non-negative integer decimal strings.
fn decimal_add(left: &str, right: &str) -> String {
    let a = left.as_bytes();
    let b = right.as_bytes();
    let mut i = a.len();
    let mut j = b.len();
    let mut carry = 0_u16;
    let mut output = Vec::with_capacity(a.len().max(b.len()) + 1);

    while i > 0 || j > 0 || carry > 0 {
        let da = if i > 0 {
            i -= 1;
            u16::from(a[i] - b'0')
        } else {
            0
        };

        let db = if j > 0 {
            j -= 1;
            u16::from(b[j] - b'0')
        } else {
            0
        };

        let sum = da + db + carry;
        output.push(b'0' + (sum % 10) as u8);
        carry = sum / 10;
    }

    output.reverse();
    String::from_utf8(output).unwrap_or_else(|_| "0".to_owned())
}

/// Subtracts the smaller non-negative integer decimal string from the larger.
fn decimal_subtract(left: &str, right: &str) -> String {
    debug_assert!(decimal_cmp(left, right) != Ordering::Less);

    let a = left.as_bytes();
    let b = right.as_bytes();
    let mut i = a.len();
    let mut j = b.len();
    let mut borrow = 0_i16;
    let mut output = Vec::with_capacity(a.len());

    while i > 0 {
        i -= 1;

        let da = i16::from(a[i] - b'0');
        let db = if j > 0 {
            j -= 1;
            i16::from(b[j] - b'0')
        } else {
            0
        };

        let mut digit = da - db - borrow;
        if digit < 0 {
            digit += 10;
            borrow = 1;
        } else {
            borrow = 0;
        }

        output.push(b'0' + digit as u8);
    }

    while output.len() > 1 && output.last() == Some(&b'0') {
        output.pop();
    }

    output.reverse();
    String::from_utf8(output).unwrap_or_else(|_| "0".to_owned())
}

/// Multiplies arbitrary-length non-negative integer decimal strings.
fn decimal_multiply(left: &str, right: &str) -> String {
    if left == "0" || right == "0" {
        return "0".to_owned();
    }

    let a: Vec<u32> = left.bytes().rev().map(|b| u32::from(b - b'0')).collect();
    let b: Vec<u32> = right.bytes().rev().map(|b| u32::from(b - b'0')).collect();

    let mut digits = vec![0_u32; a.len() + b.len()];

    for (i, da) in a.iter().copied().enumerate() {
        for (j, db) in b.iter().copied().enumerate() {
            let index = i + j;
            digits[index] += da * db;
        }
    }

    for index in 0..digits.len().saturating_sub(1) {
        let carry = digits[index] / 10;
        digits[index] %= 10;
        digits[index + 1] += carry;
    }

    while digits.last() == Some(&0) {
        digits.pop();
    }

    digits
        .iter()
        .rev()
        .map(|digit| char::from(b'0' + *digit as u8))
        .collect()
}

/// Divides arbitrary-length non-negative integers using decimal long division.
///
/// Returns `(quotient, remainder)`. Division by zero is an error.
fn decimal_div_rem(
    dividend: &str,
    divisor: &str,
) -> Result<(String, String), SymbolicError> {
    if divisor == "0" {
        return Err(SymbolicError::DivisionByZero);
    }

    let mut quotient = String::with_capacity(dividend.len());
    let mut remainder = "0".to_owned();

    for digit in dividend.bytes() {
        remainder.push(char::from(digit));

        // Restore canonical decimal form after appending the next digit.
        remainder = canonical_integer(&remainder);

        let mut quotient_digit = 0_u8;
        while decimal_cmp(&remainder, divisor) != Ordering::Less {
            remainder = decimal_subtract(&remainder, divisor);
            quotient_digit += 1;
        }

        quotient.push(char::from(b'0' + quotient_digit));
    }

    Ok((canonical_integer(&quotient), canonical_integer(&remainder)))
}

fn canonical_integer(value: &str) -> String {
    let stripped = value.trim_start_matches('0');
    if stripped.is_empty() {
        "0".to_owned()
    } else {
        stripped.to_owned()
    }
}

/// Errors produced by symbolic resource expression construction and analysis.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum SymbolicError {
    /// The supplied constant is invalid.
    InvalidConstant(String),

    /// A resource subtraction would produce a negative value.
    NegativeResourceQuantity,

    /// An expression attempts division by zero.
    DivisionByZero,

    /// A compound expression has no corresponding canonical AST identity.
    CompoundExpressionNeedsCanonicalIdentity,

    /// The normalization work counter could not be represented.
    WorkCounterOverflow,

    /// The configured normalization budget was exceeded.
    WorkBudgetExceeded {
        /// The configured number of permitted visited nodes.
        budget: usize,

        /// The number of nodes visited when the budget was exceeded.
        visited: usize,
    },

    /// The internal expression work stack did not contain the expected nodes.
    InvalidExpressionStack,
}

impl fmt::Display for SymbolicError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidConstant(message) => {
                write!(formatter, "invalid symbolic resource constant: {message}")
            }
            Self::NegativeResourceQuantity => {
                formatter.write_str("resource quantity cannot be negative")
            }
            Self::DivisionByZero => {
                formatter.write_str("symbolic resource expression divides by zero")
            }
            Self::CompoundExpressionNeedsCanonicalIdentity => {
                formatter.write_str(
                    "compound symbolic expression requires a canonical semantic-expression identity",
                )
            }
            Self::WorkCounterOverflow => {
                formatter.write_str("symbolic normalization work counter overflowed")
            }
            Self::WorkBudgetExceeded { budget, visited } => write!(
                formatter,
                "symbolic normalization work budget exceeded: budget {budget}, visited {visited}"
            ),
            Self::InvalidExpressionStack => {
                formatter.write_str("invalid symbolic expression work stack")
            }
        }
    }
}

impl Error for SymbolicError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonicalizes_exact_integer_constants() {
        let value = SymbolicResourceExpr::constant("000001200").unwrap();
        assert_eq!(value.constant_value().unwrap().as_str(), "1200");
    }

    #[test]
    fn folds_arbitrary_length_integer_addition() {
        let left = SymbolicResourceExpr::constant(
            "999999999999999999999999999999999999",
        )
        .unwrap();
        let right = SymbolicResourceExpr::constant("1").unwrap();

        let result = normalize(
            SymbolicResourceExpr::add(left, right),
            NormalizationBudget::unlimited(),
        )
        .unwrap();

        assert_eq!(
            result.expression().constant_value().unwrap().as_str(),
            "1000000000000000000000000000000000000"
        );
    }

    #[test]
    fn folds_arbitrary_length_integer_multiplication() {
        let left = SymbolicResourceExpr::constant("999999999999999999").unwrap();
        let right = SymbolicResourceExpr::constant("999999999999999999").unwrap();

        let result = normalize(
            SymbolicResourceExpr::multiply(left, right),
            NormalizationBudget::unlimited(),
        )
        .unwrap();

        assert_eq!(
            result.expression().constant_value().unwrap().as_str(),
            "999999999999999998000000000000000001"
        );
    }

    #[test]
    fn folds_exact_integer_division() {
        let left = SymbolicResourceExpr::constant("100").unwrap();
        let right = SymbolicResourceExpr::constant("4").unwrap();

        let result = normalize(
            SymbolicResourceExpr::divide(left, right),
            NormalizationBudget::unlimited(),
        )
        .unwrap();

        assert_eq!(
            result.expression().constant_value().unwrap().as_str(),
            "25"
        );
    }

    #[test]
    fn preserves_non_integral_division_symbolically() {
        let left = SymbolicResourceExpr::constant("5").unwrap();
        let right = SymbolicResourceExpr::constant("2").unwrap();

        let result = normalize(
            SymbolicResourceExpr::divide(left, right),
            NormalizationBudget::unlimited(),
        )
        .unwrap();

        assert!(matches!(
            result.expression(),
            SymbolicResourceExpr::Binary {
                operator: SymbolicOperator::Divide,
                ..
            }
        ));
    }

    #[test]
    fn rejects_negative_resource_subtraction() {
        let left = SymbolicResourceExpr::constant("2").unwrap();
        let right = SymbolicResourceExpr::constant("3").unwrap();

        let result = normalize(
            SymbolicResourceExpr::subtract(left, right),
            NormalizationBudget::unlimited(),
        );

        assert_eq!(result, Err(SymbolicError::NegativeResourceQuantity));
    }

    #[test]
    fn rejects_division_by_zero() {
        let left = SymbolicResourceExpr::constant("12").unwrap();
        let right = SymbolicResourceExpr::constant("0").unwrap();

        let result = normalize(
            SymbolicResourceExpr::divide(left, right),
            NormalizationBudget::unlimited(),
        );

        assert_eq!(result, Err(SymbolicError::DivisionByZero));
    }

    #[test]
    fn applies_safe_zero_and_one_identities() {
        let expression = SymbolicResourceExpr::AstExpression(
            crate::frontend::ast::node::node_id::NodeId::new(1).unwrap(),
        );

        let multiplied = normalize(
            SymbolicResourceExpr::multiply(
                expression.clone(),
                SymbolicResourceExpr::constant("1").unwrap(),
            ),
            NormalizationBudget::unlimited(),
        )
        .unwrap();

        assert_eq!(multiplied.expression(), &expression);

        let added = normalize(
            SymbolicResourceExpr::add(
                expression.clone(),
                SymbolicResourceExpr::constant("0").unwrap(),
            ),
            NormalizationBudget::unlimited(),
        )
        .unwrap();

        assert_eq!(added.expression(), &expression);
    }

    #[test]
    fn canonicalizes_commutative_operand_order() {
        let a = SymbolicResourceExpr::constant("7").unwrap();
        let b = SymbolicResourceExpr::constant("11").unwrap();

        let first = normalize(
            SymbolicResourceExpr::add(a.clone(), b.clone()),
            NormalizationBudget::unlimited(),
        )
        .unwrap();

        let second = normalize(
            SymbolicResourceExpr::add(b, a),
            NormalizationBudget::unlimited(),
        )
        .unwrap();

        assert_eq!(first.expression(), second.expression());
        assert_eq!(
            first.expression().canonical_key(),
            second.expression().canonical_key()
        );
    }

    #[test]
    fn reports_budget_exhaustion_explicitly() {
        let expression = SymbolicResourceExpr::add(
            SymbolicResourceExpr::constant("1").unwrap(),
            SymbolicResourceExpr::constant("2").unwrap(),
        );

        let result = normalize(expression, NormalizationBudget::new(Some(2)));

        assert!(matches!(
            result,
            Err(SymbolicError::WorkBudgetExceeded {
                budget: 2,
                visited: 3
            })
        ));
    }

    #[test]
    fn preserves_ast_identity_when_conversion_is_supported() {
        let node = crate::frontend::ast::node::node_id::NodeId::new(7).unwrap();
        let expression = SymbolicResourceExpr::ast_expression(node);

        let quantity = expression.to_resource_quantity().unwrap();

        assert_eq!(quantity.expression_id(), Some(node));
    }

    #[test]
    fn refuses_to_fabricate_identity_for_compound_expressions() {
        let expression = SymbolicResourceExpr::add(
            SymbolicResourceExpr::constant("1").unwrap(),
            SymbolicResourceExpr::constant("2").unwrap(),
        );

        assert_eq!(
            expression.to_resource_quantity(),
            Err(SymbolicError::CompoundExpressionNeedsCanonicalIdentity)
        );
    }

    #[test]
    fn decimal_constants_are_preserved_without_unsafe_folding() {
        let left = SymbolicResourceExpr::constant("1.25").unwrap();
        let right = SymbolicResourceExpr::constant("2.5").unwrap();

        let result = normalize(
            SymbolicResourceExpr::add(left, right),
            NormalizationBudget::unlimited(),
        )
        .unwrap();

        assert!(matches!(
            result.expression(),
            SymbolicResourceExpr::Binary {
                operator: SymbolicOperator::Add,
                ..
            }
        ));
    }

    #[test]
    fn exact_minimum_and_maximum_are_folded() {
        let minimum = normalize(
            SymbolicResourceExpr::minimum(
                SymbolicResourceExpr::constant("12").unwrap(),
                SymbolicResourceExpr::constant("3").unwrap(),
            ),
            NormalizationBudget::unlimited(),
        )
        .unwrap();

        assert_eq!(
            minimum.expression().constant_value().unwrap().as_str(),
            "3"
        );

        let maximum = normalize(
            SymbolicResourceExpr::maximum(
                SymbolicResourceExpr::constant("12").unwrap(),
                SymbolicResourceExpr::constant("3").unwrap(),
            ),
            NormalizationBudget::unlimited(),
        )
        .unwrap();

        assert_eq!(
            maximum.expression().constant_value().unwrap().as_str(),
            "12"
        );
    }
}
