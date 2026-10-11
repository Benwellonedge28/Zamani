#![forbid(unsafe_code)]

//! Production conformance tests for Zamani numeric literal scanning.
//!
//! File: src/frontend/lexer/tests/numbers.rs
//! Edition: Rust 2021
//! Minimum Rust: 1.97
//!
//! Grammar authority:
//! - grammar/spec/lexical.md
//! - grammar/lexer/numeric-literals.g4
//! - grammar/lexer/literals.g4
//! - grammar/antlr/ZamaniLexer.g4
//! - grammar/Zamani.g4
//! - grammar/grammar.md
//!
//! Implementation under test:
//! - src/frontend/lexer/number.rs
//! - src/frontend/lexer/cursor.rs
//! - src/frontend/lexer/config.rs
//!
//! Integration:
//! Register exactly once in src/frontend/lexer/tests/mod.rs:
//!
//!     mod numbers;
//!
//! Numeric recognition is lexical only. It does not perform numeric
//! conversion, infer types, impose machine-width limits, or select
//! quantum, classical, HDL, accelerator, or distributed targets.
//!
//! Tests use the production scanner functions and do not duplicate
//! the numeric recognizer.

use super::super::config::ResourceLimit;
use super::super::cursor::{Cursor, CursorPosition};
use super::super::number::{
    scan_number,
    scan_number_with_limit,
    IntegerRadix,
    NumberErrorKind,
    NumberKind,
};

fn scan(source: &str) -> Option<super::super::number::NumberLiteral<'_>> {
    scan_number(&mut Cursor::new(source))
}

fn assert_number(
    source: &str,
    expected_kind: NumberKind,
    expected_radix: Option<IntegerRadix>,
) {
    let mut cursor = Cursor::new(source);
    let number = scan_number(&mut cursor)
        .unwrap_or_else(|| panic!("expected numeric literal: {source:?}"));

    assert_eq!(number.spelling(), source);
    assert_eq!(number.kind(), expected_kind);
    assert_eq!(number.radix(), expected_radix);
    assert_eq!(number.byte_range(), 0..source.len());
    assert_eq!(number.byte_len(), source.len());
    assert_eq!(number.start(), CursorPosition::new(0, 1, 1));
    assert!(cursor.is_eof());
}

fn assert_error_and_restoration(
    source: &str,
    expected_kind: NumberErrorKind,
) {
    let mut cursor = Cursor::new(source);
    let before = cursor.position();
    let remaining = cursor.remaining();

    let error = scan_number_with_limit(
        &mut cursor,
        ResourceLimit::Unlimited,
    )
    .expect_err("expected malformed numeric literal");

    assert_eq!(error.kind(), expected_kind);
    assert_eq!(cursor.position(), before);
    assert_eq!(cursor.remaining(), remaining);
}

#[test]
fn recognizes_decimal_integers() {
    for source in [
        "0",
        "00",
        "000",
        "1",
        "9",
        "10",
        "42",
        "123456789",
        "1_000",
        "10_000_000",
    ] {
        assert_number(
            source,
            NumberKind::Integer,
            Some(IntegerRadix::Decimal),
        );
    }
}

#[test]
fn recognizes_binary_integers() {
    for source in ["0b0", "0b1", "0b1010", "0B1010_0101"] {
        assert_number(
            source,
            NumberKind::Integer,
            Some(IntegerRadix::Binary),
        );
    }
}

#[test]
fn recognizes_octal_integers() {
    for source in ["0o0", "0o7", "0o755", "0O755_123"] {
        assert_number(
            source,
            NumberKind::Integer,
            Some(IntegerRadix::Octal),
        );
    }
}

#[test]
fn recognizes_hexadecimal_integers() {
    for source in [
        "0x0",
        "0x1",
        "0xDEAD",
        "0xDEAD_BEEF",
        "0Xff",
        "0x0123456789abcdef",
    ] {
        assert_number(
            source,
            NumberKind::Integer,
            Some(IntegerRadix::Hexadecimal),
        );
    }
}

#[test]
fn recognizes_decimal_floating_point_literals() {
    for source in [
        "0.0",
        "1.0",
        "3.14159",
        ".5",
        ".25",
        "1.",
        "1_000.25",
        "1_000.25_50",
    ] {
        assert_number(source, NumberKind::Float, None);
    }
}

#[test]
fn recognizes_exponent_notation() {
    for source in [
        "1e0",
        "1E10",
        "1e+10",
        "1e-10",
        "1.5e10",
        ".5e2",
        "1.e2",
        "1e1_000",
        "1.25E-12",
    ] {
        assert_number(source, NumberKind::Float, None);
    }
}

#[test]
fn preserves_numeric_source_spelling() {
    for source in [
        "00042",
        "1_000",
        "0xDE_AD",
        "0b1010_0101",
        "1.2500",
        ".500",
        "1.20e+03",
    ] {
        let number = scan(source).expect("valid numeric literal");

        assert_eq!(number.spelling(), source);
        assert_eq!(&source[number.byte_range()], source);
    }
}

#[test]
fn reports_digit_counts_without_counting_separators_or_prefixes() {
    let cases = [
        ("123", 3),
        ("1_000", 4),
        ("0b1010", 4),
        ("0o755", 3),
        ("0xDE_AD", 4),
        ("1.25", 3),
        (".125", 3),
        ("1e10", 4),
    ];

    for (source, expected) in cases {
        let number = scan(source).expect("valid number");
        assert_eq!(
            number.digit_count(),
            expected,
            "unexpected digit count for {source:?}"
        );
    }
}

#[test]
fn does_not_consume_a_leading_sign() {
    for source in ["-42", "+42", "-1.5", "+.5"] {
        let mut cursor = Cursor::new(source);

        assert!(matches!(cursor.peek(), Some('+' | '-')));
        assert!(scan_number(&mut cursor).is_none());
        assert_eq!(cursor.byte_offset(), 0);
        assert_eq!(cursor.remaining(), source);
    }
}

#[test]
fn does_not_confuse_range_operators_with_floats() {
    for source in ["1..10", "1..=10"] {
        let mut cursor = Cursor::new(source);
        let number = scan_number(&mut cursor).expect("integer prefix");

        assert_eq!(number.spelling(), "1");
        assert_eq!(cursor.byte_offset(), 1);
        assert_eq!(cursor.remaining(), &source[1..]);
        assert_eq!(cursor.peek(), Some('.'));
    }
}

#[test]
fn leading_period_requires_an_ascii_decimal_digit() {
    for source in [".", "..", "..=", ".name", ".λ", ".١"] {
        let mut cursor = Cursor::new(source);

        assert!(scan_number(&mut cursor).is_none());
        assert_eq!(cursor.byte_offset(), 0);
        assert_eq!(cursor.remaining(), source);
    }
}

#[test]
fn rejects_missing_digits_after_radix_prefix() {
    for source in ["0x", "0X", "0b", "0B", "0o", "0O"] {
        assert_error_and_restoration(
            source,
            NumberErrorKind::MissingRadixDigits,
        );
    }
}

#[test]
fn rejects_invalid_numeric_separators() {
    for source in [
        "1_",
        "1__0",
        "_1",
        "0x_FF",
        "0b_10",
        "0o_7",
        "1._0",
        "1e_2",
        "1e+_2",
    ] {
        assert_error_and_restoration(
            source,
            NumberErrorKind::InvalidDigitSeparator,
        );
    }
}

#[test]
fn rejects_missing_exponent_digits() {
    for source in ["1e", "1E", "1e+", "1e-", "1.0e", "1.0e+"] {
        assert_error_and_restoration(
            source,
            NumberErrorKind::InvalidExponent,
        );
    }
}

#[test]
fn unicode_decimal_digits_are_not_numeric_literal_digits() {
    // The numeric grammar uses ASCII digits. Unicode digit support for
    // identifiers is a separate lexical concern.
    for source in ["١", "१२३", "９", "0b١", "0xＦＦ"] {
        let mut cursor = Cursor::new(source);

        if source.starts_with('0') {
            // A based prefix with no valid ASCII digits is malformed.
            let result = scan_number_with_limit(
                &mut cursor,
                ResourceLimit::Unlimited,
            );

            assert!(result.is_err(), "expected invalid based number: {source:?}");
            assert_eq!(cursor.byte_offset(), 0);
        } else {
            assert!(scan_number(&mut cursor).is_none());
            assert_eq!(cursor.byte_offset(), 0);
        }
    }
}

#[test]
fn distinguishes_integer_and_float_categories() {
    for source in ["0", "42", "0b101", "0o77", "0xFF"] {
        let number = scan(source).expect("integer");

        assert!(number.is_integer());
        assert!(!number.is_float());
    }

    for source in ["1.", ".5", "1.5", "1e3", "1.5e-3"] {
        let number = scan(source).expect("float");

        assert!(number.is_float());
        assert!(!number.is_integer());
        assert_eq!(number.radix(), None);
    }
}

#[test]
fn handles_numbers_inside_larger_source_without_overconsumption() {
    let source = "value=123;next=.5!";
    let mut cursor = Cursor::new(source);

    while cursor.peek() != Some('1') {
        assert!(cursor.consume().is_some());
    }

    let integer = scan_number(&mut cursor).expect("integer");
    assert_eq!(integer.spelling(), "123");
    assert_eq!(cursor.peek(), Some(';'));

    while cursor.peek() != Some('.') {
        assert!(cursor.consume().is_some());
    }

    let float = scan_number(&mut cursor).expect("float");
    assert_eq!(float.spelling(), ".5");
    assert_eq!(cursor.peek(), Some('!'));
}

#[test]
fn reports_multiline_positions_consistently() {
    let source = "first\n123";
    let mut cursor = Cursor::new(source);

    for _ in 0..6 {
        assert!(cursor.consume().is_some());
    }

    assert_eq!(cursor.peek(), Some('1'));

    let number = scan_number(&mut cursor).expect("integer");

    assert_eq!(number.start(), CursorPosition::new(6, 2, 1));
    assert_eq!(number.end(), CursorPosition::new(9, 2, 4));
    assert_eq!(number.byte_range(), 6..9);
}

#[test]
fn enforces_an_explicit_numeric_digit_budget() {
    let source = "123456";
    let mut cursor = Cursor::new(source);

    let limit = ResourceLimit::at_most(3)
        .expect("valid resource budget");

    let error = scan_number_with_limit(&mut cursor, limit)
        .expect_err("digit budget must be enforced");

    assert_eq!(error.kind(), NumberErrorKind::DigitBudgetExceeded);
    assert!(error.is_resource_exhaustion());

    // Failed scans restore the cursor so recovery can choose its own policy.
    assert_eq!(cursor.byte_offset(), 0);
    assert_eq!(cursor.remaining(), source);
}

#[test]
fn unlimited_budget_accepts_numbers_beyond_small_fixed_widths() {
    // Lexical recognition must not depend on u32, u64, or u128 conversion.
    // This tests spelling recognition, not arbitrary-precision arithmetic.
    let source = "9".repeat(512);
    let mut cursor = Cursor::new(&source);

    let number = scan_number_with_limit(
        &mut cursor,
        ResourceLimit::Unlimited,
    )
    .expect("unlimited lexical budget")
    .expect("valid integer");

    assert_eq!(number.spelling(), source);
    assert_eq!(number.digit_count(), 512);
    assert_eq!(number.byte_range(), 0..source.len());
    assert!(cursor.is_eof());
}

#[test]
fn handles_large_numeric_literals_without_language_level_width_limits() {
    // A practical stress sample, not a maximum supported size.
    let source = format!("0x{}", "A5".repeat(32_768));
    let mut cursor = Cursor::new(&source);

    let number = scan_number_with_limit(
        &mut cursor,
        ResourceLimit::Unlimited,
    )
    .expect("unlimited resource policy")
    .expect("valid large hexadecimal literal");

    assert_eq!(number.kind(), NumberKind::Integer);
    assert_eq!(number.radix(), Some(IntegerRadix::Hexadecimal));
    assert_eq!(number.spelling(), source);
    assert_eq!(number.digit_count(), 65_536);
    assert!(cursor.is_eof());
}

#[test]
fn does_not_consume_non_numeric_input() {
    for source in ["", "name", "λ", "_", "+", "-", "!", "∞"] {
        let mut cursor = Cursor::new(source);

        assert!(scan_number(&mut cursor).is_none());
        assert_eq!(cursor.byte_offset(), 0);
        assert_eq!(cursor.remaining(), source);
    }
}

#[test]
fn repeated_scans_are_deterministic() {
    fn collect(source: &str) -> Vec<(String, NumberKind, Option<IntegerRadix>)> {
        let mut cursor = Cursor::new(source);
        let mut result = Vec::new();

        while !cursor.is_eof() {
            match scan_number(&mut cursor) {
                Some(number) => result.push((
                    number.spelling().to_owned(),
                    number.kind(),
                    number.radix(),
                )),
                None => {
                    assert!(
                        cursor.consume().is_some(),
                        "cursor must advance until EOF"
                    );
                }
            }
        }

        result
    }

    let source = "42 0xFF .5 1.25e-3 0b1010";
    assert_eq!(collect(source), collect(source));
}

#[test]
fn numeric_recognition_is_independent_of_execution_domain() {
    // Numbers remain ordinary source-level values in classical, quantum,
    // HDL, tensor, distributed, and future computational contexts.
    // The lexer must not reserve numeric forms for particular domains.
    for source in [
        "0",
        "1",
        "0.5",
        "1e-9",
        "0xFF",
        "0b1010",
        "0o755",
        "1_000_000",
    ] {
        assert!(scan(source).is_some(), "expected valid number: {source}");
    }
}