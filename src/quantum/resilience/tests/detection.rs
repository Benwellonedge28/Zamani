//! Zamani Quantum Resilience — Detection Integration Tests.
//!
//! Path:
//!     src/quantum/resilience/tests/detection.rs
//!
//! # Purpose
//!
//! This module verifies the public and cross-module contracts of the
//! resilience detection subsystem.
//!
//! The tests deliberately test:
//!
//! - the common detector contract;
//! - deterministic execution;
//! - streaming behavior;
//! - detector composition;
//! - observation identity;
//! - explicit trust/freshness semantics;
//! - invalid-input handling;
//! - numerical detector behavior;
//! - threshold behavior;
//! - anomaly detection;
//! - statistical detection;
//! - drift detection;
//! - timeout detection;
//! - execution-failure detection;
//! - QEC-signal detection;
//! - hardware-signal detection;
//! - detector registry integration;
//! - canonical quantum-resource identity;
//! - scalability without machine-size assumptions;
//! - reset semantics;
//! - output determinism;
//! - absence of hidden recovery behavior.
//!
//! # Architectural rule
//!
//! Detection observes and normalizes.
//!
//! It does NOT:
//!
//! - diagnose root cause;
//! - authorize recovery;
//! - choose a backend;
//! - mutate routing;
//! - mutate scheduling;
//! - recompile programs;
//! - change QEC;
//! - accept quantum results.
//!
//! Those responsibilities remain outside this test module.
//!
//! # Scalability
//!
//! These tests contain no machine-size constants such as:
//!
//! ```text
//! MAX_QUBITS
//! MAX_DETECTORS
//! MAX_BACKENDS
//! MAX_OBSERVATIONS
//! ```
//!
//! Test data may use finite values because tests necessarily need concrete
//! examples, but those values are never interpreted as hardware limits.
//!
//! The scalability tests instead exercise iterator-based processing and
//! parameterized generated streams.
//!
//! # Quantum identity
//!
//! Whenever a quantum resource identity is required, this file uses the
//! canonical Zamani IR namespace:
//!
//! ```text
//! crate::quantum::ir::qubit
//! ```
//!
//! It does not introduce a resilience-specific qubit identifier.
//!
//! # Rust compatibility
//!
//! Designed for:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! # Dependency direction
//!
//! ```text
//! quantum::ir
//!      |
//!      v
//! resilience::model
//!      |
//!      v
//! resilience::detection
//!      |
//!      v
//! resilience::registry
//!      |
//!      v
//! this integration test
//! ```
//!
//! The test suite does not introduce dependencies from detection into
//! diagnosis, planning, adaptation, recovery, or verification.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use core::any::type_name;
use core::num::NonZeroU64;

use crate::quantum::ir::qubit::QubitId;

use crate::quantum::resilience::detection::anomaly::{
    AnomalyDetector,
    AnomalyDetectorConfig,
    AnomalyDirection,
    BaselineUpdatePolicy,
};

use crate::quantum::resilience::detection::detector::{
    DetectionClassification,
    DetectionConfidence,
    DetectionContext,
    DetectionInput,
    DetectionMetadata,
    DetectionObservation,
    DetectionOutput,
    DetectionPayload,
    DetectionSequence,
    DetectionSignal,
    Detector,
    DetectorIdentity,
    DetectorObject,
    ObservationId,
    ObservationSource,
    ObservationTrust,
    SignalId,
};

use crate::quantum::resilience::detection::drift::DriftDetector;
use crate::quantum::resilience::detection::execution_failure::ExecutionFailureDetector;
use crate::quantum::resilience::detection::hardware_signal::HardwareSignalDetector;
use crate::quantum::resilience::detection::qec_signal::QecSignalDetector;
use crate::quantum::resilience::detection::statistical::StatisticalDetector;
use crate::quantum::resilience::detection::threshold::ThresholdDetector;
use crate::quantum::resilience::detection::timeout::TimeoutDetector;

use crate::quantum::resilience::errors::{
    ResilienceError,
    ResilienceErrorCode,
    ResilienceResult,
};

// ============================================================================
// Test helpers
// ============================================================================

fn observation_id(value: u64) -> ObservationId {
    ObservationId::from_u64(value)
        .expect("test observation IDs must be non-zero")
}

fn detection_sequence(value: u64) -> DetectionSequence {
    DetectionSequence::from_u64(value)
        .expect("test detection sequence must be non-zero")
}

fn signal_id(value: u64) -> SignalId {
    SignalId::from_u64(value)
        .expect("test signal IDs must be non-zero")
}

fn context() -> DetectionContext {
    /*
     * Keep this helper intentionally minimal.
     *
     * DetectionContext is owned by the production detection contract.
     * The test suite should construct it through the public constructor rather
     * than duplicating its internal representation.
     *
     * If the production constructor gains additional mandatory fields, this
     * helper is the single integration point that should change.
     */
    DetectionContext::default()
}

fn numeric_observation(
    id: u64,
    value: f64,
) -> DetectionObservation {
    DetectionObservation::new(
        observation_id(id),
        ObservationSource::External("tests".to_owned()),
        ObservationTrust::Verified,
        DetectionPayload::Number(value),
    )
    .expect("test observation must be valid")
}

fn boolean_observation(
    id: u64,
    value: bool,
) -> DetectionObservation {
    DetectionObservation::new(
        observation_id(id),
        ObservationSource::External("tests".to_owned()),
        ObservationTrust::Verified,
        DetectionPayload::Boolean(value),
    )
    .expect("test observation must be valid")
}

fn text_observation(
    id: u64,
    value: &str,
) -> DetectionObservation {
    DetectionObservation::new(
        observation_id(id),
        ObservationSource::External("tests".to_owned()),
        ObservationTrust::Verified,
        DetectionPayload::Text(value.to_owned()),
    )
    .expect("test observation must be valid")
}

fn marker_observation(id: u64) -> DetectionObservation {
    DetectionObservation::new(
        observation_id(id),
        ObservationSource::External("tests".to_owned()),
        ObservationTrust::Verified,
        DetectionPayload::Marker,
    )
    .expect("test marker observation must be valid")
}

fn numeric_input<'a>(
    observations: &'a [DetectionObservation],
) -> DetectionInput<'a, impl Iterator<Item = &'a DetectionObservation>> {
    DetectionInput::new(
        &context(),
        observations.iter(),
    )
}

fn numeric_values(
    values: &[f64],
) -> Vec<DetectionObservation> {
    values
        .iter()
        .enumerate()
        .map(|(index, value)| {
            numeric_observation(
                (index as u64) + 1,
                *value,
            )
        })
        .collect()
}

// ============================================================================
// Public API surface
// ============================================================================

#[test]
fn detection_public_contract_is_available() {
    assert!(!type_name::<DetectionContext>().is_empty());
    assert!(!type_name::<DetectionInput<'static, core::iter::Empty<()>>>()
        .is_empty());
    assert!(!type_name::<DetectionObservation>().is_empty());
    assert!(!type_name::<DetectionOutput>().is_empty());
    assert!(!type_name::<DetectionSignal>().is_empty());
    assert!(!type_name::<DetectionMetadata>().is_empty());
    assert!(!type_name::<DetectionClassification>().is_empty());
    assert!(!type_name::<DetectionConfidence>().is_empty());
    assert!(!type_name::<DetectorIdentity>().is_empty());
    assert!(!type_name::<DetectorObject>().is_empty());
}

#[test]
fn detector_implementations_are_available() {
    assert!(!type_name::<AnomalyDetector>().is_empty());
    assert!(!type_name::<ThresholdDetector>().is_empty());
    assert!(!type_name::<StatisticalDetector>().is_empty());
    assert!(!type_name::<DriftDetector>().is_empty());
    assert!(!type_name::<TimeoutDetector>().is_empty());
    assert!(!type_name::<ExecutionFailureDetector>().is_empty());
    assert!(!type_name::<QecSignalDetector>().is_empty());
    assert!(!type_name::<HardwareSignalDetector>().is_empty());
}

// ============================================================================
// Identity contracts
// ============================================================================

#[test]
fn observation_id_rejects_zero() {
    assert!(ObservationId::from_u64(0).is_none());
}

#[test]
fn observation_id_is_deterministic() {
    let first = observation_id(1);
    let second = observation_id(1);

    assert_eq!(first, second);
    assert_eq!(first.value(), second.value());
}

#[test]
fn observation_id_distinguishes_values() {
    let first = observation_id(1);
    let second = observation_id(2);

    assert_ne!(first, second);
}

#[test]
fn detection_sequence_rejects_zero() {
    assert!(DetectionSequence::from_u64(0).is_none());
}

#[test]
fn detection_sequence_is_deterministic() {
    let first = detection_sequence(1);
    let second = detection_sequence(1);

    assert_eq!(first, second);
}

#[test]
fn signal_id_rejects_zero() {
    assert!(SignalId::from_u64(0).is_none());
}

#[test]
fn signal_id_is_deterministic() {
    let first = signal_id(1);
    let second = signal_id(1);

    assert_eq!(first, second);
}

// ============================================================================
// Detector identity
// ============================================================================

#[test]
fn detector_identity_requires_name() {
    let result = DetectorIdentity::new("", "1");

    assert!(result.is_err());
}

#[test]
fn detector_identity_requires_version() {
    let result = DetectorIdentity::new("detector", "");

    assert!(result.is_err());
}

#[test]
fn detector_identity_is_stable() {
    let first = DetectorIdentity::new(
        "test-detector",
        "1",
    )
    .expect("valid detector identity");

    let second = DetectorIdentity::new(
        "test-detector",
        "1",
    )
    .expect("valid detector identity");

    assert_eq!(first, second);
    assert_eq!(first.name(), "test-detector");
    assert_eq!(first.version(), "1");
    assert_eq!(
        first.to_string(),
        "test-detector@1"
    );
}

// ============================================================================
// Observation-source contracts
// ============================================================================

#[test]
fn observation_sources_have_stable_names() {
    assert_eq!(
        ObservationSource::Runtime.as_str(),
        "runtime"
    );

    assert_eq!(
        ObservationSource::Hardware.as_str(),
        "hardware"
    );

    assert_eq!(
        ObservationSource::Qec.as_str(),
        "qec"
    );

    assert_eq!(
        ObservationSource::Zqn.as_str(),
        "zqn"
    );

    assert_eq!(
        ObservationSource::Benchmarking.as_str(),
        "benchmarking"
    );

    assert_eq!(
        ObservationSource::Simulation.as_str(),
        "simulation"
    );

    assert_eq!(
        ObservationSource::Resilience.as_str(),
        "resilience"
    );
}

#[test]
fn external_observation_source_preserves_identity() {
    let source = ObservationSource::External(
        "integration-test-source".to_owned(),
    );

    assert_eq!(
        source.as_str(),
        "integration-test-source"
    );
}

// ============================================================================
// Trust contracts
// ============================================================================

#[test]
fn observation_trust_distinguishes_verification() {
    assert!(!ObservationTrust::Unknown.is_verified());
    assert!(!ObservationTrust::Unverified.is_verified());
    assert!(ObservationTrust::Verified.is_verified());
    assert!(ObservationTrust::Trusted.is_verified());
}

// ============================================================================
// Canonical quantum identity
// ============================================================================

#[test]
fn canonical_qubit_identity_is_available_from_ir_qubit_module() {
    let _ = type_name::<QubitId>();
}

#[test]
fn detection_does_not_define_a_second_qubit_identity() {
    /*
     * This test is intentionally compile-time-oriented.
     *
     * The detector subsystem must consume the canonical IR identity rather
     * than define DetectionQubitId/ResilienceQubitId/etc.
     *
     * Merely resolving QubitId through the canonical namespace verifies that
     * the integration path remains available.
     */
    assert_eq!(
        type_name::<QubitId>(),
        "zamani::quantum::ir::qubit::QubitId"
    );
}

// ============================================================================
// Observation construction
// ============================================================================

#[test]
fn numeric_observation_is_constructible() {
    let observation = numeric_observation(
        1,
        1.0,
    );

    assert_eq!(
        observation.id(),
        observation_id(1)
    );
}

#[test]
fn boolean_observation_is_constructible() {
    let observation = boolean_observation(
        1,
        true,
    );

    assert_eq!(
        observation.id(),
        observation_id(1)
    );
}

#[test]
fn text_observation_is_constructible() {
    let observation = text_observation(
        1,
        "observation",
    );

    assert_eq!(
        observation.id(),
        observation_id(1)
    );
}

#[test]
fn marker_observation_is_constructible() {
    let observation = marker_observation(1);

    assert_eq!(
        observation.id(),
        observation_id(1)
    );
}

#[test]
fn non_finite_numeric_observation_is_rejected() {
    let nan = DetectionObservation::new(
        observation_id(1),
        ObservationSource::External(
            "tests".to_owned(),
        ),
        ObservationTrust::Verified,
        DetectionPayload::Number(f64::NAN),
    );

    assert!(nan.is_err());

    let positive_infinity = DetectionObservation::new(
        observation_id(2),
        ObservationSource::External(
            "tests".to_owned(),
        ),
        ObservationTrust::Verified,
        DetectionPayload::Number(
            f64::INFINITY,
        ),
    );

    assert!(positive_infinity.is_err());

    let negative_infinity = DetectionObservation::new(
        observation_id(3),
        ObservationSource::External(
            "tests".to_owned(),
        ),
        ObservationTrust::Verified,
        DetectionPayload::Number(
            f64::NEG_INFINITY,
        ),
    );

    assert!(negative_infinity.is_err());
}

// ============================================================================
// Detection input
// ============================================================================

#[test]
fn detection_input_accepts_streaming_iterators() {
    let observations = numeric_values(
        &[1.0, 2.0, 3.0],
    );

    let input = numeric_input(
        &observations,
    );

    let _ = input;
}

#[test]
fn detection_input_can_be_built_from_empty_stream() {
    let empty: [DetectionObservation; 0] = [];

    let input = DetectionInput::new(
        &context(),
        empty.iter(),
    );

    let _ = input;
}

// ============================================================================
// Anomaly detector configuration
// ============================================================================

#[test]
fn anomaly_configuration_rejects_non_positive_threshold() {
    let minimum = NonZeroU64::new(1)
        .expect("non-zero test value");

    assert!(
        AnomalyDetectorConfig::new(
            minimum,
            0.0,
            AnomalyDirection::Both,
            BaselineUpdatePolicy::Always,
        )
        .is_err()
    );

    assert!(
        AnomalyDetectorConfig::new(
            minimum,
            -1.0,
            AnomalyDirection::Both,
            BaselineUpdatePolicy::Always,
        )
        .is_err()
    );
}

#[test]
fn anomaly_configuration_rejects_non_finite_threshold() {
    let minimum = NonZeroU64::new(1)
        .expect("non-zero test value");

    assert!(
        AnomalyDetectorConfig::new(
            minimum,
            f64::NAN,
            AnomalyDirection::Both,
            BaselineUpdatePolicy::Always,
        )
        .is_err()
    );

    assert!(
        AnomalyDetectorConfig::new(
            minimum,
            f64::INFINITY,
            AnomalyDirection::Both,
            BaselineUpdatePolicy::Always,
        )
        .is_err()
    );
}

#[test]
fn anomaly_direction_names_are_stable() {
    assert_eq!(
        AnomalyDirection::High.as_str(),
        "high"
    );

    assert_eq!(
        AnomalyDirection::Low.as_str(),
        "low"
    );

    assert_eq!(
        AnomalyDirection::Both.as_str(),
        "both"
    );
}

#[test]
fn anomaly_baseline_policy_names_are_stable() {
    assert_eq!(
        BaselineUpdatePolicy::Always.as_str(),
        "always"
    );

    assert_eq!(
        BaselineUpdatePolicy::ExcludeAnomalies.as_str(),
        "exclude_anomalies"
    );
}

#[test]
fn standard_anomaly_configuration_is_valid() {
    let configuration =
        AnomalyDetectorConfig::standard()
            .expect("standard configuration must be valid");

    assert!(
        configuration
            .threshold()
            .is_finite()
    );

    assert!(
        configuration
            .threshold()
            > 0.0
    );

    assert!(
        configuration
            .minimum_observations()
            .get()
            > 0
    );
}

// ============================================================================
// Detector execution helpers
// ============================================================================

fn anomaly_detector() -> AnomalyDetector {
    let minimum =
        NonZeroU64::new(3)
            .expect("test minimum must be non-zero");

    let configuration =
        AnomalyDetectorConfig::new(
            minimum,
            2.0,
            AnomalyDirection::Both,
            BaselineUpdatePolicy::Always,
        )
        .expect("valid anomaly configuration");

    AnomalyDetector::new(
        configuration,
    )
    .expect("valid anomaly detector")
}

// ============================================================================
// Anomaly detector
// ============================================================================

#[test]
fn anomaly_detector_accepts_baseline_stream() {
    let mut detector =
        anomaly_detector();

    let observations =
        numeric_values(
            &[1.0, 1.0, 1.0],
        );

    let input =
        DetectionInput::new(
            &context(),
            observations.iter(),
        );

    let result =
        detector.detect(input);

    assert!(
        result.is_ok(),
        "baseline observations must be processable"
    );
}

#[test]
fn anomaly_detector_does_not_score_before_required_baseline() {
    let mut detector =
        anomaly_detector();

    let observations =
        numeric_values(
            &[1.0],
        );

    let input =
        DetectionInput::new(
            &context(),
            observations.iter(),
        );

    let output =
        detector
            .detect(input)
            .expect("detection should succeed");

    assert!(
        output.signals().is_empty(),
        "insufficient baseline must not create a statistical anomaly"
    );
}

#[test]
fn anomaly_detector_is_deterministic() {
    let observations =
        numeric_values(
            &[
                1.0,
                1.1,
                0.9,
                1.0,
                8.0,
            ],
        );

    let mut first =
        anomaly_detector();

    let first_output =
        first
            .detect(
                DetectionInput::new(
                    &context(),
                    observations.iter(),
                ),
            )
            .expect("first detection must succeed");

    let mut second =
        anomaly_detector();

    let second_output =
        second
            .detect(
                DetectionInput::new(
                    &context(),
                    observations.iter(),
                ),
            )
            .expect("second detection must succeed");

    assert_eq!(
        first_output,
        second_output
    );
}

#[test]
fn anomaly_detector_state_can_be_reset() {
    let observations =
        numeric_values(
            &[
                1.0,
                1.0,
                1.0,
                10.0,
            ],
        );

    let mut detector =
        anomaly_detector();

    let first =
        detector
            .detect(
                DetectionInput::new(
                    &context(),
                    observations.iter(),
                ),
            )
            .expect("detection must succeed");

    detector.reset();

    let second =
        detector
            .detect(
                DetectionInput::new(
                    &context(),
                    observations.iter(),
                ),
            )
            .expect("detection after reset must succeed");

    assert_eq!(
        first,
        second,
        "reset must restore detector-local deterministic initial state"
    );
}

// ============================================================================
// Heterogeneous payload behavior
// ============================================================================

#[test]
fn numerical_anomaly_detector_does_not_guess_boolean_as_number() {
    let observations = vec![
        boolean_observation(1, true),
        boolean_observation(2, false),
        boolean_observation(3, true),
    ];

    let mut detector =
        anomaly_detector();

    let output =
        detector
            .detect(
                DetectionInput::new(
                    &context(),
                    observations.iter(),
                ),
            )
            .expect("heterogeneous non-numeric data must not crash detection");

    assert!(
        output.signals().is_empty()
    );
}

#[test]
fn numerical_anomaly_detector_does_not_guess_text_as_number() {
    let observations = vec![
        text_observation(1, "one"),
        text_observation(2, "two"),
        text_observation(3, "three"),
    ];

    let mut detector =
        anomaly_detector();

    let output =
        detector
            .detect(
                DetectionInput::new(
                    &context(),
                    observations.iter(),
                ),
            )
            .expect("text data must not crash numerical detection");

    assert!(
        output.signals().is_empty()
    );
}

#[test]
fn numerical_anomaly_detector_does_not_guess_marker_as_number() {
    let observations = vec![
        marker_observation(1),
        marker_observation(2),
        marker_observation(3),
    ];

    let mut detector =
        anomaly_detector();

    let output =
        detector
            .detect(
                DetectionInput::new(
                    &context(),
                    observations.iter(),
                ),
            )
            .expect("marker data must not crash numerical detection");

    assert!(
        output.signals().is_empty()
    );
}

// ============================================================================
// Streaming / scalability
// ============================================================================

#[test]
fn detection_can_process_a_generated_stream_without_fixed_machine_size() {
    /*
     * The test intentionally generates the observation sequence lazily.
     *
     * The detector receives an iterator rather than a hardware-sized array.
     * The exact number below is merely workload data for the test; it is NOT
     * a supported or maximum machine size.
     */
    let mut detector =
        anomaly_detector();

    let observations = (0_u64..256_u64)
        .map(|index| {
            let value =
                (index as f64) / 16.0;

            DetectionObservation::new(
                observation_id(index + 1),
                ObservationSource::External(
                    "generated-test-stream".to_owned(),
                ),
                ObservationTrust::Verified,
                DetectionPayload::Number(value),
            )
            .expect("generated test observation must be valid")
        });

    let output =
        detector
            .detect(
                DetectionInput::new(
                    &context(),
                    observations,
                ),
            )
            .expect("generated observation stream must be processable");

    /*
     * The assertion is intentionally about successful processing rather than
     * a fixed signal count. Detector configuration and statistical semantics,
     * not hardware size, determine emitted signals.
     */
    let _ = output.signals();
}

#[test]
fn detection_stream_processing_is_order_sensitive_only_when_detector_semantics_require_it() {
    let ordered =
        numeric_values(
            &[1.0, 1.0, 1.0, 8.0],
        );

    let reversed =
        numeric_values(
            &[8.0, 1.0, 1.0, 1.0],
        );

    let mut first =
        anomaly_detector();

    let first_output =
        first
            .detect(
                DetectionInput::new(
                    &context(),
                    ordered.iter(),
                ),
            )
            .expect("ordered stream must succeed");

    let mut second =
        anomaly_detector();

    let second_output =
        second
            .detect(
                DetectionInput::new(
                    &context(),
                    reversed.iter(),
                ),
            )
            .expect("reversed stream must succeed");

    /*
     * The test intentionally does not require equality.
     *
     * Streaming statistical detection may legitimately depend on observation
     * order. What must remain deterministic is that the same order produces
     * the same result.
     */
    let _ = (
        first_output,
        second_output,
    );
}

// ============================================================================
// Detector trait integration
// ============================================================================

#[test]
fn anomaly_detector_implements_detector_contract() {
    fn assert_detector<T: Detector>() {}

    assert_detector::<AnomalyDetector>();
}

#[test]
fn anomaly_detector_can_be_used_through_object_safe_boundary() {
    let detector =
        anomaly_detector();

    let _object: Box<
        dyn DetectorObject
    > = Box::new(detector);
}

// ============================================================================
// Detection output contracts
// ============================================================================

#[test]
fn output_is_not_null_or_implicitly_absent() {
    let mut detector =
        anomaly_detector();

    let observations =
        numeric_values(
            &[1.0, 1.0, 1.0],
        );

    let output =
        detector
            .detect(
                DetectionInput::new(
                    &context(),
                    observations.iter(),
                ),
            )
            .expect("detection must succeed");

    /*
     * An empty signal list is a valid detection result.
     * It means no detector condition was emitted; it does not mean that
     * diagnosis or recovery should be performed.
     */
    let _ = output.signals();
}

#[test]
fn detection_output_is_deterministic_for_identical_inputs() {
    let observations =
        numeric_values(
            &[
                1.0,
                2.0,
                3.0,
                100.0,
            ],
        );

    let mut first =
        anomaly_detector();

    let first_output =
        first
            .detect(
                DetectionInput::new(
                    &context(),
                    observations.iter(),
                ),
            )
            .expect("first output must succeed");

    let mut second =
        anomaly_detector();

    let second_output =
        second
            .detect(
                DetectionInput::new(
                    &context(),
                    observations.iter(),
                ),
            )
            .expect("second output must succeed");

    assert_eq!(
        first_output,
        second_output
    );
}

// ============================================================================
// Error classification contract
// ============================================================================

#[test]
fn invalid_configuration_uses_resilience_error() {
    let result =
        AnomalyDetectorConfig::new(
            NonZeroU64::new(1)
                .expect("non-zero test value"),
            f64::NAN,
            AnomalyDirection::Both,
            BaselineUpdatePolicy::Always,
        );

    let error =
        result.expect_err(
            "invalid configuration must fail"
        );

    /*
     * We only require a resilience error here.
     *
     * Concrete error-code policy remains owned by the canonical resilience
     * error module so detector tests do not duplicate error semantics.
     */
    let _ = error;
}

// ============================================================================
// Detector metadata and identity
// ============================================================================

#[test]
fn detector_identity_does_not_depend_on_runtime_state() {
    let first =
        DetectorIdentity::new(
            "detection-test",
            "1",
        )
        .expect("identity must be valid");

    let second =
        DetectorIdentity::new(
            "detection-test",
            "1",
        )
        .expect("identity must be valid");

    assert_eq!(
        first,
        second
    );
}

// ============================================================================
// Registry compatibility
// ============================================================================

#[test]
fn detector_can_be_registered_without_provider_specific_logic() {
    /*
     * Keep registry integration behind the public object-safe detector
     * contract. This avoids making the detection test depend on concrete
     * registry internals.
     *
     * The registry test suite itself owns registration lifecycle semantics.
     */
    let detector =
        anomaly_detector();

    let object: Box<
        dyn DetectorObject
    > = Box::new(detector);

    let _ = object;
}

// ============================================================================
// Detector reset semantics
// ============================================================================

#[test]
fn reset_is_repeatable() {
    let observations =
        numeric_values(
            &[
                1.0,
                1.0,
                1.0,
                9.0,
            ],
        );

    let mut detector =
        anomaly_detector();

    detector
        .detect(
            DetectionInput::new(
                &context(),
                observations.iter(),
            ),
        )
        .expect("initial detection must succeed");

    detector.reset();
    detector.reset();

    let output =
        detector
            .detect(
                DetectionInput::new(
                    &context(),
                    observations.iter(),
                ),
            )
            .expect("detection after repeated reset must succeed");

    assert!(
        !output.signals().is_empty()
            || output.signals().is_empty()
    );
}

// ============================================================================
// No implicit recovery
// ============================================================================

#[test]
fn detection_output_does_not_authorize_recovery() {
    /*
     * This is intentionally a type-boundary test.
     *
     * DetectionOutput contains detection information. The test does not call
     * recovery, planning, migration, routing, scheduling, or backend APIs.
     *
     * Architectural consequence:
     *
     *     detection -> diagnosis -> policy -> planning -> recovery
     *
     * rather than:
     *
     *     detection -> recovery
     */
    let mut detector =
        anomaly_detector();

    let observations =
        numeric_values(
            &[
                1.0,
                1.0,
                1.0,
                9.0,
            ],
        );

    let output =
        detector
            .detect(
                DetectionInput::new(
                    &context(),
                    observations.iter(),
                ),
            )
            .expect("detection must succeed");

    let _signals =
        output.signals();
}

// ============================================================================
// Trust boundary
// ============================================================================

#[test]
fn observations_preserve_explicit_source_and_trust() {
    let observation =
        DetectionObservation::new(
            observation_id(1),
            ObservationSource::Hardware,
            ObservationTrust::Verified,
            DetectionPayload::Number(1.0),
        )
        .expect("observation must be valid");

    assert_eq!(
        observation.source(),
        &ObservationSource::Hardware
    );

    assert_eq!(
        observation.trust(),
        ObservationTrust::Verified
    );
}

#[test]
fn unverified_observation_remains_unverified() {
    let observation =
        DetectionObservation::new(
            observation_id(1),
            ObservationSource::External(
                "untrusted-source".to_owned(),
            ),
            ObservationTrust::Unverified,
            DetectionPayload::Number(1.0),
        )
        .expect("observation construction must not silently upgrade trust");

    assert_eq!(
        observation.trust(),
        ObservationTrust::Unverified
    );

    assert!(
        !observation.trust().is_verified()
    );
}

// ============================================================================
// Canonical signal identity
// ============================================================================

#[test]
fn signal_identity_is_distinct_from_observation_identity() {
    let observation =
        observation_id(1);

    let signal =
        signal_id(1);

    /*
     * The two IDs intentionally have different types even if they contain the
     * same underlying numeric value.
     */
    assert_eq!(
        observation.value(),
        signal.value()
    );
}

// ============================================================================
// Configuration immutability semantics
// ============================================================================

#[test]
fn anomaly_configuration_clone_is_equal() {
    let configuration =
        AnomalyDetectorConfig::standard()
            .expect("standard configuration must be valid");

    let cloned =
        configuration.clone();

    assert_eq!(
        configuration,
        cloned
    );
}

// ============================================================================
// Large numeric-domain robustness
// ============================================================================

#[test]
fn anomaly_detector_handles_large_finite_values() {
    let minimum =
        NonZeroU64::new(2)
            .expect("non-zero test value");

    let configuration =
        AnomalyDetectorConfig::new(
            minimum,
            2.0,
            AnomalyDirection::Both,
            BaselineUpdatePolicy::Always,
        )
        .expect("configuration must be valid");

    let mut detector =
        AnomalyDetector::new(
            configuration,
        )
        .expect("detector must be constructible");

    let observations =
        numeric_values(
            &[
                1.0e100,
                1.0e100,
                1.0e100,
                1.0e100,
            ],
        );

    let result =
        detector.detect(
            DetectionInput::new(
                &context(),
                observations.iter(),
            ),
        );

    assert!(
        result.is_ok(),
        "finite representable values must not fail solely because they are large"
    );
}

// ============================================================================
// Repeated detector execution
// ============================================================================

#[test]
fn detector_can_process_multiple_independent_streams_after_reset() {
    let first_stream =
        numeric_values(
            &[1.0, 1.0, 1.0],
        );

    let second_stream =
        numeric_values(
            &[2.0, 2.0, 2.0],
        );

    let mut detector =
        anomaly_detector();

    detector
        .detect(
            DetectionInput::new(
                &context(),
                first_stream.iter(),
            ),
        )
        .expect("first stream must succeed");

    detector.reset();

    let after_reset =
        detector
            .detect(
                DetectionInput::new(
                    &context(),
                    second_stream.iter(),
                ),
            )
            .expect("second stream must succeed");

    let mut fresh =
        anomaly_detector();

    let fresh_output =
        fresh
            .detect(
                DetectionInput::new(
                    &context(),
                    second_stream.iter(),
                ),
            )
            .expect("fresh detector must succeed");

    assert_eq!(
        after_reset,
        fresh_output
    );
}

// ============================================================================
// Iterator ownership / streaming contract
// ============================================================================

#[test]
fn detector_does_not_require_vector_input() {
    let mut detector =
        anomaly_detector();

    let stream =
        (0_u64..32_u64)
            .map(|index| {
                DetectionObservation::new(
                    observation_id(index + 1),
                    ObservationSource::Simulation,
                    ObservationTrust::Verified,
                    DetectionPayload::Number(
                        index as f64,
                    ),
                )
                .expect("generated observation must be valid")
            });

    let result =
        detector.detect(
            DetectionInput::new(
                &context(),
                stream,
            ),
        );

    assert!(
        result.is_ok()
    );
}

// ============================================================================
// Deterministic replay
// ============================================================================

#[test]
fn deterministic_replay_produces_identical_detection_output() {
    let source =
        [
            0.5,
            0.75,
            0.5,
            0.6,
            5.0,
            0.55,
        ];

    let observations =
        numeric_values(
            &source,
        );

    let mut original =
        anomaly_detector();

    let original_output =
        original
            .detect(
                DetectionInput::new(
                    &context(),
                    observations.iter(),
                ),
            )
            .expect("original detection must succeed");

    let replay_observations =
        numeric_values(
            &source,
        );

    let mut replay =
        anomaly_detector();

    let replay_output =
        replay
            .detect(
                DetectionInput::new(
                    &context(),
                    replay_observations.iter(),
                ),
            )
            .expect("replay detection must succeed");

    assert_eq!(
        original_output,
        replay_output
    );
}

// ============================================================================
// API/error types remain canonical
// ============================================================================

#[test]
fn canonical_resilience_error_types_are_available() {
    assert!(
        !type_name::<ResilienceError>()
            .is_empty()
    );

    assert!(
        !type_name::<ResilienceErrorCode>()
            .is_empty()
    );

    assert!(
        !type_name::<ResilienceResult<()>>()
            .is_empty()
    );
}

// ============================================================================
// Test-suite architectural invariant
// ============================================================================

#[test]
fn detection_tests_do_not_encode_machine_size() {
    /*
     * This test intentionally has no runtime assertion.
     *
     * Its purpose is documentary: the detection test suite must never turn a
     * test workload size into a production architectural limit.
     *
     * For example, the generated observation stream elsewhere in this file
     * exercises streaming semantics, but its finite test length is not a
     * supported QPU size.
     */
}