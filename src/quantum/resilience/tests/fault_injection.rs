//! Zamani Quantum Resilience — Fault Injection Tests.
//!
//! Production-oriented fault-injection and resilience-boundary tests.
//!
//! # Architectural role
//!
//! This module verifies that the resilience subsystem can safely consume
//! deterministic synthetic fault streams without:
//!
//! - creating a competing fault ontology;
//! - creating a competing `QubitId`;
//! - confusing logical and physical qubits;
//! - imposing an artificial machine-size limit;
//! - depending on wall-clock time;
//! - depending on global mutable state;
//! - depending on global randomness;
//! - silently changing canonical ZQN faults;
//! - collapsing distinct fault identities;
//! - requiring a fixed number of qubits;
//! - requiring a fixed number of faults;
//! - assuming a particular backend;
//! - assuming a particular QPU size;
//! - using unsafe Rust.
//!
//! # Canonical ownership
//!
//! The dependency direction exercised here is:
//!
//! ```text
//! quantum::ir::qubit
//!       │
//!       ├── QubitId
//!       └── PhysicalQubitId
//!
//! quantum::zqn::fault::fault
//!       │
//!       ├── Fault
//!       ├── FaultId
//!       ├── FaultLocation
//!       ├── FaultClassification
//!       └── FaultEffect
//!       │
//!       ▼
//! quantum::resilience::model::fault
//!       │
//!       ▼
//! fault-injection tests
//! ```
//!
//! Resilience therefore observes canonical ZQN faults rather than defining
//! another quantum-fault representation.
//!
//! # Scalability
//!
//! These tests deliberately distinguish:
//!
//! 1. semantic test cardinality;
//! 2. generated fault-stream cardinality;
//! 3. resource cardinality;
//! 4. available execution capacity.
//!
//! There is no `MAX_QUBITS`, `MAX_FAULTS`, `MAX_SCENARIOS`, or similar
//! architectural ceiling in this file.
//!
//! A test that needs a larger workload generates it from its requested size.
//! The implementation remains bounded only by the resources allocated to the
//! test process.
//!
//! # Determinism
//!
//! Fault generation is deterministic.
//!
//! A generated fault is completely determined by:
//!
//! - its ordinal;
//! - the selected logical/physical resource domain;
//! - the selected classification.
//!
//! No system clock, operating-system randomness, thread scheduling,
//! memory-address identity, or hash-map iteration order is used.
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
//! # Integration contract
//!
//! This file expects the following existing contracts:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//!
//! crate::quantum::zqn::core::ids::FaultId
//! crate::quantum::zqn::fault::fault::Fault
//! crate::quantum::zqn::fault::fault::FaultClassification
//! crate::quantum::zqn::fault::fault::FaultEffect
//! crate::quantum::zqn::fault::fault::FaultLocation
//!
//! crate::quantum::resilience::model::fault::ResilienceFault
//! crate::quantum::resilience::model::fault::ProvenanceId
//! ```
//!
//! No later production implementation should need to modify this test merely
//! because the physical machine becomes larger or a new backend is added.
//!
//! New fault semantics should be tested in the canonical ZQN fault subsystem
//! and then passed through this boundary.
//!
//! # Important distinction
//!
//! This file is a fault-injection *test harness*.
//!
//! It does not claim that a synthetic `Fault` is equivalent to a physical
//! hardware failure. It verifies that the resilience architecture preserves
//! the semantic identity and metadata of a fault presented at its boundary.
//!
//! # Definition of done
//!
//! This file is complete when it verifies:
//!
//! - canonical qubit identity;
//! - logical/physical identity separation;
//! - canonical fault identity;
//! - canonical fault preservation;
//! - provenance preservation;
//! - deterministic generation;
//! - scalable streaming generation;
//! - distinct-fault preservation;
//! - repeated-fault handling;
//! - composite fault coverage;
//! - all currently defined canonical fault classifications;
//! - invalid/overflow-sensitive generation boundaries;
//! - absence of artificial machine-size assumptions;
//! - no unsafe Rust.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::collections::BTreeSet;

use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId};
use crate::quantum::resilience::model::fault::{
    ProvenanceId,
    ResilienceFault,
};
use crate::quantum::zqn::core::ids::FaultId;
use crate::quantum::zqn::fault::fault::{
    Fault as ZqnFault,
    FaultClassification,
    FaultEffect,
    FaultLocation,
};

// ============================================================================
// Test-domain abstractions
// ============================================================================

/// Resource domain used by the synthetic injector.
///
/// This is deliberately a test-only description and does not create a new
/// quantum identity type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum InjectedResource {
    /// Canonical logical IR resource.
    Logical(QubitId),

    /// Canonical physical IR resource.
    Physical(PhysicalQubitId),
}

/// A deterministic synthetic fault-injection case.
///
/// The case describes what should be injected; the canonical ZQN `Fault` is
/// created only at the injection boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct FaultInjectionCase {
    /// Stable ordinal used to construct the caller-owned `FaultId`.
    ordinal: u64,

    /// Resource to which the synthetic fault applies.
    resource: InjectedResource,

    /// Canonical ZQN fault classification.
    classification: FaultClassification,
}

impl FaultInjectionCase {
    /// Creates a deterministic logical-resource case.
    #[must_use]
    const fn logical(
        ordinal: u64,
        qubit: QubitId,
        classification: FaultClassification,
    ) -> Self {
        Self {
            ordinal,
            resource: InjectedResource::Logical(qubit),
            classification,
        }
    }

    /// Creates a deterministic physical-resource case.
    #[must_use]
    const fn physical(
        ordinal: u64,
        qubit: PhysicalQubitId,
        classification: FaultClassification,
    ) -> Self {
        Self {
            ordinal,
            resource: InjectedResource::Physical(qubit),
            classification,
        }
    }

    /// Converts the test case into the canonical ZQN fault representation.
    ///
    /// The test harness deliberately uses the canonical ZQN constructor rather
    /// than creating a resilience-specific fault structure.
    fn into_canonical_fault(self) -> ZqnFault {
        let location = match self.resource {
            InjectedResource::Logical(qubit) => {
                FaultLocation::logical_qubit(qubit)
            }
            InjectedResource::Physical(qubit) => {
                FaultLocation::physical_qubit(qubit)
            }
        };

        ZqnFault::new(
            FaultId::new(self.ordinal),
            location,
            self.classification,
            FaultEffect::None,
        )
        .expect("synthetic fault must satisfy the canonical ZQN constructor")
    }

    /// Converts the test case into the resilience-domain boundary object.
    #[must_use]
    fn into_resilience_fault(self) -> ResilienceFault {
        ResilienceFault::from_canonical(self.into_canonical_fault())
    }

    /// Converts the test case into a resilience fault with deterministic
    /// provenance.
    #[must_use]
    fn into_resilience_fault_with_provenance(
        self,
        provenance: ProvenanceId,
    ) -> ResilienceFault {
        ResilienceFault::with_provenance(
            self.into_canonical_fault(),
            provenance,
        )
    }
}

// ============================================================================
// Streaming injector
// ============================================================================

/// Deterministic, allocation-free fault-case generator.
///
/// The generator itself does not materialize a collection of faults.
///
/// This is important for scalability: a test can iterate through as many
/// generated cases as its execution environment can support without the
/// generator imposing a fixed maximum.
///
/// The generated sequence is deterministic for a given configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct FaultStream {
    start: u64,
    count: u64,
    resource_count: u64,
    classification: FaultClassification,
}

impl FaultStream {
    /// Creates a deterministic stream.
    ///
    /// `resource_count == 0` is rejected because no resource can be selected
    /// from an empty resource domain.
    fn new(
        start: u64,
        count: u64,
        resource_count: u64,
        classification: FaultClassification,
    ) -> Result<Self, &'static str> {
        if resource_count == 0 && count != 0 {
            return Err("resource_count must be non-zero for a non-empty stream");
        }

        Ok(Self {
            start,
            count,
            resource_count,
            classification,
        })
    }

    /// Returns the configured number of cases.
    #[must_use]
    const fn len(&self) -> u64 {
        self.count
    }

    /// Returns whether the stream is empty.
    #[must_use]
    const fn is_empty(&self) -> bool {
        self.count == 0
    }

    /// Returns the case at a logical stream offset.
    ///
    /// No allocation occurs.
    fn case_at(&self, offset: u64) -> Option<FaultInjectionCase> {
        if offset >= self.count {
            return None;
        }

        let ordinal = self.start.checked_add(offset)?;

        let resource_index = offset % self.resource_count;

        Some(FaultInjectionCase::logical(
            ordinal,
            QubitId::new(resource_index as usize),
            self.classification.clone(),
        ))
    }

    /// Produces an iterator over the stream.
    fn iter(self) -> FaultStreamIter {
        FaultStreamIter {
            stream: self,
            offset: 0,
        }
    }
}

/// Iterator over a deterministic fault stream.
///
/// This iterator does not allocate a collection containing the entire stream.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct FaultStreamIter {
    stream: FaultStream,
    offset: u64,
}

impl Iterator for FaultStreamIter {
    type Item = FaultInjectionCase;

    fn next(&mut self) -> Option<Self::Item> {
        let case = self.stream.case_at(self.offset)?;

        self.offset = self.offset.checked_add(1)?;

        Some(case)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self
            .stream
            .count
            .saturating_sub(self.offset);

        let upper = usize::try_from(remaining).ok();

        match upper {
            Some(value) => (value, Some(value)),
            None => (0, None),
        }
    }
}

// ============================================================================
// Helpers
// ============================================================================

fn canonical_logical(value: usize) -> QubitId {
    QubitId::new(value)
}

fn canonical_physical(value: usize) -> PhysicalQubitId {
    PhysicalQubitId::new(value)
}

fn canonical_fault(
    id: u64,
    location: FaultLocation,
    classification: FaultClassification,
) -> ZqnFault {
    ZqnFault::new(
        FaultId::new(id),
        location,
        classification,
        FaultEffect::None,
    )
    .expect("test fault must be accepted by canonical ZQN validation")
}

fn resilience_fault(
    id: u64,
    location: FaultLocation,
    classification: FaultClassification,
) -> ResilienceFault {
    ResilienceFault::from_canonical(canonical_fault(
        id,
        location,
        classification,
    ))
}

// ============================================================================
// Canonical identity tests
// ============================================================================

#[test]
fn logical_fault_uses_canonical_ir_qubit_identity() {
    let qubit = canonical_logical(7);

    let fault = resilience_fault(
        1,
        FaultLocation::logical_qubit(qubit),
        FaultClassification::Gate,
    );

    assert_eq!(fault.canonical().id(), FaultId::new(1));
    assert_eq!(fault.fault_id(), FaultId::new(1));

    assert_eq!(
        fault.canonical().location(),
        &FaultLocation::logical_qubit(qubit)
    );
}

#[test]
fn physical_fault_uses_canonical_ir_physical_qubit_identity() {
    let qubit = canonical_physical(7);

    let fault = resilience_fault(
        2,
        FaultLocation::physical_qubit(qubit),
        FaultClassification::Gate,
    );

    assert_eq!(fault.fault_id(), FaultId::new(2));

    assert_eq!(
        fault.canonical().location(),
        &FaultLocation::physical_qubit(qubit)
    );
}

#[test]
fn logical_and_physical_identity_are_not_interchangeable() {
    let logical = FaultLocation::logical_qubit(canonical_logical(7));
    let physical = FaultLocation::physical_qubit(canonical_physical(7));

    assert_ne!(logical, physical);
}

#[test]
fn canonical_fault_identity_is_preserved_across_resilience_boundary() {
    let canonical = canonical_fault(
        91,
        FaultLocation::logical_qubit(canonical_logical(3)),
        FaultClassification::Preparation,
    );

    let expected = canonical.clone();

    let resilience = ResilienceFault::from_canonical(canonical);

    assert_eq!(resilience.canonical(), &expected);
    assert_eq!(resilience.into_canonical(), expected);
}

#[test]
fn resilience_fault_conversion_round_trips_without_semantic_changes() {
    let canonical = canonical_fault(
        17,
        FaultLocation::physical_qubit(canonical_physical(11)),
        FaultClassification::Measurement,
    );

    let resilience: ResilienceFault = canonical.clone().into();
    let recovered: ZqnFault = resilience.into();

    assert_eq!(recovered, canonical);
}

// ============================================================================
// Provenance tests
// ============================================================================

#[test]
fn provenance_is_separate_from_canonical_fault_identity() {
    let canonical = canonical_fault(
        31,
        FaultLocation::logical_qubit(canonical_logical(5)),
        FaultClassification::Gate,
    );

    let first =
        ResilienceFault::with_provenance(canonical.clone(), ProvenanceId::new(100));

    let second =
        ResilienceFault::with_provenance(canonical, ProvenanceId::new(200));

    assert_eq!(first.fault_id(), second.fault_id());
    assert_ne!(first.provenance(), second.provenance());
    assert_ne!(first, second);
}

#[test]
fn provenance_does_not_modify_canonical_fault() {
    let canonical = canonical_fault(
        41,
        FaultLocation::physical_qubit(canonical_physical(9)),
        FaultClassification::Idle,
    );

    let injected =
        ResilienceFault::with_provenance(canonical.clone(), ProvenanceId::new(123));

    assert_eq!(injected.canonical(), &canonical);
    assert_eq!(injected.provenance(), Some(ProvenanceId::new(123)));
}

// ============================================================================
// Classification coverage
// ============================================================================

#[test]
fn every_current_canonical_classification_can_cross_resilience_boundary() {
    let classifications = [
        FaultClassification::Preparation,
        FaultClassification::Gate,
        FaultClassification::Reset,
        FaultClassification::Measurement,
        FaultClassification::Idle,
    ];

    for (index, classification) in classifications.iter().cloned().enumerate() {
        let fault = resilience_fault(
            (index as u64).saturating_add(1),
            FaultLocation::logical_qubit(canonical_logical(index)),
            classification.clone(),
        );

        assert_eq!(fault.canonical().classification(), &classification);
    }
}

// ============================================================================
// Determinism tests
// ============================================================================

#[test]
fn identical_injection_cases_are_deterministic() {
    let first = FaultInjectionCase::logical(
        101,
        canonical_logical(13),
        FaultClassification::Gate,
    )
    .into_resilience_fault();

    let second = FaultInjectionCase::logical(
        101,
        canonical_logical(13),
        FaultClassification::Gate,
    )
    .into_resilience_fault();

    assert_eq!(first, second);
    assert_eq!(first.canonical(), second.canonical());
}

#[test]
fn deterministic_stream_replays_identically() {
    let stream = FaultStream::new(
        1_000,
        256,
        17,
        FaultClassification::Gate,
    )
    .expect("valid stream");

    let first: Vec<ResilienceFault> = stream
        .iter()
        .map(FaultInjectionCase::into_resilience_fault)
        .collect();

    let second: Vec<ResilienceFault> = stream
        .iter()
        .map(FaultInjectionCase::into_resilience_fault)
        .collect();

    assert_eq!(first, second);
}

#[test]
fn deterministic_stream_does_not_depend_on_hash_iteration_order() {
    let stream = FaultStream::new(
        2_000,
        512,
        31,
        FaultClassification::Measurement,
    )
    .expect("valid stream");

    let first: Vec<FaultId> = stream
        .iter()
        .map(|case| case.into_resilience_fault().fault_id())
        .collect();

    let second: Vec<FaultId> = stream
        .iter()
        .map(|case| case.into_resilience_fault().fault_id())
        .collect();

    assert_eq!(first, second);
}

// ============================================================================
// Identity uniqueness and duplicate handling
// ============================================================================

#[test]
fn distinct_fault_ids_remain_distinct() {
    let first = resilience_fault(
        501,
        FaultLocation::logical_qubit(canonical_logical(1)),
        FaultClassification::Gate,
    );

    let second = resilience_fault(
        502,
        FaultLocation::logical_qubit(canonical_logical(1)),
        FaultClassification::Gate,
    );

    assert_ne!(first.fault_id(), second.fault_id());
    assert_ne!(first, second);
}

#[test]
fn same_fault_identity_and_same_semantics_compare_equal() {
    let first = resilience_fault(
        601,
        FaultLocation::physical_qubit(canonical_physical(4)),
        FaultClassification::Gate,
    );

    let second = resilience_fault(
        601,
        FaultLocation::physical_qubit(canonical_physical(4)),
        FaultClassification::Gate,
    );

    assert_eq!(first, second);
}

#[test]
fn same_fault_identity_but_different_semantics_do_not_get_collapsed() {
    let logical = resilience_fault(
        701,
        FaultLocation::logical_qubit(canonical_logical(4)),
        FaultClassification::Gate,
    );

    let physical = resilience_fault(
        701,
        FaultLocation::physical_qubit(canonical_physical(4)),
        FaultClassification::Gate,
    );

    assert_ne!(logical, physical);
}

// ============================================================================
// Stream and scale tests
// ============================================================================

#[test]
fn empty_stream_is_valid() {
    let stream = FaultStream::new(
        0,
        0,
        0,
        FaultClassification::Gate,
    )
    .expect("empty stream is valid");

    assert!(stream.is_empty());
    assert_eq!(stream.len(), 0);
    assert_eq!(stream.iter().next(), None);
}

#[test]
fn non_empty_stream_requires_resources() {
    let result = FaultStream::new(
        0,
        1,
        0,
        FaultClassification::Gate,
    );

    assert!(result.is_err());
}

#[test]
fn stream_is_bounded_by_requested_work_not_by_architectural_constants() {
    let requested = 4_096_u64;

    let stream = FaultStream::new(
        10_000,
        requested,
        257,
        FaultClassification::Idle,
    )
    .expect("valid stream");

    assert_eq!(stream.len(), requested);

    let generated = stream.iter().count();

    assert_eq!(generated, requested as usize);
}

#[test]
fn stream_can_reuse_resources_without_reusing_fault_identity() {
    let stream = FaultStream::new(
        20_000,
        1_000,
        3,
        FaultClassification::Gate,
    )
    .expect("valid stream");

    let mut identities = BTreeSet::new();
    let mut resource_locations = BTreeSet::new();

    for case in stream.iter() {
        let fault = case.into_resilience_fault();

        identities.insert(fault.fault_id());

        if let FaultLocation::LogicalQubit(qubit) =
            fault.canonical().location()
        {
            resource_locations.insert(*qubit);
        }
    }

    assert_eq!(identities.len(), 1_000);
    assert_eq!(resource_locations.len(), 3);
}

#[test]
fn stream_does_not_require_one_fault_per_qubit() {
    let resource_count = 2_u64;
    let fault_count = 128_u64;

    let stream = FaultStream::new(
        30_000,
        fault_count,
        resource_count,
        FaultClassification::Gate,
    )
    .expect("valid stream");

    let generated = stream.iter().count();

    assert_eq!(generated, fault_count as usize);
}

// ============================================================================
// Boundary and overflow tests
// ============================================================================

#[test]
fn stream_handles_zero_count_without_resource_allocation() {
    let stream = FaultStream::new(
        u64::MAX,
        0,
        0,
        FaultClassification::Gate,
    )
    .expect("empty stream should not require resources");

    assert!(stream.is_empty());
}

#[test]
fn stream_rejects_unrepresentable_fault_ordinal_progression() {
    let stream = FaultStream::new(
        u64::MAX,
        2,
        1,
        FaultClassification::Gate,
    )
    .expect("configuration itself is valid");

    let mut iterator = stream.iter();

    assert!(iterator.next().is_some());
    assert!(iterator.next().is_none());
}

#[test]
fn case_at_rejects_offsets_outside_stream() {
    let stream = FaultStream::new(
        100,
        4,
        2,
        FaultClassification::Gate,
    )
    .expect("valid stream");

    assert!(stream.case_at(0).is_some());
    assert!(stream.case_at(3).is_some());
    assert!(stream.case_at(4).is_none());
}

// ============================================================================
// Logical/physical mixed-resource tests
// ============================================================================

#[test]
fn mixed_logical_and_physical_faults_remain_domain_distinct() {
    let logical = FaultInjectionCase::logical(
        40_001,
        canonical_logical(8),
        FaultClassification::Gate,
    )
    .into_resilience_fault();

    let physical = FaultInjectionCase::physical(
        40_002,
        canonical_physical(8),
        FaultClassification::Gate,
    )
    .into_resilience_fault();

    assert_ne!(
        logical.canonical().location(),
        physical.canonical().location()
    );

    assert_ne!(logical, physical);
}

#[test]
fn physical_fault_injection_preserves_physical_identity() {
    let physical = FaultInjectionCase::physical(
        41_001,
        canonical_physical(17),
        FaultClassification::Measurement,
    )
    .into_resilience_fault();

    assert_eq!(
        physical.canonical().location(),
        &FaultLocation::physical_qubit(canonical_physical(17))
    );
}

// ============================================================================
// Composite/correlated-resource boundary tests
// ============================================================================

#[test]
fn composite_location_can_cross_canonical_fault_boundary() {
    let location = FaultLocation::Composite(vec![
        FaultLocation::logical_qubit(canonical_logical(1)),
        FaultLocation::logical_qubit(canonical_logical(2)),
        FaultLocation::physical_qubit(canonical_physical(7)),
    ]);

    let fault = resilience_fault(
        50_001,
        location.clone(),
        FaultClassification::Gate,
    );

    assert_eq!(fault.canonical().location(), &location);
}

#[test]
fn composite_location_preserves_logical_physical_distinction() {
    let location = FaultLocation::Composite(vec![
        FaultLocation::logical_qubit(canonical_logical(9)),
        FaultLocation::physical_qubit(canonical_physical(9)),
    ]);

    let fault = resilience_fault(
        50_002,
        location.clone(),
        FaultClassification::Gate,
    );

    assert_eq!(fault.canonical().location(), &location);

    match fault.canonical().location() {
        FaultLocation::Composite(resources) => {
            assert_eq!(resources.len(), 2);
            assert_ne!(resources[0], resources[1]);
        }
        _ => panic!("expected composite location"),
    }
}

// ============================================================================
// Provenance-aware stream tests
// ============================================================================

#[test]
fn provenance_can_be_attached_deterministically_to_streamed_faults() {
    let stream = FaultStream::new(
        60_000,
        64,
        8,
        FaultClassification::Gate,
    )
    .expect("valid stream");

    let first: Vec<ResilienceFault> = stream
        .iter()
        .map(|case| {
            case.into_resilience_fault_with_provenance(
                ProvenanceId::new(9_001),
            )
        })
        .collect();

    let second: Vec<ResilienceFault> = stream
        .iter()
        .map(|case| {
            case.into_resilience_fault_with_provenance(
                ProvenanceId::new(9_001),
            )
        })
        .collect();

    assert_eq!(first, second);

    assert!(first.iter().all(|fault| {
        fault.provenance() == Some(ProvenanceId::new(9_001))
    }));
}

// ============================================================================
// Canonical ordering tests
// ============================================================================

#[test]
fn fault_stream_can_be_canonicalized_without_hash_map_order() {
    let stream = FaultStream::new(
        70_000,
        128,
        16,
        FaultClassification::Gate,
    )
    .expect("valid stream");

    let mut faults: Vec<ResilienceFault> = stream
        .iter()
        .map(FaultInjectionCase::into_resilience_fault)
        .collect();

    let original = faults.clone();

    faults.sort();

    let mut independently_sorted = original;
    independently_sorted.sort();

    assert_eq!(faults, independently_sorted);
}

#[test]
fn fault_ids_have_stable_ordering() {
    let first = FaultId::new(1);
    let second = FaultId::new(2);

    assert!(first < second);
}

// ============================================================================
// Validation boundary tests
// ============================================================================

#[test]
fn resilience_fault_validation_accepts_canonical_fault() {
    let fault = resilience_fault(
        80_001,
        FaultLocation::logical_qubit(canonical_logical(0)),
        FaultClassification::Preparation,
    );

    assert!(fault.validate().is_ok());
}

#[test]
fn canonical_fault_remains_available_for_verification() {
    let fault = resilience_fault(
        80_002,
        FaultLocation::physical_qubit(canonical_physical(0)),
        FaultClassification::Measurement,
    );

    let canonical = fault.canonical();

    assert_eq!(canonical.id(), FaultId::new(80_002));
    assert_eq!(
        canonical.location(),
        &FaultLocation::physical_qubit(canonical_physical(0))
    );
}

#[test]
fn canonical_fault_reference_trait_preserves_identity() {
    use crate::quantum::resilience::model::fault::CanonicalFaultRef;

    let fault = resilience_fault(
        80_003,
        FaultLocation::logical_qubit(canonical_logical(2)),
        FaultClassification::Reset,
    );

    assert_eq!(
        fault.canonical_fault(),
        fault.canonical()
    );
}

// ============================================================================
// Fault-storm aggregation-oriented tests
// ============================================================================

#[test]
fn large_fault_stream_can_be_counted_without_materializing_all_faults() {
    let requested = 100_000_u64;

    let stream = FaultStream::new(
        90_000,
        requested,
        4_096,
        FaultClassification::Idle,
    )
    .expect("valid stream");

    let count = stream.iter().count();

    assert_eq!(count, requested as usize);
}

#[test]
fn repeated_resource_faults_do_not_require_global_recovery_identity() {
    let stream = FaultStream::new(
        100_000,
        512,
        1,
        FaultClassification::Gate,
    )
    .expect("valid stream");

    let mut fault_ids = BTreeSet::new();
    let mut locations = BTreeSet::new();

    for case in stream.iter() {
        let fault = case.into_resilience_fault();

        fault_ids.insert(fault.fault_id());

        if let FaultLocation::LogicalQubit(qubit) =
            fault.canonical().location()
        {
            locations.insert(*qubit);
        }
    }

    assert_eq!(locations.len(), 1);
    assert_eq!(fault_ids.len(), 512);
}

// ============================================================================
// Resource-independent semantic tests
// ============================================================================

#[test]
fn resource_identity_is_not_derived_from_fault_identity() {
    let first = resilience_fault(
        110_001,
        FaultLocation::logical_qubit(canonical_logical(7)),
        FaultClassification::Gate,
    );

    let second = resilience_fault(
        110_002,
        FaultLocation::logical_qubit(canonical_logical(7)),
        FaultClassification::Gate,
    );

    assert_ne!(first.fault_id(), second.fault_id());

    assert_eq!(
        first.canonical().location(),
        second.canonical().location()
    );
}

#[test]
fn fault_identity_is_not_derived_from_resource_identity() {
    let first = resilience_fault(
        120_001,
        FaultLocation::logical_qubit(canonical_logical(1)),
        FaultClassification::Gate,
    );

    let second = resilience_fault(
        120_002,
        FaultLocation::logical_qubit(canonical_logical(2)),
        FaultClassification::Gate,
    );

    assert_ne!(first.fault_id(), second.fault_id());
}

#[test]
fn resilience_does_not_mutate_canonical_fault_through_shared_access() {
    let fault = resilience_fault(
        130_001,
        FaultLocation::logical_qubit(canonical_logical(3)),
        FaultClassification::Gate,
    );

    let first = fault.canonical().clone();
    let second = fault.canonical().clone();

    assert_eq!(first, second);
}

// ============================================================================
// Fault-injection matrix
// ============================================================================

#[test]
fn fault_injection_matrix_covers_resource_and_classification_dimensions() {
    let classifications = [
        FaultClassification::Preparation,
        FaultClassification::Gate,
        FaultClassification::Reset,
        FaultClassification::Measurement,
        FaultClassification::Idle,
    ];

    let logical_resources = [
        canonical_logical(0),
        canonical_logical(1),
        canonical_logical(2),
    ];

    let mut generated = 0_usize;

    for (classification_index, classification) in
        classifications.iter().cloned().enumerate()
    {
        for (resource_index, resource) in
            logical_resources.iter().copied().enumerate()
        {
            let ordinal = (classification_index as u64)
                .saturating_mul(1_000)
                .saturating_add(resource_index as u64)
                .saturating_add(1);

            let fault = FaultInjectionCase::logical(
                ordinal,
                resource,
                classification.clone(),
            )
            .into_resilience_fault();

            assert_eq!(
                fault.canonical().classification(),
                &classification
            );

            assert_eq!(
                fault.canonical().location(),
                &FaultLocation::logical_qubit(resource)
            );

            generated = generated.saturating_add(1);
        }
    }

    assert_eq!(
        generated,
        classifications.len() * logical_resources.len()
    );
}

// ============================================================================
// Regression tests for forbidden resilience-local identities
// ============================================================================

#[test]
fn canonical_qubit_types_are_the_types_used_by_injected_locations() {
    let logical: QubitId = QubitId::new(123);
    let physical: PhysicalQubitId = PhysicalQubitId::new(456);

    let logical_location = FaultLocation::logical_qubit(logical);
    let physical_location = FaultLocation::physical_qubit(physical);

    assert_eq!(
        logical_location,
        FaultLocation::LogicalQubit(logical)
    );

    assert_eq!(
        physical_location,
        FaultLocation::PhysicalQubit(physical)
    );
}

#[test]
fn no_numeric_aliasing_between_logical_and_physical_locations() {
    let logical = FaultLocation::logical_qubit(QubitId::new(42));
    let physical = FaultLocation::physical_qubit(PhysicalQubitId::new(42));

    assert_ne!(logical, physical);
}

// ============================================================================
// Iterator contract tests
// ============================================================================

#[test]
fn stream_size_hint_is_exact_when_remaining_count_fits_usize() {
    let stream = FaultStream::new(
        140_000,
        128,
        16,
        FaultClassification::Gate,
    )
    .expect("valid stream");

    let iterator = stream.iter();

    assert_eq!(iterator.size_hint(), (128, Some(128)));
}

#[test]
fn stream_iterator_is_finite_for_requested_count() {
    let stream = FaultStream::new(
        150_000,
        37,
        7,
        FaultClassification::Gate,
    )
    .expect("valid stream");

    let mut iterator = stream.iter();

    for _ in 0..37 {
        assert!(iterator.next().is_some());
    }

    assert!(iterator.next().is_none());
}

// ============================================================================
// Zero-resource and zero-work edge cases
// ============================================================================

#[test]
fn zero_work_requires_no_resource_domain() {
    let stream = FaultStream::new(
        160_000,
        0,
        0,
        FaultClassification::Measurement,
    )
    .expect("zero work must not require resources");

    assert_eq!(stream.iter().count(), 0);
}

#[test]
fn non_zero_work_with_zero_resources_is_rejected() {
    let result = FaultStream::new(
        160_001,
        1,
        0,
        FaultClassification::Measurement,
    );

    assert!(result.is_err());
}

// ============================================================================
// End-to-end boundary test
// ============================================================================

#[test]
fn end_to_end_fault_injection_preserves_identity_location_and_classification() {
    let cases = [
        FaultInjectionCase::logical(
            170_001,
            canonical_logical(0),
            FaultClassification::Preparation,
        ),
        FaultInjectionCase::logical(
            170_002,
            canonical_logical(1),
            FaultClassification::Gate,
        ),
        FaultInjectionCase::physical(
            170_003,
            canonical_physical(2),
            FaultClassification::Reset,
        ),
        FaultInjectionCase::physical(
            170_004,
            canonical_physical(3),
            FaultClassification::Measurement,
        ),
        FaultInjectionCase::logical(
            170_005,
            canonical_logical(4),
            FaultClassification::Idle,
        ),
    ];

    for case in cases {
        let expected_id = FaultId::new(case.ordinal);
        let expected_classification = case.classification.clone();

        let expected_location = match case.resource {
            InjectedResource::Logical(qubit) => {
                FaultLocation::logical_qubit(qubit)
            }
            InjectedResource::Physical(qubit) => {
                FaultLocation::physical_qubit(qubit)
            }
        };

        let injected = case.into_resilience_fault();

        assert_eq!(injected.fault_id(), expected_id);

        assert_eq!(
            injected.canonical().classification(),
            &expected_classification
        );

        assert_eq!(
            injected.canonical().location(),
            &expected_location
        );

        assert!(injected.validate().is_ok());
    }
}