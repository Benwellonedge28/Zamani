//! Zamani Quantum Benchmarking — XEB production test suite.
//!
//! Path:
//! `src/quantum/benchmarking/tests/xeb_tests.rs`
//!
//! Purpose
//! -------
//! This file provides production-grade tests for:
//!
//! - XEB mathematical correctness;
//! - Linear XEB;
//! - cross entropy;
//! - ideal Shannon entropy;
//! - cross-entropy difference;
//! - weighted shot-count handling;
//! - ideal-distribution validation;
//! - bitstring validation;
//! - zero-probability protection;
//! - non-finite-value protection;
//! - arithmetic-boundary protection;
//! - configuration/resource limits;
//! - exact versus partial ideal-model semantics;
//! - deterministic circuit seed derivation;
//! - protocol generation/execution integration;
//! - result validation;
//! - result serialization/deserialization;
//! - reproducibility;
//! - failure propagation;
//! - circuit-count and shot-count validation;
//! - regression fixtures.
//!
//! Integration contract
//! --------------------
//! This test module intentionally depends only on the public XEB protocol
//! boundary:
//!
//! `benchmarking::protocols::xeb`
//!
//! It does NOT construct Quantum IR directly and does NOT depend on a concrete
//! simulator or hardware backend.
//!
//! This preserves the intended dependency direction:
//!
//! ```text
//! XEB tests
//!     │
//!     ▼
//! protocols::xeb
//!     │
//!     ├── generator contract
//!     ├── executor contract
//!     └── mathematical analysis
//! ```
//!
//! Production backends can therefore be tested independently through the same
//! `XebCircuitGenerator` and `XebExecutor` contracts.
//!
//! Rust compatibility
//! -------------------
//! - Rust 1.97
//! - Rust 1.97.1
//! - Rust 2021
//! - no nightly features
//! - no unsafe code
//!
//! These tests intentionally avoid floating-point equality except where the
//! expected value is mathematically exact enough for a bounded tolerance.

#![deny(unsafe_code)]

use std::collections::BTreeMap;

use serde_json;

use crate::quantum::benchmarking::protocols::xeb::{
    cross_entropy,
    cross_entropy_difference,
    derive_circuit_seed,
    ideal_entropy,
    linear_xeb_from_counts,
    IdealModelKind,
    XebCircuitExecution,
    XebCircuitMetadata,
    XebCircuitGenerator,
    XebConfig,
    XebError,
    XebExecutor,
    XebExecution,
    XebIdealModel,
    XebObservedSample,
    XebProtocol,
    XebResult,
    DEFAULT_CONFIDENCE_LEVEL,
    DEFAULT_MAX_CIRCUITS,
    DEFAULT_MAX_QUBITS,
    DEFAULT_MAX_SHOTS_PER_CIRCUIT,
    DEFAULT_MAX_TOTAL_SHOTS,
    DEFAULT_SEED,
    DISTRIBUTION_TOLERANCE,
    XEB_BENCHMARK_ID,
    XEB_PROTOCOL_VERSION,
    XEB_RESULT_SCHEMA_VERSION,
};

// =============================================================================
// Test constants and helpers
// =============================================================================

const EPSILON: f64 = 1.0e-12;
const ONE_QUBIT_ZERO: &str = "0";
const ONE_QUBIT_ONE: &str = "1";

fn assert_close(actual: f64, expected: f64, tolerance: f64) {
    let difference = (actual - expected).abs();

    assert!(
        difference <= tolerance,
        "expected {expected:.16e}, got {actual:.16e}; difference {difference:.16e} \
         exceeded tolerance {tolerance:.16e}"
    );
}

fn assert_finite(value: f64, name: &str) {
    assert!(
        value.is_finite(),
        "{name} must be finite, got {value:?}"
    );
}

fn ideal_distribution_1q(
    zero: f64,
    one: f64,
) -> BTreeMap<String, f64> {
    BTreeMap::from([
        (ONE_QUBIT_ZERO.to_string(), zero),
        (ONE_QUBIT_ONE.to_string(), one),
    ])
}

fn ideal_distribution_2q() -> BTreeMap<String, f64> {
    BTreeMap::from([
        ("00".to_string(), 0.25),
        ("01".to_string(), 0.25),
        ("10".to_string(), 0.25),
        ("11".to_string(), 0.25),
    ])
}

fn deterministic_one_qubit_generator() -> DeterministicGenerator {
    DeterministicGenerator
}

fn deterministic_one_qubit_executor() -> DeterministicExecutor {
    DeterministicExecutor
}

// =============================================================================
// Test doubles
// =============================================================================

/// Deterministic generator used to test the public XEB generator contract.
///
/// The generated circuit is deliberately a tiny opaque test value. The XEB
/// protocol must not care what concrete circuit representation is used.
#[derive(Debug, Default, Clone, Copy)]
struct DeterministicGenerator;

impl XebCircuitGenerator for DeterministicGenerator {
    type Circuit = TestCircuit;

    fn generate(
        &mut self,
        config: &XebConfig,
        circuit_index: usize,
        seed: u64,
    ) -> Result<(Self::Circuit, XebCircuitMetadata), XebError> {
        let expected_seed =
            derive_circuit_seed(config.seed, circuit_index as u64);

        assert_eq!(
            seed,
            expected_seed,
            "XEB protocol must provide the documented deterministic circuit seed"
        );

        Ok((
            TestCircuit {
                circuit_index,
                num_qubits: config.num_qubits,
                seed,
            },
            XebCircuitMetadata {
                circuit_id: format!("xeb-test-circuit-{circuit_index}"),
                num_qubits: config.num_qubits,
                depth: 1,
                gate_count: 1,
                two_qubit_gate_count: 0,
                seed,
            },
        ))
    }
}

/// Opaque test circuit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct TestCircuit {
    circuit_index: usize,
    num_qubits: usize,
    seed: u64,
}

/// Deterministic executor.
///
/// Every circuit returns all shots in the all-zero state. This makes the
/// protocol-level expected XEB value analytically known.
#[derive(Debug, Default, Clone, Copy)]
struct DeterministicExecutor;

impl XebExecutor<TestCircuit> for DeterministicExecutor {
    fn execute(
        &mut self,
        circuit: &TestCircuit,
        shots: u64,
    ) -> Result<Vec<XebObservedSample>, XebError> {
        assert!(circuit.num_qubits > 0);

        let bitstring = "0".repeat(circuit.num_qubits);

        Ok(vec![XebObservedSample::new(bitstring, shots)])
    }
}

/// Executor that fails deterministically.
///
/// Used to verify that backend execution failures cross the protocol boundary
/// without being swallowed or converted into a false benchmark result.
#[derive(Debug, Default, Clone, Copy)]
struct FailingExecutor;

impl XebExecutor<TestCircuit> for FailingExecutor {
    fn execute(
        &mut self,
        _circuit: &TestCircuit,
        _shots: u64,
    ) -> Result<Vec<XebObservedSample>, XebError> {
        Err(XebError::Execution(
            "intentional production-test execution failure".to_string(),
        ))
    }
}

/// Generator that returns a malformed circuit width.
#[derive(Debug, Default, Clone, Copy)]
struct WrongWidthGenerator;

impl XebCircuitGenerator for WrongWidthGenerator {
    type Circuit = TestCircuit;

    fn generate(
        &mut self,
        config: &XebConfig,
        circuit_index: usize,
        seed: u64,
    ) -> Result<(Self::Circuit, XebCircuitMetadata), XebError> {
        Ok((
            TestCircuit {
                circuit_index,
                num_qubits: config.num_qubits,
                seed,
            },
            XebCircuitMetadata {
                circuit_id: format!("wrong-width-{circuit_index}"),
                num_qubits: config.num_qubits + 1,
                depth: 1,
                gate_count: 1,
                two_qubit_gate_count: 0,
                seed,
            },
        ))
    }
}

/// Executor that returns an invalid bitstring.
#[derive(Debug, Default, Clone, Copy)]
struct InvalidBitstringExecutor;

impl XebExecutor<TestCircuit> for InvalidBitstringExecutor {
    fn execute(
        &mut self,
        _circuit: &TestCircuit,
        shots: u64,
    ) -> Result<Vec<XebObservedSample>, XebError> {
        Ok(vec![XebObservedSample::new("invalid", shots)])
    }
}

// =============================================================================
// Basic mathematical correctness
// =============================================================================

#[test]
fn linear_xeb_perfect_one_qubit_distribution_is_correct() {
    // Ideal distribution:
    //
    // P(0) = 1
    // P(1) = 0
    //
    // Every experimental sample is 0.
    //
    // F_XEB = 2^1 * 1 - 1 = 1.
    let ideal = ideal_distribution_1q(1.0, 0.0);

    let samples = vec![XebObservedSample::new("0", 1_000)];

    let result =
        linear_xeb_from_counts(1, &samples, &ideal)
            .expect("perfect one-qubit XEB must succeed");

    assert_close(result, 1.0, EPSILON);
}

#[test]
fn linear_xeb_uniform_distribution_is_zero() {
    // For a one-qubit uniform ideal distribution:
    //
    // P(0) = P(1) = 1/2
    //
    // F_XEB = 2 * 1/2 - 1 = 0.
    let ideal = ideal_distribution_1q(0.5, 0.5);

    let samples = vec![XebObservedSample::new("0", 10_000)];

    let result =
        linear_xeb_from_counts(1, &samples, &ideal)
            .expect("uniform XEB must succeed");

    assert_close(result, 0.0, EPSILON);
}

#[test]
fn linear_xeb_can_be_negative_for_anti_ideal_samples() {
    // Ideal:
    // P(0) = 1
    // P(1) = 0
    //
    // Observing state 1 would make the score:
    //
    // 2 * 0 - 1 = -1.
    //
    // This verifies that the implementation does NOT incorrectly clamp XEB
    // into [0, 1].
    let ideal = ideal_distribution_1q(1.0, 0.0);

    let samples = vec![XebObservedSample::new("1", 100)];

    let result =
        linear_xeb_from_counts(1, &samples, &ideal);

    assert!(
        matches!(result, Err(XebError::ZeroIdealProbability { .. })),
        "observing an output with exactly zero ideal probability must be \
         rejected because cross-entropy/XEB analysis cannot assign a valid \
         logarithmic ideal probability to it"
    );
}

#[test]
fn linear_xeb_uses_shot_weighting() {
    // P(0) = 0.75
    // P(1) = 0.25
    //
    // 3 shots of 0 and 1 shot of 1:
    //
    // mean p = (3*.75 + 1*.25) / 4
    //         = .625
    //
    // XEB = 2*.625 - 1 = .25
    let ideal = ideal_distribution_1q(0.75, 0.25);

    let samples = vec![
        XebObservedSample::new("0", 3),
        XebObservedSample::new("1", 1),
    ];

    let result =
        linear_xeb_from_counts(1, &samples, &ideal)
            .expect("weighted XEB must succeed");

    assert_close(result, 0.25, EPSILON);
}

#[test]
fn mean_ideal_probability_uses_shot_weighting() {
    let ideal = ideal_distribution_1q(0.75, 0.25);

    let samples = vec![
        XebObservedSample::new("0", 3),
        XebObservedSample::new("1", 1),
    ];

    let result =
        crate::quantum::benchmarking::protocols::xeb::mean_ideal_probability(
            &samples,
            &ideal,
        )
        .expect("mean ideal probability must succeed");

    assert_close(result, 0.625, EPSILON);
}

// =============================================================================
// Entropy and cross entropy
// =============================================================================

#[test]
fn ideal_entropy_of_uniform_one_qubit_distribution_is_ln_two() {
    let ideal = ideal_distribution_1q(0.5, 0.5);

    let entropy =
        ideal_entropy(&ideal)
            .expect("uniform entropy must succeed");

    assert_close(entropy, std::f64::consts::LN_2, EPSILON);
}

#[test]
fn cross_entropy_matches_uniform_distribution() {
    let ideal = ideal_distribution_1q(0.5, 0.5);

    let samples = vec![XebObservedSample::new("0", 1_000)];

    let entropy =
        cross_entropy(&samples, &ideal)
            .expect("cross entropy must succeed");

    assert_close(entropy, std::f64::consts::LN_2, EPSILON);
}

#[test]
fn cross_entropy_difference_is_zero_for_matching_uniform_distribution() {
    let ideal = ideal_distribution_1q(0.5, 0.5);

    let samples = vec![
        XebObservedSample::new("0", 500),
        XebObservedSample::new("1", 500),
    ];

    let difference =
        cross_entropy_difference(&samples, &ideal)
            .expect("cross entropy difference must succeed");

    assert_close(difference, 0.0, EPSILON);
}

#[test]
fn ideal_entropy_ignores_zero_probability_terms() {
    let ideal = ideal_distribution_1q(1.0, 0.0);

    let entropy =
        ideal_entropy(&ideal)
            .expect("zero probability terms must be handled");

    assert_close(entropy, 0.0, EPSILON);
}

// =============================================================================
// Distribution validation
// =============================================================================

#[test]
fn invalid_probability_above_one_is_rejected() {
    let ideal = ideal_distribution_1q(1.1, -0.1);
    let samples = vec![XebObservedSample::new("0", 10)];

    let result =
        linear_xeb_from_counts(1, &samples, &ideal);

    assert!(
        matches!(
            result,
            Err(XebError::InvalidProbability { .. })
        ),
        "probabilities outside [0, 1] must be rejected"
    );
}

#[test]
fn invalid_negative_probability_is_rejected() {
    let ideal = ideal_distribution_1q(-0.1, 1.1);
    let samples = vec![XebObservedSample::new("0", 10)];

    let result =
        linear_xeb_from_counts(1, &samples, &ideal);

    assert!(
        matches!(
            result,
            Err(XebError::InvalidProbability { .. })
        ),
        "negative probabilities must be rejected"
    );
}

#[test]
fn non_finite_probability_is_rejected() {
    let ideal = ideal_distribution_1q(f64::NAN, 1.0);

    let samples = vec![XebObservedSample::new("0", 10)];

    let result =
        linear_xeb_from_counts(1, &samples, &ideal);

    assert!(
        matches!(
            result,
            Err(XebError::NonFiniteProbability { .. })
        ),
        "NaN probability must never enter XEB arithmetic"
    );
}

#[test]
fn infinite_probability_is_rejected() {
    let ideal = ideal_distribution_1q(f64::INFINITY, 0.0);

    let samples = vec![XebObservedSample::new("0", 10)];

    let result =
        linear_xeb_from_counts(1, &samples, &ideal);

    assert!(
        matches!(
            result,
            Err(XebError::NonFiniteProbability { .. })
        ),
        "infinite probability must be rejected"
    );
}

#[test]
fn malformed_bitstring_length_is_rejected() {
    let ideal = ideal_distribution_1q(0.5, 0.5);

    let samples = vec![XebObservedSample::new("00", 10)];

    let result =
        linear_xeb_from_counts(1, &samples, &ideal);

    assert!(
        matches!(
            result,
            Err(XebError::InvalidBitstring { .. })
        ),
        "a one-qubit benchmark must not accept a two-bit output"
    );
}

#[test]
fn non_binary_bitstring_is_rejected() {
    let ideal = BTreeMap::from([
        ("x".to_string(), 1.0),
    ]);

    let samples = vec![XebObservedSample::new("x", 10)];

    let result =
        linear_xeb_from_counts(1, &samples, &ideal);

    assert!(
        matches!(
            result,
            Err(XebError::InvalidBitstring { .. })
        ),
        "XEB output identifiers must be binary computational-basis strings"
    );
}

#[test]
fn missing_ideal_probability_is_rejected() {
    let ideal = BTreeMap::from([
        ("0".to_string(), 1.0),
    ]);

    let samples = vec![XebObservedSample::new("1", 10)];

    let result =
        linear_xeb_from_counts(1, &samples, &ideal);

    assert!(
        matches!(
            result,
            Err(XebError::MissingIdealProbability { .. })
        ),
        "every observed output must have an ideal probability"
    );
}

#[test]
fn zero_shot_observation_is_rejected() {
    let ideal = ideal_distribution_1q(0.5, 0.5);

    let samples = vec![XebObservedSample::new("0", 0)];

    let result =
        linear_xeb_from_counts(1, &samples, &ideal);

    assert!(
        matches!(
            result,
            Err(XebError::InvalidObservedShotCount { .. })
        ),
        "zero-shot observations are not valid observations"
    );
}

#[test]
fn empty_observation_set_is_rejected() {
    let ideal = ideal_distribution_1q(0.5, 0.5);

    let result =
        linear_xeb_from_counts(1, &[], &ideal);

    assert!(
        matches!(
            result,
            Err(XebError::EmptyObservedSamples)
        ),
        "XEB cannot analyze an empty sample set"
    );
}

#[test]
fn empty_ideal_distribution_is_rejected() {
    let ideal = BTreeMap::new();
    let samples = vec![XebObservedSample::new("0", 10)];

    let result =
        linear_xeb_from_counts(1, &samples, &ideal);

    assert!(
        matches!(
            result,
            Err(XebError::EmptyIdealDistribution)
        ),
        "an empty ideal distribution is invalid"
    );
}

// =============================================================================
// Configuration validation
// =============================================================================

#[test]
fn default_configuration_is_valid() {
    let config = XebConfig::default();

    config
        .validate()
        .expect("default XEB configuration must be valid");

    assert_eq!(config.seed, DEFAULT_SEED);
    assert_eq!(config.confidence_level, DEFAULT_CONFIDENCE_LEVEL);
    assert_eq!(config.max_qubits, DEFAULT_MAX_QUBITS);
    assert_eq!(config.max_circuits, DEFAULT_MAX_CIRCUITS);
    assert_eq!(
        config.max_shots_per_circuit,
        DEFAULT_MAX_SHOTS_PER_CIRCUIT
    );
    assert_eq!(config.max_total_shots, DEFAULT_MAX_TOTAL_SHOTS);
}

#[test]
fn zero_qubits_are_rejected() {
    let result = XebConfig::new(0, 1, 100, DEFAULT_SEED);

    assert!(
        matches!(
            result,
            Err(XebError::InvalidQubitCount { value: 0 })
        ),
        "zero-qubit XEB must be rejected"
    );
}

#[test]
fn zero_circuits_are_rejected() {
    let result = XebConfig::new(1, 0, 100, DEFAULT_SEED);

    assert!(
        matches!(
            result,
            Err(XebError::InvalidCircuitCount { value: 0 })
        ),
        "zero-circuit XEB must be rejected"
    );
}

#[test]
fn zero_shots_are_rejected() {
    let result = XebConfig::new(1, 1, 0, DEFAULT_SEED);

    assert!(
        matches!(
            result,
            Err(XebError::InvalidShotCount { value: 0 })
        ),
        "zero-shot XEB must be rejected"
    );
}

#[test]
fn invalid_confidence_level_is_rejected() {
    let mut config = XebConfig::default();
    config.confidence_level = f64::NAN;

    let result = config.validate();

    assert!(
        matches!(
            result,
            Err(XebError::InvalidConfidenceLevel { .. })
        ),
        "NaN confidence level must be rejected"
    );
}

#[test]
fn confidence_level_one_is_rejected() {
    let mut config = XebConfig::default();
    config.confidence_level = 1.0;

    let result = config.validate();

    assert!(
        matches!(
            result,
            Err(XebError::InvalidConfidenceLevel { .. })
        ),
        "confidence level 1.0 must be rejected"
    );
}

#[test]
fn configured_qubit_limit_is_enforced() {
    let mut config = XebConfig::default();
    config.max_qubits = 2;
    config.num_qubits = 3;

    let result = config.validate();

    assert!(
        matches!(
            result,
            Err(XebError::QubitLimitExceeded {
                requested: 3,
                maximum: 2
            })
        ),
        "XEB must enforce configured qubit limits"
    );
}

#[test]
fn configured_circuit_limit_is_enforced() {
    let mut config = XebConfig::default();
    config.max_circuits = 2;
    config.circuits = 3;

    let result = config.validate();

    assert!(
        matches!(
            result,
            Err(XebError::CircuitLimitExceeded {
                requested: 3,
                maximum: 2
            })
        ),
        "XEB must enforce configured circuit limits"
    );
}

#[test]
fn configured_shot_limit_is_enforced() {
    let mut config = XebConfig::default();
    config.max_shots_per_circuit = 100;
    config.shots_per_circuit = 101;

    let result = config.validate();

    assert!(
        matches!(
            result,
            Err(XebError::ShotsPerCircuitLimitExceeded {
                requested: 101,
                maximum: 100
            })
        ),
        "XEB must enforce per-circuit shot limits"
    );
}

#[test]
fn configured_total_shot_limit_is_enforced() {
    let mut config = XebConfig::default();
    config.circuits = 10;
    config.shots_per_circuit = 100;
    config.max_total_shots = 999;

    let result = config.validate();

    assert!(
        matches!(
            result,
            Err(XebError::TotalShotsLimitExceeded {
                requested: 1_000,
                maximum: 999
            })
        ),
        "XEB must enforce total experiment shot limits"
    );
}

#[test]
fn total_shot_calculation_is_deterministic() {
    let config =
        XebConfig::new(2, 25, 400, DEFAULT_SEED)
            .expect("test configuration must be valid");

    assert_eq!(config.total_shots(), 10_000);
}

// =============================================================================
// Ideal model semantics
// =============================================================================

#[test]
fn exact_ideal_model_has_exact_semantics() {
    let model =
        XebIdealModel::exact("test-reference", "1.0.0");

    model
        .validate()
        .expect("exact ideal model must validate");

    assert_eq!(model.kind, IdealModelKind::Exact);
    assert!(model.kind.is_exact());
    assert!(model.complete);
    assert_close(model.covered_probability, 1.0, EPSILON);
}

#[test]
fn approximate_model_can_be_constructed() {
    let model =
        XebIdealModel::approximate(
            "approx-reference",
            "2.0.0",
            0.75,
        )
        .expect("approximate model metadata must be valid");

    model
        .validate()
        .expect("approximate model must validate");

    assert_eq!(model.kind, IdealModelKind::Approximate);
    assert!(!model.complete);
    assert_close(model.covered_probability, 0.75, EPSILON);
}

#[test]
fn partial_model_can_be_constructed() {
    let model =
        XebIdealModel::partial(
            "partial-reference",
            "1.2.3",
            0.5,
        )
        .expect("partial model metadata must be valid");

    model
        .validate()
        .expect("partial model must validate");

    assert_eq!(model.kind, IdealModelKind::Partial);
    assert!(!model.complete);
    assert_close(model.covered_probability, 0.5, EPSILON);
}

#[test]
fn exact_model_rejects_incomplete_metadata() {
    let model = XebIdealModel {
        kind: IdealModelKind::Exact,
        source: "test".to_string(),
        algorithm_version: "1.0".to_string(),
        complete: false,
        covered_probability: 0.5,
    };

    let result = model.validate();

    assert!(
        matches!(result, Err(XebError::UnsupportedIdealModel { .. })),
        "exact model metadata must never masquerade as partial"
    );
}

#[test]
fn empty_ideal_model_source_is_rejected() {
    let model =
        XebIdealModel::exact("", "1.0");

    let result = model.validate();

    assert!(
        matches!(result, Err(XebError::UnsupportedIdealModel { .. })),
        "ideal-model provenance must identify its source"
    );
}

#[test]
fn empty_ideal_model_version_is_rejected() {
    let model =
        XebIdealModel::exact("reference", "");

    let result = model.validate();

    assert!(
        matches!(result, Err(XebError::UnsupportedIdealModel { .. })),
        "ideal-model provenance must identify its version"
    );
}

#[test]
fn partial_model_requires_explicit_opt_in() {
    let mut config =
        XebConfig::new(1, 1, 100, DEFAULT_SEED)
            .expect("configuration must be valid");

    config.require_exact_ideal_model = true;

    let model =
        XebIdealModel::partial(
            "partial",
            "1.0",
            0.5,
        )
        .expect("partial model metadata must be valid");

    let ideal = BTreeMap::from([
        ("0".to_string(), 0.5),
    ]);

    let execution = XebExecution {
        circuits: vec![
            XebCircuitExecution::new(
                XebCircuitMetadata {
                    circuit_id: "partial-test".to_string(),
                    num_qubits: 1,
                    depth: 1,
                    gate_count: 1,
                    two_qubit_gate_count: 0,
                    seed: DEFAULT_SEED,
                },
                vec![XebObservedSample::new("0", 100)],
            )
            .expect("execution fixture must be valid"),
        ],
    };

    let result =
        XebProtocol::new().analyze(
            &config,
            model,
            &ideal,
            &execution,
        );

    assert!(
        matches!(
            result,
            Err(XebError::UnsupportedIdealModel { .. })
        ),
        "partial ideal distributions must not silently become exact XEB"
    );
}

#[test]
fn partial_model_can_be_used_when_explicitly_allowed() {
    let mut config =
        XebConfig::new(1, 1, 100, DEFAULT_SEED)
            .expect("configuration must be valid");

    config.require_exact_ideal_model = false;

    let model =
        XebIdealModel::partial(
            "partial",
            "1.0",
            0.5,
        )
        .expect("partial model metadata must be valid");

    let ideal = BTreeMap::from([
        ("0".to_string(), 0.5),
    ]);

    let execution = XebExecution {
        circuits: vec![
            XebCircuitExecution::new(
                XebCircuitMetadata {
                    circuit_id: "partial-test".to_string(),
                    num_qubits: 1,
                    depth: 1,
                    gate_count: 1,
                    two_qubit_gate_count: 0,
                    seed: DEFAULT_SEED,
                },
                vec![XebObservedSample::new("0", 100)],
            )
            .expect("execution fixture must be valid"),
        ],
    };

    let result =
        XebProtocol::new().analyze(
            &config,
            model,
            &ideal,
            &execution,
        )
        .expect("explicit partial XEB analysis must succeed");

    assert_eq!(
        result.ideal_model.kind,
        IdealModelKind::Partial
    );

    assert!(
        result.aggregate_ideal_entropy.is_none(),
        "partial ideal distributions must not report complete ideal entropy"
    );

    assert!(
        result.aggregate_cross_entropy_difference.is_none(),
        "cross-entropy difference requires a complete ideal entropy"
    );
}

// =============================================================================
// Circuit and execution contracts
// =============================================================================

#[test]
fn circuit_metadata_rejects_empty_identifier() {
    let metadata = XebCircuitMetadata {
        circuit_id: String::new(),
        num_qubits: 1,
        depth: 1,
        gate_count: 1,
        two_qubit_gate_count: 0,
        seed: DEFAULT_SEED,
    };

    let result = metadata.validate(1);

    assert!(
        matches!(result, Err(XebError::Generation(_))),
        "circuit IDs are part of reproducibility/provenance and cannot be empty"
    );
}

#[test]
fn circuit_metadata_rejects_wrong_width() {
    let metadata = XebCircuitMetadata {
        circuit_id: "wrong-width".to_string(),
        num_qubits: 2,
        depth: 1,
        gate_count: 1,
        two_qubit_gate_count: 0,
        seed: DEFAULT_SEED,
    };

    let result = metadata.validate(1);

    assert!(
        matches!(
            result,
            Err(XebError::CircuitWidthMismatch {
                expected: 1,
                actual: 2
            })
        ),
        "XEB must prevent a circuit from being analyzed under the wrong width"
    );
}

#[test]
fn observed_sample_validation_rejects_wrong_width() {
    let sample =
        XebObservedSample::new("00", 10);

    let result = sample.validate(1);

    assert!(
        matches!(
            result,
            Err(XebError::InvalidBitstring { .. })
        )
    );
}

#[test]
fn circuit_execution_rejects_empty_samples() {
    let circuit =
        XebCircuitMetadata {
            circuit_id: "empty".to_string(),
            num_qubits: 1,
            depth: 1,
            gate_count: 1,
            two_qubit_gate_count: 0,
            seed: DEFAULT_SEED,
        };

    let execution =
        XebCircuitExecution::new(circuit, vec![]);

    let result =
        execution.and_then(|execution| {
            execution.validate(1, Some(10))
        });

    assert!(
        matches!(
            result,
            Err(XebError::EmptyObservedSamples)
        )
    );
}

#[test]
fn circuit_execution_detects_shot_mismatch() {
    let circuit =
        XebCircuitMetadata {
            circuit_id: "shot-mismatch".to_string(),
            num_qubits: 1,
            depth: 1,
            gate_count: 1,
            two_qubit_gate_count: 0,
            seed: DEFAULT_SEED,
        };

    let execution =
        XebCircuitExecution::new(
            circuit,
            vec![
                XebObservedSample::new("0", 50),
            ],
        )
        .expect("execution fixture must be constructible");

    let result =
        execution.validate(1, Some(100));

    assert!(
        matches!(
            result,
            Err(XebError::ObservedShotsMismatch {
                expected: 100,
                actual: 50
            })
        )
    );
}

#[test]
fn execution_detects_circuit_count_mismatch() {
    let config =
        XebConfig::new(1, 2, 100, DEFAULT_SEED)
            .expect("configuration must be valid");

    let execution = XebExecution {
        circuits: vec![],
    };

    let result =
        execution.validate(&config);

    assert!(
        matches!(
            result,
            Err(XebError::CircuitCountMismatch {
                expected: 2,
                actual: 0
            })
        )
    );
}

// =============================================================================
// Deterministic reproducibility
// =============================================================================

#[test]
fn circuit_seed_derivation_is_deterministic() {
    let expected = [
        0xce32_48dc_9d5c_fdea,
        0x899d_7f07_92a0_7415,
        0x37a3_5ac3_41e8_1fac,
        0xcdde_37a8_03f4_4d1b,
        0x0683_a7ab_13aa_8119,
    ];

    for (index, expected_seed) in expected.iter().copied().enumerate() {
        assert_eq!(
            derive_circuit_seed(
                DEFAULT_SEED,
                index as u64
            ),
            expected_seed,
            "circuit seed derivation changed for circuit index {index}"
        );
    }
}

#[test]
fn circuit_seed_changes_with_circuit_index() {
    let first =
        derive_circuit_seed(DEFAULT_SEED, 0);

    let second =
        derive_circuit_seed(DEFAULT_SEED, 1);

    assert_ne!(
        first,
        second,
        "independent circuit streams must not reuse the same seed"
    );
}

#[test]
fn circuit_seed_changes_with_experiment_seed() {
    let first =
        derive_circuit_seed(DEFAULT_SEED, 0);

    let second =
        derive_circuit_seed(
            DEFAULT_SEED.wrapping_add(1),
            0,
        );

    assert_ne!(
        first,
        second,
        "different experiment seeds must create independent circuit streams"
    );
}

// =============================================================================
// Full protocol integration
// =============================================================================

#[test]
fn full_protocol_run_produces_expected_result() {
    // One qubit, two circuits, 100 shots each.
    //
    // Ideal distribution:
    // P(0)=1, P(1)=0.
    //
    // Executor always returns 0.
    //
    // Therefore every circuit has:
    // mean ideal probability = 1
    // Linear XEB = 1.
    let config =
        XebConfig::new(1, 2, 100, DEFAULT_SEED)
            .expect("configuration must be valid");

    let ideal = ideal_distribution_1q(1.0, 0.0);

    let model =
        XebIdealModel::exact(
            "deterministic-test-reference",
            "1.0.0",
        );

    let mut generator =
        deterministic_one_qubit_generator();

    let mut executor =
        deterministic_one_qubit_executor();

    let result =
        XebProtocol::new()
            .run(
                &config,
                model,
                &ideal,
                &mut generator,
                &mut executor,
            )
            .expect("full XEB protocol must succeed");

    assert_eq!(
        result.schema_version,
        XEB_RESULT_SCHEMA_VERSION
    );

    assert_eq!(
        result.benchmark_id,
        XEB_BENCHMARK_ID
    );

    assert_eq!(
        result.protocol_version,
        XEB_PROTOCOL_VERSION
    );

    assert_eq!(result.num_qubits, 1);
    assert_eq!(result.circuits, 2);
    assert_eq!(result.total_shots, 200);
    assert_eq!(result.seed, DEFAULT_SEED);

    assert_close(
        result.linear_xeb_mean,
        1.0,
        EPSILON,
    );

    assert_close(
        result.linear_xeb_min,
        1.0,
        EPSILON,
    );

    assert_close(
        result.linear_xeb_max,
        1.0,
        EPSILON,
    );

    assert_close(
        result.linear_xeb_standard_error,
        0.0,
        EPSILON,
    );

    assert_close(
        result.linear_xeb_confidence_interval.lower,
        1.0,
        EPSILON,
    );

    assert_close(
        result.linear_xeb_confidence_interval.upper,
        1.0,
        EPSILON,
    );

    assert_eq!(
        result.circuit_results.len(),
        2
    );

    assert!(
        result.statistically_descriptive,
        "XEB must remain explicitly descriptive rather than silently becoming \
         a physical-fidelity claim"
    );

    result
        .validate()
        .expect("complete XEB result must validate");
}

#[test]
fn full_protocol_is_reproducible() {
    let config =
        XebConfig::new(1, 3, 100, DEFAULT_SEED)
            .expect("configuration must be valid");

    let ideal = ideal_distribution_1q(1.0, 0.0);

    let model =
        XebIdealModel::exact(
            "deterministic-reference",
            "1.0.0",
        );

    let mut generator_a =
        deterministic_one_qubit_generator();

    let mut executor_a =
        deterministic_one_qubit_executor();

    let result_a =
        XebProtocol::new()
            .run(
                &config,
                model.clone(),
                &ideal,
                &mut generator_a,
                &mut executor_a,
            )
            .expect("first deterministic run must succeed");

    let mut generator_b =
        deterministic_one_qubit_generator();

    let mut executor_b =
        deterministic_one_qubit_executor();

    let result_b =
        XebProtocol::new()
            .run(
                &config,
                model,
                &ideal,
                &mut generator_b,
                &mut executor_b,
            )
            .expect("second deterministic run must succeed");

    assert_eq!(
        result_a,
        result_b,
        "same configuration, seed, generator and observations must produce \
         byte-for-byte-equivalent Rust result structures"
    );
}

#[test]
fn protocol_rejects_generator_width_mismatch() {
    let config =
        XebConfig::new(1, 1, 100, DEFAULT_SEED)
            .expect("configuration must be valid");

    let ideal = ideal_distribution_1q(0.5, 0.5);

    let model =
        XebIdealModel::exact(
            "reference",
            "1.0.0",
        );

    let mut generator = WrongWidthGenerator;
    let mut executor =
        deterministic_one_qubit_executor();

    let result =
        XebProtocol::new()
            .run(
                &config,
                model,
                &ideal,
                &mut generator,
                &mut executor,
            );

    assert!(
        matches!(
            result,
            Err(XebError::CircuitWidthMismatch {
                expected: 1,
                actual: 2
            })
        ),
        "protocol must reject malformed generator output before execution"
    );
}

#[test]
fn protocol_rejects_invalid_executor_observation() {
    let config =
        XebConfig::new(1, 1, 100, DEFAULT_SEED)
            .expect("configuration must be valid");

    let ideal = ideal_distribution_1q(0.5, 0.5);

    let model =
        XebIdealModel::exact(
            "reference",
            "1.0.0",
        );

    let mut generator =
        deterministic_one_qubit_generator();

    let mut executor =
        InvalidBitstringExecutor;

    let result =
        XebProtocol::new()
            .run(
                &config,
                model,
                &ideal,
                &mut generator,
                &mut executor,
            );

    assert!(
        matches!(
            result,
            Err(XebError::InvalidBitstring { .. })
        ),
        "protocol must validate backend observations before statistical analysis"
    );
}

#[test]
fn protocol_propagates_executor_failure() {
    let config =
        XebConfig::new(1, 1, 100, DEFAULT_SEED)
            .expect("configuration must be valid");

    let ideal = ideal_distribution_1q(0.5, 0.5);

    let model =
        XebIdealModel::exact(
            "reference",
            "1.0.0",
        );

    let mut generator =
        deterministic_one_qubit_generator();

    let mut executor =
        FailingExecutor;

    let result =
        XebProtocol::new()
            .run(
                &config,
                model,
                &ideal,
                &mut generator,
                &mut executor,
            );

    assert!(
        matches!(
            result,
            Err(XebError::Execution(_))
        ),
        "backend execution failures must cross the protocol boundary unchanged"
    );
}

// =============================================================================
// Result validation
// =============================================================================

#[test]
fn result_rejects_wrong_schema_version() {
    let config =
        XebConfig::new(1, 1, 100, DEFAULT_SEED)
            .expect("configuration must be valid");

    let ideal = ideal_distribution_1q(1.0, 0.0);

    let model =
        XebIdealModel::exact(
            "reference",
            "1.0.0",
        );

    let mut generator =
        deterministic_one_qubit_generator();

    let mut executor =
        deterministic_one_qubit_executor();

    let mut result =
        XebProtocol::new()
            .run(
                &config,
                model,
                &ideal,
                &mut generator,
                &mut executor,
            )
            .expect("protocol must succeed");

    result.schema_version =
        XEB_RESULT_SCHEMA_VERSION + 1;

    let validation = result.validate();

    assert!(
        matches!(
            validation,
            Err(XebError::UnsupportedIdealModel { .. })
        ),
        "unsupported result schemas must be rejected"
    );
}

#[test]
fn result_rejects_wrong_benchmark_identifier() {
    let config =
        XebConfig::new(1, 1, 100, DEFAULT_SEED)
            .expect("configuration must be valid");

    let ideal = ideal_distribution_1q(1.0, 0.0);

    let model =
        XebIdealModel::exact(
            "reference",
            "1.0.0",
        );

    let mut generator =
        deterministic_one_qubit_generator();

    let mut executor =
        deterministic_one_qubit_executor();

    let mut result =
        XebProtocol::new()
            .run(
                &config,
                model,
                &ideal,
                &mut generator,
                &mut executor,
            )
            .expect("protocol must succeed");

    result.benchmark_id =
        "not_xeb".to_string();

    let validation = result.validate();

    assert!(
        matches!(
            validation,
            Err(XebError::UnsupportedIdealModel { .. })
        ),
        "a result must never be accepted under another benchmark identifier"
    );
}

#[test]
fn result_rejects_circuit_result_count_mismatch() {
    let config =
        XebConfig::new(1, 2, 100, DEFAULT_SEED)
            .expect("configuration must be valid");

    let ideal = ideal_distribution_1q(1.0, 0.0);

    let model =
        XebIdealModel::exact(
            "reference",
            "1.0.0",
        );

    let mut generator =
        deterministic_one_qubit_generator();

    let mut executor =
        deterministic_one_qubit_executor();

    let mut result =
        XebProtocol::new()
            .run(
                &config,
                model,
                &ideal,
                &mut generator,
                &mut executor,
            )
            .expect("protocol must succeed");

    result.circuit_results.pop();

    let validation = result.validate();

    assert!(
        matches!(
            validation,
            Err(XebError::CircuitCountMismatch {
                expected: 2,
                actual: 1
            })
        ),
        "result metadata and circuit results must remain consistent"
    );
}

#[test]
fn result_circuit_xeb_values_are_in_protocol_order() {
    let config =
        XebConfig::new(1, 2, 100, DEFAULT_SEED)
            .expect("configuration must be valid");

    let ideal = ideal_distribution_1q(1.0, 0.0);

    let model =
        XebIdealModel::exact(
            "reference",
            "1.0.0",
        );

    let mut generator =
        deterministic_one_qubit_generator();

    let mut executor =
        deterministic_one_qubit_executor();

    let result =
        XebProtocol::new()
            .run(
                &config,
                model,
                &ideal,
                &mut generator,
                &mut executor,
            )
            .expect("protocol must succeed");

    let values =
        result.circuit_xeb_values();

    assert_eq!(values.len(), 2);

    for value in values {
        assert_close(value, 1.0, EPSILON);
    }
}

// =============================================================================
// Serialization / persistence contract
// =============================================================================

#[test]
fn result_round_trips_through_json() {
    let config =
        XebConfig::new(1, 2, 100, DEFAULT_SEED)
            .expect("configuration must be valid");

    let ideal = ideal_distribution_1q(1.0, 0.0);

    let model =
        XebIdealModel::exact(
            "json-test-reference",
            "1.0.0",
        );

    let mut generator =
        deterministic_one_qubit_generator();

    let mut executor =
        deterministic_one_qubit_executor();

    let result =
        XebProtocol::new()
            .run(
                &config,
                model,
                &ideal,
                &mut generator,
                &mut executor,
            )
            .expect("protocol must succeed");

    let encoded =
        serde_json::to_string(&result)
            .expect("XEB result must serialize as JSON");

    assert!(
        encoded.contains("\"benchmark_id\":\"xeb\""),
        "serialized XEB results must retain their benchmark identity"
    );

    assert!(
        encoded.contains("\"schema_version\":1"),
        "serialized XEB results must retain their schema version"
    );

    let decoded: XebResult =
        serde_json::from_str(&encoded)
            .expect("serialized XEB result must deserialize");

    assert_eq!(
        result,
        decoded,
        "XEB result JSON serialization must be lossless"
    );
}

#[test]
fn ideal_model_serialization_preserves_kind() {
    let model =
        XebIdealModel::exact(
            "serialization-reference",
            "3.0.0",
        );

    let encoded =
        serde_json::to_string(&model)
            .expect("ideal model must serialize");

    let decoded: XebIdealModel =
        serde_json::from_str(&encoded)
            .expect("ideal model must deserialize");

    assert_eq!(model, decoded);
    assert_eq!(decoded.kind, IdealModelKind::Exact);
}

// =============================================================================
// Two-qubit regression fixture
// =============================================================================

#[test]
fn two_qubit_uniform_fixture_has_expected_entropy() {
    let ideal = ideal_distribution_2q();

    let entropy =
        ideal_entropy(&ideal)
            .expect("two-qubit uniform entropy must succeed");

    assert_close(
        entropy,
        2.0 * std::f64::consts::LN_2,
        EPSILON,
    );
}

#[test]
fn two_qubit_uniform_fixture_has_zero_linear_xeb() {
    let ideal = ideal_distribution_2q();

    let samples = vec![
        XebObservedSample::new("00", 250),
        XebObservedSample::new("01", 250),
        XebObservedSample::new("10", 250),
        XebObservedSample::new("11", 250),
    ];

    let score =
        linear_xeb_from_counts(
            2,
            &samples,
            &ideal,
        )
        .expect("two-qubit uniform XEB must succeed");

    assert_close(score, 0.0, EPSILON);
}

// =============================================================================
// Statistical sanity
// =============================================================================

#[test]
fn protocol_confidence_interval_is_finite() {
    let config =
        XebConfig::new(1, 4, 100, DEFAULT_SEED)
            .expect("configuration must be valid");

    let ideal = ideal_distribution_1q(0.75, 0.25);

    let model =
        XebIdealModel::exact(
            "statistics-reference",
            "1.0.0",
        );

    let mut generator =
        deterministic_one_qubit_generator();

    let mut executor =
        deterministic_one_qubit_executor();

    let result =
        XebProtocol::new()
            .run(
                &config,
                model,
                &ideal,
                &mut generator,
                &mut executor,
            )
            .expect("protocol must succeed");

    assert_finite(
        result.linear_xeb_mean,
        "linear_xeb_mean",
    );

    assert_finite(
        result.linear_xeb_standard_error,
        "linear_xeb_standard_error",
    );

    assert_finite(
        result.linear_xeb_confidence_interval.lower,
        "confidence lower bound",
    );

    assert_finite(
        result.linear_xeb_confidence_interval.upper,
        "confidence upper bound",
    );

    assert!(
        result.linear_xeb_confidence_interval.lower
            <= result.linear_xeb_confidence_interval.upper
    );
}

#[test]
fn identical_circuit_scores_have_zero_standard_error() {
    let config =
        XebConfig::new(1, 8, 100, DEFAULT_SEED)
            .expect("configuration must be valid");

    let ideal = ideal_distribution_1q(1.0, 0.0);

    let model =
        XebIdealModel::exact(
            "zero-error-reference",
            "1.0.0",
        );

    let mut generator =
        deterministic_one_qubit_generator();

    let mut executor =
        deterministic_one_qubit_executor();

    let result =
        XebProtocol::new()
            .run(
                &config,
                model,
                &ideal,
                &mut generator,
                &mut executor,
            )
            .expect("protocol must succeed");

    assert_close(
        result.linear_xeb_standard_error,
        0.0,
        EPSILON,
    );
}

// =============================================================================
// Regression checks for numerical/statistical boundaries
// =============================================================================

#[test]
fn confidence_interval_contains_mean_for_zero_variance_case() {
    let config =
        XebConfig::new(1, 5, 100, DEFAULT_SEED)
            .expect("configuration must be valid");

    let ideal = ideal_distribution_1q(1.0, 0.0);

    let model =
        XebIdealModel::exact(
            "confidence-reference",
            "1.0.0",
        );

    let mut generator =
        deterministic_one_qubit_generator();

    let mut executor =
        deterministic_one_qubit_executor();

    let result =
        XebProtocol::new()
            .run(
                &config,
                model,
                &ideal,
                &mut generator,
                &mut executor,
            )
            .expect("protocol must succeed");

    assert!(
        result.linear_xeb_confidence_interval.lower
            <= result.linear_xeb_mean
    );

    assert!(
        result.linear_xeb_mean
            <= result.linear_xeb_confidence_interval.upper
    );
}

#[test]
fn distribution_tolerance_constant_is_small_enough_for_scientific_results() {
    assert!(
        DISTRIBUTION_TOLERANCE > 0.0,
        "distribution tolerance must be positive"
    );

    assert!(
        DISTRIBUTION_TOLERANCE <= 1.0e-10,
        "distribution tolerance must remain sufficiently strict for benchmark data"
    );
}

// =============================================================================
// Protocol identity regression
// =============================================================================

#[test]
fn protocol_identity_is_stable() {
    assert_eq!(XEB_BENCHMARK_ID, "xeb");
    assert_eq!(XEB_PROTOCOL_VERSION, "1.0.0");
    assert_eq!(XEB_RESULT_SCHEMA_VERSION, 1);
}

// =============================================================================
// Public API consistency
// =============================================================================

#[test]
fn xeb_protocol_constructor_is_stateless() {
    let first = XebProtocol::new();
    let second = XebProtocol::new();

    assert_eq!(first, second);
}

#[test]
fn execution_total_shots_matches_circuit_totals() {
    let execution = XebExecution {
        circuits: vec![
            XebCircuitExecution::new(
                XebCircuitMetadata {
                    circuit_id: "a".to_string(),
                    num_qubits: 1,
                    depth: 1,
                    gate_count: 1,
                    two_qubit_gate_count: 0,
                    seed: 1,
                },
                vec![XebObservedSample::new("0", 25)],
            )
            .expect("fixture must be valid"),
            XebCircuitExecution::new(
                XebCircuitMetadata {
                    circuit_id: "b".to_string(),
                    num_qubits: 1,
                    depth: 2,
                    gate_count: 2,
                    two_qubit_gate_count: 0,
                    seed: 2,
                },
                vec![XebObservedSample::new("0", 75)],
            )
            .expect("fixture must be valid"),
        ],
    };

    assert_eq!(
        execution
            .total_shots()
            .expect("shot total must be calculable"),
        100
    );
}

// =============================================================================
// End-of-file production invariant
// =============================================================================
//
// The following invariant is intentionally expressed as a test rather than
// documentation only:
//
// XEB results must never silently become a physical-fidelity assertion.
// The current protocol marks every result as statistically descriptive.
// This test protects that scientific boundary from accidental removal.

#[test]
fn xeb_result_remains_statistically_descriptive() {
    let config =
        XebConfig::new(1, 1, 100, DEFAULT_SEED)
            .expect("configuration must be valid");

    let ideal = ideal_distribution_1q(1.0, 0.0);

    let model =
        XebIdealModel::exact(
            "scientific-boundary-reference",
            "1.0.0",
        );

    let mut generator =
        deterministic_one_qubit_generator();

    let mut executor =
        deterministic_one_qubit_executor();

    let result =
        XebProtocol::new()
            .run(
                &config,
                model,
                &ideal,
                &mut generator,
                &mut executor,
            )
            .expect("protocol must succeed");

    assert!(
        result.statistically_descriptive,
        "Linear XEB must not silently be presented as an unconditional physical fidelity"
    );
}