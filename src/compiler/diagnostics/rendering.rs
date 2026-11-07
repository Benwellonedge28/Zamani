
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Zamani Diagnostic Rendering
//!
//! Path: src/compiler/diagnostics/rendering.rs
//!
//! Rust: 1.97+
//! Edition: 2021
//!
//! PURPOSE
//!
//! Render canonical Zamani diagnostics into human-readable output.
//!
//! OWNERSHIP
//!
//! This module owns presentation only.
//!
//! It does not own:
//! - Diagnostic construction;
//! - Diagnostic identity or severity;
//! - Source-file identity;
//! - Source storage;
//! - Source-map construction;
//! - Diagnostic aggregation;
//! - Diagnostic ordering;
//! - Diagnostic serialization;
//! - Semantic validation;
//! - Parser recovery;
//! - Fix-it execution.
//!
//! CANONICAL AUTHORITIES
//!
//! Diagnostic:
//! crate::frontend::semantic::diagnostics
//!
//! Span:
//! crate::frontend::ast::node::source::span
//!
//! Source provenance:
//! crate::zuir::source_map
//!
//! Aggregation:
//! crate::compiler::diagnostics::aggregation
//!
//! Reporting:
//! crate::compiler::diagnostics::reporting
//!
//! SCALABILITY
//!
//! No universal source-size, diagnostic-count, hardware-count,
//! or computational-domain limit is introduced.
//!
//! Rendering budgets are optional operational policies.
//!
//! SECURITY
//!
//! User-controlled presentation text is escaped.
//! Fix-its are displayed, never executed.
//! No filesystem, network, process, or hardware access occurs.

use core::fmt;
use std::io::{self, Write};
use std::num::NonZeroUsize;

use crate::frontend::ast::node::source::span::Span;

use crate::frontend::semantic::diagnostics::{
    Diagnostic,
    FixIt,
};

// =============================================================================
// SOURCE PROVIDER
// =============================================================================

/// Borrowed source document.
///
/// Source identity is the canonical raw SourceId.
///
/// `name` is a display name, not permission to access a filesystem path.
///
/// Source storage remains owned by the source-management subsystem.
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

/// Supplies source documents for diagnostic rendering.
///
/// Implementations must return the requested source identity.
///
/// The provider must keep returned source text alive for the
/// duration of the rendering operation.
pub trait SourceProvider {
    fn source(
        &self,
        source_id: u64,
    ) -> Option<SourceFileRef<'_>>;
}

// =============================================================================
// DISPLAY WIDTH
// =============================================================================

/// Defines the presentation width of a Unicode scalar.
///
/// Source coordinates remain UTF-8 byte offsets.
///
/// Display width is exclusively a presentation concern.
pub trait DisplayWidth {
    fn character_width(&self, character: char) -> usize;
}

/// Portable scalar-width implementation.
///
/// Non-control Unicode scalars occupy one display unit.
///
/// This is deliberately not advertised as complete terminal
/// grapheme or East Asian width conformance.
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

// =============================================================================
// OPTIONS
// =============================================================================

/// Diagnostic presentation policies.
///
/// Every limit is an operational rendering budget, not a
/// restriction on valid Zamani programs.
#[derive(Debug, Clone)]
pub struct RenderingOptions {
    tab_width: NonZeroUsize,
    max_lines_per_location: Option<NonZeroUsize>,
    max_output_bytes: Option<usize>,
    show_fix_replacements: bool,
}

impl Default for RenderingOptions {
    fn default() -> Self {
        Self {
            tab_width: NonZeroUsize::new(4)
                .expect("four is nonzero"),
            max_lines_per_location: None,
            max_output_bytes: None,
            show_fix_replacements: true,
        }
    }
}

impl RenderingOptions {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn with_tab_width(
        mut self,
        width: NonZeroUsize,
    ) -> Self {
        self.tab_width = width;
        self
    }

    #[must_use]
    pub fn with_max_lines_per_location(
        mut self,
        maximum: Option<NonZeroUsize>,
    ) -> Self {
        self.max_lines_per_location = maximum;
        self
    }

    #[must_use]
    pub fn with_max_output_bytes(
        mut self,
        maximum: Option<usize>,
    ) -> Self {
        self.max_output_bytes = maximum;
        self
    }

    #[must_use]
    pub fn with_fix_replacements(
        mut self,
        enabled: bool,
    ) -> Self {
        self.show_fix_replacements = enabled;
        self
    }
}

// =============================================================================
// ERRORS
// =============================================================================

/// Infrastructure failures encountered during rendering.
#[derive(Debug)]
#[non_exhaustive]
pub enum RenderingError {
    MissingSource {
        source_id: u64,
    },

    SourceIdentityMismatch {
        requested: u64,
        returned: u64,
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

    LineBudgetExceeded {
        configured: usize,
        required: usize,
    },

    OutputBudgetExceeded {
        configured: usize,
        required: usize,
    },

    AllocationFailure,

    WriterFailure(io::Error),
}

impl fmt::Display for RenderingError {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::MissingSource { source_id } => {
                write!(
                    f,
                    "source document {source_id} is unavailable"
                )
            }

            Self::SourceIdentityMismatch {
                requested,
                returned,
            } => {
                write!(
                    f,
                    "source identity mismatch: requested {requested}, \
                     received {returned}"
                )
            }

            Self::InvalidSourceRange {
                source_id,
                start,
                end,
            } => {
                write!(
                    f,
                    "invalid source range {start}..{end} \
                     for source {source_id}"
                )
            }

            Self::OffsetNotRepresentable { offset } => {
                write!(
                    f,
                    "source offset {offset} is not representable \
                     on this platform"
                )
            }

            Self::InvalidUtf8Boundary {
                source_id,
                offset,
            } => {
                write!(
                    f,
                    "invalid UTF-8 boundary {offset} \
                     in source {source_id}"
                )
            }

            Self::InvalidOptions => {
                f.write_str("invalid diagnostic rendering options")
            }

            Self::LineBudgetExceeded {
                configured,
                required,
            } => {
                write!(
                    f,
                    "rendering requires {required} lines; \
                     configured budget is {configured}"
                )
            }

            Self::OutputBudgetExceeded {
                configured,
                required,
            } => {
                write!(
                    f,
                    "rendering requires at least {required} bytes; \
                     configured budget is {configured}"
                )
            }

            Self::AllocationFailure => {
                f.write_str("diagnostic output allocation failed")
            }

            Self::WriterFailure(error) => {
                write!(
                    f,
                    "diagnostic output writer failed: {error}"
                )
            }
        }
    }
}

impl std::error::Error for RenderingError {
    fn source(
        &self,
    ) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::WriterFailure(error) => Some(error),
            _ => None,
        }
    }
}

// =============================================================================
// OUTPUT SINK
// =============================================================================

/// Internal output abstraction.
///
/// The renderer writes through this abstraction so byte budgets
/// are enforced consistently for owned and borrowed output.
trait RenderSink {
    fn write_bytes(
        &mut self,
        bytes: &[u8],
    ) -> Result<(), RenderingError>;
}

/// String-backed output.
struct StringSink {
    value: String,
    maximum: Option<usize>,
}

impl StringSink {
    fn new(maximum: Option<usize>) -> Self {
        Self {
            value: String::new(),
            maximum,
        }
    }

    fn finish(self) -> String {
        self.value
    }
}

impl RenderSink for StringSink {
    fn write_bytes(
        &mut self,
        bytes: &[u8],
    ) -> Result<(), RenderingError> {
        let required = self.value
            .len()
            .checked_add(bytes.len())
            .ok_or(RenderingError::AllocationFailure)?;

        if let Some(maximum) = self.maximum {
            if required > maximum {
                return Err(
                    RenderingError::OutputBudgetExceeded {
                        configured: maximum,
                        required,
                    },
                );
            }
        }

        self.value
            .try_reserve(bytes.len())
            .map_err(|_| RenderingError::AllocationFailure)?;

        // All renderer-produced fragments are valid UTF-8.
        let text = std::str::from_utf8(bytes)
            .map_err(|_| RenderingError::InvalidOptions)?;

        self.value.push_str(text);

        Ok(())
    }
}

/// Borrowed writer-backed output.
///
/// A writer can receive partial output if a subsequent write fails.
/// Such output must not be treated as a complete diagnostic.
struct WriterSink<'a, W: Write + ?Sized> {
    writer: &'a mut W,
    written: usize,
    maximum: Option<usize>,
}

impl<'a, W: Write + ?Sized> WriterSink<'a, W> {
    fn new(
        writer: &'a mut W,
        maximum: Option<usize>,
    ) -> Self {
        Self {
            writer,
            written: 0,
            maximum,
        }
    }
}

impl<W: Write + ?Sized> RenderSink for WriterSink<'_, W> {
    fn write_bytes(
        &mut self,
        bytes: &[u8],
    ) -> Result<(), RenderingError> {
        let required = self.written
            .checked_add(bytes.len())
            .ok_or(RenderingError::AllocationFailure)?;

        if let Some(maximum) = self.maximum {
            if required > maximum {
                return Err(
                    RenderingError::OutputBudgetExceeded {
                        configured: maximum,
                        required,
                    },
                );
            }
        }

        self.writer
            .write_all(bytes)
            .map_err(RenderingError::WriterFailure)?;

        self.written = required;

        Ok(())
    }
}

fn write_text(
    output: &mut impl RenderSink,
    value: &str,
) -> Result<(), RenderingError> {
    output.write_bytes(value.as_bytes())
}

fn write_fmt(
    output: &mut impl RenderSink,
    arguments: fmt::Arguments<'_>,
) -> Result<(), RenderingError> {
    let mut temporary = String::new();

    temporary
        .try_reserve(arguments.to_string().len())
        .map_err(|_| RenderingError::AllocationFailure)?;

    fmt::write(&mut temporary, arguments)
        .map_err(|_| RenderingError::AllocationFailure)?;

    write_text(output, &temporary)
}

// =============================================================================
// SAFE TEXT
// =============================================================================

/// Write untrusted presentation text without permitting terminal
/// control-character injection.
///
/// Escaping is unconditional for control characters.
fn write_safe_text(
    output: &mut impl RenderSink,
    value: &str,
) -> Result<(), RenderingError> {
    for character in value.chars() {
        if character.is_control() {
            let escaped = character.escape_debug().to_string();
            write_text(output, &escaped)?;
        } else {
            let mut buffer = [0u8; 4];
            let encoded = character.encode_utf8(&mut buffer);
            write_text(output, encoded)?;
        }
    }

    Ok(())
}

// =============================================================================
// SPAN VALIDATION
// =============================================================================

fn validate_span(
    span: Span,
    source: SourceFileRef<'_>,
) -> Result<(usize, usize), RenderingError> {
    let start_raw = span.start_raw();
    let end_raw = span.end_raw();

    let start = usize::try_from(start_raw)
        .map_err(|_| {
            RenderingError::OffsetNotRepresentable {
                offset: start_raw,
            }
        })?;

    let end = usize::try_from(end_raw)
        .map_err(|_| {
            RenderingError::OffsetNotRepresentable {
                offset: end_raw,
            }
        })?;

    if start > end || end > source.text.len() {
        return Err(RenderingError::InvalidSourceRange {
            source_id: source.id,
            start: start_raw,
            end: end_raw,
        });
    }

    for offset in [start, end] {
        if !source.text.is_char_boundary(offset) {
            return Err(
                RenderingError::InvalidUtf8Boundary {
                    source_id: source.id,
                    offset,
                },
            );
        }
    }

    Ok((start, end))
}

// =============================================================================
// SOURCE LINE MODEL
// =============================================================================

#[derive(Debug, Clone, Copy)]
struct SourceLine {
    start: usize,
    content_end: usize,
    next_start: usize,
    number: usize,
}

/// Identify Unicode line terminators.
///
/// CRLF is treated as one line ending.
fn line_ending_at(
    text: &str,
    offset: usize,
) -> Option<usize> {
    let character = text.get(offset..)?.chars().next()?;

    match character {
        '\n' => Some(offset + 1),

        '\r' => {
            let next = offset + 1;

            if text.as_bytes().get(next) == Some(&b'\n') {
                Some(next + 1)
            } else {
                Some(next)
            }
        }

        '\u{0085}' | '\u{2028}' | '\u{2029}' => {
            Some(offset + character.len_utf8())
        }

        _ => None,
    }
}

/// Return the line containing a byte offset.
///
/// This is demand-driven: it does not allocate a complete
/// source-file line index.
///
/// Empty sources and trailing empty lines are supported.
fn find_line(
    text: &str,
    offset: usize,
) -> Option<SourceLine> {
    if offset > text.len() || !text.is_char_boundary(offset) {
        return None;
    }

    let mut line_start = 0usize;
    let mut line_number = 1usize;

    let mut cursor = 0usize;

    while cursor < text.len() {
        if let Some(next) = line_ending_at(text, cursor) {
            let line = SourceLine {
                start: line_start,
                content_end: cursor,
                next_start: next,
                number: line_number,
            };

            // EOF after a line ending belongs to the next,
            // empty source line.
            if offset < next {
                return Some(line);
            }

            line_start = next;
            line_number = line_number.checked_add(1)?;
            cursor = next;
        } else {
            let character = text.get(cursor..)?.chars().next()?;
            cursor += character.len_utf8();
        }
    }

    Some(SourceLine {
        start: line_start,
        content_end: text.len(),
        next_start: text.len(),
        number: line_number,
    })
}

// =============================================================================
// DISPLAY COLUMNS
// =============================================================================

fn advance_column<W: DisplayWidth>(
    column: usize,
    character: char,
    width: &W,
    tab_width: usize,
) -> Option<usize> {
    let advance = if character == '\t' {
        let remainder = column % tab_width;

        if remainder == 0 {
            tab_width
        } else {
            tab_width - remainder
        }
    } else {
        width.character_width(character)
    };

    column.checked_add(advance)
}

fn display_width<W: DisplayWidth>(
    text: &str,
    width: &W,
    tab_width: usize,
) -> Result<usize, RenderingError> {
    let mut column = 0usize;

    for character in text.chars() {
        column = advance_column(
            column,
            character,
            width,
            tab_width,
        )
        .ok_or(RenderingError::AllocationFailure)?;
    }

    Ok(column)
}

/// Expand tabs and escape control characters while retaining
/// the exact display-width policy used for marker positions.
fn write_source_text<W: DisplayWidth>(
    output: &mut impl RenderSink,
    text: &str,
    width: &W,
    tab_width: usize,
) -> Result<usize, RenderingError> {
    let mut column = 0usize;

    for character in text.chars() {
        if character == '\t' {
            let remainder = column % tab_width;

            let spaces = if remainder == 0 {
                tab_width
            } else {
                tab_width - remainder
            };

            for _ in 0..spaces {
                write_text(output, " ")?;
            }

            column = column
                .checked_add(spaces)
                .ok_or(RenderingError::AllocationFailure)?;
        } else if character.is_control() {
            let escaped = character.escape_debug().to_string();

            write_text(output, &escaped)?;

            column = column
                .checked_add(escaped.chars().count())
                .ok_or(RenderingError::AllocationFailure)?;
        } else {
            let mut buffer = [0u8; 4];
            let encoded = character.encode_utf8(&mut buffer);

            write_text(output, encoded)?;

            column = column
                .checked_add(width.character_width(character))
                .ok_or(RenderingError::AllocationFailure)?;
        }
    }

    Ok(column)
}

// =============================================================================
// SOURCE LOCATION RENDERING
// =============================================================================

fn render_source_location<W: DisplayWidth>(
    output: &mut impl RenderSink,
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

    if source.id != source_id {
        return Err(
            RenderingError::SourceIdentityMismatch {
                requested: source_id,
                returned: source.id,
            },
        );
    }

    let (start, end) = validate_span(span, source)?;

    let first = find_line(source.text, start)
        .ok_or(RenderingError::InvalidOptions)?;

    let last_offset = if end > start {
        end - 1
    } else {
        end
    };

    let last = find_line(source.text, last_offset)
        .ok_or(RenderingError::InvalidOptions)?;

    let required_lines = last.number
        .checked_sub(first.number)
        .and_then(|value| value.checked_add(1))
        .ok_or(RenderingError::AllocationFailure)?;

    if let Some(maximum) = options.max_lines_per_location {
        if required_lines > maximum.get() {
            return Err(
                RenderingError::LineBudgetExceeded {
                    configured: maximum.get(),
                    required: required_lines,
                },
            );
        }
    }

    // Derive the displayed column using canonical UTF-8 byte
    // coordinates and the selected presentation-width policy.
    let prefix = &source.text[first.start..start.max(first.start)
        .min(first.content_end)];

    let column = display_width(
        prefix,
        width,
        options.tab_width.get(),
    )?;

    write_text(output, "  --> ")?;
    write_safe_text(output, source.name)?;
    write_fmt(
        output,
        format_args!(":{}:{}\n", first.number, column + 1),
    )?;

    let number_width = last.number.to_string().len();

    let mut line = first;
    let mut rendered = 0usize;

    loop {
        render_one_line(
            output,
            source,
            line,
            start,
            end,
            number_width,
            label.filter(|_| line.number == first.number),
            width,
            options,
        )?;

        rendered += 1;

        if line.number >= last.number {
            break;
        }

        if line.next_start <= line.start
            || line.next_start >= source.text.len()
        {
            break;
        }

        line = find_line(source.text, line.next_start)
            .ok_or(RenderingError::InvalidOptions)?;

        if rendered >= required_lines {
            break;
        }
    }

    Ok(())
}

fn render_one_line<W: DisplayWidth>(
    output: &mut impl RenderSink,
    source: SourceFileRef<'_>,
    line: SourceLine,
    span_start: usize,
    span_end: usize,
    number_width: usize,
    label: Option<&str>,
    width: &W,
    options: &RenderingOptions,
) -> Result<(), RenderingError> {
    let text = &source.text[line.start..line.content_end];

    let selected_start = span_start
        .max(line.start)
        .min(line.content_end);

    let selected_end = span_end
        .max(selected_start)
        .min(line.content_end);

    let prefix = &source.text[line.start..selected_start];

    let selected = &source.text[selected_start..selected_end];

    let marker_column = display_width(
        prefix,
        width,
        options.tab_width.get(),
    )?;

    let marker_width = display_width(
        selected,
        width,
        options.tab_width.get(),
    )?.max(1);

    write_fmt(
        output,
        format_args!(
            "{:>number_width$} | ",
            line.number
        ),
    )?;

    let actual_width = write_source_text(
        output,
        text,
        width,
        options.tab_width.get(),
    )?;

    write_text(output, "\n")?;

    write_fmt(
        output,
        format_args!(
            "{:>number_width$} | ",
            ""
        ),
    )?;

    for _ in 0..marker_column {
        write_text(output, " ")?;
    }

    for _ in 0..marker_width {
        write_text(output, "^")?;
    }

    if actual_width < marker_column {
        return Err(RenderingError::InvalidOptions);
    }

    if let Some(message) = label {
        write_text(output, " ")?;
        write_safe_text(output, message)?;
    }

    write_text(output, "\n")?;

    Ok(())
}

// =============================================================================
// FIX-IT RENDERING
// =============================================================================

fn render_fix(
    output: &mut impl RenderSink,
    fix: &FixIt,
    sources: &impl SourceProvider,
    options: &RenderingOptions,
) -> Result<(), RenderingError> {
    write_text(
        output,
        if fix.is_safe() {
            "  suggestion: "
        } else {
            "  suggestion (review required): "
        },
    )?;

    write_safe_text(output, fix.title())?;
    write_text(output, "\n")?;

    if !options.show_fix_replacements {
        return Ok(());
    }

    for edit in fix.edits() {
        let span = edit.span();
        let source_id = span.source().as_raw();

        let source = sources
            .source(source_id)
            .ok_or(RenderingError::MissingSource {
                source_id,
            })?;

        if source.id != source_id {
            return Err(
                RenderingError::SourceIdentityMismatch {
                    requested: source_id,
                    returned: source.id,
                },
            );
        }

        validate_span(span, source)?;

        write_fmt(
            output,
            format_args!(
                "    source {} bytes {}..{}\n",
                source_id,
                span.start_raw(),
                span.end_raw(),
            ),
        )?;

        write_text(output, "      replacement: ")?;
        write_safe_text(output, edit.replacement())?;
        write_text(output, "\n")?;
    }

    Ok(())
}

// =============================================================================
// DIAGNOSTIC RENDERING
// =============================================================================

fn render_diagnostic_into<W: DisplayWidth>(
    diagnostic: &Diagnostic,
    sources: &impl SourceProvider,
    width: &W,
    options: &RenderingOptions,
    output: &mut impl RenderSink,
) -> Result<(), RenderingError> {
    write_safe_text(
        output,
        diagnostic.severity().as_str(),
    )?;

    write_text(output, "[")?;

    write_safe_text(
        output,
        diagnostic.code().as_str(),
    )?;

    write_text(output, "]: ")?;

    write_safe_text(
        output,
        diagnostic.message(),
    )?;

    write_text(output, "\n")?;

    if let Some(span) = diagnostic.primary_span() {
        render_source_location(
            output,
            span,
            None,
            sources,
            width,
            options,
        )?;
    }

    for label in diagnostic.labels() {
        render_source_location(
            output,
            label.span(),
            Some(label.message()),
            sources,
            width,
            options,
        )?;
    }

    for note in diagnostic.notes() {
        write_text(output, "  note: ")?;
        write_safe_text(output, note)?;
        write_text(output, "\n")?;
    }

    if let Some(help) = diagnostic.help() {
        write_text(output, "  help: ")?;
        write_safe_text(output, help)?;
        write_text(output, "\n")?;
    }

    for fix in diagnostic.fixes() {
        render_fix(
            output,
            fix,
            sources,
            options,
        )?;
    }

    Ok(())
}

// =============================================================================
// PUBLIC API
// =============================================================================

/// Render one diagnostic into an owned string.
///
/// This convenience API allocates output proportional to the
/// rendered diagnostic.
///
/// For large output, use `write_diagnostic`.
pub fn render_diagnostic<W: DisplayWidth>(
    diagnostic: &Diagnostic,
    sources: &impl SourceProvider,
    width: &W,
    options: &RenderingOptions,
) -> Result<String, RenderingError> {
    let mut output = StringSink::new(
        options.max_output_bytes,
    );

    render_diagnostic_into(
        diagnostic,
        sources,
        width,
        options,
        &mut output,
    )?;

    Ok(output.finish())
}

/// Render one diagnostic using the configured budgets.
///
/// Kept as a compatibility entry point.
pub fn render_diagnostic_with_budget<W: DisplayWidth>(
    diagnostic: &Diagnostic,
    sources: &impl SourceProvider,
    width: &W,
    options: &RenderingOptions,
) -> Result<String, RenderingError> {
    render_diagnostic(
        diagnostic,
        sources,
        width,
        options,
    )
}

/// Render multiple diagnostics in the supplied order.
///
/// Ordering and deduplication belong to the canonical diagnostic
/// ordering and aggregation authorities.
pub fn render_diagnostics<W: DisplayWidth>(
    diagnostics: &[Diagnostic],
    sources: &impl SourceProvider,
    width: &W,
    options: &RenderingOptions,
) -> Result<String, RenderingError> {
    let mut output = StringSink::new(
        options.max_output_bytes,
    );

    for (index, diagnostic) in diagnostics.iter().enumerate() {
        if index != 0 {
            write_text(&mut output, "\n")?;
        }

        render_diagnostic_into(
            diagnostic,
            sources,
            width,
            options,
            &mut output,
        )?;
    }

    Ok(output.finish())
}

/// Render directly into a caller-provided writer.
///
/// This avoids constructing a complete intermediate diagnostic
/// string. The caller retains ownership of the writer.
///
/// A failed writer may contain partial output.
pub fn write_diagnostic<W, P, D>(
    diagnostic: &Diagnostic,
    sources: &P,
    width: &D,
    options: &RenderingOptions,
    writer: &mut W,
) -> Result<(), RenderingError>
where
    W: Write + ?Sized,
    P: SourceProvider,
    D: DisplayWidth,
{
    let mut output = WriterSink::new(
        writer,
        options.max_output_bytes,
    );

    render_diagnostic_into(
        diagnostic,
        sources,
        width,
        options,
        &mut output,
    )
}

/// Render multiple diagnostics directly into a writer.
///
/// Diagnostics are emitted in their supplied order.
/// No sorting or deduplication is performed here.
pub fn write_diagnostics<W, P, D>(
    diagnostics: &[Diagnostic],
    sources: &P,
    width: &D,
    options: &RenderingOptions,
    writer: &mut W,
) -> Result<(), RenderingError>
where
    W: Write + ?Sized,
    P: SourceProvider,
    D: DisplayWidth,
{
    let mut output = WriterSink::new(
        writer,
        options.max_output_bytes,
    );

    for (index, diagnostic) in diagnostics.iter().enumerate() {
        if index != 0 {
            write_text(&mut output, "\n")?;
        }

        render_diagnostic_into(
            diagnostic,
            sources,
            width,
            options,
            &mut output,
        )?;
    }

    Ok(())
}
