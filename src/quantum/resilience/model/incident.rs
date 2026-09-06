//! Zamani Quantum Resilience — Incident Model.
//!
//! Path:
//!     src/quantum/resilience/model/incident.rs
//!
//! Purpose:
//!     Represents a resilience-level incident: a deterministic grouping of
//!     one or more normalized resilience faults that are treated as one
//!     operational condition by the resilience subsystem.
//!
//! Architectural ownership:
//!
//!     quantum::zqn::fault
//!         owns canonical realized quantum-fault semantics.
//!
//!     quantum::resilience::model::fault
//!         adapts canonical faults into the resilience fault contract.
//!
//!     quantum::resilience::model::incident
//!         groups resilience faults into an incident.
//!
//!     quantum::resilience::diagnosis
//!         determines what the incident means.
//!
//!     quantum::resilience::policy
//!         determines what is permitted.
//!
//!     quantum::resilience::planning
//!         determines what should be done.
//!
//!     quantum::resilience::recovery
//!         executes an approved recovery.
//!
//! This module deliberately does NOT perform diagnosis, recovery,
//! mitigation, verification, scheduling, routing, QEC, hardware control,
//! persistence, serialization, or policy evaluation.
//!
//! # Design principle
//!
//! A fault and an incident are different concepts.
//!
//! ```text
//! Fault
//!     = one normalized observed/realized fault condition.
//!
//! Incident
//!     = one resilience-level grouping of related faults.
//! ```
//!
//! Multiple faults may belong to one incident:
//!
//! ```text
//! fault A ─┐
//! fault B ─┼──> incident X
//! fault C ─┘
//! ```
//!
//! This is important for correlated failures. A resilience engine must not
//! blindly perform one recovery operation per physical fault when several
//! observations are manifestations of the same underlying incident.
//!
//! # Canonical identity rule
//!
//! This file does not define a replacement for:
//!
//! ```text
//! QubitId
//! PhysicalQubitId
//! ```
//!
//! Those remain owned by:
//!
//! ```text
//! crate::quantum::ir::qubit
//! ```
//!
//! Incident identity is different from quantum-resource identity.
//!
//! ```text
//! QubitId
//!     identifies a logical quantum resource.
//!
//! PhysicalQubitId
//!     identifies a physical quantum resource.
//!
//! FaultId
//!     identifies a canonical ZQN fault.
//!
//! IncidentId
//!     identifies a resilience incident.
//! ```
//!
//! None of these identifiers may be substituted for another.
//!
//! # Scalability
//!
//! No machine-size limit is encoded here.
//!
//! There is deliberately no:
//!
//! ```text
//! MAX_FAULTS_PER_INCIDENT
//! MAX_QUBITS_PER_INCIDENT
//! MAX_INCIDENTS
//! ```
//!
//! An incident contains a dynamically sized collection of faults.
//!
//! Actual resource consumption is bounded by the caller's:
//!
//! - memory;
//! - storage;
//! - execution policy;
//! - telemetry policy;
//! - distributed capacity;
//! - target capabilities;
//! - resource limits.
//!
//! "Infinity" therefore means that this semantic model imposes no artificial
//! machine-size ceiling. It does not imply physically infinite memory or
//! execution capacity.
//!
//! # Determinism
//!
//! Incident construction is deterministic.
//!
//! The type contains no:
//!
//! - random-number generation;
//! - system clock access;
//! - global mutable state;
//! - thread-local state;
//! - memory-address identity;
//! - provider-specific behavior;
//! - implicit environmental inspection.
//!
//! The caller supplies all incident inputs explicitly.
//!
//! Faults are canonicalized into deterministic order when an incident is
//! constructed. This prevents hash-map or arrival-order nondeterminism from
//! becoming part of the incident's semantic representation.
//!
//! # Immutability
//!
//! `Incident` is immutable after construction.
//!
//! This is intentional. Detection pipelines may construct a new incident
//! from observations, but an already-created incident must not silently
//! change underneath diagnosis, planning, recovery, verification, or
//! auditing.
//!
//! If a later observation changes the operational situation, construct a new
//! incident or a new incident revision at a higher orchestration layer.
//!
//! # No hidden semantics
//!
//! This type does not decide:
//!
//! - whether an incident is severe;
//! - whether recovery is possible;
//! - whether retry is permitted;
//! - whether a backend should be migrated;
//! - whether QEC should be changed;
//! - whether mitigation should be used;
//! - whether an incident is resolved.
//!
//! Those decisions belong to their respective resilience subsystems.
//!
//! # Integration contract
//!
//! ```text
//! quantum::ir::qubit
//!          │
//!          └── canonical quantum identities
//!
//! quantum::zqn::fault
//!          │
//!          └── canonical realized fault semantics
//!                    │
//!                    ▼
//! quantum::resilience::model::fault
//!          │
//!          └── ResilienceFault
//!                    │
//!                    ▼
//! quantum::resilience::model::incident
//!          │
//!          └── Incident
//!                    │
//!       ┌────────────┼──────────────┐
//!       ▼            ▼              ▼
//!   diagnosis      policy        history
//!       │            │              │
//!       └────────────┼──────────────┘
//!                    ▼
//!                 planning
//!                    │
//!                    ▼
//!                recovery
//!                    │
//!                    ▼
//!                verification
//! ```
//!
//! Downstream modules should consume `Incident` rather than reconstructing
//! fault groupings independently.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! `unsafe` is explicitly forbidden.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;
use core::num::NonZeroU64;

use super::fault::ResilienceFault;

// ============================================================================
// Incident identity
// ============================================================================

/// Stable identity of a resilience incident.
///
/// `IncidentId` identifies an incident object in the resilience domain.
///
/// It does NOT identify:
///
/// - a qubit;
/// - a physical qubit;
/// - a fault;
/// - a backend;
/// - a recovery attempt;
/// - a checkpoint;
/// - an array position.
///
/// The identifier is supplied by the owner of the incident lifecycle.
/// This type intentionally does not contain a global allocator.
///
/// `NonZeroU64` is used only as an opaque identifier representation. It is
/// not a machine-size limit, fault-count limit, or qubit-count limit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct IncidentId(NonZeroU64);

impl IncidentId {
    /// Creates an incident identifier from a non-zero value.
    ///
    /// Returns `None` for zero because zero is reserved as the absence of an
    /// explicitly assigned identifier.
    #[must_use]
    pub const fn new(value: u64) -> Option<Self> {
        match NonZeroU64::new(value) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }

    /// Returns the underlying identifier value.
    ///
    /// The value has no quantum-resource semantics.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0.get()
    }
}

impl fmt::Display for IncidentId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "incident-{}", self.value())
    }
}

// ============================================================================
// Incident
// ============================================================================

/// Immutable resilience-level grouping of related faults.
///
/// An `Incident` is the bridge between low-level fault observations and
/// higher-level resilience decisions.
///
/// It deliberately contains only information necessary to identify and
/// reproduce the grouping:
///
/// - incident identity;
/// - normalized resilience faults.
///
/// Diagnosis, severity, confidence, health, recovery state, policy and
/// verification results are deliberately kept outside this object.
///
/// # Ordering
///
/// Faults are stored in deterministic canonical order.
///
/// The order has no temporal or causal meaning.
///
/// Consumers must not interpret the collection order as:
///
/// - earliest fault first;
/// - most severe fault first;
/// - causal order;
/// - recovery priority.
///
/// Those semantics belong elsewhere.
///
/// # Empty incidents
///
/// An empty incident is representable.
///
/// This is intentional because construction and validation are separate
/// concerns. An orchestration layer may create an incident identity before
/// observations arrive, or may use an empty incident as an explicit
/// placeholder in a transactional workflow.
///
/// Production decision layers should normally require at least one fault
/// before treating an incident as actionable.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Incident {
    id: IncidentId,
    faults: Vec<ResilienceFault>,
}

impl Incident {
    /// Creates an incident from an identifier and a fault collection.
    ///
    /// The supplied collection is consumed and deterministically canonicalized.
    ///
    /// No resource-size limit is imposed by this type.
    ///
    /// # Determinism
    ///
    /// The resulting fault collection does not depend on the caller's
    /// original arrival order.
    #[must_use]
    pub fn new<I>(id: IncidentId, faults: I) -> Self
    where
        I: IntoIterator<Item = ResilienceFault>,
    {
        let mut faults: Vec<ResilienceFault> = faults.into_iter().collect();

        faults.sort();

        Self { id, faults }
    }

    /// Creates an incident containing one fault.
    #[must_use]
    pub fn from_fault(id: IncidentId, fault: ResilienceFault) -> Self {
        Self {
            id,
            faults: vec![fault],
        }
    }

    /// Returns the incident identity.
    #[must_use]
    pub const fn id(&self) -> IncidentId {
        self.id
    }

    /// Returns the number of normalized faults in the incident.
    ///
    /// This is a property of the materialized incident, not a system-wide
    /// limit.
    #[must_use]
    pub fn fault_count(&self) -> usize {
        self.faults.len()
    }

    /// Returns whether the incident contains no faults.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.faults.is_empty()
    }

    /// Returns an immutable view of all faults.
    ///
    /// Faults are returned in deterministic canonical order.
    #[must_use]
    pub fn faults(&self) -> &[ResilienceFault] {
        &self.faults
    }

    /// Returns the first canonical fault, if any.
    ///
    /// "First" refers only to deterministic canonical ordering. It does not
    /// imply temporal priority or causal importance.
    #[must_use]
    pub fn first_fault(&self) -> Option<&ResilienceFault> {
        self.faults.first()
    }

    /// Returns the last canonical fault, if any.
    ///
    /// "Last" refers only to deterministic canonical ordering.
    #[must_use]
    pub fn last_fault(&self) -> Option<&ResilienceFault> {
        self.faults.last()
    }

    /// Returns whether this incident contains the supplied fault.
    #[must_use]
    pub fn contains_fault(&self, fault: &ResilienceFault) -> bool {
        self.faults.binary_search(fault).is_ok()
    }

    /// Returns an iterator over the incident's faults.
    ///
    /// The iterator is deterministic because the underlying collection is
    /// canonicalized during construction.
    pub fn iter(&self) -> core::slice::Iter<'_, ResilienceFault> {
        self.faults.iter()
    }

    /// Returns a new incident with one additional fault.
    ///
    /// The original incident remains unchanged.
    ///
    /// This method is intentionally persistent/immutable so that an incident
    /// already handed to another subsystem cannot be mutated underneath it.
    #[must_use]
    pub fn with_fault(&self, fault: ResilienceFault) -> Self {
        let mut faults = self.faults.clone();
        faults.push(fault);
        faults.sort();

        Self {
            id: self.id,
            faults,
        }
    }

    /// Returns a new incident containing the supplied replacement fault
    /// collection.
    ///
    /// This is useful when an observation pipeline discovers that multiple
    /// faults belong to one correlation group.
    #[must_use]
    pub fn with_faults<I>(&self, faults: I) -> Self
    where
        I: IntoIterator<Item = ResilienceFault>,
    {
        Self::new(self.id, faults)
    }

    /// Returns a new incident with the supplied incident identity.
    ///
    /// The fault collection is reused without changing its semantics.
    #[must_use]
    pub fn with_id(&self, id: IncidentId) -> Self {
        Self {
            id,
            faults: self.faults.clone(),
        }
    }

    /// Consumes the incident and returns its identity and faults.
    #[must_use]
    pub fn into_parts(self) -> (IncidentId, Vec<ResilienceFault>) {
        (self.id, self.faults)
    }

    /// Consumes the incident and returns its faults.
    #[must_use]
    pub fn into_faults(self) -> Vec<ResilienceFault> {
        self.faults
    }

    /// Consumes the incident and returns its identity.
    #[must_use]
    pub const fn into_id(self) -> IncidentId {
        self.id
    }

    /// Performs structural validation of the incident.
    ///
    /// This validation deliberately does not depend on:
    ///
    /// - policy;
    /// - hardware;
    /// - topology;
    /// - QEC;
    /// - routing;
    /// - scheduling;
    /// - current time;
    /// - backend state.
    ///
    /// It verifies only invariants owned by this value object.
    ///
    /// An incident is structurally valid when:
    ///
    /// 1. its identity is non-zero;
    /// 2. its fault collection is canonically ordered.
    ///
    /// Empty incidents are structurally valid because construction and
    /// actionability are separate concepts.
    #[must_use]
    pub fn is_structurally_valid(&self) -> bool {
        self.id.value() != 0
            && self
                .faults
                .windows(2)
                .all(|window| window[0] <= window[1])
    }

    /// Returns whether the incident has at least one actionable fault.
    ///
    /// This is deliberately weaker than saying that recovery is possible.
    /// Recovery feasibility belongs to diagnosis/policy/planning.
    #[must_use]
    pub fn is_actionable(&self) -> bool {
        !self.faults.is_empty()
    }

    /// Returns the number of distinct fault values in the incident.
    ///
    /// Because the incident is canonicalized but does not silently alter
    /// caller semantics, duplicate values remain representable.
    ///
    /// This method computes distinctness without introducing a separate
    /// indexing structure or hidden state.
    #[must_use]
    pub fn distinct_fault_count(&self) -> usize {
        if self.faults.is_empty() {
            return 0;
        }

        let mut count = 1usize;

        for pair in self.faults.windows(2) {
            if pair[0] != pair[1] {
                count += 1;
            }
        }

        count
    }

    /// Returns whether at least two equal fault values occur.
    #[must_use]
    pub fn contains_duplicate_faults(&self) -> bool {
        self.distinct_fault_count() != self.faults.len()
    }
}

impl AsRef<[ResilienceFault]> for Incident {
    fn as_ref(&self) -> &[ResilienceFault] {
        self.faults()
    }
}

impl IntoIterator for Incident {
    type Item = ResilienceFault;
    type IntoIter = std::vec::IntoIter<ResilienceFault>;

    fn into_iter(self) -> Self::IntoIter {
        self.faults.into_iter()
    }
}

impl<'a> IntoIterator for &'a Incident {
    type Item = &'a ResilienceFault;
    type IntoIter = core::slice::Iter<'a, ResilienceFault>;

    fn into_iter(self) -> Self::IntoIter {
        self.faults.iter()
    }
}

// ============================================================================
// Incident builder
// ============================================================================

/// Incremental builder for an immutable [`Incident`].
///
/// The builder is useful when faults arrive from several detectors or
/// correlation stages.
///
/// It owns only explicitly supplied data. It has:
///
/// - no global state;
/// - no background worker;
/// - no clock;
/// - no RNG;
/// - no fixed capacity;
/// - no hidden hardware dependency.
///
/// Once `build` is called, the resulting `Incident` is immutable.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IncidentBuilder {
    id: IncidentId,
    faults: Vec<ResilienceFault>,
}

impl IncidentBuilder {
    /// Creates an empty incident builder.
    #[must_use]
    pub const fn new(id: IncidentId) -> Self {
        Self {
            id,
            faults: Vec::new(),
        }
    }

    /// Adds one fault to the builder.
    ///
    /// The builder does not sort after every insertion. Canonicalization is
    /// performed once during `build`, avoiding repeated work for large
    /// incidents.
    pub fn push(&mut self, fault: ResilienceFault) {
        self.faults.push(fault);
    }

    /// Adds many faults to the builder.
    pub fn extend<I>(&mut self, faults: I)
    where
        I: IntoIterator<Item = ResilienceFault>,
    {
        self.faults.extend(faults);
    }

    /// Returns the incident identity.
    #[must_use]
    pub const fn id(&self) -> IncidentId {
        self.id
    }

    /// Returns the number of faults currently accumulated.
    #[must_use]
    pub fn fault_count(&self) -> usize {
        self.faults.len()
    }

    /// Returns whether no faults have been accumulated.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.faults.is_empty()
    }

    /// Builds the immutable incident.
    ///
    /// Faults are deterministically canonicalized exactly once.
    #[must_use]
    pub fn build(self) -> Incident {
        Incident::new(self.id, self.faults)
    }
}

// ============================================================================
// Incident correlation helpers
// ============================================================================

/// Describes why faults were grouped into one incident.
///
/// This is intentionally a small, provider-neutral vocabulary.
///
/// It does not claim that the selected grouping is the true physical cause.
/// Causal diagnosis belongs to `quantum::resilience::diagnosis`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum IncidentCorrelation {
    /// Faults were grouped because they share an explicitly supplied
    /// correlation identity.
    Explicit,

    /// Faults were grouped because an upstream detector reported them as a
    /// correlated observation.
    ObservedCorrelation,

    /// Faults were grouped by a resilience correlation algorithm.
    InferredCorrelation,

    /// Correlation could not be established.
    Unknown,
}

impl fmt::Display for IncidentCorrelation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Explicit => formatter.write_str("explicit"),
            Self::ObservedCorrelation => formatter.write_str("observed"),
            Self::InferredCorrelation => formatter.write_str("inferred"),
            Self::Unknown => formatter.write_str("unknown"),
        }
    }
}

// ============================================================================
// Incident grouping metadata
// ============================================================================

/// Immutable metadata describing how an incident was formed.
///
/// This type deliberately does not contain timestamps, backend handles,
/// credentials, mutable telemetry objects, or execution state.
///
/// Time and execution provenance belong to the telemetry/history/provenance
/// layers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct IncidentGrouping {
    correlation: IncidentCorrelation,
}

impl IncidentGrouping {
    /// Creates grouping metadata.
    #[must_use]
    pub const fn new(correlation: IncidentCorrelation) -> Self {
        Self { correlation }
    }

    /// Returns the correlation classification.
    #[must_use]
    pub const fn correlation(self) -> IncidentCorrelation {
        self.correlation
    }
}

impl Default for IncidentGrouping {
    fn default() -> Self {
        Self::new(IncidentCorrelation::Unknown)
    }
}

// ============================================================================
// Correlated incident
// ============================================================================

/// Immutable incident plus explicit grouping metadata.
///
/// `CorrelatedIncident` exists separately from `Incident` so that the core
/// incident identity remains minimal while correlation provenance can be
/// carried when required.
///
/// The distinction is useful because an incident can exist before its
/// correlation mechanism is known.
///
/// No causal claim is implied by `IncidentGrouping`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CorrelatedIncident {
    incident: Incident,
    grouping: IncidentGrouping,
}

impl CorrelatedIncident {
    /// Creates a correlated incident.
    #[must_use]
    pub const fn new(incident: Incident, grouping: IncidentGrouping) -> Self {
        Self {
            incident,
            grouping,
        }
    }

    /// Returns the underlying incident.
    #[must_use]
    pub const fn incident(&self) -> &Incident {
        &self.incident
    }

    /// Returns the grouping metadata.
    #[must_use]
    pub const fn grouping(&self) -> IncidentGrouping {
        self.grouping
    }

    /// Returns the incident identity.
    #[must_use]
    pub const fn id(&self) -> IncidentId {
        self.incident.id()
    }

    /// Returns the incident's faults.
    #[must_use]
    pub fn faults(&self) -> &[ResilienceFault] {
        self.incident.faults()
    }
}

impl AsRef<Incident> for CorrelatedIncident {
    fn as_ref(&self) -> &Incident {
        &self.incident
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// Creates deterministic IDs without introducing a global allocator.
    fn incident_id(value: u64) -> IncidentId {
        IncidentId::new(value).expect("test IDs must be non-zero")
    }

    #[test]
    fn incident_id_rejects_zero() {
        assert!(IncidentId::new(0).is_none());
    }

    #[test]
    fn incident_id_round_trips_value() {
        let id = incident_id(42);

        assert_eq!(id.value(), 42);
        assert_eq!(id.to_string(), "incident-42");
    }

    #[test]
    fn empty_incident_is_representable() {
        let incident = Incident::new(incident_id(1), core::iter::empty());

        assert!(incident.is_empty());
        assert_eq!(incident.fault_count(), 0);
        assert!(!incident.is_actionable());
        assert!(incident.is_structurally_valid());
    }

    #[test]
    fn builder_produces_immutable_incident() {
        let builder = IncidentBuilder::new(incident_id(2));

        let incident = builder.build();

        assert_eq!(incident.id(), incident_id(2));
        assert!(incident.is_empty());
    }

    #[test]
    fn grouping_is_explicit() {
        let grouping = IncidentGrouping::new(IncidentCorrelation::Explicit);

        assert_eq!(
            grouping.correlation(),
            IncidentCorrelation::Explicit
        );
        assert_eq!(grouping.correlation().to_string(), "explicit");
    }

    #[test]
    fn correlated_incident_preserves_underlying_identity() {
        let incident = Incident::new(incident_id(3), core::iter::empty());

        let correlated = CorrelatedIncident::new(
            incident,
            IncidentGrouping::new(IncidentCorrelation::ObservedCorrelation),
        );

        assert_eq!(correlated.id(), incident_id(3));
        assert_eq!(
            correlated.grouping().correlation(),
            IncidentCorrelation::ObservedCorrelation
        );
        assert!(correlated.faults().is_empty());
    }

    #[test]
    fn incident_order_is_deterministic() {
        // This test intentionally uses an empty collection because the
        // concrete construction of ResilienceFault belongs to
        // model/fault.rs. The ordering guarantee is exercised structurally
        // by the Incident implementation itself.
        let incident = Incident::new(incident_id(4), core::iter::empty());

        assert!(incident.is_structurally_valid());
    }

    #[test]
    fn with_id_preserves_fault_collection() {
        let original = Incident::new(incident_id(5), core::iter::empty());

        let changed = original.with_id(incident_id(6));

        assert_eq!(original.id(), incident_id(5));
        assert_eq!(changed.id(), incident_id(6));
        assert_eq!(original.faults(), changed.faults());
    }

    #[test]
    fn with_faults_replaces_collection_without_mutating_original() {
        let original = Incident::new(incident_id(7), core::iter::empty());

        let replacement = original.with_faults(core::iter::empty());

        assert!(original.is_empty());
        assert!(replacement.is_empty());
        assert_eq!(original.id(), replacement.id());
    }

    #[test]
    fn distinct_count_for_empty_incident_is_zero() {
        let incident = Incident::new(incident_id(8), core::iter::empty());

        assert_eq!(incident.distinct_fault_count(), 0);
        assert!(!incident.contains_duplicate_faults());
    }

    #[test]
    fn default_grouping_is_unknown() {
        assert_eq!(
            IncidentGrouping::default().correlation(),
            IncidentCorrelation::Unknown
        );
    }

    #[test]
    fn incident_as_ref_exposes_fault_slice() {
        let incident = Incident::new(incident_id(9), core::iter::empty());

        let faults: &[ResilienceFault] = incident.as_ref();

        assert!(faults.is_empty());
    }

    #[test]
    fn incident_iteration_is_empty_for_empty_incident() {
        let incident = Incident::new(incident_id(10), core::iter::empty());

        assert_eq!(incident.into_iter().count(), 0);
    }
}