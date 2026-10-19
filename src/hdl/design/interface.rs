
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani HDL — Logical Hardware Interfaces
//!
//! Path: `src/hdl/design/interface.rs`
//! Rust baseline: 1.97 or later; Rust 2021 edition.
//!
//! ## Purpose
//!
//! Represent target-independent, reusable HDL interface declarations.
//! Interfaces describe logical boundary members and references to related
//! semantic declarations. They do not describe physical connectors.
//!
//! ## Authority
//!
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/interfaces.g4`
//! - `grammar/hdl/ports.g4`
//! - `grammar/hdl/signals.g4`
//! - `grammar/hdl/hardware-modules.g4`
//! - `src/frontend/semantic/domains/hdl.rs`
//! - `src/hdl/design/port.rs`
//!
//! The grammar owns syntax, the canonical AST owns source structure, and
//! shared semantic infrastructure owns name resolution and semantic meaning.
//! This module owns interface declaration records and their local structural
//! validation.
//!
//! ## Does not own
//!
//! - Parsing or a replacement AST.
//! - General expression parsing or evaluation.
//! - A second semantic type, width, identity, or source-location system.
//! - General name resolution or generic specialization.
//! - Protocol implementation or proof of interface contracts.
//! - Physical pins, package connectors, device selection, or routing.
//! - Simulation, synthesis, placement, scheduling, or runtime execution.
//! - Any intermediate representation.
//!
//! All HDLIR implementation remains under `src/hdl/hdlir/` and is deferred.
//!
//! ## Scalability and determinism
//!
//! There are no fixed limits on interfaces, members, parent references,
//! protocol identifiers, or attributes. Collections grow according to
//! available resources. Inheritance-cycle checking is iterative rather than
//! recursive. Tables use ordered maps and sets for deterministic results.
//!
//! Resource exhaustion remains possible on finite machines; callers may
//! impose explicit operational budgets outside this data model. Such budgets
//! must not be misrepresented as language-level restrictions.
//!
//! ## Integration
//!
//! - `design/port.rs` may refer to an interface through `HdlId`.
//! - `design/module.rs` and the canonical AST adapter associate interface
//!   declarations with their owning declarations.
//! - Semantic analysis resolves IDs, checks type compatibility, evaluates
//!   requirements and guarantees, and validates generic specialization.
//! - `design/instance.rs` and connectivity analysis consume resolved interface
//!   definitions when validating instance bindings.
//! - Interoperability adapters map external interface declarations into this
//!   model and report information that cannot be preserved.
//!
//! This module does not depend on those downstream consumers.
//!
//! ## Important boundary
//!
//! A successfully constructed interface is locally well-formed. Construction
//! does not prove that referenced IDs resolve, that requirements are true,
//! that protocol behavior is correct, or that a particular target can realize
//! the interface. Those are separate semantic and feasibility questions.

use core::fmt;
use std::collections::{BTreeMap, BTreeSet};

use crate::frontend::semantic::domains::hdl::{
    HdlDirection, HdlId, HdlSourceLocation, HdlValueType,
};

/// Schema version of the logical interface data model.
///
/// This is independent of the language version, AST schema, semantic-model
/// schema, and any future HDLIR schema.
pub const HDL_INTERFACE_SCHEMA_VERSION: u16 = 1;

/// An interface declaration member.
///
/// `kind` is an extensible, namespaced semantic category rather than a closed
/// Rust enum. This permits the canonical AST adapter to represent interface
/// ports, signals, parameters, protocols, requirements, guarantees,
/// capabilities, contracts, nested interfaces, and future grammar-supported
/// member kinds without changing this type for each addition.
///
/// The canonical AST and semantic model remain authoritative for the
/// member's complete syntax and meaning. Optional fields here are normalized
/// facts or references, not replacement expression trees.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HdlInterfaceMember {
    id: HdlId,
    name: String,
    kind: String,
    direction: Option<HdlDirection>,
    value_type: Option<HdlValueType>,
    referenced_declaration: Option<HdlId>,
    attributes: BTreeMap<String, String>,
    location: Option<HdlSourceLocation>,
}

impl HdlInterfaceMember {
    /// Construct and validate an interface member.
    ///
    /// `kind` should be a stable namespaced value such as `hdl.port`,
    /// `hdl.signal`, `hdl.parameter`, `hdl.protocol`, or
    /// `hdl.requirement`. The vocabulary remains open for extensions.
    ///
    /// `referenced_declaration` is a semantic identity reference; this
    /// constructor does not resolve it.
    ///
    /// # Errors
    ///
    /// Returns an error for an empty identity, name, or kind; surrounding
    /// whitespace in a name or kind; an empty attribute key; or an invalid
    /// source location.
    pub fn new(
        id: HdlId,
        name: impl Into<String>,
        kind: impl Into<String>,
        direction: Option<HdlDirection>,
        value_type: Option<HdlValueType>,
        referenced_declaration: Option<HdlId>,
        attributes: BTreeMap<String, String>,
        location: Option<HdlSourceLocation>,
    ) -> Result<Self, HdlInterfaceError> {
        let member = Self {
            id,
            name: name.into(),
            kind: kind.into(),
            direction,
            value_type,
            referenced_declaration,
            attributes,
            location,
        };
        member.validate()?;
        Ok(member)
    }

    /// Return the canonical semantic identity of this member.
    #[must_use]
    pub fn id(&self) -> &HdlId {
        &self.id
    }

    /// Return the source-level member name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return the extensible semantic member kind.
    #[must_use]
    pub fn kind(&self) -> &str {
        &self.kind
    }

    /// Return the optional direction relative to the interface boundary.
    #[must_use]
    pub const fn direction(&self) -> Option<HdlDirection> {
        self.direction
    }

    /// Return the optional canonical semantic value type.
    #[must_use]
    pub fn value_type(&self) -> Option<&HdlValueType> {
        self.value_type.as_ref()
    }

    /// Return the optional referenced declaration identity.
    ///
    /// This is a reference, not proof that the declaration exists.
    #[must_use]
    pub fn referenced_declaration(&self) -> Option<&HdlId> {
        self.referenced_declaration.as_ref()
    }

    /// Return member attributes in deterministic key order.
    #[must_use]
    pub fn attributes(&self) -> &BTreeMap<String, String> {
        &self.attributes
    }

    /// Return the optional source location.
    #[must_use]
    pub fn location(&self) -> Option<&HdlSourceLocation> {
        self.location.as_ref()
    }

    /// Validate the member's local invariants.
    ///
    /// # Errors
    ///
    /// Returns the first local structural error in deterministic order.
    pub fn validate(&self) -> Result<(), HdlInterfaceError> {
        if self.name.trim().is_empty() {
            return Err(HdlInterfaceError::EmptyMemberName);
        }
        if self.name.trim() != self.name {
            return Err(HdlInterfaceError::SurroundingWhitespace {
                field: "member name",
            });
        }
        if self.kind.trim().is_empty() {
            return Err(HdlInterfaceError::EmptyMemberKind);
        }
        if self.kind.trim() != self.kind {
            return Err(HdlInterfaceError::SurroundingWhitespace {
                field: "member kind",
            });
        }
        if self
            .attributes
            .keys()
            .any(|key| key.trim().is_empty() || key.trim() != key)
        {
            return Err(HdlInterfaceError::InvalidAttributeKey);
        }
        if let Some(location) = &self.location {
            validate_location(location)?;
        }
        Ok(())
    }
}

/// A validated, target-independent logical HDL interface declaration.
///
/// `id` is supplied by the canonical frontend adapter. `name` preserves the
/// qualified source-level spelling in a normalized string form. The adapter
/// must retain the original syntax and source identity in the canonical AST.
///
/// `parents` represents interface inheritance or extension references.
/// `protocols`, `requirements`, `guarantees`, and `capabilities` contain
/// semantic identities or names normalized by the adapter. Their meaning is
/// resolved by semantic analysis, not inferred by this record.
///
/// Member order is preserved from the source declaration.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HdlInterface {
    id: HdlId,
    name: String,
    members: Vec<HdlInterfaceMember>,
    parents: Vec<HdlId>,
    protocols: Vec<String>,
    requirements: Vec<HdlId>,
    guarantees: Vec<HdlId>,
    capabilities: Vec<String>,
    attributes: BTreeMap<String, String>,
    location: Option<HdlSourceLocation>,
}

impl HdlInterface {
    /// Construct and locally validate an interface declaration.
    ///
    /// This constructor checks member identities and names within the
    /// interface, duplicate parent references, duplicate protocol and
    /// capability names, and local field invariants. It does not resolve
    /// external references or prove that inherited interfaces are valid.
    ///
    /// # Errors
    ///
    /// Returns an error for invalid names, duplicate members or references,
    /// invalid members, or an invalid source location.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: HdlId,
        name: impl Into<String>,
        members: Vec<HdlInterfaceMember>,
        parents: Vec<HdlId>,
        protocols: Vec<String>,
        requirements: Vec<HdlId>,
        guarantees: Vec<HdlId>,
        capabilities: Vec<String>,
        attributes: BTreeMap<String, String>,
        location: Option<HdlSourceLocation>,
    ) -> Result<Self, HdlInterfaceError> {
        let interface = Self {
            id,
            name: name.into(),
            members,
            parents,
            protocols,
            requirements,
            guarantees,
            capabilities,
            attributes,
            location,
        };
        interface.validate()?;
        Ok(interface)
    }

    /// Return the canonical semantic identity of this interface.
    #[must_use]
    pub fn id(&self) -> &HdlId {
        &self.id
    }

    /// Return the normalized qualified source-level name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return members in source declaration order.
    #[must_use]
    pub fn members(&self) -> &[HdlInterfaceMember] {
        &self.members
    }

    /// Return parent interface identities in source order.
    #[must_use]
    pub fn parents(&self) -> &[HdlId] {
        &self.parents
    }

    /// Return the open-world protocol identifiers in source order.
    #[must_use]
    pub fn protocols(&self) -> &[String] {
        &self.protocols
    }

    /// Return requirement identities in source order.
    #[must_use]
    pub fn requirements(&self) -> &[HdlId] {
        &self.requirements
    }

    /// Return guarantee identities in source order.
    #[must_use]
    pub fn guarantees(&self) -> &[HdlId] {
        &self.guarantees
    }

    /// Return the open-world capability identifiers in source order.
    #[must_use]
    pub fn capabilities(&self) -> &[String] {
        &self.capabilities
    }

    /// Return attributes in deterministic key order.
    #[must_use]
    pub fn attributes(&self) -> &BTreeMap<String, String> {
        &self.attributes
    }

    /// Return the optional source location.
    #[must_use]
    pub fn location(&self) -> Option<&HdlSourceLocation> {
        self.location.as_ref()
    }

    /// Find a member by its source-level name.
    ///
    /// Names are compared exactly. Case folding and language-specific name
    /// equivalence belong to the canonical semantic resolver.
    #[must_use]
    pub fn member(&self, name: &str) -> Option<&HdlInterfaceMember> {
        self.members.iter().find(|member| member.name() == name)
    }

    /// Validate local interface invariants, including values supplied by
    /// adapters or future deserialization.
    ///
    /// This does not resolve references or check inheritance across
    /// declarations; use [`HdlInterfaceTable::validate_references`] for
    /// table-wide structural checks.
    ///
    /// # Errors
    ///
    /// Returns the first local invariant violation in deterministic order.
    pub fn validate(&self) -> Result<(), HdlInterfaceError> {
        if self.name.trim().is_empty() {
            return Err(HdlInterfaceError::EmptyInterfaceName);
        }
        if self.name.trim() != self.name {
            return Err(HdlInterfaceError::SurroundingWhitespace {
                field: "interface name",
            });
        }

        let mut member_ids = BTreeSet::new();
        let mut member_names = BTreeSet::new();

        for member in &self.members {
            member.validate()?;

            if !member_ids.insert(member.id().clone()) {
                return Err(HdlInterfaceError::DuplicateMemberId(
                    member.id().clone(),
                ));
            }
            if !member_names.insert(member.name().to_owned()) {
                return Err(HdlInterfaceError::DuplicateMemberName(
                    member.name().to_owned(),
                ));
            }
        }

        ensure_unique_ids(&self.parents, "parent interface")?;
        ensure_unique_ids(&self.requirements, "requirement")?;
        ensure_unique_ids(&self.guarantees, "guarantee")?;
        ensure_unique_strings(&self.protocols, "protocol")?;
        ensure_unique_strings(&self.capabilities, "capability")?;

        if self
            .attributes
            .keys()
            .any(|key| key.trim().is_empty() || key.trim() != key)
        {
            return Err(HdlInterfaceError::InvalidAttributeKey);
        }

        if let Some(location) = &self.location {
            validate_location(location)?;
        }

        Ok(())
    }
}

/// Deterministic collection of interface declarations.
///
/// This is a design-layer table, not a replacement for the canonical
/// semantic symbol table. It indexes already-normalized declarations by
/// their canonical `HdlId` and offers explicit reference validation.
#[derive(Clone, Debug, Default)]
pub struct HdlInterfaceTable {
    interfaces: BTreeMap<HdlId, HdlInterface>,
}

impl HdlInterfaceTable {
    /// Create an empty interface table.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert an interface after local validation.
    ///
    /// # Errors
    ///
    /// Returns an error if the interface is invalid or its identity is
    /// already present. Existing entries are not replaced on failure.
    pub fn insert(
        &mut self,
        interface: HdlInterface,
    ) -> Result<(), HdlInterfaceError> {
        interface.validate()?;

        let id = interface.id().clone();
        if self.interfaces.contains_key(&id) {
            return Err(HdlInterfaceError::DuplicateInterfaceId(id));
        }

        self.interfaces.insert(id, interface);
        Ok(())
    }

    /// Return an interface by canonical identity.
    #[must_use]
    pub fn get(&self, id: &HdlId) -> Option<&HdlInterface> {
        self.interfaces.get(id)
    }

    /// Iterate over interfaces in deterministic identity order.
    pub fn iter(&self) -> impl Iterator<Item = (&HdlId, &HdlInterface)> {
        self.interfaces.iter()
    }

    /// Return the number of registered interfaces.
    #[must_use]
    pub fn len(&self) -> usize {
        self.interfaces.len()
    }

    /// Return whether the table is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.interfaces.is_empty()
    }

    /// Validate parent references and inheritance cycles.
    ///
    /// Every parent reference must identify a declaration in this table.
    /// Cycles are reported as structural errors. The traversal is iterative
    /// to avoid consuming the Rust call stack on deeply nested hierarchies.
    ///
    /// This function does not validate requirement or guarantee identities:
    /// those may be owned by other semantic registries and must be resolved
    /// by the shared semantic pipeline.
    ///
    /// # Errors
    ///
    /// Returns an unresolved-parent error or an inheritance-cycle error.
    pub fn validate_references(&self) -> Result<(), HdlInterfaceError> {
        for interface in self.interfaces.values() {
            for parent in interface.parents() {
                if !self.interfaces.contains_key(parent) {
                    return Err(HdlInterfaceError::UnresolvedParent {
                        interface: interface.id().clone(),
                        parent: parent.clone(),
                    });
                }
            }
        }

        // 0 = unseen, 1 = active on the current DFS path, 2 = complete.
        let mut state: BTreeMap<HdlId, u8> = BTreeMap::new();

        for root in self.interfaces.keys() {
            if state.get(root).copied().unwrap_or(0) != 0 {
                continue;
            }

            let mut stack: Vec<(HdlId, usize)> = vec![(root.clone(), 0)];
            state.insert(root.clone(), 1);

            while let Some((current, next_parent_index)) = stack.last_mut() {
                let parent_id = *current;
                let parents = self
                    .interfaces
                    .get(&parent_id)
                    .map(HdlInterface::parents)
                    .unwrap_or(&[]);

                if *next_parent_index >= parents.len() {
                    state.insert(parent_id, 2);
                    stack.pop();
                    continue;
                }

                let parent = parents[*next_parent_index].clone();
                *next_parent_index += 1;

                match state.get(&parent).copied().unwrap_or(0) {
                    0 => {
                        state.insert(parent.clone(), 1);
                        stack.push((parent, 0));
                    }
                    1 => {
                        return Err(HdlInterfaceError::InheritanceCycle {
                            interface: parent,
                        });
                    }
                    _ => {}
                }
            }
        }

        Ok(())
    }
}

/// Structural errors for the logical interface model.
///
/// These errors describe malformed model inputs or invalid structural
/// references. They are not substitutes for canonical source diagnostics,
/// type errors, failed contracts, or target-feasibility results.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HdlInterfaceError {
    /// The interface name is empty.
    EmptyInterfaceName,
    /// The member name is empty.
    EmptyMemberName,
    /// The member kind is empty.
    EmptyMemberKind,
    /// A name or kind has leading or trailing whitespace.
    SurroundingWhitespace {
        /// Field that violated the invariant.
        field: &'static str,
    },
    /// An attribute key is empty or has surrounding whitespace.
    InvalidAttributeKey,
    /// A source identity is empty.
    EmptySourceIdentity,
    /// A source range has an end before its start.
    InvalidSourceRange {
        /// Inclusive start offset.
        start: u64,
        /// Exclusive end offset.
        end: u64,
    },
    /// A member identity is duplicated within one interface.
    DuplicateMemberId(HdlId),
    /// A member name is duplicated within one interface.
    DuplicateMemberName(String),
    /// A parent, requirement, or guarantee identity is duplicated.
    DuplicateReference {
        /// Reference category.
        category: &'static str,
        /// Duplicated identity.
        id: HdlId,
    },
    /// A protocol or capability identifier is duplicated.
    DuplicateName {
        /// Identifier category.
        category: &'static str,
        /// Duplicated identifier.
        name: String,
    },
    /// An interface identity is already registered.
    DuplicateInterfaceId(HdlId),
    /// A parent interface reference cannot be resolved in the table.
    UnresolvedParent {
        /// Interface containing the unresolved reference.
        interface: HdlId,
        /// Missing parent identity.
        parent: HdlId,
    },
    /// An inheritance cycle exists.
    InheritanceCycle {
        /// Identity participating in the detected cycle.
        interface: HdlId,
    },
}

impl fmt::Display for HdlInterfaceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyInterfaceName => {
                f.write_str("HDL interface name cannot be empty")
            }
            Self::EmptyMemberName => {
                f.write_str("HDL interface member name cannot be empty")
            }
            Self::EmptyMemberKind => {
                f.write_str("HDL interface member kind cannot be empty")
            }
            Self::SurroundingWhitespace { field } => {
                write!(f, "HDL interface {field} has surrounding whitespace")
            }
            Self::InvalidAttributeKey => {
                f.write_str("HDL interface attribute key is invalid")
            }
            Self::EmptySourceIdentity => {
                f.write_str("HDL interface source identity cannot be empty")
            }
            Self::InvalidSourceRange { start, end } => {
                write!(f, "invalid HDL source range {start}..{end}")
            }
            Self::DuplicateMemberId(id) => {
                write!(f, "duplicate HDL interface member identity `{id}`")
            }
            Self::DuplicateMemberName(name) => {
                write!(f, "duplicate HDL interface member name `{name}`")
            }
            Self::DuplicateReference { category, id } => {
                write!(f, "duplicate HDL interface {category} reference `{id}`")
            }
            Self::DuplicateName { category, name } => {
                write!(f, "duplicate HDL interface {category} `{name}`")
            }
            Self::DuplicateInterfaceId(id) => {
                write!(f, "duplicate HDL interface identity `{id}`")
            }
            Self::UnresolvedParent { interface, parent } => write!(
                f,
                "HDL interface `{interface}` refers to unresolved parent `{parent}`"
            ),
            Self::InheritanceCycle { interface } => write!(
                f,
                "HDL interface inheritance cycle detected at `{interface}`"
            ),
        }
    }
}

impl std::error::Error for HdlInterfaceError {}

fn validate_location(
    location: &HdlSourceLocation,
) -> Result<(), HdlInterfaceError> {
    if location.source.trim().is_empty() {
        return Err(HdlInterfaceError::EmptySourceIdentity);
    }
    if location.end < location.start {
        return Err(HdlInterfaceError::InvalidSourceRange {
            start: location.start,
            end: location.end,
        });
    }
    Ok(())
}

fn ensure_unique_ids(
    ids: &[HdlId],
    category: &'static str,
) -> Result<(), HdlInterfaceError> {
    let mut seen = BTreeSet::new();
    for id in ids {
        if !seen.insert(id.clone()) {
            return Err(HdlInterfaceError::DuplicateReference {
                category,
                id: id.clone(),
            });
        }
    }
    Ok(())
}

fn ensure_unique_strings(
    values: &[String],
    category: &'static str,
) -> Result<(), HdlInterfaceError> {
    let mut seen = BTreeSet::new();
    for value in values {
        if value.trim().is_empty() || value.trim() != value {
            return Err(HdlInterfaceError::SurroundingWhitespace {
                field: "protocol or capability identifier",
            });
        }
        if !seen.insert(value) {
            return Err(HdlInterfaceError::DuplicateName {
                category,
                name: value.clone(),
            });
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn id(value: &str) -> HdlId {
        HdlId::new(value).expect("test identity should be valid")
    }

    fn interface(
        name: &str,
        identity: &str,
        parents: Vec<HdlId>,
    ) -> HdlInterface {
        HdlInterface::new(
            id(identity),
            name,
            Vec::new(),
            parents,
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            BTreeMap::new(),
            None,
        )
        .expect("test interface should be valid")
    }

    fn member(name: &str, identity: &str) -> HdlInterfaceMember {
        HdlInterfaceMember::new(
            id(identity),
            name,
            "hdl.signal",
            None,
            None,
            None,
            BTreeMap::new(),
            None,
        )
        .expect("test member should be valid")
    }

    #[test]
    fn accepts_empty_interface_when_language_semantics_allow_it() {
        let interface = interface("local::Empty", "interface.empty", Vec::new());
        assert!(interface.members().is_empty());
    }

    #[test]
    fn preserves_member_source_order() {
        let interface = HdlInterface::new(
            id("interface.bus"),
            "Bus",
            vec![member("valid", "member.valid"), member("data", "member.data")],
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            BTreeMap::new(),
            None,
        )
        .expect("interface should be valid");

        assert_eq!(interface.members()[0].name(), "valid");
        assert_eq!(interface.members()[1].name(), "data");
        assert_eq!(interface.member("data").map(HdlInterfaceMember::name), Some("data"));
    }

    #[test]
    fn rejects_duplicate_member_names() {
        let result = HdlInterface::new(
            id("interface.duplicate"),
            "Duplicate",
            vec![
                member("ready", "member.ready.1"),
                member("ready", "member.ready.2"),
            ],
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            BTreeMap::new(),
            None,
        );

        assert!(matches!(
            result,
            Err(HdlInterfaceError::DuplicateMemberName(name))
                if name == "ready"
        ));
    }

    #[test]
    fn rejects_duplicate_interface_identity() {
        let mut table = HdlInterfaceTable::new();
        table
            .insert(interface("First", "same.id", Vec::new()))
            .expect("first insert should succeed");

        assert!(matches!(
            table.insert(interface("Second", "same.id", Vec::new())),
            Err(HdlInterfaceError::DuplicateInterfaceId(_))
        ));
    }

    #[test]
    fn detects_unresolved_parent() {
        let mut table = HdlInterfaceTable::new();
        table
            .insert(interface("Child", "child", vec![id("missing.parent")]))
            .expect("local construction should succeed");

        assert!(matches!(
            table.validate_references(),
            Err(HdlInterfaceError::UnresolvedParent { .. })
        ));
    }

    #[test]
    fn accepts_valid_inheritance_chain() {
        let mut table = HdlInterfaceTable::new();
        table
            .insert(interface("Base", "base", Vec::new()))
            .expect("base insert should succeed");
        table
            .insert(interface("Child", "child", vec![id("base")]))
            .expect("child insert should succeed");

        assert!(table.validate_references().is_ok());
    }

    #[test]
    fn detects_inheritance_cycle() {
        let mut table = HdlInterfaceTable::new();
        table
            .insert(interface("A", "a", vec![id("b")]))
            .expect("A insert should succeed");
        table
            .insert(interface("B", "b", vec![id("a")]))
            .expect("B insert should succeed");

        assert!(matches!(
            table.validate_references(),
            Err(HdlInterfaceError::InheritanceCycle { .. })
        ));
    }

    #[test]
    fn reports_empty_table_as_valid() {
        let table = HdlInterfaceTable::new();
        assert!(table.validate_references().is_ok());
        assert!(table.is_empty());
        assert_eq!(table.len(), 0);
    }
}
