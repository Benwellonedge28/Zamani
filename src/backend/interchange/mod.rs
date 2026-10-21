
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! Backend interchange API for Zamani.
//!
//! Path: `src/backend/interchange/mod.rs`
//! Edition: Rust 2021.
//! Minimum supported Rust version: Rust 1.97.
//!
//! # Purpose
//!
//! This module is the public integration boundary for exporting existing
//! Zamani IR through explicitly selected interchange encoders.
//!
//! It deliberately delegates encoding and artifact construction to
//! [`universal_ir`]. It does not introduce a new IR, define a universal
//! serialization format, select compilation targets, execute programs,
//! write files, or launch external tools.
//!
//! # Important representation boundary
//!
//! The existing `crate::ir_gen::IrModule` is the input representation used
//! by the current adapter. It is not thereby declared to be the canonical
//! ZUIR representation or a lossless representation of every Zamani domain.
//!
//! In particular, quantum, HDL, AI/tensor, distributed, and future-domain
//! representations must be translated through their authoritative domain
//! and compiler integration paths. An encoder must explicitly reject input
//! it cannot faithfully represent.
//!
//! # Public API
//!
//! - [`export_to`] supports encoders that write incrementally to a checked
//!   writer.
//! - [`export_text`] adapts encoders that return a complete `String`.
//! - [`UniversalIrArtifact`] represents the completed encoded bytes.
//! - [`UniversalIrExportOptions`] supplies caller-selected output budgets.
//! - [`UniversalIrExportError`] distinguishes verification, encoding, and
//!   output failures.
//!
//! # Ownership and integration
//!
//! - `crate::ir_gen` owns the legacy IR input type.
//! - `crate::ir_verify` owns structural verification of that input.
//! - `universal_ir` owns the export adapter and its output contract.
//! - Concrete encoders own their format-specific syntax, schema, version,
//!   supported constructs, and semantic-preservation guarantees.
//! - `crate::compiler` owns compilation orchestration.
//! - The canonical ZUIR implementation, if used, remains a separate typed
//!   integration boundary; this module must not equate it with legacy IR.
//!
//! The existing root file `src/backend.rs` must declare
//! `pub mod interchange;` for this module to be accessible as
//! `crate::backend::interchange`.
//!
//! # Scalability and safety
//!
//! This module imposes no universal hardware, operation-count, module-count,
//! or program-size ceiling. Callers may select an output-byte budget through
//! [`UniversalIrExportOptions`]. Actual memory, address-space, time, and
//! operating-system constraints still apply.
//!
//! The streaming API can bound bytes accepted by its output writer, but it
//! cannot independently bound allocations made internally by an encoder.
//! Encoders must cooperate with resource policies where strict memory or
//! execution budgets are required.
//!
//! No unsafe code is permitted.
//!
//! # Grammar boundary
//!
//! The grammar defines source syntax, not interchange formats or backend
//! behavior. This module intentionally does not depend on ANTLR-generated
//! types, parse grammar files, or reconstruct source semantics.

pub mod universal_ir;

pub use universal_ir::{
    export_text,
    export_to,
    UniversalIrArtifact,
    UniversalIrExportError,
    UniversalIrExportOptions,
};

#[cfg(test)]
mod tests {
    use super::{
        export_text,
        export_to,
        UniversalIrArtifact,
        UniversalIrExportError,
        UniversalIrExportOptions,
    };
    use crate::ir_gen::IrModule;
    use std::io::Write;

    fn empty_module() -> IrModule {
        IrModule::new("interchange_module_api_test")
    }

    #[test]
    fn public_api_exposes_export_options_and_artifact_types() {
        let options = UniversalIrExportOptions::unbounded();
        let artifact: Option<UniversalIrArtifact> = None;
        let error: Option<UniversalIrExportError> = None;

        assert_eq!(options.max_output_bytes, None);
        assert!(artifact.is_none());
        assert!(error.is_none());
    }

    #[test]
    fn public_text_export_returns_a_complete_artifact() {
        let module = empty_module();

        let artifact = export_text(
            &module,
            "zamani.test-text-ir",
            UniversalIrExportOptions::unbounded(),
            |ir_text| {
                Ok::<String, &'static str>(format!(
                    "TEST-INTERCHANGE\n{ir_text}"
                ))
            },
        )
        .expect("the empty test module should pass IR verification");

        assert_eq!(artifact.format_id, "zamani.test-text-ir");
        assert_eq!(artifact.module_name, "interchange_module_api_test");
        assert_eq!(artifact.size_bytes, artifact.bytes.len());
        assert!(artifact.bytes.starts_with(b"TEST-INTERCHANGE\n"));
    }

    #[test]
    fn public_streaming_export_returns_complete_bytes() {
        let module = empty_module();

        let artifact = export_to(
            &module,
            "zamani.test-stream",
            UniversalIrExportOptions::with_max_output_bytes(128),
            |_ir_text, writer| {
                writer.write_all(b"streamed-artifact")?;
                Ok::<(), std::io::Error>(())
            },
        )
        .expect("the artifact fits the configured output budget");

        assert_eq!(artifact.as_bytes(), b"streamed-artifact");
        assert_eq!(artifact.size_bytes, artifact.as_bytes().len());
    }

    #[test]
    fn public_api_preserves_output_budget_failures() {
        let module = empty_module();

        let result = export_to(
            &module,
            "zamani.test-budget",
            UniversalIrExportOptions::with_max_output_bytes(3),
            |_ir_text, writer| {
                writer.write_all(b"four")?;
                Ok::<(), std::io::Error>(())
            },
        );

        assert!(matches!(
            result,
            Err(UniversalIrExportError::OutputLimitExceeded {
                limit_bytes: 3,
                ..
            })
        ));
    }
}
