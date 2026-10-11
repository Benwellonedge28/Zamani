#![forbid(unsafe_code)]

//! Contract tests for the canonical Zamani frontend token model.
//!
//! File: src/frontend/lexer/tests/token_contract.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Verify the public structural contract shared by tokens, token kinds,
//! source spans, token channels, and downstream parser-facing consumers.
//!
//! These tests intentionally avoid depending on scanner implementation
//! details. Scanner behavior, Unicode traversal, keyword classification,
//! operator matching, diagnostics, and recovery have dedicated test modules.
//!
//! # Integration contract
//!
//! - `token.rs` owns token storage, lexeme borrowing, channels, and accessors.
//! - `token_kind.rs` owns canonical lexical identities and categories.
//! - `span.rs` owns half-open source ranges.
//! - `cursor.rs` owns UTF-8-safe source positions.
//! - `token_metadata.rs` owns token metadata and lifecycle descriptions.
//! - `keyword_registry.rs` owns keyword spelling classification.
//! - `operator_registry.rs` owns operator spelling classification.
//! - `dialect.rs` owns dialect activation and extension policy.
//! - `scanner.rs` constructs tokens from recognized source ranges.
//! - `api.rs` exposes tokens to parsers and tooling.
//! - `diagnostics.rs` and `recovery.rs` consume token locations and kinds.
//! - `grammar/spec/lexical.md` is the normative lexical contract.
//! - `grammar/Zamani.g4` composes grammar components; it does not redefine
//!   the Rust token representation.
//!
//! This file must not become a second token registry or a second lexer.
//! Adding a new lexical identity requires coordinated changes to the
//! canonical token kind, its metadata and registries, grammar contracts,
//! compatibility mappings where applicable, and focused tests.
//!
//! # Portability and scalability
//!
//! The token model must not assume a particular computational domain,
//! quantum-operation catalog, processor count, register width, tensor rank,
//! device count, or hardware topology.
//!
//! Tests use small fixtures to verify invariants, not to define production
//! capacity. Actual input sizes remain subject to configured resource
//! budgets and host resources.
//!
//! # Test execution
//!
//! Include this module from `tests/mod.rs`, which is itself included by
//! `lexer/mod.rs` under `#[cfg(test)]`.
//!
//! Run:
//!
//!     cargo test frontend::lexer::tests::token_contract
//!
//! These tests establish structural contracts. They do not, by themselves,
//! prove that every grammar production has a working parser or that every
//! token spelling is recognized by the scanner.

use crate::frontend::lexer::cursor::CursorPosition;
use crate::frontend::lexer::span::Span;
use crate::frontend::lexer::token::{Token, TokenChannel};
use crate::frontend::lexer::token_kind::{TokenCategory, TokenKind};

/// Constructs a valid span using the canonical source-position convention.
///
/// Byte offsets are zero-based UTF-8 offsets. Lines and columns are one-based.
fn span(start: usize, end: usize) -> Span {
    Span::new(
        CursorPosition::new(start, 1, start.saturating_add(1)),
        CursorPosition::new(end, 1, end.saturating_add(1)),
    )
}

#[test]
fn borrowed_token_preserves_exact_source_spelling() {
    let source = "quantum_register";

    let token = Token::borrowed(
        TokenKind::Identifier,
        source,
        span(0, source.len()),
    );

    assert_eq!(token.kind(), TokenKind::Identifier);
    assert_eq!(token.lexeme(), source);
    assert!(token.borrows_source());
    assert!(!token.has_empty_lexeme());
}

#[test]
fn borrowed_token_retains_the_original_source_borrow() {
    let source = String::from("processor_count");

    let token = Token::borrowed(
        TokenKind::Identifier,
        source.as_str(),
        span(0, source.len()),
    );

    assert!(token.borrows_source());
    assert_eq!(token.lexeme(), source);
}

#[test]
fn owned_token_preserves_generated_spelling() {
    let spelling = String::from("generated_identifier");

    let token = Token::owned(
        TokenKind::Identifier,
        spelling.clone(),
        span(0, spelling.len()),
    );

    assert_eq!(token.lexeme(), spelling);
    assert!(!token.borrows_source());
}

#[test]
fn generic_constructor_preserves_borrowed_lexemes() {
    let source = "custom_operation";

    let token = Token::new(
        TokenKind::Identifier,
        source,
        span(0, source.len()),
    );

    assert_eq!(token.lexeme(), source);
    assert!(token.borrows_source());
}

#[test]
fn synthetic_eof_has_an_empty_lexeme_and_zero_width_span() {
    let position = CursorPosition::new(17, 3, 6);
    let eof_span = Span::at(position);

    let token = Token::synthetic(TokenKind::Eof, eof_span);

    assert!(token.is_eof());
    assert!(token.has_empty_lexeme());
    assert_eq!(token.kind(), TokenKind::Eof);
    assert_eq!(token.span().start(), position);
    assert_eq!(token.span().end(), position);
    assert!(token.span().is_empty());
}

#[test]
fn synthetic_non_eof_tokens_are_not_mistaken_for_eof() {
    let token = Token::synthetic(
        TokenKind::Invalid,
        Span::default(),
    );

    assert!(!token.is_eof());
    assert!(token.has_empty_lexeme());
    assert!(token.kind().is_invalid());
}

#[test]
fn token_channel_changes_do_not_change_token_identity() {
    let source = "documentation";
    let original_span = span(0, source.len());

    let token = Token::borrowed(
        TokenKind::Identifier,
        source,
        original_span,
    )
    .with_channel(TokenChannel::Documentation);

    assert_eq!(token.kind(), TokenKind::Identifier);
    assert_eq!(token.lexeme(), source);
    assert_eq!(token.span(), &original_span);
    assert_eq!(token.channel(), TokenChannel::Documentation);
    assert!(token.is_documentation_channel());
    assert!(!token.is_default_channel());
}

#[test]
fn mutable_channel_update_preserves_kind_lexeme_and_span() {
    let source = "retained_comment";
    let original_span = span(0, source.len());

    let mut token = Token::borrowed(
        TokenKind::Identifier,
        source,
        original_span,
    );

    token.set_channel(TokenChannel::Trivia);

    assert_eq!(token.kind(), TokenKind::Identifier);
    assert_eq!(token.lexeme(), source);
    assert_eq!(token.span(), &original_span);
    assert!(token.is_trivia_channel());
}

#[test]
fn all_declared_channels_have_consistent_predicates() {
    let cases = [
        (TokenChannel::Default, true, false, false, false),
        (TokenChannel::Trivia, false, true, false, false),
        (
            TokenChannel::Documentation,
            false,
            false,
            true,
            false,
        ),
        (TokenChannel::Error, false, false, false, true),
    ];

    for (channel, is_default, is_trivia, is_documentation, is_error) in cases {
        let token = Token::synthetic(TokenKind::Identifier, Span::default())
            .with_channel(channel);

        assert_eq!(token.is_default_channel(), is_default);
        assert_eq!(token.is_trivia_channel(), is_trivia);
        assert_eq!(
            token.is_documentation_channel(),
            is_documentation
        );
        assert_eq!(token.is_error_channel(), is_error);
        assert_eq!(token.channel(), channel);
    }
}

#[test]
fn token_span_uses_half_open_utf8_byte_ranges() {
    let source = "aλ界";

    // UTF-8 widths: 'a' = 1 byte, 'λ' = 2 bytes, '界' = 3 bytes.
    let token_span = Span::new(
        CursorPosition::new(1, 1, 2),
        CursorPosition::new(3, 1, 3),
    );

    let token = Token::borrowed(
        TokenKind::Identifier,
        &source[1..3],
        token_span,
    );

    assert_eq!(token.lexeme(), "λ");
    assert_eq!(token.span().byte_range(), 1..3);
    assert_eq!(token.span().byte_len(), token.lexeme().len());
    assert_eq!(&source[token.span().byte_range()], token.lexeme());
}

#[test]
fn empty_spelling_does_not_implicitly_mean_eof() {
    let token = Token::borrowed(
        TokenKind::Identifier,
        "",
        Span::default(),
    );

    assert!(token.has_empty_lexeme());
    assert!(!token.is_eof());
    assert_eq!(token.kind(), TokenKind::Identifier);
}

#[test]
fn owned_conversion_preserves_the_complete_token_contract() {
    let source = String::from("quantum");
    let original_span = span(0, source.len());

    let token = Token::borrowed(
        TokenKind::Identifier,
        source.as_str(),
        original_span,
    )
    .with_channel(TokenChannel::Default);

    let owned = token.into_owned();

    assert_eq!(owned.kind(), TokenKind::Identifier);
    assert_eq!(owned.lexeme(), "quantum");
    assert_eq!(owned.span(), &original_span);
    assert_eq!(owned.channel(), TokenChannel::Default);
    assert!(!owned.borrows_source());
}

#[test]
fn consuming_a_token_can_extract_its_lexeme() {
    let token = Token::owned(
        TokenKind::Identifier,
        String::from("owned_name"),
        span(0, 10),
    );

    assert_eq!(token.into_owned_lexeme(), "owned_name");
}

#[test]
fn token_equality_includes_kind_lexeme_span_and_channel() {
    let first_span = span(0, 4);

    let first = Token::borrowed(
        TokenKind::Identifier,
        "name",
        first_span,
    );

    let same = Token::borrowed(
        TokenKind::Identifier,
        "name",
        first_span,
    );

    let different_kind = Token::borrowed(
        TokenKind::StringLiteral,
        "name",
        first_span,
    );

    let different_span = Token::borrowed(
        TokenKind::Identifier,
        "name",
        span(1, 5),
    );

    let different_channel = Token::borrowed(
        TokenKind::Identifier,
        "name",
        first_span,
    )
    .with_channel(TokenChannel::Trivia);

    assert_eq!(first, same);
    assert_ne!(first, different_kind);
    assert_ne!(first, different_span);
    assert_ne!(first, different_channel);
}

#[test]
fn token_display_is_diagnostic_text_not_source_serialization() {
    let token = Token::borrowed(
        TokenKind::Identifier,
        "sample",
        span(0, 6),
    );

    let rendered = token.to_string();

    assert!(rendered.contains("Identifier"));
    assert!(rendered.contains("sample"));

    // The display representation is deliberately not the source spelling.
    assert_ne!(rendered, token.lexeme());
}

#[test]
fn core_lexical_categories_are_consistent() {
    assert_eq!(
        TokenKind::Identifier.category(),
        TokenCategory::Identifier
    );

    assert_eq!(
        TokenKind::IntegerLiteral.category(),
        TokenCategory::Literal
    );

    assert_eq!(
        TokenKind::StringLiteral.category(),
        TokenCategory::Literal
    );

    assert_eq!(
        TokenKind::QuantumLiteral.category(),
        TokenCategory::Literal
    );

    assert_eq!(
        TokenKind::Annotation.category(),
        TokenCategory::Annotation
    );

    assert_eq!(
        TokenKind::LineComment.category(),
        TokenCategory::Comment
    );

    assert_eq!(
        TokenKind::BlockComment.category(),
        TokenCategory::Comment
    );

    assert_eq!(
        TokenKind::Whitespace.category(),
        TokenCategory::Whitespace
    );

    assert_eq!(
        TokenKind::Eof.category(),
        TokenCategory::EndOfInput
    );

    assert_eq!(
        TokenKind::Invalid.category(),
        TokenCategory::Invalid
    );
}

#[test]
fn token_kind_predicates_agree_with_categories() {
    assert!(TokenKind::Identifier.is_trivia() == false);
    assert!(TokenKind::Identifier.is_literal() == false);
    assert!(TokenKind::Identifier.is_operator() == false);
    assert!(TokenKind::Identifier.is_delimiter() == false);
    assert!(TokenKind::Identifier.is_keyword() == false);

    assert!(TokenKind::IntegerLiteral.is_literal());
    assert!(TokenKind::Plus.is_operator());
    assert!(TokenKind::LeftParen.is_delimiter());
    assert!(TokenKind::KeywordQuantum.is_keyword());
    assert!(TokenKind::LineComment.is_trivia());
    assert!(TokenKind::BlockComment.is_trivia());
    assert!(TokenKind::Whitespace.is_trivia());
    assert!(TokenKind::Newline.is_trivia());
    assert!(TokenKind::Eof.is_eof());
    assert!(TokenKind::Invalid.is_invalid());
}

#[test]
fn lexical_identity_does_not_depend_on_computational_domain() {
    // These examples exercise the shared token contract, not the validity
    // of the complete programs in which such names might appear.
    //
    // Operation names and vendor identifiers remain ordinary identifiers
    // unless an active registry explicitly classifies them otherwise.
    let identifiers = [
        "classical_kernel",
        "quantum_operation",
        "hdl_component",
        "tensor_transform",
        "distributed_worker",
        "future_accelerator",
    ];

    for spelling in identifiers {
        let token = Token::borrowed(
            TokenKind::Identifier,
            spelling,
            span(0, spelling.len()),
        );

        assert_eq!(token.kind(), TokenKind::Identifier);
        assert_eq!(token.lexeme(), spelling);
        assert_eq!(token.kind().category(), TokenCategory::Identifier);
    }
}

#[test]
fn token_kind_diagnostic_labels_are_non_empty() {
    let kinds = [
        TokenKind::Identifier,
        TokenKind::IntegerLiteral,
        TokenKind::FloatLiteral,
        TokenKind::StringLiteral,
        TokenKind::CharacterLiteral,
        TokenKind::QuantumLiteral,
        TokenKind::Annotation,
        TokenKind::LineComment,
        TokenKind::BlockComment,
        TokenKind::Whitespace,
        TokenKind::Invalid,
        TokenKind::Eof,
        TokenKind::LeftParen,
        TokenKind::RightParen,
        TokenKind::Plus,
        TokenKind::Assign,
        TokenKind::KeywordFn,
        TokenKind::KeywordQuantum,
        TokenKind::KeywordQubit,
    ];

    for kind in kinds {
        assert!(!kind.diagnostic_name().trim().is_empty());
        assert!(!kind.variant_name().trim().is_empty());
    }
}

#[test]
fn zero_width_spans_are_valid_for_synthetic_tokens() {
    let position = CursorPosition::new(42, 5, 9);
    let token = Token::synthetic(TokenKind::Eof, Span::at(position));

    assert_eq!(token.span().start_byte(), 42);
    assert_eq!(token.span().end_byte(), 42);
    assert_eq!(token.span().byte_len(), 0);
    assert!(token.span().is_empty());
}

#[test]
fn token_contract_has_no_domain_specific_capacity_assumption() {
    // A token stores lexical identity and spelling; it does not encode
    // qubit counts, processor counts, register widths, or device capacity.
    //
    // This deliberately tests the common representation rather than
    // introducing artificial "large hardware" constants into the suite.
    let spelling = "operation_defined_by_an_extension";

    let token = Token::borrowed(
        TokenKind::Identifier,
        spelling,
        span(0, spelling.len()),
    );

    assert_eq!(token.lexeme(), spelling);
    assert_eq!(token.kind(), TokenKind::Identifier);
    assert_eq!(token.span().byte_len(), spelling.len());
}