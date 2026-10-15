#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR Definition-Use Analysis
//!
//! Path: `src/zuir/analysis/def_use.rs`
//! Edition: Rust 2021
//! Minimum intended Rust version: 1.97
//! Safety: Unsafe Rust is forbidden.
//!
//! STATUS
//!
//! Proposed analysis facade. Freeze after repository integration and tests.
//!
//! PURPOSE
//!
//! Expose the canonical ZUIR definition-use index through the analysis
//! namespace. Provide a stable entry point for data-flow analysis,
//! optimization, verification, dominance analysis, and future incremental
//! compilation.
//!
//! AUTHORITY
//!
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/ids.rs`
//! - `src/zuir/ir/use_def.rs`
//! - `src/zuir/ir/operation.rs`
//! - `src/zuir/ir/block.rs`
//! - `src/zuir/ir/successor.rs`
//! - `src/zuir/ir/region.rs`
//! - `grammar/DESIGN.md`
//! - `grammar/Zamani.g4`
//! - `grammar/antlr/ZamaniParser.g4`
//! - `grammar/specification/semantic-model.md`
//!
//! OWNERSHIP
//!
//! This module owns the analysis-facing API only.
//!
//! `ir/use_def.rs` remains the sole owner of:
//! - `UseDefIndex`;
//! - `ValueDefinition`;
//! - `ValueUse`;
//! - `DefinitionKind`;
//! - `UseSite`;
//! - `UseDefError`;
//! - definition registration and use indexing;
//! - structural index consistency checks.
//!
//! DOES_NOT_OWN
//!
//! - Canonical value, block, operation, function, or type identifiers.
//! - Operation operands and results.
//! - Block arguments, terminators, and control-flow successors.
//! - AST construction, source parsing, or name resolution.
//! - Type inference or type compatibility.
//! - Dominance or path-sensitive SSA verification.
//! - Ownership, effect, resource, or dialect verification.
//! - IR mutation, optimization, serialization, or target lowering.
//!
//! INPUTS
//!
//! Already-constructed ZUIR blocks and operations using canonical IDs.
//! Inputs must belong to the same intended IR scope.
//!
//! OUTPUTS
//!
//! A derived `UseDefIndex`, or a structured `UseDefError` describing a
//! detected indexing or consistency failure.
//!
//! DEPENDENCIES
//!
//! Only canonical ZUIR IR types and the canonical `ir/use_def.rs`
//! implementation. This module must not introduce another index,
//! another identifier system, or another source of semantic truth.
//!
//! INTEGRATION
//!
//! - `analysis/mod.rs` exports this module.
//! - Whole-IR verification consumes the index and performs its own
//!   dominance, type, scope, ownership, and control-flow checks.
//! - Transformations rebuild or invalidate the index after relevant IR
//!   mutations.
//! - The frontend and grammar do not depend on this analysis module.
//!
//! SCALABILITY AND POCO-REAF
//!
//! This facade imposes no fixed limit on blocks, operations, values,
//! uses, qubits, registers, processors, devices, tensor dimensions,
//! or distributed nodes.
//!
//! The canonical index uses dynamically sized collections. Actual
//! capacity remains bounded by available memory, address space, and
//! execution time. No finite implementation can guarantee literal
//! infinity.
//!
//! DETERMINISM
//!
//! Ordering and indexing behavior are defined by the canonical
//! `ir/use_def.rs` implementation. This facade must not reorder results
//! or introduce a competing traversal policy.
//!
//! VALIDITY BOUNDARY
//!
//! Successful index construction does not establish full IR validity.
//! Call `verify_resolved()` only after all definitions belonging to the
//! analyzed scope, including enclosing function parameters, have been
//! registered.
//!
//! A resolved use is not necessarily dominated by its definition.
//! The whole-IR verifier remains responsible for that distinction.
//!
//! FREEZE CRITERIA
//!
//! - The canonical index remains owned by `ir/use_def.rs`.
//! - Public types are re-exported without duplication.
//! - Missing and duplicate references are reported by the canonical index.
//! - The analysis module is registered in the actual module hierarchy.
//! - Public API, documentation, and integration tests pass.
//! - No unsafe code or artificial hardware-size limits are introduced.

use crate::zuir::ids::{BlockId, OperationId};
use crate::zuir::ir::block::ZuirBlock;
use crate::zuir::ir::operation::ZuirOperation;

/// Canonical definition-use index implementation.
///
/// Re-exported rather than reimplemented so all ZUIR consumers share
/// one authoritative index representation.
pub use crate::zuir::ir::use_def::{
    DefinitionKind,
    UseDefError,
    UseDefIndex,
    UseDefResult,
    UseSite,
    ValueDefinition,
    ValueUse,
};

/// Build a definition-use index from a collection of blocks and operations.
///
/// The canonical implementation indexes:
///
/// - block-argument definitions;
/// - operation-result definitions;
/// - operation operand uses;
/// - returned values;
/// - conditional branch conditions;
/// - switch selectors;
/// - successor-edge arguments.
///
/// Repeated operand references are retained as distinct uses.
///
/// # Scope requirements
///
/// `blocks` and `operations` must describe the same intended IR scope.
/// Every operation referenced by a block must occur in `operations`.
/// Function parameters and other externally owned definitions must be
/// registered separately before resolution verification.
///
/// # Errors
///
/// Returns the canonical `UseDefError` when duplicate identities,
/// missing operation references, conflicting operation ownership,
/// or other index construction errors are detected.
///
/// # Important
///
/// This function does not prove dominance, type compatibility,
/// region visibility, ownership legality, effect ordering, or dialect
/// validity. Those checks belong to the appropriate verifier passes.
pub fn build_index(
    blocks: &[ZuirBlock],
    operations: &[ZuirOperation],
) -> UseDefResult<UseDefIndex> {
    UseDefIndex::build(blocks, operations)
}

/// Register a definition owned by an enclosing scope.
///
/// Use this for definitions that the canonical block-and-operation
/// builder cannot infer, such as function parameters.
///
/// The canonical index rejects a value identity that already has a
/// definition.
pub fn register_definition(
    index: &mut UseDefIndex,
    value: crate::zuir::ids::ValueId,
    ty: crate::zuir::ids::TypeId,
    kind: DefinitionKind,
) -> UseDefResult<()> {
    index.define(value, ty, kind)
}

/// Record a value use in an existing index.
///
/// This is useful when a consumer discovers additional uses outside
/// the blocks and operations supplied to `build_index`.
///
/// The caller must ensure that the use site accurately represents the
/// IR. Resolution is checked separately.
pub fn register_use(
    index: &mut UseDefIndex,
    value_use: ValueUse,
) -> UseDefResult<()> {
    index.record_use(value_use)
}

/// Verify that the index's internal representation is consistent.
///
/// This checks the index itself, not the complete IR's semantic validity.
pub fn verify_index(index: &UseDefIndex) -> UseDefResult<()> {
    index.verify_internal_consistency()
}

/// Verify that every indexed use has a corresponding definition.
///
/// Register all definitions in the intended scope before calling this
/// function. This check does not establish SSA dominance.
pub fn verify_resolved(index: &UseDefIndex) -> UseDefResult<()> {
    index.verify_resolved()
}

/// Return the definition of a value, if it is indexed.
#[must_use]
pub fn definition_of(
    index: &UseDefIndex,
    value: crate::zuir::ids::ValueId,
) -> Option<&ValueDefinition> {
    index.definition(value)
}

/// Return the uses of a value in deterministic order.
///
/// An unused or unknown value returns an empty slice. Use
/// `definition_of` and `verify_resolved` when distinguishing those cases
/// matters.
#[must_use]
pub fn uses_of(
    index: &UseDefIndex,
    value: crate::zuir::ids::ValueId,
) -> &[ValueUse] {
    index.uses_of(value)
}

/// Return the total number of indexed uses.
///
/// `None` indicates that the count cannot be represented by `usize`.
#[must_use]
pub fn total_use_count(index: &UseDefIndex) -> Option<usize> {
    index.use_count()
}

/// Return whether an operation identity is present in the indexed scope.
///
/// This checks the identities tracked by the canonical index. It does not
/// establish that the operation is semantically valid.
#[must_use]
pub fn contains_operation(
    index: &UseDefIndex,
    operation: OperationId,
) -> bool {
    index
        .operation_count()
        .checked_sub(0)
        .is_some()
        && index
            .operation_ids_for_analysis()
            .contains(&operation)
}

/// Return whether a block identity is present in the indexed scope.
///
/// This checks the identities tracked by the canonical index only.
#[must_use]
pub fn contains_block(
    index: &UseDefIndex,
    block: BlockId,
) -> bool {
    index
        .block_ids_for_analysis()
        .contains(&block)
}