
#![forbid(unsafe_code)]

//! Production expression-parser conformance tests for Zamani.
//!
//! File: src/frontend/parser/tests/expressions.rs
//! Rust: 1.97 or later
//! Edition: 2021
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Verify expression syntax parsing against the canonical lexer, expression
//! syntax representation, parser configuration, and normative grammar.
//!
//! # Ownership
//!
//! This file owns tests and test-only helpers. It does not implement grammar,
//! define AST nodes, introduce a second token model, or perform semantic
//! analysis.
//!
//! # Dependencies
//!
//! - `crate::frontend::lexer::span::Span`
//! - `crate::frontend::lexer::token::Token`
//! - `crate::frontend::lexer::token_kind::TokenKind`
//! - `crate::frontend::parser::grammar::expressions`
//!
//! # Normative references
//!
//! - `grammar/expressions/precedence.md`
//! - `grammar/expressions/postfix.g4`
//! - `grammar/expressions/calls.g4`
//! - `grammar/expressions/indexing.g4`
//! - `grammar/expressions/member-access.g4`
//! - `grammar/expressions/lambdas.g4`
//! - `grammar/expressions/expressions.g4`
//!
//! # Integration contract
//!
//! These tests exercise the expression syntax parser directly with canonical
//! lexer tokens. They intentionally do not duplicate tokenization, operator
//! registration, source-span calculations, AST conversion, or semantic
//! validation.
//!
//! The parent `tests/mod.rs` must include this module. The parser's production
//! test suite must also test the public parser API and canonical AST adapter.
//!
//! # Scalability and safety
//!
//! Tests introduce no hardware-capacity ceilings. Stress sizes are test
//! workload parameters, not language limits. Production resource budgets
//! remain configurable and must fail explicitly when exhausted.
//!
//! No test relies on unsafe Rust, undefined behavior, global mutable state,
//! target-specific capacities, or timing-dependent assertions.
//!
//! # Important limitation
//!
//! Passing this suite does not by itself prove that the entire Zamani
//! expression grammar is implemented. Known conformance gaps are documented
//! in explicitly ignored tests and must be resolved before production release.

use crate::frontend::lexer::span::Span;
use crate::frontend::lexer::token::Token;
use crate::frontend::lexer::token_kind::TokenKind;

use crate::frontend::parser::grammar::expressions::{
    parse_expression,
    parse_expression_prefix,
    ExpressionParseErrorKind,
    ExpressionParserConfig,
    ExpressionSyntax,
    ExpressionSyntaxKind,
    TokenRange,
};

/// Create a borrowed token using the canonical lexer token representation.
///
/// All spellings used by this helper are static test strings. Production
/// lexers may instead produce tokens borrowing from an entire source buffer.
fn token(kind: TokenKind, lexeme: &'static str) -> Token<'static> {
    Token::borrowed(kind, lexeme, Span::default())
}

/// Construct an explicit end-of-input token.
fn eof() -> Token<'static> {
    Token::synthetic(TokenKind::Eof, Span::default())
}

/// Construct a complete expression token stream.
///
/// The parser permits an optional EOF token, but tests consistently include
/// one to exercise the normal lexer-to-parser integration boundary.
fn stream(mut tokens: Vec<Token<'static>>) -> Vec<Token<'static>> {
    tokens.push(eof());
    tokens
}

/// Parse an expression using the standard expression-parser configuration.
fn parse(tokens: &[Token<'_>]) -> Result<ExpressionSyntax, crate::frontend::parser::grammar::expressions::ExpressionParseError> {
    parse_expression(tokens, ExpressionParserConfig::default())
}

/// Require a successful expression parse.
fn parsed(tokens: &[Token<'static>]) -> ExpressionSyntax {
    parse(tokens).expect("expected expression to parse successfully")
}

/// Assert that a syntax node is a binary expression and return its children.
fn binary(
    expression: &ExpressionSyntax,
    expected_operator: &str,
) -> (&ExpressionSyntax, &ExpressionSyntax) {
    match &expression.kind {
        ExpressionSyntaxKind::Binary {
            operator,
            left,
            right,
        } => {
            assert_eq!(operator, expected_operator);
            (left.as_ref(), right.as_ref())
        }
        other => panic!("expected binary expression, found {other:?}"),
    }
}

/// Assert that a syntax node is an identifier with the expected spelling.
fn identifier(expression: &ExpressionSyntax, expected: &str) {
    match &expression.kind {
        ExpressionSyntaxKind::Identifier { name } => {
            assert_eq!(name, expected);
        }
        other => panic!("expected identifier {expected:?}, found {other:?}"),
    }
}

/// Build an identifier token.
fn ident(name: &'static str) -> Token<'static> {
    token(TokenKind::Identifier, name)
}

/// Build an integer-literal token.
fn integer(raw: &'static str) -> Token<'static> {
    token(TokenKind::IntegerLiteral, raw)
}

// ---------------------------------------------------------------------------
// Basic expressions and source-token fidelity
// ---------------------------------------------------------------------------

#[test]
fn parses_identifier() {
    let tokens = stream(vec![ident("value")]);
    let expression = parsed(&tokens);

    identifier(&expression, "value");
    assert_eq!(expression.tokens, TokenRange::new(0, 1));
}

#[test]
fn parses_integer_literal_without_target_specific_conversion() {
    let tokens = stream(vec![integer("123456789012345678901234567890")]);
    let expression = parsed(&tokens);

    match expression.kind {
        ExpressionSyntaxKind::Literal { raw, token_kind } => {
            assert_eq!(raw, "123456789012345678901234567890");
            assert_eq!(token_kind, TokenKind::IntegerLiteral);
        }
        other => panic!("expected literal syntax, found {other:?}"),
    }
}

#[test]
fn parses_boolean_and_nil_literals() {
    let cases = [
        (TokenKind::KeywordTrue, "true"),
        (TokenKind::KeywordFalse, "false"),
        (TokenKind::KeywordNil, "nil"),
    ];

    for (kind, spelling) in cases {
        let tokens = stream(vec![token(kind, spelling)]);
        let expression = parsed(&tokens);

        assert!(
            matches!(
                expression.kind,
                ExpressionSyntaxKind::Literal { .. }
            ),
            "expected literal syntax for {spelling}"
        );
    }
}

#[test]
fn preserves_unicode_identifier_spelling() {
    let tokens = stream(vec![ident("данные_Δ_変数")]);
    let expression = parsed(&tokens);

    identifier(&expression, "данные_Δ_変数");
}

#[test]
fn preserves_the_original_token_range() {
    let tokens = stream(vec![
        ident("left"),
        token(TokenKind::Plus, "+"),
        ident("right"),
    ]);

    let expression = parsed(&tokens);

    assert_eq!(expression.tokens, TokenRange::new(0, 3));

    let (left, right) = binary(&expression, "+");
    identifier(left, "left");
    identifier(right, "right");
}

// ---------------------------------------------------------------------------
// Precedence and associativity
// ---------------------------------------------------------------------------

#[test]
fn multiplication_binds_more_tightly_than_addition() {
    // a + b * c => a + (b * c)
    let tokens = stream(vec![
        ident("a"),
        token(TokenKind::Plus, "+"),
        ident("b"),
        token(TokenKind::Star, "*"),
        ident("c"),
    ]);

    let expression = parsed(&tokens);
    let (left, right) = binary(&expression, "+");

    identifier(left, "a");

    let (multiply_left, multiply_right) = binary(right, "*");
    identifier(multiply_left, "b");
    identifier(multiply_right, "c");
}

#[test]
fn subtraction_is_left_associative() {
    // a - b - c => (a - b) - c
    let tokens = stream(vec![
        ident("a"),
        token(TokenKind::Minus, "-"),
        ident("b"),
        token(TokenKind::Minus, "-"),
        ident("c"),
    ]);

    let expression = parsed(&tokens);
    let (left, right) = binary(&expression, "-");

    let (inner_left, inner_right) = binary(left, "-");
    identifier(inner_left, "a");
    identifier(inner_right, "b");
    identifier(right, "c");
}

#[test]
fn assignment_is_right_associative() {
    // a = b = c => a = (b = c)
    let tokens = stream(vec![
        ident("a"),
        token(TokenKind::Assign, "="),
        ident("b"),
        token(TokenKind::Assign, "="),
        ident("c"),
    ]);

    let expression = parsed(&tokens);

    match &expression.kind {
        ExpressionSyntaxKind::Assignment {
            target,
            value,
            ..
        } => {
            identifier(target, "a");

            match &value.kind {
                ExpressionSyntaxKind::Assignment {
                    target: inner_target,
                    value: inner_value,
                    ..
                } => {
                    identifier(inner_target, "b");
                    identifier(inner_value, "c");
                }
                other => {
                    panic!("expected right-nested assignment, found {other:?}")
                }
            }
        }
        other => panic!("expected assignment expression, found {other:?}"),
    }
}

#[test]
fn logical_and_binds_more_tightly_than_logical_or() {
    // a || b && c => a || (b && c)
    let tokens = stream(vec![
        ident("a"),
        token(TokenKind::LogicalOr, "||"),
        ident("b"),
        token(TokenKind::LogicalAnd, "&&"),
        ident("c"),
    ]);

    let expression = parsed(&tokens);
    let (left, right) = binary(&expression, "||");

    identifier(left, "a");

    let (and_left, and_right) = binary(right, "&&");
    identifier(and_left, "b");
    identifier(and_right, "c");
}

#[test]
fn prefix_operators_bind_more_tightly_than_multiplication() {
    // -a * b => (-a) * b
    let tokens = stream(vec![
        token(TokenKind::Minus, "-"),
        ident("a"),
        token(TokenKind::Star, "*"),
        ident("b"),
    ]);

    let expression = parsed(&tokens);
    let (left, right) = binary(&expression, "*");

    match &left.kind {
        ExpressionSyntaxKind::Unary { operator, operand } => {
            assert_eq!(operator, "-");
            identifier(operand, "a");
        }
        other => panic!("expected unary left operand, found {other:?}"),
    }

    identifier(right, "b");
}

#[test]
fn parentheses_override_operator_precedence() {
    // (a + b) * c
    let tokens = stream(vec![
        token(TokenKind::LeftParen, "("),
        ident("a"),
        token(TokenKind::Plus, "+"),
        ident("b"),
        token(TokenKind::RightParen, ")"),
        token(TokenKind::Star, "*"),
        ident("c"),
    ]);

    let expression = parsed(&tokens);
    let (left, right) = binary(&expression, "*");

    match &left.kind {
        ExpressionSyntaxKind::Group { expression } => {
            let (group_left, group_right) = binary(expression, "+");
            identifier(group_left, "a");
            identifier(group_right, "b");
        }
        other => panic!("expected explicit group, found {other:?}"),
    }

    identifier(right, "c");
}

#[test]
fn conditional_expression_preserves_all_three_operands() {
    // condition ? consequent : alternative
    let tokens = stream(vec![
        ident("condition"),
        token(TokenKind::Question, "?"),
        ident("consequent"),
        token(TokenKind::Colon, ":"),
        ident("alternative"),
    ]);

    let expression = parsed(&tokens);

    match expression.kind {
        ExpressionSyntaxKind::Conditional {
            condition,
            then_value,
            else_value,
        } => {
            identifier(&condition, "condition");
            identifier(&then_value, "consequent");
            identifier(&else_value, "alternative");
        }
        other => panic!("expected conditional expression, found {other:?}"),
    }
}

// ---------------------------------------------------------------------------
// Grouping, tuples, arrays, calls, indexing, and member access
// ---------------------------------------------------------------------------

#[test]
fn parses_parenthesized_expression() {
    let tokens = stream(vec![
        token(TokenKind::LeftParen, "("),
        ident("value"),
        token(TokenKind::RightParen, ")"),
    ]);

    let expression = parsed(&tokens);

    match expression.kind {
        ExpressionSyntaxKind::Group { expression } => {
            identifier(&expression, "value");
        }
        other => panic!("expected grouped expression, found {other:?}"),
    }
}

#[test]
fn parses_empty_tuple() {
    let tokens = stream(vec![
        token(TokenKind::LeftParen, "("),
        token(TokenKind::RightParen, ")"),
    ]);

    let expression = parsed(&tokens);

    assert!(matches!(
        expression.kind,
        ExpressionSyntaxKind::Tuple { ref elements } if elements.is_empty()
    ));
}

#[test]
fn parses_tuple_with_multiple_elements_and_trailing_comma() {
    let tokens = stream(vec![
        token(TokenKind::LeftParen, "("),
        ident("first"),
        token(TokenKind::Comma, ","),
        integer("2"),
        token(TokenKind::Comma, ","),
        ident("third"),
        token(TokenKind::Comma, ","),
        token(TokenKind::RightParen, ")"),
    ]);

    let expression = parsed(&tokens);

    match expression.kind {
        ExpressionSyntaxKind::Tuple { elements } => {
            assert_eq!(elements.len(), 3);
            identifier(&elements[0], "first");
            assert!(matches!(
                elements[1].kind,
                ExpressionSyntaxKind::Literal { .. }
            ));
            identifier(&elements[2], "third");
        }
        other => panic!("expected tuple expression, found {other:?}"),
    }
}

#[test]
fn parses_empty_array() {
    let tokens = stream(vec![
        token(TokenKind::LeftBracket, "["),
        token(TokenKind::RightBracket, "]"),
    ]);

    let expression = parsed(&tokens);

    assert!(matches!(
        expression.kind,
        ExpressionSyntaxKind::Array { ref elements } if elements.is_empty()
    ));
}

#[test]
fn parses_array_with_multiple_elements_and_trailing_comma() {
    let tokens = stream(vec![
        token(TokenKind::LeftBracket, "["),
        integer("1"),
        token(TokenKind::Comma, ","),
        integer("2"),
        token(TokenKind::Comma, ","),
        integer("3"),
        token(TokenKind::Comma, ","),
        token(TokenKind::RightBracket, "]"),
    ]);

    let expression = parsed(&tokens);

    assert!(matches!(
        expression.kind,
        ExpressionSyntaxKind::Array { ref elements } if elements.len() == 3
    ));
}

#[test]
fn parses_function_call() {
    // calculate(first, second)
    let tokens = stream(vec![
        ident("calculate"),
        token(TokenKind::LeftParen, "("),
        ident("first"),
        token(TokenKind::Comma, ","),
        ident("second"),
        token(TokenKind::RightParen, ")"),
    ]);

    let expression = parsed(&tokens);

    match expression.kind {
        ExpressionSyntaxKind::Call { callee, arguments } => {
            identifier(&callee, "calculate");
            assert_eq!(arguments.len(), 2);
            identifier(&arguments[0], "first");
            identifier(&arguments[1], "second");
        }
        other => panic!("expected call expression, found {other:?}"),
    }
}

#[test]
fn parses_empty_call() {
    let tokens = stream(vec![
        ident("compute"),
        token(TokenKind::LeftParen, "("),
        token(TokenKind::RightParen, ")"),
    ]);

    let expression = parsed(&tokens);

    assert!(matches!(
        expression.kind,
        ExpressionSyntaxKind::Call { ref arguments, .. }
            if arguments.is_empty()
    ));
}

#[test]
fn parses_indexing() {
    let tokens = stream(vec![
        ident("values"),
        token(TokenKind::LeftBracket, "["),
        ident("index"),
        token(TokenKind::RightBracket, "]"),
    ]);

    let expression = parsed(&tokens);

    match expression.kind {
        ExpressionSyntaxKind::Index { object, index } => {
            identifier(&object, "values");
            identifier(&index, "index");
        }
        other => panic!("expected indexing expression, found {other:?}"),
    }
}

#[test]
fn parses_chained_indexing() {
    // values[i][j]
    let tokens = stream(vec![
        ident("values"),
        token(TokenKind::LeftBracket, "["),
        ident("i"),
        token(TokenKind::RightBracket, "]"),
        token(TokenKind::LeftBracket, "["),
        ident("j"),
        token(TokenKind::RightBracket, "]"),
    ]);

    let expression = parsed(&tokens);

    match expression.kind {
        ExpressionSyntaxKind::Index { object, index } => {
            identifier(&index, "j");

            match object.kind {
                ExpressionSyntaxKind::Index {
                    object: inner_object,
                    index: inner_index,
                } => {
                    identifier(&inner_object, "values");
                    identifier(&inner_index, "i");
                }
                other => panic!("expected inner indexing, found {other:?}"),
            }
        }
        other => panic!("expected outer indexing, found {other:?}"),
    }
}

#[test]
fn parses_member_access() {
    let tokens = stream(vec![
        ident("record"),
        token(TokenKind::Dot, "."),
        ident("field"),
    ]);

    let expression = parsed(&tokens);

    match expression.kind {
        ExpressionSyntaxKind::MemberAccess { object, member } => {
            identifier(&object, "record");
            assert_eq!(member, "field");
        }
        other => panic!("expected member access, found {other:?}"),
    }
}

#[test]
fn parses_chained_member_and_call_access() {
    // object.method(argument).result
    let tokens = stream(vec![
        ident("object"),
        token(TokenKind::Dot, "."),
        ident("method"),
        token(TokenKind::LeftParen, "("),
        ident("argument"),
        token(TokenKind::RightParen, ")"),
        token(TokenKind::Dot, "."),
        ident("result"),
    ]);

    let expression = parsed(&tokens);

    match expression.kind {
        ExpressionSyntaxKind::MemberAccess { object, member } => {
            assert_eq!(member, "result");

            match object.kind {
                ExpressionSyntaxKind::Call { callee, arguments } => {
                    assert_eq!(arguments.len(), 1);
                    identifier(&arguments[0], "argument");

                    match callee.kind {
                        ExpressionSyntaxKind::MemberAccess {
                            object: receiver,
                            member: method,
                        } => {
                            identifier(&receiver, "object");
                            assert_eq!(method, "method");
                        }
                        other => {
                            panic!("expected method selection, found {other:?}")
                        }
                    }
                }
                other => panic!("expected method call, found {other:?}"),
            }
        }
        other => panic!("expected final member access, found {other:?}"),
    }
}

// ---------------------------------------------------------------------------
// Trivia, EOF, and token consumption
// ---------------------------------------------------------------------------

#[test]
fn ignores_trivia_between_expression_tokens() {
    let tokens = stream(vec![
        ident("left"),
        token(TokenKind::Whitespace, " "),
        token(TokenKind::Plus, "+"),
        token(TokenKind::Whitespace, "\t"),
        ident("right"),
    ]);

    let expression = parsed(&tokens);

    let (left, right) = binary(&expression, "+");
    identifier(left, "left");
    identifier(right, "right");
}

#[test]
fn accepts_expression_without_explicit_eof() {
    let tokens = vec![ident("value")];
    let expression = parsed(&tokens);

    identifier(&expression, "value");
}

#[test]
fn rejects_trailing_non_trivia_tokens() {
    let tokens = stream(vec![ident("first"), ident("unexpected")]);

    let error = parse(&tokens).expect_err("trailing token must be rejected");

    assert_eq!(error.kind, ExpressionParseErrorKind::TrailingTokens);
}

#[test]
fn prefix_parser_leaves_enclosing_grammar_input_unconsumed() {
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
    .expect("expression prefix should parse");

    identifier(&expression, "value");
    assert_eq!(next_position, 1);
    assert_eq!(tokens[next_position].kind(), TokenKind::Comma);
}

#[test]
fn prefix_parser_rejects_start_beyond_token_slice() {
    let tokens = vec![ident("value"), eof()];

    let error = parse_expression_prefix(
        &tokens,
        tokens.len() + 1,
        ExpressionParserConfig::default(),
    )
    .expect_err("out-of-range starting position must be rejected");

    assert_eq!(error.kind, ExpressionParseErrorKind::UnexpectedToken);
}

#[test]
fn rejects_zero_nesting_budget() {
    let tokens = stream(vec![ident("value")]);

    let error = parse_expression(
        &tokens,
        ExpressionParserConfig { max_nesting: 0 },
    )
    .expect_err("zero nesting budget must be rejected");

    assert_eq!(
        error.kind,
        ExpressionParseErrorKind::InvalidConfiguration
    );
}

// ---------------------------------------------------------------------------
// Malformed expressions and deterministic failures
// ---------------------------------------------------------------------------

#[test]
fn rejects_missing_right_operand() {
    let tokens = stream(vec![
        ident("left"),
        token(TokenKind::Plus, "+"),
    ]);

    let error = parse(&tokens).expect_err("missing operand must be rejected");

    assert_eq!(error.kind, ExpressionParseErrorKind::ExpectedExpression);
}

#[test]
fn rejects_missing_closing_parenthesis() {
    let tokens = stream(vec![
        token(TokenKind::LeftParen, "("),
        ident("value"),
    ]);

    let error = parse(&tokens).expect_err("missing ')' must be rejected");

    assert_eq!(error.kind, ExpressionParseErrorKind::ExpectedDelimiter);
}

#[test]
fn rejects_missing_closing_bracket() {
    let tokens = stream(vec![
        ident("values"),
        token(TokenKind::LeftBracket, "["),
        ident("index"),
    ]);

    let error = parse(&tokens).expect_err("missing ']' must be rejected");

    assert_eq!(error.kind, ExpressionParseErrorKind::ExpectedDelimiter);
}

#[test]
fn rejects_call_with_missing_argument() {
    let tokens = stream(vec![
        ident("call"),
        token(TokenKind::LeftParen, "("),
        ident("first"),
        token(TokenKind::Comma, ","),
        token(TokenKind::RightParen, ")"),
    ]);

    let error = parse(&tokens).expect_err("missing argument must be rejected");

    assert_eq!(error.kind, ExpressionParseErrorKind::ExpectedExpression);
}

#[test]
fn rejects_member_access_without_member_name() {
    let tokens = stream(vec![
        ident("object"),
        token(TokenKind::Dot, "."),
        token(TokenKind::Eof, ""),
    ]);

    let error = parse(&tokens).expect_err("member name must be required");

    assert_eq!(error.kind, ExpressionParseErrorKind::ExpectedExpression);
}

#[test]
fn repeated_parse_of_identical_input_is_deterministic() {
    let tokens = stream(vec![
        ident("a"),
        token(TokenKind::Plus, "+"),
        ident("b"),
        token(TokenKind::Star, "*"),
        integer("7"),
    ]);

    let first = parse(&tokens).expect("first parse should succeed");
    let second = parse(&tokens).expect("second parse should succeed");

    assert_eq!(first, second);
}

// ---------------------------------------------------------------------------
// Configurable nesting and scalable syntax collections
// ---------------------------------------------------------------------------

#[test]
fn parses_nested_grouping_within_configured_budget() {
    let nesting = 64usize;
    let mut tokens = Vec::with_capacity(nesting * 2 + 2);

    for _ in 0..nesting {
        tokens.push(token(TokenKind::LeftParen, "("));
    }

    tokens.push(ident("value"));

    for _ in 0..nesting {
        tokens.push(token(TokenKind::RightParen, ")"));
    }

    tokens.push(eof());

    let expression = parse_expression(
        &tokens,
        ExpressionParserConfig {
            max_nesting: nesting + 8,
        },
    )
    .expect("configured nesting budget should permit this expression");

    assert_eq!(expression.tokens.start, 0);
    assert_eq!(expression.tokens.end, nesting * 2 + 1);
}

#[test]
fn reports_nesting_budget_exhaustion() {
    let tokens = stream(vec![
        token(TokenKind::LeftParen, "("),
        token(TokenKind::LeftParen, "("),
        token(TokenKind::LeftParen, "("),
        ident("value"),
        token(TokenKind::RightParen, ")"),
        token(TokenKind::RightParen, ")"),
        token(TokenKind::RightParen, ")"),
    ]);

    let error = parse_expression(
        &tokens,
        ExpressionParserConfig { max_nesting: 2 },
    )
    .expect_err("nested input must respect the configured budget");

    assert_eq!(
        error.kind,
        ExpressionParseErrorKind::NestingBudgetExceeded
    );
}

#[test]
fn parses_a_collection_with_many_elements_without_a_language_level_cap() {
    // This is a test workload, not a language-level capacity limit.
    let element_count = 512usize;
    let mut tokens = Vec::with_capacity(element_count * 2 + 2);

    tokens.push(token(TokenKind::LeftBracket, "["));

    for index in 0..element_count {
        if index > 0 {
            tokens.push(token(TokenKind::Comma, ","));
        }

        // Static spellings avoid tying this test helper to a numeric
        // conversion policy. Each element is still a valid literal token.
        tokens.push(integer("1"));
    }

    tokens.push(token(TokenKind::RightBracket, "]"));
    tokens.push(eof());

    let expression = parsed(&tokens);

    match expression.kind {
        ExpressionSyntaxKind::Array { elements } => {
            assert_eq!(elements.len(), element_count);
        }
        other => panic!("expected array expression, found {other:?}"),
    }
}

#[test]
fn range_type_reports_empty_ranges_correctly() {
    let range = TokenRange::new(4, 4);

    assert!(range.is_empty());
    assert_eq!(range.len(), 0);
}

// ---------------------------------------------------------------------------
// Explicitly tracked conformance gaps
// ---------------------------------------------------------------------------

/// The normative lambda grammar supports forms such as `|x| x + 1`.
///
/// The current expression syntax enum and parser implementation do not yet
/// expose a lambda-expression node or lambda parser. Keep this test as an
/// explicit release gate, not as a passing conformance claim.
///
/// Remove `ignore` only after the parser and expression syntax representation
/// support the normative lambda forms and this test is updated to assert the
/// canonical resulting syntax tree.
#[test]
#[ignore = "release gate: implement lambda syntax and syntax representation first"]
fn conformance_lambda_expression() {
    let tokens = stream(vec![
        token(TokenKind::BitOr, "|"),
        ident("x"),
        token(TokenKind::BitOr, "|"),
        ident("x"),
        token(TokenKind::Plus, "+"),
        integer("1"),
    ]);

    let expression = parsed(&tokens);

    assert!(
        matches!(expression.kind, ExpressionSyntaxKind::Identifier { .. }),
        "replace this provisional assertion with the canonical lambda AST assertion"
    );
}

/// The normative indexing grammar includes slices and multidimensional index
/// lists. The current syntax representation models one index expression per
/// suffix and does not model slice bounds or slice steps.
///
/// Unignore after the expression syntax representation and parser support
/// the normative indexing forms without losing their structure.
#[test]
#[ignore = "release gate: implement slice and multidimensional indexing syntax"]
fn conformance_slice_expression() {
    // Conceptual source: values[start:end:step]
    //
    // The exact canonical token sequence and AST assertion must be added
    // when slice syntax is implemented. This test intentionally does not
    // pretend that a single Index node faithfully represents a slice.
}

/// The canonical precedence specification is the authority for the full
/// operator hierarchy. This parser currently implements only a subset of the
/// normative grammar and temporarily owns its own binding-power table.
///
/// Unignore the comprehensive conformance suite only after grammar parsing
/// consumes the shared precedence service and all specified operator forms
/// are represented in the expression syntax model.
#[test]
#[ignore = "release gate: integrate complete normative operator precedence table"]
fn conformance_complete_operator_precedence_table() {
    // Required follow-up coverage includes every operator registered by
    // precedence.rs, each supported fixity, associativity, ambiguity rules,
    // extension conflicts, and the canonical expression grammar.
}
