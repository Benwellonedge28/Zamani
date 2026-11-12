
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Runtime — Memory Regions
//!
//! Path: `src/zur/memory/region.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # PURPOSE
//!
//! Provides the ZUR-facing representation of resolved memory-region
//! descriptors and their runtime-relevant access properties.
//!
//! A memory region is a logical memory-management abstraction.
//! It may describe an allocation domain, ownership boundary,
//! isolation boundary, lifetime association, locality intent,
//! persistence association, or other validated memory grouping.
//!
//! This module preserves the distinction between logical memory
//! regions and their eventual physical realization.
//!
//! # ARCHITECTURAL OWNERSHIP
//!
//! Canonical ownership remains with existing repository authorities:
//!
//! - `crate::zuir::ids::RegionId` owns region identity.
//! - `crate::zuir::ir::region` owns structural IR regions.
//! - `crate::classical::memory::regions` owns semantic lifetime scopes.
//! - `crate::classical::memory::allocation` owns allocations.
//! - `crate::classical::memory::ownership` owns ownership contracts.
//! - Semantic analysis owns source-level region validity.
//! - Resource analysis owns requirements and feasibility.
//! - Platform and device providers own physical realization.
//!
//! # THIS MODULE OWNS
//!
//! - MemoryRegion.
//! - RegionAccess.
//! - RegionError.
//! - RegionResult.
//! - Region-local validation.
//! - Runtime-facing region metadata.
//! - Allocation-request/region consistency validation.
//!
//! # THIS MODULE DOES NOT OWN
//!
//! - Region identifiers or identifier allocation.
//! - Source grammar or AST construction.
//! - Semantic region hierarchy analysis.
//! - Lifetime inference.
//! - Borrow checking.
//! - Ownership inference.
//! - Physical memory addresses.
//! - Physical memory-bank discovery.
//! - Device discovery.
//! - Resource feasibility.
//! - Resource reservation.
//! - Hardware placement.
//! - Allocation algorithms.
//! - Allocation accounting.
//! - Allocation handles.
//! - Garbage collection.
//! - Quantum-state ownership.
//! - Scheduling or routing.
//! - A universal hardware abstraction layer.
//!
//! # IMPORTANT SEMANTIC DISTINCTIONS
//!
//! A memory region is not proof that physical memory has been acquired.
//!
//! The following remain separate:
//!
//! Requirement
//! Capability
//! Constraint
//! Preference
//! Hint
//! Feasibility
//! Reservation
//! Region declaration
//! Allocation
//! Actual execution
//!
//! This module represents resolved region metadata.
//!
//! It does not convert a requirement into a successful allocation.
//!
//! # OPEN-WORLD DESIGN
//!
//! Region names, memory-space identifiers, region kinds, and metadata
//! keys are represented as strings rather than closed hardware enums.
//!
//! This allows future technologies and domains to introduce new
//! semantic identifiers without modifying the ZUR memory core.
//!
//! # SCALABILITY
//!
//! No arbitrary maximum is imposed on:
//!
//! - Region count.
//! - Region identity values.
//! - Region-name length.
//! - Region metadata count.
//! - Region hierarchy depth.
//! - Address-space identities.
//! - Memory-space identities.
//! - Allocation sizes.
//! - Device counts.
//! - Processor counts.
//! - Distributed participants.
//! - Computational domains.
//!
//! Collections grow dynamically.
//!
//! Operations are subject to available memory, integer representation,
//! platform constraints, and explicitly supplied operational policies.
//!
//! Literal infinite physical memory is not promised.
//!
//! # GRAMMAR INTEGRATION
//!
//! Source-level authorities include:
//!
//! - `grammar/memory/regions.g4`
//! - `grammar/memory/memory.g4`
//! - `grammar/memory/address-spaces.g4`
//! - `grammar/memory/allocation.g4`
//! - `grammar/memory/ownership.g4`
//! - `grammar/memory/borrowing.g4`
//! - `grammar/memory/lifetimes.g4`
//! - `grammar/spec/resources.md`
//! - `grammar/specification/semantic-model.md`
//!
//! The frontend and semantic analyzer must resolve source-level
//! declarations before constructing runtime-facing descriptors.
//!
//! This module does not parse or interpret source syntax.
//!
//! # DOMAIN INTEGRATION
//!
//! Classical:
//! `crate::classical::memory`
//!
//! Quantum:
//! `crate::quantum::memory`
//! `crate::quantum::ir`
//! `crate::quantum::hardware`
//!
//! HDL and hardware:
//! `crate::hdl`
//! `crate::backend`
//!
//! Hybrid:
//! `crate::hybrid`
//!
//! Distributed:
//! `crate::distributed`
//!
//! Unified representation:
//! `crate::zuir`
//!
//! The region descriptor remains domain-neutral.
//!
//! Domain-specific memory semantics and physical implementations
//! remain with their existing authorities.
//!
//! # FREEZE CONTRACT
//!
//! This module must not become another allocator, semantic analyzer,
//! memory-region hierarchy index, or physical placement engine.
//!
//! New domains must integrate through open-world identifiers and
//! existing provider contracts.

use serde::{Deserialize, Deserializer, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use thiserror::Error;

use crate::classical::memory::allocation::AllocationRequest;
use crate::zuir::ids::RegionId;

/// Result type for runtime memory-region operations.
pub type RegionResult<T> = Result<T, RegionError>;

/// Access operation that a region may explicitly permit.
///
/// These values describe common access properties, not hardware
/// instructions or device-specific memory operations.
///
/// An unspecified access policy is distinct from an explicitly
/// specified empty policy.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum RegionAccess {
    /// Read access.
    Read,

    /// Write access.
    Write,

    /// Atomic access where supported by the relevant implementation.
    Atomic,
}

/// Errors associated with memory-region descriptors.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[non_exhaustive]
pub enum RegionError {
    /// A required region identifier is invalid.
    #[error("invalid memory-region identifier")]
    InvalidIdentity,

    /// A region cannot directly identify itself as its parent.
    #[error("memory region {0:?} cannot be its own parent")]
    SelfParent(RegionId),

    /// A required region kind is empty or malformed.
    #[error("invalid memory-region kind: {0}")]
    InvalidKind(String),

    /// A memory-space identifier is invalid.
    #[error("invalid memory-space identifier: {0}")]
    InvalidMemorySpace(String),

    /// A region name is invalid.
    #[error("invalid memory-region name: {0}")]
    InvalidName(String),

    /// A metadata key is invalid.
    #[error("invalid memory-region metadata key: {0}")]
    InvalidMetadataKey(String),

    /// The region does not accept the requested memory space.
    #[error(
        "memory-space mismatch: region expects {expected}, request specifies {actual}"
    )]
    MemorySpaceMismatch {
        /// Memory space declared by the region.
        expected: String,

        /// Memory space declared by the allocation request.
        actual: String,
    },

    /// A region descriptor contains inconsistent serialized data.
    #[error("invalid serialized memory-region descriptor: {0}")]
    InvalidSerializedData(String),
}

/// A validated, target-independent memory-region descriptor.
///
/// `MemoryRegion` describes logical memory-region identity and
/// properties. It does not represent an acquired allocation or
/// physical memory location.
///
/// The canonical `RegionId` is reused from ZUIR.
///
/// # Fields
///
/// - `id`: Canonical region identity.
/// - `parent`: Optional enclosing logical region.
/// - `name`: Optional semantic name.
/// - `kind`: Open-world region-kind identifier.
/// - `memory_space`: Optional resolved logical memory-space identity.
/// - `access`: Optional access policy.
/// - `metadata`: Open-world metadata.
///
/// # Hierarchy
///
/// Parent relationships are descriptive references.
///
/// The complete semantic hierarchy and lifetime relationships
/// remain owned by `classical::memory::regions` and the relevant
/// semantic verification infrastructure.
///
/// This descriptor does not independently establish that its
/// parent exists or that its lifetime relationship is valid.
///
/// # Physical realization
///
/// `memory_space` is a logical identifier.
///
/// It is not a physical address, device identifier, memory bank,
/// NUMA node, GPU memory pool, or hardware placement decision.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct MemoryRegion {
    id: RegionId,
    parent: Option<RegionId>,
    name: Option<String>,
    kind: String,
    memory_space: Option<String>,
    access: Option<BTreeSet<RegionAccess>>,
    metadata: BTreeMap<String, String>,
}

/// Private deserialization representation.
///
/// Deserialization must pass through validation instead of
/// directly trusting serialized region metadata.
#[derive(Deserialize)]
struct MemoryRegionWire {
    id: RegionId,
    parent: Option<RegionId>,
    name: Option<String>,
    kind: String,
    memory_space: Option<String>,
    access: Option<BTreeSet<RegionAccess>>,
    metadata: BTreeMap<String, String>,
}

impl<'de> Deserialize<'de> for MemoryRegion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = MemoryRegionWire::deserialize(deserializer)?;

        let region = Self {
            id: wire.id,
            parent: wire.parent,
            name: wire.name,
            kind: wire.kind,
            memory_space: wire.memory_space,
            access: wire.access,
            metadata: wire.metadata,
        };

        region.validate().map_err(serde::de::Error::custom)?;

        Ok(region)
    }
}

impl MemoryRegion {
    /// Construct a memory-region descriptor.
    ///
    /// The region starts without a parent, name, memory-space
    /// association, access policy, or additional metadata.
    ///
    /// The supplied kind is an open-world semantic identifier.
    ///
    /// # Errors
    ///
    /// Returns an error when the region kind is invalid.
    pub fn new(
        id: RegionId,
        kind: impl Into<String>,
    ) -> RegionResult<Self> {
        let region = Self {
            id,
            parent: None,
            name: None,
            kind: kind.into(),
            memory_space: None,
            access: None,
            metadata: BTreeMap::new(),
        };

        region.validate()?;

        Ok(region)
    }

    /// Return the canonical region identity.
    #[must_use]
    pub const fn id(&self) -> RegionId {
        self.id
    }

    /// Return the optional enclosing region identity.
    ///
    /// This is a reference to semantic hierarchy information,
    /// not a physical memory hierarchy.
    #[must_use]
    pub const fn parent(&self) -> Option<RegionId> {
        self.parent
    }

    /// Return the optional semantic region name.
    #[must_use]
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Return the open-world region-kind identifier.
    #[must_use]
    pub fn kind(&self) -> &str {
        &self.kind
    }

    /// Return the optional logical memory-space identifier.
    #[must_use]
    pub fn memory_space(&self) -> Option<&str> {
        self.memory_space.as_deref()
    }

    /// Return the optional access policy.
    ///
    /// `None` means that this descriptor does not declare an
    /// access policy.
    ///
    /// `Some(empty_set)` means that the descriptor explicitly
    /// declares no permitted access modes.
    #[must_use]
    pub fn access(&self) -> Option<&BTreeSet<RegionAccess>> {
        self.access.as_ref()
    }

    /// Return the immutable region metadata.
    #[must_use]
    pub fn metadata(&self) -> &BTreeMap<String, String> {
        &self.metadata
    }

    /// Return whether the region has an explicitly declared access policy.
    #[must_use]
    pub const fn has_access_policy(&self) -> bool {
        self.access.is_some()
    }

    /// Return whether an access mode is explicitly permitted.
    ///
    /// Returns `None` when no access policy has been declared.
    #[must_use]
    pub fn permits(&self, access: RegionAccess) -> Option<bool> {
        self.access
            .as_ref()
            .map(|policy| policy.contains(&access))
    }

    /// Set the optional enclosing logical region.
    ///
    /// This method performs local self-parent validation only.
    /// Existence, cycles across multiple descriptors, and semantic
    /// lifetime validity remain responsibilities of the authoritative
    /// region hierarchy and semantic verifier.
    ///
    /// # Errors
    ///
    /// Returns `RegionError::SelfParent` when the supplied identity
    /// equals this region's identity.
    pub fn with_parent(
        mut self,
        parent: Option<RegionId>,
    ) -> RegionResult<Self> {
        if parent == Some(self.id) {
            return Err(RegionError::SelfParent(self.id));
        }

        self.parent = parent;

        Ok(self)
    }

    /// Set the optional semantic region name.
    ///
    /// Names are treated as semantic data rather than a closed
    /// enumeration of memory technologies.
    ///
    /// # Errors
    ///
    /// Returns an error when the name is empty or contains
    /// forbidden control characters.
    pub fn with_name(
        mut self,
        name: impl Into<String>,
    ) -> RegionResult<Self> {
        let name = name.into();

        validate_identifier_text(&name)
            .map_err(|_| RegionError::InvalidName(name.clone()))?;

        self.name = Some(name);

        Ok(self)
    }

    /// Set the logical memory-space association.
    ///
    /// Examples include:
    ///
    /// - `memory::host`
    /// - `memory::shared`
    /// - `memory::distributed`
    /// - `memory::persistent`
    /// - `vendor::domain::memory`
    ///
    /// These are identifiers, not physical placement decisions.
    ///
    /// # Errors
    ///
    /// Returns an error when the identifier is invalid.
    pub fn with_memory_space(
        mut self,
        memory_space: impl Into<String>,
    ) -> RegionResult<Self> {
        let memory_space = memory_space.into();

        validate_identifier_text(&memory_space).map_err(|_| {
            RegionError::InvalidMemorySpace(memory_space.clone())
        })?;

        self.memory_space = Some(memory_space);

        Ok(self)
    }

    /// Declare the region's permitted access modes.
    ///
    /// Supplying an empty set explicitly declares that no access
    /// mode is permitted.
    ///
    /// This method does not perform authorization or enforce
    /// hardware-level protection.
    pub fn with_access(
        mut self,
        access: impl IntoIterator<Item = RegionAccess>,
    ) -> Self {
        self.access = Some(access.into_iter().collect());

        self
    }

    /// Add an open-world metadata entry.
    ///
    /// Metadata is descriptive. It must not independently authorize
    /// hardware operations or bypass semantic validation.
    ///
    /// Duplicate keys replace their previous values.
    ///
    /// # Errors
    ///
    /// Returns an error when the metadata key is invalid.
    pub fn with_metadata(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> RegionResult<Self> {
        let key = key.into();

        validate_identifier_text(&key).map_err(|_| {
            RegionError::InvalidMetadataKey(key.clone())
        })?;

        self.metadata.insert(key, value.into());

        Ok(self)
    }

    /// Return an individual metadata value.
    #[must_use]
    pub fn metadata_value(&self, key: &str) -> Option<&str> {
        self.metadata.get(key).map(String::as_str)
    }

    /// Validate all locally owned descriptor invariants.
    ///
    /// This does not verify parent existence, full hierarchy
    /// consistency, ownership, borrowing, lifetime legality,
    /// allocation feasibility, or physical resource availability.
    ///
    /// # Errors
    ///
    /// Returns the first locally detectable descriptor error.
    pub fn validate(&self) -> RegionResult<()> {
        if self.kind.trim().is_empty()
            || self.kind.chars().any(char::is_control)
        {
            return Err(RegionError::InvalidKind(self.kind.clone()));
        }

        if self.parent == Some(self.id) {
            return Err(RegionError::SelfParent(self.id));
        }

        if let Some(name) = &self.name {
            validate_identifier_text(name)
                .map_err(|_| RegionError::InvalidName(name.clone()))?;
        }

        if let Some(space) = &self.memory_space {
            validate_identifier_text(space).map_err(|_| {
                RegionError::InvalidMemorySpace(space.clone())
            })?;
        }

        for key in self.metadata.keys() {
            validate_identifier_text(key).map_err(|_| {
                RegionError::InvalidMetadataKey(key.clone())
            })?;
        }

        Ok(())
    }

    /// Check that an allocation request is consistent with the
    /// region's explicitly declared memory-space association.
    ///
    /// This method does not allocate storage, select an allocator,
    /// discover hardware, or establish resource feasibility.
    ///
    /// If the region does not specify a memory space, this method
    /// performs no memory-space restriction.
    ///
    /// # Errors
    ///
    /// Returns `MemorySpaceMismatch` when the request specifies a
    /// different memory space from the region.
    pub fn validate_allocation_request(
        &self,
        request: &AllocationRequest,
    ) -> RegionResult<()> {
        if let Some(expected) = &self.memory_space {
            let actual = request.space();

            if expected != actual {
                return Err(RegionError::MemorySpaceMismatch {
                    expected: expected.clone(),
                    actual: actual.to_owned(),
                });
            }
        }

        Ok(())
    }
}

/// Validate open-world semantic identifier text.
///
/// This deliberately does not implement a source-language parser
/// or enforce a particular qualified-name grammar.
///
/// The frontend and semantic authority perform canonical name
/// resolution and validation.
fn validate_identifier_text(value: &str) -> Result<(), ()> {
    if value.trim().is_empty() {
        return Err(());
    }

    if value.chars().any(char::is_control) {
        return Err(());
    }

    Ok(())
}

impl fmt::Display for MemoryRegion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "MemoryRegion(id={}, kind={}",
            self.id,
            self.kind
        )?;

        if let Some(name) = &self.name {
            write!(f, ", name={name}")?;
        }

        if let Some(parent) = self.parent {
            write!(f, ", parent={parent}")?;
        }

        if let Some(space) = &self.memory_space {
            write!(f, ", memory_space={space}")?;
        }

        write!(f, ")")
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::classical::memory::allocation::AllocationRequest;

    fn test_region() -> MemoryRegion {
        MemoryRegion::new(
            RegionId::new(1).expect("valid region identity"),
            "memory::region",
        )
        .expect("valid region")
    }

    #[test]
    fn creates_valid_region() {
        let region = test_region();

        assert_eq!(region.id().get(), 1);
        assert_eq!(region.kind(), "memory::region");
        assert!(region.parent().is_none());
        assert!(region.name().is_none());
        assert!(region.memory_space().is_none());
    }

    #[test]
    fn supports_open_world_region_kinds() {
        let region = MemoryRegion::new(
            RegionId::new(2).expect("valid identity"),
            "future::photonic::memory::region",
        )
        .expect("future region kind should be accepted");

        assert_eq!(
            region.kind(),
            "future::photonic::memory::region"
        );
    }

    #[test]
    fn supports_open_world_memory_spaces() {
        let region = test_region()
            .with_memory_space("vendor::future::memory")
            .expect("valid memory space");

        assert_eq!(
            region.memory_space(),
            Some("vendor::future::memory")
        );
    }

    #[test]
    fn rejects_self_parent() {
        let id = RegionId::new(3).expect("valid identity");
        let region = MemoryRegion::new(id, "memory::region")
            .expect("valid region");

        assert!(matches!(
            region.with_parent(Some(id)),
            Err(RegionError::SelfParent(_))
        ));
    }

    #[test]
    fn supports_parent_relationships() {
        let parent = RegionId::new(4).expect("valid parent");
        let child = RegionId::new(5).expect("valid child");

        let region = MemoryRegion::new(child, "memory::region")
            .expect("valid region")
            .with_parent(Some(parent))
            .expect("valid parent relationship");

        assert_eq!(region.parent(), Some(parent));
    }

    #[test]
    fn supports_explicit_access_policies() {
        let region = test_region().with_access([
            RegionAccess::Read,
            RegionAccess::Write,
        ]);

        assert_eq!(region.permits(RegionAccess::Read), Some(true));
        assert_eq!(region.permits(RegionAccess::Write), Some(true));
        assert_eq!(region.permits(RegionAccess::Atomic), Some(false));
    }

    #[test]
    fn unspecified_access_is_distinct_from_empty_access() {
        let unspecified = test_region();

        let explicitly_empty =
            test_region().with_access(std::iter::empty());

        assert_eq!(
            unspecified.permits(RegionAccess::Read),
            None
        );

        assert_eq!(
            explicitly_empty.permits(RegionAccess::Read),
            Some(false)
        );
    }

    #[test]
    fn supports_open_world_metadata() {
        let region = test_region()
            .with_metadata("vendor::property", "value")
            .expect("valid metadata");

        assert_eq!(
            region.metadata_value("vendor::property"),
            Some("value")
        );
    }

    #[test]
    fn rejects_invalid_names() {
        let region = test_region();

        assert!(matches!(
            region.with_name(""),
            Err(RegionError::InvalidName(_))
        ));
    }

    #[test]
    fn rejects_invalid_memory_spaces() {
        let region = test_region();

        assert!(matches!(
            region.with_memory_space(""),
            Err(RegionError::InvalidMemorySpace(_))
        ));
    }

    #[test]
    fn accepts_matching_allocation_space() {
        let region = test_region()
            .with_memory_space("host")
            .expect("valid space");

        let request = AllocationRequest::host(64);

        assert!(region
            .validate_allocation_request(&request)
            .is_ok());
    }

    #[test]
    fn rejects_mismatching_allocation_space() {
        let region = test_region()
            .with_memory_space("device")
            .expect("valid space");

        let request = AllocationRequest::host(64);

        assert!(matches!(
            region.validate_allocation_request(&request),
            Err(RegionError::MemorySpaceMismatch { .. })
        ));
    }

    #[test]
    fn unspecified_memory_space_does_not_impose_a_restriction() {
        let region = test_region();
        let request = AllocationRequest::host(64);

        assert!(region
            .validate_allocation_request(&request)
            .is_ok());
    }

    #[test]
    fn validates_region_metadata() {
        let region = test_region()
            .with_name("workspace")
            .expect("valid name")
            .with_memory_space("memory::shared")
            .expect("valid space")
            .with_access([RegionAccess::Read])
            .with_metadata("origin", "semantic-analysis")
            .expect("valid metadata");

        assert!(region.validate().is_ok());
    }

    #[test]
    fn serializes_and_deserializes_valid_regions() {
        let region = test_region()
            .with_name("workspace")
            .expect("valid name")
            .with_memory_space("memory::shared")
            .expect("valid space")
            .with_access([RegionAccess::Read, RegionAccess::Write]);

        let serialized =
            serde_json::to_string(&region)
                .expect("serialization should succeed");

        let restored: MemoryRegion =
            serde_json::from_str(&serialized)
                .expect("valid region should deserialize");

        assert_eq!(region, restored);
    }

    #[test]
    fn region_identity_is_canonical_zuir_identity() {
        let region = test_region();

        let _: RegionId = region.id();
    }

    #[test]
    fn region_validation_does_not_claim_physical_allocation() {
        let region = test_region();

        // A descriptor is valid without an allocated memory handle.
        // Physical realization remains the allocator/provider's job.
        assert!(region.validate().is_ok());
    }
}
