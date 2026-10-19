
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! VHDL interoperability boundary for Zamani HDL.
//!
//! Path: `src/hdl/interoperability/vhdl.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Authority
//!
//! - `grammar/spec/hdl.md`
//! - `grammar/spec/interoperability.md`
//! - `grammar/hdl/hdl.g4`
//! - `grammar/interoperability/hdl.g4`
//! - `src/frontend/parser/domains/hdl.rs`
//! - `src/frontend/semantic/domains/hdl.rs`
//! - `src/hdl/interoperability/verilog.rs`
//!
//! # Purpose
//!
//! Provide a source-preserving structural scanner for VHDL design units.
//! The scanner identifies common entity, architecture, package, package-body,
//! configuration, and context declaration boundaries.
//!
//! This is a structural discovery API, not a complete IEEE 1076 parser,
//! elaborator, type checker, simulator, or synthesizer.
//!
//! # Ownership
//!
//! This module owns:
//! - VHDL-specific scan configuration and result types;
//! - VHDL lexical handling required for structural discovery;
//! - source byte ranges into the original UTF-8 input;
//! - structural diagnostics and explicit scan-completeness information.
//!
//! This module does not own:
//! - the Zamani lexer, canonical AST, source map, or semantic model;
//! - a replacement Zamani HDL grammar;
//! - complete VHDL syntax or semantic validation;
//! - library/work resolution, package elaboration, or configuration binding;
//! - target discovery, synthesis, physical implementation, or HDLIR.
//!
//! All HDLIR implementation remains under `src/hdl/hdlir/` and is out of
//! scope for this module.
//!
//! # Integration
//!
//! 1. The caller supplies original VHDL source and a scan configuration.
//! 2. `scan_vhdl` identifies likely design-unit boundaries and diagnostics.
//! 3. The caller maps source ranges to the canonical frontend source/span API.
//! 4. A VHDL grammar parser or selected external VHDL tool performs full parsing.
//! 5. A dedicated adapter maps supported constructs into the canonical AST and
//!    semantic model, recording unsupported or lossy conversions explicitly.
//!
//! The scanner never treats a successful structural scan as proof that the
//! VHDL source is valid.
//!
//! # Scalability
//!
//! There is no universal source-size or design-unit-count ceiling. Collections
//! grow dynamically. An optional source-byte budget is an operational policy,
//! not a VHDL language restriction. Finite processing remains subject to
//! available memory, time, and caller-configured budgets.
//!
//! # Freeze criteria
//!
//! - Returned ranges refer to valid UTF-8 boundaries in the original source.
//! - Comments, strings, character literals, and extended identifiers are not
//!   mistaken for structural keywords.
//! - Diagnostics and discovered units preserve source order.
//! - Full VHDL grammar and semantic validation remain separate.
//! - Integration and conformance tests pass.

use std::error::Error;
use std::fmt;
use std::ops::Range;

/// VHDL design-unit category recognized by the structural scanner.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub enum VhdlUnitKind {
    /// An `entity` declaration.
    Entity,
    /// An `architecture` body.
    Architecture,
    /// A `package` declaration.
    Package,
    /// A `package body` declaration.
    PackageBody,
    /// A `configuration` declaration.
    Configuration,
    /// A VHDL-2008 `context` declaration.
    Context,
}

impl VhdlUnitKind {
    fn keyword(self) -> &'static str {
        match self {
            Self::Entity => "entity",
            Self::Architecture => "architecture",
            Self::Package => "package",
            Self::PackageBody => "package body",
            Self::Configuration => "configuration",
            Self::Context => "context",
        }
    }

    fn from_keyword(value: &str) -> Option<Self> {
        if value.eq_ignore_ascii_case("entity") {
            Some(Self::Entity)
        } else if value.eq_ignore_ascii_case("architecture") {
            Some(Self::Architecture)
        } else if value.eq_ignore_ascii_case("package") {
            Some(Self::Package)
        } else if value.eq_ignore_ascii_case("configuration") {
            Some(Self::Configuration)
        } else if value.eq_ignore_ascii_case("context") {
            Some(Self::Context)
        } else {
            None
        }
    }
}

/// Configuration for one VHDL structural scan.
///
/// Operational budgets do not change VHDL language semantics.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VhdlScanConfig {
    /// Optional maximum input size in UTF-8 bytes.
    ///
    /// `None` imposes no scanner-defined source-size budget.
    pub max_source_bytes: Option<usize>,

    /// Whether an unmatched design-unit `end` should produce an error
    /// diagnostic instead of being silently ignored.
    pub report_unmatched_end: bool,
}

impl Default for VhdlScanConfig {
    fn default() -> Self {
        Self {
            max_source_bytes: None,
            report_unmatched_end: true,
        }
    }
}

impl VhdlScanConfig {
    /// Construct a configuration with no scanner-defined source-size budget.
    #[must_use]
    pub const fn unbounded() -> Self {
        Self {
            max_source_bytes: None,
            report_unmatched_end: true,
        }
    }
}

/// A checked, half-open UTF-8 byte range into the original source.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct VhdlSourceRange {
    /// Inclusive start byte offset.
    pub start: usize,
    /// Exclusive end byte offset.
    pub end: usize,
}

impl VhdlSourceRange {
    /// Create a range, rejecting an end before its start.
    pub fn new(start: usize, end: usize) -> Result<Self, VhdlError> {
        if end < start {
            return Err(VhdlError::InvalidRange { start, end });
        }
        Ok(Self { start, end })
    }

    /// Return this range as a standard half-open Rust range.
    #[must_use]
    pub fn as_range(&self) -> Range<usize> {
        self.start..self.end
    }

    /// Return the number of bytes in this range.
    #[must_use]
    pub fn len(&self) -> usize {
        self.end - self.start
    }

    /// Return whether this range is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }

    /// Return the corresponding source slice if the range is valid for `source`.
    ///
    /// A range must not be reused against a different source without validation.
    #[must_use]
    pub fn slice<'a>(&self, source: &'a str) -> Option<&'a str> {
        source.get(self.as_range())
    }
}

/// A structurally discovered VHDL design unit.
///
/// The scanner preserves spelling and ranges; it does not build a VHDL AST.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VhdlDesignUnit {
    /// Recognized design-unit category.
    pub kind: VhdlUnitKind,
    /// Declared unit name, preserving source spelling.
    pub name: String,
    /// Source range covering the leading design-unit keyword.
    pub declaration_keyword: VhdlSourceRange,
    /// Source range covering the declared unit name.
    pub name_range: VhdlSourceRange,
    /// Source range covering the matching design-unit `end`, if found.
    pub end_keyword: Option<VhdlSourceRange>,
    /// Complete unit range from its opening keyword through the end statement.
    ///
    /// This is `None` when the scanner could not find a matching end statement.
    pub full_range: Option<VhdlSourceRange>,
}

impl VhdlDesignUnit {
    /// Return the complete unit text if both boundaries were found.
    #[must_use]
    pub fn source<'a>(&self, source: &'a str) -> Option<&'a str> {
        self.full_range.as_ref()?.slice(source)
    }

    /// Return whether a matching end statement was found.
    #[must_use]
    pub const fn is_closed(&self) -> bool {
        self.end_keyword.is_some()
    }
}

/// Severity of a VHDL scanner diagnostic.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum VhdlSeverity {
    /// Informational note.
    Info,
    /// The caller should inspect the condition before relying on scan results.
    Warning,
    /// A structural problem was detected.
    Error,
}

/// Stable category for a VHDL structural diagnostic.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub enum VhdlDiagnosticCode {
    /// A design-unit keyword was not followed by a discoverable name.
    MissingDesignUnitName,
    /// An end statement did not match the currently open design unit.
    UnmatchedEnd,
    /// A design unit had no discoverable closing end statement.
    UnclosedDesignUnit,
    /// Structural scanning is not full VHDL grammar or semantic validation.
    StructuralValidationIncomplete,
}

/// A deterministic diagnostic emitted by the scanner.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VhdlDiagnostic {
    /// Stable diagnostic category.
    pub code: VhdlDiagnosticCode,
    /// Diagnostic severity.
    pub severity: VhdlSeverity,
    /// Human-readable explanation.
    pub message: String,
    /// Optional source range associated with this diagnostic.
    pub range: Option<VhdlSourceRange>,
}

/// A structurally scanned VHDL source document.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VhdlDocument<'source> {
    /// Original source, borrowed without rewriting or normalization.
    pub source: &'source str,
    /// Discovered design units in source order.
    pub units: Vec<VhdlDesignUnit>,
    /// Diagnostics in deterministic source order.
    pub diagnostics: Vec<VhdlDiagnostic>,
    /// Whether lexical scanning reached the end of the source.
    ///
    /// This does not certify full VHDL grammar or semantic validity.
    pub scan_complete: bool,
}

impl<'source> VhdlDocument<'source> {
    /// Return the source slice for a discovered design unit.
    #[must_use]
    pub fn unit_source(&self, unit: &VhdlDesignUnit) -> Option<&'source str> {
        unit.source(self.source)
    }

    /// Return whether any diagnostic has error severity.
    #[must_use]
    pub fn has_structural_errors(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|diagnostic| diagnostic.severity == VhdlSeverity::Error)
    }
}

/// Errors that prevent structural scanning from completing.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum VhdlError {
    /// The source exceeded the configured operational byte budget.
    SourceBudgetExceeded {
        /// Actual source size in bytes.
        actual: usize,
        /// Configured source budget in bytes.
        budget: usize,
    },
    /// A range ended before its start.
    InvalidRange {
        /// Requested start offset.
        start: usize,
        /// Requested end offset.
        end: usize,
    },
    /// A VHDL block comment (`--`) needs no terminator; this variant is
    /// reserved for lexical errors involving a quoted string.
    UnterminatedString {
        /// Byte offset of the opening quote.
        offset: usize,
    },
    /// An extended identifier was not terminated.
    UnterminatedExtendedIdentifier {
        /// Byte offset of the opening backslash.
        offset: usize,
    },
    /// A source offset could not be used as a UTF-8 boundary.
    InvalidSourceBoundary {
        /// Offending byte offset.
        offset: usize,
    },
}

impl fmt::Display for VhdlError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SourceBudgetExceeded { actual, budget } => write!(
                f,
                "VHDL source size {actual} bytes exceeds configured operational budget of {budget} bytes"
            ),
            Self::InvalidRange { start, end } => {
                write!(f, "invalid VHDL source range {start}..{end}")
            }
            Self::UnterminatedString { offset } => {
                write!(f, "unterminated VHDL string literal at byte {offset}")
            }
            Self::UnterminatedExtendedIdentifier { offset } => write!(
                f,
                "unterminated VHDL extended identifier at byte {offset}"
            ),
            Self::InvalidSourceBoundary { offset } => {
                write!(f, "invalid VHDL source boundary at byte {offset}")
            }
        }
    }
}

impl Error for VhdlError {}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Token {
    text: String,
    range: VhdlSourceRange,
    kind: TokenKind,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum TokenKind {
    Word,
    Symbol,
}

/// Scan VHDL source for common design-unit boundaries.
///
/// The scanner recognizes common entity, architecture, package, package-body,
/// configuration, and context declarations. VHDL keywords are case-insensitive.
/// Extended identifiers preserve their exact spelling.
///
/// Comments, strings, and character literals are skipped for boundary
/// recognition. The source is never rewritten.
///
/// A successful result means the structural scan completed, not that the source
/// is a valid VHDL design.
///
/// # Errors
///
/// Returns an error if the configured source-byte budget is exceeded, a string
/// or extended identifier is unterminated, or an internal source boundary is
/// invalid.
pub fn scan_vhdl<'source>(
    source: &'source str,
    config: &VhdlScanConfig,
) -> Result<VhdlDocument<'source>, VhdlError> {
    if let Some(budget) = config.max_source_bytes {
        if source.len() > budget {
            return Err(VhdlError::SourceBudgetExceeded {
                actual: source.len(),
                budget,
            });
        }
    }

    let tokens = lex(source)?;
    let mut units: Vec<VhdlDesignUnit> = Vec::new();
    let mut diagnostics: Vec<VhdlDiagnostic> = Vec::new();
    let mut open_unit: Option<usize> = None;

    let mut index = 0;
    while index < tokens.len() {
        if let Some((kind, name_index)) = detect_unit_start(&tokens, index) {
            let name_token = &tokens[name_index];
            let name = name_token
                .range
                .slice(source)
                .ok_or(VhdlError::InvalidSourceBoundary {
                    offset: name_token.range.start,
                })?
                .to_owned();

            // VHDL design units are separate compilation units, not nested
            // units. If a new declaration appears before the prior one closes,
            // report the prior one as incomplete and recover at this boundary.
            if let Some(previous_index) = open_unit.take() {
                let previous = &units[previous_index];
                diagnostics.push(VhdlDiagnostic {
                    code: VhdlDiagnosticCode::UnclosedDesignUnit,
                    severity: VhdlSeverity::Error,
                    message: format!(
                        "{} `{}` has no discoverable matching end before the next design unit",
                        previous.kind.keyword(),
                        previous.name
                    ),
                    range: Some(previous.declaration_keyword.clone()),
                });
            }

            let unit_index = units.len();
            units.push(VhdlDesignUnit {
                kind,
                name,
                declaration_keyword: tokens[index].range.clone(),
                name_range: name_token.range.clone(),
                end_keyword: None,
                full_range: None,
            });
            open_unit = Some(unit_index);
            index = name_index + 1;
            continue;
        }

        if is_word(&tokens[index], "end") {
            if let Some((end_token_index, consumed_end)) =
                detect_unit_end(&tokens, index, open_unit.map(|i| &units[i]))
            {
                if let Some(unit_index) = open_unit.take() {
                    let unit = &mut units[unit_index];
                    let end_range = tokens[end_token_index].range.clone();
                    unit.end_keyword = Some(end_range.clone());
                    unit.full_range = Some(VhdlSourceRange {
                        start: unit.declaration_keyword.start,
                        end: end_range.end,
                    });
                } else if config.report_unmatched_end {
                    diagnostics.push(VhdlDiagnostic {
                        code: VhdlDiagnosticCode::UnmatchedEnd,
                        severity: VhdlSeverity::Error,
                        message: "VHDL design-unit end statement has no matching open design unit"
                            .to_owned(),
                        range: Some(tokens[end_token_index].range.clone()),
                    });
                }
                index = consumed_end;
                continue;
            }
        }

        index += 1;
    }

    if let Some(unit_index) = open_unit {
        let unit = &units[unit_index];
        diagnostics.push(VhdlDiagnostic {
            code: VhdlDiagnosticCode::UnclosedDesignUnit,
            severity: VhdlSeverity::Error,
            message: format!(
                "{} `{}` has no discoverable matching end statement",
                unit.kind.keyword(),
                unit.name
            ),
            range: Some(unit.declaration_keyword.clone()),
        });
    }

    // This notice is intentionally unconditional: structural discovery is not
    // full VHDL parsing, elaboration, name resolution, or semantic validation.
    diagnostics.push(VhdlDiagnostic {
        code: VhdlDiagnosticCode::StructuralValidationIncomplete,
        severity: VhdlSeverity::Info,
        message: "structural scanning does not validate complete VHDL grammar or semantics"
            .to_owned(),
        range: None,
    });

    diagnostics.sort_by(|left, right| {
        let left_start = left.range.as_ref().map_or(usize::MAX, |r| r.start);
        let right_start = right.range.as_ref().map_or(usize::MAX, |r| r.start);
        left_start
            .cmp(&right_start)
            .then_with(|| left.code.cmp(&right.code))
            .then_with(|| left.message.cmp(&right.message))
    });

    Ok(VhdlDocument {
        source,
        units,
        diagnostics,
        scan_complete: true,
    })
}

fn detect_unit_start(tokens: &[Token], index: usize) -> Option<(VhdlUnitKind, usize)> {
    let first = tokens.get(index)?;

    if is_word(first, "entity")
        || is_word(first, "architecture")
        || is_word(first, "configuration")
        || is_word(first, "context")
    {
        let kind = VhdlUnitKind::from_keyword(&first.text)?;
        let name_index = index + 1;
        let name = tokens.get(name_index)?;
        if name.kind != TokenKind::Word {
            return None;
        }

        let expected = match kind {
            VhdlUnitKind::Entity | VhdlUnitKind::Context => name_index + 1,
            VhdlUnitKind::Architecture | VhdlUnitKind::Configuration => {
                if !tokens.get(name_index + 1).is_some_and(|t| is_word(t, "of")) {
                    return None;
                }
                name_index + 2
            }
            VhdlUnitKind::Package | VhdlUnitKind::PackageBody => return None,
        };

        if tokens.get(expected).is_some_and(|t| is_word(t, "is")) {
            return Some((kind, name_index));
        }
        return None;
    }

    if is_word(first, "package") {
        if tokens.get(index + 1).is_some_and(|t| is_word(t, "body")) {
            let name_index = index + 2;
            let name = tokens.get(name_index)?;
            if name.kind == TokenKind::Word
                && tokens.get(name_index + 1).is_some_and(|t| is_word(t, "is"))
            {
                return Some((VhdlUnitKind::PackageBody, name_index));
            }
        } else {
            let name_index = index + 1;
            let name = tokens.get(name_index)?;
            if name.kind == TokenKind::Word
                && tokens.get(name_index + 1).is_some_and(|t| is_word(t, "is"))
            {
                return Some((VhdlUnitKind::Package, name_index));
            }
        }
    }

    None
}

/// Recognize a design-unit end without confusing `end if`, `end process`,
/// `end loop`, and similar nested-language constructs with unit terminators.
///
/// Returns the `end` token index and the next scan index when a unit end is
/// recognized. The scanner is conservative and does not claim full grammar
/// correctness.
fn detect_unit_end(
    tokens: &[Token],
    index: usize,
    current: Option<&VhdlDesignUnit>,
) -> Option<(usize, usize)> {
    let next_index = index + 1;
    let next = tokens.get(next_index)?;

    if is_symbol(next, ";") {
        // A bare `end;` is accepted as a unit terminator by this structural
        // scanner when a unit is open. Full grammar validation is separate.
        return current.map(|_| (index, next_index + 1));
    }

    if let Some(mut end_kind) = VhdlUnitKind::from_keyword(&next.text)
        .filter(|_| next.kind == TokenKind::Word)
    {
        let mut cursor = next_index + 1;

        // `end package body name;` is a distinct spelling.
        if end_kind == VhdlUnitKind::Package
            && tokens.get(cursor).is_some_and(|t| is_word(t, "body"))
        {
            end_kind = VhdlUnitKind::PackageBody;
            cursor += 1;
        }

        let possible_name = tokens.get(cursor);
        let has_name = possible_name.is_some_and(|t| t.kind == TokenKind::Word);
        if has_name {
            cursor += 1;
        }

        if !tokens.get(cursor).is_some_and(|t| is_symbol(t, ";")) {
            return None;
        }

        let Some(unit) = current else {
            return Some((index, cursor + 1));
        };

        if end_kind != unit.kind {
            return None;
        }

        if has_name && !names_equal(&unit.name, &possible_name?.text) {
            return None;
        }

        return Some((index, cursor + 1));
    }

    // The abbreviated form `end unit_name;` is recognized only when the
    // name matches the open unit. Other `end <identifier>` statements belong
    // to nested VHDL constructs and are left to the real parser.
    if next.kind == TokenKind::Word
        && tokens
            .get(next_index + 1)
            .is_some_and(|t| is_symbol(t, ";"))
    {
        if let Some(unit) = current {
            if names_equal(&unit.name, &next.text) {
                return Some((index, next_index + 2));
            }
        }
    }

    None
}

fn names_equal(left: &str, right: &str) -> bool {
    // Basic identifiers are case-insensitive. Extended identifiers, which
    // retain their enclosing backslashes, are case-sensitive.
    if left.starts_with('\\') || right.starts_with('\\') {
        left == right
    } else {
        left.eq_ignore_ascii_case(right)
    }
}

fn is_word(token: &Token, word: &str) -> bool {
    token.kind == TokenKind::Word && token.text.eq_ignore_ascii_case(word)
}

fn is_symbol(token: &Token, symbol: &str) -> bool {
    token.kind == TokenKind::Symbol && token.text == symbol
}

fn lex(source: &str) -> Result<Vec<Token>, VhdlError> {
    let bytes = source.as_bytes();
    let mut tokens = Vec::new();
    let mut index = 0;

    while index < bytes.len() {
        let byte = bytes[index];

        if byte.is_ascii_whitespace() {
            index += 1;
            continue;
        }

        // VHDL line comments start with two hyphens and end at a line break.
        if byte == b'-' && bytes.get(index + 1) == Some(&b'-') {
            index += 2;
            while index < bytes.len() && bytes[index] != b'\n' && bytes[index] != b'\r' {
                index += 1;
            }
            continue;
        }

        // String literals use doubled quotes to represent a quote within
        // the literal. A quote inside a string is never a keyword boundary.
        if byte == b'"' {
            let start = index;
            index += 1;
            let mut closed = false;

            while index < bytes.len() {
                if bytes[index] == b'"' {
                    if bytes.get(index + 1) == Some(&b'"') {
                        index += 2;
                    } else {
                        index += 1;
                        closed = true;
                        break;
                    }
                } else if bytes[index].is_ascii() {
                    index += 1;
                } else {
                    let tail = source.get(index..).ok_or(
                        VhdlError::InvalidSourceBoundary { offset: index },
                    )?;
                    let ch = tail.chars().next().ok_or(
                        VhdlError::InvalidSourceBoundary { offset: index },
                    )?;
                    index += ch.len_utf8();
                }
            }

            if !closed {
                return Err(VhdlError::UnterminatedString { offset: start });
            }
            continue;
        }

        // Extended identifiers are delimited by backslashes. Their contents
        // are opaque and must not be reclassified as VHDL keywords.
        if byte == b'\\' {
            let start = index;
            index += 1;
            let mut closed = false;

            while index < bytes.len() {
                if bytes[index] == b'\\' {
                    index += 1;
                    closed = true;
                    break;
                }
                if bytes[index].is_ascii() {
                    index += 1;
                } else {
                    let tail = source.get(index..).ok_or(
                        VhdlError::InvalidSourceBoundary { offset: index },
                    )?;
                    let ch = tail.chars().next().ok_or(
                        VhdlError::InvalidSourceBoundary { offset: index },
                    )?;
                    index += ch.len_utf8();
                }
            }

            if !closed {
                return Err(VhdlError::UnterminatedExtendedIdentifier { offset: start });
            }

            tokens.push(Token {
                text: source[start..index].to_owned(),
                range: VhdlSourceRange { start, end: index },
                kind: TokenKind::Word,
            });
            continue;
        }

        // Skip a character literal such as '0' or 'A'. An attribute tick
        // such as signal'event does not satisfy this three-part pattern.
        if byte == b'\'' {
            if let Some(after_character) = character_literal_end(source, index) {
                index = after_character;
                continue;
            }
        }

        if byte.is_ascii_alphabetic() {
            let start = index;
            index += 1;
            while index < bytes.len()
                && (bytes[index].is_ascii_alphanumeric() || bytes[index] == b'_')
            {
                index += 1;
            }
            tokens.push(Token {
                text: source[start..index].to_owned(),
                range: VhdlSourceRange { start, end: index },
                kind: TokenKind::Word,
            });
            continue;
        }

        // Numbers and punctuation are opaque to this structural scanner.
        // Advance over one complete UTF-8 scalar to preserve valid ranges.
        let start = index;
        if byte.is_ascii() {
            index += 1;
        } else {
            let tail = source.get(index..).ok_or(
                VhdlError::InvalidSourceBoundary { offset: index },
            )?;
            let ch = tail.chars().next().ok_or(
                VhdlError::InvalidSourceBoundary { offset: index },
            )?;
            index += ch.len_utf8();
        }

        tokens.push(Token {
            text: source[start..index].to_owned(),
            range: VhdlSourceRange { start, end: index },
            kind: TokenKind::Symbol,
        });
    }

    Ok(tokens)
}

fn character_literal_end(source: &str, quote_index: usize) -> Option<usize> {
    let after_open = quote_index.checked_add(1)?;
    let tail = source.get(after_open..)?;
    let character = tail.chars().next()?;
    let after_character = after_open.checked_add(character.len_utf8())?;
    if source.as_bytes().get(after_character) == Some(&b'\'') {
        after_character.checked_add(1)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scan(source: &str) -> VhdlDocument<'_> {
        scan_vhdl(source, &VhdlScanConfig::default())
            .expect("test source should be lexically scannable")
    }

    #[test]
    fn discovers_entity_and_architecture() {
        let source = r#"
            entity Counter is
            end entity Counter;

            architecture RTL of Counter is
            begin
            end architecture rtl;
        "#;
        let document = scan(source);

        assert_eq!(document.units.len(), 2);
        assert_eq!(document.units[0].kind, VhdlUnitKind::Entity);
        assert_eq!(document.units[0].name, "Counter");
        assert!(document.units[0].is_closed());
        assert_eq!(document.units[1].kind, VhdlUnitKind::Architecture);
        assert_eq!(document.units[1].name, "RTL");
        assert!(document.units[1].is_closed());
        assert!(!document.has_structural_errors());
    }

    #[test]
    fn recognizes_package_body_as_a_distinct_unit() {
        let source = r#"
            package Math is
            end package Math;

            package body Math is
            end package body Math;
        "#;
        let document = scan(source);

        assert_eq!(document.units.len(), 2);
        assert_eq!(document.units[0].kind, VhdlUnitKind::Package);
        assert_eq!(document.units[1].kind, VhdlUnitKind::PackageBody);
        assert!(document.units.iter().all(VhdlDesignUnit::is_closed));
    }

    #[test]
    fn ignores_keywords_in_comments_strings_and_character_literals() {
        let source = r#"
            -- entity false_comment is end entity;
            entity real_unit is
                constant message : string := "end entity fake_unit;";
                constant marker : character := 'x';
            end entity real_unit;
        "#;
        let document = scan(source);

        assert_eq!(document.units.len(), 1);
        assert_eq!(document.units[0].name, "real_unit");
        assert!(document.units[0].is_closed());
    }

    #[test]
    fn accepts_case_insensitive_basic_unit_names() {
        let document = scan(
            "ENTITY Counter IS END ENTITY counter;",
        );

        assert_eq!(document.units.len(), 1);
        assert!(document.units[0].is_closed());
    }

    #[test]
    fn reports_an_unclosed_unit() {
        let document = scan("entity incomplete is");

        assert_eq!(document.units.len(), 1);
        assert!(!document.units[0].is_closed());
        assert!(document.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == VhdlDiagnosticCode::UnclosedDesignUnit
        }));
    }

    #[test]
    fn does_not_treat_end_if_as_a_design_unit_end() {
        let source = r#"
            architecture rtl of demo is
            begin
                process
                begin
                    if true then
                        null;
                    end if;
                end process;
            end architecture rtl;
        "#;
        let document = scan(source);

        assert_eq!(document.units.len(), 1);
        assert!(document.units[0].is_closed());
    }

    #[test]
    fn rejects_unterminated_strings() {
        let error = scan_vhdl(
            r#"entity demo is constant x : string := "unterminated;"#,
            &VhdlScanConfig::default(),
        )
        .expect_err("unterminated string must fail");

        assert!(matches!(error, VhdlError::UnterminatedString { .. }));
    }

    #[test]
    fn rejects_unterminated_extended_identifiers() {
        let error = scan_vhdl(
            r"entity \extended_name is",
            &VhdlScanConfig::default(),
        )
        .expect_err("unterminated extended identifier must fail");

        assert!(matches!(
            error,
            VhdlError::UnterminatedExtendedIdentifier { .. }
        ));
    }

    #[test]
    fn enforces_an_explicit_operational_source_budget() {
        let config = VhdlScanConfig {
            max_source_bytes: Some(4),
            ..VhdlScanConfig::default()
        };

        let error = scan_vhdl("entity", &config)
            .expect_err("source budget must be enforced");

        assert!(matches!(
            error,
            VhdlError::SourceBudgetExceeded {
                actual: 6,
                budget: 4
            }
        ));
    }

    #[test]
    fn source_ranges_are_checked_and_half_open() {
        let range = VhdlSourceRange::new(1, 4).expect("valid range");

        assert_eq!(range.len(), 3);
        assert_eq!(range.slice("abcdef"), Some("bcd"));
        assert!(VhdlSourceRange::new(4, 1).is_err());
    }

    #[test]
    fn retains_multiple_units_in_source_order() {
        let source = "entity first is end entity first; entity second is end entity second;";
        let document = scan(source);

        assert_eq!(
            document
                .units
                .iter()
                .map(|unit| unit.name.as_str())
                .collect::<Vec<_>>(),
            vec!["first", "second"]
        );
    }
}
