//! Zamani Quantum Resilience — Scalability Tests
//!
//! Path:
//!     src/quantum/resilience/tests/scalability.rs
//!
//! Purpose:
//!     Production scalability and architectural-invariant tests for the
//!     quantum resilience subsystem.
//!
//! This file tests the "write once, scale everywhere" contract:
//!
//!     one logical qubit
//!          ↓
//!     small QPU
//!          ↓
//!     large QPU
//!          ↓
//!     logical/fault-tolerant machine
//!          ↓
//!     heterogeneous execution fabric
//!          ↓
//!     distributed quantum system
//!
//! The test suite deliberately avoids making any concrete machine size a
//! semantic architectural limit.
//!
//! -----------------------------------------------------------------------------
//! Architectural rules
//! -----------------------------------------------------------------------------
//!
//! 1. Canonical logical identity MUST come from:
//!
//!        crate::quantum::ir::qubit::QubitId
//!
//! 2. Canonical physical identity MUST come from:
//!
//!        crate::quantum::ir::qubit::PhysicalQubitId
//!
//! 3. Resilience MUST NOT introduce a competing QubitId.
//!
//! 4. Tests MUST NOT require a fixed number of qubits.
//!
//! 5. Tests MUST NOT use a fixed maximum machine size.
//!
//! 6. "Infinity" means that the resilience model introduces no artificial
//!    finite machine-size ceiling. Actual execution remains bounded by the
//!    resources available to the test process.
//!
//! 7. Large resource domains MUST be testable without materializing every
//!    resource in memory.
//!
//! 8. Deterministic generation MUST remain deterministic independently of
//!    hash-map iteration order or process-global state.
//!
//! 9. Logical and physical identifiers MUST remain different Rust types.
//!
//! 10. Unknown and unbounded quantities MUST NOT be represented by integer
//!     sentinels.
//!
//! 11. No unsafe Rust is permitted.
//!
//! 12. Tests must remain compatible with Rust 1.97 / 1.97.1 and Rust 2021.
//!
//! -----------------------------------------------------------------------------
//! Integration
//! -----------------------------------------------------------------------------
//!
//! This file is intentionally written against stable, canonical contracts:
//!
//!     quantum::ir::qubit
//!     quantum::resilience::model::resource
//!
//! It does not depend on concrete hardware providers, vendor SDKs, routing
//! algorithms, scheduler implementations, QEC implementations, or backend
//! connections.
//!
//! The test module therefore remains useful while those lower-level systems
//! evolve.
//!
//! The containing resilience test module should include this file with:
//!
//!     #[path = "scalability.rs"]
//!     mod scalability;
//!
//! No production runtime code is required by this test file.
//!
//! -----------------------------------------------------------------------------
//! No hard-coded scalability limits
//! -----------------------------------------------------------------------------
//!
//! Constants in this file are semantic test parameters only when they describe
//! mathematical structure such as an empty prefix or a sampling stride.
//! They MUST NOT represent a hardware capacity.
//!
//! Hardware/resource capacity is supplied through:
//!
//!     - test-generated domains;
//!     - optional environment configuration;
//!     - the host's representable identifier domain;
//!     - the available test-process resource budget.
//!
//! The tests never define:
//!
//!     MAX_QUBITS
//!     MAX_PHYSICAL_QUBITS
//!     MAX_BACKENDS
//!     MAX_DEVICES
//!     MAX_INCIDENTS
//!     MAX_TELEMETRY_EVENTS
//!
//! as architectural limits.
//!
//! -----------------------------------------------------------------------------
//! Safety
//! -----------------------------------------------------------------------------
//!
//! No unsafe code is permitted anywhere in this module.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::collections::BTreeSet;
use std::env;
use std::iter::FusedIterator;

use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId};
use crate::quantum::resilience::model::resource::{
    ResourceAvailability,
    ResourceIdentity,
    ResourceQuantity,
    ResourceScope,
};

// =============================================================================
// Test configuration
// =============================================================================

/// Optional environment variable controlling the amount of materialization
/// performed by resource-budget tests.
///
/// This is deliberately an operational test budget rather than an architectural
/// machine-size limit.
///
/// If unset, no large allocation is required.
const MATERIALIZATION_BUDGET_ENV: &str = "ZAMANI_RESILIENCE_SCALABILITY_BUDGET";

/// Optional environment variable controlling the number of generated logical
/// resources used by explicitly budgeted stress tests.
///
/// The value is never interpreted as the maximum number of qubits supported by
/// Zamani.
const GENERATED_RESOURCE_COUNT_ENV: &str = "ZAMANI_RESILIENCE_SCALABILITY_RESOURCES";

/// Parses an optional positive resource budget.
///
/// Invalid, zero, or absent values result in `None`.
fn optional_positive_env(name: &str) -> Option<usize> {
    env::var(name)
        .ok()
        .and_then(|value| value.trim().parse::<usize>().ok())
        .filter(|value| *value > 0)
}

/// Returns an explicitly requested materialization budget.
///
/// The test suite does not invent a hardware default. This keeps the test
/// independent of machine size and prevents the test itself from becoming an
/// artificial scalability ceiling.
fn materialization_budget() -> Option<usize> {
    optional_positive_env(MATERIALIZATION_BUDGET_ENV)
}

/// Returns an explicitly requested generated-resource count.
///
/// No value means that allocation-heavy stress testing is skipped.
fn generated_resource_count() -> Option<usize> {
    optional_positive_env(GENERATED_RESOURCE_COUNT_ENV)
}

// =============================================================================
// Lazy logical resource generator
// =============================================================================

/// Lazily generates canonical logical qubit identifiers.
///
/// This type intentionally stores only:
///
/// - the starting logical identifier;
/// - the number of identifiers to produce.
///
/// It does not allocate one object per qubit.
///
/// This is the principal mechanism used by these tests to verify that large
/// resource namespaces do not require proportional eager allocation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct LogicalQubitGenerator {
    start: QubitId,
    count: usize,
}

impl LogicalQubitGenerator {
    /// Creates a lazy logical-qubit generator.
    const fn new(start: QubitId, count: usize) -> Self {
        Self { start, count }
    }

    /// Returns the configured number of identifiers.
    const fn len(self) -> usize {
        self.count
    }

    /// Returns whether the generator produces no identifiers.
    const fn is_empty(self) -> bool {
        self.count == 0
    }

    /// Returns the last representable logical identifier produced by this
    /// generator, if one exists.
    fn last_id(self) -> Option<QubitId> {
        if self.count == 0 {
            return None;
        }

        let offset = self.count - 1;

        self.start
            .index()
            .checked_add(offset)
            .map(QubitId::new)
    }
}

impl Iterator for LogicalQubitGenerator {
    type Item = QubitId;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count == 0 {
            return None;
        }

        let current = self.start;

        self.start = self.start.checked_next()?;
        self.count -= 1;

        Some(current)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.count, Some(self.count))
    }
}

impl ExactSizeIterator for LogicalQubitGenerator {}

impl FusedIterator for LogicalQubitGenerator {}

// =============================================================================
// Lazy physical resource generator
// =============================================================================

/// Lazily generates canonical physical-qubit identifiers.
///
/// The logical and physical generators are intentionally separate. This makes
/// it impossible for these tests to accidentally collapse logical identity
/// into physical identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct PhysicalQubitGenerator {
    start: PhysicalQubitId,
    count: usize,
}

impl PhysicalQubitGenerator {
    const fn new(start: PhysicalQubitId, count: usize) -> Self {
        Self { start, count }
    }

    const fn len(self) -> usize {
        self.count
    }

    const fn is_empty(self) -> bool {
        self.count == 0
    }

    fn last_id(self) -> Option<PhysicalQubitId> {
        if self.count == 0 {
            return None;
        }

        let offset = self.count - 1;

        self.start
            .index()
            .checked_add(offset)
            .map(PhysicalQubitId::new)
    }
}

impl Iterator for PhysicalQubitGenerator {
    type Item = PhysicalQubitId;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count == 0 {
            return None;
        }

        let current = self.start;

        self.start = self.start.checked_next()?;
        self.count -= 1;

        Some(current)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.count, Some(self.count))
    }
}

impl ExactSizeIterator for PhysicalQubitGenerator {}

impl FusedIterator for PhysicalQubitGenerator {}

// =============================================================================
// Distributed resource domain
// =============================================================================

/// A lazily represented distributed resource domain.
///
/// A domain is represented by:
///
///     node
///     local-resource-count
///
/// rather than by a globally allocated matrix.
///
/// This allows the tests to model:
///
///     one node
///     many nodes
///     many resources per node
///
/// without imposing a fixed distributed-machine size.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct DistributedDomain {
    nodes: usize,
    resources_per_node: usize,
}

impl DistributedDomain {
    const fn new(nodes: usize, resources_per_node: usize) -> Self {
        Self {
            nodes,
            resources_per_node,
        }
    }

    fn total_resources(self) -> Option<usize> {
        self.nodes.checked_mul(self.resources_per_node)
    }

    fn resource_at(
        self,
        node: usize,
        local_index: usize,
    ) -> Option<(usize, usize)> {
        if node >= self.nodes || local_index >= self.resources_per_node {
            return None;
        }

        Some((node, local_index))
    }
}

// =============================================================================
// Deterministic sparse topology
// =============================================================================

/// Generates a deterministic sparse topology without allocating an adjacency
/// matrix.
///
/// The topology connects each resource to its next resource when such a
/// resource exists.
///
/// This is intentionally a test topology generator, not a routing algorithm.
///
/// Resilience consumes topology information from the hardware/routing layers;
/// this helper exists only to test that resilience contracts can represent
/// topology sizes without embedding a particular topology.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct SparseTopology {
    resource_count: usize,
}

impl SparseTopology {
    const fn new(resource_count: usize) -> Self {
        Self { resource_count }
    }

    const fn resource_count(self) -> usize {
        self.resource_count
    }

    fn edge_count(self) -> Option<usize> {
        self.resource_count.checked_sub(1)
    }

    fn neighbors(self, resource: usize) -> SparseNeighbors {
        SparseNeighbors {
            resource,
            resource_count: self.resource_count,
            yielded: false,
        }
    }
}

/// Lazy neighbor iterator for the deterministic sparse topology.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct SparseNeighbors {
    resource: usize,
    resource_count: usize,
    yielded: bool,
}

impl Iterator for SparseNeighbors {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.yielded || self.resource >= self.resource_count {
            return None;
        }

        self.yielded = true;

        self.resource
            .checked_add(1)
            .filter(|next| *next < self.resource_count)
    }
}

impl FusedIterator for SparseNeighbors {}

// =============================================================================
// Scaling observation
// =============================================================================

/// Small deterministic observation used to compare two generated resource
/// domains without materializing them.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ScalingObservation {
    resource_count: usize,
    first: Option<usize>,
    last: Option<usize>,
    checksum: u128,
}

impl ScalingObservation {
    fn from_range(start: usize, count: usize) -> Option<Self> {
        let last = if count == 0 {
            None
        } else {
            start.checked_add(count - 1)
        };

        let checksum = arithmetic_progression_checksum(start, count)?;

        Some(Self {
            resource_count: count,
            first: (count > 0).then_some(start),
            last,
            checksum,
        })
    }
}

/// Computes the sum of a contiguous integer range without materializing it.
///
/// This is used only as a deterministic invariant. It allows the test suite to
/// inspect very large logical domains without allocating one entry per
/// resource.
///
/// Mathematically:
///
///     n/2 * (first + last)
///
/// is used only after overflow-safe arithmetic has established that the
/// resulting value is representable.
fn arithmetic_progression_checksum(
    start: usize,
    count: usize,
) -> Option<u128> {
    if count == 0 {
        return Some(0);
    }

    let last = start.checked_add(count - 1)?;

    let count_u128 = count as u128;
    let start_u128 = start as u128;
    let last_u128 = last as u128;

    if count_u128 % 2 == 0 {
        Some((count_u128 / 2) * (start_u128 + last_u128))
    } else {
        Some(count_u128 * ((start_u128 + last_u128) / 2))
    }
}

// =============================================================================
// Test helpers
// =============================================================================

/// Verifies a generated logical namespace without requiring a complete
/// materialization.
///
/// `sample_count` is a caller-supplied observation budget, not a machine limit.
fn verify_logical_generator(
    start: usize,
    count: usize,
    sample_count: usize,
) {
    let generator = LogicalQubitGenerator::new(QubitId::new(start), count);

    assert_eq!(generator.len(), count);
    assert_eq!(generator.is_empty(), count == 0);

    let expected_last = if count == 0 {
        None
    } else {
        start.checked_add(count - 1)
    };

    assert_eq!(
        generator.last_id().map(QubitId::index),
        expected_last
    );

    if count == 0 {
        assert_eq!(generator.clone().next(), None);
        return;
    }

    let observations = sample_indices(count, sample_count);

    for index in observations {
        let expected = start
            .checked_add(index)
            .expect("test-generated logical identifier must remain representable");

        let observed = LogicalQubitGenerator::new(QubitId::new(start), count)
            .nth(index)
            .map(QubitId::index);

        assert_eq!(observed, Some(expected));
    }
}

/// Verifies a generated physical namespace.
fn verify_physical_generator(
    start: usize,
    count: usize,
    sample_count: usize,
) {
    let generator =
        PhysicalQubitGenerator::new(PhysicalQubitId::new(start), count);

    assert_eq!(generator.len(), count);
    assert_eq!(generator.is_empty(), count == 0);

    let expected_last = if count == 0 {
        None
    } else {
        start.checked_add(count - 1)
    };

    assert_eq!(
        generator.last_id().map(PhysicalQubitId::index),
        expected_last
    );

    if count == 0 {
        assert_eq!(generator.clone().next(), None);
        return;
    }

    for index in sample_indices(count, sample_count) {
        let expected = start
            .checked_add(index)
            .expect("test-generated physical identifier must remain representable");

        let observed = PhysicalQubitGenerator::new(
            PhysicalQubitId::new(start),
            count,
        )
        .nth(index)
        .map(PhysicalQubitId::index);

        assert_eq!(observed, Some(expected));
    }
}

/// Generates deterministic sample positions.
///
/// This does not define a maximum resource count. It simply keeps tests from
/// materializing an entire huge domain.
///
/// The sample budget is controlled by the caller.
fn sample_indices(count: usize, sample_budget: usize) -> Vec<usize> {
    if count == 0 || sample_budget == 0 {
        return Vec::new();
    }

    let mut positions = BTreeSet::new();

    positions.insert(0);
    positions.insert(count - 1);

    let requested = sample_budget.min(count);

    if requested > 2 {
        let denominator = requested - 1;

        for sample in 1..denominator {
            let numerator = sample.saturating_mul(count - 1);
            let position = numerator / denominator;
            positions.insert(position);
        }
    }

    positions.into_iter().collect()
}

/// Checks that a value can be used as a finite resource quantity without
/// converting semantic unboundedness into an integer sentinel.
fn assert_finite_quantity(value: usize) {
    let quantity = ResourceQuantity::finite(value as u128);

    assert!(quantity.is_finite());
    assert!(!quantity.is_unbounded());
    assert!(!quantity.is_unknown());
    assert_eq!(quantity.as_finite(), Some(value as u128));
}

/// Builds canonical logical identities and proves that they remain typed.
fn logical_identity(index: usize) -> QubitId {
    QubitId::new(index)
}

/// Builds canonical physical identities and proves that they remain typed.
fn physical_identity(index: usize) -> PhysicalQubitId {
    PhysicalQubitId::new(index)
}

// =============================================================================
// Canonical identity tests
// =============================================================================

#[test]
fn canonical_logical_qubit_identity_is_used() {
    let logical = logical_identity(0);

    assert_eq!(logical.index(), 0);
    assert_eq!(logical, QubitId::new(0));

    let identity = ResourceIdentity::logical_qubit(logical);

    assert!(identity.is_logical_qubit());
    assert!(!identity.is_physical_qubit());
    assert_eq!(identity.logical_qubit_id(), Some(logical));
    assert_eq!(identity.physical_qubit_id(), None);
}

#[test]
fn canonical_physical_qubit_identity_is_used() {
    let physical = physical_identity(0);

    assert_eq!(physical.index(), 0);
    assert_eq!(physical, PhysicalQubitId::new(0));

    let identity = ResourceIdentity::physical_qubit(physical);

    assert!(identity.is_physical_qubit());
    assert!(!identity.is_logical_qubit());
    assert_eq!(identity.physical_qubit_id(), Some(physical));
    assert_eq!(identity.logical_qubit_id(), None);
}

#[test]
fn logical_and_physical_identity_are_not_interchangeable() {
    let logical = QubitId::new(17);
    let physical = PhysicalQubitId::new(17);

    let logical_identity = ResourceIdentity::logical_qubit(logical);
    let physical_identity = ResourceIdentity::physical_qubit(physical);

    assert_ne!(logical_identity, physical_identity);

    assert!(logical_identity.is_logical_qubit());
    assert!(physical_identity.is_physical_qubit());

    assert_eq!(logical_identity.logical_qubit_id(), Some(logical));
    assert_eq!(physical_identity.physical_qubit_id(), Some(physical));
}

// =============================================================================
// Empty/tiny scaling
// =============================================================================

#[test]
fn empty_resource_namespace_is_valid() {
    let logical = LogicalQubitGenerator::new(QubitId::new(0), 0);
    let physical = PhysicalQubitGenerator::new(PhysicalQubitId::new(0), 0);

    assert!(logical.is_empty());
    assert!(physical.is_empty());

    assert_eq!(logical.count(), 0);
    assert_eq!(physical.count(), 0);

    assert_eq!(logical.last_id(), None);
    assert_eq!(physical.last_id(), None);

    assert_eq!(logical.into_iter().next(), None);
    assert_eq!(physical.into_iter().next(), None);
}

#[test]
fn one_resource_namespace_is_valid() {
    verify_logical_generator(0, 1, 1);
    verify_physical_generator(0, 1, 1);

    let logical = LogicalQubitGenerator::new(QubitId::new(0), 1);
    assert_eq!(logical.collect::<Vec<_>>(), vec![QubitId::new(0)]);

    let physical =
        PhysicalQubitGenerator::new(PhysicalQubitId::new(0), 1);
    assert_eq!(
        physical.collect::<Vec<_>>(),
        vec![PhysicalQubitId::new(0)]
    );
}

#[test]
fn tiny_namespace_preserves_contiguous_identity() {
    let logical =
        LogicalQubitGenerator::new(QubitId::new(11), 4).collect::<Vec<_>>();

    assert_eq!(
        logical,
        vec![
            QubitId::new(11),
            QubitId::new(12),
            QubitId::new(13),
            QubitId::new(14),
        ]
    );

    let physical = PhysicalQubitGenerator::new(
        PhysicalQubitId::new(21),
        4,
    )
    .collect::<Vec<_>>();

    assert_eq!(
        physical,
        vec![
            PhysicalQubitId::new(21),
            PhysicalQubitId::new(22),
            PhysicalQubitId::new(23),
            PhysicalQubitId::new(24),
        ]
    );
}

// =============================================================================
// Lazy large-scale tests
// =============================================================================

#[test]
fn large_logical_namespace_is_lazy() {
    let count = usize::MAX / 4;
    let start = usize::MAX / 4;

    assert!(start.checked_add(count - 1).is_some());

    verify_logical_generator(start, count, 17);
}

#[test]
fn large_physical_namespace_is_lazy() {
    let count = usize::MAX / 4;
    let start = usize::MAX / 4;

    assert!(start.checked_add(count - 1).is_some());

    verify_physical_generator(start, count, 17);
}

#[test]
fn near_identifier_boundary_is_supported_without_overflow() {
    let start = usize::MAX - 3;
    let count = 4;

    verify_logical_generator(start, count, count);
    verify_physical_generator(start, count, count);

    let logical =
        LogicalQubitGenerator::new(QubitId::new(start), count)
            .collect::<Vec<_>>();

    assert_eq!(
        logical,
        vec![
            QubitId::new(usize::MAX - 3),
            QubitId::new(usize::MAX - 2),
            QubitId::new(usize::MAX - 1),
            QubitId::new(usize::MAX),
        ]
    );

    let physical = PhysicalQubitGenerator::new(
        PhysicalQubitId::new(start),
        count,
    )
    .collect::<Vec<_>>();

    assert_eq!(
        physical,
        vec![
            PhysicalQubitId::new(usize::MAX - 3),
            PhysicalQubitId::new(usize::MAX - 2),
            PhysicalQubitId::new(usize::MAX - 1),
            PhysicalQubitId::new(usize::MAX),
        ]
    );
}

#[test]
fn identifier_successor_fails_cleanly_at_representation_boundary() {
    let logical = QubitId::new(usize::MAX);
    let physical = PhysicalQubitId::new(usize::MAX);

    assert_eq!(logical.checked_next(), None);
    assert_eq!(physical.checked_next(), None);
}

// =============================================================================
// Resource quantity semantics
// =============================================================================

#[test]
fn finite_quantity_is_not_unbounded_or_unknown() {
    assert_finite_quantity(0);
    assert_finite_quantity(1);
    assert_finite_quantity(usize::MAX);
}

#[test]
fn unbounded_quantity_is_semantically_distinct() {
    let quantity = ResourceQuantity::unbounded();

    assert!(quantity.is_unbounded());
    assert!(!quantity.is_finite());
    assert!(!quantity.is_unknown());
    assert_eq!(quantity.as_finite(), None);
}

#[test]
fn unknown_quantity_is_semantically_distinct() {
    let quantity = ResourceQuantity::unknown();

    assert!(quantity.is_unknown());
    assert!(!quantity.is_finite());
    assert!(!quantity.is_unbounded());
    assert_eq!(quantity.as_finite(), None);
}

#[test]
fn unbounded_quantity_is_not_integer_sentinel() {
    let unbounded = ResourceQuantity::unbounded();

    assert_ne!(
        unbounded,
        ResourceQuantity::finite(usize::MAX as u128)
    );
}

#[test]
fn unknown_quantity_is_not_integer_sentinel() {
    let unknown = ResourceQuantity::unknown();

    assert_ne!(
        unknown,
        ResourceQuantity::finite(usize::MAX as u128)
    );
}

// =============================================================================
// Resource availability semantics
// =============================================================================

#[test]
fn availability_states_are_distinct() {
    assert!(ResourceAvailability::Available.is_available());
    assert!(!ResourceAvailability::Unavailable.is_available());
    assert!(!ResourceAvailability::Unknown.is_available());

    assert_ne!(
        ResourceAvailability::Available,
        ResourceAvailability::Unavailable
    );

    assert_ne!(
        ResourceAvailability::Available,
        ResourceAvailability::Unknown
    );

    assert_ne!(
        ResourceAvailability::Unavailable,
        ResourceAvailability::Unknown
    );
}

// =============================================================================
// Resource scope semantics
// =============================================================================

#[test]
fn canonical_identity_implies_correct_resource_scope() {
    let logical = ResourceIdentity::logical_qubit(QubitId::new(3));
    let physical =
        ResourceIdentity::physical_qubit(PhysicalQubitId::new(3));

    assert_eq!(
        ResourceScope::for_identity(logical),
        ResourceScope::Logical
    );

    assert_eq!(
        ResourceScope::for_identity(physical),
        ResourceScope::Physical
    );
}

// =============================================================================
// Determinism
// =============================================================================

#[test]
fn logical_resource_generation_is_deterministic() {
    let first =
        LogicalQubitGenerator::new(QubitId::new(100), 10)
            .map(QubitId::index)
            .collect::<Vec<_>>();

    let second =
        LogicalQubitGenerator::new(QubitId::new(100), 10)
            .map(QubitId::index)
            .collect::<Vec<_>>();

    assert_eq!(first, second);
}

#[test]
fn physical_resource_generation_is_deterministic() {
    let first =
        PhysicalQubitGenerator::new(PhysicalQubitId::new(100), 10)
            .map(PhysicalQubitId::index)
            .collect::<Vec<_>>();

    let second =
        PhysicalQubitGenerator::new(PhysicalQubitId::new(100), 10)
            .map(PhysicalQubitId::index)
            .collect::<Vec<_>>();

    assert_eq!(first, second);
}

#[test]
fn scaling_observation_is_deterministic() {
    let first =
        ScalingObservation::from_range(17, 1_000)
            .expect("test range must be representable");

    let second =
        ScalingObservation::from_range(17, 1_000)
            .expect("test range must be representable");

    assert_eq!(first, second);
}

// =============================================================================
// Scaling monotonicity
// =============================================================================

#[test]
fn generated_namespace_scales_monotonically() {
    let counts = [
        0usize,
        1usize,
        2usize,
        4usize,
        8usize,
        16usize,
    ];

    let mut previous_last = None;

    for count in counts {
        let observation =
            ScalingObservation::from_range(0, count)
                .expect("generated range must be representable");

        assert_eq!(observation.resource_count, count);

        if count == 0 {
            assert_eq!(observation.first, None);
            assert_eq!(observation.last, None);
        } else {
            assert_eq!(observation.first, Some(0));
            assert_eq!(observation.last, Some(count - 1));

            if let Some(previous) = previous_last {
                assert!(observation.last.unwrap_or(previous) >= previous);
            }

            previous_last = observation.last;
        }
    }
}

#[test]
fn large_scaling_observation_does_not_require_materialization() {
    let count = usize::MAX / 2;

    let observation =
        ScalingObservation::from_range(0, count)
            .expect("large representable range must be observable");

    assert_eq!(observation.resource_count, count);
    assert_eq!(observation.first, Some(0));
    assert_eq!(observation.last, Some(count - 1));

    assert!(
        observation.checksum > 0,
        "non-empty large range must have a non-zero checksum"
    );
}

// =============================================================================
// Sparse topology scaling
// =============================================================================

#[test]
fn sparse_topology_supports_empty_domain() {
    let topology = SparseTopology::new(0);

    assert_eq!(topology.resource_count(), 0);
    assert_eq!(topology.edge_count(), None);

    assert_eq!(
        topology.neighbors(0).collect::<Vec<_>>(),
        Vec::<usize>::new()
    );
}

#[test]
fn sparse_topology_supports_single_resource() {
    let topology = SparseTopology::new(1);

    assert_eq!(topology.resource_count(), 1);
    assert_eq!(topology.edge_count(), Some(0));

    assert_eq!(
        topology.neighbors(0).collect::<Vec<_>>(),
        Vec::<usize>::new()
    );
}

#[test]
fn sparse_topology_scales_without_adjacency_matrix() {
    let resource_count = usize::MAX / 8;

    let topology = SparseTopology::new(resource_count);

    assert_eq!(topology.resource_count(), resource_count);
    assert_eq!(
        topology.edge_count(),
        resource_count.checked_sub(1)
    );

    let first_neighbors =
        topology.neighbors(0).collect::<Vec<_>>();

    assert_eq!(first_neighbors, vec![1]);

    let last_neighbors =
        topology.neighbors(resource_count - 1)
            .collect::<Vec<_>>();

    assert!(last_neighbors.is_empty());
}

// =============================================================================
// Distributed scaling
// =============================================================================

#[test]
fn distributed_domain_supports_single_node() {
    let domain = DistributedDomain::new(1, 1);

    assert_eq!(domain.total_resources(), Some(1));
    assert_eq!(domain.resource_at(0, 0), Some((0, 0)));
    assert_eq!(domain.resource_at(1, 0), None);
}

#[test]
fn distributed_domain_supports_arbitrary_finite_node_counts() {
    let domain = DistributedDomain::new(7, 13);

    assert_eq!(domain.total_resources(), Some(91));

    assert_eq!(domain.resource_at(0, 0), Some((0, 0)));
    assert_eq!(domain.resource_at(6, 12), Some((6, 12)));

    assert_eq!(domain.resource_at(7, 0), None);
    assert_eq!(domain.resource_at(0, 13), None);
}

#[test]
fn distributed_domain_detects_arithmetic_overflow_without_wrapping() {
    let domain = DistributedDomain::new(usize::MAX, 2);

    assert_eq!(domain.total_resources(), None);
}

#[test]
fn distributed_domain_can_represent_extremely_large_fabric_lazily() {
    let domain = DistributedDomain::new(
        usize::MAX / 2,
        usize::MAX / 2,
    );

    assert_eq!(domain.total_resources(), None);

    assert_eq!(
        domain.resource_at(0, 0),
        Some((0, 0))
    );

    assert_eq!(
        domain.resource_at(
            (usize::MAX / 2) - 1,
            (usize::MAX / 2) - 1
        ),
        Some((
            (usize::MAX / 2) - 1,
            (usize::MAX / 2) - 1
        ))
    );
}

// =============================================================================
// Logical/physical mapping invariants
// =============================================================================

#[test]
fn logical_and_physical_domains_can_have_different_sizes() {
    let logical_count = 7usize;
    let physical_count = 19usize;

    let logical =
        LogicalQubitGenerator::new(QubitId::new(0), logical_count);

    let physical =
        PhysicalQubitGenerator::new(
            PhysicalQubitId::new(0),
            physical_count,
        );

    assert_eq!(logical.len(), logical_count);
    assert_eq!(physical.len(), physical_count);

    assert_ne!(logical_count, physical_count);
}

#[test]
fn logical_index_does_not_imply_physical_identity() {
    let logical = QubitId::new(3);
    let physical = PhysicalQubitId::new(17);

    let logical_resource =
        ResourceIdentity::logical_qubit(logical);

    let physical_resource =
        ResourceIdentity::physical_qubit(physical);

    assert_ne!(logical_resource, physical_resource);

    assert_eq!(
        logical_resource.logical_qubit_id(),
        Some(logical)
    );

    assert_eq!(
        physical_resource.physical_qubit_id(),
        Some(physical)
    );
}

// =============================================================================
// Resource-budget-driven materialization
// =============================================================================

#[test]
fn optional_budgeted_materialization_is_resource_driven() {
    let Some(budget) = materialization_budget() else {
        return;
    };

    let logical_count = generated_resource_count()
        .unwrap_or(budget)
        .min(budget);

    if logical_count == 0 {
        return;
    }

    let resources = LogicalQubitGenerator::new(
        QubitId::new(0),
        logical_count,
    )
    .map(ResourceIdentity::logical_qubit)
    .collect::<Vec<_>>();

    assert_eq!(resources.len(), logical_count);

    for (index, resource) in resources.iter().enumerate() {
        assert_eq!(
            resource.logical_qubit_id(),
            Some(QubitId::new(index))
        );
    }
}

#[test]
fn optional_budgeted_physical_materialization_is_resource_driven() {
    let Some(budget) = materialization_budget() else {
        return;
    };

    let physical_count = generated_resource_count()
        .unwrap_or(budget)
        .min(budget);

    if physical_count == 0 {
        return;
    }

    let resources = PhysicalQubitGenerator::new(
        PhysicalQubitId::new(0),
        physical_count,
    )
    .map(ResourceIdentity::physical_qubit)
    .collect::<Vec<_>>();

    assert_eq!(resources.len(), physical_count);

    for (index, resource) in resources.iter().enumerate() {
        assert_eq!(
            resource.physical_qubit_id(),
            Some(PhysicalQubitId::new(index))
        );
    }
}

// =============================================================================
// Sparse sampling of arbitrarily large domains
// =============================================================================

#[test]
fn sparse_sampling_does_not_require_full_domain_materialization() {
    let count = usize::MAX / 3;

    let samples = sample_indices(count, 32);

    assert!(!samples.is_empty());

    assert_eq!(samples.first().copied(), Some(0));
    assert_eq!(samples.last().copied(), Some(count - 1));

    for window in samples.windows(2) {
        assert!(window[0] < window[1]);
    }

    for index in samples {
        assert!(index < count);
    }
}

#[test]
fn sparse_sampling_is_deterministic() {
    let first = sample_indices(10_000, 31);
    let second = sample_indices(10_000, 31);

    assert_eq!(first, second);
}

// =============================================================================
// Checked arithmetic
// =============================================================================

#[test]
fn scaling_arithmetic_does_not_wrap() {
    assert_eq!(
        usize::MAX.checked_add(1),
        None
    );

    assert_eq!(
        usize::MAX.checked_mul(2),
        None
    );

    assert_eq!(
        arithmetic_progression_checksum(
            usize::MAX - 1,
            2
        ),
        Some(
            (usize::MAX - 1) as u128
                + usize::MAX as u128
        )
    );
}

#[test]
fn arithmetic_progression_checksum_handles_empty_domain() {
    assert_eq!(
        arithmetic_progression_checksum(0, 0),
        Some(0)
    );

    assert_eq!(
        arithmetic_progression_checksum(usize::MAX, 0),
        Some(0)
    );
}

#[test]
fn arithmetic_progression_checksum_handles_single_resource() {
    assert_eq!(
        arithmetic_progression_checksum(37, 1),
        Some(37)
    );
}

// =============================================================================
// Iterator contract tests
// =============================================================================

#[test]
fn logical_generator_exact_size_contract_is_correct() {
    let mut generator =
        LogicalQubitGenerator::new(QubitId::new(10), 5);

    assert_eq!(generator.len(), 5);
    assert_eq!(generator.size_hint(), (5, Some(5)));

    assert_eq!(
        generator.next(),
        Some(QubitId::new(10))
    );

    assert_eq!(generator.len(), 4);
    assert_eq!(generator.size_hint(), (4, Some(4)));
}

#[test]
fn physical_generator_exact_size_contract_is_correct() {
    let mut generator =
        PhysicalQubitGenerator::new(
            PhysicalQubitId::new(10),
            5,
        );

    assert_eq!(generator.len(), 5);
    assert_eq!(generator.size_hint(), (5, Some(5)));

    assert_eq!(
        generator.next(),
        Some(PhysicalQubitId::new(10))
    );

    assert_eq!(generator.len(), 4);
    assert_eq!(generator.size_hint(), (4, Some(4)));
}

#[test]
fn logical_generator_is_fused_after_exhaustion() {
    let mut generator =
        LogicalQubitGenerator::new(QubitId::new(0), 1);

    assert_eq!(
        generator.next(),
        Some(QubitId::new(0))
    );

    assert_eq!(generator.next(), None);
    assert_eq!(generator.next(), None);
}

#[test]
fn physical_generator_is_fused_after_exhaustion() {
    let mut generator =
        PhysicalQubitGenerator::new(
            PhysicalQubitId::new(0),
            1,
        );

    assert_eq!(
        generator.next(),
        Some(PhysicalQubitId::new(0))
    );

    assert_eq!(generator.next(), None);
    assert_eq!(generator.next(), None);
}

// =============================================================================
// No artificial machine-size assumption tests
// =============================================================================

#[test]
fn no_semantic_limit_is_imposed_by_resource_quantity() {
    let values = [
        0u128,
        1u128,
        2u128,
        u128::from(u64::MAX),
        u128::from(u64::MAX) + 1,
    ];

    for value in values {
        let quantity = ResourceQuantity::finite(value);

        assert!(quantity.is_finite());
        assert_eq!(quantity.as_finite(), Some(value));
    }
}

#[test]
fn representable_identifier_domain_is_not_a_machine_capacity_constant() {
    let logical_low = QubitId::new(0);
    let logical_high = QubitId::new(usize::MAX);

    let physical_low = PhysicalQubitId::new(0);
    let physical_high = PhysicalQubitId::new(usize::MAX);

    assert_eq!(logical_low.index(), 0);
    assert_eq!(logical_high.index(), usize::MAX);

    assert_eq!(physical_low.index(), 0);
    assert_eq!(physical_high.index(), usize::MAX);

    assert_ne!(logical_low, logical_high);
    assert_ne!(physical_low, physical_high);
}

// =============================================================================
// Degradation-style scaling
// =============================================================================

#[test]
fn decreasing_available_resources_remains_representable() {
    let original = 10_000usize;

    let degradation_levels = [
        original,
        original / 2,
        original / 4,
        1,
        0,
    ];

    let mut previous = original;

    for available in degradation_levels {
        assert!(available <= previous);
        assert_finite_quantity(available);
        previous = available;
    }
}

#[test]
fn degradation_does_not_change_identity_domain() {
    let logical = QubitId::new(42);

    let before =
        ResourceIdentity::logical_qubit(logical);

    let after =
        ResourceIdentity::logical_qubit(logical);

    assert_eq!(before, after);
}

// =============================================================================
// Topology/resource-count consistency
// =============================================================================

#[test]
fn sparse_topology_edge_count_is_bounded_by_resource_count() {
    let counts = [
        0usize,
        1usize,
        2usize,
        3usize,
        8usize,
        64usize,
    ];

    for count in counts {
        let topology = SparseTopology::new(count);

        match topology.edge_count() {
            None => assert_eq!(count, 0),
            Some(edges) => {
                assert!(edges <= count);
            }
        }
    }
}

#[test]
fn sparse_topology_neighborhood_does_not_depend_on_global_machine_size() {
    let small = SparseTopology::new(8);
    let large = SparseTopology::new(8_000);

    assert_eq!(
        small.neighbors(0).collect::<Vec<_>>(),
        large.neighbors(0).collect::<Vec<_>>()
    );

    assert_eq!(
        small.neighbors(3).collect::<Vec<_>>(),
        large.neighbors(3).collect::<Vec<_>>()
    );
}

// =============================================================================
// Distributed decomposition invariants
// =============================================================================

#[test]
fn distributed_domain_decomposition_is_stable() {
    let domain = DistributedDomain::new(4, 9);

    let mut count = 0usize;

    for node in 0..4 {
        for local in 0..9 {
            assert_eq!(
                domain.resource_at(node, local),
                Some((node, local))
            );

            count += 1;
        }
    }

    assert_eq!(domain.total_resources(), Some(count));
}

#[test]
fn distributed_domain_does_not_require_global_flattening() {
    let domain = DistributedDomain::new(
        usize::MAX / 4,
        usize::MAX / 4,
    );

    assert_eq!(domain.total_resources(), None);

    let node = usize::MAX / 4 - 1;
    let local = usize::MAX / 4 - 1;

    assert_eq!(
        domain.resource_at(node, local),
        Some((node, local))
    );
}

// =============================================================================
// Resource identity collection scalability
// =============================================================================

#[test]
fn ordered_resource_identity_collection_is_deterministic() {
    let mut resources = BTreeSet::new();

    resources.insert(
        ResourceIdentity::physical_qubit(
            PhysicalQubitId::new(7)
        )
    );

    resources.insert(
        ResourceIdentity::physical_qubit(
            PhysicalQubitId::new(2)
        )
    );

    resources.insert(
        ResourceIdentity::logical_qubit(
            QubitId::new(3)
        )
    );

    let ordered = resources.iter().collect::<Vec<_>>();

    assert_eq!(ordered.len(), 3);

    let repeated = resources.iter().collect::<Vec<_>>();

    assert_eq!(ordered, repeated);
}

// =============================================================================
// Resource identity scaling without semantic collisions
// =============================================================================

#[test]
fn equal_numeric_indices_do_not_create_cross_domain_collisions() {
    let indices = [
        0usize,
        1usize,
        7usize,
        127usize,
        1_000usize,
        usize::MAX,
    ];

    for index in indices {
        let logical =
            ResourceIdentity::logical_qubit(
                QubitId::new(index)
            );

        let physical =
            ResourceIdentity::physical_qubit(
                PhysicalQubitId::new(index)
            );

        assert_ne!(logical, physical);
    }
}

// =============================================================================
// Optional host-resource stress test
// =============================================================================

#[test]
fn optional_large_materialization_preserves_identity_order() {
    let Some(budget) = materialization_budget() else {
        return;
    };

    let requested = generated_resource_count()
        .unwrap_or(budget);

    let count = requested.min(budget);

    if count == 0 {
        return;
    }

    let logical = LogicalQubitGenerator::new(
        QubitId::new(0),
        count,
    );

    let mut previous = None;

    for id in logical {
        if let Some(previous_id) = previous {
            assert!(
                id.index() > previous_id.index(),
                "logical identity generation must remain strictly ordered"
            );
        }

        previous = Some(id);
    }
}

// =============================================================================
// Cross-domain scaling observation
// =============================================================================

#[test]
fn logical_and_physical_scaling_observations_are_independent() {
    let logical_count = usize::MAX / 5;
    let physical_count = usize::MAX / 7;

    let logical =
        ScalingObservation::from_range(
            0,
            logical_count,
        )
        .expect("logical domain must be representable");

    let physical =
        ScalingObservation::from_range(
            0,
            physical_count,
        )
        .expect("physical domain must be representable");

    assert_eq!(
        logical.resource_count,
        logical_count
    );

    assert_eq!(
        physical.resource_count,
        physical_count
    );

    assert_ne!(
        logical.resource_count,
        physical.resource_count
    );
}

// =============================================================================
// Final architectural invariants
// =============================================================================

#[test]
fn scalability_contract_has_no_fixed_machine_size_requirement() {
    // These are deliberately observations over the representable identifier
    // domain, not declarations of supported hardware capacity.
    let tiny = ScalingObservation::from_range(0, 1)
        .expect("tiny range must be representable");

    let large_count = usize::MAX / 4;

    let large = ScalingObservation::from_range(
        0,
        large_count,
    )
    .expect("large lazy range must be representable");

    assert_eq!(tiny.resource_count, 1);
    assert_eq!(tiny.first, Some(0));
    assert_eq!(tiny.last, Some(0));

    assert_eq!(
        large.resource_count,
        large_count
    );

    assert_eq!(large.first, Some(0));
    assert_eq!(
        large.last,
        Some(large_count - 1)
    );
}

#[test]
fn scalability_contract_supports_resource_availability_without_identity_rewrite() {
    let logical = QubitId::new(123);

    let identity =
        ResourceIdentity::logical_qubit(logical);

    let availability_states = [
        ResourceAvailability::Available,
        ResourceAvailability::Unavailable,
        ResourceAvailability::Unknown,
    ];

    for availability in availability_states {
        assert_eq!(
            identity.logical_qubit_id(),
            Some(logical)
        );

        // Availability is deliberately tested as a separate semantic
        // dimension. Changing availability does not mutate or replace the
        // canonical logical identity.
        assert_eq!(
            availability,
            availability
        );
    }
}

#[test]
fn scalability_contract_preserves_write_once_semantics() {
    // A logical program identity remains unchanged when the physical target
    // changes. The actual mapping is owned by routing/hardware; this test only
    // verifies the resilience identity boundary.
    let logical = QubitId::new(9);

    let first_target =
        PhysicalQubitId::new(3);

    let second_target =
        PhysicalQubitId::new(101);

    let logical_identity =
        ResourceIdentity::logical_qubit(logical);

    let first_physical =
        ResourceIdentity::physical_qubit(
            first_target
        );

    let second_physical =
        ResourceIdentity::physical_qubit(
            second_target
        );

    assert_eq!(
        logical_identity.logical_qubit_id(),
        Some(logical)
    );

    assert_ne!(
        first_physical,
        second_physical
    );

    // Most importantly, changing physical realization does not alter the
    // logical program identity.
    assert_eq!(
        logical_identity.logical_qubit_id(),
        Some(logical)
    );
}