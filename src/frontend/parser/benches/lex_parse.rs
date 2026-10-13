#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! Zamani frontend lexing and expression-parsing benchmark.
//!
//! File: src/frontend/parser/benches/lex_parse.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Measure three separate workloads:
//!
//! 1. Lexing generated Zamani source with the canonical frontend scanner.
//! 2. Parsing expression statements from a pre-tokenized stream using the
//!    canonical domain-neutral expression parser.
//! 3. End-to-end lexing plus expression parsing.
//!
//! This benchmark does not use the legacy parser and does not implement a
//! replacement parser.
//!
//! # Scope
//!
//! Until a stable complete-program parser is available, this benchmark
//! measures expression-statement workloads, not the entire Zamani grammar.
//! Its generated syntax is intentionally limited to identifiers, addition,
//! multiplication, and semicolon-terminated statements.
//!
//! # Environment configuration
//!
//! ZAMANI_BENCH_STATEMENTS  Number of generated statements.
//! ZAMANI_BENCH_TERMS       Operands per expression.
//! ZAMANI_BENCH_ITERATIONS  Number of measured iterations.
//! ZAMANI_BENCH_NESTING     Expression-parser nesting budget.
//!
//! Every value must be a positive integer. These are workload settings,
//! not language-level limits. Workloads remain bounded by available memory,
//! address space, execution time, and configured lexer/parser safety budgets.
//!
//! # Cargo integration
//!
//! Add the following benchmark target to Cargo.toml while preserving existing
//! benchmark targets:
//
//! [[bench]]
//! name = "lex_parse"
//! path = "src/frontend/parser/benches/lex_parse.rs"
//! harness = false
//!
//! Run with:
//!
//! cargo bench --bench lex_parse
//!
//! Example workload configuration:
//!
//! ZAMANI_BENCH_STATEMENTS=100000 \
//! ZAMANI_BENCH_TERMS=32 \
//! ZAMANI_BENCH_ITERATIONS=10 \
//! cargo bench --bench lex_parse

use std::error::Error;
use std::fmt::Write as FmtWrite;
use std::hint::black_box;
use std::io;
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

type BenchResult<T> = Result<T, Box<dyn Error>>;

const DEFAULT_STATEMENTS: usize = 10_000;
const DEFAULT_TERMS: usize = 16;
const DEFAULT_ITERATIONS: usize = 10;
const DEFAULT_NESTING_BUDGET: usize = 256;

/// User-configurable workload parameters.
///
/// These values control benchmark workload generation only. They do not
/// impose language or hardware capacity restrictions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct BenchmarkConfig {
    statements: usize,
    terms: usize,
    iterations: usize,
    nesting_budget: usize,
}

impl BenchmarkConfig {
    fn from_environment() -> BenchResult<Self> {
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
            nesting_budget: read_positive_usize(
                "ZAMANI_BENCH_NESTING",
                DEFAULT_NESTING_BUDGET,
            )?,
        };

        // Reject workloads whose basic operand count cannot be represented.
        // This is arithmetic validation, not a workload ceiling.
        config
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

    fn expression_parser_config(self) -> ExpressionParserConfig {
        ExpressionParserConfig {
            max_nesting: self.nesting_budget,
        }
    }
}

/// Counts for a successfully parsed benchmark workload.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct ParseStats {
    statements: usize,
    expressions: usize,
    consumed_tokens: usize,
}

impl ParseStats {
    fn record_expression(&mut self) -> BenchResult<()> {
        self.expressions = self
            .expressions
            .checked_add(1)
            .ok_or_else(|| benchmark_error("expression counter overflow"))?;

        Ok(())
    }

    fn record_statement(&mut self) -> BenchResult<()> {
        self.statements = self
            .statements
            .checked_add(1)
            .ok_or_else(|| benchmark_error("statement counter overflow"))?;

        Ok(())
    }
}

fn benchmark_error(message: impl Into<String>) -> Box<dyn Error> {
    Box::new(io::Error::other(message.into()))
}

fn read_positive_usize(name: &str, default: usize) -> BenchResult<usize> {
    let Some(raw) = std::env::var_os(name) else {
        return Ok(default);
    };

    let value = raw
        .into_string()
        .map_err(|_| benchmark_error(format!("{name} must be valid Unicode")))?;

    let parsed = value.parse::<usize>().map_err(|error| {
        benchmark_error(format!("invalid {name} value {value:?}: {error}"))
    })?;

    if parsed == 0 {
        return Err(benchmark_error(format!(
            "{name} must be greater than zero"
        )));
    }

    Ok(parsed)
}

/// Create the operator registry required by the generated source.
///
/// Operator spelling and tokenization remain the responsibility of the
/// canonical lexer registry and scanner. Operator precedence remains the
/// responsibility of the expression parser.
fn build_operator_registry() -> BenchResult<OperatorRegistry> {
    let mut registry = OperatorRegistry::new();

    registry.register(OperatorDefinition::new("+", TokenKind::Plus)?)?;
    registry.register(OperatorDefinition::new("*", TokenKind::Star)?)?;
    registry.validate()?;

    Ok(registry)
}

/// Generate independent expression statements.
///
/// For example:
///
///     value_0_0 + value_0_1 * value_0_2;
///     value_1_0 + value_1_1 * value_1_2;
///
/// The workload exercises identifier scanning, operator scanning, expression
/// parsing, precedence handling, and statement-boundary recognition.
///
/// `terms_per_expression` must be positive.
fn generate_source(
    statement_count: usize,
    terms_per_expression: usize,
) -> BenchResult<String> {
    if statement_count == 0 || terms_per_expression == 0 {
        return Err(benchmark_error(
            "statement count and terms per expression must be positive",
        ));
    }

    let operand_count = statement_count
        .checked_mul(terms_per_expression)
        .ok_or_else(|| benchmark_error("operand count overflow"))?;

    // Reserve a conservative estimate. It is only an allocation hint and
    // does not limit the final source length.
    let estimated_bytes = operand_count
        .checked_mul(32)
        .and_then(|bytes| {
            statement_count
                .checked_mul(2)
                .and_then(|terminators| bytes.checked_add(terminators))
        })
        .ok_or_else(|| benchmark_error("source-size estimate overflow"))?;

    let mut source = String::new();
    source.try_reserve(estimated_bytes)?;

    for statement_index in 0..statement_count {
        for term_index in 0..terms_per_expression {
            if term_index > 0 {
                // Alternate operators so the expression parser exercises
                // both addition and multiplication precedence.
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
/// The tokens borrow their lexemes from `source`, so the source must remain
/// alive while the returned tokens are being used.
///
/// The function requires the canonical scanner to emit an explicit EOF token.
/// It does not silently accept a truncated stream.
fn lex_source<'src>(source: &'src str) -> BenchResult<Vec<Token<'src>>> {
    let mut scanner = Scanner::new(
        source,
        LexerConfig::new(),
        KeywordRegistry::new(),
        build_operator_registry()?,
    )?;

    let mut tokens = Vec::new();

    // This is an estimate, not a token-count ceiling.
    tokens.try_reserve(source.len() / 2)?;

    loop {
        match scanner.next_token()? {
            Some(token) => {
                let is_eof = token.kind() == TokenKind::Eof;
                tokens.push(token);

                if is_eof {
                    return Ok(tokens);
                }
            }
            None => {
                return Err(benchmark_error(
                    "canonical lexer ended without emitting an EOF token",
                ));
            }
        }
    }
}

/// Parse expression statements from an already-tokenized stream.
///
/// The function uses the canonical expression parser and checks every
/// statement boundary. It fails rather than looping forever if parsing does
/// not advance, the stream is truncated, or the generated syntax is rejected.
///
/// This benchmark intentionally does not perform statement recovery:
/// recovery performance belongs in recovery.rs.
fn parse_expression_statements(
    tokens: &[Token<'_>],
    parser_config: ExpressionParserConfig,
    expected_statements: usize,
) -> BenchResult<ParseStats> {
    let mut position = 0usize;
    let mut stats = ParseStats::default();

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
                "expression parser failed to make forward progress",
            ));
        }

        position = next_position;

        let Some(terminator) = tokens.get(position) else {
            return Err(benchmark_error(
                "expression ended without a statement terminator",
            ));
        };

        if terminator.kind() != TokenKind::Semicolon {
            return Err(benchmark_error(format!(
                "expected semicolon at token index {position}, found {:?}",
                terminator.kind()
            )));
        }

        position = position
            .checked_add(1)
            .ok_or_else(|| benchmark_error("token position overflow"))?;

        stats.record_expression()?;
        stats.record_statement()?;

        stats.consumed_tokens = position;
    }

    if stats.statements != expected_statements {
        return Err(benchmark_error(format!(
            "statement count mismatch: expected {expected_statements}, \
             parsed {}",
            stats.statements
        )));
    }

    Ok(stats)
}

/// Run one parse pass and expose the result to the optimizer barrier.
///
/// The result is checked so benchmark measurements cannot accidentally
/// measure a fast failure while reporting it as successful parsing.
fn run_parse(
    tokens: &[Token<'_>],
    parser_config: ExpressionParserConfig,
    expected_statements: usize,
) -> BenchResult<ParseStats> {
    let stats = parse_expression_statements(
        black_box(tokens),
        parser_config,
        expected_statements,
    )?;

    black_box(stats);
    Ok(stats)
}

/// Measure an operation after one untimed warm-up.
///
/// The returned duration covers only the timed iterations. Errors are
/// propagated immediately instead of being hidden inside benchmark output.
fn measure<F>(
    label: &str,
    iterations: usize,
    mut operation: F,
) -> BenchResult<Duration>
where
    F: FnMut() -> BenchResult<()>,
{
    operation()?;

    let started = Instant::now();

    for _ in 0..iterations {
        operation()?;
    }

    let elapsed = started.elapsed();

    println!(
        "{label}: total={elapsed:?}, iterations={iterations}, \
         ns/iteration={:.0}",
        elapsed.as_nanos() as f64 / iterations as f64
    );

    Ok(elapsed)
}

fn print_throughput(
    label: &str,
    elapsed: Duration,
    iterations: usize,
    source_bytes: usize,
    token_count: usize,
) {
    let total_seconds = elapsed.as_secs_f64();

    if total_seconds <= 0.0 {
        println!("{label}: elapsed time too small for stable throughput");
        return;
    }

    let bytes_processed = (source_bytes as f64) * (iterations as f64);
    let tokens_processed = (token_count as f64) * (iterations as f64);

    println!(
        "{label}: source_bytes_per_second={:.0}, \
         tokens_per_second={:.0}",
        bytes_processed / total_seconds,
        tokens_processed / total_seconds
    );
}

fn run() -> BenchResult<()> {
    let config = BenchmarkConfig::from_environment()?;
    let parser_config = config.expression_parser_config();

    println!("Zamani canonical frontend benchmark");
    println!("Rust baseline: 1.97+");
    println!("Statements: {}", config.statements);
    println!("Operands per expression: {}", config.terms);
    println!("Iterations: {}", config.iterations);
    println!("Expression nesting budget: {}", config.nesting_budget);
    println!("Parser scope: expression statements, not complete-program grammar");

    let source = generate_source(config.statements, config.terms)?;

    println!("Generated source bytes: {}", source.len());

    // Perform one canonical lex and parse pass before timing. This catches
    // incompatible generated syntax and configuration errors up front.
    let tokens = lex_source(&source)?;
    let baseline_stats = run_parse(
        &tokens,
        parser_config,
        config.statements,
    )?;

    if baseline_stats.expressions != config.statements {
        return Err(benchmark_error(
            "baseline expression count does not match generated workload",
        ));
    }

    println!("Baseline token count: {}", tokens.len());
    println!("Baseline parsed statements: {}", baseline_stats.statements);
    println!("Baseline parsed expressions: {}", baseline_stats.expressions);

    // Benchmark lexer throughput independently of parser work.
    let lex_elapsed = measure(
        "lex_only",
        config.iterations,
        || {
            let measured_tokens = lex_source(black_box(source.as_str()))?;

            if measured_tokens.last().map(Token::kind) != Some(TokenKind::Eof) {
                return Err(benchmark_error(
                    "measured lexer stream has no final EOF token",
                ));
            }

            black_box(measured_tokens);
            Ok(())
        },
    )?;

    print_throughput(
        "lex_only_throughput",
        lex_elapsed,
        config.iterations,
        source.len(),
        tokens.len(),
    );

    // Benchmark parser throughput using a single pre-tokenized stream.
    // Tokenization is intentionally outside this measurement.
    let parse_elapsed = measure(
        "parse_only",
        config.iterations,
        || {
            let stats = run_parse(
                black_box(tokens.as_slice()),
                parser_config,
                config.statements,
            )?;

            if stats != baseline_stats {
                return Err(benchmark_error(
                    "parser results changed between identical benchmark runs",
                ));
            }

            Ok(())
        },
    )?;

    print_throughput(
        "parse_only_throughput",
        parse_elapsed,
        config.iterations,
        source.len(),
        tokens.len(),
    );

    // Benchmark the combined path. Every timed iteration creates a fresh
    // scanner, token vector, and expression syntax tree.
    let combined_elapsed = measure(
        "lex_parse",
        config.iterations,
        || {
            let measured_tokens = lex_source(black_box(source.as_str()))?;
            let stats = run_parse(
                black_box(measured_tokens.as_slice()),
                parser_config,
                config.statements,
            )?;

            if stats != baseline_stats {
                return Err(benchmark_error(
                    "combined results changed between identical benchmark runs",
                ));
            }

            black_box(measured_tokens);
            Ok(())
        },
    )?;

    print_throughput(
        "lex_parse_throughput",
        combined_elapsed,
        config.iterations,
        source.len(),
        tokens.len(),
    );

    println!("Benchmark completed successfully.");

    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("lex_parse benchmark failed: {error}");

        let mut source = error.source();
        while let Some(cause) = source {
            eprintln!("  caused by: {cause}");
            source = cause.source();
        }

        std::process::exit(1);
    }
}