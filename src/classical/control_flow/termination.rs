
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Classical Computing — Termination and Control-Transfer Plans
//!
//! Path: `src/classical/control_flow/termination.rs`
//! Edition: Rust 2021
//! Intended baseline: Rust 1.97 or later
//! Safety: Unsafe Rust is forbidden.
//!
//! # File contract
//!
//! Status: Proposed implementation; full repository integration and
//! compilation must pass before this file is marked verified.
//!
//! ## Purpose
//!
//! Represent semantically resolved source-level control transfers:
//! `return`, `break`, and `continue`.
//!
//! These are target-independent adapter plans. They are not AST nodes,
//! runtime instructions, CFG edges, IR terminators, or a second IR.
//!
//! ## Authority
//!
//! - `grammar/statements/returns.g4` owns return syntax.
//! - `grammar/statements/breaks.g4` owns break syntax.
//! - `grammar/statements/continues.g4` owns continue syntax.
//! - `grammar/statements/loops.g4` owns native while/for loop syntax.
//! - `grammar/statements/control-flow.g4` owns statement composition.
//! - `grammar/spec/semantics.md` and the applicable language specifications
//!   own semantic meaning.
//! - The frontend AST owns parsed statements and source spans.
//! - Semantic analysis owns scope, label resolution, return-type checking,
//!   loop-context checking, reachability, effects, and legality.
//! - `crate::classical::ids` re-exports canonical ZUIR identities.
//! - Canonical ZUIR owns CFGs, edges, terminators, and IR verification.
//!
//! ## Owns
//!
//! - `ReturnPlan`: the resolved function and optional return-value reference.
//! - `BreakPlan`: the resolved break destination and optional label identity.
//! - `ContinuePlan`: the resolved continuation destination and optional label.
//! - `TerminationPlan`: a sum type for these three control transfers.
//! - Structural accessors and decomposition methods.
//!
//! ## Does not own
//!
//! - Parsing, AST nodes, source spans, or source formatting.
//! - Name/label resolution or loop nesting analysis.
//! - Function return-type checking or expression evaluation.
//! - Exception throwing or catch/finally routing; see `exceptions.rs`.
//! - CFG creation, dominance, reachability, or IR terminator construction.
//! - Runtime execution, scheduling, cancellation, or resource discovery.
//! - Hardware-specific control flow or target selection.
//!
//! ## Semantic contract
//!
//! A `ReturnPlan` names the enclosing function and an optional already-resolved
//! value. `None` means a value-less return in the source representation; it
//! does not decide whether that return is valid for the function's result type.
//!
//! A `BreakPlan` names the resolved destination block and, when applicable,
//! the resolved label identity. A `ContinuePlan` names the resolved
//! continuation destination and optional label identity. Label spelling,
//! scope, validity, and resolution are semantic-analysis responsibilities.
//!
//! The destination references are not proof that a block exists or that the
//! transfer is legal. The canonical IR verifier must check block existence,
//! function/region membership, terminator placement, and edge legality.
//!
//! `TerminationPlan` intentionally excludes `throw`: throwing and exceptional
//! propagation belong to `exceptions.rs` and the canonical exceptional-edge
//! contract. It also excludes an `unreachable`/trap terminator, which is an IR
//! construct unless the language specification explicitly introduces a
//! corresponding source statement.
//!
//! ## Identity and scalability contract
//!
//! IDs are canonical opaque identity tokens. This module neither allocates IDs
//! nor treats them as indices, ordering keys, or hardware identifiers.
//!
//! No fixed limits are imposed on program size, loop nesting, number of
//! functions, number of control transfers, or number of labels. Actual memory,
//! compiler budgets, and identity exhaustion remain explicit operational
//! constraints handled by their owning subsystems.
//!
//! ## Integration
//!
//! - `src/classical/ids.rs`: `BlockId`, `FunctionId`, `SymbolId`, `ValueId`.
//! - `src/classical/control_flow/mod.rs`: declares and re-exports this module.
//! - `src/classical/control_flow/loops.rs`: provides loop plans whose
//!   resolved destinations are used by break/continue planning.
//! - `src/classical/control_flow/functions.rs`: owns function-level planning;
//!   this file only references the canonical `FunctionId`.
//! - `src/classical/control_flow/exceptions.rs`: remains the owner of throw
//!   and try/catch/finally planning.
//! - `src/classical/validation/`: checks transfer legality and scope.
//! - `src/classical/lowering/control_flow.rs`: translates validated plans to
//!   canonical ZUIR terminators and edges.
//! - Canonical ZUIR verification checks the final CFG and IR invariants.
//!
//! ## Tests and freeze criteria
//!
//! Unit tests cover return with/without values, labelled/unlabelled transfer
//! metadata, decomposition, enum classification, equality, and large IDs.
//! Repository integration must additionally test frontend-to-IR behavior,
//! invalid transfers, nested loops, function boundaries, labels, and
//! unreachable-code diagnostics.
//!
//! This file is not verified until the actual workspace builds and all
//! applicable tests pass. Changes to public fields or constructors require
//! compatibility review of semantic validation and lowering.

use crate::classical::ids::{BlockId, FunctionId, SymbolId, ValueId};

/// A resolved return from a specific function.
///
/// The optional value must already be resolved by the frontend/semantic
/// pipeline. This plan does not evaluate the expression or check its type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ReturnPlan {
    function: FunctionId,
    value: Option<ValueId>,
}

impl ReturnPlan {
    /// Creates a return plan for the enclosing function.
    ///
    /// `value` is `None` for a source-level value-less return and `Some`
    /// for a return with an already-resolved value.
    ///
    /// This constructor performs no semantic validation.
    #[must_use]
    pub const fn new(function: FunctionId, value: Option<ValueId>) -> Self {
        Self { function, value }
    }

    /// Returns the function from which control returns.
    #[must_use]
    pub const fn function(&self) -> FunctionId {
        self.function
    }

    /// Returns the optional resolved return value.
    #[must_use]
    pub const fn value(&self) -> Option<ValueId> {
        self.value
    }

    /// Returns whether this is a value-less return.
    #[must_use]
    pub const fn is_value_less(&self) -> bool {
        self.value.is_none()
    }

    /// Consumes the plan and returns its canonical references.
    #[must_use]
    pub const fn into_parts(self) -> (FunctionId, Option<ValueId>) {
        (self.function, self.value)
    }
}

/// A resolved `break` transfer.
///
/// `destination` is the resolved exit destination selected by semantic
/// analysis for the targeted loop or other language-supported breakable
/// construct. `label` is the resolved label identity when the source used a
/// label; it is not the original label text.
///
/// The canonical CFG verifier remains responsible for validating the edge.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BreakPlan {
    destination: BlockId,
    label: Option<SymbolId>,
}

impl BreakPlan {
    /// Creates a break plan using an already-resolved destination.
    ///
    /// Pass `None` for an unlabelled break. Pass `Some(label)` only after the
    /// source label has been resolved by semantic analysis.
    #[must_use]
    pub const fn new(
        destination: BlockId,
        label: Option<SymbolId>,
    ) -> Self {
        Self { destination, label }
    }

    /// Returns the canonical destination block.
    #[must_use]
    pub const fn destination(&self) -> BlockId {
        self.destination
    }

    /// Returns the resolved label identity, if present.
    #[must_use]
    pub const fn label(&self) -> Option<SymbolId> {
        self.label
    }

    /// Returns whether this plan represents a labelled break.
    #[must_use]
    pub const fn is_labelled(&self) -> bool {
        self.label.is_some()
    }

    /// Consumes the plan and returns its canonical references.
    #[must_use]
    pub const fn into_parts(self) -> (BlockId, Option<SymbolId>) {
        (self.destination, self.label)
    }
}

/// A resolved `continue` transfer.
///
/// `destination` is the resolved continuation point. For a simple loop this
/// may be the loop header; for an iterator loop or a transformed CFG it may be
/// a distinct condition, latch, or iterator-advance block.
///
/// The plan deliberately does not assume that every loop has the same CFG
/// shape. Canonical lowering chooses the correct destination and the IR
/// verifier checks that the edge preserves the language's loop semantics.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ContinuePlan {
    destination: BlockId,
    label: Option<SymbolId>,
}

impl ContinuePlan {
    /// Creates a continue plan using an already-resolved destination.
    ///
    /// Pass `None` for an unlabelled continue. A label, when present, must
    /// already be resolved and valid for the source construct.
    #[must_use]
    pub const fn new(
        destination: BlockId,
        label: Option<SymbolId>,
    ) -> Self {
        Self { destination, label }
    }

    /// Returns the canonical continuation destination.
    #[must_use]
    pub const fn destination(&self) -> BlockId {
        self.destination
    }

    /// Returns the resolved label identity, if present.
    #[must_use]
    pub const fn label(&self) -> Option<SymbolId> {
        self.label
    }

    /// Returns whether this plan represents a labelled continue.
    #[must_use]
    pub const fn is_labelled(&self) -> bool {
        self.label.is_some()
    }

    /// Consumes the plan and returns its canonical references.
    #[must_use]
    pub const fn into_parts(self) -> (BlockId, Option<SymbolId>) {
        (self.destination, self.label)
    }
}

/// A resolved source-level control transfer.
///
/// Exception throwing is intentionally not included: `exceptions.rs` owns
/// throw planning and exceptional control flow. IR-only terminators such as
/// unreachable/trap also remain with the canonical IR and lowering layers.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TerminationPlan {
    Return(ReturnPlan),
    Break(BreakPlan),
    Continue(ContinuePlan),
}

impl TerminationPlan {
    /// Returns `true` if this plan returns from a function.
    #[must_use]
    pub const fn is_return(&self) -> bool {
        matches!(self, Self::Return(_))
    }

    /// Returns `true` if this plan breaks out to a resolved destination.
    #[must_use]
    pub const fn is_break(&self) -> bool {
        matches!(self, Self::Break(_))
    }

    /// Returns `true` if this plan continues at a resolved destination.
    #[must_use]
    pub const fn is_continue(&self) -> bool {
        matches!(self, Self::Continue(_))
    }

    /// Returns the return plan, if this is a return transfer.
    #[must_use]
    pub const fn as_return(&self) -> Option<&ReturnPlan> {
        match self {
            Self::Return(plan) => Some(plan),
            Self::Break(_) | Self::Continue(_) => None,
        }
    }

    /// Returns the break plan, if this is a break transfer.
    #[must_use]
    pub const fn as_break(&self) -> Option<&BreakPlan> {
        match self {
            Self::Break(plan) => Some(plan),
            Self::Return(_) | Self::Continue(_) => None,
        }
    }

    /// Returns the continue plan, if this is a continue transfer.
    #[must_use]
    pub const fn as_continue(&self) -> Option<&ContinuePlan> {
        match self {
            Self::Continue(plan) => Some(plan),
            Self::Return(_) | Self::Break(_) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn block(raw: u128) -> BlockId {
        BlockId::new(raw).expect("test block ID must be nonzero")
    }

    fn function(raw: u128) -> FunctionId {
        FunctionId::new(raw).expect("test function ID must be nonzero")
    }

    fn symbol(raw: u128) -> SymbolId {
        SymbolId::new(raw).expect("test symbol ID must be nonzero")
    }

    fn value(raw: u128) -> ValueId {
        ValueId::new(raw).expect("test value ID must be nonzero")
    }

    #[test]
    fn return_with_value_preserves_function_and_value() {
        let function = function(1);
        let value = value(2);
        let plan = ReturnPlan::new(function, Some(value));

        assert_eq!(plan.function(), function);
        assert_eq!(plan.value(), Some(value));
        assert!(!plan.is_value_less());
        assert_eq!(plan.into_parts(), (function, Some(value)));
    }

    #[test]
    fn value_less_return_is_representable() {
        let function = function(1);
        let plan = ReturnPlan::new(function, None);

        assert_eq!(plan.function(), function);
        assert_eq!(plan.value(), None);
        assert!(plan.is_value_less());
    }

    #[test]
    fn unlabelled_break_preserves_destination() {
        let destination = block(3);
        let plan = BreakPlan::new(destination, None);

        assert_eq!(plan.destination(), destination);
        assert_eq!(plan.label(), None);
        assert!(!plan.is_labelled());
    }

    #[test]
    fn labelled_break_preserves_resolved_label() {
        let destination = block(3);
        let label = symbol(4);
        let plan = BreakPlan::new(destination, Some(label));

        assert_eq!(plan.destination(), destination);
        assert_eq!(plan.label(), Some(label));
        assert!(plan.is_labelled());
        assert_eq!(plan.into_parts(), (destination, Some(label)));
    }

    #[test]
    fn continue_can_target_a_distinct_latch_or_advance_block() {
        let continuation = block(8);
        let label = symbol(9);
        let plan = ContinuePlan::new(continuation, Some(label));

        assert_eq!(plan.destination(), continuation);
        assert_eq!(plan.label(), Some(label));
        assert!(plan.is_labelled());
    }

    #[test]
    fn termination_variants_are_distinguishable() {
        let returned = TerminationPlan::Return(ReturnPlan::new(
            function(1),
            Some(value(2)),
        ));
        let broken =
            TerminationPlan::Break(BreakPlan::new(block(3), None));
        let continued =
            TerminationPlan::Continue(ContinuePlan::new(block(4), None));

        assert!(returned.is_return());
        assert!(!returned.is_break());
        assert!(!returned.is_continue());
        assert!(returned.as_return().is_some());
        assert!(returned.as_break().is_none());

        assert!(broken.is_break());
        assert!(broken.as_break().is_some());
        assert!(broken.as_return().is_none());

        assert!(continued.is_continue());
        assert!(continued.as_continue().is_some());
        assert!(continued.as_break().is_none());
    }

    #[test]
    fn plans_support_equality_and_copy() {
        let original = ReturnPlan::new(function(1), Some(value(2)));
        let copied = original;

        assert_eq!(original, copied);
        assert_eq!(original, original);
    }

    #[test]
    fn large_identifiers_are_not_truncated() {
        let large_function = function(u128::MAX);
        let large_value = value(u128::MAX - 1);
        let large_block = block(u128::MAX - 2);
        let large_label = symbol(u128::MAX - 3);

        let returned =
            ReturnPlan::new(large_function, Some(large_value));
        let broken =
            BreakPlan::new(large_block, Some(large_label));
        let continued =
            ContinuePlan::new(large_block, Some(large_label));

        assert_eq!(returned.function(), large_function);
        assert_eq!(returned.value(), Some(large_value));
        assert_eq!(broken.destination(), large_block);
        assert_eq!(broken.label(), Some(large_label));
        assert_eq!(continued.destination(), large_block);
    }

    #[test]
    fn termination_plan_keeps_its_payload() {
        let original = TerminationPlan::Continue(ContinuePlan::new(
            block(10),
            Some(symbol(11)),
        ));

        let copied = original;
        assert_eq!(original, copied);
        assert!(matches!(original, TerminationPlan::Continue(_)));
    }
}
