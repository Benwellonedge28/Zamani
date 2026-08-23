//! Production Quantum Frontend boundary for Zamani.
//!
//! This module is the format-independent entry point for importing external
//! quantum programs into Zamani's canonical Quantum IR and exporting that IR
//! to external quantum formats.
//!
//! # Architectural boundary
//!
//! ```text
//! external source / IR
//!        │
//!        ▼
//! format-specific frontend
//! (lex / parse / validate)
//!        │
//!        ▼
//! format-independent contracts
//!        │
//!        ▼
//! controlled lowering
//!        │
//!        ▼
//! crate::quantum::ir::QuantumCircuit
//! ```
//!
//! The canonical Quantum IR remains the owner of quantum semantics and
//! invariants. The IR explicitly excludes frontend parsing, hardware mapping,
//! routing, scheduling, pulse/calibration ownership, execution, and backend
//! optimization. The frontend therefore must never introduce a second
//! semantic quantum model.
//!
//! # Format isolation
//!
//! Every external format is an independent adapter:
//!
//! ```text
//! OpenQASM ──┐
//! QIR      ──┼──► frontend contracts ──► Zamani Quantum IR
//! Quil     ──┘
//! ```
//!
//! A format implementation must not depend on another format implementation.
//! Adding or removing a format must not require changing an existing format's
//! parser, AST, validator, importer, exporter, or tests.
//!
//! The generic layer deliberately contains no OpenQASM-shaped AST and no
//! central `match` over all known formats. Format implementations advertise
//! their own identity/capabilities and implement the generic importer/exporter
//! contracts independently.
//!
//! # Module layout
//!
//! ```text
//! frontend/
//! ├── core/
//! │   ├── diagnostics.rs
//! │   ├── errors.rs
//! │   ├── limits.rs
//! │   └── source.rs
//! ├── formats/
//! │   └── openqasm/
//! │       ├── ast.rs
//! │       ├── exporter.rs
//! │       ├── importer.rs
//! │       ├── lexer.rs
//! │       ├── parser.rs
//! │       ├── stdgates.rs
//! │       └── validation.rs
//! ├── exporter.rs
//! ├── format.rs
//! ├── importer.rs
//! ├── lowering.rs
//! └── mod.rs
//! ```
//!
//! `core` and `formats` are declared here intentionally. This keeps the
//! current repository layout buildable without adding placeholder `mod.rs`
//! files that contain no architectural policy.
//!
//! # Public API policy
//!
//! The stable frontend API is exposed from this module. Implementation details
//! such as lexer/parser AST internals remain private to their format module.
//!
//! Downstream code should normally depend on:
//!
//! - frontend format identity/capabilities;
//! - import/export contracts;
//! - import/export configuration and results;
//! - lowering contracts;
//! - the selected format's stable importer/exporter.
//!
//! # Security boundary
//!
//! Frontend input is untrusted. Every format implementation must enforce
//! `FrontendLimits`, preserve source spans for diagnostics, remain panic-free
//! on malformed input, avoid uncontrolled recursion/allocation, and never
//! perform implicit filesystem, network, process, or hardware effects.
//!
//! Includes, `extern`, calibration, pulse, timing, and similar source-level
//! constructs are data to parse and validate; they are never executed merely
//! because they occur in a source document.
//!
//! # Rust compatibility
//!
//! Rust 2021 and Rust 1.97.1.
//! No nightly features are required by this module.


// =============================================================================
// Format-independent frontend infrastructure
// =============================================================================

/// Shared diagnostics, errors, input limits, and source-location primitives.
///
/// No type in this module may depend on a concrete quantum format or on the
/// canonical Quantum IR.
pub mod core {
    /// Deterministic structured diagnostics and diagnostic collections.
    pub mod diagnostics;

    /// Stable frontend error vocabulary and result type.
    pub mod errors;

    /// Resource limits for hostile or pathological frontend input.
    pub mod limits;

    /// Source identity, spans, files, maps, and location conversion.
    pub mod source;
}


// =============================================================================
// Stable format contracts
// =============================================================================

/// Format identity, version, and capability contracts.
pub mod format;

/// Format-independent import contract and importer registry.
pub mod importer;

/// Format-independent export contract and exporter registry.
pub mod exporter;

/// Controlled boundary from validated format representations into Quantum IR.
pub mod lowering;


// =============================================================================
// Concrete, independently removable formats
// =============================================================================

/// Independently implemented external quantum formats.
///
/// Each child format owns its own AST, lexical/syntactic rules, semantic
/// validation, lowering adapter, and exporter. The parent frontend exposes no
/// format-specific implementation details through this module.
pub mod formats {
    /// OpenQASM is the first production frontend format.
    ///
    /// Future formats such as QIR and Quil belong beside this module and must
    /// implement the generic contracts independently. They must not be added
    /// to OpenQASM's implementation or require OpenQASM changes.
    pub mod openqasm;
}


// =============================================================================
// Stable generic API re-exports
// =============================================================================

// The names below are deliberately limited to APIs confirmed in the existing
// frontend implementation. Do not introduce aliases such as ExportConfig or
// FormatExporter merely for naming symmetry; the current exporter contract
// uses ExportOptions and QuantumExporter.

pub use exporter::{
    ExportOptions,
    ExportVersionPolicy,
    ExportedArtifact,
    QuantumExporter,
    DEFAULT_MAX_OUTPUT_BYTES,
    validate_export_request,
    validate_exported_artifact,
};

pub use format::{
    FormatCapabilities,
    FormatCapability,
    FormatError,
    FormatId,
    FormatResult,
    FormatVersion,
    FrontendFormat,
};

pub use importer::{
    BoxedImporter,
    FormatImporter,
    ImportConfig,
    ImportInput,
    ImportOutput,
    ImportResult,
    ImportSelection,
    ImporterRegistry,
};

pub use lowering::{
    LoweredGateResult,
    LoweringConfig,
    LoweringContext,
    LoweringOperation,
    LoweringResult,
    LoweringSource,
};


// =============================================================================
// Stable core re-exports
// =============================================================================

pub use core::diagnostics::{
    Diagnostic,
    DiagnosticBag,
    DiagnosticHelp,
    DiagnosticLabel,
    DiagnosticNote,
    DiagnosticSeverity,
};

pub use core::errors::{
    FrontendError,
    FrontendErrorKind,
    FrontendResult,
};

pub use core::limits::FrontendLimits;

pub use core::source::{
    LineColumn,
    SourceFile,
    SourceId,
    SourceMap,
    SourcePosition,
    SourceSpan,
};


// =============================================================================
// Stable OpenQASM facade
// =============================================================================

pub use formats::openqasm::{
    OpenQasmExporter,
    OpenQasmImporter,
    OPENQASM_3_0,
    OPENQASM_3_1,
    OPENQASM_FORMAT_ID,
    OPENQASM_MEDIA_TYPE,
};


// =============================================================================
// Prelude
// =============================================================================

/// Stable frontend types intended for normal compiler/frontend consumers.
///
/// The prelude deliberately exposes contracts and stable results rather than
/// parser implementation details. Concrete format internals remain private.
pub mod prelude {
    pub use super::{
        Diagnostic,
        DiagnosticBag,
        DiagnosticHelp,
        DiagnosticLabel,
        DiagnosticNote,
        DiagnosticSeverity,

        ExportOptions,
        ExportVersionPolicy,
        ExportedArtifact,
        QuantumExporter,
        DEFAULT_MAX_OUTPUT_BYTES,

        FormatCapabilities,
        FormatCapability,
        FormatError,
        FormatId,
        FormatResult,
        FormatVersion,
        FrontendFormat,

        FormatImporter,
        ImportConfig,
        ImportInput,
        ImportOutput,
        ImportResult,
        ImportSelection,
        ImporterRegistry,

        FrontendError,
        FrontendErrorKind,
        FrontendLimits,
        FrontendResult,

        LineColumn,
        SourceFile,
        SourceId,
        SourceMap,
        SourcePosition,
        SourceSpan,

        LoweredGateResult,
        LoweringConfig,
        LoweringContext,
        LoweringOperation,
        LoweringResult,
        LoweringSource,
    };
}


// =============================================================================
// Architectural smoke tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn public_frontend_contracts_are_reachable() {
        let _format =
            FormatId::new("openqasm")
                .expect("openqasm is a valid format id");

        let _version =
            FormatVersion::major_minor(3, 1);

        let _limits =
            FrontendLimits::default();

        let _import_config =
            ImportConfig::default();

        let _import_registry =
            ImporterRegistry::new();

        let _export_options =
            ExportOptions::default();

        assert_eq!(
            DEFAULT_MAX_OUTPUT_BYTES,
            64 * 1024 * 1024
        );
    }

    #[test]
    fn openqasm_is_exposed_as_an_independent_format() {
        assert_eq!(
            OPENQASM_FORMAT_ID,
            "openqasm"
        );

        assert_eq!(
            OPENQASM_3_0.major(),
            3
        );

        assert_eq!(
            OPENQASM_3_0.minor(),
            0
        );

        assert_eq!(
            OPENQASM_3_1.major(),
            3
        );

        assert_eq!(
            OPENQASM_3_1.minor(),
            1
        );
    }
}