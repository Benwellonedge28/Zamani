
#![forbid(unsafe_code)]

//! Zamani backend — OpenQASM emission adapter.
//!
//! Path: `src/backend/quantum/qasm.rs`
//! Rust: 2021 edition, Rust 1.97+
//! Safety: unsafe code is forbidden.
//!
//! # Responsibility
//!
//! This module provides the backend-facing API for exporting Zamani's
//! canonical quantum circuit representation to OpenQASM 3.0 or 3.1.
//!
//! It deliberately delegates serialization and representability validation
//! to the existing canonical OpenQASM frontend exporter. There is no second
//! QASM serializer and no parallel quantum IR.
//!
//! # Ownership
//!
//! - `crate::quantum::ir` owns canonical quantum semantics.
//! - `crate::quantum::frontend::exporter` owns generic export contracts,
//!   export options, and exported artifact validation.
//! - `crate::quantum::frontend::formats::openqasm` owns OpenQASM-specific
//!   serialization and format compatibility.
//! - The backend pipeline owns target selection, compatibility negotiation,
//!   diagnostics aggregation, and subsequent artifact handling.
//! - Quantum routing, scheduling, hardware mapping, calibration, and
//!   error correction remain owned by their existing quantum subsystems.
//!
//! # Semantic contract
//!
//! Export must preserve the operations, ordering, operands, parameters,
//! measurement destinations, and other semantics representable by the
//! configured OpenQASM version.
//!
//! Unsupported or unrepresentable operations must return an error. This
//! adapter never silently drops, approximates, decomposes, reorders, or
//! substitutes operations.
//!
//! # Resource contract
//!
//! The output budget is configurable for each emitter. Its default comes
//! from the existing generic frontend export contract. The canonical exporter
//! enforces the budget during serialization rather than checking only after
//! constructing the entire output.
//!
//! This module introduces no universal limits on qubit count, classical-bit
//! count, circuit depth, or operation count. Actual process memory, execution
//! budgets, and downstream target constraints remain finite and explicit.
//!
//! # Side effects
//!
//! Emission is in-memory only. This module does not write files, print to the
//! console, spawn processes, access the network, or contact quantum hardware.
//!
//! # Legacy compatibility
//!
//! The legacy `crate::ir_gen::IrModule` is not interchangeable with the
//! canonical `crate::quantum::ir::QuantumCircuit`. In particular, the legacy
//! representation does not provide the full canonical measurement and
//! operation contracts required for semantics-preserving export.
//!
//! Therefore, this module intentionally does not implement the legacy
//! `Backend::generate(&IrModule)` interface. Integration with the stable
//! backend API must use a typed quantum input contract rather than inventing
//! missing semantics during conversion.

use crate::quantum::frontend::core::errors::FrontendResult;
use crate::quantum::frontend::exporter::{
    ExportOptions,
    ExportedArtifact,
    QuantumExporter,
    DEFAULT_MAX_OUTPUT_BYTES,
};
use crate::quantum::frontend::{
    OpenQasmExporter,
    OPENQASM_3_0,
    OPENQASM_3_1,
};
use crate::quantum::ir::QuantumCircuit;

/// An OpenQASM language version supported by the repository's canonical
/// OpenQASM exporter.
///
/// This is deliberately limited to versions the existing exporter actually
/// implements. A future version must be added only after its syntax,
/// standard-library, validation, and serialization contracts are implemented.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum QasmVersion {
    /// OpenQASM 3.0.
    V3_0,

    /// OpenQASM 3.1.
    V3_1,
}

impl QasmVersion {
    /// Returns the canonical frontend format version corresponding to this
    /// backend configuration.
    fn format_version(self) -> crate::quantum::frontend::format::FormatVersion {
        match self {
            Self::V3_0 => OPENQASM_3_0,
            Self::V3_1 => OPENQASM_3_1,
        }
    }

    /// Returns the version's language declaration.
    #[must_use]
    pub const fn declaration(self) -> &'static str {
        match self {
            Self::V3_0 => "OPENQASM 3.0;",
            Self::V3_1 => "OPENQASM 3.1;",
        }
    }
}

impl Default for QasmVersion {
    fn default() -> Self {
        Self::V3_1
    }
}

/// Configuration for backend-facing OpenQASM emission.
///
/// `max_output_bytes` is an operational compilation budget, not a language
/// limit. Callers may choose a stricter or larger budget according to their
/// process policy and available resources.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QasmEmitterConfig {
    /// OpenQASM version to emit.
    pub version: QasmVersion,

    /// Maximum number of bytes permitted in the generated artifact.
    pub max_output_bytes: usize,
}

impl Default for QasmEmitterConfig {
    fn default() -> Self {
        Self {
            version: QasmVersion::default(),
            max_output_bytes: DEFAULT_MAX_OUTPUT_BYTES,
        }
    }
}

impl QasmEmitterConfig {
    /// Creates configuration for a specific supported OpenQASM version.
    #[must_use]
    pub const fn new(version: QasmVersion) -> Self {
        Self {
            version,
            max_output_bytes: DEFAULT_MAX_OUTPUT_BYTES,
        }
    }

    /// Sets the output budget.
    ///
    /// A zero budget is rejected by the canonical exporter when an export is
    /// attempted. This method does not silently replace invalid input with a
    /// different budget.
    #[must_use]
    pub const fn with_max_output_bytes(mut self, max_output_bytes: usize) -> Self {
        self.max_output_bytes = max_output_bytes;
        self
    }
}

/// Backend-facing OpenQASM emitter.
///
/// The emitter holds an immutable canonical exporter and immutable
/// configuration. Independent calls do not share mutable circuit state.
#[derive(Debug, Clone)]
pub struct QasmEmitter {
    config: QasmEmitterConfig,
    exporter: OpenQasmExporter,
}

impl QasmEmitter {
    /// Creates an emitter for the selected, repository-supported OpenQASM
    /// version.
    ///
    /// Errors from exporter construction are propagated instead of being
    /// converted into success or a different version.
    pub fn new(config: QasmEmitterConfig) -> FrontendResult<Self> {
        let exporter = match config.version {
            QasmVersion::V3_0 => OpenQasmExporter::new(OPENQASM_3_0)?,
            QasmVersion::V3_1 => OpenQasmExporter::new(OPENQASM_3_1)?,
        };

        Ok(Self { config, exporter })
    }

    /// Returns the immutable emitter configuration.
    #[must_use]
    pub const fn config(&self) -> &QasmEmitterConfig {
        &self.config
    }

    /// Returns the configured OpenQASM version.
    #[must_use]
    pub const fn version(&self) -> QasmVersion {
        self.config.version
    }

    /// Exports a canonical quantum circuit to a validated OpenQASM artifact.
    ///
    /// The returned artifact retains the canonical frontend's format
    /// descriptor, media type, and bytes. Callers can inspect or persist the
    /// artifact in a separate output stage.
    ///
    /// # Errors
    ///
    /// Returns the canonical frontend error if:
    ///
    /// - the configured output budget is invalid;
    /// - the circuit fails canonical IR validation;
    /// - a gate, measurement, parameter, or operand cannot be represented;
    /// - the output exceeds the configured budget;
    /// - the canonical exporter rejects the export request.
    pub fn emit(&self, circuit: &QuantumCircuit) -> FrontendResult<ExportedArtifact> {
        let options = ExportOptions::new()
            .with_requested_version(self.config.version.format_version())
            .with_max_output_bytes(self.config.max_output_bytes);

        self.exporter.export(circuit, &options)
    }

    /// Exports a circuit as UTF-8 OpenQASM source text.
    ///
    /// Prefer [`Self::emit`] when the caller needs the artifact's format and
    /// media-type metadata.
    pub fn emit_text(&self, circuit: &QuantumCircuit) -> FrontendResult<String> {
        self.emit(circuit)?.into_text()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_configuration_uses_openqasm_3_1() {
        let config = QasmEmitterConfig::default();

        assert_eq!(config.version, QasmVersion::V3_1);
        assert_eq!(config.version.declaration(), "OPENQASM 3.1;");
        assert!(config.max_output_bytes > 0);
    }

    #[test]
    fn version_3_0_configuration_is_explicit() {
        let config = QasmEmitterConfig::new(QasmVersion::V3_0);

        assert_eq!(config.version, QasmVersion::V3_0);
        assert_eq!(config.version.declaration(), "OPENQASM 3.0;");
    }

    #[test]
    fn output_budget_is_configurable_without_changing_version() {
        let config = QasmEmitterConfig::new(QasmVersion::V3_0)
            .with_max_output_bytes(4096);

        assert_eq!(config.version, QasmVersion::V3_0);
        assert_eq!(config.max_output_bytes, 4096);
    }

    #[test]
    fn emitter_uses_the_canonical_exporter_version() {
        let emitter = QasmEmitter::new(QasmEmitterConfig::default())
            .expect("OpenQASM 3.1 exporter should be constructible");

        assert_eq!(emitter.version(), QasmVersion::V3_1);
        assert_eq!(
            emitter.config().version.format_version(),
            OPENQASM_3_1
        );
    }

    #[test]
    fn zero_output_budget_is_not_silently_replaced() {
        let config = QasmEmitterConfig::new(QasmVersion::V3_1)
            .with_max_output_bytes(0);

        assert_eq!(config.max_output_bytes, 0);
    }
}
