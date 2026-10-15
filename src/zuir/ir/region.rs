
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR Region
//!
//! Canonical path: `src/zuir/ir/region.rs`
//! Rust edition: 2021
//! Minimum Rust baseline: Rust 1.97 or later.
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Represent a target-independent nested computation region.
//!
//! A region groups basic blocks, describes how it is owned and interpreted,
//! records explicitly permitted captures from an enclosing scope, and stores
//! region-scoped attributes. It does not own block contents or operations.
//!
//! ## AUTHORITY
//!
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/zuir/ids.rs`
//! - `src/zuir/schema/attributes.rs`
//! - `src/zuir/schema/types.rs`
//! - `src/zuir/ir/function.rs`
//! - `grammar/DESIGN.md`
//! - `grammar/Zamani.g4`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/specification/syntax-model.md`
//!
//! ## OWNS
//!
//! - Region identity and declared owner.
//! - Region structural interpretation.
//! - Ordered block membership.
//! - Explicit capture declarations and capture modes.
//! - Region-local attributes.
//! - Local validation and deterministic traversal.
//!
//! ## DOES_NOT_OWN
//!
//! - Parsing, AST construction, or source-name resolution.
//! - Allocation or global uniqueness of IDs.
//! - Block arguments, operations, terminators, or successor edges.
//! - SSA dominance, def-use analysis, or full control-flow verification.
//! - Type inference, effect inference, or ownership proof.
//! - Dialect registration or extension execution.
//! - Target layout, physical allocation, scheduling, or runtime execution.
//!
//! ## INTEGRATION CONTRACT
//!
//! `ir/function.rs` stores `RegionId` references in `FunctionBody`.
//! A function-owned region must identify that `FunctionId` as its owner.
//!
//! An operation that owns nested regions must identify its `OperationId` as
//! the owner. The full verifier must check that the operation actually
//! contains the referenced region and that ownership is reciprocal.
//!
//! `ir/block.rs` owns each block's arguments, ordered operations, and
//! terminator. This region stores only ordered `BlockId` references.
//!
//! `ir/operation.rs` owns operation contents and any operation-to-region
//! references. This file must not duplicate operation storage.
//!
//! `ids.rs` owns all identifier types. `schema/attributes.rs` owns
//! `AttributeKey` and `AttributeValue`; `schema/types.rs` owns `QualifiedName`.
//!
//! Capture references are external `ValueId`s. The full verifier must check
//! that each captured value exists, is visible from the enclosing scope, and
//! is legal to capture under the region's isolation and ownership rules.
//! This file cannot prove those cross-entity properties on its own.
//!
//! The grammar and semantic model own source syntax and source meaning.
//! This file accepts already-resolved IDs and never parses source constructs.
//!
//! ## SCALABILITY AND POCO-REAF
//!
//! Blocks, captures, and attributes use dynamically sized ordered collections.
//! No machine count, hardware capacity, register width, qubit count, tensor
//! rank, or universal region-size ceiling is introduced here.
//!
//! The implementation remains subject to available memory, address space,
//! allocator behavior, and explicitly configured compiler resource budgets.
//! Resource exhaustion must be reported by the layer that owns that budget;
//! it must not silently change region semantics.
//!
//! ## TRUST BOUNDARY
//!
//! Deserialization is not proof of validity. Call `validate()` after decoding
//! untrusted data and run full-program verification before optimization or
//! lowering. Local validation does not prove cross-region correctness.
//!
//! ## FREEZE CRITERIA
//!
//! - Region ownership is explicit and typed.
//! - Block membership is ordered and duplicate-free.
//! - Capture declarations are unique and explicit.
//! - Isolated regions cannot declare captures.
//! - Extension semantics use qualified names, not a closed operation list.
//! - Attributes are validated using the canonical schema API.
//! - Deserialized state can be revalidated.
//! - Tests cover normal, invalid, and deterministic behavior.
//! - No unsafe code or physical hardware assumptions are introduced.

use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use crate::zuir::ids::{BlockId, FunctionId, OperationId, RegionId, ValueId};
use crate::zuir::schema::attributes::{AttributeKey, AttributeValue};
use crate::zuir::schema::types::QualifiedName;

// =============================================================================
// Region ownership
// =============================================================================

/// Entity that owns a region.
///
/// Function bodies are owned by a function. Regions nested under an
/// operation are owned by that operation. The verifier must ensure the
/// declared owner and the enclosing entity's region references agree.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(tag = "kind", content = "id", rename_all = "snake_case")]
pub enum RegionOwner {
    /// A region forming part of a function body.
    Function(FunctionId),

    /// A region nested under an operation.
    Operation(OperationId),
}

// =============================================================================
// Region semantics
// =============================================================================

/// Broad structural interpretation of a region.
///
/// These variants describe general control/data organization. They do not
/// prescribe the meaning of any domain operation. New domain-specific
/// behavior belongs to registered dialect contracts.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(tag = "kind", content = "extension", rename_all = "snake_case")]
pub enum RegionKind {
    /// A region whose structure is interpreted by its owning construct.
    Structured,

    /// A region interpreted using explicit control-flow blocks and edges.
    ControlFlow,

    /// A region interpreted using data-dependency semantics.
    Dataflow,

    /// A region whose additional semantics are defined by a registered
    /// extension identified by a qualified name.
    Extension(QualifiedName),
}

impl RegionKind {
    /// Validate local invariants, including extension-name validity.
    pub fn validate(&self) -> Result<(), RegionError> {
        if let Self::Extension(name) = self {
            name.validate()
                .map_err(|error| RegionError::InvalidExtensionName(error.to_string()))?;
        }
        Ok(())
    }
}

/// Whether a region is permitted to capture values from an enclosing scope.
///
/// Isolation is a structural declaration, not proof that a region is pure
/// or free of external effects. Those properties belong to other contracts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RegionIsolation {
    /// Captures may be declared explicitly and verified by the full verifier.
    CapturesAllowed,

    /// The region must not capture values from an enclosing scope.
    ///
    /// Region-local block arguments and operation results are not captures.
    Isolated,
}

/// How a region declares its use of an enclosing value.
///
/// These modes express intent. The full ownership/effect verifier determines
/// whether a mode is legal for the value's actual type and enclosing scope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CaptureMode {
    /// Read the captured value without declaring mutation.
    Read,

    /// Read and potentially mutate the captured value, subject to verification.
    Write,

    /// Transfer the value into the region, subject to ownership verification.
    Move,

    /// Borrow the value immutably for the region's permitted lifetime.
    BorrowShared,

    /// Borrow the value mutably for the region's permitted lifetime.
    BorrowExclusive,
}

// =============================================================================
// Region errors
// =============================================================================

/// Errors for invariants owned directly by a region.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegionError {
    /// A complete region has no blocks.
    EmptyRegion,

    /// The same block is listed more than once.
    DuplicateBlock(BlockId),

    /// The same captured value was declared more than once.
    DuplicateCapture(ValueId),

    /// An isolated region attempted to declare an enclosing capture.
    CaptureInIsolatedRegion(ValueId),

    /// A region kind contains an invalid extension name.
    InvalidExtensionName(String),

    /// A region attribute key is invalid.
    InvalidAttributeKey(String),
}

impl fmt::Display for RegionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyRegion => {
                f.write_str("a complete ZUIR region must contain at least one block")
            }
            Self::DuplicateBlock(block) => {
                write!(f, "region contains block {block} more than once")
            }
            Self::DuplicateCapture(value) => {
                write!(f, "region captures value {value} more than once")
            }
            Self::CaptureInIsolatedRegion(value) => {
                write!(f, "isolated region cannot capture enclosing value {value}")
            }
            Self::InvalidExtensionName(reason) => {
                write!(f, "invalid region extension name: {reason}")
            }
            Self::InvalidAttributeKey(reason) => {
                write!(f, "invalid region attribute key: {reason}")
            }
        }
    }
}

impl std::error::Error for RegionError {}

// =============================================================================
// Canonical region
// =============================================================================

/// Canonical target-independent ZUIR region.
///
/// A region stores references to blocks rather than embedding their contents.
/// Block ordering is preserved because the IR contract may use it for
/// deterministic traversal and representation. Control-flow semantics must
/// be determined from the block terminators and successor contracts, not
/// inferred solely from this order.
///
/// Captures are keyed by `ValueId` so duplicate captures are rejected and
/// traversal is deterministic. Capture order is not represented as semantic
/// ordering by this type.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ZuirRegion {
    id: RegionId,
    owner: RegionOwner,
    kind: RegionKind,
    isolation: RegionIsolation,

    /// Ordered block references owned by this region.
    blocks: Vec<BlockId>,

    /// Explicit enclosing-scope captures and their declared access modes.
    captures: BTreeMap<ValueId, CaptureMode>,

    /// Namespaced, declarative attributes.
    attributes: BTreeMap<AttributeKey, AttributeValue>,
}

impl ZuirRegion {
    /// Create an empty region under construction.
    ///
    /// An empty region is an incomplete construction state: `validate()`
    /// rejects it until at least one block has been added. This permits
    /// builders to establish identities and references before all blocks
    /// have been assembled.
    #[must_use]
    pub fn new(
        id: RegionId,
        owner: RegionOwner,
        kind: RegionKind,
        isolation: RegionIsolation,
    ) -> Self {
        Self {
            id,
            owner,
            kind,
            isolation,
            blocks: Vec::new(),
            captures: BTreeMap::new(),
            attributes: BTreeMap::new(),
        }
    }

    /// Construct a region with its initial ordered block list.
    ///
    /// The block list must be nonempty and contain no duplicates.
    pub fn with_blocks(
        id: RegionId,
        owner: RegionOwner,
        kind: RegionKind,
        isolation: RegionIsolation,
        blocks: Vec<BlockId>,
    ) -> Result<Self, RegionError> {
        let mut region = Self::new(id, owner, kind, isolation);

        for block in blocks {
            region.add_block(block)?;
        }

        region.validate()?;
        Ok(region)
    }

    /// Return the region identity.
    #[must_use]
    pub const fn id(&self) -> RegionId {
        self.id
    }

    /// Return the declared owner.
    #[must_use]
    pub const fn owner(&self) -> RegionOwner {
        self.owner
    }

    /// Return the region's structural interpretation.
    #[must_use]
    pub const fn kind(&self) -> &RegionKind {
        &self.kind
    }

    /// Return the region's capture-isolation policy.
    #[must_use]
    pub const fn isolation(&self) -> RegionIsolation {
        self.isolation
    }

    /// Replace the region kind after validating its local invariants.
    pub fn set_kind(&mut self, kind: RegionKind) -> Result<(), RegionError> {
        kind.validate()?;
        self.kind = kind;
        Ok(())
    }

    /// Change the isolation policy.
    ///
    /// Switching to `Isolated` is rejected if captures already exist. The
    /// caller must remove those captures explicitly before changing policy.
    pub fn set_isolation(
        &mut self,
        isolation: RegionIsolation,
    ) -> Result<(), RegionError> {
        if isolation == RegionIsolation::Isolated {
            if let Some(value) = self.captures.keys().next().copied() {
                return Err(RegionError::CaptureInIsolatedRegion(value));
            }
        }

        self.isolation = isolation;
        Ok(())
    }

    /// Return whether the region currently contains any block references.
    ///
    /// This reports construction state only; it does not imply verification.
    #[must_use]
    pub fn has_blocks(&self) -> bool {
        !self.blocks.is_empty()
    }

    /// Return the number of block references.
    #[must_use]
    pub fn block_count(&self) -> usize {
        self.blocks.len()
    }

    /// Return block references in declared order.
    #[must_use]
    pub fn blocks(&self) -> &[BlockId] {
        &self.blocks
    }

    /// Test whether the region references a block.
    #[must_use]
    pub fn contains_block(&self, block: BlockId) -> bool {
        self.blocks.contains(&block)
    }

    /// Add a block reference at the end of the region.
    ///
    /// The block must exist and belong to this region when the complete IR
    /// is verified. This method enforces only local uniqueness.
    pub fn add_block(&mut self, block: BlockId) -> Result<(), RegionError> {
        if self.blocks.contains(&block) {
            return Err(RegionError::DuplicateBlock(block));
        }

        self.blocks.push(block);
        Ok(())
    }

    /// Insert a block at a declared position.
    ///
    /// `index == block_count()` appends. An index beyond the current end is
    /// rejected rather than clamped or silently reordered.
    pub fn insert_block(
        &mut self,
        index: usize,
        block: BlockId,
    ) -> Result<(), RegionError> {
        if self.blocks.contains(&block) {
            return Err(RegionError::DuplicateBlock(block));
        }

        if index > self.blocks.len() {
            return Err(RegionError::InvalidBlockIndex {
                index,
                block_count: self.blocks.len(),
            });
        }

        self.blocks.insert(index, block);
        Ok(())
    }

    /// Remove a block reference and return whether it was present.
    ///
    /// The caller must update related control-flow edges and verify the
    /// enclosing IR before using it again.
    pub fn remove_block(&mut self, block: BlockId) -> bool {
        if let Some(index) = self.blocks.iter().position(|candidate| *candidate == block) {
            self.blocks.remove(index);
            true
        } else {
            false
        }
    }

    /// Declare a capture of a value from an enclosing scope.
    ///
    /// Duplicate capture declarations are rejected even if their modes match:
    /// silently merging declarations could hide inconsistent construction.
    pub fn add_capture(
        &mut self,
        value: ValueId,
        mode: CaptureMode,
    ) -> Result<(), RegionError> {
        if self.isolation == RegionIsolation::Isolated {
            return Err(RegionError::CaptureInIsolatedRegion(value));
        }

        if self.captures.contains_key(&value) {
            return Err(RegionError::DuplicateCapture(value));
        }

        self.captures.insert(value, mode);
        Ok(())
    }

    /// Replace the mode of an existing capture.
    ///
    /// This does not add a capture if it is absent.
    pub fn set_capture_mode(
        &mut self,
        value: ValueId,
        mode: CaptureMode,
    ) -> Result<bool, RegionError> {
        if self.isolation == RegionIsolation::Isolated {
            return Err(RegionError::CaptureInIsolatedRegion(value));
        }

        if let Some(existing) = self.captures.get_mut(&value) {
            *existing = mode;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Remove a capture declaration.
    pub fn remove_capture(&mut self, value: ValueId) -> bool {
        self.captures.remove(&value).is_some()
    }

    /// Return the mode declared for a captured value, if any.
    #[must_use]
    pub fn capture_mode(&self, value: ValueId) -> Option<CaptureMode> {
        self.captures.get(&value).copied()
    }

    /// Return the number of capture declarations.
    #[must_use]
    pub fn capture_count(&self) -> usize {
        self.captures.len()
    }

    /// Iterate over captures in deterministic ValueId order.
    pub fn captures(
        &self,
    ) -> impl ExactSizeIterator<Item = (ValueId, CaptureMode)> + '_ {
        self.captures.iter().map(|(value, mode)| (*value, *mode))
    }

    /// Return a read-only view of the capture map.
    #[must_use]
    pub const fn captures_map(&self) -> &BTreeMap<ValueId, CaptureMode> {
        &self.captures
    }

    /// Add or replace a region attribute.
    ///
    /// Attribute-key syntax is validated here. Attribute value type checking,
    /// namespace authorization, and extension-specific meaning belong to the
    /// schema and dialect verifier.
    pub fn set_attribute(
        &mut self,
        key: AttributeKey,
        value: AttributeValue,
    ) -> Result<Option<AttributeValue>, RegionError> {
        key.validate().map_err(|error| {
            RegionError::InvalidAttributeKey(error.to_string())
        })?;

        Ok(self.attributes.insert(key, value))
    }

    /// Remove a region attribute.
    pub fn remove_attribute(
        &mut self,
        key: &AttributeKey,
    ) -> Option<AttributeValue> {
        self.attributes.remove(key)
    }

    /// Get a region attribute by its fully qualified key.
    #[must_use]
    pub fn attribute(&self, key: &AttributeKey) -> Option<&AttributeValue> {
        self.attributes.get(key)
    }

    /// Iterate over attributes in deterministic key order.
    pub fn attributes(
        &self,
    ) -> impl ExactSizeIterator<Item = (&AttributeKey, &AttributeValue)> + '_ {
        self.attributes.iter()
    }

    /// Return a read-only view of the attribute map.
    #[must_use]
    pub const fn attributes_map(
        &self,
    ) -> &BTreeMap<AttributeKey, AttributeValue> {
        &self.attributes
    }

    /// Validate invariants owned directly by this region.
    ///
    /// This method must be called after deserialization and before full IR
    /// verification. It does not establish block existence, reciprocal
    /// ownership, capture visibility, SSA dominance, or control-flow validity.
    pub fn validate(&self) -> Result<(), RegionError> {
        if self.blocks.is_empty() {
            return Err(RegionError::EmptyRegion);
        }

        self.kind.validate()?;

        let mut seen_blocks = BTreeSet::new();
        for block in &self.blocks {
            if !seen_blocks.insert(*block) {
                return Err(RegionError::DuplicateBlock(*block));
            }
        }

        if self.isolation == RegionIsolation::Isolated {
            if let Some(value) = self.captures.keys().next().copied() {
                return Err(RegionError::CaptureInIsolatedRegion(value));
            }
        }

        for key in self.attributes.keys() {
            key.validate().map_err(|error| {
                RegionError::InvalidAttributeKey(error.to_string())
            })?;
        }

        Ok(())
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn region_id(value: u128) -> RegionId {
        RegionId::new(value).expect("nonzero region ID")
    }

    fn function_id(value: u128) -> FunctionId {
        FunctionId::new(value).expect("nonzero function ID")
    }

    fn operation_id(value: u128) -> OperationId {
        OperationId::new(value).expect("nonzero operation ID")
    }

    fn block_id(value: u128) -> BlockId {
        BlockId::new(value).expect("nonzero block ID")
    }

    fn value_id(value: u128) -> ValueId {
        ValueId::new(value).expect("nonzero value ID")
    }

    fn structured_region() -> ZuirRegion {
        ZuirRegion::with_blocks(
            region_id(1),
            RegionOwner::Function(function_id(2)),
            RegionKind::Structured,
            RegionIsolation::CapturesAllowed,
            vec![block_id(3)],
        )
        .expect("valid region")
    }

    #[test]
    fn constructs_a_function_owned_region() {
        let region = structured_region();

        assert_eq!(region.id(), region_id(1));
        assert_eq!(region.owner(), RegionOwner::Function(function_id(2)));
        assert_eq!(region.blocks(), &[block_id(3)]);
        assert!(region.validate().is_ok());
    }

    #[test]
    fn supports_operation_owned_nested_regions() {
        let region = ZuirRegion::with_blocks(
            region_id(10),
            RegionOwner::Operation(operation_id(20)),
            RegionKind::Structured,
            RegionIsolation::CapturesAllowed,
            vec![block_id(30)],
        )
        .expect("valid nested region");

        assert_eq!(region.owner(), RegionOwner::Operation(operation_id(20)));
    }

    #[test]
    fn rejects_empty_complete_regions() {
        let region = ZuirRegion::new(
            region_id(1),
            RegionOwner::Function(function_id(2)),
            RegionKind::Structured,
            RegionIsolation::CapturesAllowed,
        );

        assert_eq!(region.validate(), Err(RegionError::EmptyRegion));
    }

    #[test]
    fn rejects_duplicate_blocks() {
        let mut region = structured_region();

        assert_eq!(
            region.add_block(block_id(3)),
            Err(RegionError::DuplicateBlock(block_id(3)))
        );
    }

    #[test]
    fn preserves_block_order() {
        let mut region = structured_region();

        region.add_block(block_id(4)).expect("unique block");
        region
            .insert_block(1, block_id(5))
            .expect("valid insertion");

        assert_eq!(
            region.blocks(),
            &[block_id(3), block_id(5), block_id(4)]
        );
    }

    #[test]
    fn rejects_out_of_range_block_insertion() {
        let mut region = structured_region();

        assert_eq!(
            region.insert_block(2, block_id(4)),
            Err(RegionError::InvalidBlockIndex {
                index: 2,
                block_count: 1,
            })
        );
    }

    #[test]
    fn removes_only_the_requested_block() {
        let mut region = structured_region();

        region.add_block(block_id(4)).expect("unique block");

        assert!(region.remove_block(block_id(3)));
        assert!(!region.remove_block(block_id(3)));
        assert_eq!(region.blocks(), &[block_id(4)]);
    }

    #[test]
    fn records_captures_deterministically() {
        let mut region = structured_region();

        region
            .add_capture(value_id(8), CaptureMode::Read)
            .expect("valid capture");
        region
            .add_capture(value_id(4), CaptureMode::BorrowShared)
            .expect("valid capture");

        let captures: Vec<_> = region.captures().collect();

        assert_eq!(
            captures,
            vec![
                (value_id(4), CaptureMode::BorrowShared),
                (value_id(8), CaptureMode::Read),
            ]
        );
    }

    #[test]
    fn rejects_duplicate_captures() {
        let mut region = structured_region();

        region
            .add_capture(value_id(8), CaptureMode::Read)
            .expect("first capture");

        assert_eq!(
            region.add_capture(value_id(8), CaptureMode::Move),
            Err(RegionError::DuplicateCapture(value_id(8)))
        );
    }

    #[test]
    fn isolated_regions_reject_captures() {
        let mut region = ZuirRegion::with_blocks(
            region_id(1),
            RegionOwner::Function(function_id(2)),
            RegionKind::Structured,
            RegionIsolation::Isolated,
            vec![block_id(3)],
        )
        .expect("valid isolated region");

        assert_eq!(
            region.add_capture(value_id(4), CaptureMode::Read),
            Err(RegionError::CaptureInIsolatedRegion(value_id(4)))
        );
    }

    #[test]
    fn isolation_cannot_be_enabled_while_captures_exist() {
        let mut region = structured_region();

        region
            .add_capture(value_id(4), CaptureMode::Read)
            .expect("valid capture");

        assert_eq!(
            region.set_isolation(RegionIsolation::Isolated),
            Err(RegionError::CaptureInIsolatedRegion(value_id(4)))
        );

        assert_eq!(region.isolation(), RegionIsolation::CapturesAllowed);
    }

    #[test]
    fn capture_mode_can_be_updated_or_removed() {
        let mut region = structured_region();

        region
            .add_capture(value_id(4), CaptureMode::Read)
            .expect("valid capture");

        assert_eq!(
            region.set_capture_mode(value_id(4), CaptureMode::Write),
            Ok(true)
        );
        assert_eq!(region.capture_mode(value_id(4)), Some(CaptureMode::Write));

        assert_eq!(
            region.set_capture_mode(value_id(5), CaptureMode::Read),
            Ok(false)
        );

        assert!(region.remove_capture(value_id(4)));
        assert!(!region.remove_capture(value_id(4)));
    }

    #[test]
    fn validates_extension_region_names() {
        let valid = RegionKind::Extension(
            QualifiedName::parse("vendor_example::region")
                .expect("valid extension name"),
        );

        assert!(valid.validate().is_ok());
    }

    #[test]
    fn removal_can_leave_a_region_incomplete_until_rebuilt() {
        let mut region = structured_region();

        assert!(region.remove_block(block_id(3)));
        assert_eq!(region.validate(), Err(RegionError::EmptyRegion));

        region.add_block(block_id(9)).expect("restore region");
        assert!(region.validate().is_ok());
    }
}

// =============================================================================
// Additional structured errors
// =============================================================================

// Keep index errors explicit rather than silently clamping or reordering
// caller input. The variant is part of the local RegionError contract.
impl RegionError {
    /// Construct an invalid-index error for region block insertion.
    fn invalid_block_index(index: usize, block_count: usize) -> Self {
        Self::InvalidBlockIndex { index, block_count }
    }
}
