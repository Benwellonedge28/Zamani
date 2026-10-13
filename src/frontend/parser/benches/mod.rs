#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! Shared benchmark support for the Zamani frontend parser.
//!
//! File: src/frontend/parser/benches/mod.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Provide reusable, consistent utilities for the parser benchmark executables:
//!
//! - validated environment-based workload configuration;
//! - checked workload-size calculations;
//! - canonical operator-registry construction;
//! - canonical lexer token collection;
//! - valid expression-statement source generation;
//! - malformed statement source generation for recovery benchmarks;
//! - repeatable benchmark timing and reporting.
//!
//! This module is benchmark infrastructure. It is not a production parser,
//! grammar implementation, lexer replacement, or semantic-analysis layer.
//!
//! # Integration contract
//!
//! Benchmark executables may include this file explicitly:
//!
//! ```rust,ignore
//! #[path = "mod.rs"]
//! mod bench_support;
//! ```
//!
//! They can then use the shared utilities through `bench_support`.
//!
//! The support code uses the canonical frontend lexer. Expression parsing
//! remains the responsibility of `grammar::expressions` in the relevant
//! benchmark executable.
//!
//! # Scalability and resource policy
//!
//! There are no fixed language-level or hardware-capacity limits here.
//! Workload parameters are supplied by the caller or environment.
//!
//! Arithmetic overflow is rejected before workload construction. Fallible
//! reservation is used where practical. Actual workloads remain constrained
//! by available memory, address space, execution time, and operating-system
//! behavior. Rust allocation failure is not guaranteed to be recoverable.
//!
//! # Grammar authority
//!
//! Generated examples are benchmark inputs, not a normative definition of
//! Zamani syntax. Normative syntax remains governed by the repository's
//! specification and canonical grammar composition under `grammar/`.
//!
//! This module does not import or call the legacy parser.

use std::error::Error;
use std::fmt::Write as FmtWrite;
use std::hint::black_box;
use std::io;
use std::time::{Duration, Instant};

use zamani_compiler::frontend::lexer::api::TokenSource;
use zamani_compiler::frontend::lexer::config::LexerConfig;
use zamani_compiler::frontend::lexer::keyword_registry::KeywordRegistry;
use zamani_compiler::frontend::lexer::operator_registry::{
    OperatorDefinition, OperatorRegistry,
};
use zamani_compiler::frontend::lexer::scanner::Scanner;
use zamani_compiler::frontend::lexer::token::Token;
use zamani_compiler::frontend::lexer::token_kind::TokenKind;

/// The common fallible result type used by benchmark support functions.
pub(crate) type BenchResult<T> = Result<T, Box<dyn Error>>;

/// A completed benchmark measurement.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Measurement {
    /// Number of timed executions.
    pub(crate) iterations: usize,

    /// Total elapsed wall-clock time for the timed executions.
    pub(crate) elapsed: Duration,
}

impl Measurement {
    /// Return the arithmetic mean duration of one measured execution.
    ///
    /// Callers must provide a measurement with at least one iteration.
    #[must_use]
    pub(crate) fn mean_duration(self) -> Duration {
        self.elapsed / u32::try_from(self.iterations).unwrap_or(u32::MAX)
    }

    /// Return mean elapsed nanoseconds as a floating-point value.
    #[must_use]
    pub(crate) fn mean_nanoseconds(self) -> f64 {
        self.elapsed.as_nanos() as f64 / self.iterations as f64
    }
}

/// Construct a descriptive error for invalid benchmark configuration.
pub(crate) fn benchmark_error(message: impl Into<String>) -> Box<dyn Error> {
    Box::new(io::Error::new(
        io::ErrorKind::InvalidInput,
        message.into(),
    ))
}

/// Read a positive integer from an environment variable.
///
/// If the variable is absent, `default` is returned. Invalid Unicode,
/// invalid integers, and zero are reported as errors.
///
/// A returned value is a workload setting, not a language-level limit.
pub(crate) fn read_positive_usize(
    name: &str,
    default: usize,
) -> BenchResult<usize> {
    let Some(raw) = std::env::var_os(name) else {
        if default == 0 {
            return Err(benchmark_error(format!(
                "default for {name} must be greater than zero"
            )));
        }

        return Ok(default);
    };

    let text = raw
        .into_string()
        .map_err(|_| benchmark_error(format!("{name} must contain valid Unicode text")))?;

    let value = text.parse::<usize>().map_err(|error| {
        benchmark_error(format!("invalid {name} value {text:?}: {error}"))
    })?;

    if value == 0 {
        return Err(benchmark_error(format!(
            "{name} must be greater than zero"
        )));
    }

    Ok(value)
}

/// Calculate a workload size using checked multiplication.
///
/// Use this before reserving or generating a workload. This helper detects
/// representational overflow; it does not impose an arbitrary workload cap.
pub(crate) fn checked_product(
    left: usize,
    right: usize,
    description: &str,
) -> BenchResult<usize> {
    left.checked_mul(right).ok_or_else(|| {
        benchmark_error(format!(
            "{description} exceeds the representable workload size"
        ))
    })
}

/// Calculate a workload size using checked addition.
pub(crate) fn checked_sum(
    left: usize,
    right: usize,
    description: &str,
) -> BenchResult<usize> {
    left.checked_add(right).ok_or_else(|| {
        benchmark_error(format!(
            "{description} exceeds the representable workload size"
        ))
    })
}

/// Build an operator registry for a benchmark's required operators.
///
/// The spelling-to-token mapping remains owned by the canonical lexer
/// registry. Pass only the operators actually used by the workload.
///
/// The caller must ensure each supplied `TokenKind` is a valid operator kind.
pub(crate) fn build_operator_registry(
    operators: &[(&str, TokenKind)],
) -> BenchResult<OperatorRegistry> {
    let mut registry = OperatorRegistry::new();

    for &(spelling, kind) in operators {
        registry.register(OperatorDefinition::new(spelling, kind)?)?;
    }

    registry.validate()?;
    Ok(registry)
}

/// Lex a source string using the canonical frontend scanner.
///
/// Returned tokens borrow from `source`. The source must therefore remain
/// alive for at least as long as the returned vector.
///
/// The explicit EOF token is retained if the configured scanner emits one.
/// Token collection stops at EOF or when the token source returns `None`.
///
/// This helper intentionally collects the complete token stream because
/// expression-parser benchmarks operate on token slices. For workloads where
/// retaining every token is undesirable, benchmark the streaming lexer
/// separately instead of calling this helper.
pub(crate) fn lex_source<'src>(
    source: &'src str,
    operators: &[(&str, TokenKind)],
) -> BenchResult<Vec<Token<'src>>> {
    let mut scanner = Scanner::new(
        source,
        LexerConfig::new(),
        KeywordRegistry::new(),
        build_operator_registry(operators)?,
    )?;

    let mut tokens = Vec::new();

    loop {
        let Some(token) = scanner.next_token()? else {
            break;
        };

        let is_eof = token.kind() == TokenKind::Eof;
        tokens.push(token);

        if is_eof {
            break;
        }
    }

    Ok(tokens)
}

/// Generate valid semicolon-terminated expression statements.
///
/// Example output for two statements with three terms each:
///
/// ```text
/// value_0_0 + value_0_1 + value_0_2;
/// value_1_0 + value_1_1 + value_1_2;
/// ```
///
/// `statement_count` and `terms_per_expression` must both be positive.
/// The generated program is intentionally simple: this is a workload
/// generator, not a substitute for conformance fixtures covering the
/// complete Zamani grammar.
pub(crate) fn generate_expression_statements(
    statement_count: usize,
    terms_per_expression: usize,
) -> BenchResult<String> {
    if statement_count == 0 {
        return Err(benchmark_error(
            "statement_count must be greater than zero",
        ));
    }

    if terms_per_expression == 0 {
        return Err(benchmark_error(
            "terms_per_expression must be greater than zero",
        ));
    }

    let term_count = checked_product(
        statement_count,
        terms_per_expression,
        "total expression term count",
    )?;

    // This is an estimate used to reserve capacity, not a language limit.
    // The generator still relies on normal String growth if it is too small.
    let estimated_terms_bytes =
        checked_product(term_count, 24, "estimated expression source size")?;
    let estimated_statement_bytes =
        checked_product(statement_count, 2, "estimated statement separators")?;
    let estimated_capacity = checked_sum(
        estimated_terms_bytes,
        estimated_statement_bytes,
        "estimated expression source size",
    )?;

    let mut source = String::new();
    source.try_reserve(estimated_capacity)?;

    for statement_index in 0..statement_count {
        for term_index in 0..terms_per_expression {
            if term_index != 0 {
                source.push_str(" + ");
            }

            // Writing to String is infallible under the fmt::Write contract.
            let _ = write!(
                source,
                "value_{statement_index}_{term_index}"
            );
        }

        source.push_str(";\n");
    }

    Ok(source)
}

/// Generate expression statements with periodic syntax errors.
///
/// Every `malformed_interval`th statement is emitted as an intentionally
/// malformed expression. For example, with interval 3:
///
/// ```text
/// value_0_left + value_0_right;
/// value_1_left + value_1_right;
/// value_2_left + ;
/// value_3_left + value_3_right;
/// ```
///
/// The caller's recovery benchmark should assert the expected error count
/// rather than assuming that elapsed time alone proves recovery correctness.
pub(crate) fn generate_recovery_statements(
    statement_count: usize,
    malformed_interval: usize,
) -> BenchResult<String> {
    if statement_count == 0 {
        return Err(benchmark_error(
            "statement_count must be greater than zero",
        ));
    }

    if malformed_interval == 0 {
        return Err(benchmark_error(
            "malformed_interval must be greater than zero",
        ));
    }

    let estimated_capacity =
        checked_product(statement_count, 64, "estimated recovery source size")?;

    let mut source = String::new();
    source.try_reserve(estimated_capacity)?;

    for index in 0..statement_count {
        if (index + 1) % malformed_interval == 0 {
            let _ = writeln!(source, "value_{index}_left + ;");
        } else {
            let _ = writeln!(
                source,
                "value_{index}_left + value_{index}_right;"
            );
        }
    }

    Ok(source)
}

/// Count the malformed statements emitted by `generate_recovery_statements`.
#[must_use]
pub(crate) fn expected_recovery_errors(
    statement_count: usize,
    malformed_interval: usize,
) -> Option<usize> {
    if malformed_interval == 0 {
        return None;
    }

    Some(statement_count / malformed_interval)
}

/// Execute one warm-up and then measure `iterations` executions.
///
/// The warm-up is deliberately excluded from the reported elapsed time.
/// The operation should return an error if the benchmarked work fails its
/// correctness checks. Results are passed through `black_box` to discourage
/// unwanted optimization of the measured operation.
///
/// Each execution should construct fresh mutable parser/session state where
/// appropriate. Reuse immutable inputs when that matches the workload being
/// measured, and document that choice in the individual benchmark.
pub(crate) fn measure<F>(
    iterations: usize,
    mut operation: F,
) -> BenchResult<Measurement>
where
    F: FnMut() -> BenchResult<()>,
{
    if iterations == 0 {
        return Err(benchmark_error(
            "benchmark iterations must be greater than zero",
        ));
    }

    black_box(operation()?);

    let start = Instant::now();

    for _ in 0..iterations {
        black_box(operation()?);
    }

    Ok(Measurement {
        iterations,
        elapsed: start.elapsed(),
    })
}

/// Print a consistent summary for a completed measurement.
pub(crate) fn report_measurement(
    label: &str,
    measurement: Measurement,
) {
    println!(
        "{label}: iterations={}, total={:?}, mean={:.2} ns/iteration",
        measurement.iterations,
        measurement.elapsed,
        measurement.mean_nanoseconds(),
    );
}

#[cfg(test)]
mod tests {
    use super::{
        checked_product, expected_recovery_errors, generate_expression_statements,
        generate_recovery_statements,
    };

    #[test]
    fn generates_valid_expression_statements() {
        let source = generate_expression_statements(2, 3).expect("generation succeeds");

        assert_eq!(
            source,
            "value_0_0 + value_0_1 + value_0_2;\n\
             value_1_0 + value_1_1 + value_1_2;\n"
        );
    }

    #[test]
    fn generates_recovery_workload_deterministically() {
        let source = generate_recovery_statements(3, 3).expect("generation succeeds");

        assert_eq!(
            source,
            "value_0_left + value_0_right;\n\
             value_1_left + value_1_right;\n\
             value_2_left + ;\n"
        );

        assert_eq!(expected_recovery_errors(10, 3), Some(3));
        assert_eq!(expected_recovery_errors(10, 0), None);
    }

    #[test]
    fn rejects_zero_workload_dimensions() {
        assert!(generate_expression_statements(0, 1).is_err());
        assert!(generate_expression_statements(1, 0).is_err());
        assert!(generate_recovery_statements(1, 0).is_err());
    }

    #[test]
    fn checked_workload_arithmetic_detects_overflow() {
        assert_eq!(checked_product(4, 5, "test").expect("fits"), 20);
        assert!(checked_product(usize::MAX, 2, "test").is_err());
    }
}