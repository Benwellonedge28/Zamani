#![forbid(unsafe_code)]

//! Production conformance tests for Zamani identifier recognition.
//!
//! File: src/frontend/lexer/tests/identifiers.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! Normative references:
//! - grammar/lexer/identifiers.g4
//! - grammar/spec/lexical.md
//! - grammar/Zamani.g4
//! - grammar/grammar.md
//!
//! Implementation under test:
//! - src/frontend/lexer/identifier.rs
//! - src/frontend/lexer/cursor.rs
//!
//! # Integration
//!
//! This module is included by src/frontend/lexer/tests/mod.rs:
//!
//!     mod identifiers;
//!
//! The tests access the production identifier recognizer and cursor
//! directly. They do not introduce a second implementation of the
//! identifier rules.
//!
//! # Architectural invariants
//!
//! - Identifier syntax is independent of computational domain.
//! - Unicode recognition follows the normative character categories.
//! - Identifier recognition does not classify keywords.
//! - Original source spelling is preserved.
//! - Byte ranges are half-open UTF-8 byte ranges.
//! - Failed recognition must not consume source.
//! - Successful recognition must make forward progress.
//! - No artificial language-level identifier length limit exists.
//! - Resource exhaustion is distinct from identifier syntax.
//! - All tests use safe Rust.
//!
//! These tests deliberately avoid depending on a particular processor,
//! register width, qubit count, hardware vendor, or execution target.

use super::super::cursor::{Cursor, CursorPosition};
use super::super::identifier::{
    is_identifier,
    is_identifier_continue,
    is_identifier_start,
    scan_identifier,
};

/// Assert that the supplied spelling is accepted as one complete
/// identifier by the production recognizer.
fn assert_valid_identifier(spelling: &str) {
    assert!(
        is_identifier(spelling),
        "expected a valid identifier, got {spelling:?}"
    );
}

/// Assert that the supplied spelling is not one complete identifier.
fn assert_invalid_identifier(spelling: &str) {
    assert!(
        !is_identifier(spelling),
        "expected an invalid identifier, got {spelling:?}"
    );
}

#[test]
fn accepts_ascii_identifier_forms() {
    for spelling in [
        "_",
        "__",
        "_value",
        "value",
        "Value",
        "VALUE",
        "value123",
        "a_b_c",
        "module_name",
        "type2",
        "identifier_with_many_parts",
    ] {
        assert_valid_identifier(spelling);
    }
}

#[test]
fn rejects_empty_and_digit_started_identifiers() {
    for spelling in [
        "",
        "0",
        "1",
        "123",
        "123value",
        "9_name",
        "0λ",
        "١name",
        "９name",
    ] {
        assert_invalid_identifier(spelling);
    }
}

#[test]
fn accepts_unicode_letter_categories_at_identifier_start() {
    // Latin letters.
    assert_valid_identifier("café");
    assert_valid_identifier("Éclair");

    // Greek and other scripts.
    assert_valid_identifier("λ");
    assert_valid_identifier("Ωmega");
    assert_valid_identifier("变量");
    assert_valid_identifier("नमस्ते");
    assert_valid_identifier("名前");
    assert_valid_identifier("Программа");
    assert_valid_identifier("مرحبا");

    // Unicode letter-number category (Nl).
    assert_valid_identifier("Ⅻname");
    assert_valid_identifier("Ⅳ");

    // The recognizer must not impose an ASCII-only policy.
    assert!(is_identifier_start('λ'));
    assert!(is_identifier_start('字'));
    assert!(is_identifier_start('Ⅻ'));
}

#[test]
fn accepts_underscore_as_identifier_start() {
    assert!(is_identifier_start('_'));
    assert_valid_identifier("_");
    assert_valid_identifier("__");
    assert_valid_identifier("_λ");
    assert_valid_identifier("_变量");
}

#[test]
fn accepts_unicode_decimal_digits_after_the_first_character() {
    for spelling in [
        "a0",
        "a9",
        "name١",
        "name२",
        "名前３",
        "value９",
    ] {
        assert_valid_identifier(spelling);
    }

    assert!(is_identifier_continue('0'));
    assert!(is_identifier_continue('١'));
    assert!(is_identifier_continue('२'));
    assert!(is_identifier_continue('９'));
}

#[test]
fn accepts_combining_marks_after_a_valid_start() {
    // U+0301 COMBINING ACUTE ACCENT: Mn.
    assert_valid_identifier("a\u{0301}");
    assert_valid_identifier("e\u{0301}clair");
    assert_valid_identifier("λ\u{0301}");

    // U+0903 DEVANAGARI SIGN VISARGA: Mc.
    assert_valid_identifier("a\u{0903}");

    assert!(is_identifier_continue('\u{0301}'));
    assert!(is_identifier_continue('\u{0903}'));
}

#[test]
fn rejects_combining_marks_as_the_first_character() {
    for ch in [
        '\u{0301}',
        '\u{0327}',
        '\u{0903}',
    ] {
        assert!(
            !is_identifier_start(ch),
            "combining mark must not start an identifier: {ch:?}"
        );
    }

    assert_invalid_identifier("\u{0301}a");
    assert_invalid_identifier("\u{0903}name");
}

#[test]
fn accepts_connector_punctuation_in_continuation_positions() {
    // Underscore is both a permitted start and continuation character.
    assert_valid_identifier("first_second");

    // U+203F UNDERTIE and U+2040 CHARACTER TIE.
    assert_valid_identifier("first\u{203F}second");
    assert_valid_identifier("first\u{2040}second");

    assert!(is_identifier_continue('\u{203F}'));
    assert!(is_identifier_continue('\u{2040}'));
}

#[test]
fn rejects_non_identifier_symbols_and_punctuation() {
    for spelling in [
        "-",
        ".",
        "!",
        "@",
        "#",
        "$",
        "%",
        "&",
        "*",
        "+",
        "/",
        ":",
        ";",
        "=",
        "?",
        "\\",
        "a-b",
        "a.b",
        "a/b",
        "a:b",
        "a@b",
        "a#b",
        "a$b",
        "a+b",
        "a=b",
        "a?b",
    ] {
        assert_invalid_identifier(spelling);
    }
}

#[test]
fn rejects_whitespace_in_identifiers() {
    for spelling in [
        " ",
        "\t",
        "\n",
        "\r",
        "a b",
        "a\tb",
        "a\nb",
        "a\rb",
        "a\u{00A0}b",
        "a\u{2003}b",
    ] {
        assert_invalid_identifier(spelling);
    }
}

#[test]
fn rejects_format_controls_and_joiners() {
    // Zero-width non-joiner and zero-width joiner.
    assert_invalid_identifier("a\u{200C}b");
    assert_invalid_identifier("a\u{200D}b");

    // Other format controls must not silently become identifier
    // continuation characters.
    assert_invalid_identifier("a\u{200E}b");
    assert_invalid_identifier("a\u{200F}b");
    assert_invalid_identifier("a\u{FEFF}b");
}

#[test]
fn preserves_original_spelling_without_normalization() {
    let composed = "é";
    let decomposed = "e\u{0301}";

    assert_valid_identifier(composed);
    assert_valid_identifier(decomposed);

    // Lexical recognition preserves spelling. It does not normalize
    // Unicode or silently rewrite source text.
    assert_ne!(composed, decomposed);

    let source = format!("{composed} {decomposed}");
    let mut cursor = Cursor::new(&source);

    let first = scan_identifier(&mut cursor).expect("composed identifier");
    assert_eq!(first.spelling(), composed);
    assert_eq!(&source[first.byte_range()], composed);

    assert_eq!(cursor.peek(), Some(' '));
    let _ = cursor.consume();

    let second = scan_identifier(&mut cursor).expect("decomposed identifier");
    assert_eq!(second.spelling(), decomposed);
    assert_eq!(&source[second.byte_range()], decomposed);
}

#[test]
fn preserves_identifier_case() {
    for spelling in ["value", "Value", "VALUE", "vAlUe"] {
        assert_valid_identifier(spelling);
    }

    let source = "value Value VALUE";
    let mut cursor = Cursor::new(source);

    for expected in ["value", "Value", "VALUE"] {
        let identifier =
            scan_identifier(&mut cursor).expect("expected identifier");

        assert_eq!(identifier.spelling(), expected);
        assert_eq!(&source[identifier.byte_range()], expected);

        if cursor.peek().is_some() {
            let _ = cursor.consume();
        }
    }
}

#[test]
fn failed_recognition_does_not_consume_input() {
    for source in [
        "",
        "123name",
        "١name",
        " name",
        "\u{0301}name",
        "-name",
        "\u{200C}name",
    ] {
        let mut cursor = Cursor::new(source);
        let before = cursor.position();
        let before_offset = cursor.byte_offset();
        let before_remaining = cursor.remaining();

        assert!(
            scan_identifier(&mut cursor).is_none(),
            "expected recognition failure for {source:?}"
        );

        assert_eq!(cursor.position(), before);
        assert_eq!(cursor.byte_offset(), before_offset);
        assert_eq!(cursor.remaining(), before_remaining);
    }
}

#[test]
fn scans_one_identifier_without_consuming_the_separator() {
    let source = "alpha beta";
    let mut cursor = Cursor::new(source);

    let first = scan_identifier(&mut cursor).expect("first identifier");

    assert_eq!(first.spelling(), "alpha");
    assert_eq!(first.byte_range(), 0..5);
    assert_eq!(cursor.peek(), Some(' '));
    assert_eq!(cursor.remaining(), " beta");

    let _ = cursor.consume();

    let second = scan_identifier(&mut cursor).expect("second identifier");

    assert_eq!(second.spelling(), "beta");
    assert_eq!(second.byte_range(), 6..10);
    assert_eq!(cursor.remaining(), "");
}

#[test]
fn scans_multibyte_unicode_with_correct_byte_ranges() {
    let source = "λ变量42!";
    let mut cursor = Cursor::new(source);

    let identifier =
        scan_identifier(&mut cursor).expect("Unicode identifier");

    assert_eq!(identifier.spelling(), "λ变量42");
    assert_eq!(identifier.byte_range(), 0..10);
    assert_eq!(&source[identifier.byte_range()], "λ变量42");

    // The recognizer stops before punctuation.
    assert_eq!(cursor.peek(), Some('!'));
    assert_eq!(cursor.remaining(), "!");
}

#[test]
fn reports_consistent_start_and_end_positions() {
    let source = "first λvalue";
    let mut cursor = Cursor::new(source);

    let first = scan_identifier(&mut cursor).expect("first identifier");

    assert_eq!(first.start(), CursorPosition::new(0, 1, 1));
    assert_eq!(first.end(), CursorPosition::new(5, 1, 6));

    let _ = cursor.consume();

    let second = scan_identifier(&mut cursor).expect("second identifier");

    // The second identifier starts after six ASCII bytes and a space.
    assert_eq!(second.start().byte_offset(), 6);
    assert_eq!(second.start().line(), 1);
    assert_eq!(second.start().column(), 7);

    // λ occupies two UTF-8 bytes but one Unicode scalar column.
    assert_eq!(second.end().byte_offset(), 13);
    assert_eq!(second.end().line(), 1);
    assert_eq!(second.end().column(), 13);
}

#[test]
fn recognizes_identifiers_across_source_lines() {
    let source = "first\nsecond";
    let mut cursor = Cursor::new(source);

    let first = scan_identifier(&mut cursor).expect("first identifier");
    assert_eq!(first.spelling(), "first");
    assert_eq!(first.start().line(), 1);
    assert_eq!(first.end().line(), 1);

    let _ = cursor.consume();

    let second = scan_identifier(&mut cursor).expect("second identifier");
    assert_eq!(second.spelling(), "second");
    assert_eq!(second.start().line(), 2);
    assert_eq!(second.start().column(), 1);
}

#[test]
fn identifier_recognition_is_independent_of_computing_domain() {
    // These are examples, not an exhaustive vocabulary. Adding a new
    // operation, device, vendor, target, or computational domain must
    // not require changes to identifier.rs.
    for spelling in [
        "H",
        "X",
        "Y",
        "Z",
        "CNOT",
        "custom_operation",
        "vendor_gate",
        "qpu_name",
        "gpu",
        "fpga",
        "asic",
        "tensor_compute",
        "hdl_module",
        "clock_signal",
        "quantum_register",
        "distributed_node",
        "future_accelerator",
        "application_specific_operation",
    ] {
        assert_valid_identifier(spelling);
    }
}

#[test]
fn keyword_spellings_remain_lexically_valid_identifiers() {
    // The identifier recognizer determines lexical shape only.
    // Whether a spelling becomes a keyword token is decided separately
    // by the keyword registry and scanner integration.
    for spelling in [
        "fn",
        "let",
        "module",
        "quantum",
        "classical",
        "hdl",
        "requires",
        "ensures",
        "infer",
        "deduce",
        "reason",
        "learn",
        "adapt",
        "query",
        "provenance",
        "policy",
    ] {
        assert_valid_identifier(spelling);
    }
}

#[test]
fn identifiers_can_contain_mixed_unicode_scripts() {
    for spelling in [
        "alpha变量",
        "λvalue",
        "名前value",
        "данныеValue",
        "مرحبا_name",
        "café变量",
    ] {
        assert_valid_identifier(spelling);
    }
}

#[test]
fn identifier_predicates_are_consistent() {
    // Every accepted complete identifier must begin with a valid start
    // character and contain only valid continuation characters.
    for spelling in [
        "_",
        "name",
        "name123",
        "λ",
        "变量",
        "Ⅻname",
        "a\u{0301}",
        "a\u{0903}",
        "a\u{203F}b",
    ] {
        assert_valid_identifier(spelling);

        let mut characters = spelling.chars();
        let first = characters.next().expect("nonempty valid identifier");

        assert!(is_identifier_start(first));

        assert!(
            characters.all(is_identifier_continue),
            "invalid continuation in {spelling:?}"
        );
    }
}

#[test]
fn scans_identifiers_repeatedly_until_non_identifier_input() {
    let source = "alpha βeta gamma42!";
    let mut cursor = Cursor::new(source);

    let expected = ["alpha", "βeta", "gamma42"];

    for spelling in expected {
        let identifier =
            scan_identifier(&mut cursor).expect("expected identifier");

        assert_eq!(identifier.spelling(), spelling);

        if cursor.peek() == Some(' ') {
            let _ = cursor.consume();
        }
    }

    assert_eq!(cursor.peek(), Some('!'));

    let before = cursor.position();

    assert!(scan_identifier(&mut cursor).is_none());
    assert_eq!(cursor.position(), before);
}

#[test]
fn handles_long_identifiers_without_a_language_level_length_cap() {
    // This is a scalability regression test, not a specification limit.
    // The test length may be increased as test resources permit.
    let spelling = "a".repeat(131_072);

    assert_valid_identifier(&spelling);

    let mut cursor = Cursor::new(&spelling);
    let identifier =
        scan_identifier(&mut cursor).expect("long identifier");

    assert_eq!(identifier.spelling(), spelling);
    assert_eq!(identifier.byte_range(), 0..spelling.len());
    assert_eq!(identifier.byte_len(), spelling.len());
    assert!(cursor.is_eof());
}

#[test]
fn handles_long_unicode_identifiers_with_utf8_boundaries() {
    // Repetition exercises multibyte boundaries and source slicing.
    let spelling = "λ变量".repeat(16_384);

    assert_valid_identifier(&spelling);

    let mut cursor = Cursor::new(&spelling);
    let identifier =
        scan_identifier(&mut cursor).expect("long Unicode identifier");

    assert_eq!(identifier.spelling(), spelling);
    assert_eq!(identifier.byte_range(), 0..spelling.len());
    assert_eq!(identifier.byte_len(), spelling.len());
    assert!(cursor.is_eof());
}

#[test]
fn empty_source_is_not_an_identifier_and_does_not_advance_cursor() {
    let mut cursor = Cursor::new("");

    assert!(!is_identifier(""));
    assert!(scan_identifier(&mut cursor).is_none());
    assert!(cursor.is_eof());
    assert_eq!(cursor.byte_offset(), 0);
}

#[test]
fn identifiers_are_deterministic_for_identical_source() {
    let source = "alpha λ变量42 e\u{0301} custom_operation";

    fn collect(source: &str) -> Vec<(String, usize, usize)> {
        let mut cursor = Cursor::new(source);
        let mut found = Vec::new();

        while !cursor.is_eof() {
            if let Some(identifier) = scan_identifier(&mut cursor) {
                let range = identifier.byte_range();

                found.push((
                    identifier.spelling().to_owned(),
                    range.start,
                    range.end,
                ));
            } else {
                let _ = cursor.consume();
            }
        }

        found
    }

    assert_eq!(collect(source), collect(source));
}