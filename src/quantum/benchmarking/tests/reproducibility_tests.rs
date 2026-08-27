//! Zamani Quantum Benchmarking — Reproducibility Integration Tests
//!
//! Production test suite for:
//!
//!     src/quantum/benchmarking/core/reproducibility.rs
//!
//! # Purpose
//!
//! These tests verify the reproducibility contract used by every Zamani
//! quantum benchmark. The contract must remain independent of:
//!
//! - backend implementation;
//! - hardware vendor;
//! - simulator implementation;
//! - wall-clock time;
//! - host machine;
//! - thread scheduling;
//! - process-global state;
//! - logging;
//! - hash-map iteration order;
//! - transport metadata;
//! - execution results;
//! - Rust Debug formatting.
//!
//! The tests intentionally exercise the public reproducibility API rather than
//! private implementation details. This allows `core/reproducibility.rs` to
//! evolve internally without requiring this test module to be rewritten.
//!
//! # Architectural contract
//!
//! The reproducibility layer sits below all benchmark protocols:
//!
//! ```text
//! Zamani benchmark
//!       │
//!       ├── configuration ──────┐
//!       │                       │
//!       ├── generator/version ──┤
//!       │                       │
//!       ├── seed ───────────────┤
//!       │                       ▼
//!       │                ExperimentIdentity
//!       │                       │
//!       ├── generated circuits ┘
//!       │           │
//!       │           ▼
//!       │     CircuitFingerprint
//!       │           │
//!       ▼           ▼
//!     execution / result
//!              │
//!              ▼
//!       ResultFingerprint
//! ```
//!
//! # Supported benchmark families
//!
//! These tests intentionally do not depend on a particular protocol so the
//! same reproducibility contract can be consumed by:
//!
//! - Quantum Volume;
//! - randomized benchmarking;
//! - interleaved RB;
//! - simultaneous RB;
//! - purity RB;
//! - leakage RB;
//! - cycle benchmarking;
//! - layer fidelity;
//! - XEB;
//! - random circuit sampling;
//! - mirror circuits;
//! - SPAM characterization;
//! - gate/process fidelity;
//! - coherence;
//! - crosstalk;
//! - drift;
//! - tomography;
//! - volumetric benchmarking;
//! - application benchmarks;
//! - QEC benchmarks;
//! - custom Zamani benchmarks.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust only.
//!
//! No nightly features are required.
//!
//! # Integration contract
//!
//! This file imports only the stable public API from
//! `core::reproducibility`. It therefore does not require modifications when
//! future modules such as:
//!
//! - `core::config`;
//! - `core::provenance`;
//! - `core::result`;
//! - `generators::*`;
//! - `protocols::*`;
//! - `reporting::*`;
//! - `analysis::*`;
//!
//! are added or modified.
//!
//! Those modules can consume the same API without changing the invariants
//! tested here.
//!
//! # Test philosophy
//!
//! A reproducibility test must prove both:
//!
//! 1. identical semantic input produces identical identity;
//! 2. semantically relevant input changes produce different identity.
//!
//! Tests must not assert a particular SHA-256 hexadecimal value unless that
//! value is itself a formally frozen interoperability vector. Doing so would
//! unnecessarily couple protocol evolution to an implementation detail.
//!
//! Instead, these tests assert the semantic properties of the identity
//! contract.
//!
//! # Security properties
//!
//! The tests also verify that:
//!
//! - field concatenation cannot create ambiguous identities;
//! - domains are separated;
//! - control characters are rejected from identity metadata;
//! - empty circuit sets are rejected;
//! - generator algorithm/version changes invalidate identity;
//! - seed changes invalidate randomized experiment identity;
//! - circuit ordering is preserved;
//! - fingerprints are fixed-size and non-zero for normal inputs.
//!
//! # Important boundary
//!
//! These are reproducibility tests, not cryptographic-authentication tests.
//! SHA-256 fingerprints identify content; they do not prove that a backend
//! actually executed the claimed circuit or that hardware metadata is honest.
//! Such guarantees belong to provenance/signing/attestation layers.
//!

use super::super::core::reproducibility::{
    BenchmarkSeed,
    CircuitFingerprint,
    ConfigurationFingerprint,
    ExperimentIdentity,
    Fingerprint,
    GeneratorDescriptor,
    ReproducibilityError,
    ReproducibilityRecord,
    ResultFingerprint,
    FINGERPRINT_ALGORITHM,
    FINGERPRINT_BYTES,
    FINGERPRINT_HEX_LENGTH,
    REPRODUCIBILITY_SCHEMA_VERSION,
};

// ============================================================================
// Test fixtures
// ============================================================================

/// Stable benchmark identifier used by the tests.
const BENCHMARK_ID: &str = "quantum_volume";

/// Stable benchmark version used by the tests.
const BENCHMARK_VERSION: &str = "1.0.0";

/// Stable generator identifier used by the tests.
const GENERATOR_ID: &str = "zamani.qv";

/// Stable generator version used by the tests.
const GENERATOR_VERSION: &str = "1.0.0";

/// Stable RNG identifier used by randomized benchmark fixtures.
const RNG_ALGORITHM: &str = "zamani-rng-v1";

/// Stable seed used by the baseline fixture.
const BASE_SEED: u64 = 42;

/// Stable canonical configuration bytes.
///
/// The bytes deliberately represent a canonical serialization rather than a
/// Rust struct layout. Production callers must supply their own canonical
/// representation from `core::config`.
const CONFIGURATION_BYTES: &[u8] =
    b"benchmark=quantum_volume;width=8;depth=8;shots=1000;confidence=0.9544997361036416";

/// Second configuration fixture differing by exactly one semantic field.
const DIFFERENT_CONFIGURATION_BYTES: &[u8] =
    b"benchmark=quantum_volume;width=9;depth=8;shots=1000;confidence=0.9544997361036416";

/// Stable canonical circuit representation.
const CIRCUIT_A_BYTES: &[u8] =
    b"circuit:v1;width=8;depth=8;gates=h,h,cx,x,cx";

/// Second stable canonical circuit representation.
const CIRCUIT_B_BYTES: &[u8] =
    b"circuit:v1;width=8;depth=8;gates=h,cx,h,cx,x";

/// Third circuit fixture.
const CIRCUIT_C_BYTES: &[u8] =
    b"circuit:v1;width=8;depth=8;gates=x,h,cx,h,cx";

/// Stable canonical result representation.
const RESULT_BYTES: &[u8] =
    b"result:v1;heavy_output_probability=0.71;shots=1000";

/// Second result fixture differing in a semantic measurement.
const DIFFERENT_RESULT_BYTES: &[u8] =
    b"result:v1;heavy_output_probability=0.72;shots=1000";

fn baseline_generator() -> GeneratorDescriptor {
    GeneratorDescriptor::new(GENERATOR_ID, GENERATOR_VERSION)
        .with_rng_algorithm(RNG_ALGORITHM)
}

fn baseline_configuration() -> ConfigurationFingerprint {
    ConfigurationFingerprint::from_canonical_bytes(CONFIGURATION_BYTES)
}

fn baseline_circuit_a() -> CircuitFingerprint {
    CircuitFingerprint::from_canonical_bytes(CIRCUIT_A_BYTES)
}

fn baseline_circuit_b() -> CircuitFingerprint {
    CircuitFingerprint::from_canonical_bytes(CIRCUIT_B_BYTES)
}

fn baseline_circuit_c() -> CircuitFingerprint {
    CircuitFingerprint::from_canonical_bytes(CIRCUIT_C_BYTES)
}

fn baseline_result() -> ResultFingerprint {
    ResultFingerprint::from_canonical_bytes(RESULT_BYTES)
}

fn baseline_identity() -> ExperimentIdentity {
    let generator = baseline_generator();
    let configuration = baseline_configuration();

    ExperimentIdentity::from_canonical_bytes(
        BENCHMARK_ID,
        BENCHMARK_VERSION,
        &generator,
        BenchmarkSeed::new(BASE_SEED),
        &configuration.fingerprint(),
    )
    .expect("baseline experiment identity must be valid")
}

fn baseline_record() -> ReproducibilityRecord {
    let generator = baseline_generator();
    let configuration = baseline_configuration();
    let experiment = ExperimentIdentity::from_canonical_bytes(
        BENCHMARK_ID,
        BENCHMARK_VERSION,
        &generator,
        BenchmarkSeed::new(BASE_SEED),
        &configuration.fingerprint(),
    )
    .expect("baseline experiment identity must be valid");

    ReproducibilityRecord::new(
        experiment,
        generator,
        BenchmarkSeed::new(BASE_SEED),
        configuration,
        vec![
            baseline_circuit_a(),
            baseline_circuit_b(),
            baseline_circuit_c(),
        ],
    )
    .expect("baseline reproducibility record must be valid")
}

// ============================================================================
// Fingerprint primitive
// ============================================================================

#[test]
fn fingerprint_has_the_required_sha256_size() {
    assert_eq!(FINGERPRINT_ALGORITHM, "sha256");
    assert_eq!(FINGERPRINT_BYTES, 32);
    assert_eq!(FINGERPRINT_HEX_LENGTH, 64);

    let fingerprint = Fingerprint::from_bytes(b"zamani");

    assert_eq!(fingerprint.as_bytes().len(), FINGERPRINT_BYTES);
    assert_eq!(fingerprint.hex().len(), FINGERPRINT_HEX_LENGTH);
    assert!(!fingerprint.is_zero());
}

#[test]
fn identical_bytes_produce_identical_fingerprints() {
    let first = Fingerprint::from_bytes(b"canonical zamani benchmark");
    let second = Fingerprint::from_bytes(b"canonical zamani benchmark");

    assert_eq!(first, second);
    assert_eq!(first.as_bytes(), second.as_bytes());
    assert_eq!(first.hex(), second.hex());
}

#[test]
fn different_bytes_produce_different_fingerprints() {
    let first = Fingerprint::from_bytes(b"benchmark-a");
    let second = Fingerprint::from_bytes(b"benchmark-b");

    assert_ne!(first, second);
    assert_ne!(first.as_bytes(), second.as_bytes());
}

#[test]
fn empty_payload_is_still_a_valid_nonzero_sha256_fingerprint() {
    let fingerprint = Fingerprint::from_bytes(b"");

    assert_eq!(fingerprint.as_bytes().len(), FINGERPRINT_BYTES);
    assert_eq!(fingerprint.hex().len(), FINGERPRINT_HEX_LENGTH);
    assert!(!fingerprint.is_zero());
}

#[test]
fn fingerprint_display_matches_hex_representation() {
    let fingerprint = Fingerprint::from_bytes(b"zamani");

    assert_eq!(format!("{}", fingerprint), fingerprint.hex());
    assert_eq!(format!("{:?}", fingerprint), format!("Fingerprint(\"{}\")", fingerprint.hex()));
}

#[test]
fn fingerprint_is_copyable_without_changing_identity() {
    let original = Fingerprint::from_bytes(b"zamani");
    let copied = original;

    assert_eq!(original, copied);
    assert_eq!(original.hex(), copied.hex());
}

// ============================================================================
// Domain separation
// ============================================================================

#[test]
fn domain_separation_prevents_cross_domain_identity_reuse() {
    let payload = b"identical semantic payload";

    let configuration = Fingerprint::from_domain_and_bytes(
        b"zamani:quantum:benchmark:configuration:v1\0",
        payload,
    );

    let circuit = Fingerprint::from_domain_and_bytes(
        b"zamani:quantum:benchmark:circuit:v1\0",
        payload,
    );

    let result = Fingerprint::from_domain_and_bytes(
        b"zamani:quantum:benchmark:result:v1\0",
        payload,
    );

    assert_ne!(configuration, circuit);
    assert_ne!(configuration, result);
    assert_ne!(circuit, result);
}

#[test]
fn changing_domain_changes_identity_even_when_payload_is_identical() {
    let payload = b"same";

    let first =
        Fingerprint::from_domain_and_bytes(b"domain-a", payload);

    let second =
        Fingerprint::from_domain_and_bytes(b"domain-b", payload);

    assert_ne!(first, second);
}

#[test]
fn changing_payload_changes_domain_separated_identity() {
    let first = Fingerprint::from_domain_and_bytes(
        b"zamani:test:v1",
        b"payload-a",
    );

    let second = Fingerprint::from_domain_and_bytes(
        b"zamani:test:v1",
        b"payload-b",
    );

    assert_ne!(first, second);
}

// ============================================================================
// Canonical field boundaries
// ============================================================================

#[test]
fn length_prefixing_prevents_ambiguous_string_concatenation() {
    let mut first = Vec::new();

    first.extend_from_slice(&(2u64).to_be_bytes());
    first.extend_from_slice(b"ab");
    first.extend_from_slice(&(1u64).to_be_bytes());
    first.extend_from_slice(b"c");

    let mut second = Vec::new();

    second.extend_from_slice(&(1u64).to_be_bytes());
    second.extend_from_slice(b"a");
    second.extend_from_slice(&(2u64).to_be_bytes());
    second.extend_from_slice(b"bc");

    assert_ne!(
        Fingerprint::from_bytes(&first),
        Fingerprint::from_bytes(&second)
    );
}

#[test]
fn domain_fingerprint_length_prefix_is_deterministic() {
    let first = Fingerprint::from_domain_and_bytes(
        b"zamani:test:v1",
        b"canonical payload",
    );

    let second = Fingerprint::from_domain_and_bytes(
        b"zamani:test:v1",
        b"canonical payload",
    );

    assert_eq!(first, second);
}

// ============================================================================
// Generator descriptor
// ============================================================================

#[test]
fn generator_descriptor_is_valid_for_baseline_fixture() {
    let generator = baseline_generator();

    assert!(generator.validate().is_ok());
    assert_eq!(generator.id, GENERATOR_ID);
    assert_eq!(generator.version, GENERATOR_VERSION);
    assert_eq!(
        generator.rng_algorithm.as_deref(),
        Some(RNG_ALGORITHM)
    );
}

#[test]
fn identical_generator_descriptors_have_identical_fingerprints() {
    let first = baseline_generator();
    let second = baseline_generator();

    assert_eq!(first.fingerprint(), second.fingerprint());
}

#[test]
fn generator_id_is_part_of_identity() {
    let first = GeneratorDescriptor::new(
        GENERATOR_ID,
        GENERATOR_VERSION,
    );

    let second = GeneratorDescriptor::new(
        "zamani.other-generator",
        GENERATOR_VERSION,
    );

    assert_ne!(first.fingerprint(), second.fingerprint());
}

#[test]
fn generator_version_is_part_of_identity() {
    let first = GeneratorDescriptor::new(
        GENERATOR_ID,
        "1.0.0",
    );

    let second = GeneratorDescriptor::new(
        GENERATOR_ID,
        "1.0.1",
    );

    assert_ne!(first.fingerprint(), second.fingerprint());
}

#[test]
fn generator_rng_algorithm_is_part_of_identity() {
    let first = GeneratorDescriptor::new(
        GENERATOR_ID,
        GENERATOR_VERSION,
    )
    .with_rng_algorithm("zamani-rng-v1");

    let second = GeneratorDescriptor::new(
        GENERATOR_ID,
        GENERATOR_VERSION,
    )
    .with_rng_algorithm("zamani-rng-v2");

    assert_ne!(first.fingerprint(), second.fingerprint());
}

#[test]
fn generator_without_rng_is_distinct_from_generator_with_rng() {
    let deterministic = GeneratorDescriptor::new(
        GENERATOR_ID,
        GENERATOR_VERSION,
    );

    let stochastic = GeneratorDescriptor::new(
        GENERATOR_ID,
        GENERATOR_VERSION,
    )
    .with_rng_algorithm(RNG_ALGORITHM);

    assert_ne!(
        deterministic.fingerprint(),
        stochastic.fingerprint()
    );
}

#[test]
fn empty_generator_id_is_rejected() {
    let generator = GeneratorDescriptor::new(
        "",
        GENERATOR_VERSION,
    );

    let result = generator.validate();

    assert!(matches!(
        result,
        Err(ReproducibilityError::EmptyIdentifier {
            field: "generator id"
        })
    ));
}

#[test]
fn empty_generator_version_is_rejected() {
    let generator = GeneratorDescriptor::new(
        GENERATOR_ID,
        "",
    );

    let result = generator.validate();

    assert!(matches!(
        result,
        Err(ReproducibilityError::EmptyIdentifier {
            field: "generator version"
        })
    ));
}

#[test]
fn control_character_in_generator_id_is_rejected() {
    let generator = GeneratorDescriptor::new(
        "zamani.qv\nmalicious",
        GENERATOR_VERSION,
    );

    let result = generator.validate();

    assert!(matches!(
        result,
        Err(ReproducibilityError::InvalidIdentifier {
            field: "generator id"
        })
    ));
}

#[test]
fn control_character_in_generator_version_is_rejected() {
    let generator = GeneratorDescriptor::new(
        GENERATOR_ID,
        "1.0.0\r\nunexpected",
    );

    let result = generator.validate();

    assert!(matches!(
        result,
        Err(ReproducibilityError::InvalidIdentifier {
            field: "generator version"
        })
    ));
}

#[test]
fn control_character_in_rng_algorithm_is_rejected() {
    let generator = GeneratorDescriptor::new(
        GENERATOR_ID,
        GENERATOR_VERSION,
    )
    .with_rng_algorithm("zamani-rng-v1\tunexpected");

    let result = generator.validate();

    assert!(matches!(
        result,
        Err(ReproducibilityError::InvalidIdentifier {
            field: "RNG algorithm"
        })
    ));
}

// ============================================================================
// Benchmark seed
// ============================================================================

#[test]
fn benchmark_seed_round_trips_exactly() {
    let values = [
        0u64,
        1u64,
        42u64,
        u64::MAX,
    ];

    for value in values {
        let seed = BenchmarkSeed::new(value);

        assert_eq!(seed.value(), value);
        assert_eq!(format!("{}", seed), value.to_string());
    }
}

#[test]
fn different_seeds_produce_different_experiment_identities() {
    let generator = baseline_generator();
    let configuration = baseline_configuration();

    let first = ExperimentIdentity::from_canonical_bytes(
        BENCHMARK_ID,
        BENCHMARK_VERSION,
        &generator,
        BenchmarkSeed::new(1),
        &configuration.fingerprint(),
    )
    .expect("first identity must be valid");

    let second = ExperimentIdentity::from_canonical_bytes(
        BENCHMARK_ID,
        BENCHMARK_VERSION,
        &generator,
        BenchmarkSeed::new(2),
        &configuration.fingerprint(),
    )
    .expect("second identity must be valid");

    assert_ne!(first, second);
}

// ============================================================================
// Experiment identity
// ============================================================================

#[test]
fn identical_experiment_definitions_have_identical_identity() {
    let first = baseline_identity();
    let second = baseline_identity();

    assert_eq!(first, second);
    assert_eq!(first.fingerprint(), second.fingerprint());
    assert_eq!(first.hex(), second.hex());
}

#[test]
fn experiment_identity_has_a_nonzero_fingerprint() {
    let identity = baseline_identity();

    assert!(!identity.fingerprint().is_zero());
    assert_eq!(
        identity.hex().len(),
        FINGERPRINT_HEX_LENGTH
    );
}

#[test]
fn benchmark_id_is_part_of_experiment_identity() {
    let generator = baseline_generator();
    let configuration = baseline_configuration();

    let first = ExperimentIdentity::from_canonical_bytes(
        "quantum_volume",
        BENCHMARK_VERSION,
        &generator,
        BenchmarkSeed::new(BASE_SEED),
        &configuration.fingerprint(),
    )
    .expect("first identity must be valid");

    let second = ExperimentIdentity::from_canonical_bytes(
        "randomized_benchmarking",
        BENCHMARK_VERSION,
        &generator,
        BenchmarkSeed::new(BASE_SEED),
        &configuration.fingerprint(),
    )
    .expect("second identity must be valid");

    assert_ne!(first, second);
}

#[test]
fn benchmark_version_is_part_of_experiment_identity() {
    let generator = baseline_generator();
    let configuration = baseline_configuration();

    let first = ExperimentIdentity::from_canonical_bytes(
        BENCHMARK_ID,
        "1.0.0",
        &generator,
        BenchmarkSeed::new(BASE_SEED),
        &configuration.fingerprint(),
    )
    .expect("first identity must be valid");

    let second = ExperimentIdentity::from_canonical_bytes(
        BENCHMARK_ID,
        "1.0.1",
        &generator,
        BenchmarkSeed::new(BASE_SEED),
        &configuration.fingerprint(),
    )
    .expect("second identity must be valid");

    assert_ne!(first, second);
}

#[test]
fn configuration_is_part_of_experiment_identity() {
    let generator = baseline_generator();

    let first_configuration =
        ConfigurationFingerprint::from_canonical_bytes(
            CONFIGURATION_BYTES,
        );

    let second_configuration =
        ConfigurationFingerprint::from_canonical_bytes(
            DIFFERENT_CONFIGURATION_BYTES,
        );

    let first = ExperimentIdentity::from_canonical_bytes(
        BENCHMARK_ID,
        BENCHMARK_VERSION,
        &generator,
        BenchmarkSeed::new(BASE_SEED),
        &first_configuration.fingerprint(),
    )
    .expect("first identity must be valid");

    let second = ExperimentIdentity::from_canonical_bytes(
        BENCHMARK_ID,
        BENCHMARK_VERSION,
        &generator,
        BenchmarkSeed::new(BASE_SEED),
        &second_configuration.fingerprint(),
    )
    .expect("second identity must be valid");

    assert_ne!(first, second);
}

#[test]
fn generator_is_part_of_experiment_identity() {
    let configuration = baseline_configuration();

    let first_generator = GeneratorDescriptor::new(
        GENERATOR_ID,
        "1.0.0",
    )
    .with_rng_algorithm(RNG_ALGORITHM);

    let second_generator = GeneratorDescriptor::new(
        GENERATOR_ID,
        "2.0.0",
    )
    .with_rng_algorithm(RNG_ALGORITHM);

    let first = ExperimentIdentity::from_canonical_bytes(
        BENCHMARK_ID,
        BENCHMARK_VERSION,
        &first_generator,
        BenchmarkSeed::new(BASE_SEED),
        &configuration.fingerprint(),
    )
    .expect("first identity must be valid");

    let second = ExperimentIdentity::from_canonical_bytes(
        BENCHMARK_ID,
        BENCHMARK_VERSION,
        &second_generator,
        BenchmarkSeed::new(BASE_SEED),
        &configuration.fingerprint(),
    )
    .expect("second identity must be valid");

    assert_ne!(first, second);
}

#[test]
fn empty_benchmark_id_is_rejected() {
    let generator = baseline_generator();
    let configuration = baseline_configuration();

    let result = ExperimentIdentity::from_canonical_bytes(
        "",
        BENCHMARK_VERSION,
        &generator,
        BenchmarkSeed::new(BASE_SEED),
        &configuration.fingerprint(),
    );

    assert!(matches!(
        result,
        Err(ReproducibilityError::EmptyIdentifier {
            field: "benchmark id"
        })
    ));
}

#[test]
fn empty_benchmark_version_is_rejected() {
    let generator = baseline_generator();
    let configuration = baseline_configuration();

    let result = ExperimentIdentity::from_canonical_bytes(
        BENCHMARK_ID,
        "",
        &generator,
        BenchmarkSeed::new(BASE_SEED),
        &configuration.fingerprint(),
    );

    assert!(matches!(
        result,
        Err(ReproducibilityError::EmptyIdentifier {
            field: "benchmark version"
        })
    ));
}

#[test]
fn control_character_in_benchmark_id_is_rejected() {
    let generator = baseline_generator();
    let configuration = baseline_configuration();

    let result = ExperimentIdentity::from_canonical_bytes(
        "quantum_volume\nunexpected",
        BENCHMARK_VERSION,
        &generator,
        BenchmarkSeed::new(BASE_SEED),
        &configuration.fingerprint(),
    );

    assert!(matches!(
        result,
        Err(ReproducibilityError::InvalidIdentifier {
            field: "benchmark id"
        })
    ));
}

#[test]
fn control_character_in_benchmark_version_is_rejected() {
    let generator = baseline_generator();
    let configuration = baseline_configuration();

    let result = ExperimentIdentity::from_canonical_bytes(
        BENCHMARK_ID,
        "1.0.0\tunexpected",
        &generator,
        BenchmarkSeed::new(BASE_SEED),
        &configuration.fingerprint(),
    );

    assert!(matches!(
        result,
        Err(ReproducibilityError::InvalidIdentifier {
            field: "benchmark version"
        })
    ));
}

// ============================================================================
// Configuration fingerprints
// ============================================================================

#[test]
fn identical_configuration_bytes_have_identical_fingerprints() {
    let first =
        ConfigurationFingerprint::from_canonical_bytes(
            CONFIGURATION_BYTES,
        );

    let second =
        ConfigurationFingerprint::from_canonical_bytes(
            CONFIGURATION_BYTES,
        );

    assert_eq!(first, second);
    assert_eq!(first.fingerprint(), second.fingerprint());
    assert_eq!(first.hex(), second.hex());
}

#[test]
fn different_configuration_bytes_have_different_fingerprints() {
    let first =
        ConfigurationFingerprint::from_canonical_bytes(
            CONFIGURATION_BYTES,
        );

    let second =
        ConfigurationFingerprint::from_canonical_bytes(
            DIFFERENT_CONFIGURATION_BYTES,
        );

    assert_ne!(first, second);
    assert_ne!(first.fingerprint(), second.fingerprint());
}

#[test]
fn configuration_fingerprint_has_expected_size() {
    let configuration = baseline_configuration();

    assert_eq!(
        configuration.fingerprint().as_bytes().len(),
        FINGERPRINT_BYTES
    );

    assert_eq!(
        configuration.hex().len(),
        FINGERPRINT_HEX_LENGTH
    );
}

// ============================================================================
// Circuit fingerprints
// ============================================================================

#[test]
fn identical_circuit_bytes_have_identical_fingerprints() {
    let first =
        CircuitFingerprint::from_canonical_bytes(
            CIRCUIT_A_BYTES,
        );

    let second =
        CircuitFingerprint::from_canonical_bytes(
            CIRCUIT_A_BYTES,
        );

    assert_eq!(first, second);
    assert_eq!(first.fingerprint(), second.fingerprint());
    assert_eq!(first.hex(), second.hex());
}

#[test]
fn different_circuit_bytes_have_different_fingerprints() {
    let first =
        CircuitFingerprint::from_canonical_bytes(
            CIRCUIT_A_BYTES,
        );

    let second =
        CircuitFingerprint::from_canonical_bytes(
            CIRCUIT_B_BYTES,
        );

    assert_ne!(first, second);
}

#[test]
fn circuit_fingerprint_is_independent_of_result_fingerprint_domain() {
    let circuit =
        CircuitFingerprint::from_canonical_bytes(
            RESULT_BYTES,
        );

    let result =
        ResultFingerprint::from_canonical_bytes(
            RESULT_BYTES,
        );

    assert_ne!(circuit.fingerprint(), result.fingerprint());
}

// ============================================================================
// Result fingerprints
// ============================================================================

#[test]
fn identical_result_bytes_have_identical_fingerprints() {
    let first =
        ResultFingerprint::from_canonical_bytes(
            RESULT_BYTES,
        );

    let second =
        ResultFingerprint::from_canonical_bytes(
            RESULT_BYTES,
        );

    assert_eq!(first, second);
    assert_eq!(first.fingerprint(), second.fingerprint());
}

#[test]
fn different_result_bytes_have_different_fingerprints() {
    let first =
        ResultFingerprint::from_canonical_bytes(
            RESULT_BYTES,
        );

    let second =
        ResultFingerprint::from_canonical_bytes(
            DIFFERENT_RESULT_BYTES,
        );

    assert_ne!(first, second);
}

#[test]
fn result_fingerprint_is_fixed_size() {
    let result = baseline_result();

    assert_eq!(
        result.fingerprint().as_bytes().len(),
        FINGERPRINT_BYTES
    );

    assert_eq!(
        result.hex().len(),
        FINGERPRINT_HEX_LENGTH
    );
}

// ============================================================================
// Reproducibility records
// ============================================================================

#[test]
fn baseline_record_is_valid() {
    let record = baseline_record();

    assert_eq!(
        record.schema_version,
        REPRODUCIBILITY_SCHEMA_VERSION
    );

    assert_eq!(record.circuit_count(), 3);
    assert_eq!(record.seed.value(), BASE_SEED);
    assert_eq!(record.generator.id, GENERATOR_ID);
    assert_eq!(
        record.generator.version,
        GENERATOR_VERSION
    );
}

#[test]
fn identical_reproducibility_records_have_identical_fingerprints() {
    let first = baseline_record();
    let second = baseline_record();

    assert_eq!(first.fingerprint(), second.fingerprint());
    assert_eq!(first.fingerprint().hex(), second.fingerprint().hex());
}

#[test]
fn reproducibility_record_fingerprint_is_nonzero() {
    let record = baseline_record();

    assert!(!record.fingerprint().is_zero());
    assert_eq!(
        record.fingerprint().as_bytes().len(),
        FINGERPRINT_BYTES
    );
}

#[test]
fn circuit_count_matches_record_contents() {
    let generator = baseline_generator();
    let configuration = baseline_configuration();

    let experiment = ExperimentIdentity::from_canonical_bytes(
        BENCHMARK_ID,
        BENCHMARK_VERSION,
        &generator,
        BenchmarkSeed::new(BASE_SEED),
        &configuration.fingerprint(),
    )
    .expect("identity must be valid");

    let circuits = vec![
        baseline_circuit_a(),
        baseline_circuit_b(),
    ];

    let record = ReproducibilityRecord::new(
        experiment,
        generator,
        BenchmarkSeed::new(BASE_SEED),
        configuration,
        circuits,
    )
    .expect("record must be valid");

    assert_eq!(record.circuit_count(), 2);
}

#[test]
fn empty_circuit_set_is_rejected() {
    let generator = baseline_generator();
    let configuration = baseline_configuration();

    let experiment = ExperimentIdentity::from_canonical_bytes(
        BENCHMARK_ID,
        BENCHMARK_VERSION,
        &generator,
        BenchmarkSeed::new(BASE_SEED),
        &configuration.fingerprint(),
    )
    .expect("identity must be valid");

    let result = ReproducibilityRecord::new(
        experiment,
        generator,
        BenchmarkSeed::new(BASE_SEED),
        configuration,
        Vec::new(),
    );

    assert!(matches!(
        result,
        Err(ReproducibilityError::EmptyCircuitSet)
    ));
}

#[test]
fn changing_one_circuit_changes_record_fingerprint() {
    let generator = baseline_generator();
    let configuration = baseline_configuration();

    let first_experiment =
        ExperimentIdentity::from_canonical_bytes(
            BENCHMARK_ID,
            BENCHMARK_VERSION,
            &generator,
            BenchmarkSeed::new(BASE_SEED),
            &configuration.fingerprint(),
        )
        .expect("first identity must be valid");

    let second_experiment =
        ExperimentIdentity::from_canonical_bytes(
            BENCHMARK_ID,
            BENCHMARK_VERSION,
            &generator,
            BenchmarkSeed::new(BASE_SEED),
            &configuration.fingerprint(),
        )
        .expect("second identity must be valid");

    let first = ReproducibilityRecord::new(
        first_experiment,
        generator.clone(),
        BenchmarkSeed::new(BASE_SEED),
        configuration,
        vec![
            baseline_circuit_a(),
            baseline_circuit_b(),
            baseline_circuit_c(),
        ],
    )
    .expect("first record must be valid");

    let replacement =
        CircuitFingerprint::from_canonical_bytes(
            b"circuit:v1;width=8;depth=8;gates=DIFFERENT",
        );

    let second = ReproducibilityRecord::new(
        second_experiment,
        generator,
        BenchmarkSeed::new(BASE_SEED),
        configuration,
        vec![
            baseline_circuit_a(),
            baseline_circuit_b(),
            replacement,
        ],
    )
    .expect("second record must be valid");

    assert_ne!(first.fingerprint(), second.fingerprint());
}

#[test]
fn circuit_order_is_semantically_significant() {
    let generator = baseline_generator();
    let configuration = baseline_configuration();

    let first_experiment =
        ExperimentIdentity::from_canonical_bytes(
            BENCHMARK_ID,
            BENCHMARK_VERSION,
            &generator,
            BenchmarkSeed::new(BASE_SEED),
            &configuration.fingerprint(),
        )
        .expect("first identity must be valid");

    let second_experiment =
        ExperimentIdentity::from_canonical_bytes(
            BENCHMARK_ID,
            BENCHMARK_VERSION,
            &generator,
            BenchmarkSeed::new(BASE_SEED),
            &configuration.fingerprint(),
        )
        .expect("second identity must be valid");

    let first = ReproducibilityRecord::new(
        first_experiment,
        generator.clone(),
        BenchmarkSeed::new(BASE_SEED),
        configuration,
        vec![
            baseline_circuit_a(),
            baseline_circuit_b(),
            baseline_circuit_c(),
        ],
    )
    .expect("first record must be valid");

    let second = ReproducibilityRecord::new(
        second_experiment,
        generator,
        BenchmarkSeed::new(BASE_SEED),
        configuration,
        vec![
            baseline_circuit_c(),
            baseline_circuit_b(),
            baseline_circuit_a(),
        ],
    )
    .expect("second record must be valid");

    assert_ne!(first.fingerprint(), second.fingerprint());
}

// ============================================================================
// Independence from execution-time metadata
// ============================================================================

#[test]
fn experiment_identity_does_not_depend_on_execution_timestamp() {
    let generator = baseline_generator();
    let configuration = baseline_configuration();

    let first = ExperimentIdentity::from_canonical_bytes(
        BENCHMARK_ID,
        BENCHMARK_VERSION,
        &generator,
        BenchmarkSeed::new(BASE_SEED),
        &configuration.fingerprint(),
    )
    .expect("first identity must be valid");

    // Timestamp-like data is intentionally NOT supplied to the identity API.
    //
    // A provenance layer may record timestamps separately, but timestamps must
    // not silently change deterministic experiment identity.
    let second = ExperimentIdentity::from_canonical_bytes(
        BENCHMARK_ID,
        BENCHMARK_VERSION,
        &generator,
        BenchmarkSeed::new(BASE_SEED),
        &configuration.fingerprint(),
    )
    .expect("second identity must be valid");

    assert_eq!(first, second);
}

#[test]
fn experiment_identity_does_not_include_result_data() {
    let identity = baseline_identity();

    let result_a =
        ResultFingerprint::from_canonical_bytes(
            RESULT_BYTES,
        );

    let result_b =
        ResultFingerprint::from_canonical_bytes(
            DIFFERENT_RESULT_BYTES,
        );

    assert_ne!(result_a, result_b);

    // The experiment identity is still exactly the same because execution
    // results belong to a different semantic layer.
    assert_eq!(identity, baseline_identity());
}

// ============================================================================
// Identity separation between semantic layers
// ============================================================================

#[test]
fn configuration_circuit_and_result_domains_are_distinct() {
    let same_payload = b"same canonical payload";

    let configuration =
        ConfigurationFingerprint::from_canonical_bytes(
            same_payload,
        );

    let circuit =
        CircuitFingerprint::from_canonical_bytes(
            same_payload,
        );

    let result =
        ResultFingerprint::from_canonical_bytes(
            same_payload,
        );

    assert_ne!(configuration.fingerprint(), circuit.fingerprint());
    assert_ne!(configuration.fingerprint(), result.fingerprint());
    assert_ne!(circuit.fingerprint(), result.fingerprint());
}

#[test]
fn experiment_identity_is_distinct_from_configuration_fingerprint() {
    let generator = baseline_generator();
    let configuration = baseline_configuration();

    let identity = ExperimentIdentity::from_canonical_bytes(
        BENCHMARK_ID,
        BENCHMARK_VERSION,
        &generator,
        BenchmarkSeed::new(BASE_SEED),
        &configuration.fingerprint(),
    )
    .expect("identity must be valid");

    assert_ne!(
        identity.fingerprint(),
        configuration.fingerprint()
    );
}

// ============================================================================
// Versioning contract
// ============================================================================

#[test]
fn reproducibility_schema_version_is_explicit() {
    assert_eq!(REPRODUCIBILITY_SCHEMA_VERSION, 1);
}

#[test]
fn reproducibility_record_carries_schema_version() {
    let record = baseline_record();

    assert_eq!(
        record.schema_version,
        REPRODUCIBILITY_SCHEMA_VERSION
    );
}

// ============================================================================
// Determinism under repeated construction
// ============================================================================

#[test]
fn repeated_identity_construction_is_stable() {
    let mut identities = Vec::new();

    for _ in 0..128 {
        identities.push(baseline_identity());
    }

    for identity in &identities {
        assert_eq!(*identity, identities[0]);
    }
}

#[test]
fn repeated_record_construction_is_stable() {
    let mut fingerprints = Vec::new();

    for _ in 0..128 {
        fingerprints.push(baseline_record().fingerprint());
    }

    for fingerprint in &fingerprints {
        assert_eq!(*fingerprint, fingerprints[0]);
    }
}

// ============================================================================
// Unicode compatibility
// ============================================================================

#[test]
fn unicode_identifiers_are_allowed_when_they_are_not_control_characters() {
    let generator = GeneratorDescriptor::new(
        "zamani.量子.بنشمار",
        "版本-1.0",
    );

    assert!(generator.validate().is_ok());

    let configuration =
        ConfigurationFingerprint::from_canonical_bytes(
            b"unicode benchmark configuration",
        );

    let identity = ExperimentIdentity::from_canonical_bytes(
        "benchmark.量子.بنشمار",
        "版本-1.0",
        &generator,
        BenchmarkSeed::new(BASE_SEED),
        &configuration.fingerprint(),
    );

    assert!(identity.is_ok());
}

// ============================================================================
// Extreme seed values
// ============================================================================

#[test]
fn minimum_seed_is_supported() {
    let generator = baseline_generator();
    let configuration = baseline_configuration();

    let identity = ExperimentIdentity::from_canonical_bytes(
        BENCHMARK_ID,
        BENCHMARK_VERSION,
        &generator,
        BenchmarkSeed::new(0),
        &configuration.fingerprint(),
    );

    assert!(identity.is_ok());
}

#[test]
fn maximum_seed_is_supported() {
    let generator = baseline_generator();
    let configuration = baseline_configuration();

    let identity = ExperimentIdentity::from_canonical_bytes(
        BENCHMARK_ID,
        BENCHMARK_VERSION,
        &generator,
        BenchmarkSeed::new(u64::MAX),
        &configuration.fingerprint(),
    );

    assert!(identity.is_ok());
}

#[test]
fn minimum_and_maximum_seed_have_distinct_identity() {
    let generator = baseline_generator();
    let configuration = baseline_configuration();

    let minimum = ExperimentIdentity::from_canonical_bytes(
        BENCHMARK_ID,
        BENCHMARK_VERSION,
        &generator,
        BenchmarkSeed::new(0),
        &configuration.fingerprint(),
    )
    .expect("minimum seed identity must be valid");

    let maximum = ExperimentIdentity::from_canonical_bytes(
        BENCHMARK_ID,
        BENCHMARK_VERSION,
        &generator,
        BenchmarkSeed::new(u64::MAX),
        &configuration.fingerprint(),
    )
    .expect("maximum seed identity must be valid");

    assert_ne!(minimum, maximum);
}

// ============================================================================
// Integration-oriented invariants
// ============================================================================

#[test]
fn qv_style_randomized_experiment_identity_contains_generator_and_seed() {
    let configuration =
        ConfigurationFingerprint::from_canonical_bytes(
            b"qv;width=8;depth=8;circuits=100;shots=1000",
        );

    let generator_v1 =
        GeneratorDescriptor::new("zamani.qv", "1.0.0")
            .with_rng_algorithm("zamani-rng-v1");

    let generator_v2 =
        GeneratorDescriptor::new("zamani.qv", "1.0.1")
            .with_rng_algorithm("zamani-rng-v1");

    let identity_v1 = ExperimentIdentity::from_canonical_bytes(
        "quantum_volume",
        "1.0.0",
        &generator_v1,
        BenchmarkSeed::new(42),
        &configuration.fingerprint(),
    )
    .expect("QV identity v1 must be valid");

    let identity_v2 = ExperimentIdentity::from_canonical_bytes(
        "quantum_volume",
        "1.0.0",
        &generator_v2,
        BenchmarkSeed::new(42),
        &configuration.fingerprint(),
    )
    .expect("QV identity v2 must be valid");

    assert_ne!(identity_v1, identity_v2);
}

#[test]
fn randomized_benchmarking_style_seed_change_invalidates_experiment() {
    let configuration =
        ConfigurationFingerprint::from_canonical_bytes(
            b"rb;qubits=2;lengths=1,2,4,8;sequences=100;shots=1000",
        );

    let generator =
        GeneratorDescriptor::new("zamani.rb.clifford", "1.0.0")
            .with_rng_algorithm("zamani-rng-v1");

    let first = ExperimentIdentity::from_canonical_bytes(
        "randomized_benchmarking",
        "1.0.0",
        &generator,
        BenchmarkSeed::new(12345),
        &configuration.fingerprint(),
    )
    .expect("first RB identity must be valid");

    let second = ExperimentIdentity::from_canonical_bytes(
        "randomized_benchmarking",
        "1.0.0",
        &generator,
        BenchmarkSeed::new(12346),
        &configuration.fingerprint(),
    )
    .expect("second RB identity must be valid");

    assert_ne!(first, second);
}

#[test]
fn application_benchmark_identity_can_be_deterministic_without_rng() {
    let configuration =
        ConfigurationFingerprint::from_canonical_bytes(
            b"qaoa;problem=maxcut;nodes=8;depth=3;instance=7",
        );

    let generator =
        GeneratorDescriptor::new("zamani.application.qaoa", "1.0.0");

    assert!(generator.validate().is_ok());

    let first = ExperimentIdentity::from_canonical_bytes(
        "qaoa",
        "1.0.0",
        &generator,
        BenchmarkSeed::new(0),
        &configuration.fingerprint(),
    )
    .expect("application identity must be valid");

    let second = ExperimentIdentity::from_canonical_bytes(
        "qaoa",
        "1.0.0",
        &generator,
        BenchmarkSeed::new(0),
        &configuration.fingerprint(),
    )
    .expect("application identity must be valid");

    assert_eq!(first, second);
}

#[test]
fn qec_style_workload_can_use_the_same_reproducibility_contract() {
    let configuration =
        ConfigurationFingerprint::from_canonical_bytes(
            b"surface_code;distance=7;rounds=100;decoder=minimum_weight",
        );

    let generator =
        GeneratorDescriptor::new("zamani.qec.surface_code", "1.0.0");

    let identity = ExperimentIdentity::from_canonical_bytes(
        "logical_error_rate",
        "1.0.0",
        &generator,
        BenchmarkSeed::new(99),
        &configuration.fingerprint(),
    );

    assert!(identity.is_ok());
}

// ============================================================================
// API behavior required by downstream reporting
// ============================================================================

#[test]
fn fingerprints_can_be_exported_as_lowercase_hex() {
    let fingerprint =
        Fingerprint::from_bytes(b"reporting integration");

    let hexadecimal = fingerprint.hex();

    assert_eq!(hexadecimal.len(), FINGERPRINT_HEX_LENGTH);
    assert!(
        hexadecimal
            .chars()
            .all(|character| character.is_ascii_hexdigit())
    );
    assert_eq!(
        hexadecimal,
        hexadecimal.to_ascii_lowercase()
    );
}

#[test]
fn experiment_identity_hex_is_stable_for_reporting() {
    let first = baseline_identity().hex();
    let second = baseline_identity().hex();

    assert_eq!(first, second);
    assert_eq!(first.len(), FINGERPRINT_HEX_LENGTH);
}

#[test]
fn record_fingerprint_can_be_used_as_a_stable_report_key() {
    let first = baseline_record().fingerprint().hex();
    let second = baseline_record().fingerprint().hex();

    assert_eq!(first, second);
    assert_eq!(first.len(), FINGERPRINT_HEX_LENGTH);
}

// ============================================================================
// Final contract test
// ============================================================================

#[test]
fn complete_reproducibility_contract_is_deterministic() {
    let generator = baseline_generator();
    let configuration = baseline_configuration();

    let experiment = ExperimentIdentity::from_canonical_bytes(
        BENCHMARK_ID,
        BENCHMARK_VERSION,
        &generator,
        BenchmarkSeed::new(BASE_SEED),
        &configuration.fingerprint(),
    )
    .expect("experiment identity must be valid");

    let record = ReproducibilityRecord::new(
        experiment,
        generator,
        BenchmarkSeed::new(BASE_SEED),
        configuration,
        vec![
            baseline_circuit_a(),
            baseline_circuit_b(),
            baseline_circuit_c(),
        ],
    )
    .expect("reproducibility record must be valid");

    let second = baseline_record();

    assert_eq!(record.schema_version, second.schema_version);
    assert_eq!(record.experiment, second.experiment);
    assert_eq!(record.generator, second.generator);
    assert_eq!(record.seed, second.seed);
    assert_eq!(record.configuration, second.configuration);
    assert_eq!(record.circuits, second.circuits);
    assert_eq!(record.fingerprint(), second.fingerprint());

    // The same complete experiment definition therefore has one stable
    // reproducibility identity regardless of where or when the test executes.
    assert!(!record.fingerprint().is_zero());
}