
#![forbid(unsafe_code)]

//! Hybrid syntax and architecture conformance tests for Zamani.
//!
//! File: src/hybrid/tests/conformance.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: safe Rust only.
//!
//! # Purpose
//!
//! Verify the currently available hybrid syntax adapter against the
//! normative hybrid specification and the hybrid grammar's ownership
//! boundaries.
//!
//! These tests intentionally do not claim to verify semantic analysis,
//! quantum execution, hardware discovery, target lowering, scheduling,
//! routing, or runtime execution. Those require functioning canonical
//! integration APIs and must be tested by their owning subsystems.
//!
//! # Canonical owners
//!
//! - Grammar: grammar/hybrid/hybrid.g4
//! - Normative contract: grammar/spec/hybrid.md
//! - Syntax adapter: frontend::parser::domains::hybrid
//! - Canonical tokens and spans: frontend::lexer
//! - Quantum semantics: quantum::ir
//! - Existing hybrid compilation pipeline: compiler::hybrid_pipeline
//! - Existing runtime integration point: runtime::hybrid_kernel
//!
//! The test suite must not define another AST, semantic model, quantum IR,
//! compiler pipeline, or runtime.
//!
//! # Integration prerequisite
//!
//! The crate must expose `crate::frontend`, and the hybrid module must
//! register this test module under `#[cfg(test)]`. See the integration
//! notes accompanying this file.

use crate::frontend::lexer::cursor::CursorPosition;
use crate::frontend::lexer::span::Span;
use crate::frontend::lexer::token::{Token, TokenChannel};
use crate::frontend::lexer::token_kind::TokenKind;
use crate::frontend::parser::domains::hybrid::{
    parse_hybrid_region, HybridElementKind, HybridSyntaxErrorKind,
};

/// Build a token stream for the syntax adapter without introducing a
/// competing lexer.
///
/// Every item carries its canonical token kind and original spelling.
/// The helper inserts whitespace between items to keep source offsets
/// unambiguous. It appends the canonical EOF token.
///
/// The spelling must agree with the supplied token kind for the cases
/// under test. Full lexical correctness belongs to the lexer test suite.
fn token_stream(items: &[(&'static str, TokenKind)]) -> Vec<Token<'static>> {
    let mut result = Vec::with_capacity(items.len() + 1);
    let mut offset = 0_usize;

    for &(spelling, kind) in items {
        let start = offset;
        let end = start + spelling.len();

        let span = Span::new(
            CursorPosition::new(start, 1, start + 1),
            CursorPosition::new(end, 1, end + 1),
        );

        result.push(Token::borrowed(kind, spelling, span));
        offset = end + 1;
    }

    let eof_position = CursorPosition::new(offset, 1, offset + 1);
    result.push(Token::synthetic(TokenKind::Eof, Span::at(eof_position)));
    result
}

fn identifier(value: &'static str) -> (&'static str, TokenKind) {
    (value, TokenKind::Identifier)
}

fn punctuation(value: &'static str, kind: TokenKind) -> (&'static str, TokenKind) {
    (value, kind)
}

fn empty_region_tokens() -> Vec<Token<'static>> {
    token_stream(&[
        identifier("hybrid"),
        punctuation("{", TokenKind::LeftBrace),
        punctuation("}", TokenKind::RightBrace),
    ])
}

fn simple_quantum_invocation_tokens() -> Vec<Token<'static>> {
    token_stream(&[
        identifier("hybrid"),
        punctuation("{", TokenKind::LeftBrace),
        identifier("quantum"),
        punctuation("::", TokenKind::DoubleColon),
        identifier("measure"),
        punctuation("(", TokenKind::LeftParen),
        identifier("state"),
        punctuation(")", TokenKind::RightParen),
        punctuation(";", TokenKind::Semicolon),
        punctuation("}", TokenKind::RightBrace),
    ])
}

#[test]
fn accepts_an_empty_hybrid_region() {
    let tokens = empty_region_tokens();

    let region = parse_hybrid_region(&tokens)
        .expect("an empty, balanced hybrid region must be accepted");

    assert!(region.is_empty());
    assert!(region.elements().is_empty());
    assert!(region.span().start_byte() <= region.span().end_byte());
}

#[test]
fn recognizes_a_domain_qualified_invocation() {
    let tokens = simple_quantum_invocation_tokens();

    let region = parse_hybrid_region(&tokens)
        .expect("a balanced domain-qualified invocation must be accepted");

    assert_eq!(region.elements().len(), 1);
    assert_eq!(
        region.elements()[0].kind(),
        HybridElementKind::DomainInvocation
    );
}

#[test]
fn preserves_region_and_element_source_spans() {
    let tokens = simple_quantum_invocation_tokens();

    let region = parse_hybrid_region(&tokens)
        .expect("valid source structure must parse");

    let region_span = region.span();
    let element_span = region.elements()[0].span();

    assert!(region_span.start_byte() <= element_span.start_byte());
    assert!(element_span.start_byte() <= element_span.end_byte());
    assert!(element_span.end_byte() <= region_span.end_byte());

    assert!(
        region.marker_span().start_byte() <= region.open_brace_span().start_byte(),
        "the hybrid marker must precede the opening brace"
    );

    assert!(
        region.open_brace_span().start_byte() < region.close_brace_span().start_byte(),
        "the opening brace must precede the closing brace"
    );
}

#[test]
fn preserves_element_order_and_token_ranges() {
    let tokens = token_stream(&[
        identifier("hybrid"),
        punctuation("{", TokenKind::LeftBrace),
        identifier("let"),
        identifier("result"),
        punctuation("=", TokenKind::Equal),
        identifier("value"),
        punctuation(";", TokenKind::Semicolon),
        identifier("quantum"),
        punctuation("::", TokenKind::DoubleColon),
        identifier("measure"),
        punctuation("(", TokenKind::LeftParen),
        identifier("state"),
        punctuation(")", TokenKind::RightParen),
        punctuation(";", TokenKind::Semicolon),
        punctuation("}", TokenKind::RightBrace),
    ]);

    let region = parse_hybrid_region(&tokens)
        .expect("both elements should be structurally accepted");

    assert_eq!(region.elements().len(), 2);
    assert_eq!(region.elements()[0].kind(), HybridElementKind::Binding);
    assert_eq!(
        region.elements()[1].kind(),
        HybridElementKind::DomainInvocation
    );

    let first = region.elements()[0].token_range();
    let second = region.elements()[1].token_range();

    assert!(first.start < first.end);
    assert!(second.start < second.end);
    assert!(first.end <= second.start);
}

#[test]
fn supports_extensible_domain_names_without_a_closed_domain_enum() {
    let tokens = token_stream(&[
        identifier("hybrid"),
        punctuation("{", TokenKind::LeftBrace),
        identifier("future_domain"),
        punctuation("::", TokenKind::DoubleColon),
        identifier("compute"),
        punctuation("(", TokenKind::LeftParen),
        identifier("input"),
        punctuation(")", TokenKind::RightParen),
        punctuation(";", TokenKind::Semicolon),
        punctuation("}", TokenKind::RightBrace),
    ]);

    let region = parse_hybrid_region(&tokens)
        .expect("an unfamiliar domain identifier must not be rejected solely by name");

    assert_eq!(region.elements().len(), 1);
    assert_eq!(
        region.elements()[0].kind(),
        HybridElementKind::DomainInvocation
    );
}

#[test]
fn recognizes_a_classical_quantum_classical_dependency_sequence() {
    let tokens = token_stream(&[
        identifier("hybrid"),
        punctuation("{", TokenKind::LeftBrace),
        identifier("let"),
        identifier("input"),
        punctuation("=", TokenKind::Equal),
        identifier("prepare"),
        punctuation("(", TokenKind::LeftParen),
        identifier("data"),
        punctuation(")", TokenKind::RightParen),
        punctuation(";", TokenKind::Semicolon),
        identifier("let"),
        identifier("measurement"),
        punctuation("=", TokenKind::Equal),
        identifier("quantum"),
        punctuation("::", TokenKind::DoubleColon),
        identifier("measure"),
        punctuation("(", TokenKind::LeftParen),
        identifier("input"),
        punctuation(")", TokenKind::RightParen),
        punctuation(";", TokenKind::Semicolon),
        identifier("let"),
        identifier("result"),
        punctuation("=", TokenKind::Equal),
        identifier("classical"),
        punctuation("::", TokenKind::DoubleColon),
        identifier("process"),
        punctuation("(", TokenKind::LeftParen),
        identifier("measurement"),
        punctuation(")", TokenKind::RightParen),
        punctuation(";", TokenKind::Semicolon),
        punctuation("}", TokenKind::RightBrace),
    ]);

    let region = parse_hybrid_region(&tokens)
        .expect("the syntax adapter must preserve a sequence of domain operations");

    assert_eq!(region.elements().len(), 3);
    assert_eq!(region.elements()[0].kind(), HybridElementKind::Binding);
    assert_eq!(
        region.elements()[1].kind(),
        HybridElementKind::DomainInvocation
    );
    assert_eq!(
        region.elements()[2].kind(),
        HybridElementKind::Binding
    );
}

#[test]
fn recognizes_a_nested_block_bearing_control_construct() {
    let tokens = token_stream(&[
        identifier("hybrid"),
        punctuation("{", TokenKind::LeftBrace),
        identifier("when"),
        identifier("condition"),
        punctuation("{", TokenKind::LeftBrace),
        identifier("quantum"),
        punctuation("::", TokenKind::DoubleColon),
        identifier("apply"),
        punctuation("(", TokenKind::LeftParen),
        identifier("operation"),
        punctuation(")", TokenKind::RightParen),
        punctuation(";", TokenKind::Semicolon),
        punctuation("}", TokenKind::RightBrace),
        punctuation("}", TokenKind::RightBrace),
    ]);

    let region = parse_hybrid_region(&tokens)
        .expect("balanced nested control blocks must be accepted");

    assert_eq!(region.elements().len(), 1);
    assert_eq!(region.elements()[0].kind(), HybridElementKind::Control);
}

#[test]
fn rejects_input_without_the_hybrid_marker() {
    let tokens = token_stream(&[
        identifier("classical"),
        punctuation("{", TokenKind::LeftBrace),
        punctuation("}", TokenKind::RightBrace),
    ]);

    let error = parse_hybrid_region(&tokens)
        .expect_err("a non-hybrid construct must not be accepted as a hybrid region");

    assert_eq!(error.kind(), HybridSyntaxErrorKind::ExpectedHybridMarker);
}

#[test]
fn rejects_a_missing_opening_brace() {
    let tokens = token_stream(&[
        identifier("hybrid"),
        punctuation(";", TokenKind::Semicolon),
    ]);

    let error = parse_hybrid_region(&tokens)
        .expect_err("a hybrid marker must be followed by an opening brace");

    assert_eq!(error.kind(), HybridSyntaxErrorKind::ExpectedOpeningBrace);
}

#[test]
fn rejects_a_missing_closing_brace() {
    let tokens = token_stream(&[
        identifier("hybrid"),
        punctuation("{", TokenKind::LeftBrace),
        identifier("quantum"),
        punctuation("::", TokenKind::DoubleColon),
        identifier("measure"),
        punctuation("(", TokenKind::LeftParen),
        identifier("state"),
        punctuation(")", TokenKind::RightParen),
        punctuation(";", TokenKind::Semicolon),
    ]);

    let error = parse_hybrid_region(&tokens)
        .expect_err("an unterminated hybrid region must not be accepted");

    assert!(
        matches!(
            error.kind(),
            HybridSyntaxErrorKind::MissingClosingBrace
                | HybridSyntaxErrorKind::UnexpectedEndOfInput
        ),
        "unexpected error category: {:?}",
        error.kind()
    );
}

#[test]
fn rejects_mismatched_nested_delimiters() {
    let tokens = token_stream(&[
        identifier("hybrid"),
        punctuation("{", TokenKind::LeftBrace),
        identifier("call"),
        punctuation("(", TokenKind::LeftParen),
        identifier("value"),
        punctuation("]", TokenKind::RightBracket),
        punctuation(";", TokenKind::Semicolon),
        punctuation("}", TokenKind::RightBrace),
    ]);

    let error = parse_hybrid_region(&tokens)
        .expect_err("mismatched delimiters must be rejected");

    assert_eq!(
        error.kind(),
        HybridSyntaxErrorKind::UnexpectedClosingDelimiter
    );
}

#[test]
fn rejects_unclosed_nested_delimiters() {
    let tokens = token_stream(&[
        identifier("hybrid"),
        punctuation("{", TokenKind::LeftBrace),
        identifier("call"),
        punctuation("(", TokenKind::LeftParen),
        identifier("value"),
        punctuation(";", TokenKind::Semicolon),
        punctuation("}", TokenKind::RightBrace),
    ]);

    let error = parse_hybrid_region(&tokens)
        .expect_err("unclosed nested delimiters must be rejected");

    assert!(
        matches!(
            error.kind(),
            HybridSyntaxErrorKind::UnclosedDelimiter
                | HybridSyntaxErrorKind::UnexpectedClosingDelimiter
        ),
        "unexpected error category: {:?}",
        error.kind()
    );
}

#[test]
fn rejects_trailing_non_eof_tokens_after_the_region() {
    let tokens = token_stream(&[
        identifier("hybrid"),
        punctuation("{", TokenKind::LeftBrace),
        punctuation("}", TokenKind::RightBrace),
        identifier("unexpected"),
    ]);

    let error = parse_hybrid_region(&tokens)
        .expect_err("a complete-region parser must reject trailing input");

    assert_eq!(error.kind(), HybridSyntaxErrorKind::TrailingTokens);
}

#[test]
fn ignores_trivia_channel_tokens_without_changing_region_elements() {
    let mut tokens = simple_quantum_invocation_tokens();

    let trivia_span = Span::new(
        CursorPosition::new(6, 1, 7),
        CursorPosition::new(7, 1, 8),
    );

    tokens.insert(
        1,
        Token::borrowed(TokenKind::Identifier, " ", trivia_span)
            .with_channel(TokenChannel::Trivia),
    );

    let region = parse_hybrid_region(&tokens)
        .expect("trivia-channel tokens must not alter parser-visible structure");

    assert_eq!(region.elements().len(), 1);
    assert_eq!(
        region.elements()[0].kind(),
        HybridElementKind::DomainInvocation
    );
}

#[test]
fn supports_large_configurable_element_counts_without_a_language_level_cap() {
    // This is a workload parameter for the test, not a production capacity
    // ceiling or a claim that all workloads fit in every process.
    let element_count = std::env::var("ZAMANI_HYBRID_CONFORMANCE_ELEMENTS")
        .ok()
        .and_then(|value| value.parse::<usize>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(1_000);

    let mut items: Vec<(&'static str, TokenKind)> =
        Vec::with_capacity(element_count.saturating_mul(4).saturating_add(2));

    items.push(identifier("hybrid"));
    items.push(punctuation("{", TokenKind::LeftBrace));

    for _ in 0..element_count {
        items.push(identifier("future_domain"));
        items.push(punctuation("::", TokenKind::DoubleColon));
        items.push(identifier("compute"));
        items.push(punctuation("(", TokenKind::LeftParen));
        items.push(identifier("input"));
        items.push(punctuation(")", TokenKind::RightParen));
        items.push(punctuation(";", TokenKind::Semicolon));
    }

    items.push(punctuation("}", TokenKind::RightBrace));

    let tokens = token_stream(&items);
    let region = parse_hybrid_region(&tokens)
        .expect("all generated domain invocations should be structurally accepted");

    assert_eq!(region.elements().len(), element_count);

    assert!(region.elements().iter().all(|element| {
        element.kind() == HybridElementKind::DomainInvocation
    }));
}

#[test]
fn hybrid_specification_declares_portability_and_resource_separation() {
    let specification = include_str!("../../../grammar/spec/hybrid.md");

    assert!(
        specification.contains("POCO-REAF")
            || specification.contains("Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever"),
        "the normative hybrid specification must identify the portability contract"
    );

    assert!(
        specification.contains("quantum::ir"),
        "quantum semantics must retain their canonical IR owner"
    );

    assert!(
        specification.contains("capabilit"),
        "the specification must address capability semantics"
    );

    assert!(
        specification.contains("resource"),
        "the specification must address resource requirements"
    );
}

#[test]
fn hybrid_grammar_remains_a_composition_boundary() {
    let grammar = include_str!("../../../grammar/hybrid/hybrid.g4");

    assert!(
        grammar.contains("parser grammar Hybrid")
            || grammar.contains("grammar Hybrid"),
        "the hybrid grammar must declare its grammar identity"
    );

    assert!(
        grammar.contains("quantum")
            && grammar.contains("classical"),
        "the hybrid grammar must describe cross-domain composition"
    );

    assert!(
        !grammar.contains("MAX_QUBITS")
            && !grammar.contains("MAX_GPUS")
            && !grammar.contains("MAX_CPUS")
            && !grammar.contains("MAX_TENSOR_RANK"),
        "the hybrid grammar must not introduce universal hardware ceilings"
    );
}

#[test]
fn classical_quantum_grammar_keeps_its_own_authority() {
    let grammar = include_str!("../../../grammar/hybrid/classical-quantum.g4");

    assert!(
        grammar.contains("quantum")
            && grammar.contains("classical"),
        "the grammar must describe classical/quantum interaction"
    );

    assert!(
        !grammar.contains("QuantumIR")
            && !grammar.contains("QuantumDeclarationIR"),
        "the grammar must not define a competing quantum IR"
    );
}

#[test]
fn quantum_classical_control_grammar_remains_a_separate_composition_component() {
    let grammar =
        include_str!("../../../grammar/hybrid/quantum-classical-control.g4");

    assert!(
        grammar.contains("control") || grammar.contains("Control"),
        "the component must document its control-composition responsibility"
    );

    assert!(
        grammar.contains("quantum") && grammar.contains("classical"),
        "the component must retain both participating domains"
    );
}

#[test]
fn accelerator_interoperability_does_not_select_physical_hardware_in_the_test_layer() {
    let grammar =
        include_str!("../../../grammar/hybrid/accelerator-interoperability.g4");

    assert!(
        grammar.contains("accelerator"),
        "the grammar component must define accelerator interoperability"
    );

    assert!(
        grammar.contains("does NOT determine")
            || grammar.contains("does not determine")
            || grammar.contains("does NOT own")
            || grammar.contains("does not own"),
        "the grammar should explicitly preserve downstream target-selection ownership"
    );
}

#[test]
fn hybrid_resource_grammar_preserves_resource_and_capability_intent() {
    let grammar = include_str!("../../../grammar/hybrid/hybrid-resources.g4");

    assert!(
        grammar.contains("Requirement")
            || grammar.contains("requirement")
            || grammar.contains("Capability")
            || grammar.contains("capability"),
        "the resource adapter must preserve requirement/capability concepts"
    );

    assert!(
        !grammar.contains("MAX_QUBITS")
            && !grammar.contains("MAX_GPUS")
            && !grammar.contains("MAX_CPUS"),
        "the resource grammar must not define universal hardware ceilings"
    );
}
