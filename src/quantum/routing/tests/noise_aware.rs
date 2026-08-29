//! Zamani Quantum Routing — Noise-Aware Router Test Suite
//!
//! `src/quantum/routing/tests/noise_aware.rs`
//!
//! Production integration tests for the noise-aware routing subsystem.
//!
//! # Purpose
//!
//! This module verifies the public, externally observable contract of
//! `NoiseAwareRouter` and its `NoiseModel` abstraction.
//!
//! The tests cover:
//!
//! - construction and stable algorithm identity;
//! - default configuration compatibility;
//! - explicit noise-model injection;
//! - shared noise-model injection;
//! - candidate-route configuration;
//! - weight validation;
//! - probability validation;
//! - duration accounting;
//! - independent-operation error accumulation;
//! - exact certainty-of-failure handling;
//! - zero-error fidelity;
//! - unknown calibration handling;
//! - calibrated versus uncalibrated operation accounting;
//! - custom hardware noise models;
//! - route scoring;
//! - weighted objective calculation;
//! - SWAP-count tie-breaking;
//! - deterministic route selection;
//! - reproducibility;
//! - result noise metrics;
//! - result quality metrics;
//! - caller-input immutability;
//! - mapping preservation;
//! - physical routing correctness;
//! - standard verification compatibility;
//! - algorithm-trait integration;
//! - `Auto` configuration compatibility;
//! - invalid algorithm rejection;
//! - invalid configuration rejection;
//! - candidate-count limits;
//! - numerical safety;
//! - non-finite calibration rejection;
//! - duration overflow protection at the model boundary;
//! - operation-count accounting;
//! - multi-operation route scoring;
//! - large-but-bounded route scoring;
//! - repeated execution stability;
//! - no global mutable state;
//! - no unsafe code.
//!
//! # Architectural boundary
//!
//! These tests deliberately consume the routing subsystem through its public
//! contracts rather than reaching into private implementation details.
//!
//! The dependency boundary is:
//!
//! ```text
//! quantum::routing::types
//!          │
//!          ├── QuantumOperation
//!          ├── RoutingOperation
//!          └── RoutingMove
//!
//! quantum::routing::topology
//!          │
//!          └── Topology
//!
//! quantum::routing::mapping
//!          │
//!          └── QubitMapping
//!
//! quantum::routing::config
//!          │
//!          └── RoutingConfig
//!
//! quantum::routing::result
//!          │
//!          └── RoutingResult
//!
//! quantum::routing::algorithms::noise_aware
//!          │
//!          ├── NoiseAwareRouter
//!          ├── NoiseModel
//!          ├── NoiseEstimate
//!          └── RouteNoiseScore
//! ```
//!
//! No test depends on:
//!
//! - compiler IR implementation details;
//! - OpenQASM;
//! - hardware-provider SDKs;
//! - filesystem access;
//! - network access;
//! - environment variables;
//! - wall-clock sleeps;
//! - global mutable state;
//! - unsafe Rust.
//!
//! # Integration contract
//!
//! This file assumes the following stable contracts already provided by the
//! routing subsystem:
//!
//! 1. `NoiseAwareRouter::new()` constructs a valid fallback router.
//! 2. `NoiseAwareRouter::with_noise_model()` accepts a `NoiseModel`.
//! 3. `NoiseAwareRouter::with_shared_noise_model()` accepts an `Arc` model.
//! 4. `NoiseAwareRouter::score_route()` scores semantic routing operations.
//! 5. `NoiseAwareRouter::route()` accepts a `RoutingInput`.
//! 6. `RoutingInput::new()` accepts operations, topology, mapping and config.
//! 7. `RoutingOperation::Move(RoutingMove::Swap { .. })` represents semantic
//!    movement.
//! 8. `RoutingOperation::Gate { .. }` represents a routed gate.
//! 9. `RoutingResult` exposes operations, metrics, mappings and quality.
//! 10. `Topology::line()` constructs a deterministic connected topology.
//! 11. `QubitMapping` provides authoritative logical-to-physical mapping.
//! 12. `RoutingConfig` provides algorithm selection and routing limits.
//!
//! These tests intentionally do not require changes to the implementation
//! merely because additional routing algorithms are introduced later.
//!
//! # Production test philosophy
//!
//! The tests assert contractual invariants instead of fragile implementation
//! details whenever the implementation does not explicitly promise an exact
//! route.
//!
//! In particular, a noise-aware heuristic is not required to produce one exact
//! SWAP sequence unless the public contract guarantees that sequence.
//!
//! The suite therefore prefers:
//!
//! - correctness;
//! - finite numerical values;
//! - valid probabilities;
//! - deterministic repeated execution;
//! - correct mapping evolution;
//! - correct metric population;
//! - safe failure;
//! - reproducibility;
//! - objective consistency.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features.
//!
//! # Safety
//!
//! This module contains no unsafe code.
//!
//! ```text
//! #![deny(unsafe_code)]
//! #![deny(unsafe_op_in_unsafe_fn)]
//! #![deny(unused_must_use)]
//! ```
//!
//! # Definition of done
//!
//! This test file is complete when:
//!
//! - the noise-model contract is exercised;
//! - invalid calibration cannot silently enter scoring;
//! - exact probability boundaries are tested;
//! - duration accumulation is tested;
//! - weighted scoring is tested;
//! - unknown calibration is observable;
//! - custom hardware models are supported;
//! - routed results expose noise metrics;
//! - deterministic execution is tested;
//! - mapping correctness is tested;
//! - input immutability is tested;
//! - algorithm-trait integration is tested;
//! - invalid configuration is rejected;
//! - candidate limits are tested;
//! - numerical safety is tested;
//! - no unsafe code is permitted.
//!
//! The test module is deliberately self-contained and does not require any
//! subsequent source-file modifications.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::sync::{
    atomic::{AtomicUsize, Ordering as AtomicOrdering},
    Arc,
};
use std::time::Duration;

use crate::quantum::routing::algorithms::noise_aware::{
    ConservativeNoiseModel,
    NoiseAwareRouter,
    NoiseEstimate,
    NoiseModel,
    RouteNoiseScore,
    DEFAULT_CANDIDATE_ROUTES,
    DEFAULT_DURATION_WEIGHT,
    DEFAULT_ERROR_WEIGHT,
    DEFAULT_SWAP_WEIGHT,
    MAX_CANDIDATE_ROUTES,
    MAX_WEIGHT,
};

use crate::quantum::routing::algorithms::RoutingAlgorithm as RoutingAlgorithmTrait;

use crate::quantum::routing::config::{
    RoutingAlgorithm,
    RoutingConfig,
};

use crate::quantum::routing::errors::RoutingError;

use crate::quantum::routing::mapping::QubitMapping;

use crate::quantum::routing::result::{
    RoutingInput,
    RoutingResult,
};

use crate::quantum::routing::topology::Topology;

use crate::quantum::routing::types::{
    GateIdentity,
    LogicalQubitId,
    PhysicalQubitId,
    QuantumOperation,
    RoutingMove,
    RoutingOperation,
};

// =============================================================================
// Test helpers
// =============================================================================

fn lq(index: usize) -> LogicalQubitId {
    LogicalQubitId::new(index)
}

fn pq(index: usize) -> PhysicalQubitId {
    PhysicalQubitId::new(index)
}

fn line_topology(count: usize) -> Topology {
    Topology::line(count)
        .expect("production test topology must be valid")
}

fn identity_mapping(count: usize) -> QubitMapping {
    let mut mapping = QubitMapping::new();

    for index in 0..count {
        mapping
            .assign(lq(index), pq(index))
            .expect("identity mapping must be valid");
    }

    mapping
}

fn test_config() -> RoutingConfig {
    let mut config = RoutingConfig::default();
    config.algorithm = RoutingAlgorithm::NoiseAware;
    config
}

fn auto_config() -> RoutingConfig {
    let mut config = RoutingConfig::default();
    config.algorithm = RoutingAlgorithm::Auto;
    config
}

fn two_qubit_gate(
    gate: GateIdentity,
    first: usize,
    second: usize,
) -> QuantumOperation {
    QuantumOperation::new(
        gate,
        vec![lq(first), lq(second)],
    )
}

fn single_qubit_gate(
    gate: GateIdentity,
    qubit: usize,
) -> QuantumOperation {
    QuantumOperation::new(
        gate,
        vec![lq(qubit)],
    )
}

fn swap_count(
    operations: &[RoutingOperation],
) -> usize {
    operations
        .iter()
        .filter(|operation| {
            matches!(
                operation,
                RoutingOperation::Move(
                    RoutingMove::Swap { .. }
                )
            )
        })
        .count()
}

fn route_signature(
    result: &RoutingResult,
) -> Vec<String> {
    result
        .operations()
        .iter()
        .map(|operation| format!("{operation:?}"))
        .collect()
}

fn assert_valid_score(score: &RouteNoiseScore) {
    assert!(
        score.is_valid(),
        "noise score must satisfy its numerical invariants: {score:?}"
    );

    assert!(
        score.error_probability.is_finite(),
        "estimated error must be finite"
    );

    assert!(
        score.fidelity.is_finite(),
        "estimated fidelity must be finite"
    );

    assert!(
        score.weighted_score.is_finite(),
        "weighted score must be finite"
    );

    assert!(
        (0.0..=1.0).contains(&score.error_probability),
        "error probability must be in [0, 1]"
    );

    assert!(
        (0.0..=1.0).contains(&score.fidelity),
        "fidelity must be in [0, 1]"
    );
}

// =============================================================================
// Test noise models
// =============================================================================

/// A deterministic calibrated model used to exercise the hardware-provider
/// integration boundary.
///
/// Every SWAP has a configurable error/duration while ordinary gates receive
/// their own configurable calibration.
///
/// The model contains no topology logic and no compiler logic. That is
/// intentional: a real hardware adapter can implement the same contract.
#[derive(Debug, Clone, Copy)]
struct CalibratedTestModel {
    gate_error: f64,
    gate_duration: Duration,
    swap_error: f64,
    swap_duration: Duration,
}

impl CalibratedTestModel {
    fn new(
        gate_error: f64,
        gate_duration: Duration,
        swap_error: f64,
        swap_duration: Duration,
    ) -> Self {
        Self {
            gate_error,
            gate_duration,
            swap_error,
            swap_duration,
        }
    }
}

impl NoiseModel for CalibratedTestModel {
    fn estimate(
        &self,
        operation: &RoutingOperation,
    ) -> Result<NoiseEstimate, RoutingError> {
        let (error, duration) = match operation {
            RoutingOperation::Move(
                RoutingMove::Swap { .. },
            ) => (
                self.swap_error,
                self.swap_duration,
            ),

            RoutingOperation::Move(
                RoutingMove::Bridge { .. },
            ) => (
                self.swap_error,
                self.swap_duration,
            ),

            RoutingOperation::Move(
                RoutingMove::Permutation { .. },
            ) => (
                self.swap_error,
                self.swap_duration,
            ),

            RoutingOperation::Gate { .. } => (
                self.gate_error,
                self.gate_duration,
            ),

            RoutingOperation::Barrier { .. } => (
                0.0,
                Duration::ZERO,
            ),
        };

        NoiseEstimate::new(
            error,
            duration,
            true,
        )
    }

    fn name(&self) -> &'static str {
        "calibrated-test"
    }

    fn version(&self) -> &'static str {
        "zamani.test.calibrated.v1"
    }
}

/// A model that records how many estimates the router requested.
///
/// This verifies that the model is genuinely injected and used rather than
/// being ignored by the noise-aware implementation.
#[derive(Debug)]
struct CountingNoiseModel {
    calls: Arc<AtomicUsize>,
}

impl CountingNoiseModel {
    fn new() -> (
        Self,
        Arc<AtomicUsize>,
    ) {
        let calls = Arc::new(
            AtomicUsize::new(0)
        );

        (
            Self {
                calls: Arc::clone(&calls),
            },
            calls,
        )
    }
}

impl NoiseModel for CountingNoiseModel {
    fn estimate(
        &self,
        operation: &RoutingOperation,
    ) -> Result<NoiseEstimate, RoutingError> {
        self.calls.fetch_add(
            1,
            AtomicOrdering::Relaxed,
        );

        ConservativeNoiseModel::new()
            .estimate(operation)
    }

    fn name(&self) -> &'static str {
        "counting-test"
    }

    fn version(&self) -> &'static str {
        "zamani.test.counting.v1"
    }
}

/// A model intentionally returning an invalid probability.
///
/// The production router must reject it rather than allow NaN/infinity/out of
/// range values to enter route ordering.
#[derive(Debug, Clone, Copy)]
struct InvalidProbabilityModel {
    probability: f64,
}

impl NoiseModel for InvalidProbabilityModel {
    fn estimate(
        &self,
        _operation: &RoutingOperation,
    ) -> Result<NoiseEstimate, RoutingError> {
        NoiseEstimate::new(
            self.probability,
            Duration::ZERO,
            true,
        )
    }

    fn name(&self) -> &'static str {
        "invalid-probability-test"
    }

    fn version(&self) -> &'static str {
        "zamani.test.invalid-probability.v1"
    }
}

/// A model that returns an extremely long duration.
///
/// This is used to ensure duration accumulation is explicit and checked rather
/// than silently overflowing.
#[derive(Debug, Clone, Copy)]
struct HugeDurationModel;

impl NoiseModel for HugeDurationModel {
    fn estimate(
        &self,
        _operation: &RoutingOperation,
    ) -> Result<NoiseEstimate, RoutingError> {
        NoiseEstimate::new(
            0.0,
            Duration::from_nanos(
                u64::MAX,
            ),
            true,
        )
    }

    fn name(&self) -> &'static str {
        "huge-duration-test"
    }

    fn version(&self) -> &'static str {
        "zamani.test.huge-duration.v1"
    }
}

/// A model with no calibration information.
///
/// It deliberately returns `NoiseEstimate::unknown()` rather than claiming
/// perfect hardware.
#[derive(Debug, Clone, Copy)]
struct ExplicitUnknownModel;

impl NoiseModel for ExplicitUnknownModel {
    fn estimate(
        &self,
        _operation: &RoutingOperation,
    ) -> Result<NoiseEstimate, RoutingError> {
        Ok(NoiseEstimate::unknown())
    }

    fn name(&self) -> &'static str {
        "explicit-unknown-test"
    }

    fn version(&self) -> &'static str {
        "zamani.test.unknown.v1"
    }
}

// =============================================================================
// Construction and identity
// =============================================================================

#[test]
fn noise_aware_router_can_be_constructed() {
    let router = NoiseAwareRouter::new();

    assert_eq!(
        router.name(),
        "noise_aware"
    );
}

#[test]
fn noise_aware_router_is_default_constructible() {
    let router = NoiseAwareRouter::default();

    assert_eq!(
        router.name(),
        "noise_aware"
    );
}

#[test]
fn noise_aware_router_has_stable_default_parameters() {
    let router = NoiseAwareRouter::new();

    assert_eq!(
        router.candidate_routes(),
        DEFAULT_CANDIDATE_ROUTES
    );

    assert_eq!(
        router.error_weight(),
        DEFAULT_ERROR_WEIGHT
    );

    assert_eq!(
        router.duration_weight(),
        DEFAULT_DURATION_WEIGHT
    );

    assert_eq!(
        router.swap_weight(),
        DEFAULT_SWAP_WEIGHT
    );
}

#[test]
fn noise_aware_router_exposes_stable_noise_model_identity() {
    let router = NoiseAwareRouter::new();

    assert_eq!(
        router.noise_model_name(),
        "conservative"
    );

    assert_eq!(
        router.noise_model_version(),
        "zamani.conservative-noise.v1"
    );
}

// =============================================================================
// Noise-model injection
// =============================================================================

#[test]
fn calibrated_noise_model_can_be_injected() {
    let model = CalibratedTestModel::new(
        0.001,
        Duration::from_micros(20),
        0.02,
        Duration::from_micros(300),
    );

    let router =
        NoiseAwareRouter::with_noise_model(model);

    assert_eq!(
        router.noise_model_name(),
        "calibrated-test"
    );

    assert_eq!(
        router.noise_model_version(),
        "zamani.test.calibrated.v1"
    );
}

#[test]
fn shared_noise_model_can_be_injected() {
    let model = Arc::new(
        CalibratedTestModel::new(
            0.001,
            Duration::from_micros(10),
            0.01,
            Duration::from_micros(100),
        )
    );

    let router =
        NoiseAwareRouter::new()
            .with_shared_noise_model(
                Arc::clone(&model)
            );

    assert_eq!(
        router.noise_model_name(),
        "calibrated-test"
    );
}

#[test]
fn injected_noise_model_is_actually_used() {
    let (model, calls) =
        CountingNoiseModel::new();

    let router =
        NoiseAwareRouter::with_noise_model(model);

    let operations = vec![
        RoutingOperation::Barrier {
            operands: Vec::new(),
        },
        RoutingOperation::Move(
            RoutingMove::Swap {
                a: pq(0),
                b: pq(1),
            }
        ),
    ];

    let score =
        router
            .score_route(&operations)
            .expect("injected model must score");

    assert_valid_score(&score);

    assert_eq!(
        calls.load(
            AtomicOrdering::Relaxed
        ),
        operations.len()
    );
}

// =============================================================================
// NoiseEstimate validation
// =============================================================================

#[test]
fn noise_estimate_accepts_zero_error() {
    let estimate =
        NoiseEstimate::new(
            0.0,
            Duration::ZERO,
            true,
        )
        .expect("zero error is valid");

    assert_eq!(
        estimate.error_probability,
        0.0
    );

    assert_eq!(
        estimate.duration,
        Duration::ZERO
    );

    assert!(estimate.calibrated);
}

#[test]
fn noise_estimate_accepts_exact_unit_error() {
    let estimate =
        NoiseEstimate::new(
            1.0,
            Duration::ZERO,
            true,
        )
        .expect("unit error is a valid probability");

    assert_eq!(
        estimate.error_probability,
        1.0
    );
}

#[test]
fn noise_estimate_rejects_negative_probability() {
    assert!(
        NoiseEstimate::new(
            -0.0001,
            Duration::ZERO,
            true,
        )
        .is_err()
    );
}

#[test]
fn noise_estimate_rejects_probability_above_one() {
    assert!(
        NoiseEstimate::new(
            1.000001,
            Duration::ZERO,
            true,
        )
        .is_err()
    );
}

#[test]
fn noise_estimate_rejects_nan_probability() {
    assert!(
        NoiseEstimate::new(
            f64::NAN,
            Duration::ZERO,
            true,
        )
        .is_err()
    );
}

#[test]
fn noise_estimate_rejects_positive_infinity() {
    assert!(
        NoiseEstimate::new(
            f64::INFINITY,
            Duration::ZERO,
            true,
        )
        .is_err()
    );
}

#[test]
fn noise_estimate_rejects_negative_infinity() {
    assert!(
        NoiseEstimate::new(
            f64::NEG_INFINITY,
            Duration::ZERO,
            true,
        )
        .is_err()
    );
}

#[test]
fn unknown_noise_estimate_is_explicitly_uncalibrated() {
    let estimate =
        NoiseEstimate::unknown();

    assert!(!estimate.calibrated);

    assert!(
        estimate.error_probability > 0.0,
        "unknown calibration must not silently mean perfect hardware"
    );

    assert!(
        estimate.error_probability.is_finite()
    );

    assert!(
        estimate.error_probability <= 1.0
    );
}

// =============================================================================
// Conservative model
// =============================================================================

#[test]
fn conservative_model_marks_swap_as_uncalibrated() {
    let operation =
        RoutingOperation::Move(
            RoutingMove::Swap {
                a: pq(0),
                b: pq(1),
            },
        );

    let estimate =
        ConservativeNoiseModel::new()
            .estimate(&operation)
            .expect("fallback model must return a valid estimate");

    assert!(!estimate.calibrated);

    assert!(
        estimate.error_probability > 0.0
    );
}

#[test]
fn conservative_model_does_not_claim_swap_is_perfect() {
    let operation =
        RoutingOperation::Move(
            RoutingMove::Swap {
                a: pq(0),
                b: pq(1),
            },
        );

    let estimate =
        ConservativeNoiseModel::new()
            .estimate(&operation)
            .expect("fallback model must be valid");

    assert!(
        estimate.error_probability > 0.0,
        "a movement operation with unknown calibration must carry a conservative penalty"
    );
}

// =============================================================================
// Basic route scoring
// =============================================================================

#[test]
fn empty_route_has_zero_error_and_unit_fidelity() {
    let router =
        NoiseAwareRouter::new();

    let score =
        router
            .score_route(&[])
            .expect("empty route must be scoreable");

    assert_valid_score(&score);

    assert_eq!(
        score.operations_evaluated,
        0
    );

    assert_eq!(
        score.error_probability,
        0.0
    );

    assert_eq!(
        score.fidelity,
        1.0
    );

    assert_eq!(
        score.duration,
        Duration::ZERO
    );

    assert_eq!(
        score.unknown_operations,
        0
    );
}

#[test]
fn zero_error_route_has_unit_fidelity() {
    let router =
        NoiseAwareRouter::with_noise_model(
            CalibratedTestModel::new(
                0.0,
                Duration::ZERO,
                0.0,
                Duration::ZERO,
            )
        );

    let operations = vec![
        RoutingOperation::Barrier {
            operands: Vec::new(),
        }
    ];

    let score =
        router
            .score_route(&operations)
            .expect("zero-error route must score");

    assert_eq!(
        score.error_probability,
        0.0
    );

    assert_eq!(
        score.fidelity,
        1.0
    );

    assert_eq!(
        score.duration,
        Duration::ZERO
    );
}

#[test]
fn certain_failure_produces_zero_fidelity() {
    let router =
        NoiseAwareRouter::with_noise_model(
            CalibratedTestModel::new(
                1.0,
                Duration::ZERO,
                1.0,
                Duration::ZERO,
            )
        );

    let operations = vec![
        RoutingOperation::Barrier {
            operands: Vec::new(),
        }
    ];

    let score =
        router
            .score_route(&operations)
            .expect("certain failure must be represented safely");

    assert_eq!(
        score.error_probability,
        1.0
    );

    assert_eq!(
        score.fidelity,
        0.0
    );

    assert_valid_score(&score);
}

#[test]
fn independent_operation_errors_accumulate_as_failure_probability() {
    let router =
        NoiseAwareRouter::with_noise_model(
            CalibratedTestModel::new(
                0.1,
                Duration::ZERO,
                0.1,
                Duration::ZERO,
            )
        );

    let operations = vec![
        RoutingOperation::Barrier {
            operands: Vec::new(),
        },
        RoutingOperation::Barrier {
            operands: Vec::new(),
        },
    ];

    let score =
        router
            .score_route(&operations)
            .expect("independent errors must be scoreable");

    let expected_fidelity =
        0.9_f64 * 0.9_f64;

    let expected_error =
        1.0_f64 - expected_fidelity;

    assert!(
        (score.fidelity - expected_fidelity).abs()
            < 1.0e-12,
        "fidelity mismatch: got {}, expected {}",
        score.fidelity,
        expected_fidelity
    );

    assert!(
        (score.error_probability - expected_error).abs()
            < 1.0e-12,
        "error mismatch: got {}, expected {}",
        score.error_probability,
        expected_error
    );
}

// =============================================================================
// Duration accounting
// =============================================================================

#[test]
fn route_duration_is_accumulated() {
    let gate_duration =
        Duration::from_micros(10);

    let swap_duration =
        Duration::from_micros(100);

    let router =
        NoiseAwareRouter::with_noise_model(
            CalibratedTestModel::new(
                0.001,
                gate_duration,
                0.01,
                swap_duration,
            )
        );

    let operations = vec![
        RoutingOperation::Gate {
            gate: GateIdentity::H,
            operands: vec![lq(0)],
        },
        RoutingOperation::Move(
            RoutingMove::Swap {
                a: pq(0),
                b: pq(1),
            }
        ),
        RoutingOperation::Gate {
            gate: GateIdentity::Cx,
            operands: vec![lq(0), lq(1)],
        },
    ];

    let score =
        router
            .score_route(&operations)
            .expect("duration must be scoreable");

    assert_eq!(
        score.duration,
        Duration::from_micros(120)
    );

    assert_eq!(
        score.operations_evaluated,
        3
    );
}

#[test]
fn duration_weight_contributes_to_objective() {
    let router =
        NoiseAwareRouter::with_noise_model(
            CalibratedTestModel::new(
                0.0,
                Duration::from_secs(1),
                0.0,
                Duration::ZERO,
            )
        )
        .with_error_weight(0.0)
        .expect("zero error weight is valid")
        .with_duration_weight(2.0)
        .expect("positive duration weight is valid")
        .with_swap_weight(0.0)
        .expect("zero swap weight is valid");

    let operations = vec![
        RoutingOperation::Gate {
            gate: GateIdentity::H,
            operands: vec![lq(0)],
        }
    ];

    let score =
        router
            .score_route(&operations)
            .expect("duration-weighted score must succeed");

    assert!(
        (score.weighted_score - 2.0).abs()
            < 1.0e-12,
        "one second at weight 2 must produce objective 2"
    );
}

// =============================================================================
// Calibration accounting
// =============================================================================

#[test]
fn calibrated_operations_are_counted() {
    let router =
        NoiseAwareRouter::with_noise_model(
            CalibratedTestModel::new(
                0.001,
                Duration::from_micros(10),
                0.01,
                Duration::from_micros(100),
            )
        );

    let operations = vec![
        RoutingOperation::Gate {
            gate: GateIdentity::H,
            operands: vec![lq(0)],
        },
        RoutingOperation::Move(
            RoutingMove::Swap {
                a: pq(0),
                b: pq(1),
            }
        ),
    ];

    let score =
        router
            .score_route(&operations)
            .expect("calibrated route must score");

    assert_eq!(
        score.calibrated_operations,
        2
    );

    assert_eq!(
        score.unknown_operations,
        0
    );
}

#[test]
fn unknown_calibration_is_counted() {
    let router =
        NoiseAwareRouter::with_noise_model(
            ExplicitUnknownModel
        );

    let operations = vec![
        RoutingOperation::Gate {
            gate: GateIdentity::H,
            operands: vec![lq(0)],
        },
        RoutingOperation::Move(
            RoutingMove::Swap {
                a: pq(0),
                b: pq(1),
            }
        ),
    ];

    let score =
        router
            .score_route(&operations)
            .expect("explicit unknown model must score");

    assert_eq!(
        score.calibrated_operations,
        0
    );

    assert_eq!(
        score.unknown_operations,
        2
    );

    assert!(
        score.error_probability > 0.0,
        "unknown calibration must not produce a falsely perfect route"
    );
}

// =============================================================================
// Weight validation
// =============================================================================

#[test]
fn error_weight_accepts_zero() {
    NoiseAwareRouter::new()
        .with_error_weight(0.0)
        .expect("zero error weight must be valid");
}

#[test]
fn duration_weight_accepts_zero() {
    NoiseAwareRouter::new()
        .with_duration_weight(0.0)
        .expect("zero duration weight must be valid");
}

#[test]
fn swap_weight_accepts_zero() {
    NoiseAwareRouter::new()
        .with_swap_weight(0.0)
        .expect("zero SWAP weight must be valid");
}

#[test]
fn error_weight_rejects_negative_values() {
    assert!(
        NoiseAwareRouter::new()
            .with_error_weight(-1.0)
            .is_err()
    );
}

#[test]
fn duration_weight_rejects_negative_values() {
    assert!(
        NoiseAwareRouter::new()
            .with_duration_weight(-1.0)
            .is_err()
    );
}

#[test]
fn swap_weight_rejects_negative_values() {
    assert!(
        NoiseAwareRouter::new()
            .with_swap_weight(-1.0)
            .is_err()
    );
}

#[test]
fn error_weight_rejects_nan() {
    assert!(
        NoiseAwareRouter::new()
            .with_error_weight(f64::NAN)
            .is_err()
    );
}

#[test]
fn duration_weight_rejects_infinity() {
    assert!(
        NoiseAwareRouter::new()
            .with_duration_weight(f64::INFINITY)
            .is_err()
    );
}

#[test]
fn swap_weight_rejects_weight_above_safety_ceiling() {
    assert!(
        NoiseAwareRouter::new()
            .with_swap_weight(MAX_WEIGHT + 1.0)
            .is_err()
    );
}

// =============================================================================
// Candidate-route configuration
// =============================================================================

#[test]
fn candidate_route_count_can_be_configured() {
    let router =
        NoiseAwareRouter::new()
            .with_candidate_routes(8)
            .expect("eight candidates must be valid");

    assert_eq!(
        router.candidate_routes(),
        8
    );
}

#[test]
fn zero_candidate_routes_are_rejected() {
    assert!(
        NoiseAwareRouter::new()
            .with_candidate_routes(0)
            .is_err()
    );
}

#[test]
fn excessive_candidate_routes_are_rejected() {
    assert!(
        NoiseAwareRouter::new()
            .with_candidate_routes(
                MAX_CANDIDATE_ROUTES + 1
            )
            .is_err()
    );
}

#[test]
fn maximum_candidate_route_count_is_accepted() {
    let router =
        NoiseAwareRouter::new()
            .with_candidate_routes(
                MAX_CANDIDATE_ROUTES
            )
            .expect("configured safety maximum must be accepted");

    assert_eq!(
        router.candidate_routes(),
        MAX_CANDIDATE_ROUTES
    );
}

// =============================================================================
// Numerical safety
// =============================================================================

#[test]
fn invalid_probability_from_model_is_rejected() {
    let router =
        NoiseAwareRouter::with_noise_model(
            InvalidProbabilityModel {
                probability: 1.5,
            }
        );

    let operations = vec![
        RoutingOperation::Barrier {
            operands: Vec::new(),
        }
    ];

    assert!(
        router
            .score_route(&operations)
            .is_err(),
        "invalid calibration must never enter route scoring"
    );
}

#[test]
fn nan_probability_from_model_is_rejected() {
    let router =
        NoiseAwareRouter::with_noise_model(
            InvalidProbabilityModel {
                probability: f64::NAN,
            }
        );

    let operations = vec![
        RoutingOperation::Barrier {
            operands: Vec::new(),
        }
    ];

    assert!(
        router
            .score_route(&operations)
            .is_err()
    );
}

#[test]
fn infinite_probability_from_model_is_rejected() {
    let router =
        NoiseAwareRouter::with_noise_model(
            InvalidProbabilityModel {
                probability: f64::INFINITY,
            }
        );

    let operations = vec![
        RoutingOperation::Barrier {
            operands: Vec::new(),
        }
    ];

    assert!(
        router
            .score_route(&operations)
            .is_err()
    );
}

#[test]
fn duration_accumulation_overflow_is_rejected() {
    let router =
        NoiseAwareRouter::with_noise_model(
            HugeDurationModel
        );

    let operations = vec![
        RoutingOperation::Barrier {
            operands: Vec::new(),
        },
        RoutingOperation::Barrier {
            operands: Vec::new(),
        },
    ];

    assert!(
        router
            .score_route(&operations)
            .is_err(),
        "duration overflow must be rejected rather than wrapped"
    );
}

// =============================================================================
// Weighted scoring
// =============================================================================

#[test]
fn weighted_score_combines_error_duration_and_swap_count() {
    let router =
        NoiseAwareRouter::with_noise_model(
            CalibratedTestModel::new(
                0.1,
                Duration::from_secs(2),
                0.2,
                Duration::from_secs(3),
            )
        )
        .with_error_weight(2.0)
        .expect("error weight must be valid")
        .with_duration_weight(4.0)
        .expect("duration weight must be valid")
        .with_swap_weight(5.0)
        .expect("swap weight must be valid");

    let operations = vec![
        RoutingOperation::Gate {
            gate: GateIdentity::H,
            operands: vec![lq(0)],
        },
        RoutingOperation::Move(
            RoutingMove::Swap {
                a: pq(0),
                b: pq(1),
            }
        ),
    ];

    let score =
        router
            .score_route(&operations)
            .expect("weighted score must succeed");

    let expected_fidelity =
        (1.0 - 0.1) * (1.0 - 0.2);

    let expected_error =
        1.0 - expected_fidelity;

    let expected_duration_seconds =
        5.0;

    let expected =
        2.0 * expected_error
            + 4.0 * expected_duration_seconds
            + 5.0;

    assert!(
        (score.weighted_score - expected).abs()
            < 1.0e-12,
        "got {}, expected {}",
        score.weighted_score,
        expected
    );
}

#[test]
fn swap_weight_changes_objective_without_changing_fidelity() {
    let operations = vec![
        RoutingOperation::Move(
            RoutingMove::Swap {
                a: pq(0),
                b: pq(1),
            }
        )
    ];

    let first =
        NoiseAwareRouter::with_noise_model(
            CalibratedTestModel::new(
                0.01,
                Duration::ZERO,
                0.01,
                Duration::ZERO,
            )
        )
        .with_swap_weight(0.0)
        .expect("zero SWAP weight must be valid")
        .score_route(&operations)
        .expect("first score must succeed");

    let second =
        NoiseAwareRouter::with_noise_model(
            CalibratedTestModel::new(
                0.01,
                Duration::ZERO,
                0.01,
                Duration::ZERO,
            )
        )
        .with_swap_weight(10.0)
        .expect("SWAP weight must be valid")
        .score_route(&operations)
        .expect("second score must succeed");

    assert_eq!(
        first.error_probability,
        second.error_probability
    );

    assert_eq!(
        first.fidelity,
        second.fidelity
    );

    assert!(
        second.weighted_score
            > first.weighted_score
    );
}

// =============================================================================
// Direct route integration
// =============================================================================

#[test]
fn adjacent_noise_aware_route_succeeds_without_required_movement() {
    let topology =
        line_topology(3);

    let mapping =
        identity_mapping(3);

    let operations = vec![
        two_qubit_gate(
            GateIdentity::Cx,
            0,
            1,
        )
    ];

    let result =
        NoiseAwareRouter::new()
            .route(
                &RoutingInput::new(
                    &operations,
                    &topology,
                    &mapping,
                    &test_config(),
                )
                .expect("routing input must be valid")
            )
            .expect("adjacent interaction must route");

    assert_eq!(
        result.metrics().routed_two_qubit_operations,
        1
    );

    assert_eq!(
        result.metrics().original_operations,
        1
    );

    assert!(
        result
            .metrics()
            .estimated_error
            .is_some()
    );

    assert!(
        result
            .metrics()
            .estimated_fidelity
            .is_some()
    );

    assert!(
        result
            .metrics()
            .objective_value
            .is_some()
    );
}

#[test]
fn non_adjacent_noise_aware_route_succeeds() {
    let topology =
        line_topology(3);

    let mapping =
        identity_mapping(3);

    let operations = vec![
        two_qubit_gate(
            GateIdentity::Cx,
            0,
            2,
        )
    ];

    let input =
        RoutingInput::new(
            &operations,
            &topology,
            &mapping,
            &test_config(),
        )
        .expect("routing input must be valid");

    let result =
        NoiseAwareRouter::new()
            .route(&input)
            .expect("non-adjacent interaction must route");

    assert!(
        result.metrics().inserted_swaps >= 1,
        "non-adjacent interaction must require movement on a three-node line"
    );

    assert_eq!(
        swap_count(result.operations()),
        result.metrics().inserted_swaps
    );

    assert!(
        result.metrics().estimated_error.is_some()
    );

    assert!(
        result.metrics().estimated_fidelity.is_some()
    );

    assert!(
        result.metrics().estimated_execution_duration.is_some()
    );
}

#[test]
fn single_qubit_operation_routes_without_movement() {
    let topology =
        line_topology(2);

    let mapping =
        identity_mapping(2);

    let operations = vec![
        single_qubit_gate(
            GateIdentity::H,
            0,
        )
    ];

    let result =
        NoiseAwareRouter::new()
            .route(
                &RoutingInput::new(
                    &operations,
                    &topology,
                    &mapping,
                    &test_config(),
                )
                .expect("routing input must be valid")
            )
            .expect("single-qubit operation must route");

    assert_eq!(
        result.metrics().inserted_swaps,
        0
    );

    assert_eq!(
        result.metrics().original_operations,
        1
    );
}

// =============================================================================
// Result metrics and quality
// =============================================================================

#[test]
fn routed_result_contains_noise_quality_metrics() {
    let topology =
        line_topology(3);

    let mapping =
        identity_mapping(3);

    let operations = vec![
        two_qubit_gate(
            GateIdentity::Cx,
            0,
            2,
        )
    ];

    let result =
        NoiseAwareRouter::with_noise_model(
            CalibratedTestModel::new(
                0.001,
                Duration::from_micros(10),
                0.01,
                Duration::from_micros(100),
            )
        )
        .route(
            &RoutingInput::new(
                &operations,
                &topology,
                &mapping,
                &test_config(),
            )
            .expect("routing input must be valid")
        )
        .expect("route must succeed");

    let estimated_error =
        result
            .metrics()
            .estimated_error
            .expect("noise-aware result must contain estimated error");

    let estimated_fidelity =
        result
            .metrics()
            .estimated_fidelity
            .expect("noise-aware result must contain estimated fidelity");

    let objective =
        result
            .metrics()
            .objective_value
            .expect("noise-aware result must contain objective value");

    assert!(
        estimated_error.is_finite()
    );

    assert!(
        estimated_fidelity.is_finite()
    );

    assert!(
        objective.is_finite()
    );

    assert!(
        (0.0..=1.0).contains(
            &estimated_error
        )
    );

    assert!(
        (0.0..=1.0).contains(
            &estimated_fidelity
        )
    );
}

#[test]
fn result_quality_matches_metric_quality() {
    let topology =
        line_topology(3);

    let mapping =
        identity_mapping(3);

    let operations = vec![
        two_qubit_gate(
            GateIdentity::Cx,
            0,
            2,
        )
    ];

    let result =
        NoiseAwareRouter::with_noise_model(
            CalibratedTestModel::new(
                0.002,
                Duration::from_micros(10),
                0.01,
                Duration::from_micros(100),
            )
        )
        .route(
            &RoutingInput::new(
                &operations,
                &topology,
                &mapping,
                &test_config(),
            )
            .expect("routing input must be valid")
        )
        .expect("route must succeed");

    assert_eq!(
        result.quality().estimated_error,
        result.metrics().estimated_error
    );

    assert_eq!(
        result.quality().estimated_fidelity,
        result.metrics().estimated_fidelity
    );

    assert_eq!(
        result.quality().objective_value,
        result.metrics().objective_value
    );

    assert!(
        result.quality().comparable,
        "noise-aware results must be marked comparable"
    );
}

// =============================================================================
// Mapping correctness
// =============================================================================

#[test]
fn caller_mapping_is_not_mutated_by_routing() {
    let topology =
        line_topology(3);

    let mapping =
        identity_mapping(3);

    let original =
        mapping.clone();

    let operations = vec![
        two_qubit_gate(
            GateIdentity::Cx,
            0,
            2,
        )
    ];

    let _result =
        NoiseAwareRouter::new()
            .route(
                &RoutingInput::new(
                    &operations,
                    &topology,
                    &mapping,
                    &test_config(),
                )
                .expect("routing input must be valid")
            )
            .expect("routing must succeed");

    assert_eq!(
        mapping,
        original,
        "caller-owned mapping must remain immutable"
    );
}

#[test]
fn initial_mapping_in_result_matches_caller_mapping() {
    let topology =
        line_topology(3);

    let mapping =
        identity_mapping(3);

    let operations = vec![
        two_qubit_gate(
            GateIdentity::Cx,
            0,
            2,
        )
    ];

    let result =
        NoiseAwareRouter::new()
            .route(
                &RoutingInput::new(
                    &operations,
                    &topology,
                    &mapping,
                    &test_config(),
                )
                .expect("routing input must be valid")
            )
            .expect("routing must succeed");

    assert_eq!(
        result.initial_mapping(),
        &mapping
    );
}

#[test]
fn final_mapping_remains_valid() {
    let topology =
        line_topology(4);

    let mapping =
        identity_mapping(4);

    let operations = vec![
        two_qubit_gate(
            GateIdentity::Cx,
            0,
            3,
        ),
        two_qubit_gate(
            GateIdentity::Cx,
            1,
            2,
        ),
    ];

    let result =
        NoiseAwareRouter::new()
            .route(
                &RoutingInput::new(
                    &operations,
                    &topology,
                    &mapping,
                    &test_config(),
                )
                .expect("routing input must be valid")
            )
            .expect("routing must succeed");

    result
        .final_mapping()
        .validate(&topology)
        .expect(
            "noise-aware routing must preserve mapping validity"
        );
}

// =============================================================================
// Determinism and reproducibility
// =============================================================================

#[test]
fn repeated_noise_aware_execution_is_deterministic() {
    let topology =
        line_topology(5);

    let mapping =
        identity_mapping(5);

    let operations = vec![
        two_qubit_gate(
            GateIdentity::Cx,
            0,
            4,
        ),
        two_qubit_gate(
            GateIdentity::Cx,
            1,
            3,
        ),
        two_qubit_gate(
            GateIdentity::Cz,
            0,
            2,
        ),
    ];

    let config =
        test_config();

    let input =
        RoutingInput::new(
            &operations,
            &topology,
            &mapping,
            &config,
        )
        .expect("routing input must be valid");

    let router =
        NoiseAwareRouter::new()
            .with_candidate_routes(4)
            .expect("four candidates must be valid");

    let first =
        router
            .route(&input)
            .expect("first route must succeed");

    let second =
        router
            .route(&input)
            .expect("second route must succeed");

    assert_eq!(
        route_signature(&first),
        route_signature(&second),
        "same input/configuration must produce the same route"
    );

    assert_eq!(
        first.final_mapping(),
        second.final_mapping()
    );

    assert_eq!(
        first.metrics().inserted_swaps,
        second.metrics().inserted_swaps
    );

    assert_eq!(
        first.metrics().estimated_error,
        second.metrics().estimated_error
    );

    assert_eq!(
        first.metrics().estimated_fidelity,
        second.metrics().estimated_fidelity
    );

    assert_eq!(
        first.metrics().objective_value,
        second.metrics().objective_value
    );
}

#[test]
fn explicit_seed_preserves_reproducibility() {
    let topology =
        line_topology(5);

    let mapping =
        identity_mapping(5);

    let operations = vec![
        two_qubit_gate(
            GateIdentity::Cx,
            0,
            4,
        )
    ];

    let mut config =
        test_config();

    config.seed =
        Some(123_456_789);

    let input =
        RoutingInput::new(
            &operations,
            &topology,
            &mapping,
            &config,
        )
        .expect("routing input must be valid");

    let router =
        NoiseAwareRouter::new();

    let first =
        router
            .route(&input)
            .expect("first seeded route must succeed");

    let second =
        router
            .route(&input)
            .expect("second seeded route must succeed");

    assert_eq!(
        route_signature(&first),
        route_signature(&second)
    );

    assert_eq!(
        first.final_mapping(),
        second.final_mapping()
    );
}

// =============================================================================
// Algorithm trait integration
// =============================================================================

#[test]
fn noise_aware_router_implements_routing_algorithm_contract() {
    let router =
        NoiseAwareRouter::new();

    let config =
        test_config();

    assert!(
        router.supports(&config),
        "NoiseAwareRouter must support RoutingAlgorithm::NoiseAware"
    );

    assert_eq!(
        RoutingAlgorithmTrait::name(&router),
        "noise_aware"
    );

    assert_eq!(
        RoutingAlgorithmTrait::version(&router),
        "zamani.noise_aware.v1"
    );
}

#[test]
fn noise_aware_router_supports_auto_selection() {
    let router =
        NoiseAwareRouter::new();

    let config =
        auto_config();

    assert!(
        router.supports(&config),
        "NoiseAwareRouter must support Auto selection"
    );
}

#[test]
fn noise_aware_router_rejects_explicitly_incompatible_algorithm() {
    let router =
        NoiseAwareRouter::new();

    let mut config =
        RoutingConfig::default();

    config.algorithm =
        RoutingAlgorithm::Basic;

    let topology =
        line_topology(2);

    let mapping =
        identity_mapping(2);

    let operations = vec![
        single_qubit_gate(
            GateIdentity::H,
            0,
        )
    ];

    let input =
        RoutingInput::new(
            &operations,
            &topology,
            &mapping,
            &config,
        )
        .expect("input construction must succeed");

    assert!(
        router.route(&input).is_err(),
        "noise-aware router must reject an incompatible explicit algorithm"
    );
}

// =============================================================================
// Configuration validation through route()
// =============================================================================

#[test]
fn zero_sabre_trials_are_rejected() {
    let router =
        NoiseAwareRouter::new();

    let mut config =
        test_config();

    config.limits.sabre_trials =
        0;

    let topology =
        line_topology(2);

    let mapping =
        identity_mapping(2);

    let operations = vec![
        single_qubit_gate(
            GateIdentity::H,
            0,
        )
    ];

    let input =
        RoutingInput::new(
            &operations,
            &topology,
            &mapping,
            &config,
        )
        .expect("input construction must succeed");

    assert!(
        router.route(&input).is_err(),
        "zero SABRE trials must be rejected"
    );
}

#[test]
fn zero_routing_iterations_are_rejected() {
    let router =
        NoiseAwareRouter::new();

    let mut config =
        test_config();

    config.limits.max_iterations =
        0;

    let topology =
        line_topology(2);

    let mapping =
        identity_mapping(2);

    let operations = vec![
        single_qubit_gate(
            GateIdentity::H,
            0,
        )
    ];

    let input =
        RoutingInput::new(
            &operations,
            &topology,
            &mapping,
            &config,
        )
        .expect("input construction must succeed");

    assert!(
        router.route(&input).is_err(),
        "zero routing iterations must be rejected"
    );
}

// =============================================================================
// Multiple operations and accounting
// =============================================================================

#[test]
fn multiple_operations_are_scored_independently() {
    let router =
        NoiseAwareRouter::with_noise_model(
            CalibratedTestModel::new(
                0.01,
                Duration::from_micros(10),
                0.02,
                Duration::from_micros(20),
            )
        );

    let operations = vec![
        RoutingOperation::Gate {
            gate: GateIdentity::H,
            operands: vec![lq(0)],
        },
        RoutingOperation::Gate {
            gate: GateIdentity::H,
            operands: vec![lq(1)],
        },
        RoutingOperation::Move(
            RoutingMove::Swap {
                a: pq(0),
                b: pq(1),
            }
        ),
    ];

    let score =
        router
            .score_route(&operations)
            .expect("multi-operation score must succeed");

    assert_eq!(
        score.operations_evaluated,
        3
    );

    assert_eq!(
        score.calibrated_operations,
        3
    );

    assert_eq!(
        score.unknown_operations,
        0
    );

    assert_eq!(
        score.duration,
        Duration::from_micros(40)
    );

    let expected_fidelity =
        0.99_f64
            * 0.99_f64
            * 0.98_f64;

    assert!(
        (score.fidelity - expected_fidelity).abs()
            < 1.0e-12
    );
}

#[test]
fn swap_count_is_accounted_separately_from_operation_count() {
    let router =
        NoiseAwareRouter::new();

    let operations = vec![
        RoutingOperation::Gate {
            gate: GateIdentity::H,
            operands: vec![lq(0)],
        },
        RoutingOperation::Move(
            RoutingMove::Swap {
                a: pq(0),
                b: pq(1),
            }
        ),
        RoutingOperation::Move(
            RoutingMove::Swap {
                a: pq(1),
                b: pq(2),
            }
        ),
    ];

    let score =
        router
            .score_route(&operations)
            .expect("route must score");

    assert_eq!(
        score.operations_evaluated,
        3
    );

    assert_eq!(
        swap_count(&operations),
        2
    );
}

// =============================================================================
// Route quality boundary tests
// =============================================================================

#[test]
fn calibrated_model_can_assign_different_quality_to_gate_and_swap() {
    let router =
        NoiseAwareRouter::with_noise_model(
            CalibratedTestModel::new(
                0.001,
                Duration::from_micros(10),
                0.25,
                Duration::from_micros(500),
            )
        );

    let gate =
        RoutingOperation::Gate {
            gate: GateIdentity::H,
            operands: vec![lq(0)],
        };

    let swap =
        RoutingOperation::Move(
            RoutingMove::Swap {
                a: pq(0),
                b: pq(1),
            }
        );

    let gate_score =
        router
            .score_route(&[gate])
            .expect("gate score must succeed");

    let swap_score =
        router
            .score_route(&[swap])
            .expect("swap score must succeed");

    assert!(
        swap_score.error_probability
            > gate_score.error_probability
    );

    assert!(
        swap_score.duration
            > gate_score.duration
    );

    assert!(
        swap_score.fidelity
            < gate_score.fidelity
    );
}

// =============================================================================
// Bounded larger-route test
// =============================================================================

#[test]
fn bounded_large_route_remains_numerically_valid() {
    let router =
        NoiseAwareRouter::with_noise_model(
            CalibratedTestModel::new(
                1.0e-6,
                Duration::from_nanos(10),
                1.0e-5,
                Duration::from_nanos(20),
            )
        );

    let mut operations =
        Vec::with_capacity(1_000);

    for index in 0..1_000 {
        operations.push(
            RoutingOperation::Gate {
                gate: GateIdentity::H,
                operands: vec![
                    lq(index % 16)
                ],
            }
        );

        if index % 2 == 0 {
            operations.push(
                RoutingOperation::Move(
                    RoutingMove::Swap {
                        a: pq(index % 16),
                        b: pq((index + 1) % 16),
                    }
                )
            );
        }
    }

    let score =
        router
            .score_route(&operations)
            .expect("bounded large route must remain scoreable");

    assert_valid_score(&score);

    assert_eq!(
        score.operations_evaluated,
        operations.len()
    );

    assert!(
        score.duration > Duration::ZERO
    );

    assert!(
        score.error_probability >= 0.0
    );

    assert!(
        score.fidelity >= 0.0
    );
}

// =============================================================================
// Result reproducibility metadata
// =============================================================================

#[test]
fn routed_result_records_noise_aware_algorithm_version() {
    let topology =
        line_topology(3);

    let mapping =
        identity_mapping(3);

    let operations = vec![
        two_qubit_gate(
            GateIdentity::Cx,
            0,
            2,
        )
    ];

    let result =
        NoiseAwareRouter::new()
            .route(
                &RoutingInput::new(
                    &operations,
                    &topology,
                    &mapping,
                    &test_config(),
                )
                .expect("routing input must be valid")
            )
            .expect("routing must succeed");

    assert_eq!(
        result
            .reproducibility()
            .algorithm_version
            .as_deref(),
        Some("zamani.noise_aware.v1")
    );

    assert_eq!(
        result
            .reproducibility()
            .routing_version
            .as_deref(),
        Some("zamani-routing-noise-aware-v1")
    );
}

#[test]
fn routed_result_records_candidate_trial_count() {
    let topology =
        line_topology(3);

    let mapping =
        identity_mapping(3);

    let operations = vec![
        two_qubit_gate(
            GateIdentity::Cx,
            0,
            2,
        )
    ];

    let router =
        NoiseAwareRouter::new()
            .with_candidate_routes(3)
            .expect("three candidates must be valid");

    let result =
        router
            .route(
                &RoutingInput::new(
                    &operations,
                    &topology,
                    &mapping,
                    &test_config(),
                )
                .expect("routing input must be valid")
            )
            .expect("routing must succeed");

    assert_eq!(
        result
            .reproducibility()
            .total_trials,
        Some(3)
    );
}

// =============================================================================
// Failure safety
// =============================================================================

#[test]
fn invalid_model_does_not_produce_a_partial_successful_score() {
    let router =
        NoiseAwareRouter::with_noise_model(
            InvalidProbabilityModel {
                probability: -0.5,
            }
        );

    let operations = vec![
        RoutingOperation::Gate {
            gate: GateIdentity::H,
            operands: vec![lq(0)],
        }
    ];

    let result =
        router.score_route(&operations);

    assert!(
        result.is_err()
    );
}

#[test]
fn route_failure_does_not_mutate_caller_mapping() {
    let topology =
        line_topology(2);

    let mapping =
        identity_mapping(2);

    let original =
        mapping.clone();

    let mut config =
        test_config();

    config.limits.max_iterations =
        0;

    let operations = vec![
        two_qubit_gate(
            GateIdentity::Cx,
            0,
            1,
        )
    ];

    let input =
        RoutingInput::new(
            &operations,
            &topology,
            &mapping,
            &config,
        )
        .expect("input construction must succeed");

    let result =
        NoiseAwareRouter::new()
            .route(&input);

    assert!(
        result.is_err()
    );

    assert_eq!(
        mapping,
        original,
        "failed routing must not mutate caller-owned mapping"
    );
}

// =============================================================================
// Default constants remain coherent
// =============================================================================

#[test]
fn production_defaults_are_finite_and_non_negative() {
    assert!(
        DEFAULT_ERROR_WEIGHT.is_finite()
    );

    assert!(
        DEFAULT_DURATION_WEIGHT.is_finite()
    );

    assert!(
        DEFAULT_SWAP_WEIGHT.is_finite()
    );

    assert!(
        DEFAULT_ERROR_WEIGHT >= 0.0
    );

    assert!(
        DEFAULT_DURATION_WEIGHT >= 0.0
    );

    assert!(
        DEFAULT_SWAP_WEIGHT >= 0.0
    );

    assert!(
        DEFAULT_CANDIDATE_ROUTES > 0
    );

    assert!(
        DEFAULT_CANDIDATE_ROUTES
            <= MAX_CANDIDATE_ROUTES
    );
}

// =============================================================================
// Public route-operation contract
// =============================================================================

#[test]
fn semantic_swap_operations_are_visible_in_result() {
    let topology =
        line_topology(3);

    let mapping =
        identity_mapping(3);

    let operations = vec![
        two_qubit_gate(
            GateIdentity::Cx,
            0,
            2,
        )
    ];

    let result =
        NoiseAwareRouter::new()
            .route(
                &RoutingInput::new(
                    &operations,
                    &topology,
                    &mapping,
                    &test_config(),
                )
                .expect("routing input must be valid")
            )
            .expect("routing must succeed");

    let swaps =
        result
            .operations()
            .iter()
            .filter_map(|operation| {
                match operation {
                    RoutingOperation::Move(
                        RoutingMove::Swap { a, b }
                    ) => Some((*a, *b)),

                    _ => None,
                }
            })
            .collect::<Vec<_>>();

    assert_eq!(
        swaps.len(),
        result.metrics().inserted_swaps
    );

    for (a, b) in swaps {
        assert_ne!(
            a,
            b,
            "a semantic SWAP cannot connect a qubit to itself"
        );
    }
}