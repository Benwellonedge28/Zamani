//! Zamani Quantum Memory — Measurement Integration Tests
//!
//! Production contract tests for:
//!
//! `crate::quantum::memory::measurement`
//!
//! These tests deliberately exercise the PUBLIC measurement boundary rather
//! than representation-specific internals.
//!
//! The suite verifies:
//!
//! - logical/physical/classical identity separation;
//! - request construction and validation;
//! - duplicate-qubit rejection;
//! - physical mapping validation;
//! - classical destination validation;
//! - measurement basis validation;
//! - arbitrary projective measurement validation;
//! - provider-defined measurement extensibility;
//! - destructive measurement;
//! - mid-circuit measurement;
//! - non-destructive measurement capability negotiation;
//! - collapse capability negotiation;
//! - raw-shot capability negotiation;
//! - histogram capability negotiation;
//! - probability derivation;
//! - deterministic simulator sampling;
//! - invalid RNG rejection;
//! - histogram invariants;
//! - probability invariants;
//! - expectation values;
//! - parity expectation;
//! - provider execution validation;
//! - raw-shot normalization;
//! - QPU/provider metadata safety;
//! - readout calibration validation;
//! - schema validation;
//! - classical-result mapping;
//! - provider-neutral execution;
//! - provider-controlled randomness capability;
//! - distributed-measurement capability;
//! - hardware-independent behavior.
//!
//! IMPORTANT:
//!
//! This file does NOT assume that a QPU exposes amplitudes, a state vector,
//! density matrix, deterministic randomness, arbitrary projective measurement,
//! raw shots, or non-destructive measurement.
//!
//! Those capabilities must be explicitly negotiated through
//! `MeasurementCapabilities`.
//!
//! Rust target:
//! - Rust 1.97
//! - Rust 1.97.1
//! - Rust 2021
//!
//! Unsafe Rust is forbidden.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::collections::BTreeMap;
use std::num::NonZeroU64;

use crate::quantum::ir::{
    ClassicalBitId,
    PhysicalQubitId,
    QubitId,
};

use crate::quantum::memory::measurement::{
    sample_distribution,
    BlochAxis,
    CollapsePolicy,
    ClassicalDestination,
    DeterministicMeasurementRng,
    FinalMeasurementResult,
    MeasurementBasis,
    MeasurementBit,
    MeasurementBitString,
    MeasurementCapabilities,
    MeasurementDistribution,
    MeasurementExecution,
    MeasurementHistogram,
    MeasurementMode,
    MeasurementObservable,
    MeasurementProvider,
    MeasurementProviderMetadata,
    MeasurementRequest,
    MeasurementResult,
    MeasurementShot,
    ReadoutMetadata,
    MEASUREMENT_SCHEMA_ID,
    MEASUREMENT_SCHEMA_VERSION,
};

use crate::quantum::memory::measurement::MAX_MEASUREMENT_QUBITS;

// =============================================================================
// Test helpers
// =============================================================================

fn nz(value: u64) -> NonZeroU64 {
    NonZeroU64::new(value).expect("test value must be non-zero")
}

fn q(index: usize) -> QubitId {
    QubitId::new(index)
}

fn p(index: usize) -> PhysicalQubitId {
    PhysicalQubitId::new(index)
}

fn c(index: usize) -> ClassicalBitId {
    ClassicalBitId::new(index)
}

fn zero(width: usize) -> MeasurementBitString {
    MeasurementBitString::zeros(width).expect("valid zero bit string")
}

fn bits(values: &[u8]) -> MeasurementBitString {
    MeasurementBitString::new(values.to_vec()).expect("valid measurement bits")
}

fn full_standard_capabilities() -> MeasurementCapabilities {
    MeasurementCapabilities::Z_BASIS
        | MeasurementCapabilities::X_BASIS
        | MeasurementCapabilities::Y_BASIS
        | MeasurementCapabilities::ARBITRARY_PROJECTIVE
        | MeasurementCapabilities::PROVIDER_DEFINED
        | MeasurementCapabilities::MULTI_QUBIT
        | MeasurementCapabilities::MID_CIRCUIT
        | MeasurementCapabilities::NON_DESTRUCTIVE
        | MeasurementCapabilities::COLLAPSE
        | MeasurementCapabilities::CLASSICAL_DESTINATION
        | MeasurementCapabilities::RAW_SHOTS
        | MeasurementCapabilities::HISTOGRAM
        | MeasurementCapabilities::PROBABILITIES
        | MeasurementCapabilities::DETERMINISTIC_SAMPLING
        | MeasurementCapabilities::PROVIDER_RANDOMNESS
        | MeasurementCapabilities::READOUT_CALIBRATION
        | MeasurementCapabilities::READOUT_MITIGATION
        | MeasurementCapabilities::CLASSICAL_FEEDBACK
        | MeasurementCapabilities::DISTRIBUTED
}

// =============================================================================
// Provider test doubles
// =============================================================================

/// Provider that returns a deterministic all-zero result.
///
/// This deliberately behaves like a provider boundary rather than a state
/// vector. It demonstrates that the measurement contract can be tested
/// without assuming a particular state representation.
#[derive(Debug, Clone)]
struct ZeroResultProvider {
    capabilities: MeasurementCapabilities,
    provider_name: Option<&'static str>,
}

impl ZeroResultProvider {
    fn new(capabilities: MeasurementCapabilities) -> Self {
        Self {
            capabilities,
            provider_name: Some("test-provider"),
        }
    }

    fn with_name(
        capabilities: MeasurementCapabilities,
        provider_name: &'static str,
    ) -> Self {
        Self {
            capabilities,
            provider_name: Some(provider_name),
        }
    }
}

impl MeasurementProvider for ZeroResultProvider {
    fn capabilities(&self) -> MeasurementCapabilities {
        self.capabilities
    }

    fn provider_name(&self) -> Option<&str> {
        self.provider_name
    }

    fn measure(
        &mut self,
        request: &MeasurementRequest,
    ) -> MeasurementResult<MeasurementExecution> {
        self.supports_request(request)?;

        let mut histogram = MeasurementHistogram::new();

        histogram.record_count(
            zero(request.qubit_count()),
            request.shot_count(),
        )?;

        let mut execution = MeasurementExecution::empty();

        execution.histogram = Some(histogram);
        execution.state_collapsed =
            matches!(request.collapse, CollapsePolicy::Collapse);
        execution.destructive =
            request.mode == MeasurementMode::Destructive;

        Ok(execution)
    }
}

/// Provider that returns raw shots only.
#[derive(Debug, Clone)]
struct RawShotProvider {
    capabilities: MeasurementCapabilities,
}

impl RawShotProvider {
    fn new() -> Self {
        Self {
            capabilities: MeasurementCapabilities::Z_BASIS
                | MeasurementCapabilities::RAW_SHOTS
                | MeasurementCapabilities::COLLAPSE,
        }
    }
}

impl MeasurementProvider for RawShotProvider {
    fn capabilities(&self) -> MeasurementCapabilities {
        self.capabilities
    }

    fn measure(
        &mut self,
        request: &MeasurementRequest,
    ) -> MeasurementResult<MeasurementExecution> {
        self.supports_request(request)?;

        let mut shots = Vec::new();

        for _ in 0..request.shot_count() {
            shots.push(MeasurementShot::new(
                zero(request.qubit_count()),
                Vec::new(),
                true,
            )?);
        }

        let mut execution = MeasurementExecution::empty();
        execution.shots = Some(shots);
        execution.state_collapsed = true;
        execution.destructive = true;

        Ok(execution)
    }
}

/// Provider that reports physical-QPU style provider-controlled randomness.
///
/// It intentionally does not advertise deterministic simulator sampling.
#[derive(Debug, Clone)]
struct PhysicalQpuProvider {
    capabilities: MeasurementCapabilities,
}

impl PhysicalQpuProvider {
    fn new() -> Self {
        Self {
            capabilities: MeasurementCapabilities::Z_BASIS
                | MeasurementCapabilities::MULTI_QUBIT
                | MeasurementCapabilities::MID_CIRCUIT
                | MeasurementCapabilities::COLLAPSE
                | MeasurementCapabilities::CLASSICAL_DESTINATION
                | MeasurementCapabilities::HISTOGRAM
                | MeasurementCapabilities::PROBABILITIES
                | MeasurementCapabilities::PROVIDER_RANDOMNESS
                | MeasurementCapabilities::READOUT_CALIBRATION,
        }
    }
}

impl MeasurementProvider for PhysicalQpuProvider {
    fn capabilities(&self) -> MeasurementCapabilities {
        self.capabilities
    }

    fn provider_name(&self) -> Option<&str> {
        Some("physical-qpu-test-double")
    }

    fn measure(
        &mut self,
        request: &MeasurementRequest,
    ) -> MeasurementResult<MeasurementExecution> {
        self.supports_request(request)?;

        let mut histogram = MeasurementHistogram::new();

        histogram.record_count(
            zero(request.qubit_count()),
            request.shot_count(),
        )?;

        let mut execution = MeasurementExecution::empty();

        execution.histogram = Some(histogram);
        execution.state_collapsed = true;
        execution.destructive =
            request.mode == MeasurementMode::Destructive;

        execution.provider_execution_id =
            Some("opaque-execution-id".to_owned());

        Ok(execution)
    }
}

/// RNG which deliberately violates the measurement RNG contract.
struct InvalidRng;

impl crate::quantum::memory::measurement::MeasurementRandomSource
    for InvalidRng
{
    fn next_u64(&mut self) -> u64 {
        u64::MAX
    }

    fn next_unit_f64(&mut self) -> f64 {
        f64::NAN
    }
}

// =============================================================================
// Identity and request construction
// =============================================================================

#[test]
fn measurement_uses_canonical_ir_identities() {
    let logical = q(7);
    let physical = p(11);
    let classical = c(13);

    let destination = ClassicalDestination::new(logical, classical);

    assert_eq!(logical.index(), 7);
    assert_eq!(physical.index(), 11);
    assert_eq!(classical.index(), 13);
    assert_eq!(destination.qubit, logical);
    assert_eq!(destination.classical_bit, classical);
}

#[test]
fn standard_measurement_request_defaults_to_z_basis() {
    let request =
        MeasurementRequest::new(vec![q(0)], nz(100))
            .expect("valid request");

    assert_eq!(request.qubit_count(), 1);
    assert_eq!(request.shot_count(), 100);
    assert!(!request.requires_deterministic_sampling());
    assert_eq!(
        request.observable,
        MeasurementObservable::Standard(MeasurementBasis::Z)
    );
    assert_eq!(request.mode, MeasurementMode::Destructive);
    assert_eq!(request.collapse, CollapsePolicy::Collapse);
    assert!(request.retain_counts);
    assert!(request.calculate_probabilities);
}

#[test]
fn empty_measurement_request_is_rejected() {
    let result = MeasurementRequest::new(Vec::new(), nz(1));

    assert!(result.is_err());
}

#[test]
fn duplicate_logical_qubits_are_rejected() {
    let result =
        MeasurementRequest::new(vec![q(0), q(0)], nz(1));

    assert!(result.is_err());
}

#[test]
fn physical_mapping_must_match_logical_width() {
    let mut request =
        MeasurementRequest::new(vec![q(0), q(1)], nz(1))
            .expect("valid request");

    request.physical_qubits = vec![p(0)];

    assert!(request.validate().is_err());
}

#[test]
fn physical_mapping_is_allowed_when_width_matches() {
    let mut request =
        MeasurementRequest::new(vec![q(0), q(1)], nz(1))
            .expect("valid request");

    request.physical_qubits = vec![p(3), p(7)];

    assert!(request.validate().is_ok());
}

#[test]
fn classical_destination_must_reference_measured_qubit() {
    let mut request =
        MeasurementRequest::new(vec![q(0)], nz(1))
            .expect("valid request");

    request.classical_destinations = vec![
        ClassicalDestination::new(q(1), c(0)),
    ];

    assert!(request.validate().is_err());
}

#[test]
fn classical_destination_cannot_be_duplicated() {
    let mut request =
        MeasurementRequest::new(vec![q(0), q(1)], nz(1))
            .expect("valid request");

    request.classical_destinations = vec![
        ClassicalDestination::new(q(0), c(0)),
        ClassicalDestination::new(q(1), c(0)),
    ];

    assert!(request.validate().is_err());
}

#[test]
fn valid_classical_destinations_are_accepted() {
    let mut request =
        MeasurementRequest::new(vec![q(0), q(1)], nz(4))
            .expect("valid request");

    request.classical_destinations = vec![
        ClassicalDestination::new(q(0), c(0)),
        ClassicalDestination::new(q(1), c(1)),
    ];

    assert!(request.validate().is_ok());
}

// =============================================================================
// Measurement basis validation
// =============================================================================

#[test]
fn all_standard_pauli_bases_are_available() {
    assert_eq!(MeasurementBasis::Z.as_str(), "z");
    assert_eq!(MeasurementBasis::X.as_str(), "x");
    assert_eq!(MeasurementBasis::Y.as_str(), "y");

    assert_eq!(
        MeasurementBasis::Z.axis().norm_squared(),
        1.0
    );
    assert_eq!(
        MeasurementBasis::X.axis().norm_squared(),
        1.0
    );
    assert_eq!(
        MeasurementBasis::Y.axis().norm_squared(),
        1.0
    );
}

#[test]
fn arbitrary_bloch_axis_requires_normalization() {
    assert!(BlochAxis::new(1.0, 0.0, 0.0).is_ok());
    assert!(BlochAxis::new(0.0, 1.0, 0.0).is_ok());
    assert!(BlochAxis::new(0.0, 0.0, 1.0).is_ok());

    assert!(BlochAxis::new(2.0, 0.0, 0.0).is_err());
    assert!(BlochAxis::new(f64::NAN, 0.0, 0.0).is_err());
    assert!(BlochAxis::new(f64::INFINITY, 0.0, 0.0).is_err());
}

#[test]
fn provider_defined_measurement_accepts_finite_parameters() {
    let observable = MeasurementObservable::ProviderDefined {
        name: "native_readout".to_owned(),
        parameters: vec![0.25, 0.5, 0.75],
    };

    assert!(observable.validate().is_ok());
}

#[test]
fn provider_defined_measurement_rejects_non_finite_parameters() {
    let observable = MeasurementObservable::ProviderDefined {
        name: "native_readout".to_owned(),
        parameters: vec![f64::NAN],
    };

    assert!(observable.validate().is_err());
}

#[test]
fn provider_defined_measurement_rejects_invalid_names() {
    let empty = MeasurementObservable::ProviderDefined {
        name: String::new(),
        parameters: Vec::new(),
    };

    let whitespace = MeasurementObservable::ProviderDefined {
        name: " native ".to_owned(),
        parameters: Vec::new(),
    };

    assert!(empty.validate().is_err());
    assert!(whitespace.validate().is_err());
}

// =============================================================================
// Measurement bit and bit-string contract
// =============================================================================

#[test]
fn measurement_bit_accepts_only_zero_and_one() {
    assert_eq!(
        MeasurementBit::from_u8(0).expect("zero"),
        MeasurementBit::Zero
    );

    assert_eq!(
        MeasurementBit::from_u8(1).expect("one"),
        MeasurementBit::One
    );

    assert!(MeasurementBit::from_u8(2).is_err());
    assert!(MeasurementBit::from_u8(u8::MAX).is_err());
}

#[test]
fn measurement_bit_string_rejects_invalid_values() {
    assert!(MeasurementBitString::new(vec![0, 1, 0, 1]).is_ok());
    assert!(MeasurementBitString::new(vec![0, 1, 2]).is_err());
}

#[test]
fn measurement_bit_string_preserves_result_order() {
    let result = bits(&[1, 0, 1, 1]);

    assert_eq!(result.len(), 4);
    assert_eq!(result.as_bits(), &[1, 0, 1, 1]);
    assert_eq!(result.get(0), Some(MeasurementBit::One));
    assert_eq!(result.get(1), Some(MeasurementBit::Zero));
    assert_eq!(result.get(2), Some(MeasurementBit::One));
    assert_eq!(result.get(3), Some(MeasurementBit::One));
}

#[test]
fn zero_bit_string_has_requested_width() {
    let result = zero(32);

    assert_eq!(result.len(), 32);
    assert!(result.as_bits().iter().all(|bit| *bit == 0));
}

#[test]
fn measurement_bit_string_hex_conversion_is_deterministic() {
    let result = bits(&[1, 0, 1, 1]);

    assert_eq!(result.to_hex_string(), "b");
}

// =============================================================================
// Histogram and probability invariants
// =============================================================================

#[test]
fn histogram_preserves_counts_and_total_shots() {
    let mut histogram = MeasurementHistogram::new();

    histogram
        .record_count(bits(&[0]), 60)
        .expect("record zero");

    histogram
        .record_count(bits(&[1]), 40)
        .expect("record one");

    assert_eq!(histogram.count(&bits(&[0])), 60);
    assert_eq!(histogram.count(&bits(&[1])), 40);
    assert_eq!(histogram.total_shots(), 100);
    assert_eq!(histogram.outcome_count(), 2);

    histogram.validate().expect("valid histogram");
}

#[test]
fn histogram_probabilities_are_normalized() {
    let mut histogram = MeasurementHistogram::new();

    histogram
        .record_count(bits(&[0]), 25)
        .expect("record zero");

    histogram
        .record_count(bits(&[1]), 75)
        .expect("record one");

    let distribution =
        histogram.probabilities().expect("valid distribution");

    assert!((distribution.probability(&bits(&[0])) - 0.25).abs() < 1e-12);
    assert!((distribution.probability(&bits(&[1])) - 0.75).abs() < 1e-12);
}

#[test]
fn zero_shot_histogram_cannot_be_converted_to_probabilities() {
    let histogram = MeasurementHistogram::new();

    assert!(histogram.probabilities().is_err());
}

#[test]
fn probability_distribution_rejects_negative_probability() {
    let mut probabilities = BTreeMap::new();

    probabilities.insert(bits(&[0]), -0.1);
    probabilities.insert(bits(&[1]), 1.1);

    assert!(MeasurementDistribution::new(probabilities).is_err());
}

#[test]
fn probability_distribution_must_sum_to_one() {
    let mut probabilities = BTreeMap::new();

    probabilities.insert(bits(&[0]), 0.4);
    probabilities.insert(bits(&[1]), 0.4);

    assert!(MeasurementDistribution::new(probabilities).is_err());
}

#[test]
fn valid_probability_distribution_is_accepted() {
    let mut probabilities = BTreeMap::new();

    probabilities.insert(bits(&[0]), 0.5);
    probabilities.insert(bits(&[1]), 0.5);

    let distribution =
        MeasurementDistribution::new(probabilities)
            .expect("valid distribution");

    assert_eq!(distribution.len(), 2);
    assert!(!distribution.is_empty());
}

// =============================================================================
// Deterministic sampling
// =============================================================================

#[test]
fn deterministic_sampling_is_reproducible() {
    let mut probabilities = BTreeMap::new();

    probabilities.insert(bits(&[0]), 0.25);
    probabilities.insert(bits(&[1]), 0.75);

    let distribution =
        MeasurementDistribution::new(probabilities)
            .expect("valid distribution");

    let mut rng_a = DeterministicMeasurementRng::new(42);
    let mut rng_b = DeterministicMeasurementRng::new(42);

    let result_a = sample_distribution(
        &distribution,
        nz(1_000),
        &mut rng_a,
    )
    .expect("sampling succeeds");

    let result_b = sample_distribution(
        &distribution,
        nz(1_000),
        &mut rng_b,
    )
    .expect("sampling succeeds");

    assert_eq!(result_a, result_b);
    assert_eq!(result_a.total_shots(), 1_000);
}

#[test]
fn deterministic_rng_seed_zero_is_still_deterministic() {
    let mut rng_a = DeterministicMeasurementRng::new(0);
    let mut rng_b = DeterministicMeasurementRng::new(0);

    for _ in 0..100 {
        assert_eq!(
            crate::quantum::memory::measurement::MeasurementRandomSource::next_u64(
                &mut rng_a
            ),
            crate::quantum::memory::measurement::MeasurementRandomSource::next_u64(
                &mut rng_b
            )
        );
    }
}

#[test]
fn invalid_rng_output_is_rejected() {
    let mut probabilities = BTreeMap::new();

    probabilities.insert(bits(&[0]), 1.0);

    let distribution =
        MeasurementDistribution::new(probabilities)
            .expect("valid distribution");

    let mut rng = InvalidRng;

    assert!(
        sample_distribution(
            &distribution,
            nz(1),
            &mut rng,
        )
        .is_err()
    );
}

// =============================================================================
// Final result and expectation values
// =============================================================================

#[test]
fn final_result_can_be_constructed_from_histogram_execution() {
    let request =
        MeasurementRequest::new(vec![q(0)], nz(100))
            .expect("valid request");

    let mut histogram = MeasurementHistogram::new();

    histogram
        .record_count(bits(&[0]), 60)
        .expect("record zero");

    histogram
        .record_count(bits(&[1]), 40)
        .expect("record one");

    let mut execution = MeasurementExecution::empty();
    execution.histogram = Some(histogram);
    execution.state_collapsed = true;
    execution.destructive = true;

    let result =
        FinalMeasurementResult::from_execution(
            request,
            execution,
        )
        .expect("valid final result");

    assert_eq!(result.shots(), 100);
    assert_eq!(result.count(&bits(&[0])), 60);
    assert_eq!(result.count(&bits(&[1])), 40);
    assert!((result.probability(&bits(&[0])) - 0.60).abs() < 1e-12);
    assert!((result.probability(&bits(&[1])) - 0.40).abs() < 1e-12);
}

#[test]
fn single_qubit_expectation_is_correct() {
    let request =
        MeasurementRequest::new(vec![q(0)], nz(100))
            .expect("valid request");

    let mut histogram = MeasurementHistogram::new();

    histogram
        .record_count(bits(&[0]), 75)
        .expect("record zero");

    histogram
        .record_count(bits(&[1]), 25)
        .expect("record one");

    let mut execution = MeasurementExecution::empty();
    execution.histogram = Some(histogram);

    let result =
        FinalMeasurementResult::from_execution(
            request,
            execution,
        )
        .expect("valid result");

    let expectation = result
        .single_qubit_expectation(0)
        .expect("expectation");

    assert!((expectation - 0.5).abs() < 1e-12);
}

#[test]
fn parity_expectation_is_correct() {
    let request =
        MeasurementRequest::new(
            vec![q(0), q(1)],
            nz(100),
        )
        .expect("valid request");

    let mut histogram = MeasurementHistogram::new();

    histogram
        .record_count(bits(&[0, 0]), 50)
        .expect("record 00");

    histogram
        .record_count(bits(&[0, 1]), 25)
        .expect("record 01");

    histogram
        .record_count(bits(&[1, 0]), 25)
        .expect("record 10");

    let mut execution = MeasurementExecution::empty();
    execution.histogram = Some(histogram);

    let result =
        FinalMeasurementResult::from_execution(
            request,
            execution,
        )
        .expect("valid result");

    let expectation =
        result.parity_expectation().expect("parity");

    assert!((expectation - 0.0).abs() < 1e-12);
}

#[test]
fn expectation_rejects_out_of_range_qubit() {
    let request =
        MeasurementRequest::new(vec![q(0)], nz(1))
            .expect("valid request");

    let mut histogram = MeasurementHistogram::new();

    histogram
        .record(bits(&[0]))
        .expect("record");

    let mut execution = MeasurementExecution::empty();
    execution.histogram = Some(histogram);

    let result =
        FinalMeasurementResult::from_execution(
            request,
            execution,
        )
        .expect("valid result");

    assert!(
        result.single_qubit_expectation(1).is_err()
    );
}

// =============================================================================
// Raw-shot provider integration
// =============================================================================

#[test]
fn_raw_shots_are_normalized_into_a_histogram() {
    let request =
        MeasurementRequest::new(vec![q(0)], nz(10))
            .expect("valid request");

    let mut provider = RawShotProvider::new();

    let mut request_with_shots = request.clone();
    request_with_shots.retain_shots = true;
    request_with_shots.retain_counts = false;
    request_with_shots.calculate_probabilities = true;

    let result = provider
        .measure_final(request_with_shots)
        .expect("raw-shot execution");

    assert_eq!(result.shots(), 10);
    assert!(result.shots.is_some());
    assert_eq!(result.count(&bits(&[0])), 10);
}

// =============================================================================
// Capability negotiation
// =============================================================================

#[test]
fn_z_basis_requires_z_capability() {
    let request =
        MeasurementRequest::new(vec![q(0)], nz(1))
            .expect("valid request");

    let capabilities =
        MeasurementCapabilities::HISTOGRAM
            | MeasurementCapabilities::COLLAPSE;

    assert!(
        crate::quantum::memory::measurement::validate_capabilities(
            &request,
            capabilities,
        )
        .is_err()
    );
}

#[test]
fn_x_basis_requires_x_capability() {
    let mut request =
        MeasurementRequest::new(vec![q(0)], nz(1))
            .expect("valid request");

    request.observable =
        MeasurementObservable::Standard(MeasurementBasis::X);

    let capabilities =
        MeasurementCapabilities::Z_BASIS
            | MeasurementCapabilities::COLLAPSE
            | MeasurementCapabilities::HISTOGRAM;

    assert!(
        crate::quantum::memory::measurement::validate_capabilities(
            &request,
            capabilities,
        )
        .is_err()
    );
}

#[test]
fn_y_basis_requires_y_capability() {
    let mut request =
        MeasurementRequest::new(vec![q(0)], nz(1))
            .expect("valid request");

    request.observable =
        MeasurementObservable::Standard(MeasurementBasis::Y);

    let capabilities =
        MeasurementCapabilities::Z_BASIS
            | MeasurementCapabilities::COLLAPSE
            | MeasurementCapabilities::HISTOGRAM;

    assert!(
        crate::quantum::memory::measurement::validate_capabilities(
            &request,
            capabilities,
        )
        .is_err()
    );
}

#[test]
fn_arbitrary_projective_measurement_requires_explicit_capability() {
    let mut request =
        MeasurementRequest::new(vec![q(0)], nz(1))
            .expect("valid request");

    request.observable =
        MeasurementObservable::BlochAxis(
            BlochAxis::new(1.0, 0.0, 0.0)
                .expect("valid axis"),
        );

    let capabilities =
        MeasurementCapabilities::Z_BASIS
            | MeasurementCapabilities::COLLAPSE
            | MeasurementCapabilities::HISTOGRAM;

    assert!(
        crate::quantum::memory::measurement::validate_capabilities(
            &request,
            capabilities,
        )
        .is_err()
    );
}

#[test]
fn_provider_defined_measurement_requires_provider_defined_capability() {
    let mut request =
        MeasurementRequest::new(vec![q(0)], nz(1))
            .expect("valid request");

    request.observable =
        MeasurementObservable::ProviderDefined {
            name: "native_basis".to_owned(),
            parameters: Vec::new(),
        };

    let capabilities =
        MeasurementCapabilities::Z_BASIS
            | MeasurementCapabilities::COLLAPSE
            | MeasurementCapabilities::HISTOGRAM;

    assert!(
        crate::quantum::memory::measurement::validate_capabilities(
            &request,
            capabilities,
        )
        .is_err()
    );
}

#[test]
fn multi_qubit_measurement_requires_multi_qubit_capability() {
    let request =
        MeasurementRequest::new(
            vec![q(0), q(1)],
            nz(1),
        )
        .expect("valid request");

    let capabilities =
        MeasurementCapabilities::Z_BASIS
            | MeasurementCapabilities::COLLAPSE
            | MeasurementCapabilities::HISTOGRAM;

    assert!(
        crate::quantum::memory::measurement::validate_capabilities(
            &request,
            capabilities,
        )
        .is_err()
    );
}

#[test]
fn mid_circuit_measurement_requires_mid_circuit_capability() {
    let mut request =
        MeasurementRequest::new(vec![q(0)], nz(1))
            .expect("valid request");

    request.mode = MeasurementMode::MidCircuit;

    let capabilities =
        MeasurementCapabilities::Z_BASIS
            | MeasurementCapabilities::COLLAPSE
            | MeasurementCapabilities::HISTOGRAM;

    assert!(
        crate::quantum::memory::measurement::validate_capabilities(
            &request,
            capabilities,
        )
        .is_err()
    );
}

#[test]
fn non_destructive_measurement_requires_non_destructive_capability() {
    let mut request =
        MeasurementRequest::new(vec![q(0)], nz(1))
            .expect("valid request");

    request.mode = MeasurementMode::NonDestructive;

    let capabilities =
        MeasurementCapabilities::Z_BASIS
            | MeasurementCapabilities::COLLAPSE
            | MeasurementCapabilities::HISTOGRAM;

    assert!(
        crate::quantum::memory::measurement::validate_capabilities(
            &request,
            capabilities,
        )
        .is_err()
    );
}

#[test]
fn collapse_requires_collapse_capability() {
    let request =
        MeasurementRequest::new(vec![q(0)], nz(1))
            .expect("valid request");

    let capabilities =
        MeasurementCapabilities::Z_BASIS
            | MeasurementCapabilities::HISTOGRAM;

    assert!(
        crate::quantum::memory::measurement::validate_capabilities(
            &request,
            capabilities,
        )
        .is_err()
    );
}

#[test]
fn classical_destinations_require_explicit_capability() {
    let mut request =
        MeasurementRequest::new(vec![q(0)], nz(1))
            .expect("valid request");

    request.classical_destinations = vec![
        ClassicalDestination::new(q(0), c(0)),
    ];

    let capabilities =
        MeasurementCapabilities::Z_BASIS
            | MeasurementCapabilities::COLLAPSE
            | MeasurementCapabilities::HISTOGRAM;

    assert!(
        crate::quantum::memory::measurement::validate_capabilities(
            &request,
            capabilities,
        )
        .is_err()
    );
}

#[test]
fn raw_shots_require_explicit_capability() {
    let mut request =
        MeasurementRequest::new(vec![q(0)], nz(1))
            .expect("valid request");

    request.retain_shots = true;

    let capabilities =
        MeasurementCapabilities::Z_BASIS
            | MeasurementCapabilities::COLLAPSE
            | MeasurementCapabilities::HISTOGRAM;

    assert!(
        crate::quantum::memory::measurement::validate_capabilities(
            &request,
            capabilities,
        )
        .is_err()
    );
}

#[test]
fn deterministic_sampling_requires_deterministic_capability() {
    let mut request =
        MeasurementRequest::new(vec![q(0)], nz(1))
            .expect("valid request");

    request.deterministic_seed = Some(42);

    let capabilities =
        MeasurementCapabilities::Z_BASIS
            | MeasurementCapabilities::COLLAPSE
            | MeasurementCapabilities::HISTOGRAM;

    assert!(
        crate::quantum::memory::measurement::validate_capabilities(
            &request,
            capabilities,
        )
        .is_err()
    );
}

// =============================================================================
// Provider integration
// =============================================================================

#[test]
fn provider_can_execute_a_standard_measurement() {
    let request =
        MeasurementRequest::new(vec![q(0)], nz(100))
            .expect("valid request");

    let mut provider =
        ZeroResultProvider::new(full_standard_capabilities());

    let result = provider
        .measure_final(request)
        .expect("provider execution");

    assert_eq!(result.shots(), 100);
    assert_eq!(result.count(&bits(&[0])), 100);
    assert!(result.state_collapsed);
    assert!(result.destructive);
}

#[test]
fn provider_name_is_optional_and_opaque() {
    let provider = ZeroResultProvider::with_name(
        full_standard_capabilities(),
        "example-provider",
    );

    assert_eq!(
        provider.provider_name(),
        Some("example-provider")
    );
}

#[test]
fn full_capability_provider_accepts_mid_circuit_measurement() {
    let mut request =
        MeasurementRequest::new(vec![q(0)], nz(10))
            .expect("valid request");

    request.mode = MeasurementMode::MidCircuit;

    let mut provider =
        ZeroResultProvider::new(full_standard_capabilities());

    let result = provider
        .measure_final(request)
        .expect("mid-circuit measurement");

    assert!(!result.destructive);
    assert!(result.state_collapsed);
}

#[test]
fn full_capability_provider_accepts_non_destructive_measurement() {
    let mut request =
        MeasurementRequest::new(vec![q(0)], nz(10))
            .expect("valid request");

    request.mode = MeasurementMode::NonDestructive;
    request.collapse =
        CollapsePolicy::PreserveIfSupported;

    let mut provider =
        ZeroResultProvider::new(full_standard_capabilities());

    let result = provider
        .measure_final(request)
        .expect("non-destructive measurement");

    assert!(!result.destructive);
}

#[test]
fn physical_qpu_provider_does_not_claim_deterministic_sampling() {
    let request =
        MeasurementRequest::new(vec![q(0)], nz(1))
            .expect("valid request");

    let mut deterministic_request = request.clone();
    deterministic_request.deterministic_seed = Some(123);

    let mut provider = PhysicalQpuProvider::new();

    assert!(
        provider
            .measure_final(deterministic_request)
            .is_err()
    );

    assert!(
        provider
            .measure_final(request)
            .is_ok()
    );
}

// =============================================================================
// Execution validation
// =============================================================================

#[test]
fn execution_requires_at_least_one_result_representation() {
    let request =
        MeasurementRequest::new(vec![q(0)], nz(1))
            .expect("valid request");

    let execution = MeasurementExecution::empty();

    assert!(execution.validate(&request).is_err());
}

#[test]
fn execution_histogram_width_must_match_request() {
    let request =
        MeasurementRequest::new(
            vec![q(0), q(1)],
            nz(1),
        )
        .expect("valid request");

    let mut histogram = MeasurementHistogram::new();

    histogram
        .record(bits(&[0]), 1)
        .expect("record");

    let mut execution = MeasurementExecution::empty();
    execution.histogram = Some(histogram);

    assert!(execution.validate(&request).is_err());
}

#[test]
fn execution_histogram_shot_count_must_match_request() {
    let request =
        MeasurementRequest::new(vec![q(0)], nz(10))
            .expect("valid request");

    let mut histogram = MeasurementHistogram::new();

    histogram
        .record_count(bits(&[0]), 9)
        .expect("record");

    let mut execution = MeasurementExecution::empty();
    execution.histogram = Some(histogram);

    assert!(execution.validate(&request).is_err());
}

#[test]
fn execution_schema_id_must_match() {
    let request =
        MeasurementRequest::new(vec![q(0)], nz(1))
            .expect("valid request");

    let mut histogram = MeasurementHistogram::new();

    histogram
        .record(bits(&[0]))
        .expect("record");

    let mut execution = MeasurementExecution::empty();
    execution.schema_id = "wrong.schema";
    execution.histogram = Some(histogram);

    assert!(execution.validate(&request).is_err());
}

#[test]
fn execution_schema_version_must_match() {
    let request =
        MeasurementRequest::new(vec![q(0)], nz(1))
            .expect("valid request");

    let mut histogram = MeasurementHistogram::new();

    histogram
        .record(bits(&[0]))
        .expect("record");

    let mut execution = MeasurementExecution::empty();
    execution.schema_version =
        MEASUREMENT_SCHEMA_VERSION + 1;
    execution.histogram = Some(histogram);

    assert!(execution.validate(&request).is_err());
}

#[test]
fn raw_shots_must_match_requested_shot_count() {
    let request =
        MeasurementRequest::new(vec![q(0)], nz(2))
            .expect("valid request");

    let shots = vec![
        MeasurementShot::new(
            bits(&[0]),
            Vec::new(),
            true,
        )
        .expect("shot"),
    ];

    let mut execution = MeasurementExecution::empty();
    execution.shots = Some(shots);

    assert!(execution.validate(&request).is_err());
}

#[test]
fn raw_shots_must_match_requested_width() {
    let request =
        MeasurementRequest::new(
            vec![q(0), q(1)],
            nz(1),
        )
        .expect("valid request");

    let shots = vec![
        MeasurementShot::new(
            bits(&[0]),
            Vec::new(),
            true,
        )
        .expect("shot"),
    ];

    let mut execution = MeasurementExecution::empty();
    execution.shots = Some(shots);

    assert!(execution.validate(&request).is_err());
}

// =============================================================================
// Readout metadata
// =============================================================================

#[test]
fn valid_readout_metadata_is_accepted() {
    let metadata = ReadoutMetadata {
        calibrated: true,
        assignment_error_rates: vec![
            (0.01, 0.02),
            (0.03, 0.04),
        ],
        mitigated: false,
        calibration_id: Some("calibration-2026-01".to_owned()),
    };

    assert!(metadata.validate(2).is_ok());
}

#[test]
fn readout_error_rates_must_be_in_range() {
    let metadata = ReadoutMetadata {
        calibrated: true,
        assignment_error_rates: vec![
            (1.1, 0.0),
        ],
        mitigated: false,
        calibration_id: None,
    };

    assert!(metadata.validate(1).is_err());
}

#[test]
fn readout_metadata_cannot_exceed_measured_width() {
    let metadata = ReadoutMetadata {
        calibrated: true,
        assignment_error_rates: vec![
            (0.01, 0.02),
            (0.01, 0.02),
        ],
        mitigated: false,
        calibration_id: None,
    };

    assert!(metadata.validate(1).is_err());
}

// =============================================================================
// Provider metadata security boundary
// =============================================================================

#[test]
fn provider_metadata_accepts_non_sensitive_information() {
    let mut metadata =
        MeasurementProviderMetadata::new();

    metadata
        .insert("job_id", "job-123")
        .expect("safe metadata");

    metadata
        .insert("device_family", "test-qpu")
        .expect("safe metadata");

    assert_eq!(
        metadata.get("job_id"),
        Some("job-123")
    );
}

#[test]
fn provider_metadata_rejects_api_keys() {
    let mut metadata =
        MeasurementProviderMetadata::new();

    assert!(
        metadata
            .insert("api_key", "secret-value")
            .is_err()
    );
}

#[test]
fn provider_metadata_rejects_tokens() {
    let mut metadata =
        MeasurementProviderMetadata::new();

    assert!(
        metadata
            .insert("access_token", "secret-value")
            .is_err()
    );
}

#[test]
fn provider_metadata_rejects_private_keys() {
    let mut metadata =
        MeasurementProviderMetadata::new();

    assert!(
        metadata
            .insert("private_key", "secret-value")
            .is_err()
    );
}

// =============================================================================
// Classical result mapping
// =============================================================================

#[test]
fn measurement_shot_can_carry_classical_destination_values() {
    let outcome = bits(&[1]);

    let shot = MeasurementShot::new(
        outcome.clone(),
        vec![MeasurementBit::One],
        true,
    )
    .expect("valid shot");

    assert_eq!(shot.outcome, outcome);
    assert_eq!(
        shot.classical_values,
        vec![MeasurementBit::One]
    );
    assert!(shot.state_collapsed);
}

#[test]
fn final_result_preserves_provider_metadata() {
    let request =
        MeasurementRequest::new(vec![q(0)], nz(1))
            .expect("valid request");

    let mut histogram = MeasurementHistogram::new();

    histogram
        .record(bits(&[0]))
        .expect("record");

    let mut provider_metadata =
        MeasurementProviderMetadata::new();

    provider_metadata
        .insert("job_id", "job-42")
        .expect("safe metadata");

    let mut execution = MeasurementExecution::empty();

    execution.histogram = Some(histogram);
    execution.provider_execution_id =
        Some("opaque-id".to_owned());
    execution.provider_metadata = provider_metadata;

    let result =
        FinalMeasurementResult::from_execution(
            request,
            execution,
        )
        .expect("valid result");

    assert_eq!(
        result.provider_execution_id.as_deref(),
        Some("opaque-id")
    );

    assert_eq!(
        result.provider_metadata.get("job_id"),
        Some("job-42")
    );
}

// =============================================================================
// Capability surface regression test
// =============================================================================

#[test]
fn capability_flags_are_independent_and_composable() {
    let capabilities =
        MeasurementCapabilities::Z_BASIS
            | MeasurementCapabilities::MULTI_QUBIT
            | MeasurementCapabilities::HISTOGRAM;

    assert!(
        capabilities.contains(
            MeasurementCapabilities::Z_BASIS
        )
    );

    assert!(
        capabilities.contains(
            MeasurementCapabilities::MULTI_QUBIT
        )
    );

    assert!(
        capabilities.contains(
            MeasurementCapabilities::HISTOGRAM
        )
    );

    assert!(
        !capabilities.contains(
            MeasurementCapabilities::X_BASIS
        )
    );
}

#[test]
fn none_capabilities_are_empty() {
    assert!(
        MeasurementCapabilities::NONE.is_empty()
    );

    assert_eq!(
        MeasurementCapabilities::NONE.bits(),
        0
    );
}

// =============================================================================
// Structural safety
// =============================================================================

#[test]
fn measurement_request_has_a_structural_qubit_limit() {
    let mut request =
        MeasurementRequest::new(vec![q(0)], nz(1))
            .expect("valid request");

    request.qubits =
        vec![q(0); MAX_MEASUREMENT_QUBITS + 1];

    assert!(request.validate().is_err());
}

// =============================================================================
// Full integration contract
// =============================================================================

#[test]
fn complete_provider_neutral_measurement_pipeline() {
    let mut request =
        MeasurementRequest::new(
            vec![q(0), q(1)],
            nz(1_000),
        )
        .expect("valid request");

    request.physical_qubits =
        vec![p(4), p(7)];

    request.classical_destinations = vec![
        ClassicalDestination::new(q(0), c(0)),
        ClassicalDestination::new(q(1), c(1)),
    ];

    request.mode = MeasurementMode::MidCircuit;
    request.collapse = CollapsePolicy::Collapse;
    request.retain_counts = true;
    request.calculate_probabilities = true;

    let mut provider =
        ZeroResultProvider::new(
            full_standard_capabilities(),
        );

    let result = provider
        .measure_final(request)
        .expect("complete measurement pipeline");

    assert_eq!(result.shots(), 1_000);
    assert_eq!(
        result.histogram.outcome_count(),
        1
    );
    assert_eq!(
        result.count(&bits(&[0, 0])),
        1_000
    );

    assert!(
        result
            .probability(&bits(&[0, 0]))
            .is_finite()
    );

    assert_eq!(
        result
            .request
            .physical_qubits
            .len(),
        2
    );

    assert_eq!(
        result
            .request
            .classical_destinations
            .len(),
        2
    );
}

// =============================================================================
// QPU-neutrality regression tests
// =============================================================================

#[test]
fn measurement_contract_does_not_require_state_vector_access() {
    let request =
        MeasurementRequest::new(vec![q(0)], nz(10))
            .expect("valid request");

    let mut provider =
        PhysicalQpuProvider::new();

    let result = provider
        .measure_final(request)
        .expect("provider execution");

    assert_eq!(result.shots(), 10);

    // No amplitude/state-vector API is required to consume the result.
    assert_eq!(
        result.count(&bits(&[0])),
        10
    );
}

#[test]
fn provider_controlled_randomness_is_distinct_from_deterministic_sampling() {
    let provider =
        PhysicalQpuProvider::new();

    assert!(
        provider
            .capabilities()
            .contains(
                MeasurementCapabilities::PROVIDER_RANDOMNESS
            )
    );

    assert!(
        !provider
            .capabilities()
            .contains(
                MeasurementCapabilities::DETERMINISTIC_SAMPLING
            )
    );
}

#[test]
fn distributed_capability_is_explicit() {
    let capabilities =
        full_standard_capabilities();

    assert!(
        capabilities.contains(
            MeasurementCapabilities::DISTRIBUTED
        )
    );
}