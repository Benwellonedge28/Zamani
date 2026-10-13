
#![forbid(unsafe_code)]

//! End-to-end Zamani lexer and parser benchmark.
//!
//! Path:
//!     src/frontend/parser/benches/lex_parse.rs
//!
//! Rust:
//!     1.97 or later
//! Edition:
//!     2021
//!
//! # Purpose
//!
//! Measure the existing repository lexer and parser on generated Zamani
//! source programs of increasing size. Report lexer throughput separately
//! from the combined lexer-plus-parser pipeline.
//!
//! # Architecture
//!
//! Generated source
//!     -> SourceFile
//!     -> legacy Lexer
//!     -> legacy Parser
//!     -> parsed program and diagnostics
//!
//! # Important implementation boundary
//!
//! This benchmark intentionally uses crate::lexer and crate::parser,
//! because src/frontend/parser/mod.rs is not yet a functioning parser
//! implementation. It must be migrated to the canonical frontend API
//! after that API and its AST integration are implemented and verified.
//!
//! It does not benchmark ANTLR-generated parser code.
//!
//! # Resource behavior
//!
//! Workload sizes and iteration counts are configurable through environment
//! variables. There is no fixed source-size or hardware-capacity ceiling
//! encoded in this benchmark. Real memory, address-space, time and process
//! limits still apply.
//!
//! # Environment
//!
//! ZAMANI_BENCH_DECLARATIONS
//!     Comma-separated positive declaration counts.
//!     Default: 1,16,256,1024
//!
//! ZAMANI_BENCH_ITERATIONS
//!     Positive iteration count per workload and benchmark mode.
//!     Default: 10
//!
//! ZAMANI_BENCH_WARMUP
//!     Positive iteration count for warm-up.
//!     Default: 2
//!
//! # Safety
//!
//! This file forbids unsafe Rust.

use std::env;
use std::hint::black_box;
use std::process::ExitCode;
use std::sync::Arc;
use std::time::{Duration, Instant};

use zamani_compiler::lexer::{Lexer, TokenType};
use zamani_compiler::parser::Parser;
use zamani_compiler::source_map::{FileId, SourceFile};

const DEFAULT_DECLARATIONS: &str = "1,16,256,1024";
const DEFAULT_ITERATIONS: usize = 10;
const DEFAULT_WARMUP: usize = 2;

#[derive(Debug, Clone)]
struct BenchmarkConfig {
    declaration_counts: Vec<usize>,
    iterations: usize,
    warmup_iterations: usize,
}

#[derive(Debug)]
struct Measurement {
    elapsed: Duration,
    iterations: usize,
    source_bytes: usize,
    token_count: usize,
    diagnostic_count: usize,
}

impl Measurement {
    fn elapsed_per_iteration(&self) -> Duration {
        self.elapsed / u32::try_from(self.iterations).unwrap_or(u32::MAX).max(1)
    }

    fn bytes_per_second(&self) -> f64 {
        let bytes = (self.source_bytes as f64) * (self.iterations as f64);
        let seconds = self.elapsed.as_secs_f64();

        if seconds > 0.0 {
            bytes / seconds
        } else {
            0.0
        }
    }

    fn tokens_per_second(&self) -> f64 {
        let tokens = (self.token_count as f64) * (self.iterations as f64);
        let seconds = self.elapsed.as_secs_f64();

        if seconds > 0.0 {
            tokens / seconds
        } else {
            0.0
        }
    }
}

fn read_positive_usize(name: &str, default: usize) -> Result<usize, String> {
    match env::var(name) {
        Ok(raw) => {
            let value = raw
                .trim()
                .parse::<usize>()
                .map_err(|_| format!("{name} must be a positive integer, got {raw:?}"))?;

            if value == 0 {
                return Err(format!("{name} must be greater than zero"));
            }

            Ok(value)
        }
        Err(env::VarError::NotPresent) => Ok(default),
        Err(error) => Err(format!("cannot read {name}: {error}")),
    }
}

fn read_declaration_counts() -> Result<Vec<usize>, String> {
    let raw = match env::var("ZAMANI_BENCH_DECLARATIONS") {
        Ok(value) => value,
        Err(env::VarError::NotPresent) => DEFAULT_DECLARATIONS.to_owned(),
        Err(error) => {
            return Err(format!(
                "cannot read ZAMANI_BENCH_DECLARATIONS: {error}"
            ));
        }
    };

    let mut counts = Vec::new();

    for component in raw.split(',') {
        let component = component.trim();

        if component.is_empty() {
            return Err(
                "ZAMANI_BENCH_DECLARATIONS contains an empty entry".to_owned(),
            );
        }

        let count = component.parse::<usize>().map_err(|_| {
            format!(
                "invalid declaration count {component:?}; expected a positive integer"
            )
        })?;

        if count == 0 {
            return Err(
                "declaration counts must be greater than zero".to_owned(),
            );
        }

        counts.push(count);
    }

    if counts.is_empty() {
        return Err(
            "ZAMANI_BENCH_DECLARATIONS must contain at least one count".to_owned(),
        );
    }

    // Sort and deduplicate to make output deterministic regardless of
    // the order supplied by the caller.
    counts.sort_unstable();
    counts.dedup();

    Ok(counts)
}

fn load_config() -> Result<BenchmarkConfig, String> {
    Ok(BenchmarkConfig {
        declaration_counts: read_declaration_counts()?,
        iterations: read_positive_usize(
            "ZAMANI_BENCH_ITERATIONS",
            DEFAULT_ITERATIONS,
        )?,
        warmup_iterations: read_positive_usize(
            "ZAMANI_BENCH_WARMUP",
            DEFAULT_WARMUP,
        )?,
    })
}

/// Generate a repeatable corpus using constructs recognized by the
/// repository's existing classical parser.
///
/// Keep this generator intentionally conservative. Domain-specific quantum
/// and HDL workloads belong in their own benchmark corpora once their
/// supported syntax has been verified against the normative grammar.
fn generate_source(declaration_count: usize) -> Result<String, String> {
    let mut source = String::new();

    for index in 0..declaration_count {
        use std::fmt::Write as _;

        // String growth is fallible where the standard library exposes
        // fallible reservation. Allocation failure may still be fatal
        // elsewhere in the process or allocator.
        let declaration = format!(
            "fn bench_{index}() {{ let value = {index}; }}\n"
        );

        source
            .try_reserve(declaration.len())
            .map_err(|error| {
                format!(
                    "unable to reserve source storage for workload \
                     with {declaration_count} declarations: {error}"
                )
            })?;

        source.write_str(&declaration).map_err(|error| {
            format!("unable to append generated source: {error}")
        })?;
    }

    Ok(source)
}

fn make_lexer(source: &str) -> Lexer {
    let source_file = Arc::new(SourceFile::new(
        "<lex-parse-benchmark>".to_owned(),
        source.to_owned(),
    ));

    Lexer::new(FileId::new(1), source_file)
}

/// Consume the entire legacy token stream and count tokens, including
/// the terminal EOF token.
///
/// Lexical diagnostics are collected by the existing lexer. The benchmark
/// reports their count rather than silently treating malformed input as
/// a successful measurement.
fn lex_all(source: &str) -> (usize, usize) {
    let mut lexer = make_lexer(source);
    let mut token_count = 0usize;

    loop {
        let token = lexer.next_token();
        token_count = token_count.saturating_add(1);

        if token.token_type == TokenType::EOF {
            break;
        }

        black_box(&token);
    }

    let diagnostic_count = lexer.errors.len();

    (token_count, diagnostic_count)
}

/// Run the existing parser over a fresh lexer.
///
/// The parser owns the lexer, so lexer diagnostics are not separately
/// exposed by this wrapper. Parser diagnostics are counted directly.
fn parse_all(source: &str) -> (usize, usize) {
    let lexer = make_lexer(source);
    let mut parser = Parser::new(lexer);

    let program = parser.parse_program();
    black_box(&program);

    let diagnostic_count = parser.get_errors().len();

    (0, diagnostic_count)
}

fn measure_lexer(
    source: &str,
    iterations: usize,
) -> Measurement {
    let mut elapsed = Duration::ZERO;
    let mut token_count = 0usize;
    let mut diagnostic_count = 0usize;

    for _ in 0..iterations {
        let start = Instant::now();
        let result = lex_all(black_box(source));
        elapsed += start.elapsed();

        token_count = result.0;
        diagnostic_count = result.1;

        black_box(result);
    }

    Measurement {
        elapsed,
        iterations,
        source_bytes: source.len(),
        token_count,
        diagnostic_count,
    }
}

fn measure_lex_parse(
    source: &str,
    iterations: usize,
) -> Measurement {
    let mut elapsed = Duration::ZERO;
    let mut diagnostic_count = 0usize;

    for _ in 0..iterations {
        let start = Instant::now();
        let result = parse_all(black_box(source));
        elapsed += start.elapsed();

        diagnostic_count = result.1;
        black_box(result);
    }

    // Count tokens separately, outside the combined timing interval.
    // This avoids adding a second lexer pass to the reported pipeline
    // time.
    let (token_count, lexer_diagnostics) = lex_all(source);

    Measurement {
        elapsed,
        iterations,
        source_bytes: source.len(),
        token_count,
        diagnostic_count: diagnostic_count.saturating_add(lexer_diagnostics),
    }
}

fn warm_up(source: &str, iterations: usize) {
    for _ in 0..iterations {
        black_box(lex_all(black_box(source)));
        black_box(parse_all(black_box(source)));
    }
}

fn print_measurement(
    mode: &str,
    declarations: usize,
    measurement: &Measurement,
) {
    println!(
        "{mode:12} \
         declarations={declarations:>8} \
         bytes={:>10} \
         tokens={:>10} \
         iterations={:>6} \
         total_ms={:>12.3} \
         ns_per_iteration={:>14.0} \
         bytes_per_second={:>14.0} \
         tokens_per_second={:>14.0} \
         diagnostics={}",
        measurement.source_bytes,
        measurement.token_count,
        measurement.iterations,
        measurement.elapsed.as_secs_f64() * 1_000.0,
        measurement.elapsed_per_iteration().as_nanos(),
        measurement.bytes_per_second(),
        measurement.tokens_per_second(),
        measurement.diagnostic_count,
    );
}

fn run() -> Result<(), String> {
    let config = load_config()?;

    println!("Zamani lexer/parser benchmark");
    println!("Rust package: zamani");
    println!("Profile: use `cargo bench` for optimized measurements");
    println!(
        "Iterations: {}; warm-up: {}",
        config.iterations, config.warmup_iterations
    );
    println!(
        "Declaration workloads: {:?}",
        config.declaration_counts
    );
    println!();

    println!(
        "{:12} {:>8} {:>10} {:>10} {:>6} {:>12} {:>14} {:>14} {:>14} {}",
        "mode",
        "decls",
        "bytes",
        "tokens",
        "iters",
        "total_ms",
        "ns/iter",
        "bytes/sec",
        "tokens/sec",
        "diagnostics"
    );

    for declaration_count in &config.declaration_counts {
        let source = generate_source(*declaration_count)?;

        warm_up(&source, config.warmup_iterations);

        let lexer_measurement =
            measure_lexer(&source, config.iterations);

        print_measurement(
            "lexer",
            *declaration_count,
            &lexer_measurement,
        );

        let combined_measurement =
            measure_lex_parse(&source, config.iterations);

        print_measurement(
            "lex+parse",
            *declaration_count,
            &combined_measurement,
        );

        // Diagnostics mean the workload may not be exercising the intended
        // valid-input path. Keep the result visible and fail after printing
        // measurements so CI cannot mistake a malformed corpus for a clean
        // correctness run.
        if lexer_measurement.diagnostic_count != 0
            || combined_measurement.diagnostic_count != 0
        {
            return Err(format!(
                "workload with {declaration_count} declarations produced \
                 lexer or parser diagnostics; verify the corpus against \
                 the current implementation and normative grammar"
            ));
        }

        println!();
    }

    Ok(())
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("benchmark error: {error}");
            ExitCode::FAILURE
        }
    }
}
