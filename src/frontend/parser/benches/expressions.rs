#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! Zamani expression parser benchmarks.
//!
//! File: src/frontend/parser/benches/expressions.rs
//! Rust: 1.97+
//! Edition: 2021
//!
//! Measures:
//! - canonical lexer throughput;
//! - parsing an already-tokenized expression;
//! - end-to-end lexing and expression parsing;
//! - operator-precedence parsing;
//! - nested grouping and nesting-budget behavior.
//!
//! This is a benchmark executable, not a second parser. All parsing is
//! delegated to the canonical frontend expression parser.
//!
//! Run after the frontend modules are publicly exported:
//!
//!     cargo bench --bench expressions
//!
//! Optional environment variables:
//!
//!     ZAMANI_EXPR_TERMS=1024
//!     ZAMANI_EXPR_NESTING=64
//!     ZAMANI_BENCH_ITERATIONS=20
//!
//! These values control benchmark workload, not language or hardware limits.

use std::error::Error;
use std::fmt::Write as FmtWrite;
use std::hint::black_box;
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
use zamani_compiler::frontend::parser::grammar::expressions::{
    parse_expression, ExpressionParserConfig,
};

type BenchResult<T> = Result<T, Box<dyn Error>>;

/// Workload settings are user-configurable. No setting represents a
/// language-level limit or a hardware-capacity assumption.
#[derive(Debug, Clone, Copy)]
struct BenchmarkConfig {
    terms: usize,
    nesting: usize,
    iterations: usize,
}

impl BenchmarkConfig {
    fn from_environment() -> BenchResult<Self> {
        let config = Self {
            terms: read_positive_usize("ZAMANI_EXPR_TERMS", 256)?,
            nesting: read_positive_usize("ZAMANI_EXPR_NESTING", 32)?,
            iterations: read_positive_usize("ZAMANI_BENCH_ITERATIONS", 10)?,
        };

        // Catch arithmetic overflow before generating a workload or
        // calculating a parser budget.
        config
            .nesting
            .checked_add(16)
            .ok_or("nesting value overflows the parser-budget calculation")?;

        config
            .terms
            .checked_mul(16)
            .ok_or("term count overflows the estimated source-size calculation")?;

        Ok(config)
    }

    fn expression_parser_config(self) -> BenchResult<ExpressionParserConfig> {
        let max_nesting = self
            .nesting
            .checked_add(16)
            .ok_or("nesting value overflows the parser-budget calculation")?;

        Ok(ExpressionParserConfig { max_nesting })
    }
}

fn read_positive_usize(name: &str, default: usize) -> BenchResult<usize> {
    let Some(value) = std::env::var_os(name) else {
        return Ok(default);
    };

    let value = value
        .into_string()
        .map_err(|_| format!("{name} must contain valid Unicode text"))?;

    let parsed = value
        .parse::<usize>()
        .map_err(|error| format!("invalid {name} value {value:?}: {error}"))?;

    if parsed == 0 {
        return Err(format!("{name} must be greater than zero").into());
    }

    Ok(parsed)
}

/// Register only the operators used by the generated benchmark expressions.
///
/// Operator spellings and token identities remain owned by the canonical
/// lexer registry. Precedence and associativity remain parser concerns.
fn build_operator_registry() -> BenchResult<OperatorRegistry> {
    let mut registry = OperatorRegistry::new();

    registry.register(OperatorDefinition::new("+", TokenKind::Plus)?)?;
    registry.register(OperatorDefinition::new("*", TokenKind::Star)?)?;

    registry.validate()?;

    Ok(registry)
}

/// Generate a long expression with alternating addition and multiplication.
///
/// Example:
///
///     value_0 + value_1 * value_2 + value_3 * value_4
///
/// The generated identifiers deliberately remain ordinary identifiers:
/// the benchmark does not hard-code hardware names or domain operations.
fn generate_precedence_expression(terms: usize) -> BenchResult<String> {
    if terms == 0 {
        return Err("an expression must contain at least one term".into());
    }

    let estimated_capacity = terms
        .checked_mul(16)
        .ok_or("expression size estimate overflowed")?;

    let mut source = String::new();
    source.try_reserve(estimated_capacity)?;

    for index in 0..terms {
        if index > 0 {
            let operator = if index % 2 == 1 { " + " } else { " * " };
            source.push_str(operator);
        }

        write!(&mut source, "value_{index}")?;
    }

    Ok(source)
}

/// Generate a grouping-heavy expression to exercise nested parsing.
///
/// The configured parser budget is passed separately. This function does
/// not change parser limits or reinterpret them as language restrictions.
fn generate_nested_expression(nesting: usize) -> BenchResult<String> {
    let capacity = nesting
        .checked_mul(2)
        .and_then(|value| value.checked_add("value".len()))
        .ok_or("nested expression size estimate overflowed")?;

    let mut source = String::new();
    source.try_reserve(capacity)?;

    for _ in 0..nesting {
        source.push('(');
    }

    source.push_str("value");

    for _ in 0..nesting {
        source.push(')');
    }

    Ok(source)
}

/// Tokenize source with the canonical scanner and its existing registries.
///
/// The returned tokens borrow their lexemes from `source`. Callers must
/// retain the source for as long as the token vector is used.
fn lex_source<'src>(source: &'src str) -> BenchResult<Vec<Token<'src>>> {
    let operators = build_operator_registry()?;

    let mut scanner = Scanner::new(
        source,
        LexerConfig::default(),
        KeywordRegistry::new(),
        operators,
    )?;

    let mut tokens = Vec::new();

    loop {
        match scanner.next_token()? {
            Some(token) => {
                let reached_eof = token.kind() == TokenKind::Eof;
                tokens.push(token);

                if reached_eof {
                    break;
                }
            }
            None => break,
        }
    }

    Ok(tokens)
}

/// Parse exactly one expression using the canonical expression grammar.
///
/// This intentionally does not invoke a legacy parser, duplicate expression
/// parsing logic, or construct a competing AST representation.
fn parse_tokens(
    tokens: &[Token<'_>],
    parser_config: ExpressionParserConfig,
) -> BenchResult<()> {
    let syntax = parse_expression(black_box(tokens), parser_config)?;
    black_box(syntax);
    Ok(())
}

/// Measure a repeatable operation, including one warm-up execution.
///
/// The reported throughput is based on source bytes presented to the
/// operation. It is a benchmark measurement, not a performance guarantee.
fn measure<F>(
    label: &str,
    source_bytes: usize,
    iterations: usize,
    mut operation: F,
) -> BenchResult<Duration>
where
    F: FnMut() -> BenchResult<()>,
{
    operation()?;

    let start = Instant::now();

    for _ in 0..iterations {
        operation()?;
    }

    let elapsed = start.elapsed();
    let seconds = elapsed.as_secs_f64();

    let operations_per_second = if seconds > 0.0 {
        iterations as f64 / seconds
    } else {
        f64::INFINITY
    };

    let bytes_processed = source_bytes
        .checked_mul(iterations)
        .ok_or("benchmark byte counter overflowed")?;

    let bytes_per_second = if seconds > 0.0 {
        bytes_processed as f64 / seconds
    } else {
        f64::INFINITY
    };

    let nanoseconds_per_operation =
        elapsed.as_nanos() as f64 / iterations as f64;

    println!(
        "{label}: iterations={iterations}, \
         source_bytes={source_bytes}, \
         elapsed={elapsed:?}, \
         ns/op={nanoseconds_per_operation:.0}, \
         expressions/s={operations_per_second:.2}, \
         input_bytes/s={bytes_per_second:.2}"
    );

    Ok(elapsed)
}

fn benchmark_expression(
    label: &str,
    source: &str,
    config: BenchmarkConfig,
) -> BenchResult<()> {
    let parser_config = config.expression_parser_config()?;

    // Validate the input before timing so malformed benchmark inputs cannot
    // produce misleading performance measurements.
    let tokens = lex_source(source)?;
    parse_tokens(&tokens, parser_config)?;

    println!(
        "\nWorkload: {label}; source_bytes={}; tokens={}",
        source.len(),
        tokens.len()
    );

    // Lexer-only: includes scanner and registry construction.
    measure(
        &format!("{label}/lex"),
        source.len(),
        config.iterations,
        || {
            let tokens = lex_source(black_box(source))?;
            black_box(tokens.len());
            Ok(())
        },
    )?;

    // Parser-only: tokenization is performed once, outside the timed loop.
    measure(
        &format!("{label}/parse"),
        source.len(),
        config.iterations,
        || parse_tokens(&tokens, parser_config),
    )?;

    // End-to-end frontend expression path.
    measure(
        &format!("{label}/lex+parse"),
        source.len(),
        config.iterations,
        || {
            let tokens = lex_source(black_box(source))?;
            parse_tokens(&tokens, parser_config)
        },
    )?;

    Ok(())
}

fn run() -> BenchResult<()> {
    let config = BenchmarkConfig::from_environment()?;

    println!("Zamani canonical expression benchmark");
    println!("Rust baseline: 1.97+");
    println!("Unsafe code: forbidden");
    println!("Terms: {}", config.terms);
    println!("Requested nesting: {}", config.nesting);
    println!("Iterations: {}", config.iterations);
    println!(
        "Parser nesting budget: {}",
        config.expression_parser_config()?.max_nesting
    );

    let precedence_source = generate_precedence_expression(config.terms)?;
    benchmark_expression("precedence", &precedence_source, config)?;

    let nested_source = generate_nested_expression(config.nesting)?;
    benchmark_expression("nested", &nested_source, config)?;

    println!("\nBenchmark completed successfully.");
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("expression benchmark failed: {error}");

        let mut cause = error.source();
        while let Some(source) = cause {
            eprintln!("  caused by: {source}");
            cause = source.source();
        }

        std::process::exit(1);
    }
}