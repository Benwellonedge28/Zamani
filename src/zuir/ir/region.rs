
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR Region
//!
//! Canonical path: `src/zuir/ir/region.rs`
//! Edition: Rust 2021
//! Minimum intended Rust version: 1.97
//! Safety: unsafe Rust is forbidden.
//!
//! PURPOSE
//!
//! Represent a target-independent region as an ordered collection of block
//! references, with explicit ownership, structural interpretation, captures,
//! isolation policy, and region-local attributes.
//!
//! OWNS
//!
//! - RegionOwner, RegionKind, RegionIsolation, and CaptureMode.
//! - ZuirRegion and region-local validation.
//! - RegionError and RegionResult.
//!
//! DOES NOT OWN
//!
//! - IDs: `crate::zuir::ids`.
//! - Blocks and block contents: `crate::zuir::ir::block`.
//! - Operations and nested-operation references:
//!   `crate::zuir::ir::operation`.
//! - Attribute keys and values: `crate::zuir::schema::attributes`.
//! - Qualified names: `crate::zuir::schema::types`.
//! - SSA, dominance, global reference resolution, capture visibility,
//!   effect verification, or full-program verification.
//! - Source parsing, AST construction, name resolution, lowering, or runtime.
//!
//! INTEGRATION CONTRACT
//!
//! Function-owned regions use `RegionOwner::Function(FunctionId)`.
//! Operation-owned regions use `RegionOwner::Operation(OperationId)`.
//! The full verifier must confirm reciprocal ownership and ensure that each
//! region is referenced by its declared owner.
//!
//! `blocks` contains ordered `BlockId` references. Block contents remain
//! exclusively owned by `ir/block.rs` and `ir/operation.rs`.
//!
//! Captures are declarations of enclosing-scope `ValueId`s. This module
//! checks duplicate declarations and isolation policy; the full verifier
//! checks existence, visibility, type, lifetime, and ownership legality.
//!
//! The grammar and semantic model define source meaning. A region accepts
//! already-resolved IDs and never interprets source syntax.
//!
//! SCALABILITY
//!
//! No universal maximum number of blocks, captures, attributes, qubits,
//! registers, processors, nodes, devices, or tensor dimensions is imposed.
//! Ordered block references use a dynamically sized Vec. `try_reserve_blocks`
//! exposes fallible reservation for that Vec. Standard-library BTreeMap
//! insertion does not expose a stable fallible-reservation API; this module
//! therefore does not claim to make every allocation failure recoverable.
//!
//! No finite machine provides literal infinity. POCO-REAF means this module
//! imposes no artificial universal machine-size ceiling and preserves
//! semantics independently of the eventual target.
//!
//! VALIDITY BOUNDARY
//!
//! `validate()` checks region-local invariants. It does not establish that
//! referenced blocks or captured values exist. Deserialized regions are
//! locally validated, but still require full-IR verification before trusted
//! optimization, lowering, or execution.

use serde::{Deserialize, Deserializer, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use crate::zuir::ids::{BlockId, FunctionId, OperationId, RegionId, ValueId};
use crate::zuir::schema::attributes::{AttributeKey, AttributeValue};
use crate::zuir::schema::types::QualifiedName;

/// Result type for region construction and validation.
pub type RegionResult<T> = Result<T, RegionError>;

/// Entity that declares ownership of a region.
///
/// The full verifier checks that the owner references this region and that
/// the region's declared owner agrees with that reference.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
#[serde(tag = "kind", content = "id", rename_all = "snake_case")]
pub enum RegionOwner {
    /// Region owned by a function.
    Function(FunctionId),

    /// Region owned by an operation.
    Operation(OperationId),
}

/// General structural interpretation of a region.
///
/// This is intentionally not a list of computational operations. Domain
/// behavior belongs to dialect contracts and registered extensions.
#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
#[serde(tag = "kind", content = "extension", rename_all = "snake_case")]
pub enum RegionKind {
    /// Structure interpreted by the containing construct.
    Structured,

    /// Explicit blocks and control-flow edges define execution flow.
    ControlFlow,

    /// Dependencies define a data-flow computation.
    Dataflow,

    /// Additional semantics are defined by a qualified extension.
    Extension(QualifiedName),
}

impl RegionKind {
    /// Validate invariants owned by the region-kind declaration.
    pub fn validate(&self) -> RegionResult<()> {
        if let Self::Extension(name) = self {
            name.validate().map_err(|error| {
                RegionError::InvalidExtensionName(error.to_string())
            })?;
        }

        Ok(())
    }
}

/// Whether a region may declare captures from an enclosing scope.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum RegionIsolation {
    /// Enclosing values may be captured explicitly.
    CapturesAllowed,

    /// No enclosing values may be captured.
    ///
    /// Local block arguments and locally defined operation results are not
    /// captures. Isolation does not itself prove purity.
    Isolated,
}

/// Declared use of an enclosing-scope value.
///
/// The full verifier determines whether the declared mode is legal for the
/// captured value's type, ownership, lifetime, and execution domain.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum CaptureMode {
    /// Read the value without declaring mutation.
    Read,

    /// Read and potentially mutate the value.
    Write,

    /// Transfer the value into the region.
    Move,

    /// Borrow the value immutably.
    BorrowShared,

    /// Borrow the value mutably.
    BorrowExclusive,
}

/// Errors detectable from a region and its locally owned state.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum RegionError {
    /// A region requiring a complete structure has no blocks.
    EmptyRegion,

    /// The same block appears more than once.
    DuplicateBlock(BlockId),

    /// A block insertion position exceeds the current block count.
    InvalidBlockIndex {
        index: usize,
        block_count: usize,
    },

    /// A captured value is declared more than once.
    DuplicateCapture(ValueId),

    /// An isolated region declares or attempts to add a capture.
    CaptureInIsolatedRegion(ValueId),

    /// A region extension name is malformed.
    InvalidExtensionName(String),

    /// An attribute key is malformed.
    InvalidAttributeKey(String),

    /// Capacity reservation for the block vector failed.
    AllocationFailure {
        collection: &'static str,
    },
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
            Self::InvalidBlockIndex { index, block_count } => write!(
                f,
                "block insertion index {index} exceeds block count {block_count}"
            ),
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
            Self::AllocationFailure { collection } => {
                write!(f, "unable to reserve memory for region {collection}")
            }
        }
    }
}

impl std::error::Error for RegionError {}

/// Canonical target-independent ZUIR region.
///
/// Blocks preserve declaration order. Captures and attributes use ordered
/// maps so their traversal is deterministic. The order of blocks does not
/// implicitly define control flow: the block terminators and successor
/// contracts owned by the corresponding IR modules define control flow.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ZuirRegion {
    id: RegionId,
    owner: RegionOwner,
    kind: RegionKind,
    isolation: RegionIsolation,
    blocks: Vec<BlockId>,
    captures: BTreeMap<ValueId, CaptureMode>,
    attributes: BTreeMap<AttributeKey, AttributeValue>,
}

/// Wire representation used to validate regions as they are deserialized.
///
/// A separate wire type avoids exposing an unchecked deserialization path
/// for the canonical region.
#[derive(Deserialize)]
struct ZuirRegionWire {
    id: RegionId,
    owner: RegionOwner,
    kind: RegionKind,
    isolation: RegionIsolation,
    blocks: Vec<BlockId>,
    #[serde(default)]
    captures: BTreeMap<ValueId, CaptureMode>,
    #[serde(default)]
    attributes: BTreeMap<AttributeKey, AttributeValue>,
}

impl<'de> Deserialize<'de> for ZuirRegion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = ZuirRegionWire::deserialize(deserializer)?;

        let region = Self {
            id: wire.id,
            owner: wire.owner,
            kind: wire.kind,
            isolation: wire.isolation,
            blocks: wire.blocks,
            captures: wire.captures,
            attributes: wire.attributes,
        };

        region.validate().map_err(serde::de::Error::custom)?;
        Ok(region)
    }
}

impl ZuirRegion {
    /// Create an empty region under construction.
    ///
    /// An empty region is not valid as a complete serialized region.
    /// Add at least one block before calling `validate()` or serializing
    /// it for interchange.
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
    /// The block list must be nonempty and duplicate-free.
    pub fn with_blocks(
        id: RegionId,
        owner: RegionOwner,
        kind: RegionKind,
        isolation: RegionIsolation,
        blocks: Vec<BlockId>,
    ) -> RegionResult<Self> {
        let mut region = Self::new(id, owner, kind, isolation);
        region.set_blocks(blocks)?;
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

    /// Return the structural interpretation.
    #[must_use]
    pub const fn kind(&self) -> &RegionKind {
        &self.kind
    }

    /// Return the capture-isolation policy.
    #[must_use]
    pub const fn isolation(&self) -> RegionIsolation {
        self.isolation
    }

    /// Replace the structural interpretation after validation.
    pub fn set_kind(&mut self, kind: RegionKind) -> RegionResult<()> {
        kind.validate()?;
        self.kind = kind;
        Ok(())
    }

    /// Change the isolation policy.
    ///
    /// Enabling isolation while captures exist is rejected. Remove those
    /// captures explicitly before changing the policy.
    pub fn set_isolation(&mut self, isolation: RegionIsolation) -> RegionResult<()> {
        if isolation == RegionIsolation::Isolated {
            if let Some(value) = self.captures.keys().next().copied() {
                return Err(RegionError::CaptureInIsolatedRegion(value));
            }
        }

        self.isolation = isolation;
        Ok(())
    }

    /// Whether this region currently references any blocks.
    #[must_use]
    pub fn has_blocks(&self) -> bool {
        !self.blocks.is_empty()
    }

    /// Number of referenced blocks.
    #[must_use]
    pub fn block_count(&self) -> usize {
        self.blocks.len()
    }

    /// Borrow the ordered block references.
    #[must_use]
    pub fn blocks(&self) -> &[BlockId] {
        &self.blocks
    }

    /// Whether the region references the given block.
    #[must_use]
    pub fn contains_block(&self, block: BlockId) -> bool {
        self.blocks.contains(&block)
    }

    /// Reserve capacity for additional block references.
    ///
    /// This can return an allocation error instead of relying on a later
    /// vector growth. It does not impose a semantic block-count limit.
    pub fn try_reserve_blocks(&mut self, additional: usize) -> RegionResult<()> {
        self.blocks
            .try_reserve(additional)
            .map_err(|_| RegionError::AllocationFailure {
                collection: "block references",
            })
    }

    /// Add a block reference at the end of the region.
    ///
    /// Existence and reciprocal ownership are checked by the full verifier.
    pub fn add_block(&mut self, block: BlockId) -> RegionResult<()> {
        if self.blocks.contains(&block) {
            return Err(RegionError::DuplicateBlock(block));
        }

        self.try_reserve_blocks(1)?;
        self.blocks.push(block);
        Ok(())
    }

    /// Insert a block reference at a specified position.
    ///
    /// `index == block_count()` appends. An out-of-range position is rejected.
    pub fn insert_block(&mut self, index: usize, block: BlockId) -> RegionResult<()> {
        if index > self.blocks.len() {
            return Err(RegionError::InvalidBlockIndex {
                index,
                block_count: self.blocks.len(),
            });
        }

        if self.blocks.contains(&block) {
            return Err(RegionError::DuplicateBlock(block));
        }

        self.try_reserve_blocks(1)?;
        self.blocks.insert(index, block);
        Ok(())
    }

    /// Replace the complete ordered block list.
    ///
    /// The replacement is validated before the current list is modified.
    pub fn set_blocks(&mut self, blocks: Vec<BlockId>) -> RegionResult<()> {
        let mut seen = BTreeSet::new();

        for block in &blocks {
            if !seen.insert(*block) {
                return Err(RegionError::DuplicateBlock(*block));
            }
        }

        self.blocks = blocks;
        Ok(())
    }

    /// Remove a block reference.
    ///
    /// Returns `true` when the reference was present. The caller must repair
    /// related control-flow edges and verify the enclosing IR afterwards.
    pub fn remove_block(&mut self, block: BlockId) -> bool {
        if let Some(index) = self.blocks.iter().position(|candidate| *candidate == block) {
            self.blocks.remove(index);
            true
        } else {
            false
        }
    }

    /// Declare an enclosing-scope capture.
    ///
    /// Duplicate declarations are rejected even if their modes are equal.
    pub fn add_capture(
        &mut self,
        value: ValueId,
        mode: CaptureMode,
    ) -> RegionResult<()> {
        if self.isolation == RegionIsolation::Isolated {
            return Err(RegionError::CaptureInIsolatedRegion(value));
        }

        if self.captures.contains_key(&value) {
            return Err(RegionError::DuplicateCapture(value));
        }

        self.captures.insert(value, mode);
        Ok(())
    }

    /// Change the mode of an existing capture.
    ///
    /// This does not add a capture. Returns `false` when the value is absent.
    pub fn set_capture_mode(
        &mut self,
        value: ValueId,
        mode: CaptureMode,
    ) -> RegionResult<bool> {
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

    /// Return a capture's declared mode.
    #[must_use]
    pub fn capture_mode(&self, value: ValueId) -> Option<CaptureMode> {
        self.captures.get(&value).copied()
    }

    /// Number of capture declarations.
    #[must_use]
    pub fn capture_count(&self) -> usize {
        self.captures.len()
    }

    /// Iterate over captures in deterministic `ValueId` order.
    pub fn captures(&self) -> impl ExactSizeIterator<Item = (ValueId, CaptureMode)> + '_ {
        self.captures.iter().map(|(value, mode)| (*value, *mode))
    }

    /// Borrow the canonical capture map.
    #[must_use]
    pub const fn captures_map(&self) -> &BTreeMap<ValueId, CaptureMode> {
        &self.captures
    }

    /// Insert or replace a region attribute after validating its key.
    ///
    /// Returns the previous value when a value was replaced. Attribute value
    /// semantics, namespace authorization, and dialect-specific constraints
    /// remain the responsibility of the schema and full verifier.
    pub fn set_attribute(
        &mut self,
        key: AttributeKey,
        value: AttributeValue,
    ) -> RegionResult<Option<AttributeValue>> {
        key.validate().map_err(|error| {
            RegionError::InvalidAttributeKey(error.to_string())
        })?;

        Ok(self.attributes.insert(key, value))
    }

    /// Remove a region attribute.
    pub fn remove_attribute(&mut self, key: &AttributeKey) -> Option<AttributeValue> {
        self.attributes.remove(key)
    }

    /// Look up a region attribute.
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

    /// Borrow the canonical attribute map.
    #[must_use]
    pub const fn attributes_map(&self) -> &BTreeMap<AttributeKey, AttributeValue> {
        &self.attributes
    }

    /// Validate invariants owned directly by this region.
    ///
    /// This does not resolve block/capture references, prove ownership
    /// reciprocity, or establish whole-program correctness.
    pub fn validate(&self) -> RegionResult<()> {
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
    fn constructs_and_validates_function_owned_region() {
        let region = structured_region();

        assert_eq!(region.id(), region_id(1));
        assert_eq!(region.owner(), RegionOwner::Function(function_id(2)));
        assert_eq!(region.blocks(), &[block_id(3)]);
        assert!(region.validate().is_ok());
    }

    #[test]
    fn supports_operation_owned_region() {
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
    fn rejects_empty_complete_region() {
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
        region.insert_block(1, block_id(5)).expect("valid insertion");

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
    fn replaces_block_list_only_after_validation() {
        let mut region = structured_region();
        let original = region.blocks().to_vec();

        assert_eq!(
            region.set_blocks(vec![block_id(4), block_id(4)]),
            Err(RegionError::DuplicateBlock(block_id(4)))
        );
        assert_eq!(region.blocks(), original);
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
    fn captures_iterate_in_deterministic_order() {
        let mut region = structured_region();

        region.add_capture(value_id(8), CaptureMode::Read).expect("capture");
        region
            .add_capture(value_id(4), CaptureMode::BorrowShared)
            .expect("capture");

        assert_eq!(
            region.captures().collect::<Vec<_>>(),
            vec![
                (value_id(4), CaptureMode::BorrowShared),
                (value_id(8), CaptureMode::Read),
            ]
        );
    }

    #[test]
    fn rejects_duplicate_captures() {
        let mut region = structured_region();
        region.add_capture(value_id(8), CaptureMode::Read).expect("first");

        assert_eq!(
            region.add_capture(value_id(8), CaptureMode::Move),
            Err(RegionError::DuplicateCapture(value_id(8)))
        );
    }

    #[test]
    fn isolated_region_rejects_capture() {
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
    fn cannot_enable_isolation_while_captures_exist() {
        let mut region = structured_region();
        region.add_capture(value_id(4), CaptureMode::Read).expect("capture");

        assert_eq!(
            region.set_isolation(RegionIsolation::Isolated),
            Err(RegionError::CaptureInIsolatedRegion(value_id(4)))
        );
    }

    #[test]
    fn changing_capture_mode_does_not_create_capture() {
        let mut region = structured_region();

        assert_eq!(
            region
                .set_capture_mode(value_id(4), CaptureMode::Write)
                .expect("missing capture is not an error"),
            false
        );

        region.add_capture(value_id(4), CaptureMode::Read).expect("capture");

        assert_eq!(
            region
                .set_capture_mode(value_id(4), CaptureMode::Write)
                .expect("existing capture"),
            true
        );
        assert_eq!(region.capture_mode(value_id(4)), Some(CaptureMode::Write));
    }

    #[test]
    fn validates_extension_names() {
        let name = QualifiedName::parse("vendor::region").expect("valid name");
        let mut region = structured_region();

        region
            .set_kind(RegionKind::Extension(name))
            .expect("valid extension name");
        assert!(region.validate().is_ok());
    }

    #[test]
    fn validates_attribute_keys() {
        let mut region = structured_region();
        let key = AttributeKey::parse("core::region_kind").expect("valid key");

        region
            .set_attribute(key.clone(), AttributeValue::Boolean(true))
            .expect("valid attribute");
        assert!(region.attribute(&key).is_some());
        assert!(region.validate().is_ok());
    }

    #[test]
    fn block_reservation_does_not_change_semantics() {
        let mut region = structured_region();
        region.try_reserve_blocks(32).expect("reservation");
        assert_eq!(region.blocks(), &[block_id(3)]);
    }
}
