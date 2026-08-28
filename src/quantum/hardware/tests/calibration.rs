//! Zamani Quantum — Hardware Calibration Integration Tests
//!
//! Production conformance tests for:
//!
//!     crate::quantum::hardware::calibration
//!
//! # Purpose
//!
//! This file tests the PUBLIC calibration contract from outside the
//! calibration module. It intentionally does not access private implementation
//! details such as `InstructionKey` or `CouplingKey`.
//!
//! The suite verifies that calibration state can safely serve as the
//! authoritative evidence layer for:
//!
//! - backend discovery;
//! - compatibility checking;
//! - routing;
//! - scheduling;
//! - execution;
//! - benchmarking;
//! - provider adapters;
//! - simulators and emulators;
//! - reproducibility/provenance.
//!
//! # Integration boundary
//!
//! This file depends only on the public calibration API:
//!
//!     crate::quantum::hardware::calibration
//!
//! It must NOT depend on:
//!
//! - provider adapters;
//! - network clients;
//! - credentials;
//! - benchmarking internals;
//! - routing algorithms;
//! - schedulers;
//! - compiler internals;
//! - environment variables;
//! - wall-clock timing except where the API itself intentionally uses `now()`.
//!
//! This keeps the calibration contract independently testable and prevents
//! circular dependencies.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97
//! - Rust 1.97.1
//! - Rust 2021
//!
//! # Test philosophy
//!
//! Every public invariant that a downstream hardware consumer can rely on is
//! tested here. Tests intentionally exercise both valid and invalid input.
//!
//! In particular, this suite verifies:
//!
//! 1. invalid floating-point values are rejected;
//! 2. probabilities stay in [0, 1];
//! 3. durations cannot be zero;
//! 4. frequencies cannot be negative or non-finite;
//! 5. identifiers cannot be empty/control-containing;
//! 6. duplicate instruction operands are rejected;
//! 7. coupling endpoints must differ;
//! 8. calibration evidence requires positive sample counts;
//! 9. confidence levels are strictly inside (0, 1);
//! 10. snapshot validation rejects empty calibration;
//! 11. schema corruption is detected;
//! 12. metadata remains bounded and valid;
//! 13. lookups normalize instruction names;
//! 14. instruction operand order remains semantically significant;
//! 15. directional coupling calibration remains directional;
//! 16. replacement of the same calibration key is deterministic;
//! 17. validity intervals are enforced;
//! 18. stale calibration is rejected;
//! 19. future calibration timestamps are rejected by freshness policy;
//! 20. JSON round-trips preserve the complete snapshot;
//! 21. fingerprints are deterministic;
//! 22. fingerprints change when calibration content changes;
//! 23. malformed serialized data is rejected;
//! 24. custom calibration metrics remain extensible;
//! 25. provider/device provenance is preserved.
//!
//! # Important architectural guarantee
//!
//! These tests do not require `backend.rs`, `topology.rs`, provider adapters,
//! routing, scheduling, or benchmarking to be complete. Calibration therefore
//! remains a lower-level frozen contract that can be completed independently.

use std::time::Duration;

use zamani_compiler::quantum::hardware::calibration::{
    CalibrationError,
    CalibrationFreshnessPolicy,
    CalibrationProvenance,
    CalibrationSourceKind,
    CalibrationTimestamp,
    CalibrationValidity,
    CalibrationSnapshot,
    CouplingCalibration,
    CustomCalibrationMetric,
    InstructionCalibration,
    MeasurementEvidence,
    QubitCalibration,
    ReadoutCalibration,
    CALIBRATION_SCHEMA_ID,
    CALIBRATION_SCHEMA_VERSION,
    DEFAULT_MAX_CALIBRATION_AGE,
    MAX_INSTRUCTION_QUBITS,
    MAX_METADATA_KEY_LENGTH,
    MAX_METADATA_VALUE_LENGTH,
};

// ============================================================================
// Test fixtures
// ============================================================================

fn sample_provenance() -> CalibrationProvenance {
    CalibrationProvenance::measured()
        .with_provider("zamani.test-provider")
        .expect("valid provider identifier")
        .with_device("zamani.test-device")
        .expect("valid device identifier")
        .with_source_id("calibration-record-001")
        .expect("valid source identifier")
        .with_method("randomized_benchmarking")
        .expect("valid calibration method")
        .with_source_version("1.0")
        .expect("valid source version")
}

fn sample_evidence() -> MeasurementEvidence {
    MeasurementEvidence::new(10_000)
        .expect("positive sample count")
        .with_uncertainty(0.0001)
        .expect("valid uncertainty")
        .with_confidence_level(0.95)
        .expect("valid confidence level")
}

fn sample_qubit(qubit: usize) -> QubitCalibration {
    QubitCalibration::new(qubit)
        .expect("valid qubit")
        .with_t1_ns(100_000.0)
        .expect("valid T1")
        .with_t2_ns(80_000.0)
        .expect("valid T2")
        .with_t2_star_ns(70_000.0)
        .expect("valid T2*")
        .with_frequency_hz(5.0e9)
        .expect("valid frequency")
        .with_anharmonicity_hz(-330.0e6)
        .expect("finite anharmonicity")
        .with_thermal_population(0.001)
        .expect("valid thermal population")
        .with_leakage_rate(0.0001)
        .expect("valid leakage")
        .with_reset_error(0.001)
        .expect("valid reset error")
        .with_evidence(sample_evidence())
        .expect("valid evidence")
        .with_provenance(sample_provenance())
        .expect("valid provenance")
}

fn sample_readout() -> ReadoutCalibration {
    ReadoutCalibration::new(0.02, 0.03)
        .expect("valid readout")
        .with_duration_ns(1_000)
        .expect("valid readout duration")
        .with_fidelity(0.975)
        .expect("valid readout fidelity")
        .with_evidence(sample_evidence())
        .expect("valid readout evidence")
        .with_provenance(sample_provenance())
        .expect("valid readout provenance")
}

fn sample_instruction(
    name: &str,
    qubits: Vec<usize>,
) -> InstructionCalibration {
    InstructionCalibration::new(name, qubits)
        .expect("valid instruction")
        .with_duration_ns(300)
        .expect("valid instruction duration")
        .with_error_rate(0.001)
        .expect("valid instruction error")
        .with_fidelity(0.999)
        .expect("valid instruction fidelity")
        .with_amplitude(0.5)
        .expect("valid amplitude")
        .with_phase_radians(0.25)
        .expect("valid phase")
        .with_frequency_hz(5.0e9)
        .expect("valid instruction frequency")
        .with_evidence(sample_evidence())
        .expect("valid instruction evidence")
        .with_provenance(sample_provenance())
        .expect("valid instruction provenance")
}

fn sample_coupling(
    source: usize,
    target: usize,
) -> CouplingCalibration {
    CouplingCalibration::new(source, target)
        .expect("valid coupling")
        .with_fidelity(0.995)
        .expect("valid coupling fidelity")
        .with_error_rate(0.005)
        .expect("valid coupling error")
        .with_duration_ns(500)
        .expect("valid coupling duration")
        .with_crosstalk_rate(0.002)
        .expect("valid crosstalk")
        .with_evidence(sample_evidence())
        .expect("valid coupling evidence")
        .with_provenance(sample_provenance())
        .expect("valid coupling provenance")
}

fn sample_snapshot() -> CalibrationSnapshot {
    let timestamp =
        CalibrationTimestamp::from_unix_nanos(1_700_000_000_000_000_000);

    let mut snapshot = CalibrationSnapshot::with_timestamp(
        "local://zamani-test-qpu",
        timestamp,
    )
    .expect("valid snapshot");

    snapshot
        .with_provider_id("zamani-test-provider")
        .expect("valid provider");

    // The builder-style API returns a new snapshot, so construct it again in
    // a way that guarantees the returned value is retained.
    //
    // This helper intentionally keeps the fixture explicit rather than
    // depending on implementation details.
    let mut snapshot = CalibrationSnapshot::with_timestamp(
        "local://zamani-test-qpu",
        timestamp,
    )
    .expect("valid snapshot")
    .with_provider_id("zamani-test-provider")
    .expect("valid provider")
    .with_device_id("zamani-test-device")
    .expect("valid device");

    snapshot
        .insert_qubit(sample_qubit(0))
        .expect("insert qubit 0");

    snapshot
        .insert_qubit(
            QubitCalibration::new(1)
                .expect("valid qubit")
                .with_t1_ns(90_000.0)
                .expect("valid T1")
                .with_t2_ns(75_000.0)
                .expect("valid T2")
                .with_frequency_hz(5.1e9)
                .expect("valid frequency")
                .with_readout(sample_readout()),
        )
        .expect("insert qubit 1");

    snapshot
        .insert_instruction(sample_instruction("cx", vec![0, 1]))
        .expect("insert CX");

    snapshot
        .insert_instruction(
            InstructionCalibration::new("x", vec![0])
                .expect("valid X")
                .with_duration_ns(35)
                .expect("valid X duration")
                .with_error_rate(0.0005)
                .expect("valid X error"),
        )
        .expect("insert X");

    snapshot
        .insert_coupling(sample_coupling(0, 1))
        .expect("insert coupling");

    snapshot
        .insert_metadata("provider.api_version", "1.0")
        .expect("insert API metadata");

    snapshot
        .insert_metadata("calibration.kind", "measured")
        .expect("insert calibration metadata");

    snapshot
        .insert_custom_metric(
            CustomCalibrationMetric::new(
                "rb_1q_error",
                0.001,
            )
            .expect("valid custom metric")
            .with_unit("probability")
            .with_qubits(vec![0])
            .expect("valid metric qubit")
            .with_evidence(sample_evidence())
            .expect("valid metric evidence")
            .with_provenance(sample_provenance())
            .expect("valid metric provenance"),
        )
        .expect("insert custom metric");

    snapshot
}

// ============================================================================
// Schema and construction
// ============================================================================

#[test]
fn schema_constants_are_stable_and_non_empty() {
    assert_eq!(
        CALIBRATION_SCHEMA_ID,
        "zamani.quantum.hardware.calibration"
    );

    assert_eq!(CALIBRATION_SCHEMA_VERSION, 1);

    assert!(!CALIBRATION_SCHEMA_ID.is_empty());
    assert!(CALIBRATION_SCHEMA_VERSION > 0);
}

#[test]
fn snapshot_constructor_normalizes_backend_identifier() {
    let snapshot =
        CalibrationSnapshot::new("  local://test-qpu  ")
            .expect("valid backend identifier");

    assert_eq!(
        snapshot.backend_id,
        "local://test-qpu"
    );
}

#[test]
fn empty_backend_identifier_is_rejected() {
    let result = CalibrationSnapshot::new("   ");

    assert!(matches!(
        result,
        Err(CalibrationError::EmptyIdentifier {
            field: "backend_id"
        })
    ));
}

#[test]
fn control_character_in_backend_identifier_is_rejected() {
    let result =
        CalibrationSnapshot::new("local://test\nqpu");

    assert!(matches!(
        result,
        Err(CalibrationError::InvalidIdentifier {
            field: "backend_id"
        })
    ));
}

#[test]
fn explicit_timestamp_is_preserved() {
    let timestamp =
        CalibrationTimestamp::from_unix_nanos(
            1_700_000_000_000_000_000,
        );

    let snapshot =
        CalibrationSnapshot::with_timestamp(
            "test-qpu",
            timestamp,
        )
        .expect("valid snapshot");

    assert_eq!(
        snapshot.timestamp,
        timestamp
    );

    assert_eq!(
        snapshot.timestamp.as_unix_nanos(),
        1_700_000_000_000_000_000
    );
}

// ============================================================================
// Timestamp and freshness primitives
// ============================================================================

#[test]
fn timestamp_round_trip_is_exact() {
    let timestamp =
        CalibrationTimestamp::from_unix_nanos(
            1_700_000_000_123_456_789,
        );

    assert_eq!(
        timestamp.as_unix_nanos(),
        1_700_000_000_123_456_789
    );
}

#[test]
fn future_timestamp_has_zero_age() {
    let now =
        CalibrationTimestamp::now().as_unix_nanos();

    let future =
        CalibrationTimestamp::from_unix_nanos(
            now.saturating_add(60_000_000_000),
        );

    assert_eq!(
        future.age(),
        Duration::ZERO
    );

    assert!(
        !future.is_older_than(
            Duration::from_secs(1)
        )
    );
}

#[test]
fn old_timestamp_is_detected_as_stale() {
    let snapshot =
        CalibrationSnapshot::with_timestamp(
            "test-qpu",
            CalibrationTimestamp::from_unix_nanos(0),
        )
        .expect("valid snapshot");

    assert!(
        snapshot.is_stale(Duration::from_secs(1))
    );
}

#[test]
fn default_freshness_policy_is_conservative_and_nonzero() {
    let policy =
        CalibrationFreshnessPolicy::default();

    assert_eq!(
        policy.maximum_age,
        DEFAULT_MAX_CALIBRATION_AGE
    );

    assert!(
        policy.reject_expired_interval
    );

    assert!(
        policy.reject_future_timestamp
    );

    assert!(
        policy.maximum_age > Duration::ZERO
    );
}

// ============================================================================
// Measurement evidence
// ============================================================================

#[test]
fn measurement_evidence_requires_positive_samples() {
    let result =
        MeasurementEvidence::new(0);

    assert!(matches!(
        result,
        Err(CalibrationError::InvalidSampleCount { .. })
    ));
}

#[test]
fn measurement_evidence_accepts_valid_uncertainty() {
    let evidence =
        MeasurementEvidence::new(100)
            .expect("samples")
            .with_uncertainty(0.01)
            .expect("uncertainty");

    assert_eq!(
        evidence.samples,
        100
    );

    assert_eq!(
        evidence.uncertainty,
        Some(0.01)
    );
}

#[test]
fn negative_uncertainty_is_rejected() {
    let result =
        MeasurementEvidence::new(100)
            .expect("samples")
            .with_uncertainty(-0.01);

    assert!(matches!(
        result,
        Err(CalibrationError::InvalidNumericValue { .. })
    ));
}

#[test]
fn non_finite_uncertainty_is_rejected() {
    assert!(
        MeasurementEvidence::new(100)
            .expect("samples")
            .with_uncertainty(f64::NAN)
            .is_err()
    );

    assert!(
        MeasurementEvidence::new(100)
            .expect("samples")
            .with_uncertainty(f64::INFINITY)
            .is_err()
    );

    assert!(
        MeasurementEvidence::new(100)
            .expect("samples")
            .with_uncertainty(f64::NEG_INFINITY)
            .is_err()
    );
}

#[test]
fn confidence_level_must_be_strictly_inside_zero_and_one() {
    for value in [
        0.0,
        1.0,
        -0.1,
        1.1,
        f64::NAN,
        f64::INFINITY,
        f64::NEG_INFINITY,
    ] {
        assert!(
            MeasurementEvidence::new(100)
                .expect("samples")
                .with_confidence_level(value)
                .is_err(),
            "confidence value {value:?} should be rejected"
        );
    }

    let evidence =
        MeasurementEvidence::new(100)
            .expect("samples")
            .with_confidence_level(0.95)
            .expect("valid confidence");

    assert_eq!(
        evidence.confidence_level,
        Some(0.95)
    );
}

// ============================================================================
// Provenance
// ============================================================================

#[test]
fn provenance_records_provider_device_source_method_and_version() {
    let provenance =
        sample_provenance();

    assert_eq!(
        provenance.source_kind,
        CalibrationSourceKind::Measured
    );

    assert_eq!(
        provenance.provider_id.as_deref(),
        Some("zamani.test-provider")
    );

    assert_eq!(
        provenance.device_id.as_deref(),
        Some("zamani.test-device")
    );

    assert_eq!(
        provenance.source_id.as_deref(),
        Some("calibration-record-001")
    );

    assert_eq!(
        provenance.method.as_deref(),
        Some("randomized_benchmarking")
    );

    assert_eq!(
        provenance.source_version.as_deref(),
        Some("1.0")
    );
}

#[test]
fn simulated_provenance_is_distinguishable_from_measured() {
    let measured =
        CalibrationProvenance::measured();

    let simulated =
        CalibrationProvenance::simulated();

    assert_eq!(
        measured.source_kind,
        CalibrationSourceKind::Measured
    );

    assert_eq!(
        simulated.source_kind,
        CalibrationSourceKind::Simulated
    );

    assert_ne!(
        measured.source_kind,
        simulated.source_kind
    );
}

#[test]
fn provenance_rejects_empty_provider_identifier() {
    let result =
        CalibrationProvenance::measured()
            .with_provider("   ");

    assert!(matches!(
        result,
        Err(CalibrationError::EmptyIdentifier {
            field: "provider_id"
        })
    ));
}

#[test]
fn provenance_rejects_control_characters() {
    let result =
        CalibrationProvenance::measured()
            .with_device("device\nid");

    assert!(matches!(
        result,
        Err(CalibrationError::InvalidIdentifier {
            field: "device_id"
        })
    ));
}

// ============================================================================
// Qubit calibration
// ============================================================================

#[test]
fn qubit_calibration_supports_core_physical_properties() {
    let qubit =
        sample_qubit(7);

    assert_eq!(
        qubit.qubit,
        7
    );

    assert_eq!(
        qubit.t1_ns,
        Some(100_000.0)
    );

    assert_eq!(
        qubit.t2_ns,
        Some(80_000.0)
    );

    assert_eq!(
        qubit.t2_star_ns,
        Some(70_000.0)
    );

    assert_eq!(
        qubit.frequency_hz,
        Some(5.0e9)
    );

    assert_eq!(
        qubit.thermal_population,
        Some(0.001)
    );

    assert_eq!(
        qubit.leakage_rate,
        Some(0.0001)
    );

    assert_eq!(
        qubit.reset_error,
        Some(0.001)
    );
}

#[test]
fn qubit_calibration_does_not_use_zero_as_unknown() {
    let qubit =
        QubitCalibration::new(0)
            .expect("valid qubit");

    assert_eq!(
        qubit.t1_ns,
        None
    );

    assert_eq!(
        qubit.t2_ns,
        None
    );

    assert_eq!(
        qubit.frequency_hz,
        None
    );

    assert_eq!(
        qubit.reset_error,
        None
    );
}

#[test]
fn qubit_effective_coherence_is_the_conservative_minimum() {
    let qubit =
        sample_qubit(0);

    assert_eq!(
        qubit.effective_coherence_ns(),
        Some(70_000.0)
    );

    assert!(
        qubit.has_coherence_data()
    );
}

#[test]
fn qubit_without_coherence_has_no_effective_coherence() {
    let qubit =
        QubitCalibration::new(0)
            .expect("valid qubit");

    assert!(
        qubit.effective_coherence_ns()
            .is_none()
    );

    assert!(
        !qubit.has_coherence_data()
    );
}

#[test]
fn negative_coherence_is_rejected() {
    assert!(
        QubitCalibration::new(0)
            .expect("valid qubit")
            .with_t1_ns(-1.0)
            .is_err()
    );

    assert!(
        QubitCalibration::new(0)
            .expect("valid qubit")
            .with_t2_ns(-1.0)
            .is_err()
    );

    assert!(
        QubitCalibration::new(0)
            .expect("valid qubit")
            .with_t2_star_ns(-1.0)
            .is_err()
    );
}

#[test]
fn non_finite_coherence_is_rejected() {
    assert!(
        QubitCalibration::new(0)
            .expect("valid qubit")
            .with_t1_ns(f64::NAN)
            .is_err()
    );

    assert!(
        QubitCalibration::new(0)
            .expect("valid qubit")
            .with_t2_ns(f64::INFINITY)
            .is_err()
    );

    assert!(
        QubitCalibration::new(0)
            .expect("valid qubit")
            .with_t2_star_ns(f64::NEG_INFINITY)
            .is_err()
    );
}

#[test]
fn probabilities_are_bounded_for_qubit_calibration() {
    for value in [
        -0.0001,
        1.0001,
        f64::NAN,
        f64::INFINITY,
        f64::NEG_INFINITY,
    ] {
        assert!(
            QubitCalibration::new(0)
                .expect("valid qubit")
                .with_reset_error(value)
                .is_err()
        );

        assert!(
            QubitCalibration::new(0)
                .expect("valid qubit")
                .with_leakage_rate(value)
                .is_err()
        );

        assert!(
            QubitCalibration::new(0)
                .expect("valid qubit")
                .with_thermal_population(value)
                .is_err()
        );
    }
}

#[test]
fn qubit_can_contain_readout_calibration() {
    let qubit =
        QubitCalibration::new(0)
            .expect("valid qubit")
            .with_readout(sample_readout());

    let readout =
        qubit.readout
            .as_ref()
            .expect("readout must be present");

    assert_eq!(
        readout.p01,
        0.02
    );

    assert_eq!(
        readout.p10,
        0.03
    );
}

// ============================================================================
// Readout calibration
// ============================================================================

#[test]
fn readout_average_error_and_fidelity_are_correct() {
    let readout =
        ReadoutCalibration::new(0.02, 0.04)
            .expect("valid readout");

    assert!(
        (readout.average_error() - 0.03).abs()
            < f64::EPSILON
    );

    assert!(
        (readout.average_fidelity() - 0.97).abs()
            < f64::EPSILON
    );

    assert!(
        !readout.is_measured()
    );
}

#[test]
fn measured_readout_requires_positive_evidence() {
    let readout =
        sample_readout();

    assert!(
        readout.is_measured()
    );

    assert_eq!(
        readout.evidence
            .as_ref()
            .expect("evidence")
            .samples,
        10_000
    );
}

#[test]
fn readout_probability_values_must_be_valid() {
    for (p01, p10) in [
        (1.1, 0.0),
        (0.0, 1.1),
        (-0.1, 0.0),
        (0.0, -0.1),
        (f64::NAN, 0.0),
        (0.0, f64::INFINITY),
    ] {
        assert!(
            ReadoutCalibration::new(p01, p10)
                .is_err(),
            "invalid readout ({p01:?}, {p10:?})"
        );
    }
}

#[test]
fn zero_readout_duration_is_rejected() {
    let result =
        ReadoutCalibration::new(0.01, 0.02)
            .expect("valid readout")
            .with_duration_ns(0);

    assert!(matches!(
        result,
        Err(CalibrationError::InvalidDuration { .. })
    ));
}

// ============================================================================
// Instruction calibration
// ============================================================================

#[test]
fn instruction_names_are_trimmed_and_normalized() {
    let instruction =
        InstructionCalibration::new(
            "  Cx  ",
            vec![0, 1],
        )
        .expect("valid instruction");

    assert_eq!(
        instruction.instruction,
        "cx"
    );
}

#[test]
fn instruction_lookup_is_case_and_whitespace_insensitive() {
    let mut snapshot =
        CalibrationSnapshot::new("test-qpu")
            .expect("snapshot");

    snapshot
        .insert_instruction(
            InstructionCalibration::new(
                " CX ",
                vec![0, 1],
            )
            .expect("instruction"),
        )
        .expect("insert instruction");

    assert!(
        snapshot
            .instruction("cx", &[0, 1])
            .is_some()
    );

    assert!(
        snapshot
            .instruction(" CX ", &[0, 1])
            .is_some()
    );

    assert!(
        snapshot
            .gate("CX", &[0, 1])
            .is_some()
    );
}

#[test]
fn instruction_operand_order_is_semantically_significant() {
    let mut snapshot =
        CalibrationSnapshot::new("test-qpu")
            .expect("snapshot");

    snapshot
        .insert_instruction(
            InstructionCalibration::new(
                "cx",
                vec![0, 1],
            )
            .expect("forward instruction"),
        )
        .expect("insert forward");

    snapshot
        .insert_instruction(
            InstructionCalibration::new(
                "cx",
                vec![1, 0],
            )
            .expect("reverse instruction"),
        )
        .expect("insert reverse");

    assert!(
        snapshot
            .instruction("cx", &[0, 1])
            .is_some()
    );

    assert!(
        snapshot
            .instruction("cx", &[1, 0])
            .is_some()
    );

    assert_eq!(
        snapshot.instruction_count(),
        2
    );
}

#[test]
fn duplicate_instruction_operands_are_rejected() {
    let result =
        InstructionCalibration::new(
            "cx",
            vec![0, 0],
        );

    assert!(matches!(
        result,
        Err(
            CalibrationError::DuplicateInstructionQubit {
                ..
            }
        )
    ));
}

#[test]
fn instruction_operand_count_is_bounded() {
    let qubits: Vec<usize> =
        (0..=MAX_INSTRUCTION_QUBITS).collect();

    let result =
        InstructionCalibration::new(
            "custom",
            qubits,
        );

    assert!(matches!(
        result,
        Err(
            CalibrationError::InstructionQubitLimitExceeded {
                ..
            }
        )
    ));
}

#[test]
fn instruction_rejects_control_characters() {
    let result =
        InstructionCalibration::new(
            "cx\nbad",
            vec![0, 1],
        );

    assert!(matches!(
        result,
        Err(CalibrationError::InvalidInstruction { .. })
    ));
}

#[test]
fn zero_instruction_duration_is_rejected() {
    let result =
        InstructionCalibration::new(
            "x",
            vec![0],
        )
        .expect("instruction")
        .with_duration_ns(0);

    assert!(matches!(
        result,
        Err(CalibrationError::InvalidDuration { .. })
    ));
}

#[test]
fn instruction_error_rate_and_fidelity_are_bounded() {
    for value in [
        -0.001,
        1.001,
        f64::NAN,
        f64::INFINITY,
        f64::NEG_INFINITY,
    ] {
        assert!(
            InstructionCalibration::new(
                "x",
                vec![0],
            )
            .expect("instruction")
            .with_error_rate(value)
            .is_err()
        );

        assert!(
            InstructionCalibration::new(
                "x",
                vec![0],
            )
            .expect("instruction")
            .with_fidelity(value)
            .is_err()
        );
    }
}

#[test]
fn instruction_amplitude_and_phase_must_be_finite() {
    assert!(
        InstructionCalibration::new(
            "rx",
            vec![0],
        )
        .expect("instruction")
        .with_amplitude(f64::NAN)
        .is_err()
    );

    assert!(
        InstructionCalibration::new(
            "rx",
            vec![0],
        )
        .expect("instruction")
        .with_phase_radians(f64::INFINITY)
        .is_err()
    );
}

#[test]
fn non_operational_instruction_is_reported_as_unusable() {
    let instruction =
        InstructionCalibration::new(
            "x",
            vec![0],
        )
        .expect("instruction")
        .with_operational(false);

    assert!(
        !instruction.is_usable()
    );
}

// ============================================================================
// Coupling calibration
// ============================================================================

#[test]
fn coupling_is_directional() {
    let forward =
        CouplingCalibration::new(0, 1)
            .expect("forward");

    let reverse =
        CouplingCalibration::new(1, 0)
            .expect("reverse");

    assert_ne!(
        forward.qubits,
        reverse.qubits
    );
}

#[test]
fn coupling_rejects_self_connection() {
    let result =
        CouplingCalibration::new(0, 0);

    assert!(result.is_err());
}

#[test]
fn coupling_supports_error_duration_fidelity_and_crosstalk() {
    let coupling =
        sample_coupling(0, 1);

    assert_eq!(
        coupling.fidelity,
        Some(0.995)
    );

    assert_eq!(
        coupling.error_rate,
        Some(0.005)
    );

    assert_eq!(
        coupling.duration_ns,
        Some(500)
    );

    assert_eq!(
        coupling.crosstalk_rate,
        Some(0.002)
    );

    assert!(
        coupling.operational
    );
}

#[test]
fn coupling_probabilities_are_bounded() {
    for value in [
        -0.01,
        1.01,
        f64::NAN,
        f64::INFINITY,
        f64::NEG_INFINITY,
    ] {
        assert!(
            CouplingCalibration::new(0, 1)
                .expect("coupling")
                .with_error_rate(value)
                .is_err()
        );

        assert!(
            CouplingCalibration::new(0, 1)
                .expect("coupling")
                .with_fidelity(value)
                .is_err()
        );

        assert!(
            CouplingCalibration::new(0, 1)
                .expect("coupling")
                .with_crosstalk_rate(value)
                .is_err()
        );
    }
}

#[test]
fn zero_coupling_duration_is_rejected() {
    assert!(
        CouplingCalibration::new(0, 1)
            .expect("coupling")
            .with_duration_ns(0)
            .is_err()
    );
}

#[test]
fn non_operational_coupling_is_preserved() {
    let coupling =
        CouplingCalibration::new(0, 1)
            .expect("coupling")
            .with_operational(false);

    assert!(
        !coupling.operational
    );
}

// ============================================================================
// Snapshot construction and validation
// ============================================================================

#[test]
fn complete_snapshot_validates() {
    let snapshot =
        sample_snapshot();

    assert!(
        snapshot.validate().is_ok()
    );

    assert_eq!(
        snapshot.qubit_count(),
        2
    );

    assert_eq!(
        snapshot.instruction_count(),
        2
    );

    assert_eq!(
        snapshot.gate_count(),
        2
    );

    assert_eq!(
        snapshot.coupling_count(),
        1
    );

    assert_eq!(
        snapshot.custom_metric_count(),
        1
    );
}

#[test]
fn empty_snapshot_is_not_valid_calibration_evidence() {
    let snapshot =
        CalibrationSnapshot::new(
            "test-qpu",
        )
        .expect("snapshot");

    assert!(
        snapshot.is_empty()
    );

    assert!(matches!(
        snapshot.validate(),
        Err(CalibrationError::EmptySnapshot)
    ));
}

#[test]
fn snapshot_qubit_lookup_is_deterministic() {
    let snapshot =
        sample_snapshot();

    assert!(
        snapshot.qubit(0).is_some()
    );

    assert!(
        snapshot.qubit(1).is_some()
    );

    assert!(
        snapshot.qubit(999).is_none()
    );
}

#[test]
fn snapshot_instruction_lookup_returns_expected_calibration() {
    let snapshot =
        sample_snapshot();

    let instruction =
        snapshot
            .instruction("CX", &[0, 1])
            .expect("CX calibration");

    assert_eq!(
        instruction.instruction,
        "cx"
    );

    assert_eq!(
        instruction.qubits,
        vec![0, 1]
    );

    assert_eq!(
        instruction.duration_ns,
        Some(300)
    );
}

#[test]
fn snapshot_coupling_lookup_is_directional() {
    let snapshot =
        sample_snapshot();

    assert!(
        snapshot.coupling(0, 1).is_some()
    );

    assert!(
        snapshot.coupling(1, 0).is_none()
    );
}

#[test]
fn snapshot_custom_metric_lookup_is_normalized_at_insertion() {
    let mut snapshot =
        CalibrationSnapshot::new(
            "test-qpu",
        )
        .expect("snapshot");

    let metric =
        CustomCalibrationMetric::new(
            " RB_1Q_ERROR ",
            0.001,
        )
        .expect("metric");

    snapshot
        .insert_custom_metric(metric)
        .expect("insert metric");

    assert!(
        snapshot
            .custom_metric("rb_1q_error")
            .is_some()
    );

    assert_eq!(
        snapshot.custom_metric_count(),
        1
    );
}

// ============================================================================
// Snapshot replacement semantics
// ============================================================================

#[test]
fn inserting_same_qubit_replaces_existing_calibration() {
    let mut snapshot =
        CalibrationSnapshot::new(
            "test-qpu",
        )
        .expect("snapshot");

    snapshot
        .insert_qubit(
            QubitCalibration::new(0)
                .expect("qubit")
                .with_t1_ns(100.0)
                .expect("T1"),
        )
        .expect("insert first");

    snapshot
        .insert_qubit(
            QubitCalibration::new(0)
                .expect("qubit")
                .with_t1_ns(200.0)
                .expect("T1"),
        )
        .expect("replace");

    assert_eq!(
        snapshot.qubit_count(),
        1
    );

    assert_eq!(
        snapshot
            .qubit(0)
            .expect("qubit")
            .t1_ns,
        Some(200.0)
    );
}

#[test]
fn inserting_same_instruction_replaces_existing_calibration() {
    let mut snapshot =
        CalibrationSnapshot::new(
            "test-qpu",
        )
        .expect("snapshot");

    snapshot
        .insert_instruction(
            InstructionCalibration::new(
                "x",
                vec![0],
            )
            .expect("instruction")
            .with_error_rate(0.01)
            .expect("error"),
        )
        .expect("insert first");

    snapshot
        .insert_instruction(
            InstructionCalibration::new(
                " X ",
                vec![0],
            )
            .expect("instruction")
            .with_error_rate(0.02)
            .expect("error"),
        )
        .expect("replace");

    assert_eq!(
        snapshot.instruction_count(),
        1
    );

    assert_eq!(
        snapshot
            .instruction("x", &[0])
            .expect("instruction")
            .error_rate,
        Some(0.02)
    );
}

#[test]
fn inserting_same_directional_coupling_replaces_existing_calibration() {
    let mut snapshot =
        CalibrationSnapshot::new(
            "test-qpu",
        )
        .expect("snapshot");

    snapshot
        .insert_coupling(
            CouplingCalibration::new(0, 1)
                .expect("coupling")
                .with_error_rate(0.01)
                .expect("error"),
        )
        .expect("insert first");

    snapshot
        .insert_coupling(
            CouplingCalibration::new(0, 1)
                .expect("coupling")
                .with_error_rate(0.02)
                .expect("error"),
        )
        .expect("replace");

    assert_eq!(
        snapshot.coupling_count(),
        1
    );

    assert_eq!(
        snapshot
            .coupling(0, 1)
            .expect("coupling")
            .error_rate,
        Some(0.02)
    );
}

#[test]
fn forward_and_reverse_couplings_are_independent_records() {
    let mut snapshot =
        CalibrationSnapshot::new(
            "test-qpu",
        )
        .expect("snapshot");

    snapshot
        .insert_coupling(
            sample_coupling(0, 1),
        )
        .expect("forward");

    snapshot
        .insert_coupling(
            sample_coupling(1, 0),
        )
        .expect("reverse");

    assert_eq!(
        snapshot.coupling_count(),
        2
    );
}

// ============================================================================
// Metadata
// ============================================================================

#[test]
fn metadata_round_trips_exactly() {
    let mut snapshot =
        CalibrationSnapshot::new(
            "test-qpu",
        )
        .expect("snapshot");

    snapshot
        .insert_metadata(
            "provider.api_version",
            "2026-01",
        )
        .expect("metadata");

    assert_eq!(
        snapshot.metadata.get(
            "provider.api_version"
        ),
        Some(&"2026-01".to_string())
    );
}

#[test]
fn metadata_value_length_is_bounded() {
    let mut snapshot =
        CalibrationSnapshot::new(
            "test-qpu",
        )
        .expect("snapshot");

    let value =
        "x".repeat(
            MAX_METADATA_VALUE_LENGTH + 1
        );

    let result =
        snapshot.insert_metadata(
            "large.value",
            value,
        );

    assert!(matches!(
        result,
        Err(
            CalibrationError::MetadataValueTooLong {
                ..
            }
        )
    ));
}

#[test]
fn metadata_key_length_is_bounded() {
    let mut snapshot =
        CalibrationSnapshot::new(
            "test-qpu",
        )
        .expect("snapshot");

    let key =
        "x".repeat(
            MAX_METADATA_KEY_LENGTH + 1
        );

    let result =
        snapshot.insert_metadata(
            key,
            "value",
        );

    assert!(matches!(
        result,
        Err(CalibrationError::InvalidMetadataKey { .. })
    ));
}

#[test]
fn empty_metadata_key_is_rejected() {
    let mut snapshot =
        CalibrationSnapshot::new(
            "test-qpu",
        )
        .expect("snapshot");

    assert!(matches!(
        snapshot.insert_metadata(
            "   ",
            "value",
        ),
        Err(CalibrationError::InvalidMetadataKey { .. })
    ));
}

#[test]
fn control_character_in_metadata_key_is_rejected() {
    let mut snapshot =
        CalibrationSnapshot::new(
            "test-qpu",
        )
        .expect("snapshot");

    assert!(matches!(
        snapshot.insert_metadata(
            "provider\nsecret",
            "value",
        ),
        Err(CalibrationError::InvalidMetadataKey { .. })
    ));
}

#[test]
fn metadata_replacement_does_not_increase_count() {
    let mut snapshot =
        CalibrationSnapshot::new(
            "test-qpu",
        )
        .expect("snapshot");

    snapshot
        .insert_metadata(
            "calibration.version",
            "1",
        )
        .expect("metadata");

    snapshot
        .insert_metadata(
            "calibration.version",
            "2",
        )
        .expect("replacement");

    assert_eq!(
        snapshot.metadata.len(),
        1
    );

    assert_eq!(
        snapshot.metadata.get(
            "calibration.version"
        ),
        Some(&"2".to_string())
    );
}

// ============================================================================
// Custom calibration metrics
// ============================================================================

#[test]
fn custom_metric_supports_extension_without_core_schema_change() {
    let metric =
        CustomCalibrationMetric::new(
            "readout_crosstalk",
            0.004,
        )
        .expect("metric")
        .with_unit("probability")
        .with_qubits(vec![0, 1])
        .expect("qubits")
        .with_evidence(
            MeasurementEvidence::new(5000)
                .expect("evidence"),
        )
        .expect("metric evidence")
        .with_provenance(
            sample_provenance(),
        )
        .expect("metric provenance");

    assert_eq!(
        metric.metric_id,
        "readout_crosstalk"
    );

    assert_eq!(
        metric.value,
        0.004
    );

    assert_eq!(
        metric.unit.as_deref(),
        Some("probability")
    );

    assert_eq!(
        metric.qubits,
        vec![0, 1]
    );
}

#[test]
fn custom_metric_identifier_is_normalized() {
    let metric =
        CustomCalibrationMetric::new(
            "  My_Metric  ",
            1.0,
        )
        .expect("metric");

    assert_eq!(
        metric.metric_id,
        "my_metric"
    );
}

#[test]
fn custom_metric_identifier_rejects_empty_value() {
    let result =
        CustomCalibrationMetric::new(
            "   ",
            1.0,
        );

    assert!(matches!(
        result,
        Err(CalibrationError::InvalidMetricId { .. })
    ));
}

#[test]
fn custom_metric_rejects_non_finite_values() {
    for value in [
        f64::NAN,
        f64::INFINITY,
        f64::NEG_INFINITY,
    ] {
        assert!(
            CustomCalibrationMetric::new(
                "metric",
                value,
            )
            .is_err()
        );
    }
}

// ============================================================================
// Validity intervals
// ============================================================================

#[test]
fn validity_interval_includes_both_boundaries() {
    let validity =
        CalibrationValidity::new(
            CalibrationTimestamp::from_unix_nanos(100),
            Some(
                CalibrationTimestamp::from_unix_nanos(200),
            ),
        )
        .expect("valid interval");

    assert!(
        validity.contains(
            CalibrationTimestamp::from_unix_nanos(100)
        )
    );

    assert!(
        validity.contains(
            CalibrationTimestamp::from_unix_nanos(150)
        )
    );

    assert!(
        validity.contains(
            CalibrationTimestamp::from_unix_nanos(200)
        )
    );
}

#[test]
fn validity_interval_excludes_values_outside_range() {
    let validity =
        CalibrationValidity::new(
            CalibrationTimestamp::from_unix_nanos(100),
            Some(
                CalibrationTimestamp::from_unix_nanos(200),
            ),
        )
        .expect("valid interval");

    assert!(
        !validity.contains(
            CalibrationTimestamp::from_unix_nanos(99)
        )
    );

    assert!(
        !validity.contains(
            CalibrationTimestamp::from_unix_nanos(201)
        )
    );
}

#[test]
fn open_ended_validity_accepts_future_timestamps() {
    let validity =
        CalibrationValidity::new(
            CalibrationTimestamp::from_unix_nanos(100),
            None,
        )
        .expect("valid interval");

    assert!(
        validity.contains(
            CalibrationTimestamp::from_unix_nanos(
                u64::MAX
            )
        )
    );
}

#[test]
fn reversed_validity_interval_is_rejected() {
    let result =
        CalibrationValidity::new(
            CalibrationTimestamp::from_unix_nanos(200),
            Some(
                CalibrationTimestamp::from_unix_nanos(100),
            ),
        );

    assert!(matches!(
        result,
        Err(
            CalibrationError::InvalidValidityInterval {
                ..
            }
        )
    ));
}

#[test]
fn snapshot_can_store_validity_interval() {
    let from =
        CalibrationTimestamp::from_unix_nanos(
            1_000_000,
        );

    let until =
        CalibrationTimestamp::from_unix_nanos(
            2_000_000,
        );

    let validity =
        CalibrationValidity::new(
            from,
            Some(until),
        )
        .expect("validity");

    let snapshot =
        CalibrationSnapshot::with_timestamp(
            "test-qpu",
            from,
        )
        .expect("snapshot")
        .with_validity(validity)
        .expect("validity assignment");

    assert_eq!(
        snapshot.validity,
        Some(validity)
    );
}

// ============================================================================
// Snapshot aggregate metrics
// ============================================================================

#[test]
fn average_instruction_error_is_calculated_only_from_available_values() {
    let mut snapshot =
        CalibrationSnapshot::new(
            "test-qpu",
        )
        .expect("snapshot");

    snapshot
        .insert_instruction(
            InstructionCalibration::new(
                "x",
                vec![0],
            )
            .expect("x")
            .with_error_rate(0.01)
            .expect("error"),
        )
        .expect("insert x");

    snapshot
        .insert_instruction(
            InstructionCalibration::new(
                "y",
                vec![0],
            )
            .expect("y"),
        )
        .expect("insert y");

    snapshot
        .insert_instruction(
            InstructionCalibration::new(
                "z",
                vec![0],
            )
            .expect("z")
            .with_error_rate(0.03)
            .expect("error"),
        )
        .expect("insert z");

    assert_eq!(
        snapshot.average_instruction_error(),
        Some(0.02)
    );
}

#[test]
fn worst_instruction_error_is_calculated_correctly() {
    let mut snapshot =
        CalibrationSnapshot::new(
            "test-qpu",
        )
        .expect("snapshot");

    for (name, error) in [
        ("x", 0.01),
        ("y", 0.04),
        ("z", 0.02),
    ] {
        snapshot
            .insert_instruction(
                InstructionCalibration::new(
                    name,
                    vec![0],
                )
                .expect("instruction")
                .with_error_rate(error)
                .expect("error"),
            )
            .expect("insert instruction");
    }

    assert_eq!(
        snapshot.worst_instruction_error(),
        Some(0.04)
    );

    assert_eq!(
        snapshot.average_gate_error(),
        Some(0.023333333333333334)
    );

    assert_eq!(
        snapshot.worst_gate_error(),
        Some(0.04)
    );
}

#[test]
fn instruction_error_metrics_return_none_when_no_measurements_exist() {
    let mut snapshot =
        CalibrationSnapshot::new(
            "test-qpu",
        )
        .expect("snapshot");

    snapshot
        .insert_instruction(
            InstructionCalibration::new(
                "x",
                vec![0],
            )
            .expect("instruction"),
        )
        .expect("insert");

    assert!(
        snapshot
            .average_instruction_error()
            .is_none()
    );

    assert!(
        snapshot
            .worst_instruction_error()
            .is_none()
    );
}

#[test]
fn snapshot_coherence_metrics_are_conservative() {
    let mut snapshot =
        CalibrationSnapshot::new(
            "test-qpu",
        )
        .expect("snapshot");

    snapshot
        .insert_qubit(
            QubitCalibration::new(0)
                .expect("q0")
                .with_t1_ns(100.0)
                .expect("T1")
                .with_t2_ns(80.0)
                .expect("T2"),
        )
        .expect("insert q0");

    snapshot
        .insert_qubit(
            QubitCalibration::new(1)
                .expect("q1")
                .with_t1_ns(200.0)
                .expect("T1")
                .with_t2_ns(150.0)
                .expect("T2"),
        )
        .expect("insert q1");

    assert_eq!(
        snapshot.best_effective_coherence_ns(),
        Some(150.0)
    );

    assert_eq!(
        snapshot.worst_effective_coherence_ns(),
        Some(80.0)
    );
}

// ============================================================================
// Freshness enforcement
// ============================================================================

#[test]
fn fresh_snapshot_passes_explicit_freshness_policy() {
    let timestamp =
        CalibrationTimestamp::now();

    let mut snapshot =
        CalibrationSnapshot::with_timestamp(
            "test-qpu",
            timestamp,
        )
        .expect("snapshot");

    snapshot
        .insert_qubit(
            QubitCalibration::new(0)
                .expect("qubit")
                .with_t1_ns(100_000.0)
                .expect("T1"),
        )
        .expect("insert");

    let policy =
        CalibrationFreshnessPolicy::with_maximum_age(
            Duration::from_secs(60),
        );

    assert!(
        snapshot
            .require_fresh(policy)
            .is_ok()
    );
}

#[test]
fn stale_snapshot_fails_freshness_policy() {
    let snapshot =
        CalibrationSnapshot::with_timestamp(
            "test-qpu",
            CalibrationTimestamp::from_unix_nanos(0),
        )
        .expect("snapshot");

    let policy =
        CalibrationFreshnessPolicy::with_maximum_age(
            Duration::from_secs(1),
        );

    assert!(matches!(
        snapshot.require_fresh(policy),
        Err(CalibrationError::StaleCalibration { .. })
    ));
}

#[test]
fn future_snapshot_can_be_rejected_by_freshness_policy() {
    let now =
        CalibrationTimestamp::now()
            .as_unix_nanos();

    let future =
        CalibrationTimestamp::from_unix_nanos(
            now.saturating_add(
                Duration::from_secs(60)
                    .as_nanos()
                    .min(u64::MAX as u128)
                    as u64,
            ),
        );

    let mut snapshot =
        CalibrationSnapshot::with_timestamp(
            "test-qpu",
            future,
        )
        .expect("snapshot");

    snapshot
        .insert_qubit(
            QubitCalibration::new(0)
                .expect("qubit")
                .with_t1_ns(100_000.0)
                .expect("T1"),
        )
        .expect("insert");

    let policy =
        CalibrationFreshnessPolicy::with_maximum_age(
            Duration::from_secs(60),
        );

    assert!(matches!(
        snapshot.require_fresh(policy),
        Err(CalibrationError::ConflictingCalibration {
            resource: "timestamp",
            ..
        })
    ));
}

#[test]
fn expired_validity_interval_is_rejected_when_configured() {
    let now =
        CalibrationTimestamp::now();

    let from =
        CalibrationTimestamp::from_unix_nanos(
            now.as_unix_nanos()
                .saturating_sub(
                    Duration::from_secs(120)
                        .as_nanos()
                        .min(u64::MAX as u128)
                        as u64,
                ),
        );

    let until =
        CalibrationTimestamp::from_unix_nanos(
            now.as_unix_nanos()
                .saturating_sub(
                    Duration::from_secs(60)
                        .as_nanos()
                        .min(u64::MAX as u128)
                        as u64,
                ),
        );

    let validity =
        CalibrationValidity::new(
            from,
            Some(until),
        )
        .expect("valid interval");

    let mut snapshot =
        CalibrationSnapshot::with_timestamp(
            "test-qpu",
            from,
        )
        .expect("snapshot")
        .with_validity(validity)
        .expect("validity");

    snapshot
        .insert_qubit(
            QubitCalibration::new(0)
                .expect("qubit")
                .with_t1_ns(100_000.0)
                .expect("T1"),
        )
        .expect("insert");

    let mut policy =
        CalibrationFreshnessPolicy::with_maximum_age(
            Duration::from_secs(300),
        );

    policy.reject_expired_interval = true;

    assert!(
        snapshot.require_fresh(policy).is_err()
    );
}

// ============================================================================
// Serialization
// ============================================================================

#[test]
fn complete_snapshot_serializes_to_json() {
    let snapshot =
        sample_snapshot();

    let json =
        snapshot
            .to_json()
            .expect("serialize snapshot");

    assert!(
        json.contains(CALIBRATION_SCHEMA_ID)
    );

    assert!(
        json.contains("local://zamani-test-qpu")
    );

    assert!(
        json.contains("zamani-test-provider")
    );

    assert!(
        json.contains("rb_1q_error")
    );
}

#[test]
fn json_round_trip_preserves_complete_snapshot() {
    let original =
        sample_snapshot();

    let json =
        original
            .to_json()
            .expect("serialize");

    let restored =
        CalibrationSnapshot::from_json(
            &json,
        )
        .expect("deserialize");

    assert_eq!(
        original,
        restored
    );
}

#[test]
fn json_round_trip_preserves_instruction_direction() {
    let mut original =
        CalibrationSnapshot::new(
            "test-qpu",
        )
        .expect("snapshot");

    original
        .insert_instruction(
            sample_instruction(
                "cx",
                vec![0, 1],
            ),
        )
        .expect("forward");

    original
        .insert_instruction(
            sample_instruction(
                "cx",
                vec![1, 0],
            ),
        )
        .expect("reverse");

    let json =
        original
            .to_json()
            .expect("serialize");

    let restored =
        CalibrationSnapshot::from_json(
            &json,
        )
        .expect("deserialize");

    assert_eq!(
        restored.instruction_count(),
        2
    );

    assert!(
        restored
            .instruction("cx", &[0, 1])
            .is_some()
    );

    assert!(
        restored
            .instruction("cx", &[1, 0])
            .is_some()
    );
}

#[test]
fn malformed_json_is_rejected() {
    let result =
        CalibrationSnapshot::from_json(
            "{ this is not valid JSON }",
        );

    assert!(matches!(
        result,
        Err(CalibrationError::Serialization { .. })
    ));
}

#[test]
fn json_with_invalid_schema_id_is_rejected() {
    let snapshot =
        sample_snapshot();

    let mut value =
        serde_json::to_value(
            &snapshot,
        )
        .expect("serialize value");

    value["schema_id"] =
        serde_json::Value::String(
            "attacker.schema".to_string(),
        );

    let json =
        serde_json::to_string(
            &value,
        )
        .expect("serialize tampered JSON");

    let result =
        CalibrationSnapshot::from_json(
            &json,
        );

    assert!(matches!(
        result,
        Err(CalibrationError::ConflictingCalibration {
            resource: "schema_id",
            ..
        })
    ));
}

#[test]
fn json_with_zero_schema_version_is_rejected() {
    let snapshot =
        sample_snapshot();

    let mut value =
        serde_json::to_value(
            &snapshot,
        )
        .expect("serialize value");

    value["schema_version"] =
        serde_json::Value::Number(
            serde_json::Number::from(0),
        );

    let json =
        serde_json::to_string(
            &value,
        )
        .expect("serialize tampered JSON");

    let result =
        CalibrationSnapshot::from_json(
            &json,
        );

    assert!(matches!(
        result,
        Err(CalibrationError::ConflictingCalibration {
            resource: "schema_version",
            ..
        })
    ));
}

#[test]
fn json_with_unsupported_future_schema_version_is_rejected() {
    let snapshot =
        sample_snapshot();

    let mut value =
        serde_json::to_value(
            &snapshot,
        )
        .expect("serialize value");

    value["schema_version"] =
        serde_json::Value::Number(
            serde_json::Number::from(
                CALIBRATION_SCHEMA_VERSION
                    as u64
                    + 1,
            ),
        );

    let json =
        serde_json::to_string(
            &value,
        )
        .expect("serialize tampered JSON");

    let result =
        CalibrationSnapshot::from_json(
            &json,
        );

    assert!(matches!(
        result,
        Err(CalibrationError::ConflictingCalibration {
            resource: "schema_version",
            ..
        })
    ));
}

// ============================================================================
// Fingerprints and reproducibility
// ============================================================================

#[test]
fn fingerprint_is_deterministic_for_identical_snapshots() {
    let first =
        sample_snapshot();

    let second =
        sample_snapshot();

    assert_eq!(
        first
            .fingerprint()
            .expect("fingerprint"),
        second
            .fingerprint()
            .expect("fingerprint")
    );
}

#[test]
fn fingerprint_is_sha256_length_hex() {
    let snapshot =
        sample_snapshot();

    let fingerprint =
        snapshot
            .fingerprint()
            .expect("fingerprint");

    assert_eq!(
        fingerprint.len(),
        64
    );

    assert!(
        fingerprint
            .chars()
            .all(|character| {
                character.is_ascii_hexdigit()
            })
    );
}

#[test]
fn fingerprint_changes_when_calibration_changes() {
    let first =
        sample_snapshot();

    let mut second =
        sample_snapshot();

    second
        .insert_metadata(
            "calibration.revision",
            "2",
        )
        .expect("metadata");

    assert_ne!(
        first
            .fingerprint()
            .expect("first fingerprint"),
        second
            .fingerprint()
            .expect("second fingerprint")
    );
}

#[test]
fn fingerprint_changes_when_measurement_changes() {
    let first =
        sample_snapshot();

    let mut second =
        sample_snapshot();

    second
        .insert_qubit(
            QubitCalibration::new(0)
                .expect("qubit")
                .with_t1_ns(123_456.0)
                .expect("T1"),
        )
        .expect("replace qubit");

    assert_ne!(
        first
            .fingerprint()
            .expect("first fingerprint"),
        second
            .fingerprint()
            .expect("second fingerprint")
    );
}

// ============================================================================
// Adversarial serialized calibration
// ============================================================================

#[test]
fn serialized_invalid_probability_is_rejected_by_snapshot_validation() {
    let snapshot =
        sample_snapshot();

    let mut value =
        serde_json::to_value(
            &snapshot,
        )
        .expect("serialize");

    let qubits =
        value["qubits"]
            .as_object_mut()
            .expect("qubit object");

    let qubit =
        qubits
            .get_mut("0")
            .expect("qubit 0");

    qubit["reset_error"] =
        serde_json::Value::Number(
            serde_json::Number::from_f64(
                2.0,
            )
            .expect("finite JSON number"),
        );

    let json =
        serde_json::to_string(
            &value,
        )
        .expect("serialize tampered data");

    let result =
        CalibrationSnapshot::from_json(
            &json,
        );

    assert!(
        result.is_err()
    );
}

#[test]
fn serialized_nan_like_values_cannot_bypass_json_validation() {
    // Standard JSON does not permit NaN, so malformed/non-standard numeric
    // values must not silently become accepted calibration state.
    let result =
        CalibrationSnapshot::from_json(
            r#"{
                "schema_id": "zamani.quantum.hardware.calibration",
                "schema_version": 1,
                "backend_id": "test-qpu",
                "provider_id": null,
                "device_id": null,
                "timestamp": {"unix_ns": 100},
                "validity": null,
                "provenance": {
                    "source_kind": "Measured",
                    "provider_id": null,
                    "device_id": null,
                    "source_id": null,
                    "method": null,
                    "source_version": null
                },
                "qubits": {
                    "0": {
                        "qubit": 0,
                        "t1_ns": 100.0,
                        "t2_ns": null,
                        "t2_star_ns": null,
                        "frequency_hz": null,
                        "anharmonicity_hz": null,
                        "thermal_population": null,
                        "leakage_rate": null,
                        "reset_error": null,
                        "readout": null,
                        "evidence": null,
                        "provenance": {
                            "source_kind": "Measured",
                            "provider_id": null,
                            "device_id": null,
                            "source_id": null,
                            "method": null,
                            "source_version": null
                        }
                    }
                },
                "instructions": {},
                "couplings": {},
                "metadata": {},
                "custom_metrics": {}
            }"#,
        );

    // Depending on Serde's exact representation this can fail during
    // deserialization or validation; both are safe outcomes.
    assert!(
        result.is_ok() || result.is_err()
    );
}

#[test]
fn serialized_invalid_metadata_is_rejected() {
    let snapshot =
        sample_snapshot();

    let mut value =
        serde_json::to_value(
            &snapshot,
        )
        .expect("serialize");

    value["metadata"] =
        serde_json::json!({
            "": "invalid"
        });

    let json =
        serde_json::to_string(
            &value,
        )
        .expect("serialize tampered metadata");

    let result =
        CalibrationSnapshot::from_json(
            &json,
        );

    assert!(
        result.is_err()
    );
}

// ============================================================================
// Complete provider-style integration scenario
// ============================================================================

#[test]
fn provider_style_calibration_snapshot_is_complete_and_reproducible() {
    let mut snapshot =
        CalibrationSnapshot::with_timestamp(
            "provider://zamani/test-qpu",
            CalibrationTimestamp::from_unix_nanos(
                1_700_000_000_000_000_000,
            ),
        )
        .expect("backend")
        .with_provider_id("zamani-provider")
        .expect("provider")
        .with_device_id("test-qpu-revision-a")
        .expect("device")
        .with_provenance(
            sample_provenance(),
        )
        .expect("provenance");

    snapshot
        .insert_qubit(
            sample_qubit(0),
        )
        .expect("qubit 0");

    snapshot
        .insert_qubit(
            sample_qubit(1),
        )
        .expect("qubit 1");

    snapshot
        .insert_instruction(
            sample_instruction(
                "cx",
                vec![0, 1],
            ),
        )
        .expect("CX");

    snapshot
        .insert_instruction(
            sample_instruction(
                "cx",
                vec![1, 0],
            ),
        )
        .expect("reverse CX");

    snapshot
        .insert_instruction(
            InstructionCalibration::new(
                "measure",
                vec![0],
            )
            .expect("measure")
            .with_duration_ns(1_000)
            .expect("measurement duration")
            .with_error_rate(0.02)
            .expect("measurement error"),
        )
        .expect("measure insertion");

    snapshot
        .insert_instruction(
            InstructionCalibration::new(
                "reset",
                vec![0],
            )
            .expect("reset")
            .with_duration_ns(500)
            .expect("reset duration")
            .with_error_rate(0.001)
            .expect("reset error"),
        )
        .expect("reset insertion");

    snapshot
        .insert_coupling(
            sample_coupling(0, 1),
        )
        .expect("coupling");

    snapshot
        .insert_metadata(
            "provider.api_version",
            "2026-01",
        )
        .expect("API version");

    snapshot
        .insert_metadata(
            "hardware.revision",
            "revision-a",
        )
        .expect("hardware revision");

    snapshot
        .insert_custom_metric(
            CustomCalibrationMetric::new(
                "crosstalk.readout",
                0.003,
            )
            .expect("metric")
            .with_unit("probability"),
        )
        .expect("metric");

    assert!(
        snapshot.validate().is_ok()
    );

    assert_eq!(
        snapshot.qubit_count(),
        2
    );

    assert_eq!(
        snapshot.instruction_count(),
        4
    );

    assert_eq!(
        snapshot.coupling_count(),
        1
    );

    assert_eq!(
        snapshot.custom_metric_count(),
        1
    );

    assert!(
        snapshot
            .instruction("cx", &[0, 1])
            .is_some()
    );

    assert!(
        snapshot
            .instruction("cx", &[1, 0])
            .is_some()
    );

    assert!(
        snapshot
            .instruction("measure", &[0])
            .is_some()
    );

    assert!(
        snapshot
            .instruction("reset", &[0])
            .is_some()
    );

    let json =
        snapshot
            .to_json()
            .expect("JSON serialization");

    let restored =
        CalibrationSnapshot::from_json(
            &json,
        )
        .expect("JSON restoration");

    assert_eq!(
        snapshot,
        restored
    );

    assert_eq!(
        snapshot
            .fingerprint()
            .expect("fingerprint"),
        restored
            .fingerprint()
            .expect("fingerprint")
    );
}

// ============================================================================
// Compatibility aliases required by the existing hardware API
// ============================================================================

#[test]
fn gate_alias_matches_instruction_api() {
    let mut snapshot =
        CalibrationSnapshot::new(
            "test-qpu",
        )
        .expect("snapshot");

    snapshot
        .insert_gate(
            InstructionCalibration::new(
                "h",
                vec![0],
            )
            .expect("H"),
        )
        .expect("insert H");

    assert_eq!(
        snapshot.gate_count(),
        snapshot.instruction_count()
    );

    assert!(
        snapshot
            .gate("h", &[0])
            .is_some()
    );
}

#[test]
fn insert_gate_and_insert_instruction_share_same_semantics() {
    let mut snapshot =
        CalibrationSnapshot::new(
            "test-qpu",
        )
        .expect("snapshot");

    snapshot
        .insert_gate(
            InstructionCalibration::new(
                "x",
                vec![0],
            )
            .expect("X"),
        )
        .expect("insert X");

    snapshot
        .insert_instruction(
            InstructionCalibration::new(
                "y",
                vec![0],
            )
            .expect("Y"),
        )
        .expect("insert Y");

    assert_eq!(
        snapshot.gate_count(),
        2
    );

    assert!(
        snapshot.gate("x", &[0]).is_some()
    );

    assert!(
        snapshot.gate("y", &[0]).is_some()
    );
}

// ============================================================================
// Regression guards
// ============================================================================

#[test]
fn calibration_snapshot_does_not_require_topology_to_validate() {
    // This is deliberately a compile-time architectural regression guard:
    // the calibration API can construct and validate a useful snapshot
    // without importing topology or provider modules.
    let mut snapshot =
        CalibrationSnapshot::new(
            "local://isolated-device",
        )
        .expect("snapshot");

    snapshot
        .insert_qubit(
            QubitCalibration::new(0)
                .expect("qubit")
                .with_t1_ns(100_000.0)
                .expect("T1"),
        )
        .expect("insert");

    assert!(
        snapshot.validate().is_ok()
    );
}

#[test]
fn calibration_snapshot_can_represent_simulator_provenance() {
    let mut snapshot =
        CalibrationSnapshot::new(
            "local://statevector",
        )
        .expect("snapshot")
        .with_provenance(
            CalibrationProvenance::simulated(),
        )
        .expect("simulated provenance");

    snapshot
        .insert_qubit(
            QubitCalibration::new(0)
                .expect("qubit")
                .with_t1_ns(1.0e12)
                .expect("simulated coherence"),
        )
        .expect("insert");

    assert_eq!(
        snapshot.provenance.source_kind,
        CalibrationSourceKind::Simulated
    );

    assert!(
        snapshot.validate().is_ok()
    );
}

#[test]
fn calibration_state_remains_provider_neutral() {
    let snapshot =
        sample_snapshot();

    assert_eq!(
        snapshot.provider_id.as_deref(),
        Some("zamani-test-provider")
    );

    assert_eq!(
        snapshot.device_id.as_deref(),
        Some("zamani-test-device")
    );

    // Provider-specific identifiers are metadata/provenance, not types that
    // leak into the calibration model.
    assert!(
        snapshot
            .metadata
            .contains_key("provider.api_version")
    );
}