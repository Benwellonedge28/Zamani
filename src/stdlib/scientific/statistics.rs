
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Standard Library — Scientific Statistics
//!
//! File: src/stdlib/scientific/statistics.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: unsafe Rust is forbidden.
//!
//! # Purpose
//!
//! Expose portable statistical APIs through the Zamani standard library.
//!
//! This module is a public standard-library integration facade over the
//! canonical classical statistics implementation. It deliberately does
//! not implement a second collection, numeric representation, statistical
//! algorithm engine, or execution system.
//!
//! # Canonical ownership
//!
//! Statistical algorithms:
//!     crate::classical::numeric::statistics
//!
//! Canonical floating-point representation:
//!     crate::quantum::ir::classical::float::ClassicalFloat
//!
//! Canonical collection and storage infrastructure:
//!     crate::classical::collections
//!
//! Numerical module composition:
//!     crate::classical::numeric
//!
//! # Public responsibilities
//!
//! This module exposes:
//!
//! - checked statistical summation;
//! - arithmetic mean;
//! - minimum and maximum;
//! - population variance;
//! - sample variance;
//! - population standard deviation;
//! - sample standard deviation;
//! - statistical summaries;
//! - quantiles;
//! - median;
//! - population covariance;
//! - sample covariance;
//! - Pearson correlation;
//! - canonical statistical result and error types.
//!
//! # Ownership restrictions
//!
//! This module MUST NOT:
//!
//! - define another floating-point type;
//! - define another vector, matrix, array, or tensor storage type;
//! - duplicate algorithms already owned by classical/numeric/statistics;
//! - silently convert non-finite values into valid observations;
//! - silently truncate statistical inputs;
//! - introduce fixed observation-count limits;
//! - impose hardware-specific limits;
//! - select CPUs, GPUs, FPGAs, QPUs, or distributed workers;
//! - implement a scheduler or execution runtime;
//! - create a competing semantic model or IR.
//!
//! # Numerical contract
//!
//! All concrete observations and successful numeric results use the
//! canonical ClassicalFloat representation.
//!
//! The canonical implementation defines:
//!
//! - finite-value validation;
//! - checked arithmetic;
//! - population versus sample divisors;
//! - insufficient-sample errors;
//! - the R-7 quantile convention;
//! - undefined correlation behavior;
//! - allocation failures;
//! - numerical overflow behavior.
//!
//! This facade preserves those semantics without changing them.
//!
//! # Scalability and POCO-REAF
//!
//! This module introduces no arbitrary maximum observation count,
//! vector length, matrix dimension, tensor rank, processor count,
//! accelerator count, or distributed-node count.
//!
//! APIs borrow caller-provided slices and leave storage ownership with
//! the caller. Operations requiring working storage use the canonical
//! implementation's fallible allocation behavior.
//!
//! Actual execution remains bounded by:
//!
//! - representable input sizes;
//! - available memory;
//! - available computation time;
//! - canonical numeric representation;
//! - explicit execution and resource policies.
//!
//! This is resource-scalable API design, not a promise of infinite
//! physical memory or computation.
//!
//! # Compiler and grammar integration
//!
//! Numerical syntax is owned by the existing grammar:
//!
//! - grammar/classical/numeric.g4
//! - grammar/classical/numerical.g4
//! - grammar/expressions/arithmetic.g4
//! - grammar/expressions/expressions.g4
//!
//! Semantic analysis owns type checking, operand validation, and
//! symbolic-expression meaning.
//!
//! This module receives already valid concrete numeric values.
//! It does not parse source text, construct AST nodes, perform type
//! inference, or create IR.
//!
//! # Cross-domain integration
//!
//! Classical statistical operations may be used by:
//!
//! - classical programs;
//! - scientific applications;
//! - AI and machine-learning applications;
//! - quantum-classical hybrid programs;
//! - HDL parameter analysis;
//! - distributed data-processing applications;
//! - simulation and optimization systems.
//!
//! The compiler and runtime determine whether and how those operations
//! are parallelized, distributed, or accelerated.
//!
//! # Compatibility
//!
//! Public statistical types and functions are re-exported from their
//! canonical implementation. Their behavior and error semantics are
//! therefore not independently versioned by this facade.
//!
//! A future API change must preserve the canonical implementation's
//! compatibility contract and review downstream standard-library users.

/// Canonical finite floating-point value used by statistical operations.
pub use crate::quantum::ir::classical::float::ClassicalFloat;

/// Canonical statistical errors and result types.
pub use crate::classical::numeric::statistics::{
    StatisticsError,
    StatisticsResult,
    StatisticsSummary,
};

/// Canonical checked statistical algorithms.
///
/// These are re-exports, not duplicate implementations.
pub use crate::classical::numeric::statistics::{
    mean,
    median,
    min_max,
    pearson_correlation,
    population_covariance,
    population_standard_deviation,
    population_variance,
    quantile,
    sample_covariance,
    sample_standard_deviation,
    sample_variance,
    sum,
    summarize,
};

#[cfg(test)]
mod tests {
    use super::{
        mean,
        median,
        min_max,
        pearson_correlation,
        population_variance,
        quantile,
        sample_variance,
        summarize,
        ClassicalFloat,
        StatisticsError,
    };

    fn values(input: &[f64]) -> Vec<ClassicalFloat> {
        input
            .iter()
            .copied()
            .map(|value| {
                ClassicalFloat::new(value)
                    .expect("test observations must be finite")
            })
            .collect()
    }

    fn assert_close(actual: ClassicalFloat, expected: f64) {
        let difference = (actual.get() - expected).abs();

        assert!(
            difference <= 1.0e-10,
            "expected {expected}, received {}",
            actual.get()
        );
    }

    #[test]
    fn standard_library_exposes_canonical_mean() {
        let observations = values(&[1.0, 2.0, 3.0, 4.0]);

        assert_close(
            mean(&observations).expect("mean"),
            2.5,
        );
    }

    #[test]
    fn standard_library_exposes_canonical_variance() {
        let observations = values(&[1.0, 2.0, 3.0, 4.0]);

        assert_close(
            population_variance(&observations)
                .expect("population variance"),
            1.25,
        );

        assert_close(
            sample_variance(&observations)
                .expect("sample variance"),
            5.0 / 3.0,
        );
    }

    #[test]
    fn standard_library_exposes_canonical_quantiles() {
        let observations = values(&[1.0, 2.0, 3.0, 4.0]);

        assert_close(
            median(&observations).expect("median"),
            2.5,
        );

        assert_close(
            quantile(&observations, 0.0)
                .expect("minimum quantile"),
            1.0,
        );

        assert_close(
            quantile(&observations, 1.0)
                .expect("maximum quantile"),
            4.0,
        );
    }

    #[test]
    fn standard_library_exposes_canonical_minimum_and_maximum() {
        let observations = values(&[8.0, -3.0, 2.0, 10.0]);

        let (minimum, maximum) =
            min_max(&observations).expect("minimum and maximum");

        assert_close(minimum, -3.0);
        assert_close(maximum, 10.0);
    }

    #[test]
    fn standard_library_exposes_canonical_correlation() {
        let x = values(&[1.0, 2.0, 3.0, 4.0]);
        let y = values(&[3.0, 5.0, 7.0, 9.0]);

        assert_close(
            pearson_correlation(&x, &y)
                .expect("correlation"),
            1.0,
        );
    }

    #[test]
    fn standard_library_exposes_canonical_summary() {
        let observations = values(&[1.0, 2.0, 3.0]);

        let summary =
            summarize(&observations).expect("summary");

        assert_eq!(summary.count, 3);
        assert_close(summary.sum, 6.0);
        assert_close(summary.mean, 2.0);
        assert_close(summary.minimum, 1.0);
        assert_close(summary.maximum, 3.0);
    }

    #[test]
    fn empty_input_returns_canonical_error() {
        let observations: Vec<ClassicalFloat> = Vec::new();

        assert_eq!(
            mean(&observations),
            Err(StatisticsError::EmptyInput)
        );
    }

    #[test]
    fn invalid_quantile_returns_canonical_error() {
        let observations = values(&[1.0, 2.0, 3.0]);

        assert_eq!(
            quantile(&observations, -0.1),
            Err(StatisticsError::InvalidQuantile)
        );
    }
}
