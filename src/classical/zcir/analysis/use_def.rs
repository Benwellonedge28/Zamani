
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZCIR Definition–Use Analysis
//!
//! Canonical path:
//! `src/classical/zcir/analysis/use_def.rs`
//!
//! Rust edition: 2021.
//! Minimum intended Rust version: 1.97.
//! Safety policy: unsafe Rust is forbidden.
//!
//! ## Purpose
//!
//! Expose definition–use indexing to classical ZCIR consumers while retaining
//! ZUIR as the single owner of the canonical definition–use implementation.
//!
//! A definition–use index answers questions such as:
//!
//! - Where is a value defined?
//! - Which operation results or block arguments define values?
//! - Which operations and terminators consume a value?
//! - Which values have no indexed definition?
//! - How many definitions, operations, blocks and uses were indexed?
//!
//! ## Architectural authority
//!
//! This module is subordinate to:
//!
//! - `src/zuir/ids.rs`
//! - `src/zuir/ir/use_def.rs`
//! - `src/zuir/ir/block.rs`
//! - `src/zuir/ir/operation.rs`
//! - `src/zuir/ir/successor.rs`
//! - `src/zuir/ir/region.rs`
//! - `src/zuir/ZUIR-SPEC.md`
//! - `grammar/Zamani.g4`
//! - `grammar/antlr/ZamaniLexer.g4`
//! - `grammar/antlr/ZamaniParser.g4`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/validation/semantic-boundaries.md`
//!
//! The canonical implementation in `src/zuir/ir/use_def.rs` owns the index's
//! data structures, construction, mutation, consistency checks and errors.
//! This module does not reimplement any of those responsibilities.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - The classical-facing public integration boundary.
//! - Explicit aliases that improve discoverability for ZCIR clients.
//! - Documentation of the lifecycle and validity contract.
//!
//! This module does not own:
//!
//! - Value, operation, block, function or type identifiers.
//! - Definitions, use sites, use lists or definition–use storage.
//! - IR operations, operands, results, blocks or terminators.
//! - Canonical source maps or source provenance.
//! - Dominance, SSA legality, type verification or control-flow verification.
//! - Alias analysis, liveness, optimization or lowering.
//! - Memory allocation policy or physical resource assignment.
//! - Source parsing, AST construction or semantic analysis.
//!
//! ## Canonical integration
//!
//! The public ZUIR types are re-exported directly. Consequently, a
//! `ClassicalUseDefIndex` and a `UseDefIndex` are the same underlying type;
//! there is no conversion, duplicated state or independently maintained
//! definition–use model.
//!
//! Definitions use canonical `ValueId` and `TypeId` values. Definition
//! locations use canonical operation, block and function identities.
//!
//! The caller supplies the appropriate enclosing IR scope and is responsible
//! for registering definitions that the canonical builder cannot infer,
//! including function parameters when they are not represented in the
//! supplied block and operation collections.
//!
//! ## Validity boundary
//!
//! A successfully constructed index is not proof that an IR is executable.
//! In particular, this adapter does not establish:
//!
//! - Dominance or definition-before-use legality.
//! - Region visibility or capture legality.
//! - Type compatibility.
//! - Complete control-flow graph validity.
//! - Effect ordering, ownership or aliasing constraints.
//! - Dialect-specific operation legality.
//! - Target capabilities or resource feasibility.
//!
//! Consumers must invoke the appropriate canonical verifiers and analyses
//! for those properties before optimization, lowering or execution.
//!
//! ## Lifecycle and invalidation
//!
//! The index is a derived snapshot, not authoritative IR state.
//!
//! Rebuild it after mutations affecting definitions, operands, results,
//! block arguments, terminators, successor arguments or scope membership.
//! Do not retain an index across such mutations unless the owning IR layer
//! provides a sound invalidation or incremental-update protocol.
//!
//! Register all externally owned definitions before calling
//! `verify_resolved()`. Recording a use and resolving that use are separate
//! operations in the canonical API.
//!
//! ## Scalability and POCO-REAF
//!
//! This adapter introduces no fixed limits on:
//!
//! - Values, definitions or uses.
//! - Operations, blocks or functions.
//! - Operands, results or block arguments.
//! - Successor edges or edge arguments.
//! - Processor count, register width, memory size or device count.
//!
//! Capacity and processing costs are governed by the canonical
//! implementation, available resources and explicit compilation policies.
//! No finite implementation can promise literal infinity.
//!
//! This adapter does not introduce a smaller artificial ceiling or silently
//! truncate an index. Canonical errors are propagated without reinterpretation.
//!
//! ## Determinism
//!
//! Ordering, duplicate handling, identity validation and lookup semantics
//! are those of the canonical ZUIR implementation. This module must not add
//! a second ordering rule or independently normalize the index.
//!
//! ## Error handling
//!
//! Canonical errors are exposed unchanged. Callers can distinguish errors
//! such as duplicate definitions, duplicate identities, missing operation
//! references, operations assigned to multiple blocks, undefined values,
//! inconsistent index entries, allocation-reservation failures and count
//! overflow.
//!
//! The adapter must not replace errors with panics, silently discard invalid
//! entries or claim that an unsuccessful build produced a valid index.
//!
//! ## Integration and freeze conditions
//!
//! Before this module is registered:
//!
//! 1. Confirm `crate::zuir::ir::use_def` is publicly reachable.
//! 2. Confirm the canonical ZUIR module and classical module roots compile.
//! 3. Register this module through `analysis/mod.rs`.
//! 4. Confirm the ZCIR analysis root is registered by its parent.
//! 5. Add integration tests against real canonical ZUIR blocks and operations.
//! 6. Run the repository's formatting, compilation, test and lint gates.
//!
//! This file must not be made to compile by introducing duplicate canonical
//! IDs, a parallel index or speculative replacements for existing ZUIR APIs.

/// Canonical definition location and category.
pub use crate::zuir::ir::use_def::DefinitionKind;

/// Canonical definition–use error type.
pub use crate::zuir::ir::use_def::UseDefError;

/// Canonical result type for definition–use operations.
pub use crate::zuir::ir::use_def::UseDefResult;

/// Canonical derived definition–use index.
pub use crate::zuir::ir::use_def::UseDefIndex;

/// Canonical value definition record.
pub use crate::zuir::ir::use_def::ValueDefinition;

/// Canonical value use record.
pub use crate::zuir::ir::use_def::ValueUse;

/// Canonical value use-site representation.
pub use crate::zuir::ir::use_def::UseSite;

/// Classical-facing name for the canonical ZUIR definition–use index.
///
/// This is a type alias, not a new index implementation. Both names refer
/// to the same type and share the same construction, validation and lifecycle
/// contract.
pub type ClassicalUseDefIndex = UseDefIndex;

/// Classical-facing name for canonical definition–use errors.
pub type ClassicalUseDefError = UseDefError;

/// Classical-facing result type for definition–use operations.
pub type ClassicalUseDefResult<T> = UseDefResult<T>;

#[cfg(test)]
mod tests {
    use super::*;

    // Compile-time API contract: ZCIR aliases must remain exactly the same
    // types as the canonical ZUIR implementation.
    fn assert_same_index_type(_: Option<ClassicalUseDefIndex>) {
        let _: Option<UseDefIndex> = None;
    }

    fn assert_same_error_type(_: Option<ClassicalUseDefError>) {
        let _: Option<UseDefError> = None;
    }

    #[test]
    fn classical_index_is_the_canonical_index() {
        assert_same_index_type(None);
    }

    #[test]
    fn classical_error_is_the_canonical_error() {
        assert_same_error_type(None);
    }

    #[test]
    fn empty_index_has_consistent_state() {
        let index = ClassicalUseDefIndex::new();

        assert_eq!(index.definition_count(), 0);
        assert_eq!(index.used_value_count(), 0);
        assert_eq!(index.use_count(), Some(0));
        assert_eq!(index.operation_count(), 0);
        assert_eq!(index.block_count(), 0);
        assert!(index.verify_internal_consistency().is_ok());
        assert!(index.verify_resolved().is_ok());
    }

    #[test]
    fn clearing_index_removes_all_indexed_state() {
        let mut index = ClassicalUseDefIndex::new();

        index.clear();

        assert_eq!(index.definition_count(), 0);
        assert_eq!(index.used_value_count(), 0);
        assert_eq!(index.use_count(), Some(0));
        assert_eq!(index.operation_count(), 0);
        assert_eq!(index.block_count(), 0);
        assert!(index.verify_internal_consistency().is_ok());
    }
}
