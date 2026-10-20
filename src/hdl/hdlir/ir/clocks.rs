
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani HDLIR — Logical Clocks and Clock Domains
//!
//! Path: `src/hdl/hdlir/ir/clocks.rs`
//! Edition: Rust 2021.
//! Minimum supported Rust version: Rust 1.97.
//! Safety: Safe Rust only.
//! Status: Logical clock-domain foundation; requires integration and
//! repository-wide qualification before being declared production-ready.
//!
//! ## Authority
//!
//! This module follows the responsibilities established by:
//!
//! - `grammar/DESIGN.md`
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/clocks.g4`
//! - `grammar/hdl/clocking.g4`
//! - `grammar/hdl/processes.g4`
//! - `grammar/specification/semantic-model.md`
//! - `src/frontend/semantic/domains/hdl.rs`
//! - `src/hdl/api.rs`
//! - `src/hdl/ids.rs`
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/hdl/hdlir/HDLIR-SPEC.md`
//! - `src/hdl/hdlir/ids.rs`
//! - `src/hdl/hdlir/errors.rs`
//! - `src/hdl/hdlir/ir/processes.rs`
//!
//! The grammar owns clock syntax. The frontend owns source-level name
//! resolution and semantic validation. ZUIR owns universal computational
//! meaning. HDLIR owns this logical hardware-domain representation.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Logical clock-source references.
//! - Active-edge declarations.
//! - Optional reset-source and reset-polarity declarations.
//! - Logical clock-domain records.
//! - Optional logical parent-clock relationships.
//! - Deterministically ordered storage of clock-domain records.
//! - Local invariants and clock-domain reference/cycle checks.
//!
//! This module does not own:
//!
//! - Parsing, source ASTs, source names, or general name resolution.
//! - A competing source-span, diagnostic, semantic-ID, or type system.
//! - Physical clock pins, oscillators, PLLs, DLLs, clock buffers, or clock
//!   trees.
//! - Clock synthesis, scheduling, timing closure, placement, or routing.
//! - Clock-domain-crossing analysis or synchronizer insertion.
//! - Runtime clock control or target capability discovery.
//! - Evaluation of symbolic timing expressions.
//!
//! ## Integration contract
//!
//! `HdlirProcess` refers to `HdlirClockDomainId`; the enclosing design graph
//! must resolve that ID against this collection. The graph verifier must also
//! check source existence, module ownership, process/clock compatibility,
//! reset legality, generated-clock semantics, and dialect-specific rules.
//!
//! `HdlirClockDomains::validate_references` checks references and parent
//! cycles, but cannot prove source-level semantics or physical timing.
//!
//! Clock period, frequency, phase, duty cycle, jitter, uncertainty, gating,
//! multiplication, and division require explicit typed or schema-defined
//! constraint representations and lowering contracts. They must not be
//! inferred from this file's logical source/edge/reset metadata.
//!
//! ## Scalability and determinism
//!
//! There are no universal limits on the number of clock domains or hierarchy
//! depth. The collection grows subject to memory, available resources, and
//! representable IDs. Parent-cycle checking is iterative, not recursive.
//! Ordered maps make iteration deterministic by identifier.
//!
//! No physical resource is allocated here. An unresolved reference, cycle,
//! or exhausted downstream budget must be reported explicitly.
//!
//! ## Freeze criteria
//!
//! This file is ready to freeze only after:
//!
//! - `ir/mod.rs` registers it and the crate module tree compiles.
//! - The HDL semantic-to-HDLIR mapping preserves all supported clock facts.
//! - The graph verifier integrates source and module ownership checks.
//! - Timing constraints and generated-clock semantics have an explicit
//!   representation elsewhere under `src/hdl/hdlir/`.
//! - Unit, integration, malformed-graph, determinism, and scalability tests
//!   pass, along with repository-wide formatting, build, test, and Clippy.

use std::collections::{BTreeMap, BTreeSet};

use crate::hdl::hdlir::errors::{HdlirError, HdlirResult};
use crate::hdl::hdlir::ids::{
    HdlirClockDomainId, HdlirSignalId, HdlirValueId,
};

/// A logical entity that may act as a clock or reset source.
///
/// This is a reference into the containing HDLIR graph, not a declaration
/// of a physical pin or clock-generating device. The graph verifier must
/// resolve the referenced entity and validate its use in context.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum HdlirClockSource {
    /// A declared logical HDLIR signal.
    Signal(HdlirSignalId),

    /// An HDLIR value whose registered semantics permit clock observation.
    ///
    /// A value ID alone does not establish that the value can serve as a
    /// clock. The graph and dialect verifiers must prove that contract.
    Value(HdlirValueId),
}

impl HdlirClockSource {
    /// Validate the local representation of this source.
    ///
    /// Typed IDs already reject zero. Existence and legal clock use require
    /// the containing graph and registered dialect definitions.
    pub fn validate_structure(self) -> HdlirResult<()> {
        match self {
            Self::Signal(id) if id.get() == 0 => Err(HdlirError::invalid_invariant(
                "clock_source.nonzero_signal_id",
                "a clock signal reference must use a valid non-zero ID",
            )),
            Self::Value(id) if id.get() == 0 => Err(HdlirError::invalid_invariant(
                "clock_source.nonzero_value_id",
                "a clock value reference must use a valid non-zero ID",
            )),
            _ => Ok(()),
        }
    }
}

/// The logical transition that activates edge-triggered behavior.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum HdlirClockEdge {
    /// Activate on a logical low-to-high transition.
    Rising,

    /// Activate on a logical high-to-low transition.
    Falling,
}

/// The active level of a reset source.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum HdlirResetPolarity {
    /// Reset is asserted when the reset source is logically low.
    ActiveLow,

    /// Reset is asserted when the reset source is logically high.
    ActiveHigh,
}

/// The declared timing relationship of a reset to the clock.
///
/// This describes logical intent only. It does not implement reset
/// synchronization or prove that a reset network is physically safe.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum HdlirResetBehavior {
    /// Reset assertion/deassertion is governed by the clocking contract.
    Synchronous,

    /// Reset is declared asynchronous relative to the clock.
    Asynchronous,
}

/// A reset declaration associated with a logical clock domain.
///
/// The reset source is a logical signal or value. The enclosing graph
/// verifier must resolve it and validate its type, ownership, and use.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct HdlirClockReset {
    source: HdlirClockSource,
    polarity: HdlirResetPolarity,
    behavior: HdlirResetBehavior,
}

impl HdlirClockReset {
    /// Construct reset metadata for a clock domain.
    ///
    /// This constructor checks local representation invariants only. It does
    /// not establish that the source exists or that the selected reset
    /// behavior is legal for a process or target.
    pub fn new(
        source: HdlirClockSource,
        polarity: HdlirResetPolarity,
        behavior: HdlirResetBehavior,
    ) -> HdlirResult<Self> {
        source.validate_structure()?;

        Ok(Self {
            source,
            polarity,
            behavior,
        })
    }

    /// Return the logical reset source.
    #[must_use]
    pub const fn source(self) -> HdlirClockSource {
        self.source
    }

    /// Return the reset's active polarity.
    #[must_use]
    pub const fn polarity(self) -> HdlirResetPolarity {
        self.polarity
    }

    /// Return the declared reset behavior.
    #[must_use]
    pub const fn behavior(self) -> HdlirResetBehavior {
        self.behavior
    }
}

/// One logical HDLIR clock-domain declaration.
///
/// A clock domain captures the clock source and active edge, optional reset
/// metadata, and an optional parent-domain reference. It does not by itself
/// describe a generated clock's complete frequency, phase, duty-cycle,
/// jitter, uncertainty, or gating contract.
///
/// `name` is optional descriptive metadata. Name resolution and the scope in
/// which names must be unique remain the responsibility of the containing
/// module/design and the canonical frontend.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HdlirClockDomain {
    id: HdlirClockDomainId,
    name: Option<String>,
    source: HdlirClockSource,
    active_edge: HdlirClockEdge,
    reset: Option<HdlirClockReset>,
    parent: Option<HdlirClockDomainId>,
}

impl HdlirClockDomain {
    /// Construct a logical clock-domain declaration.
    ///
    /// # Errors
    ///
    /// Returns an error if the optional name is empty or whitespace-only,
    /// the source representation is invalid, or the domain directly names
    /// itself as its parent.
    ///
    /// Parent existence, indirect parent cycles, source existence, module
    /// ownership, and clock/reset semantics require graph-level verification.
    pub fn new(
        id: HdlirClockDomainId,
        name: Option<String>,
        source: HdlirClockSource,
        active_edge: HdlirClockEdge,
        reset: Option<HdlirClockReset>,
        parent: Option<HdlirClockDomainId>,
    ) -> HdlirResult<Self> {
        if name.as_ref().is_some_and(|value| value.trim().is_empty()) {
            return Err(HdlirError::invalid_input(
                "clock_domain.name",
                "a provided clock-domain name must not be empty or whitespace-only",
            ));
        }

        source.validate_structure()?;

        if let Some(reset) = reset {
            reset.source.validate_structure()?;
        }

        if parent == Some(id) {
            return Err(HdlirError::invalid_invariant(
                "clock_domain.not_own_parent",
                format!("clock domain {id} cannot be its own parent"),
            ));
        }

        Ok(Self {
            id,
            name,
            source,
            active_edge,
            reset,
            parent,
        })
    }

    /// Return the clock-domain identity.
    #[must_use]
    pub const fn id(&self) -> HdlirClockDomainId {
        self.id
    }

    /// Return the optional descriptive name.
    #[must_use]
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Return the logical clock source.
    #[must_use]
    pub const fn source(&self) -> HdlirClockSource {
        self.source
    }

    /// Return the active clock edge.
    #[must_use]
    pub const fn active_edge(&self) -> HdlirClockEdge {
        self.active_edge
    }

    /// Return optional reset metadata.
    #[must_use]
    pub const fn reset(&self) -> Option<HdlirClockReset> {
        self.reset
    }

    /// Return the optional parent clock-domain identity.
    #[must_use]
    pub const fn parent(&self) -> Option<HdlirClockDomainId> {
        self.parent
    }

    /// Validate this record's local invariants.
    ///
    /// This method deliberately does not resolve references against a design.
    pub fn validate_structure(&self) -> HdlirResult<()> {
        if self.name.as_ref().is_some_and(|value| value.trim().is_empty()) {
            return Err(HdlirError::invalid_invariant(
                "clock_domain.name_nonempty",
                format!("clock domain {} has an empty descriptive name", self.id),
            ));
        }

        self.source.validate_structure()?;

        if let Some(reset) = self.reset {
            reset.source.validate_structure()?;
        }

        if self.parent == Some(self.id) {
            return Err(HdlirError::invalid_invariant(
                "clock_domain.not_own_parent",
                format!("clock domain {} names itself as parent", self.id),
            ));
        }

        Ok(())
    }
}

/// An ordered collection of logical clock domains.
///
/// The map key is the typed clock-domain identity. Its ordering makes
/// iteration deterministic and avoids relying on hash-map iteration order.
/// IDs are references, not array indexes or physical clock resource numbers.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct HdlirClockDomains {
    domains: BTreeMap<HdlirClockDomainId, HdlirClockDomain>,
}

impl HdlirClockDomains {
    /// Create an empty clock-domain collection.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            domains: BTreeMap::new(),
        }
    }

    /// Insert a clock domain without replacing an existing record.
    ///
    /// Validation occurs before mutation. A failed insertion leaves the
    /// collection unchanged.
    ///
    /// # Errors
    ///
    /// Returns an error if the record is locally invalid or its ID already
    /// exists in this collection. Parent existence and cycle validation can
    /// be performed after related domains have been inserted.
    pub fn insert(&mut self, domain: HdlirClockDomain) -> HdlirResult<()> {
        domain.validate_structure()?;

        if self.domains.contains_key(&domain.id()) {
            return Err(HdlirError::DuplicateEntity {
                entity_kind: "HDLIR clock domain",
                identity: domain.id().to_string(),
                scope: None,
            });
        }

        self.domains.insert(domain.id(), domain);
        Ok(())
    }

    /// Remove and return a clock domain by ID.
    ///
    /// Removal does not rewrite references from processes or child clock
    /// domains. Callers must run graph verification before accepting a
    /// modified design as valid.
    pub fn remove(
        &mut self,
        id: HdlirClockDomainId,
    ) -> Option<HdlirClockDomain> {
        self.domains.remove(&id)
    }

    /// Look up a clock domain by its typed identity.
    #[must_use]
    pub fn get(&self, id: HdlirClockDomainId) -> Option<&HdlirClockDomain> {
        self.domains.get(&id)
    }

    /// Return domains in deterministic ascending-ID order.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = (&HdlirClockDomainId, &HdlirClockDomain)> {
        self.domains.iter()
    }

    /// Return the number of clock-domain records.
    #[must_use]
    pub fn len(&self) -> usize {
        self.domains.len()
    }

    /// Return whether this collection has no clock-domain records.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.domains.is_empty()
    }

    /// Validate local record invariants and parent-domain relationships.
    ///
    /// This verifies that every parent reference resolves within this
    /// collection and that parent relationships contain no cycles. It does
    /// not validate clock-source IDs, reset-source IDs, process references,
    /// module ownership, or timing semantics.
    ///
    /// The cycle check is iterative and does not consume call-stack space
    /// proportional to the clock hierarchy depth.
    pub fn validate_structure(&self) -> HdlirResult<()> {
        for domain in self.domains.values() {
            domain.validate_structure()?;

            if let Some(parent) = domain.parent() {
                if !self.domains.contains_key(&parent) {
                    return Err(HdlirError::MissingEntity {
                        entity_kind: "parent clock domain",
                        identity: parent.to_string(),
                        scope: Some(format!("clock domain {}", domain.id())),
                    });
                }
            }
        }

        self.validate_parent_cycles()
    }

    /// Validate clock and reset source references against the enclosing graph.
    ///
    /// `source_exists` must return true only if the referenced entity exists
    /// in the correct identity namespace and is visible in the required
    /// scope. This callback does not validate whether an entity is semantically
    /// legal as a clock or reset; the graph/dialect verifier must do that.
    ///
    /// Parent references and cycles are checked by `validate_structure`.
    pub fn validate_references<F>(&self, mut source_exists: F) -> HdlirResult<()>
    where
        F: FnMut(HdlirClockSource) -> bool,
    {
        self.validate_structure()?;

        for domain in self.domains.values() {
            if !source_exists(domain.source()) {
                return Err(HdlirError::MissingEntity {
                    entity_kind: "clock source",
                    identity: format!("{:?}", domain.source()),
                    scope: Some(format!("clock domain {}", domain.id())),
                });
            }

            if let Some(reset) = domain.reset() {
                if !source_exists(reset.source()) {
                    return Err(HdlirError::MissingEntity {
                        entity_kind: "reset source",
                        identity: format!("{:?}", reset.source()),
                        scope: Some(format!("clock domain {}", domain.id())),
                    });
                }
            }
        }

        Ok(())
    }

    fn validate_parent_cycles(&self) -> HdlirResult<()> {
        // Each walk follows at most one parent edge from each domain.
        // `completed` avoids repeating walks already proven acyclic.
        let mut completed = BTreeSet::new();

        for start in self.domains.keys().copied() {
            if completed.contains(&start) {
                continue;
            }

            let mut path = Vec::new();
            let mut positions = BTreeMap::new();
            let mut current = Some(start);

            while let Some(id) = current {
                if completed.contains(&id) {
                    break;
                }

                if let Some(first_position) = positions.get(&id).copied() {
                    let cycle = path[first_position..]
                        .iter()
                        .map(ToString::to_string)
                        .collect::<Vec<_>>()
                        .join(" -> ");

                    return Err(HdlirError::invalid_invariant(
                        "clock_domain.parent_acyclic",
                        format!("clock-domain parent cycle detected: {cycle} -> {id}"),
                    ));
                }

                let Some(domain) = self.domains.get(&id) else {
                    // Parent existence is checked before cycle validation.
                    return Err(HdlirError::MissingEntity {
                        entity_kind: "clock domain",
                        identity: id.to_string(),
                        scope: None,
                    });
                };

                positions.insert(id, path.len());
                path.push(id);
                current = domain.parent();
            }

            completed.extend(path);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn domain_id(raw: u128) -> HdlirClockDomainId {
        HdlirClockDomainId::new(raw).expect("non-zero clock-domain ID")
    }

    fn signal_id(raw: u128) -> HdlirSignalId {
        HdlirSignalId::new(raw).expect("non-zero signal ID")
    }

    fn value_id(raw: u128) -> HdlirValueId {
        HdlirValueId::new(raw).expect("non-zero value ID")
    }

    fn domain(
        id: u128,
        source_id: u128,
        parent: Option<u128>,
    ) -> HdlirClockDomain {
        HdlirClockDomain::new(
            domain_id(id),
            None,
            HdlirClockSource::Signal(signal_id(source_id)),
            HdlirClockEdge::Rising,
            None,
            parent.map(domain_id),
        )
        .expect("valid logical clock domain")
    }

    #[test]
    fn constructs_clock_domain_and_preserves_fields() {
        let reset = HdlirClockReset::new(
            HdlirClockSource::Signal(signal_id(9)),
            HdlirResetPolarity::ActiveLow,
            HdlirResetBehavior::Asynchronous,
        )
        .expect("valid reset declaration");

        let clock = HdlirClockDomain::new(
            domain_id(1),
            Some("core_clock".to_owned()),
            HdlirClockSource::Signal(signal_id(2)),
            HdlirClockEdge::Falling,
            Some(reset),
            None,
        )
        .expect("valid clock domain");

        assert_eq!(clock.id(), domain_id(1));
        assert_eq!(clock.name(), Some("core_clock"));
        assert_eq!(
            clock.source(),
            HdlirClockSource::Signal(signal_id(2))
        );
        assert_eq!(clock.active_edge(), HdlirClockEdge::Falling);
        assert_eq!(clock.reset(), Some(reset));
        assert_eq!(clock.parent(), None);
    }

    #[test]
    fn rejects_empty_descriptive_name() {
        let result = HdlirClockDomain::new(
            domain_id(1),
            Some("  ".to_owned()),
            HdlirClockSource::Signal(signal_id(2)),
            HdlirClockEdge::Rising,
            None,
            None,
        );

        assert!(result.is_err());
    }

    #[test]
    fn rejects_direct_self_parent() {
        let result = HdlirClockDomain::new(
            domain_id(1),
            None,
            HdlirClockSource::Signal(signal_id(2)),
            HdlirClockEdge::Rising,
            None,
            Some(domain_id(1)),
        );

        assert!(result.is_err());
    }

    #[test]
    fn collection_rejects_duplicate_domain_id_without_replacement() {
        let mut domains = HdlirClockDomains::new();
        domains.insert(domain(1, 10, None)).expect("first insertion");

        let error = domains
            .insert(domain(1, 11, None))
            .expect_err("duplicate ID must be rejected");

        assert_eq!(error.code().as_str(), "hdlir.duplicate_entity");
        assert_eq!(domains.len(), 1);
        assert_eq!(
            domains.get(domain_id(1)).expect("original remains").source(),
            HdlirClockSource::Signal(signal_id(10))
        );
    }

    #[test]
    fn iteration_is_deterministic_by_typed_id() {
        let mut domains = HdlirClockDomains::new();
        domains.insert(domain(8, 80, None)).expect("insert");
        domains.insert(domain(2, 20, None)).expect("insert");
        domains.insert(domain(5, 50, None)).expect("insert");

        let ids: Vec<_> = domains.iter().map(|(id, _)| id.get()).collect();

        assert_eq!(ids, vec![2, 5, 8]);
    }

    #[test]
    fn accepts_valid_parent_relationship() {
        let mut domains = HdlirClockDomains::new();
        domains.insert(domain(1, 10, None)).expect("insert parent");
        domains
            .insert(domain(2, 20, Some(1)))
            .expect("insert child");

        assert!(domains.validate_structure().is_ok());
    }

    #[test]
    fn rejects_missing_parent() {
        let mut domains = HdlirClockDomains::new();
        domains
            .insert(domain(2, 20, Some(1)))
            .expect("local construction");

        assert!(domains.validate_structure().is_err());
    }

    #[test]
    fn rejects_indirect_parent_cycle() {
        let mut domains = HdlirClockDomains::new();
        domains
            .insert(domain(1, 10, Some(2)))
            .expect("insert first");
        domains
            .insert(domain(2, 20, Some(3)))
            .expect("insert second");
        domains
            .insert(domain(3, 30, Some(1)))
            .expect("insert third");

        assert!(domains.validate_structure().is_err());
    }

    #[test]
    fn validates_clock_and_reset_source_references() {
        let reset = HdlirClockReset::new(
            HdlirClockSource::Value(value_id(12)),
            HdlirResetPolarity::ActiveHigh,
            HdlirResetBehavior::Synchronous,
        )
        .expect("valid reset");

        let clock = HdlirClockDomain::new(
            domain_id(1),
            None,
            HdlirClockSource::Signal(signal_id(11)),
            HdlirClockEdge::Rising,
            Some(reset),
            None,
        )
        .expect("valid clock");

        let mut domains = HdlirClockDomains::new();
        domains.insert(clock).expect("insert clock");

        assert!(domains.validate_references(|source| match source {
            HdlirClockSource::Signal(id) => id == signal_id(11),
            HdlirClockSource::Value(id) => id == value_id(12),
        }).is_ok());

        assert!(domains.validate_references(|_| false).is_err());
    }

    #[test]
    fn removal_is_explicit_and_reports_absence() {
        let mut domains = HdlirClockDomains::new();
        domains.insert(domain(1, 10, None)).expect("insert");

        assert!(domains.remove(domain_id(1)).is_some());
        assert!(domains.remove(domain_id(1)).is_none());
        assert!(domains.is_empty());
    }

    #[test]
    fn supports_value_based_clock_sources() {
        let clock = HdlirClockDomain::new(
            domain_id(1),
            None,
            HdlirClockSource::Value(value_id(7)),
            HdlirClockEdge::Rising,
            None,
            None,
        )
        .expect("value source is representable");

        assert_eq!(clock.source(), HdlirClockSource::Value(value_id(7)));
    }
}
