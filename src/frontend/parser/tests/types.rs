
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! Production conformance tests for Zamani source-level type parsing.
//!
//! This module tests the existing type grammar and canonical `TypeExpr`
//! representation. It intentionally does not define a second type AST,
//! lexer, semantic type system, or hardware-resource model.
//!
//! # Integration
//!
//! Register this file from `src/frontend/parser/tests/mod.rs`:
//
//! ```rust
//! mod types;
//! ```
//!
//! The parent test module must itself be registered by the parser module.
//! The parser's module layout must first resolve the existing `grammar.rs`
//! and `grammar/mod.rs` conflict.
//!
//! # Test boundary
//!
//! These tests exercise `grammar::types::parse_type` using a minimal
//! deterministic implementation of `TypeTokenCursor`. Production parsing
//! must use the shared parser cursor and canonical lexer output.
//!
//! The adapter here is test-only. It must not be copied into production
//! parsing code.
//!
//! # Resource policy
//!
//! Test input sizes are workload parameters, not language-level limits.
//! Nesting-budget tests verify explicit parser safety policy. They do not
//! define limits on hardware capacity, type arity, or program scale.

use crate::frontend::ast::node::types::type_expr::TypeExpr;
use crate::frontend::lexer::cursor::CursorPosition;
use crate::frontend::lexer::span::Span;
use crate::frontend::lexer::token_kind::TokenKind;
use crate::frontend::parser::grammar::types::{
    parse_type, TypeParseErrorKind, TypeParseOptions, TypeTokenCursor,
};

/// One test token, with a canonical kind, source spelling, and source span.
#[derive(Clone, Debug)]
struct TestToken {
    kind: TokenKind,
    lexeme: String,
    span: Span,
}

/// Minimal cursor for isolated grammar-production tests.
///
/// The cursor always has a valid EOF position, including for empty input.
/// It can split a right-shift token into two logical type closers, allowing
/// nested generic types to be tested independently of lexer tokenization
/// choices.
#[derive(Debug)]
struct TestCursor {
    tokens: Vec<TestToken>,
    index: usize,
    pending_type_closer: bool,
    split_closer_span: Option<Span>,
    eof_span: Span,
}

impl TestCursor {
    fn new(input: &[(TokenKind, &str)]) -> Self {
        let mut tokens = Vec::with_capacity(input.len() + 1);
        let mut byte_offset = 0usize;
        let mut column = 1usize;

        for (kind, lexeme) in input {
            let start = CursorPosition::new(byte_offset, 1, column);

            byte_offset = byte_offset.saturating_add(lexeme.len());
            column = column.saturating_add(lexeme.chars().count());

            let end = CursorPosition::new(byte_offset, 1, column);
            tokens.push(TestToken {
                kind: *kind,
                lexeme: (*lexeme).to_owned(),
                span: Span::new(start, end),
            });
        }

        let eof_position = CursorPosition::new(byte_offset, 1, column);
        let eof_span = Span::new(eof_position, eof_position);

        tokens.push(TestToken {
            kind: TokenKind::Eof,
            lexeme: String::new(),
            span: eof_span,
        });

        Self {
            tokens,
            index: 0,
            pending_type_closer: false,
            split_closer_span: None,
            eof_span,
        }
    }

    fn current(&self) -> &TestToken {
        if self.pending_type_closer {
            // The pending logical `>` belongs to the original `>>` token.
            // Its original span is retained because this adapter does not
            // manufacture byte-accurate subspans for individual characters.
            let index = self.index.saturating_sub(1);
            return self.tokens.get(index).unwrap_or_else(|| {
                self.tokens.last().expect("EOF token is always present")
            });
        }

        self.tokens.get(self.index).unwrap_or_else(|| {
            self.tokens.last().expect("EOF token is always present")
        })
    }

    fn current_is_pending_closer(&self) -> bool {
        self.pending_type_closer
    }

    fn position(&self) -> usize {
        self.index
    }

    fn is_at_eof(&self) -> bool {
        !self.pending_type_closer && self.current_kind() == TokenKind::Eof
    }
}

impl TypeTokenCursor for TestCursor {
    fn current_kind(&self) -> TokenKind {
        if self.pending_type_closer {
            return TokenKind::GreaterThan;
        }

        self.tokens
            .get(self.index)
            .map(|token| token.kind)
            .unwrap_or(TokenKind::Eof)
    }

    fn current_lexeme(&self) -> &str {
        if self.pending_type_closer {
            return ">";
        }

        self.tokens
            .get(self.index)
            .map(|token| token.lexeme.as_str())
            .unwrap_or("")
    }

    fn current_span(&self) -> Span {
        if self.pending_type_closer {
            return self.split_closer_span.unwrap_or(self.eof_span);
        }

        self.tokens
            .get(self.index)
            .map(|token| token.span)
            .unwrap_or(self.eof_span)
    }

    fn advance(&mut self) {
        if self.pending_type_closer {
            self.pending_type_closer = false;
            self.split_closer_span = None;
            return;
        }

        if self.index < self.tokens.len() {
            self.index += 1;
        }
    }

    fn consume_type_closer(&mut self) -> bool {
        if self.pending_type_closer {
            self.pending_type_closer = false;
            self.split_closer_span = None;
            return true;
        }

        match self.current_kind() {
            TokenKind::GreaterThan => {
                self.advance();
                true
            }
            TokenKind::RightShift => {
                self.split_closer_span = Some(self.current_span());
                self.index = self.index.saturating_add(1);
                self.pending_type_closer = true;
                true
            }
            _ => false,
        }
    }
}

fn token(kind: TokenKind, lexeme: &'static str) -> (TokenKind, &'static str) {
    (kind, lexeme)
}

fn parse(tokens: &[(TokenKind, &str)]) -> Result<TypeExpr, crate::frontend::parser::grammar::types::TypeParseError> {
    let mut cursor = TestCursor::new(tokens);
    let result = parse_type(&mut cursor, TypeParseOptions::default());

    if result.is_ok() {
        assert!(
            cursor.is_at_eof(),
            "test input must contain exactly one type expression; \
             parser left {:?} ({:?}) unconsumed",
            cursor.current_kind(),
            cursor.current_lexeme()
        );
    }

    result
}

fn named(name: &str) -> TypeExpr {
    TypeExpr::name(name)
}

fn assert_missing_type(result: Result<TypeExpr, crate::frontend::parser::grammar::types::TypeParseError>) {
    assert!(matches!(
        result,
        Err(error) if matches!(error.kind, TypeParseErrorKind::MissingType)
    ));
}

#[test]
fn parses_named_and_keyword_types() {
    for (kind, spelling) in [
        (TokenKind::Identifier, "Customer"),
        (TokenKind::KeywordInt, "int"),
        (TokenKind::KeywordFloat, "float"),
        (TokenKind::KeywordBool, "bool"),
        (TokenKind::KeywordStr, "str"),
        (TokenKind::KeywordQubit, "Qubit"),
    ] {
        let result = parse(&[token(kind, spelling)]);
        assert!(
            result.is_ok(),
            "failed to parse type {spelling:?}: {result:?}"
        );
    }

    assert_eq!(
        parse(&[token(TokenKind::Identifier, "Customer")]).unwrap(),
        named("Customer")
    );
}

#[test]
fn parses_qualified_type_names() {
    let result = parse(&[
        token(TokenKind::Identifier, "collections"),
        token(TokenKind::DoubleColon, "::"),
        token(TokenKind::Identifier, "maps"),
        token(TokenKind::DoubleColon, "::"),
        token(TokenKind::Identifier, "Map"),
    ]);

    let ty = result.expect("qualified type path should parse");
    assert_eq!(ty.name(), Some("Map"));
}

#[test]
fn parses_generic_types_and_preserves_argument_order() {
    let result = parse(&[
        token(TokenKind::Identifier, "Map"),
        token(TokenKind::LessThan, "<"),
        token(TokenKind::Identifier, "Text"),
        token(TokenKind::Comma, ","),
        token(TokenKind::Identifier, "Count"),
        token(TokenKind::GreaterThan, ">"),
    ])
    .expect("generic type should parse");

    match result {
        TypeExpr::Generic { base, arguments } => {
            assert_eq!(*base, named("Map"));
            assert_eq!(arguments, vec![named("Text"), named("Count")]);
        }
        other => panic!("expected generic type, got {other:?}"),
    }
}

#[test]
fn parses_nested_generics_with_separate_closers() {
    let result = parse(&[
        token(TokenKind::Identifier, "Map"),
        token(TokenKind::LessThan, "<"),
        token(TokenKind::Identifier, "Text"),
        token(TokenKind::Comma, ","),
        token(TokenKind::Identifier, "Vec"),
        token(TokenKind::LessThan, "<"),
        token(TokenKind::Identifier, "Int"),
        token(TokenKind::GreaterThan, ">"),
        token(TokenKind::GreaterThan, ">"),
    ])
    .expect("nested generic type should parse");

    match result {
        TypeExpr::Generic { arguments, .. } => {
            assert_eq!(arguments.len(), 2);
            assert!(matches!(
                arguments.get(1),
                Some(TypeExpr::Generic { .. })
            ));
        }
        other => panic!("expected outer generic type, got {other:?}"),
    }
}

#[test]
fn parses_nested_generics_when_closers_are_lexed_as_right_shift() {
    let result = parse(&[
        token(TokenKind::Identifier, "Outer"),
        token(TokenKind::LessThan, "<"),
        token(TokenKind::Identifier, "Inner"),
        token(TokenKind::LessThan, "<"),
        token(TokenKind::Identifier, "Value"),
        token(TokenKind::RightShift, ">>"),
    ]);

    assert!(
        result.is_ok(),
        "nested generic closer splitting failed: {result:?}"
    );
}

#[test]
fn rejects_empty_generic_arguments() {
    let result = parse(&[
        token(TokenKind::Identifier, "Container"),
        token(TokenKind::LessThan, "<"),
        token(TokenKind::GreaterThan, ">"),
    ]);

    assert!(matches!(
        result,
        Err(error) if matches!(
            error.kind,
            TypeParseErrorKind::EmptyGenericArguments
        )
    ));
}

#[test]
fn parses_parenthesized_and_tuple_types() {
    assert_eq!(
        parse(&[
            token(TokenKind::LeftParen, "("),
            token(TokenKind::Identifier, "Value"),
            token(TokenKind::RightParen, ")"),
        ])
        .unwrap(),
        named("Value")
    );

    let tuple = parse(&[
        token(TokenKind::LeftParen, "("),
        token(TokenKind::Identifier, "Value"),
        token(TokenKind::Comma, ","),
        token(TokenKind::KeywordBool, "bool"),
        token(TokenKind::RightParen, ")"),
    ])
    .expect("tuple type should parse");

    assert!(matches!(
        tuple,
        TypeExpr::Tuple(elements)
            if elements == vec![named("Value"), named("bool")]
    ));
}

#[test]
fn parses_unit_and_empty_parameter_function_types() {
    assert_eq!(
        parse(&[
            token(TokenKind::LeftParen, "("),
            token(TokenKind::RightParen, ")"),
        ])
        .unwrap(),
        TypeExpr::Unit
    );

    let function = parse(&[
        token(TokenKind::LeftParen, "("),
        token(TokenKind::RightParen, ")"),
        token(TokenKind::ThinArrow, "->"),
        token(TokenKind::KeywordInt, "int"),
    ])
    .expect("empty-parameter function type should parse");

    assert!(matches!(
        function,
        TypeExpr::Function { parameters, return_type }
            if parameters.is_empty() && *return_type == named("int")
    ));
}

#[test]
fn parses_function_types_with_multiple_parameters() {
    let function = parse(&[
        token(TokenKind::KeywordFn, "fn"),
        token(TokenKind::LeftParen, "("),
        token(TokenKind::KeywordInt, "int"),
        token(TokenKind::Comma, ","),
        token(TokenKind::KeywordBool, "bool"),
        token(TokenKind::RightParen, ")"),
        token(TokenKind::ThinArrow, "->"),
        token(TokenKind::Identifier, "Result"),
        token(TokenKind::LessThan, "<"),
        token(TokenKind::KeywordInt, "int"),
        token(TokenKind::Comma, ","),
        token(TokenKind::Identifier, "Error"),
        token(TokenKind::GreaterThan, ">"),
    ])
    .expect("function type should parse");

    assert!(matches!(
        function,
        TypeExpr::Function { parameters, return_type }
            if parameters == vec![named("int"), named("bool")]
                && matches!(*return_type, TypeExpr::Generic { .. })
    ));
}

#[test]
fn parses_array_and_slice_types_without_machine_sized_lengths() {
    let array = parse(&[
        token(TokenKind::LeftBracket, "["),
        token(TokenKind::Identifier, "Element"),
        token(TokenKind::Semicolon, ";"),
        token(TokenKind::Identifier, "capacity"),
        token(TokenKind::RightBracket, "]"),
    ])
    .expect("symbolically sized array should parse");

    assert!(matches!(
        array,
        TypeExpr::Array {
            element,
            length: Some(_)
        } if *element == named("Element")
    ));

    let slice = parse(&[
        token(TokenKind::LeftBracket, "["),
        token(TokenKind::Identifier, "Element"),
        token(TokenKind::RightBracket, "]"),
    ])
    .expect("slice type should parse");

    assert!(matches!(
        slice,
        TypeExpr::Slice(element) if *element == named("Element")
    ));
}

#[test]
fn parses_numeric_array_lengths_without_converting_them_to_usize() {
    let result = parse(&[
        token(TokenKind::LeftBracket, "["),
        token(TokenKind::Identifier, "Qubit"),
        token(TokenKind::Semicolon, ";"),
        token(TokenKind::IntegerLiteral, "999999999999999999999999999999999999"),
        token(TokenKind::RightBracket, "]"),
    ])
    .expect("large source-level array length should remain representable");

    assert!(matches!(
        result,
        TypeExpr::Array {
            element,
            length: Some(_)
        } if *element == named("Qubit")
    ));
}

#[test]
fn parses_reference_and_pointer_types() {
    let reference = parse(&[
        token(TokenKind::BitAnd, "&"),
        token(TokenKind::Identifier, "Value"),
    ])
    .expect("reference type should parse");

    assert!(matches!(
        reference,
        TypeExpr::Reference {
            mutable: false,
            lifetime: None,
            inner
        } if *inner == named("Value")
    ));

    let pointer = parse(&[
        token(TokenKind::Star, "*"),
        token(TokenKind::Identifier, "Value"),
    ])
    .expect("pointer type should parse");

    assert!(matches!(
        pointer,
        TypeExpr::Pointer {
            mutable: false,
            inner
        } if *inner == named("Value")
    ));
}

#[test]
fn parses_mutable_reference_and_pointer_types() {
    let reference = parse(&[
        token(TokenKind::BitAnd, "&"),
        token(TokenKind::KeywordMut, "mut"),
        token(TokenKind::Identifier, "Value"),
    ])
    .expect("mutable reference type should parse");

    assert!(matches!(
        reference,
        TypeExpr::Reference {
            mutable: true,
            inner,
            ..
        } if *inner == named("Value")
    ));

    let pointer = parse(&[
        token(TokenKind::Star, "*"),
        token(TokenKind::KeywordMut, "mut"),
        token(TokenKind::Identifier, "Value"),
    ])
    .expect("mutable pointer type should parse");

    assert!(matches!(
        pointer,
        TypeExpr::Pointer {
            mutable: true,
            inner
        } if *inner == named("Value")
    ));
}

#[test]
fn parses_optional_types_and_repeated_optional_suffixes() {
    let optional = parse(&[
        token(TokenKind::Identifier, "Value"),
        token(TokenKind::Question, "?"),
    ])
    .expect("optional type should parse");

    assert!(matches!(
        optional,
        TypeExpr::Optional(inner) if *inner == named("Value")
    ));

    let nested = parse(&[
        token(TokenKind::Identifier, "Value"),
        token(TokenKind::Question, "?"),
        token(TokenKind::Question, "?"),
    ])
    .expect("repeated optional suffixes should preserve nesting");

    assert!(matches!(
        nested,
        TypeExpr::Optional(inner)
            if matches!(*inner, TypeExpr::Optional(_))
    ));
}

#[test]
fn parses_union_types() {
    let union = parse(&[
        token(TokenKind::Identifier, "Text"),
        token(TokenKind::BitOr, "|"),
        token(TokenKind::Identifier, "Number"),
    ])
    .expect("union type should parse");

    assert!(matches!(
        union,
        TypeExpr::Union(members)
            if members == vec![named("Text"), named("Number")]
    ));
}

#[test]
fn reports_intersection_type_support_as_a_conformance_requirement() {
    // `&` is both a prefix reference marker and an intersection separator.
    // The grammar must distinguish these contexts rather than interpreting
    // every ampersand as a prefix reference. This test is deliberately
    // structured to detect that ambiguity in the current implementation.
    let intersection = parse(&[
        token(TokenKind::Identifier, "Readable"),
        token(TokenKind::BitAnd, "&"),
        token(TokenKind::Identifier, "Writable"),
    ]);

    assert!(
        matches!(
            intersection,
            Ok(TypeExpr::Intersection(ref members))
                if members == &vec![named("Readable"), named("Writable")]
        ),
        "intersection syntax must produce TypeExpr::Intersection; got {intersection:?}"
    );
}

#[test]
fn rejects_missing_type_input() {
    let empty: &[(TokenKind, &str)] = &[];
    assert_missing_type(parse(empty));
}

#[test]
fn rejects_incomplete_generic_type() {
    let result = parse(&[
        token(TokenKind::Identifier, "Container"),
        token(TokenKind::LessThan, "<"),
        token(TokenKind::Eof, ""),
    ]);

    assert!(
        result.is_err(),
        "an incomplete generic type must not be accepted"
    );
}

#[test]
fn rejects_missing_array_element_and_delimiters() {
    let missing_element = parse(&[
        token(TokenKind::LeftBracket, "["),
        token(TokenKind::RightBracket, "]"),
    ]);

    assert!(matches!(
        missing_element,
        Err(error) if matches!(
            error.kind,
            TypeParseErrorKind::MissingArrayElement
        )
    ));

    let missing_closer = parse(&[
        token(TokenKind::LeftBracket, "["),
        token(TokenKind::Identifier, "Value"),
    ]);

    assert!(missing_closer.is_err());
}

#[test]
fn rejects_incomplete_function_types() {
    let result = parse(&[
        token(TokenKind::KeywordFn, "fn"),
        token(TokenKind::LeftParen, "("),
        token(TokenKind::KeywordInt, "int"),
        token(TokenKind::RightParen, ")"),
        token(TokenKind::ThinArrow, "->"),
    ]);

    assert!(matches!(
        result,
        Err(error) if matches!(
            error.kind,
            TypeParseErrorKind::MissingFunctionReturnType
                | TypeParseErrorKind::MissingType
        )
    ));
}

#[test]
fn parser_does_not_consume_the_callers_following_separator() {
    let mut cursor = TestCursor::new(&[
        token(TokenKind::Identifier, "Value"),
        token(TokenKind::Comma, ","),
        token(TokenKind::Identifier, "Next"),
    ]);

    let result = parse_type(&mut cursor, TypeParseOptions::default())
        .expect("the first type should parse");

    assert_eq!(result, named("Value"));
    assert_eq!(cursor.current_kind(), TokenKind::Comma);
    assert_eq!(cursor.current_lexeme(), ",");
}

#[test]
fn reports_configured_nesting_budget_exhaustion() {
    let mut tokens = Vec::new();

    // The count is a test workload selected to exceed this test's explicit
    // budget; it is not a universal maximum type depth.
    for _ in 0..12 {
        tokens.push(token(TokenKind::LeftBracket, "["));
    }

    tokens.push(token(TokenKind::Identifier, "Value"));

    for _ in 0..12 {
        tokens.push(token(TokenKind::RightBracket, "]"));
    }

    let mut cursor = TestCursor::new(&tokens);
    let options = TypeParseOptions {
        max_nesting_depth: Some(4),
    };

    let result = parse_type(&mut cursor, options);

    assert!(matches!(
        result,
        Err(error) if matches!(
            error.kind,
            TypeParseErrorKind::NestingBudgetExceeded { maximum: 4 }
        )
    ));
}

#[test]
fn accepts_deep_types_when_the_explicit_budget_allows_them() {
    let nesting = 64;
    let mut tokens = Vec::with_capacity(nesting * 2 + 1);

    for _ in 0..nesting {
        tokens.push(token(TokenKind::LeftBracket, "["));
    }

    tokens.push(token(TokenKind::Identifier, "Value"));

    for _ in 0..nesting {
        tokens.push(token(TokenKind::RightBracket, "]"));
    }

    let mut cursor = TestCursor::new(&tokens);
    let options = TypeParseOptions {
        max_nesting_depth: Some(nesting.saturating_add(8)),
    };

    let result = parse_type(&mut cursor, options);

    assert!(
        result.is_ok(),
        "type within the configured budget should parse: {result:?}"
    );
    assert!(cursor.is_at_eof());
}

#[test]
fn repeated_parses_are_deterministic_and_isolated() {
    let input = [
        token(TokenKind::Identifier, "Map"),
        token(TokenKind::LessThan, "<"),
        token(TokenKind::Identifier, "Text"),
        token(TokenKind::Comma, ","),
        token(TokenKind::Identifier, "Vector"),
        token(TokenKind::LessThan, "<"),
        token(TokenKind::KeywordFloat, "float"),
        token(TokenKind::GreaterThan, ">"),
        token(TokenKind::GreaterThan, ">"),
    ];

    let first = parse(&input).expect("first parse should succeed");
    let second = parse(&input).expect("second parse should succeed");

    assert_eq!(first, second);
}

#[test]
fn source_spans_are_available_for_parse_errors() {
    let mut cursor = TestCursor::new(&[
        token(TokenKind::Identifier, "Container"),
        token(TokenKind::LessThan, "<"),
        token(TokenKind::Comma, ","),
    ]);

    let error = parse_type(&mut cursor, TypeParseOptions::default())
        .expect_err("missing generic argument should be diagnosed");

    // The canonical span type is carried by the structured error. Its exact
    // location is determined by the shared cursor in integrated parsing.
    assert!(error.span.start().byte_offset() <= error.span.end().byte_offset());
}

#[test]
fn unicode_type_names_remain_valid_source_identifiers() {
    let result = parse(&[token(TokenKind::Identifier, "Δομή")]);

    assert!(
        result.is_ok(),
        "the type grammar must preserve Unicode identifiers: {result:?}"
    );
}

#[test]
fn type_ast_structural_validation_accepts_parsed_composites() {
    let ty = parse(&[
        token(TokenKind::Identifier, "Result"),
        token(TokenKind::LessThan, "<"),
        token(TokenKind::LeftParen, "("),
        token(TokenKind::Identifier, "Input"),
        token(TokenKind::Comma, ","),
        token(TokenKind::Identifier, "Context"),
        token(TokenKind::RightParen, ")"),
        token(TokenKind::Comma, ","),
        token(TokenKind::Identifier, "Failure"),
        token(TokenKind::GreaterThan, ">"),
    ])
    .expect("composite generic type should parse");

    assert!(
        ty.validate().is_ok(),
        "parser output should satisfy default AST structural invariants"
    );
}
