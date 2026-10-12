
#![forbid(unsafe_code)]

//! Zamani parser round-trip and syntax-fidelity tests.
//!
//! File: src/frontend/parser/tests/roundtrip.rs
//! Rust: 1.97 or later
//! Edition: 2021
//! Safety: Safe Rust only; unsafe Rust is prohibited.
//!
//! # Purpose
//!
//! Verify that the expression parser preserves source syntax deterministically
//! and that parsing the same canonical token stream repeatedly produces an
//! equivalent syntax tree.
//!
//! This file deliberately distinguishes parser round-trip invariants from
//! formatter round-trip invariants. A true parse -> format -> parse test
//! requires an authoritative Zamani expression formatter and a canonical
//! source-to-token lexer entry point. This module does not invent either API.
//!
//! # Ownership
//!
//! This file owns test cases and test-only helpers. It does not own:
//! - lexical scanning or token identities;
//! - grammar productions or operator precedence;
//! - canonical AST definitions or AST conversion;
//! - source formatting or pretty-printing;
//! - semantic analysis, execution, or target selection.
//!
//! # Integration
//!
//! The tests use:
//! - the canonical frontend lexer Token, TokenKind, and Span;
//! - grammar::expressions::parse_expression;
//! - grammar::expressions::parse_expression_prefix;
//! - ExpressionSyntax, ExpressionSyntaxKind, and TokenRange.
//!
//! The parent parser test module must register this module. If the repository
//! uses a different test-module convention, follow that convention rather
//! than creating a second test harness.
//!
//! # Scalability
//!
//! There are no hardware-capacity limits in these tests. Test input sizes
//! are workload parameters, not language restrictions. Parser nesting limits
//! remain explicit safety budgets. No finite machine can process literally
//! infinite input; processing is bounded by available resources and policy.
//!
//! # Completion contract
//!
//! This file validates parse determinism, syntax shape, source-token ranges,
//! prefix consumption, and stable failure classification. It does not claim
//! to prove formatting round-trips until the canonical formatter is integrated.

use crate::frontend::lexer::span::Span;
use crate::frontend::lexer::token::Token;
use crate::frontend::lexer::token_kind::TokenKind;

use crate::frontend::parser::grammar::expressions::{
    parse_expression,
    parse_expression_prefix,
    ExpressionParseError,
    ExpressionParseErrorKind,
    ExpressionParserConfig,
    ExpressionSyntax,
    ExpressionSyntaxKind,
    TokenRange,
};

/// Construct a borrowed canonical lexer token.
///
/// Static spellings keep test lifetimes simple. Production tokens may borrow
/// from a source buffer with a lifetime determined by the caller.
fn token(kind: TokenKind, lexeme: &'static str) -> Token<'static> {
    Token::borrowed(kind, lexeme, Span::default())
}

/// Construct the canonical explicit EOF token.
fn eof() -> Token<'static> {
    Token::synthetic(TokenKind::Eof, Span::default())
}

/// Add an explicit EOF token to a complete expression stream.
fn stream(mut tokens: Vec<Token<'static>>) -> Vec<Token<'static>> {
    tokens.push(eof());
    tokens
}

fn ident(name: &'static str) -> Token<'static> {
    token(TokenKind::Identifier, name)
}

fn integer(raw: &'static str) -> Token<'static> {
    token(TokenKind::IntegerLiteral, raw)
}

fn parse(
    tokens: &[Token<'_>],
) -> Result<ExpressionSyntax, ExpressionParseError> {
    parse_expression(tokens, ExpressionParserConfig::default())
}

fn parse_ok(tokens: &[Token<'static>]) -> ExpressionSyntax {
    parse(tokens).expect("the test expression should parse successfully")
}

fn assert_identifier(expression: &ExpressionSyntax, expected: &str) {
    match &expression.kind {
        ExpressionSyntaxKind::Identifier { name } => {
            assert_eq!(name, expected);
        }
        other => {
            panic!("expected identifier {expected:?}, found {other:?}");
        }
    }
}

fn assert_binary<'a>(
    expression: &'a ExpressionSyntax,
    expected_operator: &str,
) -> (&'a ExpressionSyntax, &'a ExpressionSyntax) {
    match &expression.kind {
        ExpressionSyntaxKind::Binary {
            operator,
            left,
            right,
        } => {
            assert_eq!(operator, expected_operator);
            (left.as_ref(), right.as_ref())
        }
        other => {
            panic!(
                "expected binary operator {expected_operator:?}, found {other:?}"
            );
        }
    }
}

// ---------------------------------------------------------------------------
// Determinism and structural preservation
// ---------------------------------------------------------------------------

#[test]
fn repeated_parses_produce_identical_syntax_trees() {
    let tokens = stream(vec![
        ident("result"),
        token(TokenKind::Assign, "="),
        ident("left"),
        token(TokenKind::Plus, "+"),
        ident("right"),
        token(TokenKind::Star, "*"),
        integer("17"),
    ]);

    let first = parse_ok(&tokens);
    let second = parse_ok(&tokens);
    let third = parse_ok(&tokens);

    assert_eq!(first, second);
    assert_eq!(second, third);
}

#[test]
fn repeated_parses_preserve_token_ranges() {
    let tokens = stream(vec![
        ident("alpha"),
        token(TokenKind::Plus, "+"),
        ident("beta"),
    ]);

    let first = parse_ok(&tokens);
    let second = parse_ok(&tokens);

    assert_eq!(first.tokens, TokenRange::new(0, 3));
    assert_eq!(first.tokens, second.tokens);
    assert_eq!(first, second);
}

#[test]
fn binary_expression_retains_operand_structure_and_operator_spelling() {
    let tokens = stream(vec![
        ident("left"),
        token(TokenKind::Plus, "+"),
        ident("right"),
    ]);

    let expression = parse_ok(&tokens);
    let (left, right) = assert_binary(&expression, "+");

    assert_identifier(left, "left");
    assert_identifier(right, "right");
    assert_eq!(expression.tokens, TokenRange::new(0, 3));
}

#[test]
fn operator_precedence_is_stable_across_repeated_parses() {
    // a + b * c must remain a + (b * c).
    let tokens = stream(vec![
        ident("a"),
        token(TokenKind::Plus, "+"),
        ident("b"),
        token(TokenKind::Star, "*"),
        ident("c"),
    ]);

    let first = parse_ok(&tokens);
    let second = parse_ok(&tokens);

    assert_eq!(first, second);

    let (left, right) = assert_binary(&first, "+");
    assert_identifier(left, "a");

    let (multiply_left, multiply_right) = assert_binary(right, "*");
    assert_identifier(multiply_left, "b");
    assert_identifier(multiply_right, "c");
}

#[test]
fn explicit_parentheses_survive_as_group_syntax() {
    let tokens = stream(vec![
        token(TokenKind::LeftParen, "("),
        ident("a"),
        token(TokenKind::Plus, "+"),
        ident("b"),
        token(TokenKind::RightParen, ")"),
        token(TokenKind::Star, "*"),
        ident("c"),
    ]);

    let expression = parse_ok(&tokens);
    let (left, right) = assert_binary(&expression, "*");

    match &left.kind {
        ExpressionSyntaxKind::Group { expression: inner } => {
            let (inner_left, inner_right) = assert_binary(inner, "+");
            assert_identifier(inner_left, "a");
            assert_identifier(inner_right, "b");
        }
        other => panic!("expected preserved group syntax, found {other:?}"),
    }

    assert_identifier(right, "c");
}

// ---------------------------------------------------------------------------
// Literal and identifier fidelity
// ---------------------------------------------------------------------------

#[test]
fn literal_spelling_is_not_narrowed_to_a_machine_integer() {
    let raw = "12345678901234567890123456789012345678901234567890";
    let tokens = stream(vec![integer(raw)]);

    let expression = parse_ok(&tokens);

    match &expression.kind {
        ExpressionSyntaxKind::Literal {
            raw: preserved,
            token_kind,
        } => {
            assert_eq!(preserved, raw);
            assert_eq!(*token_kind, TokenKind::IntegerLiteral);
        }
        other => panic!("expected literal syntax, found {other:?}"),
    }
}

#[test]
fn unicode_identifier_spelling_is_preserved() {
    let name = "данные_Δ_変数";
    let tokens = stream(vec![ident(name)]);

    let first = parse_ok(&tokens);
    let second = parse_ok(&tokens);

    assert_identifier(&first, name);
    assert_eq!(first, second);
}

#[test]
fn boolean_and_nil_literals_preserve_their_lexical_kinds() {
    let cases = [
        (TokenKind::KeywordTrue, "true"),
        (TokenKind::KeywordFalse, "false"),
        (TokenKind::KeywordNil, "nil"),
    ];

    for (kind, spelling) in cases {
        let tokens = stream(vec![token(kind, spelling)]);

        let first = parse_ok(&tokens);
        let second = parse_ok(&tokens);

        assert_eq!(first, second);

        match &first.kind {
            ExpressionSyntaxKind::Literal {
                raw,
                token_kind,
            } => {
                assert_eq!(raw, spelling);
                assert_eq!(*token_kind, kind);
            }
            other => {
                panic!("expected literal for {spelling:?}, found {other:?}");
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Compound expression round-trip invariants
// ---------------------------------------------------------------------------

#[test]
fn assignment_expression_preserves_target_and_value() {
    let tokens = stream(vec![
        ident("destination"),
        token(TokenKind::Assign, "="),
        ident("source"),
    ]);

    let first = parse_ok(&tokens);
    let second = parse_ok(&tokens);

    assert_eq!(first, second);

    match &first.kind {
        ExpressionSyntaxKind::Assignment {
            operator,
            target,
            value,
        } => {
            assert_eq!(operator, "=");
            assert_identifier(target, "destination");
            assert_identifier(value, "source");
        }
        other => panic!("expected assignment syntax, found {other:?}"),
    }
}

#[test]
fn conditional_expression_preserves_all_three_operands() {
    let tokens = stream(vec![
        ident("condition"),
        token(TokenKind::Question, "?"),
        ident("when_true"),
        token(TokenKind::Colon, ":"),
        ident("when_false"),
    ]);

    let first = parse_ok(&tokens);
    let second = parse_ok(&tokens);

    assert_eq!(first, second);

    match &first.kind {
        ExpressionSyntaxKind::Conditional {
            condition,
            then_value,
            else_value,
        } => {
            assert_identifier(condition, "condition");
            assert_identifier(then_value, "when_true");
            assert_identifier(else_value, "when_false");
        }
        other => panic!("expected conditional syntax, found {other:?}"),
    }
}

#[test]
fn nested_calls_and_member_access_remain_structurally_stable() {
    let tokens = stream(vec![
        ident("service"),
        token(TokenKind::Dot, "."),
        ident("compute"),
        token(TokenKind::LeftParen, "("),
        ident("input"),
        token(TokenKind::RightParen, ")"),
    ]);

    let first = parse_ok(&tokens);
    let second = parse_ok(&tokens);

    assert_eq!(first, second);

    match &first.kind {
        ExpressionSyntaxKind::Call { callee, arguments } => {
            assert_eq!(arguments.len(), 1);
            assert_identifier(&arguments[0], "input");

            match &callee.kind {
                ExpressionSyntaxKind::MemberAccess { object, member } => {
                    assert_identifier(object, "service");
                    assert_eq!(member, "compute");
                }
                other => {
                    panic!("expected member-access callee, found {other:?}");
                }
            }
        }
        other => panic!("expected call syntax, found {other:?}"),
    }
}

#[test]
fn array_elements_remain_in_source_order() {
    let tokens = stream(vec![
        token(TokenKind::LeftBracket, "["),
        ident("first"),
        token(TokenKind::Comma, ","),
        ident("second"),
        token(TokenKind::Comma, ","),
        ident("third"),
        token(TokenKind::RightBracket, "]"),
    ]);

    let first = parse_ok(&tokens);
    let second = parse_ok(&tokens);

    assert_eq!(first, second);

    match &first.kind {
        ExpressionSyntaxKind::Array { elements } => {
            assert_eq!(elements.len(), 3);
            assert_identifier(&elements[0], "first");
            assert_identifier(&elements[1], "second");
            assert_identifier(&elements[2], "third");
        }
        other => panic!("expected array syntax, found {other:?}"),
    }
}

// ---------------------------------------------------------------------------
// Prefix parsing and input-consumption contracts
// ---------------------------------------------------------------------------

#[test]
fn prefix_parser_leaves_the_enclosing_separator_unconsumed() {
    let tokens = vec![
        ident("value"),
        token(TokenKind::Comma, ","),
        ident("next"),
        eof(),
    ];

    let (expression, next_position) = parse_expression_prefix(
        &tokens,
        0,
        ExpressionParserConfig::default(),
    )
    .expect("the first expression should parse");

    assert_identifier(&expression, "value");
    assert_eq!(next_position, 1);
    assert_eq!(tokens[next_position].kind, TokenKind::Comma);
}

#[test]
fn prefix_parser_can_resume_at_the_next_expression() {
    let tokens = vec![
        ident("first"),
        token(TokenKind::Comma, ","),
        ident("second"),
        eof(),
    ];

    let (_, after_first) = parse_expression_prefix(
        &tokens,
        0,
        ExpressionParserConfig::default(),
    )
    .expect("first expression should parse");

    assert_eq!(tokens[after_first].kind, TokenKind::Comma);

    let (second, after_second) = parse_expression_prefix(
        &tokens,
        after_first + 1,
        ExpressionParserConfig::default(),
    )
    .expect("second expression should parse");

    assert_identifier(&second, "second");
    assert_eq!(tokens[after_second].kind, TokenKind::Eof);
}

#[test]
fn complete_expression_parser_rejects_trailing_non_eof_tokens() {
    let tokens = stream(vec![
        ident("first"),
        ident("unexpected"),
    ]);

    let error = parse(&tokens)
        .expect_err("a complete expression must reject trailing tokens");

    assert_eq!(error.kind, ExpressionParseErrorKind::TrailingTokens);
}

#[test]
fn prefix_parser_rejects_a_start_position_past_the_token_slice() {
    let tokens = stream(vec![ident("value")]);

    let error = parse_expression_prefix(
        &tokens,
        tokens.len() + 1,
        ExpressionParserConfig::default(),
    )
    .expect_err("an invalid start index must be rejected");

    assert_eq!(error.kind, ExpressionParseErrorKind::UnexpectedToken);
}

// ---------------------------------------------------------------------------
// Malformed input and resource-budget behavior
// ---------------------------------------------------------------------------

#[test]
fn empty_input_returns_a_structured_error() {
    let tokens: Vec<Token<'static>> = Vec::new();

    let error = parse(&tokens)
        .expect_err("empty input cannot produce an expression");

    assert_eq!(error.kind, ExpressionParseErrorKind::ExpectedExpression);
}

#[test]
fn missing_closing_parenthesis_returns_a_structured_error() {
    let tokens = stream(vec![
        token(TokenKind::LeftParen, "("),
        ident("value"),
    ]);

    let error = parse(&tokens)
        .expect_err("an unclosed group must not parse successfully");

    assert_eq!(error.kind, ExpressionParseErrorKind::ExpectedDelimiter);
}

#[test]
fn zero_nesting_budget_is_rejected() {
    let tokens = stream(vec![ident("value")]);

    let error = parse_expression(
        &tokens,
        ExpressionParserConfig { max_nesting: 0 },
    )
    .expect_err("a zero nesting budget is invalid");

    assert_eq!(error.kind, ExpressionParseErrorKind::InvalidConfiguration);
}

#[test]
fn configured_nesting_budget_exhaustion_is_reported() {
    let tokens = stream(vec![
        token(TokenKind::LeftParen, "("),
        token(TokenKind::LeftParen, "("),
        ident("value"),
        token(TokenKind::RightParen, ")"),
        token(TokenKind::RightParen, ")"),
    ]);

    let config = ExpressionParserConfig { max_nesting: 1 };

    let first = parse_expression(&tokens, config)
        .expect_err("nested input should exceed the configured budget");
    let second = parse_expression(&tokens, config)
        .expect_err("the same input should fail deterministically");

    assert_eq!(first.kind, ExpressionParseErrorKind::NestingBudgetExceeded);
    assert_eq!(first, second);
}

#[test]
fn malformed_input_failure_is_deterministic() {
    let tokens = stream(vec![
        ident("value"),
        token(TokenKind::Plus, "+"),
    ]);

    let first = parse(&tokens)
        .expect_err("an operator without a right operand is invalid");
    let second = parse(&tokens)
        .expect_err("repeated parsing should return the same failure");

    assert_eq!(first, second);
}

// ---------------------------------------------------------------------------
// Canonical formatter integration gate
// ---------------------------------------------------------------------------

/// A true source-format round-trip test must be added when the repository
/// exposes both:
///
/// 1. a canonical formatter for `ExpressionSyntax` or the canonical AST; and
/// 2. the canonical source-to-token lexer entry point.
///
/// The test must then perform:
///
///     source -> canonical lexer -> parser -> canonical formatter
///            -> canonical lexer -> parser
///
/// and compare syntax structurally while explicitly handling source spans
/// and token ranges, which naturally change when formatting changes.
///
/// Do not replace this contract with `Debug` formatting, ad-hoc string
/// concatenation, or a test-only lexer. Those would test a different format
/// from the language's actual source syntax.
