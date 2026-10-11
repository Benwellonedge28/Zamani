#![forbid(unsafe_code)]

//! Unicode conformance tests for the Zamani frontend lexer.
//!
//! File: src/frontend/lexer/tests/unicode.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! # Purpose
//!
//! Verify that the production cursor and identifier recognizer correctly
//! handle Unicode source text without corrupting UTF-8 boundaries, changing
//! source spelling, introducing artificial language limits, or depending
//! on a particular computing domain.
//!
//! # Normative references
//!
//! - grammar/lexer/identifiers.g4
//! - grammar/spec/lexical.md
//! - grammar/Zamani.g4
//! - grammar/Zamani-Grammar.md
//!
//! The normative specification and canonical grammar determine the intended
//! language contract. The extended grammar reference supplies design context
//! but does not independently promote proposed syntax into legal syntax.
//!
//! # Production implementation under test
//!
//! - src/frontend/lexer/cursor.rs
//! - src/frontend/lexer/identifier.rs
//! - src/frontend/lexer/span.rs, when integrated with source spans
//! - src/frontend/lexer/scanner.rs, through scanner-level conformance tests
//!
//! This module tests the canonical cursor and identifier recognizer directly.
//! It does not reimplement Unicode classification, tokenization, or scanning.
//!
//! # Registration
//!
//! Register this module in src/frontend/lexer/tests/mod.rs:
//!
//!     mod unicode;
//!
//! The parent test module should be registered through the crate's existing
//! test configuration. Do not add a second lexer implementation for tests.
//!
//! # Unicode policy
//!
//! The tests assume the identifier categories specified by identifiers.g4:
//!
//! Start:
//! - underscore;
//! - Unicode general category L;
//! - Unicode general category Nl.
//!
//! Continuation:
//! - every valid start character;
//! - Mn, Mc, Nd, and Pc.
//!
//! Identifier recognition preserves original spelling. It does not silently
//! normalize, case-fold, transliterate, or remove characters.
//!
//! Unicode normalization and confusable-identifier security analysis are
//! separate concerns; neither should silently change lexical spelling.
//!
//! # Scalability and safety
//!
//! No hardware-specific counts, fixed identifier-length limits, fixed
//! Unicode table sizes in this test module, or unsafe Rust are introduced.
//! All generated inputs remain bounded by the available test environment.
//!
//! A finite test suite cannot prove support for literally infinite input.
//! It can establish the invariants that allow input to scale with available
//! memory, address space, configured resource budgets, and implementation
//! capabilities.

use super::super::cursor::{Cursor, CursorPosition};
use super::super::identifier::{
    is_identifier,
    is_identifier_continue,
    is_identifier_start,
    scan_identifier,
};

// ---------------------------------------------------------------------------
// Shared assertions
// ---------------------------------------------------------------------------

fn assert_valid_identifier(spelling: &str) {
    assert!(
        is_identifier(spelling),
        "expected valid Unicode identifier: {spelling:?}"
    );
}

fn assert_invalid_identifier(spelling: &str) {
    assert!(
        !is_identifier(spelling),
        "expected invalid Unicode identifier: {spelling:?}"
    );
}

fn assert_cursor_at(
    cursor: &Cursor<'_>,
    byte_offset: usize,
    line: usize,
    column: usize,
) {
    assert_eq!(
        cursor.position(),
        CursorPosition::new(byte_offset, line, column),
        "unexpected source position"
    );
}

// ---------------------------------------------------------------------------
// Unicode identifier categories
// ---------------------------------------------------------------------------

#[test]
fn accepts_letters_from_multiple_unicode_scripts() {
    for spelling in [
        "café",
        "Éclair",
        "λ",
        "Ωmega",
        "变量",
        "名前",
        "Программа",
        "مرحبا",
        "नमस्ते",
        "தமிழ்",
        "ქართული",
        "한글",
        "ไทย",
        "עברית",
        "Ελληνικά",
        "日本語",
    ] {
        assert_valid_identifier(spelling);
    }
}

#[test]
fn accepts_unicode_letter_numbers_at_identifier_start() {
    // Unicode general category Nl.
    for spelling in [
        "Ⅻ",
        "Ⅳ",
        "Ⅻname",
    ] {
        assert_valid_identifier(spelling);
    }

    assert!(is_identifier_start('Ⅻ'));
}

#[test]
fn accepts_unicode_decimal_digits_after_a_valid_start() {
    for spelling in [
        "value0",
        "value9",
        "name١",
        "name२",
        "名前３",
        "value９",
        "variable𝟘",
    ] {
        assert_valid_identifier(spelling);
    }

    for ch in ['0', '١', '२', '３', '９', '𝟘'] {
        assert!(
            is_identifier_continue(ch),
            "expected decimal digit to be valid in continuation position: {ch:?}"
        );
    }
}

#[test]
fn rejects_decimal_digits_at_identifier_start() {
    for spelling in [
        "0",
        "123",
        "9value",
        "١name",
        "２名前",
        "９value",
        "𝟘variable",
    ] {
        assert_invalid_identifier(spelling);
    }

    for ch in ['0', '١', '２', '９', '𝟘'] {
        assert!(
            !is_identifier_start(ch),
            "decimal digit must not start an identifier: {ch:?}"
        );
    }
}

#[test]
fn accepts_combining_marks_after_valid_start_characters() {
    // U+0301 COMBINING ACUTE ACCENT: Mn.
    // U+0327 COMBINING CEDILLA: Mn.
    // U+0903 DEVANAGARI SIGN VISARGA: Mc.
    for spelling in [
        "a\u{0301}",
        "e\u{0301}clair",
        "cafe\u{0301}",
        "a\u{0327}",
        "a\u{0903}",
        "λ\u{0301}",
        "变量\u{0301}",
    ] {
        assert_valid_identifier(spelling);
    }

    assert!(is_identifier_continue('\u{0301}'));
    assert!(is_identifier_continue('\u{0327}'));
    assert!(is_identifier_continue('\u{0903}'));
}

#[test]
fn combining_marks_cannot_start_identifiers() {
    for ch in [
        '\u{0301}',
        '\u{0327}',
        '\u{0903}',
    ] {
        assert!(
            !is_identifier_start(ch),
            "combining mark unexpectedly accepted at start: {ch:?}"
        );
    }

    assert_invalid_identifier("\u{0301}name");
    assert_invalid_identifier("\u{0327}name");
    assert_invalid_identifier("\u{0903}name");
}

#[test]
fn accepts_connector_punctuation_in_continuation_positions() {
    for spelling in [
        "first_second",
        "first\u{203F}second",
        "first\u{2040}second",
    ] {
        assert_valid_identifier(spelling);
    }

    assert!(is_identifier_continue('_'));
    assert!(is_identifier_continue('\u{203F}'));
    assert!(is_identifier_continue('\u{2040}'));
}

#[test]
fn rejects_symbols_punctuation_and_non_identifier_characters() {
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
fn rejects_whitespace_inside_identifiers() {
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
        "a\u{1680}b",
        "a\u{2003}b",
        "a\u{2028}b",
        "a\u{2029}b",
        "a\u{3000}b",
    ] {
        assert_invalid_identifier(spelling);
    }
}

#[test]
fn rejects_format_controls_and_joiners() {
    // These characters are not included in the identifier profile.
    for spelling in [
        "a\u{200C}b", // ZERO WIDTH NON-JOINER
        "a\u{200D}b", // ZERO WIDTH JOINER
        "a\u{200E}b", // LEFT-TO-RIGHT MARK
        "a\u{200F}b", // RIGHT-TO-LEFT MARK
        "a\u{2060}b", // WORD JOINER
        "a\u{FEFF}b", // ZERO WIDTH NO-BREAK SPACE
    ] {
        assert_invalid_identifier(spelling);
    }
}

// ---------------------------------------------------------------------------
// Unicode preservation and normalization policy
// ---------------------------------------------------------------------------

#[test]
fn preserves_composed_and_decomposed_identifier_spellings() {
    let composed = "é";
    let decomposed = "e\u{0301}";

    assert_valid_identifier(composed);
    assert_valid_identifier(decomposed);

    // These strings may look equivalent to a reader, but have different
    // Unicode scalar sequences and UTF-8 encodings.
    assert_ne!(composed, decomposed);

    let source = format!("{composed} {decomposed}");
    let mut cursor = Cursor::new(source.as_str());

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
fn preserves_identifier_case_without_case_folding() {
    let spellings = ["value", "Value", "VALUE", "vAlUe"];

    for spelling in spellings {
        assert_valid_identifier(spelling);
    }

    let source = spellings.join(" ");
    let mut cursor = Cursor::new(source.as_str());

    for expected in spellings {
        let identifier = scan_identifier(&mut cursor).expect("identifier");

        assert_eq!(identifier.spelling(), expected);
        assert_eq!(&source[identifier.byte_range()], expected);

        if cursor.peek().is_some() {
            let _ = cursor.consume();
        }
    }
}

#[test]
fn preserves_non_bmp_unicode_scalars() {
    // These characters are represented by four bytes in UTF-8.
    for spelling in [
        "𐐀name",
        "𐓐value",
        "𝒜lgebra",
        "𠀀文字",
    ] {
        assert_valid_identifier(spelling);
    }

    let source = "𐐀name!";
    let mut cursor = Cursor::new(source);

    let identifier = scan_identifier(&mut cursor).expect("non-BMP identifier");

    assert_eq!(identifier.spelling(), "𐐀name");
    assert_eq!(&source[identifier.byte_range()], "𐐀name");
    assert_eq!(cursor.peek(), Some('!'));
}

#[test]
fn keeps_domain_vocabulary_as_ordinary_identifiers() {
    // These examples are deliberately not an exhaustive operation list.
    // New computational domains must not require new Unicode rules.
    for spelling in [
        "H",
        "X",
        "CNOT",
        "quantum_operation",
        "custom_operation",
        "vendor_gate",
        "qpu_name",
        "gpu",
        "fpga",
        "asic",
        "tensor_compute",
        "hdl_module",
        "clock_signal",
        "distributed_node",
        "photonic_operation",
        "neuromorphic_unit",
        "变量计算",
        "量子操作",
    ] {
        assert_valid_identifier(spelling);
    }
}

// ---------------------------------------------------------------------------
// UTF-8 byte offsets, source slices, and scalar columns
// ---------------------------------------------------------------------------

#[test]
fn cursor_counts_utf8_bytes_and_unicode_scalar_columns_separately() {
    let mut cursor = Cursor::new("aλ界");

    assert_cursor_at(&cursor, 0, 1, 1);

    assert_eq!(cursor.consume(), Some('a'));
    assert_cursor_at(&cursor, 1, 1, 2);

    assert_eq!(cursor.consume(), Some('λ'));
    assert_cursor_at(&cursor, 3, 1, 3);

    assert_eq!(cursor.consume(), Some('界'));
    assert_cursor_at(&cursor, 6, 1, 4);

    assert!(cursor.is_eof());
}

#[test]
fn unicode_lookahead_does_not_change_cursor_state() {
    let cursor = Cursor::new("aλ界𐐀");

    let before = cursor.position();

    assert_eq!(cursor.peek(), Some('a'));
    assert_eq!(cursor.peek_n(0), Some('a'));
    assert_eq!(cursor.peek_n(1), Some('λ'));
    assert_eq!(cursor.peek_n(2), Some('界'));
    assert_eq!(cursor.peek_n(3), Some('𐐀'));
    assert_eq!(cursor.peek_n(4), None);

    assert_eq!(cursor.position(), before);
}

#[test]
fn cursor_never_splits_a_unicode_scalar() {
    let source = "𐐀λ界";
    let mut cursor = Cursor::new(source);

    for expected in source.chars() {
        let before = cursor.byte_offset();

        assert_eq!(cursor.consume(), Some(expected));

        let after = cursor.byte_offset();

        assert_eq!(
            after - before,
            expected.len_utf8(),
            "cursor consumed an unexpected number of bytes for {expected:?}"
        );

        assert!(source.is_char_boundary(after));
    }

    assert_eq!(cursor.consume(), None);
    assert!(cursor.is_eof());
}

#[test]
fn identifier_byte_ranges_are_half_open_utf8_ranges() {
    let source = "α变量42𐐀!";
    let mut cursor = Cursor::new(source);

    let identifier = scan_identifier(&mut cursor).expect("Unicode identifier");

    let expected = "α变量42𐐀";
    let range = identifier.byte_range();

    assert_eq!(identifier.spelling(), expected);
    assert_eq!(&source[range.clone()], expected);
    assert_eq!(range.start, 0);
    assert_eq!(range.end, expected.len());
    assert_eq!(identifier.byte_len(), expected.len());

    assert!(source.is_char_boundary(range.start));
    assert!(source.is_char_boundary(range.end));
    assert_eq!(cursor.peek(), Some('!'));
}

#[test]
fn identifier_positions_use_scalar_columns_not_utf8_bytes() {
    let source = "λ界 name";
    let mut cursor = Cursor::new(source);

    let first = scan_identifier(&mut cursor).expect("first identifier");

    assert_eq!(first.start(), CursorPosition::new(0, 1, 1));
    assert_eq!(first.end(), CursorPosition::new(5, 1, 3));

    assert_eq!(cursor.consume(), Some(' '));

    let second = scan_identifier(&mut cursor).expect("second identifier");

    assert_eq!(second.spelling(), "name");
    assert_eq!(second.start(), CursorPosition::new(6, 1, 4));
    assert_eq!(second.end(), CursorPosition::new(10, 1, 8));
}

#[test]
fn identifier_recognition_stops_at_the_first_non_identifier_scalar() {
    let cases = [
        ("λ变量!", "λ变量", '!'),
        ("café+", "café", '+'),
        ("名前.", "名前", '.'),
        ("αβγ;", "αβγ", ';'),
        ("value𐐀?", "value𐐀", '?'),
    ];

    for (source, expected, separator) in cases {
        let mut cursor = Cursor::new(source);

        let identifier = scan_identifier(&mut cursor).expect("identifier");

        assert_eq!(identifier.spelling(), expected, "source: {source:?}");
        assert_eq!(cursor.peek(), Some(separator), "source: {source:?}");
        assert_eq!(cursor.remaining(), &source[expected.len()..]);
    }
}

#[test]
fn failed_unicode_identifier_recognition_does_not_consume_input() {
    for source in [
        "",
        "123name",
        "١name",
        " name",
        "\u{0301}name",
        "-name",
        "\u{200C}name",
        "\u{200D}name",
    ] {
        let mut cursor = Cursor::new(source);

        let before_position = cursor.position();
        let before_offset = cursor.byte_offset();
        let before_remaining = cursor.remaining();

        assert!(
            scan_identifier(&mut cursor).is_none(),
            "expected recognition failure for {source:?}"
        );

        assert_eq!(cursor.position(), before_position);
        assert_eq!(cursor.byte_offset(), before_offset);
        assert_eq!(cursor.remaining(), before_remaining);
    }
}

// ---------------------------------------------------------------------------
// Unicode line separators and source coordinates
// ---------------------------------------------------------------------------

#[test]
fn cursor_tracks_line_feed_after_unicode_source_text() {
    let mut cursor = Cursor::new("λ界\n名前");

    for expected in ['λ', '界', '\n'] {
        assert_eq!(cursor.consume(), Some(expected));
    }

    assert_cursor_at(&cursor, "λ界\n".len(), 2, 1);
    assert_eq!(cursor.peek(), Some('名'));
}

#[test]
fn cursor_tracks_carriage_return_and_line_feed() {
    let mut cursor = Cursor::new("λ\r\n界");

    assert_eq!(cursor.consume(), Some('λ'));
    assert_eq!(cursor.consume(), Some('\r'));

    let after_cr = cursor.position();

    assert_eq!(after_cr.byte_offset(), "λ\r".len());
    assert_eq!(after_cr.line(), 2);
    assert_eq!(after_cr.column(), 1);

    assert_eq!(cursor.consume(), Some('\n'));

    // CRLF is one logical line break. Consuming LF after CR must not
    // increment the line a second time.
    assert_cursor_at(&cursor, "λ\r\n".len(), 2, 1);

    assert_eq!(cursor.consume(), Some('界'));
    assert_cursor_at(&cursor, "λ\r\n界".len(), 2, 2);
}

#[test]
fn cursor_tracks_unicode_line_and_paragraph_separators() {
    for separator in ['\u{2028}', '\u{2029}'] {
        let source = format!("λ{separator}界");
        let mut cursor = Cursor::new(source.as_str());

        assert_eq!(cursor.consume(), Some('λ'));
        assert_eq!(cursor.consume(), Some(separator));

        assert_cursor_at(
            &cursor,
            "λ".len() + separator.len_utf8(),
            2,
            1,
        );

        assert_eq!(cursor.consume(), Some('界'));
        assert_eq!(cursor.line(), 2);
        assert_eq!(cursor.column(), 2);
    }
}

#[test]
fn cursor_coordinates_remain_consistent_across_mixed_unicode_lines() {
    let source = "λ\r\n界\n𐐀\u{2028}名前";
    let mut cursor = Cursor::new(source);

    for expected in source.chars() {
        assert_eq!(cursor.consume(), Some(expected));

        assert!(
            source.is_char_boundary(cursor.byte_offset()),
            "cursor offset is not a UTF-8 boundary after {expected:?}"
        );

        assert!(cursor.line() >= 1);
        assert!(cursor.column() >= 1);
    }

    assert!(cursor.is_eof());
    assert_eq!(cursor.remaining(), "");
}

// ---------------------------------------------------------------------------
// Empty input, EOF, and progress
// ---------------------------------------------------------------------------

#[test]
fn empty_source_is_a_valid_cursor_state_but_not_an_identifier() {
    let mut cursor = Cursor::new("");

    assert_invalid_identifier("");
    assert!(cursor.is_eof());
    assert_eq!(cursor.peek(), None);
    assert_eq!(cursor.remaining(), "");
    assert_cursor_at(&cursor, 0, 1, 1);

    let before = cursor.position();

    assert_eq!(cursor.consume(), None);
    assert_eq!(cursor.consume(), None);
    assert_eq!(cursor.position(), before);
}

#[test]
fn successful_identifier_scanning_always_makes_progress() {
    let samples = [
        "_",
        "λ",
        "变量",
        "नमस्ते",
        "Ⅻname",
        "a\u{0301}",
        "value９",
        "𐐀name",
    ];

    for source in samples {
        let mut cursor = Cursor::new(source);

        let before = cursor.byte_offset();
        let identifier = scan_identifier(&mut cursor)
            .unwrap_or_else(|| panic!("expected identifier for {source:?}"));

        assert!(
            cursor.byte_offset() > before,
            "successful recognition made no progress for {source:?}"
        );

        assert_eq!(identifier.spelling(), source);
        assert!(cursor.is_eof());
    }
}

// ---------------------------------------------------------------------------
// Long identifiers and resource-aware scalability
// ---------------------------------------------------------------------------

#[test]
fn accepts_identifiers_longer_than_small_arbitrary_thresholds() {
    // This is a regression test against accidental small fixed limits,
    // not a claim that the language has an infinite finite-memory input.
    //
    // The generated input is bounded by the test environment and does not
    // establish a normative maximum identifier length.
    let source = "λ".repeat(16 * 1024);

    assert_valid_identifier(source.as_str());

    let mut cursor = Cursor::new(source.as_str());
    let identifier = scan_identifier(&mut cursor).expect("long identifier");

    assert_eq!(identifier.spelling(), source.as_str());
    assert_eq!(identifier.byte_range(), 0..source.len());
    assert_eq!(identifier.byte_len(), source.len());
    assert!(cursor.is_eof());
}

#[test]
fn long_mixed_unicode_identifiers_preserve_all_source_bytes() {
    let fragment = "α变量e\u{0301}９𐐀";
    let repetitions = 2048;
    let source = fragment.repeat(repetitions);

    assert_valid_identifier(source.as_str());

    let mut cursor = Cursor::new(source.as_str());
    let identifier = scan_identifier(&mut cursor).expect("long mixed identifier");

    assert_eq!(identifier.spelling(), source.as_str());
    assert_eq!(identifier.byte_range(), 0..source.len());
    assert_eq!(&source[identifier.byte_range()], source.as_str());
    assert!(cursor.is_eof());
}

#[test]
fn long_unicode_source_followed_by_invalid_input_stops_without_overconsuming() {
    let prefix = "变量".repeat(1024);
    let source = format!("{prefix}💥");

    let mut cursor = Cursor::new(source.as_str());
    let identifier = scan_identifier(&mut cursor).expect("long identifier");

    assert_eq!(identifier.spelling(), prefix.as_str());
    assert_eq!(identifier.byte_range(), 0..prefix.len());
    assert_eq!(cursor.byte_offset(), prefix.len());
    assert_eq!(cursor.peek(), Some('💥'));
    assert_eq!(cursor.remaining(), "💥");
}

// ---------------------------------------------------------------------------
// Determinism and repeatability
// ---------------------------------------------------------------------------

#[test]
fn unicode_identifier_scanning_is_deterministic() {
    let samples = [
        "λ",
        "变量",
        "नमस्ते",
        "Ⅻname",
        "e\u{0301}",
        "value９",
        "𐐀name",
        "α界𐐀!",
    ];

    for source in samples {
        let mut first_cursor = Cursor::new(source);
        let mut second_cursor = Cursor::new(source);

        let first = scan_identifier(&mut first_cursor);
        let second = scan_identifier(&mut second_cursor);

        assert_eq!(first, second, "source: {source:?}");
        assert_eq!(
            first_cursor.position(),
            second_cursor.position(),
            "source: {source:?}"
        );
        assert_eq!(
            first_cursor.remaining(),
            second_cursor.remaining(),
            "source: {source:?}"
        );
    }
}

#[test]
fn unicode_character_classification_is_repeatable() {
    let characters = [
        'a',
        '_',
        'λ',
        '界',
        'Ⅻ',
        '0',
        '９',
        '\u{0301}',
        '\u{0903}',
        '\u{203F}',
        '\u{200C}',
        '\u{200D}',
        '💥',
        '\u{2028}',
        '\u{2029}',
    ];

    for ch in characters {
        assert_eq!(
            is_identifier_start(ch),
            is_identifier_start(ch),
            "start classification changed for {ch:?}"
        );

        assert_eq!(
            is_identifier_continue(ch),
            is_identifier_continue(ch),
            "continuation classification changed for {ch:?}"
        );
    }
}

// ---------------------------------------------------------------------------
// Unicode scalar invariants
// ---------------------------------------------------------------------------

#[test]
fn every_valid_identifier_has_a_valid_start_and_valid_continuation() {
    let samples = [
        "_",
        "a",
        "λ",
        "变量",
        "Ⅻname",
        "a\u{0301}",
        "first\u{203F}second",
        "value９",
        "𐐀name",
    ];

    for source in samples {
        assert_valid_identifier(source);

        let mut chars = source.chars();
        let first = chars.next().expect("nonempty identifier");

        assert!(
            is_identifier_start(first),
            "invalid start in accepted identifier {source:?}"
        );

        for ch in chars {
            assert!(
                is_identifier_continue(ch),
                "invalid continuation {ch:?} in accepted identifier {source:?}"
            );
        }
    }
}

#[test]
fn unicode_symbols_do_not_become_identifier_characters_by_default() {
    for ch in [
        '💥',
        '→',
        '∞',
        '∑',
        '中' // Letter: valid, included here as a contrast below.
    ] {
        if ch == '中' {
            assert!(is_identifier_start(ch));
            assert_valid_identifier("中");
        } else {
            assert!(!is_identifier_start(ch));
            assert!(!is_identifier_continue(ch));
            assert_invalid_identifier(ch.encode_utf8(&mut [0; 4]));
        }
    }
}