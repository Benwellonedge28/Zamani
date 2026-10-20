
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani HDLIR — Behavioral Processes
//!
//! Path: `src/hdl/hdlir/ir/processes.rs`
//! Edition: Rust 2021.
//! Minimum supported Rust version: Rust 1.97.
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Represent logical HDL processes and their event/sensitivity contracts.
//! A process refers to a body region owned by the HDLIR graph. Its execution
//! category and trigger metadata remain explicit until a dialect-aware
//! verifier establishes their legality.
//!
//! ## Authority
//!
//! This module follows:
//!
//! - `grammar/spec/hdl.md`.
//! - `grammar/hdl/hdl.g4`.
//! - `grammar/hdl/processes.g4`.
//! - `grammar/hdl/combinational.g4`.
//! - `grammar/hdl/sequential.g4`.
//! - `grammar/hdl/clocking.g4` and `grammar/hdl/clocks.g4`.
//! - `src/hdl/api.rs` and the canonical HDL semantic validator.
//! - `src/hdl/hdlir/HDLIR-SPEC.md`.
//! - `src/hdl/hdlir/ids.rs`.
//! - `src/hdl/hdlir/errors.rs`.
//! - `src/hdl/hdlir/ir/operations.rs`.
//! - `src/hdl/hdlir/ir/regions.rs`.
//! - `src/zuir/ZUIR-SPEC.md`.
//!
//! Grammar defines syntax; the canonical frontend owns source-level meaning;
//! ZUIR owns universal computational meaning; HDLIR owns this hardware-domain
//! process representation.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Process identity and optional logical name.
//! - Explicit process execution category.
//! - Ordered event/sensitivity specifications.
//! - Optional logical clock-domain association.
//! - The body-region reference.
//! - Schema-neutral process attribute storage.
//! - A deterministic, checked collection of process declarations.
//! - Local invariants that can be verified without the enclosing graph.
//!
//! This module does not own:
//!
//! - Parsing, AST construction, source-level name resolution, or semantic
//!   validation.
//! - The authoritative source-level `HdlProcess` representation.
//! - Region or block contents, operation execution, or value definitions.
//! - Clock/reset declarations, clock-tree synthesis, or CDC implementation.
//! - Assignment ordering, next-state computation, simulation, or scheduling.
//! - Physical registers, gates, memory cells, placement, or routing.
//! - Whole-design reference, type, connectivity, or process verification.
//!
//! ## Semantic model
//!
//! A process body is a reference to `HdlirRegionId`, not a copied body.
//! Event sources are typed references to a logical signal or HDLIR value.
//! The graph verifier must prove that the referenced entity exists and is
//! legal as an event source.
//!
//! `AnyChange`, `RisingEdge`, and `FallingEdge` describe logical trigger
//! intent. They do not choose a physical clock or imply a target-specific
//! implementation.
//!
//! A sequential process may express its clock relationship through event
//! metadata or an explicit clock-domain reference. This permits the grammar's
//! optional event specification without silently treating an unclocked,
//! otherwise-unspecified sequential process as valid.
//!
//! Process attributes are stored as `OperationAttributeValue` records to
//! reuse HDLIR's schema-neutral value representation. Attribute meaning,
//! allowed names, and version compatibility must be established by a
//! registered process/dialect contract. Opaque attributes must not be
//! interpreted as known semantics.
//!
//! ## Scalability and determinism
//!
//! There are no universal limits on process count, event count, or attribute
//! count. Collections grow subject to available resources and representable
//! memory. Higher-level operational budgets must report exhaustion explicitly.
//!
//! Event and insertion order is preserved. Attribute keys and identity indexes
//! use ordered maps. IDs are identities, not collection indices.
//!
//! ## Integration
//!
//! - `ir/mod.rs` must declare `pub mod processes;`.
//! - `ir/regions.rs` owns process body regions.
//! - `ir/operations.rs` owns operation records and schema-neutral attribute
//!   values.
//! - `ids.rs` owns process, region, clock-domain, signal, and value identities.
//! - The graph verifier must resolve region/event/clock references and verify
//!   process-kind, dialect, type, assignment, and clock-domain rules.
//! - The lowering adapter must preserve the source process kind, events,
//!   attributes, clock/reset meaning, and provenance.
//! - Builders must use the checked constructors and collection APIs.
//!
//! Local validation does not prove that a process is semantically legal or
//! that its body region exists.
//!
//! ## Freeze criteria
//!
//! - Constructors and collection mutations preserve all local invariants.
//! - Sequential, combinational, event-driven, and custom process categories
//!   remain distinct.
//! - Unknown custom semantics are not interpreted without a registered schema.
//! - Graph-wide verification covers all external references and behavior.
//! - Focused tests and repository-wide formatting, compilation, testing, and
//!   Clippy checks pass.

use std::collections::{BTreeMap, BTreeSet};

use crate::hdl::hdlir::errors::{HdlirError, HdlirResult};
use crate::hdl::hdlir::ids::{
    HdlirClockDomainId, HdlirProcessId, HdlirRegionId, HdlirSignalId,
    HdlirValueId,
};
use crate::hdl::hdlir::ir::operations::OperationAttributeValue;
use crate::hdl::hdlir::schema::registry::QualifiedName;

/// The execution category declared for a logical hardware process.
///
/// These variants classify intent; they do not define a closed list of all
/// possible process semantics. An extension-defined category must be backed
/// by a registered and version-compatible process contract before it can be
/// verified, transformed, simulated, or lowered.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum HdlirProcessKind {
    /// Combinational behavior without implicitly introduced persistent state.
    ///
    /// Event metadata, when present, must not declare edge-triggered behavior.
    Combinational,

    /// Sequential or state-transition behavior.
    ///
    /// A sequential process must declare an edge event or an explicit logical
    /// clock-domain association. This is a local consistency requirement, not
    /// proof that the referenced clock domain or state transitions are valid.
    Sequential,

    /// General event-driven behavior whose activation is described explicitly.
    EventDriven,

    /// Protocol-oriented behavior whose detailed contract is dialect-defined.
    Protocol,

    /// An extension-defined process category.
    ///
    /// The qualified name identifies the contract that defines its meaning.
    /// The name alone does not register or implement that contract.
    Custom(QualifiedName),
}

/// A logical source from which a process event may be observed.
///
/// The enclosing graph verifier must resolve the identity and confirm that
/// the referenced entity can legally serve as an event source.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum HdlirEventSource {
    /// A declared logical HDLIR signal.
    Signal(HdlirSignalId),

    /// A value defined by the HDLIR graph.
    ///
    /// The verifier must establish that the value's semantics permit event
    /// observation in the containing process dialect.
    Value(HdlirValueId),
}

/// The logical trigger condition associated with an event source.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum HdlirEventEdge {
    /// The event is triggered by a change permitted by the source semantics.
    AnyChange,

    /// The event is triggered by a logical low-to-high transition.
    RisingEdge,

    /// The event is triggered by a logical high-to-low transition.
    FallingEdge,
}

/// A qualifier attached to an event specification.
///
/// These are semantic qualifiers, not a request to insert a synchronizer or
/// implement a physical clock/reset network.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum HdlirEventQualifier {
    /// The event is associated with synchronous behavior.
    Synchronous,

    /// The event is associated with asynchronous behavior.
    Asynchronous,
}

/// One ordered event/sensitivity entry in a process declaration.
///
/// The source and edge define the trigger relationship. Qualifiers preserve
/// explicit source intent; the process/dialect verifier determines which
/// combinations are legal. Qualifier order is preserved as supplied.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HdlirProcessEvent {
    source: HdlirEventSource,
    edge: HdlirEventEdge,
    qualifiers: Vec<HdlirEventQualifier>,
}

impl HdlirProcessEvent {
    /// Construct an event and validate its local qualifier list.
    ///
    /// # Errors
    ///
    /// Returns an error if a qualifier is repeated. The source reference and
    /// the legality of its edge/qualifier combination require graph- and
    /// dialect-aware verification.
    pub fn new(
        source: HdlirEventSource,
        edge: HdlirEventEdge,
        qualifiers: Vec<HdlirEventQualifier>,
    ) -> HdlirResult<Self> {
        let mut seen = BTreeSet::new();

        for qualifier in &qualifiers {
            if !seen.insert(*qualifier) {
                return Err(HdlirError::DuplicateEntity {
                    entity_kind: "HDLIR process event qualifier",
                    identity: format!("{qualifier:?}"),
                    scope: Some(format!("{source:?}")),
                });
            }
        }

        Ok(Self {
            source,
            edge,
            qualifiers,
        })
    }

    /// Return the event source.
    #[must_use]
    pub const fn source(&self) -> HdlirEventSource {
        self.source
    }

    /// Return the logical trigger edge.
    #[must_use]
    pub const fn edge(&self) -> HdlirEventEdge {
        self.edge
    }

    /// Return event qualifiers in their declared order.
    #[must_use]
    pub fn qualifiers(&self) -> &[HdlirEventQualifier] {
        &self.qualifiers
    }

    /// Return whether the event declares synchronous behavior.
    #[must_use]
    pub fn is_synchronous(&self) -> bool {
        self.qualifiers
            .contains(&HdlirEventQualifier::Synchronous)
    }

    /// Return whether the event declares asynchronous behavior.
    #[must_use]
    pub fn is_asynchronous(&self) -> bool {
        self.qualifiers
            .contains(&HdlirEventQualifier::Asynchronous)
    }

    /// Validate local event invariants.
    ///
    /// # Errors
    ///
    /// Returns an error if the qualifier list contains duplicates.
    pub fn validate_structure(&self) -> HdlirResult<()> {
        let mut seen = BTreeSet::new();

        for qualifier in &self.qualifiers {
            if !seen.insert(*qualifier) {
                return Err(HdlirError::invalid_invariant(
                    "process_event.unique_qualifiers",
                    format!(
                        "event source {:?} repeats qualifier {:?}",
                        self.source, qualifier
                    ),
                ));
            }
        }

        Ok(())
    }
}

/// A logical hardware process declaration.
///
/// This is a graph record, not a process executor. Its body is owned by the
/// enclosing HDLIR graph and referenced by `body_region`. Its event sources
/// and optional clock domain are also resolved against that graph.
///
/// Attributes use HDLIR's existing schema-neutral attribute value model.
/// Their interpretation is delegated to the registered process/dialect
/// definition.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HdlirProcess {
    id: HdlirProcessId,
    name: Option<String>,
    kind: HdlirProcessKind,
    body_region: HdlirRegionId,
    events: Vec<HdlirProcessEvent>,
    clock_domain: Option<HdlirClockDomainId>,
    attributes: BTreeMap<String, OperationAttributeValue>,
}

impl HdlirProcess {
    /// Construct a process declaration and validate its local invariants.
    ///
    /// The body region, event sources, and clock domain are references; this
    /// constructor does not establish that they exist or belong to the same
    /// module. Those checks belong to whole-graph verification.
    ///
    /// A sequential process must provide at least one edge-triggered event or
    /// an explicit clock-domain reference. A combinational process cannot
    /// declare an edge-triggered event or a clock domain. Event-driven
    /// processes must declare at least one event. Other category-specific
    /// rules are owned by the registered dialect contract.
    ///
    /// # Errors
    ///
    /// Returns an error for an invalid name, repeated event specification,
    /// repeated/empty attribute name, or incompatible local process metadata.
    pub fn new(
        id: HdlirProcessId,
        name: Option<String>,
        kind: HdlirProcessKind,
        body_region: HdlirRegionId,
        events: Vec<HdlirProcessEvent>,
        clock_domain: Option<HdlirClockDomainId>,
        attributes: BTreeMap<String, OperationAttributeValue>,
    ) -> HdlirResult<Self> {
        let process = Self {
            id,
            name,
            kind,
            body_region,
            events,
            clock_domain,
            attributes,
        };

        process.validate_structure()?;
        Ok(process)
    }

    /// Return this process's identity.
    #[must_use]
    pub const fn id(&self) -> HdlirProcessId {
        self.id
    }

    /// Return the optional logical source-level name.
    #[must_use]
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Return the process execution category.
    #[must_use]
    pub fn kind(&self) -> &HdlirProcessKind {
        &self.kind
    }

    /// Return the body region identity.
    #[must_use]
    pub const fn body_region(&self) -> HdlirRegionId {
        self.body_region
    }

    /// Return the ordered event/sensitivity specification.
    #[must_use]
    pub fn events(&self) -> &[HdlirProcessEvent] {
        &self.events
    }

    /// Return the explicit logical clock-domain association, if any.
    #[must_use]
    pub const fn clock_domain(&self) -> Option<HdlirClockDomainId> {
        self.clock_domain
    }

    /// Return process attributes in deterministic key order.
    #[must_use]
    pub fn attributes(&self) -> &BTreeMap<String, OperationAttributeValue> {
        &self.attributes
    }

    /// Return one named attribute, if present.
    #[must_use]
    pub fn attribute(&self, name: &str) -> Option<&OperationAttributeValue> {
        self.attributes.get(name)
    }

    /// Return the number of event specifications.
    #[must_use]
    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    /// Return the number of process attributes.
    #[must_use]
    pub fn attribute_count(&self) -> usize {
        self.attributes.len()
    }

    /// Validate invariants that can be checked without the enclosing graph.
    ///
    /// This method does not verify body-region existence, event-source
    /// existence, clock-domain existence, source-level semantics, assignment
    /// legality, signal types, or dialect-defined attribute contracts.
    ///
    /// # Errors
    ///
    /// Returns a structured error if local metadata is malformed or violates
    /// the process-category rules documented on [`Self::new`].
    pub fn validate_structure(&self) -> HdlirResult<()> {
        if let Some(name) = &self.name {
            if name.trim().is_empty() {
                return Err(HdlirError::invalid_input(
                    "process.name",
                    "a declared process name must not be empty or whitespace-only",
                ));
            }

            if name.trim() != name {
                return Err(HdlirError::invalid_input(
                    "process.name",
                    "a process name must not contain leading or trailing whitespace",
                ));
            }
        }

        for attribute_name in self.attributes.keys() {
            if attribute_name.trim().is_empty() {
                return Err(HdlirError::invalid_input(
                    "process.attribute_name",
                    "a process attribute name must not be empty or whitespace-only",
                ));
            }

            if attribute_name.trim() != attribute_name {
                return Err(HdlirError::invalid_input(
                    "process.attribute_name",
                    "a process attribute name must not contain leading or trailing whitespace",
                ));
            }
        }

        let mut seen_events = BTreeSet::new();

        for event in &self.events {
            event.validate_structure()?;

            let identity = (event.source(), event.edge());
            if !seen_events.insert(identity) {
                return Err(HdlirError::DuplicateEntity {
                    entity_kind: "HDLIR process event",
                    identity: format!("{:?} with {:?}", event.source(), event.edge()),
                    scope: Some(self.id.to_string()),
                });
            }
        }

        match &self.kind {
            HdlirProcessKind::Combinational => {
                if self.clock_domain.is_some() {
                    return Err(HdlirError::invalid_input(
                        "process.clock_domain",
                        "a combinational process cannot declare a sequential clock domain",
                    ));
                }

                if self
                    .events
                    .iter()
                    .any(|event| event.edge() != HdlirEventEdge::AnyChange)
                {
                    return Err(HdlirError::invalid_input(
                        "process.events",
                        "a combinational process cannot declare edge-triggered events",
                    ));
                }
            }

            HdlirProcessKind::Sequential => {
                let has_edge_event = self
                    .events
                    .iter()
                    .any(|event| event.edge() != HdlirEventEdge::AnyChange);

                if !has_edge_event && self.clock_domain.is_none() {
                    return Err(HdlirError::invalid_input(
                        "process.clock_or_event",
                        "a sequential process requires an edge-triggered event or an explicit clock-domain association",
                    ));
                }
            }

            HdlirProcessKind::EventDriven => {
                if self.events.is_empty() {
                    return Err(HdlirError::invalid_input(
                        "process.events",
                        "an event-driven process requires at least one event specification",
                    ));
                }
            }

            HdlirProcessKind::Protocol | HdlirProcessKind::Custom(_) => {
                // The registered process/dialect contract owns further rules.
            }
        }

        Ok(())
    }
}

/// A deterministic collection of HDLIR process declarations.
///
/// Processes are stored in insertion order, while a `BTreeMap` provides
/// deterministic identity lookup. The index is an implementation detail:
/// process IDs are not vector indices and remain stable if records are
/// removed or collection order changes.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct HdlirProcessTable {
    processes: Vec<HdlirProcess>,
    by_id: BTreeMap<HdlirProcessId, usize>,
}

impl HdlirProcessTable {
    /// Create an empty process table.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Construct a process table from declarations.
    ///
    /// Input order is preserved. Construction stops at the first invalid
    /// declaration or duplicate identity; no partially built table is
    /// returned to the caller.
    ///
    /// # Errors
    ///
    /// Returns an error if any process is invalid or its identity is already
    /// present in the input.
    pub fn from_processes(
        processes: impl IntoIterator<Item = HdlirProcess>,
    ) -> HdlirResult<Self> {
        let mut table = Self::new();

        for process in processes {
            table.insert(process)?;
        }

        Ok(table)
    }

    /// Insert a process after checking its local invariants and identity.
    ///
    /// The operation is transactional: on error, neither the ordered storage
    /// nor the identity index is modified.
    ///
    /// # Errors
    ///
    /// Returns an error if the process is malformed or its ID is duplicated.
    pub fn insert(&mut self, process: HdlirProcess) -> HdlirResult<()> {
        process.validate_structure()?;

        let id = process.id();

        if self.by_id.contains_key(&id) {
            return Err(HdlirError::DuplicateEntity {
                entity_kind: "HDLIR process",
                identity: id.to_string(),
                scope: None,
            });
        }

        let index = self.processes.len();
        self.processes.push(process);
        self.by_id.insert(id, index);

        Ok(())
    }

    /// Remove a process by identity.
    ///
    /// Remaining process order is preserved. The identity index is rebuilt
    /// after removal so every index continues to refer to the correct record.
    ///
    /// # Errors
    ///
    /// Returns an error if the identity index is inconsistent with storage.
    pub fn remove(
        &mut self,
        id: HdlirProcessId,
    ) -> HdlirResult<Option<HdlirProcess>> {
        let Some(index) = self.by_id.get(&id).copied() else {
            return Ok(None);
        };

        if self.processes.get(index).map(HdlirProcess::id) != Some(id) {
            return Err(HdlirError::invalid_invariant(
                "process_table.identity_index",
                format!(
                    "identity index for process {id} does not match ordered storage"
                ),
            ));
        }

        let removed = self.processes.remove(index);
        self.rebuild_index()?;
        Ok(Some(removed))
    }

    /// Look up a process by identity.
    #[must_use]
    pub fn get(&self, id: HdlirProcessId) -> Option<&HdlirProcess> {
        self.by_id
            .get(&id)
            .and_then(|index| self.processes.get(*index))
    }

    /// Iterate over processes in insertion order.
    pub fn iter(&self) -> impl ExactSizeIterator<Item = &HdlirProcess> {
        self.processes.iter()
    }

    /// Iterate over process identities in insertion order.
    pub fn ids(&self) -> impl ExactSizeIterator<Item = HdlirProcessId> + '_ {
        self.processes.iter().map(HdlirProcess::id)
    }

    /// Return the number of stored processes.
    #[must_use]
    pub fn len(&self) -> usize {
        self.processes.len()
    }

    /// Return whether the table contains no processes.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.processes.is_empty()
    }

    /// Revalidate each process and the consistency of the identity index.
    ///
    /// This is useful after decoding, migration, or any operation that crosses
    /// a trust boundary. It does not perform graph-wide reference validation.
    ///
    /// # Errors
    ///
    /// Returns an error if a process is invalid, an identity is duplicated,
    /// or the lookup index disagrees with ordered storage.
    pub fn validate_structure(&self) -> HdlirResult<()> {
        if self.processes.len() != self.by_id.len() {
            return Err(HdlirError::invalid_invariant(
                "process_table.index_cardinality",
                format!(
                    "ordered storage contains {} processes but the identity index contains {} entries",
                    self.processes.len(),
                    self.by_id.len()
                ),
            ));
        }

        let mut seen = BTreeSet::new();

        for (index, process) in self.processes.iter().enumerate() {
            process.validate_structure()?;

            let id = process.id();

            if !seen.insert(id) {
                return Err(HdlirError::DuplicateEntity {
                    entity_kind: "HDLIR process",
                    identity: id.to_string(),
                    scope: None,
                });
            }

            if self.by_id.get(&id) != Some(&index) {
                return Err(HdlirError::invalid_invariant(
                    "process_table.identity_index",
                    format!(
                        "identity index for process {id} does not match its ordered position"
                    ),
                ));
            }
        }

        Ok(())
    }

    fn rebuild_index(&mut self) -> HdlirResult<()> {
        let mut index = BTreeMap::new();

        for (position, process) in self.processes.iter().enumerate() {
            if index.insert(process.id(), position).is_some() {
                return Err(HdlirError::DuplicateEntity {
                    entity_kind: "HDLIR process",
                    identity: process.id().to_string(),
                    scope: None,
                });
            }
        }

        self.by_id = index;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn process_id(raw: u128) -> HdlirProcessId {
        HdlirProcessId::new(raw).expect("test process identity must be non-zero")
    }

    fn region_id(raw: u128) -> HdlirRegionId {
        HdlirRegionId::new(raw).expect("test region identity must be non-zero")
    }

    fn signal_id(raw: u128) -> HdlirSignalId {
        HdlirSignalId::new(raw).expect("test signal identity must be non-zero")
    }

    fn clock_domain_id(raw: u128) -> HdlirClockDomainId {
        HdlirClockDomainId::new(raw)
            .expect("test clock-domain identity must be non-zero")
    }

    fn event(
        source: HdlirEventSource,
        edge: HdlirEventEdge,
    ) -> HdlirProcessEvent {
        HdlirProcessEvent::new(source, edge, Vec::new())
            .expect("test event must be locally valid")
    }

    fn process(
        id: u128,
        kind: HdlirProcessKind,
        events: Vec<HdlirProcessEvent>,
        clock_domain: Option<HdlirClockDomainId>,
    ) -> HdlirProcess {
        HdlirProcess::new(
            process_id(id),
            Some(format!("process_{id}")),
            kind,
            region_id(id),
            events,
            clock_domain,
            BTreeMap::new(),
        )
        .expect("test process must be locally valid")
    }

    #[test]
    fn accepts_combinational_process_without_stateful_triggers() {
        let value = process(
            1,
            HdlirProcessKind::Combinational,
            vec![event(
                HdlirEventSource::Signal(signal_id(1)),
                HdlirEventEdge::AnyChange,
            )],
            None,
        );

        assert_eq!(value.kind(), &HdlirProcessKind::Combinational);
        assert_eq!(value.event_count(), 1);
        assert!(value.validate_structure().is_ok());
    }

    #[test]
    fn rejects_edge_triggered_combinational_process() {
        let result = HdlirProcess::new(
            process_id(1),
            None,
            HdlirProcessKind::Combinational,
            region_id(1),
            vec![event(
                HdlirEventSource::Signal(signal_id(1)),
                HdlirEventEdge::RisingEdge,
            )],
            None,
            BTreeMap::new(),
        );

        assert!(matches!(result, Err(HdlirError::InvalidInput { .. })));
    }

    #[test]
    fn sequential_process_can_use_explicit_clock_domain_without_inline_event() {
        let value = process(
            1,
            HdlirProcessKind::Sequential,
            Vec::new(),
            Some(clock_domain_id(7)),
        );

        assert_eq!(value.clock_domain(), Some(clock_domain_id(7)));
        assert!(value.events().is_empty());
    }

    #[test]
    fn rejects_sequential_process_without_clock_or_edge_event() {
        let result = HdlirProcess::new(
            process_id(1),
            None,
            HdlirProcessKind::Sequential,
            region_id(1),
            Vec::new(),
            None,
            BTreeMap::new(),
        );

        assert!(matches!(result, Err(HdlirError::InvalidInput { .. })));
    }

    #[test]
    fn event_driven_process_requires_an_event() {
        let result = HdlirProcess::new(
            process_id(1),
            None,
            HdlirProcessKind::EventDriven,
            region_id(1),
            Vec::new(),
            None,
            BTreeMap::new(),
        );

        assert!(matches!(result, Err(HdlirError::InvalidInput { .. })));
    }

    #[test]
    fn rejects_duplicate_event_source_and_edge() {
        let source = HdlirEventSource::Signal(signal_id(1));
        let result = HdlirProcess::new(
            process_id(1),
            None,
            HdlirProcessKind::Sequential,
            region_id(1),
            vec![
                event(source, HdlirEventEdge::RisingEdge),
                event(source, HdlirEventEdge::RisingEdge),
            ],
            None,
            BTreeMap::new(),
        );

        assert!(matches!(result, Err(HdlirError::DuplicateEntity { .. })));
    }

    #[test]
    fn rejects_duplicate_event_qualifiers() {
        let source = HdlirEventSource::Signal(signal_id(1));
        let result = HdlirProcessEvent::new(
            source,
            HdlirEventEdge::AnyChange,
            vec![
                HdlirEventQualifier::Asynchronous,
                HdlirEventQualifier::Asynchronous,
            ],
        );

        assert!(matches!(result, Err(HdlirError::DuplicateEntity { .. })));
    }

    #[test]
    fn rejects_blank_process_names() {
        let result = HdlirProcess::new(
            process_id(1),
            Some("   ".to_owned()),
            HdlirProcessKind::Combinational,
            region_id(1),
            Vec::new(),
            None,
            BTreeMap::new(),
        );

        assert!(matches!(result, Err(HdlirError::InvalidInput { .. })));
    }

    #[test]
    fn process_table_preserves_insertion_order_and_lookup() {
        let first = process(1, HdlirProcessKind::Combinational, Vec::new(), None);
        let second = process(2, HdlirProcessKind::Combinational, Vec::new(), None);

        let table =
            HdlirProcessTable::from_processes([first.clone(), second.clone()])
                .expect("unique process IDs");

        assert_eq!(table.len(), 2);
        assert_eq!(table.get(process_id(1)), Some(&first));
        assert_eq!(table.get(process_id(2)), Some(&second));
        assert_eq!(
            table.ids().collect::<Vec<_>>(),
            vec![process_id(1), process_id(2)]
        );
        assert!(table.validate_structure().is_ok());
    }

    #[test]
    fn process_table_rejects_duplicate_identity_without_mutation() {
        let original =
            process(1, HdlirProcessKind::Combinational, Vec::new(), None);
        let duplicate =
            process(1, HdlirProcessKind::Combinational, Vec::new(), None);

        let mut table = HdlirProcessTable::new();
        table.insert(original.clone()).expect("first insertion");

        assert!(matches!(
            table.insert(duplicate),
            Err(HdlirError::DuplicateEntity { .. })
        ));

        assert_eq!(table.len(), 1);
        assert_eq!(table.get(process_id(1)), Some(&original));
        assert!(table.validate_structure().is_ok());
    }

    #[test]
    fn removal_preserves_order_and_rebuilds_index() {
        let first = process(1, HdlirProcessKind::Combinational, Vec::new(), None);
        let second = process(2, HdlirProcessKind::Combinational, Vec::new(), None);
        let third = process(3, HdlirProcessKind::Combinational, Vec::new(), None);

        let mut table = HdlirProcessTable::from_processes([
            first.clone(),
            second.clone(),
            third.clone(),
        ])
        .expect("unique process IDs");

        assert_eq!(table.remove(process_id(2)).expect("valid index"), Some(second));
        assert_eq!(
            table.ids().collect::<Vec<_>>(),
            vec![process_id(1), process_id(3)]
        );
        assert_eq!(table.get(process_id(3)), Some(&third));
        assert!(table.validate_structure().is_ok());
    }

    #[test]
    fn removing_unknown_identity_is_a_no_op() {
        let mut table = HdlirProcessTable::new();

        assert_eq!(table.remove(process_id(99)).expect("valid empty table"), None);
        assert!(table.is_empty());
    }
}
