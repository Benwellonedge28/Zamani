//! Zamani Quantum Frontend — OpenQASM format module.
//!
//! This module is the stable module boundary for the OpenQASM frontend.
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
//! The OpenQASM implementation is intentionally isolated from every other
//! external quantum format.
//!
//! A future format such as QIR or Quil must implement its own frontend under
//! `frontend::formats::<format>` and must lower directly to the canonical
//! Zamani Quantum IR. OpenQASM must never depend on another frontend format.
//!
//! # Module ownership
//!
//! ```text
//! ast.rs
//!   └── OpenQASM source representation
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
//!   └── OpenQASM → Zamani Quantum IR
//!
//! exporter.rs
//!   └── Zamani Quantum IR → OpenQASM
//! ```
//!
//! # Important ownership rule
//!
//! This module does NOT define another quantum IR.
//!
//! Canonical quantum semantics remain owned by
//! `crate::quantum::ir`.
//!
//! OpenQASM AST nodes represent source-language constructs only. They are
//! lowered into the canonical IR by the importer/lowering boundary.
//!
//! # Public API policy
//!
//! Parser, lexer, AST, and validation implementation details are kept private
//! to this format module. Consumers should normally use:
//!
//! - [`OpenQasmImporter`] for importing OpenQASM;
//! - [`OpenQasmExporter`] for exporting canonical Quantum IR;
//! - the exported OpenQASM format/version constants for format selection.
//!
//! This prevents downstream code from coupling itself to implementation
//! details that may change while preserving a stable OpenQASM frontend API.
//!
//! # No implicit effects
//!
//! Importing or exporting OpenQASM must never:
//!
//! - execute source code;
//! - execute `extern` declarations;
//! - access the network;
//! - access arbitrary filesystem paths;
//! - execute calibration code;
//! - communicate with a QPU;
//! - perform hardware mapping;
//! - route qubits;
//! - schedule operations;
//! - optimize the circuit;
//! - silently discard unsupported semantics;
//! - invent measurements;
//! - invent qubit operands.
//!
//! Those responsibilities belong to the appropriate compiler/backend layer.
//!
//! # Versioning
//!
//! OpenQASM versions are represented by the generic frontend
//! [`crate::quantum::frontend::format::FormatVersion`] type.
//!
//! The current production implementation exposes OpenQASM 3.0 and 3.1
//! constants through the exporter module.
//!
//! # Rust compatibility
//!
//! Rust 2021.
//! Rust 1.97.1.
//! No nightly features.
//! No additional dependencies.
//!
//! # Integration contract
//!
//! The parent frontend module must expose this module through:
//!
//! ```text
//! quantum::frontend::formats::openqasm
//! ```
//!
//! The OpenQASM implementation itself must not require modifications when
//! another format is added or removed.
//!
//! Conversely, removing this module and its implementation must not require
//! changes to another format implementation.
//!
//! # Internal module visibility
//!
//! The implementation modules remain private here. This is intentional:
//! downstream users should consume the importer/exporter contracts rather
//! than depend directly on lexer/parser/AST internals.

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

pub use exporter::{
    OpenQasmExporter,
    OPENQASM_3_0,
    OPENQASM_3_1,
    OPENQASM_FORMAT_ID,
    OPENQASM_MEDIA_TYPE,
};

pub use importer::OpenQasmImporter;

// -----------------------------------------------------------------------------
// Compile-time API smoke tests
// -----------------------------------------------------------------------------
//
// These tests intentionally verify only the public module boundary. They do
// not duplicate the lexer/parser/importer/exporter test suites.
//
// Their purpose is to ensure that:
//   1. the public OpenQASM entry points remain available;
//   2. the production constructors remain usable;
//   3. the module does not accidentally expose implementation internals.
//
// Format-specific behavior belongs in the corresponding implementation
// modules and integration tests.

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
    fn production_exporter_is_constructible() {
        let exporter =
            OpenQasmExporter::production()
                .expect("production OpenQASM exporter must be constructible");

        assert_eq!(
            exporter.configured_version(),
            OPENQASM_3_1
        );
    }

    #[test]
    fn openqasm_version_constants_are_openqasm_3() {
        assert_eq!(OPENQASM_3_0.major(), 3);
        assert_eq!(OPENQASM_3_0.minor(), 0);

        assert_eq!(OPENQASM_3_1.major(), 3);
        assert_eq!(OPENQASM_3_1.minor(), 1);
    }

    #[test]
    fn openqasm_format_identity_is_stable() {
        assert_eq!(OPENQASM_FORMAT_ID, "openqasm");
        assert_eq!(
            OPENQASM_MEDIA_TYPE,
            "text/x-openqasm"
        );
    }
}