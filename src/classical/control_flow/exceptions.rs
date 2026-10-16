
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Classical Computing — Exception Control-Flow Planning
//!
//! Path: `src/classical/control_flow/exceptions.rs`
//! Rust edition: 2021
//! Minimum intended Rust version: 1.97
//! Safety: Unsafe Rust is forbidden.
//!
//! # File contract
//!
//! Status: Proposed implementation; repository integration and compilation
//! must be verified before this file can be marked production-ready.
//!
//! ## Purpose
//!
//! Represent the resolved identities required to lower source-level throw,
//! try, catch, and finally constructs into canonical Zamani control flow.
//!
//! These structures are planning adapters. They are not an exception runtime,
//! a source AST, a semantic type system, a control-flow graph, or an IR.
//!
//! ## Authority
//!
//! - `grammar/statements/exceptions.g4` owns exception syntax.
//! - `grammar/statements/control-flow.g4` owns control-flow composition.
//! - `grammar/spec/semantics.md` owns the language's control-flow semantics.
//! - The frontend AST owns parsed source constructs and source locations.
//! - Semantic analysis owns name resolution, throwable-value validity,
//!   exception type compatibility, effects, handler reachability, and
//!   propagation analysis.
//! - `crate::zuir::ids` owns canonical IR identities.
//! - Canonical ZUIR owns the CFG, terminators, successors, and verification.
//! - `crate::classical::error` owns the classical error vocabulary.
//!
//! ## Owns
//!
//! - `ThrowPlan`: the resolved value reference associated with a throw.
//! - `CatchHandlerPlan`: a catch binding, optional resolved exception type,
//!   and handler-entry block.
//! - `TryExceptionPlan`: the try-body entry, ordered handlers, and optional
//!   finally-entry block.
//! - Local construction invariants for these adapter structures.
//!
//! ## Does not own
//!
//! - Parsing, AST definitions, or source spans.
//! - Runtime exception objects or their representation.
//! - Exception class hierarchies or type compatibility rules.
//! - Name resolution, variable binding, ownership, or effects.
//! - Exception matching, catch reachability, or exhaustive handling.
//! - CFG construction, terminators, edge allocation, or IR verification.
//! - Resource allocation, hardware selection, scheduling, or execution.
//!
//! ## Semantic contract
//!
//! A `ThrowPlan` refers to an already-resolved canonical value. It does not
//! prove that the value is throwable; semantic analysis must do that.
//!
//! A `CatchHandlerPlan` represents one source catch clause. Handler order is
//! preserved exactly as supplied because matching and catch precedence are
//! semantic concerns. `None` for `exception_type` represents an untyped catch
//! clause; it does not mean that the binding is unresolved.
//!
//! A `TryExceptionPlan` requires at least one catch handler or a finally
//! block. This mirrors the source grammar's rule that a try construct cannot
//! stand alone without catch or finally.
//!
//! The finally block must be reached according to the language's semantics
//! on all applicable exits, including exceptional exits. These data
//! structures do not themselves implement that routing. Canonical CFG
//! construction and verification must preserve pending outcomes and ensure
//! that finally behavior is correct when cleanup itself transfers control.
//!
//! This module does not reject repeated exception types or handler targets.
//! Whether a catch is unreachable, redundant, or otherwise invalid is a
//! semantic-analysis responsibility.
//!
//! ## Canonical identity contract
//!
//! All IDs are borrowed from `crate::classical::ids`, which re-exports
//! canonical ZUIR identities. IDs are opaque identity tokens, not array
//! indices, block counts, source positions, or hardware identifiers.
//!
//! ## POCO-REAF and scalability
//!
//! No fixed limit is imposed on catch-handler count, exception nesting,
//! block count, or program size. The handler collection is dynamically sized.
//!
//! This module does not promise infinite physical resources. Callers must
//! account for ordinary allocation failure and configured compiler budgets.
//! It introduces no universal machine-size or hardware-specific ceilings.
//!
//! ## Integration
//!
//! - `src/classical/ids.rs` supplies `BlockId`, `SymbolId`, `TypeId`, and
//!   `ValueId`.
//! - `src/classical/error.rs` supplies `ClassicalError` and `ClassicalResult`.
//! - `src/classical/control_flow/mod.rs` should declare this module.
//! - `src/classical/validation/` must validate exception types, binding
//!   scope, effects, handler reachability, and propagation.
//! - `src/classical/lowering/control_flow.rs` should translate validated
//!   plans into canonical ZUIR control flow.
//! - The canonical CFG/IR verifier must validate exceptional edges,
//!   terminators, region membership, and finally routing.
//! - `grammar/statements/exceptions.g4` remains the syntax authority.
//!
//! No canonical IR successor type is duplicated here. The lowering adapter
//! must use the repository's canonical error/exception successor convention.
//!
//! ## Completion and freeze criteria
//!
//! This file is complete only after:
//! - its module path and exports compile in the real workspace;
//! - positive, negative, boundary, and ordering tests pass;
//! - the parser-to-AST-to-semantics-to-IR path is tested;
//! - exception propagation and finally behavior are verified in canonical IR;
//! - existing downstream consumers and compatibility requirements pass;
//! - no duplicate AST, exception type system, CFG, or IR is introduced.
//!
//! Public contract changes require review of semantic validation, lowering,
//! canonical IR verification, and compatibility tests.

use crate::classical::error::{ClassicalError, ClassicalResult};
use crate::classical::ids::{BlockId, SymbolId, TypeId, ValueId};

/// A resolved value reference associated with a source-level `throw`.
///
/// This is a reference to an existing canonical value, not an exception
/// object or a newly allocated IR value.
///
/// Whether the value is throwable, which handler receives it, and how it
/// propagates when no handler matches are decided by semantic analysis and
/// canonical control-flow lowering.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ThrowPlan {
    value: ValueId,
}

impl ThrowPlan {
    /// Creates a throw plan for an already-resolved value.
    ///
    /// This constructor is intentionally structural. It does not check the
    /// value's existence or type; those checks belong to semantic analysis
    /// and canonical IR verification.
    #[must_use]
    pub const fn new(value: ValueId) -> Self {
        Self { value }
    }

    /// Returns the canonical value associated with the throw.
    #[must_use]
    pub const fn value(&self) -> ValueId {
        self.value
    }

    /// Consumes the plan and returns its canonical value reference.
    #[must_use]
    pub const fn into_parts(self) -> ValueId {
        self.value
    }
}

/// The resolved information needed for one source-level `catch` clause.
///
/// The handler's position in its containing [`TryExceptionPlan`] is
/// significant: this module preserves source order and never sorts handlers.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CatchHandlerPlan {
    binding: SymbolId,
    exception_type: Option<TypeId>,
    entry: BlockId,
}

impl CatchHandlerPlan {
    /// Creates a handler plan from canonical semantic identities.
    ///
    /// `binding` identifies the catch binding after name resolution.
    /// `exception_type` is `Some` when the source catch clause has a resolved
    /// type annotation, or `None` for an untyped catch.
    /// `entry` identifies the handler's canonical entry block.
    ///
    /// The constructor does not verify that the symbol or type exists, that
    /// the type is throwable, or that the handler block belongs to the
    /// correct function or region.
    #[must_use]
    pub const fn new(
        binding: SymbolId,
        exception_type: Option<TypeId>,
        entry: BlockId,
    ) -> Self {
        Self {
            binding,
            exception_type,
            entry,
        }
    }

    /// Returns the canonical symbol for the catch binding.
    #[must_use]
    pub const fn binding(&self) -> SymbolId {
        self.binding
    }

    /// Returns the resolved exception type, if the clause is typed.
    #[must_use]
    pub const fn exception_type(&self) -> Option<TypeId> {
        self.exception_type
    }

    /// Returns the canonical handler-entry block.
    #[must_use]
    pub const fn entry(&self) -> BlockId {
        self.entry
    }

    /// Consumes the plan and returns its canonical references.
    #[must_use]
    pub const fn into_parts(self) -> (SymbolId, Option<TypeId>, BlockId) {
        (self.binding, self.exception_type, self.entry)
    }
}

/// Resolved exception-region references for one source-level try construct.
///
/// The `handlers` vector preserves source order. It is intentionally not a
/// set: sorting or deduplicating catch clauses could change language meaning.
///
/// `finally_entry` identifies the finally block when present. The canonical
/// CFG builder remains responsible for routing every applicable exit through
/// that block and preserving the pending control-flow outcome.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TryExceptionPlan {
    body_entry: BlockId,
    handlers: Vec<CatchHandlerPlan>,
    finally_entry: Option<BlockId>,
}

impl TryExceptionPlan {
    /// Constructs a try plan from already-resolved canonical references.
    ///
    /// A try construct must have one or more handlers, a finally block, or
    /// both. A construct with neither is rejected because it has no exception
    /// handling clause and is invalid under the canonical exception grammar.
    ///
    /// This function deliberately does not impose a maximum handler count.
    /// It accepts the caller's dynamically sized vector without copying it.
    ///
    /// # Errors
    ///
    /// Returns `ClassicalError::InvalidInput` if both `handlers` and
    /// `finally_entry` are absent/empty.
    ///
    /// This constructor does not verify block membership, type compatibility,
    /// handler reachability, or exception propagation. Those checks belong to
    /// semantic analysis and the canonical IR verifier.
    pub fn new(
        body_entry: BlockId,
        handlers: Vec<CatchHandlerPlan>,
        finally_entry: Option<BlockId>,
    ) -> ClassicalResult<Self> {
        if handlers.is_empty() && finally_entry.is_none() {
            return Err(ClassicalError::invalid_input(
                "a try construct requires at least one catch handler or a finally block",
            ));
        }

        Ok(Self {
            body_entry,
            handlers,
            finally_entry,
        })
    }

    /// Returns the canonical entry block of the try body.
    #[must_use]
    pub const fn body_entry(&self) -> BlockId {
        self.body_entry
    }

    /// Returns catch handlers in source order.
    #[must_use]
    pub fn handlers(&self) -> &[CatchHandlerPlan] {
        &self.handlers
    }

    /// Returns the number of catch clauses.
    #[must_use]
    pub fn handler_count(&self) -> usize {
        self.handlers.len()
    }

    /// Returns whether this construct has any catch clauses.
    #[must_use]
    pub fn has_handlers(&self) -> bool {
        !self.handlers.is_empty()
    }

    /// Returns the optional finally-entry block.
    #[must_use]
    pub const fn finally_entry(&self) -> Option<BlockId> {
        self.finally_entry
    }

    /// Returns whether this construct contains a finally block.
    #[must_use]
    pub const fn has_finally(&self) -> bool {
        self.finally_entry.is_some()
    }

    /// Consumes the plan and returns its canonical references.
    ///
    /// The handler vector retains its original source order.
    #[must_use]
    pub fn into_parts(
        self,
    ) -> (BlockId, Vec<CatchHandlerPlan>, Option<BlockId>) {
        (self.body_entry, self.handlers, self.finally_entry)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::classical::error::ClassicalError;

    fn block(value: u128) -> BlockId {
        BlockId::new(value).expect("test block ID must be nonzero")
    }

    fn value(value: u128) -> ValueId {
        ValueId::new(value).expect("test value ID must be nonzero")
    }

    fn symbol(value: u128) -> SymbolId {
        SymbolId::new(value).expect("test symbol ID must be nonzero")
    }

    fn ty(value: u128) -> TypeId {
        TypeId::new(value).expect("test type ID must be nonzero")
    }

    fn handler(
        binding: u128,
        exception_type: Option<TypeId>,
        entry: u128,
    ) -> CatchHandlerPlan {
        CatchHandlerPlan::new(symbol(binding), exception_type, block(entry))
    }

    #[test]
    fn throw_plan_preserves_canonical_value() {
        let thrown = value(1);
        let plan = ThrowPlan::new(thrown);

        assert_eq!(plan.value(), thrown);
        assert_eq!(plan.into_parts(), thrown);
    }

    #[test]
    fn typed_handler_preserves_binding_type_and_entry() {
        let binding = symbol(1);
        let exception_type = ty(2);
        let entry = block(3);

        let plan = CatchHandlerPlan::new(
            binding,
            Some(exception_type),
            entry,
        );

        assert_eq!(plan.binding(), binding);
        assert_eq!(plan.exception_type(), Some(exception_type));
        assert_eq!(plan.entry(), entry);
    }

    #[test]
    fn untyped_handler_is_supported() {
        let plan = handler(1, None, 2);

        assert_eq!(plan.exception_type(), None);
        assert_eq!(plan.binding(), symbol(1));
        assert_eq!(plan.entry(), block(2));
    }

    #[test]
    fn try_catch_plan_preserves_handler_source_order() {
        let first = handler(10, Some(ty(20)), 30);
        let second = handler(11, Some(ty(21)), 31);
        let third = handler(12, None, 32);

        let plan = TryExceptionPlan::new(
            block(1),
            vec![first, second, third],
            None,
        )
        .expect("try with catch handlers should be valid");

        assert_eq!(plan.handler_count(), 3);
        assert_eq!(plan.handlers(), &[first, second, third]);
        assert!(plan.has_handlers());
        assert!(!plan.has_finally());
    }

    #[test]
    fn try_finally_without_catch_is_supported() {
        let finally = block(3);

        let plan = TryExceptionPlan::new(
            block(1),
            Vec::new(),
            Some(finally),
        )
        .expect("try/finally is valid without a catch");

        assert!(!plan.has_handlers());
        assert!(plan.has_finally());
        assert_eq!(plan.finally_entry(), Some(finally));
    }

    #[test]
    fn try_catch_finally_is_supported() {
        let finally = block(40);
        let plan = TryExceptionPlan::new(
            block(1),
            vec![handler(2, None, 3)],
            Some(finally),
        )
        .expect("try/catch/finally should be valid");

        assert_eq!(plan.handler_count(), 1);
        assert_eq!(plan.finally_entry(), Some(finally));
    }

    #[test]
    fn try_without_catch_or_finally_is_rejected() {
        let result = TryExceptionPlan::new(block(1), Vec::new(), None);

        assert!(matches!(
            result,
            Err(ClassicalError::InvalidInput { .. })
        ));
    }

    #[test]
    fn handler_order_and_contents_survive_decomposition() {
        let handlers = vec![
            handler(10, Some(ty(100)), 20),
            handler(11, None, 21),
        ];
        let expected = handlers.clone();
        let finally = Some(block(30));

        let plan = TryExceptionPlan::new(
            block(1),
            handlers,
            finally,
        )
        .expect("valid try plan");

        let (body, actual_handlers, actual_finally) = plan.into_parts();

        assert_eq!(body, block(1));
        assert_eq!(actual_handlers, expected);
        assert_eq!(actual_finally, finally);
    }

    #[test]
    fn large_canonical_identifiers_are_preserved() {
        let large_block = block(u128::MAX);
        let large_value = value(u128::MAX - 1);
        let large_symbol = symbol(u128::MAX - 2);
        let large_type = ty(u128::MAX - 3);

        let throw_plan = ThrowPlan::new(large_value);
        let catch_plan = CatchHandlerPlan::new(
            large_symbol,
            Some(large_type),
            large_block,
        );

        assert_eq!(throw_plan.value(), large_value);
        assert_eq!(catch_plan.binding(), large_symbol);
        assert_eq!(catch_plan.exception_type(), Some(large_type));
        assert_eq!(catch_plan.entry(), large_block);
    }

    #[test]
    fn handler_count_has_no_module_defined_ceiling() {
        // A test workload, not a production limit.
        let handlers = (1_u128..=1_024)
            .map(|n| handler(n, None, n + 2_000))
            .collect::<Vec<_>>();

        let plan = TryExceptionPlan::new(
            block(1),
            handlers,
            None,
        )
        .expect("all supplied handlers should be retained");

        assert_eq!(plan.handler_count(), 1_024);
    }

    #[test]
    fn plans_support_clone_and_equality() {
        let plan = TryExceptionPlan::new(
            block(1),
            vec![handler(2, Some(ty(3)), 4)],
            Some(block(5)),
        )
        .expect("valid try plan");

        assert_eq!(plan.clone(), plan);
    }
}
