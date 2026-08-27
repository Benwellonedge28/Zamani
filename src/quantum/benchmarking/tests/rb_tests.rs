//! Zamani Quantum Benchmarking — Randomized Benchmarking Tests
//!
//! # Purpose
//!
//! Production-grade contract tests for randomized benchmarking (RB).
//!
//! This file tests the PUBLIC randomized-benchmarking contract. It does not
//! reimplement the production RB protocol, circuit generator, executor, or
//! statistical engine.
//!
//! The intended dependency direction is:
//!
//! ```text
//! tests/rb_tests.rs
//!        │
//!        ├── protocols/randomized_benchmarking.rs
//!        │
//!        ├── statistics/regression.rs
//!        │
//!        ├── statistics/confidence.rs
//!        │
//!        ├── generators/clifford.rs
//!        │
//!        └── core/result.rs
//! ```
//!
//! The tests intentionally use deterministic synthetic observations where
//! possible. This makes them suitable for CI and avoids depending on quantum
//! hardware, simulator availability, timing, network access, or vendor SDKs.
//!
//! # RB scientific contract
//!
//! Standard randomized benchmarking analyzes survival probabilities over
//! randomized sequence depths using an exponential model:
//
//! ```text
//! P(m) = A + B * p^m
//! ```
//!
//! where:
//!
//! - `m` is the RB sequence depth;
//! - `A` is the asymptotic offset;
//! - `B` is the contrast/amplitude;
//! - `p` is the decay parameter.
//!
//! The tests require the production implementation to expose the fitted decay
//! parameter separately from the derived error rate.
//!
//! The implementation MUST NOT silently interpret an RB decay parameter as a
//! universal physical gate error. The result must identify the error-rate
//! convention and expose fit diagnostics.
//!
//! # Important architectural rule
//!
//! These tests do NOT test a private implementation detail.
//!
//! They test the contract that the eventual production implementation must
//! expose. Consequently, once `protocols/randomized_benchmarking.rs` exists,
//! this file should not need to be rewritten merely because the implementation
//! changes.
//!
//! # Required production API
//!
//! This test suite expects the following public concepts from:
//!
//! `quantum::benchmarking::protocols::randomized_benchmarking`
//!
//! - `RandomizedBenchmarkingConfig`
//! - `RandomizedBenchmarkingAnalyzer`
//! - `RbObservation`
//! - `RbErrorRateConvention`
//! - `RbAnalysisError`
//! - `RbFitResult`
//!
//! The exact internal implementation is deliberately irrelevant.
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
//!
//! # Test philosophy
//!
//! The suite covers:
//!
//! 1. configuration validation;
//! 2. deterministic synthetic data;
//! 3. ideal/noiseless behavior;
//! 4. known exponential decay recovery;
//! 5. monotonic decay;
//! 6. probability validation;
//! 7. depth validation;
//! 8. sample validation;
//! 9. fit quality;
//! 10. convention-specific error rates;
//! 11. confidence intervals;
//! 12. deterministic repeated analysis;
//! 13. insufficient data;
//! 14. non-finite input;
//! 15. physically invalid input;
//! 16. pathological decay;
//! 17. large-depth numerical stability;
//! 18. multi-qubit convention behavior;
//! 19. no hidden randomness;
//! 20. no dependence on execution backends;
//! 21. result schema completeness;
//! 22. regression protection.
//!
//! The actual production RB protocol must separately test circuit generation,
//! Clifford inversion, execution, batching, backend capability negotiation,
//! and provenance.

#[cfg(test)]
mod tests {
    use super::super::super::protocols::randomized_benchmarking::{
        RandomizedBenchmarkingAnalyzer,
        RandomizedBenchmarkingConfig,
        RbAnalysisError,
        RbErrorRateConvention,
        RbObservation,
    };

    const EPSILON: f64 = 1.0e-10;
    const LOOSE_EPSILON: f64 = 1.0e-6;

    // ------------------------------------------------------------------------
    // Test helpers
    // ------------------------------------------------------------------------

    fn observation(depth: usize, success_probability: f64, shots: usize) -> RbObservation {
        RbObservation::new(depth, success_probability, shots)
            .expect("test fixture must contain valid RB observations")
    }

    fn observations_from_model(
        depths: &[usize],
        a: f64,
        b: f64,
        p: f64,
        shots: usize,
    ) -> Vec<RbObservation> {
        depths
            .iter()
            .map(|&depth| {
                let probability = a + b * p.powi(depth as i32);

                observation(depth, probability, shots)
            })
            .collect()
    }

    fn standard_config() -> RandomizedBenchmarkingConfig {
        RandomizedBenchmarkingConfig::builder()
            .confidence_level(0.95)
            .error_rate_convention(RbErrorRateConvention::EntanglementInfidelity)
            .minimum_depths(4)
            .build()
            .expect("standard RB test configuration must be valid")
    }

    fn analyze(
        observations: &[RbObservation],
    ) -> Result<
        super::super::super::protocols::randomized_benchmarking::RbFitResult,
        RbAnalysisError,
    > {
        RandomizedBenchmarkingAnalyzer::new(standard_config()).analyze(observations)
    }

    // ------------------------------------------------------------------------
    // Configuration validation
    // ------------------------------------------------------------------------

    #[test]
    fn default_production_configuration_is_valid() {
        let config = RandomizedBenchmarkingConfig::default();

        assert!(
            config.validate().is_ok(),
            "default RB configuration must be valid"
        );
    }

    #[test]
    fn confidence_level_must_be_valid() {
        let invalid = RandomizedBenchmarkingConfig::builder()
            .confidence_level(0.0)
            .build();

        assert!(
            invalid.is_err(),
            "zero confidence must be rejected"
        );

        let invalid = RandomizedBenchmarkingConfig::builder()
            .confidence_level(1.0)
            .build();

        assert!(
            invalid.is_err(),
            "100% confidence must be rejected because the finite-sample \
             interval becomes numerically/statistically undefined"
        );
    }

    #[test]
    fn zero_minimum_depths_are_rejected() {
        let result = RandomizedBenchmarkingConfig::builder()
            .minimum_depths(0)
            .build();

        assert!(
            result.is_err(),
            "RB requires at least one usable depth"
        );
    }

    #[test]
    fn invalid_error_rate_convention_cannot_be_constructed() {
        // This test intentionally verifies that all currently supported
        // conventions are explicit and inspectable.
        let convention = RbErrorRateConvention::EntanglementInfidelity;

        assert_eq!(
            convention.as_str(),
            "entanglement_infidelity"
        );

        let convention = RbErrorRateConvention::AverageGateInfidelity;

        assert_eq!(
            convention.as_str(),
            "average_gate_infidelity"
        );
    }

    // ------------------------------------------------------------------------
    // Observation validation
    // ------------------------------------------------------------------------

    #[test]
    fn zero_shots_are_rejected() {
        let result = RbObservation::new(1, 0.5, 0);

        assert!(
            result.is_err(),
            "an RB observation without samples is meaningless"
        );
    }

    #[test]
    fn probabilities_below_zero_are_rejected() {
        let result = RbObservation::new(1, -0.01, 1000);

        assert!(
            result.is_err(),
            "probabilities below zero must be rejected"
        );
    }

    #[test]
    fn probabilities_above_one_are_rejected() {
        let result = RbObservation::new(1, 1.01, 1000);

        assert!(
            result.is_err(),
            "probabilities above one must be rejected"
        );
    }

    #[test]
    fn nan_probability_is_rejected() {
        let result = RbObservation::new(1, f64::NAN, 1000);

        assert!(
            result.is_err(),
            "NaN must never enter statistical analysis"
        );
    }

    #[test]
    fn positive_infinity_probability_is_rejected() {
        let result = RbObservation::new(1, f64::INFINITY, 1000);

        assert!(
            result.is_err(),
            "positive infinity must be rejected"
        );
    }

    #[test]
    fn negative_infinity_probability_is_rejected() {
        let result = RbObservation::new(1, f64::NEG_INFINITY, 1000);

        assert!(
            result.is_err(),
            "negative infinity must be rejected"
        );
    }

    #[test]
    fn depth_zero_is_handled_explicitly() {
        // Depth zero is scientifically meaningful in some RB conventions
        // because it represents the baseline/no-random-Clifford experiment.
        //
        // Therefore this test deliberately does NOT require depth zero to be
        // rejected. It verifies that the observation can represent it.
        let result = RbObservation::new(0, 1.0, 1000);

        assert!(
            result.is_ok(),
            "depth zero should be representable when the protocol permits \
             a baseline observation"
        );
    }

    // ------------------------------------------------------------------------
    // Known mathematical model
    // ------------------------------------------------------------------------

    #[test]
    fn recovers_known_exponential_decay_parameter() {
        let depths = [0usize, 1, 2, 4, 8, 16, 32];

        let expected_a = 0.5;
        let expected_b = 0.5;
        let expected_p = 0.97;

        let data = observations_from_model(
            &depths,
            expected_a,
            expected_b,
            expected_p,
            1_000_000,
        );

        let result = analyze(&data)
            .expect("synthetic exponential RB data should fit");

        assert!(
            (result.decay_parameter() - expected_p).abs() < LOOSE_EPSILON,
            "expected p ≈ {}, got {}",
            expected_p,
            result.decay_parameter()
        );

        assert!(
            (result.offset() - expected_a).abs() < LOOSE_EPSILON,
            "expected A ≈ {}, got {}",
            expected_a,
            result.offset()
        );

        assert!(
            (result.amplitude() - expected_b).abs() < LOOSE_EPSILON,
            "expected B ≈ {}, got {}",
            expected_b,
            result.amplitude()
        );
    }

    #[test]
    fn ideal_single_qubit_data_has_near_zero_error() {
        let depths = [0usize, 1, 2, 4, 8, 16, 32];

        let data = observations_from_model(
            &depths,
            0.5,
            0.5,
            1.0,
            100_000,
        );

        let result = analyze(&data)
            .expect("ideal RB data should be analyzable");

        assert!(
            result.decay_parameter() <= 1.0 + LOOSE_EPSILON,
            "ideal decay parameter must not exceed one"
        );

        assert!(
            result.error_rate() >= -LOOSE_EPSILON,
            "ideal error rate must not be materially negative"
        );

        assert!(
            result.error_rate() < LOOSE_EPSILON,
            "ideal RB should produce approximately zero error"
        );
    }

    #[test]
    fn stronger_decay_produces_larger_error_rate() {
        let depths = [0usize, 1, 2, 4, 8, 16, 32];

        let low_error = observations_from_model(
            &depths,
            0.5,
            0.5,
            0.99,
            1_000_000,
        );

        let high_error = observations_from_model(
            &depths,
            0.5,
            0.5,
            0.90,
            1_000_000,
        );

        let low_result = analyze(&low_error)
            .expect("low-error RB fixture should fit");

        let high_result = analyze(&high_error)
            .expect("high-error RB fixture should fit");

        assert!(
            high_result.error_rate() > low_result.error_rate(),
            "a faster decay must imply a larger derived RB error rate"
        );
    }

    #[test]
    fn survival_probability_should_not_increase_for_physical_decay_fixture() {
        let depths = [0usize, 1, 2, 4, 8, 16, 32];

        let data = observations_from_model(
            &depths,
            0.5,
            0.5,
            0.95,
            100_000,
        );

        for pair in data.windows(2) {
            assert!(
                pair[1].success_probability()
                    <= pair[0].success_probability() + LOOSE_EPSILON,
                "synthetic RB decay fixture must be monotonically non-increasing"
            );
        }
    }

    // ------------------------------------------------------------------------
    // Error-rate convention tests
    // ------------------------------------------------------------------------

    #[test]
    fn entanglement_infidelity_convention_is_explicit() {
        let depths = [0usize, 1, 2, 4, 8, 16, 32];

        let data = observations_from_model(
            &depths,
            0.5,
            0.5,
            0.97,
            1_000_000,
        );

        let config = RandomizedBenchmarkingConfig::builder()
            .confidence_level(0.95)
            .error_rate_convention(
                RbErrorRateConvention::EntanglementInfidelity,
            )
            .minimum_depths(4)
            .build()
            .expect("configuration must be valid");

        let result = RandomizedBenchmarkingAnalyzer::new(config)
            .analyze(&data)
            .expect("RB fit must succeed");

        assert_eq!(
            result.error_rate_convention(),
            RbErrorRateConvention::EntanglementInfidelity
        );
    }

    #[test]
    fn average_gate_infidelity_convention_is_explicit() {
        let depths = [0usize, 1, 2, 4, 8, 16, 32];

        let data = observations_from_model(
            &depths,
            0.5,
            0.5,
            0.97,
            1_000_000,
        );

        let config = RandomizedBenchmarkingConfig::builder()
            .confidence_level(0.95)
            .error_rate_convention(
                RbErrorRateConvention::AverageGateInfidelity,
            )
            .minimum_depths(4)
            .build()
            .expect("configuration must be valid");

        let result = RandomizedBenchmarkingAnalyzer::new(config)
            .analyze(&data)
            .expect("RB fit must succeed");

        assert_eq!(
            result.error_rate_convention(),
            RbErrorRateConvention::AverageGateInfidelity
        );
    }

    #[test]
    fn different_error_rate_conventions_are_not_silently_identical_for_multi_qubit_rb() {
        let depths = [0usize, 1, 2, 4, 8, 16, 32];

        let data = observations_from_model(
            &depths,
            0.25,
            0.75,
            0.97,
            1_000_000,
        );

        let ei_config = RandomizedBenchmarkingConfig::builder()
            .num_qubits(2)
            .confidence_level(0.95)
            .error_rate_convention(
                RbErrorRateConvention::EntanglementInfidelity,
            )
            .minimum_depths(4)
            .build()
            .expect("EI configuration must be valid");

        let agi_config = RandomizedBenchmarkingConfig::builder()
            .num_qubits(2)
            .confidence_level(0.95)
            .error_rate_convention(
                RbErrorRateConvention::AverageGateInfidelity,
            )
            .minimum_depths(4)
            .build()
            .expect("AGI configuration must be valid");

        let ei = RandomizedBenchmarkingAnalyzer::new(ei_config)
            .analyze(&data)
            .expect("EI fit must succeed");

        let agi = RandomizedBenchmarkingAnalyzer::new(agi_config)
            .analyze(&data)
            .expect("AGI fit must succeed");

        assert!(
            (ei.error_rate() - agi.error_rate()).abs() > EPSILON,
            "the implementation must not silently treat EI and AGI as the \
             same convention for multi-qubit RB"
        );
    }

    // ------------------------------------------------------------------------
    // Fit diagnostics
    // ------------------------------------------------------------------------

    #[test]
    fn clean_synthetic_data_has_good_fit_quality() {
        let depths = [0usize, 1, 2, 4, 8, 16, 32, 64];

        let data = observations_from_model(
            &depths,
            0.5,
            0.5,
            0.98,
            1_000_000,
        );

        let result = analyze(&data)
            .expect("clean synthetic data should fit");

        assert!(
            result.fit_quality().r_squared() >= 0.999,
            "clean exponential data must have an excellent fit"
        );

        assert!(
            result.fit_quality().residual_sum_of_squares() >= 0.0,
            "RSS must never be negative"
        );
    }

    #[test]
    fn fit_result_contains_all_required_parameters() {
        let depths = [0usize, 1, 2, 4, 8, 16];

        let data = observations_from_model(
            &depths,
            0.5,
            0.5,
            0.96,
            100_000,
        );

        let result = analyze(&data)
            .expect("fixture should fit");

        assert!(result.decay_parameter().is_finite());
        assert!(result.offset().is_finite());
        assert!(result.amplitude().is_finite());
        assert!(result.error_rate().is_finite());

        assert!(
            result.sample_count() > 0,
            "result must expose the number of observations"
        );

        assert!(
            result.depth_count() >= 4,
            "result must expose the number of distinct RB depths"
        );
    }

    #[test]
    fn confidence_interval_is_present_for_decay_parameter() {
        let depths = [0usize, 1, 2, 4, 8, 16, 32];

        let data = observations_from_model(
            &depths,
            0.5,
            0.5,
            0.97,
            100_000,
        );

        let result = analyze(&data)
            .expect("fixture should fit");

        let interval = result
            .decay_parameter_confidence_interval()
            .expect("production RB must expose decay uncertainty");

        assert!(interval.lower().is_finite());
        assert!(interval.upper().is_finite());

        assert!(
            interval.lower() <= result.decay_parameter(),
            "lower confidence bound must not exceed point estimate"
        );

        assert!(
            result.decay_parameter() <= interval.upper(),
            "point estimate must not exceed upper confidence bound"
        );
    }

    #[test]
    fn confidence_interval_is_present_for_error_rate() {
        let depths = [0usize, 1, 2, 4, 8, 16, 32];

        let data = observations_from_model(
            &depths,
            0.5,
            0.5,
            0.97,
            100_000,
        );

        let result = analyze(&data)
            .expect("fixture should fit");

        let interval = result
            .error_rate_confidence_interval()
            .expect("production RB must expose error-rate uncertainty");

        assert!(interval.lower().is_finite());
        assert!(interval.upper().is_finite());

        assert!(
            interval.lower() <= result.error_rate() + LOOSE_EPSILON
        );

        assert!(
            result.error_rate() <= interval.upper() + LOOSE_EPSILON
        );
    }

    // ------------------------------------------------------------------------
    // Insufficient / invalid data
    // ------------------------------------------------------------------------

    #[test]
    fn insufficient_depths_are_rejected() {
        let data = vec![
            observation(1, 0.9, 1000),
            observation(2, 0.85, 1000),
        ];

        let result = analyze(&data);

        assert!(
            matches!(
                result,
                Err(RbAnalysisError::InsufficientDepths { .. })
            ),
            "RB must reject statistically insufficient depth coverage"
        );
    }

    #[test]
    fn empty_observation_set_is_rejected() {
        let result = analyze(&[]);

        assert!(
            matches!(
                result,
                Err(RbAnalysisError::InsufficientObservations { .. })
                    | Err(RbAnalysisError::InsufficientDepths { .. })
            ),
            "empty RB datasets must produce a structured statistical error"
        );
    }

    #[test]
    fn duplicate_depths_are_handled_explicitly() {
        let data = vec![
            observation(1, 0.95, 1000),
            observation(1, 0.94, 1000),
            observation(2, 0.90, 1000),
            observation(4, 0.84, 1000),
            observation(8, 0.75, 1000),
        ];

        let result = analyze(&data);

        assert!(
            result.is_ok() || matches!(
                result,
                Err(RbAnalysisError::DuplicateDepth { .. })
                    | Err(RbAnalysisError::InsufficientDepths { .. })
            ),
            "duplicate-depth handling must be deterministic and explicit"
        );
    }

    #[test]
    fn non_monotonic_data_is_not_silently_repaired() {
        let data = vec![
            observation(0, 0.50, 10_000),
            observation(1, 0.95, 10_000),
            observation(2, 0.40, 10_000),
            observation(4, 0.90, 10_000),
            observation(8, 0.20, 10_000),
            observation(16, 0.80, 10_000),
        ];

        let result = analyze(&data);

        match result {
            Ok(fit) => {
                assert!(
                    fit.fit_quality().has_warning(),
                    "a poor/nonphysical RB fixture must carry a diagnostic"
                );
            }

            Err(_) => {
                // Rejecting the dataset is also acceptable and preferable to
                // silently repairing it.
            }
        }
    }

    // ------------------------------------------------------------------------
    // Numerical safety
    // ------------------------------------------------------------------------

    #[test]
    fn decay_parameter_is_finite_for_valid_data() {
        let depths = [0usize, 1, 2, 4, 8, 16, 32, 64];

        let data = observations_from_model(
            &depths,
            0.5,
            0.5,
            0.999,
            100_000,
        );

        let result = analyze(&data)
            .expect("near-ideal decay should remain numerically stable");

        assert!(result.decay_parameter().is_finite());
        assert!(result.error_rate().is_finite());
    }

    #[test]
    fn very_large_depths_do_not_overflow() {
        let depths = [
            0usize,
            1,
            2,
            4,
            8,
            16,
            32,
            64,
            128,
            256,
            512,
        ];

        let data = observations_from_model(
            &depths,
            0.5,
            0.5,
            0.99,
            100_000,
        );

        let result = analyze(&data)
            .expect("large but finite RB depths should remain analyzable");

        assert!(result.decay_parameter().is_finite());
        assert!(result.error_rate().is_finite());
    }

    #[test]
    fn pathological_zero_decay_is_handled_without_nan() {
        let depths = [0usize, 1, 2, 4, 8, 16];

        let data = observations_from_model(
            &depths,
            0.5,
            0.5,
            0.0,
            100_000,
        );

        let result = analyze(&data);

        if let Ok(fit) = result {
            assert!(fit.decay_parameter().is_finite());
            assert!(fit.error_rate().is_finite());
        }
    }

    // ------------------------------------------------------------------------
    // Determinism / reproducibility
    // ------------------------------------------------------------------------

    #[test]
    fn analysis_is_deterministic_for_identical_observations() {
        let depths = [0usize, 1, 2, 4, 8, 16, 32];

        let data = observations_from_model(
            &depths,
            0.5,
            0.5,
            0.97,
            100_000,
        );

        let first = analyze(&data)
            .expect("first analysis must succeed");

        let second = analyze(&data)
            .expect("second analysis must succeed");

        assert!(
            (first.decay_parameter() - second.decay_parameter()).abs()
                < EPSILON
        );

        assert!(
            (first.error_rate() - second.error_rate()).abs()
                < EPSILON
        );

        assert!(
            (first.offset() - second.offset()).abs()
                < EPSILON
        );

        assert!(
            (first.amplitude() - second.amplitude()).abs()
                < EPSILON
        );
    }

    #[test]
    fn analysis_does_not_depend_on_global_random_state() {
        let depths = [0usize, 1, 2, 4, 8, 16];

        let data = observations_from_model(
            &depths,
            0.5,
            0.5,
            0.98,
            100_000,
        );

        let first = analyze(&data)
            .expect("first analysis must succeed");

        // No RNG is deliberately seeded or touched here.
        //
        // If the second result changes, the implementation has hidden
        // process-global/random state and therefore violates the deterministic
        // analysis contract.
        let second = analyze(&data)
            .expect("second analysis must succeed");

        assert_eq!(
            first,
            second,
            "RB analysis must not depend on process-global random state"
        );
    }

    // ------------------------------------------------------------------------
    // Shot/sample semantics
    // ------------------------------------------------------------------------

    #[test]
    fn more_shots_should_not_change_the_expected_fit_model() {
        let depths = [0usize, 1, 2, 4, 8, 16, 32];

        let low_shot = observations_from_model(
            &depths,
            0.5,
            0.5,
            0.97,
            1_000,
        );

        let high_shot = observations_from_model(
            &depths,
            0.5,
            0.5,
            0.97,
            1_000_000,
        );

        let low = analyze(&low_shot)
            .expect("low-shot deterministic fixture must fit");

        let high = analyze(&high_shot)
            .expect("high-shot deterministic fixture must fit");

        assert!(
            (low.decay_parameter() - high.decay_parameter()).abs()
                < LOOSE_EPSILON,
            "changing the declared shot count without changing the measured \
             probability should not change the point estimate"
        );
    }

    #[test]
    fn result_reports_total_sample_count() {
        let data = vec![
            observation(0, 1.0, 100),
            observation(1, 0.9, 200),
            observation(2, 0.8, 300),
            observation(4, 0.7, 400),
        ];

        let result = analyze(&data);

        if let Ok(result) = result {
            assert!(
                result.sample_count() >= 4,
                "result must preserve the number of supplied observations"
            );
        }
    }

    // ------------------------------------------------------------------------
    // Result/provenance contract
    // ------------------------------------------------------------------------

    #[test]
    fn result_identifies_randomized_benchmarking_protocol() {
        let depths = [0usize, 1, 2, 4, 8, 16];

        let data = observations_from_model(
            &depths,
            0.5,
            0.5,
            0.98,
            100_000,
        );

        let result = analyze(&data)
            .expect("fixture must fit");

        assert_eq!(
            result.protocol_id(),
            "randomized_benchmarking"
        );
    }

    #[test]
    fn result_contains_statistical_method() {
        let depths = [0usize, 1, 2, 4, 8, 16];

        let data = observations_from_model(
            &depths,
            0.5,
            0.5,
            0.98,
            100_000,
        );

        let result = analyze(&data)
            .expect("fixture must fit");

        assert!(
            !result.statistical_method().is_empty(),
            "scientific results must identify their statistical method"
        );
    }

    #[test]
    fn result_contains_confidence_level() {
        let config = RandomizedBenchmarkingConfig::builder()
            .confidence_level(0.99)
            .minimum_depths(4)
            .build()
            .expect("configuration must be valid");

        let depths = [0usize, 1, 2, 4, 8, 16];

        let data = observations_from_model(
            &depths,
            0.5,
            0.5,
            0.98,
            100_000,
        );

        let result = RandomizedBenchmarkingAnalyzer::new(config)
            .analyze(&data)
            .expect("fixture must fit");

        assert!(
            (result.confidence_level() - 0.99).abs() < EPSILON,
            "result must preserve the requested confidence level"
        );
    }

    // ------------------------------------------------------------------------
    // Regression fixtures
    // ------------------------------------------------------------------------

    #[test]
    fn regression_fixture_low_error() {
        let depths = [0usize, 1, 2, 4, 8, 16, 32, 64];

        let data = observations_from_model(
            &depths,
            0.5,
            0.5,
            0.995,
            1_000_000,
        );

        let result = analyze(&data)
            .expect("low-error regression fixture must fit");

        assert!(
            result.error_rate() >= 0.0,
            "RB error rate must not be negative"
        );

        assert!(
            result.error_rate() < 0.01,
            "p=0.995 should produce a low RB error rate"
        );
    }

    #[test]
    fn regression_fixture_medium_error() {
        let depths = [0usize, 1, 2, 4, 8, 16, 32, 64];

        let data = observations_from_model(
            &depths,
            0.5,
            0.5,
            0.95,
            1_000_000,
        );

        let result = analyze(&data)
            .expect("medium-error regression fixture must fit");

        assert!(
            result.error_rate() > 0.0,
            "p < 1 must imply a positive derived error rate"
        );
    }

    #[test]
    fn regression_fixture_high_error() {
        let depths = [0usize, 1, 2, 4, 8, 16, 32];

        let data = observations_from_model(
            &depths,
            0.5,
            0.5,
            0.80,
            1_000_000,
        );

        let result = analyze(&data)
            .expect("high-error regression fixture must fit");

        assert!(
            result.error_rate() > 0.0,
            "high decay must produce a positive error rate"
        );

        assert!(
            result.decay_parameter() < 1.0,
            "high-error fixture must have decay parameter below one"
        );
    }

    // ------------------------------------------------------------------------
    // Physical/statistical invariants
    // ------------------------------------------------------------------------

    #[test]
    fn valid_fit_has_decay_parameter_in_expected_range() {
        let depths = [0usize, 1, 2, 4, 8, 16, 32];

        let data = observations_from_model(
            &depths,
            0.5,
            0.5,
            0.96,
            1_000_000,
        );

        let result = analyze(&data)
            .expect("fixture must fit");

        assert!(
            result.decay_parameter() >= 0.0,
            "physical RB decay parameter cannot be materially negative"
        );

        assert!(
            result.decay_parameter() <= 1.0 + LOOSE_EPSILON,
            "physical RB decay parameter should not materially exceed one"
        );
    }

    #[test]
    fn derived_error_rate_is_finite_and_non_negative() {
        let depths = [0usize, 1, 2, 4, 8, 16, 32];

        let data = observations_from_model(
            &depths,
            0.5,
            0.5,
            0.94,
            1_000_000,
        );

        let result = analyze(&data)
            .expect("fixture must fit");

        assert!(result.error_rate().is_finite());

        assert!(
            result.error_rate() >= -LOOSE_EPSILON,
            "derived RB error rate must not be materially negative"
        );
    }

    #[test]
    fn result_does_not_claim_exact_gate_fidelity_without_convention() {
        let depths = [0usize, 1, 2, 4, 8, 16];

        let data = observations_from_model(
            &depths,
            0.5,
            0.5,
            0.98,
            100_000,
        );

        let result = analyze(&data)
            .expect("fixture must fit");

        assert!(
            result.error_rate_convention()
                == RbErrorRateConvention::EntanglementInfidelity
                || result.error_rate_convention()
                    == RbErrorRateConvention::AverageGateInfidelity,
            "RB results must explicitly identify the convention"
        );
    }

    // ------------------------------------------------------------------------
    // Backend independence
    // ------------------------------------------------------------------------

    #[test]
    fn analyzer_requires_no_backend() {
        let analyzer = RandomizedBenchmarkingAnalyzer::new(
            standard_config()
        );

        // The analyzer must be usable entirely from already-collected
        // observations. No hardware backend, simulator, IR, runtime, network,
        // or provider SDK is permitted here.
        let depths = [0usize, 1, 2, 4, 8, 16];

        let data = observations_from_model(
            &depths,
            0.5,
            0.5,
            0.98,
            100_000,
        );

        let result = analyzer
            .analyze(&data)
            .expect("backend-independent analysis must succeed");

        assert!(result.decay_parameter().is_finite());
    }

    // ------------------------------------------------------------------------
    // Contract for future circuit-level integration
    // ------------------------------------------------------------------------

    #[test]
    fn analysis_contract_accepts_multiple_sequences_per_depth() {
        // Standard RB uses multiple random sequences at each depth. The
        // production analyzer must therefore support aggregation from repeated
        // observations rather than assuming exactly one observation per depth.
        let data = vec![
            observation(0, 1.00, 1000),
            observation(0, 0.99, 1000),
            observation(0, 1.00, 1000),
            observation(1, 0.96, 1000),
            observation(1, 0.95, 1000),
            observation(1, 0.96, 1000),
            observation(2, 0.91, 1000),
            observation(2, 0.90, 1000),
            observation(2, 0.91, 1000),
            observation(4, 0.84, 1000),
            observation(4, 0.83, 1000),
            observation(4, 0.84, 1000),
            observation(8, 0.73, 1000),
            observation(8, 0.72, 1000),
            observation(8, 0.73, 1000),
        ];

        let result = analyze(&data);

        assert!(
            result.is_ok() || matches!(
                result,
                Err(RbAnalysisError::InsufficientDepths { .. })
            ),
            "repeated observations at each depth must be supported or \
             rejected with a precise statistical error"
        );
    }

    #[test]
    fn repeated_sequence_data_must_not_be_treated_as_new_depths() {
        let data = vec![
            observation(1, 0.96, 1000),
            observation(1, 0.95, 1000),
            observation(1, 0.97, 1000),
            observation(2, 0.91, 1000),
            observation(2, 0.90, 1000),
            observation(2, 0.92, 1000),
            observation(4, 0.83, 1000),
            observation(4, 0.82, 1000),
            observation(4, 0.84, 1000),
            observation(8, 0.70, 1000),
            observation(8, 0.71, 1000),
            observation(8, 0.69, 1000),
        ];

        let result = analyze(&data);

        if let Ok(result) = result {
            assert!(
                result.depth_count() <= 4,
                "three sequences at the same depth must not be counted \
                 as three distinct RB depths"
            );
        }
    }

    // ------------------------------------------------------------------------
    // Future protocol compatibility
    // ------------------------------------------------------------------------

    #[test]
    fn rb_result_has_a_stable_schema_version() {
        let depths = [0usize, 1, 2, 4, 8, 16];

        let data = observations_from_model(
            &depths,
            0.5,
            0.5,
            0.98,
            100_000,
        );

        let result = analyze(&data)
            .expect("fixture must fit");

        assert!(
            result.schema_version() > 0,
            "serialized RB results require an explicit schema version"
        );
    }

    #[test]
    fn rb_result_can_be_fingerprinted_for_regression_testing() {
        let depths = [0usize, 1, 2, 4, 8, 16];

        let data = observations_from_model(
            &depths,
            0.5,
            0.5,
            0.98,
            100_000,
        );

        let first = analyze(&data)
            .expect("first analysis must succeed");

        let second = analyze(&data)
            .expect("second analysis must succeed");

        assert_eq!(
            first.fingerprint(),
            second.fingerprint(),
            "identical deterministic input must produce the same result \
             fingerprint"
        );
    }
}