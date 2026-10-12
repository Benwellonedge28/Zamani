"src/frontend/parser/tests/determinism.rs"

#![forbid(unsafe_code)]

//! Determinism and reproducibility tests for the Zamani frontend parser.
//!
//! File: src/frontend/parser/tests/determinism.rs
//! Rust edition: 2021
//! Minimum Rust version: 1.97.1
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Verify that the parser orchestration and extension registry behave
//! deterministically when supplied with equivalent inputs and configuration.
//!
//! # Scope
//!
//! This suite tests:
//! - repeatable parsing of identical canonical token streams;
//! - preservation of token kinds, spellings, and source spans;
//! - stable failures for malformed token streams;
//! - deterministic cursor lookahead, advancement, and EOF behavior;
//! - deterministic registry contents and dispatch mappings regardless of
//!   independent extension registration order;
//! - rejection of ambiguous registry ownership rather than implicit
//!   last-registration-wins behavior.
//!
//! # Ownership and boundaries
//!
//! These tests exercise the existing parser orchestration and registry
//! contracts. They do not pretend that a complete source-to-AST parser,
//! production grammar, or public ParserService implementation exists.
//!
//! The canonical lexer owns tokenization. The parser owns syntax recognition.
//! AST and semantic integration tests belong with the actual grammar and AST
//! adapter once those components are available.
//!
//! # Determinism invariant
//!
//! For equivalent token streams, grammar behavior, and registry definitions,
//! observable parser results and structured failures must be equivalent.
//! No test relies on wall-clock timing, process-global mutable state, hardware
//! discovery, randomized iteration order, or a particular machine capacity.
//!
//! # Scalability
//!
//! These tests introduce no language-level ceiling for tokens, declarations,
//! quantum operations, register widths, tensor dimensions, devices, or nodes.
//! Test data sizes are workload choices, not language limits.
//!
//! # Integration
//!
//! Register this module through `src/frontend/parser/tests/mod.rs`, and
//! register the test module from `src/frontend/parser/mod.rs` under
//! `#[cfg(test)]`. The parent parser module must also declare the existing
//! `parser` and `registry` modules.
//!
//! These tests intentionally use the repository's canonical `Token`, `Span`,
//! `CursorPosition`, and `TokenKind` types.

use std::sync::Arc;

use crate::frontend::lexer::cursor::CursorPosition;
use crate::frontend::lexer::span::Span;
use crate::frontend::lexer::token::{Token, TokenChannel};
use crate::frontend::lexer::token_kind::TokenKind;

use super::super::parser::{
    Parser, ParserError, ParserErrorKind, ParserGrammar, TokenCursor,
};
use super::super::registry::{
    DispatchKey, ExtensionId, ExtensionMetadata, ExtensionVersion, GrammarRegistry,
};

/// A stable snapshot of syntax-relevant token data.
///
/// The snapshot deliberately excludes allocation strategy and internal token
/// representation. It retains the fields that downstream syntax recognition
/// and source diagnostics depend on.
#[derive(Debug, Clone, PartialEq, Eq)]
struct TokenSnapshot {
    kind: TokenKind,
    lexeme: String,
    span: Span,
}

/// A minimal grammar used to test parser orchestration independently of the
/// not-yet-integrated full language grammar.
///
/// It consumes ordinary tokens and leaves the cursor at EOF, matching the
/// `ParserGrammar::parse_program` postcondition. It does not claim to validate
/// Zamani source syntax or construct the canonical AST.
#[derive(Debug, Default)]
struct SnapshotGrammar;

impl<'tokens, 'src> ParserGrammar<'tokens, 'src> for SnapshotGrammar {
    type Program = Vec<TokenSnapshot>;

    fn parse_program(
        &mut self,
        cursor: &mut TokenCursor<'tokens, 'src>,
    ) -> Result<Self::Program, ParserError> {
        let mut snapshots = Vec::new();

        loop {
            let Some(token) = cursor.current() else {
                return Err(ParserError::new(
                    ParserErrorKind::InvalidParserState,
                    Span::at(CursorPosition::default()),
                    "test grammar unexpectedly reached an invalid cursor position",
                ));
            };

            if token.kind() == TokenKind::Eof {
                break;
            }

            snapshots.push(TokenSnapshot {
                kind: token.kind(),
                lexeme: token.lexeme().to_owned(),
                span: *token.span(),
            });

            cursor.advance()?;
        }

        Ok(snapshots)
    }
}

/// Construct a source-backed token with a checked, half-open span.
///
/// Test inputs use ASCII text, so byte offsets and one-based scalar columns
/// correspond directly. Production Unicode coordinate calculation remains
/// the responsibility of the canonical lexer cursor.
fn source_token<'src>(
    source: &'src str,
    kind: TokenKind,
    start: usize,
    end: usize,
) -> Token<'src> {
    assert!(start <= end, "test token range must be ordered");
    assert!(end <= source.len(), "test token range must be in bounds");

    let lexeme = source
        .get(start..end)
        .expect("test token boundaries must be valid UTF-8 boundaries");

    let start_position = CursorPosition::new(start, 1, start + 1);
    let end_position = CursorPosition::new(end, 1, end + 1);
    let span = Span::try_new(start_position, end_position)
        .expect("test token span must be valid");

    Token::borrowed(kind, lexeme, span)
}

/// Construct the same representative token stream for repeated-run tests.
fn representative_tokens(source: &str) -> Vec<Token<'_>> {
    assert_eq!(source, "alpha + 42", "fixture source must match its ranges");

    vec![
        source_token(source, TokenKind::Identifier, 0, 5),
        source_token(source, TokenKind::Plus, 6, 7),
        source_token(source, TokenKind::IntegerLiteral, 8, 10),
        source_token(source, TokenKind::Eof, 10, 10),
    ]
}

/// Create a registry with the supplied extension registrations.
///
/// Every registration has a unique dispatch key, so the order of registration
/// is the only intentional difference between equivalent registry builds.
fn build_registry(
    registrations: &[(&str, &str, &str)],
) -> GrammarRegistry<String> {
    let mut builder = GrammarRegistry::<String>::builder(1);

    for &(extension_name, namespace, dispatch_name) in registrations {
        let id = ExtensionId::new(extension_name)
            .expect("test extension identity must be valid");
        let key = DispatchKey::new(namespace, dispatch_name)
            .expect("test dispatch key must be valid");
        let metadata = ExtensionMetadata::new(
            id,
            ExtensionVersion::new(1, 0, 0),
            1,
        );

        builder
            .register(metadata, [key], Arc::new(extension_name.to_owned()))
            .expect("test registration must not conflict");
    }

    builder.build().expect("test registry must be internally consistent")
}

/// Produce a stable representation of registry metadata and dispatch
/// ownership. This is test-only comparison data, not a public serialization
/// format.
fn registry_snapshot(registry: &GrammarRegistry<String>) -> Vec<String> {
    let extensions = registry
        .extensions()
        .map(|(id, registered)| {
            format!(
                "extension:{}:{}:{}:{}",
                id.as_str(),
                registered.metadata().version.major,
                registered.metadata().version.minor,
                registered.metadata().version.patch,
            )
        });

    let dispatch = registry
        .dispatch_entries()
        .map(|(key, id)| format!("dispatch:{key}={}", id.as_str()));

    extensions.chain(dispatch).collect()
}

#[test]
fn repeated_parses_of_identical_tokens_produce_identical_results() {
    let source = "alpha + 42";
    let tokens = representative_tokens(source);

    let first = Parser::new(&tokens, SnapshotGrammar)
        .expect("valid token stream must construct a parser")
        .parse_program()
        .expect("snapshot grammar must parse the complete token stream");

    let second = Parser::new(&tokens, SnapshotGrammar)
        .expect("identical token stream must construct a parser")
        .parse_program()
        .expect("identical token stream must parse successfully");

    assert_eq!(first, second);
    assert_eq!(first.len(), 3);
    assert_eq!(first[0].kind, TokenKind::Identifier);
    assert_eq!(first[0].lexeme, "alpha");
    assert_eq!(first[1].kind, TokenKind::Plus);
    assert_eq!(first[1].lexeme, "+");
    assert_eq!(first[2].kind, TokenKind::IntegerLiteral);
    assert_eq!(first[2].lexeme, "42");
}

#[test]
fn parsing_preserves_token_spelling_and_source_spans() {
    let source = "alpha + 42";
    let tokens = representative_tokens(source);

    let parsed = Parser::new(&tokens, SnapshotGrammar)
        .expect("valid token stream must construct a parser")
        .parse_program()
        .expect("snapshot grammar must parse successfully");

    assert_eq!(parsed.len(), 3);

    for (snapshot, original) in parsed.iter().zip(tokens.iter()) {
        assert_eq!(snapshot.kind, original.kind());
        assert_eq!(snapshot.lexeme, original.lexeme());
        assert_eq!(snapshot.span, *original.span());
    }
}

#[test]
fn malformed_stream_without_eof_fails_deterministically() {
    let source = "alpha";
    let tokens = vec![source_token(
        source,
        TokenKind::Identifier,
        0,
        source.len(),
    )];

    let first = Parser::new(&tokens, SnapshotGrammar)
        .expect_err("a stream without EOF must be rejected");
    let second = Parser::new(&tokens, SnapshotGrammar)
        .expect_err("the same malformed stream must be rejected");

    assert_eq!(first, second);
    assert_eq!(first.kind, ParserErrorKind::MissingEof);
}

#[test]
fn eof_before_the_last_token_fails_deterministically() {
    let source = "a b";
    let tokens = vec![
        source_token(source, TokenKind::Identifier, 0, 1),
        source_token(source, TokenKind::Eof, 1, 1),
        source_token(source, TokenKind::Identifier, 2, 3),
    ];

    let first = TokenCursor::new(&tokens)
        .expect_err("EOF before the final token must be rejected");
    let second = TokenCursor::new(&tokens)
        .expect_err("the same malformed stream must fail identically");

    assert_eq!(first, second);
    assert_eq!(first.kind, ParserErrorKind::EofNotLast);
}

#[test]
fn lexical_error_channel_is_rejected_consistently() {
    let source = "alpha";
    let invalid = source_token(source, TokenKind::Identifier, 0, 5)
        .with_channel(TokenChannel::Error);
    let eof = source_token(source, TokenKind::Eof, 5, 5);
    let tokens = vec![invalid, eof];

    let first = TokenCursor::new(&tokens)
        .expect_err("error-channel tokens must not enter normal parsing");
    let second = TokenCursor::new(&tokens)
        .expect_err("error-channel rejection must be repeatable");

    assert_eq!(first, second);
    assert_eq!(first.kind, ParserErrorKind::InvalidToken);
}

#[test]
fn cursor_lookahead_and_advancement_are_repeatable() {
    let source = "alpha + 42";
    let tokens = representative_tokens(source);

    let mut first = TokenCursor::new(&tokens).expect("valid stream");
    let mut second = TokenCursor::new(&tokens).expect("identical valid stream");

    assert_eq!(first.current_kind(), second.current_kind());
    assert_eq!(first.current().map(Token::lexeme), second.current().map(Token::lexeme));

    assert_eq!(
        first.lookahead(1).map(Token::kind),
        second.lookahead(1).map(Token::kind)
    );
    assert_eq!(
        first.lookahead(2).map(Token::lexeme),
        second.lookahead(2).map(Token::lexeme)
    );

    for _ in 0..tokens.len() {
        let first_token = first.advance().expect("cursor must advance safely");
        let second_token = second.advance().expect("second cursor must advance safely");

        assert_eq!(first_token.kind(), second_token.kind());
        assert_eq!(first_token.lexeme(), second_token.lexeme());
        assert_eq!(first_token.span(), second_token.span());
    }

    assert!(first.at_eof());
    assert!(second.at_eof());
    assert_eq!(first.position(), second.position());
}

#[test]
fn eof_advancement_is_idempotent() {
    let source = "";
    let tokens = vec![source_token(source, TokenKind::Eof, 0, 0)];
    let mut cursor = TokenCursor::new(&tokens).expect("single EOF is valid");

    let initial_position = cursor.position();
    let first = cursor.advance().expect("EOF can be observed");
    let second = cursor.advance().expect("repeated EOF observation is valid");

    assert_eq!(first.kind(), TokenKind::Eof);
    assert_eq!(second.kind(), TokenKind::Eof);
    assert_eq!(cursor.position(), initial_position);
    assert!(cursor.at_eof());
}

#[test]
fn checkpoint_restore_is_deterministic() {
    let source = "alpha + 42";
    let tokens = representative_tokens(source);
    let mut cursor = TokenCursor::new(&tokens).expect("valid stream");

    let initial_checkpoint = cursor.checkpoint();

    assert_eq!(cursor.current_kind(), Some(TokenKind::Identifier));
    cursor.advance().expect("advance to operator");
    assert_eq!(cursor.current_kind(), Some(TokenKind::Plus));

    cursor.restore(initial_checkpoint).expect("restore initial position");
    assert_eq!(cursor.current_kind(), Some(TokenKind::Identifier));

    cursor.advance().expect("advance after restore");
    assert_eq!(cursor.current_kind(), Some(TokenKind::Plus));
}

#[test]
fn equivalent_registry_definitions_are_independent_of_registration_order() {
    let registrations = [
        ("vendor.quantum", "quantum", "operation"),
        ("vendor.hdl", "hdl", "declaration"),
        ("vendor.data", "data", "construct"),
    ];

    let forward = build_registry(&registrations);

    let reverse_registrations = [
        ("vendor.data", "data", "construct"),
        ("vendor.hdl", "hdl", "declaration"),
        ("vendor.quantum", "quantum", "operation"),
    ];
    let reverse = build_registry(&reverse_registrations);

    assert_eq!(forward.extension_count(), reverse.extension_count());
    assert_eq!(forward.dispatch_key_count(), reverse.dispatch_key_count());
    assert_eq!(registry_snapshot(&forward), registry_snapshot(&reverse));

    for (_, namespace, dispatch_name) in registrations {
        let key = DispatchKey::new(namespace, dispatch_name)
            .expect("test dispatch key must be valid");

        assert_eq!(
            forward.owner_of(&key).map(ExtensionId::as_str),
            reverse.owner_of(&key).map(ExtensionId::as_str)
        );
    }
}

#[test]
fn conflicting_dispatch_ownership_is_rejected_not_order_dependent() {
    let mut builder = GrammarRegistry::<String>::builder(1);

    let shared_key = DispatchKey::new("custom", "construct")
        .expect("test dispatch key must be valid");

    let first_id = ExtensionId::new("vendor.first")
        .expect("test extension ID must be valid");
    let first_metadata = ExtensionMetadata::new(
        first_id,
        ExtensionVersion::new(1, 0, 0),
        1,
    );

    builder
        .register(
            first_metadata,
            [shared_key.clone()],
            Arc::new(String::from("first")),
        )
        .expect("first owner must register");

    let second_id = ExtensionId::new("vendor.second")
        .expect("test extension ID must be valid");
    let second_metadata = ExtensionMetadata::new(
        second_id,
        ExtensionVersion::new(1, 0, 0),
        1,
    );

    let result = builder.register(
        second_metadata,
        [shared_key.clone()],
        Arc::new(String::from("second")),
    );

    assert!(result.is_err());
    assert_eq!(
        builder
            .build()
            .expect("failed registration must not corrupt the builder")
            .owner_of(&shared_key)
            .map(ExtensionId::as_str),
        Some("vendor.first")
    );
}

#[test]
fn independent_parser_instances_do_not_share_cursor_state() {
    let source = "alpha + 42";
    let tokens = representative_tokens(source);

    let mut first = Parser::new(&tokens, SnapshotGrammar)
        .expect("first parser construction succeeds");
    let second = Parser::new(&tokens, SnapshotGrammar)
        .expect("second parser construction succeeds");

    first.cursor_mut().advance().expect("advance first cursor");

    assert_eq!(first.cursor().current_kind(), Some(TokenKind::Plus));
    assert_eq!(second.cursor().current_kind(), Some(TokenKind::Identifier));

    let first_result = first
        .parse_program()
        .expect("test grammar consumes the remaining stream");
    let second_result = second
        .parse_program()
        .expect("fresh parser consumes the full stream");

    assert_ne!(first_result, second_result);
    assert_eq!(second_result.len(), 3);
}
