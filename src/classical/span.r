#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! Zamani Classical Computing — Source-Span Integration
//!
//! Path: `src/classical/span.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Expose the repository's existing source-location model to classical
//! computing and provide checked operations for validating source ranges.
//!
//! This module is deliberately a facade. It does not introduce a competing
//! Span, FileId, BytePos, SourceFile, or SourceMap type.
//!
//! # Authority
//!
//! Normative contracts:
//! - `grammar/spec/source-spans.md`
//! - `grammar/spec/source-map.md`
//! - `grammar/validation/source-spans.md`
//! - `grammar/spec/diagnostics.md`
//!
//! Executable source-location infrastructure:
//! - `crate::source_map`
//!
//! Existing consumers:
//! - `crate::lexer`
//! - `crate::parser`
//! - `crate::ast`
//!
//! Canonical IR provenance:
//! - `crate::zuir::source_map`
//!
//! The legacy frontend and ZUIR source-map representations must be
//! reconciled through an explicit adapter or migration. This module must
//! not silently convert between their different identity and offset models.
//!
//! # Owns
//!
//! - The classical-facing source-span import boundary.
//! - Checked byte-range extraction.
//! - Validation of span ordering and source bounds.
//! - Validation of UTF-8 character boundaries when source text is available.
//! - Explicit validation against a registered source file.
//!
//! # Does not own
//!
//! - Source-file identity allocation.
//! - Source text storage or file loading.
//! - The canonical Span or BytePos representation.
//! - Line-index construction or line/column conversion.
//! - Lexical or syntactic rules.
//! - AST, semantic-model, or IR entity identity.
//! - Diagnostic rendering or recovery policy.
//! - Classical execution, hardware selection, or resource scheduling.
//!
//! # Range contract
//!
//! Source spans use half-open byte ranges `[start, end)`.
//! Byte offsets are measured in UTF-8 bytes, not Unicode characters,
//! grapheme clusters, display columns, or UTF-16 code units.
//!
//! Empty ranges are valid, including EOF and insertion positions.
//! A range must not be reversed, extend past the source text, or split a
//! UTF-8 encoded character.
//!
//! A dummy span denotes absent source provenance. It must not be rendered
//! as though it identifies a real location.
//!
//! # Scalability and POCO-REAF
//!
//! This module imposes no hardware, program-size, token-count, or collection
//! limit. Its validation work is constant-time apart from the constant-time
//! UTF-8 boundary checks provided by `str`.
//!
//! The existing `crate::source_map::BytePos` uses `u32`; consequently, this
//! facade does not claim that the current legacy source-map implementation
//! can represent every source supported by the normative specification.
//! A wider canonical offset representation requires a coordinated migration
//! of the source map and its lexer, parser, and AST consumers.
//!
//! Literal infinity is not a realizable storage guarantee. Implementations
//! must report representation or resource exhaustion rather than wrap,
//! truncate, or silently reinterpret a source location.
//!
//! # Integration contract
//!
//! 1. `crate::source_map` remains the owner of legacy source-location types.
//! 2. Classical modules import these types from `classical::span` or directly
//!    from their canonical owner; they must not define replacements.
//! 3. `crate::lexer`, `crate::parser`, and `crate::ast` retain their existing
//!    source-map contract until an explicit compatibility migration.
//! 4. ZUIR provenance uses `crate::zuir::source_map::SourceSpan`. Conversion
//!    between that model and the legacy `Span` requires a source-ID mapping
//!    and checked offset conversion; numeric IDs must not be assumed equal.
//! 5. Callers validate source-backed spans before slicing source text.
//! 6. Diagnostics and transformations preserve source identity and byte
//!    ranges. Missing or synthetic provenance must remain explicit.
//!
//! # Freeze criteria
//!
//! This file is ready to freeze when its module is registered, its imports
//! compile against the repository, its tests pass, and its consumers follow
//! the ownership rules above. Repository-wide production readiness additionally
//! requires the legacy source-map limitations to be resolved and the full
//! compiler test suite to pass.

use core::fmt;
use core::ops::Range;

/// Re-export the canonical legacy source-location types.
///
/// These are the original types from `crate::source_map`, not new classical
/// definitions. Keeping the types identical avoids incompatible spans
/// between the existing lexer, parser, AST, and classical integration.
pub use crate::source_map::{BytePos, FileId, SourceFile, SourceMap, Span};

/// Errors returned by checked source-span operations.
///
/// This type describes validation failures only. It does not replace the
/// compiler's canonical diagnostic type.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum SpanValidationError {
    /// The end byte offset precedes the start byte offset.
    ReversedRange {
        start: usize,
        end: usize,
    },

    /// The range extends beyond the registered source text.
    OutOfBounds {
        start: usize,
        end: usize,
        source_len: usize,
    },

    /// A range boundary splits a UTF-8 encoded character.
    InvalidUtf8Boundary {
        offset: usize,
    },

    /// The requested file ID is not registered in the supplied source map.
    SourceNotFound {
        file_id: usize,
    },

    /// The span represents missing provenance and cannot identify a source.
    MissingSourceProvenance,
}

impl fmt::Display for SpanValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ReversedRange { start, end } => write!(
                f,
                "source span has a reversed byte range: start {start}, end {end}"
            ),
            Self::OutOfBounds {
                start,
                end,
                source_len,
            } => write!(
                f,
                "source span {start}..{end} exceeds source length {source_len}"
            ),
            Self::InvalidUtf8Boundary { offset } => write!(
                f,
                "source span boundary at byte {offset} is not a UTF-8 character boundary"
            ),
            Self::SourceNotFound { file_id } => {
                write!(f, "source file ID {file_id} is not registered")
            }
            Self::MissingSourceProvenance => {
                f.write_str("source span has no real source provenance")
            }
        }
    }
}

impl std::error::Error for SpanValidationError {}

/// Extract a span's half-open byte range after checking its ordering.
///
/// This function deliberately does not call `Span::len()`: the legacy
/// implementation subtracts its endpoints without first checking their
/// ordering. The public fields of the legacy span mean that callers can
/// construct an invalid range, so checked extraction must validate it.
///
/// The returned offsets are relative to the source file associated with
/// the span's `FileId`.
pub fn checked_byte_range(
    span: &Span,
) -> Result<Range<usize>, SpanValidationError> {
    let start = span.start.0 as usize;
    let end = span.end.0 as usize;

    if end < start {
        return Err(SpanValidationError::ReversedRange { start, end });
    }

    Ok(start..end)
}

/// Validate a span against source text.
///
/// Checks:
/// - the range is ordered;
/// - the end does not exceed the source byte length;
/// - both endpoints are UTF-8 character boundaries.
///
/// This function cannot establish that `span.file_id` belongs to `source`,
/// because the legacy `SourceFile` does not store its own `FileId`.
/// Use [`validate_span_in_map`] when validating a registered source.
pub fn validate_span(
    span: &Span,
    source: &SourceFile,
) -> Result<Range<usize>, SpanValidationError> {
    let range = checked_byte_range(span)?;
    let content: &str = source.content.as_str();
    let source_len = content.len();

    if range.end > source_len {
        return Err(SpanValidationError::OutOfBounds {
            start: range.start,
            end: range.end,
            source_len,
        });
    }

    if !content.is_char_boundary(range.start) {
        return Err(SpanValidationError::InvalidUtf8Boundary {
            offset: range.start,
        });
    }

    if !content.is_char_boundary(range.end) {
        return Err(SpanValidationError::InvalidUtf8Boundary {
            offset: range.end,
        });
    }

    Ok(range)
}

/// Validate a span against the source file registered under its own ID.
///
/// This is the preferred validation entry point when the caller has a
/// `SourceMap`. It checks source identity by resolving `span.file_id` before
/// validating the byte range against that source's text.
///
/// Dummy spans represent missing provenance and return an explicit error.
/// Use [`validate_optional_span_in_map`] if missing provenance is allowed.
pub fn validate_span_in_map(
    span: &Span,
    source_map: &SourceMap,
) -> Result<Range<usize>, SpanValidationError> {
    if span.is_dummy() {
        return Err(SpanValidationError::MissingSourceProvenance);
    }

    let source = source_map
        .get_file(span.file_id)
        .ok_or(SpanValidationError::SourceNotFound {
            file_id: span.file_id.0,
        })?;

    validate_span(span, source.as_ref())
}

/// Validate an optional source span.
///
/// Returns `Ok(None)` for a dummy span, explicitly preserving the distinction
/// between missing provenance and a real zero-width span. A real empty span
/// at EOF returns `Ok(Some(offset..offset))`.
///
/// Non-dummy spans must resolve to a registered source and pass all checks.
pub fn validate_optional_span_in_map(
    span: &Span,
    source_map: &SourceMap,
) -> Result<Option<Range<usize>>, SpanValidationError> {
    if span.is_dummy() {
        return Ok(None);
    }

    validate_span_in_map(span, source_map).map(Some)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn source_map_with(content: &str) -> (SourceMap, FileId) {
        let mut map = SourceMap::new();
        let (file_id, _) =
            map.add_file("test.zm".to_owned(), content.to_owned());
        (map, file_id)
    }

    fn span(
        file_id: FileId,
        start: u32,
        end: u32,
    ) -> Span {
        Span::new(
            file_id,
            BytePos::new(start),
            BytePos::new(end),
            1,
            1,
        )
    }

    #[test]
    fn reexports_are_the_canonical_source_map_types() {
        assert_eq!(
            core::any::type_name::<Span>(),
            core::any::type_name::<crate::source_map::Span>()
        );
        assert_eq!(
            core::any::type_name::<SourceMap>(),
            core::any::type_name::<crate::source_map::SourceMap>()
        );
    }

    #[test]
    fn validates_an_ascii_range() {
        let (map, file_id) = source_map_with("let value = 42;");
        let range =
            validate_span_in_map(&span(file_id, 4, 9), &map)
                .expect("valid ASCII span");

        assert_eq!(range, 4..9);
        assert_eq!(range.len(), 5);
    }

    #[test]
    fn accepts_an_empty_range_at_eof() {
        let (map, file_id) = source_map_with("abc");
        let range =
            validate_span_in_map(&span(file_id, 3, 3), &map)
                .expect("valid EOF span");

        assert_eq!(range, 3..3);
        assert!(range.is_empty());
    }

    #[test]
    fn accepts_a_complete_multibyte_character() {
        // π occupies two bytes in UTF-8.
        let (map, file_id) = source_map_with("aπb");
        let range =
            validate_span_in_map(&span(file_id, 1, 3), &map)
                .expect("valid UTF-8 character range");

        assert_eq!(range, 1..3);
    }

    #[test]
    fn rejects_a_boundary_inside_a_multibyte_character() {
        let (map, file_id) = source_map_with("aπb");
        let error =
            validate_span_in_map(&span(file_id, 2, 3), &map)
                .expect_err("byte 2 is inside π");

        assert_eq!(
            error,
            SpanValidationError::InvalidUtf8Boundary { offset: 2 }
        );
    }

    #[test]
    fn rejects_a_range_past_the_source_end() {
        let (map, file_id) = source_map_with("abc");
        let error =
            validate_span_in_map(&span(file_id, 1, 4), &map)
                .expect_err("end offset is outside the source");

        assert_eq!(
            error,
            SpanValidationError::OutOfBounds {
                start: 1,
                end: 4,
                source_len: 3,
            }
        );
    }

    #[test]
    fn rejects_reversed_ranges_without_subtraction_underflow() {
        let malformed = span(FileId::new(1), 3, 2);
        let error =
            checked_byte_range(&malformed)
                .expect_err("reversed ranges must be rejected");

        assert_eq!(
            error,
            SpanValidationError::ReversedRange { start: 3, end: 2 }
        );
    }

    #[test]
    fn rejects_unregistered_source_ids() {
        let (map, _) = source_map_with("abc");
        let error =
            validate_span_in_map(&span(FileId::new(99), 0, 1), &map)
                .expect_err("unknown file ID must not be accepted");

        assert_eq!(
            error,
            SpanValidationError::SourceNotFound { file_id: 99 }
        );
    }

    #[test]
    fn dummy_provenance_is_not_confused_with_an_empty_real_span() {
        let (map, _) = source_map_with("");
        let dummy = Span::dummy();

        assert_eq!(
            validate_optional_span_in_map(&dummy, &map)
                .expect("missing provenance is permitted"),
            None
        );

        assert_eq!(
            validate_span_in_map(&dummy, &map),
            Err(SpanValidationError::MissingSourceProvenance)
        );
    }

    #[test]
    fn rejects_a_byte_range_that_splits_a_unicode_character_at_either_end() {
        let (map, file_id) = source_map_with("π");

        let invalid_start =
            validate_span_in_map(&span(file_id, 1, 2), &map)
                .expect_err("start splits π");

        assert_eq!(
            invalid_start,
            SpanValidationError::InvalidUtf8Boundary { offset: 1 }
        );

        let invalid_end =
            validate_span_in_map(&span(file_id, 0, 1), &map)
                .expect_err("end splits π");

        assert_eq!(
            invalid_end,
            SpanValidationError::InvalidUtf8Boundary { offset: 1 }
        );
    }
}