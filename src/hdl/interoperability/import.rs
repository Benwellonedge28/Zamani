
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! HDL source-import boundary for Zamani.
//!
//! Path: `src/hdl/interoperability/import.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: safe Rust only.
//!
//! # Authority
//!
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/hdl.g4` and its delegate grammars
//! - `src/frontend/parser/domains/hdl.rs`
//! - `src/frontend/semantic/domains/hdl.rs`
//! - `super::verilog`
//! - `super::system_verilog`
//!
//! # Purpose
//!
//! Provide one explicit API for accepting externally supplied Verilog or
//! SystemVerilog source and producing normalized structural import metadata.
//! Preserve the original source and source offsets. Report structural
//! diagnostics without claiming that the source has been fully parsed,
//! elaborated, or semantically validated.
//!
//! # Ownership
//!
//! This module owns:
//! - the common import request and result contract;
//! - explicit dialect selection;
//! - normalization of the existing scanner outputs;
//! - import-level diagnostics and errors;
//! - source identity supplied by the caller.
//!
//! It does not own:
//! - filesystem or network access, include resolution, or source discovery;
//! - preprocessor execution or macro expansion;
//! - a Verilog/SystemVerilog parser or AST;
//! - Zamani AST construction or semantic validation;
//! - source-map identity allocation;
//! - synthesis, simulation, target realization, or HDLIR.
//!
//! # Integration
//!
//! 1. The caller obtains the source using its own trusted source provider.
//! 2. The caller selects `ImportDialect` explicitly and invokes
//!    `scan_imported_source`.
//! 3. This module delegates structural scanning to the existing Verilog or
//!    SystemVerilog scanner and normalizes the resulting units and diagnostics.
//! 4. The caller checks the report and sends the original source to the
//!    appropriate grammar-driven parser.
//! 5. The canonical AST adapter and semantic pipeline remain authoritative.
//!
//! `source_name` is descriptive provenance supplied by the caller. It is not
//! opened, interpreted as a path, or treated as a trusted filesystem identity.
//!
//! # Scalability and budgets
//!
//! No language-level limit is imposed on design-unit count or legal hardware
//! capacity. Collections grow dynamically. An optional source-byte budget is
//! an operational safeguard, not a language restriction. The scanners may
//! allocate memory proportional to source tokens, units, and diagnostics.
//! Resource exhaustion by the host process cannot be ruled out by this API.
//!
//! # Preprocessing
//!
//! The scanners do not expand macros or evaluate conditional-compilation
//! directives. If directives are present, the report marks structural
//! discovery as preprocessing-dependent. Callers must not treat the discovered
//! units as definitive for a particular macro environment until preprocessing
//! has been performed by an appropriate component.
//!
//! # HDLIR boundary
//!
//! This file contains no intermediate-representation types or transformations.
//! All HDLIR implementation remains exclusively under `src/hdl/hdlir/`.
//!
//! # Freeze criteria
//!
//! - Dialect selection is explicit.
//! - Existing scanners remain the sole structural-scanning owners.
//! - Every returned range refers to the original UTF-8 source.
//! - Scanner errors are preserved as typed import errors.
//! - Preprocessor-dependent structure and incomplete validation are explicit.
//! - No AST, semantic model, or IR is duplicated here.
//! - Unit tests and repository-wide checks pass.

use std::error::Error;
use std::fmt;

use super::system_verilog::{
    scan_system_verilog, SystemVerilogDiagnostic, SystemVerilogScanConfig,
    SystemVerilogSeverity, SystemVerilogSourceRange, SystemVerilogUnitBoundary,
    SystemVerilogUnitKind,
};
use super::verilog::{
    scan_verilog, VerilogDiagnostic, VerilogDialect, VerilogScanConfig,
    VerilogSeverity, VerilogSourceRange,
};

/// Source dialect selected by the caller.
///
/// Dialect detection is intentionally not guessed from filenames or source
/// contents. A source provider or caller must make the selection explicitly.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ImportDialect {
    /// Verilog-family source scanned using the existing Verilog scanner.
    Verilog,

    /// SystemVerilog source scanned using the existing SystemVerilog scanner.
    SystemVerilog,
}

/// Configuration for one source-import scan.
///
/// The source-size budget controls the work permitted for this invocation.
/// It does not limit the legal size of a hardware design.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImportConfig {
    /// Explicitly selected source dialect.
    pub dialect: ImportDialect,

    /// Optional maximum source length in UTF-8 bytes.
    ///
    /// `None` means no caller-configured source-size budget.
    pub max_source_bytes: Option<usize>,

    /// Report unmatched closing design-unit keywords as structural errors
    /// where the underlying scanner supports that policy.
    pub report_unmatched_end_keywords: bool,
}

impl ImportConfig {
    /// Create an import configuration without a source-size budget.
    #[must_use]
    pub const fn unbounded(dialect: ImportDialect) -> Self {
        Self {
            dialect,
            max_source_bytes: None,
            report_unmatched_end_keywords: true,
        }
    }

    /// Create an import configuration with an explicit byte budget.
    #[must_use]
    pub const fn with_source_budget(
        dialect: ImportDialect,
        max_source_bytes: usize,
    ) -> Self {
        Self {
            dialect,
            max_source_bytes: Some(max_source_bytes),
            report_unmatched_end_keywords: true,
        }
    }
}

impl Default for ImportConfig {
    /// Default to Verilog with no source-size budget.
    ///
    /// This default does not infer that arbitrary source is Verilog. Callers
    /// importing SystemVerilog must select that dialect explicitly.
    fn default() -> Self {
        Self::unbounded(ImportDialect::Verilog)
    }
}

/// Normalized design-unit category returned by the import boundary.
///
/// These categories are structural metadata, not AST node kinds. The
/// `Other` variant allows callers to handle future scanner categories without
/// assuming this enum exhaustively describes every HDL construct.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub enum ImportedUnitKind {
    /// A Verilog `module`.
    Module,

    /// A Verilog `macromodule`.
    MacroModule,

    /// A SystemVerilog `interface`.
    Interface,

    /// A SystemVerilog `program`.
    Program,

    /// A SystemVerilog `package`.
    Package,

    /// A SystemVerilog `checker`.
    Checker,

    /// A Verilog or SystemVerilog `primitive`.
    Primitive,

    /// A SystemVerilog `class`.
    Class,

    /// A future or scanner-specific unit not represented above.
    Other,
}

/// A validated half-open UTF-8 byte range into the original source.
///
/// The range uses byte offsets, not Unicode scalar-value indices or character
/// columns. It is not a replacement for Zamani's canonical frontend `Span`.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ImportSourceRange {
    /// Inclusive starting byte offset.
    pub start: usize,

    /// Exclusive ending byte offset.
    pub end: usize,
}

impl ImportSourceRange {
    /// Construct a range, rejecting reversed endpoints.
    pub fn new(start: usize, end: usize) -> Result<Self, ImportError> {
        if end < start {
            return Err(ImportError::InvalidRange { start, end });
        }

        Ok(Self { start, end })
    }

    /// Return this range as a standard Rust half-open range.
    #[must_use]
    pub fn as_range(&self) -> std::ops::Range<usize> {
        self.start..self.end
    }

    /// Return the byte length of this range.
    #[must_use]
    pub fn len(&self) -> usize {
        self.end - self.start
    }

    /// Return whether the range is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }

    /// Return the source slice when the range is valid for `source`.
    ///
    /// This checks bounds and UTF-8 boundaries. A range must not be reused
    /// against unrelated source text merely because the offsets fit.
    #[must_use]
    pub fn slice<'source>(&self, source: &'source str) -> Option<&'source str> {
        source.get(self.as_range())
    }
}

impl From<&VerilogSourceRange> for ImportSourceRange {
    fn from(range: &VerilogSourceRange) -> Self {
        Self {
            start: range.start,
            end: range.end,
        }
    }
}

impl From<&SystemVerilogSourceRange> for ImportSourceRange {
    fn from(range: &SystemVerilogSourceRange) -> Self {
        Self {
            start: range.start,
            end: range.end,
        }
    }
}

/// Structural metadata for one discovered imported design unit.
///
/// The exact original spelling of the unit name is retained where the
/// underlying scanner discovered it. This type does not claim the name or
/// unit body is grammatically or semantically valid.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImportedDesignUnit {
    /// Normalized structural category.
    pub kind: ImportedUnitKind,

    /// Name exactly as recognized by the underlying scanner, if available.
    pub name: Option<String>,

    /// Range covering the opening keyword.
    pub opening_range: ImportSourceRange,

    /// Range covering the identifier, when available.
    pub name_range: Option<ImportSourceRange>,

    /// Range covering the matching closing keyword, when available.
    pub closing_range: Option<ImportSourceRange>,

    /// Range covering the complete unit, when a closing boundary was found.
    pub full_range: Option<ImportSourceRange>,
}

impl ImportedDesignUnit {
    /// Return whether the scanner found a closing boundary for this unit.
    #[must_use]
    pub const fn is_closed(&self) -> bool {
        self.closing_range.is_some()
    }

    /// Return the original source for this complete unit.
    #[must_use]
    pub fn source<'source>(&self, source: &'source str) -> Option<&'source str> {
        self.full_range.as_ref()?.slice(source)
    }
}

/// Stable import-level diagnostic category.
///
/// `StructuralScannerDiagnostic` means that the underlying scanner reported
/// a dialect-specific condition. The original message is preserved; the
/// import layer does not promise that scanner-specific code names remain
/// stable across scanner revisions.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub enum ImportDiagnosticCode {
    /// The underlying scanner found a structural issue or limitation.
    StructuralScannerDiagnostic,

    /// Preprocessor directives make structure conditional.
    PreprocessingRequired,

    /// Structural scanning is not full language validation.
    ValidationIncomplete,
}

/// Severity of an import diagnostic.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ImportSeverity {
    /// Informational limitation or status.
    Info,

    /// The caller should inspect this condition.
    Warning,

    /// A structural inconsistency was detected.
    Error,
}

/// An import diagnostic normalized from an underlying scanner.
///
/// `range` is relative to the original source supplied to
/// `scan_imported_source`. Diagnostics without a known source location have
/// `None` for this field.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImportDiagnostic {
    /// Stable import-level category.
    pub code: ImportDiagnosticCode,

    /// Severity.
    pub severity: ImportSeverity,

    /// Human-readable explanation.
    pub message: String,

    /// Related range in the original source, if known.
    pub range: Option<ImportSourceRange>,
}

impl From<&VerilogDiagnostic> for ImportDiagnostic {
    fn from(diagnostic: &VerilogDiagnostic) -> Self {
        Self {
            code: ImportDiagnosticCode::StructuralScannerDiagnostic,
            severity: match diagnostic.severity {
                VerilogSeverity::Info => ImportSeverity::Info,
                VerilogSeverity::Warning => ImportSeverity::Warning,
                VerilogSeverity::Error => ImportSeverity::Error,
            },
            message: diagnostic.message.clone(),
            range: diagnostic.range.as_ref().map(ImportSourceRange::from),
        }
    }
}

impl From<&SystemVerilogDiagnostic> for ImportDiagnostic {
    fn from(diagnostic: &SystemVerilogDiagnostic) -> Self {
        Self {
            code: ImportDiagnosticCode::StructuralScannerDiagnostic,
            severity: match diagnostic.severity {
                SystemVerilogSeverity::Info => ImportSeverity::Info,
                SystemVerilogSeverity::Warning => ImportSeverity::Warning,
                SystemVerilogSeverity::Error => ImportSeverity::Error,
            },
            message: diagnostic.message.clone(),
            range: diagnostic.range.as_ref().map(ImportSourceRange::from),
        }
    }
}

/// Result of scanning an externally supplied HDL source.
///
/// `source` borrows the exact original input. No preprocessing, rewriting,
/// newline normalization, or source substitution is performed.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HdlImportReport<'source> {
    /// Caller-supplied source identity or display name.
    ///
    /// This is descriptive metadata, not a filesystem path opened by this
    /// module and not a canonical source-map identity.
    pub source_name: String,

    /// Explicitly selected input dialect.
    pub dialect: ImportDialect,

    /// Exact original source.
    pub source: &'source str,

    /// Discovered design units in source order.
    pub units: Vec<ImportedDesignUnit>,

    /// Deterministically ordered import diagnostics.
    pub diagnostics: Vec<ImportDiagnostic>,

    /// Whether source contains preprocessing directives.
    ///
    /// When true, the discovered structure may change after preprocessing.
    pub has_preprocessor_directives: bool,

    /// Whether the structural scanner reached the end of the input.
    ///
    /// This is not a statement that the grammar or semantics were validated.
    pub scan_complete: bool,
}

impl<'source> HdlImportReport<'source> {
    /// Return whether any diagnostic is an error.
    #[must_use]
    pub fn has_errors(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|diagnostic| diagnostic.severity == ImportSeverity::Error)
    }

    /// Return whether definitive structure requires preprocessing.
    #[must_use]
    pub const fn requires_preprocessing(&self) -> bool {
        self.has_preprocessor_directives
    }

    /// Return whether all discovered units have closing boundaries.
    ///
    /// This is a structural observation only. It is not full language
    /// validation and does not account for preprocessing-dependent branches.
    #[must_use]
    pub fn all_discovered_units_are_closed(&self) -> bool {
        self.units.iter().all(ImportedDesignUnit::is_closed)
    }

    /// Return the original source slice for a complete design unit.
    #[must_use]
    pub fn unit_source(&self, unit: &ImportedDesignUnit) -> Option<&'source str> {
        unit.source(self.source)
    }
}

/// Errors that prevent import scanning from completing.
///
/// Dialect-specific scanner errors are retained as typed values rather than
/// flattened into strings, so callers can inspect their exact failure cause.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ImportError {
    /// The source identity is empty or consists only of whitespace.
    InvalidSourceName,

    /// The requested range has a reversed endpoint.
    InvalidRange {
        /// Inclusive starting offset.
        start: usize,

        /// Exclusive ending offset.
        end: usize,
    },

    /// The Verilog scanner could not complete its scan.
    Verilog(super::verilog::VerilogError),

    /// The SystemVerilog scanner could not complete its scan.
    SystemVerilog(super::system_verilog::SystemVerilogError),
}

impl fmt::Display for ImportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidSourceName => {
                f.write_str("HDL import source name cannot be empty")
            }
            Self::InvalidRange { start, end } => {
                write!(f, "invalid HDL import source range {start}..{end}")
            }
            Self::Verilog(error) => write!(f, "Verilog import scan failed: {error}"),
            Self::SystemVerilog(error) => {
                write!(f, "SystemVerilog import scan failed: {error}")
            }
        }
    }
}

impl Error for ImportError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Verilog(error) => Some(error),
            Self::SystemVerilog(error) => Some(error),
            Self::InvalidSourceName | Self::InvalidRange { .. } => None,
        }
    }
}

fn normalize_system_verilog_kind(kind: SystemVerilogUnitKind) -> ImportedUnitKind {
    match kind {
        SystemVerilogUnitKind::Module => ImportedUnitKind::Module,
        SystemVerilogUnitKind::MacroModule => ImportedUnitKind::MacroModule,
        SystemVerilogUnitKind::Interface => ImportedUnitKind::Interface,
        SystemVerilogUnitKind::Program => ImportedUnitKind::Program,
        SystemVerilogUnitKind::Package => ImportedUnitKind::Package,
        SystemVerilogUnitKind::Checker => ImportedUnitKind::Checker,
        SystemVerilogUnitKind::Primitive => ImportedUnitKind::Primitive,
        SystemVerilogUnitKind::Class => ImportedUnitKind::Class,
        _ => ImportedUnitKind::Other,
    }
}

fn normalize_system_verilog_unit(
    unit: &SystemVerilogUnitBoundary,
) -> ImportedDesignUnit {
    ImportedDesignUnit {
        kind: normalize_system_verilog_kind(unit.kind),
        name: unit.name.clone(),
        opening_range: ImportSourceRange::from(&unit.opening_range),
        name_range: unit.name_range.as_ref().map(ImportSourceRange::from),
        closing_range: unit.closing_range.as_ref().map(ImportSourceRange::from),
        full_range: unit.full_range.as_ref().map(ImportSourceRange::from),
    }
}

fn normalize_diagnostics(
    mut diagnostics: Vec<ImportDiagnostic>,
) -> Vec<ImportDiagnostic> {
    diagnostics.sort_by(|left, right| {
        let left_start = left.range.as_ref().map_or(usize::MAX, |range| range.start);
        let right_start = right.range.as_ref().map_or(usize::MAX, |range| range.start);

        left_start
            .cmp(&right_start)
            .then_with(|| left.code.cmp(&right.code))
            .then_with(|| left.severity.cmp(&right.severity))
            .then_with(|| left.message.cmp(&right.message))
    });
    diagnostics
}

/// Scan externally supplied Verilog or SystemVerilog source.
///
/// This function performs structural discovery only. It does not read files,
/// access the network, expand preprocessor directives, parse complete
/// declarations, construct AST nodes, or validate HDL semantics.
///
/// # Arguments
///
/// - `source_name`: caller-supplied descriptive identity for diagnostics and
///   provenance. It is not opened or interpreted as a path.
/// - `source`: the exact source text to scan.
/// - `config`: explicit dialect and operational scanning policy.
///
/// # Errors
///
/// Returns an error if the source name is invalid or the selected underlying
/// scanner cannot complete its work, including when its source-size budget is
/// exceeded or it encounters an unterminated comment/string.
///
/// # Integration
///
/// After inspecting the report, the caller must pass the original source to
/// the appropriate grammar-driven parser and canonical AST adapter. A report
/// without structural errors is not proof of valid Verilog/SystemVerilog.
pub fn scan_imported_source<'source>(
    source_name: impl Into<String>,
    source: &'source str,
    config: &ImportConfig,
) -> Result<HdlImportReport<'source>, ImportError> {
    let source_name = source_name.into();
    if source_name.trim().is_empty() {
        return Err(ImportError::InvalidSourceName);
    }

    let (units, diagnostics, has_preprocessor_directives, scan_complete) =
        match config.dialect {
            ImportDialect::Verilog => {
                let scanner_config = VerilogScanConfig {
                    dialect: VerilogDialect::Verilog,
                    max_source_bytes: config.max_source_bytes,
                    report_unmatched_endmodule: config.report_unmatched_end_keywords,
                };

                let document = scan_verilog(source, &scanner_config)
                    .map_err(ImportError::Verilog)?;

                let units = document
                    .modules
                    .iter()
                    .map(|unit| ImportedDesignUnit {
                        kind: if unit.name.is_empty() {
                            ImportedUnitKind::Other
                        } else if unit
                            .declaration_keyword
                            .slice(source)
                            .is_some_and(|keyword| keyword == "macromodule")
                        {
                            ImportedUnitKind::MacroModule
                        } else {
                            ImportedUnitKind::Module
                        },
                        name: Some(unit.name.clone()),
                        opening_range: ImportSourceRange::from(
                            &unit.declaration_keyword,
                        ),
                        name_range: Some(ImportSourceRange::from(&unit.name_range)),
                        closing_range: unit
                            .end_keyword
                            .as_ref()
                            .map(ImportSourceRange::from),
                        full_range: unit
                            .full_range
                            .as_ref()
                            .map(ImportSourceRange::from),
                    })
                    .collect();

                let diagnostics = document
                    .diagnostics
                    .iter()
                    .map(ImportDiagnostic::from)
                    .collect();

                (
                    units,
                    diagnostics,
                    document.has_preprocessor_directives,
                    document.scan_complete,
                )
            }

            ImportDialect::SystemVerilog => {
                let scanner_config = SystemVerilogScanConfig {
                    max_source_bytes: config.max_source_bytes,
                    report_unmatched_end_keywords: config.report_unmatched_end_keywords,
                };

                let document = scan_system_verilog(source, &scanner_config)
                    .map_err(ImportError::SystemVerilog)?;

                let units = document
                    .units
                    .iter()
                    .map(normalize_system_verilog_unit)
                    .collect();

                let diagnostics = document
                    .diagnostics
                    .iter()
                    .map(ImportDiagnostic::from)
                    .collect();

                (
                    units,
                    diagnostics,
                    document.has_preprocessor_directives,
                    document.scan_complete,
                )
            }
        };

    let mut diagnostics = normalize_diagnostics(diagnostics);

    // The scanner already reports these conditions. The import API makes the
    // status available as fields as well, avoiding the need for consumers to
    // parse diagnostic text.
    if has_preprocessor_directives
        && !diagnostics.iter().any(|diagnostic| {
            diagnostic.code == ImportDiagnosticCode::PreprocessingRequired
        })
    {
        diagnostics.push(ImportDiagnostic {
            code: ImportDiagnosticCode::PreprocessingRequired,
            severity: ImportSeverity::Warning,
            message: "preprocessor directives were found; structural boundaries may depend on preprocessing".to_owned(),
            range: None,
        });
    }

    if !diagnostics.iter().any(|diagnostic| {
        diagnostic.code == ImportDiagnosticCode::ValidationIncomplete
    }) {
        diagnostics.push(ImportDiagnostic {
            code: ImportDiagnosticCode::ValidationIncomplete,
            severity: ImportSeverity::Info,
            message: "structural import scanning is not full grammar parsing, elaboration, or semantic validation".to_owned(),
            range: None,
        });
    }

    diagnostics = normalize_diagnostics(diagnostics);

    Ok(HdlImportReport {
        source_name,
        dialect: config.dialect,
        source,
        units,
        diagnostics,
        has_preprocessor_directives,
        scan_complete,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_is_explicitly_verilog_and_unbounded() {
        let config = ImportConfig::default();

        assert_eq!(config.dialect, ImportDialect::Verilog);
        assert_eq!(config.max_source_bytes, None);
        assert!(config.report_unmatched_end_keywords);
    }

    #[test]
    fn scans_verilog_and_preserves_original_unit_source() {
        let source = "module top(input wire a); endmodule\n";
        let report = scan_imported_source(
            "memory://fixture/top.v",
            source,
            &ImportConfig::unbounded(ImportDialect::Verilog),
        )
        .expect("valid source should be structurally scannable");

        assert_eq!(report.units.len(), 1);
        assert_eq!(report.units[0].kind, ImportedUnitKind::Module);
        assert_eq!(report.units[0].name.as_deref(), Some("top"));
        assert!(report.units[0].is_closed());
        assert_eq!(
            report.unit_source(&report.units[0]),
            Some("module top(input wire a); endmodule")
        );
        assert!(report.scan_complete);
        assert!(report.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == ImportDiagnosticCode::ValidationIncomplete
        }));
    }

    #[test]
    fn scans_system_verilog_design_units() {
        let source = "package types; endpackage\nmodule top; endmodule\n";
        let report = scan_imported_source(
            "fixture.sv",
            source,
            &ImportConfig::unbounded(ImportDialect::SystemVerilog),
        )
        .expect("valid source should be structurally scannable");

        assert_eq!(report.units.len(), 2);
        assert_eq!(report.units[0].kind, ImportedUnitKind::Package);
        assert_eq!(report.units[0].name.as_deref(), Some("types"));
        assert_eq!(report.units[1].kind, ImportedUnitKind::Module);
        assert_eq!(report.units[1].name.as_deref(), Some("top"));
        assert!(report.all_discovered_units_are_closed());
    }

    #[test]
    fn reports_preprocessor_dependent_structure() {
        let source = "`ifdef FEATURE\nmodule top; endmodule\n`endif\n";
        let report = scan_imported_source(
            "fixture.sv",
            source,
            &ImportConfig::unbounded(ImportDialect::SystemVerilog),
        )
        .expect("scanner should report directive presence");

        assert!(report.requires_preprocessing());
        assert!(report.diagnostics.iter().any(|diagnostic| {
            diagnostic.severity == ImportSeverity::Warning
        }));
    }

    #[test]
    fn source_budget_is_operational_and_enforced() {
        let source = "module top; endmodule";
        let result = scan_imported_source(
            "fixture.v",
            source,
            &ImportConfig::with_source_budget(ImportDialect::Verilog, 4),
        );

        assert!(matches!(
            result,
            Err(ImportError::Verilog(
                super::super::verilog::VerilogError::SourceBudgetExceeded {
                    ..
                }
            ))
        ));
    }

    #[test]
    fn rejects_empty_source_identity() {
        let result = scan_imported_source(
            "  ",
            "module top; endmodule",
            &ImportConfig::default(),
        );

        assert_eq!(result, Err(ImportError::InvalidSourceName));
    }

    #[test]
    fn import_ranges_check_utf8_boundaries_when_slicing() {
        let source = "module café; endmodule";
        let range = ImportSourceRange::new(7, 12)
            .expect("range endpoints are ordered");

        // The method safely returns None when offsets do not match UTF-8
        // boundaries; it never slices the string unchecked.
        assert!(range.slice(source).is_none());
    }

    #[test]
    fn scanner_errors_are_not_silently_converted_to_success() {
        let result = scan_imported_source(
            "broken.v",
            "module top; /* unterminated",
            &ImportConfig::default(),
        );

        assert!(matches!(
            result,
            Err(ImportError::Verilog(
                super::super::verilog::VerilogError::UnterminatedBlockComment {
                    ..
                }
            ))
        ));
    }
}
