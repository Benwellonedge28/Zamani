
#![forbid(unsafe_code)]

//! String-literal recognition for the Zamani frontend.
//!
//! Rust baseline: 1.97 or later
//! Edition: 2021
//!
//! # Ownership
//!
//! This module recognizes the source spelling of ordinary, double-quoted
//! string literals. It validates escape syntax, preserves source spelling,
//! and reports malformed strings without decoding their semantic values.
//!
//! It does not own string types, escape decoding, allocation policy for
//! semantic values, interpolation, raw strings, byte strings, AST nodes,
//! or target-specific representations.
//!
//! # Grammar authority
//!
//! - `grammar/spec/lexical.md`
//! - `grammar/lexer/string-literals.g4`
//! - `grammar/lexer/literals.g4`
//! - `grammar/antlr/ZamaniLexer.g4`
//!
//! These files must agree on delimiters, escape forms, control characters,
//! Unicode escape syntax, and malformed-input behavior.
//!
//! # Integration
//!
//! `scanner.rs` calls `scan_string_with_limit` when the next character is
//! a double quote. It maps successful results to `TokenKind::StringLiteral`
//! and constructs a token with the exact spelling and source span.
//!
//! It maps `StringErrorKind` to the canonical diagnostic representation
//! in `diagnostics.rs`. Recovery policy belongs to `recovery.rs`.
//!
//! `config.rs` supplies `LexerLimits::literal_bytes`. The limit is an
//! implementation resource policy, not a language-level string maximum.
//!
//! Semantic literal processing later decodes the spelling and validates
//! Unicode scalar values. This module deliberately does not construct an
//! owned `String` or silently normalize or rewrite source text.

use core::fmt;
use core::ops::Range;

use super::config::ResourceLimit;
use super::cursor::{Cursor, CursorPosition};

/// A successfully recognized ordinary string literal.
///
/// The spelling borrows directly from the source and includes both
/// quotation-mark delimiters. The source is never decoded or modified.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StringLiteral<'src> {
    source: &'src str,
    byte_range: Range<usize>,
    start: CursorPosition,
    end: CursorPosition,
    content_range: Range<usize>,
    escape_count: usize,
}

impl<'src> StringLiteral<'src> {
    fn new(
        source: &'src str,
        byte_range: Range<usize>,
        start: CursorPosition,
        end: CursorPosition,
        content_range: Range<usize>,
        escape_count: usize,
    ) -> Self {
        Self {
            source,
            byte_range,
            start,
            end,
            content_range,
            escape_count,
        }
    }

    /// Returns the exact source spelling, including the delimiters.
    #[must_use]
    pub fn spelling(&self) -> &'src str {
        &self.source[self.byte_range.clone()]
    }

    /// Returns the source contents between the delimiters.
    ///
    /// Escapes remain in their original source spelling.
    #[must_use]
    pub fn raw_content(&self) -> &'src str {
        &self.source[self.content_range.clone()]
    }

    /// Returns the half-open source byte range of the entire literal.
    #[must_use]
    pub fn byte_range(&self) -> Range<usize> {
        self.byte_range.clone()
    }

    /// Returns the half-open source byte range of the content.
    #[must_use]
    pub fn content_range(&self) -> Range<usize> {
        self.content_range.clone()
    }

    /// Returns the starting cursor position.
    #[must_use]
    pub const fn start(&self) -> CursorPosition {
        self.start
    }

    /// Returns the ending cursor position, immediately after the closing
    /// delimiter.
    #[must_use]
    pub const fn end(&self) -> CursorPosition {
        self.end
    }

    /// Returns the literal's source byte length, including delimiters.
    #[must_use]
    pub fn byte_len(&self) -> usize {
        self.byte_range.end - self.byte_range.start
    }

    /// Returns the number of escape sequences recognized.
    ///
    /// This counts escape introducers, not decoded characters or Unicode
    /// scalar values. Unicode escapes each count as one sequence.
    #[must_use]
    pub const fn escape_count(&self) -> usize {
        self.escape_count
    }

    /// Returns whether the source content contains no characters.
    ///
    /// An escaped character is still content.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.content_range.start == self.content_range.end
    }
}

/// Why recognition of a string literal failed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum StringErrorKind {
    /// A string began but reached EOF without a closing quote.
    Unterminated,

    /// A backslash was the final character or was not followed by an escape.
    IncompleteEscape,

    /// The escape letter is not in the supported escape table.
    InvalidEscape,

    /// A Unicode escape did not match a supported lexical form.
    InvalidUnicodeEscape,

    /// A raw control character or line separator occurred in the string.
    InvalidRawCharacter,

    /// The source spelling exceeded the configured literal-byte budget.
    LiteralBudgetExceeded,

    /// A checked position or counter could not represent the observed value.
    CounterOverflow,
}

/// A structured string-literal recognition error.
///
/// Byte ranges are zero-based and half-open. Errors restore the cursor to
/// its starting checkpoint so that scanner recovery can apply one
/// consistent policy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StringError {
    kind: StringErrorKind,
    byte_range: Range<usize>,
    start: CursorPosition,
    end: CursorPosition,
    observed_bytes: usize,
}

impl StringError {
    fn new(
        kind: StringErrorKind,
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
    pub const fn kind(&self) -> StringErrorKind {
        self.kind
    }

    /// Returns the source byte range associated with the error.
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

    /// Returns the number of source bytes observed when the error occurred.
    #[must_use]
    pub const fn observed_bytes(&self) -> usize {
        self.observed_bytes
    }

    /// Returns whether the failure represents a resource-policy or
    /// implementation-counter limit rather than invalid source syntax.
    #[must_use]
    pub const fn is_resource_exhaustion(&self) -> bool {
        matches!(
            self.kind,
            StringErrorKind::LiteralBudgetExceeded
                | StringErrorKind::CounterOverflow
        )
    }
}

impl fmt::Display for StringError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            StringErrorKind::Unterminated => {
                formatter.write_str("unterminated string literal")
            }
            StringErrorKind::IncompleteEscape => {
                formatter.write_str("incomplete escape sequence")
            }
            StringErrorKind::InvalidEscape => {
                formatter.write_str("unsupported string escape sequence")
            }
            StringErrorKind::InvalidUnicodeEscape => {
                formatter.write_str("invalid Unicode escape syntax")
            }
            StringErrorKind::InvalidRawCharacter => {
                formatter.write_str(
                    "raw control character or line separator in string literal",
                )
            }
            StringErrorKind::LiteralBudgetExceeded => {
                write!(
                    formatter,
                    "string literal exceeded the configured byte budget \
                     after observing {} bytes",
                    self.observed_bytes
                )
            }
            StringErrorKind::CounterOverflow => {
                formatter.write_str(
                    "string scanning counter exceeded the implementation's \
                     representable range",
                )
            }
        }
    }
}

impl std::error::Error for StringError {}

/// Recognize a string literal without applying an explicit byte budget.
///
/// This function preserves the source spelling and returns structured
/// syntax errors. For untrusted source input, prefer
/// [`scan_string_with_limit`] with a configured budget.
///
/// Returns `Ok(None)` when the cursor is not positioned at a double quote.
pub fn scan_string<'src>(
    cursor: &mut Cursor<'src>,
) -> Result<Option<StringLiteral<'src>>, StringError> {
    scan_string_with_limit(cursor, ResourceLimit::Unlimited)
}

/// Recognize an ordinary double-quoted string literal under a configurable
/// source-byte budget.
///
/// The budget includes both quotation marks and all escape syntax.
/// `ResourceLimit::Unlimited` means no explicit literal-byte budget is
/// applied; actual memory, address-space, and execution resources still
/// constrain processing.
///
/// On error, the cursor is restored to its original checkpoint. This makes
/// recovery deterministic and prevents this function from leaving a
/// partially consumed literal behind.
///
/// Supported escape syntax follows the string-literal grammar:
///
/// - `\'`, `\"`, `\\`
/// - `\b`, `\f`, `\n`, `\r`, `\t`, `\v`, `\0`
/// - `\uXXXX`
/// - `\u{HEX_DIGITS}`
///
/// The exact supported table must remain synchronized with the normative
/// lexical specification. Unicode scalar-value validation and decoding
/// are downstream responsibilities.
pub fn scan_string_with_limit<'src>(
    cursor: &mut Cursor<'src>,
    byte_limit: ResourceLimit,
) -> Result<Option<StringLiteral<'src>>, StringError> {
    if cursor.peek() != Some('"') {
        return Ok(None);
    }

    let checkpoint = cursor.checkpoint();
    let source = cursor.source();
    let start = cursor.position();
    let start_offset = cursor.byte_offset();

    let result = scan_inner(
        cursor,
        source,
        start,
        start_offset,
        byte_limit,
    );

    if result.is_err() {
        // The checkpoint was obtained from this cursor.
        // Restore before returning so recovery owns the next action.
        let _ = cursor.restore(checkpoint);
    }

    result
}

fn scan_inner<'src>(
    cursor: &mut Cursor<'src>,
    source: &'src str,
    start: CursorPosition,
    start_offset: usize,
    byte_limit: ResourceLimit,
) -> Result<Option<StringLiteral<'src>>, StringError> {
    cursor.consume(); // opening quote

    check_byte_budget(
        cursor,
        start,
        start_offset,
        byte_limit,
    )?;

    let content_start = cursor.byte_offset();
    let mut escape_count = 0usize;

    loop {
        match cursor.peek() {
            None => {
                return Err(error_at(
                    cursor,
                    StringErrorKind::Unterminated,
                    start,
                    start_offset,
                ));
            }

            Some('"') => {
                let content_end = cursor.byte_offset();
                cursor.consume(); // closing quote

                check_byte_budget(
                    cursor,
                    start,
                    start_offset,
                    byte_limit,
                )?;

                let end = cursor.position();
                let end_offset = cursor.byte_offset();

                return Ok(Some(StringLiteral::new(
                    source,
                    start_offset..end_offset,
                    start,
                    end,
                    content_start..content_end,
                    escape_count,
                )));
            }

            Some('\\') => {
                cursor.consume();

                check_byte_budget(
                    cursor,
                    start,
                    start_offset,
                    byte_limit,
                )?;

                consume_escape(
                    cursor,
                    start,
                    start_offset,
                    byte_limit,
                )?;

                escape_count = escape_count.checked_add(1).ok_or_else(|| {
                    error_at(
                        cursor,
                        StringErrorKind::CounterOverflow,
                        start,
                        start_offset,
                    )
                })?;
            }

            Some(ch) if is_forbidden_raw_character(ch) => {
                return Err(error_at(
                    cursor,
                    StringErrorKind::InvalidRawCharacter,
                    start,
                    start_offset,
                ));
            }

            Some(_) => {
                cursor.consume();

                check_byte_budget(
                    cursor,
                    start,
                    start_offset,
                    byte_limit,
                )?;
            }
        }
    }
}

fn consume_escape(
    cursor: &mut Cursor<'_>,
    start: CursorPosition,
    start_offset: usize,
    byte_limit: ResourceLimit,
) -> Result<(), StringError> {
    let escape = match cursor.peek() {
        Some(ch) => ch,
        None => {
            return Err(error_at(
                cursor,
                StringErrorKind::IncompleteEscape,
                start,
                start_offset,
            ));
        }
    };

    cursor.consume();

    check_byte_budget(
        cursor,
        start,
        start_offset,
        byte_limit,
    )?;

    match escape {
        '\'' | '"' | '\\' | 'b' | 'f' | 'n' | 'r' | 't' | 'v' | '0' => {
            Ok(())
        }

        'u' => consume_unicode_escape(
            cursor,
            start,
            start_offset,
            byte_limit,
        ),

        _ => Err(error_at(
            cursor,
            StringErrorKind::InvalidEscape,
            start,
            start_offset,
        )),
    }
}

fn consume_unicode_escape(
    cursor: &mut Cursor<'_>,
    start: CursorPosition,
    start_offset: usize,
    byte_limit: ResourceLimit,
) -> Result<(), StringError> {
    match cursor.peek() {
        Some('{') => {
            cursor.consume();

            check_byte_budget(
                cursor,
                start,
                start_offset,
                byte_limit,
            )?;

            let mut digit_count = 0usize;

            loop {
                match cursor.peek() {
                    Some(ch) if ch.is_ascii_hexdigit() => {
                        cursor.consume();

                        check_byte_budget(
                            cursor,
                            start,
                            start_offset,
                            byte_limit,
                        )?;

                        digit_count = digit_count.checked_add(1).ok_or_else(|| {
                            error_at(
                                cursor,
                                StringErrorKind::CounterOverflow,
                                start,
                                start_offset,
                            )
                        })?;
                    }

                    Some('}') if digit_count > 0 => {
                        cursor.consume();

                        check_byte_budget(
                            cursor,
                            start,
                            start_offset,
                            byte_limit,
                        )?;

                        return Ok(());
                    }

                    _ => {
                        return Err(error_at(
                            cursor,
                            StringErrorKind::InvalidUnicodeEscape,
                            start,
                            start_offset,
                        ));
                    }
                }
            }
        }

        Some(ch) if ch.is_ascii_hexdigit() => {
            // Fixed-width Unicode escape: exactly four hexadecimal digits.
            for _ in 0..4 {
                match cursor.peek() {
                    Some(ch) if ch.is_ascii_hexdigit() => {
                        cursor.consume();

                        check_byte_budget(
                            cursor,
                            start,
                            start_offset,
                            byte_limit,
                        )?;
                    }

                    _ => {
                        return Err(error_at(
                            cursor,
                            StringErrorKind::InvalidUnicodeEscape,
                            start,
                            start_offset,
                        ));
                    }
                }
            }

            Ok(())
        }

        _ => Err(error_at(
            cursor,
            StringErrorKind::InvalidUnicodeEscape,
            start,
            start_offset,
        )),
    }
}

fn is_forbidden_raw_character(ch: char) -> bool {
    // Ordinary strings do not contain raw line breaks or control characters.
    // Escaped spellings may represent such values and are decoded later.
    ch.is_control() || matches!(ch, '\u{2028}' | '\u{2029}')
}

fn check_byte_budget(
    cursor: &Cursor<'_>,
    start: CursorPosition,
    start_offset: usize,
    byte_limit: ResourceLimit,
) -> Result<(), StringError> {
    let observed = cursor
        .byte_offset()
        .checked_sub(start_offset)
        .ok_or_else(|| {
            error_at(
                cursor,
                StringErrorKind::CounterOverflow,
                start,
                start_offset,
            )
        })?;

    if byte_limit.permits(observed) {
        Ok(())
    } else {
        Err(error_at(
            cursor,
            StringErrorKind::LiteralBudgetExceeded,
            start,
            start_offset,
        ))
    }
}

fn error_at(
    cursor: &Cursor<'_>,
    kind: StringErrorKind,
    start: CursorPosition,
    start_offset: usize,
) -> StringError {
    let end_offset = cursor.byte_offset();

    StringError::new(
        kind,
        start_offset..end_offset,
        start,
        cursor.position(),
        end_offset.saturating_sub(start_offset),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frontend::lexer::cursor::Cursor;

    fn scan(source: &str) -> Result<Option<StringLiteral<'_>>, StringError> {
        scan_string(&mut Cursor::new(source))
    }

    #[test]
    fn recognizes_empty_and_nonempty_strings() {
        for source in [r#""""#, r#""hello""#, r#""Zamani""#] {
            let literal = scan(source)
                .expect("recognition should not fail")
                .expect("expected string");

            assert_eq!(literal.spelling(), source);
            assert_eq!(literal.byte_len(), source.len());
        }

        let empty = scan(r#""""#)
            .expect("valid")
            .expect("empty string");

        assert!(empty.is_empty());
        assert_eq!(empty.raw_content(), "");
    }

    #[test]
    fn preserves_raw_content_without_decoding() {
        let source = r#""line\nnext\tend""#;
        let literal = scan(source)
            .expect("valid")
            .expect("string");

        assert_eq!(literal.spelling(), source);
        assert_eq!(literal.raw_content(), r"line\nnext\tend");
        assert_eq!(literal.escape_count(), 2);
    }

    #[test]
    fn accepts_supported_simple_escapes() {
        for source in [
            r#""\'""#,
            r#""\"""#,
            r#""\\""#,
            r#""\b""#,
            r#""\f""#,
            r#""\n""#,
            r#""\r""#,
            r#""\t""#,
            r#""\v""#,
            r#""\0""#,
        ] {
            let literal = scan(source)
                .expect("valid escape")
                .expect("string");

            assert_eq!(literal.spelling(), source);
            assert_eq!(literal.escape_count(), 1);
        }
    }

    #[test]
    fn accepts_fixed_width_unicode_escape_syntax() {
        for source in [r#""\u0041""#, r#""\u03BB""#, r#""\uFFFF""#] {
            let literal = scan(source)
                .expect("valid syntax")
                .expect("string");

            assert_eq!(literal.spelling(), source);
            assert_eq!(literal.escape_count(), 1);
        }
    }

    #[test]
    fn accepts_braced_unicode_escape_syntax() {
        for source in [r#""\u{41}""#, r#""\u{03BB}""#, r#""\u{1F680}""#] {
            let literal = scan(source)
                .expect("valid syntax")
                .expect("string");

            assert_eq!(literal.spelling(), source);
            assert_eq!(literal.escape_count(), 1);
        }
    }

    #[test]
    fn rejects_unknown_escape_sequences() {
        for source in [r#""\q""#, r#""\x41""#, r#""\z""#] {
            let mut cursor = Cursor::new(source);

            let error = scan_string(&mut cursor)
                .expect_err("unknown escape must fail");

            assert_eq!(error.kind(), StringErrorKind::InvalidEscape);
            assert_eq!(cursor.byte_offset(), 0);
        }
    }

    #[test]
    fn rejects_incomplete_escape_sequences() {
        let mut cursor = Cursor::new("\"abc\\");

        let error = scan_string(&mut cursor)
            .expect_err("incomplete escape");

        assert_eq!(error.kind(), StringErrorKind::IncompleteEscape);
        assert_eq!(cursor.byte_offset(), 0);
    }

    #[test]
    fn rejects_invalid_unicode_escape_syntax() {
        for source in [
            r#""\u""#,
            r#""\u12""#,
            r#""\u12G4""#,
            r#""\u{}""#,
            r#""\u{G}""#,
            r#""\u{12""#,
        ] {
            let mut cursor = Cursor::new(source);

            let error = scan_string(&mut cursor)
                .expect_err("invalid Unicode escape");

            assert_eq!(
                error.kind(),
                StringErrorKind::InvalidUnicodeEscape
            );
            assert_eq!(cursor.byte_offset(), 0);
        }
    }

    #[test]
    fn rejects_unterminated_strings() {
        for source in ["\"", "\"hello", "\"hello\\n", "\"hello\\"] {
            let mut cursor = Cursor::new(source);

            let error = scan_string(&mut cursor)
                .expect_err("unterminated string");

            assert!(
                matches!(
                    error.kind(),
                    StringErrorKind::Unterminated
                        | StringErrorKind::IncompleteEscape
                ),
                "unexpected error for {source:?}: {:?}",
                error.kind()
            );

            assert_eq!(cursor.byte_offset(), 0);
        }
    }

    #[test]
    fn rejects_raw_newlines_and_control_characters() {
        for source in ["\"a\nb\"", "\"a\rb\"", "\"a\tb\"", "\"a\u{2028}b\""] {
            let mut cursor = Cursor::new(source);

            let error = scan_string(&mut cursor)
                .expect_err("raw control or line separator");

            assert_eq!(
                error.kind(),
                StringErrorKind::InvalidRawCharacter
            );
            assert_eq!(cursor.byte_offset(), 0);
        }
    }

    #[test]
    fn unicode_source_content_is_preserved() {
        let source = "\"λ 世界 🛰️\"";
        let literal = scan(source)
            .expect("valid")
            .expect("string");

        assert_eq!(literal.spelling(), source);
        assert_eq!(literal.raw_content(), "λ 世界 🛰️");
    }

    #[test]
    fn does_not_consume_non_string_input() {
        for source in ["", "name", "'x'", "123", " ", "\\\""] {
            let mut cursor = Cursor::new(source);

            let result = scan_string(&mut cursor)
                .expect("not a string error");

            assert!(result.is_none());
            assert_eq!(cursor.byte_offset(), 0);
        }
    }

    #[test]
    fn byte_budget_includes_delimiters() {
        let source = "\"abc\"";
        let mut cursor = Cursor::new(source);
        let limit = ResourceLimit::at_most(4).expect("nonzero limit");

        let error = scan_string_with_limit(&mut cursor, limit)
            .expect_err("five-byte literal exceeds four-byte budget");

        assert_eq!(error.kind(), StringErrorKind::LiteralBudgetExceeded);
        assert!(error.is_resource_exhaustion());
        assert_eq!(cursor.byte_offset(), 0);
    }

    #[test]
    fn byte_budget_is_inclusive() {
        let source = "\"abc\"";
        let mut cursor = Cursor::new(source);
        let limit = ResourceLimit::at_most(source.len())
            .expect("nonzero limit");

        let literal = scan_string_with_limit(&mut cursor, limit)
            .expect("within budget")
            .expect("string");

        assert_eq!(literal.spelling(), source);
    }

    #[test]
    fn no_explicit_budget_means_no_lexer_defined_length_ceiling() {
        let content = "λ".repeat(4096);
        let source = format!("\"{content}\"");
        let mut cursor = Cursor::new(&source);

        let literal = scan_string_with_limit(
            &mut cursor,
            ResourceLimit::Unlimited,
        )
        .expect("valid string")
        .expect("string");

        assert_eq!(literal.raw_content(), content);
        assert_eq!(literal.byte_len(), source.len());
    }

    #[test]
    fn positions_account_for_utf8_bytes() {
        let source = "λ \"hi\"";
        let mut cursor = Cursor::new(source);
        cursor.consume();
        cursor.consume();

        let literal = scan_string(&mut cursor)
            .expect("valid")
            .expect("string");

        assert_eq!(literal.spelling(), "\"hi\"");
        assert_eq!(literal.byte_range(), 3..7);
        assert_eq!(literal.start().byte_offset(), 3);
        assert_eq!(literal.end().byte_offset(), 7);
    }

    #[test]
    fn errors_restore_cursor_state() {
        let source = "\"bad\\q\" next";
        let mut cursor = Cursor::new(source);
        let initial = cursor.position();

        let _ = scan_string(&mut cursor)
            .expect_err("bad escape");

        assert_eq!(cursor.position(), initial);
        assert_eq!(cursor.remaining(), source);
    }
}
