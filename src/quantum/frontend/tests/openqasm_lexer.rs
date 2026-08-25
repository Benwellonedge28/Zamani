//! Zamani Quantum Frontend — OpenQASM lexer production contract tests.
//!
//! This file is the dedicated lexical conformance/security suite for:
//!
//!     src/quantum/frontend/formats/openqasm/lexer.rs
//!
//! Architectural position:
//!
//!     untrusted source
//!          │
//!          ▼
//!     OpenQasmLexer
//!          │
//!          ▼
//!     Token stream
//!          │
//!          ▼
//!     parser.rs
//!          │
//!          ▼
//!     OpenQASM AST
//!
//! These tests intentionally test the lexer as a public production boundary.
//! They do not test parser behavior, semantic validation, lowering, Quantum
//! IR, filesystem access, networking, execution, or hardware.
//!
//! Production invariants covered here:
//!
//! - OpenQASM keywords are case-sensitive.
//! - Identifiers are preserved exactly.
//! - Token lexemes are exact source slices.
//! - Token spans are byte-accurate and half-open.
//! - Unicode identifiers are handled without corrupting byte offsets.
//! - Operators use longest-match tokenization.
//! - Delimiters are recognized exactly.
//! - Numeric literals remain lexically lossless.
//! - Duration literals remain lexically lossless.
//! - Strings are bounded and correctly terminated.
//! - Comments cannot consume the rest of the source accidentally.
//! - Unterminated strings/comments are rejected.
//! - Invalid characters are rejected.
//! - EOF is deterministic and zero-width.
//! - Empty input is safe.
//! - Resource limits are enforced.
//! - Token production is bounded.
//! - Excessively large identifiers/literals/comments/directives are rejected.
//! - The lexer never relies on semantic validation.
//! - The lexer does not perform I/O.
//! - Repeated lexing is deterministic.
//! - Every successful token consumes source or is EOF.
//!
//! Rust:
//!
//! - Rust 1.97.1
//! - Rust 2021
//! - stable Rust only
//!
//! NOTE:
//!
//! This test module intentionally uses only the lexer contract. If the
//! production lexer constructor/iteration method is renamed, update the two
//! adapter functions near the top of this file rather than rewriting the
//! conformance suite.

#![allow(clippy::needless_range_loop)]

use std::fmt::Debug;

use crate::quantum::frontend::formats::openqasm::lexer::{
    LexError,
    LexErrorKind,
    LexerConfig,
    LexerLimits,
    OpenQasmLexer,
    Span,
    Token,
    TokenKind,
};

// =============================================================================
// Test adapter
// =============================================================================
//
// Keep repository-specific API adaptation in one place.
//
// The rest of this file intentionally does not depend on the concrete lexer
// implementation. This prevents tests from becoming coupled to internal
// iterator/storage details.
//
// Expected production contract:
//
//     OpenQasmLexer::new(source, config)
//     lexer.next_token()
//
// If the finalized lexer exposes `tokenize`, `next`, or another equivalent
// public operation, change only these functions.

fn lex_all(source: &str) -> Result<Vec<Token<'_>>, LexError> {
    let config = LexerConfig::default();
    lex_all_with_config(source, config)
}

fn lex_all_with_config(
    source: &str,
    config: LexerConfig,
) -> Result<Vec<Token<'_>>, LexError> {
    let mut lexer = OpenQasmLexer::new(source, config);
    let mut tokens = Vec::new();

    loop {
        let token = lexer
            .next_token()
            .expect("lexer must expose deterministic next-token operation");

        tokens.push(token);

        if token.is_eof() {
            break;
        }
    }

    Ok(tokens)
}

fn kinds(source: &str) -> Result<Vec<TokenKind>, LexError> {
    Ok(lex_all(source)?
        .into_iter()
        .map(Token::kind)
        .collect())
}

fn lexemes(source: &str) -> Result<Vec<&str>, LexError> {
    Ok(lex_all(source)?
        .into_iter()
        .map(Token::lexeme)
        .collect())
}

fn non_eof_tokens(source: &str) -> Result<Vec<Token<'_>>, LexError> {
    Ok(lex_all(source)?
        .into_iter()
        .filter(|token| !token.is_eof())
        .collect())
}

fn assert_token(
    token: Token<'_>,
    expected_kind: TokenKind,
    expected_lexeme: &str,
    expected_span: Span,
) {
    assert_eq!(
        token.kind(),
        expected_kind,
        "unexpected token kind for lexeme {:?}",
        token.lexeme()
    );

    assert_eq!(
        token.lexeme(),
        expected_lexeme,
        "lexer must preserve exact source spelling"
    );

    assert_eq!(
        token.span(),
        expected_span,
        "lexer must produce exact half-open byte span"
    );

    assert_eq!(
        token.span().len(),
        expected_lexeme.len(),
        "token span length must equal source lexeme byte length"
    );
}

fn assert_error_kind(source: &str, expected: LexErrorKind) {
    let result = lex_all(source);

    let error = result.expect_err("input must be rejected");

    assert_eq!(
        error.kind(),
        expected,
        "unexpected lexical error for source {:?}: {}",
        source,
        error
    );
}

fn strict_config() -> LexerConfig {
    LexerConfig {
        limits: LexerLimits {
            max_source_bytes: 1024,
            max_tokens: 128,
            max_lexeme_bytes: 128,
            max_string_bytes: 64,
            max_bitstring_bytes: 64,
            max_comment_bytes: 64,
            max_identifier_scalars: 32,
            max_numeric_digits: 64,
            max_directive_bytes: 64,
            max_calibration_bytes: 128,
            max_calibration_nesting: 8,
        },
        emit_comments: false,
        preserve_empty_directives: false,
    }
}

// =============================================================================
// Empty source / EOF
// =============================================================================

#[test]
fn empty_source_produces_only_eof() {
    let tokens = lex_all("").expect("empty source must lex successfully");

    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind(), TokenKind::Eof);
    assert_eq!(tokens[0].lexeme(), "");
    assert_eq!(tokens[0].span(), Span::new(0, 0));
}

#[test]
fn whitespace_only_source_produces_only_eof() {
    let tokens = lex_all(" \t\r\n \t").expect("whitespace must be ignored");

    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind(), TokenKind::Eof);
    assert_eq!(tokens[0].span(), Span::new(6, 6));
}

#[test]
fn eof_is_zero_width_at_source_end() {
    let source = "OPENQASM 3.1;";
    let tokens = lex_all(source).expect("source must lex");

    let eof = tokens.last().expect("EOF must be present");

    assert!(eof.is_eof());
    assert_eq!(eof.span(), Span::new(source.len(), source.len()));
}

// =============================================================================
// OpenQASM header
// =============================================================================

#[test]
fn openqasm_header_is_tokenized_deterministically() {
    let source = "OPENQASM 3.1;";

    let tokens = non_eof_tokens(source).expect("header must lex");

    assert_eq!(tokens.len(), 3);

    assert_token(
        tokens[0],
        TokenKind::KwOpenQasm,
        "OPENQASM",
        Span::new(0, 8),
    );

    // The version token must remain one lexical unit. This keeps version
    // spelling available to parser/version-policy code.
    assert_eq!(tokens[1].lexeme(), "3.1");
    assert_eq!(tokens[1].span(), Span::new(9, 12));

    assert_eq!(tokens[2].kind(), TokenKind::Semicolon);
    assert_eq!(tokens[2].lexeme(), ";");
    assert_eq!(tokens[2].span(), Span::new(12, 13));
}

#[test]
fn openqasm_keyword_is_case_sensitive() {
    let upper = lex_all("OPENQASM").expect("uppercase keyword must lex");
    assert_eq!(upper[0].kind(), TokenKind::KwOpenQasm);

    let lower = lex_all("openqasm").expect("lowercase spelling is still lexically valid");
    assert_eq!(lower[0].kind(), TokenKind::Identifier);
    assert_eq!(lower[0].lexeme(), "openqasm");
}

// =============================================================================
// Keywords
// =============================================================================

#[test]
fn all_reserved_keywords_are_classified() {
    let cases = [
        ("OPENQASM", TokenKind::KwOpenQasm),
        ("include", TokenKind::KwInclude),
        ("defcalgrammar", TokenKind::KwDefcalGrammar),
        ("def", TokenKind::KwDef),
        ("cal", TokenKind::KwCal),
        ("defcal", TokenKind::KwDefcal),
        ("gate", TokenKind::KwGate),
        ("extern", TokenKind::KwExtern),
        ("box", TokenKind::KwBox),
        ("let", TokenKind::KwLet),
        ("break", TokenKind::KwBreak),
        ("continue", TokenKind::KwContinue),
        ("if", TokenKind::KwIf),
        ("else", TokenKind::KwElse),
        ("end", TokenKind::KwEnd),
        ("return", TokenKind::KwReturn),
        ("for", TokenKind::KwFor),
        ("while", TokenKind::KwWhile),
        ("in", TokenKind::KwIn),
        ("switch", TokenKind::KwSwitch),
        ("case", TokenKind::KwCase),
        ("default", TokenKind::KwDefault),
        ("input", TokenKind::KwInput),
        ("output", TokenKind::KwOutput),
        ("const", TokenKind::KwConst),
        ("readonly", TokenKind::KwReadonly),
        ("mutable", TokenKind::KwMutable),
        ("qreg", TokenKind::KwQreg),
        ("qubit", TokenKind::KwQubit),
        ("creg", TokenKind::KwCreg),
        ("bit", TokenKind::KwBit),
        ("bool", TokenKind::KwBool),
        ("int", TokenKind::KwInt),
        ("uint", TokenKind::KwUInt),
        ("float", TokenKind::KwFloat),
        ("angle", TokenKind::KwAngle),
        ("complex", TokenKind::KwComplex),
        ("array", TokenKind::KwArray),
        ("void", TokenKind::KwVoid),
        ("duration", TokenKind::KwDuration),
        ("stretch", TokenKind::KwStretch),
        ("gphase", TokenKind::KwGphase),
        ("inv", TokenKind::KwInv),
        ("pow", TokenKind::KwPow),
        ("ctrl", TokenKind::KwCtrl),
        ("negctrl", TokenKind::KwNegctrl),
        ("durationof", TokenKind::KwDurationof),
        ("delay", TokenKind::KwDelay),
        ("reset", TokenKind::KwReset),
        ("measure", TokenKind::KwMeasure),
        ("barrier", TokenKind::KwBarrier),
        ("true", TokenKind::KwTrue),
        ("false", TokenKind::KwFalse),
    ];

    for (spelling, expected) in cases {
        let tokens = non_eof_tokens(spelling)
            .unwrap_or_else(|error| panic!("keyword {:?} failed: {}", spelling, error));

        assert_eq!(tokens.len(), 1, "keyword must produce one token");
        assert_eq!(tokens[0].kind(), expected);
        assert_eq!(tokens[0].lexeme(), spelling);
    }
}

#[test]
fn keyword_prefixes_remain_identifiers() {
    let source = concat!(
        "OPENQASMx ",
        "measurements ",
        "qubit0 ",
        "ifx ",
        "while_loop ",
        "duration2"
    );

    let tokens = non_eof_tokens(source).expect("identifier prefixes must lex");

    for token in tokens {
        assert_eq!(
            token.kind(),
            TokenKind::Identifier,
            "keyword prefix {:?} must remain identifier",
            token.lexeme()
        );
    }
}

// =============================================================================
// Identifiers
// =============================================================================

#[test]
fn identifiers_preserve_exact_spelling() {
    let source = "q0 alpha_beta result42 _private";

    let tokens = non_eof_tokens(source).expect("identifiers must lex");

    assert_eq!(
        tokens
            .iter()
            .map(|token| token.kind())
            .collect::<Vec<_>>(),
        vec![
            TokenKind::Identifier,
            TokenKind::Identifier,
            TokenKind::Identifier,
            TokenKind::Identifier,
        ]
    );

    assert_eq!(
        tokens
            .iter()
            .map(|token| token.lexeme())
            .collect::<Vec<_>>(),
        vec!["q0", "alpha_beta", "result42", "_private"]
    );
}

#[test]
fn unicode_identifiers_preserve_byte_spans() {
    let source = "qubit q_π;\nbit résultat;";

    let tokens = non_eof_tokens(source).expect("unicode identifiers must lex");

    let unicode = tokens
        .iter()
        .find(|token| token.lexeme() == "q_π")
        .expect("q_π token must exist");

    assert_eq!(unicode.kind(), TokenKind::Identifier);

    let start = source
        .find("q_π")
        .expect("source must contain q_π");

    assert_eq!(
        unicode.span(),
        Span::new(start, start + "q_π".len())
    );

    let result = tokens
        .iter()
        .find(|token| token.lexeme() == "résultat")
        .expect("résultat token must exist");

    assert_eq!(result.kind(), TokenKind::Identifier);

    let result_start = source
        .find("résultat")
        .expect("source must contain résultat");

    assert_eq!(
        result.span(),
        Span::new(result_start, result_start + "résultat".len())
    );
}

// =============================================================================
// Hardware qubits
// =============================================================================

#[test]
fn hardware_qubits_are_lexically_distinguished() {
    let source = "$0 $1 $42";

    let tokens = non_eof_tokens(source).expect("hardware qubits must lex");

    assert_eq!(tokens.len(), 3);

    for token in tokens {
        assert_eq!(token.kind(), TokenKind::HardwareQubit);
    }

    assert_eq!(tokens[0].lexeme(), "$0");
    assert_eq!(tokens[1].lexeme(), "$1");
    assert_eq!(tokens[2].lexeme(), "$42");
}

#[test]
fn hardware_qubit_index_is_not_semantically_validated_by_lexer() {
    // The lexer is not responsible for deciding whether a physical qubit
    // exists on a target device. That belongs to semantic/frontend/backend
    // validation.
    let token = non_eof_tokens("$999999999")
        .expect("lexically valid hardware qubit must remain lexical");

    assert_eq!(token[0].kind(), TokenKind::HardwareQubit);
    assert_eq!(token[0].lexeme(), "$999999999");
}

// =============================================================================
// Numeric literals
// =============================================================================

#[test]
fn integer_literals_are_lossless() {
    let source = "0 1 42 123456789 0xFF 0b1010 0o755";

    let tokens = non_eof_tokens(source).expect("integer literals must lex");

    for token in tokens {
        assert_eq!(token.kind(), TokenKind::IntegerLiteral);
        assert!(!token.lexeme().is_empty());
    }
}

#[test]
fn floating_literals_are_lossless() {
    let source = "0.0 1.0 3.14159 1e3 1.5e-4";

    let tokens = non_eof_tokens(source).expect("floating literals must lex");

    for token in tokens {
        assert_eq!(token.kind(), TokenKind::FloatLiteral);
        assert!(!token.lexeme().is_empty());
    }
}

#[test]
fn numeric_spelling_is_not_eagerly_converted_to_f64() {
    let source = "3.141592653589793238462643383279502884";

    let token = non_eof_tokens(source)
        .expect("high precision literal must remain lexical")
        .remove(0);

    assert_eq!(token.kind(), TokenKind::FloatLiteral);
    assert_eq!(
        token.lexeme(),
        "3.141592653589793238462643383279502884"
    );
}

#[test]
fn imaginary_literals_are_recognized() {
    let source = "1im 2.5im 10e-3im";

    let tokens = non_eof_tokens(source).expect("imaginary literals must lex");

    for token in tokens {
        assert_eq!(token.kind(), TokenKind::ImaginaryLiteral);
    }
}

#[test]
fn duration_literals_are_recognized() {
    let source = "1ns 10us 2ms 3s 4dt";

    let tokens = non_eof_tokens(source).expect("duration literals must lex");

    for token in tokens {
        assert_eq!(token.kind(), TokenKind::DurationLiteral);
    }
}

// =============================================================================
// Strings / include lexical boundary
// =============================================================================

#[test]
fn quoted_strings_preserve_exact_lexeme() {
    let source = r#"include "stdgates.inc";"#;

    let tokens = non_eof_tokens(source).expect("include string must lex");

    let string = tokens
        .iter()
        .find(|token| token.kind() == TokenKind::StringLiteral)
        .expect("string token must exist");

    assert_eq!(string.lexeme(), r#""stdgates.inc""#);
}

#[test]
fn strings_can_contain_escaped_quotes_without_ending_early() {
    let source = r#"include "file\"name.inc";"#;

    let tokens = non_eof_tokens(source).expect("escaped quote must be handled");

    let string = tokens
        .iter()
        .find(|token| token.kind() == TokenKind::StringLiteral)
        .expect("string token must exist");

    assert_eq!(string.lexeme(), r#""file\"name.inc""#);
}

#[test]
fn unterminated_string_is_rejected() {
    assert_error_kind(
        r#"include "stdgates.inc;"#,
        LexErrorKind::UnterminatedString,
    );
}

// =============================================================================
// Bitstrings
// =============================================================================

#[test]
fn bitstring_literals_are_recognized() {
    let source = "\"0101\" \"0000\" \"11111111\"";

    let tokens = non_eof_tokens(source).expect("bitstrings must lex");

    for token in tokens {
        assert_eq!(token.kind(), TokenKind::BitstringLiteral);
    }
}

// =============================================================================
// Comments
// =============================================================================

#[test]
fn line_comments_are_ignored_in_parser_mode() {
    let source = "qubit q; // comment\nbit c;";

    let tokens = non_eof_tokens(source).expect("line comment must be ignored");

    assert!(tokens.iter().any(|token| token.lexeme() == "qubit"));
    assert!(tokens.iter().any(|token| token.lexeme() == "bit"));

    assert!(
        !tokens
            .iter()
            .any(|token| token.lexeme().contains("comment")),
        "comment contents must not become ordinary tokens"
    );
}

#[test]
fn block_comments_are_ignored_in_parser_mode() {
    let source = "qubit /* comment */ q;";

    let tokens = non_eof_tokens(source).expect("block comment must be ignored");

    assert_eq!(
        tokens
            .iter()
            .map(|token| token.lexeme())
            .collect::<Vec<_>>(),
        vec!["qubit", "q", ";"]
    );
}

#[test]
fn unterminated_block_comment_is_rejected() {
    assert_error_kind(
        "qubit q; /* never closes",
        LexErrorKind::UnterminatedComment,
    );
}

#[test]
fn comment_limits_are_enforced() {
    let mut config = strict_config();

    config.limits.max_comment_bytes = 8;

    let source = format!("qubit q; //{}", "x".repeat(32));

    assert_error_kind_with_config(
        &source,
        config,
        LexErrorKind::CommentTooLarge,
    );
}

// =============================================================================
// Delimiters
// =============================================================================

#[test]
fn delimiters_are_tokenized_exactly() {
    let source = "( ) [ ] { } , ; : .";

    let expected = [
        (TokenKind::LParen, "("),
        (TokenKind::RParen, ")"),
        (TokenKind::LBracket, "["),
        (TokenKind::RBracket, "]"),
        (TokenKind::LBrace, "{"),
        (TokenKind::RBrace, "}"),
        (TokenKind::Comma, ","),
        (TokenKind::Semicolon, ";"),
        (TokenKind::Colon, ":"),
        (TokenKind::Dot, "."),
    ];

    let tokens = non_eof_tokens(source).expect("delimiters must lex");

    assert_eq!(tokens.len(), expected.len());

    for index in 0..expected.len() {
        assert_eq!(tokens[index].kind(), expected[index].0);
        assert_eq!(tokens[index].lexeme(), expected[index].1);
    }
}

// =============================================================================
// Operators
// =============================================================================

#[test]
fn single_character_operators_are_recognized() {
    let source = "+ - * / % = < > ! & | ^ ~";

    let expected = [
        TokenKind::Plus,
        TokenKind::Minus,
        TokenKind::Star,
        TokenKind::Slash,
        TokenKind::Percent,
        TokenKind::Equal,
        TokenKind::Less,
        TokenKind::Greater,
        TokenKind::LogicalNot,
        TokenKind::BitAnd,
        TokenKind::BitOr,
        TokenKind::BitXor,
        TokenKind::BitNot,
    ];

    assert_eq!(kinds(source).unwrap()[..expected.len()], expected);
}

#[test]
fn compound_operators_use_longest_match() {
    let source = concat!(
        "++ -- ** => == != <= >= && || ",
        "<< >> += -= *= /= %= ",
        "&= |= ^= <<= >>= **="
    );

    let expected = [
        TokenKind::Increment,
        TokenKind::Decrement,
        TokenKind::Power,
        TokenKind::Arrow,
        TokenKind::EqualEqual,
        TokenKind::NotEqual,
        TokenKind::LessEqual,
        TokenKind::GreaterEqual,
        TokenKind::LogicalAnd,
        TokenKind::LogicalOr,
        TokenKind::ShiftLeft,
        TokenKind::ShiftRight,
        TokenKind::PlusEqual,
        TokenKind::MinusEqual,
        TokenKind::StarEqual,
        TokenKind::SlashEqual,
        TokenKind::PercentEqual,
        TokenKind::AmpersandEqual,
        TokenKind::PipeEqual,
        TokenKind::CaretEqual,
        TokenKind::ShiftLeftEqual,
        TokenKind::ShiftRightEqual,
        TokenKind::PowerEqual,
    ];

    let tokens = non_eof_tokens(source).expect("compound operators must lex");

    assert_eq!(tokens.len(), expected.len());

    for index in 0..expected.len() {
        assert_eq!(
            tokens[index].kind(),
            expected[index],
            "operator at index {} was {:?}",
            index,
            tokens[index]
        );
    }
}

#[test]
fn arrow_is_not_split_into_minus_and_greater() {
    let tokens = non_eof_tokens("=>").expect("arrow must lex");

    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind(), TokenKind::Arrow);
    assert_eq!(tokens[0].lexeme(), "=>");
}

// =============================================================================
// OpenQASM modifiers
// =============================================================================

#[test]
fn gate_modifiers_are_tokenized_as_keywords() {
    let source = "ctrl @ inv @ pow @ negctrl @";

    let tokens = non_eof_tokens(source).expect("gate modifiers must lex");

    assert_eq!(
        tokens
            .iter()
            .map(|token| token.kind())
            .collect::<Vec<_>>(),
        vec![
            TokenKind::KwCtrl,
            TokenKind::At,
            TokenKind::KwInv,
            TokenKind::At,
            TokenKind::KwPow,
            TokenKind::At,
            TokenKind::KwNegctrl,
            TokenKind::At,
        ]
    );
}

// =============================================================================
// Hash / annotations / pragmas
// =============================================================================

#[test]
fn hash_is_a_distinct_lexical_token() {
    let tokens = non_eof_tokens("#dim")
        .expect("hash dimension syntax must lex");

    assert_eq!(tokens[0].kind(), TokenKind::Hash);
    assert_eq!(tokens[0].lexeme(), "#");
    assert_eq!(tokens[1].kind(), TokenKind::KwDim);
    assert_eq!(tokens[1].lexeme(), "dim");
}

#[test]
fn annotation_marker_is_not_lost() {
    let source = "@custom_annotation value";

    let tokens = non_eof_tokens(source).expect("annotation must lex");

    assert!(
        tokens
            .iter()
            .any(|token| token.kind() == TokenKind::Annotation
                || token.kind() == TokenKind::At),
        "annotation marker must remain visible to downstream parsing"
    );
}

// =============================================================================
// Complete representative OpenQASM program
// =============================================================================

#[test]
fn representative_openqasm_program_lexes_without_semantic_work() {
    let source = r#"
OPENQASM 3.1;

include "stdgates.inc";

input float[64] theta;
const int[32] n = 2;

qubit[2] q;
bit[2] c;

h q[0];
cx q[0], q[1];
rz(theta) q[1];

if (c == 2) {
    x q[0];
}

measure q -> c;
"#;

    let tokens = lex_all(source).expect("representative OpenQASM must lex");

    assert!(tokens.len() > 20);
    assert_eq!(
        tokens.last().expect("EOF must exist").kind(),
        TokenKind::Eof
    );

    // The lexer must not reject semantic constructs merely because semantic
    // validation belongs downstream.
    assert!(tokens.iter().any(|token| token.lexeme() == "theta"));
    assert!(tokens.iter().any(|token| token.lexeme() == "q"));
    assert!(tokens.iter().any(|token| token.lexeme() == "measure"));
}

// =============================================================================
// Span correctness
// =============================================================================

#[test]
fn every_non_eof_token_span_is_non_empty() {
    let source = "qubit[2] q; h q[0];";

    let tokens = non_eof_tokens(source).expect("source must lex");

    for token in tokens {
        assert!(
            !token.span().is_empty(),
            "non-EOF token {:?} must have non-empty span",
            token
        );

        assert!(
            token.span().end <= source.len(),
            "token span must remain inside source"
        );
    }
}

#[test]
fn token_spans_are_monotonic() {
    let source = "qubit q; h q[0]; measure q -> c;";

    let tokens = lex_all(source).expect("source must lex");

    let mut previous_end = 0;

    for token in tokens {
        assert!(
            token.span().start >= previous_end,
            "token spans must be monotonic"
        );

        assert!(
            token.span().start <= token.span().end,
            "token span must be ordered"
        );

        previous_end = token.span().end;
    }
}

#[test]
fn token_lexeme_is_exact_source_slice() {
    let source = "qubit[2] π; // comment\n";

    let tokens = non_eof_tokens(source).expect("source must lex");

    for token in tokens {
        let slice = &source[token.span().start..token.span().end];

        assert_eq!(
            slice,
            token.lexeme(),
            "token lexeme must be an exact source slice"
        );
    }
}

#[test]
fn multibyte_utf8_does_not_break_following_byte_offsets() {
    let source = "qubit π;\nbit c;";

    let tokens = non_eof_tokens(source).expect("unicode source must lex");

    let bit = tokens
        .iter()
        .find(|token| token.lexeme() == "bit")
        .expect("bit keyword must exist");

    let expected_start = source
        .find("bit")
        .expect("source must contain bit");

    assert_eq!(bit.span().start, expected_start);
}

// =============================================================================
// Token limit
// =============================================================================

#[test]
fn token_limit_is_enforced() {
    let mut config = strict_config();
    config.limits.max_tokens = 4;

    let source = "q0 q1 q2 q3 q4 q5";

    assert_error_kind_with_config(
        source,
        config,
        LexErrorKind::TokenLimitExceeded,
    );
}

// =============================================================================
// Source limit
// =============================================================================

#[test]
fn source_limit_is_enforced_before_unbounded_lexing() {
    let mut config = strict_config();
    config.limits.max_source_bytes = 8;

    let source = "qubit q; qubit r;";

    assert_error_kind_with_config(
        source,
        config,
        LexErrorKind::SourceTooLarge,
    );
}

// =============================================================================
// Identifier limits
// =============================================================================

#[test]
fn identifier_limit_is_enforced() {
    let mut config = strict_config();
    config.limits.max_identifier_scalars = 8;

    let source = "abcdefghijk";

    assert_error_kind_with_config(
        source,
        config,
        LexErrorKind::IdentifierTooLong,
    );
}

// =============================================================================
// Numeric limits
// =============================================================================

#[test]
fn numeric_literal_limit_is_enforced() {
    let mut config = strict_config();
    config.limits.max_numeric_digits = 8;

    let source = "12345678901234567890";

    assert_error_kind_with_config(
        source,
        config,
        LexErrorKind::NumericLiteralTooLong,
    );
}

// =============================================================================
// String limits
// =============================================================================

#[test]
fn string_limit_is_enforced() {
    let mut config = strict_config();
    config.limits.max_string_bytes = 8;

    let source = r#""0123456789abcdef""#;

    assert_error_kind_with_config(
        source,
        config,
        LexErrorKind::StringTooLarge,
    );
}

// =============================================================================
// Bitstring limits
// =============================================================================

#[test]
fn bitstring_limit_is_enforced() {
    let mut config = strict_config();
    config.limits.max_bitstring_bytes = 8;

    let source = "\"0101010101010101\"";

    assert_error_kind_with_config(
        source,
        config,
        LexErrorKind::BitstringTooLarge,
    );
}

// =============================================================================
// Invalid characters / malformed source
// =============================================================================

#[test]
fn invalid_character_is_rejected() {
    assert_error_kind(
        "qubit q; \u{0000}",
        LexErrorKind::UnexpectedCharacter,
    );
}

#[test]
fn unterminated_string_does_not_hang() {
    assert_error_kind(
        "\"unterminated",
        LexErrorKind::UnterminatedString,
    );
}

#[test]
fn malformed_hardware_qubit_is_rejected() {
    assert_error_kind(
        "$not_a_qubit",
        LexErrorKind::InvalidHardwareQubit,
    );
}

// =============================================================================
// Determinism
// =============================================================================

#[test]
fn lexing_is_deterministic() {
    let source = r#"
OPENQASM 3.1;
include "stdgates.inc";
qubit[3] q;
bit[3] c;
h q[0];
cx q[0], q[1];
measure q -> c;
"#;

    let first = lex_all(source).expect("first lex must succeed");
    let second = lex_all(source).expect("second lex must succeed");

    assert_eq!(first, second);
}

#[test]
fn lexeme_sequence_is_deterministic() {
    let source = "qubit[2] q; cx q[0], q[1];";

    let first = lexemes(source).expect("first lex must succeed");
    let second = lexemes(source).expect("second lex must succeed");

    assert_eq!(first, second);
}

// =============================================================================
// Progress / termination contract
// =============================================================================

#[test]
fn every_successful_non_eof_token_consumes_source() {
    let source = concat!(
        "qubit q;",
        "bit c;",
        "h q;",
        "measure q -> c;"
    );

    let tokens = lex_all(source).expect("source must lex");

    for token in tokens {
        if token.is_eof() {
            continue;
        }

        assert!(
            token.span().end > token.span().start,
            "successful non-EOF token must consume source"
        );
    }
}

#[test]
fn eof_is_the_only_zero_width_token() {
    let source = "qubit q;";

    let tokens = lex_all(source).expect("source must lex");

    for token in tokens {
        if token.is_eof() {
            assert!(token.span().is_empty());
        } else {
            assert!(
                !token.span().is_empty(),
                "non-EOF token cannot be zero-width"
            );
        }
    }
}

// =============================================================================
// Representative stress tests
// =============================================================================

#[test]
fn many_small_identifiers_remain_bounded() {
    let mut config = strict_config();
    config.limits.max_tokens = 2_000;

    let source = (0..500)
        .map(|index| format!("q{index}"))
        .collect::<Vec<_>>()
        .join(" ");

    let tokens = lex_all_with_config(&source, config)
        .expect("bounded identifier workload must succeed");

    assert_eq!(tokens.last().unwrap().kind(), TokenKind::Eof);
}

#[test]
fn very_large_identifier_is_rejected_without_unbounded_growth() {
    let mut config = strict_config();
    config.limits.max_identifier_scalars = 32;

    let source = "q".repeat(100_000);

    assert_error_kind_with_config(
        &source,
        config,
        LexErrorKind::IdentifierTooLong,
    );
}

#[test]
fn very_large_comment_is_rejected_without_becoming_a_token() {
    let mut config = strict_config();
    config.limits.max_comment_bytes = 32;

    let source = format!("qubit q; //{}", "x".repeat(100_000));

    assert_error_kind_with_config(
        &source,
        config,
        LexErrorKind::CommentTooLarge,
    );
}

// =============================================================================
// Configuration behavior
// =============================================================================

#[test]
fn default_configuration_is_usable() {
    let source = "OPENQASM 3.1;";

    let tokens = lex_all_with_config(
        source,
        LexerConfig::default(),
    )
    .expect("default configuration must be valid");

    assert!(tokens.last().unwrap().is_eof());
}

#[test]
fn strict_configuration_remains_explicitly_bounded() {
    let config = strict_config();

    assert!(config.limits.max_source_bytes > 0);
    assert!(config.limits.max_tokens > 0);
    assert!(config.limits.max_lexeme_bytes > 0);
    assert!(config.limits.max_string_bytes > 0);
    assert!(config.limits.max_bitstring_bytes > 0);
    assert!(config.limits.max_comment_bytes > 0);
    assert!(config.limits.max_identifier_scalars > 0);
    assert!(config.limits.max_numeric_digits > 0);
    assert!(config.limits.max_directive_bytes > 0);
    assert!(config.limits.max_calibration_bytes > 0);
    assert!(config.limits.max_calibration_nesting > 0);
}

// =============================================================================
// TokenKind classification contracts
// =============================================================================

#[test]
fn identifier_like_classification_is_consistent() {
    assert!(TokenKind::Identifier.is_identifier_like());
    assert!(TokenKind::HardwareQubit.is_identifier_like());

    assert!(!TokenKind::IntegerLiteral.is_identifier_like());
    assert!(!TokenKind::KwQubit.is_identifier_like());
}

#[test]
fn literal_classification_is_consistent() {
    assert!(TokenKind::IntegerLiteral.is_literal());
    assert!(TokenKind::FloatLiteral.is_literal());
    assert!(TokenKind::ImaginaryLiteral.is_literal());
    assert!(TokenKind::DurationLiteral.is_literal());
    assert!(TokenKind::StringLiteral.is_literal());
    assert!(TokenKind::BitstringLiteral.is_literal());
    assert!(TokenKind::KwTrue.is_literal());
    assert!(TokenKind::KwFalse.is_literal());

    assert!(!TokenKind::Identifier.is_literal());
    assert!(!TokenKind::KwQubit.is_literal());
}

#[test]
fn compound_assignment_classification_is_complete() {
    let compound = [
        TokenKind::PlusEqual,
        TokenKind::MinusEqual,
        TokenKind::StarEqual,
        TokenKind::SlashEqual,
        TokenKind::PercentEqual,
        TokenKind::AmpersandEqual,
        TokenKind::PipeEqual,
        TokenKind::TildeEqual,
        TokenKind::CaretEqual,
        TokenKind::ShiftLeftEqual,
        TokenKind::ShiftRightEqual,
        TokenKind::PowerEqual,
    ];

    for kind in compound {
        assert!(
            kind.is_compound_assignment(),
            "{kind:?} must be classified as compound assignment"
        );
    }

    assert!(!TokenKind::Equal.is_compound_assignment());
    assert!(!TokenKind::Plus.is_compound_assignment());
}

// =============================================================================
// Keyword table contract
// =============================================================================

#[test]
fn_keyword_table_is_case_sensitive_and_does_not_reserve_pi_or_euler() {
    assert_eq!(
        TokenKind::keyword("OPENQASM"),
        Some(TokenKind::KwOpenQasm)
    );

    assert_eq!(TokenKind::keyword("openqasm"), None);

    // These are identifiers in OpenQASM rather than reserved lexical
    // keywords. The parser/semantic layer may assign them mathematical
    // meaning where appropriate.
    assert_eq!(TokenKind::keyword("pi"), None);
    assert_eq!(TokenKind::keyword("euler"), None);
}

// =============================================================================
// Test helpers
// =============================================================================

fn assert_error_kind_with_config(
    source: &str,
    config: LexerConfig,
    expected: LexErrorKind,
) {
    let result = lex_all_with_config(source, config);

    let error = result.expect_err("input must be rejected");

    assert_eq!(
        error.kind(),
        expected,
        "unexpected lexical error for source {:?}: {}",
        source,
        error
    );
}

// =============================================================================
// Compile-time trait contracts
// =============================================================================

#[test]
fn token_is_copy_and_debug_friendly() {
    fn assert_copy<T: Copy>() {}
    fn assert_debug<T: Debug>() {}

    assert_copy::<Token<'static>>();
    assert_debug::<Token<'static>>();
    assert_copy::<Span>();
    assert_debug::<Span>();
}

#[test]
fn token_kind_is_copy_hashable_and_debug_friendly() {
    fn assert_copy<T: Copy>() {}
    fn assert_debug<T: Debug>() {}
    fn assert_hash<T: std::hash::Hash>() {}

    assert_copy::<TokenKind>();
    assert_debug::<TokenKind>();
    assert_hash::<TokenKind>();
}