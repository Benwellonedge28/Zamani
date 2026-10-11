#![forbid(unsafe_code)]

//! Production conformance tests for Zamani string-literal scanning.
//!
//! File: src/frontend/lexer/tests/strings.rs
//! Rust edition: 2021
//! Rust baseline: 1.97 or later
//!
//! # Scope
//!
//! These tests validate the existing string scanner's public contract:
//! exact spelling preservation, supported escape syntax, Unicode source
//! handling, byte-based spans, error classification, cursor restoration,
//! configurable resource budgets, and deterministic behavior.
//!
//! They intentionally do not test raw strings, byte strings, interpolation,
//! triple-quoted strings, or multiline strings. Those are not part of the
//! current ordinary-string baseline.
//!
//! # Integration
//!
//! This module is intended to be declared from:
//!
//!     src/frontend/lexer/tests/mod.rs
//!
//! with:
//!
//!     mod strings;
//!
//! The tests exercise string.rs and its Cursor and ResourceLimit contracts.
//! They do not duplicate scanning logic or introduce a second string grammar.

use crate::frontend::lexer::config::ResourceLimit;
use crate::frontend::lexer::cursor::{Cursor, CursorPosition};
use crate::frontend::lexer::string::{
    scan_string,
    scan_string_with_limit,
    StringErrorKind,
    StringLiteral,
};

fn scan(source: &str) -> Result<Option<StringLiteral<'_>>, crate::frontend::lexer::string::StringError> {
    scan_string(&mut Cursor::new(source))
}

fn assert_valid_string(source: &str) -> StringLiteral<'_> {
    let literal = scan(source)
        .unwrap_or_else(|error| panic!("expected valid string {source:?}: {error}"))
        .unwrap_or_else(|| panic!("expected string scanner to recognize {source:?}"));

    assert_eq!(
        literal.spelling(),
        source,
        "scanner must preserve the complete original spelling"
    );
    assert_eq!(
        literal.byte_len(),
        source.len(),
        "literal length is measured in UTF-8 bytes"
    );
    assert_eq!(
        literal.byte_range(),
        0..source.len(),
        "the literal must cover the exact source spelling"
    );

    literal
}

fn assert_error_restores_cursor(
    source: &str,
    expected_kind: StringErrorKind,
) {
    let mut cursor = Cursor::new(source);
    let initial_position = cursor.position();
    let initial_remaining = cursor.remaining();

    let error = scan_string(&mut cursor)
        .expect_err("malformed string must produce a structured error");

    assert_eq!(
        error.kind(),
        expected_kind,
        "unexpected error kind for {source:?}"
    );
    assert_eq!(
        cursor.position(),
        initial_position,
        "failed scanning must restore the full cursor position"
    );
    assert_eq!(
        cursor.remaining(),
        initial_remaining,
        "failed scanning must leave all source text available for recovery"
    );
}

#[test]
fn recognizes_empty_and_ordinary_strings() {
    for source in [
        r#""""#,
        r#""hello""#,
        r#""Zamani""#,
        r#""classical computation""#,
        r#""quantum computation""#,
        r#""hardware description""#,
    ] {
        let literal = assert_valid_string(source);
        assert_eq!(literal.raw_content().len(), source.len() - 2);
    }

    let empty = assert_valid_string(r#""""#);
    assert!(empty.is_empty());
    assert_eq!(empty.raw_content(), "");
    assert_eq!(empty.escape_count(), 0);
}

#[test]
fn preserves_content_without_decoding_or_normalizing() {
    let cases = [
        (r#""plain text""#, "plain text"),
        (r#""line\nnext""#, r"line\nnext"),
        (r#""tab\tend""#, r"tab\tend"),
        (r#""quote: \""#, r#"quote: \"#),
        (r#""backslash: \\""#, r"backslash: \\"),
        (r#""unicode: \u03BB""#, r"unicode: \u03BB"),
        (r#""unicode: \u{03BB}""#, r"unicode: \u{03BB}"),
    ];

    for (source, expected_content) in cases {
        let literal = assert_valid_string(source);
        assert_eq!(
            literal.raw_content(),
            expected_content,
            "raw content must retain source escape spelling for {source:?}"
        );
    }
}

#[test]
fn accepts_every_documented_simple_escape() {
    let cases = [
        (r#""\'""#, r"\'"),
        (r#""\"""#, r#"\""#),
        (r#""\\""#, r"\\"),
        (r#""\b""#, r"\b"),
        (r#""\f""#, r"\f"),
        (r#""\n""#, r"\n"),
        (r#""\r""#, r"\r"),
        (r#""\t""#, r"\t"),
        (r#""\v""#, r"\v"),
        (r#""\0""#, r"\0"),
    ];

    for (source, expected_content) in cases {
        let literal = assert_valid_string(source);
        assert_eq!(literal.raw_content(), expected_content);
        assert_eq!(literal.escape_count(), 1);
    }
}

#[test]
fn accepts_fixed_width_unicode_escape_syntax() {
    for source in [
        r#""\u0000""#,
        r#""\u0041""#,
        r#""\u03BB""#,
        r#""\uFFFF""#,
        r#""prefix \u0041 suffix""#,
    ] {
        let literal = assert_valid_string(source);
        assert_eq!(literal.escape_count(), 1);
    }
}

#[test]
fn accepts_braced_unicode_escape_syntax() {
    for source in [
        r#""\u{0}""#,
        r#""\u{41}""#,
        r#""\u{03BB}""#,
        r#""\u{1F680}""#,
        r#""prefix \u{1F680} suffix""#,
    ] {
        let literal = assert_valid_string(source);
        assert_eq!(literal.escape_count(), 1);
    }
}

#[test]
fn counts_escape_sequences_not_escape_characters() {
    let source = r#""a\nb\u0041c\u{1F680}d\\""#;
    let literal = assert_valid_string(source);

    assert_eq!(literal.escape_count(), 4);
    assert_eq!(
        literal.raw_content(),
        r"a\nb\u0041c\u{1F680}d\\"
    );
}

#[test]
fn preserves_unicode_source_text_exactly() {
    let source = "\"λ 世界 🛰️ café e\u{301}\"";
    let literal = assert_valid_string(source);

    assert_eq!(literal.raw_content(), "λ 世界 🛰️ café e\u{301}");
    assert_eq!(literal.byte_len(), source.len());
    assert!(literal.byte_len() > literal.raw_content().chars().count());
}

#[test]
fn uses_utf8_byte_ranges_and_cursor_positions() {
    let source = "λ \"hello\"";
    let mut cursor = Cursor::new(source);

    assert_eq!(cursor.position(), CursorPosition::new(0, 1, 1));
    cursor.consume();
    cursor.consume();

    let literal = scan_string(&mut cursor)
        .expect("valid string")
        .expect("string should be recognized");

    assert_eq!(literal.spelling(), "\"hello\"");
    assert_eq!(literal.byte_range(), 3..10);
    assert_eq!(literal.start().byte_offset(), 3);
    assert_eq!(literal.end().byte_offset(), 10);
    assert_eq!(literal.start().line(), 1);
    assert_eq!(literal.end().line(), 1);
    assert_eq!(literal.start().column(), 3);
    assert_eq!(literal.end().column(), 10);
}

#[test]
fn reports_the_content_range_without_delimiters() {
    let source = r#""hello\nworld""#;
    let literal = assert_valid_string(source);
    let range = literal.content_range();

    assert_eq!(&source[range], r"hello\nworld");
    assert_eq!(literal.raw_content(), r"hello\nworld");
    assert_eq!(literal.byte_range(), 0..source.len());
}

#[test]
fn rejects_unknown_simple_escapes() {
    for source in [
        r#""\q""#,
        r#""\x41""#,
        r#""\z""#,
        r#""\a""#,
        r#""\e""#,
    ] {
        assert_error_restores_cursor(source, StringErrorKind::InvalidEscape);
    }
}

#[test]
fn rejects_incomplete_escape_at_end_of_input() {
    assert_error_restores_cursor(
        "\"unfinished\\",
        StringErrorKind::IncompleteEscape,
    );
}

#[test]
fn rejects_invalid_unicode_escape_syntax() {
    for source in [
        r#""\u""#,
        r#""\u1""#,
        r#""\u12""#,
        r#""\u123""#,
        r#""\u12G4""#,
        r#""\u{}""#,
        r#""\u{G}""#,
        r#""\u{12""#,
        r#""\u{12 G}""#,
        r#""\u{12-34}""#,
    ] {
        assert_error_restores_cursor(source, StringErrorKind::InvalidUnicodeEscape);
    }
}

#[test]
fn rejects_unterminated_strings() {
    for source in [
        "\"",
        "\"hello",
        "\"hello world",
        "\"hello\\n",
    ] {
        assert_error_restores_cursor(source, StringErrorKind::Unterminated);
    }
}

#[test]
fn rejects_raw_line_breaks_and_control_characters() {
    for source in [
        "\"line\nbreak\"",
        "\"line\rbreak\"",
        "\"tab\tcharacter\"",
        "\"nul\u{0000}character\"",
        "\"line\u{2028}separator\"",
        "\"paragraph\u{2029}separator\"",
    ] {
        assert_error_restores_cursor(source, StringErrorKind::InvalidRawCharacter);
    }
}

#[test]
fn escaped_controls_are_accepted_but_raw_controls_are_not() {
    for source in [
        r#""\n""#,
        r#""\r""#,
        r#""\t""#,
        r#""\0""#,
        r#""\u000A""#,
        r#""\u{000A}""#,
    ] {
        assert_valid_string(source);
    }
}

#[test]
fn does_not_consume_input_that_does_not_start_with_a_quote() {
    for source in [
        "",
        "name",
        "'character-like text'",
        "123",
        " ",
        "\t",
        "\\\"",
        "λ",
    ] {
        let mut cursor = Cursor::new(source);
        let initial = cursor.position();
        let remaining = cursor.remaining();

        let result = scan_string(&mut cursor)
            .expect("non-string input is not a string-scanning error");

        assert!(result.is_none(), "unexpected recognition for {source:?}");
        assert_eq!(cursor.position(), initial);
        assert_eq!(cursor.remaining(), remaining);
    }
}

#[test]
fn scans_one_string_and_leaves_following_source_untouched() {
    let source = "\"first\" next \"second\"";
    let mut cursor = Cursor::new(source);

    let first = scan_string(&mut cursor)
        .expect("first string is valid")
        .expect("first string should be recognized");

    assert_eq!(first.spelling(), "\"first\"");
    assert_eq!(cursor.remaining(), " next \"second\"");

    cursor.consume_while(char::is_whitespace);

    let next = scan_string(&mut cursor)
        .expect("second string is valid")
        .expect("second string should be recognized");

    assert_eq!(next.spelling(), "\"second\"");
    assert_eq!(cursor.remaining(), "");
    assert!(cursor.is_eof());
}

#[test]
fn byte_budget_includes_both_delimiters_and_is_inclusive() {
    let source = "\"abc\"";

    let mut too_small_cursor = Cursor::new(source);
    let too_small = ResourceLimit::at_most(source.len() - 1)
        .expect("the test budget is nonzero");

    let error = scan_string_with_limit(&mut too_small_cursor, too_small)
        .expect_err("literal must exceed a budget one byte below its length");

    assert_eq!(error.kind(), StringErrorKind::LiteralBudgetExceeded);
    assert!(error.is_resource_exhaustion());
    assert_eq!(too_small_cursor.byte_offset(), 0);

    let mut exact_cursor = Cursor::new(source);
    let exact = ResourceLimit::at_most(source.len())
        .expect("the test budget is nonzero");

    let literal = scan_string_with_limit(&mut exact_cursor, exact)
        .expect("a literal exactly at the budget is valid")
        .expect("expected string");

    assert_eq!(literal.spelling(), source);
    assert_eq!(exact_cursor.byte_offset(), source.len());
}

#[test]
fn budget_error_restores_cursor_after_multibyte_content() {
    let source = "\"λλ\"";
    let limit = ResourceLimit::at_most(3).expect("nonzero limit");
    let mut cursor = Cursor::new(source);
    let initial = cursor.position();

    let error = scan_string_with_limit(&mut cursor, limit)
        .expect_err("UTF-8 byte length must be used for the budget");

    assert_eq!(error.kind(), StringErrorKind::LiteralBudgetExceeded);
    assert!(error.is_resource_exhaustion());
    assert_eq!(cursor.position(), initial);
    assert_eq!(cursor.remaining(), source);
}

#[test]
fn unlimited_budget_does_not_add_a_fixed_string_length_ceiling() {
    let content = "λ🛰️".repeat(2048);
    let source = format!("\"{content}\"");
    let mut cursor = Cursor::new(&source);

    let literal = scan_string_with_limit(&mut cursor, ResourceLimit::Unlimited)
        .expect("string should be valid without an explicit budget")
        .expect("expected string");

    assert_eq!(literal.raw_content(), content);
    assert_eq!(literal.spelling(), source);
    assert_eq!(literal.byte_len(), source.len());
    assert_eq!(cursor.byte_offset(), source.len());
    assert!(cursor.is_eof());
}

#[test]
fn repeated_scans_are_deterministic() {
    let sources = [
        r#""""#,
        r#""Zamani""#,
        r#""λ 世界""#,
        r#""a\nb\u0041c\u{1F680}""#,
        r#""\q""#,
        "\"unterminated",
        "\"raw\nnewline\"",
    ];

    for source in sources {
        let first = scan(source);
        let second = scan(source);
        assert_eq!(first, second, "different results for {source:?}");
    }
}

#[test]
fn errors_have_consistent_ranges_and_observed_byte_counts() {
    let source = r#""bad\q""#;
    let mut cursor = Cursor::new(source);

    let error = scan_string(&mut cursor).expect_err("invalid escape");
    let range = error.byte_range();

    assert_eq!(range.start, 0);
    assert!(range.end <= source.len());
    assert!(source.is_char_boundary(range.start));
    assert!(source.is_char_boundary(range.end));
    assert_eq!(error.observed_bytes(), range.end - range.start);
    assert_eq!(error.start().byte_offset(), range.start);
    assert_eq!(error.end().byte_offset(), range.end);
    assert_eq!(cursor.byte_offset(), 0);
}

#[test]
fn display_messages_are_available_for_diagnostics() {
    for source in [
        "\"unterminated",
        r#""\q""#,
        r#""\u{}""#,
        "\"raw\nnewline\"",
    ] {
        let error = scan(source).expect_err("expected malformed string");
        assert!(
            !error.to_string().is_empty(),
            "diagnostic text must be available for {source:?}"
        );
    }
}