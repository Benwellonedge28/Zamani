
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani HDLIR — Regions
//!
//! Path: `src/hdl/hdlir/ir/regions.rs`
//! Edition: Rust 2021.
//! Minimum supported Rust version: Rust 1.97.
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Represent ordered, nested structural regions in the hardware-domain IR.
//! A region references blocks owned by the enclosing HDLIR graph; it does
//! not duplicate block contents or define their execution semantics.
//!
//! ## Authority
//!
//! This module follows:
//!
//! - `grammar/DESIGN.md`
//! - `grammar/spec/hdl.md`
//! - `grammar/specification/semantic-model.md`
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/hdl/hdlir/HDLIR-SPEC.md`, when established
//! - `src/hdl/hdlir/ids.rs`
//! - `src/hdl/hdlir/errors.rs`
//! - `src/hdl/hdlir/ir/operations.rs`
//!
//! Grammar defines syntax; the frontend owns source-level meaning;
//! ZUIR owns the universal computational representation; HDLIR owns
//! hardware-domain region structure.
//!
//! ## Owns
//!
//! - Region identity.
//! - Region kind metadata.
//! - Ordered block references.
//! - An optional entry-block reference.
//! - Local region invariants and checked construction.
//!
//! ## Does not own
//!
//! - Block contents, block arguments, operations, or terminators.
//! - Operation-to-region ownership across the complete graph.
//! - Source ASTs, semantic symbols, source spans, or source diagnostics.
//! - Control-flow legality, process evaluation, clock behavior, or state.
//! - Physical hardware allocation, placement, routing, or synthesis.
//!
//! ## Integration contract
//!
//! `HdlirOperation` stores `HdlirRegionId` references. The enclosing graph
//! must store each region exactly once in its identity namespace and verify
//! that operation references resolve to existing regions.
//!
//! `blocks.rs` owns `HdlirBlockId` definitions and block contents. A region
//! references those IDs without depending on the concrete block type.
//! This avoids a module dependency cycle and allows block implementation
//! to evolve independently.
//!
//! Whole-graph verification must establish:
//!
//! - Every referenced block exists.
//! - A block belongs to the intended region and is not illegally shared.
//! - Entry blocks satisfy the owning region's dialect contract.
//! - Region ownership agrees with operation region references.
//! - Region nesting is valid and does not contain forbidden cycles.
//! - Required terminators, block arguments, and control-flow edges are valid.
//! - Region kinds are compatible with the registered operation definition.
//!
//! Local construction alone does not prove any of these graph-wide facts.
//!
//! ## Scalability and determinism
//!
//! There are no universal region-count or block-count limits.
//! Storage grows subject to available memory and representable collection
//! sizes. The caller may enforce operational budgets at higher layers.
//!
//! Block ordering is semantically preserved. The implementation does not
//! sort blocks, infer physical layout, or use hash iteration order.
//! Identity values are references, not vector indices.
//!
//! ## Completion criteria
//!
//! Local invariants and negative cases must be tested. Integration is not
//! complete until `ir/mod.rs`, `blocks.rs`, `operations.rs`, the graph
//! owner, and whole-graph verification agree on ownership and references.

use std::collections::BTreeSet;

use crate::hdl::hdlir::errors::{HdlirError, HdlirResult};
use crate::hdl::hdlir::ids::{HdlirBlockId, HdlirRegionId};
use crate::hdl::hdlir::schema::registry::QualifiedName;

/// The declared structural role of a region.
///
/// These variants describe broad structural categories, not a closed set
/// of legal operations. The owning operation's registered schema determines
/// which region kinds and contents are valid in a particular context.
///
/// `Custom` permits an extension to identify a region role without adding
/// a new variant to this enum. Its semantics remain owned by the extension.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum HdlirRegionKind {
    /// A region representing structural composition or a graph-like body.
    Structural,

    /// A region whose blocks represent explicit control flow.
    ///
    /// This label does not itself validate branches, terminators, or cycles.
    ControlFlow,

    /// A region containing a behavioral hardware process.
    ///
    /// Clocking, sensitivity, assignment, and process semantics are verified
    /// by the corresponding process and dialect contracts.
    Process,

    /// A region containing initialization behavior.
    ///
    /// Whether initialization is supported, synthesizable, or executable
    /// depends on the owning operation and target-independent semantics.
    Initialization,

    /// An extension-defined region kind.
    ///
    /// The qualified name must be registered or otherwise accepted by the
    /// applicable extension and compatibility policy before its semantics
    /// can be interpreted.
    Custom(QualifiedName),
}

/// A hardware-domain region containing an ordered sequence of blocks.
///
/// `HdlirRegion` is a structural record. Its block references are resolved
/// against the enclosing graph; this type does not own or clone block
/// definitions.
///
/// Regions may be nested through operations that reference other regions.
/// The complete graph owner is responsible for establishing ownership and
/// checking nesting invariants.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HdlirRegion {
    id: HdlirRegionId,
    kind: HdlirRegionKind,
    blocks: Vec<HdlirBlockId>,
    entry_block: Option<HdlirBlockId>,
}

impl HdlirRegion {
    /// Construct a region with an ordered list of block references.
    ///
    /// The constructor validates local identity and membership invariants.
    /// It does not require the region to contain blocks: some region
    /// contracts permit an empty body, while others require a non-empty
    /// body. That decision belongs to the owning operation's schema.
    ///
    /// The order of `blocks` is preserved exactly as provided.
    ///
    /// # Errors
    ///
    /// Returns an error if a block identity appears more than once.
    /// The region ID and block IDs are already non-zero typed identities,
    /// so this constructor does not need to revalidate their numeric values.
    pub fn new(
        id: HdlirRegionId,
        kind: HdlirRegionKind,
        blocks: Vec<HdlirBlockId>,
    ) -> HdlirResult<Self> {
        Self::with_entry_block(id, kind, blocks, None)
    }

    /// Construct a region with an optional entry-block reference.
    ///
    /// If `entry_block` is present, it must occur in `blocks`. This is a
    /// local membership check; existence and ownership are verified against
    /// the enclosing graph by the whole-graph verifier.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    ///
    /// - A block reference occurs more than once.
    /// - The entry block is not a member of this region.
    pub fn with_entry_block(
        id: HdlirRegionId,
        kind: HdlirRegionKind,
        blocks: Vec<HdlirBlockId>,
        entry_block: Option<HdlirBlockId>,
    ) -> HdlirResult<Self> {
        let mut seen = BTreeSet::new();

        for block_id in &blocks {
            if !seen.insert(*block_id) {
                return Err(HdlirError::DuplicateEntity {
                    entity_kind: "HDLIR region block reference",
                    identity: block_id.to_string(),
                    scope: Some(id.to_string()),
                });
            }
        }

        if let Some(entry_id) = entry_block {
            if !seen.contains(&entry_id) {
                return Err(HdlirError::invalid_input(
                    "region.entry_block",
                    format!(
                        "entry block {entry_id} is not a member of region {id}"
                    ),
                ));
            }
        }

        Ok(Self {
            id,
            kind,
            blocks,
            entry_block,
        })
    }

    /// Return this region's identity.
    #[must_use]
    pub const fn id(&self) -> HdlirRegionId {
        self.id
    }

    /// Return the declared structural role of this region.
    #[must_use]
    pub const fn kind(&self) -> &HdlirRegionKind {
        &self.kind
    }

    /// Return block references in their declared order.
    ///
    /// These are identities, not indices into an enclosing graph's storage.
    #[must_use]
    pub fn blocks(&self) -> &[HdlirBlockId] {
        &self.blocks
    }

    /// Return the entry block, if this region declares one.
    #[must_use]
    pub const fn entry_block(&self) -> Option<HdlirBlockId> {
        self.entry_block
    }

    /// Return the number of block references in this region.
    #[must_use]
    pub fn block_count(&self) -> usize {
        self.blocks.len()
    }

    /// Return whether this region has no block references.
    ///
    /// An empty region is not inherently invalid. Its legality is determined
    /// by the owning operation's schema and the applicable dialect contract.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.blocks.is_empty()
    }

    /// Check invariants that can be established from this region alone.
    ///
    /// This method intentionally does not verify block existence, exclusive
    /// ownership, operation attachment, nesting, or terminator semantics.
    ///
    /// # Errors
    ///
    /// Returns an error if a block is referenced more than once or if the
    /// entry block is not included in the region's block list.
    pub fn validate_structure(&self) -> HdlirResult<()> {
        let mut seen = BTreeSet::new();

        for block_id in &self.blocks {
            if !seen.insert(*block_id) {
                return Err(HdlirError::invalid_invariant(
                    "region.unique_blocks",
                    format!(
                        "block identity {block_id} occurs more than once \
                         in region {}",
                        self.id
                    ),
                ));
            }
        }

        if let Some(entry_id) = self.entry_block {
            if !seen.contains(&entry_id) {
                return Err(HdlirError::invalid_invariant(
                    "region.entry_membership",
                    format!(
                        "entry block {entry_id} is not a member of region {}",
                        self.id
                    ),
                ));
            }
        }

        if let HdlirRegionKind::Custom(name) = &self.kind {
            // QualifiedName validates its own spelling at construction.
            // Registration and extension compatibility remain graph/schema
            // responsibilities, so no guessed registry lookup is performed.
            if name.to_string().trim().is_empty() {
                return Err(HdlirError::invalid_invariant(
                    "region.custom_kind",
                    "custom region kind has an empty qualified name",
                ));
            }
        }

        Ok(())
    }

    /// Check whether the region contains the specified block reference.
    #[must_use]
    pub fn contains_block(&self, block_id: HdlirBlockId) -> bool {
        self.blocks.contains(&block_id)
    }

    /// Return the position of a block reference, if present.
    ///
    /// The position is only the region-local ordinal in the ordered list.
    /// It is not the block's identity or a physical hardware position.
    #[must_use]
    pub fn position_of(&self, block_id: HdlirBlockId) -> Option<usize> {
        self.blocks.iter().position(|candidate| *candidate == block_id)
    }
}

#[cfg(test)]
mod tests {
    use super::{HdlirRegion, HdlirRegionKind};
    use crate::hdl::hdlir::errors::HdlirError;
    use crate::hdl::hdlir::ids::{HdlirBlockId, HdlirRegionId};
    use crate::hdl::hdlir::schema::registry::QualifiedName;

    fn region_id(raw: u128) -> HdlirRegionId {
        HdlirRegionId::new(raw).expect("non-zero region identity")
    }

    fn block_id(raw: u128) -> HdlirBlockId {
        HdlirBlockId::new(raw).expect("non-zero block identity")
    }

    #[test]
    fn preserves_block_order() {
        let region = HdlirRegion::new(
            region_id(1),
            HdlirRegionKind::Structural,
            vec![block_id(8), block_id(3), block_id(5)],
        )
        .expect("valid region");

        assert_eq!(
            region.blocks(),
            &[block_id(8), block_id(3), block_id(5)]
        );
        assert_eq!(region.position_of(block_id(3)), Some(1));
        assert!(region.validate_structure().is_ok());
    }

    #[test]
    fn accepts_an_empty_region() {
        let region = HdlirRegion::new(
            region_id(1),
            HdlirRegionKind::Structural,
            Vec::new(),
        )
        .expect("empty region is structurally representable");

        assert!(region.is_empty());
        assert_eq!(region.block_count(), 0);
        assert_eq!(region.entry_block(), None);
        assert!(region.validate_structure().is_ok());
    }

    #[test]
    fn rejects_duplicate_block_references() {
        let result = HdlirRegion::new(
            region_id(1),
            HdlirRegionKind::Structural,
            vec![block_id(2), block_id(2)],
        );

        assert!(matches!(
            result,
            Err(HdlirError::DuplicateEntity {
                entity_kind: "HDLIR region block reference",
                ..
            })
        ));
    }

    #[test]
    fn rejects_entry_block_outside_region() {
        let result = HdlirRegion::with_entry_block(
            region_id(1),
            HdlirRegionKind::ControlFlow,
            vec![block_id(2)],
            Some(block_id(3)),
        );

        assert!(matches!(
            result,
            Err(HdlirError::InvalidInput {
                field: "region.entry_block",
                ..
            })
        ));
    }

    #[test]
    fn accepts_entry_block_that_is_a_member() {
        let region = HdlirRegion::with_entry_block(
            region_id(1),
            HdlirRegionKind::ControlFlow,
            vec![block_id(4), block_id(2)],
            Some(block_id(2)),
        )
        .expect("entry block belongs to region");

        assert_eq!(region.entry_block(), Some(block_id(2)));
        assert!(region.contains_block(block_id(2)));
        assert!(region.validate_structure().is_ok());
    }

    #[test]
    fn supports_extension_defined_region_kinds() {
        let kind = HdlirRegionKind::Custom(
            QualifiedName::parse("vendor.pipeline").expect("valid qualified name"),
        );

        let region =
            HdlirRegion::new(region_id(1), kind, vec![block_id(1)])
                .expect("custom region kind is representable");

        assert!(region.validate_structure().is_ok());
    }

    #[test]
    fn block_membership_is_not_a_block_index() {
        let region = HdlirRegion::new(
            region_id(1),
            HdlirRegionKind::Structural,
            vec![block_id(90), block_id(7)],
        )
        .expect("valid region");

        assert!(region.contains_block(block_id(90)));
        assert_eq!(region.position_of(block_id(90)), Some(0));
        assert_eq!(region.position_of(block_id(7)), Some(1));
        assert_eq!(region.position_of(block_id(8)), None);
    }

    #[test]
    fn region_identity_is_preserved() {
        let region = HdlirRegion::new(
            region_id(42),
            HdlirRegionKind::Process,
            Vec::new(),
        )
        .expect("valid region");

        assert_eq!(region.id(), region_id(42));
    }
}
