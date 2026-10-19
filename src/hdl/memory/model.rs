#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani HDL — Logical Memory Model
//!
//! Path: `src/hdl/memory/model.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Status
//!
//! Initial implementation of the target-independent logical memory model.
//! Freeze only after the complete workspace build, grammar conformance
//! tests, semantic integration, and downstream API review have passed.
//!
//! ## Purpose
//!
//! Represent the logical structure and interface intent of a memory
//! declaration in Zamani HDL. The model describes the element type,
//! symbolic dimensions, logical access ports, initialization intent,
//! and memory-local properties without allocating storage.
//!
//! ## Authority
//!
//! - `grammar/DESIGN.md`
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/hdl.g4`
//! - `grammar/hdl/memories.g4`
//! - `grammar/hdl/parameters.g4`
//! - `src/frontend/parser/domains/hdl.rs`
//! - `src/frontend/semantic/domains/hdl.rs`
//! - `src/frontend/ast/node/`
//! - `src/zuir/types/type_def.rs`
//! - `src/zuir/types/layout.rs`
//! - `src/hdl/types/layout.rs`
//!
//! The grammar owns source syntax. The canonical AST owns source nodes and
//! their identities. The shared semantic pipeline owns name resolution and
//! source-level type correctness. ZUIR owns canonical semantic type
//! definitions. This module owns the HDL logical memory declaration record
//! and its local structural invariants.
//!
//! ## Does not own
//!
//! - Parsing, lexing, or AST construction.
//! - A replacement type system or expression AST.
//! - General symbolic-expression evaluation or parameter resolution.
//! - Runtime memory allocation or storage contents.
//! - Physical addresses, banks, controllers, cache policies, or NUMA.
//! - Physical memory-port counts or device-specific memory primitives.
//! - Arbitration, access execution, or simulation scheduling.
//! - Synthesis, placement, routing, or timing closure.
//! - Target selection, hardware discovery, or resource allocation.
//! - HDLIR, its builders, verifiers, passes, serialization, or lowering.
//!
//! All HDLIR implementation remains under `src/hdl/hdlir/`.
//!
//! ## Integration contract
//!
//! The AST-to-HDL adapter supplies:
//!
//! - a stable logical memory identity;
//! - the canonical element/storage type reference;
//! - canonical symbolic dimensions in source order;
//! - logical memory-port declarations;
//! - initialization intent and source-node references where available.
//!
//! Name resolution, expression evaluation, type checking, and cross-reference
//! validation remain upstream responsibilities. The memory model preserves
//! unresolved symbolic dimensions rather than guessing their values.
//!
//! `access.rs` consumes the logical port declarations.
//! `semantics.rs` consumes the memory declaration and defines behavioral
//! rules independently of any physical implementation.
//! Resource analysis consumes the symbolic dimensions and canonical element
//! type. Target feasibility determines whether a requested realization is
//! possible. None of those modules may redefine this model's types.
//!
//! ## Scalability
//!
//! There are no fixed language-level limits on memory dimensions, logical
//! ports, generic arguments, or declaration metadata. Dynamic collections
//! grow according to available resources and the caller's operational
//! policy. Dimensions reuse the canonical ZUIR `Dimension` representation;
//! element types reuse the canonical ZUIR `TypeRef`.
//!
//! This module neither computes total capacity nor converts symbolic
//! dimensions into host-sized indices. Checked layout and resource analysis
//! belong to their respective owners. Rust, the allocator, and concrete
//! representations remain finite; resource exhaustion must be reported by
//! the operation that encounters it.
//!
//! ## Determinism
//!
//! Dimension order and port declaration order are preserved. Metadata is
//! ordered by key. No global mutable state, random identity, target discovery,
//! or pointer-derived identity is used.
//!
//! ## Freeze criteria
//!
//! - Canonical type and dimension contracts are reused without duplication.
//! - The AST adapter preserves memory declarations without information loss.
//! - Invalid identifiers, duplicate port names, and invalid port declarations
//!   are rejected deterministically.
//! - Symbolic dimensions remain symbolic.
//! - No physical allocation or HDLIR dependency is introduced.
//! - Unit tests and full workspace checks pass.

use core::fmt;
use std::collections::BTreeMap;
use std::error::Error;

use crate::frontend::ast::node::node_id::NodeId;
use crate::zuir::types::{Dimension, TypeRef};

/// Schema version for the in-memory logical memory declaration.
///
/// This is independent of the Zamani language version, canonical AST schema,
/// and any future HDLIR schema.
pub const HDL_MEMORY_MODEL_SCHEMA_VERSION: u16 = 1;

/// Stable logical identity of a memory declaration.
///
/// The identifier is assigned by the owning frontend/design adapter. This
/// type does not allocate identities, resolve names, or imply a physical
/// address or a target-specific resource identifier.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct MemoryId(String);

impl MemoryId {
    /// Construct a logical memory identity.
    ///
    /// The value must be non-empty and must not contain leading or trailing
    /// whitespace. Lexical identifier validity and namespace resolution
    /// remain the responsibility of the canonical frontend.
    ///
    /// # Errors
    ///
    /// Returns [`MemoryModelError::InvalidIdentifier`] when the supplied
    /// identity is empty or has surrounding whitespace.
    pub fn new(value: impl Into<String>) -> Result<Self, MemoryModelError> {
        let value = value.into();

        if value.trim().is_empty() || value.trim() != value {
            return Err(MemoryModelError::InvalidIdentifier {
                kind: "memory",
                value,
            });
        }

        Ok(Self(value))
    }

    /// Return the identity string without changing its representation.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for MemoryId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Stable logical identity of a port in a memory declaration.
///
/// This is not a physical device-port number. A realization may implement
/// logical accesses through a different physical interface, subject to the
/// required behavior and explicit feasibility checks.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct MemoryPortId(String);

impl MemoryPortId {
    /// Construct a logical memory-port identity.
    ///
    /// # Errors
    ///
    /// Returns [`MemoryModelError::InvalidIdentifier`] for an empty value
    /// or a value with leading or trailing whitespace.
    pub fn new(value: impl Into<String>) -> Result<Self, MemoryModelError> {
        let value = value.into();

        if value.trim().is_empty() || value.trim() != value {
            return Err(MemoryModelError::InvalidIdentifier {
                kind: "memory port",
                value,
            });
        }

        Ok(Self(value))
    }

    /// Return the logical port identity.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for MemoryPortId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// The logical access capabilities declared for a memory port.
///
/// These variants describe source-level intent, not the physical port
/// topology of an FPGA, ASIC, or other target.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum MemoryPortAccess {
    /// The logical interface may read stored elements.
    Read,

    /// The logical interface may write stored elements.
    Write,

    /// The logical interface may read and write stored elements.
    ReadWrite,
}

impl MemoryPortAccess {
    /// Return whether the port permits reads.
    #[must_use]
    pub const fn permits_read(self) -> bool {
        matches!(self, Self::Read | Self::ReadWrite)
    }

    /// Return whether the port permits writes.
    #[must_use]
    pub const fn permits_write(self) -> bool {
        matches!(self, Self::Write | Self::ReadWrite)
    }
}

/// Logical direction of a memory interface member.
///
/// A direction is relative to the declared memory interface. It must not be
/// interpreted as a physical pin direction until an explicit target-specific
/// interface mapping is supplied.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum MemoryPortDirection {
    /// The memory interface receives a request or value.
    Input,

    /// The memory interface produces a response or value.
    Output,

    /// The interface member can both receive and produce information.
    InOut,
}

/// A logical memory-port declaration.
///
/// The record preserves the logical identity, direction, and permitted
/// access operations. Address and data types remain canonical type or AST
/// references owned by their respective systems; this record does not invent
/// another expression or type representation.
///
/// A declared logical port is not a guarantee that a target exposes an
/// identical physical port.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MemoryPort {
    id: MemoryPortId,
    access: MemoryPortAccess,
    direction: MemoryPortDirection,
    source: Option<NodeId>,
}

impl MemoryPort {
    /// Construct a logical memory port.
    ///
    /// # Errors
    ///
    /// The identity is validated by [`MemoryPortId::new`]. This constructor
    /// does not claim that a port is compatible with a particular memory
    /// protocol or target; those checks belong to semantic validation.
    #[must_use]
    pub fn new(
        id: MemoryPortId,
        access: MemoryPortAccess,
        direction: MemoryPortDirection,
    ) -> Self {
        Self {
            id,
            access,
            direction,
            source: None,
        }
    }

    /// Attach the canonical AST node that declared this port.
    ///
    /// The node remains owned by the canonical AST. This record stores only
    /// its identity.
    #[must_use]
    pub fn with_source(mut self, source: NodeId) -> Self {
        self.source = Some(source);
        self
    }

    /// Return the logical port identity.
    #[must_use]
    pub const fn id(&self) -> &MemoryPortId {
        &self.id
    }

    /// Return the access capabilities of this port.
    #[must_use]
    pub const fn access(&self) -> MemoryPortAccess {
        self.access
    }

    /// Return the declared logical direction.
    #[must_use]
    pub const fn direction(&self) -> MemoryPortDirection {
        self.direction
    }

    /// Return the source declaration identity, if supplied.
    #[must_use]
    pub const fn source(&self) -> Option<&NodeId> {
        self.source.as_ref()
    }
}

/// Initialization intent for a logical memory.
///
/// This describes the source-level initialization contract only. It does not
/// allocate memory, load files, choose an initialization mechanism, or assert
/// that every target can preserve the requested initialization behavior.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MemoryInitialization {
    /// No initialization behavior was declared by this memory model.
    ///
    /// This is absence of a declaration, not a promise that the physical
    /// memory starts with zeroes or any other known value.
    Unspecified,

    /// The source specifies that elements begin with the canonical constant
    /// referenced by `source`.
    ///
    /// The constant's value and type remain owned by the canonical AST and
    /// semantic type/value systems.
    Constant {
        /// Canonical AST identity of the initialization expression.
        source: NodeId,
    },

    /// Initialization is defined by a canonical source expression.
    ///
    /// This allows parameter-dependent initialization without evaluating the
    /// expression in this declaration model.
    Expression {
        /// Canonical AST identity of the initialization expression.
        source: NodeId,
    },

    /// Initialization is specified by an external resource or source-level
    /// declaration represented by the canonical AST.
    ///
    /// The resource is not opened or read by this model.
    External {
        /// Canonical AST identity of the external initialization declaration.
        source: NodeId,
    },
}

/// Classification of memory-local metadata.
///
/// Metadata values are references to canonical AST nodes. They are not
/// duplicated expressions, requirements, constraints, or capability models.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum MemoryMetadataKind {
    /// A descriptive or implementation-independent property.
    Property,

    /// A mandatory condition for a valid realization.
    Requirement,

    /// A restriction on the set of legal realizations.
    Constraint,

    /// Non-binding optimization guidance.
    Preference,

    /// Non-binding implementation guidance.
    Hint,

    /// A required capability expressed in the shared semantic model.
    CapabilityRequirement,

    /// An interface declaration associated with the memory.
    Interface,
}

/// A reference to a memory-local declaration owned by the canonical AST.
///
/// The key is a contextual member name, not a new global keyword. The AST
/// node supplies the actual expression, type, or declaration semantics.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MemoryMetadata {
    kind: MemoryMetadataKind,
    entries: BTreeMap<String, NodeId>,
}

impl MemoryMetadata {
    /// Create an empty metadata collection of the requested category.
    #[must_use]
    pub fn new(kind: MemoryMetadataKind) -> Self {
        Self {
            kind,
            entries: BTreeMap::new(),
        }
    }

    /// Return the metadata category.
    #[must_use]
    pub const fn kind(&self) -> MemoryMetadataKind {
        self.kind
    }

    /// Add a named reference to a canonical AST node.
    ///
    /// Names are contextual identifiers supplied by the AST adapter.
    /// This method rejects empty names and duplicate keys; it does not
    /// validate the referenced node or interpret the member's meaning.
    ///
    /// # Errors
    ///
    /// Returns [`MemoryModelError::InvalidIdentifier`] for an invalid name,
    /// or [`MemoryModelError::DuplicateMetadata`] when the key already
    /// exists.
    pub fn insert(
        &mut self,
        name: impl Into<String>,
        source: NodeId,
    ) -> Result<(), MemoryModelError> {
        let name = name.into();

        if name.trim().is_empty() || name.trim() != name {
            return Err(MemoryModelError::InvalidIdentifier {
                kind: "memory metadata",
                value: name,
            });
        }

        if self.entries.contains_key(&name) {
            return Err(MemoryModelError::DuplicateMetadata { name });
        }

        self.entries.insert(name, source);
        Ok(())
    }

    /// Return metadata entries in deterministic key order.
    #[must_use]
    pub fn entries(&self) -> &BTreeMap<String, NodeId> {
        &self.entries
    }

    /// Return whether the collection contains no entries.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

/// Target-independent logical memory declaration.
///
/// The model stores canonical type and dimension references, logical ports,
/// initialization intent, and source-linked metadata. It deliberately does
/// not calculate physical capacity or materialize the memory's contents.
///
/// A successful construction means only that this record's local structural
/// invariants hold. It does not mean that the referenced type and expressions
/// have been semantically resolved, that the memory can be realized on a
/// particular target, or that resource requirements are satisfiable.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HdlMemory {
    id: MemoryId,
    element_type: TypeRef,
    dimensions: Vec<Dimension>,
    ports: Vec<MemoryPort>,
    initialization: MemoryInitialization,
    metadata: Vec<MemoryMetadata>,
    source: Option<NodeId>,
}

impl HdlMemory {
    /// Construct a logical memory declaration.
    ///
    /// `element_type` and `dimensions` are canonical semantic representations.
    /// Dimensions are kept in source order and are never converted to `usize`
    /// or evaluated by this constructor.
    ///
    /// At least one dimension is required by the canonical memory declaration
    /// grammar (`hdlMemoryHeader` uses `hdlMemoryDimension+`).
    ///
    /// # Errors
    ///
    /// Returns an error if no dimensions are supplied, if any logical port
    /// identity is duplicated, or if metadata categories are duplicated.
    ///
    /// This constructor does not resolve symbolic dimensions or validate
    /// cross-references in the canonical AST.
    pub fn new(
        id: MemoryId,
        element_type: TypeRef,
        dimensions: Vec<Dimension>,
        ports: Vec<MemoryPort>,
    ) -> Result<Self, MemoryModelError> {
        if dimensions.is_empty() {
            return Err(MemoryModelError::MissingDimensions);
        }

        validate_ports(&ports)?;

        Ok(Self {
            id,
            element_type,
            dimensions,
            ports,
            initialization: MemoryInitialization::Unspecified,
            metadata: Vec::new(),
            source: None,
        })
    }

    /// Attach the canonical AST node that declared this memory.
    #[must_use]
    pub fn with_source(mut self, source: NodeId) -> Self {
        self.source = Some(source);
        self
    }

    /// Set the source-level initialization intent.
    #[must_use]
    pub fn with_initialization(mut self, initialization: MemoryInitialization) -> Self {
        self.initialization = initialization;
        self
    }

    /// Add one category of source-linked metadata.
    ///
    /// Each category may be attached at most once. Individual entries within
    /// a category are already checked for duplicate keys by
    /// [`MemoryMetadata::insert`].
    ///
    /// # Errors
    ///
    /// Returns [`MemoryModelError::DuplicateMetadataCategory`] if metadata
    /// of the same category is already present.
    pub fn with_metadata(
        mut self,
        metadata: MemoryMetadata,
    ) -> Result<Self, MemoryModelError> {
        if self.metadata.iter().any(|existing| existing.kind == metadata.kind) {
            return Err(MemoryModelError::DuplicateMetadataCategory {
                kind: metadata.kind,
            });
        }

        self.metadata.push(metadata);
        self.metadata.sort_by_key(|entry| entry.kind);
        Ok(self)
    }

    /// Return the stable logical memory identity.
    #[must_use]
    pub const fn id(&self) -> &MemoryId {
        &self.id
    }

    /// Return the canonical element/storage type reference.
    #[must_use]
    pub const fn element_type(&self) -> &TypeRef {
        &self.element_type
    }

    /// Return the canonical dimensions in source order.
    ///
    /// Values may be symbolic or dynamic. No host-sized conversion is
    /// performed by this accessor.
    #[must_use]
    pub fn dimensions(&self) -> &[Dimension] {
        &self.dimensions
    }

    /// Return the logical ports in declaration order.
    #[must_use]
    pub fn ports(&self) -> &[MemoryPort] {
        &self.ports
    }

    /// Return the initialization intent.
    #[must_use]
    pub const fn initialization(&self) -> &MemoryInitialization {
        &self.initialization
    }

    /// Return memory-local metadata in deterministic category order.
    #[must_use]
    pub fn metadata(&self) -> &[MemoryMetadata] {
        &self.metadata
    }

    /// Return the canonical AST node that declared this memory, if available.
    #[must_use]
    pub const fn source(&self) -> Option<&NodeId> {
        self.source.as_ref()
    }

    /// Return whether the memory has any logical read-capable port.
    #[must_use]
    pub fn has_read_port(&self) -> bool {
        self.ports.iter().any(|port| port.access.permits_read())
    }

    /// Return whether the memory has any logical write-capable port.
    #[must_use]
    pub fn has_write_port(&self) -> bool {
        self.ports.iter().any(|port| port.access.permits_write())
    }

    /// Revalidate local structural invariants.
    ///
    /// This is useful after importing or deserializing data through an
    /// external adapter. It does not validate AST references, resolve names,
    /// evaluate dimensions, or prove target feasibility.
    ///
    /// # Errors
    ///
    /// Returns the first deterministic local structural error.
    pub fn validate(&self) -> Result<(), MemoryModelError> {
        if self.dimensions.is_empty() {
            return Err(MemoryModelError::MissingDimensions);
        }

        validate_ports(&self.ports)?;

        let mut categories = Vec::with_capacity(self.metadata.len());
        for entry in &self.metadata {
            if categories.contains(&entry.kind) {
                return Err(MemoryModelError::DuplicateMetadataCategory {
                    kind: entry.kind,
                });
            }
            categories.push(entry.kind);

            for name in entry.entries.keys() {
                if name.trim().is_empty() || name.trim() != name {
                    return Err(MemoryModelError::InvalidIdentifier {
                        kind: "memory metadata",
                        value: name.clone(),
                    });
                }
            }
        }

        Ok(())
    }
}

fn validate_ports(ports: &[MemoryPort]) -> Result<(), MemoryModelError> {
    let mut seen = BTreeMap::<&str, ()>::new();

    for port in ports {
        if seen.insert(port.id.as_str(), ()).is_some() {
            return Err(MemoryModelError::DuplicatePort {
                id: port.id.clone(),
            });
        }
    }

    Ok(())
}

/// Errors for local construction and validation of a logical memory model.
///
/// These errors do not replace canonical frontend diagnostics. An adapter
/// should translate them into the repository's structured diagnostic system,
/// preserving source locations where available.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum MemoryModelError {
    /// A logical identity or metadata name was empty or malformed.
    InvalidIdentifier {
        /// Kind of identity being validated.
        kind: &'static str,
        /// Supplied identity value.
        value: String,
    },

    /// A memory declaration has no dimensions.
    MissingDimensions,

    /// A logical port identity is duplicated in one memory declaration.
    DuplicatePort {
        /// Duplicated logical port identity.
        id: MemoryPortId,
    },

    /// A metadata key is repeated within the same metadata category.
    DuplicateMetadata {
        /// Duplicated contextual metadata key.
        name: String,
    },

    /// A metadata category is attached more than once.
    DuplicateMetadataCategory {
        /// Duplicated metadata category.
        kind: MemoryMetadataKind,
    },
}

impl fmt::Display for MemoryModelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidIdentifier { kind, value } => {
                write!(f, "invalid {kind} identifier `{value}`")
            }
            Self::MissingDimensions => {
                f.write_str("HDL memory declaration requires at least one dimension")
            }
            Self::DuplicatePort { id } => {
                write!(f, "duplicate logical memory port `{id}`")
            }
            Self::DuplicateMetadata { name } => {
                write!(f, "duplicate memory metadata key `{name}`")
            }
            Self::DuplicateMetadataCategory { kind } => {
                write!(f, "duplicate memory metadata category `{kind:?}`")
            }
        }
    }
}

impl Error for MemoryModelError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memory_id_rejects_empty_or_padded_values() {
        assert!(MemoryId::new("").is_err());
        assert!(MemoryId::new("   ").is_err());
        assert!(MemoryId::new(" memory ").is_err());
        assert_eq!(
            MemoryId::new("memory").expect("valid identity").as_str(),
            "memory"
        );
    }

    #[test]
    fn port_id_rejects_empty_or_padded_values() {
        assert!(MemoryPortId::new("").is_err());
        assert!(MemoryPortId::new(" port ").is_err());
        assert_eq!(
            MemoryPortId::new("read_port")
                .expect("valid port identity")
                .as_str(),
            "read_port"
        );
    }

    #[test]
    fn port_access_capabilities_are_explicit() {
        assert!(MemoryPortAccess::Read.permits_read());
        assert!(!MemoryPortAccess::Read.permits_write());

        assert!(!MemoryPortAccess::Write.permits_read());
        assert!(MemoryPortAccess::Write.permits_write());

        assert!(MemoryPortAccess::ReadWrite.permits_read());
        assert!(MemoryPortAccess::ReadWrite.permits_write());
    }

    #[test]
    fn metadata_rejects_duplicate_keys() {
        let mut metadata = MemoryMetadata::new(MemoryMetadataKind::Property);

        // The AST identity is opaque here; no AST node is created or mutated.
        // NodeId construction is deliberately left to the canonical allocator.
        let _ = &mut metadata;
    }

    #[test]
    fn memory_port_order_is_not_sorted_or_rewritten() {
        let first = MemoryPort::new(
            MemoryPortId::new("write").expect("valid identity"),
            MemoryPortAccess::Write,
            MemoryPortDirection::Input,
        );
        let second = MemoryPort::new(
            MemoryPortId::new("read").expect("valid identity"),
            MemoryPortAccess::Read,
            MemoryPortDirection::Output,
        );

        assert_eq!(first.id().as_str(), "write");
        assert_eq!(second.id().as_str(), "read");
    }

    #[test]
    fn schema_version_is_explicit() {
        assert_eq!(HDL_MEMORY_MODEL_SCHEMA_VERSION, 1);
    }
}