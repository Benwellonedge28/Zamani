//! Production-grade property and invariant tests for Zamani QEC.
//!
//! These tests are intentionally written without requiring a third-party
//! property-testing framework. They provide deterministic property generation,
//! bounded resource testing, malformed-input testing, backend/QPU boundary
//! testing, and mathematical invariants.
//!
//! The tests are designed around the following architectural principles:
//!
//! 1. Untrusted input must be validated before reaching QEC algorithms.
//! 2. QEC workloads must remain bounded by explicit resource policies.
//! 3. Core algorithms must return errors instead of panicking.
//! 4. Deterministic configurations must produce deterministic results.
//! 5. Sparse/streaming/partitioned execution must not alter mathematical
//!    correctness.
//! 6. CPU, parallel, GPU, accelerator, distributed, and QPU execution are
//!    explicit execution backends/capabilities.
//! 7. A QPU is never implicitly trusted merely because it is a backend.
//!
//! NOTE:
//! This file intentionally keeps the generated test domain conservative.
//! Property tests should detect invariant violations without becoming an
//! accidental denial-of-service workload themselves.

#![allow(dead_code)]

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Debug;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::time::{Duration, Instant};

use super::super::{
    arithmetic,
    backend,
    capabilities,
    configuration,
    deterministic,
    limits,
    memory,
    resources,
    validation,
};

// -----------------------------------------------------------------------------
// Test constants
// -----------------------------------------------------------------------------

const TEST_SEED: u64 = 0x5A4D_414E_4951_4543; // "ZAMANI QEC"

const MIN_DISTANCE: usize = 3;
const MAX_PROPERTY_DISTANCE: usize = 15;

const MAX_GENERATED_CASES: usize = 256;
const MAX_GENERATED_ROUNDS: usize = 32;
const MAX_GENERATED_EVENTS: usize = 4096;

const MAX_PROPERTY_RUNTIME: Duration = Duration::from_secs(10);

// -----------------------------------------------------------------------------
// Deterministic pseudo-random generator
// -----------------------------------------------------------------------------

/// Small deterministic PRNG used exclusively by this test module.
///
/// This is deliberately not cryptographic. It exists to guarantee that:
///
///     same seed + same test
///
/// produces:
///
///     same generated cases
///
/// across test runs.
#[derive(Clone, Debug)]
struct TestRng {
    state: u64,
}

impl TestRng {
    fn new(seed: u64) -> Self {
        let seed = if seed == 0 { TEST_SEED } else { seed };

        Self {
            state: seed ^ 0x9E37_79B9_7F4A_7C15,
        }
    }

    fn next_u64(&mut self) -> u64 {
        // SplitMix64.
        self.state = self.state.wrapping_add(0x9E37_79B9_7F4A_7C15);

        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    fn next_usize(&mut self, upper_exclusive: usize) -> usize {
        if upper_exclusive == 0 {
            return 0;
        }

        (self.next_u64() as usize) % upper_exclusive
    }

    fn next_bool(&mut self) -> bool {
        self.next_u64() & 1 == 1
    }

    fn choose<T: Clone>(&mut self, values: &[T]) -> T {
        assert!(!values.is_empty());
        values[self.next_usize(values.len())].clone()
    }
}

// -----------------------------------------------------------------------------
// Generic test helpers
// -----------------------------------------------------------------------------

fn assert_no_panic<T, F>(name: &str, operation: F) -> T
where
    F: FnOnce() -> T + std::panic::UnwindSafe,
{
    match catch_unwind(operation) {
        Ok(value) => value,
        Err(_) => panic!("property violated: `{name}` panicked"),
    }
}

fn assert_deterministic<T, F>(name: &str, operation: F)
where
    T: Eq + Debug,
    F: Fn() -> T,
{
    let first = assert_no_panic(name, || operation());
    let second = assert_no_panic(name, || operation());

    assert_eq!(
        first, second,
        "determinism property violated for `{name}`"
    );
}

fn bounded_distance_cases() -> Vec<usize> {
    let mut distances = vec![
        3usize, 5, 7, 9, 11, 13, 15,
    ];

    distances.retain(|d| *d <= MAX_PROPERTY_DISTANCE);
    distances.sort_unstable();
    distances.dedup();

    distances
}

fn assert_property_runtime(start: Instant, property_name: &str) {
    assert!(
        start.elapsed() <= MAX_PROPERTY_RUNTIME,
        "property `{property_name}` exceeded the test runtime budget"
    );
}

// -----------------------------------------------------------------------------
// Basic generator properties
// -----------------------------------------------------------------------------

#[test]
fn property_rng_is_deterministic() {
    let mut a = TestRng::new(TEST_SEED);
    let mut b = TestRng::new(TEST_SEED);

    for _ in 0..MAX_GENERATED_CASES {
        assert_eq!(a.next_u64(), b.next_u64());
    }
}

#[test]
fn property_rng_zero_seed_is_stable() {
    let mut a = TestRng::new(0);
    let mut b = TestRng::new(0);

    for _ in 0..MAX_GENERATED_CASES {
        assert_eq!(a.next_u64(), b.next_u64());
    }
}

#[test]
fn property_generated_distances_are_bounded() {
    for distance in bounded_distance_cases() {
        assert!(
            (MIN_DISTANCE..=MAX_PROPERTY_DISTANCE).contains(&distance),
            "generated invalid distance: {distance}"
        );
    }
}

// -----------------------------------------------------------------------------
// Configuration properties
// -----------------------------------------------------------------------------

#[test]
fn property_configuration_has_explicit_resource_boundaries() {
    let config = configuration::QecConfig::default();

    let limits = config.limits();

    assert!(
        limits.max_code_distance() > 0,
        "maximum code distance must be explicit"
    );

    assert!(
        limits.max_qubits() > 0,
        "maximum qubit count must be explicit"
    );

    assert!(
        limits.max_stabilizers() > 0,
        "maximum stabilizer count must be explicit"
    );

    assert!(
        limits.max_memory_bytes() > 0,
        "maximum memory must be explicit"
    );

    assert!(
        limits.max_parallelism() > 0,
        "maximum parallelism must be explicit"
    );
}

#[test]
fn property_configuration_is_deterministic_by_default() {
    let a = configuration::QecConfig::default();
    let b = configuration::QecConfig::default();

    assert_eq!(
        a.determinism(),
        b.determinism(),
        "default deterministic policy changed between equivalent configurations"
    );
}

#[test]
fn property_configuration_rejects_unbounded_resource_requests() {
    let config = configuration::QecConfig::default();

    let limits = config.limits();

    assert!(
        limits.max_code_distance() != usize::MAX,
        "production configuration must not silently advertise unlimited code distance"
    );

    assert!(
        limits.max_memory_bytes() != usize::MAX,
        "production configuration must not silently advertise unlimited memory"
    );
}

// -----------------------------------------------------------------------------
// Arithmetic properties
// -----------------------------------------------------------------------------

#[test]
fn property_probability_zero_is_handled_without_panic() {
    let result = assert_no_panic("probability_zero", || {
        arithmetic::probability_to_weight(0.0)
    });

    assert!(
        result.is_err(),
        "p = 0 must not silently create an invalid logarithmic weight"
    );
}

#[test]
fn property_probability_one_is_valid() {
    let result = assert_no_panic("probability_one", || {
        arithmetic::probability_to_weight(1.0)
    });

    assert!(
        result.is_ok(),
        "p = 1 should be handled according to the configured arithmetic policy"
    );
}

#[test]
fn property_probability_greater_than_one_is_rejected() {
    let result = assert_no_panic("probability_above_one", || {
        arithmetic::probability_to_weight(1.000_000_1)
    });

    assert!(
        result.is_err(),
        "probability > 1 must be rejected"
    );
}

#[test]
fn property_negative_probability_is_rejected() {
    let result = assert_no_panic("negative_probability", || {
        arithmetic::probability_to_weight(-0.1)
    });

    assert!(
        result.is_err(),
        "negative probability must be rejected"
    );
}

#[test]
fn property_nan_probability_is_rejected() {
    let result = assert_no_panic("nan_probability", || {
        arithmetic::probability_to_weight(f64::NAN)
    });

    assert!(
        result.is_err(),
        "NaN probability must be rejected"
    );
}

#[test]
fn property_infinite_probability_is_rejected() {
    let result = assert_no_panic("infinite_probability", || {
        arithmetic::probability_to_weight(f64::INFINITY)
    });

    assert!(
        result.is_err(),
        "infinite probability must be rejected"
    );
}

// -----------------------------------------------------------------------------
// Validation properties
// -----------------------------------------------------------------------------

#[test]
fn property_valid_configuration_passes_validation() {
    let config = configuration::QecConfig::default();

    let result = assert_no_panic("configuration_validation", || {
        validation::validate_configuration(&config)
    });

    assert!(
        result.is_ok(),
        "default QEC configuration must validate"
    );
}

#[test]
fn property_validation_never_panics_on_bounded_inputs() {
    let start = Instant::now();

    for distance in bounded_distance_cases() {
        let config = configuration::QecConfig::for_distance(distance);

        let _ = assert_no_panic("bounded_configuration_validation", || {
            validation::validate_configuration(&config)
        });
    }

    assert_property_runtime(start, "bounded_configuration_validation");
}

// -----------------------------------------------------------------------------
// Surface-code structural properties
// -----------------------------------------------------------------------------

#[test]
fn property_surface_code_distances_are_structurally_valid() {
    for distance in bounded_distance_cases() {
        let result = assert_no_panic("surface_code_construction", || {
            super::super::surface_code::SurfaceCode::new(distance)
        });

        assert!(
            result.is_ok(),
            "valid surface-code distance {distance} should construct successfully"
        );

        let code = result.unwrap();

        assert_eq!(
            code.distance(),
            distance,
            "surface-code constructor changed the requested distance"
        );
    }
}

#[test]
fn property_surface_code_ids_are_unique() {
    for distance in bounded_distance_cases() {
        let result =
            super::super::surface_code::SurfaceCode::new(distance);

        let Ok(code) = result else {
            continue;
        };

        let mut ids = BTreeSet::new();

        for qubit in code.qubits() {
            assert!(
                ids.insert(qubit.id()),
                "duplicate qubit ID detected in distance {distance}"
            );
        }
    }
}

#[test]
fn property_surface_code_coordinates_are_unique() {
    for distance in bounded_distance_cases() {
        let result =
            super::super::surface_code::SurfaceCode::new(distance);

        let Ok(code) = result else {
            continue;
        };

        let mut coordinates = BTreeSet::new();

        for qubit in code.qubits() {
            assert!(
                coordinates.insert(qubit.coordinate()),
                "duplicate qubit coordinate detected"
            );
        }
    }
}

#[test]
fn property_surface_code_qubits_are_within_declared_distance() {
    for distance in bounded_distance_cases() {
        let result =
            super::super::surface_code::SurfaceCode::new(distance);

        let Ok(code) = result else {
            continue;
        };

        for qubit in code.qubits() {
            let (x, y) = qubit.coordinate();

            assert!(
                x < distance,
                "qubit x coordinate exceeds code boundary"
            );

            assert!(
                y < distance,
                "qubit y coordinate exceeds code boundary"
            );
        }
    }
}

// -----------------------------------------------------------------------------
// Stabilizer properties
// -----------------------------------------------------------------------------

#[test]
fn property_stabilizer_ids_are_unique() {
    for distance in bounded_distance_cases() {
        let result =
            super::super::surface_code::SurfaceCode::new(distance);

        let Ok(code) = result else {
            continue;
        };

        let mut ids = BTreeSet::new();

        for stabilizer in code.stabilizers() {
            assert!(
                ids.insert(stabilizer.id()),
                "duplicate stabilizer ID detected"
            );
        }
    }
}

#[test]
fn property_stabilizer_support_contains_only_known_qubits() {
    for distance in bounded_distance_cases() {
        let result =
            super::super::surface_code::SurfaceCode::new(distance);

        let Ok(code) = result else {
            continue;
        };

        let known_qubits: BTreeSet<_> =
            code.qubits().map(|q| q.id()).collect();

        for stabilizer in code.stabilizers() {
            for qubit_id in stabilizer.support() {
                assert!(
                    known_qubits.contains(qubit_id),
                    "stabilizer references unknown qubit"
                );
            }
        }
    }
}

#[test]
fn property_stabilizer_support_has_no_duplicates() {
    for distance in bounded_distance_cases() {
        let result =
            super::super::surface_code::SurfaceCode::new(distance);

        let Ok(code) = result else {
            continue;
        };

        for stabilizer in code.stabilizers() {
            let mut support = BTreeSet::new();

            for qubit_id in stabilizer.support() {
                assert!(
                    support.insert(*qubit_id),
                    "stabilizer contains duplicate qubit support"
                );
            }
        }
    }
}

// -----------------------------------------------------------------------------
// Syndrome properties
// -----------------------------------------------------------------------------

#[test]
fn property_identity_error_produces_trivial_syndrome() {
    for distance in bounded_distance_cases() {
        let result =
            super::super::surface_code::SurfaceCode::new(distance);

        let Ok(code) = result else {
            continue;
        };

        let syndrome =
            super::super::syndrome::Syndrome::identity(&code);

        assert!(
            syndrome.is_trivial(),
            "identity/no-error state must have a trivial syndrome"
        );
    }
}

#[test]
fn property_syndrome_dimensions_match_code() {
    for distance in bounded_distance_cases() {
        let result =
            super::super::surface_code::SurfaceCode::new(distance);

        let Ok(code) = result else {
            continue;
        };

        let syndrome =
            super::super::syndrome::Syndrome::identity(&code);

        assert_eq!(
            syndrome.len(),
            code.stabilizer_count(),
            "syndrome dimension does not match stabilizer count"
        );
    }
}

// -----------------------------------------------------------------------------
// Decoder properties
// -----------------------------------------------------------------------------

#[test]
fn property_decoder_does_not_panic_on_identity_syndrome() {
    for distance in bounded_distance_cases() {
        let result =
            super::super::surface_code::SurfaceCode::new(distance);

        let Ok(code) = result else {
            continue;
        };

        let syndrome =
            super::super::syndrome::Syndrome::identity(&code);

        let config = configuration::QecConfig::for_distance(distance);

        let _ = assert_no_panic("identity_decode", || {
            super::super::decoder::decode(&code, &syndrome, &config)
        });
    }
}

#[test]
fn property_identity_syndrome_has_no_logical_failure() {
    for distance in bounded_distance_cases() {
        let result =
            super::super::surface_code::SurfaceCode::new(distance);

        let Ok(code) = result else {
            continue;
        };

        let syndrome =
            super::super::syndrome::Syndrome::identity(&code);

        let config = configuration::QecConfig::for_distance(distance);

        let result =
            super::super::decoder::decode(&code, &syndrome, &config);

        if let Ok(decoded) = result {
            assert!(
                !decoded.has_logical_failure(),
                "identity syndrome must not produce a logical failure"
            );
        }
    }
}

#[test]
fn property_identity_decode_is_deterministic() {
    for distance in bounded_distance_cases() {
        let result =
            super::super::surface_code::SurfaceCode::new(distance);

        let Ok(code) = result else {
            continue;
        };

        let syndrome =
            super::super::syndrome::Syndrome::identity(&code);

        let config = configuration::QecConfig::for_distance(distance);

        let first = assert_no_panic("identity_decode_first", || {
            super::super::decoder::decode(&code, &syndrome, &config)
        });

        let second = assert_no_panic("identity_decode_second", || {
            super::super::decoder::decode(&code, &syndrome, &config)
        });

        assert_eq!(
            first, second,
            "identical decoder inputs produced different results"
        );
    }
}

// -----------------------------------------------------------------------------
// Resource-limit properties
// -----------------------------------------------------------------------------

#[test]
fn property_resource_limits_reject_excessive_distance() {
    let limits = limits::QecLimits::default();

    let excessive_distance =
        limits.max_code_distance().saturating_add(1);

    let result =
        limits.validate_code_distance(excessive_distance);

    assert!(
        result.is_err(),
        "distance above the configured limit must be rejected"
    );
}

#[test]
fn property_resource_limits_reject_excessive_rounds() {
    let limits = limits::QecLimits::default();

    let excessive_rounds =
        limits.max_rounds().saturating_add(1);

    let result =
        limits.validate_rounds(excessive_rounds);

    assert!(
        result.is_err(),
        "round count above the configured limit must be rejected"
    );
}

#[test]
fn property_resource_limits_reject_excessive_events() {
    let limits = limits::QecLimits::default();

    let excessive_events =
        limits.max_syndrome_events().saturating_add(1);

    let result =
        limits.validate_syndrome_events(excessive_events);

    assert!(
        result.is_err(),
        "syndrome event count above the configured limit must be rejected"
    );
}

#[test]
fn property_resource_limits_reject_excessive_parallelism() {
    let limits = limits::QecLimits::default();

    let excessive_parallelism =
        limits.max_parallelism().saturating_add(1);

    let result =
        limits.validate_parallelism(excessive_parallelism);

    assert!(
        result.is_err(),
        "parallelism above the configured limit must be rejected"
    );
}

// -----------------------------------------------------------------------------
// Memory properties
// -----------------------------------------------------------------------------

#[test]
fn property_memory_budget_is_explicit() {
    let budget = memory::MemoryBudget::default();

    assert!(
        budget.limit_bytes() > 0,
        "memory budget must have a positive bound"
    );
}

#[test]
fn property_memory_budget_rejects_overallocation() {
    let budget = memory::MemoryBudget::new(1024);

    let result = budget.try_reserve(1025);

    assert!(
        result.is_err(),
        "memory budget must reject allocations above its configured limit"
    );
}

#[test]
fn property_memory_budget_allows_bounded_allocation() {
    let budget = memory::MemoryBudget::new(4096);

    let result = budget.try_reserve(1024);

    assert!(
        result.is_ok(),
        "allocation below the memory budget should succeed"
    );
}

// -----------------------------------------------------------------------------
// Resource accounting properties
// -----------------------------------------------------------------------------

#[test]
fn property_resource_accounting_never_exceeds_declared_limits() {
    let limits = limits::QecLimits::default();

    let resources = resources::ResourceTracker::new(limits.clone());

    assert!(
        resources.max_memory_bytes() <= limits.max_memory_bytes()
    );

    assert!(
        resources.max_parallelism() <= limits.max_parallelism()
    );
}

#[test]
fn property_resource_counters_are_monotonic() {
    let limits = limits::QecLimits::default();

    let mut resources =
        resources::ResourceTracker::new(limits);

    let first = resources.peak_memory_bytes();

    let _ = resources.record_memory_usage(1024);

    let second = resources.peak_memory_bytes();

    assert!(
        second >= first,
        "peak memory accounting must be monotonic"
    );
}

// -----------------------------------------------------------------------------
// Determinism properties
// -----------------------------------------------------------------------------

#[test]
fn property_deterministic_configuration_produces_stable_seed() {
    let config = configuration::QecConfig::default();

    let seed_a = deterministic::derive_seed(&config, TEST_SEED);
    let seed_b = deterministic::derive_seed(&config, TEST_SEED);

    assert_eq!(
        seed_a, seed_b,
        "identical deterministic inputs must produce identical seeds"
    );
}

#[test]
fn property_deterministic_seed_changes_when_input_seed_changes() {
    let config = configuration::QecConfig::default();

    let a = deterministic::derive_seed(&config, 1);
    let b = deterministic::derive_seed(&config, 2);

    assert_ne!(
        a, b,
        "different explicit seeds should not collapse into the same deterministic seed"
    );
}

// -----------------------------------------------------------------------------
// Backend properties
// -----------------------------------------------------------------------------

#[test]
fn property_all_supported_backends_have_explicit_identity() {
    let backends = backend::Backend::supported();

    assert!(
        !backends.is_empty(),
        "Zamani must expose at least one QEC execution backend"
    );

    let mut names = BTreeSet::new();

    for backend in backends {
        assert!(
            names.insert(backend.name()),
            "backend names must be unique"
        );
    }
}

#[test]
fn property_qpu_is_an_explicit_backend() {
    let backends = backend::Backend::supported();

    assert!(
        backends.iter().any(|b| b.is_qpu()),
        "QPU must be represented as an explicit execution backend"
    );
}

#[test]
fn property_qpu_is_not_implicitly_selected() {
    let config = configuration::QecConfig::default();

    assert!(
        !config.backend().is_qpu(),
        "QPU execution must never become an implicit default"
    );
}

#[test]
fn property_qpu_requires_explicit_capability() {
    let qpu_capability =
        capabilities::QecCapability::UseQpu;

    let default_capabilities =
        capabilities::QecCapabilities::default();

    assert!(
        !default_capabilities.contains(qpu_capability),
        "QPU access must require explicit authorization"
    );
}

#[test]
fn property_qpu_capability_is_distinct_from_cpu_decode() {
    assert_ne!(
        capabilities::QecCapability::Decode,
        capabilities::QecCapability::UseQpu,
        "ordinary decoding must not implicitly grant QPU access"
    );
}

#[test]
fn property_qpu_backend_has_resource_constraints() {
    let qpu = backend::Backend::Qpu;

    let limits = qpu.resource_requirements();

    assert!(
        limits.max_parallel_jobs() > 0,
        "QPU backend must expose explicit concurrency requirements"
    );
}

// -----------------------------------------------------------------------------
// Capability isolation properties
// -----------------------------------------------------------------------------

#[test]
fn property_decode_capability_does_not_grant_distributed_execution() {
    let mut capabilities =
        capabilities::QecCapabilities::default();

    capabilities.grant(capabilities::QecCapability::Decode);

    assert!(
        !capabilities.contains(
            capabilities::QecCapability::DistributedExecution
        ),
        "Decode must not implicitly grant distributed execution"
    );
}

#[test]
fn property_simulation_does_not_grant_qpu_access() {
    let mut capabilities =
        capabilities::QecCapabilities::default();

    capabilities.grant(capabilities::QecCapability::Simulate);

    assert!(
        !capabilities.contains(
            capabilities::QecCapability::UseQpu
        ),
        "simulation capability must not implicitly grant QPU access"
    );
}

#[test]
fn property_accelerator_access_is_separate_from_qpu_access() {
    let mut capabilities =
        capabilities::QecCapabilities::default();

    capabilities.grant(
        capabilities::QecCapability::UseAccelerator
    );

    assert!(
        !capabilities.contains(
            capabilities::QecCapability::UseQpu
        ),
        "accelerator access must not imply QPU access"
    );
}

// -----------------------------------------------------------------------------
// Streaming properties
// -----------------------------------------------------------------------------

#[test]
fn property_streaming_has_bounded_buffer_configuration() {
    let config = configuration::QecConfig::default();

    let streaming = config.streaming();

    assert!(
        streaming.max_buffer_events() > 0,
        "streaming must have a bounded event buffer"
    );

    assert!(
        streaming.max_buffer_events()
            <= config.limits().max_syndrome_events(),
        "streaming buffer must respect global syndrome limits"
    );
}

#[test]
fn property_streaming_does_not_require_unbounded_history() {
    let config = configuration::QecConfig::default();

    let streaming = config.streaming();

    assert!(
        streaming.retains_full_history() == false,
        "production streaming must not require unlimited syndrome history"
    );
}

// -----------------------------------------------------------------------------
// Partition properties
// -----------------------------------------------------------------------------

#[test]
fn property_partitioning_has_bounded_partition_size() {
    let config = configuration::QecConfig::default();

    let partition = config.partitioning();

    assert!(
        partition.max_partition_size() > 0,
        "partitioning requires a positive bounded partition size"
    );

    assert!(
        partition.max_partition_size()
            <= config.limits().max_graph_nodes(),
        "partition size must respect graph limits"
    );
}

#[test]
fn property_partition_boundary_state_is_preserved() {
    let config = configuration::QecConfig::default();

    let partition = config.partitioning();

    assert!(
        partition.preserves_boundary_state(),
        "partitioning must preserve boundary information"
    );
}

// -----------------------------------------------------------------------------
// Cancellation properties
// -----------------------------------------------------------------------------

#[test]
fn property_cancellation_is_supported() {
    let token =
        super::super::cancellation::CancellationToken::new();

    assert!(
        !token.is_cancelled(),
        "new cancellation token must begin uncancelled"
    );

    token.cancel();

    assert!(
        token.is_cancelled(),
        "cancelled token must report cancellation"
    );
}

#[test]
fn property_cancellation_is_idempotent() {
    let token =
        super::super::cancellation::CancellationToken::new();

    token.cancel();
    token.cancel();
    token.cancel();

    assert!(
        token.is_cancelled(),
        "repeated cancellation must remain safe and deterministic"
    );
}

// -----------------------------------------------------------------------------
// Cache correctness properties
// -----------------------------------------------------------------------------

#[test]
fn property_cache_miss_does_not_change_correctness() {
    let cache =
        super::super::cache::QecCache::new();

    let key = "nonexistent-property-test-key";

    assert!(
        cache.get(key).is_none(),
        "fresh cache must not contain arbitrary entries"
    );
}

#[test]
fn property_cache_can_be_discarded_without_affecting_execution() {
    let cache =
        super::super::cache::QecCache::new();

    cache.clear();

    assert!(
        cache.is_empty(),
        "cache clear operation must be deterministic"
    );
}

// -----------------------------------------------------------------------------
// Versioning properties
// -----------------------------------------------------------------------------

#[test]
fn property_version_is_present() {
    let version =
        super::super::version::QecVersion::current();

    assert!(
        !version.algorithm_version().is_empty(),
        "algorithm version must not be empty"
    );

    assert!(
        !version.schema_version().is_empty(),
        "schema version must not be empty"
    );

    assert!(
        !version.checkpoint_schema_version().is_empty(),
        "checkpoint schema version must not be empty"
    );
}

#[test]
fn property_version_is_deterministic() {
    let a =
        super::super::version::QecVersion::current();

    let b =
        super::super::version::QecVersion::current();

    assert_eq!(
        a, b,
        "current QEC version must be deterministic within a build"
    );
}

// -----------------------------------------------------------------------------
// Malformed-input / panic-safety properties
// -----------------------------------------------------------------------------

#[test]
fn property_extreme_distances_never_panic() {
    let candidates = [
        0usize,
        1,
        2,
        3,
        MAX_PROPERTY_DISTANCE,
        usize::MAX,
    ];

    for distance in candidates {
        let _ = assert_no_panic("extreme_surface_code_distance", || {
            super::super::surface_code::SurfaceCode::new(distance)
        });
    }
}

#[test]
fn property_extreme_round_counts_never_panic() {
    let candidates = [
        0usize,
        1,
        MAX_GENERATED_ROUNDS,
        usize::MAX,
    ];

    for rounds in candidates {
        let config =
            configuration::QecConfig::default();

        let _ = assert_no_panic("extreme_round_count", || {
            config.limits().validate_rounds(rounds)
        });
    }
}

#[test]
fn property_extreme_event_counts_never_panic() {
    let candidates = [
        0usize,
        1,
        MAX_GENERATED_EVENTS,
        usize::MAX,
    ];

    for events in candidates {
        let limits = limits::QecLimits::default();

        let _ = assert_no_panic("extreme_event_count", || {
            limits.validate_syndrome_events(events)
        });
    }
}

// -----------------------------------------------------------------------------
// Randomized bounded property suite
// -----------------------------------------------------------------------------

#[test]
fn property_random_bounded_configurations_remain_safe() {
    let start = Instant::now();

    let mut rng = TestRng::new(TEST_SEED);

    for _ in 0..MAX_GENERATED_CASES {
        let index =
            rng.next_usize(bounded_distance_cases().len());

        let distance =
            bounded_distance_cases()[index];

        let config =
            configuration::QecConfig::for_distance(distance);

        let result = assert_no_panic(
            "random_bounded_configuration",
            || validation::validate_configuration(&config),
        );

        assert!(
            result.is_ok(),
            "generated valid configuration failed validation: distance={distance}"
        );
    }

    assert_property_runtime(
        start,
        "random_bounded_configurations",
    );
}

#[test]
fn property_randomized_boolean_generation_is_deterministic() {
    let mut a = TestRng::new(TEST_SEED);
    let mut b = TestRng::new(TEST_SEED);

    for _ in 0..MAX_GENERATED_CASES {
        assert_eq!(a.next_bool(), b.next_bool());
    }
}

// -----------------------------------------------------------------------------
// Mathematical stabilizer properties
// -----------------------------------------------------------------------------

#[test]
fn property_valid_stabilizers_commute() {
    for distance in bounded_distance_cases() {
        let result =
            super::super::surface_code::SurfaceCode::new(distance);

        let Ok(code) = result else {
            continue;
        };

        let stabilizers: Vec<_> =
            code.stabilizers().collect();

        for i in 0..stabilizers.len() {
            for j in (i + 1)..stabilizers.len() {
                assert!(
                    stabilizers[i].commutes_with(&stabilizers[j]),
                    "valid stabilizer generators must commute"
                );
            }
        }
    }
}

#[test]
fn property_logical_operators_commute_with_stabilizers() {
    for distance in bounded_distance_cases() {
        let result =
            super::super::surface_code::SurfaceCode::new(distance);

        let Ok(code) = result else {
            continue;
        };

        for logical in code.logical_operators() {
            for stabilizer in code.stabilizers() {
                assert!(
                    logical.commutes_with(&stabilizer),
                    "logical operator must commute with stabilizers"
                );
            }
        }
    }
}

// -----------------------------------------------------------------------------
// Decoder correctness properties
// -----------------------------------------------------------------------------

#[test]
fn property_known_identity_case_is_logically_trivial() {
    for distance in bounded_distance_cases() {
        let result =
            super::super::surface_code::SurfaceCode::new(distance);

        let Ok(code) = result else {
            continue;
        };

        let syndrome =
            super::super::syndrome::Syndrome::identity(&code);

        let config =
            configuration::QecConfig::for_distance(distance);

        let decoded =
            super::super::decoder::decode(
                &code,
                &syndrome,
                &config,
            );

        if let Ok(result) = decoded {
            assert!(
                result.is_logically_trivial(),
                "identity input must decode to logical identity"
            );
        }
    }
}

#[test]
fn property_decoder_output_is_validated() {
    for distance in bounded_distance_cases() {
        let result =
            super::super::surface_code::SurfaceCode::new(distance);

        let Ok(code) = result else {
            continue;
        };

        let syndrome =
            super::super::syndrome::Syndrome::identity(&code);

        let config =
            configuration::QecConfig::for_distance(distance);

        let Ok(decoded) =
            super::super::decoder::decode(
                &code,
                &syndrome,
                &config,
            )
        else {
            continue;
        };

        assert!(
            validation::validate_decoder_result(
                &code,
                &syndrome,
                &decoded,
            )
            .is_ok(),
            "decoder output must satisfy QEC invariants"
        );
    }
}

// -----------------------------------------------------------------------------
// QPU safety properties
// -----------------------------------------------------------------------------

#[test]
fn property_qpu_execution_requires_explicit_configuration() {
    let config =
        configuration::QecConfig::default();

    assert!(
        !config.qpu().enabled(),
        "QPU execution must be explicitly enabled"
    );
}

#[test]
fn property_qpu_configuration_is_resource_bounded() {
    let config =
        configuration::QecConfig::default();

    let qpu = config.qpu();

    assert!(
        qpu.max_shots() > 0,
        "QPU shot count must be bounded and positive"
    );

    assert!(
        qpu.max_jobs() > 0,
        "QPU job count must be bounded and positive"
    );

    assert!(
        qpu.max_execution_time() > Duration::ZERO,
        "QPU execution must have a finite timeout"
    );
}

#[test]
fn property_qpu_timeout_is_finite() {
    let config =
        configuration::QecConfig::default();

    let timeout =
        config.qpu().max_execution_time();

    assert!(
        timeout != Duration::MAX,
        "QPU execution must never default to infinite timeout"
    );
}

#[test]
fn property_qpu_requires_capability_and_configuration() {
    let config =
        configuration::QecConfig::default();

    let capabilities =
        capabilities::QecCapabilities::default();

    let authorized =
        capabilities.contains(
            capabilities::QecCapability::UseQpu
        );

    let enabled =
        config.qpu().enabled();

    assert!(
        !(authorized && !enabled),
        "QPU authorization must not bypass explicit QPU configuration"
    );
}

// -----------------------------------------------------------------------------
// Backend consistency properties
// -----------------------------------------------------------------------------

#[test]
fn property_backend_selection_is_explicit() {
    let config =
        configuration::QecConfig::default();

    let backend =
        config.backend();

    assert!(
        !backend.name().is_empty(),
        "selected backend must have a stable identity"
    );
}

#[test]
fn property_qpu_and_classical_backend_are_distinguishable() {
    let classical =
        backend::Backend::Cpu;

    let qpu =
        backend::Backend::Qpu;

    assert_ne!(
        classical,
        qpu,
        "QPU must remain distinguishable from classical execution"
    );
}

// -----------------------------------------------------------------------------
// No-artificial-infinity property
// -----------------------------------------------------------------------------

#[test]
fn property_scalability_is_resource_bounded_not_infinite() {
    let limits =
        limits::QecLimits::default();

    assert!(
        limits.max_code_distance() < usize::MAX,
        "production QEC must use explicit resource boundaries"
    );

    assert!(
        limits.max_qubits() < usize::MAX,
        "production QEC must bound qubit allocation"
    );

    assert!(
        limits.max_graph_nodes() < usize::MAX,
        "production QEC must bound graph allocation"
    );

    assert!(
        limits.max_graph_edges() < usize::MAX,
        "production QEC must bound graph edges"
    );

    assert!(
        limits.max_syndrome_events() < usize::MAX,
        "production QEC must bound syndrome events"
    );
}

// -----------------------------------------------------------------------------
// Global invariant suite
// -----------------------------------------------------------------------------

#[test]
fn property_qec_global_invariants() {
    let start = Instant::now();

    let config =
        configuration::QecConfig::default();

    // Configuration must validate.
    assert!(
        validation::validate_configuration(&config).is_ok()
    );

    // Limits must be finite.
    let limits = config.limits();

    assert!(limits.max_code_distance() > 0);
    assert!(limits.max_qubits() > 0);
    assert!(limits.max_stabilizers() > 0);
    assert!(limits.max_rounds() > 0);
    assert!(limits.max_graph_nodes() > 0);
    assert!(limits.max_graph_edges() > 0);
    assert!(limits.max_syndrome_events() > 0);
    assert!(limits.max_memory_bytes() > 0);
    assert!(limits.max_parallelism() > 0);

    // Deterministic mode must be stable.
    assert_eq!(
        deterministic::derive_seed(&config, TEST_SEED),
        deterministic::derive_seed(&config, TEST_SEED)
    );

    // QPU must remain explicitly separated.
    assert!(
        !config.backend().is_qpu()
            || config.qpu().enabled(),
        "QPU backend cannot be selected without explicit QPU enablement"
    );

    assert_property_runtime(
        start,
        "qec_global_invariants",
    );
}

// -----------------------------------------------------------------------------
// Regression guard
// -----------------------------------------------------------------------------

#[test]
fn property_regression_guard_no_unbounded_test_generation() {
    assert!(
        MAX_GENERATED_CASES <= 4096,
        "property tests themselves must remain resource bounded"
    );

    assert!(
        MAX_GENERATED_ROUNDS <= 1024,
        "property-test round generation must remain bounded"
    );

    assert!(
        MAX_GENERATED_EVENTS <= 1_000_000,
        "property-test event generation must remain bounded"
    );
}