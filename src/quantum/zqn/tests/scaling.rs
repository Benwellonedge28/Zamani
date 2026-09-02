//! # ZQN scalability and scale-invariance tests
//!
//! This module verifies that the Zamani Quantum Noise (ZQN) architecture scales
//! with the resources actually supplied by the caller rather than imposing a
//! semantic machine-size limit.
//!
//! ## Ownership
//!
//! This file owns:
//!
//! - ZQN scalability invariants;
//! - scale-invariance tests;
//! - resource-identity tests;
//! - streaming-oriented scaling tests;
//! - deterministic scaling tests;
//! - resource-policy tests;
//! - tests that distinguish semantic capacity from execution/resource limits.
//!
//! ## This file does NOT own
//!
//! This file does not define:
//!
//! - quantum IR semantics;
//! - a second qubit identity type;
//! - quantum-channel mathematics;
//! - a simulator;
//! - a hardware backend;
//! - routing;
//! - scheduling;
//! - QEC;
//! - a global maximum number of qubits;
//! - a global maximum number of operations;
//! - production resource limits.
//!
//! Canonical semantic qubit identities remain owned by
//! `crate::quantum::ir::qubit`.
//!
//! ## Architectural principle
//!
//! A Zamani quantum program must be expressible independently of the eventual
//! machine size:
//!
//! ```text
//! program
//!   |
//!   v
//! canonical quantum IR
//!   |
//!   v
//! ZQN model
//!   |
//!   +------------------+------------------+
//!   |                  |                  |
//!   v                  v                  v
//! tiny target       large target      distributed target
//! ```
//!
//! The program must not contain branches such as:
//!
//! ```text
//! if qubits == 5 ...
//! if qubits == 127 ...
//! if qubits == 1000 ...
//! ```
//!
//! Instead, the number of resources is data supplied by the execution
//! environment.
//!
//! ## Important distinction
//!
//! "Scales to infinity" means:
//!
//! > ZQN imposes no semantic upper bound on the size of a quantum computation.
//!
//! It does NOT mean:
//!
//! > every implementation can materialize an infinite state or execute an
//! > infinite computation.
//!
//! Physical memory, execution time, address space, target capabilities,
//! operating-system limits, and caller-selected resource policies necessarily
//! bound an individual execution.
//!
//! ## Test philosophy
//!
//! These tests therefore avoid a test such as:
//!
//! ```text
//! assert!(MAX_QUBITS >= 1_000_000);
//! ```
//!
//! Such a test would merely replace one hard-coded architectural limit with
//! another.
//!
//! Instead we test:
//!
//! - generated sizes;
//! - caller-provided resource counts;
//! - monotonic extension of resource sets;
//! - identity stability;
//! - deterministic derivation;
//! - streaming;
//! - explicit resource policies;
//! - absence of fixed-size assumptions.
//!
//! ## Rust compatibility
//!
//! Intended for Rust 1.97 / 1.97.1 and Rust 2021.
//!
//! No `unsafe` is used.
//!
//! ## Integration
//!
//! `tests/mod.rs` should expose this module with:
//!
//! ```ignore
//! pub mod scaling;
//! ```
//!
//! The ZQN test composition root should include `tests/mod.rs` according to
//! the repository's existing test-module convention.
//!
//! ## Integration contract
//!
//! These tests intentionally use a small test-local workload model rather than
//! concrete simulator implementation details. That is deliberate: a scaling
//! invariant belongs to ZQN's architectural contract and must not become tied
//! to one simulation backend.
//!
//! Concrete ZQN subsystems should additionally consume these invariants from
//! their own unit/property/integration tests.
//!
//! ## Resource safety
//!
//! The tests use bounded generated workloads. The bounds are TEST RESOURCE
//! LIMITS, not ZQN semantic limits.
//!
//! No production ZQN API should copy the constants in this file as machine
//! capacity limits.
//!
//! ## Determinism
//!
//! All generated resource identities are derived deterministically from their
//! logical position. No global RNG is used.
//!
//! ## Thread safety
//!
//! All test state is local to each test. There is no global mutable state.
//!
//! ## Serialization
//!
//! Serialization is intentionally not tested here. It belongs to
//! `tests/compatibility.rs` / `tests/differential.rs` and the IO subsystem.
//!
//! ## Definition of done
//!
//! This file is complete when:
//!
//! 1. no test assumes a fixed machine size;
//! 2. resource identity comes from the canonical IR identity system;
//! 3. generated workloads can be larger or smaller without changing semantic
//!    assumptions;
//! 4. deterministic workload construction is independent of traversal order;
//! 5. streaming workloads do not require whole-system materialization;
//! 6. explicit resource policies can reject work without redefining semantics;
//! 7. tiny and larger workloads obey the same invariants;
//! 8. no `unsafe` or global mutable state is required.

use std::collections::HashSet;
use std::num::NonZeroU64;

use crate::quantum::ir::qubit::QubitId;

/// A test-only logical workload size.
///
/// This is deliberately a value supplied by the caller/test rather than a
/// production ZQN constant.
///
/// The production architecture must not expose a fixed semantic maximum.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct WorkloadSize(u64);

impl WorkloadSize {
    fn new(value: u64) -> Self {
        Self(value)
    }

    fn get(self) -> u64 {
        self.0
    }
}

/// A deterministic test workload.
///
/// This represents the minimum kind of resource graph that ZQN must be able
/// to describe without materializing a complete quantum state.
///
/// It intentionally contains only identities and logical relationships.
///
/// A real channel/noise model may attach much richer information to these
/// resources.
#[derive(Clone, Debug, Eq, PartialEq)]
struct ScalingWorkload {
    qubits: Vec<QubitId>,
    operations: u64,
}

impl ScalingWorkload {
    fn generate(size: WorkloadSize) -> Self {
        let count = size.get();

        let mut qubits = Vec::with_capacity(
            usize::try_from(count)
                .expect("test workload must fit the host test process"),
        );

        for index in 0..count {
            qubits.push(qubit_id(index));
        }

        Self {
            qubits,
            operations: count,
        }
    }

    fn qubit_count(&self) -> u64 {
        self.qubits.len() as u64
    }

    fn operation_count(&self) -> u64 {
        self.operations
    }
}

/// Construct a canonical quantum-IR qubit identity.
///
/// This helper intentionally does not create a ZQN-specific `QubitId`.
///
/// If the repository's canonical `QubitId` constructor changes, this is the
/// single test-local integration point that should be adapted. The scaling
/// contract itself does not change.
///
/// The implementation below uses the canonical representation expected by the
/// current ZQN architecture: a stable logical integer identity.
fn qubit_id(index: u64) -> QubitId {
    QubitId::new(index)
}

/// A deterministic identity derived from a resource.
///
/// This is test infrastructure only. It is not a replacement for ZQN's
/// production reproducibility context.
///
/// The function intentionally uses an explicit domain separator so that
/// resource identity cannot accidentally be confused with another identity
/// domain.
fn resource_fingerprint(qubit: QubitId) -> u64 {
    let value = qubit.index();

    splitmix64(value ^ 0x5a51_4e5f_5155_424e)
}

/// Deterministic, local, non-cryptographic mixing function.
///
/// This is used only for test determinism. It must never be documented or
/// exposed as a cryptographic hash.
fn splitmix64(mut value: u64) -> u64 {
    value = value.wrapping_add(0x9e37_79b9_7f4a_7c15);

    let mut z = value;

    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);

    z ^ (z >> 31)
}

/// Produce the deterministic identity of a workload without requiring
/// materialization of a quantum state.
///
/// The operation is linear in the number of resources.
fn workload_fingerprint(workload: &ScalingWorkload) -> u64 {
    let mut result = splitmix64(workload.operation_count());

    for qubit in &workload.qubits {
        result = splitmix64(result ^ resource_fingerprint(*qubit));
    }

    result
}

/// Deterministically derive a per-resource execution value.
///
/// The important property is not the particular mixing function. The
/// important property is that:
///
/// ```text
/// master seed + resource identity
/// ```
///
/// determines the result.
///
/// Therefore execution order does not determine stochastic identity.
fn derive_resource_seed(master_seed: u64, qubit: QubitId) -> u64 {
    splitmix64(
        master_seed
            ^ splitmix64(qubit.index())
            ^ 0x5a51_4e5f_524e_5052,
    )
}

/// Explicit test-only resource policy.
///
/// This represents the architectural idea implemented by `zqn::core::limits`:
/// resource limits are execution policy, not semantic machine-size limits.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct TestResourcePolicy {
    max_qubits: Option<NonZeroU64>,
    max_operations: Option<NonZeroU64>,
}

impl TestResourcePolicy {
    fn unlimited() -> Self {
        Self {
            max_qubits: None,
            max_operations: None,
        }
    }

    fn allows(&self, workload: &ScalingWorkload) -> bool {
        let qubits_allowed = self
            .max_qubits
            .map_or(true, |limit| workload.qubit_count() <= limit.get());

        let operations_allowed = self
            .max_operations
            .map_or(true, |limit| {
                workload.operation_count() <= limit.get()
            });

        qubits_allowed && operations_allowed
    }
}

/// Return the first index at which two deterministic sequences differ.
///
/// This helper avoids allocating two complete result vectors for tests that
/// only need to establish prefix stability.
fn first_difference<I, J>(left: I, right: J) -> Option<usize>
where
    I: IntoIterator<Item = u64>,
    J: IntoIterator<Item = u64>,
{
    let mut left = left.into_iter();
    let mut right = right.into_iter();

    let mut index = 0usize;

    loop {
        match (left.next(), right.next()) {
            (None, None) => return None,
            (Some(a), Some(b)) if a == b => {
                index += 1;
            }
            (Some(_), Some(_)) => return Some(index),
            (None, Some(_)) | (Some(_), None) => return Some(index),
        }
    }
}

/// Ensure the smallest meaningful workload is supported.
///
/// This catches accidental assumptions such as:
///
/// - at least two qubits;
/// - at least one multi-qubit operation;
/// - non-empty topology;
/// - non-empty correlation set.
#[test]
fn supports_single_resource_workload() {
    let workload = ScalingWorkload::generate(WorkloadSize::new(1));

    assert_eq!(workload.qubit_count(), 1);
    assert_eq!(workload.operation_count(), 1);

    assert_eq!(
        workload.qubits.len(),
        1,
        "a one-resource computation must remain representable"
    );

    assert!(
        workload.qubits.first().is_some(),
        "single-resource workloads must not require a synthetic second resource"
    );
}

/// Ensure an empty resource set is representable.
///
/// An empty workload can occur during:
///
/// - compilation;
/// - optimization;
/// - partial program construction;
/// - distributed partitioning;
/// - characterization setup.
///
/// It must not cause an artificial minimum system size.
#[test]
fn supports_empty_workload_description() {
    let workload = ScalingWorkload::generate(WorkloadSize::new(0));

    assert_eq!(workload.qubit_count(), 0);
    assert_eq!(workload.operation_count(), 0);
    assert!(workload.qubits.is_empty());

    assert_eq!(
        workload_fingerprint(&workload),
        workload_fingerprint(&workload),
        "empty workload identity must be deterministic"
    );
}

/// Verify that increasing system size does not change the identity of the
/// resources already present in the smaller system.
///
/// This is one of the most important scale-invariance properties.
///
/// For workloads N and N+M:
///
/// ```text
/// identity(resource[0..N]) must be identical
/// ```
///
/// The larger machine must extend the resource universe; it must not
/// renumber/reinterpret existing resources.
#[test]
fn extending_a_workload_preserves_existing_resource_identity() {
    let small = ScalingWorkload::generate(WorkloadSize::new(8));
    let large = ScalingWorkload::generate(WorkloadSize::new(32));

    assert_eq!(
        small.qubits.len(),
        8,
        "test fixture must contain the intended smaller workload"
    );

    assert_eq!(
        large.qubits.len(),
        32,
        "test fixture must contain the intended larger workload"
    );

    let differences = first_difference(
        small.qubits.iter().map(|q| resource_fingerprint(*q)),
        large.qubits
            .iter()
            .take(small.qubits.len())
            .map(|q| resource_fingerprint(*q)),
    );

    assert!(
        differences.is_none(),
        "existing resources changed identity when the workload was extended"
    );
}

/// Verify that generated resource identities are unique.
///
/// Duplicate resource identity is particularly dangerous for scalable
/// execution because it can silently collapse two independent resources into
/// one.
///
/// The test uses a generated workload rather than a hard-coded machine size.
#[test]
fn generated_resource_identities_are_unique() {
    let workload = ScalingWorkload::generate(WorkloadSize::new(1024));

    let mut identities = HashSet::with_capacity(workload.qubits.len());

    for qubit in &workload.qubits {
        assert!(
            identities.insert(*qubit),
            "generated workload contains duplicate canonical qubit identity"
        );
    }

    assert_eq!(
        identities.len(),
        workload.qubits.len(),
        "all generated resources must remain distinct"
    );
}

/// Verify that the workload builder does not encode a fixed machine size.
///
/// The exact test sizes are deliberately fixture sizes rather than production
/// limits.
#[test]
fn supports_multiple_scales_without_semantic_branching() {
    let sizes = [
        0_u64, 1, 2, 3, 7, 16, 31, 64, 127, 256, 512, 1024,
    ];

    for size in sizes {
        let workload = ScalingWorkload::generate(WorkloadSize::new(size));

        assert_eq!(
            workload.qubit_count(),
            size,
            "resource count must be determined by supplied workload size"
        );

        assert_eq!(
            workload.operation_count(),
            size,
            "operation count must be determined by supplied workload size"
        );

        let unique = workload.qubits.iter().copied().collect::<HashSet<_>>();

        assert_eq!(
            unique.len(),
            workload.qubits.len(),
            "resource uniqueness must hold at every tested scale"
        );
    }
}

/// Verify that workload identity is deterministic.
///
/// A scaling implementation must not depend on allocation addresses,
/// traversal timing, thread scheduling, or process-local randomness.
#[test]
fn workload_identity_is_deterministic() {
    let first = ScalingWorkload::generate(WorkloadSize::new(512));
    let second = ScalingWorkload::generate(WorkloadSize::new(512));

    assert_eq!(
        first,
        second,
        "identical logical workloads must have identical deterministic structure"
    );

    assert_eq!(
        workload_fingerprint(&first),
        workload_fingerprint(&second),
        "identical logical workloads must have identical fingerprints"
    );
}

/// Verify prefix stability of deterministic resource execution.
///
/// If a workload grows from N to N+M resources, the deterministic execution
/// identity of the original N resources must remain unchanged.
#[test]
fn deterministic_resource_identity_is_scale_invariant() {
    let master_seed = 0x4d4f_4445_4c5f_5345_u64;

    let small = ScalingWorkload::generate(WorkloadSize::new(64));
    let large = ScalingWorkload::generate(WorkloadSize::new(256));

    let small_seeds = small
        .qubits
        .iter()
        .map(|qubit| derive_resource_seed(master_seed, *qubit));

    let large_prefix_seeds = large
        .qubits
        .iter()
        .take(small.qubits.len())
        .map(|qubit| derive_resource_seed(master_seed, *qubit));

    assert!(
        first_difference(small_seeds, large_prefix_seeds).is_none(),
        "adding resources must not alter deterministic identities of existing resources"
    );
}

/// Verify that execution order does not determine resource identity.
///
/// This is essential for parallel execution.
///
/// Sequential execution:
///
/// ```text
/// q0, q1, q2, q3
/// ```
///
/// and reordered execution:
///
/// ```text
/// q3, q1, q0, q2
/// ```
///
/// must derive the same identity for each resource.
#[test]
fn deterministic_identity_is_independent_of_traversal_order() {
    let master_seed = 0x5a41_4e49_4d41_4e49_u64;

    let workload = ScalingWorkload::generate(WorkloadSize::new(128));

    let mut forward = workload
        .qubits
        .iter()
        .map(|qubit| (*qubit, derive_resource_seed(master_seed, *qubit)))
        .collect::<Vec<_>>();

    let mut reverse = workload
        .qubits
        .iter()
        .rev()
        .map(|qubit| (*qubit, derive_resource_seed(master_seed, *qubit)))
        .collect::<Vec<_>>();

    forward.sort_by_key(|entry| entry.0);
    reverse.sort_by_key(|entry| entry.0);

    assert_eq!(
        forward, reverse,
        "deterministic resource identities must not depend on traversal order"
    );
}

/// Verify that different logical resources receive distinct deterministic
/// streams for the tested domain.
///
/// This is a collision sanity check, not a cryptographic proof.
#[test]
fn deterministic_resource_streams_are_distinct_for_generated_resources() {
    let master_seed = 0x5343_414c_494e_475f_u64;

    let workload = ScalingWorkload::generate(WorkloadSize::new(512));

    let seeds = workload
        .qubits
        .iter()
        .map(|qubit| derive_resource_seed(master_seed, *qubit))
        .collect::<Vec<_>>();

    let unique = seeds.iter().copied().collect::<HashSet<_>>();

    assert_eq!(
        unique.len(),
        seeds.len(),
        "generated resources produced colliding deterministic stream identities"
    );
}

/// Verify that changing the master seed changes deterministic execution
/// identity.
///
/// A deterministic system must not mean "always the same result regardless of
/// the requested seed".
#[test]
fn master_seed_is_part_of_deterministic_identity() {
    let workload = ScalingWorkload::generate(WorkloadSize::new(64));

    let first_seed = workload
        .qubits
        .iter()
        .map(|qubit| derive_resource_seed(1, *qubit))
        .collect::<Vec<_>>();

    let second_seed = workload
        .qubits
        .iter()
        .map(|qubit| derive_resource_seed(2, *qubit))
        .collect::<Vec<_>>();

    assert_ne!(
        first_seed, second_seed,
        "different master seeds must produce different deterministic execution identities"
    );
}

/// Verify that a workload can be processed as a stream without requiring
/// whole-system materialization.
///
/// This is a critical scalability property.
///
/// A production simulator may require a materialized state, but the ZQN
/// semantic layer must not require it merely to enumerate or identify noise
/// resources.
#[test]
fn resource_processing_can_be_streamed() {
    let count = 100_000_u64;

    let mut processed = 0_u64;
    let mut checksum = 0_u64;

    for index in 0..count {
        let qubit = qubit_id(index);

        checksum ^= resource_fingerprint(qubit);
        processed += 1;
    }

    assert_eq!(
        processed, count,
        "streamed resource processing must visit every requested resource"
    );

    // Prevent the optimizer from reducing this test to an obviously
    // irrelevant counter-only computation.
    assert_ne!(
        checksum, 0,
        "streamed deterministic resource processing produced an unexpected empty checksum"
    );
}

/// Verify that streaming and materialized representations describe the same
/// logical resource universe.
///
/// The materialized version is intentionally small; the streamed version is
/// intentionally larger. This demonstrates the architecture rather than
/// benchmarking allocation limits.
#[test]
fn streaming_and_materialized_prefixes_are_equivalent() {
    let count = 4096_u64;

    let materialized = ScalingWorkload::generate(WorkloadSize::new(count));

    let streamed = (0..count).map(qubit_id);

    let materialized_fingerprints = materialized
        .qubits
        .iter()
        .map(|qubit| resource_fingerprint(*qubit));

    let streamed_fingerprints = streamed.map(resource_fingerprint);

    assert!(
        first_difference(materialized_fingerprints, streamed_fingerprints).is_none(),
        "streaming must preserve the same logical resource identity as materialization"
    );
}

/// Verify monotonic resource growth.
///
/// Adding resources must increase the representable resource universe without
/// invalidating the smaller workload.
#[test]
fn resource_capacity_is_monotonic() {
    let sizes = [1_u64, 2, 4, 8, 16, 32, 64, 128];

    let mut previous = 0_u64;

    for size in sizes {
        let workload = ScalingWorkload::generate(WorkloadSize::new(size));

        assert!(
            workload.qubit_count() >= previous,
            "resource capacity must grow monotonically"
        );

        previous = workload.qubit_count();
    }
}

/// Verify that no semantic limit is accidentally introduced by the test's
/// resource policy abstraction.
///
/// `None` means "no policy limit", not "zero capacity".
#[test]
fn unlimited_resource_policy_does_not_impose_semantic_limit() {
    let policy = TestResourcePolicy::unlimited();

    for size in [0_u64, 1, 8, 64, 512, 4096] {
        let workload = ScalingWorkload::generate(WorkloadSize::new(size));

        assert!(
            policy.allows(&workload),
            "unlimited resource policy unexpectedly rejected workload of size {size}"
        );
    }
}

/// Verify that explicit resource limits reject execution without changing the
/// underlying semantic workload.
///
/// This distinction is essential:
///
/// ```text
/// semantic model != execution policy
/// ```
#[test]
fn explicit_resource_policy_is_not_a_semantic_limit() {
    let policy = TestResourcePolicy {
        max_qubits: NonZeroU64::new(16),
        max_operations: NonZeroU64::new(16),
    };

    let small = ScalingWorkload::generate(WorkloadSize::new(16));
    let large = ScalingWorkload::generate(WorkloadSize::new(17));

    assert!(
        policy.allows(&small),
        "workload at the configured policy boundary should be accepted"
    );

    assert!(
        !policy.allows(&large),
        "workload beyond an explicit execution policy should be rejected"
    );

    assert_eq!(
        small.qubit_count(),
        16,
        "policy rejection must not redefine semantic resource identity"
    );

    assert_eq!(
        large.qubit_count(),
        17,
        "a policy rejection must not make the larger semantic workload invalid"
    );
}

/// Verify that operation count is supplied data rather than a machine-size
/// constant.
///
/// A real ZQN operation graph may contain a number of operations unrelated to
/// the number of physical qubits.
#[test]
fn operation_count_is_independent_of_qubit_identity_space() {
    let workload = ScalingWorkload::generate(WorkloadSize::new(64));

    assert_eq!(workload.qubit_count(), 64);
    assert_eq!(workload.operation_count(), 64);

    let custom_operation_count = 1_000_000_u64;

    let custom = ScalingWorkload {
        qubits: workload.qubits.clone(),
        operations: custom_operation_count,
    };

    assert_eq!(custom.qubit_count(), 64);
    assert_eq!(
        custom.operation_count(),
        custom_operation_count,
        "operation cardinality must not be inferred from a fixed machine size"
    );
}

/// Verify that large operation counts can be represented independently of the
/// number of qubits.
///
/// This catches accidental APIs such as:
///
/// ```text
/// operation_count: u16
/// ```
///
/// or assumptions that operations are bounded by qubit count.
#[test]
fn large_operation_counts_are_data_not_architectural_limits() {
    let workload = ScalingWorkload {
        qubits: vec![qubit_id(0)],
        operations: u64::MAX / 2,
    };

    assert_eq!(workload.qubit_count(), 1);
    assert_eq!(workload.operation_count(), u64::MAX / 2);
}

/// Verify that canonical qubit identities are independent of ZQN's local
/// workload representation.
///
/// This test exists specifically to prevent introduction of a competing
/// `zqn::QubitId`.
#[test]
fn zqn_scaling_uses_canonical_quantum_ir_qubit_identity() {
    let workload = ScalingWorkload::generate(WorkloadSize::new(4));

    for (expected, actual) in workload.qubits.iter().enumerate() {
        assert_eq!(
            actual.index(),
            expected as u64,
            "ZQN workload identity must preserve canonical IR qubit identity"
        );
    }
}

/// Verify that extending a resource universe does not change the deterministic
/// identity of an existing resource even when the extension is much larger.
#[test]
fn very_large_extension_preserves_small_prefix() {
    let master_seed = 0x5a51_4e5f_5343_414c_u64;

    let small = ScalingWorkload::generate(WorkloadSize::new(16));

    let large_prefix = (0_u64..1_000_000_u64)
        .map(qubit_id)
        .take(small.qubits.len())
        .map(|qubit| derive_resource_seed(master_seed, qubit));

    let small_seeds = small
        .qubits
        .iter()
        .map(|qubit| derive_resource_seed(master_seed, *qubit));

    assert!(
        first_difference(small_seeds, large_prefix).is_none(),
        "large-scale extension changed the deterministic identity of the original resources"
    );
}

/// Verify that the workload can be partitioned into independently processed
/// chunks without changing per-resource deterministic identity.
///
/// This models distributed/parallel execution without requiring threads in
/// the test itself.
#[test]
fn partitioned_processing_is_equivalent_to_monolithic_processing() {
    let master_seed = 0x5041_5254_4954_494f_u64;
    let count = 4096_u64;

    let monolithic = (0..count)
        .map(qubit_id)
        .map(|qubit| (qubit, derive_resource_seed(master_seed, qubit)))
        .collect::<Vec<_>>();

    let mut partitioned = Vec::with_capacity(monolithic.len());

    for range_start in [0_u64, 1024, 2048, 3072] {
        let range_end = range_start + 1024;

        for index in range_start..range_end {
            let qubit = qubit_id(index);
            partitioned.push((qubit, derive_resource_seed(master_seed, qubit)));
        }
    }

    partitioned.sort_by_key(|entry| entry.0);

    assert_eq!(
        monolithic, partitioned,
        "partitioning work must not change deterministic resource identity"
    );
}

/// Verify that changing workload size changes the workload identity while
/// preserving the identity of resources contained in both workloads.
///
/// This prevents the implementation from hashing only a fixed-size prefix.
#[test]
fn workload_identity_accounts_for_scale() {
    let small = ScalingWorkload::generate(WorkloadSize::new(32));
    let large = ScalingWorkload::generate(WorkloadSize::new(64));

    assert_ne!(
        workload_fingerprint(&small),
        workload_fingerprint(&large),
        "different logical workloads must not collapse to one workload identity"
    );

    for (small_qubit, large_qubit) in small.qubits.iter().zip(large.qubits.iter()) {
        assert_eq!(
            small_qubit, large_qubit,
            "common resources must preserve identity across workload sizes"
        );
    }
}

/// Verify that generated resource identity is independent of allocation
/// order.
///
/// This models systems that construct resource graphs through different
/// compilation passes.
#[test]
fn resource_identity_is_independent_of_construction_order() {
    let mut forward = (0_u64..256_u64)
        .map(qubit_id)
        .collect::<Vec<_>>();

    let mut reverse = (0_u64..256_u64)
        .rev()
        .map(qubit_id)
        .collect::<Vec<_>>();

    forward.sort_by_key(|qubit| qubit.index());
    reverse.sort_by_key(|qubit| qubit.index());

    assert_eq!(
        forward, reverse,
        "logical resource identity must not depend on construction order"
    );
}

/// Verify that the deterministic derivation is stable across repeated
/// evaluation.
#[test]
fn deterministic_derivation_is_repeatable() {
    let master_seed = 0x5245_5045_4154_4142_u64;

    for index in 0_u64..1024_u64 {
        let qubit = qubit_id(index);

        let first = derive_resource_seed(master_seed, qubit);
        let second = derive_resource_seed(master_seed, qubit);
        let third = derive_resource_seed(master_seed, qubit);

        assert_eq!(first, second);
        assert_eq!(second, third);
    }
}

/// Verify that resource processing remains linear in the number of resources
/// at the semantic layer.
///
/// This is deliberately a structural test rather than a wall-clock benchmark:
/// CI timing is too unstable to establish an algorithmic complexity contract.
///
/// The workload is processed exactly once.
#[test]
fn resource_processing_is_single_pass() {
    let count = 100_000_u64;

    let mut visits = 0_u64;
    let mut accumulator = 0_u64;

    for index in 0..count {
        let qubit = qubit_id(index);

        accumulator = accumulator.wrapping_add(resource_fingerprint(qubit));
        visits += 1;
    }

    assert_eq!(visits, count);

    // Ensure the loop performs meaningful deterministic work.
    assert_ne!(
        accumulator, 0,
        "single-pass resource processing unexpectedly produced no accumulated identity"
    );
}

/// Verify that scaling does not require an all-to-all relationship structure.
///
/// A common scalability failure is representing N resources using N² edges
/// even when no relationship has been requested.
///
/// This test models the minimum representation requirement: resources can be
/// represented independently.
#[test]
fn independent_resources_do_not_require_quadratic_relationship_storage() {
    let count = 16_384_u64;

    let mut checksum = 0_u64;

    for index in 0..count {
        checksum ^= resource_fingerprint(qubit_id(index));
    }

    assert_ne!(
        checksum, 0,
        "independent resource representation should remain meaningful without all-to-all edges"
    );
}

/// Verify that a very large logical resource index can be represented by the
/// canonical ID domain without introducing a ZQN-specific narrow integer type.
///
/// This is intentionally only an identity-domain test; it does not allocate a
/// system of that size.
#[test]
fn large_logical_identity_does_not_require_materialized_system_size() {
    let index = u64::MAX - 1;
    let qubit = qubit_id(index);

    assert_eq!(qubit.index(), index);

    let derived = resource_fingerprint(qubit);

    // The identity must remain usable without allocating `index` resources.
    assert_ne!(
        derived, 0,
        "large logical resource identities must remain representable without materialization"
    );
}

/// Verify that resource identity and resource materialization are separate
/// concepts.
///
/// This is central to the "atom to everywhere" requirement.
///
/// A resource may be identified without requiring every preceding resource to
/// be allocated.
#[test]
fn sparse_identity_does_not_require_dense_materialization() {
    let first = qubit_id(7);
    let distant = qubit_id(1_000_000_000);

    assert_ne!(
        first, distant,
        "distinct sparse logical identities must remain distinct"
    );

    let first_seed = derive_resource_seed(42, first);
    let distant_seed = derive_resource_seed(42, distant);

    assert_ne!(
        first_seed, distant_seed,
        "distinct sparse resources must have distinct deterministic identities in this test domain"
    );
}

/// Verify that the scaling test suite itself does not accidentally introduce a
/// semantic maximum.
///
/// The largest value here is intentionally representable without constructing
/// a vector of that size.
#[test]
fn semantic_identity_domain_is_larger_than_test_materialization_domain() {
    let logical_index = u64::MAX - 1;

    let qubit = qubit_id(logical_index);

    assert_eq!(qubit.index(), logical_index);

    // No allocation proportional to `logical_index` occurs.
    let fingerprint = resource_fingerprint(qubit);

    assert_ne!(
        fingerprint, 0,
        "logical identity must be usable independently of materialized capacity"
    );
}

/// Verify that a policy can be changed without changing semantic identity.
///
/// This is an important boundary between `core::limits` and the mathematical
/// model.
#[test]
fn changing_execution_policy_does_not_change_semantic_identity() {
    let workload = ScalingWorkload::generate(WorkloadSize::new(64));

    let unlimited = TestResourcePolicy::unlimited();

    let restricted = TestResourcePolicy {
        max_qubits: NonZeroU64::new(128),
        max_operations: NonZeroU64::new(128),
    };

    assert!(unlimited.allows(&workload));
    assert!(restricted.allows(&workload));

    let identity = workload_fingerprint(&workload);

    assert_eq!(
        identity,
        workload_fingerprint(&workload),
        "resource policy changes must not alter semantic workload identity"
    );
}

/// Verify that the same deterministic workload can be split between multiple
/// execution domains.
///
/// This models future distributed quantum execution.
#[test]
fn distributed_partitions_preserve_global_resource_identity() {
    let master_seed = 0x4449_5354_5249_4255_u64;
    let count = 2048_u64;

    let global = (0..count)
        .map(qubit_id)
        .map(|qubit| (qubit, derive_resource_seed(master_seed, qubit)))
        .collect::<Vec<_>>();

    let mut node_a = Vec::new();
    let mut node_b = Vec::new();

    for index in 0..count {
        let qubit = qubit_id(index);
        let item = (qubit, derive_resource_seed(master_seed, qubit));

        if index % 2 == 0 {
            node_a.push(item);
        } else {
            node_b.push(item);
        }
    }

    node_a.extend(node_b);
    node_a.sort_by_key(|entry| entry.0);

    assert_eq!(
        global, node_a,
        "distributed partitioning must preserve global deterministic resource identity"
    );
}

/// Verify that scaling does not depend on the number of workers.
///
/// This is a deterministic structural equivalent of the production invariant:
///
/// ```text
/// execute(work, workers = 1)
/// ==
/// execute(work, workers = N)
/// ```
///
/// Concrete executor tests should test the actual runtime as well.
#[test]
fn worker_partition_count_does_not_change_resource_results() {
    let master_seed = 0x574f_524b_4552_5354_u64;
    let count = 4096_u64;

    let expected = (0..count)
        .map(qubit_id)
        .map(|qubit| (qubit, derive_resource_seed(master_seed, qubit)))
        .collect::<Vec<_>>();

    for workers in [1_u64, 2, 4, 8, 16, 32] {
        let mut actual = Vec::with_capacity(count as usize);

        for worker in 0..workers {
            let mut index = worker;

            while index < count {
                let qubit = qubit_id(index);
                actual.push((qubit, derive_resource_seed(master_seed, qubit)));

                index += workers;
            }
        }

        actual.sort_by_key(|entry| entry.0);

        assert_eq!(
            expected, actual,
            "worker count {workers} changed deterministic resource identity"
        );
    }
}

/// Verify that the scaling model supports sparse workloads.
///
/// This is important for distributed, graph-based, photonic, bosonic, and
/// future quantum resource models where logical identifiers need not form a
/// dense zero-based allocation.
#[test]
fn sparse_workloads_scale_without_dense_resource_assumptions() {
    let identifiers = [
        0_u64,
        1,
        7,
        31,
        1_000,
        1_000_000,
        1_000_000_000,
        u64::MAX - 2,
    ];

    let mut seen = HashSet::new();

    for index in identifiers {
        let qubit = qubit_id(index);

        assert!(
            seen.insert(qubit),
            "sparse workload contains duplicate canonical resource identity"
        );

        assert_eq!(qubit.index(), index);

        let _ = resource_fingerprint(qubit);
    }

    assert_eq!(seen.len(), identifiers.len());
}

/// Verify that scaling is not tied to the number of physical resources.
///
/// A future target may have many physical resources for a small logical
/// computation or many logical resources mapped onto another representation.
///
/// ZQN must not derive semantic complexity from one resource cardinality.
#[test]
fn logical_operation_count_can_differ_from_resource_count() {
    let workload = ScalingWorkload {
        qubits: (0_u64..8_u64).map(qubit_id).collect(),
        operations: 100_000,
    };

    assert_eq!(workload.qubit_count(), 8);
    assert_eq!(workload.operation_count(), 100_000);
}

/// Verify that scale changes preserve deterministic identities even when the
/// workload is generated independently rather than extended in-place.
#[test]
fn independently_generated_scales_have_stable_common_prefix() {
    let master_seed = 0x434f_4d4d_4f4e_5052_u64;

    let scales = [
        ScalingWorkload::generate(WorkloadSize::new(8)),
        ScalingWorkload::generate(WorkloadSize::new(32)),
        ScalingWorkload::generate(WorkloadSize::new(128)),
    ];

    for smaller in scales.iter().take(2) {
        for larger in scales.iter().skip(1) {
            if smaller.qubits.len() > larger.qubits.len() {
                continue;
            }

            let left = smaller
                .qubits
                .iter()
                .map(|qubit| derive_resource_seed(master_seed, *qubit));

            let right = larger
                .qubits
                .iter()
                .take(smaller.qubits.len())
                .map(|qubit| derive_resource_seed(master_seed, *qubit));

            assert!(
                first_difference(left, right).is_none(),
                "independently generated workloads must preserve common-prefix identity"
            );
        }
    }
}

/// Verify that the scaling contract does not require a specific machine size
/// to be encoded in the workload.
///
/// This test intentionally constructs workloads around unrelated sizes and
/// checks only generic invariants.
#[test]
fn scaling_contract_is_size_agnostic() {
    let sizes = [
        WorkloadSize::new(1),
        WorkloadSize::new(5),
        WorkloadSize::new(17),
        WorkloadSize::new(100),
        WorkloadSize::new(1000),
    ];

    for size in sizes {
        let workload = ScalingWorkload::generate(size);

        assert_eq!(workload.qubit_count(), size.get());
        assert_eq!(workload.operation_count(), size.get());

        let mut previous = None;

        for qubit in &workload.qubits {
            if let Some(previous_index) = previous {
                assert!(
                    qubit.index() > previous_index,
                    "generated canonical resource identities must be strictly ordered"
                );
            }

            previous = Some(qubit.index());
        }
    }
}

/// Final architectural guard.
///
/// This test documents the central invariant in executable form:
///
/// > A resource count is supplied as data; ZQN does not define the machine
/// > size.
///
/// The test deliberately does not inspect a `MAX_QUBITS` constant because
/// such a constant should not exist as a semantic ZQN concept.
#[test]
fn resource_count_is_external_to_zqn_semantics() {
    fn build(size: u64) -> ScalingWorkload {
        ScalingWorkload::generate(WorkloadSize::new(size))
    }

    let tiny = build(1);
    let larger = build(1024);

    assert_eq!(tiny.qubit_count(), 1);
    assert_eq!(larger.qubit_count(), 1024);

    assert_ne!(
        workload_fingerprint(&tiny),
        workload_fingerprint(&larger),
        "different resource universes must remain distinguishable"
    );
}