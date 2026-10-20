
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani HDLIR — Logical Memory Declarations
//!
//! Path: `src/hdl/hdlir/ir/memories.rs`
//! Edition: Rust 2021.
//! Minimum supported Rust version: 1.97.
//! Safety: Safe Rust only.
//! Status: Proposed core implementation; freeze after repository integration.
//!
//! ## Purpose
//!
//! Represent logical memory declarations in Zamani's hardware-domain IR.
//! A declaration associates a typed memory identity with its owning logical
//! module, name, existing `MemoryType`, logical port bindings, and
//! schema-defined memory intent.
//!
//! ## Authority
//!
//! This module follows:
//!
//! - `grammar/spec/hdl.md`.
//! - `grammar/hdl/memories.g4`.
//! - `grammar/hdl/arrays.g4` for the separation between array expressions
//!   and memory dimensions.
//! - `grammar/hdl/hdl.g4` and the applicable grammar composition.
//! - `src/frontend/semantic/domains/hdl.rs`.
//! - `src/hdl/api.rs`.
//! - `src/hdl/ids.rs`.
//! - `src/hdl/hdlir/HDLIR-SPEC.md`.
//! - `src/hdl/hdlir/ids.rs`.
//! - `src/hdl/hdlir/errors.rs`.
//! - `src/hdl/hdlir/types/memory.rs`.
//! - `src/hdl/hdlir/ir/operations.rs`.
//!
//! The grammar and canonical frontend own source syntax and source semantics.
//! `types/memory.rs` owns logical memory type semantics. This module owns
//! memory declarations and their logical associations in the HDLIR graph.
//! ZUIR remains the universal computational representation.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Memory declaration identity, owning module identity, and logical name.
//! - The association between a declaration and its existing `MemoryType`.
//! - Named, logical memory-port bindings to HDLIR signals, values, or clock
//!   domains.
//! - Categorized, schema-defined memory clauses.
//! - A deterministic memory declaration table.
//! - Local consistency checks and graph-reference validation hooks.
//!
//! This module does not own:
//!
//! - Memory type definitions, dimensions, element types, or collision policy.
//! - Parsing, AST construction, name resolution, or source semantic truth.
//! - A second general-purpose expression language.
//! - Runtime memory contents or memory read/write execution.
//! - Physical memory allocation, banks, caches, controllers, DMA, placement,
//!   routing, synthesis, technology selection, or vendor primitives.
//! - Clock-tree synthesis, timing closure, or synchronizer insertion.
//! - The enclosing design, module, signal, value, or clock-domain registries.
//! - Serialization encoding or the interpretation of unknown schema payloads.
//!
//! ## Semantic rules
//!
//! A memory declaration is logical intent, not a physical allocation.
//! Its `MemoryType` remains authoritative for dimensions, element type,
//! logical port contracts, initialization, and read-during-write behavior.
//!
//! A port binding's name must match a port declared by that `MemoryType`.
//! The binding keys within that port are deliberately open-ended because the
//! grammar permits named bindings and properties without a closed list of
//! universal memory-port roles. Their meaning must be established by the
//! canonical semantic model or a compatible registered schema.
//!
//! A binding endpoint is a logical HDLIR reference, not a physical address,
//! pin, cell, or runtime handle. The enclosing verifier must establish that
//! every referenced entity exists and is legal in the declaration's scope.
//!
//! Memory clauses retain the distinction between properties, requirements,
//! constraints, preferences, hints, and capability requirements. Their
//! payload is not evaluated or interpreted by this module. In particular,
//! an opaque requirement must never be silently treated as a preference.
//!
//! ## Scalability and determinism
//!
//! No universal maximum is imposed on the number of memories, dimensions,
//! logical ports, bindings, clauses, or represented hardware resources.
//! Storage grows subject to available resources and the host's representable
//! collection sizes. No finite implementation can process literally infinite
//! input.
//!
//! Symbolic dimensions and other symbolic type facts remain in `MemoryType`.
//! This module never multiplies dimensions or prematurely evaluates them.
//! Memory IDs are identities, not vector indexes or hardware resource numbers.
//!
//! The declaration table uses ordered maps for deterministic iteration and
//! scoped name lookup. Binding maps use ordered keys. Clause order is
//! preserved because source ordering may be relevant to provenance or to a
//! schema-defined contract.
//!
//! ## Failure behavior
//!
//! Duplicate IDs, duplicate names within one module, malformed names,
//! invalid binding labels, duplicate bindings, unknown declared port names,
//! and inconsistent table indexes return structured HDLIR errors.
//!
//! Missing module, signal, value, or clock-domain references are reported
//! by the relevant graph-validation method. Removal does not silently rewrite
//! other graph references; the caller must re-verify the modified design.
//!
//! ## Integration
//!
//! - `ids.rs` owns all IDs used here.
//! - `types/memory.rs` owns `MemoryType` and logical memory-port semantics.
//! - `ir/operations.rs` owns the schema-neutral `OperationAttributeValue`
//!   payload model used by memory clauses.
//! - `ir/clocks.rs` owns clock-domain declarations.
//! - The enclosing design/graph verifier must call `validate_structure`,
//!   `validate_module_references`, and `validate_endpoint_references`.
//! - `verify/types.rs` must validate element types, dimensions, port contracts,
//!   and binding-specific types.
//! - `verify/connectivity.rs` must validate the connected entities, direction,
//!   driver/sink legality, and signal/value compatibility.
//! - `verify/clocks.rs` must resolve synchronous memory clocking intent and
//!   validate clock-domain references.
//! - `verify/resources.rs` must check resource requirements without confusing
//!   logical requirements with physical allocation.
//! - `lowering/from_semantic.rs` must preserve source semantics and provenance.
//! - Serialization must preserve symbolic type facts, all clause categories,
//!   binding names, binding endpoints, and declaration order.
//!
//! This module does not assume that these downstream consumers are already
//! implemented or registered.
//!
//! ## Freeze criteria
//!
//! - All public constructors enforce local invariants.
//! - Failed insertions leave the table unchanged.
//! - IDs and names are unique in their documented scopes.
//! - Bindings resolve to declared logical ports, without a closed binding-key
//!   vocabulary.
//! - External references are checked through explicit graph-level callbacks.
//! - Clause categories remain distinct and payloads remain uninterpreted here.
//! - Deterministic iteration and scoped lookup pass tests.
//! - The module compiles in the real module tree and repository checks pass.

use std::collections::BTreeMap;

use crate::hdl::hdlir::errors::{HdlirError, HdlirResult};
use crate::hdl::hdlir::ids::{
    HdlirClockDomainId, HdlirMemoryId, HdlirModuleId, HdlirSignalId, HdlirValueId,
};
use crate::hdl::hdlir::ir::operations::OperationAttributeValue;
use crate::hdl::hdlir::types::memory::MemoryType;

/// A logical endpoint that may be referenced by a memory-port binding.
///
/// The variant identifies the HDLIR namespace of the endpoint. It does not
/// prove that the endpoint exists, is visible from the memory's module, or
/// has a type and direction compatible with the binding.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum HdlirMemoryEndpoint {
    /// A declared logical HDLIR signal.
    Signal(HdlirSignalId),

    /// A value defined by the HDLIR graph.
    Value(HdlirValueId),

    /// A logical clock domain.
    ///
    /// This is useful for explicitly schema-defined clock bindings. It is not
    /// a physical clock pin or clock-tree resource.
    ClockDomain(HdlirClockDomainId),
}

impl HdlirMemoryEndpoint {
    /// Validate that the endpoint's typed identity is a valid HDLIR ID.
    ///
    /// The ID types themselves prevent zero-valued identities. This method
    /// exists as a stable local-validation hook and deliberately does not
    /// resolve the endpoint against a graph.
    pub fn validate_structure(self) -> HdlirResult<()> {
        match self {
            Self::Signal(id) if id.get() == 0 => Err(HdlirError::invalid_invariant(
                "memory.endpoint.signal_id_nonzero",
                "memory binding contains a zero signal ID",
            )),
            Self::Value(id) if id.get() == 0 => Err(HdlirError::invalid_invariant(
                "memory.endpoint.value_id_nonzero",
                "memory binding contains a zero value ID",
            )),
            Self::ClockDomain(id) if id.get() == 0 => {
                Err(HdlirError::invalid_invariant(
                    "memory.endpoint.clock_domain_id_nonzero",
                    "memory binding contains a zero clock-domain ID",
                ))
            }
            _ => Ok(()),
        }
    }
}

/// One logical memory-port binding.
///
/// `port_name` must match a `MemoryPortType` name in the owning memory's
/// `MemoryType`. The `bindings` map associates open-ended, schema-defined
/// binding names with logical HDLIR endpoints. No universal set of names
/// such as `address`, `data`, `enable`, or `mask` is hard-coded here.
///
/// An empty binding map is permitted as an intermediate declaration state.
/// A dialect or whole-graph verifier decides which bindings are mandatory
/// before the design can be accepted as complete.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HdlirMemoryPortBinding {
    port_name: String,
    bindings: BTreeMap<String, HdlirMemoryEndpoint>,
}

impl HdlirMemoryPortBinding {
    /// Construct a logical memory-port binding.
    ///
    /// The port name and binding names are preserved exactly. Empty names,
    /// whitespace-only names, and names with leading or trailing whitespace
    /// are rejected. Endpoint existence and binding semantics are graph-level
    /// checks.
    ///
    /// # Errors
    ///
    /// Returns an HDLIR error if a name is invalid or an endpoint fails local
    /// validation.
    pub fn new(
        port_name: impl Into<String>,
        bindings: BTreeMap<String, HdlirMemoryEndpoint>,
    ) -> HdlirResult<Self> {
        let port_name = port_name.into();
        validate_name("memory.port_binding.port_name", &port_name)?;

        for (name, endpoint) in &bindings {
            validate_name("memory.port_binding.binding_name", name)?;
            endpoint.validate_structure()?;
        }

        Ok(Self {
            port_name,
            bindings,
        })
    }

    /// Return the exact logical port name being bound.
    #[must_use]
    pub fn port_name(&self) -> &str {
        &self.port_name
    }

    /// Return the named endpoint bindings in deterministic key order.
    #[must_use]
    pub fn bindings(&self) -> &BTreeMap<String, HdlirMemoryEndpoint> {
        &self.bindings
    }

    /// Look up one named binding.
    #[must_use]
    pub fn binding(&self, name: &str) -> Option<HdlirMemoryEndpoint> {
        self.bindings.get(name).copied()
    }

    /// Validate local names and endpoint identity representations.
    ///
    /// This does not resolve references against a design graph.
    ///
    /// # Errors
    ///
    /// Returns an error if a name or endpoint is locally invalid.
    pub fn validate_structure(&self) -> HdlirResult<()> {
        validate_name("memory.port_binding.port_name", &self.port_name)?;

        for (name, endpoint) in &self.bindings {
            validate_name("memory.port_binding.binding_name", name)?;
            endpoint.validate_structure()?;
        }

        Ok(())
    }
}

/// Semantic category of a memory-local clause from the HDL grammar.
///
/// These categories are intentionally distinct. In particular, a preference
/// or hint is not a requirement, and a property is not automatically a
/// constraint. The canonical semantic model or registered schema owns the
/// interpretation of each clause's payload.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum HdlirMemoryClauseKind {
    /// A named memory property.
    Property,

    /// A mandatory semantic requirement.
    Requirement,

    /// A condition restricting the permitted realization space.
    Constraint,

    /// Non-mandatory implementation guidance.
    Preference,

    /// Non-binding implementation guidance.
    Hint,

    /// A requirement for a declared semantic capability.
    CapabilityRequirement,
}

/// One memory-local semantic clause.
///
/// The `value` payload is stored using HDLIR's existing schema-neutral
/// attribute model. For expression-backed clauses, lowering should use a
/// compatible `SymbolicExpressionPayload` or other explicitly registered
/// representation. This record does not decode, evaluate, or infer meaning
/// from opaque bytes.
///
/// `label` is optional for expression-style clauses. A property clause must
/// have a non-empty label. The clause order is retained by `HdlirMemory`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HdlirMemoryClause {
    kind: HdlirMemoryClauseKind,
    label: Option<String>,
    value: OperationAttributeValue,
}

impl HdlirMemoryClause {
    /// Construct a memory-local semantic clause.
    ///
    /// # Errors
    ///
    /// Returns an error if a provided label is invalid or if a property
    /// clause has no label.
    pub fn new(
        kind: HdlirMemoryClauseKind,
        label: Option<String>,
        value: OperationAttributeValue,
    ) -> HdlirResult<Self> {
        if let Some(label) = &label {
            validate_name("memory.clause.label", label)?;
        }

        if kind == HdlirMemoryClauseKind::Property && label.is_none() {
            return Err(HdlirError::invalid_input(
                "memory.clause.label",
                "a memory property clause requires a non-empty label",
            ));
        }

        Ok(Self { kind, label, value })
    }

    /// Return the clause category.
    #[must_use]
    pub const fn kind(&self) -> HdlirMemoryClauseKind {
        self.kind
    }

    /// Return the optional exact label.
    #[must_use]
    pub fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    /// Return the schema-neutral payload.
    ///
    /// The caller must use the registered contract appropriate to this
    /// clause's category before interpreting the payload.
    #[must_use]
    pub fn value(&self) -> &OperationAttributeValue {
        &self.value
    }

    /// Validate local category and label invariants.
    ///
    /// # Errors
    ///
    /// Returns an error if the label is invalid or a property has no label.
    pub fn validate_structure(&self) -> HdlirResult<()> {
        if let Some(label) = &self.label {
            validate_name("memory.clause.label", label)?;
        }

        if self.kind == HdlirMemoryClauseKind::Property && self.label.is_none() {
            return Err(HdlirError::invalid_invariant(
                "memory.property_has_label",
                "a property clause must have a non-empty label",
            ));
        }

        Ok(())
    }
}

/// A logical memory declaration in the HDLIR graph.
///
/// `memory_type` is the authoritative logical memory type from
/// `types/memory.rs`. The declaration does not copy or reinterpret its
/// dimensions, element type, clocking contracts, initialization, or collision
/// behavior.
///
/// `module_id` defines the logical module that owns this declaration.
/// Existence and ownership of the referenced module are checked by the
/// containing graph verifier.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HdlirMemory {
    id: HdlirMemoryId,
    module_id: HdlirModuleId,
    name: String,
    memory_type: MemoryType,
    port_bindings: Vec<HdlirMemoryPortBinding>,
    clauses: Vec<HdlirMemoryClause>,
}

impl HdlirMemory {
    /// Construct a logical memory declaration.
    ///
    /// Bindings may be incomplete during incremental construction, but every
    /// supplied binding must name a port declared by `memory_type`. A caller
    /// must run the appropriate completeness and graph-level checks before
    /// treating the enclosing design as verified.
    ///
    /// # Errors
    ///
    /// Returns an error for an invalid memory name, invalid local binding or
    /// clause, duplicate binding for one logical port, or binding to a port
    /// that is not declared by the memory type.
    pub fn new(
        id: HdlirMemoryId,
        module_id: HdlirModuleId,
        name: impl Into<String>,
        memory_type: MemoryType,
        port_bindings: Vec<HdlirMemoryPortBinding>,
        clauses: Vec<HdlirMemoryClause>,
    ) -> HdlirResult<Self> {
        let memory = Self {
            id,
            module_id,
            name: name.into(),
            memory_type,
            port_bindings,
            clauses,
        };

        memory.validate_structure()?;
        Ok(memory)
    }

    /// Return this memory declaration's identity.
    #[must_use]
    pub const fn id(&self) -> HdlirMemoryId {
        self.id
    }

    /// Return the ID of the logical module that owns this declaration.
    #[must_use]
    pub const fn module_id(&self) -> HdlirModuleId {
        self.module_id
    }

    /// Return the exact logical name of this declaration.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return the authoritative logical memory type.
    #[must_use]
    pub fn memory_type(&self) -> &MemoryType {
        &self.memory_type
    }

    /// Return the memory's port bindings in declaration order.
    #[must_use]
    pub fn port_bindings(&self) -> &[HdlirMemoryPortBinding] {
        &self.port_bindings
    }

    /// Return memory-local clauses in their retained order.
    #[must_use]
    pub fn clauses(&self) -> &[HdlirMemoryClause] {
        &self.clauses
    }

    /// Find a binding for a logical memory port by its exact name.
    #[must_use]
    pub fn port_binding(&self, port_name: &str) -> Option<&HdlirMemoryPortBinding> {
        self.port_bindings
            .iter()
            .find(|binding| binding.port_name() == port_name)
    }

    /// Validate local invariants without resolving external graph references.
    ///
    /// This verifies names, clause metadata, binding uniqueness, and that
    /// every supplied port binding names a port in `memory_type`. It does not
    /// prove that bindings are complete or that their endpoints exist.
    ///
    /// # Errors
    ///
    /// Returns a structured error if a local invariant is violated.
    pub fn validate_structure(&self) -> HdlirResult<()> {
        validate_name("memory.name", &self.name)?;

        let declared_ports = self.memory_type.ports();

        for (index, binding) in self.port_bindings.iter().enumerate() {
            binding.validate_structure()?;

            if !declared_ports
                .iter()
                .any(|port| port.name() == binding.port_name())
            {
                return Err(HdlirError::MissingEntity {
                    entity_kind: "declared memory type port",
                    identity: binding.port_name().to_owned(),
                    scope: Some(format!("memory `{}`", self.name)),
                });
            }

            if self.port_bindings[..index]
                .iter()
                .any(|previous| previous.port_name() == binding.port_name())
            {
                return Err(HdlirError::DuplicateEntity {
                    entity_kind: "HDLIR memory port binding",
                    identity: binding.port_name().to_owned(),
                    scope: Some(format!("memory `{}`", self.name)),
                });
            }
        }

        for clause in &self.clauses {
            clause.validate_structure()?;
        }

        Ok(())
    }

    /// Validate that every declared memory-type port has a binding record.
    ///
    /// This is a stronger completeness check than `validate_structure`.
    /// A dialect may define exceptions for intentionally unconnected or
    /// externally bound ports; such exceptions must be checked by that
    /// dialect's verifier rather than inferred here.
    ///
    /// # Errors
    ///
    /// Returns a missing-entity error for the first unbound declared port.
    pub fn validate_port_binding_completeness(&self) -> HdlirResult<()> {
        for port in self.memory_type.ports() {
            if self.port_binding(port.name()).is_none() {
                return Err(HdlirError::MissingEntity {
                    entity_kind: "memory port binding",
                    identity: port.name().to_owned(),
                    scope: Some(format!("memory `{}`", self.name)),
                });
            }
        }

        Ok(())
    }

    /// Validate the owning module reference.
    ///
    /// The callback must return true only if the module exists in the
    /// correct design/namespace and is a legal owner for this declaration.
    ///
    /// # Errors
    ///
    /// Returns a missing-entity error if the module cannot be resolved.
    pub fn validate_module_reference(
        &self,
        module_exists: impl FnOnce(HdlirModuleId) -> bool,
    ) -> HdlirResult<()> {
        if module_exists(self.module_id) {
            Ok(())
        } else {
            Err(HdlirError::MissingEntity {
                entity_kind: "HDLIR memory owner module",
                identity: self.module_id.to_string(),
                scope: Some(format!("memory `{}`", self.name)),
            })
        }
    }

    /// Validate all supplied signal, value, and clock-domain references.
    ///
    /// Each callback must resolve only its own identity namespace and enforce
    /// the required visibility rules. This method does not check type
    /// compatibility, directionality, or dialect-specific binding meaning.
    ///
    /// # Errors
    ///
    /// Returns a missing-entity error if any endpoint cannot be resolved.
    pub fn validate_endpoint_references<FS, FV, FC>(
        &self,
        mut signal_exists: FS,
        mut value_exists: FV,
        mut clock_domain_exists: FC,
    ) -> HdlirResult<()>
    where
        FS: FnMut(HdlirSignalId) -> bool,
        FV: FnMut(HdlirValueId) -> bool,
        FC: FnMut(HdlirClockDomainId) -> bool,
    {
        for port_binding in &self.port_bindings {
            for (binding_name, endpoint) in port_binding.bindings() {
                let exists = match endpoint {
                    HdlirMemoryEndpoint::Signal(id) => signal_exists(*id),
                    HdlirMemoryEndpoint::Value(id) => value_exists(*id),
                    HdlirMemoryEndpoint::ClockDomain(id) => clock_domain_exists(*id),
                };

                if !exists {
                    return Err(HdlirError::MissingEntity {
                        entity_kind: "memory binding endpoint",
                        identity: format!("{endpoint:?}"),
                        scope: Some(format!(
                            "memory `{}`, port `{}`, binding `{binding_name}`",
                            self.name,
                            port_binding.port_name()
                        )),
                    });
                }
            }
        }

        Ok(())
    }
}

/// Deterministic collection of HDLIR memory declarations.
///
/// IDs are unique within the table. Logical names are unique within their
/// owning module, while identical names in different modules are permitted.
/// The ordered indexes are implementation details and are checked against the
/// authoritative declaration table by `validate_structure`.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct HdlirMemoryTable {
    memories: BTreeMap<HdlirMemoryId, HdlirMemory>,
    by_module_and_name: BTreeMap<(HdlirModuleId, String), HdlirMemoryId>,
}

impl HdlirMemoryTable {
    /// Create an empty memory declaration table.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Return the number of declarations in the table.
    #[must_use]
    pub fn len(&self) -> usize {
        self.memories.len()
    }

    /// Return whether the table contains no declarations.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.memories.is_empty()
    }

    /// Insert a declaration without replacing an existing declaration.
    ///
    /// All local checks and uniqueness checks happen before mutation. A
    /// failed insertion leaves both indexes unchanged.
    ///
    /// # Errors
    ///
    /// Returns an error for an invalid declaration, duplicate ID, or duplicate
    /// name within the same module.
    pub fn insert(&mut self, memory: HdlirMemory) -> HdlirResult<()> {
        memory.validate_structure()?;

        if self.memories.contains_key(&memory.id()) {
            return Err(HdlirError::DuplicateEntity {
                entity_kind: "HDLIR memory ID",
                identity: memory.id().to_string(),
                scope: None,
            });
        }

        let name_key = (memory.module_id(), memory.name().to_owned());

        if self.by_module_and_name.contains_key(&name_key) {
            return Err(HdlirError::DuplicateEntity {
                entity_kind: "HDLIR memory name",
                identity: memory.name().to_owned(),
                scope: Some(format!("module {}", memory.module_id())),
            });
        }

        self.by_module_and_name.insert(name_key, memory.id());
        self.memories.insert(memory.id(), memory);

        Ok(())
    }

    /// Look up a declaration by its typed identity.
    #[must_use]
    pub fn get(&self, id: HdlirMemoryId) -> Option<&HdlirMemory> {
        self.memories.get(&id)
    }

    /// Look up a declaration by owning module and exact logical name.
    #[must_use]
    pub fn get_by_name(
        &self,
        module_id: HdlirModuleId,
        name: &str,
    ) -> Option<&HdlirMemory> {
        let id = self
            .by_module_and_name
            .get(&(module_id, name.to_owned()))?;

        self.memories.get(id)
    }

    /// Return whether a declaration ID exists in this table.
    #[must_use]
    pub fn contains(&self, id: HdlirMemoryId) -> bool {
        self.memories.contains_key(&id)
    }

    /// Remove and return a declaration by ID.
    ///
    /// This removes the declaration's local name-index entry. It does not
    /// rewrite references in operations, bindings, or other graph structures.
    /// The enclosing design must be re-verified after removal.
    pub fn remove(&mut self, id: HdlirMemoryId) -> Option<HdlirMemory> {
        let memory = self.memories.remove(&id)?;
        self.by_module_and_name
            .remove(&(memory.module_id(), memory.name().to_owned()));
        Some(memory)
    }

    /// Iterate over declarations in ascending memory-ID order.
    pub fn iter(&self) -> impl ExactSizeIterator<Item = &HdlirMemory> {
        self.memories.values()
    }

    /// Iterate over `(ID, declaration)` pairs in ascending ID order.
    pub fn iter_with_ids(
        &self,
    ) -> impl ExactSizeIterator<Item = (&HdlirMemoryId, &HdlirMemory)> {
        self.memories.iter()
    }

    /// Validate declaration invariants and both table indexes.
    ///
    /// This method checks local declaration structure and index consistency.
    /// It does not resolve owner modules or binding endpoints; use the
    /// reference-validation methods and the complete graph verifier for those.
    ///
    /// # Errors
    ///
    /// Returns an error if a declaration is malformed or an index is stale,
    /// inconsistent, or missing an expected entry.
    pub fn validate_structure(&self) -> HdlirResult<()> {
        if self.memories.len() != self.by_module_and_name.len() {
            return Err(HdlirError::invalid_invariant(
                "memory_table.index_cardinality",
                format!(
                    "memory table has {} declarations but {} name-index entries",
                    self.memories.len(),
                    self.by_module_and_name.len()
                ),
            ));
        }

        for (id, memory) in &self.memories {
            if *id != memory.id() {
                return Err(HdlirError::invalid_invariant(
                    "memory_table.key_identity",
                    format!(
                        "memory table key {id} does not match declaration ID {}",
                        memory.id()
                    ),
                ));
            }

            memory.validate_structure()?;

            let name_key = (memory.module_id(), memory.name().to_owned());
            if self.by_module_and_name.get(&name_key) != Some(id) {
                return Err(HdlirError::invalid_invariant(
                    "memory_table.name_index",
                    format!(
                        "name index for memory `{}` in module {} is inconsistent",
                        memory.name(),
                        memory.module_id()
                    ),
                ));
            }
        }

        for (name_key, id) in &self.by_module_and_name {
            let Some(memory) = self.memories.get(id) else {
                return Err(HdlirError::MissingEntity {
                    entity_kind: "memory declaration for name index",
                    identity: id.to_string(),
                    scope: Some(format!(
                        "module {}, memory `{}`",
                        name_key.0, name_key.1
                    )),
                });
            };

            if memory.module_id() != name_key.0 || memory.name() != name_key.1 {
                return Err(HdlirError::invalid_invariant(
                    "memory_table.name_index_identity",
                    format!(
                        "name index entry for module {} and memory `{}` points to ID {id}, \
                         whose declaration is module {} memory `{}`",
                        name_key.0,
                        name_key.1,
                        memory.module_id(),
                        memory.name()
                    ),
                ));
            }
        }

        Ok(())
    }

    /// Validate every memory's owning module reference.
    ///
    /// # Errors
    ///
    /// Returns the first missing or invalid module reference.
    pub fn validate_module_references(
        &self,
        mut module_exists: impl FnMut(HdlirModuleId) -> bool,
    ) -> HdlirResult<()> {
        self.validate_structure()?;

        for memory in self.memories.values() {
            memory.validate_module_reference(|module_id| module_exists(module_id))?;
        }

        Ok(())
    }

    /// Validate every endpoint referenced by every memory-port binding.
    ///
    /// The callbacks must check namespace membership and the required scope.
    /// This method does not replace type, direction, or dialect verification.
    ///
    /// # Errors
    ///
    /// Returns the first unresolved endpoint.
    pub fn validate_endpoint_references<FS, FV, FC>(
        &self,
        mut signal_exists: FS,
        mut value_exists: FV,
        mut clock_domain_exists: FC,
    ) -> HdlirResult<()>
    where
        FS: FnMut(HdlirSignalId) -> bool,
        FV: FnMut(HdlirValueId) -> bool,
        FC: FnMut(HdlirClockDomainId) -> bool,
    {
        self.validate_structure()?;

        for memory in self.memories.values() {
            memory.validate_endpoint_references(
                &mut signal_exists,
                &mut value_exists,
                &mut clock_domain_exists,
            )?;
        }

        Ok(())
    }
}

/// Validate a logical name without normalizing or rewriting it.
///
/// Empty names, whitespace-only names, and leading/trailing whitespace are
/// rejected. Exact equality and source-language name resolution remain the
/// responsibility of the owning semantic contract.
fn validate_name(field: &'static str, name: &str) -> HdlirResult<()> {
    if name.trim().is_empty() {
        return Err(HdlirError::invalid_input(
            field,
            "name must not be empty or whitespace-only",
        ));
    }

    if name.trim() != name {
        return Err(HdlirError::invalid_input(
            field,
            "name must not contain leading or trailing whitespace",
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frontend::semantic::domains::hdl::HdlId;
    use crate::hdl::hdlir::types::memory::{
        MemoryCollisionBehavior, MemoryDepth, MemoryInitialization, MemoryPortAccess,
        MemoryPortClocking, MemoryPortType, MemoryType,
    };
    use crate::hdl::hdlir::types::signal::SignalType;

    fn id_memory(raw: u128) -> HdlirMemoryId {
        HdlirMemoryId::new(raw).expect("non-zero memory ID")
    }

    fn id_module(raw: u128) -> HdlirModuleId {
        HdlirModuleId::new(raw).expect("non-zero module ID")
    }

    fn id_signal(raw: u128) -> HdlirSignalId {
        HdlirSignalId::new(raw).expect("non-zero signal ID")
    }

    fn id_value(raw: u128) -> HdlirValueId {
        HdlirValueId::new(raw).expect("non-zero value ID")
    }

    fn id_clock(raw: u128) -> HdlirClockDomainId {
        HdlirClockDomainId::new(raw).expect("non-zero clock-domain ID")
    }

    fn test_memory_type() -> MemoryType {
        let element_type = SignalType::named(
            HdlId::new("Word").expect("test type name is non-empty"),
        );

        let port = MemoryPortType::new(
            "rw",
            MemoryPortAccess::ReadWrite,
            MemoryPortClocking::Asynchronous,
        )
        .expect("valid memory port");

        MemoryType::new(
            element_type,
            vec![MemoryDepth::known(16).expect("positive depth")],
            vec![port],
            MemoryInitialization::Unspecified,
            MemoryCollisionBehavior::Unspecified,
        )
        .expect("valid memory type")
    }

    fn binding(port_name: &str) -> HdlirMemoryPortBinding {
        let mut values = BTreeMap::new();
        values.insert(
            "data".to_owned(),
            HdlirMemoryEndpoint::Signal(id_signal(7)),
        );
        HdlirMemoryPortBinding::new(port_name, values).expect("valid binding")
    }

    fn memory(
        id: u128,
        module_id: u128,
        name: &str,
        port_bindings: Vec<HdlirMemoryPortBinding>,
    ) -> HdlirMemory {
        HdlirMemory::new(
            id_memory(id),
            id_module(module_id),
            name,
            test_memory_type(),
            port_bindings,
            Vec::new(),
        )
        .expect("valid memory declaration")
    }

    #[test]
    fn constructs_a_logical_memory_without_selecting_physical_technology() {
        let memory = memory(1, 1, "buffer", vec![binding("rw")]);

        assert_eq!(memory.id(), id_memory(1));
        assert_eq!(memory.module_id(), id_module(1));
        assert_eq!(memory.name(), "buffer");
        assert_eq!(memory.memory_type().dimensions().len(), 1);
        assert_eq!(memory.port_binding("rw").unwrap().port_name(), "rw");
    }

    #[test]
    fn rejects_empty_or_whitespace_memory_names() {
        assert!(HdlirMemory::new(
            id_memory(1),
            id_module(1),
            "  ",
            test_memory_type(),
            Vec::new(),
            Vec::new(),
        )
        .is_err());
    }

    #[test]
    fn rejects_bindings_for_undeclared_ports() {
        let result = HdlirMemory::new(
            id_memory(1),
            id_module(1),
            "buffer",
            test_memory_type(),
            vec![binding("not_declared")],
            Vec::new(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn rejects_duplicate_bindings_for_one_declared_port() {
        let result = HdlirMemory::new(
            id_memory(1),
            id_module(1),
            "buffer",
            test_memory_type(),
            vec![binding("rw"), binding("rw")],
            Vec::new(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn binding_completeness_is_explicit() {
        let incomplete = memory(1, 1, "buffer", Vec::new());

        assert!(incomplete.validate_structure().is_ok());
        assert!(incomplete.validate_port_binding_completeness().is_err());

        let complete = memory(2, 1, "buffer2", vec![binding("rw")]);
        assert!(complete.validate_port_binding_completeness().is_ok());
    }

    #[test]
    fn preserves_distinct_memory_clause_categories() {
        let requirement = HdlirMemoryClause::new(
            HdlirMemoryClauseKind::Requirement,
            None,
            OperationAttributeValue::Boolean(true),
        )
        .expect("valid requirement clause");

        let preference = HdlirMemoryClause::new(
            HdlirMemoryClauseKind::Preference,
            None,
            OperationAttributeValue::Boolean(true),
        )
        .expect("valid preference clause");

        assert_ne!(requirement.kind(), preference.kind());
    }

    #[test]
    fn rejects_property_clause_without_a_label() {
        let result = HdlirMemoryClause::new(
            HdlirMemoryClauseKind::Property,
            None,
            OperationAttributeValue::Boolean(true),
        );

        assert!(result.is_err());
    }

    #[test]
    fn table_rejects_duplicate_ids_without_mutating_existing_data() {
        let mut table = HdlirMemoryTable::new();
        table
            .insert(memory(1, 1, "buffer", vec![binding("rw")]))
            .unwrap();

        let duplicate = table.insert(memory(1, 2, "other", vec![binding("rw")]));

        assert!(duplicate.is_err());
        assert_eq!(table.len(), 1);
        assert_eq!(table.get(id_memory(1)).unwrap().name(), "buffer");
    }

    #[test]
    fn names_are_unique_per_module_not_globally() {
        let mut table = HdlirMemoryTable::new();
        table
            .insert(memory(1, 1, "buffer", vec![binding("rw")]))
            .unwrap();

        assert!(table
            .insert(memory(2, 1, "buffer", vec![binding("rw")]))
            .is_err());

        table
            .insert(memory(3, 2, "buffer", vec![binding("rw")]))
            .unwrap();

        assert_eq!(table.len(), 2);
        assert_eq!(table.get_by_name(id_module(2), "buffer").unwrap().id(), id_memory(3));
    }

    #[test]
    fn iteration_is_deterministic_by_memory_id() {
        let mut table = HdlirMemoryTable::new();
        table
            .insert(memory(9, 1, "z", vec![binding("rw")]))
            .unwrap();
        table
            .insert(memory(2, 1, "a", vec![binding("rw")]))
            .unwrap();

        let ids: Vec<_> = table.iter().map(HdlirMemory::id).collect();

        assert_eq!(ids, vec![id_memory(2), id_memory(9)]);
    }

    #[test]
    fn removal_cleans_the_scoped_name_index() {
        let mut table = HdlirMemoryTable::new();
        table
            .insert(memory(1, 1, "buffer", vec![binding("rw")]))
            .unwrap();

        assert!(table.remove(id_memory(1)).is_some());
        assert!(table.get_by_name(id_module(1), "buffer").is_none());
        assert!(table.validate_structure().is_ok());
    }

    #[test]
    fn validates_module_and_endpoint_references() {
        let mut table = HdlirMemoryTable::new();
        table
            .insert(memory(1, 1, "buffer", vec![binding("rw")]))
            .unwrap();

        assert!(table
            .validate_module_references(|id| id == id_module(1))
            .is_ok());

        assert!(table
            .validate_endpoint_references(
                |id| id == id_signal(7),
                |_| false,
                |_| false,
            )
            .is_ok());

        assert!(table
            .validate_endpoint_references(
                |_| false,
                |_| false,
                |_| false,
            )
            .is_err());
    }

    #[test]
    fn supports_value_and_clock_domain_binding_namespaces() {
        let mut bindings = BTreeMap::new();
        bindings.insert("data".to_owned(), HdlirMemoryEndpoint::Value(id_value(2)));
        bindings.insert(
            "clock".to_owned(),
            HdlirMemoryEndpoint::ClockDomain(id_clock(3)),
        );

        let binding = HdlirMemoryPortBinding::new("rw", bindings).unwrap();

        assert_eq!(binding.binding("data"), Some(HdlirMemoryEndpoint::Value(id_value(2))));
        assert_eq!(
            binding.binding("clock"),
            Some(HdlirMemoryEndpoint::ClockDomain(id_clock(3)))
        );
    }

    #[test]
    fn validates_table_indexes_after_insertion_and_removal() {
        let mut table = HdlirMemoryTable::new();

        table
            .insert(memory(2, 1, "second", vec![binding("rw")]))
            .unwrap();
        table
            .insert(memory(1, 1, "first", vec![binding("rw")]))
            .unwrap();

        assert!(table.validate_structure().is_ok());

        table.remove(id_memory(2));

        assert!(table.validate_structure().is_ok());
    }
}
