// src/hdl/interoperability/mod.rs

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani HDL Interoperability
//!
//! Path: `src/hdl/interoperability/mod.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//! Status: Public interoperability composition boundary.
//!
//! ## Purpose
//!
//! Compose the existing HDL source import, export, and format-specific
//! scanning implementations into one stable public API.
//!
//! Supported format-specific boundaries currently include:
//!
//! - Verilog;
//! - SystemVerilog;
//! - VHDL;
//! - Zamani's format-preserving source export;
//! - extensible export formats through the existing backend contract.
//!
//! This module registers existing implementations. It does not duplicate
//! their scanners, define another AST, or implement format translation.
//!
//! ## Authority
//!
//! Normative authority:
//!
//! - `grammar/DESIGN.md`
//! - `grammar/spec/hdl.md`
//! - `grammar/spec/interoperability.md`
//! - `grammar/hdl/hdl.g4`
//! - `grammar/interoperability/hdl.g4`
//! - `grammar/interoperability/verilog.g4`
//! - the existing HDL grammar delegate files
//!
//! Implementation authority:
//!
//! - `src/frontend/lexer/`
//! - `src/frontend/parser/domains/hdl.rs`
//! - `src/frontend/semantic/domains/hdl.rs`
//! - `src/frontend/ast/node/`
//! - `src/frontend/semantic/model/`
//! - `src/hdl/interoperability/import.rs`
//! - `src/hdl/interoperability/export.rs`
//! - `src/hdl/interoperability/verilog.rs`
//! - `src/hdl/interoperability/system_verilog.rs`
//! - `src/hdl/interoperability/vhdl.rs`
//!
//! The normative specifications govern semantics. The existing grammar
//! owns syntax, the canonical AST owns parsed source structure, and semantic
//! analysis owns source-language validation.
//!
//! ## Ownership
//!
//! This composition module owns:
//!
//! - registration of the existing interoperability modules;
//! - the stable public API aggregation boundary;
//! - explicit re-exports of commonly used interoperability operations;
//! - facade identity and schema version;
//! - facade-level integration tests.
//!
//! ## Does not own
//!
//! This module does not own:
//!
//! - source parsing, lexing, or preprocessing;
//! - include resolution, filesystem access, or network access;
//! - a replacement AST, semantic model, or source-map implementation;
//! - complete Verilog, SystemVerilog, or VHDL parsing;
//! - macro expansion, elaboration, name resolution, or type checking;
//! - semantic translation between external HDL formats;
//! - synthesis, simulation, placement, routing, or physical allocation;
//! - target discovery or vendor-specific device interfaces;
//! - any intermediate representation implementation.
//!
//! All HDLIR implementation remains exclusively under
//! `src/hdl/hdlir/` and is outside this module's implementation scope.
//!
//! ## Interoperability correctness
//!
//! Structural scanning is not complete language parsing. A successful scan
//! MUST NOT be interpreted as proof that an input is syntactically valid,
//! semantically valid, synthesizable, or feasible on a target.
//!
//! The existing scanner diagnostics and completeness information must be
//! preserved by callers. The canonical grammar-driven parser and semantic
//! pipeline remain responsible for their respective validation stages.
//!
//! Export must not silently change source meaning. The existing export
//! implementation supports exact source preservation for identical formats
//! and exposes an explicit backend contract for translation. Unsupported
//! translations remain explicit errors.
//!
//! ## Scalability and POCO-REAF
//!
//! The composition boundary introduces no language-level maximum for source
//! bytes, design units, signals, ports, modules, or hardware dimensions.
//!
//! The underlying implementations may accept caller-configured operational
//! budgets. Such budgets are safeguards against excessive work or resource
//! consumption, not universal HDL semantic restrictions.
//!
//! An unbounded configuration does not guarantee that finite hardware,
//! memory, or execution time can process literally infinite input. Actual
//! processing remains subject to available resources.
//!
//! Source ranges remain byte offsets into the original UTF-8 source where
//! specified by the underlying module. They must not be confused with the
//! canonical frontend `Span` or source-map identities.
//!
//! ## Dependency direction
//!
//! ```text
//! External HDL source
//!        |
//!        v
//! interoperability::{import, verilog, system_verilog, vhdl}
//!        |
//!        v
//! Canonical grammar-driven parser
//!        |
//!        v
//! Canonical AST and semantic validation
//!        |
//!        v
//! Downstream compilation and realization
//! ```
//!
//! Export follows an explicit caller-selected path through `export.rs`.
//! Neither direction requires this composition module to depend on HDLIR.
//!
//! ## Integration and freeze criteria
//!
//! The five child files are authoritative implementations and must not be
//! copied into this file. A child API change should be reconciled here only
//! when it affects the documented public aggregation contract.
//!
//! This module is ready to freeze when:
//!
//! 1. Every registered child module exists and compiles.
//! 2. Public re-exports resolve to the authoritative child definitions.
//! 3. Import and scanning results retain their structural limitations.
//! 4. Unsupported translations and budget failures remain explicit.
//! 5. No universal language capacity limit is introduced.
//! 6. No unsafe Rust is introduced.
//! 7. The parent HDL module registers this module.
//! 8. Workspace formatting, compilation, linting, and relevant tests pass.

// -----------------------------------------------------------------------------
// Existing implementation modules
// -----------------------------------------------------------------------------

/// Source import request, structural import report, and import diagnostics.
///
/// This module currently normalizes Verilog and SystemVerilog scanner output.
/// It does not perform complete parsing or semantic validation.
pub mod import;

/// Source-preserving export and the explicit translation-backend contract.
///
/// Cross-format translation is not implied by registering this module.
pub mod export;

/// Verilog structural scanner and source-boundary representation.
///
/// Use the canonical grammar-driven parser for complete syntax validation.
pub mod verilog;

/// SystemVerilog structural scanner and design-unit discovery.
pub mod system_verilog;

/// VHDL structural scanner and design-unit discovery.
pub mod vhdl;

// -----------------------------------------------------------------------------
// Stable public API
// -----------------------------------------------------------------------------

/// Export source or invoke an explicitly selected translation backend.
pub use export::{
    export_source,
    export_with_backend,
    ExportConfig,
    ExportError,
    ExportReport,
    HdlExportBackend,
    HdlSourceFormat,
};

/// Import and structurally inspect Verilog or SystemVerilog source.
pub use import::{
    scan_imported_source,
    HdlImportReport,
    ImportConfig,
    ImportDiagnostic,
    ImportDiagnosticCode,
    ImportDialect,
    ImportError,
    ImportSeverity,
    ImportedDesignUnit,
    ImportedUnitKind,
    ImportSourceRange,
};

/// Scan Verilog source for structural module boundaries.
pub use verilog::{
    scan_verilog,
    VerilogDiagnostic,
    VerilogDiagnosticCode,
    VerilogDialect,
    VerilogDocument,
    VerilogError,
    VerilogModuleBoundary,
    VerilogScanConfig,
    VerilogSeverity,
    VerilogSourceRange,
};

/// Scan SystemVerilog source for structural design-unit boundaries.
pub use system_verilog::{
    scan_system_verilog,
    SystemVerilogDiagnostic,
    SystemVerilogDiagnosticCode,
    SystemVerilogDocument,
    SystemVerilogError,
    SystemVerilogScanConfig,
    SystemVerilogSeverity,
    SystemVerilogSourceRange,
    SystemVerilogUnitBoundary,
    SystemVerilogUnitKind,
};

/// Scan VHDL source for structural design-unit boundaries.
pub use vhdl::{
    scan_vhdl,
    VhdlDiagnostic,
    VhdlDiagnosticCode,
    VhdlDesignUnit,
    VhdlDocument,
    VhdlError,
    VhdlScanConfig,
    VhdlSeverity,
    VhdlSourceRange,
    VhdlUnitKind,
};

// -----------------------------------------------------------------------------
// Composition-boundary identity
// -----------------------------------------------------------------------------

/// Stable architectural name of the HDL interoperability API.
pub const HDL_INTEROPERABILITY_MODULE_NAME: &str =
    "zamani.hdl.interoperability";

/// Schema version of this public aggregation contract.
///
/// This is not a language version, external HDL standard version, grammar
/// version, AST version, or IR version.
pub const HDL_INTEROPERABILITY_SCHEMA_VERSION: u16 = 1;

/// Return the stable architectural module name.
#[must_use]
pub const fn module_name() -> &'static str {
    HDL_INTEROPERABILITY_MODULE_NAME
}

/// Return the schema version of this aggregation boundary.
#[must_use]
pub const fn schema_version() -> u16 {
    HDL_INTEROPERABILITY_SCHEMA_VERSION
}

// -----------------------------------------------------------------------------
// Integration tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn module_identity_and_schema_are_stable() {
        assert_eq!(
            module_name(),
            "zamani.hdl.interoperability"
        );
        assert_eq!(
            schema_version(),
            HDL_INTEROPERABILITY_SCHEMA_VERSION
        );
        assert!(schema_version() > 0);
    }

    #[test]
    fn same_format_export_preserves_source_exactly() {
        let source =
            "module top;\r\n  // retain formatting\r\nendmodule\r\n";
        let mut output = Vec::new();

        let report = export_source(
            source,
            &HdlSourceFormat::Verilog,
            &HdlSourceFormat::Verilog,
            &ExportConfig::unbounded(),
            &mut output,
        )
        .expect("same-format export should succeed");

        assert_eq!(output, source.as_bytes());
        assert!(report.preserves_format());
        assert!(report.lossless);
        assert_eq!(report.bytes_written, source.len());
    }

    #[test]
    fn verilog_scanner_is_available_through_the_facade() {
        let source = "module top; endmodule";

        let document = scan_verilog(
            source,
            &VerilogScanConfig::default(),
        )
        .expect("the structural scan should complete");

        assert!(!document.has_structural_errors());
    }

    #[test]
    fn system_verilog_scanner_is_available_through_the_facade() {
        let source = "module top; endmodule";

        let document = scan_system_verilog(
            source,
            &SystemVerilogScanConfig::unbounded(),
        )
        .expect("the structural scan should complete");

        assert!(!document.has_structural_errors());
    }

    #[test]
    fn vhdl_scanner_is_available_through_the_facade() {
        let source = "entity top is end entity top;";

        let document = scan_vhdl(
            source,
            &VhdlScanConfig::default(),
        )
        .expect("the structural scan should complete");

        assert!(!document.has_structural_errors());
    }

    #[test]
    fn import_facade_preserves_explicit_dialect_selection() {
        let source = "module top; endmodule";
        let config = ImportConfig::unbounded(
            ImportDialect::Verilog,
        );

        let report = scan_imported_source(
            "top.v",
            source,
            &config,
        )
        .expect("the import scan should complete");

        assert!(!report.has_errors());
        assert!(report.all_discovered_units_are_closed());
    }

    #[test]
    fn cross_format_export_is_not_silently_assumed() {
        let source = "module top; endmodule";
        let mut output = Vec::new();

        let result = export_source(
            source,
            &HdlSourceFormat::Verilog,
            &HdlSourceFormat::SystemVerilog,
            &ExportConfig::unbounded(),
            &mut output,
        );

        // The existing same-format export operation preserves source bytes;
        // it is not a substitute for a semantic translation backend.
        assert!(result.is_err());
    }
}