
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # HDLIR symbolic bit widths
//!
//! Path: `src/hdl/hdlir/types/bit_width.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//! Status: Foundational HDLIR type contract; freeze after integration tests.
//!
//! ## Purpose
//!
//! Represent concrete and symbolic hardware bit widths without imposing a
//! universal hardware-width ceiling or forcing symbolic expressions into a
//! host-sized integer.
//!
//! ## Authority
//!
//! - `grammar/spec/hdl.md`
//! - `grammar/specification/semantic-model.md`
//! - `src/frontend/semantic/domains/hdl.rs`
//! - `src/hdl/api.rs`
//! - `src/hdl/hdlir/HDLIR-SPEC.md` (to be established)
//!
//! The existing semantic boundary owns source-level width meaning.
//! This module owns the HDLIR representation and checked evaluation of widths
//! after the appropriate semantic facts and parameter bindings are supplied.
//!
//! ## Owns
//!
//! - Positive concrete widths representable by `u128`.
//! - Named symbolic and unresolved width expressions.
//! - Addition and multiplication of widths.
//! - Checked, budget-aware expression evaluation.
//! - Width-specific evaluation errors.
//!
//! ## Does not own
//!
//! - Source parsing, name resolution, or semantic type inference.
//! - Source spans, canonical diagnostics, or semantic identity allocation.
//! - Physical register allocation, bus selection, pin assignment, synthesis,
//!   placement, routing, or target capability discovery.
//! - A competing source-level semantic model.
//! - A universal maximum hardware width.
//!
//! ## Scalability contract
//!
//! `u128` is the concrete integer representation used by the existing HDL
//! semantic API. It is not a claim that hardware widths are universally
//! limited to that range. Expressions whose values are not yet representable
//! remain symbolic; concrete evaluation reports overflow explicitly.
//!
//! Evaluation is iterative rather than recursively traversing the expression
//! tree, avoiding call-stack growth proportional to expression depth.
//! The caller supplies an operational node budget. Exhaustion is an explicit
//! error, not successful partial evaluation.
//!
//! No implementation can process literally infinite input. Practical work is
//! bounded by available memory, representable values, and caller-supplied
//! operational budgets.
//!
//! ## Integration contract
//!
//! - `types/mod.rs` should export `BitWidth`, `WidthSymbol`,
//!   `WidthEvaluationBudget`, and `WidthEvaluationError`.
//! - `types/mod.rs` must not duplicate this representation.
//! - `values/expressions.rs` may construct width expressions, but this module
//!   remains the owner of width-specific arithmetic and evaluation.
//! - `values/parameters.rs` or the lowering layer supplies resolved bindings
//!   through the evaluator's resolver callback.
//! - `lowering/from_semantic.rs` maps the existing semantic `HdlWidth`
//!   variants into this representation, preserving symbol names and expression
//!   structure. That adapter must be implemented against the actual semantic
//!   API; this file deliberately does not assume an unverified adapter API.
//! - `verify/types.rs` consumes evaluation results and explicit errors.
//! - `analysis/widths.rs` decides when evaluation is appropriate and retains
//!   symbolic constraints when bindings are unavailable.
//! - Serialization must encode each variant distinctly and must not silently
//!   evaluate or truncate symbolic expressions.
//!
//! ## Invariants
//!
//! - Concrete widths are strictly positive.
//! - Symbol names are non-empty after trimming.
//! - Arithmetic never wraps.
//! - An absent binding is not treated as a successful evaluation.
//! - Expression structure is preserved unless a separately specified
//!   transformation proves a rewrite valid.
//! - Budget exhaustion never returns a partial width as a complete result.
//!
//! ## Freeze criteria
//!
//! - Public constructors reject invalid concrete widths and symbols.
//! - Boundary arithmetic, unresolved names, overflow, and budget exhaustion
//!   have deterministic tests.
//! - Semantic-to-HDLIR mapping preserves all five existing `HdlWidth` cases.
//! - The module compiles under the repository's Rust 1.97+ toolchain.
//! - Formatting, focused tests, full repository tests, and Clippy pass.

use core::fmt;
use core::num::NonZeroU128;

/// A validated symbolic name used to identify a width parameter or expression.
///
/// This is a name in the width-expression namespace, not a physical signal ID,
/// source AST node ID, or hardware resource identifier.
///
/// Name resolution and namespace qualification remain the responsibility of
/// the semantic or lowering layer.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct WidthSymbol(String);

impl WidthSymbol {
    /// Construct a symbol from a non-empty name.
    ///
    /// The original spelling is retained. This constructor does not perform
    /// case folding, Unicode normalization, or semantic name resolution.
    pub fn new(name: impl Into<String>) -> Result<Self, WidthEvaluationError> {
        let name = name.into();

        if name.trim().is_empty() {
            return Err(WidthEvaluationError::EmptySymbol);
        }

        Ok(Self(name))
    }

    /// Return the retained symbol spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for WidthSymbol {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// A hardware bit-width expression.
///
/// Concrete widths are positive `u128` values, matching the current public
/// HDL semantic contract. Symbolic expressions are preserved until the
/// caller has suitable bindings and requests checked evaluation.
///
/// `Add` and `Multiply` describe width arithmetic, not hardware operations.
/// This type does not decide whether a width expression is legal in every
/// HDL context; contextual legality belongs to semantic validation and IR
/// verification.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum BitWidth {
    /// A positive concrete bit width.
    Known(NonZeroU128),

    /// A named width parameter or symbolic width expression.
    Symbol(WidthSymbol),

    /// The sum of two width expressions.
    Add(Box<BitWidth>, Box<BitWidth>),

    /// The product of two width expressions.
    Multiply(Box<BitWidth>, Box<BitWidth>),

    /// A width whose defining expression or binding is not yet available.
    ///
    /// This differs from `Symbol`: it explicitly records that the producer
    /// could not resolve the defining expression at the time it constructed
    /// the HDLIR value. A resolver may still provide a binding later.
    Unresolved(WidthSymbol),
}

impl BitWidth {
    /// Construct a positive concrete width.
    ///
    /// Zero is rejected because this API models a bit width, not a general
    /// integer or array length. If a language construct permits a zero-sized
    /// object, its semantic contract must represent that case separately.
    pub fn known(value: u128) -> Result<Self, WidthEvaluationError> {
        NonZeroU128::new(value)
            .map(Self::Known)
            .ok_or(WidthEvaluationError::ZeroWidth)
    }

    /// Construct a symbolic width expression.
    pub fn symbol(name: impl Into<String>) -> Result<Self, WidthEvaluationError> {
        WidthSymbol::new(name).map(Self::Symbol)
    }

    /// Construct an explicitly unresolved width expression.
    pub fn unresolved(name: impl Into<String>) -> Result<Self, WidthEvaluationError> {
        WidthSymbol::new(name).map(Self::Unresolved)
    }

    /// Construct a symbolic addition without evaluating it.
    ///
    /// This preserves source/semantic structure and avoids premature
    /// evaluation before parameter bindings are available.
    #[must_use]
    pub fn add(lhs: Self, rhs: Self) -> Self {
        Self::Add(Box::new(lhs), Box::new(rhs))
    }

    /// Construct a symbolic multiplication without evaluating it.
    #[must_use]
    pub fn multiply(lhs: Self, rhs: Self) -> Self {
        Self::Multiply(Box::new(lhs), Box::new(rhs))
    }

    /// Return the concrete width if this expression is already known.
    #[must_use]
    pub const fn as_known(&self) -> Option<u128> {
        match self {
            Self::Known(value) => Some(value.get()),
            Self::Symbol(_)
            | Self::Add(_, _)
            | Self::Multiply(_, _)
            | Self::Unresolved(_) => None,
        }
    }

    /// Return whether this expression is explicitly unresolved.
    #[must_use]
    pub const fn is_unresolved(&self) -> bool {
        matches!(self, Self::Unresolved(_))
    }

    /// Return whether this expression contains no symbolic or unresolved
    /// nodes.
    ///
    /// This is a structural query. It does not prove that evaluating the
    /// arithmetic will fit in `u128`.
    #[must_use]
    pub fn is_fully_concrete(&self) -> bool {
        let mut pending = vec![self];

        while let Some(width) = pending.pop() {
            match width {
                Self::Known(_) => {}
                Self::Symbol(_) | Self::Unresolved(_) => return false,
                Self::Add(lhs, rhs) | Self::Multiply(lhs, rhs) => {
                    pending.push(lhs);
                    pending.push(rhs);
                }
            }
        }

        true
    }

    /// Evaluate the expression using a caller-provided symbol resolver and
    /// the supplied operational budget.
    ///
    /// The resolver returns a concrete positive width for a recognized
    /// symbol, or `None` when the name has no available binding.
    ///
    /// Evaluation is deterministic for a deterministic resolver. The
    /// resolver is called in expression traversal order; callers must avoid
    /// relying on external side effects.
    ///
    /// The expression tree is traversed iteratively. `max_nodes` counts every
    /// visited expression node, including concrete leaves. No partial result
    /// is returned if the budget is exhausted or arithmetic overflows.
    pub fn evaluate_with<F>(
        &self,
        budget: WidthEvaluationBudget,
        mut resolve: F,
    ) -> Result<u128, WidthEvaluationError>
    where
        F: FnMut(&str) -> Option<u128>,
    {
        // The boolean indicates whether an operation node is being revisited
        // after its operands have been evaluated.
        let mut pending: Vec<(&BitWidth, bool)> = vec![(self, false)];
        let mut values: Vec<u128> = Vec::new();
        let mut visited_nodes: usize = 0;

        while let Some((node, operands_ready)) = pending.pop() {
            if !operands_ready {
                visited_nodes = visited_nodes
                    .checked_add(1)
                    .ok_or(WidthEvaluationError::NodeCountOverflow)?;

                if visited_nodes > budget.max_nodes {
                    return Err(WidthEvaluationError::BudgetExceeded {
                        limit: budget.max_nodes,
                        visited: visited_nodes,
                    });
                }

                match node {
                    Self::Known(width) => values.push(width.get()),

                    Self::Symbol(symbol) => {
                        let value = resolve(symbol.as_str()).ok_or_else(|| {
                            WidthEvaluationError::UnboundSymbol(symbol.clone())
                        })?;

                        if value == 0 {
                            return Err(WidthEvaluationError::ZeroResolvedWidth(
                                symbol.clone(),
                            ));
                        }

                        values.push(value);
                    }

                    Self::Unresolved(symbol) => {
                        let value = resolve(symbol.as_str()).ok_or_else(|| {
                            WidthEvaluationError::UnresolvedWidth(symbol.clone())
                        })?;

                        if value == 0 {
                            return Err(WidthEvaluationError::ZeroResolvedWidth(
                                symbol.clone(),
                            ));
                        }

                        values.push(value);
                    }

                    Self::Add(lhs, rhs) | Self::Multiply(lhs, rhs) => {
                        pending.push((node, true));
                        pending.push((rhs, false));
                        pending.push((lhs, false));
                    }
                }
            } else {
                let rhs = values
                    .pop()
                    .ok_or(WidthEvaluationError::InvalidEvaluationState)?;
                let lhs = values
                    .pop()
                    .ok_or(WidthEvaluationError::InvalidEvaluationState)?;

                let result = match node {
                    Self::Add(_, _) => lhs
                        .checked_add(rhs)
                        .ok_or(WidthEvaluationError::ArithmeticOverflow {
                            operation: WidthArithmetic::Add,
                            lhs,
                            rhs,
                        })?,
                    Self::Multiply(_, _) => lhs.checked_mul(rhs).ok_or(
                        WidthEvaluationError::ArithmeticOverflow {
                            operation: WidthArithmetic::Multiply,
                            lhs,
                            rhs,
                        },
                    )?,
                    Self::Known(_)
                    | Self::Symbol(_)
                    | Self::Unresolved(_) => {
                        return Err(WidthEvaluationError::InvalidEvaluationState);
                    }
                };

                // Both operands are positive, so checked addition and
                // multiplication cannot produce zero on successful evaluation.
                values.push(result);
            }
        }

        if values.len() != 1 {
            return Err(WidthEvaluationError::InvalidEvaluationState);
        }

        values
            .pop()
            .ok_or(WidthEvaluationError::InvalidEvaluationState)
    }

    /// Evaluate a fully concrete expression without resolving any symbols.
    ///
    /// Returns an explicit unresolved-symbol error when a binding is needed.
    pub fn evaluate_concrete(
        &self,
        budget: WidthEvaluationBudget,
    ) -> Result<u128, WidthEvaluationError> {
        self.evaluate_with(budget, |_| None)
    }
}

/// Caller-controlled operational budget for width-expression evaluation.
///
/// The budget limits compiler work, not the width of a legal hardware design.
/// It can be set according to the caller's resource policy. A budget of
/// `usize::MAX` imposes no practical node-count limit beyond representability
/// and available resources.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WidthEvaluationBudget {
    /// Maximum number of expression nodes that may be evaluated.
    pub max_nodes: usize,
}

impl WidthEvaluationBudget {
    /// Create an explicit node-count budget.
    #[must_use]
    pub const fn new(max_nodes: usize) -> Self {
        Self { max_nodes }
    }

    /// Create a budget limited only by the representable node count and
    /// available resources.
    ///
    /// This does not guarantee that evaluation will complete: allocation
    /// failure, arithmetic overflow, or unresolved symbols remain possible.
    #[must_use]
    pub const fn unbounded() -> Self {
        Self {
            max_nodes: usize::MAX,
        }
    }
}

impl Default for WidthEvaluationBudget {
    /// Use the representable node-count range by default.
    ///
    /// Applications handling untrusted input should select an explicit
    /// operational budget appropriate to their environment.
    fn default() -> Self {
        Self::unbounded()
    }
}

/// The arithmetic operation involved in a width-evaluation failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WidthArithmetic {
    /// Addition of two widths.
    Add,
    /// Multiplication of two widths.
    Multiply,
}

/// Errors produced while constructing or evaluating width expressions.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WidthEvaluationError {
    /// A concrete bit width was zero.
    ZeroWidth,

    /// A symbolic name was empty or contained only whitespace.
    EmptySymbol,

    /// A symbolic width had no binding in the supplied resolver.
    UnboundSymbol(WidthSymbol),

    /// An explicitly unresolved width could not be resolved.
    UnresolvedWidth(WidthSymbol),

    /// A resolver supplied zero for a width binding.
    ZeroResolvedWidth(WidthSymbol),

    /// A concrete arithmetic operation exceeded the `u128` range.
    ArithmeticOverflow {
        /// Arithmetic operation that overflowed.
        operation: WidthArithmetic,
        /// Left operand.
        lhs: u128,
        /// Right operand.
        rhs: u128,
    },

    /// Evaluation exceeded its caller-provided node budget.
    BudgetExceeded {
        /// Configured maximum number of visited nodes.
        limit: usize,
        /// Number of nodes reached when exhaustion was detected.
        visited: usize,
    },

    /// The evaluator's internal value-stack invariant was violated.
    ///
    /// This indicates an implementation defect, not invalid user source.
    InvalidEvaluationState,

    /// The evaluated-node counter exceeded its representable range.
    NodeCountOverflow,
}

impl fmt::Display for WidthEvaluationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ZeroWidth => {
                formatter.write_str("HDL bit width must be greater than zero")
            }
            Self::EmptySymbol => {
                formatter.write_str("HDL width symbol cannot be empty")
            }
            Self::UnboundSymbol(symbol) => {
                write!(formatter, "no binding is available for width symbol `{symbol}`")
            }
            Self::UnresolvedWidth(symbol) => {
                write!(formatter, "HDL width `{symbol}` remains unresolved")
            }
            Self::ZeroResolvedWidth(symbol) => {
                write!(formatter, "width binding `{symbol}` resolved to zero")
            }
            Self::ArithmeticOverflow {
                operation,
                lhs,
                rhs,
            } => {
                let operator = match operation {
                    WidthArithmetic::Add => "+",
                    WidthArithmetic::Multiply => "*",
                };
                write!(
                    formatter,
                    "HDL width arithmetic overflow: {lhs} {operator} {rhs}"
                )
            }
            Self::BudgetExceeded { limit, visited } => {
                write!(
                    formatter,
                    "HDL width evaluation budget exceeded: limit {limit}, visited {visited}"
                )
            }
            Self::InvalidEvaluationState => {
                formatter.write_str("internal HDL width evaluation invariant failed")
            }
            Self::NodeCountOverflow => {
                formatter.write_str("HDL width evaluation node counter overflowed")
            }
        }
    }
}

impl std::error::Error for WidthEvaluationError {}

#[cfg(test)]
mod tests {
    use super::{
        BitWidth, WidthArithmetic, WidthEvaluationBudget, WidthEvaluationError, WidthSymbol,
    };

    fn budget() -> WidthEvaluationBudget {
        WidthEvaluationBudget::unbounded()
    }

    #[test]
    fn concrete_width_must_be_positive() {
        assert!(matches!(
            BitWidth::known(0),
            Err(WidthEvaluationError::ZeroWidth)
        ));
        assert_eq!(BitWidth::known(1).unwrap().as_known(), Some(1));
    }

    #[test]
    fn symbols_reject_empty_names_and_preserve_spelling() {
        assert!(matches!(
            WidthSymbol::new(" \t "),
            Err(WidthEvaluationError::EmptySymbol)
        ));

        let symbol = WidthSymbol::new("BusWidth").unwrap();
        assert_eq!(symbol.as_str(), "BusWidth");
    }

    #[test]
    fn concrete_addition_and_multiplication_are_checked() {
        let sum = BitWidth::add(
            BitWidth::known(7).unwrap(),
            BitWidth::known(9).unwrap(),
        );
        assert_eq!(sum.evaluate_concrete(budget()), Ok(16));

        let product = BitWidth::multiply(
            BitWidth::known(7).unwrap(),
            BitWidth::known(9).unwrap(),
        );
        assert_eq!(product.evaluate_concrete(budget()), Ok(63));
    }

    #[test]
    fn symbolic_widths_resolve_without_changing_the_expression() {
        let width = BitWidth::add(
            BitWidth::symbol("DATA_WIDTH").unwrap(),
            BitWidth::known(1).unwrap(),
        );

        let original = width.clone();
        let result = width.evaluate_with(budget(), |name| match name {
            "DATA_WIDTH" => Some(31),
            _ => None,
        });

        assert_eq!(result, Ok(32));
        assert_eq!(width, original);
        assert!(!width.is_fully_concrete());
    }

    #[test]
    fn unresolved_widths_can_be_resolved_later() {
        let width = BitWidth::unresolved("PARAMETER_N").unwrap();

        assert!(matches!(
            width.evaluate_concrete(budget()),
            Err(WidthEvaluationError::UnresolvedWidth(_))
        ));

        assert_eq!(
            width.evaluate_with(budget(), |name| {
                (name == "PARAMETER_N").then_some(128)
            }),
            Ok(128)
        );
    }

    #[test]
    fn unbound_symbol_is_not_silent_success() {
        let width = BitWidth::symbol("MISSING").unwrap();

        assert!(matches!(
            width.evaluate_concrete(budget()),
            Err(WidthEvaluationError::UnboundSymbol(_))
        ));
    }

    #[test]
    fn zero_resolved_width_is_rejected() {
        let width = BitWidth::symbol("N").unwrap();

        assert!(matches!(
            width.evaluate_with(budget(), |_| Some(0)),
            Err(WidthEvaluationError::ZeroResolvedWidth(_))
        ));
    }

    #[test]
    fn addition_overflow_is_reported() {
        let width = BitWidth::add(
            BitWidth::known(u128::MAX).unwrap(),
            BitWidth::known(1).unwrap(),
        );

        assert!(matches!(
            width.evaluate_concrete(budget()),
            Err(WidthEvaluationError::ArithmeticOverflow {
                operation: WidthArithmetic::Add,
                ..
            })
        ));
    }

    #[test]
    fn multiplication_overflow_is_reported() {
        let width = BitWidth::multiply(
            BitWidth::known(u128::MAX).unwrap(),
            BitWidth::known(2).unwrap(),
        );

        assert!(matches!(
            width.evaluate_concrete(budget()),
            Err(WidthEvaluationError::ArithmeticOverflow {
                operation: WidthArithmetic::Multiply,
                ..
            })
        ));
    }

    #[test]
    fn budget_exhaustion_is_explicit() {
        let width = BitWidth::add(
            BitWidth::known(2).unwrap(),
            BitWidth::known(3).unwrap(),
        );

        assert!(matches!(
            width.evaluate_concrete(WidthEvaluationBudget::new(2)),
            Err(WidthEvaluationError::BudgetExceeded {
                limit: 2,
                visited: 3
            })
        ));
    }

    #[test]
    fn evaluation_handles_deep_trees_iteratively() {
        let mut width = BitWidth::known(1).unwrap();

        // This constructs a deeper expression without making evaluation
        // consume one Rust call-stack frame per expression node.
        for _ in 0..10_000 {
            width = BitWidth::add(width, BitWidth::known(1).unwrap());
        }

        assert_eq!(width.evaluate_concrete(budget()), Ok(10_001));
    }

    #[test]
    fn expression_construction_does_not_evaluate_prematurely() {
        let width = BitWidth::multiply(
            BitWidth::symbol("ROWS").unwrap(),
            BitWidth::symbol("COLUMNS").unwrap(),
        );

        assert_eq!(
            width.evaluate_with(budget(), |name| match name {
                "ROWS" => Some(8),
                "COLUMNS" => Some(16),
                _ => None,
            }),
            Ok(128)
        );
    }
}
