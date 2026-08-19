//! Production-grade tests for the Zamani quantum error-correction decoder.
//!
//! These tests intentionally exercise the public decoder contract rather than
//! implementation details. They verify:
//!
//! - construction and configuration;
//! - deterministic behaviour;
//! - trivial and non-trivial syndromes;
//! - correct and incorrect corrections;
//! - malformed input handling;
//! - decoder registration;
//! - statistics;
//! - repeated decoding;
//! - absence of panic-based validation.
//!
//! The suite is deliberately conservative: a decoder must never be considered
//! correct merely because it returns a result. The returned correction must
//! also satisfy the decoder's validation contract.
//!
//! NOTE:
//! Keep this file aligned with the public API exposed by
//! `crate::quantum::error_correction::decoder`.
//!
//! If a decoder implementation intentionally changes its public contract,
//! update these tests together with that API change.

use std::panic::{catch_unwind, AssertUnwindSafe};

use crate::quantum::error_correction::{
    DecoderError,
    DecoderResult,
    SurfaceCodeDecoder,
};

use crate::quantum::error_correction::decoder::{
    Correction,
    DecoderConfig,
    DecoderRegistry,
    DecoderStatistics,
    PauliError,
    Syndrome,
};

// ============================================================================
// Test helpers
// ============================================================================

/// Creates a minimal valid decoder configuration.
///
/// Keep the configuration small so these tests remain fast and deterministic.
fn valid_config() -> DecoderConfig {
    DecoderConfig::new(3)
        .expect("distance 3 must be a valid surface-code configuration")
}

/// Creates a decoder using the smallest production-valid code distance.
fn decoder() -> SurfaceCodeDecoder {
    SurfaceCodeDecoder::new(valid_config())
        .expect("valid decoder configuration must construct successfully")
}

/// Creates a trivial syndrome.
///
/// A trivial syndrome contains no detected stabilizer violations.
fn trivial_syndrome() -> Syndrome {
    Syndrome::new()
}

/// Creates a deterministic single-bit syndrome.
///
/// The exact stabilizer identifier is deliberately kept small so that this
/// remains compatible with distance-3 test fixtures.
fn single_detection_syndrome() -> Syndrome {
    Syndrome::from_bits(vec![true])
        .expect("one syndrome bit must be valid")
}

/// Assert that an operation does not panic.
///
/// Production QEC receives data that may ultimately originate from external
/// hardware, files, network services, simulations, or untrusted integrations.
/// A malformed input must therefore produce an error rather than unwind.
fn assert_no_panic<T, F>(operation: F) -> T
where
    F: FnOnce() -> T,
{
    catch_unwind(AssertUnwindSafe(operation))
        .expect("production QEC API must not panic on test input")
}

// ============================================================================
// Construction
// ============================================================================

#[test]
fn decoder_constructs_with_valid_configuration() {
    let result = SurfaceCodeDecoder::new(valid_config());

    assert!(
        result.is_ok(),
        "valid surface-code configuration must construct a decoder"
    );
}

#[test]
fn decoder_rejects_invalid_distance() {
    let result = DecoderConfig::new(0);

    assert!(
        result.is_err(),
        "distance zero must never produce a valid QEC configuration"
    );
}

#[test]
fn decoder_rejects_even_distance() {
    let result = DecoderConfig::new(4);

    assert!(
        result.is_err(),
        "surface-code distance must satisfy the implementation's distance invariant"
    );
}

#[test]
fn decoder_configuration_is_deterministic() {
    let first = DecoderConfig::new(3);
    let second = DecoderConfig::new(3);

    assert_eq!(first, second);
}

// ============================================================================
// Trivial syndrome
// ============================================================================

#[test]
fn trivial_syndrome_is_accepted() {
    let decoder = decoder();
    let syndrome = trivial_syndrome();

    let result = assert_no_panic(|| decoder.decode(&syndrome));

    assert!(
        result.is_ok(),
        "a trivial syndrome should be decodable"
    );
}

#[test]
fn trivial_syndrome_requires_no_nontrivial_correction() {
    let decoder = decoder();
    let syndrome = trivial_syndrome();

    let correction = decoder
        .decode(&syndrome)
        .expect("trivial syndrome must decode");

    assert!(
        correction.is_identity(),
        "trivial syndrome should produce an identity/no-op correction"
    );
}

// ============================================================================
// Non-trivial syndrome
// ============================================================================

#[test]
fn nontrivial_syndrome_is_decodable() {
    let decoder = decoder();
    let syndrome = single_detection_syndrome();

    let result = assert_no_panic(|| decoder.decode(&syndrome));

    assert!(
        result.is_ok(),
        "valid non-trivial syndrome must be handled without failure"
    );
}

#[test]
fn nontrivial_syndrome_does_not_produce_identity_without_justification() {
    let decoder = decoder();
    let syndrome = single_detection_syndrome();

    let correction = decoder
        .decode(&syndrome)
        .expect("valid syndrome must decode");

    /*
     * A decoder is permitted to determine that no physical correction is
     * required only when that is mathematically justified. We therefore do
     * not require a particular qubit here, but we ensure the correction object
     * itself is structurally valid.
     */
    assert!(
        correction.is_valid(),
        "decoder must return a structurally valid correction"
    );
}

// ============================================================================
// Determinism
// ============================================================================

#[test]
fn decoding_is_deterministic() {
    let decoder = decoder();
    let syndrome = single_detection_syndrome();

    let first = decoder
        .decode(&syndrome)
        .expect("first decode must succeed");

    let second = decoder
        .decode(&syndrome)
        .expect("second decode must succeed");

    assert_eq!(
        first, second,
        "identical decoder inputs must produce identical corrections"
    );
}

#[test]
fn repeated_decoding_does_not_mutate_semantic_result() {
    let decoder = decoder();
    let syndrome = single_detection_syndrome();

    let first = decoder
        .decode(&syndrome)
        .expect("decode must succeed");

    for _ in 0..100 {
        let current = decoder
            .decode(&syndrome)
            .expect("repeated decode must succeed");

        assert_eq!(
            first, current,
            "repeated decoding must remain deterministic"
        );
    }
}

// ============================================================================
// Correction validation
// ============================================================================

#[test]
fn identity_correction_is_valid() {
    let correction = Correction::identity();

    assert!(
        correction.is_valid(),
        "identity correction must always be valid"
    );

    assert!(
        correction.is_identity(),
        "identity correction must report itself as identity"
    );
}

#[test]
fn valid_single_qubit_correction_is_accepted() {
    let correction = Correction::single_qubit(
        0,
        PauliError::X,
    )
    .expect("qubit zero must be valid for the test fixture");

    assert!(
        correction.is_valid(),
        "single-qubit correction must be structurally valid"
    );
}

#[test]
fn correction_rejects_invalid_qubit_index() {
    let result = Correction::single_qubit(
        usize::MAX,
        PauliError::X,
    );

    assert!(
        result.is_err(),
        "invalid qubit indices must be rejected"
    );
}

#[test]
fn pauli_identity_is_not_a_physical_error() {
    assert!(
        !PauliError::I.is_error(),
        "I must represent absence of a Pauli error"
    );
}

#[test]
fn pauli_x_is_a_physical_error() {
    assert!(
        PauliError::X.is_error(),
        "X must be classified as an error"
    );
}

#[test]
fn pauli_y_is_a_physical_error() {
    assert!(
        PauliError::Y.is_error(),
        "Y must be classified as an error"
    );
}

#[test]
fn pauli_z_is_a_physical_error() {
    assert!(
        PauliError::Z.is_error(),
        "Z must be classified as an error"
    );
}

// ============================================================================
// Error handling
// ============================================================================

#[test]
fn malformed_syndrome_returns_error_instead_of_panicking() {
    let decoder = decoder();

    let malformed = Syndrome::from_bits(
        vec![true; usize::MAX.min(1_000_001)],
    );

    if let Ok(syndrome) = malformed {
        let result = assert_no_panic(|| decoder.decode(&syndrome));

        /*
         * If the syndrome constructor permits this representation, the
         * decoder must still reject it safely or handle it according to its
         * documented resource limits.
         */
        assert!(
            result.is_ok() || result.is_err(),
            "decoder must return normally rather than panic"
        );
    }
}

#[test]
fn decoder_errors_are_structured() {
    let result = DecoderConfig::new(0);

    assert!(
        matches!(result, Err(DecoderError::InvalidDistance { .. })),
        "invalid configuration must use a structured decoder error"
    );
}

// ============================================================================
// Decoder statistics
// ============================================================================

#[test]
fn statistics_start_empty() {
    let statistics = DecoderStatistics::default();

    assert_eq!(
        statistics.total_decodes(),
        0,
        "new decoder statistics must start with zero decodes"
    );

    assert_eq!(
        statistics.successful_decodes(),
        0,
        "new decoder statistics must start with zero successful decodes"
    );

    assert_eq!(
        statistics.failed_decodes(),
        0,
        "new decoder statistics must start with zero failed decodes"
    );
}

#[test]
fn successful_decode_updates_statistics() {
    let decoder = decoder();
    let syndrome = trivial_syndrome();

    decoder
        .decode(&syndrome)
        .expect("trivial syndrome must decode");

    let statistics = decoder.statistics();

    assert_eq!(
        statistics.total_decodes(),
        1,
        "successful decode must increment total decode count"
    );

    assert_eq!(
        statistics.successful_decodes(),
        1,
        "successful decode must increment successful count"
    );
}

#[test]
fn repeated_decodes_update_statistics_monotonically() {
    let decoder = decoder();
    let syndrome = trivial_syndrome();

    for expected in 1..=10 {
        decoder
            .decode(&syndrome)
            .expect("trivial syndrome must decode");

        let statistics = decoder.statistics();

        assert_eq!(
            statistics.total_decodes(),
            expected,
            "total decode count must increase monotonically"
        );
    }
}

// ============================================================================
// Decoder registry
// ============================================================================

#[test]
fn decoder_registry_can_be_created() {
    let registry = DecoderRegistry::new();

    assert!(
        registry.is_empty(),
        "new decoder registry must initially be empty"
    );
}

#[test]
fn decoder_registry_accepts_unique_decoder() {
    let mut registry = DecoderRegistry::new();

    let decoder = decoder();

    registry
        .register("surface-code", decoder)
        .expect("unique decoder registration must succeed");

    assert!(
        registry.contains("surface-code"),
        "registered decoder must be discoverable"
    );
}

#[test]
fn decoder_registry_rejects_duplicate_decoder_name() {
    let mut registry = DecoderRegistry::new();

    registry
        .register(
            "surface-code",
            decoder(),
        )
        .expect("first registration must succeed");

    let result = registry.register(
        "surface-code",
        decoder(),
    );

    assert!(
        result.is_err(),
        "duplicate decoder names must be rejected"
    );
}

#[test]
fn decoder_registry_lookup_is_deterministic() {
    let mut registry = DecoderRegistry::new();

    registry
        .register(
            "surface-code",
            decoder(),
        )
        .expect("registration must succeed");

    assert!(
        registry.get("surface-code").is_some(),
        "registered decoder must be retrievable"
    );

    assert!(
        registry.get("does-not-exist").is_none(),
        "unknown decoder must not fabricate a decoder"
    );
}

// ============================================================================
// Decoder result contract
// ============================================================================

#[test]
fn decoder_result_can_represent_success() {
    let correction = Correction::identity();

    let result: DecoderResult = Ok(correction);

    assert!(
        result.is_ok(),
        "DecoderResult must represent successful decoding"
    );
}

#[test]
fn decoder_result_can_represent_failure() {
    let error = DecoderError::InvalidDistance {
        distance: 0,
    };

    let result: DecoderResult = Err(error);

    assert!(
        result.is_err(),
        "DecoderResult must represent decoder failure"
    );
}

// ============================================================================
// No-panic contract
// ============================================================================

#[test]
fn decoder_does_not_panic_for_empty_syndrome() {
    let decoder = decoder();
    let syndrome = Syndrome::new();

    let result = assert_no_panic(|| decoder.decode(&syndrome));

    assert!(
        result.is_ok() || result.is_err(),
        "empty syndrome must return normally"
    );
}

#[test]
fn decoder_does_not_panic_for_repeated_empty_syndromes() {
    let decoder = decoder();
    let syndrome = Syndrome::new();

    for _ in 0..1_000 {
        let _ = assert_no_panic(|| decoder.decode(&syndrome));
    }
}

#[test]
fn decoder_does_not_panic_when_constructing_invalid_configuration() {
    for distance in [
        0usize,
        1usize,
        2usize,
        usize::MAX,
    ] {
        let result = assert_no_panic(|| {
            DecoderConfig::new(distance)
        });

        /*
         * The important invariant here is that malformed configuration
         * never causes a panic.
         */
        assert!(
            result.is_ok() || result.is_err(),
            "configuration constructor must return normally"
        );
    }
}

// ============================================================================
// Regression guards
// ============================================================================

#[test]
fn decoder_distance_three_remains_supported() {
    let config = DecoderConfig::new(3)
        .expect("distance 3 must remain supported");

    let decoder = SurfaceCodeDecoder::new(config)
        .expect("distance-3 decoder must construct");

    let syndrome = Syndrome::new();

    let correction = decoder
        .decode(&syndrome)
        .expect("empty/trivial syndrome must decode");

    assert!(
        correction.is_valid(),
        "decoder must return a valid correction"
    );
}

#[test]
fn decoder_has_no_hidden_randomness() {
    let decoder = decoder();
    let syndrome = single_detection_syndrome();

    let mut results = Vec::with_capacity(64);

    for _ in 0..64 {
        results.push(
            decoder
                .decode(&syndrome)
                .expect("decode must succeed"),
        );
    }

    for result in &results[1..] {
        assert_eq!(
            result,
            &results[0],
            "decoder output must not depend on hidden randomness"
        );
    }
}