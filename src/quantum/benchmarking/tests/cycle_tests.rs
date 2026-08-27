//! Zamani Quantum Benchmarking — Cycle Benchmarking Integration Tests
//!
//! Production integration tests for:
//!
//!     src/quantum/benchmarking/protocols/cycle_benchmarking.rs
//!
//! # Purpose
//!
//! This file verifies the stable public contracts of the Cycle Benchmarking
//! subsystem without depending on:
//!
//! - private implementation details;
//! - a real quantum processor;
//! - network access;
//! - cloud credentials;
//! - a particular simulator;
//! - hardware timing;
//! - operating-system randomness;
//! - machine-specific floating-point formatting.
//!
//! The tests exercise the complete backend-independent path:
//!
//! ```text
//! CycleBenchmarkConfig
//!        │
//!        ▼
//! CycleDefinition
//!        │
//!        ▼
//! CycleBenchmarkProtocol
//!        │
//!        ▼
//! deterministic experiment generation
//!        │
//!        ▼
//! CycleBenchmarkInstance
//!        │
//!        ▼
//! CycleExecutionRequest
//!        │
//!        ▼
//! CycleBenchmarkExecutor adapter
//!        │
//!        ▼
//! CycleExecutionObservation
//!        │
//!        ▼
//! CycleBenchmarkProtocol::analyze
//!        │
//!        ▼
//! CycleBenchmarkProtocolResult
//!        │
//!        ▼
//! CompositeCycleFidelity
//! ```
//!
//! # Production invariants
//!
//! These tests verify:
//!
//! - configuration validation;
//! - resource-limit validation;
//! - strictly increasing sequence lengths;
//! - periodic-cycle requirements;
//! - Pauli-width validation;
//! - unique non-identity Pauli selection;
//! - deterministic random generation;
//! - generated instance cardinality;
//! - `m + 1` random Pauli cycles;
//! - stable instance identifiers;
//! - execution-request validation;
//! - observation validation;
//! - matching-probability calculation;
//! - expectation-value calculation;
//! - executor integration;
//! - executor result validation;
//! - complete experiment analysis;
//! - missing-observation rejection;
//! - duplicate-observation rejection;
//! - deterministic mathematical analysis;
//! - exponential-decay fitting;
//! - fit diagnostics;
//! - process-fidelity calculation;
//! - average-gate-fidelity calculation;
//! - sampled versus exhaustive Pauli characterization;
//! - timing aggregation;
//! - shot/circuit accounting;
//! - scientific assumption metadata;
//! - public protocol identity/version metadata;
//! - rejection of invalid numerical observations;
//! - boundary conditions;
//! - reproducibility.
//!
//! # Architectural rule
//!
//! This test file intentionally consumes the public API of
//! `protocols::cycle_benchmarking`.
//!
//! It must not access private helpers such as:
//!
//! - `fit_decay`;
//! - `optimize_decay_parameter`;
//! - `linear_parameters_for_decay`;
//! - `safe_power`;
//! - `hilbert_dimension`;
//! - `checked_non_identity_pauli_count`;
//! - `validate_fidelity`.
//!
//! Those implementation details are already covered by the protocol's own
//! unit tests. Integration tests verify behavior visible to other modules.
//!
//! # Integration contract
//!
//! The tested dependency direction is:
//!
//! ```text
//! tests/cycle_tests.rs
//!        │
//!        ▼
//! protocols/cycle_benchmarking.rs
//!        │
//!        ├──── generators/random.rs
//!        └──── generators/pauli.rs
//!
//! Future integration:
//!
//! protocols/cycle_benchmarking.rs
//!        │
//!        ├──── execution adapter
//!        ├──── quantum::ir
//!        ├──── quantum::hardware
//!        ├──── statistics
//!        ├──── metrics
//!        ├──── reporting
//!        └──── registry
//! ```
//!
//! No changes to those future modules are required for this test file.
//!
//! # Hardware policy
//!
//! No real hardware is contacted by these tests.
//!
//! Hardware execution belongs to the extended benchmark tiers.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97
//! - Rust 1.97.1
//! - Rust 2021
//!
//! No nightly features are required.
//! No unsafe code is used.

#![deny(unsafe_code)]

use std::error::Error;
use std::fmt;

use crate::quantum::benchmarking::generators::pauli::Pauli;
use crate::quantum::benchmarking::generators::random::{
    BenchmarkSeed,
    RandomStream,
};
use crate::quantum::benchmarking::protocols::cycle_benchmarking::{
    select_random_paulis,
    CycleBenchmarkConfig,
    CycleBenchmarkError,
    CycleBenchmarkExecutor,
    CycleBenchmarkInstance,
    CycleBenchmarkProtocol,
    CycleBenchmarkProtocolResult,
    CycleDecayPoint,
    CycleDefinition,
    CycleExecutionObservation,
    CycleExecutionRequest,
    CycleFitModel,
    PauliFrame,
    CYCLE_BENCHMARK_CIRCUIT_CONVENTION,
    CYCLE_BENCHMARK_FIT_MODEL,
    CYCLE_BENCHMARK_ID,
    CYCLE_BENCHMARK_PROTOCOL_VERSION,
};

// =============================================================================
// Test constants
// =============================================================================

const TEST_SEED: u64 = 0xCB_2026_08_27;

const TEST_QUBITS: usize = 1;

const TEST_SEQUENCE_LENGTHS: &[usize] = &[1, 2, 4, 8];

const TEST_PAULI_COUNT: usize = 3;

const TEST_RANDOMIZATIONS: usize = 2;

const TEST_SHOTS: usize = 1_000;

// =============================================================================
// Test helpers
// =============================================================================

/// Creates the canonical deterministic benchmark RNG used by this test suite.
///
/// The production random subsystem explicitly requires benchmark generators
/// to receive an explicit deterministic random source rather than relying on
/// process-global randomness.
fn test_rng() -> RandomStream {
    RandomStream::from_seed(BenchmarkSeed::from_u64(TEST_SEED))
}

/// Creates a small but complete one-qubit CB configuration.
///
/// One qubit has exactly three non-identity Pauli operators:
///
///     X, Y, Z
///
/// Therefore `pauli_count = 3` gives an exhaustive non-identity Pauli basis.
fn test_config() -> CycleBenchmarkConfig {
    CycleBenchmarkConfig {
        qubits: TEST_QUBITS,
        sequence_lengths: TEST_SEQUENCE_LENGTHS.to_vec(),
        pauli_count: TEST_PAULI_COUNT,
        randomizations_per_length: TEST_RANDOMIZATIONS,
        shots: TEST_SHOTS,
        confidence_level: 0.95,
        fit_model: CycleFitModel::NoOffset,
        include_identity_pauli: false,
        require_periodic_lengths: true,
        ..CycleBenchmarkConfig::default()
    }
}

/// Creates a valid cycle with an explicitly verified identity period.
///
/// Periodic sequence lengths are required by the default production
/// configuration, so the cycle must carry an explicit verified period.
fn test_cycle() -> CycleDefinition {
    CycleDefinition::new("test-cycle", TEST_QUBITS)
        .expect("test cycle must be constructible")
        .with_operation_count(1)
        .with_two_qubit_operation_count(0)
        .with_depth(1)
        .expect("cycle depth must be valid")
        .with_identity_period(1)
        .expect("identity period must be valid")
}

/// Generates the canonical test experiment.
fn test_experiment() -> crate::quantum::benchmarking::protocols::cycle_benchmarking::CycleBenchmarkExperiment
{
    let protocol = CycleBenchmarkProtocol::new(test_config())
        .expect("test protocol configuration must be valid");

    let mut rng = test_rng();

    protocol
        .generate(test_cycle(), &mut rng)
        .expect("test experiment generation must succeed")
}

/// Asserts that two floating-point values are sufficiently close.
fn assert_close(actual: f64, expected: f64, tolerance: f64) {
    let difference = (actual - expected).abs();

    assert!(
        difference <= tolerance,
        "expected {expected:.17e}, got {actual:.17e}; \
         absolute difference {difference:.17e} exceeded tolerance \
         {tolerance:.17e}"
    );
}

/// Asserts that a value is finite.
fn assert_finite(value: f64, name: &str) {
    assert!(
        value.is_finite(),
        "{name} must be finite, got {value:?}"
    );
}

/// Asserts that a fidelity lies in the physical interval.
fn assert_fidelity(value: f64, name: &str) {
    assert_finite(value, name);

    assert!(
        (-1.0e-12..=1.0 + 1.0e-12).contains(&value),
        "{name} must be in [0, 1], got {value}"
    );
}

/// Constructs a perfect execution observation.
///
/// A perfect matching result gives expectation +1 and therefore creates a
/// mathematically ideal decay curve. This is useful for validating the full
/// protocol pipeline without a simulator.
fn perfect_observation(
    instance: &CycleBenchmarkInstance,
    shots: usize,
) -> CycleExecutionObservation {
    CycleExecutionObservation::new(
        instance.id.clone(),
        shots,
        shots,
    )
    .expect("perfect observation must be valid")
}

/// Constructs a deterministic mock executor that returns perfect results.
#[derive(Debug, Default)]
struct PerfectExecutor;

impl CycleBenchmarkExecutor for PerfectExecutor {
    type Error = TestExecutorError;

    fn execute(
        &mut self,
        request: &CycleExecutionRequest,
    ) -> Result<CycleExecutionObservation, Self::Error> {
        Ok(
            CycleExecutionObservation::new(
                request.instance.id.clone(),
                request.shots,
                request.shots,
            )
            .expect("executor-generated perfect observation must be valid"),
        )
    }
}

/// Executor error used by negative integration tests.
#[derive(Debug, Clone, PartialEq, Eq)]
struct TestExecutorError {
    message: &'static str,
}

impl fmt::Display for TestExecutorError {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        f.write_str(self.message)
    }
}

impl Error for TestExecutorError {}

/// Executor deliberately returning an incorrect instance identifier.
#[derive(Debug, Default)]
struct WrongIdExecutor;

impl CycleBenchmarkExecutor for WrongIdExecutor {
    type Error = TestExecutorError;

    fn execute(
        &mut self,
        request: &CycleExecutionRequest,
    ) -> Result<CycleExecutionObservation, Self::Error> {
        CycleExecutionObservation::new(
            "incorrect-instance-id",
            request.shots,
            request.shots,
        )
        .map_err(|_| TestExecutorError {
            message: "unable to construct invalid test observation",
        })
    }
}

/// Executor deliberately returning the wrong shot count.
#[derive(Debug, Default)]
struct WrongShotsExecutor;

impl CycleBenchmarkExecutor for WrongShotsExecutor {
    type Error = TestExecutorError;

    fn execute(
        &mut self,
        request: &CycleExecutionRequest,
    ) -> Result<CycleExecutionObservation, Self::Error> {
        CycleExecutionObservation::new(
            request.instance.id.clone(),
            request.shots.saturating_sub(1).max(1),
            request.shots.saturating_sub(1).max(1),
        )
        .map_err(|_| TestExecutorError {
            message: "unable to construct invalid test observation",
        })
    }
}

/// Executor deliberately returning a backend failure.
#[derive(Debug, Default)]
struct FailingExecutor;

impl CycleBenchmarkExecutor for FailingExecutor {
    type Error = TestExecutorError;

    fn execute(
        &mut self,
        _request: &CycleExecutionRequest,
    ) -> Result<CycleExecutionObservation, Self::Error> {
        Err(TestExecutorError {
            message: "synthetic backend failure",
        })
    }
}

// =============================================================================
// Public protocol identity
// =============================================================================

#[test]
fn cycle_benchmark_identity_is_stable() {
    assert_eq!(
        CYCLE_BENCHMARK_ID,
        "cycle_benchmarking"
    );

    assert_eq!(
        CYCLE_BENCHMARK_PROTOCOL_VERSION,
        "1.0.0"
    );

    assert_eq!(
        CYCLE_BENCHMARK_CIRCUIT_CONVENTION,
        "pauli-twirled-cycle-r_m-g-r_m-1-g-r_1"
            .replace("r_1", "r_0")
            .as_str()
    );

    assert_eq!(
        CYCLE_BENCHMARK_FIT_MODEL,
        "A*p^m"
    );
}

// =============================================================================
// Configuration validation
// =============================================================================

#[test]
fn valid_production_configuration_passes_validation() {
    let config = test_config();

    config
        .validate()
        .expect("valid production configuration must pass validation");

    assert_eq!(config.qubits, 1);
    assert_eq!(config.sequence_lengths, TEST_SEQUENCE_LENGTHS);
    assert_eq!(config.pauli_count, TEST_PAULI_COUNT);
    assert_eq!(
        config.randomizations_per_length,
        TEST_RANDOMIZATIONS
    );
    assert_eq!(config.shots, TEST_SHOTS);
}

#[test]
fn configuration_rejects_zero_qubits() {
    let config = CycleBenchmarkConfig {
        qubits: 0,
        ..test_config()
    };

    assert!(matches!(
        config.validate(),
        Err(CycleBenchmarkError::InvalidQubitCount {
            qubits: 0
        })
    ));
}

#[test]
fn configuration_rejects_zero_sequence_lengths() {
    let config = CycleBenchmarkConfig {
        sequence_lengths: Vec::new(),
        ..test_config()
    };

    assert!(matches!(
        config.validate(),
        Err(CycleBenchmarkError::InvalidConfiguration {
            field: "sequence_lengths",
            ..
        })
    ));
}

#[test]
fn configuration_rejects_zero_sequence_length() {
    let config = CycleBenchmarkConfig {
        sequence_lengths: vec![0, 1, 2],
        ..test_config()
    };

    assert!(matches!(
        config.validate(),
        Err(CycleBenchmarkError::InvalidSequenceLength {
            length: 0
        })
    ));
}

#[test]
fn configuration_rejects_non_increasing_sequence_lengths() {
    let config = CycleBenchmarkConfig {
        sequence_lengths: vec![1, 4, 4, 8],
        ..test_config()
    };

    assert!(matches!(
        config.validate(),
        Err(CycleBenchmarkError::NonIncreasingSequenceLengths)
    ));
}

#[test]
fn configuration_rejects_zero_pauli_count() {
    let config = CycleBenchmarkConfig {
        pauli_count: 0,
        ..test_config()
    };

    assert!(matches!(
        config.validate(),
        Err(CycleBenchmarkError::InvalidConfiguration {
            field: "pauli_count",
            ..
        })
    ));
}

#[test]
fn configuration_rejects_zero_randomizations() {
    let config = CycleBenchmarkConfig {
        randomizations_per_length: 0,
        ..test_config()
    };

    assert!(matches!(
        config.validate(),
        Err(CycleBenchmarkError::InvalidConfiguration {
            field: "randomizations_per_length",
            ..
        })
    ));
}

#[test]
fn configuration_rejects_zero_shots() {
    let config = CycleBenchmarkConfig {
        shots: 0,
        ..test_config()
    };

    assert!(matches!(
        config.validate(),
        Err(CycleBenchmarkError::InvalidConfiguration {
            field: "shots",
            ..
        })
    ));
}

#[test]
fn configuration_rejects_invalid_confidence_level() {
    for confidence_level in [
        f64::NAN,
        f64::NEG_INFINITY,
        f64::INFINITY,
        0.0,
        1.0,
        -0.1,
        1.1,
    ] {
        let config = CycleBenchmarkConfig {
            confidence_level,
            ..test_config()
        };

        assert!(
            config.validate().is_err(),
            "invalid confidence level {confidence_level:?} must be rejected"
        );
    }
}

#[test]
fn configuration_enforces_instance_limit() {
    let config = CycleBenchmarkConfig {
        pauli_count: 3,
        sequence_lengths: vec![1, 2, 4],
        randomizations_per_length: 10,
        max_instances: 10,
        ..test_config()
    };

    assert!(matches!(
        config.validate(),
        Err(CycleBenchmarkError::ResourceLimitExceeded {
            resource: "instances",
            ..
        })
    ));
}

#[test]
fn configuration_instance_count_is_exact() {
    let config = CycleBenchmarkConfig {
        pauli_count: 3,
        sequence_lengths: vec![1, 2, 4, 8],
        randomizations_per_length: 2,
        ..test_config()
    };

    assert_eq!(
        config
            .instance_count()
            .expect("instance count must fit"),
        3 * 4 * 2
    );
}

#[test]
fn configuration_total_shots_is_exact() {
    let config = CycleBenchmarkConfig {
        pauli_count: 3,
        sequence_lengths: vec![1, 2, 4],
        randomizations_per_length: 2,
        shots: 100,
        ..test_config()
    };

    assert_eq!(
        config
            .total_shots()
            .expect("total shots must fit"),
        3 * 3 * 2 * 100
    );
}

// =============================================================================
// Cycle-definition validation
// =============================================================================

#[test]
fn cycle_definition_rejects_empty_identifier() {
    let result = CycleDefinition::new("   ", 1);

    assert!(matches!(
        result,
        Err(CycleBenchmarkError::InvalidConfiguration {
            field: "cycle.id",
            ..
        })
    ));
}

#[test]
fn cycle_definition_rejects_zero_qubits() {
    let result = CycleDefinition::new("cycle", 0);

    assert!(matches!(
        result,
        Err(CycleBenchmarkError::InvalidQubitCount {
            qubits: 0
        })
    ));
}

#[test]
fn cycle_definition_rejects_zero_depth() {
    let result = CycleDefinition::new("cycle", 1)
        .expect("base cycle should be valid")
        .with_depth(0);

    assert!(matches!(
        result,
        Err(CycleBenchmarkError::InvalidConfiguration {
            field: "cycle.depth",
            ..
        })
    ));
}

#[test]
fn cycle_definition_rejects_zero_identity_period() {
    let result = CycleDefinition::new("cycle", 1)
        .expect("base cycle should be valid")
        .with_identity_period(0);

    assert!(matches!(
        result,
        Err(CycleBenchmarkError::InvalidConfiguration {
            field: "cycle.identity_period",
            ..
        })
    ));
}

#[test]
fn periodic_lengths_require_verified_cycle_period() {
    let cycle = CycleDefinition::new("cycle", 1)
        .expect("cycle should be valid");

    let config = CycleBenchmarkConfig {
        require_periodic_lengths: true,
        ..test_config()
    };

    assert!(matches!(
        cycle.validate(&config),
        Err(CycleBenchmarkError::InvalidConfiguration {
            field: "cycle.identity_period",
            ..
        })
    ));
}

#[test]
fn periodic_lengths_must_match_identity_period() {
    let cycle = CycleDefinition::new("cycle", 1)
        .expect("cycle should be valid")
        .with_identity_period(2)
        .expect("period must be valid");

    let config = CycleBenchmarkConfig {
        sequence_lengths: vec![1, 2, 4],
        require_periodic_lengths: true,
        ..test_config()
    };

    assert!(matches!(
        cycle.validate(&config),
        Err(
            CycleBenchmarkError::SequenceLengthNotMultipleOfPeriod {
                length: 1,
                period: 2
            }
        )
    ));
}

#[test]
fn periodic_lengths_pass_when_all_are_valid_multiples() {
    let cycle = CycleDefinition::new("cycle", 1)
        .expect("cycle should be valid")
        .with_identity_period(2)
        .expect("period must be valid");

    let config = CycleBenchmarkConfig {
        sequence_lengths: vec![2, 4, 8],
        require_periodic_lengths: true,
        ..test_config()
    };

    cycle
        .validate(&config)
        .expect("valid periodic lengths must pass");
}

#[test]
fn periodic_length_requirement_can_be_disabled() {
    let cycle = CycleDefinition::new("cycle", 1)
        .expect("cycle should be valid");

    let config = CycleBenchmarkConfig {
        sequence_lengths: vec![1, 3, 5],
        require_periodic_lengths: false,
        ..test_config()
    };

    cycle
        .validate(&config)
        .expect("periodic validation should be optional");
}

// =============================================================================
// Pauli-frame contract
// =============================================================================

#[test]
fn identity_pauli_frame_has_expected_shape() {
    let frame =
        PauliFrame::identity(4)
            .expect("identity frame should be valid");

    assert_eq!(frame.qubits(), 4);
    assert_eq!(frame.factors().len(), 4);
    assert_eq!(frame.label(), "IIII");
    assert_eq!(frame.weight(), 0);
    assert!(frame.is_identity());
}

#[test]
fn identity_pauli_frame_rejects_zero_width() {
    assert!(matches!(
        PauliFrame::identity(0),
        Err(CycleBenchmarkError::InvalidQubitCount {
            qubits: 0
        })
    ));
}

#[test]
fn random_non_identity_pauli_frame_is_never_identity() {
    let mut rng = test_rng();

    for _ in 0..256 {
        let frame =
            PauliFrame::random_non_identity(4, &mut rng)
                .expect("random non-identity Pauli must be valid");

        assert_eq!(frame.qubits(), 4);
        assert!(!frame.is_identity());
        assert!(frame.weight() >= 1);
        assert_eq!(frame.label().chars().count(), 4);
    }
}

#[test]
fn random_pauli_selection_is_unique() {
    let mut rng = test_rng();

    let paulis =
        select_random_paulis(2, 12, false, &mut rng)
            .expect("12 unique non-identity two-qubit Paulis must be available");

    assert_eq!(paulis.len(), 12);

    let mut labels = std::collections::BTreeSet::new();

    for pauli in &paulis {
        assert!(!pauli.is_identity());
        assert_eq!(pauli.qubits(), 2);
        assert!(labels.insert(pauli.label()));
    }
}

#[test]
fn one_qubit_non_identity_pauli_basis_has_exactly_three_elements() {
    let mut rng = test_rng();

    let paulis =
        select_random_paulis(1, 3, false, &mut rng)
            .expect("X/Y/Z must all be selectable");

    let labels = paulis
        .iter()
        .map(PauliFrame::label)
        .collect::<std::collections::BTreeSet<_>>();

    assert_eq!(
        labels,
        ["X", "Y", "Z"]
            .into_iter()
            .map(str::to_owned)
            .collect()
    );
}

#[test]
fn Pauli_selection_rejects_exhaustion() {
    let mut rng = test_rng();

    // For one qubit:
    //
    //     4 total Paulis
    //     3 non-identity Paulis
    //
    // Therefore four unique non-identity Paulis cannot exist.
    let result =
        select_random_paulis(1, 4, false, &mut rng);

    assert!(matches!(
        result,
        Err(CycleBenchmarkError::PauliSetExhausted {
            requested: 4,
            available: 3
        })
    ));
}

#[test]
fn Pauli_selection_can_include_identity_when_requested() {
    let mut rng = test_rng();

    let paulis =
        select_random_paulis(1, 4, true, &mut rng)
            .expect("all four one-qubit Paulis must be selectable");

    let labels = paulis
        .iter()
        .map(PauliFrame::label)
        .collect::<std::collections::BTreeSet<_>>();

    assert_eq!(labels.len(), 4);
    assert!(labels.contains("I"));
    assert!(labels.contains("X"));
    assert!(labels.contains("Y"));
    assert!(labels.contains("Z"));
}

// =============================================================================
// Reproducibility
// =============================================================================

#[test]
fn identical_seed_and_configuration_generate_identical_experiments() {
    let protocol_a =
        CycleBenchmarkProtocol::new(test_config())
            .expect("configuration must be valid");

    let protocol_b =
        CycleBenchmarkProtocol::new(test_config())
            .expect("configuration must be valid");

    let mut rng_a = test_rng();
    let mut rng_b = test_rng();

    let experiment_a =
        protocol_a
            .generate(test_cycle(), &mut rng_a)
            .expect("experiment A must generate");

    let experiment_b =
        protocol_b
            .generate(test_cycle(), &mut rng_b)
            .expect("experiment B must generate");

    assert_eq!(
        experiment_a.config,
        experiment_b.config
    );

    assert_eq!(
        experiment_a.cycle,
        experiment_b.cycle
    );

    assert_eq!(
        experiment_a.paulis,
        experiment_b.paulis
    );

    assert_eq!(
        experiment_a.instances,
        experiment_b.instances
    );
}

#[test]
fn different_seeds_can_produce_different_experiment_streams() {
    let protocol =
        CycleBenchmarkProtocol::new(test_config())
            .expect("configuration must be valid");

    let mut rng_a =
        RandomStream::from_seed(
            BenchmarkSeed::from_u64(1),
        );

    let mut rng_b =
        RandomStream::from_seed(
            BenchmarkSeed::from_u64(2),
        );

    let experiment_a =
        protocol
            .generate(test_cycle(), &mut rng_a)
            .expect("experiment A must generate");

    let experiment_b =
        protocol
            .generate(test_cycle(), &mut rng_b)
            .expect("experiment B must generate");

    assert_ne!(
        experiment_a.paulis,
        experiment_b.paulis,
        "different explicit benchmark seeds should not collapse \
         into the same generated Pauli stream"
    );
}

// =============================================================================
// Experiment generation
// =============================================================================

#[test]
fn generated_experiment_passes_its_own_validation() {
    let experiment = test_experiment();

    experiment
        .validate()
        .expect("generated experiment must validate");
}

#[test]
fn generated_experiment_has_expected_pauli_count() {
    let experiment = test_experiment();

    assert_eq!(
        experiment.paulis.len(),
        TEST_PAULI_COUNT
    );
}

#[test]
fn generated_experiment_has_exact_instance_count() {
    let experiment = test_experiment();

    let expected =
        TEST_PAULI_COUNT
            * TEST_SEQUENCE_LENGTHS.len()
            * TEST_RANDOMIZATIONS;

    assert_eq!(
        experiment.instances.len(),
        expected
    );
}

#[test]
fn every_generated_instance_has_m_plus_one_random_pauli_cycles() {
    let experiment = test_experiment();

    for instance in &experiment.instances {
        assert_eq!(
            instance.random_pauli_cycles.len(),
            instance.sequence_length + 1,
            "instance {} violates m+1 random-Pauli invariant",
            instance.id
        );
    }
}

#[test]
fn every_generated_instance_has_correct_pauli_width() {
    let experiment = test_experiment();

    for instance in &experiment.instances {
        assert_eq!(
            instance.measured_pauli.qubits(),
            experiment.config.qubits
        );

        for random_pauli in &instance.random_pauli_cycles {
            assert_eq!(
                random_pauli.qubits(),
                experiment.config.qubits
            );
        }
    }
}

#[test]
fn generated_paulis_are_non_identity_when_identity_is_disabled() {
    let experiment = test_experiment();

    assert!(
        experiment
            .paulis
            .iter()
            .all(|pauli| !pauli.is_identity())
    );
}

#[test]
fn generated_instance_ids_are_unique() {
    let experiment = test_experiment();

    let ids = experiment
        .instances
        .iter()
        .map(|instance| instance.id.clone())
        .collect::<std::collections::BTreeSet<_>>();

    assert_eq!(
        ids.len(),
        experiment.instances.len()
    );
}

#[test]
fn generated_instance_id_contains_cycle_pauli_length_and_trial_identity() {
    let experiment = test_experiment();

    for instance in &experiment.instances {
        assert!(
            instance.id.contains("test-cycle"),
            "instance ID must contain cycle identity"
        );

        assert!(
            instance
                .id
                .contains(&instance.measured_pauli.label()),
            "instance ID must contain Pauli identity"
        );

        assert!(
            instance
                .id
                .contains(&format!("m{}", instance.sequence_length)),
            "instance ID must contain sequence length"
        );

        assert!(
            instance
                .id
                .contains(&format!("trial{}", instance.trial_index)),
            "instance ID must contain trial index"
        );
    }
}

#[test]
fn experiment_rejects_wrong_pauli_width() {
    let mut experiment = test_experiment();

    experiment.paulis[0] =
        PauliFrame::identity(2)
            .expect("two-qubit identity must be valid");

    assert!(matches!(
        experiment.validate(),
        Err(CycleBenchmarkError::PauliWidthMismatch {
            expected: 1,
            actual: 2
        })
    ));
}

// =============================================================================
// Instance contract
// =============================================================================

#[test]
fn cycle_instance_requires_m_plus_one_random_pauli_cycles() {
    let cycle = test_cycle();

    let measured_pauli =
        PauliFrame::identity(1)
            .expect("identity Pauli must be valid");

    let invalid =
        CycleBenchmarkInstance::new(
            &cycle,
            measured_pauli,
            4,
            vec![
                PauliFrame::identity(1)
                    .expect("identity Pauli must be valid");
                4
            ],
            0,
        );

    assert!(matches!(
        invalid,
        Err(CycleBenchmarkError::InvalidConfiguration {
            field: "random_pauli_cycles",
            ..
        })
    ));
}

#[test]
fn cycle_instance_rejects_zero_sequence_length() {
    let cycle = test_cycle();

    let measured_pauli =
        PauliFrame::identity(1)
            .expect("identity Pauli must be valid");

    let invalid =
        CycleBenchmarkInstance::new(
            &cycle,
            measured_pauli,
            0,
            Vec::new(),
            0,
        );

    assert!(matches!(
        invalid,
        Err(CycleBenchmarkError::InvalidSequenceLength {
            length: 0
        })
    ));
}

#[test]
fn cycle_instance_rejects_wrong_measured_pauli_width() {
    let cycle = test_cycle();

    let measured_pauli =
        PauliFrame::identity(2)
            .expect("two-qubit Pauli must be valid");

    let random_paulis =
        vec![
            PauliFrame::identity(1)
                .expect("one-qubit Pauli must be valid");
            2
        ];

    let result =
        CycleBenchmarkInstance::new(
            &cycle,
            measured_pauli,
            1,
            random_paulis,
            0,
        );

    assert!(matches!(
        result,
        Err(CycleBenchmarkError::PauliWidthMismatch {
            expected: 1,
            actual: 2
        })
    ));
}

// =============================================================================
// Execution-request contract
// =============================================================================

#[test]
fn execution_request_accepts_positive_shots() {
    let experiment = test_experiment();

    let instance = experiment
        .instances
        .first()
        .expect("generated experiment must contain an instance")
        .clone();

    let request =
        CycleExecutionRequest::new(
            instance,
            1_000,
        )
        .expect("positive shot count must be accepted");

    assert_eq!(request.shots, 1_000);
}

#[test]
fn execution_request_rejects_zero_shots() {
    let experiment = test_experiment();

    let instance = experiment
        .instances
        .first()
        .expect("generated experiment must contain an instance")
        .clone();

    let result =
        CycleExecutionRequest::new(
            instance,
            0,
        );

    assert!(matches!(
        result,
        Err(CycleBenchmarkError::InvalidConfiguration {
            field: "shots",
            ..
        })
    ));
}

// =============================================================================
// Observation contract
// =============================================================================

#[test]
fn observation_accepts_valid_matching_counts() {
    let observation =
        CycleExecutionObservation::new(
            "instance",
            750,
            1_000,
        )
        .expect("valid observation must be accepted");

    assert_eq!(
        observation.matching_outcomes,
        750
    );

    assert_eq!(
        observation.shots,
        1_000
    );

    assert_close(
        observation.matching_probability(),
        0.75,
        1.0e-15,
    );

    assert_close(
        observation.expectation(),
        0.5,
        1.0e-15,
    );
}

#[test]
fn observation_accepts_zero_matching_outcomes() {
    let observation =
        CycleExecutionObservation::new(
            "instance",
            0,
            1_000,
        )
        .expect("zero matching outcomes are valid");

    assert_close(
        observation.matching_probability(),
        0.0,
        0.0,
    );

    assert_close(
        observation.expectation(),
        -1.0,
        0.0,
    );
}

#[test]
fn observation_accepts_all_matching_outcomes() {
    let observation =
        CycleExecutionObservation::new(
            "instance",
            1_000,
            1_000,
        )
        .expect("all matching outcomes are valid");

    assert_close(
        observation.matching_probability(),
        1.0,
        0.0,
    );

    assert_close(
        observation.expectation(),
        1.0,
        0.0,
    );
}

#[test]
fn observation_rejects_empty_instance_id() {
    let result =
        CycleExecutionObservation::new(
            "   ",
            10,
            10,
        );

    assert!(matches!(
        result,
        Err(CycleBenchmarkError::InvalidObservation {
            ..
        })
    ));
}

#[test]
fn observation_rejects_zero_shots() {
    let result =
        CycleExecutionObservation::new(
            "instance",
            0,
            0,
        );

    assert!(matches!(
        result,
        Err(CycleBenchmarkError::InvalidObservation {
            ..
        })
    ));
}

#[test]
fn observation_rejects_matching_outcomes_above_shots() {
    let result =
        CycleExecutionObservation::new(
            "instance",
            1_001,
            1_000,
        );

    assert!(matches!(
        result,
        Err(CycleBenchmarkError::InvalidObservation {
            ..
        })
    ));
}

#[test]
fn observation_can_carry_execution_timing() {
    let observation =
        CycleExecutionObservation::new(
            "instance",
            10,
            10,
        )
        .expect("observation must be valid")
        .with_execution_time_ns(42_000);

    assert_eq!(
        observation.execution_time_ns,
        Some(42_000)
    );
}

// =============================================================================
// Executor integration
// =============================================================================

#[test]
fn protocol_execute_integrates_with_backend_independent_executor() {
    let protocol =
        CycleBenchmarkProtocol::new(test_config())
            .expect("protocol must be valid");

    let experiment = test_experiment();

    let mut executor = PerfectExecutor;

    let observations =
        protocol
            .execute(&experiment, &mut executor)
            .expect("executor integration must succeed");

    assert_eq!(
        observations.len(),
        experiment.instances.len()
    );

    for observation in &observations {
        assert_eq!(
            observation.shots,
            TEST_SHOTS
        );

        assert_eq!(
            observation.matching_outcomes,
            TEST_SHOTS
        );

        assert_close(
            observation.expectation(),
            1.0,
            0.0,
        );
    }
}

#[test]
fn protocol_execute_rejects_executor_returning_wrong_instance_id() {
    let protocol =
        CycleBenchmarkProtocol::new(test_config())
            .expect("protocol must be valid");

    let experiment = test_experiment();

    let mut executor = WrongIdExecutor;

    let result =
        protocol.execute(
            &experiment,
            &mut executor,
        );

    assert!(matches!(
        result,
        Err(CycleBenchmarkError::InvalidObservation {
            ..
        })
    ));
}

#[test]
fn protocol_execute_rejects_executor_returning_wrong_shot_count() {
    let protocol =
        CycleBenchmarkProtocol::new(test_config())
            .expect("protocol must be valid");

    let experiment = test_experiment();

    let mut executor = WrongShotsExecutor;

    let result =
        protocol.execute(
            &experiment,
            &mut executor,
        );

    assert!(matches!(
        result,
        Err(CycleBenchmarkError::InvalidObservation {
            ..
        })
    ));
}

#[test]
fn protocol_execute_converts_backend_failure_to_protocol_error() {
    let protocol =
        CycleBenchmarkProtocol::new(test_config())
            .expect("protocol must be valid");

    let experiment = test_experiment();

    let mut executor = FailingExecutor;

    let result =
        protocol.execute(
            &experiment,
            &mut executor,
        );

    assert!(matches!(
        result,
        Err(CycleBenchmarkError::InvalidObservation {
            ..
        })
    ));
}

// =============================================================================
// Full analysis integration
// =============================================================================

#[test]
fn perfect_execution_produces_perfect_cycle_result() {
    let protocol =
        CycleBenchmarkProtocol::new(test_config())
            .expect("protocol must be valid");

    let experiment = test_experiment();

    let observations =
        experiment
            .instances
            .iter()
            .map(|instance| perfect_observation(
                instance,
                TEST_SHOTS,
            ))
            .collect::<Vec<_>>();

    let result =
        protocol
            .analyze(
                &experiment,
                &observations,
            )
            .expect("perfect observations must be analyzable");

    assert_eq!(
        result.benchmark_id,
        CYCLE_BENCHMARK_ID
    );

    assert_eq!(
        result.protocol_version,
        CYCLE_BENCHMARK_PROTOCOL_VERSION
    );

    assert_eq!(
        result.executed_circuits,
        experiment.instances.len()
    );

    assert_eq!(
        result.executed_shots,
        experiment.instances.len() * TEST_SHOTS
    );

    assert_close(
        result.process_fidelity(),
        1.0,
        1.0e-8,
    );

    assert_close(
        result.process_infidelity(),
        0.0,
        1.0e-8,
    );

    assert_close(
        result.composite_fidelity.average_gate_fidelity,
        1.0,
        1.0e-8,
    );

    assert_close(
        result.composite_fidelity.average_gate_infidelity,
        0.0,
        1.0e-8,
    );

    assert_eq!(
        result.composite_fidelity.pauli_terms,
        3
    );

    assert_eq!(
        result.composite_fidelity.complete_non_identity_terms,
        3
    );

    assert!(
        result.composite_fidelity.exhaustive,
        "X/Y/Z for one qubit is the complete non-identity Pauli basis"
    );

    assert!(
        result.assumptions.exhaustive_pauli_basis
    );

    assert!(
        result.assumptions.reports_dressed_cycle
    );

    assert!(
        result.assumptions.markovian_noise_assumed
    );
}

#[test]
fn analysis_requires_observations() {
    let protocol =
        CycleBenchmarkProtocol::new(test_config())
            .expect("protocol must be valid");

    let experiment = test_experiment();

    let result =
        protocol.analyze(
            &experiment,
            &[],
        );

    assert!(matches!(
        result,
        Err(CycleBenchmarkError::EmptyObservations)
    ));
}

#[test]
fn analysis_rejects_missing_observation() {
    let protocol =
        CycleBenchmarkProtocol::new(test_config())
            .expect("protocol must be valid");

    let experiment = test_experiment();

    let mut observations =
        experiment
            .instances
            .iter()
            .map(|instance| perfect_observation(
                instance,
                TEST_SHOTS,
            ))
            .collect::<Vec<_>>();

    observations.pop();

    let result =
        protocol.analyze(
            &experiment,
            &observations,
        );

    assert!(matches!(
        result,
        Err(CycleBenchmarkError::MissingObservation {
            ..
        })
    ));
}

#[test]
fn analysis_rejects_duplicate_observation_ids() {
    let protocol =
        CycleBenchmarkProtocol::new(test_config())
            .expect("protocol must be valid");

    let experiment = test_experiment();

    let first =
        experiment
            .instances
            .first()
            .expect("experiment must contain instances");

    let duplicate =
        perfect_observation(
            first,
            TEST_SHOTS,
        );

    let observations = vec![
        duplicate.clone(),
        duplicate,
    ];

    let result =
        protocol.analyze(
            &experiment,
            &observations,
        );

    assert!(matches!(
        result,
        Err(CycleBenchmarkError::DuplicateObservation {
            ..
        })
    ));
}

// =============================================================================
// Timing integration
// =============================================================================

#[test]
fn analysis_aggregates_execution_timing_when_present() {
    let protocol =
        CycleBenchmarkProtocol::new(test_config())
            .expect("protocol must be valid");

    let experiment = test_experiment();

    let observations =
        experiment
            .instances
            .iter()
            .enumerate()
            .map(|(index, instance)| {
                CycleExecutionObservation::new(
                    instance.id.clone(),
                    TEST_SHOTS,
                    TEST_SHOTS,
                )
                .expect("observation must be valid")
                .with_execution_time_ns(
                    (index as u64 + 1) * 1_000,
                )
            })
            .collect::<Vec<_>>();

    let result =
        protocol
            .analyze(
                &experiment,
                &observations,
            )
            .expect("timed observations must be analyzable");

    let expected_total =
        (1..=experiment.instances.len())
            .map(|index| index as u64 * 1_000)
            .sum::<u64>();

    assert_eq!(
        result.total_execution_time_ns,
        Some(expected_total)
    );
}

#[test]
fn analysis_reports_no_timing_when_observations_have_no_timing() {
    let protocol =
        CycleBenchmarkProtocol::new(test_config())
            .expect("protocol must be valid");

    let experiment = test_experiment();

    let observations =
        experiment
            .instances
            .iter()
            .map(|instance| perfect_observation(
                instance,
                TEST_SHOTS,
            ))
            .collect::<Vec<_>>();

    let result =
        protocol
            .analyze(
                &experiment,
                &observations,
            )
            .expect("untimed observations must be analyzable");

    assert_eq!(
        result.total_execution_time_ns,
        None
    );
}

// =============================================================================
// Decay-point behavior
// =============================================================================

#[test]
fn decay_point_aggregates_observation_expectations() {
    let observations = vec![
        CycleExecutionObservation::new(
            "a",
            750,
            1_000,
        )
        .expect("observation must be valid"),

        CycleExecutionObservation::new(
            "b",
            500,
            1_000,
        )
        .expect("observation must be valid"),

        CycleExecutionObservation::new(
            "c",
            1_000,
            1_000,
        )
        .expect("observation must be valid"),
    ];

    let point =
        CycleDecayPoint::from_observations(
            8,
            &observations,
        )
        .expect("decay point must be valid");

    // Expectations:
    //
    // 0.75 -> +0.5
    // 0.50 ->  0.0
    // 1.00 -> +1.0
    //
    // Mean = 0.5
    assert_eq!(
        point.sequence_length,
        8
    );

    assert_eq!(
        point.randomizations,
        3
    );

    assert_eq!(
        point.total_shots,
        3_000
    );

    assert_close(
        point.mean_expectation,
        0.5,
        1.0e-15,
    );

    assert_finite(
        point.standard_error,
        "standard error",
    );
}

#[test]
fn decay_point_rejects_empty_observations() {
    let result =
        CycleDecayPoint::from_observations(
            4,
            &[],
        );

    assert!(result.is_err());
}

// =============================================================================
// Statistical / fit result validation
// =============================================================================

#[test]
fn perfect_cycle_fits_are_physical() {
    let protocol =
        CycleBenchmarkProtocol::new(test_config())
            .expect("protocol must be valid");

    let experiment = test_experiment();

    let observations =
        experiment
            .instances
            .iter()
            .map(|instance| perfect_observation(
                instance,
                TEST_SHOTS,
            ))
            .collect::<Vec<_>>();

    let result =
        protocol
            .analyze(
                &experiment,
                &observations,
            )
            .expect("perfect benchmark must analyze");

    for pauli_result in &result.pauli_results {
        let fit = &pauli_result.fit;

        assert_finite(
            fit.amplitude,
            "fit amplitude",
        );

        assert_finite(
            fit.decay_parameter,
            "decay parameter",
        );

        assert_finite(
            fit.offset,
            "fit offset",
        );

        assert_finite(
            fit.sum_squared_error,
            "sum squared error",
        );

        assert_finite(
            fit.root_mean_squared_error,
            "root mean squared error",
        );

        assert_finite(
            fit.r_squared,
            "R squared",
        );

        assert!(
            fit.decay_parameter_is_physical(),
            "fitted decay parameter {} must be physical",
            fit.decay_parameter
        );

        assert!(
            fit.r_squared >= 0.99,
            "perfect synthetic data should have excellent fit quality; \
             got R²={}",
            fit.r_squared
        );

        assert_close(
            fit.decay_parameter,
            1.0,
            1.0e-6,
        );

        assert_close(
            fit.offset,
            0.0,
            1.0e-12,
        );
    }
}

#[test]
fn no_offset_fit_model_is_reported_consistently() {
    let protocol =
        CycleBenchmarkProtocol::new(
            CycleBenchmarkConfig {
                fit_model: CycleFitModel::NoOffset,
                ..test_config()
            }
        )
        .expect("protocol must be valid");

    assert_eq!(
        protocol.config.fit_model,
        CycleFitModel::NoOffset
    );

    assert_eq!(
        protocol.config.fit_model.as_str(),
        CYCLE_BENCHMARK_FIT_MODEL
    );
}

#[test]
fn offset_fit_model_is_available_as_explicit_diagnostic_mode() {
    let protocol =
        CycleBenchmarkProtocol::new(
            CycleBenchmarkConfig {
                fit_model: CycleFitModel::WithOffset,
                ..test_config()
            }
        )
        .expect("offset model configuration must be valid");

    assert_eq!(
        protocol.config.fit_model,
        CycleFitModel::WithOffset
    );

    assert_eq!(
        protocol.config.fit_model.as_str(),
        "A*p^m+B"
    );
}

// =============================================================================
// Process-fidelity semantics
// =============================================================================

#[test]
fn one_qubit_complete_non_identity_pauli_set_is_exhaustive() {
    let protocol =
        CycleBenchmarkProtocol::new(
            CycleBenchmarkConfig {
                pauli_count: 3,
                ..test_config()
            }
        )
        .expect("protocol must be valid");

    let experiment = test_experiment();

    let observations =
        experiment
            .instances
            .iter()
            .map(|instance| perfect_observation(
                instance,
                TEST_SHOTS,
            ))
            .collect::<Vec<_>>();

    let result =
        protocol
            .analyze(
                &experiment,
                &observations,
            )
            .expect("benchmark must analyze");

    assert_eq!(
        result.composite_fidelity.complete_non_identity_terms,
        3
    );

    assert_eq!(
        result.composite_fidelity.pauli_terms,
        3
    );

    assert!(
        result.composite_fidelity.exhaustive
    );
}

#[test]
fn sampled_pauli_set_is_not_mislabelled_as_exhaustive() {
    let config = CycleBenchmarkConfig {
        pauli_count: 2,
        ..test_config()
    };

    let protocol =
        CycleBenchmarkProtocol::new(config)
            .expect("sampled configuration must be valid");

    let mut rng = test_rng();

    let experiment =
        protocol
            .generate(test_cycle(), &mut rng)
            .expect("experiment must generate");

    let observations =
        experiment
            .instances
            .iter()
            .map(|instance| perfect_observation(
                instance,
                TEST_SHOTS,
            ))
            .collect::<Vec<_>>();

    let result =
        protocol
            .analyze(
                &experiment,
                &observations,
            )
            .expect("sampled benchmark must analyze");

    assert_eq!(
        result.composite_fidelity.pauli_terms,
        2
    );

    assert_eq!(
        result.composite_fidelity.complete_non_identity_terms,
        3
    );

    assert!(
        !result.composite_fidelity.exhaustive,
        "two of three non-identity one-qubit Paulis must be \
         reported as a sampled characterization"
    );

    assert!(
        !result.assumptions.exhaustive_pauli_basis
    );
}

// =============================================================================
// Result accounting
// =============================================================================

#[test]
fn result_accounts_for_all_executed_circuits() {
    let protocol =
        CycleBenchmarkProtocol::new(test_config())
            .expect("protocol must be valid");

    let experiment = test_experiment();

    let observations =
        experiment
            .instances
            .iter()
            .map(|instance| perfect_observation(
                instance,
                TEST_SHOTS,
            ))
            .collect::<Vec<_>>();

    let result =
        protocol
            .analyze(
                &experiment,
                &observations,
            )
            .expect("analysis must succeed");

    assert_eq!(
        result.executed_circuits,
        experiment.instances.len()
    );
}

#[test]
fn result_accounts_for_all_executed_shots() {
    let protocol =
        CycleBenchmarkProtocol::new(test_config())
            .expect("protocol must be valid");

    let experiment = test_experiment();

    let observations =
        experiment
            .instances
            .iter()
            .map(|instance| perfect_observation(
                instance,
                TEST_SHOTS,
            ))
            .collect::<Vec<_>>();

    let result =
        protocol
            .analyze(
                &experiment,
                &observations,
            )
            .expect("analysis must succeed");

    assert_eq!(
        result.executed_shots,
        experiment.instances.len()
            * TEST_SHOTS
    );
}

#[test]
fn all_reported_fidelity_metrics_are_physical() {
    let protocol =
        CycleBenchmarkProtocol::new(test_config())
            .expect("protocol must be valid");

    let experiment = test_experiment();

    let observations =
        experiment
            .instances
            .iter()
            .map(|instance| perfect_observation(
                instance,
                TEST_SHOTS,
            ))
            .collect::<Vec<_>>();

    let result =
        protocol
            .analyze(
                &experiment,
                &observations,
            )
            .expect("analysis must succeed");

    assert_fidelity(
        result.composite_fidelity.process_fidelity,
        "process fidelity",
    );

    assert_fidelity(
        result.composite_fidelity.average_gate_fidelity,
        "average gate fidelity",
    );

    assert_finite(
        result.composite_fidelity.process_infidelity,
        "process infidelity",
    );

    assert_finite(
        result.composite_fidelity.average_gate_infidelity,
        "average gate infidelity",
    );
}

// =============================================================================
// Scientific interpretation metadata
// =============================================================================

#[test]
fn result_marks_the_cycle_as_a_dressed_twirled_cycle() {
    let protocol =
        CycleBenchmarkProtocol::new(test_config())
            .expect("protocol must be valid");

    let experiment = test_experiment();

    let observations =
        experiment
            .instances
            .iter()
            .map(|instance| perfect_observation(
                instance,
                TEST_SHOTS,
            ))
            .collect::<Vec<_>>();

    let result =
        protocol
            .analyze(
                &experiment,
                &observations,
            )
            .expect("analysis must succeed");

    assert!(
        result.assumptions.reports_dressed_cycle,
        "CB must not silently relabel a dressed/twirled cycle \
         as an untwirled physical-gate fidelity"
    );
}

#[test]
fn verified_periodic_cycle_is_recorded_in_result_assumptions() {
    let protocol =
        CycleBenchmarkProtocol::new(test_config())
            .expect("protocol must be valid");

    let experiment = test_experiment();

    let observations =
        experiment
            .instances
            .iter()
            .map(|instance| perfect_observation(
                instance,
                TEST_SHOTS,
            ))
            .collect::<Vec<_>>();

    let result =
        protocol
            .analyze(
                &experiment,
                &observations,
            )
            .expect("analysis must succeed");

    assert!(
        result.assumptions.periodic_cycle_verified
    );
}

#[test]
fn result_records_exhaustive_pauli_status() {
    let protocol =
        CycleBenchmarkProtocol::new(test_config())
            .expect("protocol must be valid");

    let experiment = test_experiment();

    let observations =
        experiment
            .instances
            .iter()
            .map(|instance| perfect_observation(
                instance,
                TEST_SHOTS,
            ))
            .collect::<Vec<_>>();

    let result =
        protocol
            .analyze(
                &experiment,
                &observations,
            )
            .expect("analysis must succeed");

    assert_eq!(
        result.assumptions.exhaustive_pauli_basis,
        result.composite_fidelity.exhaustive
    );
}

// =============================================================================
// End-to-end protocol integration
// =============================================================================

#[test]
fn protocol_run_completes_full_generate_execute_analyze_pipeline() {
    let protocol =
        CycleBenchmarkProtocol::new(test_config())
            .expect("protocol must be valid");

    let mut rng = test_rng();

    let mut executor = PerfectExecutor;

    let result =
        protocol
            .run(
                test_cycle(),
                &mut rng,
                &mut executor,
            )
            .expect("complete protocol run must succeed");

    assert_eq!(
        result.benchmark_id,
        CYCLE_BENCHMARK_ID
    );

    assert_eq!(
        result.protocol_version,
        CYCLE_BENCHMARK_PROTOCOL_VERSION
    );

    assert!(
        result.executed_circuits > 0
    );

    assert!(
        result.executed_shots > 0
    );

    assert_fidelity(
        result.process_fidelity(),
        "process fidelity",
    );
}

// =============================================================================
// Regression protection
// =============================================================================

#[test]
fn generated_experiment_is_stable_for_the_published_test_seed() {
    let experiment = test_experiment();

    assert_eq!(
        experiment.instances.len(),
        3 * 4 * 2
    );

    // The first instance must remain structurally stable for this published
    // deterministic seed. We intentionally test structural identity rather
    // than hard-coding every generated random Pauli factor. If the canonical
    // RNG algorithm changes, the RNG algorithm identifier must also change.
    let first =
        experiment
            .instances
            .first()
            .expect("experiment must contain a first instance");

    assert_eq!(
        first.cycle_id,
        "test-cycle"
    );

    assert_eq!(
        first.sequence_length,
        1
    );

    assert_eq!(
        first.trial_index,
        0
    );

    assert_eq!(
        first.random_pauli_cycles.len(),
        2
    );

    assert_eq!(
        first.measured_pauli.qubits(),
        1
    );
}

#[test]
fn result_is_deterministic_for_identical_observations() {
    let protocol =
        CycleBenchmarkProtocol::new(test_config())
            .expect("protocol must be valid");

    let experiment = test_experiment();

    let observations =
        experiment
            .instances
            .iter()
            .map(|instance| perfect_observation(
                instance,
                TEST_SHOTS,
            ))
            .collect::<Vec<_>>();

    let result_a =
        protocol
            .analyze(
                &experiment,
                &observations,
            )
            .expect("analysis A must succeed");

    let result_b =
        protocol
            .analyze(
                &experiment,
                &observations,
            )
            .expect("analysis B must succeed");

    assert_eq!(
        result_a.benchmark_id,
        result_b.benchmark_id
    );

    assert_eq!(
        result_a.protocol_version,
        result_b.protocol_version
    );

    assert_eq!(
        result_a.executed_circuits,
        result_b.executed_circuits
    );

    assert_eq!(
        result_a.executed_shots,
        result_b.executed_shots
    );

    assert_close(
        result_a.process_fidelity(),
        result_b.process_fidelity(),
        0.0,
    );

    assert_close(
        result_a.process_infidelity(),
        result_b.process_infidelity(),
        0.0,
    );

    assert_eq!(
        result_a.pauli_results.len(),
        result_b.pauli_results.len()
    );

    for (a, b) in result_a
        .pauli_results
        .iter()
        .zip(result_b.pauli_results.iter())
    {
        assert_eq!(a.pauli, b.pauli);
        assert_eq!(
            a.decay_points.len(),
            b.decay_points.len()
        );

        assert_close(
            a.fit.decay_parameter,
            b.fit.decay_parameter,
            0.0,
        );

        assert_close(
            a.fit.amplitude,
            b.fit.amplitude,
            0.0,
        );

        assert_close(
            a.fit.offset,
            b.fit.offset,
            0.0,
        );
    }
}

// =============================================================================
// Error-display regression tests
// =============================================================================

#[test]
fn protocol_errors_have_non_empty_display_messages() {
    let errors = [
        CycleBenchmarkError::InvalidQubitCount {
            qubits: 0,
        },

        CycleBenchmarkError::InvalidSequenceLength {
            length: 0,
        },

        CycleBenchmarkError::NonIncreasingSequenceLengths,

        CycleBenchmarkError::EmptyObservations,

        CycleBenchmarkError::MissingObservation {
            instance_id: "missing".to_owned(),
        },

        CycleBenchmarkError::DuplicateObservation {
            instance_id: "duplicate".to_owned(),
        },
    ];

    for error in errors {
        let message = error.to_string();

        assert!(
            !message.trim().is_empty(),
            "every production error must have a useful display message"
        );
    }
}

// =============================================================================
// Public-result type integration
// =============================================================================

#[test]
fn protocol_result_contains_expected_public_components() {
    fn assert_result_shape(
        result: &CycleBenchmarkProtocolResult,
    ) {
        assert_eq!(
            result.benchmark_id,
            CYCLE_BENCHMARK_ID
        );

        assert_eq!(
            result.protocol_version,
            CYCLE_BENCHMARK_PROTOCOL_VERSION
        );

        assert!(
            !result.pauli_results.is_empty()
        );

        assert!(
            result.executed_circuits > 0
        );

        assert!(
            result.executed_shots > 0
        );
    }

    let protocol =
        CycleBenchmarkProtocol::new(test_config())
            .expect("protocol must be valid");

    let experiment = test_experiment();

    let observations =
        experiment
            .instances
            .iter()
            .map(|instance| perfect_observation(
                instance,
                TEST_SHOTS,
            ))
            .collect::<Vec<_>>();

    let result =
        protocol
            .analyze(
                &experiment,
                &observations,
            )
            .expect("analysis must succeed");

    assert_result_shape(&result);
}

// =============================================================================
// Sanity check for the imported Pauli API
// =============================================================================

#[test]
fn pauli_enum_is_available_through_the_canonical_generator_boundary() {
    assert_eq!(
        Pauli::I.symbol(),
        'I'
    );

    assert_eq!(
        Pauli::X.symbol(),
        'X'
    );

    assert_eq!(
        Pauli::Y.symbol(),
        'Y'
    );

    assert_eq!(
        Pauli::Z.symbol(),
        'Z'
    );
}