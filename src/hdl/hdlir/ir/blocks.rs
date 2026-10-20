
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani HDLIR — Blocks
//!
//! Path: `src/hdl/hdlir/ir/blocks.rs`
//! Edition: Rust 2021.
//! Minimum supported Rust version: Rust 1.97.
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Represent ordered blocks in the Zamani hardware-domain intermediate
//! representation (HDLIR).
//!
//! A block declares zero or more block arguments, an ordered sequence of
//! operation references, and an optional terminator reference. The block
//! stores identities rather than owning duplicate copies of operations.
//!
//! ## Architectural authority
//!
//! This module follows:
//!
//! - `grammar/DESIGN.md`
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/hdl.g4` and its applicable delegate grammars
//! - `grammar/specification/semantic-model.md`
//! - `src/hdl/api.rs`
//! - `src/hdl/ids.rs`
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/hdl/hdlir/HDLIR-SPEC.md`, when established
//! - `src/hdl/hdlir/ids.rs`
//! - `src/hdl/hdlir/errors.rs`
//! - `src/hdl/hdlir/ir/operations.rs`
//! - `src/hdl/hdlir/ir/regions.rs`
//! - `src/hdl/hdlir/ir/values.rs`
//!
//! Grammar defines source syntax. The canonical frontend owns source-level
//! meaning. ZUIR owns the universal computational representation. HDLIR owns
//! hardware-domain block structure and its local invariants.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Block identity.
//! - Ordered block-argument declarations.
//! - Ordered references to operations.
//! - An optional reference identifying the block's terminator operation.
//! - Local block-structure validation.
//!
//! This module does not own:
//!
//! - Operation definitions, operation execution, or operation storage.
//! - The authoritative type system or symbolic width evaluator.
//! - The region containing this block.
//! - The complete value-definition or use-def graph.
//! - Control-flow edge definitions or branch interpretation.
//! - Hardware signal connectivity, process semantics, clock behavior,
//!   memory semantics, or source-level semantic analysis.
//! - Physical hardware allocation, placement, routing, synthesis,
//!   or target-specific realization.
//!
//! ## Integration contract
//!
//! - `ids.rs` owns the typed block, value, type, and operation identities.
//! - `operations.rs` owns operation instances and their result declarations.
//! - `regions.rs` owns ordered block references and optional entry-block IDs.
//! - `values.rs` owns graph-level value definitions and derived use information.
//! - `control_flow.rs`, when implemented, owns branch and control-flow meaning.
//! - `verify/structure.rs` must check operation existence, exclusive ownership,
//!   region membership, argument/result uniqueness, and reference consistency.
//! - `verify/types.rs` must check block-argument types against the type registry.
//! - Builders must use the checked constructors and must not bypass required
//!   graph-wide verification.
//!
//! This module intentionally stores operation IDs rather than importing the
//! graph owner or operation registry. That keeps the block representation
//! independent of graph storage and avoids a dependency cycle.
//!
//! ## Block semantics
//!
//! A block argument is a value declaration with a block-local position and
//! an HDLIR value identity. Its type is represented by an HDLIR type identity.
//! The type identity must resolve in the enclosing graph's type namespace.
//!
//! Operations are ordered. Their order is preserved exactly as supplied.
//! Operation IDs are identities, not vector indices.
//!
//! A terminator is represented by an operation ID already present in the
//! block's ordered operation list. This local representation does not assert
//! that the operation is a legal terminator. The registered operation schema
//! and whole-graph verifier must establish that fact.
//!
//! Some hardware regions use structural or dataflow semantics and may not
//! require a conventional control-flow terminator. Consequently, the absence
//! of a terminator is representable. Whether a terminator is required depends
//! on the region kind, operation schema, and applicable dialect contract.
//!
//! ## Scalability and determinism
//!
//! There are no universal limits on the number of block arguments or
//! operation references. Storage is bounded only by available resources,
//! Rust collection representational limits, and explicit caller budgets.
//!
//! This module does not claim to process literally infinite input. It does
//! not impose arbitrary hardware-size ceilings or convert identity values
//! into collection indices.
//!
//! Argument and operation order is semantically preserved. No unordered-map
//! iteration, pointer identity, or host address contributes to block meaning.
//!
//! ## Error behavior
//!
//! Constructors reject duplicate block-argument identities, duplicate
//! operation references, and a terminator that is not the final operation
//! in the block. Invalid references that require access to the enclosing
//! graph are reported by graph-wide verification, not guessed locally.
//!
//! ## Completion criteria
//!
//! - All public fields remain encapsulated behind documented accessors.
//! - IDs reuse the canonical HDLIR identity types.
//! - Local invariants are checked at construction and revalidated explicitly.
//! - Operation order is preserved.
//! - Empty and unterminated blocks remain representable where their owning
//!   dialect permits them.
//! - Focused tests cover duplicate identities, terminator placement,
//!   ordering, empty blocks, and large identity values.
//! - Integration tests cover regions, operation storage, and graph verification.

use std::collections::BTreeSet;

use crate::hdl::hdlir::errors::{HdlirError, HdlirResult};
use crate::hdl::hdlir::ids::{
    HdlirBlockId, HdlirOperationId, HdlirTypeId, HdlirValueId,
};

/// A typed value declaration supplied to a block.
///
/// A block argument represents a value entering a block through the
/// enclosing IR's dataflow, structural, or control-flow contract. The
/// argument's identity and type must be unique and resolvable in the
/// enclosing graph.
///
/// The argument's position is its ordinal in [`HdlirBlock::arguments`].
/// It is not encoded in the value identity.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct HdlirBlockArgument {
    id: HdlirValueId,
    type_id: HdlirTypeId,
}

impl HdlirBlockArgument {
    /// Construct a block argument declaration.
    ///
    /// This constructor checks neither type registration nor uniqueness in
    /// an enclosing graph. Those checks require the relevant graph and
    /// type registry.
    #[must_use]
    pub const fn new(id: HdlirValueId, type_id: HdlirTypeId) -> Self {
        Self { id, type_id }
    }

    /// Return the value identity assigned to this argument.
    #[must_use]
    pub const fn id(self) -> HdlirValueId {
        self.id
    }

    /// Return the referenced HDLIR type identity.
    #[must_use]
    pub const fn type_id(self) -> HdlirTypeId {
        self.type_id
    }
}

/// An ordered block in the HDLIR graph.
///
/// `HdlirBlock` stores declarations and references, not copies of the
/// referenced operations. Its identity is independent of its position in
/// any region or graph storage collection.
///
/// The block is intentionally neutral about the interpretation of its
/// operations. A structural region, behavioral process, and control-flow
/// region may impose different rules through their registered contracts.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HdlirBlock {
    id: HdlirBlockId,
    arguments: Vec<HdlirBlockArgument>,
    operations: Vec<HdlirOperationId>,
    terminator: Option<HdlirOperationId>,
}

impl HdlirBlock {
    /// Construct a block without declaring a terminator.
    ///
    /// The supplied argument and operation order is preserved. The block
    /// may be empty, and it may omit a terminator when the enclosing
    /// region's contract permits that representation.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    ///
    /// - Two block arguments have the same value identity.
    /// - An operation identity occurs more than once in the block.
    pub fn new(
        id: HdlirBlockId,
        arguments: Vec<HdlirBlockArgument>,
        operations: Vec<HdlirOperationId>,
    ) -> HdlirResult<Self> {
        Self::with_optional_terminator(id, arguments, operations, None)
    }

    /// Construct a block with an optional terminator operation.
    ///
    /// When a terminator is supplied, its identity must appear exactly once
    /// in `operations` and must identify the final operation. The caller
    /// remains responsible for ensuring that the operation is registered
    /// with terminator semantics appropriate to the enclosing region.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    ///
    /// - Two block arguments have the same value identity.
    /// - An operation identity occurs more than once in the block.
    /// - The terminator is absent from the operation list.
    /// - The terminator is not the final operation.
    pub fn with_terminator(
        id: HdlirBlockId,
        arguments: Vec<HdlirBlockArgument>,
        operations: Vec<HdlirOperationId>,
        terminator: HdlirOperationId,
    ) -> HdlirResult<Self> {
        Self::with_optional_terminator(
            id,
            arguments,
            operations,
            Some(terminator),
        )
    }

    /// Construct a block with an optional terminator reference.
    ///
    /// This is the common implementation path for checked construction.
    /// Terminator legality and operation existence are graph-wide concerns;
    /// this constructor validates only the identities and ordering that
    /// can be checked from the block's own data.
    ///
    /// # Errors
    ///
    /// Returns an error when a local block invariant is violated.
    pub fn with_optional_terminator(
        id: HdlirBlockId,
        arguments: Vec<HdlirBlockArgument>,
        operations: Vec<HdlirOperationId>,
        terminator: Option<HdlirOperationId>,
    ) -> HdlirResult<Self> {
        let block = Self {
            id,
            arguments,
            operations,
            terminator,
        };

        block.validate_structure()?;
        Ok(block)
    }

    /// Return this block's identity.
    #[must_use]
    pub const fn id(&self) -> HdlirBlockId {
        self.id
    }

    /// Return block arguments in declaration order.
    ///
    /// The returned slice is read-only. Argument order is preserved because
    /// its position may be meaningful to the enclosing region or dialect.
    #[must_use]
    pub fn arguments(&self) -> &[HdlirBlockArgument] {
        &self.arguments
    }

    /// Return operation references in their declared execution or
    /// representation order.
    ///
    /// The meaning of this order is determined by the owning dialect.
    /// The block itself does not reorder operations or infer dependencies.
    #[must_use]
    pub fn operations(&self) -> &[HdlirOperationId] {
        &self.operations
    }

    /// Return the terminator operation identity, if one is declared.
    ///
    /// The returned ID is also the final element of [`Self::operations`].
    /// Its registered semantics must be checked by the schema-aware
    /// verifier.
    #[must_use]
    pub const fn terminator(&self) -> Option<HdlirOperationId> {
        self.terminator
    }

    /// Return the number of block arguments.
    #[must_use]
    pub fn argument_count(&self) -> usize {
        self.arguments.len()
    }

    /// Return the number of operation references.
    #[must_use]
    pub fn operation_count(&self) -> usize {
        self.operations.len()
    }

    /// Return whether the block has no arguments.
    #[must_use]
    pub fn has_no_arguments(&self) -> bool {
        self.arguments.is_empty()
    }

    /// Return whether the block has no operation references.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.operations.is_empty()
    }

    /// Return whether the block declares a terminator.
    #[must_use]
    pub const fn has_terminator(&self) -> bool {
        self.terminator.is_some()
    }

    /// Check whether the block declares an argument with the given value ID.
    #[must_use]
    pub fn contains_argument(&self, value_id: HdlirValueId) -> bool {
        self.arguments.iter().any(|argument| argument.id() == value_id)
    }

    /// Find the declaration ordinal of a block argument.
    ///
    /// The returned number is the argument's position within this block.
    /// It is not a global value ID, graph index, or physical position.
    #[must_use]
    pub fn argument_position(&self, value_id: HdlirValueId) -> Option<usize> {
        self.arguments
            .iter()
            .position(|argument| argument.id() == value_id)
    }

    /// Check whether this block references the specified operation.
    #[must_use]
    pub fn contains_operation(&self, operation_id: HdlirOperationId) -> bool {
        self.operations.contains(&operation_id)
    }

    /// Find the position of an operation reference.
    ///
    /// The result is an ordinal in this block's ordered operation list,
    /// not the operation's identity or a physical hardware location.
    #[must_use]
    pub fn operation_position(
        &self,
        operation_id: HdlirOperationId,
    ) -> Option<usize> {
        self.operations
            .iter()
            .position(|candidate| *candidate == operation_id)
    }

    /// Check local invariants that do not require the enclosing graph.
    ///
    /// This method does not prove that:
    ///
    /// - Operation references resolve to existing operation definitions.
    /// - An operation belongs exclusively to this block.
    /// - Argument IDs do not collide with values defined elsewhere.
    /// - Argument type IDs exist in the type registry.
    /// - The declared terminator has legal terminator semantics.
    /// - This block belongs to a particular region.
    /// - Control-flow, process, or hardware semantics are valid.
    ///
    /// Those checks belong to the enclosing graph and its verification
    /// subsystem.
    ///
    /// # Errors
    ///
    /// Returns an error if argument IDs or operation references are
    /// duplicated, or if the optional terminator is not the final operation.
    pub fn validate_structure(&self) -> HdlirResult<()> {
        let mut argument_ids = BTreeSet::new();

        for argument in &self.arguments {
            if !argument_ids.insert(argument.id()) {
                return Err(HdlirError::DuplicateEntity {
                    entity_kind: "HDLIR block argument",
                    identity: argument.id().to_string(),
                    scope: Some(self.id.to_string()),
                });
            }
        }

        let mut operation_ids = BTreeSet::new();

        for operation_id in &self.operations {
            if !operation_ids.insert(*operation_id) {
                return Err(HdlirError::DuplicateEntity {
                    entity_kind: "HDLIR block operation reference",
                    identity: operation_id.to_string(),
                    scope: Some(self.id.to_string()),
                });
            }
        }

        if let Some(terminator_id) = self.terminator {
            match self.operations.last() {
                Some(last_id) if *last_id == terminator_id => {}
                Some(_) => {
                    return Err(HdlirError::invalid_invariant(
                        "block.terminator_is_last",
                        format!(
                            "terminator operation {terminator_id} is not the \
                             final operation in block {}",
                            self.id
                        ),
                    ));
                }
                None => {
                    return Err(HdlirError::invalid_invariant(
                        "block.terminator_exists",
                        format!(
                            "block {} declares terminator {terminator_id} \
                             but contains no operations",
                            self.id
                        ),
                    ));
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{HdlirBlock, HdlirBlockArgument};
    use crate::hdl::hdlir::errors::HdlirError;
    use crate::hdl::hdlir::ids::{
        HdlirBlockId, HdlirOperationId, HdlirTypeId, HdlirValueId,
    };

    fn block_id(raw: u128) -> HdlirBlockId {
        HdlirBlockId::new(raw).expect("non-zero block ID")
    }

    fn operation_id(raw: u128) -> HdlirOperationId {
        HdlirOperationId::new(raw).expect("non-zero operation ID")
    }

    fn value_id(raw: u128) -> HdlirValueId {
        HdlirValueId::new(raw).expect("non-zero value ID")
    }

    fn type_id(raw: u128) -> HdlirTypeId {
        HdlirTypeId::new(raw).expect("non-zero type ID")
    }

    fn argument(value: u128, ty: u128) -> HdlirBlockArgument {
        HdlirBlockArgument::new(value_id(value), type_id(ty))
    }

    #[test]
    fn preserves_argument_and_operation_order() {
        let block = HdlirBlock::new(
            block_id(1),
            vec![argument(8, 2), argument(3, 1)],
            vec![operation_id(90), operation_id(7), operation_id(42)],
        )
        .expect("valid block");

        assert_eq!(
            block.arguments(),
            &[argument(8, 2), argument(3, 1)]
        );
        assert_eq!(
            block.operations(),
            &[operation_id(90), operation_id(7), operation_id(42)]
        );
        assert_eq!(block.argument_position(value_id(3)), Some(1));
        assert_eq!(block.operation_position(operation_id(7)), Some(1));
    }

    #[test]
    fn accepts_an_empty_block_without_a_terminator() {
        let block =
            HdlirBlock::new(block_id(1), Vec::new(), Vec::new())
                .expect("empty block is structurally representable");

        assert!(block.is_empty());
        assert!(block.has_no_arguments());
        assert!(!block.has_terminator());
        assert_eq!(block.operation_count(), 0);
        assert_eq!(block.argument_count(), 0);
        assert!(block.validate_structure().is_ok());
    }

    #[test]
    fn accepts_arguments_without_operations() {
        let block = HdlirBlock::new(
            block_id(1),
            vec![argument(4, 9)],
            Vec::new(),
        )
        .expect("arguments-only block is representable");

        assert_eq!(block.argument_count(), 1);
        assert!(block.is_empty());
        assert!(block.contains_argument(value_id(4)));
        assert!(block.validate_structure().is_ok());
    }

    #[test]
    fn rejects_duplicate_argument_ids() {
        let result = HdlirBlock::new(
            block_id(1),
            vec![argument(5, 1), argument(5, 2)],
            Vec::new(),
        );

        assert!(matches!(
            result,
            Err(HdlirError::DuplicateEntity {
                entity_kind: "HDLIR block argument",
                ..
            })
        ));
    }

    #[test]
    fn permits_different_arguments_to_share_a_type() {
        let block = HdlirBlock::new(
            block_id(1),
            vec![argument(5, 2), argument(6, 2)],
            Vec::new(),
        )
        .expect("different values may have the same type");

        assert_eq!(block.arguments()[0].type_id(), type_id(2));
        assert_eq!(block.arguments()[1].type_id(), type_id(2));
    }

    #[test]
    fn rejects_duplicate_operation_references() {
        let result = HdlirBlock::new(
            block_id(1),
            Vec::new(),
            vec![operation_id(3), operation_id(3)],
        );

        assert!(matches!(
            result,
            Err(HdlirError::DuplicateEntity {
                entity_kind: "HDLIR block operation reference",
                ..
            })
        ));
    }

    #[test]
    fn accepts_a_terminator_that_is_the_final_operation() {
        let block = HdlirBlock::with_terminator(
            block_id(1),
            Vec::new(),
            vec![operation_id(2), operation_id(8)],
            operation_id(8),
        )
        .expect("final operation is the declared terminator");

        assert_eq!(block.terminator(), Some(operation_id(8)));
        assert!(block.has_terminator());
        assert!(block.validate_structure().is_ok());
    }

    #[test]
    fn rejects_a_terminator_that_is_not_the_final_operation() {
        let result = HdlirBlock::with_terminator(
            block_id(1),
            Vec::new(),
            vec![operation_id(8), operation_id(2)],
            operation_id(8),
        );

        assert!(matches!(
            result,
            Err(HdlirError::InvalidInvariant {
                invariant: "block.terminator_is_last",
                ..
            })
        ));
    }

    #[test]
    fn rejects_a_terminator_not_present_in_the_operation_list() {
        let result = HdlirBlock::with_terminator(
            block_id(1),
            Vec::new(),
            vec![operation_id(2)],
            operation_id(9),
        );

        assert!(matches!(
            result,
            Err(HdlirError::InvalidInvariant {
                invariant: "block.terminator_is_last",
                ..
            })
        ));
    }

    #[test]
    fn rejects_a_terminator_on_an_empty_block() {
        let result = HdlirBlock::with_terminator(
            block_id(1),
            Vec::new(),
            Vec::new(),
            operation_id(9),
        );

        assert!(matches!(
            result,
            Err(HdlirError::InvalidInvariant {
                invariant: "block.terminator_exists",
                ..
            })
        ));
    }

    #[test]
    fn lookup_distinguishes_identity_from_position() {
        let block = HdlirBlock::new(
            block_id(1),
            vec![argument(80, 1), argument(7, 1)],
            vec![operation_id(90), operation_id(7)],
        )
        .expect("valid block");

        assert!(block.contains_argument(value_id(80)));
        assert!(!block.contains_argument(value_id(81)));
        assert!(block.contains_operation(operation_id(90)));
        assert!(!block.contains_operation(operation_id(91)));
        assert_eq!(block.argument_position(value_id(80)), Some(0));
        assert_eq!(block.operation_position(operation_id(90)), Some(0));
    }

    #[test]
    fn supports_large_nonzero_typed_identities() {
        let largest = u128::MAX;

        let block = HdlirBlock::new(
            block_id(largest),
            vec![argument(largest, largest)],
            vec![operation_id(largest)],
        )
        .expect("maximum non-zero IDs are representable");

        assert_eq!(block.id().get(), largest);
        assert_eq!(block.arguments()[0].id().get(), largest);
        assert_eq!(block.arguments()[0].type_id().get(), largest);
        assert_eq!(block.operations()[0].get(), largest);
    }

    #[test]
    fn revalidation_checks_local_structure() {
        let block = HdlirBlock::new(
            block_id(1),
            vec![argument(2, 1)],
            vec![operation_id(3)],
        )
        .expect("valid block");

        assert!(block.validate_structure().is_ok());
    }
}
