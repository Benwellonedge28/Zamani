
#![forbid(unsafe_code)]

//! Unicode-aware identifier recognition for the Zamani frontend.
//!
//! File: src/frontend/lexer/identifier.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! # Purpose
//!
//! Recognize the lexical shape of Zamani identifiers without coupling
//! identifier syntax to keywords, dialects, quantum operations, HDL,
//! hardware vendors, semantic name resolution, or execution targets.
//!
//! # Normative grammar
//!
//! `grammar/lexer/identifiers.g4`
//!
//! The corresponding lexical contract is defined by:
//!
//! `grammar/spec/lexical.md`
//!
//! # Character rules
//!
//! Start:
//! - Unicode general category L (all letter categories);
//! - Unicode general category Nl (letter numbers);
//! - underscore.
//!
//! Continuation:
//! - any valid start character;
//! - Mn (nonspacing marks);
//! - Mc (spacing combining marks);
//! - Nd (decimal digits);
//! - Pc (connector punctuation).
//!
//! These rules intentionally do not accept arbitrary symbols, whitespace,
//! format controls, joiners, or punctuation as identifier characters.
//!
//! # Integration
//!
//! - `cursor.rs`: UTF-8-safe traversal and source positions.
//! - `token_kind.rs`: canonical `TokenKind::Identifier`.
//! - `token.rs`: original spelling and source span.
//! - `keyword_registry.rs`: reserved/contextual keyword classification,
//!   performed after identifier recognition.
//! - `dialect.rs`: active lexical extension policy.
//! - `config.rs`: lexical configuration and resource budgets.
//! - `diagnostics.rs`: malformed input and resource diagnostics.
//! - `recovery.rs`: recovery after lexical errors.
//! - `scanner.rs`: orchestration and token emission.
//! - `grammar/lexer/identifiers.g4`: normative character categories.
//!
//! This module does not construct tokens, classify keywords, normalize
//! Unicode, or resolve names.
//!
//! # Scalability
//!
//! There is no fixed identifier-length constant. The caller may enforce a
//! configurable source/token budget. Actual capacity is constrained by
//! available memory, address space, and execution resources.
//!
//! # Unicode dependency
//!
//! Add `unicode-general-category = "1"` to Cargo.toml. This supplies Unicode
//! general-category classification rather than an ASCII-only approximation.

use core::ops::Range;

use unicode_general_category::{get_general_category, GeneralCategory};

use super::cursor::{Cursor, CursorPosition};

/// The source range and spelling of a recognized identifier.
///
/// Byte ranges are half-open: `start..end`.
/// The spelling borrows directly from the original source.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Identifier<'src> {
    spelling: &'src str,
    byte_range: Range<usize>,
    start: CursorPosition,
    end: CursorPosition,
}

impl<'src> Identifier<'src> {
    /// Creates an identifier result from a recognized source slice.
    ///
    /// This constructor is intended for this module's scanner and tests.
    /// The caller must ensure the slice and positions describe the same
    /// source region and that the spelling satisfies the identifier rules.
    #[must_use]
    pub const fn new(
        spelling: &'src str,
        byte_range: Range<usize>,
        start: CursorPosition,
        end: CursorPosition,
    ) -> Self {
        Self {
            spelling,
            byte_range,
            start,
            end,
        }
    }

    /// Returns the original identifier spelling.
    #[must_use]
    pub const fn spelling(&self) -> &'src str {
        self.spelling
    }

    /// Returns the half-open UTF-8 byte range.
    #[must_use]
    pub fn byte_range(&self) -> Range<usize> {
        self.byte_range.clone()
    }

    /// Returns the identifier's starting source position.
    #[must_use]
    pub const fn start(&self) -> CursorPosition {
        self.start
    }

    /// Returns the source position immediately after the identifier.
    #[must_use]
    pub const fn end(&self) -> CursorPosition {
        self.end
    }

    /// Returns the byte length of the identifier spelling.
    #[must_use]
    pub const fn byte_len(&self) -> usize {
        self.byte_range.end - self.byte_range.start
    }

    /// Returns whether the identifier spelling is empty.
    ///
    /// Valid identifiers are never empty. This method is useful when
    /// validating results received across module boundaries.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.spelling.is_empty()
    }
}

/// Returns whether `ch` is a valid identifier-start character.
///
/// This implements the categories in `identifiers.g4`:
/// underscore, Unicode letters, and Unicode letter numbers.
///
/// It does not apply keyword rules or Unicode normalization.
#[must_use]
pub fn is_identifier_start(ch: char) -> bool {
    if ch == '_' {
        return true;
    }

    matches!(
        get_general_category(ch),
        GeneralCategory::UppercaseLetter
            | GeneralCategory::LowercaseLetter
            | GeneralCategory::TitlecaseLetter
            | GeneralCategory::ModifierLetter
            | GeneralCategory::OtherLetter
            | GeneralCategory::LetterNumber
    )
}

/// Returns whether `ch` may continue an identifier.
///
/// The continuation set is the union of the start set and the Unicode
/// categories Mn, Mc, Nd, and Pc specified by the canonical grammar.
#[must_use]
pub fn is_identifier_continue(ch: char) -> bool {
    is_identifier_start(ch)
        || matches!(
            get_general_category(ch),
            GeneralCategory::NonspacingMark
                | GeneralCategory::SpacingMark
                | GeneralCategory::DecimalNumber
                | GeneralCategory::ConnectorPunctuation
        )
}

/// Tests whether an entire string is a valid identifier.
///
/// An empty string is not an identifier. A leading decimal digit is
/// rejected, even when subsequent characters would be valid.
///
/// This function does not check whether the spelling is reserved.
#[must_use]
pub fn is_identifier(spelling: &str) -> bool {
    let mut chars = spelling.chars();

    let Some(first) = chars.next() else {
        return false;
    };

    is_identifier_start(first) && chars.all(is_identifier_continue)
}

/// Scans one identifier from the current cursor position.
///
/// Returns `None` without consuming input if the next character cannot
/// begin an identifier.
///
/// On success, consumes the complete identifier and returns a borrowed
/// spelling plus its byte range and source positions.
///
/// Keyword recognition intentionally remains the responsibility of
/// `scanner.rs`, which can consult the active keyword registry after this
/// function succeeds.
///
/// # Progress guarantee
///
/// A successful call consumes at least one Unicode scalar value.
/// A failed call consumes nothing.
///
/// # UTF-8 guarantee
///
/// Source slicing uses cursor byte offsets produced at character
/// boundaries. No unsafe code or byte-wise UTF-8 traversal is used.
pub fn scan_identifier<'src>(
    cursor: &mut Cursor<'src>,
) -> Option<Identifier<'src>> {
    let first = cursor.peek()?;

    if !is_identifier_start(first) {
        return None;
    }

    let start = cursor.position();
    let start_offset = cursor.byte_offset();

    // The initial character is known to be valid.
    let _ = cursor.consume();

    while let Some(ch) = cursor.peek() {
        if !is_identifier_continue(ch) {
            break;
        }

        let _ = cursor.consume();
    }

    let end = cursor.position();
    let end_offset = cursor.byte_offset();

    // Cursor offsets are maintained at UTF-8 boundaries.
    // The source slice is borrowed, so ordinary identifiers need no
    // allocation or source-text normalization.
    let spelling = &cursor.source()[start_offset..end_offset];

    Some(Identifier::new(
        spelling,
        start_offset..end_offset,
        start,
        end,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_ascii_identifiers() {
        for spelling in [
            "_",
            "_value",
            "value",
            "Value",
            "VALUE",
            "value123",
            "a_b_c",
        ] {
            assert!(
                is_identifier(spelling),
                "expected valid identifier: {spelling:?}"
            );
        }
    }

    #[test]
    fn rejects_empty_and_digit_started_identifiers() {
        for spelling in ["", "0", "123", "123value"] {
            assert!(
                !is_identifier(spelling),
                "expected invalid identifier: {spelling:?}"
            );
        }
    }

    #[test]
    fn accepts_unicode_letters_and_letter_numbers() {
        for spelling in ["café", "λ", "变量", "नमस्ते", "Ⅻname"] {
            assert!(
                is_identifier(spelling),
                "expected valid Unicode identifier: {spelling:?}"
            );
        }
    }

    #[test]
    fn accepts_unicode_continuation_categories() {
        // Combining marks, decimal digits, and connector punctuation
        // are continuation characters under the canonical grammar.
        assert!(is_identifier("a\u{0301}"));
        assert!(is_identifier("a١"));
        assert!(is_identifier("a\u{203F}"));
    }

    #[test]
    fn rejects_invalid_start_characters() {
        for ch in ['0', '١', ' ', '\n', '-', '.', '!', '\u{0301}'] {
            assert!(
                !is_identifier_start(ch),
                "unexpected valid start character: {ch:?}"
            );
        }
    }

    #[test]
    fn does_not_accept_format_joiners() {
        assert!(!is_identifier("a\u{200C}b"));
        assert!(!is_identifier("a\u{200D}b"));
    }

    #[test]
    fn preserves_case_and_original_spelling() {
        assert_ne!(is_identifier("value"), is_identifier("Value"));

        let source = "value Value";
        let mut cursor = Cursor::new(source);

        let first = scan_identifier(&mut cursor).expect("first identifier");
        assert_eq!(first.spelling(), "value");
        assert_eq!(first.byte_range(), 0..5);

        // This recognizer does not consume the separator.
        assert_eq!(cursor.peek(), Some(' '));

        let _ = cursor.consume();

        let second = scan_identifier(&mut cursor).expect("second identifier");
        assert_eq!(second.spelling(), "Value");
        assert_eq!(second.byte_range(), 6..11);
    }

    #[test]
    fn returns_none_without_consuming_invalid_input() {
        let mut cursor = Cursor::new("123name");

        let before = cursor.position();
        assert!(scan_identifier(&mut cursor).is_none());
        assert_eq!(cursor.position(), before);
    }

    #[test]
    fn scans_multibyte_unicode_without_corrupting_offsets() {
        let source = "λ变量42!";
        let mut cursor = Cursor::new(source);

        let ident = scan_identifier(&mut cursor).expect("Unicode identifier");

        assert_eq!(ident.spelling(), "λ变量42");
        assert_eq!(
            &source[ident.byte_range()],
            "λ变量42"
        );
        assert_eq!(cursor.peek(), Some('!'));
    }

    #[test]
    fn does_not_classify_domain_names_as_keywords() {
        for spelling in [
            "H",
            "X",
            "CNOT",
            "custom_operation",
            "qpu_name",
            "gpu",
            "vendor_gate",
            "tensor_compute",
            "hdl_module",
        ] {
            assert!(
                is_identifier(spelling),
                "domain vocabulary must remain extensible: {spelling}"
            );
        }
    }

    #[test]
    fn rejects_punctuation_and_whitespace_inside_identifiers() {
        for spelling in ["a-b", "a.b", "a b", "a/b", "a:b"] {
            assert!(
                !is_identifier(spelling),
                "unexpected valid identifier: {spelling:?}"
            );
        }
    }
}
