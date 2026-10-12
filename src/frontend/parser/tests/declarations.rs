
#![forbid(unsafe_code)]

//! Declaration-dispatch conformance tests for the Zamani frontend parser.
//!
//! File: src/frontend/parser/tests/declarations.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! # Purpose
//!
//! Verify the canonical declaration dispatcher against its checked-in
//! contracts:
//!
//! - core declaration-start classification;
//! - deterministic dispatch to the correct production;
//! - core declarations taking precedence over extensions;
//! - explicit rejection of unsupported declaration starts;
//! - extension dispatch for explicitly recognized starts;
//! - rejection of successful productions that make no progress;
//! - preservation of the canonical token cursor contract.
//!
//! # Ownership
//!
//! This file owns tests and test-only doubles. It does not implement
//! declaration grammar, define a competing AST, tokenize source, resolve
//! names, or perform semantic analysis.
//!
//! # Dependencies
//!
//! - `crate::frontend::lexer::span::Span`;
//! - `crate::frontend::lexer::token::Token`;
//! - `crate::frontend::lexer::token_kind::TokenKind`;
//! - `crate::frontend::parser::parser::{ParserError, ParserErrorKind,
//!   TokenCursor}`;
//! - `crate::frontend::parser::grammar::declarations`.
//!
//! # Normative grammar references
//!
//! - `grammar/DESIGN.md`;
//! - `grammar/Zamani.g4`;
//! - `grammar/grammar.md`;
//! - `grammar/declarations/declarations.g4`;
//! - `grammar/declarations/structs.g4`;
//! - `grammar/declarations/enums.g4`;
//! - `grammar/functions/functions.g4`;
//! - `grammar/functions/generics.g4`;
//! - `grammar/modules/modules.g4`.
//!
//! The ANTLR declaration composition grammar delegates concrete syntax to
//! specialized grammars. These tests validate the Rust declaration
//! dispatcher; they do not claim to validate every concrete grammar rule.
//!
//! # Integration
//!
//! The production dispatcher owns declaration classification and dispatch.
//! Concrete grammar implementations must implement `DeclarationProductions`
//! and construct the repository's canonical AST through the established AST
//! integration layer.
//!
//! The test module should be registered by the parser's test module tree.
//! It must not be registered as a second production implementation.
//!
//! # Resource and portability policy
//!
//! No hardware capacities or domain-specific limits are introduced.
//! Test-case counts are finite verification workloads, not language limits.
//! Production input size, nesting and recovery budgets belong to parser
//! configuration and resource-accounting modules.
//!
//! # Limitations
//!
//! These tests establish dispatcher behavior. Full production readiness also
//! requires integration tests that tokenize real Zamani source, parse
//! complete declarations, inspect canonical AST shape and spans, and verify
//! compatibility against the normative grammar.

use crate::frontend::lexer::span::Span;
use crate::frontend::lexer::token::Token;
use crate::frontend::lexer::token_kind::TokenKind;
use crate::frontend::parser::grammar::declarations::{
    is_declaration_start, parse_declaration, DeclarationKind,
    DeclarationProductions,
};
use crate::frontend::parser::parser::{
    ParserError, ParserErrorKind, TokenCursor,
};

/// Test-only result identifying which production was called.
///
/// This is deliberately not an AST node. The dispatcher is generic over its
/// production output, and its contract does not prescribe a competing AST.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Production {
    Function,
    Variable,
    Constant,
    Type,
    Struct,
    Record,
    Enum,
    Class,
    Trait,
    Interface,
    Implementation,
    Extension,
}

/// Deterministic mock for the declaration production interface.
///
/// By default each production consumes one token, matching the dispatcher
/// contract that successful productions must advance the cursor. Tests can
/// disable consumption to exercise the dispatcher's no-progress safeguard.
#[derive(Debug, Default)]
struct MockProductions {
    extension_enabled: bool,
    consume_input: bool,
    calls: Vec<Production>,
}

impl MockProductions {
    fn with_extension() -> Self {
        Self {
            extension_enabled: true,
            consume_input: true,
            calls: Vec::new(),
        }
    }

    fn without_consumption() -> Self {
        Self {
            extension_enabled: false,
            consume_input: false,
            calls: Vec::new(),
        }
    }

    fn emit(
        &mut self,
        cursor: &mut TokenCursor<'_, '_>,
        production: Production,
    ) -> Result<Production, ParserError> {
        self.calls.push(production);

        if self.consume_input {
            cursor.advance()?;
        }

        Ok(production)
    }
}

impl<'tokens, 'src> DeclarationProductions<'tokens, 'src> for MockProductions {
    type Output = Production;

    fn parse_function(
        &mut self,
        cursor: &mut TokenCursor<'tokens, 'src>,
    ) -> Result<Self::Output, ParserError> {
        self.emit(cursor, Production::Function)
    }

    fn parse_variable(
        &mut self,
        cursor: &mut TokenCursor<'tokens, 'src>,
    ) -> Result<Self::Output, ParserError> {
        self.emit(cursor, Production::Variable)
    }

    fn parse_constant(
        &mut self,
        cursor: &mut TokenCursor<'tokens, 'src>,
    ) -> Result<Self::Output, ParserError> {
        self.emit(cursor, Production::Constant)
    }

    fn parse_type(
        &mut self,
        cursor: &mut TokenCursor<'tokens, 'src>,
    ) -> Result<Self::Output, ParserError> {
        self.emit(cursor, Production::Type)
    }

    fn parse_struct(
        &mut self,
        cursor: &mut TokenCursor<'tokens, 'src>,
    ) -> Result<Self::Output, ParserError> {
        self.emit(cursor, Production::Struct)
    }

    fn parse_record(
        &mut self,
        cursor: &mut TokenCursor<'tokens, 'src>,
    ) -> Result<Self::Output, ParserError> {
        self.emit(cursor, Production::Record)
    }

    fn parse_enum(
        &mut self,
        cursor: &mut TokenCursor<'tokens, 'src>,
    ) -> Result<Self::Output, ParserError> {
        self.emit(cursor, Production::Enum)
    }

    fn parse_class(
        &mut self,
        cursor: &mut TokenCursor<'tokens, 'src>,
    ) -> Result<Self::Output, ParserError> {
        self.emit(cursor, Production::Class)
    }

    fn parse_trait(
        &mut self,
        cursor: &mut TokenCursor<'tokens, 'src>,
    ) -> Result<Self::Output, ParserError> {
        self.emit(cursor, Production::Trait)
    }

    fn parse_interface(
        &mut self,
        cursor: &mut TokenCursor<'tokens, 'src>,
    ) -> Result<Self::Output, ParserError> {
        self.emit(cursor, Production::Interface)
    }

    fn parse_implementation(
        &mut self,
        cursor: &mut TokenCursor<'tokens, 'src>,
    ) -> Result<Self::Output, ParserError> {
        self.emit(cursor, Production::Implementation)
    }

    fn recognizes_extension(&self, _kind: TokenKind) -> bool {
        self.extension_enabled
    }

    fn parse_extension(
        &mut self,
        cursor: &mut TokenCursor<'tokens, 'src>,
    ) -> Result<Self::Output, ParserError> {
        self.emit(cursor, Production::Extension)
    }
}

/// Construct a canonical borrowed token for a static test lexeme.
fn token(kind: TokenKind, lexeme: &'static str) -> Token<'static> {
    Token::borrowed(kind, lexeme, Span::default())
}

/// Construct a valid token stream with exactly one final EOF token.
fn token_stream(kind: TokenKind, lexeme: &'static str) -> Vec<Token<'static>> {
    vec![
        token(kind, lexeme),
        Token::synthetic(TokenKind::Eof, Span::default()),
    ]
}

/// Construct a cursor over a valid one-token declaration stream.
fn cursor_for(
    kind: TokenKind,
    lexeme: &'static str,
) -> (Vec<Token<'static>>, TokenCursor<'static, 'static>) {
    // This helper cannot safely return a cursor borrowing a locally owned
    // vector. Call `TokenCursor::new(&tokens)` directly in each test instead.
    let _ = (kind, lexeme);
    unreachable!("use the local-stream cursor helper in each test")
}

/// Run one core declaration-dispatch case and verify the chosen production.
fn assert_dispatches_to(
    kind: TokenKind,
    lexeme: &'static str,
    expected_kind: DeclarationKind,
    expected_production: Production,
) {
    let tokens = token_stream(kind, lexeme);
    let mut cursor = TokenCursor::new(&tokens).expect("valid token stream");
    let mut productions = MockProductions {
        extension_enabled: false,
        consume_input: true,
        calls: Vec::new(),
    };

    assert_eq!(
        DeclarationKind::from_token_kind(kind),
        Some(expected_kind),
        "declaration family classification for {kind:?}",
    );
    assert!(is_declaration_start(kind));

    let actual = parse_declaration(&mut productions, &mut cursor)
        .expect("core declaration should dispatch successfully");

    assert_eq!(actual, expected_production);
    assert_eq!(productions.calls, vec![expected_production]);
    assert_eq!(cursor.position(), 1);
    assert_eq!(cursor.current_kind(), Some(TokenKind::Eof));
}

#[test]
fn every_core_declaration_family_dispatches_to_its_owner() {
    let cases = [
        (
            TokenKind::KeywordFn,
            "fn",
            DeclarationKind::Function,
            Production::Function,
        ),
        (
            TokenKind::KeywordLet,
            "let",
            DeclarationKind::Variable,
            Production::Variable,
        ),
        (
            TokenKind::KeywordVar,
            "var",
            DeclarationKind::Variable,
            Production::Variable,
        ),
        (
            TokenKind::KeywordConst,
            "const",
            DeclarationKind::Constant,
            Production::Constant,
        ),
        (
            TokenKind::KeywordType,
            "type",
            DeclarationKind::Type,
            Production::Type,
        ),
        (
            TokenKind::KeywordStruct,
            "struct",
            DeclarationKind::Struct,
            Production::Struct,
        ),
        (
            TokenKind::KeywordRecord,
            "record",
            DeclarationKind::Record,
            Production::Record,
        ),
        (
            TokenKind::KeywordEnum,
            "enum",
            DeclarationKind::Enum,
            Production::Enum,
        ),
        (
            TokenKind::KeywordClass,
            "class",
            DeclarationKind::Class,
            Production::Class,
        ),
        (
            TokenKind::KeywordTrait,
            "trait",
            DeclarationKind::Trait,
            Production::Trait,
        ),
        (
            TokenKind::KeywordInterface,
            "interface",
            DeclarationKind::Interface,
            Production::Interface,
        ),
        (
            TokenKind::KeywordImpl,
            "impl",
            DeclarationKind::Implementation,
            Production::Implementation,
        ),
    ];

    for (kind, lexeme, declaration_kind, production) in cases {
        assert_dispatches_to(kind, lexeme, declaration_kind, production);
    }
}

#[test]
fn declaration_start_predicate_matches_the_core_classifier() {
    let declaration_kinds = [
        TokenKind::KeywordFn,
        TokenKind::KeywordLet,
        TokenKind::KeywordVar,
        TokenKind::KeywordConst,
        TokenKind::KeywordType,
        TokenKind::KeywordStruct,
        TokenKind::KeywordRecord,
        TokenKind::KeywordEnum,
        TokenKind::KeywordClass,
        TokenKind::KeywordTrait,
        TokenKind::KeywordInterface,
        TokenKind::KeywordImpl,
    ];

    for kind in declaration_kinds {
        assert_eq!(
            is_declaration_start(kind),
            DeclarationKind::from_token_kind(kind).is_some(),
            "start predicate and classifier disagree for {kind:?}",
        );
    }
}

#[test]
fn ordinary_expression_and_control_flow_tokens_are_not_declarations() {
    let non_declarations = [
        TokenKind::Identifier,
        TokenKind::IntegerLiteral,
        TokenKind::StringLiteral,
        TokenKind::LeftBrace,
        TokenKind::Semicolon,
        TokenKind::KeywordIf,
        TokenKind::KeywordReturn,
        TokenKind::Eof,
    ];

    for kind in non_declarations {
        assert_eq!(
            DeclarationKind::from_token_kind(kind),
            None,
            "{kind:?} must not be classified as a core declaration",
        );
        assert!(!is_declaration_start(kind));
    }
}

#[test]
fn enabled_extension_handles_an_unclaimed_declaration_start() {
    let tokens = token_stream(TokenKind::Identifier, "vendor_decl");
    let mut cursor = TokenCursor::new(&tokens).expect("valid token stream");
    let mut productions = MockProductions::with_extension();

    let actual = parse_declaration(&mut productions, &mut cursor)
        .expect("registered extension should handle its declaration");

    assert_eq!(actual, Production::Extension);
    assert_eq!(productions.calls, vec![Production::Extension]);
    assert_eq!(cursor.position(), 1);
    assert_eq!(cursor.current_kind(), Some(TokenKind::Eof));
}

#[test]
fn core_declaration_takes_precedence_over_enabled_extension() {
    let tokens = token_stream(TokenKind::KeywordFn, "fn");
    let mut cursor = TokenCursor::new(&tokens).expect("valid token stream");
    let mut productions = MockProductions::with_extension();

    let actual = parse_declaration(&mut productions, &mut cursor)
        .expect("core declaration must be dispatched before extensions");

    assert_eq!(actual, Production::Function);
    assert_eq!(productions.calls, vec![Production::Function]);
    assert!(!productions.calls.contains(&Production::Extension));
}

#[test]
fn unsupported_declaration_start_returns_structured_error() {
    let tokens = token_stream(TokenKind::Identifier, "not_a_declaration");
    let mut cursor = TokenCursor::new(&tokens).expect("valid token stream");
    let mut productions = MockProductions::default();

    let error = parse_declaration(&mut productions, &mut cursor)
        .expect_err("unclaimed identifier must be rejected");

    assert_eq!(error.kind, ParserErrorKind::UnexpectedToken);
    assert_eq!(productions.calls, Vec::<Production>::new());
    assert_eq!(cursor.position(), 0);
}

#[test]
fn eof_cannot_begin_a_declaration() {
    let tokens = vec![Token::synthetic(TokenKind::Eof, Span::default())];
    let mut cursor = TokenCursor::new(&tokens).expect("valid EOF-only stream");
    let mut productions = MockProductions::with_extension();

    let error = parse_declaration(&mut productions, &mut cursor)
        .expect_err("EOF is not a declaration start");

    assert_eq!(error.kind, ParserErrorKind::UnexpectedToken);
    assert!(productions.calls.is_empty());
    assert_eq!(cursor.position(), 0);
}

#[test]
fn successful_production_must_advance_the_cursor() {
    let tokens = token_stream(TokenKind::KeywordStruct, "struct");
    let mut cursor = TokenCursor::new(&tokens).expect("valid token stream");
    let mut productions = MockProductions::without_consumption();

    let error = parse_declaration(&mut productions, &mut cursor)
        .expect_err("a production that consumes nothing must be rejected");

    assert_eq!(error.kind, ParserErrorKind::NoProgress);
    assert_eq!(productions.calls, vec![Production::Struct]);
    assert_eq!(cursor.position(), 0);
}

#[test]
fn failed_production_error_is_propagated_without_extension_fallback() {
    struct FailingProductions {
        calls: Vec<Production>,
    }

    impl<'tokens, 'src> DeclarationProductions<'tokens, 'src>
        for FailingProductions
    {
        type Output = Production;

        fn parse_function(
            &mut self,
            cursor: &mut TokenCursor<'tokens, 'src>,
        ) -> Result<Self::Output, ParserError> {
            self.calls.push(Production::Function);
            let token = cursor.current().expect("test stream has a token");
            Err(ParserError::at_token(
                ParserErrorKind::UnexpectedToken,
                token,
                "deliberate test failure",
            ))
        }

        fn parse_variable(
            &mut self,
            cursor: &mut TokenCursor<'tokens, 'src>,
        ) -> Result<Self::Output, ParserError> {
            MockProductions::default().emit(cursor, Production::Variable)
        }

        fn parse_constant(
            &mut self,
            cursor: &mut TokenCursor<'tokens, 'src>,
        ) -> Result<Self::Output, ParserError> {
            MockProductions::default().emit(cursor, Production::Constant)
        }

        fn parse_type(
            &mut self,
            cursor: &mut TokenCursor<'tokens, 'src>,
        ) -> Result<Self::Output, ParserError> {
            MockProductions::default().emit(cursor, Production::Type)
        }

        fn parse_struct(
            &mut self,
            cursor: &mut TokenCursor<'tokens, 'src>,
        ) -> Result<Self::Output, ParserError> {
            MockProductions::default().emit(cursor, Production::Struct)
        }

        fn parse_record(
            &mut self,
            cursor: &mut TokenCursor<'tokens, 'src>,
        ) -> Result<Self::Output, ParserError> {
            MockProductions::default().emit(cursor, Production::Record)
        }

        fn parse_enum(
            &mut self,
            cursor: &mut TokenCursor<'tokens, 'src>,
        ) -> Result<Self::Output, ParserError> {
            MockProductions::default().emit(cursor, Production::Enum)
        }

        fn parse_class(
            &mut self,
            cursor: &mut TokenCursor<'tokens, 'src>,
        ) -> Result<Self::Output, ParserError> {
            MockProductions::default().emit(cursor, Production::Class)
        }

        fn parse_trait(
            &mut self,
            cursor: &mut TokenCursor<'tokens, 'src>,
        ) -> Result<Self::Output, ParserError> {
            MockProductions::default().emit(cursor, Production::Trait)
        }

        fn parse_interface(
            &mut self,
            cursor: &mut TokenCursor<'tokens, 'src>,
        ) -> Result<Self::Output, ParserError> {
            MockProductions::default().emit(cursor, Production::Interface)
        }

        fn parse_implementation(
            &mut self,
            cursor: &mut TokenCursor<'tokens, 'src>,
        ) -> Result<Self::Output, ParserError> {
            MockProductions::default().emit(cursor, Production::Implementation)
        }

        fn recognizes_extension(&self, _kind: TokenKind) -> bool {
            true
        }

        fn parse_extension(
            &mut self,
            cursor: &mut TokenCursor<'tokens, 'src>,
        ) -> Result<Self::Output, ParserError> {
            MockProductions::default().emit(cursor, Production::Extension)
        }
    }

    let tokens = token_stream(TokenKind::KeywordFn, "fn");
    let mut cursor = TokenCursor::new(&tokens).expect("valid token stream");
    let mut productions = FailingProductions { calls: Vec::new() };

    let error = parse_declaration(&mut productions, &mut cursor)
        .expect_err("the concrete production error must be returned");

    assert_eq!(error.kind, ParserErrorKind::UnexpectedToken);
    assert_eq!(productions.calls, vec![Production::Function]);
    assert_eq!(cursor.position(), 0);
}
