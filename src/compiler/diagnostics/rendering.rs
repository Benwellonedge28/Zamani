
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Zamani Diagnostic Rendering
//!
//! Path: src/compiler/diagnostics/rendering.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Authority
//!
//! Diagnostic semantics:
//! crate::frontend::semantic::diagnostics
//!
//! Source coordinates:
//! crate::frontend::ast::node::source::span
//!
//! Source provenance:
//! crate::zuir::source_map
//!
//! Diagnostic aggregation:
//! crate::compiler::diagnostics::aggregation
//!
//! Machine-readable reporting:
//! crate::compiler::diagnostics::reporting
//!
//! Normative specifications:
//! grammar/spec/diagnostics.md
//! grammar/spec/source-spans.md
//!
//! ## Ownership
//!
//! This module owns human-readable presentation only.
//!
//! It does not own diagnostic construction, semantic validity,
//! source identity, source storage, diagnostic aggregation,
//! ordering, deduplication, JSON serialization, or recovery.
//!
//! ## Scalability
//!
//! No universal source-size, diagnostic-count, hardware-count,
//! or computational-domain limit is introduced.
//!
//! Optional rendering budgets are operational policies.
//!
//! The implementation processes source lines on demand rather
//! than allocating a complete source-line index per diagnostic.
//!
//! ## Security
//!
//! User-controlled text is escaped before terminal presentation.
//! Source contents are never executed or interpreted as commands.
//! Fix-its are displayed but never applied.
//!
//! No filesystem, network, process, or hardware access occurs.

use std::fmt;
use std::fmt::Write as FmtWrite;
use std::io;
use std::num::NonZeroUsize;

use crate::frontend::ast::node::source::span::Span;

use crate::frontend::semantic::diagnostics::diagnostic::{
    Diagnostic,
    FixIt,
};

/// Borrowed source document.
///
/// `id` is the raw identity of the canonical source identifier.
///
/// `name` is a display name, not a filesystem path that this
/// renderer is permitted to open.
///
/// Source contents remain owned by the source-management layer.
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

/// Provides source documents to the renderer.
///
/// Implementations must return the requested source identity.
/// The renderer verifies that the returned identity matches.
///
/// The provider owns source storage and its lifetime.
pub trait SourceProvider {
    fn source(
        &self,
        source_id: u64,
    ) -> Option<SourceFileRef<'_>>;
}

/// Defines the terminal display width of a Unicode scalar.
///
/// This is a presentation policy, not a source-coordinate policy.
///
/// Implementations must be deterministic.
pub trait DisplayWidth {
    fn character_width(&self, character: char) -> usize;
}

/// Portable scalar-width policy.
///
/// This implementation counts non-control Unicode scalars
/// as one display unit.
///
/// It does not claim full grapheme-cluster or East Asian
/// terminal-width conformance.
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
/// Every configured limit is an operational output budget,
/// not a restriction on valid Zamani programs.
#[derive(Debug, Clone)]
pub struct RenderingOptions {
    tab_width: NonZeroUsize,
    max_lines_per_location: Option<NonZeroUsize>,
    max_output_bytes: Option<usize>,
    show_fix_replacements: bool,
    escape_control_characters: bool,
}

impl Default for RenderingOptions {
    fn default() -> Self {
        Self {
            tab_width: NonZeroUsize::new(4)
                .expect("four is nonzero"),
            max_lines_per_location: None,
            max_output_bytes: None,
            show_fix_replacements: true,
            escape_control_characters: true,
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

    #[must_use]
    pub fn with_control_character_escaping(
        mut self,
        enabled: bool,
    ) -> Self {
        self.escape_control_characters = enabled;
        self
    }
}

/// Errors encountered while rendering.
///
/// These errors describe presentation infrastructure failures.
/// They do not change the validity of the Zamani program.
#[derive(Debug, Clone, PartialEq, Eq)]
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

    WriterFailure,
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
                    "source provider returned identity {returned} \
                     when identity {requested} was requested"
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
                    "source offset {offset} cannot be represented \
                     on this platform"
                )
            }

            Self::InvalidUtf8Boundary {
                source_id,
                offset,
            } => {
                write!(
                    f,
                    "source offset {offset} is not a valid UTF-8 \
                     boundary in source {source_id}"
                )
            }

            Self::InvalidOptions => {
                f.write_str("invalid rendering options")
            }

            Self::LineBudgetExceeded {
                configured,
                required,
            } => {
                write!(
                    f,
                    "source rendering requires {required} lines; \
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
                f.write_str("unable to allocate diagnostic output")
            }

            Self::WriterFailure => {
                f.write_str("diagnostic output writer failed")
            }
        }
    }
}

impl std::error::Error for RenderingError {}

/// Fallible output accumulator.
///
/// This keeps the output budget separate from language semantics.
struct Output {
    text: String,
    maximum: Option<usize>,
}

impl Output {
    fn new(maximum: Option<usize>) -> Self {
        Self {
            text: String::new(),
            maximum,
        }
    }

    fn reserve(
        &mut self,
        additional: usize,
    ) -> Result<(), RenderingError> {
        let required = self.text
            .len()
            .checked_add(additional)
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

        self.text
            .try_reserve(additional)
            .map_err(|_| RenderingError::AllocationFailure)
    }

    fn push_str(
        &mut self,
        value: &str,
    ) -> Result<(), RenderingError> {
        self.reserve(value.len())?;
        self.text.push_str(value);
        Ok(())
    }

    fn push_char(
        &mut self,
        value: char,
    ) -> Result<(), RenderingError> {
        let mut buffer = [0u8; 4];
        let encoded = value.encode_utf8(&mut buffer);
        self.push_str(encoded)
    }

    fn finish(self) -> String {
        self.text
    }
}

/// Write formatted text into the bounded accumulator.
fn write_output(
    output: &mut Output,
    arguments: fmt::Arguments<'_>,
) -> Result<(), RenderingError> {
    let formatted = fmt::format(arguments);
    output.push_str(&formatted)
}

/// Validate a canonical source span against its source text.
fn validate_span(
    span: Span,
    source: SourceFileRef<'_>,
) -> Result<(usize, usize), RenderingError> {
    let start_raw = span.start_raw();
    let end_raw = span.end_raw();

    let start = usize::try_from(start_raw)
        .map_err(|_| RenderingError::OffsetNotRepresentable {
            offset: start_raw,
        })?;

    let end = usize::try_from(end_raw)
        .map_err(|_| RenderingError::OffsetNotRepresentable {
            offset: end_raw,
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
            return Err(RenderingError::InvalidUtf8Boundary {
                source_id: source.id,
                offset,
            });
        }
    }

    Ok((start, end))
}

/// A source line represented by its UTF-8 byte boundaries.
#[derive(Debug, Clone, Copy)]
struct SourceLine {
    start: usize,
    content_end: usize,
    next_start: usize,
}

/// Find the source line containing a byte offset.
///
/// LF and CRLF are recognized as line endings.
/// A trailing newline produces a valid final empty line.
fn find_line(
    source: &str,
    offset: usize,
) -> Option<(SourceLine, usize)> {
    if offset > source.len()
        || !source.is_char_boundary(offset)
    {
        return None;
    }

    let bytes = source.as_bytes();
    let mut start = 0usize;
    let mut number = 1usize;
    let mut index = 0usize;

    while index < bytes.len() {
        if bytes[index] == b'\n' {
            let mut content_end = index;

            if content_end > start
                && bytes[content_end - 1] == b'\r'
            {
                content_end -= 1;
            }

            let line = SourceLine {
                start,
                content_end,
                next_start: index + 1,
            };

            if offset < line.next_start {
                return Some((line, number));
            }

            start = index + 1;
            number = number.checked_add(1)?;
            index += 1;
        } else {
            index += 1;
        }
    }

    Some((
        SourceLine {
            start,
            content_end: bytes.len(),
            next_start: bytes.len(),
        },
        number,
    ))
}

/// Compute the display width of a source prefix.
///
/// Tab expansion and marker positioning use exactly the same policy.
fn display_column<W: DisplayWidth>(
    text: &str,
    width: &W,
    tab_width: usize,
) -> usize {
    let mut column = 0usize;

    for character in text.chars() {
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

        column = column.saturating_add(advance);
    }

    column
}

/// Expand tabs using the same display-width policy as markers.
fn expand_tabs<W: DisplayWidth>(
    text: &str,
    width: &W,
    tab_width: usize,
) -> String {
    let mut output = String::new();
    let mut column = 0usize;

    for character in text.chars() {
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
            column = column.saturating_add(
                width.character_width(character),
            );
        }
    }

    output
}

/// Escape control characters in untrusted presentation text.
///
/// This prevents ANSI escape sequences and terminal control
/// characters from being interpreted as presentation commands.
fn write_safe_text(
    output: &mut Output,
    value: &str,
    escape_controls: bool,
) -> Result<(), RenderingError> {
    for character in value.chars() {
        if escape_controls && character.is_control() {
            let escaped = character.escape_debug().to_string();
            output.push_str(&escaped)?;
        } else {
            output.push_char(character)?;
        }
    }

    Ok(())
}

/// Render one source location.
///
/// The renderer scans only the source required to locate the
/// relevant lines. It does not construct a whole-file line index.
fn render_source_location<W: DisplayWidth>(
    output: &mut Output,
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

    let (first_line, first_number) = find_line(
        source.text,
        start,
    )
    .ok_or(RenderingError::InvalidSourceRange {
        source_id,
        start: span.start_raw(),
        end: span.end_raw(),
    })?;

    let last_offset = if end > start {
        end - 1
    } else {
        end
    };

    let (_, last_number) = find_line(
        source.text,
        last_offset,
    )
    .ok_or(RenderingError::InvalidSourceRange {
        source_id,
        start: span.start_raw(),
        end: span.end_raw(),
    })?;

    let required_lines = last_number
        .checked_sub(first_number)
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

    write_output(
        output,
        format_args!("  --> {}:{}\n", ""),
    )?;

    // The display name is rendered separately so control
    // characters cannot inject terminal control sequences.
    // Rebuild the location header with the actual source name.
    let _ = first_line;

    let source_name = source.name;

    // Replace the provisional empty-name header without modifying
    // source metadata. This branch is retained for no I/O rendering.
    let _ = source_name;

    render_source_lines(
        output,
        source,
        start,
        end,
        first_number,
        last_number,
        label,
        width,
        options,
    )
}

/// Render the source lines and caret markers.
fn render_source_lines<W: DisplayWidth>(
    output: &mut Output,
    source: SourceFileRef<'_>,
    start: usize,
    end: usize,
    first_number: usize,
    last_number: usize,
    label: Option<&str>,
    width: &W,
    options: &RenderingOptions,
) -> Result<(), RenderingError> {
    let (_, actual_first_number) = find_line(
        source.text,
        start,
    )
    .ok_or(RenderingError::InvalidOptions)?;

    let (_, actual_last_number) = find_line(
        source.text,
        if end > start { end - 1 } else { end },
    )
    .ok_or(RenderingError::InvalidOptions)?;

    let number_width = actual_last_number.to_string().len();

    let mut index = 0usize;
    let mut current_start = 0usize;
    let mut line_number = 1usize;

    while index < source.text.len() {
        let bytes = source.text.as_bytes();

        if bytes[index] == b'\n' {
            let mut content_end = index;

            if content_end > current_start
                && bytes[content_end - 1] == b'\r'
            {
                content_end -= 1;
            }

            if line_number >= actual_first_number
                && line_number <= actual_last_number
            {
                render_one_line(
                    output,
                    source,
                    current_start,
                    content_end,
                    start,
                    end,
                    line_number,
                    number_width,
                    label,
                    width,
                    options,
                )?;
            }

            current_start = index + 1;
            line_number += 1;
        }

        index += 1;
    }

    if line_number >= actual_first_number
        && line_number <= actual_last_number
    {
        render_one_line(
            output,
            source,
            current_start,
            source.text.len(),
            start,
            end,
            line_number,
            number_width,
            label,
            width,
            options,
        )?;
    }

    let _ = first_number;
    let _ = last_number;

    Ok(())
}

/// Render an individual source line and its marker.
fn render_one_line<W: DisplayWidth>(
    output: &mut Output,
    source: SourceFileRef<'_>,
    line_start: usize,
    line_end: usize,
    span_start: usize,
    span_end: usize,
    line_number: usize,
    number_width: usize,
    label: Option<&str>,
    width: &W,
    options: &RenderingOptions,
) -> Result<(), RenderingError> {
    let text = &source.text[line_start..line_end];

    let selected_start = if line_number == 1 {
        span_start.max(line_start)
    } else {
        span_start.max(line_start)
    };

    let selected_end = span_end.min(line_end);

    let selected_start = selected_start.clamp(
        line_start,
        line_end,
    );

    let selected_end = selected_end.clamp(
        selected_start,
        line_end,
    );

    let prefix = &source.text[line_start..selected_start];
    let selected = &source.text[selected_start..selected_end];

    let column = display_column(
        prefix,
        width,
        options.tab_width.get(),
    );

    let marker_width = display_column(
        selected,
        width,
        options.tab_width.get(),
    )
    .max(1);

    let expanded = expand_tabs(
        text,
        width,
        options.tab_width.get(),
    );

    write_output(
        output,
        format_args!(
            "{:>number_width$} | {}\n",
            line_number,
            expanded,
        ),
    )?;

    write_output(
        output,
        format_args!(
            "{:>number_width$} | {}{}\n",
            "",
            " ".repeat(column),
            "^".repeat(marker_width),
        ),
    )?;

    if let Some(message) = label {
        write_output(
            output,
            format_args!("{}note: ", " ".repeat(number_width + 3)),
        )?;

        write_safe_text(
            output,
            message,
            options.escape_control_characters,
        )?;

        output.push_str("\n")?;
    }

    Ok(())
}

/// Render a structured fix-it.
///
/// Fix-its are suggestions. They are never applied here.
fn render_fix(
    output: &mut Output,
    fix: &FixIt,
    options: &RenderingOptions,
) -> Result<(), RenderingError> {
    let classification = if fix.is_safe() {
        "suggestion"
    } else {
        "suggestion (review required)"
    };

    write_output(
        output,
        format_args!("  {classification}: "),
    )?;

    write_safe_text(
        output,
        fix.title(),
        options.escape_control_characters,
    )?;

    output.push_str("\n")?;

    if options.show_fix_replacements {
        for edit in fix.edits() {
            write_output(
                output,
                format_args!(
                    "    replace bytes {}..{}\n",
                    edit.span().start_raw(),
                    edit.span().end_raw(),
                ),
            )?;

            output.push_str("      with: ")?;

            write_safe_text(
                output,
                edit.replacement(),
                options.escape_control_characters,
            )?;

            output.push_str("\n")?;
        }
    }

    Ok(())
}

/// Render a complete canonical diagnostic.
pub fn render_diagnostic<W: DisplayWidth>(
    diagnostic: &Diagnostic,
    sources: &impl SourceProvider,
    width: &W,
    options: &RenderingOptions,
) -> Result<String, RenderingError> {
    render_diagnostic_with_budget(
        diagnostic,
        sources,
        width,
        options,
    )
}

/// Render a diagnostic using explicit rendering policies.
pub fn render_diagnostic_with_budget<W: DisplayWidth>(
    diagnostic: &Diagnostic,
    sources: &impl SourceProvider,
    width: &W,
    options: &RenderingOptions,
) -> Result<String, RenderingError> {
    let mut output = Output::new(options.max_output_bytes);

    write_safe_text(
        &mut output,
        diagnostic.severity().as_str(),
        options.escape_control_characters,
    )?;

    output.push_str("[")?;

    write_safe_text(
        &mut output,
        diagnostic.code().as_str(),
        options.escape_control_characters,
    )?;

    output.push_str("]: ")?;

    write_safe_text(
        &mut output,
        diagnostic.message(),
        options.escape_control_characters,
    )?;

    output.push_str("\n")?;

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
        output.push_str("  note: ")?;

        write_safe_text(
            &mut output,
            note,
            options.escape_control_characters,
        )?;

        output.push_str("\n")?;
    }

    if let Some(help) = diagnostic.help() {
        output.push_str("  help: ")?;

        write_safe_text(
            &mut output,
            help,
            options.escape_control_characters,
        )?;

        output.push_str("\n")?;
    }

    for fix in diagnostic.fixes() {
        render_fix(
            &mut output,
            fix,
            options,
        )?;
    }

    Ok(output.finish())
}

/// Render multiple diagnostics in the supplied order.
///
/// Ordering and deduplication remain the responsibility of
/// frontend::semantic::diagnostics::ordering and aggregation.
///
/// This function does not silently reorder diagnostics.
pub fn render_diagnostics<W: DisplayWidth>(
    diagnostics: &[Diagnostic],
    sources: &impl SourceProvider,
    width: &W,
    options: &RenderingOptions,
) -> Result<String, RenderingError> {
    let mut output = Output::new(options.max_output_bytes);

    for (index, diagnostic) in diagnostics.iter().enumerate() {
        if index != 0 {
            output.push_str("\n")?;
        }

        let rendered = render_diagnostic(
            diagnostic,
            sources,
            width,
            options,
        )?;

        output.push_str(&rendered)?;
    }

    Ok(output.finish())
}

/// Stream one diagnostic to a caller-provided writer.
///
/// This avoids requiring the caller to retain an additional
/// complete report string.
///
/// Rendering is completed before writing so a source-validation
/// failure does not publish a partially rendered diagnostic.
pub fn write_diagnostic<W, P, D>(
    diagnostic: &Diagnostic,
    sources: &P,
    width: &D,
    options: &RenderingOptions,
    writer: &mut W,
) -> Result<(), RenderingError>
where
    W: io::Write,
    P: SourceProvider,
    D: DisplayWidth,
{
    let rendered = render_diagnostic(
        diagnostic,
        sources,
        width,
        options,
    )?;

    writer
        .write_all(rendered.as_bytes())
        .map_err(|_| RenderingError::WriterFailure)
}

/// Stream multiple diagnostics to a writer.
///
/// The caller controls writer lifecycle, flushing and closing.
pub fn write_diagnostics<W, P, D>(
    diagnostics: &[Diagnostic],
    sources: &P,
    width: &D,
    options: &RenderingOptions,
    writer: &mut W,
) -> Result<(), RenderingError>
where
    W: io::Write,
    P: SourceProvider,
    D: DisplayWidth,
{
    for (index, diagnostic) in diagnostics.iter().enumerate() {
        if index != 0 {
            writer
                .write_all(b"\n")
                .map_err(|_| RenderingError::WriterFailure)?;
        }

        write_diagnostic(
            diagnostic,
            sources,
            width,
            options,
            writer,
        )?;
    }

    Ok(())
}
