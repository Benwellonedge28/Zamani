
#![forbid(unsafe_code)]

//! Character-literal recognition for the Zamani frontend.
//!
//! Rust baseline: 1.97 or later
//! Edition: 2021
//!
//! # Responsibilities
//!
//! - Recognize single-quoted character literals.
//! - Require exactly one source character or one escape sequence.
//! - Validate the lexical syntax of simple and Unicode escapes.
//! - Preserve the original source spelling without decoding it.
//! - Report structured errors with byte ranges and cursor positions.
//! - Enforce an optional implementation-level literal byte budget.
//!
//! # Non-responsibilities
//!
//! This module does not:
//! - decode escapes into semantic values;
//! - decide whether an escaped numeric value is a valid Unicode scalar;
//! - define a target character width or encoding;
//! - construct AST nodes or canonical source spans;
//! - emit diagnostics or decide recovery policy;
//! - classify keywords, quantum operations, HDL names, or dialects;
//! - impose language-level hardware or source-size ceilings.
//!
//! # Grammar authority
//!
//! - `grammar/spec/lexical.md`
//! - `grammar/lexer/character-literals.g4`
//! - `grammar/lexer/literals.g4`
//! - `grammar/antlr/ZamaniLexer.g4`
//!
//! The grammar's public token is `CHAR`. The Rust frontend's canonical
//! identity is `TokenKind::CharacterLiteral`.
//!
//! # Integration
//!
//! `scanner.rs` calls [`scan_character_with_limit`] when the next source
//! character is an apostrophe. On success it constructs a
//! `TokenKind::CharacterLiteral` token from [`CharacterLiteral::spelling`]
//! and the start/end positions. On failure it maps [`CharacterErrorKind`]
//! to the canonical diagnostic model and applies `recovery.rs` policy.
//!
//! `config.rs` owns the budget policy. This module accepts the applicable
//! `ResourceLimit` as an argument and does not create a competing limit type.
//!
//! The lexer preserves spelling; semantic literal processing decodes escapes
//! and validates the represented Unicode scalar value.

use core::fmt;
use core::ops::Range;

use super::config::ResourceLimit;
use super::cursor::{Cursor, CursorPosition};

/// A successfully recognized character literal.
///
/// The source spelling includes both apostrophe delimiters. Escapes remain
/// in their original spelling; this type does not decode their value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CharacterLiteral<'src> {
    source: &'src str,
    byte_range: Range<usize>,
    start: CursorPosition,
    end: CursorPosition,
    uses_escape: bool,
}

impl<'src> CharacterLiteral<'src> {
    fn new(
        source: &'src str,
        byte_range: Range<usize>,
        start: CursorPosition,
        end: CursorPosition,
        uses_escape: bool,
    ) -> Self {
        Self {
            source,
            byte_range,
            start,
            end,
            uses_escape,
        }
    }

    /// Returns the exact source spelling, including apostrophe delimiters.
    #[must_use]
    pub fn spelling(&self) -> &'src str {
        &self.source[self.byte_range.clone()]
    }

    /// Returns the half-open UTF-8 byte range of the literal.
    #[must_use]
    pub fn byte_range(&self) -> Range<usize> {
        self.byte_range.clone()
    }

    /// Returns the source position at the opening apostrophe.
    #[must_use]
    pub const fn start(&self) -> CursorPosition {
        self.start
    }

    /// Returns the position immediately after the closing apostrophe.
    #[must_use]
    pub const fn end(&self) -> CursorPosition {
        self.end
    }

    /// Returns the literal's source length in UTF-8 bytes, delimiters included.
    #[must_use]
    pub fn byte_len(&self) -> usize {
        self.byte_range.end - self.byte_range.start
    }

    /// Returns whether the literal uses an escape sequence.
    #[must_use]
    pub const fn uses_escape(&self) -> bool {
        self.uses_escape
    }
}

/// The reason a character literal could not be recognized.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum CharacterErrorKind {
    /// The closing apostrophe or required content was not found.
    Unterminated,

    /// The literal contains no character, as in `''`.
    Empty,

    /// The literal contains more than one source character or escape.
    MultipleCharacters,

    /// The backslash is not followed by an escape designator.
    IncompleteEscape,

    /// The escape designator is not part of the documented escape set.
    InvalidEscape,

    /// A Unicode escape is incomplete or has invalid lexical syntax.
    InvalidUnicodeEscape,

    /// An unescaped source line break occurs inside the literal.
    InvalidRawCharacter,

    /// The configured implementation-level byte budget was exceeded.
    LiteralBudgetExceeded,

    /// A checked counter operation could not be represented.
    CounterOverflow,
}

impl CharacterErrorKind {
    /// Returns true when the error indicates a configured budget failure.
    #[must_use]
    pub const fn is_resource_exhaustion(self) -> bool {
        matches!(self, Self::LiteralBudgetExceeded | Self::CounterOverflow)
    }
}

impl fmt::Display for CharacterErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unterminated => f.write_str("unterminated character literal"),
            Self::Empty => f.write_str("character literal must not be empty"),
            Self::MultipleCharacters => {
                f.write_str("character literal must contain exactly one character")
            }
            Self::IncompleteEscape => {
                f.write_str("incomplete escape sequence in character literal")
            }
            Self::InvalidEscape => {
                f.write_str("unsupported escape sequence in character literal")
            }
            Self::InvalidUnicodeEscape => {
                f.write_str("malformed Unicode escape in character literal")
            }
            Self::InvalidRawCharacter => {
                f.write_str("unescaped line break in character literal")
            }
            Self::LiteralBudgetExceeded => {
                f.write_str("character literal exceeds the configured byte budget")
            }
            Self::CounterOverflow => {
                f.write_str("character literal byte counter overflowed")
            }
        }
    }
}

/// A character-literal recognition error.
///
/// `byte_range` is a half-open range into the original source. `start` and
/// `end` describe the inspected range before the scanner restores its cursor
/// to the position at which the literal began.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CharacterError {
    kind: CharacterErrorKind,
    byte_range: Range<usize>,
    start: CursorPosition,
    end: CursorPosition,
    observed_bytes: usize,
}

impl CharacterError {
    fn new(
        kind: CharacterErrorKind,
        byte_range: Range<usize>,
        start: CursorPosition,
        end: CursorPosition,
        observed_bytes: usize,
    ) -> Self {
        Self {
            kind,
            byte_range,
            start,
            end,
            observed_bytes,
        }
    }

    /// Returns the error category.
    #[must_use]
    pub const fn kind(&self) -> CharacterErrorKind {
        self.kind
    }

    /// Returns the half-open source byte range inspected before failure.
    #[must_use]
    pub fn byte_range(&self) -> Range<usize> {
        self.byte_range.clone()
    }

    /// Returns the position at the opening apostrophe.
    #[must_use]
    pub const fn start(&self) -> CursorPosition {
        self.start
    }

    /// Returns the cursor position at which recognition failed.
    #[must_use]
    pub const fn end(&self) -> CursorPosition {
        self.end
    }

    /// Returns the number of source bytes inspected, including the opening
    /// apostrophe when present.
    #[must_use]
    pub const fn observed_bytes(&self) -> usize {
        self.observed_bytes
    }

    /// Returns whether the error is due to resource exhaustion.
    #[must_use]
    pub const fn is_resource_exhaustion(&self) -> bool {
        self.kind.is_resource_exhaustion()
    }
}

impl fmt::Display for CharacterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} at {} (bytes {}..{})",
            self.kind,
            self.start,
            self.byte_range.start,
            self.byte_range.end
        )
    }
}

impl std::error::Error for CharacterError {}

/// Recognize a character literal at the cursor, without an explicit budget.
///
/// Returns `Ok(None)` without consuming input when the next character is not
/// an apostrophe. Otherwise returns a recognized literal or a structured
/// error. On a recognition error, the cursor is restored to its initial state.
///
/// This unlimited entry point is appropriate only when the caller's
/// configuration intentionally leaves the literal budget unlimited.
pub fn scan_character<'src>(
    cursor: &mut Cursor<'src>,
) -> Result<Option<CharacterLiteral<'src>>, CharacterError> {
    scan_character_with_limit(cursor, ResourceLimit::Unlimited)
}

/// Recognize a character literal with a configurable source-byte budget.
///
/// The budget counts UTF-8 bytes from the opening apostrophe through the
/// currently inspected position. It is an implementation resource policy,
/// not a restriction on the language's character representation.
///
/// On success, the cursor is positioned immediately after the closing
/// apostrophe. On a recognition error, the cursor is restored to its initial
/// state so the caller can apply a deliberate recovery policy.
///
/// `Ok(None)` means the next character is not an apostrophe; the cursor is
/// unchanged.
pub fn scan_character_with_limit<'src>(
    cursor: &mut Cursor<'src>,
    limit: ResourceLimit,
) -> Result<Option<CharacterLiteral<'src>>, CharacterError> {
    if cursor.peek() != Some('\'') {
        return Ok(None);
    }

    let checkpoint = cursor.checkpoint();
    let source = cursor.source();
    let start = cursor.position();

    match scan_inner(cursor, source, start, limit) {
        Ok(literal) => Ok(Some(literal)),
        Err(error) => {
            // The checkpoint was created from this cursor and source.
            // Restore is transactional; retain the original error if an
            // internal cursor invariant is ever violated.
            let _ = cursor.restore(checkpoint);
            Err(error)
        }
    }
}

fn scan_inner<'src>(
    cursor: &mut Cursor<'src>,
    source: &'src str,
    start: CursorPosition,
    limit: ResourceLimit,
) -> Result<CharacterLiteral<'src>, CharacterError> {
    consume_checked(cursor, start, limit)?;

    match cursor.peek() {
        None => return Err(make_error(
            cursor,
            start,
            CharacterErrorKind::Unterminated,
        )),
        Some('\'') => {
            consume_checked(cursor, start, limit)?;
            return Err(make_error(cursor, start, CharacterErrorKind::Empty));
        }
        Some('\r' | '\n') => {
            consume_checked(cursor, start, limit)?;
            return Err(make_error(
                cursor,
                start,
                CharacterErrorKind::InvalidRawCharacter,
            ));
        }
        _ => {}
    }

    let uses_escape = if cursor.peek() == Some('\\') {
        scan_escape(cursor, start, limit)?;
        true
    } else {
        consume_checked(cursor, start, limit)?;
        false
    };

    match cursor.peek() {
        Some('\'') => {
            consume_checked(cursor, start, limit)?;
        }
        None => {
            return Err(make_error(
                cursor,
                start,
                CharacterErrorKind::Unterminated,
            ));
        }
        Some('\r' | '\n') => {
            consume_checked(cursor, start, limit)?;
            return Err(make_error(
                cursor,
                start,
                CharacterErrorKind::InvalidRawCharacter,
            ));
        }
        Some(_) => {
            // Consume the first extra scalar for a useful, deterministic
            // error range. The outer entry point restores the cursor.
            consume_checked(cursor, start, limit)?;
            return Err(make_error(
                cursor,
                start,
                CharacterErrorKind::MultipleCharacters,
            ));
        }
    }

    let end = cursor.position();
    let byte_range = start.byte_offset()..end.byte_offset();

    Ok(CharacterLiteral::new(
        source,
        byte_range,
        start,
        end,
        uses_escape,
    ))
}

fn scan_escape(
    cursor: &mut Cursor<'_>,
    start: CursorPosition,
    limit: ResourceLimit,
) -> Result<(), CharacterError> {
    // Consume the backslash introducer.
    consume_checked(cursor, start, limit)?;

    let escaped = match cursor.peek() {
        Some(ch) => ch,
        None => {
            return Err(make_error(
                cursor,
                start,
                CharacterErrorKind::IncompleteEscape,
            ));
        }
    };

    consume_checked(cursor, start, limit)?;

    match escaped {
        // Keep this set aligned with character-literals.g4.
        '\'' | '"' | '\\' | 'b' | 'f' | 'n' | 'r' | 't' | 'v' | '0' => Ok(()),
        'u' => scan_unicode_escape(cursor, start, limit),
        _ => Err(make_error(
            cursor,
            start,
            CharacterErrorKind::InvalidEscape,
        )),
    }
}

fn scan_unicode_escape(
    cursor: &mut Cursor<'_>,
    start: CursorPosition,
    limit: ResourceLimit,
) -> Result<(), CharacterError> {
    match cursor.peek() {
        Some('{') => {
            consume_checked(cursor, start, limit)?;

            let mut digit_count = 0usize;

            loop {
                match cursor.peek() {
                    Some('}') => {
                        if digit_count == 0 {
                            consume_checked(cursor, start, limit)?;
                            return Err(make_error(
                                cursor,
                                start,
                                CharacterErrorKind::InvalidUnicodeEscape,
                            ));
                        }

                        consume_checked(cursor, start, limit)?;
                        return Ok(());
                    }
                    Some(ch) if ch.is_ascii_hexdigit() => {
                        consume_checked(cursor, start, limit)?;
                        digit_count = digit_count.checked_add(1).ok_or_else(|| {
                            make_error(cursor, start, CharacterErrorKind::CounterOverflow)
                        })?;
                    }
                    Some(_) | None => {
                        return Err(make_error(
                            cursor,
                            start,
                            CharacterErrorKind::InvalidUnicodeEscape,
                        ));
                    }
                }
            }
        }
        Some(ch) if ch.is_ascii_hexdigit() => {
            // The fixed-width form is exactly four hexadecimal digits.
            for _ in 0..4 {
                match cursor.peek() {
                    Some(digit) if digit.is_ascii_hexdigit() => {
                        consume_checked(cursor, start, limit)?;
                    }
                    Some(_) | None => {
                        return Err(make_error(
                            cursor,
                            start,
                            CharacterErrorKind::InvalidUnicodeEscape,
                        ));
                    }
                }
            }

            Ok(())
        }
        Some(_) | None => Err(make_error(
            cursor,
            start,
            CharacterErrorKind::InvalidUnicodeEscape,
        )),
    }
}

fn consume_checked(
    cursor: &mut Cursor<'_>,
    start: CursorPosition,
    limit: ResourceLimit,
) -> Result<char, CharacterError> {
    let ch = cursor.consume().ok_or_else(|| {
        make_error(cursor, start, CharacterErrorKind::Unterminated)
    })?;

    let observed = cursor
        .byte_offset()
        .checked_sub(start.byte_offset())
        .ok_or_else(|| make_error(cursor, start, CharacterErrorKind::CounterOverflow))?;

    if !limit.permits(observed) {
        return Err(make_error(
            cursor,
            start,
            CharacterErrorKind::LiteralBudgetExceeded,
        ));
    }

    Ok(ch)
}

fn make_error(
    cursor: &Cursor<'_>,
    start: CursorPosition,
    kind: CharacterErrorKind,
) -> CharacterError {
    let end = cursor.position();
    let observed_bytes = end
        .byte_offset()
        .checked_sub(start.byte_offset())
        .unwrap_or(usize::MAX);

    CharacterError::new(
        kind,
        start.byte_offset()..end.byte_offset(),
        start,
        end,
        observed_bytes,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scan(source: &str) -> Result<Option<CharacterLiteral<'_>>, CharacterError> {
        let mut cursor = Cursor::new(source);
        scan_character(&mut cursor)
    }

    #[test]
    fn recognizes_ascii_and_unicode_characters() {
        for source in ["'a'", "'Z'", "'0'", "' '", "'λ'", "'→'", "'😀'"] {
            let literal = scan(source).expect("valid literal").expect("literal found");
            assert_eq!(literal.spelling(), source);
            assert_eq!(literal.byte_range(), 0..source.len());
            assert!(!literal.uses_escape());
        }
    }

    #[test]
    fn recognizes_documented_simple_escapes() {
        for source in [
            r"'\''",
            r"'\"'",
            r"'\\'",
            r"'\b'",
            r"'\f'",
            r"'\n'",
            r"'\r'",
            r"'\t'",
            r"'\v'",
            r"'\0'",
        ] {
            let literal = scan(source).expect("valid escape").expect("literal found");
            assert_eq!(literal.spelling(), source);
            assert!(literal.uses_escape());
        }
    }

    #[test]
    fn recognizes_fixed_and_braced_unicode_escape_syntax() {
        for source in [
            r"'\u0000'",
            r"'\u0041'",
            r"'\u03BB'",
            r"'\u{0}'",
            r"'\u{41}'",
            r"'\u{03BB}'",
            r"'\u{1F600}'",
            r"'\u{00000000000000000000000041}'",
        ] {
            let literal = scan(source).expect("valid Unicode syntax").expect("literal found");
            assert_eq!(literal.spelling(), source);
            assert!(literal.uses_escape());
        }
    }

    #[test]
    fn does_not_consume_when_not_at_a_character_literal() {
        let mut cursor = Cursor::new("value");
        let before = cursor.checkpoint();

        assert!(scan_character(&mut cursor).expect("not a literal").is_none());
        assert_eq!(cursor.byte_offset(), before.byte_offset());
        assert_eq!(cursor.position(), before.position());
    }

    #[test]
    fn rejects_empty_and_multiple_character_literals() {
        let error = scan("''").expect_err("empty literal must fail");
        assert_eq!(error.kind(), CharacterErrorKind::Empty);

        let error = scan("'ab'").expect_err("multiple characters must fail");
        assert_eq!(error.kind(), CharacterErrorKind::MultipleCharacters);

        let error = scan("'😀😀'").expect_err("multiple scalars must fail");
        assert_eq!(error.kind(), CharacterErrorKind::MultipleCharacters);
    }

    #[test]
    fn rejects_unknown_and_incomplete_escapes() {
        for source in [r"'\q'", r"'\x41'", r"'\123'"] {
            let error = scan(source).expect_err("unsupported escape must fail");
            assert_eq!(error.kind(), CharacterErrorKind::InvalidEscape);
        }

        let error = scan("'\\").expect_err("trailing backslash must fail");
        assert_eq!(error.kind(), CharacterErrorKind::IncompleteEscape);
    }

    #[test]
    fn rejects_malformed_unicode_escapes() {
        for source in [
            r"'\u'",
            r"'\u1'",
            r"'\u12'",
            r"'\u123'",
            r"'\u{'",
            r"'\u{}'",
            r"'\u{XYZ}'",
            r"'\u{41'",
        ] {
            let error = scan(source).expect_err("malformed Unicode escape must fail");
            assert_eq!(error.kind(), CharacterErrorKind::InvalidUnicodeEscape);
        }
    }

    #[test]
    fn rejects_unterminated_and_raw_line_breaks() {
        for source in ["'", "'a", "'\n'", "'\r'"] {
            assert!(scan(source).is_err(), "source should fail: {source:?}");
        }

        let error = scan("'a").expect_err("unterminated literal must fail");
        assert_eq!(error.kind(), CharacterErrorKind::Unterminated);

        let error = scan("'\n'").expect_err("raw newline must fail");
        assert_eq!(error.kind(), CharacterErrorKind::InvalidRawCharacter);
    }

    #[test]
    fn configured_byte_budget_includes_both_delimiters() {
        let mut cursor = Cursor::new("'a'");
        let limit = ResourceLimit::at_most(2).expect("nonzero limit");

        let error = scan_character_with_limit(&mut cursor, limit)
            .expect_err("three-byte literal must exceed a two-byte budget");

        assert_eq!(error.kind(), CharacterErrorKind::LiteralBudgetExceeded);
        assert_eq!(cursor.byte_offset(), 0, "error must restore cursor");
    }

    #[test]
    fn exact_budget_is_permitted() {
        let mut cursor = Cursor::new("'a'");
        let limit = ResourceLimit::at_most(3).expect("nonzero limit");

        let literal = scan_character_with_limit(&mut cursor, limit)
            .expect("within budget")
            .expect("literal found");

        assert_eq!(literal.spelling(), "'a'");
        assert_eq!(cursor.byte_offset(), 3);
    }

    #[test]
    fn errors_restore_the_complete_cursor_position() {
        let source = "before\n'\\q' after";
        let mut cursor = Cursor::new(source);

        for _ in 0..7 {
            cursor.consume();
        }

        let before = cursor.checkpoint();
        let error = scan_character(&mut cursor).expect_err("bad escape must fail");

        assert_eq!(error.kind(), CharacterErrorKind::InvalidEscape);
        assert_eq!(cursor.position(), before.position());
        assert_eq!(cursor.byte_offset(), before.byte_offset());
    }

    #[test]
    fn preserves_multibyte_source_ranges() {
        let source = "'😀'";
        let literal = scan(source).expect("valid").expect("literal found");

        assert_eq!(literal.byte_len(), source.len());
        assert_eq!(literal.byte_range(), 0..source.len());
        assert_eq!(literal.end().byte_offset(), source.len());
    }

    #[test]
    fn escaped_numeric_value_is_not_semantically_decoded_here() {
        // The lexical layer accepts the escape's syntax. A later semantic
        // stage must reject values that do not denote a Unicode scalar.
        let literal = scan(r"'\u{110000}'")
            .expect("syntactically valid escape")
            .expect("literal found");

        assert_eq!(literal.spelling(), r"'\u{110000}'");
    }
}
