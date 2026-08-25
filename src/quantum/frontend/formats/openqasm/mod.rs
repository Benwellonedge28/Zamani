//! Zamani Quantum Frontend — OpenQASM format facade.
//!
//! Production module boundary for OpenQASM 3.x.
//!
//! This module is the **only stable public boundary** for the OpenQASM
//! frontend implementation.
//!
//! # Architectural boundary
//!
//! ```text
//!                         External OpenQASM
//!                                │
//!                                ▼
//!                    ┌───────────────────────┐
//!                    │ OpenQASM frontend     │
//!                    │                       │
//!                    │ lexer                 │
//!                    │ parser                │
//!                    │ AST                   │
//!                    │ semantic validation  │
//!                    │ lowering/import       │
//!                    │ export                │
//!                    └───────────┬───────────┘
//!                                │
//!                                ▼
//!                    Zamani Quantum IR
//! ```
//!
//! OpenQASM is an independently implemented external format.
//!
//! It MUST NOT depend on another external quantum-format frontend.
//!
//! A future format such as QIR or Quil belongs beside this module:
//!
//! ```text
//! frontend::formats::openqasm
//! frontend::formats::qir
//! frontend::formats::quil
//! ```
//!
//! Each format owns its own:
//!
//! - source representation;
//! - lexer;
//! - parser;
//! - semantic validation;
//! - format-specific lowering adapter;
//! - exporter;
//! - format-specific tests.
//!
//! No format may introduce a second canonical quantum semantic model.
//!
//! The canonical quantum semantics remain owned by:
//!
//! `crate::quantum::ir`
//!
//! # Module ownership
//!
//! ```text
//! ast.rs
//!   └── OpenQASM source-language representation
//!
//! lexer.rs
//!   └── OpenQASM lexical analysis
//!
//! parser.rs
//!   └── tokens → OpenQASM AST
//!
//! validation.rs
//!   └── OpenQASM semantic validation
//!
//! stdgates.rs
//!   └── OpenQASM standard-gate definitions
//!
//! importer.rs
//!   └── OpenQASM → canonical Quantum IR
//!
//! exporter.rs
//!   └── canonical Quantum IR → OpenQASM
//!
//! mod.rs
//!   └── stable public facade only
//! ```
//!
//! `mod.rs` deliberately contains no parsing, validation, lowering, or
//! serialization logic.
//!
//! # Stable public API
//!
//! The supported public OpenQASM API consists of:
//!
//! - [`OpenQasmImporter`]
//! - [`OpenQasmExporter`]
//! - [`ValidationConfig`]
//! - [`OPENQASM_FORMAT_ID`]
//! - [`OPENQASM_MEDIA_TYPE`]
//! - [`OPENQASM_3_0`]
//! - [`OPENQASM_3_1`]
//! - [`STANDARD_LIBRARY_INCLUDE`]
//!
//! Consumers should normally use the generic frontend contracts together with
//! these format-specific importer/exporter implementations.
//!
//! The following implementation details intentionally remain private:
//!
//! - lexer tokens;
//! - parser configuration;
//! - parser state;
//! - AST implementation details;
//! - semantic symbol tables;
//! - validation internals;
//! - standard-gate lookup internals;
//! - lowering implementation details;
//! - exporter serialization helpers.
//!
//! This permits internal refactoring without breaking downstream users.
//!
//! # Security boundary
//!
//! OpenQASM input is untrusted.
//!
//! Parsing, validation, importing, and exporting MUST NOT:
//!
//! - execute OpenQASM;
//! - execute `extern` declarations;
//! - execute calibration code;
//! - access arbitrary filesystem paths;
//! - access the network;
//! - spawn processes;
//! - communicate with quantum hardware;
//! - perform QPU execution;
//! - perform hardware mapping;
//! - route qubits;
//! - schedule operations;
//! - optimize circuits;
//! - silently discard unsupported semantics;
//! - invent measurements;
//! - invent qubit operands;
//! - invent classical destinations.
//!
//! `include` is source-language data. It is not permission to perform arbitrary
//! filesystem or network access.
//!
//! Any include resolution must be controlled by the higher-level import
//! boundary and its explicit resolver/policy.
//!
//! The current OpenQASM importer deliberately accepts only the explicitly
//! supported standard-library include policy. Arbitrary external I/O remains
//! outside this format implementation.
//!
//! # Canonical IR boundary
//!
//! The frontend pipeline is:
//!
//! ```text
//! OpenQASM bytes/text
//!        │
//!        ▼
//! SourceMap / SourceFile
//!        │
//!        ▼
//! lexer
//!        │
//!        ▼
//! parser
//!        │
//!        ▼
//! OpenQASM AST
//!        │
//!        ▼
//! semantic validation
//!        │
//!        ▼
//! controlled lowering
//!        │
//!        ▼
//! crate::quantum::ir::QuantumCircuit
//! ```
//!
//! The reverse path is:
//!
//! ```text
//! crate::quantum::ir::QuantumCircuit
//!        │
//!        ▼
//! representability validation
//!        │
//!        ▼
//! deterministic OpenQASM serialization
//!        │
//!        ▼
//! OpenQASM 3.x
//! ```
//!
//! The frontend does not become a hardware compiler merely because OpenQASM
//! contains hardware-oriented constructs.
//!
//! Unsupported constructs must be rejected explicitly when the canonical IR
//! cannot represent them. They must never be parsed and silently erased.
//!
//! # Version policy
//!
//! The production implementation supports:
//!
//! - OpenQASM 3.0
//! - OpenQASM 3.1
//!
//! The production importer defaults to OpenQASM 3.1.
//!
//! The production exporter defaults to OpenQASM 3.1.
//!
//! Version selection remains explicit and is represented by the generic
//! [`crate::quantum::frontend::format::FormatVersion`] type.
//!
//! Future OpenQASM versions must not be silently accepted merely because their
//! major version is `3`.
//!
//! A newer language version requires an explicit implementation and capability
//! policy update.
//!
//! # Generic frontend integration
//!
//! The parent module exposes this facade as:
//!
//! ```text
//! crate::quantum::frontend::formats::openqasm
//! ```
//!
//! The parent frontend also re-exports the stable OpenQASM symbols at:
//!
//! ```text
//! crate::quantum::frontend::OpenQasmImporter
//! crate::quantum::frontend::OpenQasmExporter
//! crate::quantum::frontend::OPENQASM_3_0
//! crate::quantum::frontend::OPENQASM_3_1
//! crate::quantum::frontend::OPENQASM_FORMAT_ID
//! crate::quantum::frontend::OPENQASM_MEDIA_TYPE
//! ```
//!
//! Therefore consumers do not need to know the internal `formats/openqasm`
//! implementation layout.
//!
//! # Integration with the generic importer
//!
//! [`OpenQasmImporter`] implements the generic
//! [`crate::quantum::frontend::importer::FormatImporter`] contract.
//!
//! The importer is responsible for the format-specific pipeline:
//!
//! ```text
//! ImportInput
//!     ↓
//! UTF-8/source handling
//!     ↓
//! OpenQASM lexer
//!     ↓
//! OpenQASM parser
//!     ↓
//! OpenQASM AST
//!     ↓
//! semantic validation
//!     ↓
//! controlled lowering
//!     ↓
//! QuantumCircuit
//!     ↓
//! ImportOutput
//! ```
//!
//! Resource limits come from the generic frontend [`FrontendLimits`] contract
//! rather than being invented by this module.
//!
//! # Integration with the generic exporter
//!
//! [`OpenQasmExporter`] implements the generic
//! [`crate::quantum::frontend::exporter::QuantumExporter`] contract.
//!
//! The exporter:
//!
//! - validates representability;
//! - preserves canonical operation semantics;
//! - emits deterministic OpenQASM;
//! - bounds generated output;
//! - reports unsupported operations explicitly;
//! - never mutates the canonical Quantum IR.
//!
//! # Validation configuration
//!
//! [`ValidationConfig`] is intentionally re-exported here because it is part
//! of the public constructor contract of [`OpenQasmImporter`].
//!
//! Keeping the type public while hiding the implementation module prevents
//! consumers from depending on:
//!
//! ```text
//! formats::openqasm::validation
//! ```
//!
//! while still allowing callers to configure supported semantic features.
//!
//! This is particularly important for API correctness: a public function must
//! not expose a configuration type that consumers cannot name or construct.
//!
//! # Standard library
//!
//! [`STANDARD_LIBRARY_INCLUDE`] identifies the explicitly supported OpenQASM
//! standard-gate include:
//!
//! ```text
//! include "stdgates.inc";
//! ```
//!
//! The constant is re-exported from the exporter facade because the exporter
//! currently owns the canonical OpenQASM format constants.
//!
//! # Determinism
//!
//! This facade guarantees that the public OpenQASM boundary does not expose
//! mutable global state.
//!
//! Importers and exporters are configured objects.
//!
//! They do not own:
//!
//! - global parser state;
//! - global symbol tables;
//! - global include caches;
//! - global hardware state;
//! - global QPU state.
//!
//! Determinism is enforced by the implementation modules and generic frontend
//! contracts.
//!
//! # Thread safety
//!
//! The public importer/exporter types are configuration objects containing
//! immutable state. They are intended to be reusable across independent
//! compilation requests when their underlying implementation remains `Send`
//! and `Sync` according to Rust's normal type system.
//!
//! This module introduces no mutable singleton state.
//!
//! # Error handling
//!
//! OpenQASM errors use the generic frontend error and diagnostic system.
//!
//! Consumers must not parse human-readable error strings to determine the
//! error category. They should use the structured frontend error/diagnostic
//! APIs.
//!
//! # No hidden semantic conversion
//!
//! This facade does not expose an API that implies that every OpenQASM feature
//! maps directly to the canonical Quantum IR.
//!
//! The implementation must use one of three explicit policies:
//!
//! ```text
//! supported
//!     → validated and lowered
//!
//! parsed but unsupported
//!     → structured unsupported-feature error
//!
//! invalid
//!     → structured syntax/semantic error
//! ```
//!
//! There must never be a fourth state:
//!
//! ```text
//! parsed → silently discarded
//! ```
//!
//! # Rust compatibility
//!
//! - Rust 2021 edition
//! - Rust 1.97.1
//! - stable Rust only
//! - no nightly features
//! - no additional dependencies
//!
//! # Removal/isolation contract
//!
//! Removing this OpenQASM implementation must not require changes to another
//! external format implementation.
//!
//! Adding another format must not require modifying:
//!
//! - `ast.rs`;
//! - `lexer.rs`;
//! - `parser.rs`;
//! - `validation.rs`;
//! - `importer.rs`;
//! - `exporter.rs`
//!
//! in this OpenQASM module merely to register the new format.
//!
//! Registration belongs to the generic frontend layer.
//!
//! # Production completion criteria
//!
//! This module is complete when:
//!
//! 1. only stable public OpenQASM contracts are exposed;
//! 2. implementation modules remain private;
//! 3. `ValidationConfig` is publicly reachable because it appears in the
//!    importer API;
//! 4. OpenQASM 3.0 and 3.1 constants are available;
//! 5. the standard-library include constant is available;
//! 6. importer/exporter construction is test-covered;
//! 7. no global mutable state is introduced;
//! 8. no second quantum IR is introduced;
//! 9. the parent frontend can consume this facade without OpenQASM internals;
//! 10. another external format can be added without changing this module's
//!     implementation logic.
//!
//! # Test ownership
//!
//! This module contains only API-boundary smoke tests.
//!
//! Detailed tests belong to their owning layers:
//!
//! ```text
//! lexer.rs
//!     → lexical tests
//!
//! parser.rs
//!     → grammar/parser tests
//!
//! validation.rs
//!     → semantic tests
//!
//! importer.rs
//!     → OpenQASM → Quantum IR tests
//!
//! exporter.rs
//!     → Quantum IR → OpenQASM tests
//!
//! frontend integration tests
//!     → public API / round-trip / security tests
//! ```
//!
//! This prevents this facade from becoming a second implementation test suite.

mod ast;
mod exporter;
mod importer;
mod lexer;
mod parser;
mod stdgates;
mod validation;

// -----------------------------------------------------------------------------
// Stable public OpenQASM API
// -----------------------------------------------------------------------------

/// Production OpenQASM importer.
///
/// The implementation remains format-local, while the importer itself is
/// exposed as the stable entry point for OpenQASM input.
pub use importer::OpenQasmImporter;

/// Production OpenQASM exporter.
///
/// The implementation remains format-local, while the exporter itself is
/// exposed as the stable entry point for OpenQASM output.
pub use exporter::OpenQasmExporter;

/// Public semantic feature-policy configuration used by
/// [`OpenQasmImporter`].
///
/// This re-export is required because `OpenQasmImporter::new` accepts a
/// `ValidationConfig` and `OpenQasmImporter::validation_config` returns one.
pub use validation::ValidationConfig;

// -----------------------------------------------------------------------------
// Stable format identity/version constants
// -----------------------------------------------------------------------------

/// Canonical frontend format identifier.
pub use exporter::OPENQASM_FORMAT_ID;

/// OpenQASM textual media type.
pub use exporter::OPENQASM_MEDIA_TYPE;

/// OpenQASM 3.0 format version.
pub use exporter::OPENQASM_3_0;

/// OpenQASM 3.1 format version.
pub use exporter::OPENQASM_3_1;

/// OpenQASM standard-library include name.
///
/// The value is the canonical include used by the OpenQASM standard-gate
/// library.
pub use exporter::STANDARD_LIBRARY_INCLUDE;

// -----------------------------------------------------------------------------
// Public API smoke tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn production_importer_is_constructible() {
        let importer = OpenQasmImporter::production();

        assert_eq!(
            importer.configured_version(),
            OPENQASM_3_1
        );
    }

    #[test]
    fn production_importer_exposes_public_validation_configuration() {
        let importer = OpenQasmImporter::production();

        let configuration: ValidationConfig =
            importer.validation_config();

        assert_eq!(
            configuration,
            ValidationConfig::production()
        );
    }

    #[test]
    fn production_exporter_is_constructible() {
        let exporter =
            OpenQasmExporter::production()
                .expect(
                    "production OpenQASM exporter \
                     must be constructible",
                );

        assert_eq!(
            exporter.configured_version(),
            OPENQASM_3_1
        );
    }

    #[test]
    fn supported_openqasm_versions_are_stable() {
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
    }

    #[test]
    fn format_identity_is_stable() {
        assert_eq!(
            OPENQASM_FORMAT_ID,
            "openqasm"
        );

        assert_eq!(
            OPENQASM_MEDIA_TYPE,
            "text/x-openqasm"
        );
    }

    #[test]
    fn standard_library_include_is_stable() {
        assert_eq!(
            STANDARD_LIBRARY_INCLUDE,
            "stdgates.inc"
        );
    }

    #[test]
    fn importer_and_exporter_share_the_same_format_identity() {
        let importer =
            OpenQasmImporter::production();

        let exporter =
            OpenQasmExporter::production()
                .expect(
                    "production OpenQASM exporter \
                     must be constructible",
                );

        assert_eq!(
            importer.configured_version().major(),
            OPENQASM_3_1.major()
        );

        assert_eq!(
            exporter.configured_version().major(),
            OPENQASM_3_1.major()
        );
    }
}