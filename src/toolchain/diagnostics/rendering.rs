
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Toolchain Diagnostic Rendering
//!
//! File: src/toolchain/diagnostics/rendering.rs
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Render canonical toolchain diagnostics as safe, deterministic,
//! human-readable output.
//!
//! ## OWNERSHIP
//!
//! This module owns presentation only.
//!
//! It does not own:
//!
//! - Diagnostic construction or identity.
//! - Diagnostic classification or semantic meaning.
//! - Diagnostic aggregation or deduplication.
//! - Diagnostic ordering.
//! - Source identity or source-map construction.
//! - Source storage.
//! - Compiler or frontend diagnostic conversion.
//! - JSON or other machine-readable serialization.
//! - Source editing or fix-it execution.
//! - Build execution or target selection.
//!
//! ## AUTHORITIES
//!
//! Structured diagnostics:
//! super::diagnostic
//!
//! Diagnostic aggregation:
//! super::aggregation
//!
//! Canonical source spans:
//! crate::frontend::ast::node::source::span
//!
//! Normative diagnostic specification:
//! grammar/spec/diagnostics.md
//!
//! ## SCALABILITY
//!
//! No fixed diagnostic-count, source-size, hardware-size,
//! domain-count or output-size ceiling is imposed by default.
//!
//! Optional rendering budgets are operational policies.
//!
//! Large output can be written to a caller-provided writer.
//!
//! Source locations are resolved on demand. This module does not
//! allocate a complete source-file line index.
//!
//! ## SECURITY
//!
//! Untrusted diagnostic text and source excerpts are escaped
//! before being presented to a terminal.
//!
//! Fixes are displayed, never applied.
//!
//! Metadata is not displayed automatically.
//!
//! Source providers are read-only interfaces.
//!
//! ## DETERMINISM
//!
//! Rendering preserves diagnostic order supplied by the caller.
//! It never reorders or deduplicates diagnostic records.
//!
//! Source positions use the canonical zero-based UTF-8 byte-offset
//! contract. Display positions are one-based.
//!
//! ## COMPLETION
//!
//! An incomplete aggregated diagnostic result remains explicitly
//! identified as incomplete in the rendered output.

use core::fmt;
use core::fmt::Write as FmtWrite;
use std::io::{self, Write as IoWrite};
use std::num::NonZeroUsize;

use crate::frontend::ast::node::source::span::{
    SourceId,
    Span,
};

use super::aggregation::{
    AggregatedToolchainDiagnostics,
    AggregationStatus,
};

use super::diagnostic::{
    DiagnosticFix,
    DiagnosticSeverity,
    ToolchainDiagnostic,
};

// ============================================================================
// SOURCE PROVIDER
// ============================================================================

/// A borrowed source document used exclusively for presentation.
///
/// The source identity is the canonical raw `SourceId`.
///
/// The name is a display name, not permission to access a filesystem path.
///
/// Source ownership remains with the frontend or source-management
/// subsystem.
#[derive(Debug, Clone, Copy)]
pub struct SourceFileRef<'a> {
    /// Canonical source identity.
    pub id: u64,

    /// Human-readable source name.
    pub name: &'a str,

    /// Original UTF-8 source text.
    pub text: &'a str,
}

impl<'a> SourceFileRef<'a> {
    /// Construct a borrowed source-document reference.
    #[must_use]
    pub const fn new(
        id: u64,
        name: &'a str,
        text: &'a str,
    ) -> Self {
        Self { id, name, text }
    }
}

/// Read-only source lookup contract.
///
/// Implementations must return the requested source identity.
///
/// The returned source text must remain valid for the duration
/// of the rendering operation.
///
/// This interface deliberately does not open files or construct
/// source maps.
pub trait SourceProvider {
    /// Resolve one canonical source identity.
    fn source(
        &self,
        source_id: u64,
    ) -> Option<SourceFileRef<'_>>;
}

// ============================================================================
// DISPLAY WIDTH
// ============================================================================

/// Configurable presentation width for Unicode scalar values.
///
/// Source positions remain UTF-8 byte offsets.
///
/// This trait controls presentation only; it cannot change
/// canonical source coordinates.
pub trait DisplayWidth {
    /// Return the presentation width of a Unicode scalar.
    fn character_width(&self, character: char) -> usize;
}

/// Portable Unicode-scalar display-width implementation.
///
/// Ordinary non-control scalars occupy one display unit.
///
/// This does not claim complete terminal grapheme-cluster
/// or East Asian width conformance.
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

// ============================================================================
// OPTIONS
// ============================================================================

/// Configurable human-readable diagnostic presentation policies.
///
/// `max_output_bytes = None` means no configured output limit.
///
/// A configured output budget does not restrict the validity or
/// supported size of a Zamani program.
#[derive(Debug, Clone)]
pub struct RenderingOptions {
    tab_width: NonZeroUsize,
    max_output_bytes: Option<usize>,
    show_source: bool,
    show_fix_replacements: bool,
    show_origin: bool,
    show_phase: bool,
}

impl Default for RenderingOptions {
    fn default() -> Self {
        Self {
            tab_width: NonZeroUsize::MIN.saturating_add(
                NonZeroUsize::new(3).unwrap_or(NonZeroUsize::MIN)
            ),
            max_output_bytes: None,
            show_source: true,
            show_fix_replacements: true,
            show_origin: true,
            show_phase: true,
        }
    }
}

impl RenderingOptions {
    /// Create default rendering options.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Configure tab expansion width.
    #[must_use]
    pub fn with_tab_width(
        mut self,
        width: NonZeroUsize,
    ) -> Self {
        self.tab_width = width;
        self
    }

    /// Configure an optional output-byte budget.
    ///
    /// `None` means unlimited output subject to available resources.
    #[must_use]
    pub fn with_max_output_bytes(
        mut self,
        maximum: Option<usize>,
    ) -> Self {
        self.max_output_bytes = maximum;
        self
    }

    /// Enable or disable source excerpts.
    #[must_use]
    pub fn with_source(
        mut self,
        enabled: bool,
    ) -> Self {
        self.show_source = enabled;
        self
    }

    /// Enable or disable fix replacement text.
    ///
    /// Fix titles remain visible when replacement text is disabled.
    #[must_use]
    pub fn with_fix_replacements(
        mut self,
        enabled: bool,
    ) -> Self {
        self.show_fix_replacements = enabled;
        self
    }

    /// Enable or disable diagnostic provenance.
    #[must_use]
    pub fn with_origin(
        mut self,
        enabled: bool,
    ) -> Self {
        self.show_origin = enabled;
        self
    }

    /// Enable or disable diagnostic phase information.
    #[must_use]
    pub fn with_phase(
        mut self,
        enabled: bool,
    ) -> Self {
        self.show_phase = enabled;
        self
    }

    /// Return the configured output-byte budget.
    #[must_use]
    pub const fn max_output_bytes(&self) -> Option<usize> {
        self.max_output_bytes
    }

    /// Return the configured tab width.
    #[must_use]
    pub const fn tab_width(&self) -> NonZeroUsize {
        self.tab_width
    }
}

// ============================================================================
// RENDERING ERRORS
// ============================================================================

/// Failures encountered while rendering diagnostic output.
#[derive(Debug)]
#[non_exhaustive]
pub enum RenderingError {
    /// The source provider returned a different identity.
    SourceIdentityMismatch {
        /// Requested source identity.
        requested: u64,

        /// Returned source identity.
        returned: u64,
    },

    /// A source span exceeds the registered source document.
    InvalidSourceRange {
        /// Source identity.
        source_id: u64,

        /// Starting byte offset.
        start: u64,

        /// Ending byte offset.
        end: u64,
    },

    /// A source offset is not a UTF-8 character boundary.
    InvalidUtf8Boundary {
        /// Source identity.
        source_id: u64,

        /// Invalid byte offset.
        offset: usize,
    },

    /// A position cannot be represented on this host.
    OffsetNotRepresentable {
        /// Original source offset.
        offset: u64,
    },

    /// The requested output exceeds its configured budget.
    OutputBudgetExceeded {
        /// Configured byte budget.
        configured: usize,

        /// Minimum bytes required by the attempted write.
        required: usize,
    },

    /// Output allocation failed.
    AllocationFailure,

    /// Formatting failed.
    FormattingFailure,

    /// The destination writer failed.
    WriterFailure(io::Error),
}

impl fmt::Display for RenderingError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::SourceIdentityMismatch {
                requested,
                returned,
            } => write!(
                formatter,
                "source identity mismatch: requested {requested}, received {returned}"
            ),

            Self::InvalidSourceRange {
                source_id,
                start,
                end,
            } => write!(
                formatter,
                "invalid source range {start}..{end} for source {source_id}"
            ),

            Self::InvalidUtf8Boundary {
                source_id,
                offset,
            } => write!(
                formatter,
                "invalid UTF-8 boundary {offset} in source {source_id}"
            ),

            Self::OffsetNotRepresentable { offset } => write!(
                formatter,
                "source offset {offset} cannot be represented on this platform"
            ),

            Self::OutputBudgetExceeded {
                configured,
                required,
            } => write!(
                formatter,
                "diagnostic output requires at least {required} bytes; configured budget is {configured}"
            ),

            Self::AllocationFailure => {
                formatter.write_str("diagnostic rendering allocation failed")
            }

            Self::FormattingFailure => {
                formatter.write_str("diagnostic formatting failed")
            }

            Self::WriterFailure(error) => {
                write!(formatter, "diagnostic output writer failed: {error}")
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

// ============================================================================
// OUTPUT BUFFER
// ============================================================================

/// Internal checked output buffer.
///
/// Output limits are enforced before each append.
struct RenderBuffer {
    output: String,
    maximum: Option<usize>,
}

impl RenderBuffer {
    fn new(maximum: Option<usize>) -> Self {
        Self {
            output: String::new(),
            maximum,
        }
    }

    fn push_str(
        &mut self,
        value: &str,
    ) -> Result<(), RenderingError> {
        let required = self.output.len()
            .checked_add(value.len())
            .ok_or(RenderingError::AllocationFailure)?;

        if let Some(maximum) = self.maximum {
            if required > maximum {
                return Err(RenderingError::OutputBudgetExceeded {
                    configured: maximum,
                    required,
                });
            }
        }

        self.output
            .try_reserve(value.len())
            .map_err(|_| RenderingError::AllocationFailure)?;

        self.output.push_str(value);

        Ok(())
    }

    fn push_char(
        &mut self,
        character: char,
    ) -> Result<(), RenderingError> {
        let mut buffer = [0u8; 4];
        let encoded = character.encode_utf8(&mut buffer);

        self.push_str(encoded)
    }

    fn push_fmt(
        &mut self,
        arguments: fmt::Arguments<'_>,
    ) -> Result<(), RenderingError> {
        let mut temporary = String::new();

        fmt::write(&mut temporary, arguments)
            .map_err(|_| RenderingError::FormattingFailure)?;

        self.push_str(&temporary)
    }

    fn finish(self) -> String {
        self.output
    }
}

// ============================================================================
// UNTRUSTED PRESENTATION TEXT
// ============================================================================

/// Identify Unicode bidirectional formatting characters that can
/// visually reorder untrusted terminal output.
fn is_bidi_control(character: char) -> bool {
    matches!(
        character,
        '\u{061C}'
            | '\u{200E}'
            | '\u{200F}'
            | '\u{202A}'
            | '\u{202B}'
            | '\u{202C}'
            | '\u{202D}'
            | '\u{202E}'
            | '\u{2066}'
            | '\u{2067}'
            | '\u{2068}'
            | '\u{2069}'
    )
}

/// Append presentation text without allowing terminal control
/// sequences or bidi formatting controls to alter the output.
fn push_safe_text(
    output: &mut RenderBuffer,
    value: &str,
) -> Result<(), RenderingError> {
    for character in value.chars() {
        if character.is_control() || is_bidi_control(character) {
            output.push_str(&character.escape_debug().to_string())?;
        } else {
            output.push_char(character)?;
        }
    }

    Ok(())
}

/// Append a stable severity label.
fn push_severity(
    output: &mut RenderBuffer,
    severity: DiagnosticSeverity,
) -> Result<(), RenderingError> {
    output.push_str(match severity {
        DiagnosticSeverity::Hint => "hint",
        DiagnosticSeverity::Note => "note",
        DiagnosticSeverity::Information => "info",
        DiagnosticSeverity::Warning => "warning",
        DiagnosticSeverity::Error => "error",
        DiagnosticSeverity::Fatal => "fatal",
    })
}

// ============================================================================
// SOURCE LOCATION RESOLUTION
// ============================================================================

/// Resolved one-based presentation location.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SourceLocation {
    /// One-based line number.
    pub line: usize,

    /// One-based display column.
    pub column: usize,

    /// Zero-based UTF-8 byte offset.
    pub byte_offset: usize,
}

/// Calculate the display column of a source prefix.
///
/// Tabs advance to the next configured tab stop.
/// Other ordinary Unicode scalar values occupy their configured
/// presentation width.
fn calculate_column<W: DisplayWidth>(
    prefix: &str,
    width: &W,
    tab_width: usize,
) -> Result<usize, RenderingError> {
    let mut column = 0usize;

    for character in prefix.chars() {
        let advance = if character == '\t' {
            let remainder = column % tab_width;

            if remainder == 0 {
                tab_width
            } else {
                tab_width - remainder
            }
        } else if character.is_control() || is_bidi_control(character) {
            character.escape_debug().to_string().chars().count()
        } else {
            width.character_width(character)
        };

        column = column
            .checked_add(advance)
            .ok_or(RenderingError::AllocationFailure)?;
    }

    column
        .checked_add(1)
        .ok_or(RenderingError::AllocationFailure)
}

/// Identify whether a character is a source line terminator.
fn is_line_terminator(character: char) -> bool {
    matches!(
        character,
        '\n' | '\r' | '\u{0085}' | '\u{2028}' | '\u{2029}'
    )
}

/// Resolve a byte offset to its source line and display column.
///
/// This performs demand-driven scanning and does not construct
/// or retain a whole-file line index.
pub fn resolve_source_location<W: DisplayWidth>(
    source: &str,
    byte_offset: usize,
    width: &W,
    tab_width: NonZeroUsize,
) -> Result<Option<SourceLocation>, RenderingError> {
    if byte_offset > source.len() {
        return Ok(None);
    }

    if !source.is_char_boundary(byte_offset) {
        return Err(RenderingError::InvalidUtf8Boundary {
            source_id: 0,
            offset: byte_offset,
        });
    }

    let mut line = 1usize;
    let mut line_start = 0usize;
    let mut cursor = 0usize;

    while cursor < byte_offset {
        let character = source[cursor..]
            .chars()
            .next()
            .ok_or(RenderingError::FormattingFailure)?;

        if is_line_terminator(character) {
            if character == '\r'
                && source.as_bytes().get(cursor + 1) == Some(&b'\n')
            {
                cursor += 2;
            } else {
                cursor += character.len_utf8();
            }

            line = line
                .checked_add(1)
                .ok_or(RenderingError::AllocationFailure)?;

            line_start = cursor;
        } else {
            cursor += character.len_utf8();
        }
    }

    let prefix = source
        .get(line_start..byte_offset)
        .ok_or(RenderingError::FormattingFailure)?;

    let column = calculate_column(
        prefix,
        width,
        tab_width.get(),
    )?;

    Ok(Some(SourceLocation {
        line,
        column,
        byte_offset,
    }))
}

// ============================================================================
// SOURCE SPAN VALIDATION
// ============================================================================

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

// ============================================================================
// SOURCE EXCERPTS
// ============================================================================

/// Find the end of the source line containing a byte offset.
fn find_line_end(
    source: &str,
    start: usize,
) -> Result<usize, RenderingError> {
    if start > source.len() || !source.is_char_boundary(start) {
        return Err(RenderingError::FormattingFailure);
    }

    for (relative, character) in source[start..].char_indices() {
        if is_line_terminator(character) {
            return start
                .checked_add(relative)
                .ok_or(RenderingError::AllocationFailure);
        }
    }

    Ok(source.len())
}

/// Expand tabs and safely render a source line.
fn push_source_line<W: DisplayWidth>(
    output: &mut RenderBuffer,
    line: &str,
    width: &W,
    tab_width: usize,
) -> Result<usize, RenderingError> {
    let mut column = 0usize;

    for character in line.chars() {
        if character == '\t' {
            let remainder = column % tab_width;

            let spaces = if remainder == 0 {
                tab_width
            } else {
                tab_width - remainder
            };

            for _ in 0..spaces {
                output.push_char(' ')?;
            }

            column = column
                .checked_add(spaces)
                .ok_or(RenderingError::AllocationFailure)?;
        } else if character.is_control() || is_bidi_control(character) {
            let escaped = character.escape_debug().to_string();

            output.push_str(&escaped)?;

            column = column
                .checked_add(escaped.chars().count())
                .ok_or(RenderingError::AllocationFailure)?;
        } else {
            output.push_char(character)?;

            column = column
                .checked_add(width.character_width(character))
                .ok_or(RenderingError::AllocationFailure)?;
        }
    }

    Ok(column)
}

/// Render a source location, including its source excerpt when available.
///
/// Missing source documents are not rendering failures. In that case,
/// the canonical source identity and byte range are shown instead.
fn render_location<W: DisplayWidth>(
    output: &mut RenderBuffer,
    span: Span,
    provider: Option<&dyn SourceProvider>,
    options: &RenderingOptions,
    width: &W,
) -> Result<(), RenderingError> {
    let source_id = span.source().as_raw();

    let Some(provider) = provider else {
        output.push_fmt(format_args!(
            "  --> source#{source_id}:{}..{}\n",
            span.start_raw(),
            span.end_raw()
        ))?;

        return Ok(());
    };

    let Some(source) = provider.source(source_id) else {
        output.push_fmt(format_args!(
            "  --> source#{source_id}:{}..{}\n",
            span.start_raw(),
            span.end_raw()
        ))?;

        return Ok(());
    };

    if source.id != source_id {
        return Err(RenderingError::SourceIdentityMismatch {
            requested: source_id,
            returned: source.id,
        });
    }

    let (start, end) = validate_span(span, source)?;

    let Some(location) = resolve_source_location(
        source.text,
        start,
        width,
        options.tab_width(),
    )? else {
        output.push_fmt(format_args!(
            "  --> source#{source_id}:{}..{}\n",
            span.start_raw(),
            span.end_raw()
        ))?;

        return Ok(());
    };

    output.push_str("  --> ")?;
    push_safe_text(output, source.name)?;

    output.push_fmt(format_args!(
        ":{}:{}\n",
        location.line,
        location.column
    ))?;

    if !options.show_source {
        return Ok(());
    }

    let line_end = find_line_end(source.text, start)?;

    let line_text = source.text
        .get(
            source.text
                .char_indices()
                .take_while(|(index, _)| *index <= start)
                .last()
                .map(|(index, _)| {
                    let mut line_start = 0usize;

                    for (offset, character) in source.text[..index].char_indices() {
                        if is_line_terminator(character) {
                            line_start = offset + character.len_utf8();
                        }
                    }

                    line_start
                })
                .unwrap_or(0)
                ..line_end
        )
        .ok_or(RenderingError::FormattingFailure)?;

    let mut line_start = start;

    while line_start > 0 {
        let previous = source.text[..line_start]
            .char_indices()
            .next_back();

        let Some((previous_offset, previous_character)) = previous else {
            break;
        };

        if is_line_terminator(previous_character) {
            break;
        }

        line_start = previous_offset;
    }

    let line_text = source.text
        .get(line_start..line_end)
        .ok_or(RenderingError::FormattingFailure)?;

    output.push_fmt(format_args!(
        "   {line_number:>4} | ",
        line_number = location.line
    ))?;

    let _ = push_source_line(
        output,
        line_text,
        width,
        options.tab_width().get(),
    )?;

    output.push_char('\n')?;

    let prefix = source.text
        .get(line_start..start)
        .ok_or(RenderingError::FormattingFailure)?;

    let caret_column = calculate_column(
        prefix,
        width,
        options.tab_width().get(),
    )?;

    output.push_str("        | ")?;

    for _ in 1..caret_column {
        output.push_char(' ')?;
    }

    output.push_char('^')?;

    if end > start {
        let same_line_end = end.min(line_end);

        let selected = source.text
            .get(start..same_line_end)
            .ok_or(RenderingError::FormattingFailure)?;

        let selected_width = calculate_column(
            selected,
            width,
            options.tab_width().get(),
        )?
        .saturating_sub(1);

        for _ in 1..selected_width {
            output.push_char('~')?;
        }
    }

    output.push_char('\n')?;

    Ok(())
}

// ============================================================================
// FIX-IT RENDERING
// ============================================================================

fn render_fix(
    output: &mut RenderBuffer,
    fix: &DiagnosticFix,
    options: &RenderingOptions,
) -> Result<(), RenderingError> {
    output.push_str("  fix: ")?;
    push_safe_text(output, fix.title())?;

    if fix.is_safe() {
        output.push_str(" [producer-marked safe]")?;
    } else {
        output.push_str(" [review required]")?;
    }

    output.push_char('\n')?;

    for edit in fix.edits() {
        let span = edit.span();

        output.push_fmt(format_args!(
            "    source#{}:{}..{}\n",
            span.source().as_raw(),
            span.start_raw(),
            span.end_raw()
        ))?;

        if options.show_fix_replacements {
            output.push_str("      replacement: \"")?;
            push_safe_text(output, edit.replacement())?;
            output.push_str("\"\n")?;
        }
    }

    Ok(())
}

// ============================================================================
// SINGLE DIAGNOSTIC
// ============================================================================

/// Render one structured diagnostic into an owned string.
///
/// The source provider is optional. Without it, source locations are
/// represented by their canonical source IDs and byte ranges.
///
/// The renderer preserves the diagnostic's original information and
/// never changes severity, category, phase, provenance or source spans.
pub fn render_diagnostic(
    diagnostic: &ToolchainDiagnostic,
    provider: Option<&dyn SourceProvider>,
    options: &RenderingOptions,
) -> Result<String, RenderingError> {
    let mut output = RenderBuffer::new(options.max_output_bytes());

    render_diagnostic_into(
        &mut output,
        diagnostic,
        provider,
        options,
        &ScalarDisplayWidth,
    )?;

    Ok(output.finish())
}

/// Render one diagnostic into an existing output buffer.
///
/// This internal operation is shared by single and aggregate rendering.
fn render_diagnostic_into<W: DisplayWidth>(
    output: &mut RenderBuffer,
    diagnostic: &ToolchainDiagnostic,
    provider: Option<&dyn SourceProvider>,
    options: &RenderingOptions,
    width: &W,
) -> Result<(), RenderingError> {
    push_severity(output, diagnostic.severity())?;

    output.push_char('[')?;
    push_safe_text(output, diagnostic.code().as_str())?;
    output.push_str("]: ")?;

    push_safe_text(output, diagnostic.message())?;
    output.push_char('\n')?;

    if let Some(span) = diagnostic.primary_span() {
        render_location(
            output,
            span,
            provider,
            options,
            width,
        )?;
    }

    output.push_str("  category: ")?;
    push_safe_text(output, diagnostic.category().as_str())?;
    output.push_char('\n')?;

    if options.show_phase {
        output.push_str("  phase: ")?;
        push_safe_text(output, diagnostic.phase().as_str())?;
        output.push_char('\n')?;
    }

    if options.show_origin {
        let origin = diagnostic.origin();

        output.push_str("  origin: ")?;
        push_safe_text(output, origin.component())?;

        if let Some(operation) = origin.operation() {
            output.push_str(" / ")?;
            push_safe_text(output, operation)?;
        }

        if let Some(subject) = origin.subject() {
            output.push_str(" / subject: ")?;
            push_safe_text(output, subject)?;
        }

        if let Some(provider) = origin.provider() {
            output.push_str(" / provider: ")?;
            push_safe_text(output, provider)?;
        }

        output.push_char('\n')?;
    }

    for label in diagnostic.labels() {
        output.push_str("  related: ")?;
        push_safe_text(output, label.message())?;
        output.push_char('\n')?;

        render_location(
            output,
            label.span(),
            provider,
            options,
            width,
        )?;
    }

    for note in diagnostic.notes() {
        output.push_str("  note: ")?;
        push_safe_text(output, note.message())?;
        output.push_char('\n')?;
    }

    if let Some(help) = diagnostic.help() {
        output.push_str("  help: ")?;
        push_safe_text(output, help)?;
        output.push_char('\n')?;
    }

    for fix in diagnostic.fixes() {
        render_fix(output, fix, options)?;
    }

    Ok(())
}

// ============================================================================
// AGGREGATED DIAGNOSTICS
// ============================================================================

/// Render an aggregated diagnostic result.
///
/// The input order is preserved. This renderer does not independently
/// sort or deduplicate records.
///
/// An incomplete aggregate is explicitly identified, together with
/// the counters explaining why it may be incomplete.
pub fn render_aggregated(
    diagnostics: &AggregatedToolchainDiagnostics,
    provider: Option<&dyn SourceProvider>,
    options: &RenderingOptions,
) -> Result<String, RenderingError> {
    let mut output = RenderBuffer::new(options.max_output_bytes());

    let statistics = diagnostics.statistics();

    output.push_str("Zamani toolchain diagnostics\n")?;

    match diagnostics.status() {
        AggregationStatus::Complete => {
            output.push_str("status: complete\n")?;
        }

        AggregationStatus::Incomplete => {
            output.push_str(
                "status: INCOMPLETE — diagnostic information may be missing\n",
            )?;
        }
    }

    output.push_fmt(format_args!(
        "observed: {}\nretained: {}\nduplicates: {}\ndropped: {}\ninvalid: {}\n\n",
        statistics.observed(),
        statistics.retained(),
        statistics.duplicates(),
        statistics.dropped(),
        statistics.invalid(),
    ))?;

    for (index, diagnostic) in diagnostics.diagnostics().iter().enumerate() {
        if index > 0 {
            output.push_char('\n')?;
        }

        render_diagnostic_into(
            &mut output,
            diagnostic,
            provider,
            options,
            &ScalarDisplayWidth,
        )?;
    }

    Ok(output.finish())
}

// ============================================================================
// WRITER INTEGRATION
// ============================================================================

/// Render one diagnostic to a caller-provided byte writer.
///
/// The destination writer may contain partial output if it fails.
/// Callers must treat an error as an incomplete rendering operation.
///
/// This API does not close, flush, own or otherwise manage the writer.
pub fn render_diagnostic_to<W: IoWrite>(
    writer: &mut W,
    diagnostic: &ToolchainDiagnostic,
    provider: Option<&dyn SourceProvider>,
    options: &RenderingOptions,
) -> Result<(), RenderingError> {
    let rendered = render_diagnostic(
        diagnostic,
        provider,
        options,
    )?;

    writer
        .write_all(rendered.as_bytes())
        .map_err(RenderingError::WriterFailure)
}

/// Render an aggregated result to a caller-provided byte writer.
pub fn render_aggregated_to<W: IoWrite>(
    writer: &mut W,
    diagnostics: &AggregatedToolchainDiagnostics,
    provider: Option<&dyn SourceProvider>,
    options: &RenderingOptions,
) -> Result<(), RenderingError> {
    let rendered = render_aggregated(
        diagnostics,
        provider,
        options,
    )?;

    writer
        .write_all(rendered.as_bytes())
        .map_err(RenderingError::WriterFailure)
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::frontend::ast::node::source::span::{
        SourceOffset,
    };

    use super::super::diagnostic::{
        DiagnosticCode,
        DiagnosticLabel,
        DiagnosticNote,
        DiagnosticOrigin,
    };

    struct TestSources {
        files: Vec<SourceFileRef<'static>>,
    }

    impl SourceProvider for TestSources {
        fn source(
            &self,
            source_id: u64,
        ) -> Option<SourceFileRef<'_>> {
            self.files
                .iter()
                .find(|file| file.id == source_id)
                .copied()
        }
    }

    fn source_span(
        source: u64,
        start: u64,
        end: u64,
    ) -> Span {
        Span::new(
            SourceId::from_raw(source),
            SourceOffset::from_raw(start),
            SourceOffset::from_raw(end),
        )
        .expect("test span must be valid")
    }

    fn diagnostic() -> ToolchainDiagnostic {
        ToolchainDiagnostic::new(
            DiagnosticCode::new("toolchain.build.failed"),
            DiagnosticSeverity::Error,
            "build",
            "build_execution",
            "Compilation failed.",
            DiagnosticOrigin::new("toolchain.build")
                .with_operation("build.execute"),
        )
    }

    fn sources() -> TestSources {
        TestSources {
            files: vec![
                SourceFileRef::new(
                    1,
                    "example.zm",
                    "let value = 42;\nlet result = value;\n",
                ),
            ],
        }
    }

    #[test]
    fn renders_structured_diagnostic() {
        let rendered = render_diagnostic(
            &diagnostic(),
            None,
            &RenderingOptions::default(),
        )
        .expect("rendering should succeed");

        assert!(rendered.contains("error[toolchain.build.failed]"));
        assert!(rendered.contains("Compilation failed."));
        assert!(rendered.contains("category: build"));
        assert!(rendered.contains("phase: build_execution"));
    }

    #[test]
    fn renders_source_location_and_excerpt() {
        let item = diagnostic()
            .with_primary_span(source_span(1, 4, 9));

        let rendered = render_diagnostic(
            &item,
            Some(&sources()),
            &RenderingOptions::default(),
        )
        .expect("source rendering should succeed");

        assert!(rendered.contains("example.zm:1:5"));
        assert!(rendered.contains("let value = 42;"));
        assert!(rendered.contains('^'));
    }

    #[test]
    fn resolves_utf8_byte_offsets_correctly() {
        let source = "aλb\nnext";

        let location = resolve_source_location(
            source,
            3,
            &ScalarDisplayWidth,
            NonZeroUsize::MIN,
        )
        .expect("location resolution should succeed")
        .expect("location should exist");

        assert_eq!(location.line, 1);
        assert_eq!(location.column, 3);
        assert_eq!(location.byte_offset, 3);
    }

    #[test]
    fn resolves_second_line() {
        let source = "first\nsecond";

        let location = resolve_source_location(
            source,
            6,
            &ScalarDisplayWidth,
            NonZeroUsize::MIN,
        )
        .expect("location resolution should succeed")
        .expect("location should exist");

        assert_eq!(location.line, 2);
        assert_eq!(location.column, 1);
    }

    #[test]
    fn expands_tabs_using_configured_width() {
        let source = "\tvalue";

        let location = resolve_source_location(
            source,
            1,
            &ScalarDisplayWidth,
            NonZeroUsize::new(4).expect("nonzero"),
        )
        .expect("location resolution should succeed")
        .expect("location should exist");

        assert_eq!(location.column, 5);
    }

    #[test]
    fn renders_secondary_labels_and_notes() {
        let item = diagnostic()
            .with_label(DiagnosticLabel::new(
                source_span(1, 4, 9),
                "Related declaration.",
            ))
            .with_note(DiagnosticNote::new(
                "Check the dependency configuration.",
            ));

        let rendered = render_diagnostic(
            &item,
            Some(&sources()),
            &RenderingOptions::default(),
        )
        .expect("rendering should succeed");

        assert!(rendered.contains("related: Related declaration."));
        assert!(rendered.contains("note: Check the dependency configuration."));
    }

    #[test]
    fn escapes_terminal_control_characters() {
        let item = ToolchainDiagnostic::new(
            "toolchain.test",
            DiagnosticSeverity::Warning,
            "test",
            "testing",
            "unsafe \u{001B}[31m text",
            DiagnosticOrigin::new("test"),
        );

        let rendered = render_diagnostic(
            &item,
            None,
            &RenderingOptions::default(),
        )
        .expect("rendering should succeed");

        assert!(!rendered.contains('\u{001B}'));
        assert!(rendered.contains("\\u{1b}"));
    }

    #[test]
    fn escapes_bidirectional_formatting_controls() {
        let item = ToolchainDiagnostic::new(
            "toolchain.test",
            DiagnosticSeverity::Warning,
            "test",
            "testing",
            "text \u{202E}hidden",
            DiagnosticOrigin::new("test"),
        );

        let rendered = render_diagnostic(
            &item,
            None,
            &RenderingOptions::default(),
        )
        .expect("rendering should succeed");

        assert!(!rendered.contains('\u{202E}'));
        assert!(rendered.contains("\\u{202e}"));
    }

    #[test]
    fn source_can_be_hidden() {
        let item = diagnostic()
            .with_primary_span(source_span(1, 4, 9));

        let options = RenderingOptions::default()
            .with_source(false);

        let rendered = render_diagnostic(
            &item,
            Some(&sources()),
            &options,
        )
        .expect("rendering should succeed");

        assert!(rendered.contains("example.zm:1:5"));
        assert!(!rendered.contains("let value"));
    }

    #[test]
    fn missing_source_falls_back_to_canonical_byte_range() {
        let item = diagnostic()
            .with_primary_span(source_span(99, 10, 15));

        let rendered = render_diagnostic(
            &item,
            Some(&sources()),
            &RenderingOptions::default(),
        )
        .expect("missing source should not prevent rendering");

        assert!(rendered.contains("source#99:10..15"));
    }

    #[test]
    fn rejects_mismatched_source_identity() {
        struct WrongSource;

        impl SourceProvider for WrongSource {
            fn source(
                &self,
                _source_id: u64,
            ) -> Option<SourceFileRef<'_>> {
                Some(SourceFileRef::new(
                    2,
                    "wrong.zm",
                    "some source",
                ))
            }
        }

        let item = diagnostic()
            .with_primary_span(source_span(1, 0, 1));

        let result = render_diagnostic(
            &item,
            Some(&WrongSource),
            &RenderingOptions::default(),
        );

        assert!(matches!(
            result,
            Err(RenderingError::SourceIdentityMismatch { .. })
        ));
    }

    #[test]
    fn rejects_invalid_utf8_boundary() {
        let source = "aλb";

        let result = resolve_source_location(
            source,
            2,
            &ScalarDisplayWidth,
            NonZeroUsize::MIN,
        );

        assert!(matches!(
            result,
            Err(RenderingError::InvalidUtf8Boundary { .. })
        ));
    }

    #[test]
    fn enforces_explicit_output_budget() {
        let options = RenderingOptions::default()
            .with_max_output_bytes(Some(8));

        let result = render_diagnostic(
            &diagnostic(),
            None,
            &options,
        );

        assert!(matches!(
            result,
            Err(RenderingError::OutputBudgetExceeded { .. })
        ));
    }

    #[test]
    fn writer_api_produces_the_same_output() {
        let item = diagnostic();

        let expected = render_diagnostic(
            &item,
            None,
            &RenderingOptions::default(),
        )
        .expect("rendering should succeed");

        let mut output = Vec::new();

        render_diagnostic_to(
            &mut output,
            &item,
            None,
            &RenderingOptions::default(),
        )
        .expect("writer rendering should succeed");

        assert_eq!(
            String::from_utf8(output).expect("valid UTF-8"),
            expected
        );
    }

    #[test]
    fn default_options_have_no_output_ceiling() {
        assert_eq!(
            RenderingOptions::default().max_output_bytes(),
            None
        );
    }
}
