
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_debug_implementations)]

//! # ZCIR Control-Flow Integration
//!
//! Canonical path: `src/classical/zcir/ir/control_flow.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: unsafe Rust is forbidden.
//!
//! ## PURPOSE
//!
//! Expose canonical ZUIR control-flow terminators and successor edges through
//! the ZCIR namespace, with small allocation-free helpers for inspecting
//! outgoing edges.
//!
//! This module is an integration facade. It does not implement a second
//! control-flow representation or maintain a separate control-flow graph.
//!
//! ## AUTHORITY
//!
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/ir/mod.rs`
//! - `src/zuir/ir/block.rs`
//! - `src/zuir/ir/successor.rs`
//! - `src/zuir/ir/region.rs`
//! - `src/zuir/ir/operation.rs`
//! - `src/zuir/ir/use_def.rs`
//! - `src/zuir/ids.rs`
//! - `src/zuir/schema/attributes.rs`
//! - `src/zuir/schema/types.rs`
//! - `grammar/DESIGN.md`
//! - `grammar/Zamani.g4`
//! - `grammar/functions/functions.g4`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/specification/poco-reaf.md`
//! - `grammar/validation/semantic-boundaries.md`
//!
//! Grammar files define source syntax. Semantic analysis establishes resolved
//! meaning. ZUIR owns the canonical IR entities. This module exposes their
//! control-flow contracts to classical-computing consumers.
//!
//! ## OWNERSHIP
//!
//! This module owns only:
//!
//! - ZCIR-facing aliases for canonical control-flow types.
//! - Allocation-free successor visitation.
//! - Small adapter-level contract tests.
//!
//! The canonical implementations remain authoritative for:
//!
//! - Block and operation identity.
//! - Terminator representation and local validation.
//! - Successor targets and ordered edge arguments.
//! - Block contents and region membership.
//! - Operation definitions and operation effects.
//!
//! ## DOES NOT OWN
//!
//! This module does not own:
//!
//! - A separate control-flow graph or graph storage.
//! - A new terminator enum or successor-edge structure.
//! - ID allocation, global identity uniqueness, or reference resolution.
//! - Whole-graph verification, reachability, dominance, or post-dominance.
//! - SSA construction or use-def index maintenance.
//! - Type checking or semantic interpretation of registered extensions.
//! - Source parsing, AST construction, or semantic analysis.
//! - Hardware placement, scheduling, register allocation, or execution.
//!
//! ## CANONICAL CONTROL-FLOW CONTRACT
//!
//! `ZcirControlFlowTerminator` is exactly `BlockTerminator`.
//! `ZcirControlFlowEdge` is exactly `Successor`.
//!
//! A successor records a target `BlockId` and an ordered sequence of
//! `ValueId` arguments. Argument order is significant. Repeated argument
//! values are permitted by the canonical edge representation.
//!
//! Terminator variants and their meanings are owned by the canonical ZUIR
//! implementation. In particular, an extension terminator must not be
//! assumed to have semantics merely because its name and outgoing edges
//! are structurally valid.
//!
//! The helper functions below delegate to canonical methods; they do not
//! independently infer control-flow meaning.
//!
//! ## VALIDATION BOUNDARY
//!
//! Inspecting outgoing edges does not establish that:
//!
//! - Every target block exists.
//! - Every target belongs to the correct region.
//! - Edge arguments match destination block arguments in number or type.
//! - Values are defined, visible, live, or dominating at the edge.
//! - Every reachable block has a complete terminator.
//! - Extension terminators are registered or semantically valid.
//! - The enclosing function or program is valid.
//!
//! These checks belong to the whole-IR verifier and applicable dialect
//! contracts. Imported, deserialized, or transformed IR must be verified
//! before trusted optimization, lowering, or execution.
//!
//! ## INTEGRATION CONTRACT
//!
//! - `ir/block.rs` exposes canonical blocks and their terminators.
//! - `ir/region.rs` exposes canonical region ownership and block references.
//! - `ir/operation.rs` exposes canonical operation definitions.
//! - `ir/value.rs` exposes canonical value identities and definition/use APIs.
//! - `verify/structural.rs` must resolve targets and verify graph structure.
//! - `verify/dominance.rs` must establish the applicable SSA visibility rules.
//! - `analysis/control_flow.rs` may build derived CFG indexes and perform
//!   reachability, predecessor, dominator, and related analyses.
//! - `analysis/use_def.rs` must remain consistent with canonical definitions
//!   and edge arguments.
//! - `lowering/to_zuir.rs` must not copy or reinterpret already-canonical
//!   control-flow entities unnecessarily.
//! - `src/quantum/ir/` and `src/quantum/ir/classical/` retain their existing
//!   ownership boundaries for hybrid and quantum-specific contracts.
//!
//! Derived graph indexes are snapshots. A transformation that changes
//! terminators, edges, blocks, or region membership must invalidate or rebuild
//! affected analyses before they are reused.
//!
//! The ZCIR IR parent module must declare this file only after it exists and
//! must avoid duplicate or ambiguous public re-exports.
//!
//! ## SCALABILITY AND POCO-REAF
//!
//! This adapter imposes no fixed universal ceiling on:
//!
//! - Blocks, operations, functions, regions, or edges.
//! - Successor arguments or switch cases.
//! - Processors, threads, devices, or distributed participants.
//! - Integer widths, register counts, or other hardware capacities.
//!
//! Successor visitation does not allocate a temporary collection. Its work is
//! proportional to the number of outgoing edges visited.
//!
//! The canonical successor and terminator representations use dynamically
//! sized collections where collections are needed. Their actual capacity is
//! bounded by the available address space, allocator behavior, identifier
//! representation, and configured operational policies.
//!
//! No finite implementation can promise literal infinity. POCO-REAF requires
//! avoiding arbitrary language-wide capacity ceilings, preserving semantics,
//! and reporting resource exhaustion or target infeasibility rather than
//! silently changing program meaning.
//!
//! Operational budgets for untrusted IR, verification, decoding, or analysis
//! belong to the relevant configuration and execution boundary. They must not
//! be presented as universal source-language limits.
//!
//! ## DETERMINISM
//!
//! This adapter preserves the canonical order of successors. It neither sorts
//! nor deduplicates edges, because doing so could change control-flow meaning.
//! Canonical serialization and stable fingerprints remain responsibilities of
//! the designated ZUIR serialization and fingerprint infrastructure.
//!
//! ## FREEZE CRITERIA
//!
//! - All exported entities reuse canonical ZUIR types.
//! - No duplicate control-flow state or identity system is introduced.
//! - Successor visitation preserves canonical edge order without allocation.
//! - The parent module registers this file without registering absent files.
//! - Focused tests, formatting, compilation, linting, and integration tests pass.
//! - Whole-graph correctness is established by the designated verifier, not
//!   inferred from the existence of these aliases or helpers.

/// Canonical control-flow terminator implementation.
///
/// The canonical enum includes open construction state, return, unconditional
/// and conditional branches, switch, unreachable, and extension terminators.
pub use crate::zuir::ir::block::BlockTerminator;

/// Canonical switch-case representation.
///
/// The canonical type preserves case-value and successor semantics.
pub use crate::zuir::ir::block::SwitchCase;

/// Canonical basic-block implementation.
pub use crate::zuir::ir::block::ZuirBlock;

/// Canonical basic-block-local error type.
pub use crate::zuir::ir::block::BlockError;

/// Canonical basic-block construction and validation result.
pub use crate::zuir::ir::block::BlockResult;

/// Canonical successor-edge representation.
pub use crate::zuir::ir::successor::Successor;

/// Error type for fallible canonical successor construction.
pub use crate::zuir::ir::successor::SuccessorError;

/// Result type for fallible canonical successor construction.
pub use crate::zuir::ir::successor::SuccessorResult;

/// ZCIR-facing alias for the canonical ZUIR terminator.
///
/// This is a type alias, not an independent terminator representation.
pub type ZcirControlFlowTerminator = BlockTerminator;

/// ZCIR-facing alias for the canonical switch case.
pub type ZcirSwitchCase = SwitchCase;

/// ZCIR-facing alias for the canonical basic block.
pub type ZcirControlFlowBlock = ZuirBlock;

/// ZCIR-facing alias for the canonical successor edge.
pub type ZcirControlFlowEdge = Successor;

/// ZCIR-facing alias for canonical block-local errors.
pub type ZcirControlFlowError = BlockError;

/// ZCIR-facing alias for canonical block construction and validation results.
pub type ZcirControlFlowResult<T> = BlockResult<T>;

/// ZCIR-facing alias for canonical successor construction errors.
pub type ZcirSuccessorError = SuccessorError;

/// ZCIR-facing alias for canonical successor construction results.
pub type ZcirSuccessorResult<T> = SuccessorResult<T>;

/// Visit a terminator's explicit outgoing edges in canonical order.
///
/// This delegates directly to [`BlockTerminator::for_each_successor`].
/// It allocates no collection and does not resolve target IDs or validate
/// edge arguments.
///
/// The visitor receives borrowed edges. It must not assume that the target
/// block exists or that the edge is valid in its enclosing graph.
///
/// # Examples
///
/// ```ignore
/// use crate::classical::zcir::ir::control_flow::for_each_successor;
/// use crate::classical::zcir::ir::control_flow::ZcirControlFlowTerminator;
/// use crate::zuir::ids::BlockId;
/// use crate::zuir::ir::successor::Successor;
///
/// let target = BlockId::new(2).expect("nonzero test ID");
/// let terminator = ZcirControlFlowTerminator::Branch {
///     successor: Successor::new(target),
/// };
///
/// let mut visited = Vec::new();
/// for_each_successor(&terminator, |edge| visited.push(edge.target()));
///
/// assert_eq!(visited, vec![target]);
/// ```
pub fn for_each_successor(
    terminator: &BlockTerminator,
    visit: impl FnMut(&Successor),
) {
    terminator.for_each_successor(visit);
}

/// Return the number of explicitly represented outgoing edges.
///
/// The result is structural information supplied by the canonical terminator.
/// It does not prove that the edges are valid or that a registered extension's
/// semantic behavior is fully described by its edge list.
#[must_use]
pub fn successor_count(terminator: &BlockTerminator) -> usize {
    terminator.successor_count()
}

/// Return whether the terminator is still in its canonical construction state.
///
/// An open terminator is not a finalized control-flow terminator. The enclosing
/// verifier should reject an open terminator wherever the IR contract requires
/// a complete block.
#[must_use]
pub const fn is_open(terminator: &BlockTerminator) -> bool {
    terminator.is_open()
}

/// Return whether the terminator is closed according to the canonical model.
///
/// This reports only the terminator's local construction state. It does not
/// establish whole-function validity or that every control-flow path returns.
#[must_use]
pub const fn is_closed(terminator: &BlockTerminator) -> bool {
    terminator.is_closed()
}

/// Validate the terminator's canonical local invariants.
///
/// This is equivalent to the canonical terminator validation call. It does
/// not resolve blocks or values, verify edge argument types, or establish
/// whole-graph correctness.
pub fn validate_terminator(
    terminator: &BlockTerminator,
) -> BlockResult<()> {
    terminator.validate()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::zuir::ids::{BlockId, ValueId};
    use crate::zuir::ir::successor::Successor;

    fn block_id(raw: u128) -> BlockId {
        BlockId::new(raw).expect("test block ID must be nonzero")
    }

    fn value_id(raw: u128) -> ValueId {
        ValueId::new(raw).expect("test value ID must be nonzero")
    }

    #[test]
    fn zcir_terminator_is_the_canonical_zuir_type() {
        fn accepts_zcir(_: ZcirControlFlowTerminator) {}
        fn accepts_zuir(_: BlockTerminator) {}

        let terminator = ZcirControlFlowTerminator::Unreachable;

        accepts_zcir(terminator.clone());
        accepts_zuir(terminator.clone());

        assert_eq!(
            core::any::type_name::<ZcirControlFlowTerminator>(),
            core::any::type_name::<BlockTerminator>(),
        );
    }

    #[test]
    fn zcir_edge_is_the_canonical_successor_type() {
        fn accepts_zcir(_: ZcirControlFlowEdge) {}
        fn accepts_zuir(_: Successor) {}

        let edge = ZcirControlFlowEdge::new(block_id(7));

        accepts_zcir(edge.clone());
        accepts_zuir(edge.clone());

        assert_eq!(
            core::any::type_name::<ZcirControlFlowEdge>(),
            core::any::type_name::<Successor>(),
        );
        assert_eq!(edge.target(), block_id(7));
        assert!(edge.arguments().is_empty());
    }

    #[test]
    fn unconditional_branch_visits_its_target() {
        let target = block_id(2);
        let terminator = BlockTerminator::Branch {
            successor: Successor::new(target),
        };

        let mut visited = Vec::new();
        for_each_successor(&terminator, |edge| {
            visited.push(edge.target());
        });

        assert_eq!(visited, vec![target]);
        assert_eq!(successor_count(&terminator), 1);
        assert!(!is_open(&terminator));
        assert!(is_closed(&terminator));
        assert!(validate_terminator(&terminator).is_ok());
    }

    #[test]
    fn conditional_branch_preserves_successor_order() {
        let then_target = block_id(2);
        let else_target = block_id(3);

        let terminator = BlockTerminator::ConditionalBranch {
            condition: value_id(1),
            then_successor: Successor::new(then_target),
            else_successor: Successor::new(else_target),
        };

        let mut visited = Vec::new();
        for_each_successor(&terminator, |edge| {
            visited.push(edge.target());
        });

        assert_eq!(visited, vec![then_target, else_target]);
        assert_eq!(successor_count(&terminator), 2);
        assert!(validate_terminator(&terminator).is_ok());
    }

    #[test]
    fn edge_arguments_preserve_order_and_repeated_values() {
        let first = value_id(10);
        let second = value_id(20);
        let target = block_id(30);

        let edge = Successor::with_arguments(
            target,
            vec![first, second, first],
        );

        assert_eq!(edge.target(), target);
        assert_eq!(edge.arguments(), &[first, second, first]);
        assert_eq!(edge.argument_count(), 3);
    }

    #[test]
    fn return_and_unreachable_have_no_explicit_successors() {
        let return_terminator = BlockTerminator::Return {
            values: vec![value_id(1), value_id(2)],
        };
        let unreachable_terminator = BlockTerminator::Unreachable;

        assert_eq!(successor_count(&return_terminator), 0);
        assert_eq!(successor_count(&unreachable_terminator), 0);

        let mut visited = 0;
        for_each_successor(&return_terminator, |_| visited += 1);
        for_each_successor(&unreachable_terminator, |_| visited += 1);

        assert_eq!(visited, 0);
        assert!(validate_terminator(&return_terminator).is_ok());
        assert!(validate_terminator(&unreachable_terminator).is_ok());
    }

    #[test]
    fn open_terminator_is_not_a_finalized_terminator() {
        let terminator = BlockTerminator::Open;

        assert!(is_open(&terminator));
        assert!(!is_closed(&terminator));
        assert_eq!(successor_count(&terminator), 0);

        // Canonical local validation accepts the construction state.
        // Whole-IR verification must reject it where a closed block is required.
        assert!(validate_terminator(&terminator).is_ok());
    }

    #[test]
    fn successor_visitor_does_not_change_the_terminator() {
        let terminator = BlockTerminator::ConditionalBranch {
            condition: value_id(1),
            then_successor: Successor::new(block_id(2)),
            else_successor: Successor::new(block_id(3)),
        };
        let before = terminator.clone();

        for_each_successor(&terminator, |_| {});

        assert_eq!(terminator, before);
    }
}
