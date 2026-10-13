#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! Zamani expression-parser benchmarks.
//!
//! File: src/frontend/parser/benches/expressions.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # PURPOSE
//!
//! Measure the existing domain-neutral expression parser across increasing
//! expression sizes. Report input size, token count, elapsed time, throughput,
//! and normalized time per token.
//!
//! # AUTHORITY
//!
//! - `grammar/expressions/precedence.md` defines normative precedence.
//! - `grammar/spec/syntax.md` defines the broader syntax contract.
//! - `grammar/Zamani.g4` defines the root grammar composition.
//! - `grammar/expressions/expressions.g4` and its subordinate grammar modules
//!   define expression syntax.
//! - `src/frontend/lexer/token.rs` owns the canonical token representation.
//! - `src/frontend/lexer/token_kind.rs` owns token identities.
//! - `src/frontend/lexer/span.rs` owns canonical source spans.
//! - `src/frontend/parser/grammar/expressions.rs` owns the expression parser
//!   benchmarked here.
//!
//! # OWNERSHIP
//!
//! This file owns benchmark workload generation, timing, reporting, and
//! benchmark-specific validation. It does not own production parsing,
//! tokenization, precedence, AST conversion, or compiler semantics.
//!
//! # INTEGRATION
//!
//! The benchmark invokes `grammar::expressions::parse_expression` directly.
//! It creates canonical tokens before starting the timed region, ensuring
//! the measurements isolate expression parsing rather than token creation.
//!
//! The benchmark is intended to be included by a harness-free Cargo benchmark
//! target. See the integration notes following this file.
//!
//! # SCALABILITY
//!
//! Workload sizes and iteration counts are configurable through environment
//! variables. The built-in workload sizes are sample points, not language
//! limits. There is no hardware-capacity ceiling or claim of literally
//! infinite processing capacity.
//!
//! The workload and token stream consume memory proportional to their size.
//! Actual execution is bounded by host resources and the expression parser's
//! configured nesting budget.
//!
//! # NO UNSAFE
//!
//! No unsafe Rust, global mutable benchmark state, or external benchmark
//! dependency is required.

use std::hint::black_box;
use std::time::{Duration, Instant};

use zamani_compiler::frontend::lexer::cursor::CursorPosition;
use zamani_compiler::frontend::lexer::span::Span;
use zamani_compiler::frontend::lexer::token::Token;
use zamani_compiler::frontend::lexer::token_kind::TokenKind;
use zamani_compiler::frontend::parser::grammar::expressions::{
    parse_expression, ExpressionParserConfig,
};

/// One prepared workload.
///
/// Source and tokens are owned by the same workload. Tokens borrow their
/// lexemes from `source`, so `tokens` must not outlive that source.
struct Workload {
    source: String,
    tokens: Vec<Token<'static>>,
    terms: usize,
}

impl Workload {
    /// Generate an arithmetic expression using canonical lexer tokens.
    ///
    /// The generated expression alternates addition and multiplication:
    ///
    ///     x + x * x + x * x + x ...
    ///
    /// This exercises identifiers, infix operators, and precedence handling
    /// without introducing hardware-specific or domain-specific assumptions.
    fn new(terms: usize) -> Result<Self, String> {
        if terms == 0 {
            return Err("expression term count must be greater than zero".into());
        }

        let mut source = String::new();

        for index in 0..terms {
            if index != 0 {
                source.push_str(if index % 2 == 1 { " + " } else { " * " });
            }
            source.push('x');
        }

        // Build owned tokens first, then convert them to 'static tokens.
        // This avoids a self-referential struct while preserving zero-copy
        // lexemes during the timed parse.
        let mut owned_tokens = Vec::new();
        let bytes = source.as_bytes();
        let mut offset = 0usize;

        while offset < bytes.len() {
            if bytes[offset].is_ascii_whitespace() {
                offset += 1;
                continue;
            }

            let start = offset;
            let kind = match bytes[offset] {
                b'x' => {
                    offset += 1;
                    TokenKind::Identifier
                }
                b'+' => {
                    offset += 1;
                    TokenKind::Plus
                }
                b'*' => {
                    offset += 1;
                    TokenKind::Star
                }
                other => {
                    return Err(format!(
                        "benchmark generator produced unexpected byte {other:?}"
                    ));
                }
            };

            let span = span_for(start, offset)?;
            let lexeme = source[start..offset].to_owned();

            owned_tokens.push(Token::owned(kind, lexeme, span));
        }

        let eof_span = span_for(source.len(), source.len())?;
        owned_tokens.push(Token::owned(TokenKind::Eof, String::new(), eof_span));

        // The parser accepts borrowed canonical tokens. Convert the owned
        // tokens to static-lexeme tokens so the workload can own both values
        // without self-referential borrowing.
        let tokens = owned_tokens
            .into_iter()
            .map(Token::into_owned)
            .collect();

        Ok(Self {
            source,
            tokens,
            terms,
        })
    }

    fn token_count(&self) -> usize {
        self.tokens.len()
    }

    fn source_bytes(&self) -> usize {
        self.source.len()
    }
}

/// Construct a canonical source span for a single-line ASCII workload.
///
/// The benchmark deliberately generates ASCII so byte offsets and
/// one-based scalar columns agree. Unicode position handling belongs in
/// the lexer and parser correctness tests, not this microbenchmark.
fn span_for(start: usize, end: usize) -> Result<Span, String> {
    let start_position = CursorPosition::new(start, 1, start.saturating_add(1));
    let end_position = CursorPosition::new(end, 1, end.saturating_add(1));

    Span::try_new(start_position, end_position)
        .map_err(|error| format!("invalid generated source span: {error}"))
}

/// Run a single parse and return the elapsed wall-clock duration.
///
/// The parser result is passed through `black_box` so the optimizer cannot
/// trivially remove the result from the benchmark.
fn time_parses(
    workload: &Workload,
    iterations: usize,
    config: ExpressionParserConfig,
) -> Result<Duration, String> {
    if iterations == 0 {
        return Err("iteration count must be greater than zero".into());
    }

    // Warm-up validates the workload and reduces first-use noise.
    black_box(
        parse_expression(black_box(&workload.tokens), config)
            .map_err(|error| format!("expression warm-up failed: {error}"))?,
    );

    let start = Instant::now();

    for _ in 0..iterations {
        let parsed = parse_expression(black_box(&workload.tokens), config)
            .map_err(|error| format!("expression parse failed: {error}"))?;

        black_box(parsed);
    }

    Ok(start.elapsed())
}

/// Read a positive integer from the environment, falling back to a default.
///
/// Invalid values fail explicitly rather than silently selecting a different
/// benchmark workload.
fn positive_env_usize(name: &str, default: usize) -> Result<usize, String> {
    match std::env::var(name) {
        Ok(value) => {
            let parsed = value
                .parse::<usize>()
                .map_err(|_| format!("{name} must be a positive integer"))?;

            if parsed == 0 {
                return Err(format!("{name} must be greater than zero"));
            }

            Ok(parsed)
        }
        Err(std::env::VarError::NotPresent) => Ok(default),
        Err(error) => Err(format!("cannot read {name}: {error}")),
    }
}

/// Benchmark a prepared expression workload and print reproducible metrics.
fn benchmark_workload(
    terms: usize,
    iterations: usize,
    config: ExpressionParserConfig,
) -> Result<(), String> {
    let workload = Workload::new(terms)?;
    let elapsed = time_parses(&workload, iterations, config)?;

    let seconds = elapsed.as_secs_f64();
    let parses_per_second = if seconds > 0.0 {
        iterations as f64 / seconds
    } else {
        f64::INFINITY
    };

    let total_tokens = workload
        .token_count()
        .checked_mul(iterations)
        .ok_or_else(|| "total token accounting overflowed usize".to_owned())?;

    let tokens_per_second = if seconds > 0.0 {
        total_tokens as f64 / seconds
    } else {
        f64::INFINITY
    };

    let nanoseconds_per_parse = elapsed.as_nanos() as f64 / iterations as f64;

    let nanoseconds_per_token = if total_tokens > 0 {
        elapsed.as_nanos() as f64 / total_tokens as f64
    } else {
        0.0
    };

    println!(
        "{:>10} {:>10} {:>12} {:>14} {:>16.2} {:>16.2} {:>16.4}",
        workload.terms,
        workload.token_count(),
        workload.source_bytes(),
        elapsed.as_secs_f64(),
        parses_per_second,
        tokens_per_second,
        nanoseconds_per_parse,
    );

    println!(
        "             normalized_ns_per_token={:.4}",
        nanoseconds_per_token
    );

    Ok(())
}

fn run() -> Result<(), String> {
    // These defaults are representative benchmark sample points, not parser
    // limits. Set ZAMANI_BENCH_MAX_TERMS to explore larger workloads.
    let max_terms = positive_env_usize("ZAMANI_BENCH_MAX_TERMS", 4096)?;
    let iterations = positive_env_usize("ZAMANI_BENCH_ITERS", 100)?;

    let config = ExpressionParserConfig::default();

    println!("Zamani expression parser benchmark");
    println!("Rust edition: 2021");
    println!("Configured iterations per workload: {iterations}");
    println!("Largest requested expression: {max_terms} terms");
    println!(
        "{:>10} {:>10} {:>12} {:>14} {:>16} {:>16} {:>16}",
        "terms",
        "tokens",
        "source_bytes",
        "elapsed_secs",
        "parses/sec",
        "tokens/sec",
        "ns/parse",
    );

    // Geometric growth highlights scaling trends without making the runtime
    // depend on an arbitrary number of benchmark cases.
    let mut terms = 1usize;

    loop {
        benchmark_workload(terms, iterations, config)?;

        if terms == max_terms {
            break;
        }

        let next = terms.saturating_mul(2);

        // Always include the requested endpoint, even when it is not a power
        // of two. Saturating arithmetic prevents size progression overflow.
        terms = if next >= max_terms || next <= terms {
            max_terms
        } else {
            next
        };
    }

    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("expression benchmark failed: {error}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_empty_expression_workload() {
        assert!(Workload::new(0).is_err());
    }

    #[test]
    fn generated_workload_has_expected_shape() {
        let workload = Workload::new(3).expect("workload generation should succeed");

        assert_eq!(workload.terms, 3);
        assert_eq!(workload.token_count(), 6);
        assert_eq!(workload.source, "x + x * x");
    }

    #[test]
    fn generated_expression_parses() {
        let workload = Workload::new(5).expect("workload generation should succeed");

        let parsed = parse_expression(
            &workload.tokens,
            ExpressionParserConfig::default(),
        );

        assert!(
            parsed.is_ok(),
            "generated arithmetic expression should parse: {parsed:?}"
        );
    }

    #[test]
    fn benchmark_rejects_zero_iterations() {
        let workload = Workload::new(1).expect("workload generation should succeed");

        assert!(time_parses(
            &workload,
            0,
            ExpressionParserConfig::default(),
        )
        .is_err());
    }
}