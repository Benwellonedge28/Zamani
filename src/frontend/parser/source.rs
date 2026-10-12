
#![forbid(unsafe_code)]

//! Parser-facing source context and source-span adapter.
//!
//! File: `src/frontend/parser/source.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Provide a stable bridge between lexer source spans and the canonical AST
//! source-span representation. Retain the source text needed for validation,
//! slicing, and diagnostics without defining a competing source-ID or span
//! model.
//!
//! # Ownership
//!
//! This module owns:
//! - the source text and optional display name for one parse input;
//! - the canonical source identity associated with that input;
//! - checked conversion from lexer spans to AST spans;
//! - validation of byte ranges against the original UTF-8 source;
//! - checked source slicing and EOF-span construction.
//!
//! This module does not own:
//! - source-ID allocation or global source-map registration;
//! - lexical scanning or tokenization;
//! - line/column calculation;
//! - diagnostic policy or recovery;
//! - AST node construction beyond source spans;
//! - name or type resolution;
//! - quantum, classical, HDL, or other domain semantics;
//! - target discovery, resource negotiation, or hardware realization.
//!
//! # Integration contract
//!
//! - `lexer/span.rs` owns lexer coordinates and lexer-span validation.
//! - `ast/node/source/` owns the canonical `SourceId` and AST `Span`.
//! - `token.rs` carries lexer spans; parser diagnostics and AST construction
//!   use this module to convert them into canonical AST spans.
//! - The parser session should retain one `SourceContext` for each source
//!   unit and pass references to grammar productions and diagnostics.
//! - A source identity is supplied by the caller. This module does not
//!   allocate IDs, use global mutable state, or assume IDs are contiguous.
//! - A span is valid only for the source identity and text against which it
//!   is checked. Numeric offsets alone do not identify a source.
//!
//! # Scalability
//!
//! Source text is shared through `Arc<str>` so cloning a context does not
//! duplicate the source buffer. Span conversion and slicing do not copy source
//! text. No machine, device, program-size, or hardware-capacity ceiling is
//! encoded here.
//!
//! The representable coordinate range, address space, allocator, and available
//! memory remain practical implementation constraints. Callers processing
//! untrusted input should enforce their configured parser resource budgets.
//!
//! # Dependency direction
//!
//! ```text
//! lexer::span::Span
//!         |
//!         v
//! parser::source::SourceContext
//!         |
//!         v
//! ast::node::source::{SourceId, Span}
//!         |
//!         v
//! parser diagnostics and AST construction
//! ```
//!
//! This module intentionally does not depend on parser grammar productions,
//! parser sessions, or domain-specific AST extensions.

use std::error::Error;
use std::fmt;
use std::ops::Range;
use std::sync::Arc;

use super::super::ast::node::source::{
    SourceId,
    Span as AstSpan,
    SpanError as AstSpanError,
};
use super::super::lexer::span::Span as LexerSpan;

/// Canonical source span type used by parser-facing APIs.
///
/// This is an alias to the AST's existing span type, not a second span
/// representation.
pub type SourceSpan = AstSpan;

/// Source text and identity associated with one parser input.
///
/// Cloning this value shares the source buffer. It does not allocate a new
/// source identity and does not imply that the source is registered in a
/// repository-wide source map.
///
/// `SourceContext` is immutable after construction. Mutable parser state,
/// diagnostics, and recovery state belong to the parser session.
#[derive(Debug, Clone)]
pub struct SourceContext {
    id: SourceId,
    name: Option<Arc<str>>,
    text: Arc<str>,
}

impl SourceContext {
    /// Create a context for a source unit.
    ///
    /// The caller is responsible for supplying the canonical source identity.
    /// This constructor deliberately does not create IDs or consult global
    /// state.
    #[must_use]
    pub fn new(id: SourceId, text: impl Into<Arc<str>>) -> Self {
        Self {
            id,
            name: None,
            text: text.into(),
        }
    }

    /// Create a context with a human-readable source name.
    ///
    /// Names are presentation metadata, not source identity. Two sources may
    /// have the same name while retaining different `SourceId` values.
    #[must_use]
    pub fn named(
        id: SourceId,
        name: impl Into<Arc<str>>,
        text: impl Into<Arc<str>>,
    ) -> Self {
        Self {
            id,
            name: Some(name.into()),
            text: text.into(),
        }
    }

    /// Return the canonical source identity.
    #[must_use]
    pub const fn id(&self) -> SourceId {
        self.id
    }

    /// Return the optional display name.
    #[must_use]
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Return the original source text.
    #[must_use]
    pub fn text(&self) -> &str {
        self.text.as_ref()
    }

    /// Return the source text as a shared allocation.
    ///
    /// This is useful when another component needs to retain the source
    /// without copying its contents.
    #[must_use]
    pub fn shared_text(&self) -> Arc<str> {
        Arc::clone(&self.text)
    }

    /// Return the source length in UTF-8 bytes.
    #[must_use]
    pub fn len_bytes(&self) -> usize {
        self.text.len()
    }

    /// Return whether the source contains no bytes.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }

    /// Construct a canonical AST span from checked byte offsets.
    ///
    /// The range is half-open: `[start, end)`.
    ///
    /// In addition to checking range order and bounds, this method verifies
    /// that both endpoints are UTF-8 character boundaries. That is required
    /// for safe slicing and avoids carrying invalid ranges into AST consumers.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - `start` exceeds `end`;
    /// - either endpoint lies beyond the source;
    /// - either endpoint is not a UTF-8 character boundary;
    /// - an offset cannot be represented by the canonical AST coordinate type.
    pub fn span_from_offsets(
        &self,
        start: usize,
        end: usize,
    ) -> Result<SourceSpan, SourceError> {
        self.validate_range(start, end)?;

        AstSpan::from_usize(self.id, start, end).map_err(SourceError::CanonicalSpan)
    }

    /// Construct a zero-width canonical AST span at a byte offset.
    ///
    /// This is appropriate for EOF locations and insertion-point diagnostics.
    pub fn point_span(&self, offset: usize) -> Result<SourceSpan, SourceError> {
        self.span_from_offsets(offset, offset)
    }

    /// Construct a canonical AST span at the end of the source.
    ///
    /// For an empty source this returns the valid point span `[0, 0)`.
    pub fn eof_span(&self) -> Result<SourceSpan, SourceError> {
        self.point_span(self.text.len())
    }

    /// Convert a lexer span into the canonical AST span.
    ///
    /// Lexer line and column information is deliberately not copied into the
    /// AST span: the canonical AST span stores a source identity and byte
    /// offsets. Line and column rendering should be derived by the source-map
    /// or diagnostic layer using its documented coordinate convention.
    ///
    /// This method verifies that the lexer range belongs to a valid range in
    /// this source text. Since a lexer span does not carry a source identity,
    /// the caller must ensure it originated from this context's source.
    pub fn from_lexer_span(
        &self,
        span: &LexerSpan,
    ) -> Result<SourceSpan, SourceError> {
        self.span_from_offsets(span.start_byte(), span.end_byte())
    }

    /// Return a checked byte range for a canonical AST span.
    ///
    /// Rejects spans belonging to another source, spans outside this source,
    /// and ranges that do not end on UTF-8 boundaries.
    pub fn byte_range(
        &self,
        span: SourceSpan,
    ) -> Result<Range<usize>, SourceError> {
        self.ensure_source(span)?;

        let start = usize::try_from(span.start_raw())
            .map_err(|_| SourceError::OffsetNotRepresentable {
                offset: span.start_raw(),
            })?;
        let end = usize::try_from(span.end_raw())
            .map_err(|_| SourceError::OffsetNotRepresentable {
                offset: span.end_raw(),
            })?;

        self.validate_range(start, end)?;
        Ok(start..end)
    }

    /// Return the source slice covered by a canonical AST span.
    ///
    /// The returned string borrows from this context and performs no
    /// allocation.
    pub fn slice(&self, span: SourceSpan) -> Result<&str, SourceError> {
        let range = self.byte_range(span)?;

        // `byte_range` has checked bounds and UTF-8 boundaries, so slicing
        // cannot panic for a valid `SourceContext` and span.
        self.text
            .get(range)
            .ok_or(SourceError::InvalidUtf8Boundary)
    }

    /// Validate and return a byte range without constructing an AST span.
    ///
    /// This is useful for parser diagnostics and internal source operations
    /// that need to validate offsets before creating a node.
    pub fn checked_range(
        &self,
        start: usize,
        end: usize,
    ) -> Result<Range<usize>, SourceError> {
        self.validate_range(start, end)?;
        Ok(start..end)
    }

    fn ensure_source(&self, span: SourceSpan) -> Result<(), SourceError> {
        if span.source() != self.id {
            return Err(SourceError::SourceMismatch {
                expected: self.id,
                actual: span.source(),
            });
        }

        Ok(())
    }

    fn validate_range(&self, start: usize, end: usize) -> Result<(), SourceError> {
        if start > end {
            return Err(SourceError::ReversedRange { start, end });
        }

        let source_len = self.text.len();

        if start > source_len {
            return Err(SourceError::OffsetOutOfBounds {
                offset: start,
                source_len,
            });
        }

        if end > source_len {
            return Err(SourceError::OffsetOutOfBounds {
                offset: end,
                source_len,
            });
        }

        if !self.text.is_char_boundary(start) || !self.text.is_char_boundary(end) {
            return Err(SourceError::InvalidUtf8Boundary);
        }

        Ok(())
    }
}

/// Errors produced when validating source ranges or converting spans.
///
/// These errors are intentionally independent of human-readable parser
/// diagnostics. The diagnostic layer may map them to stable diagnostic codes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum SourceError {
    /// The range starts after it ends.
    ReversedRange {
        /// Inclusive start byte offset.
        start: usize,
        /// Exclusive end byte offset.
        end: usize,
    },

    /// An offset is outside the source buffer.
    OffsetOutOfBounds {
        /// Offending byte offset.
        offset: usize,
        /// Source length in bytes.
        source_len: usize,
    },

    /// An offset is not a UTF-8 character boundary.
    InvalidUtf8Boundary,

    /// A span refers to a different source unit.
    SourceMismatch {
        /// Source identity expected by this context.
        expected: SourceId,
        /// Source identity carried by the span.
        actual: SourceId,
    },

    /// A canonical offset cannot be represented by this platform's `usize`.
    OffsetNotRepresentable {
        /// Canonical byte offset that could not be converted.
        offset: u64,
    },

    /// Construction of the canonical AST span failed.
    CanonicalSpan(AstSpanError),
}

impl fmt::Display for SourceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ReversedRange { start, end } => {
                write!(
                    f,
                    "source range is reversed: start byte {start} exceeds end byte {end}"
                )
            }
            Self::OffsetOutOfBounds { offset, source_len } => {
                write!(
                    f,
                    "source byte offset {offset} is outside source length {source_len}"
                )
            }
            Self::InvalidUtf8Boundary => {
                f.write_str("source range endpoint is not a UTF-8 character boundary")
            }
            Self::SourceMismatch { expected, actual } => {
                write!(
                    f,
                    "source span belongs to source {}, but source {} was expected",
                    actual.as_raw(),
                    expected.as_raw()
                )
            }
            Self::OffsetNotRepresentable { offset } => {
                write!(
                    f,
                    "canonical source offset {offset} is not representable on this platform"
                )
            }
            Self::CanonicalSpan(error) => {
                write!(f, "cannot construct canonical AST source span: {error:?}")
            }
        }
    }
}

impl Error for SourceError {}

impl From<AstSpanError> for SourceError {
    fn from(error: AstSpanError) -> Self {
        Self::CanonicalSpan(error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::lexer::cursor::CursorPosition;
    use super::super::lexer::span::Span as LexerSpan;

    fn context(text: &str) -> SourceContext {
        SourceContext::new(SourceId::from_raw(7), Arc::<str>::from(text))
    }

    fn lexer_span(start: usize, end: usize) -> LexerSpan {
        LexerSpan::try_new(
            CursorPosition::new(start, 1, start.saturating_add(1)),
            CursorPosition::new(end, 1, end.saturating_add(1)),
        )
        .expect("test lexer span must be ordered")
    }

    #[test]
    fn constructs_empty_source_and_eof_span() {
        let source = context("");

        assert!(source.is_empty());
        assert_eq!(source.len_bytes(), 0);

        let span = source.eof_span().expect("EOF span should be valid");

        assert_eq!(span.source(), source.id());
        assert_eq!(span.start_raw(), 0);
        assert_eq!(span.end_raw(), 0);
        assert_eq!(source.slice(span).expect("empty slice"), "");
    }

    #[test]
    fn creates_and_slices_a_half_open_span() {
        let source = context("classical quantum");

        let span = source
            .span_from_offsets(0, 9)
            .expect("range should be valid");

        assert_eq!(source.slice(span).expect("slice should be valid"), "classical");
        assert_eq!(source.byte_range(span).expect("range should be valid"), 0..9);
    }

    #[test]
    fn accepts_unicode_when_offsets_are_character_boundaries() {
        let source = context("aλ🧪z");

        let span = source
            .span_from_offsets(1, 3)
            .expect("lambda occupies two UTF-8 bytes");

        assert_eq!(source.slice(span).expect("slice should be valid"), "λ");

        let emoji = source
            .span_from_offsets(3, 7)
            .expect("emoji occupies four UTF-8 bytes");

        assert_eq!(source.slice(emoji).expect("slice should be valid"), "🧪");
    }

    #[test]
    fn rejects_offsets_inside_multibyte_characters() {
        let source = context("λ");

        assert_eq!(
            source.span_from_offsets(1, 2),
            Err(SourceError::InvalidUtf8Boundary)
        );

        assert_eq!(
            source.span_from_offsets(0, 1),
            Err(SourceError::InvalidUtf8Boundary)
        );
    }

    #[test]
    fn rejects_reversed_ranges() {
        let source = context("abcdef");

        assert_eq!(
            source.span_from_offsets(4, 2),
            Err(SourceError::ReversedRange { start: 4, end: 2 })
        );
    }

    #[test]
    fn rejects_offsets_past_source_end() {
        let source = context("abc");

        assert_eq!(
            source.span_from_offsets(0, 4),
            Err(SourceError::OffsetOutOfBounds {
                offset: 4,
                source_len: 3,
            })
        );
    }

    #[test]
    fn converts_lexer_span_to_canonical_ast_span() {
        let source = context("abc");

        let parsed_span = source
            .from_lexer_span(&lexer_span(1, 3))
            .expect("lexer span should convert");

        assert_eq!(parsed_span.source(), source.id());
        assert_eq!(parsed_span.start_raw(), 1);
        assert_eq!(parsed_span.end_raw(), 3);
        assert_eq!(source.slice(parsed_span).expect("slice"), "bc");
    }

    #[test]
    fn rejects_a_span_from_another_source() {
        let first = context("first");
        let second = SourceContext::new(SourceId::from_raw(8), "second");

        let span = first
            .span_from_offsets(0, 2)
            .expect("first-source span should be valid");

        assert_eq!(
            second.byte_range(span),
            Err(SourceError::SourceMismatch {
                expected: second.id(),
                actual: first.id(),
            })
        );
    }

    #[test]
    fn cloned_context_shares_source_text_and_identity() {
        let original = SourceContext::named(
            SourceId::from_raw(42),
            "quantum.zm",
            "quantum circuit",
        );
        let cloned = original.clone();

        assert_eq!(cloned.id(), original.id());
        assert_eq!(cloned.name(), Some("quantum.zm"));
        assert_eq!(cloned.text(), original.text());
        assert_eq!(cloned.shared_text().as_ref(), original.text());
    }

    #[test]
    fn point_span_is_valid_at_end_of_nonempty_source() {
        let source = context("source");

        let span = source.point_span(source.len_bytes()).expect("EOF point");

        assert!(span.is_empty());
        assert_eq!(span.start_raw(), source.len_bytes() as u64);
        assert_eq!(span.end_raw(), source.len_bytes() as u64);
    }
}
