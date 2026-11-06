
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Human-readable rendering of canonical Zamani compiler diagnostics.
//!
//! Path: src/compiler/diagnostics/rendering.rs
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//!
//! # Purpose
//!
//! Render existing structured diagnostics into deterministic human-readable
//! text suitable for terminals, logs, build tools, and compiler clients.
//!
//! # Ownership
//!
//! This module owns:
//! - Human-readable diagnostic formatting.
//! - Source-context presentation.
//! - Byte-offset to display-column conversion.
//! - Tab expansion.
//! - Rendering options and rendering errors.
//! - Source-provider and display-width integration contracts.
//!
//! This module does NOT own:
//! - Diagnostic construction or diagnostic identity.
//! - Diagnostic codes or severity definitions.
//! - Source-file identity or source-span construction.
//! - Source-file storage.
//! - Semantic analysis.
//! - Compiler recovery.
//! - Diagnostic aggregation or deduplication.
//! - IDE protocol serialization.
//! - Hardware discovery or execution.
//!
//! # Canonical dependencies
//!
//! - crate::frontend::semantic::diagnostics::diagnostic::Diagnostic
//! - crate::frontend::ast::node::source::span::Span
//!
//! Source text is obtained through SourceProvider. This avoids coupling
//! rendering to a particular source-map implementation.
//!
//! # Scalability
//!
//! No universal diagnostic-count, source-size, hardware-count, or
//! computational-domain limit is imposed.
//!
//! Rendering budgets are optional and configurable.
//! Budget exhaustion is reported explicitly.
//!
//! Actual capacity is bounded by available memory and processing resources.
//!
//! # Determinism
//!
//! Formatting does not depend on hash-map iteration, wall-clock time,
//! thread completion order, physical hardware, or memory addresses.
//!
//! # Security
//!
//! Source text and diagnostic messages are treated as untrusted text.
//! The renderer does not execute terminal control sequences intentionally.
//! It does not perform filesystem or network operations.
//!
//! # Integration
//!
//! diagnostics/mod.rs should expose this module.
//! diagnostics/aggregation.rs owns aggregation.
//! diagnostics/reporting.rs owns report-level publication.
//! src/source_map.rs or an explicit adapter implements SourceProvider.
//!
//! This module does not replace those responsibilities.

use std::fmt::Write as FmtWrite;
use std::num::NonZeroUsize;

use crate::frontend::ast::node::source::span::Span;
use crate::frontend::semantic::diagnostics::diagnostic::{
    Diagnostic,
    DiagnosticLabel,
    FixIt,
};

/// Borrowed source document supplied to the renderer.
///
/// `id` must correspond to the canonical SourceId raw value.
///
/// The renderer never takes ownership of source contents.
#[derive(Debug, Clone, Copy)]
pub struct SourceFileRef<'a> {
    pub id: u64,
    pub name: &'a str,
    pub text: &'a str,
}

impl<'a> SourceFileRef<'a> {
    #[must_use]
    pub const fn new(
        id: u64,
        name: &'a str,
        text: &'a str,
    ) -> Self {
        Self { id, name, text }
    }
}

/// Supplies source documents for diagnostic locations.
///
/// Implement this trait for the repository's authoritative source-map
/// implementation or a frontend-specific adapter.
///
/// Missing source documents are reported explicitly.
pub trait SourceProvider {
    fn source(&self, source_id: u64) -> Option<SourceFileRef<'_>>;
}

/// Calculates terminal display width for a Unicode scalar.
///
/// The default implementation counts Unicode scalar values.
/// Applications requiring terminal-accurate grapheme and East Asian
/// display widths should provide an implementation backed by their
/// chosen Unicode-width policy.
///
/// The width must be deterministic.
pub trait DisplayWidth {
    fn character_width(&self, character: char) -> usize;
}

/// Portable default display-width implementation.
///
/// This implementation counts Unicode scalar values as one display unit.
/// It is deliberately not advertised as full grapheme-cluster or
/// East Asian terminal-width handling.
#[derive(Debug, Clone, Copy, Default)]
pub struct ScalarDisplayWidth;

impl DisplayWidth for ScalarDisplayWidth {
    fn character_width(&self, character: char) -> usize {
        if character.is_control() {
            0
        } else {
            1
        }
    }
}

/// Rendering configuration.
///
/// Limits are operational budgets, not language or hardware limits.
#[derive(Debug, Clone)]
pub struct RenderingOptions {
    tab_width: NonZeroUsize,
    max_lines: Option<NonZeroUsize>,
    show_fix_replacements: bool,
}

impl Default for RenderingOptions {
    fn default() -> Self {
        Self {
            tab_width: NonZeroUsize::new(4)
                .expect("four is non-zero"),
            max_lines: None,
            show_fix_replacements: true,
        }
    }
}

impl RenderingOptions {
    /// Create default rendering options.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Configure tab expansion.
    #[must_use]
    pub fn with_tab_width(
        mut self,
        width: NonZeroUsize,
    ) -> Self {
        self.tab_width = width;
        self
    }

    /// Configure the maximum number of source lines rendered
    /// for one source location.
    ///
    /// None means no configured line limit.
    #[must_use]
    pub fn with_max_lines(
        mut self,
        maximum: Option<NonZeroUsize>,
    ) -> Self {
        self.max_lines = maximum;
        self
    }

    /// Control whether replacement text is displayed for fix-its.
    #[must_use]
    pub fn with_fix_replacements(
        mut self,
        enabled: bool,
    ) -> Self {
        self.show_fix_replacements = enabled;
        self
    }
}

/// Rendering failure.
///
/// These errors are separate from compiler semantic errors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RenderingError {
    MissingSource {
        source_id: u64,
    },

    InvalidSourceRange {
        source_id: u64,
        start: u64,
        end: u64,
    },

    OffsetNotRepresentable {
        offset: u64,
    },

    InvalidUtf8Boundary {
        source_id: u64,
        offset: usize,
    },

    InvalidOptions,

    BudgetExceeded {
        configured: usize,
        required: usize,
    },
}

impl std::fmt::Display for RenderingError {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::MissingSource { source_id } => {
                write!(
                    formatter,
                    "source document {source_id} is unavailable"
                )
            }

            Self::InvalidSourceRange {
                source_id,
                start,
                end,
            } => {
                write!(
                    formatter,
                    "invalid source range {start}..{end} \
                     for source {source_id}"
                )
            }

            Self::OffsetNotRepresentable { offset } => {
                write!(
                    formatter,
                    "source offset {offset} cannot be represented \
                     on this platform"
                )
            }

            Self::InvalidUtf8Boundary {
                source_id,
                offset,
            } => {
                write!(
                    formatter,
                    "source offset {offset} is not a valid UTF-8 \
                     boundary in source {source_id}"
                )
            }

            Self::InvalidOptions => {
                formatter.write_str("invalid diagnostic rendering options")
            }

            Self::BudgetExceeded {
                configured,
                required,
            } => {
                write!(
                    formatter,
                    "rendering budget exceeded: configured \
                     {configured} lines, required {required}"
                )
            }
        }
    }
}

impl std::error::Error for RenderingError {}

/// A single indexed source line.
///
/// Coordinates are UTF-8 byte offsets.
#[derive(Debug, Clone, Copy)]
struct SourceLine {
    start: usize,
    content_end: usize,
    next_start: usize,
}

/// Construct a source-line index.
///
/// Newline bytes are excluded from displayed content.
/// CRLF is treated as one line ending.
fn index_lines(source: &str) -> Vec<SourceLine> {
    let mut lines = Vec::new();
    let bytes = source.as_bytes();

    let mut start = 0usize;

    for (index, byte) in bytes.iter().enumerate() {
        if *byte == b'\n' {
            let mut content_end = index;

            if content_end > start
                && bytes[content_end - 1] == b'\r'
            {
                content_end -= 1;
            }

            lines.push(SourceLine {
                start,
                content_end,
                next_start: index + 1,
            });

            start = index + 1;
        }
    }

    // Always preserve the final line, including an empty final line
    // after a trailing newline.
    lines.push(SourceLine {
        start,
        content_end: bytes.len(),
        next_start: bytes.len(),
    });

    lines
}

/// Locate the source line containing a byte offset.
///
/// EOF is assigned to the final line.
fn line_index_for_offset(
    lines: &[SourceLine],
    offset: usize,
) -> Option<usize> {
    if lines.is_empty() {
        return None;
    }

    let index = lines.partition_point(|line| {
        line.start <= offset
    });

    Some(index.saturating_sub(1).min(lines.len() - 1))
}

/// Calculate the visual display column for a UTF-8 prefix.
fn display_column<W: DisplayWidth>(
    prefix: &str,
    width: &W,
    tab_width: usize,
) -> usize {
    let mut column = 0usize;

    for character in prefix.chars() {
        if character == '\t' {
            let remainder = column % tab_width;

            let advance = if remainder == 0 {
                tab_width
            } else {
                tab_width - remainder
            };

            column = column.saturating_add(advance);
        } else {
            column = column.saturating_add(
                width.character_width(character),
            );
        }
    }

    column
}

/// Validate a source span against its registered source document.
fn validate_span(
    span: Span,
    source: SourceFileRef<'_>,
) -> Result<(usize, usize), RenderingError> {
    let start = usize::try_from(span.start_raw())
        .map_err(|_| RenderingError::OffsetNotRepresentable {
            offset: span.start_raw(),
        })?;

    let end = usize::try_from(span.end_raw())
        .map_err(|_| RenderingError::OffsetNotRepresentable {
            offset: span.end_raw(),
        })?;

    if start > end || end > source.text.len() {
        return Err(RenderingError::InvalidSourceRange {
            source_id: source.id,
            start: span.start_raw(),
            end: span.end_raw(),
        });
    }

    if !source.text.is_char_boundary(start) {
        return Err(RenderingError::InvalidUtf8Boundary {
            source_id: source.id,
            offset: start,
        });
    }

    if !source.text.is_char_boundary(end) {
        return Err(RenderingError::InvalidUtf8Boundary {
            source_id: source.id,
            offset: end,
        });
    }

    Ok((start, end))
}

/// Render a source location and its marker.
///
/// This operation does not modify the canonical span.
fn render_source_location<W: DisplayWidth>(
    output: &mut String,
    span: Span,
    label: Option<&str>,
    sources: &impl SourceProvider,
    width: &W,
    options: &RenderingOptions,
) -> Result<(), RenderingError> {
    let source_id = span.source().as_raw();

    let source = sources
        .source(source_id)
        .ok_or(RenderingError::MissingSource {
            source_id,
        })?;

    let (start, end) = validate_span(span, source)?;

    let lines = index_lines(source.text);

    let first_line = line_index_for_offset(&lines, start)
        .ok_or(RenderingError::InvalidOptions)?;

    let last_offset = if end > start {
        end - 1
    } else {
        end
    };

    let last_line = line_index_for_offset(
        &lines,
        last_offset,
    )
    .ok_or(RenderingError::InvalidOptions)?;

    let required = last_line
        .saturating_sub(first_line)
        .saturating_add(1);

    if let Some(maximum) = options.max_lines {
        if required > maximum.get() {
            return Err(RenderingError::BudgetExceeded {
                configured: maximum.get(),
                required,
            });
        }
    }

    let start_line_number = first_line.saturating_add(1);

    writeln!(
        output,
        "  --> {}:{}",
        source.name,
        start_line_number,
    )
    .map_err(|_| RenderingError::InvalidOptions)?;

    let number_width = (last_line + 1).to_string().len();

    writeln!(
        output,
        "{:>width$} |",
        "",
        width = number_width,
    )
    .map_err(|_| RenderingError::InvalidOptions)?;

    for index in first_line..=last_line {
        let line = lines[index];

        let text = &source.text[line.start..line.content_end];

        let line_start = if index == first_line {
            start
        } else {
            line.start
        };

        let line_end = if index == last_line {
            end.min(line.content_end)
        } else {
            line.content_end
        };

        if line_start > line_end
            || !source.text.is_char_boundary(line_start)
            || !source.text.is_char_boundary(line_end)
        {
            return Err(RenderingError::InvalidSourceRange {
                source_id,
                start: line_start as u64,
                end: line_end as u64,
            });
        }

        let prefix = &source.text[line.start..line_start];
        let selected = &source.text[line_start..line_end];

        let column = display_column(
            prefix,
            width,
            options.tab_width.get(),
        );

        let marker_width = if selected.is_empty() {
            1
        } else {
            display_column(
                selected,
                width,
                options.tab_width.get(),
            )
            .max(1)
        };

        let expanded_line = expand_tabs(
            text,
            options.tab_width.get(),
        );

        writeln!(
            output,
            "{:>width$} | {}",
            index + 1,
            expanded_line,
            width = number_width,
        )
        .map_err(|_| RenderingError::InvalidOptions)?;

        write!(
            output,
            "{:>width$} | {}{}",
            "",
            " ".repeat(column),
            "^".repeat(marker_width),
            width = number_width,
        )
        .map_err(|_| RenderingError::InvalidOptions)?;

        if let Some(message) = label {
            write!(output, " {message}")
                .map_err(|_| RenderingError::InvalidOptions)?;
        }

        output.push('\n');
    }

    Ok(())
}

/// Expand tabs using the configured display tab width.
fn expand_tabs(
    input: &str,
    tab_width: usize,
) -> String {
    let mut output = String::new();
    let mut column = 0usize;

    for character in input.chars() {
        if character == '\t' {
            let remainder = column % tab_width;

            let advance = if remainder == 0 {
                tab_width
            } else {
                tab_width - remainder
            };

            for _ in 0..advance {
                output.push(' ');
            }

            column = column.saturating_add(advance);
        } else {
            output.push(character);
            column = column.saturating_add(1);
        }
    }

    output
}

/// Render a canonical diagnostic.
///
/// The diagnostic is borrowed and never modified.
///
/// Source text is obtained from the supplied provider.
///
/// # Errors
///
/// Returns a structured rendering error if a source is missing,
/// an offset is invalid, or an explicit rendering budget is exceeded.
pub fn render_diagnostic<W: DisplayWidth>(
    diagnostic: &Diagnostic,
    sources: &impl SourceProvider,
    width: &W,
    options: &RenderingOptions,
) -> Result<String, RenderingError> {
    if options.tab_width.get() == 0 {
        return Err(RenderingError::InvalidOptions);
    }

    let mut output = String::new();

    writeln!(
        output,
        "{}[{}]: {}",
        diagnostic.severity(),
        diagnostic.code().as_str(),
        diagnostic.message(),
    )
    .map_err(|_| RenderingError::InvalidOptions)?;

    if let Some(span) = diagnostic.primary_span() {
        render_source_location(
            &mut output,
            span,
            None,
            sources,
            width,
            options,
        )?;
    }

    for label in diagnostic.labels() {
        render_source_location(
            &mut output,
            label.span(),
            Some(label.message()),
            sources,
            width,
            options,
        )?;
    }

    for note in diagnostic.notes() {
        writeln!(
            output,
            "  note: {note}"
        )
        .map_err(|_| RenderingError::InvalidOptions)?;
    }

    if let Some(help) = diagnostic.help() {
        writeln!(
            output,
            "  help: {help}"
        )
        .map_err(|_| RenderingError::InvalidOptions)?;
    }

    for fix in diagnostic.fixes() {
        render_fix(
            &mut output,
            fix,
            options,
        )?;
    }

    Ok(output)
}

/// Render a structured fix-it.
///
/// Fix-its remain suggestions. Rendering them does not apply edits.
fn render_fix(
    output: &mut String,
    fix: &FixIt,
    options: &RenderingOptions,
) -> Result<(), RenderingError> {
    let classification = if fix.is_safe() {
        "suggestion"
    } else {
        "suggestion (review required)"
    };

    writeln!(
        output,
        "  {classification}: {}",
        fix.title(),
    )
    .map_err(|_| RenderingError::InvalidOptions)?;

    if options.show_fix_replacements {
        for edit in fix.edits() {
            writeln!(
                output,
                "    replace bytes {}..{}",
                edit.span().start_raw(),
                edit.span().end_raw(),
            )
            .map_err(|_| RenderingError::InvalidOptions)?;

            writeln!(
                output,
                "      with: {:?}",
                edit.replacement(),
            )
            .map_err(|_| RenderingError::InvalidOptions)?;
        }
    }

    Ok(())
}

/// Render a collection of diagnostics in the supplied order.
///
/// The caller is responsible for canonical sorting and deduplication.
/// This function deliberately does not implement a competing ordering
/// algorithm.
pub fn render_diagnostics<W: DisplayWidth>(
    diagnostics: &[Diagnostic],
    sources: &impl SourceProvider,
    width: &W,
    options: &RenderingOptions,
) -> Result<String, RenderingError> {
    let mut output = String::new();

    for (index, diagnostic) in diagnostics.iter().enumerate() {
        if index > 0 {
            output.push('\n');
        }

        output.push_str(&render_diagnostic(
            diagnostic,
            sources,
            width,
            options,
        )?);
    }

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::frontend::ast::node::source::span::{
        SourceId,
        SourceOffset,
    };

    use crate::frontend::semantic::diagnostics::diagnostic::{
        DiagnosticCategory,
        DiagnosticPhase,
        DiagnosticSeverity,
    };

    struct TestSources {
        documents: Vec<SourceFileRef<'static>>,
    }

    impl SourceProvider for TestSources {
        fn source(
            &self,
            source_id: u64,
        ) -> Option<SourceFileRef<'_>> {
            self.documents
                .iter()
                .copied()
                .find(|document| document.id == source_id)
        }
    }

    fn source_span(
        id: u64,
        start: u64,
        end: u64,
    ) -> Span {
        Span::new(
            SourceId::from_raw(id),
            SourceOffset::from_raw(start),
            SourceOffset::from_raw(end),
        )
        .expect("test span must be valid")
    }

    fn sources() -> TestSources {
        TestSources {
            documents: vec![
                SourceFileRef::new(
                    1,
                    "main.zm",
                    "let value = 10;\nlet result = value + true;\n",
                ),
            ],
        }
    }

    fn diagnostic(span: Span) -> Diagnostic {
        Diagnostic::new(
            "ZM-SEM-TYPE-001",
            DiagnosticPhase::TypeAnalysis,
            DiagnosticCategory::Type,
            DiagnosticSeverity::Error,
            "incompatible types",
            Some(span),
        )
    }

    #[test]
    fn renders_code_severity_and_message() {
        let diagnostic = diagnostic(source_span(1, 29, 33));

        let rendered = render_diagnostic(
            &diagnostic,
            &sources(),
            &ScalarDisplayWidth,
            &RenderingOptions::default(),
        )
        .expect("rendering must succeed");

        assert!(rendered.contains("error[ZM-SEM-TYPE-001]"));
        assert!(rendered.contains("incompatible types"));
        assert!(rendered.contains("main.zm"));
    }

    #[test]
    fn renders_source_line_and_marker() {
        let diagnostic = diagnostic(source_span(1, 29, 33));

        let rendered = render_diagnostic(
            &diagnostic,
            &sources(),
            &ScalarDisplayWidth,
            &RenderingOptions::default(),
        )
        .expect("rendering must succeed");

        assert!(rendered.contains("let result"));
        assert!(rendered.contains("^^^^"));
    }

    #[test]
    fn handles_zero_length_spans() {
        let diagnostic = diagnostic(source_span(1, 0, 0));

        let rendered = render_diagnostic(
            &diagnostic,
            &sources(),
            &ScalarDisplayWidth,
            &RenderingOptions::default(),
        )
        .expect("rendering must succeed");

        assert!(rendered.contains("^"));
    }

    #[test]
    fn missing_source_is_reported() {
        let diagnostic = diagnostic(source_span(99, 0, 1));

        let error = render_diagnostic(
            &diagnostic,
            &sources(),
            &ScalarDisplayWidth,
            &RenderingOptions::default(),
        )
        .expect_err("missing source must fail explicitly");

        assert_eq!(
            error,
            RenderingError::MissingSource {
                source_id: 99,
            },
        );
    }

    #[test]
    fn invalid_source_range_is_rejected() {
        let diagnostic = diagnostic(source_span(1, 0, 500));

        let error = render_diagnostic(
            &diagnostic,
            &sources(),
            &ScalarDisplayWidth,
            &RenderingOptions::default(),
        )
        .expect_err("invalid range must be rejected");

        assert!(matches!(
            error,
            RenderingError::InvalidSourceRange { .. }
        ));
    }

    #[test]
    fn unicode_byte_offsets_are_validated() {
        let source = TestSources {
            documents: vec![
                SourceFileRef::new(
                    1,
                    "unicode.zm",
                    "let café = 1;\n",
                ),
            ],
        };

        let diagnostic = diagnostic(source_span(1, 5, 7));

        let error = render_diagnostic(
            &diagnostic,
            &source,
            &ScalarDisplayWidth,
            &RenderingOptions::default(),
        )
        .expect_err("a position inside a UTF-8 scalar must fail");

        assert!(matches!(
            error,
            RenderingError::InvalidUtf8Boundary { .. }
        ));
    }

    #[test]
    fn tab_expansion_is_deterministic() {
        assert_eq!(
            expand_tabs("a\tb", 4),
            "a   b",
        );

        assert_eq!(
            expand_tabs("\tvalue", 4),
            "    value",
        );
    }

    #[test]
    fn line_index_preserves_trailing_empty_line() {
        let lines = index_lines("first\n");

        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0].start, 0);
        assert_eq!(lines[0].content_end, 5);
        assert_eq!(lines[1].start, 6);
    }

    #[test]
    fn line_index_handles_crlf() {
        let lines = index_lines("first\r\nsecond");

        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0].content_end, 5);
        assert_eq!(lines[1].start, 7);
    }

    #[test]
    fn rendering_budget_is_explicit() {
        let diagnostic = diagnostic(source_span(1, 0, 45));

        let options = RenderingOptions::default()
            .with_max_lines(
                NonZeroUsize::new(1),
            );

        let error = render_diagnostic(
            &diagnostic,
            &sources(),
            &ScalarDisplayWidth,
            &options,
        )
        .expect_err("multi-line rendering must respect its budget");

        assert!(matches!(
            error,
            RenderingError::BudgetExceeded { .. }
        ));
    }

    #[test]
    fn rendering_does_not_mutate_diagnostic() {
        let diagnostic = diagnostic(source_span(1, 29, 33));
        let original = diagnostic.clone();

        let _ = render_diagnostic(
            &diagnostic,
            &sources(),
            &ScalarDisplayWidth,
            &RenderingOptions::default(),
        )
        .expect("rendering must succeed");

        assert_eq!(diagnostic, original);
    }
}
