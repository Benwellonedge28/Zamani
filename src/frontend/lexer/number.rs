
#![forbid(unsafe_code)]

//! Numeric literal recognition for the Zamani frontend.
//!
//! File: src/frontend/lexer/number.rs
//! Edition: Rust 2021
//! Minimum Rust: 1.97
//! Safety: Safe Rust only.
//!
//! # Responsibilities
//!
//! - Recognize decimal, binary, octal, and hexadecimal integers.
//! - Recognize decimal floating-point literals and exponent notation.
//! - Enforce numeric separator placement.
//! - Preserve exact source spelling without allocating.
//! - Preserve source byte ranges and cursor positions.
//! - Support configurable numeric-digit budgets.
//! - Distinguish malformed numeric syntax from resource exhaustion.
//!
//! # Does not own
//!
//! - Numeric type inference or conversion.
//! - Arbitrary-precision arithmetic.
//! - Integer overflow or floating-point rounding.
//! - Numeric suffixes, which are not in the current grammar.
//! - Hardware widths, quantum parameters, or target capabilities.
//! - Token construction, diagnostics storage, or scanner recovery.
//!
//! # Grammar authority
//!
//! This implementation follows:
//!
//! - `grammar/spec/lexical.md`
//! - `grammar/lexer/numeric-literals.g4`
//! - `grammar/lexer/literals.g4`
//! - `grammar/antlr/ZamaniLexer.g4`
//!
//! Numeric syntax must remain synchronized with those specifications.
//!
//! # Integration
//!
//! `scanner.rs` invokes `scan_number` or `scan_number_with_limit` when
//! the next source character can begin a numeric literal.
//!
//! The scanner maps `NumberKind::Integer` to `TokenKind::IntegerLiteral`
//! and `NumberKind::Float` to `TokenKind::FloatLiteral`, then constructs
//! a borrowed token with the canonical source span.
//!
//! Diagnostics and recovery remain owned by their respective modules.
//! Semantic analysis decides how a numeric spelling is represented.
//!
//! This module does not impose a language-level numeric width or a fixed
//! digit-count ceiling. A configured budget is an implementation policy.

use core::fmt;
use core::ops::Range;

use super::config::ResourceLimit;
use super::cursor::{Cursor, CursorPosition};

/// The source-level category of a numeric literal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NumberKind {
    /// An integer in decimal, binary, octal, or hexadecimal notation.
    Integer,

    /// A decimal floating-point literal.
    Float,
}

/// The radix used by an integer literal.
///
/// Floating-point literals use decimal notation and therefore have no
/// integer radix associated with them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IntegerRadix {
    Decimal,
    Binary,
    Octal,
    Hexadecimal,
}

/// A successfully recognized numeric literal.
///
/// The source spelling is borrowed from the original source. No numeric
/// conversion, normalization, separator removal, or allocation occurs.
///
/// The byte range is zero-based and half-open: `start..end`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NumberLiteral<'src> {
    source: &'src str,
    byte_range: Range<usize>,
    start: CursorPosition,
    end: CursorPosition,
    kind: NumberKind,
    radix: Option<IntegerRadix>,
    digit_count: usize,
}

impl<'src> NumberLiteral<'src> {
    fn new(
        source: &'src str,
        byte_range: Range<usize>,
        start: CursorPosition,
        end: CursorPosition,
        kind: NumberKind,
        radix: Option<IntegerRadix>,
        digit_count: usize,
    ) -> Self {
        Self {
            source,
            byte_range,
            start,
            end,
            kind,
            radix,
            digit_count,
        }
    }

    /// Returns the exact numeric spelling from the source.
    #[must_use]
    pub fn spelling(&self) -> &'src str {
        // The range is created from positions reported by the cursor.
        // Those positions are UTF-8 boundaries because cursor movement
        // consumes complete Unicode scalar values.
        &self.source[self.byte_range.clone()]
    }

    /// Returns the original source byte range, half-open.
    #[must_use]
    pub fn byte_range(&self) -> Range<usize> {
        self.byte_range.clone()
    }

    /// Returns the numeric category.
    #[must_use]
    pub const fn kind(&self) -> NumberKind {
        self.kind
    }

    /// Returns the integer radix, or `None` for floating-point literals.
    #[must_use]
    pub const fn radix(&self) -> Option<IntegerRadix> {
        self.radix
    }

    /// Returns the starting cursor position.
    #[must_use]
    pub const fn start(&self) -> CursorPosition {
        self.start
    }

    /// Returns the ending cursor position.
    #[must_use]
    pub const fn end(&self) -> CursorPosition {
        self.end
    }

    /// Returns the count of numeric digits, excluding separators and
    /// radix-prefix characters.
    ///
    /// For example, `0xDE_AD` contains four numeric digits.
    #[must_use]
    pub const fn digit_count(&self) -> usize {
        self.digit_count
    }

    /// Returns the source spelling's UTF-8 byte length.
    #[must_use]
    pub fn byte_len(&self) -> usize {
        self.byte_range.end - self.byte_range.start
    }

    /// Returns whether this is an integer literal.
    #[must_use]
    pub const fn is_integer(&self) -> bool {
        matches!(self.kind, NumberKind::Integer)
    }

    /// Returns whether this is a floating-point literal.
    #[must_use]
    pub const fn is_float(&self) -> bool {
        matches!(self.kind, NumberKind::Float)
    }
}

/// The category of numeric scanning failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NumberErrorKind {
    /// A radix prefix was not followed by a valid digit.
    MissingRadixDigits,

    /// An underscore was not positioned between valid digits.
    InvalidDigitSeparator,

    /// An exponent marker was not followed by a valid exponent.
    InvalidExponent,

    /// A numeric spelling exceeded the caller's configured digit budget.
    DigitBudgetExceeded,

    /// A checked counter could not represent the observed count.
    CounterOverflow,
}

/// An error encountered while recognizing a numeric literal.
///
/// The range identifies the source region associated with the error.
/// Resource-budget exhaustion is deliberately distinguishable from
/// malformed numeric syntax.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NumberError {
    kind: NumberErrorKind,
    byte_range: Range<usize>,
    start: CursorPosition,
    end: CursorPosition,
    observed_digits: usize,
}

impl NumberError {
    fn new(
        kind: NumberErrorKind,
        byte_range: Range<usize>,
        start: CursorPosition,
        end: CursorPosition,
        observed_digits: usize,
    ) -> Self {
        Self {
            kind,
            byte_range,
            start,
            end,
            observed_digits,
        }
    }

    /// Returns the error category.
    #[must_use]
    pub const fn kind(&self) -> NumberErrorKind {
        self.kind
    }

    /// Returns the half-open source byte range associated with the error.
    #[must_use]
    pub fn byte_range(&self) -> Range<usize> {
        self.byte_range.clone()
    }

    /// Returns the error's starting position.
    #[must_use]
    pub const fn start(&self) -> CursorPosition {
        self.start
    }

    /// Returns the error's ending position.
    #[must_use]
    pub const fn end(&self) -> CursorPosition {
        self.end
    }

    /// Returns the numeric digit count observed before the error.
    #[must_use]
    pub const fn observed_digits(&self) -> usize {
        self.observed_digits
    }

    /// Returns whether the failure is due to resource policy.
    #[must_use]
    pub const fn is_resource_exhaustion(&self) -> bool {
        matches!(
            self.kind,
            NumberErrorKind::DigitBudgetExceeded
                | NumberErrorKind::CounterOverflow
        )
    }
}

impl fmt::Display for NumberError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            NumberErrorKind::MissingRadixDigits => {
                formatter.write_str("numeric radix prefix requires at least one digit")
            }
            NumberErrorKind::InvalidDigitSeparator => {
                formatter.write_str(
                    "numeric separator must occur between valid digits",
                )
            }
            NumberErrorKind::InvalidExponent => {
                formatter.write_str(
                    "floating-point exponent requires decimal digits",
                )
            }
            NumberErrorKind::DigitBudgetExceeded => {
                write!(
                    formatter,
                    "numeric literal exceeded the configured digit budget \
                     after observing {} digits",
                    self.observed_digits
                )
            }
            NumberErrorKind::CounterOverflow => {
                formatter.write_str(
                    "numeric scanning counter exceeded the implementation's \
                     representable range",
                )
            }
        }
    }
}

impl std::error::Error for NumberError {}

/// Recognize a valid numeric literal at the cursor's current position.
///
/// Returns `None` when the next character cannot begin a numeric literal.
///
/// The function does not convert the value or impose a digit budget.
/// Callers processing untrusted input should prefer
/// [`scan_number_with_limit`] with an explicit resource policy.
///
/// A leading `+` or `-` is never consumed as part of the number.
///
/// # Examples
///
/// ```
/// use zamani_compiler::frontend::lexer::number::{
///     scan_number, NumberKind, IntegerRadix,
/// };
/// use zamani_compiler::frontend::lexer::cursor::Cursor;
///
/// let mut cursor = Cursor::new("0xDE_AD + .5");
///
/// let integer = scan_number(&mut cursor).expect("integer");
/// assert_eq!(integer.spelling(), "0xDE_AD");
/// assert_eq!(integer.kind(), NumberKind::Integer);
/// assert_eq!(integer.radix(), Some(IntegerRadix::Hexadecimal));
///
/// cursor.consume_while(char::is_whitespace);
///
/// assert_eq!(cursor.peek(), Some('+'));
/// ```
///
/// This recognizer recognizes valid prefixes. The scanner must diagnose
/// malformed numeric constructs using the lexical specification and
/// structured diagnostic/recovery policies.
pub fn scan_number<'src>(
    cursor: &mut Cursor<'src>,
) -> Option<NumberLiteral<'src>> {
    scan_number_with_limit(cursor, ResourceLimit::Unlimited)
        .ok()
        .flatten()
}

/// Recognize a numeric literal under a configurable digit budget.
///
/// `ResourceLimit::Unlimited` means no explicit numeric-digit budget is
/// applied. It does not guarantee unlimited memory, time, or address space.
///
/// On a `NumberError`, the cursor is restored to its starting checkpoint.
/// This allows the scanner to choose a deterministic diagnostic and
/// recovery policy without inheriting partially consumed cursor state.
///
/// A valid number returns `Ok(Some(number))`. If the cursor cannot begin a
/// numeric literal, it returns `Ok(None)`.
pub fn scan_number_with_limit<'src>(
    cursor: &mut Cursor<'src>,
    digit_limit: ResourceLimit,
) -> Result<Option<NumberLiteral<'src>>, NumberError> {
    let checkpoint = cursor.checkpoint();
    let source = cursor.source();
    let start = cursor.position();
    let start_offset = cursor.byte_offset();

    let result = scan_inner(cursor, digit_limit, source, start, start_offset);

    if result.is_err() {
        // This checkpoint was obtained from this cursor, so restoration
        // should succeed unless the cursor implementation violates its
        // own checkpoint contract.
        let _ = cursor.restore(checkpoint);
    }

    result
}

fn scan_inner<'src>(
    cursor: &mut Cursor<'src>,
    digit_limit: ResourceLimit,
    source: &'src str,
    start: CursorPosition,
    start_offset: usize,
) -> Result<Option<NumberLiteral<'src>>, NumberError> {
    let first = match cursor.peek() {
        Some(ch) => ch,
        None => return Ok(None),
    };

    // Numeric digits are deliberately ASCII in the normative grammar.
    // Unicode decimal digits are not silently substituted for ASCII digits.
    if !first.is_ascii_digit() && first != '.' {
        return Ok(None);
    }

    // A leading period begins a float only if immediately followed by a
    // decimal digit. Otherwise it belongs to punctuation/operator scanning.
    if first == '.' && !cursor.peek_n(1).is_some_and(|ch| ch.is_ascii_digit()) {
        return Ok(None);
    }

    let mut digit_count = 0usize;
    let mut kind = NumberKind::Integer;
    let mut radix = Some(IntegerRadix::Decimal);

    if first == '.' {
        // Leading-dot decimal float: .5, .25, .5e2
        cursor.consume();
        consume_digit_sequence(
            cursor,
            10,
            digit_limit,
            &mut digit_count,
            source,
            start,
            start_offset,
        )?;
        kind = NumberKind::Float;
        radix = None;
        consume_optional_exponent(
            cursor,
            digit_limit,
            &mut digit_count,
            source,
            start,
            start_offset,
        )?;
    } else {
        // The first digit has already been checked.
        if first == '0' {
            match cursor.peek_n(1) {
                Some('x' | 'X') => {
                    cursor.consume();
                    cursor.consume();
                    radix = Some(IntegerRadix::Hexadecimal);
                    consume_required_radix_digits(
                        cursor,
                        16,
                        digit_limit,
                        &mut digit_count,
                        source,
                        start,
                        start_offset,
                    )?;
                    return finish(
                        cursor,
                        source,
                        start,
                        start_offset,
                        NumberKind::Integer,
                        radix,
                        digit_count,
                    );
                }
                Some('b' | 'B') => {
                    cursor.consume();
                    cursor.consume();
                    radix = Some(IntegerRadix::Binary);
                    consume_required_radix_digits(
                        cursor,
                        2,
                        digit_limit,
                        &mut digit_count,
                        source,
                        start,
                        start_offset,
                    )?;
                    return finish(
                        cursor,
                        source,
                        start,
                        start_offset,
                        NumberKind::Integer,
                        radix,
                        digit_count,
                    );
                }
                Some('o' | 'O') => {
                    cursor.consume();
                    cursor.consume();
                    radix = Some(IntegerRadix::Octal);
                    consume_required_radix_digits(
                        cursor,
                        8,
                        digit_limit,
                        &mut digit_count,
                        source,
                        start,
                        start_offset,
                    )?;
                    return finish(
                        cursor,
                        source,
                        start,
                        start_offset,
                        NumberKind::Integer,
                        radix,
                        digit_count,
                    );
                }
                _ => {}
            }
        }

        consume_digit_sequence(
            cursor,
            10,
            digit_limit,
            &mut digit_count,
            source,
            start,
            start_offset,
        )?;

        // Do not consume the first period in a range operator. This makes
        // 1..10 and 1..=10 tokenize as integer, range operator, integer.
        if cursor.peek() == Some('.')
            && cursor.peek_n(1) != Some('.')
        {
            cursor.consume();
            kind = NumberKind::Float;
            radix = None;

            // The fractional sequence is optional: 1. and 1.e2 are valid.
            if cursor.peek().is_some_and(|ch| ch.is_ascii_digit()) {
                consume_digit_sequence(
                    cursor,
                    10,
                    digit_limit,
                    &mut digit_count,
                    source,
                    start,
                    start_offset,
                )?;
            } else if cursor.peek() == Some('_') {
                return Err(error_at(
                    cursor,
                    NumberErrorKind::InvalidDigitSeparator,
                    start,
                    start_offset,
                    digit_count,
                ));
            }
        }

        if cursor.peek().is_some_and(|ch| matches!(ch, 'e' | 'E')) {
            kind = NumberKind::Float;
            radix = None;
            consume_required_exponent(
                cursor,
                digit_limit,
                &mut digit_count,
                source,
                start,
                start_offset,
            )?;
        }
    }

    finish(
        cursor,
        source,
        start,
        start_offset,
        kind,
        radix,
        digit_count,
    )
}

fn consume_required_radix_digits(
    cursor: &mut Cursor<'_>,
    radix: u32,
    digit_limit: ResourceLimit,
    digit_count: &mut usize,
    source: &str,
    start: CursorPosition,
    start_offset: usize,
) -> Result<(), NumberError> {
    if !cursor.peek().is_some_and(|ch| digit_value(ch).is_some_and(|d| d < radix)) {
        return Err(error_at(
            cursor,
            NumberErrorKind::MissingRadixDigits,
            start,
            start_offset,
            *digit_count,
        ));
    }

    consume_digit_sequence(
        cursor,
        radix,
        digit_limit,
        digit_count,
        source,
        start,
        start_offset,
    )
}

fn consume_optional_exponent(
    cursor: &mut Cursor<'_>,
    digit_limit: ResourceLimit,
    digit_count: &mut usize,
    source: &str,
    start: CursorPosition,
    start_offset: usize,
) -> Result<(), NumberError> {
    if cursor.peek().is_some_and(|ch| matches!(ch, 'e' | 'E')) {
        consume_required_exponent(
            cursor,
            digit_limit,
            digit_count,
            source,
            start,
            start_offset,
        )?;
    }

    Ok(())
}

fn consume_required_exponent(
    cursor: &mut Cursor<'_>,
    digit_limit: ResourceLimit,
    digit_count: &mut usize,
    source: &str,
    start: CursorPosition,
    start_offset: usize,
) -> Result<(), NumberError> {
    cursor.consume(); // e or E

    if cursor.peek().is_some_and(|ch| matches!(ch, '+' | '-')) {
        cursor.consume();
    }

    if !cursor.peek().is_some_and(|ch| ch.is_ascii_digit()) {
        return Err(error_at(
            cursor,
            NumberErrorKind::InvalidExponent,
            start,
            start_offset,
            *digit_count,
        ));
    }

    consume_digit_sequence(
        cursor,
        10,
        digit_limit,
        digit_count,
        source,
        start,
        start_offset,
    )
}

fn consume_digit_sequence(
    cursor: &mut Cursor<'_>,
    radix: u32,
    digit_limit: ResourceLimit,
    digit_count: &mut usize,
    source: &str,
    start: CursorPosition,
    start_offset: usize,
) -> Result<(), NumberError> {
    let mut consumed_any = false;

    loop {
        match cursor.peek() {
            Some(ch) if digit_value(ch).is_some_and(|digit| digit < radix) => {
                cursor.consume();
                consumed_any = true;

                *digit_count = digit_count.checked_add(1).ok_or_else(|| {
                    error_at(
                        cursor,
                        NumberErrorKind::CounterOverflow,
                        start,
                        start_offset,
                        *digit_count,
                    )
                })?;

                if !digit_limit.permits(*digit_count) {
                    return Err(error_at(
                        cursor,
                        NumberErrorKind::DigitBudgetExceeded,
                        start,
                        start_offset,
                        *digit_count,
                    ));
                }
            }
            Some('_') => {
                // Separators must have a valid digit on both sides.
                let next_is_digit = cursor
                    .peek_n(1)
                    .and_then(digit_value)
                    .is_some_and(|digit| digit < radix);

                if !consumed_any || !next_is_digit {
                    return Err(error_at(
                        cursor,
                        NumberErrorKind::InvalidDigitSeparator,
                        start,
                        start_offset,
                        *digit_count,
                    ));
                }

                cursor.consume();
                // The next loop iteration consumes the validated digit.
            }
            _ => break,
        }
    }

    let _ = source;
    Ok(())
}

fn finish<'src>(
    cursor: &Cursor<'src>,
    source: &'src str,
    start: CursorPosition,
    start_offset: usize,
    kind: NumberKind,
    radix: Option<IntegerRadix>,
    digit_count: usize,
) -> Result<Option<NumberLiteral<'src>>, NumberError> {
    let end = cursor.position();
    let end_offset = cursor.byte_offset();

    if end_offset == start_offset {
        return Ok(None);
    }

    Ok(Some(NumberLiteral::new(
        source,
        start_offset..end_offset,
        start,
        end,
        kind,
        radix,
        digit_count,
    )))
}

fn error_at(
    cursor: &Cursor<'_>,
    kind: NumberErrorKind,
    start: CursorPosition,
    start_offset: usize,
    observed_digits: usize,
) -> NumberError {
    NumberError::new(
        kind,
        start_offset..cursor.byte_offset(),
        start,
        cursor.position(),
        observed_digits,
    )
}

fn digit_value(ch: char) -> Option<u32> {
    match ch {
        '0'..='9' => Some(u32::from(ch as u8 - b'0')),
        'a'..='f' => Some(10 + u32::from(ch as u8 - b'a')),
        'A'..='F' => Some(10 + u32::from(ch as u8 - b'A')),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scan(source: &str) -> Option<NumberLiteral<'_>> {
        scan_number(&mut Cursor::new(source))
    }

    use super::super::cursor::Cursor;

    #[test]
    fn decimal_integers_preserve_their_spelling() {
        for source in ["0", "00", "000", "7", "42", "1_000", "10_000_000"] {
            let number = scan(source).expect("expected integer");
            assert_eq!(number.spelling(), source);
            assert_eq!(number.kind(), NumberKind::Integer);
            assert_eq!(number.radix(), Some(IntegerRadix::Decimal));
        }
    }

    #[test]
    fn based_integers_are_recognized() {
        let cases = [
            ("0b0", IntegerRadix::Binary),
            ("0B1010_0101", IntegerRadix::Binary),
            ("0o755_123", IntegerRadix::Octal),
            ("0O7", IntegerRadix::Octal),
            ("0xDEAD_BEEF", IntegerRadix::Hexadecimal),
            ("0Xff", IntegerRadix::Hexadecimal),
        ];

        for (source, radix) in cases {
            let number = scan(source).expect("expected based integer");
            assert_eq!(number.spelling(), source);
            assert_eq!(number.radix(), Some(radix));
            assert_eq!(number.kind(), NumberKind::Integer);
        }
    }

    #[test]
    fn decimal_floats_are_recognized() {
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
            let number = scan(source).expect("expected float");
            assert_eq!(number.spelling(), source);
            assert_eq!(number.kind(), NumberKind::Float);
            assert_eq!(number.radix(), None);
        }
    }

    #[test]
    fn exponent_forms_are_recognized() {
        for source in [
            "1e0",
            "1E10",
            "1e+10",
            "1e-10",
            "1.5e10",
            ".5e2",
            "1.e2",
            "1e1_000",
        ] {
            let number = scan(source).expect("expected exponent float");
            assert_eq!(number.spelling(), source);
            assert_eq!(number.kind(), NumberKind::Float);
        }
    }

    #[test]
    fn signs_remain_separate() {
        for source in ["-42", "+42", "-1.5", "+.5"] {
            let mut cursor = Cursor::new(source);
            assert!(matches!(cursor.peek(), Some('+' | '-')));
            assert!(scan_number(&mut cursor).is_none());
            assert_eq!(cursor.byte_offset(), 0);
        }
    }

    #[test]
    fn range_operators_are_not_consumed() {
        for source in ["1..10", "1..=10"] {
            let mut cursor = Cursor::new(source);
            let number = scan_number(&mut cursor).expect("integer");
            assert_eq!(number.spelling(), "1");
            assert_eq!(cursor.peek(), Some('.'));
            assert_eq!(cursor.remaining(), &source[1..]);
        }
    }

    #[test]
    fn leading_period_requires_a_digit() {
        for source in [".", "..", "..=", ".name"] {
            let mut cursor = Cursor::new(source);
            assert!(scan_number(&mut cursor).is_none());
            assert_eq!(cursor.byte_offset(), 0);
        }
    }

    #[test]
    fn radix_prefix_requires_digits_and_restores_cursor() {
        for source in ["0x", "0b", "0o"] {
            let mut cursor = Cursor::new(source);
            let error = scan_number_with_limit(
                &mut cursor,
                ResourceLimit::Unlimited,
            )
            .expect_err("malformed prefix");

            assert_eq!(error.kind(), NumberErrorKind::MissingRadixDigits);
            assert_eq!(cursor.byte_offset(), 0);
        }
    }

    #[test]
    fn malformed_exponents_are_reported() {
        for source in ["1e", "1e+", "1e-"] {
            let mut cursor = Cursor::new(source);
            let error = scan_number_with_limit(
                &mut cursor,
                ResourceLimit::Unlimited,
            )
            .expect_err("malformed exponent");

            assert_eq!(error.kind(), NumberErrorKind::InvalidExponent);
            assert_eq!(cursor.byte_offset(), 0);
        }
    }

    #[test]
    fn separator_errors_are_reported() {
        for source in ["1_", "1__0", "0x_FF", "0b_10", "0o_7", "1._0"] {
            let mut cursor = Cursor::new(source);
            let error = scan_number_with_limit(
                &mut cursor,
                ResourceLimit::Unlimited,
            )
            .expect_err("invalid separator");

            assert_eq!(
                error.kind(),
                NumberErrorKind::InvalidDigitSeparator
            );
            assert_eq!(cursor.byte_offset(), 0);
        }
    }

    #[test]
    fn digit_budget_is_separate_from_syntax() {
        let mut cursor = Cursor::new("12345");
        let limit = ResourceLimit::at_most(3).expect("nonzero budget");

        let error = scan_number_with_limit(&mut cursor, limit)
            .expect_err("digit budget must be enforced");

        assert_eq!(error.kind(), NumberErrorKind::DigitBudgetExceeded);
        assert!(error.is_resource_exhaustion());
        assert_eq!(cursor.byte_offset(), 0);

        let mut unlimited_cursor = Cursor::new("12345");
        let number = scan_number(&mut unlimited_cursor).expect("valid integer");
        assert_eq!(number.spelling(), "12345");
        assert_eq!(number.digit_count(), 5);
    }

    #[test]
    fn long_integers_are_not_converted_to_machine_types() {
        let source = "12345678901234567890123456789012345678901234567890";
        let number = scan(source).expect("long integer is lexically valid");

        assert_eq!(number.spelling(), source);
        assert_eq!(number.kind(), NumberKind::Integer);
        assert_eq!(number.digit_count(), source.len());
    }

    #[test]
    fn source_ranges_and_positions_are_preserved() {
        let mut cursor = Cursor::new("α 12.5");
        cursor.consume();
        cursor.consume();

        let number = scan_number(&mut cursor).expect("float");
        assert_eq!(number.spelling(), "12.5");
        assert_eq!(number.byte_range(), 3..7);
        assert_eq!(number.start().byte_offset(), 3);
        assert_eq!(number.end().byte_offset(), 7);
    }

    #[test]
    fn unicode_digits_do_not_start_numeric_literals() {
        let mut cursor = Cursor::new("١٢٣");
        assert!(scan_number(&mut cursor).is_none());
        assert_eq!(cursor.byte_offset(), 0);
    }

    #[test]
    fn integer_digit_count_excludes_prefix_and_separators() {
        let number = scan("0xDE_AD").expect("hex integer");
        assert_eq!(number.digit_count(), 4);
        assert_eq!(number.byte_len(), 7);
    }
}
