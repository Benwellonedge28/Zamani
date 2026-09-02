//! Zamani Quantum Noise (ZQN) — Determinism Tests
//!
//! # Ownership
//!
//! This module owns integration-level tests for the ZQN determinism contract.
//!
//! The tests verify that deterministic stochastic work can be addressed by
//! stable semantic coordinates rather than by:
//!
//! - thread identity;
//! - memory address;
//! - allocation order;
//! - process-local state;
//! - iteration order;
//! - hidden global RNG state;
//! - wall-clock time;
//! - operating-system scheduling.
//!
//! The tests intentionally use a small, deterministic reference derivation
//! function implemented entirely in this file. This gives the test suite a
//! stable oracle for the architectural contract without coupling the tests to
//! one particular RNG implementation.
//!
//! # Non-ownership
//!
//! This module does NOT own:
//!
//! - ZQN random-number generation;
//! - probability semantics;
//! - quantum-channel mathematics;
//! - fault semantics;
//! - simulation algorithms;
//! - hardware execution;
//! - quantum IR semantics;
//! - qubit identity;
//! - scheduling;
//! - routing;
//! - QEC;
//! - benchmarking;
//! - cryptography;
//! - production RNG implementation.
//!
//! Production stochastic implementations must consume the determinism contract
//! established by ZQN core/context and provenance rather than copying this test
//! helper as application code.
//!
//! # Architectural contract
//!
//! Deterministic ZQN execution is conceptually addressed by:
//!
//! ```text
//! root seed
//!     +
//! execution scope
//!     +
//! operation scope
//!     +
//! resource scope
//!     +
//! sample index
//!     +
//! domain
//!     ↓
//! deterministic random stream
//! ```
//!
//! A deterministic consumer MUST therefore derive stochastic work from stable
//! semantic coordinates.
//!
//! It MUST NOT derive stochastic work from:
//!
//! ```text
//! thread_id
//! memory_address
//! pointer_value
//! wall_clock
//! mutex acquisition order
//! hash-map iteration order
//! allocation order
//! ```
//!
//! # Canonical quantum-resource identity
//!
//! These tests deliberately do not create a ZQN-specific qubit identifier.
//!
//! When an actual quantum resource is required by future ZQN integration tests,
//! the canonical types remain:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! This file does not duplicate that identity system.
//!
//! # Write once, scale everywhere
//!
//! No test establishes a semantic maximum for:
//!
//! - qubits;
//! - operations;
//! - resources;
//! - shots;
//! - execution nodes;
//! - channels;
//! - faults;
//! - circuit depth.
//!
//! Scaling tests select finite workloads because every concrete test process
//! has finite CPU time and memory. The selected workload is a test budget, not
//! an architectural limit.
//!
//! # Determinism definition
//!
//! For a deterministic root seed `S` and semantic coordinate `C`, define:
//!
//! ```text
//! D(S, C) -> R
//! ```
//!
//! The determinism contract requires:
//!
//! ```text
//! D(S, C) == D(S, C)
//! ```
//!
//! for every repeated evaluation of the same semantic coordinate.
//!
//! It also requires:
//!
//! ```text
//! D(S, C_i)
//! ```
//!
//! to be independent of the order in which `C_i` is evaluated.
//!
//! Therefore:
//!
//! ```text
//! evaluate(A); evaluate(B)
//! ```
//!
//! must produce the same values as:
//!
//! ```text
//! evaluate(B); evaluate(A)
//! ```
//!
//! This is the foundation for sequential/parallel equivalence.
//!
//! # Domain separation
//!
//! Different semantic domains must not accidentally share the same stream.
//!
//! Therefore:
//!
//! ```text
//! D(S, domain_a, C) != D(S, domain_b, C)
//! ```
//!
//! for distinct domains, except for the negligible possibility of an actual
//! output collision in a finite output space.
//!
//! The test suite checks exact inequality for the chosen deterministic
//! reference output space.
//!
//! # Seed sensitivity
//!
//! Changing the root seed must change the deterministic stream for the test
//! coordinates.
//!
//! This prevents implementations from accidentally ignoring caller-supplied
//! seed material.
//!
//! # Thread independence
//!
//! The tests deliberately compare sequential and parallel-style evaluation.
//!
//! They do not make thread scheduling part of the semantic result.
//!
//! No test depends on:
//!
//! - a particular number of worker threads;
//! - a particular scheduler;
//! - a particular CPU topology.
//!
//! # Reordering
//!
//! Stable semantic coordinates must make evaluation order irrelevant.
//!
//! A deterministic implementation should be safe to partition into batches,
//! process batches in different orders, and merge results without changing the
//! result associated with each coordinate.
//!
//! # Integration with ZQN context
//!
//! `core::context` already establishes the architectural rule that deterministic
//! mode carries explicit caller-supplied seed material and that stochastic
//! consumers derive work from stable execution coordinates.
//!
//! These tests therefore validate the contract rather than introducing another
//! competing context type.
//!
//! # Integration with provenance
//!
//! `core::provenance` records deterministic seed information as provenance.
//! Deterministic tests must therefore ensure that seed material is actually
//! semantically relevant rather than decorative metadata.
//!
//! # Integration with simulation
//!
//! The simulation subsystem must eventually replace the reference helper used
//! here with its production stochastic derivation mechanism.
//!
//! The externally observable properties tested here must remain unchanged:
//!
//! - same seed + same coordinates => same result;
//! - same seed + different coordinates => independently addressable result;
//! - evaluation order does not matter;
//! - execution partitioning does not matter;
//! - thread identity does not matter.
//!
//! # Integration with fault generation
//!
//! Fault generation must derive random work from stable coordinates such as:
//!
//! ```text
//! execution
//! operation
//! resource
//! sample
//! ```
//!
//! rather than from mutable sequential RNG state whose output changes when
//! work is parallelized or reordered.
//!
//! # Integration with characterization
//!
//! Characterization experiments must use deterministic seed material when
//! reproducibility is requested.
//!
//! Experiment IDs and sample coordinates should be semantic inputs rather than
//! incidental iteration positions.
//!
//! # Integration with QEC
//!
//! QEC may consume deterministic ZQN fault streams. QEC must not introduce a
//! second incompatible randomization contract.
//!
//! # Integration with routing/scheduling
//!
//! If routing or scheduling use randomized algorithms, their own determinism
//! contract may be tested elsewhere. ZQN determinism must remain independent of
//! routing or scheduler execution order.
//!
//! # Integration with hardware
//!
//! Deterministic software generation does not imply deterministic physical QPU
//! behavior.
//!
//! Hardware noise is inherently observational and may vary physically even when
//! the requested deterministic simulation/model seed is identical.
//!
//! Therefore these tests establish deterministic *software/model generation*,
//! not a claim that a real quantum processor will produce identical physical
//! measurements.
//!
//! # Integration with serialization
//!
//! If a deterministic ZQN artifact is serialized and restored, its semantic
//! deterministic inputs must remain unchanged.
//!
//! Serialization must not silently modify:
//!
//! - seed;
//! - execution identity;
//! - operation identity;
//! - resource identity;
//! - sample index;
//! - domain identity.
//!
//! # Security
//!
//! The test helper uses only bounded, fixed-width arithmetic.
//!
//! It performs:
//!
//! - no allocation based on attacker-controlled sizes;
//! - no recursion;
//! - no unsafe operations;
//! - no global mutable state;
//! - no filesystem access;
//! - no network access;
//! - no cryptographic claims.
//!
//! # Rust compatibility
//!
//! Supported:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code;
//! - standard library only.
//!
//! # File-completion contract
//!
//! This file is complete when:
//!
//! 1. deterministic coordinate derivation is tested;
//! 2. repeated evaluation is stable;
//! 3. evaluation order cannot affect results;
//! 4. partitioning cannot affect results;
//! 5. seed changes affect results;
//! 6. domain separation is tested;
//! 7. coordinate changes affect addressed streams;
//! 8. large finite coordinate spaces can be tested without a semantic maximum;
//! 9. no hidden global RNG is used;
//! 10. no thread identity participates in the reference derivation;
//! 11. no vendor-specific behavior is encoded;
//! 12. no ZQN-specific qubit identity is introduced;
//! 13. the test suite remains independent of future ZQN implementation details;
//! 14. the test suite is safe under Rust 1.97/1.97.1;
//! 15. the file requires no later API modification merely because downstream
//!     ZQN modules are implemented.
//!
//! # Test organization
//!
//! ```text
//! determinism.rs
//! ├── primitive mixing helpers
//! ├── semantic coordinate
//! ├── deterministic reference derivation
//! ├── repeated evaluation
//! ├── seed sensitivity
//! ├── coordinate sensitivity
//! ├── domain separation
//! ├── order independence
//! ├── partition independence
//! ├── parallel-style independence
//! ├── large finite scaling
//! └── pathological coordinate values
//! ```
//!
//! # Important implementation note
//!
//! This file intentionally does NOT import a future concrete ZQN RNG type.
//!
//! Doing so would make a foundational test depend on a downstream implementation
//! contract and would force later re-editing.
//!
//! Instead, this file defines the mathematical determinism contract in a
//! dependency-free form. Concrete ZQN stochastic consumers should have their
//! own integration tests asserting that their production implementation agrees
//! with these properties.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::collections::BTreeMap;

// =============================================================================
// Reference constants
// =============================================================================

/// First fixed mixing constant.
///
/// These constants are test-oracle constants only. They are not ZQN semantic
/// limits and MUST NOT be used as production resource limits.
const MIX_A: u64 = 0x9E37_79B9_7F4A_7C15;

/// Second fixed mixing constant.
const MIX_B: u64 = 0xBF58_476D_1CE4_E5B9;

/// Third fixed mixing constant.
const MIX_C: u64 = 0x94D0_49BB_1331_11EB;

/// Fourth fixed mixing constant.
const MIX_D: u64 = 0xD6E8_FEB8_6659_FD93;

/// Finite workload used by the scaling test.
///
/// This is a test budget, not a ZQN system-size limit.
const SCALING_COORDINATES: u64 = 100_000;

// =============================================================================
// Semantic coordinate
// =============================================================================

/// Stable semantic coordinates used by the determinism oracle.
///
/// This structure deliberately contains no pointer, thread, wall-clock, or
/// allocation-derived value.
///
/// In production, equivalent coordinates may be represented by the ZQN
/// execution context, provenance system, operation identity, resource identity,
/// and sampling subsystem.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Coordinate {
    /// Semantic execution scope.
    execution: u64,

    /// Semantic operation scope.
    operation: u64,

    /// Semantic resource scope.
    resource: u64,

    /// Independent sample/shot index.
    sample: u64,

    /// Domain separator.
    domain: u64,
}

impl Coordinate {
    /// Creates a semantic coordinate.
    const fn new(
        execution: u64,
        operation: u64,
        resource: u64,
        sample: u64,
        domain: u64,
    ) -> Self {
        Self {
            execution,
            operation,
            resource,
            sample,
            domain,
        }
    }
}

// =============================================================================
// Deterministic reference derivation
// =============================================================================

/// Performs a fixed-width avalanche mix.
///
/// The function uses wrapping arithmetic deliberately. Every operation is
/// defined for every `u64` value, including the boundary values.
///
/// No overflow is treated as an error because wrapping arithmetic is part of
/// this test oracle's exact mathematical definition.
const fn mix(mut value: u64) -> u64 {
    value ^= value >> 30;
    value = value.wrapping_mul(MIX_A);

    value ^= value >> 27;
    value = value.wrapping_mul(MIX_B);

    value ^= value >> 31;

    value
}

/// Combines one semantic coordinate component into an accumulator.
///
/// Each component is mixed independently before entering the accumulator.
/// This prevents simple concatenation assumptions from becoming part of the
/// contract.
const fn absorb(accumulator: u64, value: u64, tag: u64) -> u64 {
    let tagged = value
        .wrapping_add(tag.wrapping_mul(MIX_C))
        .rotate_left((tag as u32) & 63);

    mix(
        accumulator
            ^ tagged
            ^ MIX_D.rotate_left((tag as u32) & 63),
    )
}

/// Derives one deterministic value from a root seed and semantic coordinates.
///
/// This is the test oracle.
///
/// Production ZQN code may use a different RNG/derivation implementation. It
/// must nevertheless satisfy the same externally observable properties tested
/// below.
///
/// The derivation is a pure function:
///
/// ```text
/// (seed, coordinate) -> value
/// ```
///
/// There is no:
///
/// - global state;
/// - mutable static;
/// - thread-local RNG;
/// - clock;
/// - pointer;
/// - allocation address;
/// - iteration counter.
///
/// Therefore evaluation order cannot influence the result.
const fn reference_derive(seed: u64, coordinate: Coordinate) -> u64 {
    let mut state = mix(seed ^ MIX_A);

    state = absorb(state, coordinate.execution, 1);
    state = absorb(state, coordinate.operation, 2);
    state = absorb(state, coordinate.resource, 3);
    state = absorb(state, coordinate.sample, 4);
    state = absorb(state, coordinate.domain, 5);

    mix(state ^ MIX_B)
}

/// Produces a deterministic stream of values for a coordinate list.
fn evaluate(seed: u64, coordinates: &[Coordinate]) -> Vec<u64> {
    coordinates
        .iter()
        .copied()
        .map(|coordinate| reference_derive(seed, coordinate))
        .collect()
}

// =============================================================================
// Coordinate fixtures
// =============================================================================

/// Returns a small representative coordinate set.
///
/// The fixture intentionally covers:
///
/// - repeated execution scopes;
/// - repeated operations;
/// - multiple resources;
/// - multiple samples;
/// - multiple domains.
fn representative_coordinates() -> Vec<Coordinate> {
    vec![
        Coordinate::new(0, 0, 0, 0, 0),
        Coordinate::new(0, 0, 1, 0, 0),
        Coordinate::new(0, 1, 0, 0, 0),
        Coordinate::new(1, 0, 0, 0, 0),
        Coordinate::new(0, 0, 0, 1, 0),
        Coordinate::new(0, 0, 0, 0, 1),
        Coordinate::new(7, 19, 31, 47, 59),
        Coordinate::new(
            u64::MAX,
            u64::MAX - 1,
            u64::MAX - 2,
            u64::MAX - 3,
            u64::MAX - 4,
        ),
    ]
}

// =============================================================================
// Tests: repeated evaluation
// =============================================================================

#[test]
fn same_seed_and_same_coordinate_are_identical() {
    let seed = 0x0123_4567_89AB_CDEF_u64;
    let coordinate = Coordinate::new(17, 23, 31, 41, 47);

    let first = reference_derive(seed, coordinate);
    let second = reference_derive(seed, coordinate);

    assert_eq!(
        first, second,
        "deterministic derivation changed for identical semantic inputs"
    );
}

#[test]
fn repeated_evaluation_of_a_stream_is_identical() {
    let seed = 0xA5A5_5A5A_1234_5678_u64;
    let coordinates = representative_coordinates();

    let first = evaluate(seed, &coordinates);
    let second = evaluate(seed, &coordinates);

    assert_eq!(
        first, second,
        "repeated deterministic evaluation produced different results"
    );
}

// =============================================================================
// Tests: seed sensitivity
// =============================================================================

#[test]
fn changing_seed_changes_the_deterministic_stream() {
    let coordinates = representative_coordinates();

    let first = evaluate(0x1111_2222_3333_4444, &coordinates);
    let second = evaluate(0x5555_6666_7777_8888, &coordinates);

    assert_ne!(
        first, second,
        "caller-supplied seed appears to have no semantic effect"
    );
}

#[test]
fn adjacent_seeds_are_not_treated_as_the_same_seed() {
    let coordinate = Coordinate::new(3, 5, 7, 11, 13);

    let first = reference_derive(0, coordinate);
    let second = reference_derive(1, coordinate);

    assert_ne!(
        first, second,
        "adjacent deterministic seeds unexpectedly produced identical output"
    );
}

// =============================================================================
// Tests: coordinate sensitivity
// =============================================================================

#[test]
fn changing_execution_scope_changes_addressed_stream() {
    let seed = 42;

    let first = reference_derive(seed, Coordinate::new(1, 2, 3, 4, 5));
    let second = reference_derive(seed, Coordinate::new(2, 2, 3, 4, 5));

    assert_ne!(first, second);
}

#[test]
fn changing_operation_scope_changes_addressed_stream() {
    let seed = 42;

    let first = reference_derive(seed, Coordinate::new(1, 2, 3, 4, 5));
    let second = reference_derive(seed, Coordinate::new(1, 3, 3, 4, 5));

    assert_ne!(first, second);
}

#[test]
fn changing_resource_scope_changes_addressed_stream() {
    let seed = 42;

    let first = reference_derive(seed, Coordinate::new(1, 2, 3, 4, 5));
    let second = reference_derive(seed, Coordinate::new(1, 2, 4, 4, 5));

    assert_ne!(first, second);
}

#[test]
fn changing_sample_index_changes_addressed_stream() {
    let seed = 42;

    let first = reference_derive(seed, Coordinate::new(1, 2, 3, 4, 5));
    let second = reference_derive(seed, Coordinate::new(1, 2, 3, 5, 5));

    assert_ne!(first, second);
}

// =============================================================================
// Tests: domain separation
// =============================================================================

#[test]
fn different_domains_are_independently_addressable() {
    let seed = 42;

    let first = reference_derive(seed, Coordinate::new(1, 2, 3, 4, 0));
    let second = reference_derive(seed, Coordinate::new(1, 2, 3, 4, 1));

    assert_ne!(
        first, second,
        "different stochastic domains unexpectedly shared an identical address"
    );
}

#[test]
fn domain_is_part_of_semantic_identity() {
    let seed = 0xDEAD_BEEF;

    let base = Coordinate::new(7, 11, 13, 17, 19);
    let changed_domain = Coordinate::new(7, 11, 13, 17, 23);

    assert_ne!(
        reference_derive(seed, base),
        reference_derive(seed, changed_domain)
    );
}

// =============================================================================
// Tests: order independence
// =============================================================================

#[test]
fn evaluation_order_does_not_change_coordinate_results() {
    let seed = 0x1234_5678_9ABC_DEF0_u64;

    let coordinates = representative_coordinates();

    let forward = evaluate(seed, &coordinates);

    let mut reversed_coordinates = coordinates.clone();
    reversed_coordinates.reverse();

    let reversed = evaluate(seed, &reversed_coordinates);

    let forward_map: BTreeMap<Coordinate, u64> = coordinates
        .iter()
        .copied()
        .zip(forward)
        .collect();

    let reversed_map: BTreeMap<Coordinate, u64> = reversed_coordinates
        .iter()
        .copied()
        .zip(reversed)
        .collect();

    assert_eq!(
        forward_map, reversed_map,
        "evaluation order changed deterministic coordinate results"
    );
}

#[test]
fn duplicated_coordinates_remain_identical_without_hidden_stream_state() {
    let seed = 0xCAFE_BABE;

    let coordinate = Coordinate::new(9, 8, 7, 6, 5);

    let values = evaluate(
        seed,
        &[coordinate, coordinate, coordinate, coordinate],
    );

    assert!(
        values.windows(2).all(|window| window[0] == window[1]),
        "repeated coordinate evaluation appears to consume hidden mutable RNG state"
    );
}

// =============================================================================
// Tests: partition independence
// =============================================================================

#[test]
fn partitioning_work_does_not_change_results() {
    let seed = 0x1357_9BDF_2468_ACE0_u64;

    let coordinates: Vec<Coordinate> = (0_u64..128)
        .map(|index| Coordinate::new(1, index / 8, index % 8, index, 0))
        .collect();

    let whole = evaluate(seed, &coordinates);

    let mut partitioned = Vec::with_capacity(coordinates.len());

    for chunk in coordinates.chunks(7) {
        partitioned.extend(evaluate(seed, chunk));
    }

    assert_eq!(
        whole, partitioned,
        "partitioning deterministic work changed its results"
    );
}

#[test]
fn arbitrary_partition_sizes_do_not_change_results() {
    let seed = 0xCAF0_BABE_DEAD_BEEF_u64;

    let coordinates: Vec<Coordinate> = (0_u64..257)
        .map(|index| Coordinate::new(3, index / 17, index % 17, index, 2))
        .collect();

    let expected = evaluate(seed, &coordinates);

    for partition_size in 1..=31 {
        let mut actual = Vec::with_capacity(coordinates.len());

        for chunk in coordinates.chunks(partition_size) {
            actual.extend(evaluate(seed, chunk));
        }

        assert_eq!(
            expected, actual,
            "partition size {partition_size} changed deterministic results"
        );
    }
}

// =============================================================================
// Tests: parallel-style independence
// =============================================================================

#[test]
fn simulated_parallel_worker_assignment_does_not_change_results() {
    let seed = 0xFACE_CAFE_DEAD_BEEF_u64;

    let coordinates: Vec<Coordinate> = (0_u64..512)
        .map(|index| Coordinate::new(1, index / 16, index % 16, index, 3))
        .collect();

    let sequential = evaluate(seed, &coordinates);

    // Simulate arbitrary worker assignment without creating actual threads.
    //
    // This deliberately avoids making test semantics depend on the host thread
    // scheduler. A real implementation must provide the same result when the
    // work is genuinely parallelized.
    let mut worker_results: Vec<(usize, Coordinate, u64)> = coordinates
        .iter()
        .copied()
        .enumerate()
        .map(|(index, coordinate)| {
            let worker = (index.wrapping_mul(37)) % 11;
            (worker, coordinate, reference_derive(seed, coordinate))
        })
        .collect();

    worker_results.sort_by_key(|entry| (entry.0, entry.1));

    let mut reconstructed = BTreeMap::new();

    for (_, coordinate, value) in worker_results {
        reconstructed.insert(coordinate, value);
    }

    let expected: BTreeMap<Coordinate, u64> = coordinates
        .iter()
        .copied()
        .zip(sequential)
        .collect();

    assert_eq!(
        expected, reconstructed,
        "worker assignment changed deterministic coordinate results"
    );
}

// =============================================================================
// Tests: scalability
// =============================================================================

#[test]
fn deterministic_addressing_scales_with_generated_coordinate_count() {
    let seed = 0x0123_4567_89AB_CDEF_u64;

    let mut coordinates = Vec::with_capacity(SCALING_COORDINATES as usize);

    for index in 0..SCALING_COORDINATES {
        coordinates.push(Coordinate::new(
            index / 10_000,
            index / 1_000,
            index % 1_000,
            index,
            index % 17,
        ));
    }

    let first = evaluate(seed, &coordinates);
    let second = evaluate(seed, &coordinates);

    assert_eq!(
        first, second,
        "deterministic results changed at scaling-test workload"
    );

    assert_eq!(
        first.len(),
        SCALING_COORDINATES as usize,
        "scaling test did not evaluate the complete generated workload"
    );
}

#[test]
fn large_coordinate_values_are_supported_without_size_assumptions() {
    let seed = u64::MAX;

    let coordinates = [
        Coordinate::new(
            u64::MAX,
            u64::MAX,
            u64::MAX,
            u64::MAX,
            u64::MAX,
        ),
        Coordinate::new(
            u64::MAX,
            0,
            u64::MAX,
            0,
            u64::MAX,
        ),
        Coordinate::new(
            0,
            u64::MAX,
            0,
            u64::MAX,
            0,
        ),
    ];

    let first = evaluate(seed, &coordinates);
    let second = evaluate(seed, &coordinates);

    assert_eq!(first, second);
}

// =============================================================================
// Tests: boundary coordinates
// =============================================================================

#[test]
fn zero_coordinate_is_valid_and_deterministic() {
    let coordinate = Coordinate::new(0, 0, 0, 0, 0);

    assert_eq!(
        reference_derive(0, coordinate),
        reference_derive(0, coordinate)
    );
}

#[test]
fn maximum_coordinate_is_valid_and_deterministic() {
    let coordinate = Coordinate::new(
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
        u64::MAX,
    );

    assert_eq!(
        reference_derive(u64::MAX, coordinate),
        reference_derive(u64::MAX, coordinate)
    );
}

#[test]
fn neighbouring_coordinates_are_addressable_independently() {
    let seed = 0x55AA_55AA_33CC_33CC_u64;

    let first = Coordinate::new(100, 200, 300, 400, 500);
    let second = Coordinate::new(100, 200, 300, 401, 500);

    assert_ne!(
        reference_derive(seed, first),
        reference_derive(seed, second)
    );
}

// =============================================================================
// Tests: no hidden sequential state
// =============================================================================

#[test]
fn evaluating_unrelated_coordinates_between_repeated_calls_does_not_change_result() {
    let seed = 0xA1B2_C3D4_E5F6_0718_u64;

    let target = Coordinate::new(10, 20, 30, 40, 50);

    let first = reference_derive(seed, target);

    let unrelated: Vec<Coordinate> = (0_u64..1_000)
        .map(|index| Coordinate::new(index, index + 1, index + 2, index + 3, index + 4))
        .collect();

    let _ = evaluate(seed, &unrelated);

    let second = reference_derive(seed, target);

    assert_eq!(
        first, second,
        "unrelated deterministic work changed an existing coordinate's result"
    );
}

#[test]
fn evaluating_same_coordinate_after_reordering_unrelated_work_is_stable() {
    let seed = 0x1020_3040_5060_7080_u64;

    let target = Coordinate::new(17, 19, 23, 29, 31);

    let first = reference_derive(seed, target);

    let mut unrelated = representative_coordinates();
    unrelated.reverse();

    let _ = evaluate(seed, &unrelated);

    let second = reference_derive(seed, target);

    assert_eq!(
        first, second,
        "unrelated evaluation order contaminated deterministic state"
    );
}

// =============================================================================
// Tests: reproducibility contract
// =============================================================================

#[test]
fn reproducibility_requires_same_seed_and_same_coordinates() {
    let seed = 0x0F1E_2D3C_4B5A_6978_u64;
    let coordinates = representative_coordinates();

    let run_a = evaluate(seed, &coordinates);
    let run_b = evaluate(seed, &coordinates);

    assert_eq!(
        run_a, run_b,
        "reproducibility contract failed for identical deterministic inputs"
    );
}

#[test]
fn reproducibility_is_not_promised_across_different_seed_material() {
    let coordinates = representative_coordinates();

    let run_a = evaluate(1, &coordinates);
    let run_b = evaluate(2, &coordinates);

    assert_ne!(
        run_a, run_b,
        "different deterministic seed material unexpectedly produced the same stream"
    );
}

// =============================================================================
// Tests: composition of semantic scopes
// =============================================================================

#[test]
fn operation_and_resource_scopes_are_composable() {
    let seed = 0xDEAD_C0DE_BAAD_F00D_u64;

    let a = Coordinate::new(1, 10, 100, 0, 1);
    let b = Coordinate::new(1, 10, 101, 0, 1);
    let c = Coordinate::new(1, 11, 100, 0, 1);
    let d = Coordinate::new(2, 10, 100, 0, 1);

    let values = [
        reference_derive(seed, a),
        reference_derive(seed, b),
        reference_derive(seed, c),
        reference_derive(seed, d),
    ];

    assert_ne!(values[0], values[1]);
    assert_ne!(values[0], values[2]);
    assert_ne!(values[0], values[3]);
}

#[test]
fn sample_scope_is_independent_of_operation_scope() {
    let seed = 0x1234_0000_5678_0000_u64;

    let a = Coordinate::new(1, 2, 3, 0, 4);
    let b = Coordinate::new(1, 2, 3, 1, 4);
    let c = Coordinate::new(1, 3, 3, 0, 4);

    let ab = reference_derive(seed, a);
    let bb = reference_derive(seed, b);
    let cb = reference_derive(seed, c);

    assert_ne!(ab, bb);
    assert_ne!(ab, cb);
}

// =============================================================================
// Tests: deterministic map identity
// =============================================================================

#[test]
fn coordinate_to_result_mapping_is_stable() {
    let seed = 0xBADC_0FFE_EE11_2233_u64;

    let coordinates = representative_coordinates();

    let map_a: BTreeMap<Coordinate, u64> = coordinates
        .iter()
        .copied()
        .map(|coordinate| {
            (coordinate, reference_derive(seed, coordinate))
        })
        .collect();

    let mut reversed = coordinates.clone();
    reversed.reverse();

    let map_b: BTreeMap<Coordinate, u64> = reversed
        .iter()
        .copied()
        .map(|coordinate| {
            (coordinate, reference_derive(seed, coordinate))
        })
        .collect();

    assert_eq!(map_a, map_b);
}

// =============================================================================
// Compile-time guarantees
// =============================================================================

#[test]
fn coordinate_is_copy_and_value_based() {
    let original = Coordinate::new(1, 2, 3, 4, 5);
    let copied = original;

    assert_eq!(original, copied);
}

#[test]
fn deterministic_reference_is_a_pure_value_function() {
    let seed = 987_654_321_u64;
    let coordinate = Coordinate::new(123, 456, 789, 1_234, 5_678);

    let values = [
        reference_derive(seed, coordinate),
        reference_derive(seed, coordinate),
        reference_derive(seed, coordinate),
        reference_derive(seed, coordinate),
    ];

    assert!(values.windows(2).all(|window| window[0] == window[1]));
}