#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! Zamani parser benchmark: large source workloads.
//!
//! File: src/frontend/parser/benches/large_programs.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Measure the scaling behavior of:
//! - canonical lexer tokenization for generated source programs;
//! - the existing domain-neutral expression parser over token streams;
//! - combined lexing and expression parsing across large workloads.
//!
//! # Important scope boundary
//!
//! This benchmark does not use the legacy parser and does not implement a
//! replacement parser. It uses the canonical frontend Scanner and the
//! existing grammar::expressions::parse_expression_prefix implementation.
//!
//! Until the complete-program parser is exposed through a stable public API,
//! this file measures a large collection of expression statements, not the
//! full source-program grammar.
//!
//! # Workload controls
//!
//! ZAMANI_BENCH_STATEMENTS  Number of expression statements to generate.
//! ZAMANI_BENCH_TERMS       Number of operands per expression.
//! ZAMANI_BENCH_ITERATIONS  Number of timed iterations.
//!
//! Values must be positive integers. There is no hard-coded workload ceiling.
//! Actual workload size is constrained by available resources, address space,
//! execution time, and the configured lexer/parser safety policies.
//!
//! # Run
//!
//! After registering this file as a Cargo benchmark target:
//!
//! cargo bench --bench large_programs
//!
//! Safe Rust only; no additional benchmark dependency is required.

use std::error::Error;
use std::fmt::Write as FmtWrite;
use std::hint::black_box;
use std::time::{Duration, Instant};

use zamani_compiler::frontend::lexer::api::TokenSource;
use zamani_compiler::frontend::lexer::config::LexerConfig;
use zamani_compiler::frontend::lexer::keyword_registry::KeywordRegistry;
use zamani_compiler::frontend::lexer::operator_registry::{
    OperatorDefinition,
    OperatorRegistry,
};
use zamani_compiler::frontend::lexer::scanner::Scanner;
use zamani_compiler::frontend::lexer::token::Token;
use zamani_compiler::frontend::lexer::token_kind::TokenKind;
use zamani_compiler::frontend::parser::grammar::expressions::{
    parse_expression_prefix,
    ExpressionParserConfig,
};

const DEFAULT_STATEMENTS: usize = 10_000;
const DEFAULT_TERMS: usize = 16;
const DEFAULT_ITERATIONS: usize = 10;

/// Workload settings supplied by environment variables.
#[derive(Debug, Clone, Copy)]
struct BenchmarkConfig {
    statements: usize,
    terms: usize,
    iterations: usize,
}

impl BenchmarkConfig {
    fn from_environment() -> Result<Self, Box<dyn Error>> {
        let config = Self {
            statements: read_positive_usize(
                "ZAMANI_BENCH_STATEMENTS",
                DEFAULT_STATEMENTS,
            )?,
            terms: read_positive_usize(
                "ZAMANI_BENCH_TERMS",
                DEFAULT_TERMS,
            )?,
            iterations: read_positive_usize(
                "ZAMANI_BENCH_ITERATIONS",
                DEFAULT_ITERATIONS,
            )?,
        };

        // Fail early on arithmetic overflow instead of silently wrapping.
        let _ = config
            .statements
            .checked_mul(config.terms)
            .ok_or_else(|| {
                benchmark_error(
                    "statement count multiplied by terms per expression \
                     exceeds the representable workload size",
                )
            })?;

        Ok(config)
    }
}

fn benchmark_error(message: impl Into<String>) -> Box<dyn Error> {
    Box::new(std::io::Error::other(message.into()))
}

fn read_positive_usize(
    variable: &str,
    default: usize,
) -> Result<usize, Box<dyn Error>> {
    let value = match std::env::var(variable) {
        Ok(value) => value.parse::<usize>()?,
        Err(std::env::VarError::NotPresent) => default,
        Err(error) => return Err(Box::new(error)),
    };

    if value == 0 {
        return Err(benchmark_error(format!(
            "{variable} must be greater than zero"
        )));
    }

    Ok(value)
}

/// Build the operator registry required by the generated workload.
///
/// The benchmark registers only the operators it actually uses. It does not
/// duplicate operator recognition or maximal-munch logic from the lexer.
fn build_operator_registry() -> Result<OperatorRegistry, Box<dyn Error>> {
    let mut registry = OperatorRegistry::new();

    registry.register(OperatorDefinition::new("+", TokenKind::Plus)?)?;
    registry.register(OperatorDefinition::new("*", TokenKind::Star)?)?;

    registry.validate()?;

    Ok(registry)
}

/// Generate many independent expression statements.
///
/// Example output:
///
///     value_0_0 + value_0_1 * value_0_2 + value_0_3;
///     value_1_0 + value_1_1 * value_1_2 + value_1_3;
///
/// The syntax is deliberately restricted to the operators and expression
/// forms exercised by this benchmark. It is not a substitute for normative
/// grammar fixtures.
fn generate_source(
    statement_count: usize,
    terms_per_expression: usize,
) -> Result<String, Box<dyn Error>> {
    if statement_count == 0 {
        return Err(benchmark_error(
            "statement count must be greater than zero",
        ));
    }

    if terms_per_expression == 0 {
        return Err(benchmark_error(
            "terms per expression must be greater than zero",
        ));
    }

    let estimated_terms = statement_count
        .checked_mul(terms_per_expression)
        .ok_or_else(|| benchmark_error("workload size overflow"))?;

    // Reserve a conservative initial estimate. String may grow further if
    // identifiers or formatting require more capacity.
    let estimated_bytes = estimated_terms
        .checked_mul(20)
        .and_then(|bytes| bytes.checked_add(statement_count))
        .ok_or_else(|| benchmark_error("source-size estimate overflow"))?;

    let mut source = String::new();
    source.try_reserve(estimated_bytes)?;

    for statement_index in 0..statement_count {
        for term_index in 0..terms_per_expression {
            if term_index > 0 {
                // Alternate addition and multiplication to exercise different
                // precedence levels and build a nontrivial expression tree.
                if term_index % 2 == 1 {
                    source.push_str(" + ");
                } else {
                    source.push_str(" * ");
                }
            }

            write!(
                &mut source,
                "value_{statement_index}_{term_index}"
            )?;
        }

        source.push_str(";\n");
    }

    Ok(source)
}

/// Tokenize source using the canonical frontend scanner.
///
/// Tokens borrow their lexemes from `source`, so the source must outlive the
/// returned token vector. The caller retains both for the benchmark duration.
fn lex_source<'src>(
    source: &'src str,
) -> Result<Vec<Token<'src>>, Box<dyn Error>> {
    let keywords = KeywordRegistry::new();
    let operators = build_operator_registry()?;

    let mut scanner = Scanner::new(
        source,
        LexerConfig::new(),
        keywords,
        operators,
    )?;

    let mut tokens = Vec::new();

    // The source is already in memory. Reserve an estimate rather than
    // introducing a fixed token-count ceiling.
    tokens.try_reserve(source.len() / 2)?;

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

/// Parse every expression statement in the token stream.
///
/// Each successful iteration must advance to the following semicolon.
/// Unexpected tokens, malformed expressions, or a non-progressing parser
/// produce a benchmark error instead of an infinite loop.
fn parse_expression_statements(
    tokens: &[Token<'_>],
    parser_config: ExpressionParserConfig,
) -> Result<usize, Box<dyn Error>> {
    let mut position = 0usize;
    let mut parsed_expressions = 0usize;

    loop {
        let Some(current) = tokens.get(position) else {
            return Err(benchmark_error(
                "token stream ended without an explicit EOF token",
            ));
        };

        if current.kind() == TokenKind::Eof {
            break;
        }

        let previous_position = position;

        let (_expression, next_position) = parse_expression_prefix(
            black_box(tokens),
            position,
            parser_config,
        )?;

        if next_position <= previous_position {
            return Err(benchmark_error(
                "expression parser did not consume input",
            ));
        }

        position = next_position;

        let Some(separator) = tokens.get(position) else {
            return Err(benchmark_error(
                "expected a statement separator or EOF",
            ));
        };

        if separator.kind() != TokenKind::Semicolon {
            return Err(benchmark_error(format!(
                "expected ';' after expression at token index {position}"
            )));
        }

        position = position
            .checked_add(1)
            .ok_or_else(|| benchmark_error("token position overflow"))?;

        parsed_expressions = parsed_expressions
            .checked_add(1)
            .ok_or_else(|| benchmark_error("expression counter overflow"))?;
    }

    Ok(parsed_expressions)
}

fn format_duration(duration: Duration) -> String {
    if duration.as_secs() > 0 {
        format!("{:.3} s", duration.as_secs_f64())
    } else if duration.as_millis() > 0 {
        format!("{:.3} ms", duration.as_secs_f64() * 1_000.0)
    } else if duration.as_micros() > 0 {
        format!("{:.3} µs", duration.as_secs_f64() * 1_000_000.0)
    } else {
        format!("{} ns", duration.as_nanos())
    }
}

fn report_measurement(
    name: &str,
    elapsed: Duration,
    iterations: usize,
    work_units_per_iteration: usize,
    work_unit_label: &str,
) {
    let nanos_per_iteration =
        elapsed.as_nanos() / u128::try_from(iterations).unwrap_or(1);

    let total_work_units = work_units_per_iteration
        .checked_mul(iterations)
        .unwrap_or(usize::MAX);

    let units_per_second = if elapsed.is_zero() {
        0.0
    } else {
        total_work_units as f64 / elapsed.as_secs_f64()
    };

    println!(
        "{name}: total={}, iterations={iterations}, \
         ns/iteration={nanos_per_iteration}, \
         {work_unit_label}/iteration={work_units_per_iteration}, \
         {work_unit_label}/second={units_per_second:.0}",
        format_duration(elapsed),
    );
}

fn measure<F>(
    name: &str,
    iterations: usize,
    work_units_per_iteration: usize,
    work_unit_label: &str,
    mut operation: F,
) -> Result<(), Box<dyn Error>>
where
    F: FnMut() -> Result<(), Box<dyn Error>>,
{
    if iterations == 0 {
        return Err(benchmark_error(
            "benchmark iteration count must be greater than zero",
        ));
    }

    // Warm up the relevant code paths. Warm-up is intentionally excluded from
    // the reported duration.
    operation()?;

    let start = Instant::now();

    for _ in 0..iterations {
        operation()?;
    }

    let elapsed = start.elapsed();

    report_measurement(
        name,
        elapsed,
        iterations,
        work_units_per_iteration,
        work_unit_label,
    );

    Ok(())
}

fn run() -> Result<(), Box<dyn Error>> {
    let config = BenchmarkConfig::from_environment()?;
    let source = generate_source(config.statements, config.terms)?;
    let parser_config = ExpressionParserConfig::default();

    println!("Zamani large-program frontend benchmark");
    println!("Rust edition: 2021");
    println!("Statements: {}", config.statements);
    println!("Terms per expression: {}", config.terms);
    println!("Iterations: {}", config.iterations);
    println!("Source bytes: {}", source.len());
    println!();

    // Confirm the workload is valid before collecting timing results.
    let tokens = lex_source(&source)?;
    let expected_expressions = parse_expression_statements(
        &tokens,
        parser_config,
    )?;

    if expected_expressions != config.statements {
        return Err(benchmark_error(format!(
            "generated {} statements but parsed {expected_expressions} \
             expressions",
            config.statements,
        )));
    }

    println!("Tokens emitted: {}", tokens.len());
    println!("Expressions parsed: {expected_expressions}");
    println!();

    measure(
        "lex_large_source",
        config.iterations,
        tokens.len(),
        "tokens",
        || {
            let result = lex_source(black_box(source.as_str()))?;
            black_box(result);
            Ok(())
        },
    )?;

    measure(
        "parse_large_expression_set",
        config.iterations,
        expected_expressions,
        "expressions",
        || {
            let parsed = parse_expression_statements(
                black_box(tokens.as_slice()),
                parser_config,
            )?;

            if parsed != expected_expressions {
                return Err(benchmark_error(
                    "expression count changed between benchmark iterations",
                ));
            }

            black_box(parsed);
            Ok(())
        },
    )?;

    measure(
        "lex_and_parse_large_source",
        config.iterations,
        expected_expressions,
        "expressions",
        || {
            let tokens = lex_source(black_box(source.as_str()))?;
            let parsed = parse_expression_statements(
                black_box(tokens.as_slice()),
                parser_config,
            )?;

            if parsed != expected_expressions {
                return Err(benchmark_error(
                    "end-to-end expression count changed between iterations",
                ));
            }

            black_box(parsed);
            Ok(())
        },
    )?;

    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("large-program benchmark failed: {error}");

        let mut source = error.source();
        while let Some(cause) = source {
            eprintln!("  caused by: {cause}");
            source = cause.source();
        }

        std::process::exit(1);
    }
}