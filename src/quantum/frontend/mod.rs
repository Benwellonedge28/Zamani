//! Zamani Quantum Frontend.
//!
//! Production, format-independent boundary for translating external quantum
//! programming languages and interchange formats into Zamani's canonical
//! Quantum IR, and for exporting canonical Quantum IR back into supported
//! external formats.
//!
//! # Production contract
//!
//! The frontend is a compiler boundary, not a quantum execution layer.
//!
//! ```text
//!                    UNTRUSTED INPUT
//!                          │
//!                          ▼
//!                 ┌─────────────────┐
//!                 │   ImportInput   │
//!                 │ bytes + source  │
//!                 │ identity + map  │
//!                 │ limits/config   │
//!                 └────────┬────────┘
//!                          │
//!                          ▼
//!                 ┌─────────────────┐
//!                 │ Format Frontend │
//!                 │                 │
//!                 │ lex             │
//!                 │ parse           │
//!                 │ AST             │
//!                 │ validate        │
//!                 │ lower           │
//!                 └────────┬────────┘
//!                          │
//!                          ▼
//!                 ┌─────────────────┐
//!                 │ Canonical       │
//!                 │ Quantum IR      │
//!                 │ QuantumCircuit  │
//!                 └────────┬────────┘
//!                          │
//!          ┌───────────────┼────────────────┐
//!          ▼               ▼                ▼
//!      optimizer        routing         scheduler
//!          │               │                │
//!          └───────────────┼────────────────┘
//!                          ▼
//!                       backend
//!
//! Canonical Quantum IR
//!          │
//!          ▼
//!    generic exporter
//!          │
//!          ▼
//!   format exporter
//!          │
//!          ▼
//! deterministic external representation
//! ```
//!
//! # Architectural ownership
//!
//! This module owns only the frontend boundary and public module wiring.
//!
//! The frontend owns:
//!
//! - source representation;
//! - source locations;
//! - diagnostics;
//! - frontend errors;
//! - resource limits;
//! - external-format identity/version/capabilities;
//! - generic import contracts;
//! - generic export contracts;
//! - controlled lowering contracts;
//! - independently implemented external formats.
//!
//! The frontend does **not** own:
//!
//! - canonical quantum semantics;
//! - quantum algorithms;
//! - circuit optimization;
//! - hardware topology;
//! - qubit routing;
//! - scheduling;
//! - pulse generation;
//! - calibration execution;
//! - QPU execution;
//! - backend-specific compilation.
//!
//! Those responsibilities remain downstream of the canonical Quantum IR.
//!
//! # Canonical semantic boundary
//!
//! `crate::quantum::ir` is the only canonical semantic representation of a
//! Zamani quantum circuit.
//!
//! A frontend format must never introduce a second canonical quantum model.
//!
//! The dependency direction is therefore:
//!
//! ```text
//! external format
//!       │
//!       ▼
//! frontend format implementation
//!       │
//!       ▼
//! generic frontend contracts
//!       │
//!       ▼
//! QuantumCircuit
//!       │
//!       ▼
//! downstream quantum compiler
//! ```
//!
//! In particular, the frontend must not absorb responsibilities belonging to
//! `quantum::optimization`, `quantum::routing`, `quantum::scheduling`,
//! `quantum::hardware`, or execution infrastructure.
//!
//! # Format isolation
//!
//! Each external format is an independently removable adapter.
//!
//! ```text
//!                         frontend contracts
//!                         /       |       \
//!                        /        |        \
//!                   OpenQASM     QIR      Quil
//!                       │          │         │
//!                       └──────────┼─────────┘
//!                                  ▼
//!                           QuantumCircuit
//! ```
//!
//! A concrete format implementation:
//!
//! - owns its own lexer;
//! - owns its own parser;
//! - owns its own AST;
//! - owns its own semantic validation;
//! - owns its own lowering adapter;
//! - owns its own exporter;
//! - must not depend on another concrete format;
//! - must not require another concrete format to be modified when it is added
//!   or removed.
//!
//! The generic frontend intentionally contains no format-specific AST and no
//! central `match` over every supported external language.
//!
//! # Module dependency order
//!
//! The source tree is intentionally layered:
//!
//! ```text
//! core/source.rs
//! core/limits.rs
//! core/errors.rs
//! core/diagnostics.rs
//!          │
//!          ▼
//! format.rs
//! importer.rs
//! exporter.rs
//! lowering.rs
//!          │
//!          ▼
//! formats/openqasm/ast.rs
//! formats/openqasm/lexer.rs
//! formats/openqasm/parser.rs
//! formats/openqasm/stdgates.rs
//! formats/openqasm/validation.rs
//! formats/openqasm/importer.rs
//! formats/openqasm/exporter.rs
//!          │
//!          ▼
//! formats/openqasm/mod.rs
//!          │
//!          ▼
//! frontend/mod.rs
//! ```
//!
//! `mod.rs` is therefore intentionally a facade. It must not become a second
//! implementation layer.
//!
//! # Security boundary
//!
//! Frontend input is untrusted.
//!
//! Successful parsing/importing must never imply permission to:
//!
//! - access the filesystem;
//! - access the network;
//! - spawn a process;
//! - execute source-level directives;
//! - execute calibration code;
//! - communicate with a QPU;
//! - access quantum hardware;
//! - bypass frontend resource limits.
//!
//! Source-language constructs such as OpenQASM `include`, `extern`, calibration
//! declarations, pragmas, and annotations are data handled by the frontend.
//! They are not execution permissions.
//!
//! Any operation requiring I/O must be supplied through an explicit higher-level
//! integration boundary. The frontend itself remains side-effect free with
//! respect to external systems.
//!
//! # Resource safety
//!
//! Every concrete frontend must use [`FrontendLimits`] for potentially
//! unbounded input dimensions.
//!
//! At minimum, implementations must bound:
//!
//! - source bytes;
//! - tokens;
//! - AST nodes;
//! - identifier size;
//! - expression depth;
//! - parser nesting;
//! - gate-definition depth;
//! - include depth;
//! - qubit count;
//! - classical-bit count;
//! - operation count;
//! - symbol-table entries;
//! - diagnostics;
//! - diagnostic output;
//! - generated export output;
//! - other format-specific expansion work.
//!
//! The frontend must prefer deterministic rejection over unbounded resource
//! consumption.
//!
//! # Error and diagnostic boundary
//!
//! Malformed or unsupported input must cross the public boundary through the
//! structured frontend error/diagnostic system.
//!
//! Consumers must not parse human-readable error strings to determine error
//! categories.
//!
//! The stable concepts are:
//!
//! - [`FrontendError`];
//! - [`FrontendErrorKind`];
//! - [`FrontendResult`];
//! - [`Diagnostic`];
//! - [`DiagnosticBag`];
//! - [`DiagnosticSeverity`];
//! - [`SourceSpan`].
//!
//! # Source-location invariant
//!
//! All format implementations must use the shared source-location types.
//!
//! ```text
//! source bytes
//!     │
//!     ▼
//! SourceFile / SourceMap
//!     │
//!     ▼
//! SourceSpan
//!     │
//! ├── token
//! ├── AST node
//! ├── semantic diagnostic
//! ├── lowering provenance
//! └── export/source mapping where supported
//! ```
//!
//! No format may introduce a competing public source-span type.
//!
//! # Import boundary
//!
//! [`FormatImporter`] is the generic import contract.
//!
//! A successful import must result in a valid canonical
//! [`crate::quantum::ir::QuantumCircuit`].
//!
//! The expected pipeline is:
//!
//! ```text
//! ImportInput
//!      │
//!      ▼
//! source decoding
//!      │
//!      ▼
//! lexer
//!      │
//!      ▼
//! parser
//!      │
//!      ▼
//! format AST
//!      │
//!      ▼
//! semantic validation
//!      │
//!      ▼
//! controlled lowering
//!      │
//!      ▼
//! QuantumCircuit
//!      │
//!      ▼
//! ImportOutput
//! ```
//!
//! A format must never silently discard a construct during import.
//!
//! Every construct must be one of:
//!
//! ```text
//! supported
//!     → validated → lowered
//!
//! parsed but unsupported
//!     → structured unsupported-feature error
//!
//! invalid
//!     → structured syntax/semantic error
//! ```
//!
//! There must never be an implicit fourth state:
//!
//! ```text
//! parsed → silently discarded
//! ```
//!
//! # Export boundary
//!
//! [`QuantumExporter`] is the generic export contract.
//!
//! Exporting follows:
//!
//! ```text
//! QuantumCircuit
//!      │
//!      ▼
//! canonical IR validation
//!      │
//!      ▼
//! export request validation
//!      │
//!      ▼
//! format representability validation
//!      │
//!      ▼
//! deterministic serialization
//!      │
//!      ▼
//! bounded ExportedArtifact
//! ```
//!
//! Exporters must never silently:
//!
//! - drop an operation;
//! - invent an operation;
//! - reorder semantically observable operations;
//! - mutate the supplied circuit;
//! - access hardware;
//! - perform I/O;
//! - execute source-level semantics.
//!
//! # Lowering boundary
//!
//! [`LoweringConfig`], [`LoweringContext`], [`LoweringOperation`],
//! [`LoweringResult`], [`LoweringSource`], and [`LoweredGateResult`] define the
//! controlled boundary from validated external representations to canonical
//! Quantum IR.
//!
//! Lowering must preserve, where representable:
//!
//! - qubit identity;
//! - classical identity;
//! - operation ordering;
//! - parameters;
//! - measurements;
//! - control-flow semantics supported by the IR;
//! - source provenance;
//! - explicit semantic relationships.
//!
//! Lowering is not:
//!
//! - optimization;
//! - routing;
//! - scheduling;
//! - hardware mapping;
//! - execution.
//!
//! # OpenQASM
//!
//! OpenQASM is the first independently implemented production format.
//!
//! Its implementation lives entirely under:
//!
//! `frontend::formats::openqasm`
//!
//! The parent facade re-exports only its stable public boundary:
//!
//! - [`OpenQasmImporter`];
//! - [`OpenQasmExporter`];
//! - [`ValidationConfig`];
//! - [`OPENQASM_FORMAT_ID`];
//! - [`OPENQASM_MEDIA_TYPE`];
//! - [`OPENQASM_3_0`];
//! - [`OPENQASM_3_1`];
//! - [`STANDARD_LIBRARY_INCLUDE`].
//!
//! OpenQASM lexer/parser/AST/validation implementation details remain private
//! to the OpenQASM format module.
//!
//! OpenQASM 3.0 and 3.1 are explicit supported revisions. A future OpenQASM
//! revision must not be accepted merely because it has major version `3`.
//!
//! # Stable public API
//!
//! Normal downstream users should depend on this module rather than reaching
//! into implementation files.
//!
//! ```text
//! quantum::frontend
//!     │
//!     ├── generic frontend contracts
//!     ├── diagnostics/errors/source/limits
//!     └── concrete stable format facades
//! ```
//!
//! Internal modules remain accessible as Rust modules where required by the
//! existing repository layout, but concrete implementation types are not
//! re-exported from this root unless they are deliberately part of the stable
//! API.
//!
//! # Rust compatibility
//!
//! This module targets:
//!
//! - Rust 2021 edition;
//! - Rust 1.97 / 1.97.1;
//! - stable Rust only;
//! - no nightly features;
//! - no unsafe code;
//! - no additional dependencies.
//!
//! # Integration contract
//!
//! This file is deliberately the last frontend implementation boundary to
//! finalize.
//!
//! Earlier frontend files define their own contracts:
//!
//! - `core/source.rs` owns source identity and spans;
//! - `core/limits.rs` owns resource limits;
//! - `core/errors.rs` owns structured frontend errors;
//! - `core/diagnostics.rs` owns diagnostics;
//! - `format.rs` owns format identity/version/capability contracts;
//! - `importer.rs` owns generic import contracts;
//! - `exporter.rs` owns generic export contracts;
//! - `lowering.rs` owns controlled IR lowering contracts;
//! - `formats/openqasm/*` owns OpenQASM semantics.
//!
//! This module only wires those completed contracts together.
//!
//! Consequently, completing this file must not require later architectural
//! redesign of any earlier frontend file.
//!
//! # Removal and extension guarantee
//!
//! Removing OpenQASM must require only removal of its format subtree and its
//! registration/consumer wiring where applicable.
//!
//! Adding QIR, Quil, or another format must not require modifying the OpenQASM
//! lexer, parser, AST, validator, importer, exporter, or facade implementation.
//!
//! The generic contracts are the extension point.
//!
//! # Testing strategy
//!
//! This facade owns only API-boundary smoke tests.
//!
//! Detailed tests belong to their respective implementation layers:
//!
//! ```text
//! source.rs
//!     → source/location tests
//!
//! limits.rs
//!     → resource-limit tests
//!
//! diagnostics.rs
//!     → diagnostic tests
//!
//! lexer.rs
//!     → lexical tests
//!
//! parser.rs
//!     → grammar tests
//!
//! validation.rs
//!     → semantic tests
//!
//! importer.rs
//!     → format → Quantum IR tests
//!
//! exporter.rs
//!     → Quantum IR → format tests
//!
//! frontend integration tests
//!     → public API, security, determinism, round-trip, and contract tests
//! ```
//!
//! This keeps this facade small, deterministic, and incapable of becoming a
//! second implementation of the frontend.


// =============================================================================
// Format-independent core
// =============================================================================

/// Shared frontend infrastructure.
///
/// These modules are format-independent and must never depend on a concrete
/// external quantum language.
pub mod core {
    /// Structured compiler diagnostics.
    pub mod diagnostics;

    /// Structured frontend errors.
    pub mod errors;

    /// Resource and workload limits for untrusted frontend input.
    pub mod limits;

    /// Source identity, files, maps, spans, and source locations.
    pub mod source;
}


// =============================================================================
// Generic frontend contracts
// =============================================================================

/// Format identity, version, compatibility, and capability contracts.
pub mod format;

/// Format-independent import contract and importer registry.
pub mod importer;

/// Format-independent export contract and exporter registry.
pub mod exporter;

/// Controlled lowering contract from external representations to Quantum IR.
pub mod lowering;


// =============================================================================
// Independently removable external formats
// =============================================================================

/// External quantum-language implementations.
///
/// Each child format owns its own parser/AST/semantic implementation and
/// implements the generic frontend contracts independently.
pub mod formats {
    /// Production OpenQASM frontend.
    pub mod openqasm;
}


// =============================================================================
// Stable generic API
// =============================================================================

// Export boundary.
//
// Preserve the existing public exporter names. In particular, do not introduce
// an alternate `ExportConfig`/`FormatExporter` API merely for naming symmetry.
pub use exporter::{
    validate_export_request,
    validate_exported_artifact,
    ExportOptions,
    ExportVersionPolicy,
    ExportedArtifact,
    QuantumExporter,
    DEFAULT_MAX_OUTPUT_BYTES,
};

// Format contract.
pub use format::{
    FormatCapabilities,
    FormatCapability,
    FormatError,
    FormatId,
    FormatResult,
    FormatVersion,
    FrontendFormat,
};

// Import contract.
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

// Lowering contract.
pub use lowering::{
    LoweredGateResult,
    LoweringConfig,
    LoweringContext,
    LoweringOperation,
    LoweringResult,
    LoweringSource,
};


// =============================================================================
// Stable core API
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
//
// These are intentionally re-exported from the OpenQASM facade rather than
// reaching through its private implementation modules.
//
// This also guarantees that a consumer does not need to know that the
// implementation lives under `formats/openqasm`.

pub use formats::openqasm::{
    OpenQasmExporter,
    OpenQasmImporter,
    ValidationConfig,
    OPENQASM_3_0,
    OPENQASM_3_1,
    OPENQASM_FORMAT_ID,
    OPENQASM_MEDIA_TYPE,
    STANDARD_LIBRARY_INCLUDE,
};


// =============================================================================
// Stable frontend prelude
// =============================================================================

/// Stable frontend imports for compiler/front-end consumers.
///
/// The prelude intentionally contains contracts and stable format facades,
/// rather than lexer/parser/AST implementation types.
pub mod prelude {
    pub use super::{
        // Diagnostics.
        Diagnostic,
        DiagnosticBag,
        DiagnosticHelp,
        DiagnosticLabel,
        DiagnosticNote,
        DiagnosticSeverity,

        // Errors.
        FrontendError,
        FrontendErrorKind,
        FrontendResult,

        // Limits.
        FrontendLimits,

        // Source.
        LineColumn,
        SourceFile,
        SourceId,
        SourceMap,
        SourcePosition,
        SourceSpan,

        // Format contracts.
        FormatCapabilities,
        FormatCapability,
        FormatError,
        FormatId,
        FormatResult,
        FormatVersion,
        FrontendFormat,

        // Import.
        BoxedImporter,
        FormatImporter,
        ImportConfig,
        ImportInput,
        ImportOutput,
        ImportResult,
        ImportSelection,
        ImporterRegistry,

        // Export.
        validate_export_request,
        validate_exported_artifact,
        ExportOptions,
        ExportVersionPolicy,
        ExportedArtifact,
        QuantumExporter,
        DEFAULT_MAX_OUTPUT_BYTES,

        // Lowering.
        LoweredGateResult,
        LoweringConfig,
        LoweringContext,
        LoweringOperation,
        LoweringResult,
        LoweringSource,

        // OpenQASM stable facade.
        OpenQasmExporter,
        OpenQasmImporter,
        ValidationConfig,
        OPENQASM_3_0,
        OPENQASM_3_1,
        OPENQASM_FORMAT_ID,
        OPENQASM_MEDIA_TYPE,
        STANDARD_LIBRARY_INCLUDE,
    };
}


// =============================================================================
// Public API smoke tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generic_frontend_contracts_are_reachable() {
        let format = FormatId::new("openqasm")
            .expect("openqasm must be a valid frontend format identifier");

        assert_eq!(format.as_str(), "openqasm");

        let version = FormatVersion::major_minor(3, 1);

        assert_eq!(version.major(), 3);
        assert_eq!(version.minor(), 1);
        assert_eq!(version.patch(), 0);

        let _limits = FrontendLimits::default();
        let _config = ImportConfig::default();
        let _registry = ImporterRegistry::new();
        let _options = ExportOptions::default();

        assert!(DEFAULT_MAX_OUTPUT_BYTES > 0);
    }

    #[test]
    fn openqasm_public_facade_is_complete() {
        assert_eq!(
            OPENQASM_FORMAT_ID,
            "openqasm"
        );

        assert_eq!(
            OPENQASM_MEDIA_TYPE,
            "text/x-openqasm"
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
            OPENQASM_3_0.patch(),
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

        assert_eq!(
            OPENQASM_3_1.patch(),
            0
        );

        assert_eq!(
            STANDARD_LIBRARY_INCLUDE,
            "stdgates.inc"
        );
    }

    #[test]
    fn openqasm_production_importer_is_reachable() {
        let importer = OpenQasmImporter::production();

        assert_eq!(
            importer.configured_version(),
            OPENQASM_3_1
        );

        let configuration: ValidationConfig =
            importer.validation_config();

        assert_eq!(
            configuration,
            ValidationConfig::production()
        );
    }

    #[test]
    fn openqasm_production_exporter_is_reachable() {
        let exporter = OpenQasmExporter::production()
            .expect(
                "production OpenQASM exporter must be constructible",
            );

        assert_eq!(
            exporter.configured_version(),
            OPENQASM_3_1
        );
    }

    #[test]
    fn frontend_contains_no_global_runtime_state() {
        // The frontend facade intentionally has no initialization requirement.
        //
        // This test exists as an architectural guard: frontend construction
        // happens through explicit importer/exporter objects and registries,
        // rather than process-global mutable state.
        let registry = ImporterRegistry::new();

        assert!(registry.is_empty());
        assert_eq!(registry.len(), 0);
    }

    #[test]
    fn openqasm_versions_are_not_implicitly_collapsed() {
        assert_ne!(
            OPENQASM_3_0,
            OPENQASM_3_1
        );

        assert!(
            OPENQASM_3_0.is_older_than(
                OPENQASM_3_1
            )
        );

        assert!(
            OPENQASM_3_1.is_newer_than(
                OPENQASM_3_0
            )
        );
    }
}