#![forbid(unsafe_code)]

//! Adversarial and property-oriented tests for the Zamani frontend parser.
//!
//! File: src/frontend/parser/tests/fuzz.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Exercise the parser with deterministic adversarial source inputs, including:
//! - empty, truncated, malformed, and delimiter-heavy source;
//! - arbitrary Unicode and invalid UTF-8 byte sequences converted to valid
//!   Rust strings without losing the fact that the original input was hostile;
//! - long identifiers, numeric spellings, strings, comments, and operators;
//! - deeply nested and ambiguously structured expressions;
//! - classical, quantum, HDL, hybrid, resource, capability, and extension syntax;
//! - repeated invocations and parser entry-point isolation.
//!
//! # Architectural boundaries
//!
//! This file owns test-input generation and reusable parser fuzz-contract checks.
//! It does not own the lexer, grammar, parser implementation, AST, diagnostics,
//! resource accounting, semantic analysis, or target-specific behavior.
//!
//! The canonical parser API is `super::super::api::ParserService`.
//! Parser configuration comes from `super::super::config::ParserConfig`.
//! The parser must consume the canonical lexer and preserve the repository's
//! source-span and AST contracts.
//!
//! # Resource policy
//!
//! Fuzz-test generation uses explicit test-work budgets so an ordinary test run
//! remains practical. These budgets are NOT Zamani language limits.
//!
//! A dedicated fuzzing job can increase input sizes and execution time according
//! to the resources available to that job. No hardware-capacity ceiling is
//! introduced into the language or parser by this test harness.
//!
//! # Integration status
//!
//! `ParserService` is currently an interface, not a concrete parser constructor.
//! The generic contract harness below is ready to be called from the parser's
//! integration tests once a concrete implementation exists. This file does not
//! fabricate a parser implementation or assume unverified AST constructors.
//!
//! # Required properties
//!
//! For every tested input, parser entry points must not panic on user-controlled
//! syntax. Repeated calls must not leak mutable state between parse sessions.
//! Errors, incomplete inputs, recovery, and configured budget exhaustion are
//! valid outcomes; panics, hangs, uncontrolled allocation, and silent truncation
//! are not.
//!
//! This test module complements, but does not replace, coverage-guided fuzzing,
//! sanitization, stress tests, grammar conformance tests, and benchmarks.

use std::panic::{catch_unwind, AssertUnwindSafe};

use super::super::api::ParserService;
use super::super::config::ParserConfig;

/// Upper bound for generated corpus entries in the ordinary unit-test suite.
///
/// This bounds test-work generation only; it is not a source-language limit.
const DEFAULT_CORPUS_CASES: usize = 96;

/// Upper bound for the generated test-input byte length.
///
/// Larger inputs belong in configurable stress or coverage-guided fuzz jobs.
const DEFAULT_INPUT_BYTES: usize = 16 * 1024;

/// Reproducible seeds spanning core syntax and extension boundaries.
///
/// These strings are adversarial test inputs, not normative examples of valid
/// Zamani syntax. The normative grammar and language specification determine
/// which inputs should succeed.
const SEEDS: &[&str] = &[
    "",
    " ",
    "\n",
    "\0",
    ";",
    "}",
    "{",
    "(",
    ")",
    "[",
    "]",
    "fn",
    "fn main(",
    "fn main() {",
    "let = ;",
    "if { }",
    "a +",
    "a * * b",
    "a < b > c",
    "a::b::<c>",
    "\"",
    "'",
    "\"unterminated",
    "/* unterminated",
    "/* /* nested */",
    "0",
    "00",
    "0x",
    "0b102",
    "1e",
    "1e+",
    "1..=2",
    "true && false",
    "match value {",
    "struct S { field:",
    "enum E { A, B }",
    "trait T {",
    "impl T for S {",
    "generic::<T, U>",
    "quantum",
    "quantum circuit",
    "quantum operation",
    "measure",
    "qubit",
    "requires qubits >= n;",
    "requires memory >= required_memory;",
    "requires capability(\"quantum.measurement\");",
    "requires capability(\"gpu.compute\");",
    "requires topology(required_topology);",
    "prefer capability(\"tensor.compute\");",
    "constrain resource >= requirement;",
    "allow capability(\"quantum.measurement\");",
    "forbid capability(\"unsupported.operation\");",
    "module hardware {",
    "signal data;",
    "process @(posedge clock) {",
    "always_comb {",
    "assign output = input;",
    "parallel {",
    "actor Worker {",
    "distributed topology(required_topology) {",
    "tensor<Scalar, rank> value;",
    "effect io;",
    "requires input != null;",
    "ensures result >= 0;",
    "assert invariant;",
    "infer expression;",
    "deduce conclusion;",
    "reason from evidence;",
    "learn from observations;",
    "adapt under policy;",
    "reflect capability(\"reflection.inspect\");",
    "macro generated($input) {",
    "operation \"vendor.namespace.operation\"(targets = q);",
    "let λ = 1;",
    "let 日本語 = 2;",
    "let 🚀 = 3;",
    "let café = 4;",
    "let e\u{301} = 5;",
    "// comment\nfn f() {}",
    "/* outer /* inner */ tail */",
    "/*",
    "*/",
    "))))))))",
    "{{{{{{",
    "[[[[[[",
    ">>>>>>>>",
    "<<<<<<<<",
    "&&&&&&&&",
    "||||||||",
    "????????",
    "@@@@####",
    "０１２３",
    "🧪🧬⚛️",
    "\u{FEFF}fn main() {}",
];

/// Returns the deterministic seed corpus.
///
/// Each result owns its text so test callers do not depend on global mutable
/// state or the lifetime of the seed table.
#[must_use]
pub fn seed_corpus() -> Vec<String> {
    SEEDS.iter().map(|seed| (*seed).to_owned()).collect()
}

/// Adds deterministic, hostile variations to the seed corpus.
///
/// The generated corpus is repeatable across runs and does not depend on a
/// random-number generator, wall-clock time, environment variables, or machine
/// characteristics.
///
/// Invalid UTF-8 byte sequences are deliberately included as source-byte
/// mutations. Since the parser API accepts `&str`, those byte sequences are
/// converted with `String::from_utf8_lossy`. Raw-byte validation belongs to the
/// byte-oriented source-loading layer; parser fuzzing exercises valid Rust
/// strings containing replacement characters as well.
///
/// The corpus generator is bounded independently of the parser's language
/// capabilities. Increasing these test budgets must not require parser changes.
#[must_use]
pub fn adversarial_corpus(
    max_cases: usize,
    max_input_bytes: usize,
) -> Vec<String> {
    if max_cases == 0 {
        return Vec::new();
    }

    let input_limit = max_input_bytes;
    let mut corpus = Vec::new();

    let mut push_bounded = |candidate: String| {
        if corpus.len() >= max_cases {
            return;
        }

        if candidate.len() <= input_limit {
            corpus.push(candidate);
        }
    };

    for seed in SEEDS {
        if corpus.len() >= max_cases {
            break;
        }

        push_bounded((*seed).to_owned());
    }

    if corpus.len() >= max_cases || input_limit == 0 {
        return corpus;
    }

    // Repeated delimiter, operator, Unicode, and comment sequences stress
    // cursor movement, recovery progress, nesting accounting, and spans.
    const FRAGMENTS: &[&str] = &[
        "(",
        ")",
        "{",
        "}",
        "[",
        "]",
        "<",
        ">",
        "::",
        "=>",
        "->",
        "&&",
        "||",
        "/*",
        "*/",
        "//",
        "\"",
        "'",
        "\\",
        "λ",
        "界",
        "🧪",
        "\0",
        "\u{FFFD}",
        "\u{200D}",
        "\u{202E}",
    ];

    for fragment in FRAGMENTS {
        if corpus.len() >= max_cases {
            break;
        }

        let repetitions = input_limit / fragment.len().max(1);
        if repetitions == 0 {
            continue;
        }

        let mut candidate = String::new();

        for _ in 0..repetitions {
            if candidate.len().saturating_add(fragment.len()) > input_limit {
                break;
            }

            candidate.push_str(fragment);
        }

        if !candidate.is_empty() {
            push_bounded(candidate);
        }
    }

    // Deterministic byte patterns target UTF-8 boundaries, NUL handling,
    // replacement-character handling, and long malformed token spellings.
    const BYTE_PATTERNS: &[&[u8]] = &[
        &[0x00],
        &[0x7F],
        &[0x80],
        &[0xC0, 0xAF],
        &[0xE0, 0x80, 0x80],
        &[0xF0, 0x80, 0x80, 0x80],
        &[0xF4, 0x90, 0x80, 0x80],
        &[b'/', b'*', b'/', b'*'],
        &[b'(', b'[', b'{', b'}', b']', b')'],
        &[b'0', b'x', b'F', b'F', b'F', b'F'],
        &[b'"', b'\\', b'u', b'{', b'1', b'1', b'0', b'0', b'0', b'}'],
    ];

    for pattern in BYTE_PATTERNS {
        if corpus.len() >= max_cases {
            break;
        }

        if pattern.len() > input_limit {
            continue;
        }

        let mut bytes = Vec::new();

        while bytes.len().saturating_add(pattern.len()) <= input_limit {
            bytes.extend_from_slice(pattern);
        }

        if !bytes.is_empty() {
            push_bounded(String::from_utf8_lossy(&bytes).into_owned());
        }
    }

    corpus
}

/// Runs the reusable no-panic contract against a concrete parser.
///
/// Integrate this helper into `tests/api.rs` or the parser's concrete
/// implementation tests. `service` must be the same implementation used by
/// production parsing; a mock cannot establish grammar correctness.
///
/// The helper intentionally accepts any `ParserService` implementation and
/// makes no assumptions about the AST's concrete node types.
///
/// Each entry point is isolated in its own unwind boundary so a failure
/// identifies which parser operation violated the contract. Panics are caught
/// only to turn them into a useful test failure, not to treat them as success.
///
/// # Example integration
///
/// ```ignore
/// #[test]
/// fn production_parser_handles_adversarial_source() {
///     let parser = ConcreteParser::new();
///     let config = ParserConfig::default();
///     super::fuzz::assert_parser_handles_adversarial_corpus(&parser, &config);
/// }
/// ```
///
/// Replace `ConcreteParser` with the actual production implementation after
/// that implementation and its constructor have been verified.
pub fn assert_parser_handles_adversarial_corpus<S>(
    service: &S,
    config: &ParserConfig,
)
where
    S: ParserService,
{
    let corpus = adversarial_corpus(
        DEFAULT_CORPUS_CASES,
        DEFAULT_INPUT_BYTES,
    );

    for (case_index, source) in corpus.iter().enumerate() {
        let program_result = catch_unwind(AssertUnwindSafe(|| {
            let _ = service.parse_source(source, config);
        }));

        assert!(
            program_result.is_ok(),
            "parse_source panicked for adversarial case {case_index}: \
             input length {} bytes",
            source.len(),
        );

        let expression_result = catch_unwind(AssertUnwindSafe(|| {
            let _ = service.parse_expression(source, config);
        }));

        assert!(
            expression_result.is_ok(),
            "parse_expression panicked for adversarial case {case_index}: \
             input length {} bytes",
            source.len(),
        );

        let type_result = catch_unwind(AssertUnwindSafe(|| {
            let _ = service.parse_type(source, config);
        }));

        assert!(
            type_result.is_ok(),
            "parse_type panicked for adversarial case {case_index}: \
             input length {} bytes",
            source.len(),
        );

        let item_result = catch_unwind(AssertUnwindSafe(|| {
            let _ = service.parse_item(source, config);
        }));

        assert!(
            item_result.is_ok(),
            "parse_item panicked for adversarial case {case_index}: \
             input length {} bytes",
            source.len(),
        );
    }
}

/// Checks that repeatedly invoking the same entry point does not panic.
///
/// This is a baseline isolation check. Full state-isolation verification also
/// needs assertions on result status, diagnostics, AST equivalence, and
/// extension-registry behavior in the concrete parser integration suite.
pub fn assert_parser_repeated_calls_are_panic_free<S>(
    service: &S,
    config: &ParserConfig,
    source: &str,
    repetitions: usize,
)
where
    S: ParserService,
{
    for iteration in 0..repetitions {
        let outcome = catch_unwind(AssertUnwindSafe(|| {
            let _ = service.parse_source(source, config);
        }));

        assert!(
            outcome.is_ok(),
            "parse_source panicked on repetition {iteration}",
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seed_corpus_is_nonempty_and_repeatable() {
        let first = seed_corpus();
        let second = seed_corpus();

        assert!(!first.is_empty());
        assert_eq!(first, second);
    }

    #[test]
    fn generated_corpus_is_repeatable() {
        let first = adversarial_corpus(64, 4096);
        let second = adversarial_corpus(64, 4096);

        assert_eq!(first, second);
    }

    #[test]
    fn generated_corpus_respects_its_test_budgets() {
        let max_cases = 23;
        let max_input_bytes = 512;

        let corpus = adversarial_corpus(max_cases, max_input_bytes);

        assert!(corpus.len() <= max_cases);
        assert!(
            corpus
                .iter()
                .all(|source| source.len() <= max_input_bytes)
        );
    }

    #[test]
    fn zero_case_budget_produces_no_inputs() {
        assert!(adversarial_corpus(0, 1024).is_empty());
    }

    #[test]
    fn zero_byte_budget_produces_only_empty_seeds_if_present() {
        let corpus = adversarial_corpus(16, 0);

        assert!(corpus.iter().all(String::is_empty));
    }

    #[test]
    fn all_generated_inputs_are_valid_rust_strings() {
        for source in adversarial_corpus(96, 4096) {
            assert!(std::str::from_utf8(source.as_bytes()).is_ok());
        }
    }

    #[test]
    fn corpus_covers_unicode_and_malformed_constructs() {
        let corpus = adversarial_corpus(96, 16 * 1024);

        assert!(corpus.iter().any(|s| s.contains('λ')));
        assert!(corpus.iter().any(|s| s.contains('界')));
        assert!(corpus.iter().any(|s| s.contains('\0')));
        assert!(corpus.iter().any(|s| s.contains("/*")));
        assert!(corpus.iter().any(|s| s.contains("quantum")));
        assert!(corpus.iter().any(|s| s.contains("capability")));
    }

    #[test]
    fn source_inputs_preserve_unicode_scalar_boundaries() {
        let source = "café 日本語 λ 🧪 e\u{301}";

        for (index, _) in source.char_indices() {
            assert!(source.is_char_boundary(index));
        }

        assert!(source.is_char_boundary(source.len()));
    }

    #[test]
    fn large_test_budget_does_not_overflow_generation_accounting() {
        // A large requested count must remain safe even when the generator
        // can only produce its finite seed and fragment corpus.
        let corpus = adversarial_corpus(usize::MAX, 128);

        assert!(corpus.iter().all(|source| source.len() <= 128));
    }
}