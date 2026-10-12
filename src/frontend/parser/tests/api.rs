
#![forbid(unsafe_code)]

//! Public API contract tests for the Zamani frontend parser.
//!
//! File: src/frontend/parser/tests/api.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! # Purpose
//!
//! Verify that the public parser API:
//! - forwards source, canonical tokens, and configuration unchanged;
//! - exposes all documented parsing entry points;
//! - preserves the complete ParseResult contract;
//! - does not silently convert recovered, incomplete, or failed outcomes
//!   into successful parses;
//! - retains token-consumption information;
//! - remains independent of a particular AST implementation.
//!
//! # Scope
//!
//! These tests use a deterministic mock ParserService. They verify the
//! API boundary, not grammar correctness or the implementation of the
//! production parser. Grammar behavior belongs in parser/grammar tests;
//! real lexer-to-parser integration belongs in integration tests.
//!
//! # Integration
//!
//! This file is registered by `src/frontend/parser/tests/mod.rs`, which
//! must in turn be registered by `src/frontend/parser/mod.rs` under
//! `#[cfg(test)]`. The public API is owned by `super::super::api`.
//!
//! These tests use the canonical lexer Token, TokenKind, CursorPosition,
//! and Span types. They do not define replacement token or span models.
//!
//! # Scalability and safety
//!
//! No hardware-capacity assumptions or fixed input-size ceilings are
//! introduced. Test inputs are deliberately small; stress and fuzz tests
//! exercise large and adversarial inputs in their own modules.

use std::cell::RefCell;

use crate::frontend::lexer::cursor::CursorPosition;
use crate::frontend::lexer::span::Span;
use crate::frontend::lexer::token::Token;
use crate::frontend::lexer::token_kind::TokenKind;

use super::super::api::{
    self,
    ParserService,
};
use super::super::config::{
    CompatibilityMode,
    ExtensionPolicy,
    LanguageVersion,
    ParseMode,
    ParserConfig,
    ParserLimits,
};
use super::super::result::{
    ParseResult,
    ParseStatus,
    TokenConsumption,
};

#[derive(Debug, Clone, PartialEq, Eq)]
struct MockProgram(&'static str);

#[derive(Debug, Clone, PartialEq, Eq)]
struct MockExpression(&'static str);

#[derive(Debug, Clone, PartialEq, Eq)]
struct MockType(&'static str);

#[derive(Debug, Clone, PartialEq, Eq)]
struct MockItem(&'static str);

#[derive(Debug, Clone, PartialEq, Eq)]
struct TokenSnapshot {
    kind: TokenKind,
    lexeme: String,
    start_byte: usize,
    end_byte: usize,
    is_eof: bool,
}

impl TokenSnapshot {
    fn from_token(token: &Token<'_>) -> Self {
        Self {
            kind: token.kind(),
            lexeme: token.lexeme().to_owned(),
            start_byte: token.span().start_byte(),
            end_byte: token.span().end_byte(),
            is_eof: token.is_eof(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ApiCall {
    ParseSource {
        source: String,
        config: ParserConfig,
    },
    ParseTokens {
        tokens: Vec<TokenSnapshot>,
        config: ParserConfig,
    },
    ParseExpression {
        source: String,
        config: ParserConfig,
    },
    ParseType {
        source: String,
        config: ParserConfig,
    },
    ParseItem {
        source: String,
        config: ParserConfig,
    },
}

/// Deterministic test double for the public parser-service contract.
///
/// The mock deliberately does not tokenize or parse source. Its purpose is
/// to prove that the API forwards inputs and results without altering them.
#[derive(Default)]
struct MockService {
    calls: RefCell<Vec<ApiCall>>,
}

impl MockService {
    fn calls(&self) -> Vec<ApiCall> {
        self.calls.borrow().clone()
    }

    fn take_calls(&self) -> Vec<ApiCall> {
        std::mem::take(&mut *self.calls.borrow_mut())
    }
}

impl ParserService for MockService {
    type Program = MockProgram;
    type Expression = MockExpression;
    type Type = MockType;
    type Item = MockItem;

    fn parse_source(
        &self,
        source: &str,
        config: &ParserConfig,
    ) -> ParseResult<Self::Program> {
        self.calls.borrow_mut().push(ApiCall::ParseSource {
            source: source.to_owned(),
            config: config.clone(),
        });

        ParseResult::complete(MockProgram("program"))
    }

    fn parse_tokens<'src>(
        &self,
        tokens: &[Token<'src>],
        config: &ParserConfig,
    ) -> ParseResult<Self::Program> {
        self.calls.borrow_mut().push(ApiCall::ParseTokens {
            tokens: tokens.iter().map(TokenSnapshot::from_token).collect(),
            config: config.clone(),
        });

        ParseResult::complete(MockProgram("token-program")).with_consumption(
            TokenConsumption::new(tokens.len(), Some(0)),
        )
    }

    fn parse_expression(
        &self,
        source: &str,
        config: &ParserConfig,
    ) -> ParseResult<Self::Expression> {
        self.calls.borrow_mut().push(ApiCall::ParseExpression {
            source: source.to_owned(),
            config: config.clone(),
        });

        ParseResult::complete(MockExpression("expression"))
    }

    fn parse_type(
        &self,
        source: &str,
        config: &ParserConfig,
    ) -> ParseResult<Self::Type> {
        self.calls.borrow_mut().push(ApiCall::ParseType {
            source: source.to_owned(),
            config: config.clone(),
        });

        ParseResult::complete(MockType("type"))
    }

    fn parse_item(
        &self,
        source: &str,
        config: &ParserConfig,
    ) -> ParseResult<Self::Item> {
        self.calls.borrow_mut().push(ApiCall::ParseItem {
            source: source.to_owned(),
            config: config.clone(),
        });

        ParseResult::complete(MockItem("item"))
    }
}

fn sample_config() -> ParserConfig {
    ParserConfig::new()
        .with_language_version(LanguageVersion::new(1, 2, 3))
        .with_mode(ParseMode::Strict)
        .with_limits(ParserLimits::unlimited())
        .with_extension_policy(ExtensionPolicy::Explicit)
        .with_compatibility(CompatibilityMode::Versioned)
}

fn sample_tokens<'src>(source: &'src str) -> Vec<Token<'src>> {
    let identifier_end = source.len().min(5);
    let identifier = &source[..identifier_end];

    let identifier_span = Span::new(
        CursorPosition::new(0, 1, 1),
        CursorPosition::new(identifier_end, 1, identifier_end + 1),
    );

    let eof_position =
        CursorPosition::new(source.len(), 1, source.chars().count() + 1);
    let eof_span = Span::at(eof_position);

    vec![
        Token::borrowed(
            TokenKind::Identifier,
            identifier,
            identifier_span,
        ),
        Token::synthetic(TokenKind::Eof, eof_span),
    ]
}

#[test]
fn default_configuration_is_valid_and_deterministic() {
    let first = ParserConfig::default();
    let second = ParserConfig::default();

    assert_eq!(first, second);
    assert!(first.validate().is_ok());
    assert!(first.require_eof);
}

#[test]
fn configured_language_policy_is_forwarded_unchanged() {
    let service = MockService::default();
    let config = sample_config();

    let result = api::parse_source(&service, "fn main() {}", &config);

    assert_eq!(result.status(), ParseStatus::Complete);
    assert_eq!(result.into_complete_value(), Some(MockProgram("program")));

    assert_eq!(
        service.calls(),
        vec![ApiCall::ParseSource {
            source: "fn main() {}".to_owned(),
            config,
        }]
    );
}

#[test]
fn parse_tokens_forwards_canonical_token_data_without_retokenizing() {
    let service = MockService::default();
    let config = sample_config();
    let tokens = sample_tokens("hello");

    let result = api::parse_tokens(&service, &tokens, &config);

    assert_eq!(result.status(), ParseStatus::Complete);
    assert_eq!(
        result.consumption(),
        Some(TokenConsumption::new(tokens.len(), Some(0)))
    );

    let calls = service.calls();
    assert_eq!(calls.len(), 1);

    match &calls[0] {
        ApiCall::ParseTokens {
            tokens: recorded,
            config: recorded_config,
        } => {
            assert_eq!(*recorded_config, config);
            assert_eq!(recorded.len(), tokens.len());

            assert_eq!(
                recorded[0],
                TokenSnapshot {
                    kind: TokenKind::Identifier,
                    lexeme: "hello".to_owned(),
                    start_byte: 0,
                    end_byte: 5,
                    is_eof: false,
                }
            );

            assert_eq!(recorded[1].kind, TokenKind::Eof);
            assert!(recorded[1].is_eof);
            assert_eq!(recorded[1].start_byte, 5);
            assert_eq!(recorded[1].end_byte, 5);
        }
        other => panic!("expected ParseTokens call, got {other:?}"),
    }
}

#[test]
fn parse_expression_forwards_source_and_configuration() {
    let service = MockService::default();
    let config = sample_config();

    let result = api::parse_expression(&service, "left + right", &config);

    assert_eq!(result.status(), ParseStatus::Complete);
    assert_eq!(
        result.into_complete_value(),
        Some(MockExpression("expression"))
    );

    assert_eq!(
        service.calls(),
        vec![ApiCall::ParseExpression {
            source: "left + right".to_owned(),
            config,
        }]
    );
}

#[test]
fn parse_type_forwards_source_and_configuration() {
    let service = MockService::default();
    let config = sample_config();

    let result = api::parse_type(&service, "Map<Key, Value>", &config);

    assert_eq!(result.status(), ParseStatus::Complete);
    assert_eq!(result.into_complete_value(), Some(MockType("type")));

    assert_eq!(
        service.calls(),
        vec![ApiCall::ParseType {
            source: "Map<Key, Value>".to_owned(),
            config,
        }]
    );
}

#[test]
fn parse_item_forwards_source_and_configuration() {
    let service = MockService::default();
    let config = sample_config();

    let result = api::parse_item(&service, "fn compute() {}", &config);

    assert_eq!(result.status(), ParseStatus::Complete);
    assert_eq!(result.into_complete_value(), Some(MockItem("item")));

    assert_eq!(
        service.calls(),
        vec![ApiCall::ParseItem {
            source: "fn compute() {}".to_owned(),
            config,
        }]
    );
}

#[test]
fn public_entry_points_do_not_share_hidden_mutable_state() {
    let service = MockService::default();
    let config = ParserConfig::default();

    let first = api::parse_source(&service, "first", &config);
    let second = api::parse_source(&service, "second", &config);

    assert_eq!(first.status(), ParseStatus::Complete);
    assert_eq!(second.status(), ParseStatus::Complete);

    assert_eq!(
        service.take_calls(),
        vec![
            ApiCall::ParseSource {
                source: "first".to_owned(),
                config: config.clone(),
            },
            ApiCall::ParseSource {
                source: "second".to_owned(),
                config,
            },
        ]
    );

    assert!(service.calls().is_empty());
}

#[test]
fn parse_result_keeps_complete_recovered_incomplete_and_failed_distinct() {
    let complete = ParseResult::complete(MockProgram("complete"));
    let recovered = ParseResult::recovered(MockProgram("recovered"));
    let incomplete: ParseResult<MockProgram> = ParseResult::incomplete();
    let failed: ParseResult<MockProgram> = ParseResult::new(None, ParseStatus::Failed);

    assert_eq!(complete.status(), ParseStatus::Complete);
    assert_eq!(recovered.status(), ParseStatus::Recovered);
    assert_eq!(incomplete.status(), ParseStatus::Incomplete);
    assert_eq!(failed.status(), ParseStatus::Failed);

    assert!(complete.is_complete());
    assert!(recovered.status().has_usable_result());
    assert!(incomplete.status().is_incomplete());
    assert!(failed.status().is_failed());

    assert_eq!(
        complete.into_complete_value(),
        Some(MockProgram("complete"))
    );
    assert_eq!(
        recovered.into_complete_value(),
        None,
        "a recovered parse must not be mistaken for a recovery-free parse"
    );
    assert_eq!(incomplete.into_usable_value(), None);
    assert_eq!(failed.into_usable_value(), None);
}

#[test]
fn token_consumption_distinguishes_full_and_partial_consumption() {
    let fully_consumed = TokenConsumption::new(7, Some(0));
    let trailing_tokens = TokenConsumption::new(7, Some(2));
    let unknown_remaining = TokenConsumption::new(7, None);

    assert!(fully_consumed.is_fully_consumed());
    assert_eq!(fully_consumed.consumed_tokens(), 7);
    assert_eq!(fully_consumed.remaining_tokens(), Some(0));

    assert!(!trailing_tokens.is_fully_consumed());
    assert_eq!(trailing_tokens.remaining_tokens(), Some(2));

    assert!(!unknown_remaining.is_fully_consumed());
    assert_eq!(unknown_remaining.remaining_tokens(), None);
}

#[test]
fn configuration_validation_rejects_invalid_extension_identifiers() {
    let mut config = ParserConfig::default();
    config.enabled_extensions.insert("".to_owned());

    assert!(
        config.validate().is_err(),
        "invalid extension identifiers must be rejected before parsing"
    );
}

#[test]
fn independent_calls_preserve_distinct_entry_point_types() {
    let service = MockService::default();
    let config = ParserConfig::default();

    let program = api::parse_source(&service, "program", &config);
    let expression = api::parse_expression(&service, "value", &config);
    let ty = api::parse_type(&service, "Value", &config);
    let item = api::parse_item(&service, "declaration", &config);

    assert_eq!(program.into_complete_value(), Some(MockProgram("program")));
    assert_eq!(
        expression.into_complete_value(),
        Some(MockExpression("expression"))
    );
    assert_eq!(ty.into_complete_value(), Some(MockType("type")));
    assert_eq!(item.into_complete_value(), Some(MockItem("item")));

    let calls = service.calls();
    assert_eq!(calls.len(), 4);

    assert!(matches!(calls[0], ApiCall::ParseSource { .. }));
    assert!(matches!(calls[1], ApiCall::ParseExpression { .. }));
    assert!(matches!(calls[2], ApiCall::ParseType { .. }));
    assert!(matches!(calls[3], ApiCall::ParseItem { .. }));
}
