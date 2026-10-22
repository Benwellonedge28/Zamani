//! Scalability and stress-regression tests for Zamani hybrid computation.
//!
//! Path: `src/hybrid/tests/stress.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: safe Rust only.
//!
//! # Purpose
//!
//! Exercise the repository's existing hybrid-region syntax adapter under
//! configurable workloads, including:
//!
//! - large numbers of domain-qualified invocations;
//! - repeated crossings between different computational domains;
//! - nested control constructs;
//! - deterministic parsing of identical token streams;
//! - malformed input near the end of a large region;
//! - preservation of source spans and element ordering;
//! - empty and small boundary workloads.
//!
//! # Canonical ownership
//!
//! - Hybrid grammar: `grammar/hybrid/hybrid.g4`
//! - Hybrid architecture: `grammar/hybrid/README.md`
//! - Hybrid semantics contract: `grammar/spec/hybrid.md`
//! - Determinism contract: `grammar/spec/determinism.md`
//! - Syntax adapter: `crate::frontend::parser::domains::hybrid`
//! - Token and source-span definitions: `crate::frontend::lexer`
//! - Quantum semantic IR: `crate::quantum::ir`
//! - Existing compilation pipeline: `crate::compiler::hybrid_pipeline`
//! - Existing runtime integration point: `crate::runtime::hybrid_kernel`
//!
//! These tests intentionally do not introduce another AST, semantic model,
//! graph, quantum IR, compiler pipeline, or runtime.
//!
//! # Scalability contract
//!
//! Test workload sizes are controlled by environment variables. The default
//! values are test workloads, not universal language or implementation limits.
//! No fixed maximum number of domains, nodes, operations, or hardware resources
//! is established by this module.
//!
//! An allocation request that cannot be represented or reserved by the test
//! process fails with an explanatory message. A successful test only establishes
//! behavior for the workload that actually ran; it does not prove infinite
//! capacity or guarantee that every target can execute every program.

#![forbid(unsafe_code)]

use crate::frontend::lexer::cursor::CursorPosition;
use crate::frontend::lexer::span::Span;
use crate::frontend::lexer::token::Token;
use crate::frontend::lexer::token_kind::TokenKind;
use crate::frontend::parser::domains::hybrid::{
    parse_hybrid_region, HybridElementKind, HybridSyntaxErrorKind,
};

/// Read a configurable workload size.
///
/// Invalid values are errors rather than silently falling back to a smaller
/// workload. Zero is allowed so the same test can exercise the empty boundary.
fn workload_from_env(variable: &str, default: usize) -> usize {
    match std::env::var(variable) {
        Ok(value) => value.parse::<usize>().unwrap_or_else(|error| {
            panic!(
                "{variable} must be a non-negative integer, got {value:?}: {error}"
            )
        }),
        Err(std::env::VarError::NotPresent) => default,
        Err(error) => panic!("unable to read {variable}: {error}"),
    }
}

/// Reserve storage explicitly so ordinary reservation failures produce a
/// useful test diagnostic instead of being mistaken for a parser failure.
fn reserve_tokens(tokens: &mut Vec<Token<'static>>, additional: usize) {
    tokens.try_reserve_exact(additional).unwrap_or_else(|error| {
        panic!(
            "unable to reserve storage for {additional} additional hybrid-test tokens: \
             {error}; reduce the configured test workload or provide more memory"
        )
    });
}

/// Append a token with a monotonically increasing byte span.
///
/// The spelling is static because the generated stress cases intentionally
/// reuse canonical token spellings rather than building a second lexer.
fn append_token(
    tokens: &mut Vec<Token<'static>>,
    offset: &mut usize,
    spelling: &'static str,
    kind: TokenKind,
) {
    let start = *offset;
    let end = start.checked_add(spelling.len()).unwrap_or_else(|| {
        panic!("hybrid stress input byte offset overflowed usize")
    });

    let span = Span::new(
        CursorPosition::new(start, 1, start + 1),
        CursorPosition::new(end, 1, end + 1),
    );

    tokens.push(Token::borrowed(kind, spelling, span));

    *offset = end.checked_add(1).unwrap_or_else(|| {
        panic!("hybrid stress input byte offset overflowed usize")
    });
}

fn append_identifier(
    tokens: &mut Vec<Token<'static>>,
    offset: &mut usize,
    spelling: &'static str,
) {
    append_token(tokens, offset, spelling, TokenKind::Identifier);
}

fn append_punctuation(
    tokens: &mut Vec<Token<'static>>,
    offset: &mut usize,
    spelling: &'static str,
    kind: TokenKind,
) {
    append_token(tokens, offset, spelling, kind);
}

/// Finish a token stream with the canonical synthetic EOF token.
fn finish_tokens(
    mut tokens: Vec<Token<'static>>,
    offset: usize,
) -> Vec<Token<'static>> {
    let eof_position = CursorPosition::new(offset, 1, offset + 1);
    tokens.push(Token::synthetic(
        TokenKind::Eof,
        Span::at(eof_position),
    ));
    tokens
}

fn empty_region_tokens() -> Vec<Token<'static>> {
    let mut tokens = Vec::new();
    reserve_tokens(&mut tokens, 3);

    let mut offset = 0;
    append_identifier(&mut tokens, &mut offset, "hybrid");
    append_punctuation(
        &mut tokens,
        &mut offset,
        "{",
        TokenKind::LeftBrace,
    );
    append_punctuation(
        &mut tokens,
        &mut offset,
        "}",
        TokenKind::RightBrace,
    );

    finish_tokens(tokens, offset)
}

/// Generate a flat hybrid region containing `count` domain invocations.
///
/// Each invocation is deliberately composed from canonical token kinds.
/// This tests the hybrid adapter, not the lexical correctness of a second
/// hand-written tokenizer.
fn flat_region_tokens(count: usize) -> Vec<Token<'static>> {
    let per_invocation = 7_usize;
    let capacity = count
        .checked_mul(per_invocation)
        .and_then(|value| value.checked_add(3))
        .unwrap_or_else(|| {
            panic!("configured flat-region token count overflows usize")
        });

    let mut tokens = Vec::new();
    reserve_tokens(&mut tokens, capacity);

    let mut offset = 0;

    append_identifier(&mut tokens, &mut offset, "hybrid");
    append_punctuation(
        &mut tokens,
        &mut offset,
        "{",
        TokenKind::LeftBrace,
    );

    for _ in 0..count {
        append_identifier(&mut tokens, &mut offset, "future_domain");
        append_punctuation(
            &mut tokens,
            &mut offset,
            "::",
            TokenKind::DoubleColon,
        );
        append_identifier(&mut tokens, &mut offset, "compute");
        append_punctuation(
            &mut tokens,
            &mut offset,
            "(",
            TokenKind::LeftParen,
        );
        append_identifier(&mut tokens, &mut offset, "input");
        append_punctuation(
            &mut tokens,
            &mut offset,
            ")",
            TokenKind::RightParen,
        );
        append_punctuation(
            &mut tokens,
            &mut offset,
            ";",
            TokenKind::Semicolon,
        );
    }

    append_punctuation(
        &mut tokens,
        &mut offset,
        "}",
        TokenKind::RightBrace,
    );

    finish_tokens(tokens, offset)
}

/// Generate repeated calls across several domain namespaces.
///
/// The names remain open-ended identifiers; this test does not turn the
/// currently known domains into a closed registry.
fn mixed_domain_region_tokens(count: usize) -> Vec<Token<'static>> {
    const DOMAINS: &[&str] = &[
        "classical",
        "quantum",
        "tensor",
        "accelerator",
        "hdl",
        "distributed",
        "future_domain",
    ];

    let per_invocation = 7_usize;
    let capacity = count
        .checked_mul(per_invocation)
        .and_then(|value| value.checked_add(3))
        .unwrap_or_else(|| {
            panic!("configured mixed-domain token count overflows usize")
        });

    let mut tokens = Vec::new();
    reserve_tokens(&mut tokens, capacity);

    let mut offset = 0;

    append_identifier(&mut tokens, &mut offset, "hybrid");
    append_punctuation(
        &mut tokens,
        &mut offset,
        "{",
        TokenKind::LeftBrace,
    );

    for index in 0..count {
        let domain = DOMAINS[index % DOMAINS.len()];

        append_identifier(&mut tokens, &mut offset, domain);
        append_punctuation(
            &mut tokens,
            &mut offset,
            "::",
            TokenKind::DoubleColon,
        );
        append_identifier(&mut tokens, &mut offset, "compute");
        append_punctuation(
            &mut tokens,
            &mut offset,
            "(",
            TokenKind::LeftParen,
        );
        append_identifier(&mut tokens, &mut offset, "input");
        append_punctuation(
            &mut tokens,
            &mut offset,
            ")",
            TokenKind::RightParen,
        );
        append_punctuation(
            &mut tokens,
            &mut offset,
            ";",
            TokenKind::Semicolon,
        );
    }

    append_punctuation(
        &mut tokens,
        &mut offset,
        "}",
        TokenKind::RightBrace,
    );

    finish_tokens(tokens, offset)
}

/// Generate nested `when condition { ... }` constructs.
///
/// Nesting depth is configurable and is not declared to be a language-level
/// maximum. Extremely large nesting workloads may exceed the process's
/// available memory or other implementation resources.
fn nested_control_region_tokens(depth: usize) -> Vec<Token<'static>> {
    let tokens_per_level = 4_usize;
    let capacity = depth
        .checked_mul(tokens_per_level)
        .and_then(|value| value.checked_add(3))
        .unwrap_or_else(|| {
            panic!("configured nested-region token count overflows usize")
        });

    let mut tokens = Vec::new();
    reserve_tokens(&mut tokens, capacity);

    let mut offset = 0;

    append_identifier(&mut tokens, &mut offset, "hybrid");
    append_punctuation(
        &mut tokens,
        &mut offset,
        "{",
        TokenKind::LeftBrace,
    );

    for _ in 0..depth {
        append_identifier(&mut tokens, &mut offset, "when");
        append_identifier(&mut tokens, &mut offset, "condition");
        append_punctuation(
            &mut tokens,
            &mut offset,
            "{",
            TokenKind::LeftBrace,
        );
    }

    for _ in 0..depth {
        append_punctuation(
            &mut tokens,
            &mut offset,
            "}",
            TokenKind::RightBrace,
        );
    }

    append_punctuation(
        &mut tokens,
        &mut offset,
        "}",
        TokenKind::RightBrace,
    );

    finish_tokens(tokens, offset)
}

#[test]
fn handles_empty_and_single_element_boundaries() {
    let empty = parse_hybrid_region(&empty_region_tokens())
        .expect("an empty balanced hybrid region should parse");

    assert!(empty.is_empty());
    assert!(empty.elements().is_empty());

    let single = parse_hybrid_region(&flat_region_tokens(1))
        .expect("a one-invocation hybrid region should parse");

    assert_eq!(single.elements().len(), 1);
    assert_eq!(
        single.elements()[0].kind(),
        HybridElementKind::DomainInvocation
    );
}

#[test]
fn handles_a_configurable_number_of_flat_domain_invocations() {
    let count = workload_from_env("ZAMANI_HYBRID_STRESS_ELEMENTS", 4_096);
    let tokens = flat_region_tokens(count);

    let region = parse_hybrid_region(&tokens)
        .unwrap_or_else(|error| {
            panic!(
                "hybrid parser failed for a flat workload of {count} elements: \
                 {error:?}"
            )
        });

    assert_eq!(
        region.elements().len(),
        count,
        "every generated invocation must be retained"
    );

    assert!(
        region.elements().iter().all(|element| {
            element.kind() == HybridElementKind::DomainInvocation
        }),
        "all generated domain-qualified calls must retain their classification"
    );

    let mut previous_end = region.open_brace_span().end_byte();

    for element in region.elements() {
        let span = element.span();

        assert!(
            span.start_byte() >= previous_end,
            "element source spans must remain ordered and non-overlapping"
        );
        assert!(
            span.start_byte() <= span.end_byte(),
            "every element span must be valid"
        );

        previous_end = span.end_byte();
    }

    assert!(
        previous_end <= region.close_brace_span().start_byte(),
        "all elements must end before the closing brace"
    );
}

#[test]
fn handles_repeated_crossings_between_extensible_domain_names() {
    let count = workload_from_env("ZAMANI_HYBRID_STRESS_CROSSINGS", 1_024);
    let tokens = mixed_domain_region_tokens(count);

    let region = parse_hybrid_region(&tokens)
        .unwrap_or_else(|error| {
            panic!(
                "hybrid parser failed for a mixed-domain workload of \
                 {count} invocations: {error:?}"
            )
        });

    assert_eq!(region.elements().len(), count);

    assert!(
        region.elements().iter().all(|element| {
            element.kind() == HybridElementKind::DomainInvocation
        }),
        "domain-qualified calls must remain recognizable across domain names"
    );

    // The adapter must preserve source order rather than reorder elements
    // according to domain identity or registration order.
    let mut last_start = region.open_brace_span().start_byte();

    for element in region.elements() {
        assert!(element.span().start_byte() >= last_start);
        last_start = element.span().start_byte();
    }
}

#[test]
fn handles_configurable_nested_control_constructs() {
    let depth = workload_from_env("ZAMANI_HYBRID_STRESS_NESTING", 256);
    let tokens = nested_control_region_tokens(depth);

    let region = parse_hybrid_region(&tokens)
        .unwrap_or_else(|error| {
            panic!(
                "hybrid parser failed for nested-control depth {depth}: {error:?}"
            )
        });

    if depth == 0 {
        assert!(region.is_empty());
    } else {
        // The current syntax adapter represents the outer block-bearing
        // construct as one element. It does not create a nested AST here.
        assert_eq!(region.elements().len(), 1);
        assert_eq!(
            region.elements()[0].kind(),
            HybridElementKind::Control
        );
    }
}

#[test]
fn parsing_identical_large_inputs_is_deterministic() {
    let count = workload_from_env("ZAMANI_HYBRID_STRESS_DETERMINISM", 2_048);

    let first_tokens = mixed_domain_region_tokens(count);
    let second_tokens = mixed_domain_region_tokens(count);

    let first = parse_hybrid_region(&first_tokens)
        .expect("first identical input should parse");
    let second = parse_hybrid_region(&second_tokens)
        .expect("second identical input should parse");

    assert_eq!(
        first, second,
        "identical token streams with identical spans must produce equivalent regions"
    );

    assert_eq!(
        first.elements().len(),
        second.elements().len(),
        "repeated parsing must retain the same number of elements"
    );
}

#[test]
fn reports_a_stable_structured_error_for_malformed_input_near_the_end() {
    let count = workload_from_env("ZAMANI_HYBRID_STRESS_MALFORMED_PREFIX", 1_024);
    let mut tokens = flat_region_tokens(count);

    // Insert a mismatched closing bracket immediately before the final region
    // closing brace. The parser must reject it rather than accepting a
    // partially valid prefix as a complete region.
    let close_brace_index = tokens
        .iter()
        .rposition(|token| token.kind() == TokenKind::RightBrace)
        .expect("generated region must contain a closing brace");

    let insertion_offset = tokens[close_brace_index].span().start_byte();
    let malformed_span = Span::new(
        CursorPosition::new(insertion_offset, 1, insertion_offset + 1),
        CursorPosition::new(insertion_offset + 1, 1, insertion_offset + 2),
    );

    tokens.insert(
        close_brace_index,
        Token::borrowed(
            TokenKind::RightBracket,
            "]",
            malformed_span,
        ),
    );

    let first_error = parse_hybrid_region(&tokens)
        .expect_err("mismatched closing delimiters must be rejected");

    let second_error = parse_hybrid_region(&tokens)
        .expect_err("repeated malformed input must also be rejected");

    assert_eq!(
        first_error.kind(),
        HybridSyntaxErrorKind::UnexpectedClosingDelimiter
    );

    assert_eq!(
        first_error.kind(),
        second_error.kind(),
        "repeated invalid inputs must preserve the structured error category"
    );

    assert_eq!(
        first_error.span(),
        second_error.span(),
        "repeated invalid inputs must preserve the diagnostic source span"
    );
}

#[test]
fn hybrid_grammar_documents_open_ended_composition() {
    let grammar = include_str!("../../../grammar/hybrid/hybrid.g4");
    let architecture = include_str!("../../../grammar/hybrid/README.md");
    let specification = include_str!("../../../grammar/spec/hybrid.md");
    let determinism = include_str!("../../../grammar/spec/determinism.md");

    assert!(
        grammar.contains("hybridDeclaration")
            && grammar.contains("hybridStatement"),
        "the hybrid grammar must retain its declared composition entry points"
    );

    assert!(
        architecture.contains("POCO-REAF")
            || architecture.contains(
                "Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever"
            ),
        "the hybrid architecture must document the portability contract"
    );

    assert!(
        specification.contains("quantum::ir"),
        "the hybrid specification must retain canonical quantum IR ownership"
    );

    assert!(
        determinism.to_lowercase().contains("determin"),
        "the determinism specification must document determinism requirements"
    );

    for forbidden in [
        "MAX_QUBITS",
        "MAX_CPUS",
        "MAX_GPUS",
        "MAX_FPGAS",
        "MAX_NODES",
        "MAX_MEMORY",
        "MAX_TENSOR_RANK",
        "MAX_REGISTER_WIDTH",
        "MAX_NETWORK_SIZE",
    ] {
        assert!(
            !grammar.contains(forbidden),
            "hybrid grammar must not establish the universal ceiling {forbidden}"
        );
    }
}