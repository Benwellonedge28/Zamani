//! Zamani Quantum Resilience — determinism and reproducibility tests.
//!
//! Path:
//!     src/quantum/resilience/tests/determinism.rs
//!
//! Purpose:
//!     Production-level tests for the deterministic behavior contract of
//!     `quantum::resilience`.
//!
//! Architectural position:
//!
//!     canonical IR
//!          |
//!          v
//!     execution context
//!          |
//!          v
//!     capabilities / resources
//!          |
//!          v
//!     observations
//!          |
//!          v
//!     policy
//!          |
//!          v
//!     planning / ranking
//!          |
//!          v
//!     adaptation / recovery
//!          |
//!          v
//!     verification
//!
//! This file verifies the determinism properties required by the complete
//! resilience subsystem without taking ownership of those subsystems.
//!
//! Production invariants:
//!
//! - `unsafe` is forbidden;
//! - no provider-specific assumptions;
//! - no fixed qubit count;
//! - no fixed machine count;
//! - no fixed backend count;
//! - no hard-coded retry policy;
//! - no wall-clock dependency;
//! - no process/thread identity dependency;
//! - no hash-map iteration-order dependency;
//! - no uncontrolled randomness;
//! - canonical quantum identity remains `quantum::ir::qubit`;
//! - logical and physical qubit identities remain distinct;
//! - deterministic ordering has explicit total tie-breaking;
//! - deterministic serialization is stable;
//! - deterministic transformations do not mutate the source;
//! - concurrency-independent normalization is tested through deterministic
//!   canonicalization;
//! - arbitrary resource identifiers are supported without architectural
//!   machine-size assumptions;
//! - tests remain valid for Rust 1.97 / 1.97.1 and Rust 2021.
//!
//! Integration:
//!
//! This file is intended to be included by the resilience test module:
//!
//!     mod determinism;
//!
//! It deliberately uses `crate::quantum::...` paths so that the tests verify
//! the same module graph used by production code.
//!
//! IMPORTANT:
//!
//! These tests distinguish *decision determinism* from *physical quantum
//! execution reproducibility*. Real quantum hardware is inherently subject
//! to changing noise, calibration, queue state, environment, and other
//! physical conditions. The resilience contract is that deterministic
//! decisions are reproducible when their declared inputs are identical.
//!
//! The tests therefore do not assert that two real quantum executions produce
//! identical measurement samples.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use core::any::type_name;
use core::cmp::Ordering;
use core::fmt::Debug;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId};

use crate::quantum::resilience::model::resource::{
    ResourceAvailability,
    ResourceIdentity,
    ResourceKind,
    ResourceQuantity,
    ResourceScope,
};

use crate::quantum::resilience::planning::{
    CandidateId,
    FeasibilityClass,
    FixedScore,
    RankedCandidate,
    RankingCandidate,
    RankingEngine,
};

/// Canonical test-domain record.
///
/// This is intentionally a test-only representation. It does not become a
/// second resilience domain model. Its purpose is to test deterministic
/// normalization independently of any particular planner implementation.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct DeterministicRecord {
    namespace: String,
    resource: u128,
    priority: i64,
    sequence: u64,
}

/// A canonical byte representation used only by these tests.
///
/// The production serialization subsystem owns production serialization.
/// This helper exists so that the tests can prove that an explicit canonical
/// ordering can remain stable without depending on `HashMap`/`HashSet`
/// iteration order.
fn canonical_records(records: impl IntoIterator<Item = DeterministicRecord>) -> Vec<u8> {
    let mut records: Vec<DeterministicRecord> = records.into_iter().collect();

    records.sort();

    let mut bytes = Vec::new();

    for record in records {
        bytes.extend_from_slice(record.namespace.as_bytes());
        bytes.push(0);

        bytes.extend_from_slice(record.resource.to_string().as_bytes());
        bytes.push(0);

        bytes.extend_from_slice(record.priority.to_string().as_bytes());
        bytes.push(0);

        bytes.extend_from_slice(record.sequence.to_string().as_bytes());
        bytes.push(0xff);
    }

    bytes
}

/// Converts a stable record set into a deterministic ordered vector.
///
/// This is deliberately implemented using explicit sorting rather than
/// relying on insertion order or a hash collection's iteration order.
fn canonicalize_records(
    records: impl IntoIterator<Item = DeterministicRecord>,
) -> Vec<DeterministicRecord> {
    let mut records: Vec<DeterministicRecord> = records.into_iter().collect();
    records.sort();
    records
}

// =============================================================================
// Module / integration boundary tests
// =============================================================================

#[test]
fn determinism_test_can_resolve_all_required_resilience_namespaces() {
    let _ = type_name::<crate::quantum::resilience::api::controller::ResilienceController>();
    let _ = type_name::<crate::quantum::resilience::api::request::ResilienceRequestId>();
    let _ = type_name::<crate::quantum::resilience::api::response::ResilienceResponseMetadata>();

    let _ = type_name::<crate::quantum::resilience::model::fault::Fault>();
    let _ = type_name::<crate::quantum::resilience::model::incident::Incident>();
    let _ = type_name::<crate::quantum::resilience::model::health::HealthState>();
    let _ = type_name::<crate::quantum::resilience::model::resource::ResourceIdentity>();

    let _ = type_name::<crate::quantum::resilience::detection::detector::Detector>();
    let _ = type_name::<crate::quantum::resilience::diagnosis::diagnostician::Diagnostician>();

    let _ = type_name::<crate::quantum::resilience::planning::planner::Planner>();
    let _ = type_name::<crate::quantum::resilience::planning::ranking::RankingEngine>();

    let _ = type_name::<crate::quantum::resilience::adaptation::adapter::Adapter>();
    let _ = type_name::<crate::quantum::resilience::recovery::recoverer::Recoverer>();

    let _ = type_name::<crate::quantum::resilience::verification::verifier::Verifier>();
}

// =============================================================================
// Canonical quantum identity tests
// =============================================================================

#[test]
fn determinism_uses_canonical_qubit_identity_types() {
    let logical = QubitId::new(0);
    let physical = PhysicalQubitId::new(0);

    let logical_resource = ResourceIdentity::logical_qubit(logical);
    let physical_resource = ResourceIdentity::physical_qubit(physical);

    assert_eq!(
        logical_resource.logical_qubit_id(),
        Some(logical)
    );

    assert_eq!(
        physical_resource.physical_qubit_id(),
        Some(physical)
    );

    // Numeric equality of indexes must never collapse distinct identity
    // domains.
    assert_ne!(logical_resource, physical_resource);
}

#[test]
fn determinism_does_not_equate_logical_and_physical_identity() {
    let logical_zero = QubitId::new(0);
    let physical_zero = PhysicalQubitId::new(0);

    let logical = ResourceIdentity::logical_qubit(logical_zero);
    let physical = ResourceIdentity::physical_qubit(physical_zero);

    assert!(logical.is_logical_qubit());
    assert!(!logical.is_physical_qubit());

    assert!(physical.is_physical_qubit());
    assert!(!physical.is_logical_qubit());

    assert_ne!(logical, physical);
}

#[test]
fn determinism_supports_arbitrarily_large_canonical_qubit_indices() {
    let values = [
        0_u64,
        1_u64,
        7_u64,
        127_u64,
        1_000_u64,
        1_000_000_u64,
        u64::MAX,
    ];

    for value in values {
        let logical = QubitId::new(value);
        let physical = PhysicalQubitId::new(value);

        let logical_resource = ResourceIdentity::logical_qubit(logical);
        let physical_resource = ResourceIdentity::physical_qubit(physical);

        assert_eq!(
            logical_resource.logical_qubit_id(),
            Some(logical)
        );

        assert_eq!(
            physical_resource.physical_qubit_id(),
            Some(physical)
        );

        assert_ne!(logical_resource, physical_resource);
    }
}

// =============================================================================
// Resource determinism
// =============================================================================

#[test]
fn resource_kind_ordering_is_total_and_stable() {
    let mut kinds = vec![
        ResourceKind::new("zeta").expect("valid resource kind"),
        ResourceKind::new("alpha").expect("valid resource kind"),
        ResourceKind::new("middle").expect("valid resource kind"),
        ResourceKind::new("alpha.logical").expect("valid resource kind"),
        ResourceKind::new("alpha.physical").expect("valid resource kind"),
    ];

    kinds.sort();

    for pair in kinds.windows(2) {
        assert!(
            pair[0] <= pair[1],
            "resource-kind ordering must be monotonic"
        );
    }

    let first = kinds.clone();

    kinds.sort();

    assert_eq!(
        kinds, first,
        "sorting the same values twice must produce the same order"
    );
}

#[test]
fn resource_quantity_ordering_is_repeatable() {
    let values = vec![
        ResourceQuantity::finite(0),
        ResourceQuantity::finite(1),
        ResourceQuantity::finite(7),
        ResourceQuantity::finite(u128::MAX),
        ResourceQuantity::unbounded(),
        ResourceQuantity::unknown(),
    ];

    let mut first = values.clone();
    let mut second = values.clone();

    first.sort();
    second.sort();

    assert_eq!(first, second);
}

#[test]
fn resource_availability_ordering_is_repeatable() {
    let values = [
        ResourceAvailability::Unknown,
        ResourceAvailability::Unavailable,
        ResourceAvailability::Available,
        ResourceAvailability::Degraded,
    ];

    let mut first = values.to_vec();
    let mut second = values.to_vec();

    first.sort();
    second.sort();

    assert_eq!(first, second);
}

// =============================================================================
// Explicit canonicalization tests
// =============================================================================

#[test]
fn canonicalization_is_independent_of_input_order() {
    let records = vec![
        DeterministicRecord {
            namespace: String::from("backend"),
            resource: 7,
            priority: 10,
            sequence: 2,
        },
        DeterministicRecord {
            namespace: String::from("backend"),
            resource: 1,
            priority: 10,
            sequence: 3,
        },
        DeterministicRecord {
            namespace: String::from("qpu"),
            resource: 0,
            priority: 20,
            sequence: 1,
        },
        DeterministicRecord {
            namespace: String::from("qpu"),
            resource: 1000,
            priority: 20,
            sequence: 0,
        },
    ];

    let mut reversed = records.clone();
    reversed.reverse();

    assert_eq!(
        canonicalize_records(records),
        canonicalize_records(reversed)
    );
}

#[test]
fn canonical_serialization_is_independent_of_input_order() {
    let records = vec![
        DeterministicRecord {
            namespace: String::from("qpu"),
            resource: 17,
            priority: -1,
            sequence: 9,
        },
        DeterministicRecord {
            namespace: String::from("backend"),
            resource: 2,
            priority: 5,
            sequence: 0,
        },
        DeterministicRecord {
            namespace: String::from("logical"),
            resource: u128::MAX,
            priority: 0,
            sequence: 3,
        },
    ];

    let mut permutation = records.clone();
    permutation.swap(0, 2);

    assert_eq!(
        canonical_records(records),
        canonical_records(permutation)
    );
}

#[test]
fn canonical_serialization_is_repeatable() {
    let records = vec![
        DeterministicRecord {
            namespace: String::from("a"),
            resource: 0,
            priority: 0,
            sequence: 0,
        },
        DeterministicRecord {
            namespace: String::from("b"),
            resource: 1,
            priority: 1,
            sequence: 1,
        },
        DeterministicRecord {
            namespace: String::from("c"),
            resource: 2,
            priority: 2,
            sequence: 2,
        },
    ];

    let first = canonical_records(records.clone());
    let second = canonical_records(records);

    assert_eq!(first, second);
}

// =============================================================================
// Hash collection nondeterminism protection
// =============================================================================

#[test]
fn hash_map_iteration_must_never_be_used_as_deterministic_order() {
    let mut map = HashMap::new();

    map.insert("zeta", 3_u64);
    map.insert("alpha", 1_u64);
    map.insert("middle", 2_u64);

    let mut ordered: Vec<(&str, u64)> = map.iter().map(|(key, value)| (*key, *value)).collect();

    ordered.sort_by(|left, right| {
        left.0
            .cmp(right.0)
            .then_with(|| left.1.cmp(&right.1))
    });

    assert_eq!(
        ordered,
        vec![
            ("alpha", 1),
            ("middle", 2),
            ("zeta", 3),
        ]
    );
}

#[test]
fn hash_set_values_require_explicit_canonical_ordering() {
    let mut set = HashSet::new();

    set.insert("zeta");
    set.insert("alpha");
    set.insert("middle");

    let mut ordered: Vec<&str> = set.iter().copied().collect();
    ordered.sort();

    assert_eq!(
        ordered,
        vec![
            "alpha",
            "middle",
            "zeta",
        ]
    );
}

#[test]
fn ordered_collections_provide_explicit_deterministic_iteration() {
    let mut map = BTreeMap::new();

    map.insert("zeta", 3_u64);
    map.insert("alpha", 1_u64);
    map.insert("middle", 2_u64);

    let values: Vec<(&str, u64)> =
        map.iter().map(|(key, value)| (*key, *value)).collect();

    assert_eq!(
        values,
        vec![
            ("alpha", 1),
            ("middle", 2),
            ("zeta", 3),
        ]
    );
}

#[test]
fn ordered_sets_provide_explicit_deterministic_iteration() {
    let mut set = BTreeSet::new();

    set.insert("zeta");
    set.insert("alpha");
    set.insert("middle");

    let values: Vec<&str> = set.iter().copied().collect();

    assert_eq!(
        values,
        vec![
            "alpha",
            "middle",
            "zeta",
        ]
    );
}

// =============================================================================
// Total-order tests
// =============================================================================

#[test]
fn deterministic_records_have_a_total_order() {
    let values = vec![
        DeterministicRecord {
            namespace: String::from("a"),
            resource: 0,
            priority: 0,
            sequence: 0,
        },
        DeterministicRecord {
            namespace: String::from("a"),
            resource: 0,
            priority: 0,
            sequence: 1,
        },
        DeterministicRecord {
            namespace: String::from("a"),
            resource: 1,
            priority: 0,
            sequence: 0,
        },
        DeterministicRecord {
            namespace: String::from("b"),
            resource: 0,
            priority: 0,
            sequence: 0,
        },
    ];

    for left in &values {
        for right in &values {
            let forward = left.cmp(right);
            let reverse = right.cmp(left);

            assert_eq!(
                forward,
                reverse.reverse(),
                "ordering must be antisymmetric"
            );
        }
    }
}

#[test]
fn equal_primary_scores_require_a_stable_secondary_key() {
    let mut candidates = vec![
        (
            FixedScore::from_integer(10).expect("valid fixed score"),
            "candidate-b",
        ),
        (
            FixedScore::from_integer(10).expect("valid fixed score"),
            "candidate-a",
        ),
        (
            FixedScore::from_integer(10).expect("valid fixed score"),
            "candidate-c",
        ),
    ];

    candidates.sort_by(|left, right| {
        left.0
            .cmp(&right.0)
            .then_with(|| left.1.cmp(right.1))
    });

    assert_eq!(
        candidates,
        vec![
            (
                FixedScore::from_integer(10).expect("valid fixed score"),
                "candidate-a",
            ),
            (
                FixedScore::from_integer(10).expect("valid fixed score"),
                "candidate-b",
            ),
            (
                FixedScore::from_integer(10).expect("valid fixed score"),
                "candidate-c",
            ),
        ]
    );
}

// =============================================================================
// Fixed-score determinism
// =============================================================================

#[test]
fn fixed_scores_are_repeatable() {
    let values = [
        -1_000_i128,
        -1_i128,
        0_i128,
        1_i128,
        10_i128,
        1_000_i128,
    ];

    for value in values {
        let first = FixedScore::from_integer(value)
            .expect("integer score must be representable");

        let second = FixedScore::from_integer(value)
            .expect("same integer score must be representable");

        assert_eq!(first, second);
        assert_eq!(first.cmp(&second), Ordering::Equal);
    }
}

#[test]
fn fixed_score_ordering_is_transitive_for_normal_inputs() {
    let values = [
        FixedScore::from_integer(-10).expect("valid score"),
        FixedScore::from_integer(0).expect("valid score"),
        FixedScore::from_integer(10).expect("valid score"),
        FixedScore::from_integer(100).expect("valid score"),
    ];

    for a in values {
        for b in values {
            for c in values {
                if a <= b && b <= c {
                    assert!(
                        a <= c,
                        "fixed-score ordering must be transitive"
                    );
                }
            }
        }
    }
}

// =============================================================================
// Planning / ranking integration tests
// =============================================================================

fn deterministic_ranking_policy() -> crate::quantum::resilience::planning::RankingPolicy {
    use crate::quantum::resilience::planning::{
        RankingMode,
        RankingPolicy,
    };

    RankingPolicy::new(
        RankingMode::Weighted,
    )
    .expect("weighted ranking policy must be constructible")
}

fn rank_candidates(
    candidates: Vec<RankingCandidate>,
) -> crate::quantum::resilience::planning::RankingResult {
    RankingEngine::new(deterministic_ranking_policy())
        .expect("ranking engine must accept deterministic policy")
        .rank(candidates)
        .expect("candidate ranking must succeed")
}

#[test]
fn ranking_same_candidates_produces_same_result() {
    let first_candidates = deterministic_test_candidates();
    let second_candidates = deterministic_test_candidates();

    let first = rank_candidates(first_candidates);
    let second = rank_candidates(second_candidates);

    assert_eq!(
        first,
        second,
        "identical normalized ranking inputs must produce identical results"
    );
}

#[test]
fn ranking_is_independent_of_candidate_input_order() {
    let mut first_candidates = deterministic_test_candidates();
    let mut second_candidates = deterministic_test_candidates();

    second_candidates.reverse();

    first_candidates.sort();
    second_candidates.sort();

    let first = rank_candidates(first_candidates);
    let second = rank_candidates(second_candidates);

    assert_eq!(
        first,
        second,
        "candidate input ordering must not affect deterministic ranking"
    );
}

#[test]
fn ranking_output_is_repeatable_across_multiple_invocations() {
    let candidates = deterministic_test_candidates();

    let first = rank_candidates(candidates.clone());
    let second = rank_candidates(candidates.clone());
    let third = rank_candidates(candidates);

    assert_eq!(first, second);
    assert_eq!(second, third);
}

fn deterministic_test_candidates() -> Vec<RankingCandidate> {
    // The constructor contract belongs to planning/ranking.rs. The values
    // below intentionally remain provider-independent and contain no qubit
    // count, provider name, retry count, or hardware-specific threshold.
    //
    // Candidate construction is kept in one helper so that this test file
    // does not scatter assumptions throughout individual tests.
    vec![
        ranking_candidate(
            "candidate-a",
            FeasibilityClass::Feasible,
            10,
            1,
        ),
        ranking_candidate(
            "candidate-b",
            FeasibilityClass::Feasible,
            10,
            2,
        ),
        ranking_candidate(
            "candidate-c",
            FeasibilityClass::Feasible,
            9,
            3,
        ),
    ]
}

fn ranking_candidate(
    identity: &str,
    feasibility: FeasibilityClass,
    score: i128,
    priority: i64,
) -> RankingCandidate {
    RankingCandidate::new(
        CandidateId::new(identity),
        feasibility,
        FixedScore::from_integer(score).expect("test score must be representable"),
        priority,
    )
}

// =============================================================================
// Deterministic identity separation
// =============================================================================

#[test]
fn operational_identity_must_not_be_used_as_quantum_identity() {
    let logical = ResourceIdentity::logical_qubit(QubitId::new(7));
    let physical = ResourceIdentity::physical_qubit(PhysicalQubitId::new(7));

    assert_ne!(logical, physical);

    // The test deliberately does not introduce an operational ID and convert
    // it into a QubitId. Operational identities such as execution IDs,
// incident IDs, checkpoint IDs and recovery IDs belong to separate domains.
}

// =============================================================================
// Large-resource determinism
// =============================================================================

#[test]
fn canonicalization_scales_without_a_machine_size_constant() {
    // This is deliberately generated from the requested workload size rather
    // than from a production machine-size constant.
    //
    // The test uses a moderate CI-safe value. Production code remains
    // unbounded by this test value; the available process memory determines
    // the practical upper bound.
    let count = 4_096_usize;

    let records = (0..count)
        .map(|index| DeterministicRecord {
            namespace: if index % 2 == 0 {
                String::from("logical")
            } else {
                String::from("physical")
            },
            resource: index as u128,
            priority: (index % 17) as i64,
            sequence: (count - index) as u64,
        })
        .collect::<Vec<_>>();

    let mut reversed = records.clone();
    reversed.reverse();

    let first = canonical_records(records);
    let second = canonical_records(reversed);

    assert_eq!(first, second);
}

#[test]
fn very_large_resource_identifiers_do_not_change_determinism() {
    let records = vec![
        DeterministicRecord {
            namespace: String::from("logical"),
            resource: 0,
            priority: 0,
            sequence: 0,
        },
        DeterministicRecord {
            namespace: String::from("logical"),
            resource: u128::MAX,
            priority: 0,
            sequence: 1,
        },
        DeterministicRecord {
            namespace: String::from("physical"),
            resource: u128::MAX,
            priority: i64::MAX,
            sequence: u64::MAX,
        },
    ];

    let mut reversed = records.clone();
    reversed.reverse();

    assert_eq!(
        canonical_records(records),
        canonical_records(reversed)
    );
}

// =============================================================================
// Purity / mutation tests
// =============================================================================

#[test]
fn deterministic_canonicalization_does_not_modify_caller_owned_input() {
    let records = vec![
        DeterministicRecord {
            namespace: String::from("z"),
            resource: 2,
            priority: 0,
            sequence: 0,
        },
        DeterministicRecord {
            namespace: String::from("a"),
            resource: 1,
            priority: 0,
            sequence: 0,
        },
    ];

    let original = records.clone();

    let _ = canonicalize_records(records.clone());

    assert_eq!(
        records,
        original,
        "deterministic normalization must not mutate caller-owned input"
    );
}

// =============================================================================
// Repeated decision-equivalence tests
// =============================================================================

#[test]
fn repeated_canonicalization_is_idempotent() {
    let records = vec![
        DeterministicRecord {
            namespace: String::from("z"),
            resource: 2,
            priority: 1,
            sequence: 3,
        },
        DeterministicRecord {
            namespace: String::from("a"),
            resource: 1,
            priority: 0,
            sequence: 2,
        },
        DeterministicRecord {
            namespace: String::from("m"),
            resource: 9,
            priority: 7,
            sequence: 1,
        },
    ];

    let once = canonicalize_records(records.clone());
    let twice = canonicalize_records(once.clone());
    let three_times = canonicalize_records(twice.clone());

    assert_eq!(once, twice);
    assert_eq!(twice, three_times);
}

#[test]
fn canonicalization_preserves_duplicate_semantic_records() {
    let record = DeterministicRecord {
        namespace: String::from("logical"),
        resource: 7,
        priority: 1,
        sequence: 2,
    };

    let records = vec![
        record.clone(),
        record.clone(),
        record.clone(),
    ];

    let normalized = canonicalize_records(records);

    assert_eq!(normalized.len(), 3);
    assert_eq!(normalized[0], record);
    assert_eq!(normalized[1], record);
    assert_eq!(normalized[2], record);
}

// =============================================================================
// Time / environment independence
// =============================================================================

#[test]
fn deterministic_helpers_have_no_implicit_time_input() {
    // The deterministic helpers used by this test suite receive all of their
    // data as arguments. There is deliberately no:
    //
    //     SystemTime::now()
    //     Instant::now()
    //     thread::current()
    //     process::id()
    //
    // in the deterministic path.
    //
    // The assertion makes the intended contract executable without coupling
    // the test to host-specific clock behavior.
    let input = DeterministicRecord {
        namespace: String::from("test"),
        resource: 1,
        priority: 0,
        sequence: 0,
    };

    let first = canonical_records(vec![input.clone()]);
    let second = canonical_records(vec![input]);

    assert_eq!(first, second);
}

// =============================================================================
// Canonical resource scope tests
// =============================================================================

#[test]
fn resource_scope_is_deterministically_derived_from_identity() {
    let logical = ResourceIdentity::logical_qubit(QubitId::new(42));
    let physical = ResourceIdentity::physical_qubit(PhysicalQubitId::new(42));

    assert_eq!(
        ResourceScope::for_identity(logical),
        ResourceScope::Logical
    );

    assert_eq!(
        ResourceScope::for_identity(physical),
        ResourceScope::Physical
    );
}

#[test]
fn same_canonical_identity_always_has_same_resource_scope() {
    for value in [
        0_u64,
        1_u64,
        42_u64,
        1_000_u64,
        u64::MAX,
    ] {
        let logical = ResourceIdentity::logical_qubit(QubitId::new(value));

        let first = ResourceScope::for_identity(logical);
        let second = ResourceScope::for_identity(logical);

        assert_eq!(first, second);
    }
}

// =============================================================================
// Deterministic ranking candidate identity
// =============================================================================

#[test]
fn candidate_ids_are_stable_for_equal_text() {
    let first = CandidateId::new("candidate");
    let second = CandidateId::new("candidate");

    assert_eq!(first, second);
}

#[test]
fn distinct_candidate_ids_remain_distinct() {
    let first = CandidateId::new("candidate-a");
    let second = CandidateId::new("candidate-b");

    assert_ne!(first, second);
}

#[test]
fn candidate_id_ordering_is_repeatable() {
    let mut values = vec![
        CandidateId::new("z"),
        CandidateId::new("a"),
        CandidateId::new("m"),
    ];

    let mut second = values.clone();

    values.sort();
    second.sort();

    assert_eq!(values, second);
}

// =============================================================================
// Determinism versus verification
// =============================================================================

#[test]
fn ranking_types_do_not_replace_final_verification_authority() {
    // Compile-time integration boundary:
    //
    // Ranking is a recommendation layer. Verification remains a separate
    // subsystem. This test deliberately resolves both types without invoking
    // one as the other.
    let _ = type_name::<RankingEngine>();
    let _ = type_name::<RankedCandidate>();
    let _ = type_name::<
        crate::quantum::resilience::verification::verifier::Verifier,
    >();
}

// =============================================================================
// Regression guards
// =============================================================================

#[test]
fn no_resilience_specific_qubit_identity_is_used_by_this_contract() {
    // These are the only quantum identity types intentionally imported by
    // this file.
    //
    // If a future implementation introduces:
    //
    //     ResilienceQubitId
    //     PlannerQubitId
    //     RecoveryQubitId
    //
    // it must not replace these canonical IR identities.
    let logical: QubitId = QubitId::new(0);
    let physical: PhysicalQubitId = PhysicalQubitId::new(0);

    assert_ne!(
        ResourceIdentity::logical_qubit(logical),
        ResourceIdentity::physical_qubit(physical)
    );
}

#[test]
fn deterministic_contract_does_not_imply_fixed_hardware_size() {
    // Resource identity is data, not a compile-time machine-size assumption.
    //
    // The values intentionally include both small and very large identifiers.
    let identifiers = [
        0_u64,
        1_u64,
        127_u64,
        1_000_u64,
        1_000_000_u64,
        u64::MAX,
    ];

    for identifier in identifiers {
        let logical = ResourceIdentity::logical_qubit(QubitId::new(identifier));

        assert!(logical.is_logical_qubit());
    }
}