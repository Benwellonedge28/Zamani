
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR Region Builder
//!
//! Canonical path: `src/zuir/builder/regions.rs`
//! Rust edition: 2021
//! Minimum intended Rust version: 1.97 or later.
//! Safety: unsafe Rust is forbidden.
//!
//! PURPOSE
//!
//! Construct canonical ZUIR regions and establish consistent membership
//! between a region and the blocks supplied to it.
//!
//! AUTHORITY
//!
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/ids.rs`
//! - `src/zuir/ir/region.rs`
//! - `src/zuir/ir/block.rs`
//! - `src/zuir/ir/successor.rs`
//! - `src/zuir/ir/operation.rs`
//! - `src/zuir/verify/`, when the authoritative verifier is available
//! - `grammar/DESIGN.md`
//! - `grammar/Zamani.g4`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/scalability-model.md`
//!
//! OWNS
//!
//! - RegionBuilder and RegionBuildError.
//! - Safe construction of region membership from mutable block references.
//! - Validation of parent-region consistency at the builder boundary.
//! - Finalization of a locally valid ZuirRegion.
//!
//! DOES NOT OWN
//!
//! - Canonical region or block representation.
//! - Block storage in a program/module/function.
//! - ID allocation or global uniqueness.
//! - Operation storage or nested-operation semantics.
//! - CFG, SSA, dominance, type, effect, ownership, or dialect verification.
//! - Source parsing, AST construction, or semantic name resolution.
//! - Hardware placement, physical scheduling, or runtime execution.
//!
//! INTEGRATION CONTRACT
//!
//! A successful `add_block` adds the block's ID to the region and assigns
//! the block to that region if the block had no parent. A block already
//! assigned to a different region is rejected.
//!
//! The builder does not own the supplied block. The caller remains
//! responsible for storing it in the enclosing program/module data model.
//! The full verifier must establish that every referenced block exists,
//! that region membership and parent-region declarations agree, and that
//! all related control-flow references are valid.
//!
//! `finish` validates region-local invariants. It does not establish
//! whole-program validity.
//!
//! SCALABILITY
//!
//! No universal maximum is imposed on regions, blocks, captures, qubits,
//! operations, processors, devices, tensor dimensions, or graph size.
//! The builder uses the canonical region's dynamic collections and exposes
//! fallible reservation for block references. Actual construction remains
//! bounded by address space and available resources; literal infinity is
//! not promised.
//!
//! FREEZE CRITERIA
//!
//! Freeze only after the module is declared, repository-wide call sites
//! compile against this API, tests pass, and the full verifier's ownership
//! contract has been checked against the current implementation.

use std::error::Error;
use std::fmt;

use crate::zuir::ids::{BlockId, RegionId, ValueId};
use crate::zuir::ir::block::{BlockError, ZuirBlock};
use crate::zuir::ir::region::{
    CaptureMode, RegionError, RegionIsolation, RegionKind, RegionOwner, ZuirRegion,
};
use crate::zuir::schema::attributes::{AttributeKey, AttributeValue};

/// Result type for region-builder operations.
pub type RegionBuildResult<T> = Result<T, RegionBuildError>;

/// Errors raised while constructing a region.
///
/// These errors describe failures detectable at the builder boundary.
/// They do not replace diagnostics from whole-program verification.
#[derive(Debug)]
#[non_exhaustive]
pub enum RegionBuildError {
    /// The canonical region rejected an operation.
    Region(RegionError),

    /// A block rejected a local mutation or validation.
    Block(BlockError),

    /// A block is already owned by a different region.
    ParentRegionMismatch {
        block: BlockId,
        expected: RegionId,
        actual: RegionId,
    },
}

impl fmt::Display for RegionBuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Region(error) => write!(f, "region construction failed: {error}"),
            Self::Block(error) => write!(f, "block construction failed: {error}"),
            Self::ParentRegionMismatch {
                block,
                expected,
                actual,
            } => write!(
                f,
                "block {block} belongs to region {actual}, not requested region {expected}"
            ),
        }
    }
}

impl Error for RegionBuildError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Region(error) => Some(error),
            Self::Block(error) => Some(error),
            Self::ParentRegionMismatch { .. } => None,
        }
    }
}

impl From<RegionError> for RegionBuildError {
    fn from(error: RegionError) -> Self {
        Self::Region(error)
    }
}

impl From<BlockError> for RegionBuildError {
    fn from(error: BlockError) -> Self {
        Self::Block(error)
    }
}

/// Incrementally constructs a canonical ZUIR region.
///
/// The builder owns the region under construction, but it does not own
/// blocks. Blocks are stored by the enclosing IR container and referenced
/// by ID from the region.
///
/// A builder may temporarily contain no blocks. `finish` rejects an empty
/// region because the canonical `ZuirRegion::validate` contract requires
/// a complete region to contain at least one block.
#[derive(Debug, Clone)]
pub struct RegionBuilder {
    region: ZuirRegion,
}

impl RegionBuilder {
    /// Start constructing a region.
    ///
    /// The region remains incomplete until at least one valid block is
    /// added. The owner and ID are declarations; their global existence
    /// and uniqueness are checked by the enclosing IR verifier.
    #[must_use]
    pub fn new(
        id: RegionId,
        owner: RegionOwner,
        kind: RegionKind,
        isolation: RegionIsolation,
    ) -> Self {
        Self {
            region: ZuirRegion::new(id, owner, kind, isolation),
        }
    }

    /// Return this builder's region identity.
    #[must_use]
    pub const fn id(&self) -> RegionId {
        self.region.id()
    }

    /// Return the region's declared owner.
    #[must_use]
    pub const fn owner(&self) -> RegionOwner {
        self.region.owner()
    }

    /// Borrow the region under construction.
    ///
    /// The returned reference is read-only so callers cannot bypass the
    /// builder's block-parent consistency checks through this API.
    #[must_use]
    pub const fn region(&self) -> &ZuirRegion {
        &self.region
    }

    /// Return the number of blocks currently referenced by the region.
    #[must_use]
    pub fn block_count(&self) -> usize {
        self.region.block_count()
    }

    /// Reserve capacity for additional block references.
    ///
    /// This reports reservation failure where the underlying standard
    /// library collection supports fallible reservation. It does not set
    /// a semantic maximum on the region's size.
    pub fn try_reserve_blocks(
        &mut self,
        additional: usize,
    ) -> RegionBuildResult<()> {
        self.region.try_reserve_blocks(additional)?;
        Ok(())
    }

    /// Add a block to this region and establish its parent-region ID.
    ///
    /// The operation is ordered to avoid changing the block if the region
    /// rejects its ID. A block with no parent is assigned to this region;
    /// a block already assigned to this region is accepted only if it is
    /// not already present in the region's block list. A block assigned to
    /// another region is rejected.
    ///
    /// The block remains owned by the caller. The caller must store it in
    /// the enclosing IR and ensure it remains available for verification.
    pub fn add_block(
        &mut self,
        block: &mut ZuirBlock,
    ) -> RegionBuildResult<()> {
        let region_id = self.region.id();
        let block_id = block.id();

        if let Some(actual) = block.parent_region() {
            if actual != region_id {
                return Err(RegionBuildError::ParentRegionMismatch {
                    block: block_id,
                    expected: region_id,
                    actual,
                });
            }
        }

        // Canonical region validation catches duplicate block IDs and
        // allocation failures before the block's parent is changed.
        self.region.add_block(block_id)?;

        // This setter is infallible in the canonical block API.
        // No fallible builder operation follows this mutation.
        if block.parent_region().is_none() {
            block.set_parent_region(region_id);
        }

        Ok(())
    }

    /// Declare an enclosing-scope value capture.
    ///
    /// Capture visibility, type, lifetime, aliasing, ownership, and
    /// execution-domain legality remain the full verifier's responsibility.
    pub fn add_capture(
        &mut self,
        value: ValueId,
        mode: CaptureMode,
    ) -> RegionBuildResult<()> {
        self.region.add_capture(value, mode)?;
        Ok(())
    }

    /// Change the mode of a previously declared capture.
    ///
    /// Returns `Ok(false)` when the capture does not exist.
    pub fn set_capture_mode(
        &mut self,
        value: ValueId,
        mode: CaptureMode,
    ) -> RegionBuildResult<bool> {
        Ok(self.region.set_capture_mode(value, mode)?)
    }

    /// Remove a capture declaration.
    ///
    /// Returns `true` if the declaration existed.
    pub fn remove_capture(&mut self, value: ValueId) -> bool {
        self.region.remove_capture(value)
    }

    /// Set or replace a region attribute.
    ///
    /// Attribute-key validity is checked by the canonical region.
    /// Attribute schema, namespace authorization, and semantic meaning
    /// are verified at the appropriate schema/verifier boundary.
    pub fn set_attribute(
        &mut self,
        key: AttributeKey,
        value: AttributeValue,
    ) -> RegionBuildResult<Option<AttributeValue>> {
        Ok(self.region.set_attribute(key, value)?)
    }

    /// Change the region's structural interpretation.
    pub fn set_kind(
        &mut self,
        kind: RegionKind,
    ) -> RegionBuildResult<()> {
        self.region.set_kind(kind)?;
        Ok(())
    }

    /// Change the region's capture-isolation policy.
    ///
    /// The canonical region rejects changing to `Isolated` while captures
    /// remain. Captures must be explicitly removed first.
    pub fn set_isolation(
        &mut self,
        isolation: RegionIsolation,
    ) -> RegionBuildResult<()> {
        self.region.set_isolation(isolation)?;
        Ok(())
    }

    /// Validate the region without consuming the builder.
    ///
    /// This checks canonical region-local invariants only. It does not
    /// resolve the referenced block IDs or establish global validity.
    pub fn validate(&self) -> RegionBuildResult<()> {
        self.region.validate()?;
        Ok(())
    }

    /// Finalize and return the canonical region.
    ///
    /// A successful return means the region passed its local validation.
    /// The enclosing program must still pass whole-IR verification before
    /// optimization, lowering, serialization as a trusted artifact, or
    /// execution.
    pub fn finish(self) -> RegionBuildResult<ZuirRegion> {
        self.region.validate()?;
        Ok(self.region)
    }

    /// Return the region ID and its ordered block IDs without finalizing.
    ///
    /// Useful for diagnostics and staged assembly. The returned IDs do not
    /// prove that the blocks are present in the enclosing program.
    #[must_use]
    pub fn membership(&self) -> (RegionId, &[BlockId]) {
        (self.region.id(), self.region.blocks())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::zuir::ids::{BlockId, FunctionId, RegionId};
    use crate::zuir::ir::region::{RegionIsolation, RegionKind, RegionOwner};

    fn region_id(value: u128) -> RegionId {
        RegionId::new(value).expect("test region ID must be nonzero")
    }

    fn block_id(value: u128) -> BlockId {
        BlockId::new(value).expect("test block ID must be nonzero")
    }

    fn function_id(value: u128) -> FunctionId {
        FunctionId::new(value).expect("test function ID must be nonzero")
    }

    fn builder() -> RegionBuilder {
        RegionBuilder::new(
            region_id(1),
            RegionOwner::Function(function_id(2)),
            RegionKind::Structured,
            RegionIsolation::CapturesAllowed,
        )
    }

    #[test]
    fn adds_block_and_sets_parent_region() {
        let mut builder = builder();
        let mut block = ZuirBlock::new(block_id(3));

        builder.add_block(&mut block).expect("block should be added");

        assert_eq!(block.parent_region(), Some(region_id(1)));
        assert_eq!(builder.region().blocks(), &[block_id(3)]);
        assert!(builder.validate().is_ok());
    }

    #[test]
    fn rejects_block_owned_by_another_region_without_mutating_it() {
        let mut builder = builder();
        let mut block = ZuirBlock::in_region(block_id(3), region_id(99));

        let result = builder.add_block(&mut block);

        assert!(matches!(
            result,
            Err(RegionBuildError::ParentRegionMismatch {
                block: _,
                expected: _,
                actual: _
            })
        ));
        assert_eq!(block.parent_region(), Some(region_id(99)));
        assert_eq!(builder.block_count(), 0);
    }

    #[test]
    fn rejects_duplicate_block_membership() {
        let mut builder = builder();
        let mut block = ZuirBlock::new(block_id(3));

        builder.add_block(&mut block).expect("first insertion");

        assert!(matches!(
            builder.add_block(&mut block),
            Err(RegionBuildError::Region(
                RegionError::DuplicateBlock(_)
            ))
        ));
        assert_eq!(builder.block_count(), 1);
    }

    #[test]
    fn rejects_finishing_an_empty_region() {
        let result = builder().finish();

        assert!(matches!(
            result,
            Err(RegionBuildError::Region(RegionError::EmptyRegion))
        ));
    }

    #[test]
    fn finishes_a_region_after_a_block_is_added() {
        let mut builder = builder();
        let mut block = ZuirBlock::new(block_id(3));

        builder.add_block(&mut block).expect("valid block");

        let region = builder.finish().expect("valid region");

        assert_eq!(region.id(), region_id(1));
        assert_eq!(region.blocks(), &[block_id(3)]);
    }

    #[test]
    fn reservation_does_not_change_region_semantics() {
        let mut builder = builder();

        builder
            .try_reserve_blocks(16)
            .expect("small reservation should succeed");

        assert_eq!(builder.block_count(), 0);
    }
}
