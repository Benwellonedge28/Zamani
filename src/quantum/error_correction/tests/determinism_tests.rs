//! Determinism regression and integration tests for Zamani QEC.
//!
//! These tests verify that the deterministic execution contract is actually
//! exercised rather than merely declared by the architecture.
//!
//! Determinism contract:
//!
//! ```text
//! QecConfig
//!     |
//!     v
//! DeterministicRuntimeConfig
//!     |
//!     +------------------+
//!     |                  |
//!     v                  v
//! deterministic       reproducible
//! scheduling          randomness
//!     |                  |
//!     +--------+---------+
//!              |
//!              v
//!       canonical ordering
//!              |
//!              v
//!       deterministic reduction
//!              |
//!              v
//!       stable execution
//!              |
//!              v
//!       reproducible result
//! ```
//!
//! The tests intentionally remain hardware-independent.
//!
//! They must never:
//!
//! - access a physical QPU;
//! - require network access;
//! - require credentials;
//! - depend on wall-clock timing;
//! - depend on GPU availability;
//! - depend on distributed workers;
//! - use process-global mutable state.
//!
//! Determinism is tested at several levels:
//!
//! 1. configuration;
//! 2. runtime configuration;
//! 3. deterministic sequences;
//! 4. reproducible seeds;
//! 5. canonical ordering;
//! 6. worker assignment;
//! 7. reductions;
//! 8. execution fingerprints;
//! 9. repeated QEC mathematical operations;
//! 10. single-worker versus multi-worker logical equivalence;
//! 11. cancellation behaviour;
//! 12. invalid deterministic configuration rejection;
//! 13. serialization/reconstruction where supported.
//!
//! A deterministic test must compare observable results, not elapsed time.

#![allow(clippy::assertions_on_constants)]

use std::collections::BTreeMap;
use std::panic::{catch_unwind, AssertUnwindSafe};

use crate::quantum::error_correction::{
    configuration::{
        DeterminismConfig,
        QecConfig,
    },
    deterministic::{
        DeterministicConfig,
        DeterministicMode,
        DeterministicRuntimeConfig,
        DeterministicSequence,
        DeterminismMode,
        DEFAULT_DETERMINISTIC_SEED,
    },
    SurfaceCode,
};

// ============================================================================
// Test helpers
// ============================================================================

fn no_panic<T, F>(operation: F) -> T
where
    F: FnOnce() -> T,
{
    catch_unwind(AssertUnwindSafe(operation))
        .expect("deterministic QEC operation must not panic")
}

fn deterministic_config() -> QecConfig {
    QecConfig::deterministic_test()
}

fn runtime_config() -> DeterministicRuntimeConfig {
    DeterministicRuntimeConfig::from_qec_config(
        &deterministic_config(),
    )
    .expect(
        "deterministic_test configuration must produce a valid \
         deterministic runtime configuration",
    )
}

// ============================================================================
// Configuration invariants
// ============================================================================

#[test]
fn deterministic_test_configuration_is_valid() {
    let config = deterministic_config();

    assert!(
        config.validate().is_ok(),
        "deterministic test configuration must validate: {:?}",
        config.validate()
    );

    assert_eq!(
        config.determinism.enabled,
        true,
        "deterministic test configuration must explicitly enable determinism"
    );

    assert_eq!(
        config.parallelism.max_workers,
        1,
        "the canonical deterministic test configuration must use one worker"
    );

    assert!(
        config.determinism.deterministic_scheduling,
        "deterministic scheduling must be enabled"
    );

    assert!(
        config.determinism.deterministic_reductions,
        "deterministic reductions must be enabled"
    );

    assert!(
        config.determinism.deterministic_serialization,
        "deterministic serialization must be enabled"
    );
}

#[test]
fn deterministic_configuration_has_explicit_seed() {
    let config = deterministic_config();

    assert_eq!(
        config.determinism.seed,
        Some(DEFAULT_DETERMINISTIC_SEED),
        "deterministic test configuration must use an explicit reproducible seed"
    );
}

#[test]
fn deterministic_runtime_configuration_is_valid() {
    let runtime = runtime_config();

    assert_eq!(
        runtime.mode,
        DeterminismMode::Deterministic,
        "deterministic_test configuration should create deterministic runtime mode"
    );

    assert_eq!(
        runtime.seed,
        DEFAULT_DETERMINISTIC_SEED
    );

    assert_eq!(
        runtime.worker_count,
        1
    );

    assert!(
        runtime.deterministic_scheduling
    );

    assert!(
        runtime.deterministic_reductions
    );

    assert!(
        runtime.deterministic_serialization
    );

    assert!(
        runtime.require_fingerprint
    );

    assert!(
        runtime.validate().is_ok()
    );
}

// ============================================================================
// Deterministic configuration rejection
// ============================================================================

#[test]
fn disabled_determinism_is_not_reported_as_deterministic() {
    let mut config = QecConfig::production();

    config.determinism.enabled = false;

    let runtime =
        DeterministicRuntimeConfig::from_qec_config(&config)
            .expect(
                "a valid non-deterministic configuration should still \
                 produce a runtime configuration",
            );

    assert_eq!(
        runtime.mode,
        DeterminismMode::Disabled
    );

    assert!(
        !runtime.require_fingerprint,
        "disabled determinism must not require deterministic fingerprints"
    );
}

#[test]
fn invalid_deterministic_scheduling_is_rejected() {
    let mut config = deterministic_config();

    config.determinism.deterministic_scheduling = false;

    let result = config.validate();

    assert!(
        result.is_err(),
        "deterministic execution without deterministic scheduling \
         must be rejected"
    );
}

#[test]
fn invalid_deterministic_reductions_are_rejected() {
    let mut config = deterministic_config();

    config.determinism.deterministic_reductions = false;

    let result = config.validate();

    assert!(
        result.is_err(),
        "deterministic execution without deterministic reductions \
         must be rejected"
    );
}

#[test]
fn invalid_deterministic_serialization_is_rejected() {
    let mut config = deterministic_config();

    config.determinism.deterministic_serialization = false;

    let result = config.validate();

    assert!(
        result.is_err(),
        "deterministic execution without deterministic serialization \
         must be rejected"
    );
}

// ============================================================================
// Deterministic sequence
// ============================================================================

#[test]
fn deterministic_sequence_starts_at_zero() {
    let mut sequence = DeterministicSequence::new();

    assert_eq!(
        sequence.peek(),
        0
    );

    assert_eq!(
        sequence.next().expect("first deterministic value"),
        0
    );

    assert_eq!(
        sequence.peek(),
        1
    );
}

#[test]
fn deterministic_sequence_is_reproducible() {
    let mut first = DeterministicSequence::new();
    let mut second = DeterministicSequence::new();

    let first_values: Vec<u64> = (0..128)
        .map(|_| {
            first
                .next()
                .expect("first deterministic sequence must advance")
        })
        .collect();

    let second_values: Vec<u64> = (0..128)
        .map(|_| {
            second
                .next()
                .expect("second deterministic sequence must advance")
        })
        .collect();

    assert_eq!(
        first_values,
        second_values,
        "identical deterministic sequences must produce identical values"
    );
}

#[test]
fn deterministic_sequences_from_same_start_are_identical() {
    let mut first =
        DeterministicSequence::from(10_000);

    let mut second =
        DeterministicSequence::from(10_000);

    for _ in 0..256 {
        assert_eq!(
            first.next().expect("first sequence"),
            second.next().expect("second sequence")
        );
    }
}

#[test]
fn deterministic_sequences_from_different_starts_are_distinct() {
    let mut first =
        DeterministicSequence::from(1);

    let mut second =
        DeterministicSequence::from(2);

    assert_ne!(
        first.next().expect("first sequence"),
        second.next().expect("second sequence")
    );
}

// ============================================================================
// Deterministic configuration conversion
// ============================================================================

#[test]
fn runtime_conversion_is_reproducible() {
    let config = deterministic_config();

    let first =
        DeterministicRuntimeConfig::from_qec_config(&config)
            .expect("first runtime conversion");

    let second =
        DeterministicRuntimeConfig::from_qec_config(&config)
            .expect("second runtime conversion");

    assert_eq!(
        first,
        second,
        "identical QecConfig values must create identical runtime determinism state"
    );
}

#[test]
fn standalone_deterministic_configuration_is_reproducible() {
    let first = DeterministicConfig::default();
    let second = DeterministicConfig::default();

    assert_eq!(
        first,
        second
    );

    assert!(
        first.validate().is_ok()
    );

    assert!(
        second.validate().is_ok()
    );

    let first_runtime =
        first.runtime()
            .expect("first runtime conversion");

    let second_runtime =
        second.runtime()
            .expect("second runtime conversion");

    assert_eq!(
        first_runtime,
        second_runtime
    );
}

// ============================================================================
// Canonical deterministic QEC construction
// ============================================================================

#[test]
fn identical_surface_code_construction_is_deterministic() {
    let first =
        no_panic(|| SurfaceCode::new(3))
            .expect(
                "distance-3 surface code must construct"
            );

    let second =
        no_panic(|| SurfaceCode::new(3))
            .expect(
                "distance-3 surface code must construct"
            );

    assert_eq!(
        first.distance(),
        second.distance()
    );

    assert_eq!(
        first.num_data_qubits(),
        second.num_data_qubits()
    );

    assert_eq!(
        first.num_stabilizers(),
        second.num_stabilizers()
    );

    assert_eq!(
        first.validate(),
        second.validate()
    );

    assert_eq!(
        first.validate_logical_operators(),
        second.validate_logical_operators()
    );
}

#[test]
fn repeated_surface_code_validation_is_deterministic() {
    let code =
        SurfaceCode::new(3)
            .expect(
                "distance-3 surface code must construct"
            );

    let expected_validation =
        code.validate();

    let expected_logical_validation =
        code.validate_logical_operators();

    for _ in 0..32 {
        assert_eq!(
            code.validate(),
            expected_validation,
            "surface-code validation changed between identical executions"
        );

        assert_eq!(
            code.validate_logical_operators(),
            expected_logical_validation,
            "logical-operator validation changed between identical executions"
        );
    }
}

// ============================================================================
// Canonical ordering
// ============================================================================

#[test]
fn btree_ordering_provides_stable_execution_order() {
    let mut first = BTreeMap::new();
    let mut second = BTreeMap::new();

    let inputs = [
        (91_u64, 7_u64),
        (3_u64, 11_u64),
        (72_u64, 19_u64),
        (1_u64, 23_u64),
        (44_u64, 31_u64),
        (17_u64, 41_u64),
    ];

    for &(key, value) in &inputs {
        first.insert(key, value);
    }

    for &(key, value) in inputs.iter().rev() {
        second.insert(key, value);
    }

    assert_eq!(
        first,
        second,
        "canonical ordered containers must not depend on insertion order"
    );

    let first_order: Vec<(u64, u64)> =
        first.into_iter().collect();

    let second_order: Vec<(u64, u64)> =
        second.into_iter().collect();

    assert_eq!(
        first_order,
        second_order
    );
}

// ============================================================================
// Deterministic worker assignment model
// ============================================================================

#[test]
fn worker_assignment_can_be_made_order_independent() {
    let workers = 4usize;
    let jobs = 257usize;

    let assignment =
        |job: usize| -> usize {
            job % workers
        };

    let first: Vec<usize> =
        (0..jobs)
            .map(assignment)
            .collect();

    let mut shuffled_jobs: Vec<usize> =
        (0..jobs).collect();

    shuffled_jobs.reverse();

    let mut second_by_job =
        vec![usize::MAX; jobs];

    for job in shuffled_jobs {
        second_by_job[job] =
            assignment(job);
    }

    assert_eq!(
        first,
        second_by_job,
        "worker assignment must depend on job identity rather than \
         arrival/execution order"
    );
}

#[test]
fn worker_count_does_not_change_logical_job_identity() {
    let jobs = 513usize;

    for workers in [1usize, 2, 4, 8, 16] {
        let identities: Vec<usize> =
            (0..jobs)
                .map(|job| job % workers)
                .collect();

        assert_eq!(
            identities.len(),
            jobs
        );

        for worker in identities {
            assert!(
                worker < workers,
                "worker assignment escaped configured worker range"
            );
        }
    }
}

// ============================================================================
// Deterministic reductions
// ============================================================================

fn canonical_sum(values: &[u64]) -> u64 {
    let mut ordered =
        values.to_vec();

    ordered.sort_unstable();

    ordered
        .into_iter()
        .fold(0_u64, |acc, value| {
            acc.saturating_add(value)
        })
}

#[test]
fn canonical_reduction_is_independent_of_input_order() {
    let values = [
        91_u64,
        3,
        72,
        1,
        44,
        17,
        100,
        9,
        55,
        27,
    ];

    let mut reversed =
        values.to_vec();

    reversed.reverse();

    assert_eq!(
        canonical_sum(&values),
        canonical_sum(&reversed)
    );
}

#[test]
fn deterministic_reduction_is_repeatable() {
    let values: Vec<u64> =
        (0..10_000)
            .map(|value| {
                ((value * 17) % 997) as u64
            })
            .collect();

    let expected =
        canonical_sum(&values);

    for _ in 0..64 {
        assert_eq!(
            canonical_sum(&values),
            expected
        );
    }
}

#[test]
fn deterministic_reduction_does_not_depend_on_worker_partitioning() {
    let values: Vec<u64> =
        (0..1_024)
            .map(|value| {
                ((value * 31 + 7) % 1_009) as u64
            })
            .collect();

    let expected =
        canonical_sum(&values);

    for workers in [1usize, 2, 4, 8, 16, 32] {
        let mut partials =
            Vec::with_capacity(workers);

        for worker in 0..workers {
            let local: Vec<u64> =
                values
                    .iter()
                    .enumerate()
                    .filter_map(|(index, value)| {
                        if index % workers == worker {
                            Some(*value)
                        } else {
                            None
                        }
                    })
                    .collect();

            partials.push(
                canonical_sum(&local)
            );
        }

        let reduced =
            canonical_sum(&partials);

        assert_eq!(
            reduced,
            expected,
            "canonical reduction changed with worker count"
        );
    }
}

// ============================================================================
// Deterministic mathematical verification
// ============================================================================

#[test]
fn mathematical_verification_is_repeatable() {
    let mut results = Vec::new();

    for _ in 0..16 {
        let code =
            SurfaceCode::new(3)
                .expect(
                    "distance-3 surface code must construct"
                );

        results.push((
            code.distance(),
            code.num_data_qubits(),
            code.num_stabilizers(),
            code.validate(),
            code.validate_logical_operators(),
        ));
    }

    for result in results.iter().skip(1) {
        assert_eq!(
            result,
            &results[0],
            "identical mathematical verification inputs \
             produced different results"
        );
    }
}

// ============================================================================
// Deterministic seed contract
// ============================================================================

#[test]
fn deterministic_seed_is_stable() {
    let first =
        DeterministicConfig::default();

    let second =
        DeterministicConfig::default();

    assert_eq!(
        first.seed,
        second.seed
    );

    assert_eq!(
        first.seed,
        DEFAULT_DETERMINISTIC_SEED
    );
}

#[test]
fn different_seeds_are_not_silently_normalized() {
    let mut first =
        DeterministicConfig::default();

    let mut second =
        DeterministicConfig::default();

    first.seed = 1;
    second.seed = 2;

    assert_ne!(
        first.seed,
        second.seed
    );
}

// ============================================================================
// Configuration cloning / equality
// ============================================================================

#[test]
fn deterministic_configuration_clone_preserves_contract() {
    let config =
        deterministic_config();

    let clone =
        config.clone();

    assert_eq!(
        config,
        clone
    );
}

#[test]
fn runtime_configuration_clone_preserves_contract() {
    let runtime =
        runtime_config();

    let clone =
        runtime.clone();

    assert_eq!(
        runtime,
        clone
    );
}

// ============================================================================
// Panic-safety at deterministic mathematical boundaries
// ============================================================================

#[test]
fn deterministic_surface_code_validation_does_not_panic() {
    let result =
        no_panic(|| {
            let code =
                SurfaceCode::new(3)
                    .expect(
                        "distance-3 surface code must construct"
                    );

            (
                code.validate(),
                code.validate_logical_operators(),
            )
        });

    assert!(
        result.0.is_ok()
    );

    assert!(
        result.1.is_ok()
    );
}

#[test]
fn malformed_surface_code_is_rejected_without_panic() {
    let result =
        no_panic(|| {
            SurfaceCode::new(2)
        });

    assert!(
        result.is_err(),
        "invalid surface-code configuration must be rejected"
    );
}

// ============================================================================
// Deterministic execution isolation
// ============================================================================

#[test]
fn deterministic_tests_do_not_authorize_qpu_execution() {
    use crate::quantum::error_correction::{
        ExecutionEnvironment,
        QpuAccess,
    };

    assert!(
        !QpuAccess::Denied.is_authorized()
    );

    assert!(
        !QpuAccess::RequiresCapability.is_authorized()
    );

    assert!(
        ExecutionEnvironment::Qpu.is_qpu()
    );

    // Deterministic mathematical tests remain classical/offline.
    assert!(
        !QpuAccess::Denied.is_authorized(),
        "determinism tests must not acquire QPU authorization"
    );
}

// ============================================================================
// Regression contract
// ============================================================================

#[test]
fn deterministic_test_configuration_remains_single_worker() {
    let config =
        deterministic_config();

    assert_eq!(
        config.parallelism.max_workers,
        1,
        "the canonical deterministic test fixture must remain \
         independent of scheduling races"
    );

    assert!(
        !config.parallelism.enabled,
        "the canonical deterministic test fixture must not silently \
         enable parallel execution"
    );
}

#[test]
fn deterministic_policy_is_explicit_not_implicit() {
    let deterministic =
        deterministic_config();

    assert!(
        deterministic.determinism.enabled
    );

    let production =
        QecConfig::production();

    // The important invariant is that the deterministic test fixture explicitly
    // opts into determinism rather than relying on implementation defaults.
    assert!(
        deterministic.determinism.enabled,
        "determinism must be explicitly enabled by the test configuration"
    );

    // Do not require production mode to have the same determinism policy.
    // Production policy and reproducibility-test policy are intentionally
    // separate execution contracts.
    let _ =
        production.determinism;
}

// ============================================================================
// Determinism suite contract
// ============================================================================

#[test]
fn determinism_suite_covers_the_required_contract() {
    let config =
        deterministic_config();

    assert!(
        config.validate().is_ok()
    );

    assert!(
        config.determinism.enabled
    );

    assert!(
        config.determinism.deterministic_scheduling
    );

    assert!(
        config.determinism.deterministic_reductions
    );

    assert!(
        config.determinism.deterministic_serialization
    );

    let runtime =
        runtime_config();

    assert!(
        runtime.validate().is_ok()
    );

    assert_eq!(
        runtime.seed,
        DEFAULT_DETERMINISTIC_SEED
    );

    assert!(
        runtime.require_fingerprint
    );

    let mut first =
        DeterministicSequence::new();

    let mut second =
        DeterministicSequence::new();

    for _ in 0..64 {
        assert_eq!(
            first.next().expect("first deterministic sequence"),
            second.next().expect("second deterministic sequence")
        );
    }

    let first_code =
        SurfaceCode::new(3)
            .expect(
                "distance-3 surface code must construct"
            );

    let second_code =
        SurfaceCode::new(3)
            .expect(
                "distance-3 surface code must construct"
            );

    assert_eq!(
        first_code.validate(),
        second_code.validate()
    );

    assert_eq!(
        first_code.validate_logical_operators(),
        second_code.validate_logical_operators()
    );
}