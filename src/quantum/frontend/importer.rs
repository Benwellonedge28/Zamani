//! Format-independent quantum frontend import contract.
//!
//! This module defines the stable boundary between an external quantum
//! representation and Zamani's canonical Quantum IR.
//!
//! # Architectural boundary
//!
//! ```text
//! External source
//!      │
//!      ▼
//! ┌──────────────────────┐
//! │ Format-specific      │
//! │ lexer/parser/AST     │
//! └──────────┬───────────┘
//!            │
//!            ▼
//! ┌──────────────────────┐
//! │ Format validation    │
//! └──────────┬───────────┘
//!            │
//!            ▼
//! ┌──────────────────────┐
//! │ FormatImporter       │  ← this contract
//! └──────────┬───────────┘
//!            │
//!            ▼
//! ┌──────────────────────┐
//! │ Frontend lowering    │
//! └──────────┬───────────┘
//!            │
//!            ▼
//! ┌──────────────────────┐
//! │ Zamani Quantum IR    │
//! │ QuantumCircuit       │
//! └──────────────────────┘
//! ```
//!
//! The importer contract is intentionally format-independent.
//!
//! OpenQASM, QIR, Quil, or any future format must implement this boundary
//! independently. No format implementation may depend on another format
//! implementation.
//!
//! # Important ownership rules
//!
//! This module does **not** own:
//!
//! - quantum gate semantics;
//! - qubit semantics;
//! - measurement semantics;
//! - circuit invariants;
//! - optimization;
//! - routing;
//! - scheduling;
//! - hardware mapping;
//! - execution;
//! - format-specific parsing;
//! - format-specific ASTs.
//!
//! Those responsibilities belong to the appropriate frontend format,
//! canonical Quantum IR, compiler, algorithm, or backend layers.
//!
//! # Security
//!
//! Import is an untrusted-input boundary. Implementations must respect
//! frontend resource limits and must never execute source-level external
//! effects merely because they were encountered while importing.
//!
//! In particular, an importer must not implicitly:
//!
//! - access the network;
//! - execute external programs;
//! - access arbitrary filesystem paths;
//! - allocate unbounded memory;
//! - recurse without configured limits;
//! - silently discard unsupported semantics.
//!
//! # Rust compatibility
//!
//! This module is written for the repository's Rust 2021 / Rust 1.97.1
//! toolchain and intentionally uses only standard-library facilities plus
//! the project's existing frontend/IR contracts.

use std::fmt;

use super::core::diagnostics::DiagnosticBag;
use super::core::errors::{FrontendError, FrontendResult};
use super::core::limits::FrontendLimits;
use super::core::source::{SourceId, SourceMap};
use super::format::{FormatId, FormatVersion};

use crate::quantum::ir::QuantumCircuit;

/// Stable result type for frontend imports.
pub type ImportResult = FrontendResult<ImportOutput>;

/// Configuration supplied to an importer.
///
/// The configuration is deliberately format-neutral. Format-specific
/// configuration belongs to the corresponding format module.
///
/// For example, OpenQASM-specific options belong in
/// `frontend::formats::openqasm`, not here.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ImportConfig {
    /// Resource limits applied to untrusted frontend input.
    limits: FrontendLimits,

    /// Whether warnings should be retained in the import result.
    ///
    /// Errors are always retained and always cause import failure.
    retain_warnings: bool,
}

impl ImportConfig {
    /// Creates a production configuration using the supplied frontend
    /// resource limits.
    #[must_use]
    pub fn new(limits: FrontendLimits) -> Self {
        Self {
            limits,
            retain_warnings: true,
        }
    }

    /// Returns the configured frontend resource limits.
    #[must_use]
    pub const fn limits(&self) -> &FrontendLimits {
        &self.limits
    }

    /// Returns whether warnings should be retained.
    #[must_use]
    pub const fn retain_warnings(&self) -> bool {
        self.retain_warnings
    }

    /// Enables or disables warning retention.
    ///
    /// This does not disable validation. Warnings and errors are separate
    /// concepts; an implementation must never turn an error into a warning.
    #[must_use]
    pub const fn with_retain_warnings(mut self, retain: bool) -> Self {
        self.retain_warnings = retain;
        self
    }
}

impl Default for ImportConfig {
    fn default() -> Self {
        Self::new(FrontendLimits::default())
    }
}

/// Input supplied to a format importer.
///
/// The source itself is kept as bytes rather than assuming UTF-8 at the
/// generic boundary. Individual formats decide whether their source
/// representation requires UTF-8 or another encoding.
///
/// This prevents the generic importer contract from accidentally becoming
/// OpenQASM-specific.
#[derive(Clone, Debug)]
pub struct ImportInput {
    /// Stable source identity.
    source_id: SourceId,

    /// Original source bytes.
    source: Vec<u8>,

    /// Source map containing the source associated with `source_id`.
    ///
    /// Keeping this alongside the input ensures diagnostics can preserve
    /// source identity without requiring global mutable state.
    source_map: SourceMap,

    /// Import configuration.
    config: ImportConfig,
}

impl ImportInput {
    /// Creates a new import input.
    ///
    /// Implementations should reject the input through their normal
    /// `FrontendError` path if the source exceeds configured limits.
    pub fn new(
        source_id: SourceId,
        source: Vec<u8>,
        source_map: SourceMap,
        config: ImportConfig,
    ) -> FrontendResult<Self> {
        if source.len() > config.limits().max_source_bytes() {
            return Err(FrontendError::limit_exceeded(
                "frontend source exceeds the configured maximum size",
            ));
        }

        Ok(Self {
            source_id,
            source,
            source_map,
            config,
        })
    }

    /// Returns the source identity.
    #[must_use]
    pub const fn source_id(&self) -> SourceId {
        self.source_id
    }

    /// Returns the original source bytes.
    #[must_use]
    pub fn source(&self) -> &[u8] {
        &self.source
    }

    /// Returns the source map.
    #[must_use]
    pub const fn source_map(&self) -> &SourceMap {
        &self.source_map
    }

    /// Returns the import configuration.
    #[must_use]
    pub const fn config(&self) -> &ImportConfig {
        &self.config
    }

    /// Consumes the input and returns its source bytes.
    #[must_use]
    pub fn into_source(self) -> Vec<u8> {
        self.source
    }
}

/// Successful result of an import operation.
///
/// A successful import contains a canonical Zamani Quantum IR circuit and
/// any non-fatal diagnostics generated during processing.
///
/// The circuit must have passed the canonical IR validation boundary before
/// being returned as successful output.
#[derive(Clone, Debug)]
pub struct ImportOutput {
    /// Canonical Zamani Quantum IR.
    circuit: QuantumCircuit,

    /// Format that produced the circuit.
    format: FormatId,

    /// Format version used during import.
    version: FormatVersion,

    /// Non-fatal diagnostics produced while importing.
    diagnostics: DiagnosticBag,
}

impl ImportOutput {
    /// Creates a successful import result.
    ///
    /// The importer implementation is responsible for ensuring that
    /// `circuit` has already passed canonical Quantum IR validation.
    #[must_use]
    pub fn new(
        circuit: QuantumCircuit,
        format: FormatId,
        version: FormatVersion,
        diagnostics: DiagnosticBag,
    ) -> Self {
        Self {
            circuit,
            format,
            version,
            diagnostics,
        }
    }

    /// Returns the imported canonical Quantum IR.
    #[must_use]
    pub const fn circuit(&self) -> &QuantumCircuit {
        &self.circuit
    }

    /// Consumes the output and returns the circuit.
    #[must_use]
    pub fn into_circuit(self) -> QuantumCircuit {
        self.circuit
    }

    /// Returns the source format.
    #[must_use]
    pub const fn format(&self) -> &FormatId {
        &self.format
    }

    /// Returns the source format version.
    #[must_use]
    pub const fn version(&self) -> &FormatVersion {
        &self.version
    }

    /// Returns diagnostics generated during import.
    #[must_use]
    pub const fn diagnostics(&self) -> &DiagnosticBag {
        &self.diagnostics
    }

    /// Consumes the output and returns all components.
    #[must_use]
    pub fn into_parts(
        self,
    ) -> (
        QuantumCircuit,
        FormatId,
        FormatVersion,
        DiagnosticBag,
    ) {
        (
            self.circuit,
            self.format,
            self.version,
            self.diagnostics,
        )
    }
}

/// Capability-independent importer contract.
///
/// Each supported quantum format implements this trait independently.
///
/// Example:
///
/// ```text
/// OpenQASM ──implements──► FormatImporter
/// QIR      ──implements──► FormatImporter
/// Quil     ──implements──► FormatImporter
/// ```
///
/// There is deliberately no relationship such as:
///
/// ```text
/// OpenQASM → QIR → IR
/// ```
///
/// Every format lowers independently to the canonical Zamani Quantum IR.
pub trait FormatImporter: Send + Sync {
    /// Returns the stable format identity.
    fn format(&self) -> FormatId;

    /// Returns the format version supported by this importer.
    fn version(&self) -> FormatVersion;

    /// Imports source into validated canonical Zamani Quantum IR.
    ///
    /// Implementations must:
    ///
    /// 1. enforce configured frontend limits;
    /// 2. lex/tokenize if applicable;
    /// 3. parse into the format's own representation;
    /// 4. perform format-specific semantic validation;
    /// 5. lower into Zamani Quantum IR;
    /// 6. invoke canonical IR validation;
    /// 7. return errors instead of silently discarding semantics.
    ///
    /// Implementations must not:
    ///
    /// - optimize the circuit;
    /// - route qubits;
    /// - schedule operations;
    /// - execute operations;
    /// - perform hardware mapping;
    /// - access the network implicitly;
    /// - execute external programs;
    /// - silently drop unsupported constructs.
    fn import(&self, input: ImportInput) -> ImportResult;
}

/// Object-safe alias for dynamically selected frontend importers.
pub type BoxedImporter = Box<dyn FormatImporter>;

/// Registry of independently implemented importers.
///
/// The registry deliberately stores importer objects rather than matching
/// on every possible format in a giant central `match`.
///
/// This means adding a new format does not require changing the implementation
/// of existing formats.
///
/// A caller may construct a registry containing only the formats it wants:
///
/// ```text
/// OpenQASM only
/// OpenQASM + QIR
/// OpenQASM + Quil
/// private/custom format only
/// ```
///
/// The registry itself is optional infrastructure; individual format
/// implementations remain independently removable.
#[derive(Default)]
pub struct ImporterRegistry {
    importers: Vec<BoxedImporter>,
}

impl fmt::Debug for ImporterRegistry {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ImporterRegistry")
            .field("importer_count", &self.importers.len())
            .finish()
    }
}

impl ImporterRegistry {
    /// Creates an empty importer registry.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            importers: Vec::new(),
        }
    }

    /// Creates a registry with the supplied importers.
    #[must_use]
    pub fn with_importer<I>(mut self, importer: I) -> Self
    where
        I: FormatImporter + 'static,
    {
        self.importers.push(Box::new(importer));
        self
    }

    /// Registers an importer.
    ///
    /// Registration rejects duplicate `(FormatId, FormatVersion)` pairs.
    pub fn register<I>(&mut self, importer: I) -> FrontendResult<()>
    where
        I: FormatImporter + 'static,
    {
        let format = importer.format();
        let version = importer.version();

        if self
            .importers
            .iter()
            .any(|existing| existing.format() == format && existing.version() == version)
        {
            return Err(FrontendError::invalid_input(
                "an importer for this format and version is already registered",
            ));
        }

        self.importers.push(Box::new(importer));
        Ok(())
    }

    /// Returns the number of registered importers.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.importers.len()
    }

    /// Returns whether no importers are registered.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.importers.is_empty()
    }

    /// Finds an importer for an exact format/version pair.
    #[must_use]
    pub fn get(
        &self,
        format: &FormatId,
        version: &FormatVersion,
    ) -> Option<&dyn FormatImporter> {
        self.importers
            .iter()
            .find(|importer| importer.format() == *format && importer.version() == *version)
            .map(Box::as_ref)
    }

    /// Imports using an exact format/version pair.
    ///
    /// This is the preferred registry entry point when the caller already
    /// knows the external format.
    pub fn import(
        &self,
        format: &FormatId,
        version: &FormatVersion,
        input: ImportInput,
    ) -> ImportResult {
        let importer = self.get(format, version).ok_or_else(|| {
            FrontendError::unsupported(format!(
                "no importer is registered for format `{format}` version `{version}`"
            ))
        })?;

        importer.import(input)
    }

    /// Returns an iterator over registered importers.
    pub fn iter(&self) -> impl Iterator<Item = &dyn FormatImporter> {
        self.importers.iter().map(Box::as_ref)
    }
}

/// Explicit source-format selection.
///
/// Automatic format detection is intentionally not part of the core import
/// contract. If automatic detection is eventually introduced, it should be
/// a separate bounded, deterministic facility rather than changing the
/// semantics of `FormatImporter`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ImportSelection {
    /// Explicitly select a format and version.
    Explicit {
        /// Format identity.
        format: FormatId,

        /// Format version.
        version: FormatVersion,
    },
}

impl ImportSelection {
    /// Creates an explicit format selection.
    #[must_use]
    pub const fn explicit(format: FormatId, version: FormatVersion) -> Self {
        Self::Explicit { format, version }
    }

    /// Returns the selected format.
    #[must_use]
    pub const fn format(&self) -> &FormatId {
        match self {
            Self::Explicit { format, .. } => format,
        }
    }

    /// Returns the selected version.
    #[must_use]
    pub const fn version(&self) -> &FormatVersion {
        match self {
            Self::Explicit { version, .. } => version,
        }
    }
}

/// Imports using an explicit format selection.
///
/// This function provides a small stable facade for callers that do not need
/// direct access to the registry implementation.
pub fn import(
    registry: &ImporterRegistry,
    selection: &ImportSelection,
    input: ImportInput,
) -> ImportResult {
    registry.import(selection.format(), selection.version(), input)
}

/// Imports a source using a specific importer.
///
/// This is useful for applications that deliberately avoid a registry and
/// want to compile against exactly one format implementation.
pub fn import_with<I>(importer: &I, input: ImportInput) -> ImportResult
where
    I: FormatImporter + ?Sized,
{
    importer.import(input)
}

/// Verifies that a successful import satisfies the canonical IR boundary.
///
/// Format implementations should normally perform this check as the final
/// lowering step. This helper exists so that the common importer contract
/// has one canonical place for the invariant.
///
/// The exact IR validation API remains owned by `quantum::ir`; this module
/// does not duplicate those rules.
pub fn validate_imported_circuit(
    circuit: &QuantumCircuit,
) -> FrontendResult<()> {
    circuit
        .validate()
        .map_err(|error| FrontendError::lowering(error.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn import_selection_preserves_explicit_identity() {
        let format = FormatId::new("test");
        let version = FormatVersion::new(1, 0, 0);

        let selection = ImportSelection::explicit(format.clone(), version.clone());

        assert_eq!(selection.format(), &format);
        assert_eq!(selection.version(), &version);
    }

    #[test]
    fn empty_registry_is_empty() {
        let registry = ImporterRegistry::new();

        assert!(registry.is_empty());
        assert_eq!(registry.len(), 0);
    }

    #[test]
    fn config_retains_warnings_by_default() {
        let config = ImportConfig::default();

        assert!(config.retain_warnings());
    }

    #[test]
    fn config_can_disable_warning_retention() {
        let config = ImportConfig::default().with_retain_warnings(false);

        assert!(!config.retain_warnings());
    }
}