#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! Zamani parser recovery benchmark.
//!
//! File: src/frontend/parser/benches/recovery.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! # Purpose
//!
//! Measure malformed-expression detection and statement-boundary
//! synchronization using the canonical Zamani lexer and expression parser.
//!
//! This executable is a benchmark harness. It is not the production parser's
//! recovery implementation and does not use the legacy parser.
//!
//! # Workload configuration
//!
//! ZAMANI_RECOVERY_STATEMENTS  Number of generated statements.
//! ZAMANI_RECOVERY_INTERVAL    Every Nth statement is malformed.
//! ZAMANI_BENCH_ITERATIONS     Number of timed iterations.
//! ZAMANI_RECOVERY_NESTING     Expression parser's nesting safety budget.
//!
//! All settings must be positive integers. There is no fixed workload
//! ceiling in this benchmark. Actual workloads remain subject to available
//! memory, address space, execution time, and configured frontend budgets.
//!
//! # Recovery model
//!
//! 1. Parse one expression using the canonical expression parser.
//! 2. Accept a successful expression only when followed by a semicolon.
//! 3. On syntax failure, advance to the next semicolon or EOF.
//! 4. Guarantee forward progress whenever input remains before EOF.
//! 5. Continue with subsequent statements and count recovery events.
//!
//! This deliberately exercises a simple statement-level synchronization
//! strategy. It does not claim to implement delimiter-aware production
//! recovery, diagnostic deduplication, or the full Zamani grammar.
//!
//! # Cargo integration
//!
//! Add a separate benchmark target to Cargo.toml:
//!
//! [[bench]]
//! name = "recovery"
//! path = "src/frontend/parser/benches/recovery.rs"
//! harness = false
//!
//! Run with:
//!
//! cargo bench --bench recovery

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
const DEFAULT_MALFORMED_INTERVAL: usize = 7;
const DEFAULT_ITERATIONS: usize = 10;
const DEFAULT_NESTING_BUDGET: usize = 256;

/// User-configurable benchmark workload.
///
/// These are benchmark settings, not language-level restrictions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct BenchmarkConfig {
    statements: usize,
    malformed_interval: usize,
    iterations: usize,
    nesting_budget: usize,
}

impl BenchmarkConfig {
    fn from_environment() -> BenchResult<Self> {
        Ok(Self {
            statements: read_positive_usize(
                "ZAMANI_RECOVERY_STATEMENTS",
                DEFAULT_STATEMENTS,
            )?,
            malformed_interval: read_positive_usize(
                "ZAMANI_RECOVERY_INTERVAL",
                DEFAULT_MALFORMED_INTERVAL,
            )?,
            iterations: read_positive_usize(
                "ZAMANI_BENCH_ITERATIONS",
                DEFAULT_ITERATIONS,
            )?,
            nesting_budget: read_positive_usize(
                "ZAMANI_RECOVERY_NESTING",
                DEFAULT_NESTING_BUDGET,
            )?,
        })
    }

    fn expression_parser_config(self) -> ExpressionParserConfig {
        ExpressionParserConfig {
            max_nesting: self.nesting_budget,
        }
    }

    fn expected_malformed_statements(self) -> usize {
        // The generator marks statement indices interval-1, 2*interval-1,
        // and so on. This formula avoids addition overflow.
        self.statements / self.malformed_interval
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

    let parsed = value
        .parse::<usize>()
        .map_err(|error| {
            benchmark_error(format!("invalid {name} value {value:?}: {error}"))
        })?;

    if parsed == 0 {
        return Err(benchmark_error(format!(
            "{name} must be greater than zero"
        )));
    }

    Ok(parsed)
}

/// Construct the exact operator vocabulary used by the generated workload.
///
/// Punctuation such as semicolons is handled by the canonical scanner.
/// Operator spellings are registered with the lexer rather than reimplemented
/// by this benchmark.
fn build_operator_registry() -> BenchResult<OperatorRegistry> {
    let mut registry = OperatorRegistry::new();

    registry.register(OperatorDefinition::new(
        "+",
        TokenKind::Plus,
    )?)?;

    registry.validate()?;
    Ok(registry)
}

/// Generate valid and malformed expression statements.
///
/// Every `malformed_interval`th statement is malformed, counting from the
/// end of each interval. For example, interval 3 yields:
///
///     value_0_left + value_0_right;
///     value_1_left + value_1_right;
///     value_2_left + ;
///     value_3_left + value_3_right;
///
/// This produces syntax errors without requiring invalid characters or
/// relying on lexical-error recovery.
fn generate_source(
    statement_count: usize,
    malformed_interval: usize,
) -> BenchResult<String> {
    if statement_count == 0 || malformed_interval == 0 {
        return Err(benchmark_error(
            "statement count and malformed interval must be positive",
        ));
    }

    let estimated_capacity = statement_count
        .checked_mul(64)
        .ok_or_else(|| benchmark_error("source-size estimate overflow"))?;

    let mut source = String::new();
    source.try_reserve(estimated_capacity)?;

    for index in 0..statement_count {
        let malformed = (index + 1) % malformed_interval == 0;

        if malformed {
            writeln!(&mut source, "value_{index}_left + ;")?;
        } else {
            writeln!(
                &mut source,
                "value_{index}_left + value_{index}_right;"
            )?;
        }
    }

    Ok(source)
}

/// Tokenize using the canonical frontend lexer.
///
/// The returned tokens borrow lexemes from `source`; callers must keep the
/// source alive for as long as they use the tokens.
fn lex_source<'src>(source: &'src str) -> BenchResult<Vec<Token<'src>>> {
    let mut scanner = Scanner::new(
        source,
        LexerConfig::new(),
        KeywordRegistry::new(),
        build_operator_registry()?,
    )?;

    let mut tokens = Vec::new();

    // This is an allocation estimate, not a token-count limit. Fallible
    // reservation turns a capacity-estimation failure into a reported error.
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
                    "lexer ended without emitting the required EOF token",
                ));
            }
        }
    }
}

/// Counts work performed by the recovery harness.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct RecoveryStats {
    parsed_expressions: usize,
    recovered_errors: usize,
    skipped_tokens: usize,
    statements_terminated: usize,
}

impl RecoveryStats {
    fn record_expression(&mut self) -> BenchResult<()> {
        self.parsed_expressions = self
            .parsed_expressions
            .checked_add(1)
            .ok_or_else(|| benchmark_error("expression counter overflow"))?;
        Ok(())
    }

    fn record_recovery(&mut self) -> BenchResult<()> {
        self.recovered_errors = self
            .recovered_errors
            .checked_add(1)
            .ok_or_else(|| benchmark_error("recovery counter overflow"))?;
        Ok(())
    }

    fn record_skipped_token(&mut self) -> BenchResult<()> {
        self.skipped_tokens = self
            .skipped_tokens
            .checked_add(1)
            .ok_or_else(|| benchmark_error("skipped-token counter overflow"))?;
        Ok(())
    }

    fn record_terminated_statement(&mut self) -> BenchResult<()> {
        self.statements_terminated = self
            .statements_terminated
            .checked_add(1)
            .ok_or_else(|| benchmark_error("statement counter overflow"))?;
        Ok(())
    }
}

/// Advance to the next statement boundary.
///
/// The semicolon is consumed when found. EOF is left for the caller to
/// recognize. Each loop iteration advances the cursor, so malformed input
/// cannot trap this synchronization loop in an infinite loop.
///
/// This is intentionally a benchmark-local strategy. A production recovery
/// implementation should additionally account for nested delimiters and
/// use the parser's shared diagnostics, checkpoints, and resource budgets.
fn synchronize_at_statement_boundary(
    tokens: &[Token<'_>],
    position: &mut usize,
    stats: &mut RecoveryStats,
) -> BenchResult<()> {
    while let Some(token) = tokens.get(*position) {
        match token.kind() {
            TokenKind::Eof => return Ok(()),

            TokenKind::Semicolon => {
                *position = position
                    .checked_add(1)
                    .ok_or_else(|| benchmark_error("token position overflow"))?;

                stats.record_skipped_token()?;
                stats.record_terminated_statement()?;
                return Ok(());
            }

            _ => {
                *position = position
                    .checked_add(1)
                    .ok_or_else(|| benchmark_error("token position overflow"))?;

                stats.record_skipped_token()?;
            }
        }
    }

    Err(benchmark_error(
        "token stream ended without an explicit EOF token during recovery",
    ))
}

/// Run statement-level expression parsing with bounded forward progress.
///
/// A parse error is counted and synchronized to a semicolon or EOF. Successful
/// expressions must consume input and be followed by a semicolon. If a
/// separator is missing, the harness reports a recovery event and
/// synchronizes rather than silently accepting trailing tokens.
fn parse_with_recovery(
    tokens: &[Token<'_>],
    config: ExpressionParserConfig,
) -> BenchResult<RecoveryStats> {
    let mut stats = RecoveryStats::default();
    let mut position = 0usize;

    loop {
        let Some(current) = tokens.get(position) else {
            return Err(benchmark_error(
                "token stream ended without an explicit EOF token",
            ));
        };

        if current.kind() == TokenKind::Eof {
            return Ok(stats);
        }

        let starting_position = position;

        match parse_expression_prefix(black_box(tokens), position, config) {
            Ok((_expression, next_position)) => {
                if next_position <= starting_position {
                    stats.record_recovery()?;
                    synchronize_at_statement_boundary(
                        tokens,
                        &mut position,
                        &mut stats,
                    )?;
                    continue;
                }

                position = next_position;

                match tokens.get(position) {
                    Some(separator)
                        if separator.kind() == TokenKind::Semicolon =>
                    {
                        position = position.checked_add(1).ok_or_else(|| {
                            benchmark_error("token position overflow")
                        })?;

                        stats.record_expression()?;
                        stats.record_terminated_statement()?;
                    }

                    Some(token) if token.kind() == TokenKind::Eof => {
                        // An expression without its required statement
                        // terminator is malformed. There is no later
                        // statement to recover to, but the parser has reached
                        // EOF and can terminate deterministically.
                        stats.record_recovery()?;
                        return Ok(stats);
                    }

                    Some(_) => {
                        stats.record_recovery()?;
                        synchronize_at_statement_boundary(
                            tokens,
                            &mut position,
                            &mut stats,
                        )?;
                    }

                    None => {
                        return Err(benchmark_error(
                            "expression parser advanced beyond the token stream",
                        ));
                    }
                }
            }

            Err(_error) => {
                stats.record_recovery()?;

                // Synchronize from the start of the failed expression. This
                // may scan tokens already examined by the expression parser,
                // but guarantees progress to a known statement boundary.
                position = starting_position;

                synchronize_at_statement_boundary(
                    tokens,
                    &mut position,
                    &mut stats,
                )?;
            }
        }
    }
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

fn measure<F>(
    name: &str,
    iterations: usize,
    mut operation: F,
) -> BenchResult<Duration>
where
    F: FnMut() -> BenchResult<usize>,
{
    if iterations == 0 {
        return Err(benchmark_error(
            "benchmark iteration count must be positive",
        ));
    }

    // Warm-up executes the operation but is excluded from the measurement.
    black_box(operation()?);

    let start = Instant::now();

    for _ in 0..iterations {
        black_box(operation()?);
    }

    let elapsed = start.elapsed();
    let nanos_per_iteration =
        elapsed.as_nanos() / u128::try_from(iterations).unwrap_or(1);

    println!(
        "{name}: total={}, iterations={iterations}, ns/iteration={nanos_per_iteration}",
        format_duration(elapsed),
    );

    Ok(elapsed)
}

fn run() -> BenchResult<()> {
    let config = BenchmarkConfig::from_environment()?;
    let source = generate_source(
        config.statements,
        config.malformed_interval,
    )?;
    let parser_config = config.expression_parser_config();

    println!("Zamani parser recovery benchmark");
    println!("Statements: {}", config.statements);
    println!("Malformed interval: {}", config.malformed_interval);
    println!("Expected malformed statements: {}", config.expected_malformed_statements());
    println!("Expression nesting budget: {}", config.nesting_budget);
    println!("Iterations: {}", config.iterations);
    println!("Source bytes: {}", source.len());
    println!();

    // Validate the workload before recording timing data.
    let tokens = lex_source(&source)?;
    let baseline = parse_with_recovery(&tokens, parser_config)?;

    if baseline.recovered_errors < config.expected_malformed_statements() {
        return Err(benchmark_error(format!(
            "expected at least {} recoverable syntax errors, observed {}",
            config.expected_malformed_statements(),
            baseline.recovered_errors,
        )));
    }

    if baseline.statements_terminated == 0 {
        return Err(benchmark_error(
            "recovery harness did not terminate any statements",
        ));
    }

    println!("Tokens emitted: {}", tokens.len());
    println!("Expressions parsed: {}", baseline.parsed_expressions);
    println!("Recovery events: {}", baseline.recovered_errors);
    println!("Tokens skipped during synchronization: {}", baseline.skipped_tokens);
    println!("Statement terminators consumed: {}", baseline.statements_terminated);
    println!();

    measure("lex_malformed_source", config.iterations, || {
        Ok(lex_source(black_box(source.as_str()))?.len())
    })?;

    measure("recover_pre_tokenized_source", config.iterations, || {
        let stats = parse_with_recovery(
            black_box(tokens.as_slice()),
            parser_config,
        )?;

        if stats != baseline {
            return Err(benchmark_error(
                "recovery results changed between identical benchmark iterations",
            ));
        }

        Ok(stats.recovered_errors)
    })?;

    measure("lex_and_recover_source", config.iterations, || {
        let tokens = lex_source(black_box(source.as_str()))?;
        let stats = parse_with_recovery(
            black_box(tokens.as_slice()),
            parser_config,
        )?;

        if stats != baseline {
            return Err(benchmark_error(
                "end-to-end recovery results changed between identical iterations",
            ));
        }

        Ok(stats.recovered_errors)
    })?;

    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("parser recovery benchmark failed: {error}");

        let mut cause = error.source();
        while let Some(source) = cause {
            eprintln!("  caused by: {source}");
            cause = source.source();
        }

        std::process::exit(1);
    }
}