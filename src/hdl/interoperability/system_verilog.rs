
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! SystemVerilog interoperability boundary for Zamani HDL.
//!
//! # Authority
//!
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/hdl.g4` and its delegate grammars
//! - `src/hdl/interoperability/verilog.rs`
//! - `src/frontend/parser/domains/hdl.rs`
//! - `src/frontend/semantic/domains/hdl.rs`
//!
//! # Purpose
//!
//! Provide loss-preserving, bounded-by-policy structural discovery for
//! SystemVerilog source. The scanner recognizes common design-unit boundaries
//! without rewriting source, expanding macros, or pretending to be a full
//! IEEE 1800 parser.
//!
//! # Ownership
//!
//! This file owns SystemVerilog-specific structural discovery, source ranges,
//! scan configuration, and scanner diagnostics.
//!
//! It does not own the canonical lexer, AST, semantic model, preprocessing,
//! elaboration, simulation, synthesis, target realization, or any IR.
//!
//! All HDLIR implementation remains exclusively under `src/hdl/hdlir/` and
//! is outside this module's scope.
//!
//! # Integration
//!
//! Call `scan_system_verilog` to discover source boundaries. Feed the original
//! source to the grammar-driven parser for complete syntax parsing, then map
//! the resulting constructs through the existing AST and semantic adapters.
//! The returned ranges are UTF-8 byte ranges, not replacements for the
//! canonical frontend `Span`.
//!
//! # Scalability
//!
//! No language-level ceiling is imposed on source length or design-unit count.
//! The optional source-byte budget is an operational safeguard. Collection
//! sizes grow dynamically, arithmetic on source offsets is checked, and an
//! incomplete structural scan is never represented as full language
//! validation.
//!
//! # Important limitation
//!
//! This scanner is not a preprocessor, parser, elaborator, or conformance
//! checker. SystemVerilog preprocessing and grammar validation must be
//! performed by their owning components before relying on conditional or
//! context-dependent structure.

use std::error::Error;
use std::fmt;
use std::ops::Range;

/// Operational configuration for a SystemVerilog structural scan.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SystemVerilogScanConfig {
    /// Optional maximum source size, in UTF-8 bytes, for this invocation.
    ///
    /// `None` means no caller-configured source-size budget. This is not a
    /// limit on the legal size of a SystemVerilog design.
    pub max_source_bytes: Option<usize>,

    /// Report unmatched closing design-unit keywords.
    pub report_unmatched_end_keywords: bool,
}

impl SystemVerilogScanConfig {
    /// Create a configuration with no source-size budget and structural
    /// diagnostics enabled.
    #[must_use]
    pub const fn unbounded() -> Self {
        Self {
            max_source_bytes: None,
            report_unmatched_end_keywords: true,
        }
    }
}

/// A SystemVerilog construct category recognized by structural discovery.
///
/// This is deliberately open to future extension through new variants.
/// These categories do not represent a parsed AST or semantic type.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub enum SystemVerilogUnitKind {
    /// `module ... endmodule`
    Module,
    /// `macromodule ... endmodule`
    MacroModule,
    /// `interface ... endinterface`
    Interface,
    /// `program ... endprogram`
    Program,
    /// `package ... endpackage`
    Package,
    /// `checker ... endchecker`
    Checker,
    /// `primitive ... endprimitive`
    Primitive,
    /// `class ... endclass`
    Class,
}

impl SystemVerilogUnitKind {
    fn opening_keyword(self) -> &'static str {
        match self {
            Self::Module => "module",
            Self::MacroModule => "macromodule",
            Self::Interface => "interface",
            Self::Program => "program",
            Self::Package => "package",
            Self::Checker => "checker",
            Self::Primitive => "primitive",
            Self::Class => "class",
        }
    }

    fn closing_keyword(self) -> &'static str {
        match self {
            Self::Module | Self::MacroModule => "endmodule",
            Self::Interface => "endinterface",
            Self::Program => "endprogram",
            Self::Package => "endpackage",
            Self::Checker => "endchecker",
            Self::Primitive => "endprimitive",
            Self::Class => "endclass",
        }
    }

    fn from_opening_keyword(keyword: &str) -> Option<Self> {
        match keyword {
            "module" => Some(Self::Module),
            "macromodule" => Some(Self::MacroModule),
            "interface" => Some(Self::Interface),
            "program" => Some(Self::Program),
            "package" => Some(Self::Package),
            "checker" => Some(Self::Checker),
            "primitive" => Some(Self::Primitive),
            "class" => Some(Self::Class),
            _ => None,
        }
    }

    fn from_closing_keyword(keyword: &str) -> Option<Self> {
        match keyword {
            "endmodule" => Some(Self::Module),
            "endinterface" => Some(Self::Interface),
            "endprogram" => Some(Self::Program),
            "endpackage" => Some(Self::Package),
            "endchecker" => Some(Self::Checker),
            "endprimitive" => Some(Self::Primitive),
            "endclass" => Some(Self::Class),
            _ => None,
        }
    }
}

/// A validated half-open UTF-8 byte range into the original source.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct SystemVerilogSourceRange {
    /// Inclusive starting byte offset.
    pub start: usize,
    /// Exclusive ending byte offset.
    pub end: usize,
}

impl SystemVerilogSourceRange {
    /// Construct a range, rejecting reversed endpoints.
    pub fn new(start: usize, end: usize) -> Result<Self, SystemVerilogError> {
        if end < start {
            return Err(SystemVerilogError::InvalidRange { start, end });
        }
        Ok(Self { start, end })
    }

    /// Convert to Rust's half-open range type.
    #[must_use]
    pub fn as_range(&self) -> Range<usize> {
        self.start..self.end
    }

    /// Return the byte length.
    #[must_use]
    pub fn len(&self) -> usize {
        self.end - self.start
    }

    /// Return whether this range is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }

    /// Return the referenced text if the range is valid for this source.
    #[must_use]
    pub fn slice<'a>(&self, source: &'a str) -> Option<&'a str> {
        source.get(self.as_range())
    }
}

/// A discovered design-unit boundary.
///
/// `name` preserves the exact source spelling of the identifier, including
/// the leading backslash of an escaped identifier where applicable.
///
/// This metadata does not prove that the unit is grammatically valid.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SystemVerilogUnitBoundary {
    /// Structural unit category.
    pub kind: SystemVerilogUnitKind,
    /// Declared identifier spelling, when one could be discovered.
    ///
    /// Some constructs, including anonymous or malformed forms, may not have
    /// a discoverable name at this scanning layer.
    pub name: Option<String>,
    /// Opening keyword range.
    pub opening_range: SystemVerilogSourceRange,
    /// Identifier range, if a name was discovered.
    pub name_range: Option<SystemVerilogSourceRange>,
    /// Matching closing keyword range, if found.
    pub closing_range: Option<SystemVerilogSourceRange>,
    /// Full range from opening keyword through closing keyword, if closed.
    pub full_range: Option<SystemVerilogSourceRange>,
}

impl SystemVerilogUnitBoundary {
    /// Whether a matching closing keyword was found.
    #[must_use]
    pub const fn is_closed(&self) -> bool {
        self.closing_range.is_some()
    }

    /// Return the original source for this unit when its range is complete.
    #[must_use]
    pub fn source<'a>(&self, source: &'a str) -> Option<&'a str> {
        self.full_range.as_ref()?.slice(source)
    }
}

/// Stable category for structural scanner diagnostics.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub enum SystemVerilogDiagnosticCode {
    /// An opening keyword was not followed by a discoverable identifier.
    MissingUnitName,
    /// A closing keyword had no matching open unit.
    UnmatchedEndKeyword,
    /// An opening keyword had no matching close at end of input.
    UnclosedUnit,
    /// An opening and closing unit category did not match nesting order.
    MismatchedEndKeyword,
    /// A preprocessor directive may change the actual source structure.
    PreprocessorDirectivePresent,
    /// The scanner did not perform complete grammar or semantic validation.
    StructuralValidationIncomplete,
}

/// Diagnostic severity.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum SystemVerilogSeverity {
    /// Informational limitation.
    Info,
    /// The caller should inspect the condition.
    Warning,
    /// A structural inconsistency was detected.
    Error,
}

/// A deterministic structural diagnostic.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SystemVerilogDiagnostic {
    /// Stable machine-readable category.
    pub code: SystemVerilogDiagnosticCode,
    /// Severity.
    pub severity: SystemVerilogSeverity,
    /// Human-readable explanation.
    pub message: String,
    /// Related source range, when known.
    pub range: Option<SystemVerilogSourceRange>,
}

/// Result of structural discovery over the original source.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SystemVerilogDocument<'source> {
    /// Original input, borrowed without rewriting.
    pub source: &'source str,
    /// Discovered units in source order.
    pub units: Vec<SystemVerilogUnitBoundary>,
    /// Deterministically ordered diagnostics.
    pub diagnostics: Vec<SystemVerilogDiagnostic>,
    /// Whether a backtick directive was encountered.
    ///
    /// Directives are not expanded. Unit boundaries can therefore depend on
    /// the caller's preprocessor configuration.
    pub has_preprocessor_directives: bool,
    /// Whether lexical scanning reached end-of-input.
    ///
    /// This does not imply that parsing or semantic validation succeeded.
    pub scan_complete: bool,
}

impl<'source> SystemVerilogDocument<'source> {
    /// Return the original source for a complete unit.
    #[must_use]
    pub fn unit_source(
        &self,
        unit: &SystemVerilogUnitBoundary,
    ) -> Option<&'source str> {
        unit.source(self.source)
    }

    /// Whether any structural error was reported.
    #[must_use]
    pub fn has_structural_errors(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|d| d.severity == SystemVerilogSeverity::Error)
    }

    /// Whether preprocessing is needed to establish definitive structure.
    #[must_use]
    pub const fn requires_preprocessing(&self) -> bool {
        self.has_preprocessor_directives
    }
}

/// Errors that prevent a structural scan from completing.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum SystemVerilogError {
    /// The caller's optional operational source-size budget was exceeded.
    SourceBudgetExceeded {
        /// Actual input size in bytes.
        actual: usize,
        /// Configured budget in bytes.
        budget: usize,
    },
    /// A requested range has a reversed endpoint.
    InvalidRange {
        /// Start offset.
        start: usize,
        /// End offset.
        end: usize,
    },
    /// A block comment did not terminate.
    UnterminatedBlockComment {
        /// Opening delimiter byte offset.
        offset: usize,
    },
    /// A string literal did not terminate.
    UnterminatedString {
        /// Opening quote byte offset.
        offset: usize,
    },
    /// A source boundary could not be represented safely.
    InvalidSourceBoundary {
        /// Byte offset.
        offset: usize,
    },
}

impl fmt::Display for SystemVerilogError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SourceBudgetExceeded { actual, budget } => write!(
                f,
                "SystemVerilog source size {actual} bytes exceeds the \
                 configured operational budget of {budget} bytes"
            ),
            Self::InvalidRange { start, end } => {
                write!(f, "invalid SystemVerilog source range {start}..{end}")
            }
            Self::UnterminatedBlockComment { offset } => {
                write!(f, "unterminated SystemVerilog block comment at byte {offset}")
            }
            Self::UnterminatedString { offset } => {
                write!(f, "unterminated SystemVerilog string at byte {offset}")
            }
            Self::InvalidSourceBoundary { offset } => {
                write!(f, "invalid SystemVerilog source boundary at byte {offset}")
            }
        }
    }
}

impl Error for SystemVerilogError {}

#[derive(Clone, Debug)]
struct Token {
    spelling: String,
    range: SystemVerilogSourceRange,
    kind: TokenKind,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum TokenKind {
    Identifier,
    Directive,
    Other,
}

/// Scan SystemVerilog source for common design-unit boundaries.
///
/// This function:
///
/// - preserves the original input and all returned source offsets;
/// - ignores boundary-looking words inside comments and strings;
/// - treats escaped identifiers as opaque identifiers;
/// - records, but does not expand, preprocessor directives;
/// - recognizes modules, macromodules, interfaces, programs, packages,
///   checkers, primitives, and classes;
/// - reports unmatched, unclosed, and mismatched unit endings.
///
/// It does not parse module headers, validate declarations, expand macros,
/// resolve imports, or elaborate parameterized designs.
///
/// # Errors
///
/// Returns an error if the configured source budget is exceeded or if a block
/// comment or string is unterminated.
pub fn scan_system_verilog<'source>(
    source: &'source str,
    config: &SystemVerilogScanConfig,
) -> Result<SystemVerilogDocument<'source>, SystemVerilogError> {
    if let Some(budget) = config.max_source_bytes {
        if source.len() > budget {
            return Err(SystemVerilogError::SourceBudgetExceeded {
                actual: source.len(),
                budget,
            });
        }
    }

    let (tokens, has_preprocessor_directives) = lex(source)?;
    let mut units: Vec<SystemVerilogUnitBoundary> = Vec::new();
    let mut stack: Vec<usize> = Vec::new();
    let mut diagnostics = Vec::new();

    for (index, token) in tokens.iter().enumerate() {
        match token.kind {
            TokenKind::Identifier => {
                if let Some(kind) =
                    SystemVerilogUnitKind::from_opening_keyword(&token.spelling)
                {
                    // `extern module ...;` and `extern interface ...;` are
                    // declarations rather than complete unit bodies.
                    // They must not be pushed onto the open-unit stack.
                    if is_extern_declaration(&tokens, index) {
                        continue;
                    }

                    let name_token = next_unit_name(&tokens, index + 1, kind);
                    let (name, name_range) = match name_token {
                        Some(name_index) => {
                            let name_token = &tokens[name_index];
                            (
                                Some(name_token.spelling.clone()),
                                Some(name_token.range.clone()),
                            )
                        }
                        None => {
                            diagnostics.push(SystemVerilogDiagnostic {
                                code: SystemVerilogDiagnosticCode::MissingUnitName,
                                severity: SystemVerilogSeverity::Error,
                                message: format!(
                                    "{} declaration has no discoverable identifier",
                                    kind.opening_keyword()
                                ),
                                range: Some(token.range.clone()),
                            });
                            (None, None)
                        }
                    };

                    let unit_index = units.len();
                    units.push(SystemVerilogUnitBoundary {
                        kind,
                        name,
                        opening_range: token.range.clone(),
                        name_range,
                        closing_range: None,
                        full_range: None,
                    });
                    stack.push(unit_index);
                } else if let Some(closing_kind) =
                    SystemVerilogUnitKind::from_closing_keyword(&token.spelling)
                {
                    match stack.last().copied() {
                        Some(open_index) if units[open_index].kind == closing_kind => {
                            stack.pop();
                            let unit = &mut units[open_index];
                            unit.closing_range = Some(token.range.clone());
                            unit.full_range = Some(SystemVerilogSourceRange {
                                start: unit.opening_range.start,
                                end: token.range.end,
                            });
                        }
                        Some(open_index) => {
                            diagnostics.push(SystemVerilogDiagnostic {
                                code: SystemVerilogDiagnosticCode::MismatchedEndKeyword,
                                severity: SystemVerilogSeverity::Error,
                                message: format!(
                                    "`{}` cannot close `{}`; expected `{}`",
                                    token.spelling,
                                    units[open_index].kind.opening_keyword(),
                                    units[open_index].kind.closing_keyword()
                                ),
                                range: Some(token.range.clone()),
                            });
                        }
                        None if config.report_unmatched_end_keywords => {
                            diagnostics.push(SystemVerilogDiagnostic {
                                code: SystemVerilogDiagnosticCode::UnmatchedEndKeyword,
                                severity: SystemVerilogSeverity::Error,
                                message: format!(
                                    "`{}` has no matching open design unit",
                                    token.spelling
                                ),
                                range: Some(token.range.clone()),
                            });
                        }
                        None => {}
                    }
                }
            }
            TokenKind::Directive | TokenKind::Other => {}
        }
    }

    for unit_index in stack {
        let unit = &units[unit_index];
        diagnostics.push(SystemVerilogDiagnostic {
            code: SystemVerilogDiagnosticCode::UnclosedUnit,
            severity: SystemVerilogSeverity::Error,
            message: format!(
                "{} `{}` has no matching `{}`",
                unit.kind.opening_keyword(),
                unit.name.as_deref().unwrap_or("<unknown>"),
                unit.kind.closing_keyword()
            ),
            range: Some(unit.opening_range.clone()),
        });
    }

    if has_preprocessor_directives {
        diagnostics.push(SystemVerilogDiagnostic {
            code: SystemVerilogDiagnosticCode::PreprocessorDirectivePresent,
            severity: SystemVerilogSeverity::Warning,
            message: "SystemVerilog preprocessor directives were found; \
                      definitive design-unit boundaries may depend on macro \
                      definitions and conditional compilation"
                .to_owned(),
            range: None,
        });
    }

    diagnostics.push(SystemVerilogDiagnostic {
        code: SystemVerilogDiagnosticCode::StructuralValidationIncomplete,
        severity: SystemVerilogSeverity::Info,
        message: "structural scanning is not full SystemVerilog parsing, \
                  preprocessing, elaboration, or semantic validation"
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

    Ok(SystemVerilogDocument {
        source,
        units,
        diagnostics,
        has_preprocessor_directives,
        scan_complete: true,
    })
}

fn is_extern_declaration(tokens: &[Token], index: usize) -> bool {
    index
        .checked_sub(1)
        .and_then(|previous| tokens.get(previous))
        .is_some_and(|token| token.kind == TokenKind::Identifier && token.spelling == "extern")
}

fn next_unit_name(
    tokens: &[Token],
    start: usize,
    kind: SystemVerilogUnitKind,
) -> Option<usize> {
    let mut index = start;

    // Modules, interfaces and programs may have an optional lifetime.
    if matches!(
        kind,
        SystemVerilogUnitKind::Module
            | SystemVerilogUnitKind::MacroModule
            | SystemVerilogUnitKind::Interface
            | SystemVerilogUnitKind::Program
    ) && tokens.get(index).is_some_and(|token| {
        token.kind == TokenKind::Identifier
            && matches!(token.spelling.as_str(), "automatic" | "static")
    }) {
        index += 1;
    }

    tokens.get(index).and_then(|token| {
        (token.kind == TokenKind::Identifier).then_some(index)
    })
}

fn lex(
    source: &str,
) -> Result<(Vec<Token>, bool), SystemVerilogError> {
    let bytes = source.as_bytes();
    let mut tokens = Vec::new();
    let mut index = 0usize;
    let mut has_directives = false;

    while index < bytes.len() {
        let byte = bytes[index];

        if byte.is_ascii_whitespace() {
            index += 1;
            continue;
        }

        // Line comments.
        if byte == b'/' && bytes.get(index + 1) == Some(&b'/') {
            index += 2;
            while index < bytes.len()
                && bytes[index] != b'\n'
                && bytes[index] != b'\r'
            {
                index += 1;
            }
            continue;
        }

        // Block comments.
        if byte == b'/' && bytes.get(index + 1) == Some(&b'*') {
            let start = index;
            index += 2;
            let mut closed = false;

            while index + 1 < bytes.len() {
                if bytes[index] == b'*' && bytes[index + 1] == b'/' {
                    index += 2;
                    closed = true;
                    break;
                }
                index += 1;
            }

            if !closed {
                return Err(SystemVerilogError::UnterminatedBlockComment {
                    offset: start,
                });
            }
            continue;
        }

        // String literals, including escaped quote and backslash characters.
        if byte == b'"' {
            let start = index;
            index += 1;
            let mut closed = false;

            while index < bytes.len() {
                match bytes[index] {
                    b'\\' => {
                        index = index.saturating_add(2).min(bytes.len());
                    }
                    b'"' => {
                        index += 1;
                        closed = true;
                        break;
                    }
                    _ => index += 1,
                }
            }

            if !closed {
                return Err(SystemVerilogError::UnterminatedString {
                    offset: start,
                });
            }
            continue;
        }

        // Preprocessor directives are recorded, never expanded here.
        if byte == b'`' {
            let start = index;
            index += 1;
            while index < bytes.len() && is_identifier_continue(bytes[index]) {
                index += 1;
            }

            has_directives = true;
            tokens.push(Token {
                spelling: source[start..index].to_owned(),
                range: SystemVerilogSourceRange { start, end: index },
                kind: TokenKind::Directive,
            });
            continue;
        }

        // Escaped identifiers continue through the next whitespace.
        if byte == b'\\' {
            let start = index;
            index += 1;
            while index < bytes.len() && !bytes[index].is_ascii_whitespace() {
                index += 1;
            }

            tokens.push(Token {
                spelling: source[start..index].to_owned(),
                range: SystemVerilogSourceRange { start, end: index },
                kind: TokenKind::Identifier,
            });
            continue;
        }

        if is_identifier_start(byte) {
            let start = index;
            index += 1;
            while index < bytes.len() && is_identifier_continue(bytes[index]) {
                index += 1;
            }

            tokens.push(Token {
                spelling: source[start..index].to_owned(),
                range: SystemVerilogSourceRange { start, end: index },
                kind: TokenKind::Identifier,
            });
            continue;
        }

        // Keep non-ASCII source ranges on UTF-8 scalar boundaries.
        let start = index;
        if byte.is_ascii() {
            index += 1;
        } else {
            let tail = source
                .get(index..)
                .ok_or(SystemVerilogError::InvalidSourceBoundary { offset: index })?;
            let character = tail
                .chars()
                .next()
                .ok_or(SystemVerilogError::InvalidSourceBoundary { offset: index })?;
            index = index
                .checked_add(character.len_utf8())
                .ok_or(SystemVerilogError::InvalidSourceBoundary { offset: index })?;
        }

        tokens.push(Token {
            spelling: source[start..index].to_owned(),
            range: SystemVerilogSourceRange { start, end: index },
            kind: TokenKind::Other,
        });
    }

    Ok((tokens, has_directives))
}

fn is_identifier_start(byte: u8) -> bool {
    byte.is_ascii_alphabetic() || byte == b'_'
}

fn is_identifier_continue(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'$')
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scan(source: &str) -> SystemVerilogDocument<'_> {
        scan_system_verilog(source, &SystemVerilogScanConfig::unbounded())
            .expect("fixture should be lexically scannable")
    }

    #[test]
    fn discovers_module_interface_package_and_class() {
        let source = r#"
            package shared_pkg;
                class packet;
                endclass : packet
            endpackage : shared_pkg

            interface bus_if;
            endinterface : bus_if

            module top;
            endmodule : top
        "#;

        let document = scan(source);
        assert_eq!(document.units.len(), 4);
        assert!(document.units.iter().all(SystemVerilogUnitBoundary::is_closed));
        assert!(!document.has_structural_errors());
    }

    #[test]
    fn ignores_boundary_keywords_inside_comments_and_strings() {
        let source = r#"
            // module fake; endmodule
            /* interface fake_if; endinterface */
            module actual;
                string message = "endmodule endinterface";
            endmodule
        "#;

        let document = scan(source);
        assert_eq!(document.units.len(), 1);
        assert_eq!(document.units[0].name.as_deref(), Some("actual"));
        assert!(!document.has_structural_errors());
    }

    #[test]
    fn preserves_escaped_identifiers_and_original_source() {
        let source = r"module \odd.name ; endmodule";
        let document = scan(source);

        assert_eq!(document.units.len(), 1);
        assert_eq!(document.units[0].name.as_deref(), Some(r"\odd.name"));
        assert_eq!(document.unit_source(&document.units[0]), Some(source));
    }

    #[test]
    fn recognizes_optional_module_lifetime() {
        let document = scan("module automatic counter; endmodule");
        assert_eq!(document.units[0].name.as_deref(), Some("counter"));
        assert!(document.units[0].is_closed());
    }

    #[test]
    fn reports_unclosed_units() {
        let document = scan("package unfinished;");
        assert!(document.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == SystemVerilogDiagnosticCode::UnclosedUnit
        }));
    }

    #[test]
    fn reports_unmatched_end_keywords() {
        let document = scan("endmodule");
        assert!(document.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == SystemVerilogDiagnosticCode::UnmatchedEndKeyword
        }));
    }

    #[test]
    fn reports_mismatched_nested_unit_closures() {
        let document = scan("package p; endmodule endpackage");
        assert!(document.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == SystemVerilogDiagnosticCode::MismatchedEndKeyword
        }));
    }

    #[test]
    fn records_but_does_not_expand_preprocessor_directives() {
        let source = "`ifdef FEATURE\nmodule optional_unit; endmodule\n`endif";
        let document = scan(source);

        assert!(document.requires_preprocessing());
        assert!(document.diagnostics.iter().any(|diagnostic| {
            diagnostic.code
                == SystemVerilogDiagnosticCode::PreprocessorDirectivePresent
        }));
    }

    #[test]
    fn rejects_unterminated_block_comments() {
        let error = scan_system_verilog(
            "module m; /* not closed",
            &SystemVerilogScanConfig::unbounded(),
        )
        .expect_err("unterminated block comment must fail");

        assert!(matches!(
            error,
            SystemVerilogError::UnterminatedBlockComment { .. }
        ));
    }

    #[test]
    fn rejects_unterminated_strings() {
        let error = scan_system_verilog(
            r#"module m; string s = "not closed; endmodule"#,
            &SystemVerilogScanConfig::unbounded(),
        )
        .expect_err("unterminated string must fail");

        assert!(matches!(
            error,
            SystemVerilogError::UnterminatedString { .. }
        ));
    }

    #[test]
    fn enforces_only_an_explicit_operational_source_budget() {
        let config = SystemVerilogScanConfig {
            max_source_bytes: Some(3),
            report_unmatched_end_keywords: true,
        };

        let error = scan_system_verilog("module", &config)
            .expect_err("configured budget must be enforced");

        assert!(matches!(
            error,
            SystemVerilogError::SourceBudgetExceeded {
                actual: 6,
                budget: 3
            }
        ));
    }

    #[test]
    fn source_ranges_remain_valid_for_non_ascii_text() {
        let source = "module top; string s = \"λ\"; endmodule";
        let document = scan(source);

        for unit in &document.units {
            let text = document
                .unit_source(unit)
                .expect("discovered source range must be valid UTF-8");
            assert!(text.starts_with("module"));
            assert!(text.ends_with("endmodule"));
        }
    }

    #[test]
    fn reports_that_full_validation_is_not_performed() {
        let document = scan("module top; endmodule");
        assert!(document.diagnostics.iter().any(|diagnostic| {
            diagnostic.code
                == SystemVerilogDiagnosticCode::StructuralValidationIncomplete
        }));
    }
}
