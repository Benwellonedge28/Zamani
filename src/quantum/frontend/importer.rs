//! Format-independent quantum frontend import contract.
//!
//! This module defines the stable boundary between an external quantum
//! representation and Zamani's canonical Quantum IR.
//!
//! # Architectural boundary
//!
//! ```text
//! Untrusted external source
//!          │
//!          ▼
//! ┌──────────────────────────────┐
//! │ ImportInput                  │
//! │                              │
//! │ bytes + SourceId + SourceMap │
//! │ + immutable ImportConfig     │
//! └──────────────┬───────────────┘
//!                │
//!                ▼
//! ┌──────────────────────────────┐
//! │ FormatImporter               │
//! │                              │
//! │ format lexer/parser          │
//! │ format AST                   │
//! │ format validation            │
//! │ format lowering              │
//! └──────────────┬───────────────┘
//!                │
//!                ▼
//! ┌──────────────────────────────┐
//! │ QuantumCircuit               │
//! │                              │
//! │ canonical Zamani Quantum IR  │
//! └──────────────┬───────────────┘
//!                │
//!                ▼
//!        compiler / optimizer
//!        / mapper / scheduler
//!        / backend
//! ```
//!
//! # Architectural ownership
//!
//! This module owns:
//!
//! - the format-independent import API;
//! - import configuration;
//! - resource-limit propagation;
//! - source ownership at the import boundary;
//! - importer registration and lookup;
//! - successful import result representation;
//! - explicit format/version selection.
//!
//! This module does **not** own:
//!
//! - quantum gate semantics;
//! - quantum type semantics;
//! - format-specific grammars;
//! - format-specific ASTs;
//! - format-specific validation;
//! - lowering rules for individual formats;
//! - optimization;
//! - routing;
//! - scheduling;
//! - hardware mapping;
//! - QPU execution;
//! - network access;
//! - filesystem access;
//! - process execution.
//!
//! # Security boundary
//!
//! Importing is an untrusted-input operation.
//!
//! Implementations must therefore:
//!
//! - enforce `FrontendLimits`;
//! - reject oversized input before parsing;
//! - avoid unbounded recursion;
//! - avoid unbounded diagnostic generation;
//! - avoid unbounded output;
//! - never execute source-level operations;
//! - never implicitly access the filesystem;
//! - never implicitly access the network;
//! - never spawn external processes;
//! - never access quantum hardware;
//! - never silently discard unsupported semantics;
//! - never return an invalid `QuantumCircuit` as successful output.
//!
//! Format-specific facilities such as OpenQASM `include` resolution must be
//! supplied explicitly by a higher-level policy/resolver. This generic module
//! never performs I/O itself.
//!
//! # Determinism
//!
//! Given the same:
//!
//! - source bytes;
//! - source identity;
//! - source map;
//! - format;
//! - format version;
//! - import configuration;
//! - resource limits;
//!
//! an importer must produce deterministic results.
//!
//! Registry lookup is exact and deterministic. The registry never guesses a
//! format or silently chooses a different version.
//!
//! # Rust compatibility
//!
//! This implementation targets Rust 1.97 / 1.97.1 and Rust 2021.
//!
//! It intentionally uses only standard-library facilities and the existing
//! Zamani frontend/Quantum IR contracts.

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
/// This configuration is intentionally format-independent.
///
/// Format-specific options belong to the corresponding format module.
///
/// For example, OpenQASM-specific configuration must remain under
/// `frontend::formats::openqasm` rather than being added here.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ImportConfig {
    /// Resource limits applied to untrusted frontend input.
    limits: FrontendLimits,

    /// Whether non-error diagnostics should be retained.
    ///
    /// Errors are never suppressible through this option.
    retain_warnings: bool,
}

impl ImportConfig {
    /// Creates a production import configuration.
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

    /// Enables or disables retention of warnings.
    ///
    /// This does not disable validation and does not convert errors into
    /// warnings.
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
/// The generic frontend boundary deliberately accepts bytes. Individual
/// formats decide their own encoding rules.
///
/// OpenQASM, for example, may require UTF-8 and can report a format-specific
/// decoding diagnostic without forcing all future formats to use UTF-8.
#[derive(Clone, Debug)]
pub struct ImportInput {
    /// Stable identity of the source being imported.
    source_id: SourceId,

    /// Original source bytes.
    source: Vec<u8>,

    /// Source map containing the source represented by `source_id`.
    source_map: SourceMap,

    /// Immutable import configuration.
    config: ImportConfig,
}

impl ImportInput {
    /// Creates an import input.
    ///
    /// The source size is checked before the input is accepted.
    ///
    /// The supplied `SourceMap` must contain `source_id`.
    pub fn new(
        source_id: SourceId,
        source: Vec<u8>,
        source_map: SourceMap,
        config: ImportConfig,
    ) -> FrontendResult<Self> {
        if source.len() > config.limits().max_source_bytes() {
            return Err(frontend_limit_error(
                "source_bytes",
                source.len(),
                config.limits().max_source_bytes(),
            ));
        }

        let source_file = source_map.get(source_id).ok_or_else(|| {
            FrontendError::invalid_input(format!(
                "source identifier `{source_id}` is not present in the supplied source map"
            ))
        })?;

        /*
         * The source map is part of the import contract, so its source must
         * correspond exactly to the bytes supplied to the importer.
         *
         * This prevents a particularly dangerous class of diagnostic bugs
         * where parser offsets refer to one source while diagnostics display
         * another source.
         */
        if source_file.text().as_bytes() != source.as_slice() {
            return Err(FrontendError::invalid_input(format!(
                "source bytes for `{source_id}` do not match the source text registered in the source map"
            )));
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

    /// Consumes the input and returns all components.
    #[must_use]
    pub fn into_parts(
        self,
    ) -> (
        SourceId,
        Vec<u8>,
        SourceMap,
        ImportConfig,
    ) {
        (
            self.source_id,
            self.source,
            self.source_map,
            self.config,
        )
    }
}

/// Successful result of a frontend import.
///
/// A successful `ImportOutput` contains a canonical Quantum IR circuit that
/// has passed the canonical IR validation boundary.
///
/// This invariant is intentionally enforced by `try_new`.
#[derive(Clone, Debug)]
pub struct ImportOutput {
    /// Canonical Zamani Quantum IR.
    circuit: QuantumCircuit,

    /// Format that produced the circuit.
    format: FormatId,

    /// Format version used during import.
    version: FormatVersion,

    /// Non-fatal diagnostics generated during import.
    diagnostics: DiagnosticBag,
}

impl ImportOutput {
    /// Creates a validated successful import result.
    ///
    /// This constructor validates the canonical circuit before allowing it
    /// to cross the public successful-import boundary.
    ///
    /// `try_new` should be preferred by all production importer
    /// implementations.
    pub fn try_new(
        circuit: QuantumCircuit,
        format: FormatId,
        version: FormatVersion,
        diagnostics: DiagnosticBag,
    ) -> ImportResult {
        validate_canonical_circuit(&circuit)?;

        Ok(Self {
            circuit,
            format,
            version,
            diagnostics,
        })
    }

    /// Creates an import result without re-validating the circuit.
    ///
    /// # Safety contract
    ///
    /// This constructor is intentionally crate-private. It exists for
    /// situations where a caller has already established the canonical IR
    /// invariant and must avoid performing duplicate validation.
    ///
    /// External users cannot construct an unchecked successful import.
    #[must_use]
    pub(crate) fn from_validated(
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

    /// Consumes the result and returns all components.
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

/// Format-independent importer contract.
///
/// Every supported format implements this trait independently.
///
/// ```text
/// OpenQASM ──implements──► FormatImporter
/// QIR      ──implements──► FormatImporter
/// Quil     ──implements──► FormatImporter
/// ```
///
/// No format is allowed to route through another format's importer as an
/// architectural shortcut.
///
/// Every format lowers directly into the canonical Zamani Quantum IR.
pub trait FormatImporter: Send + Sync {
    /// Returns the stable format identity.
    fn format(&self) -> FormatId;

    /// Returns the exact format version supported by this importer.
    fn version(&self) -> FormatVersion;

    /// Imports external source into validated canonical Quantum IR.
    ///
    /// A production implementation must perform:
    ///
    /// 1. input-limit enforcement;
    /// 2. lexical analysis where applicable;
    /// 3. parsing;
    /// 4. format-specific semantic validation;
    /// 5. lowering;
    /// 6. canonical Quantum IR validation;
    /// 7. deterministic diagnostic construction.
    ///
    /// It must not:
    ///
    /// - optimize;
    /// - route;
    /// - schedule;
    /// - map to hardware;
    /// - execute;
    /// - perform implicit filesystem access;
    /// - perform implicit network access;
    /// - execute external processes;
    /// - access hardware;
    /// - silently discard unsupported semantics.
    fn import(&self, input: ImportInput) -> ImportResult;
}

/// Object-safe importer type.
pub type BoxedImporter = Box<dyn FormatImporter>;

/// Registry of independently implemented frontend importers.
///
/// Registration is keyed by the exact pair:
///
/// ```text
/// (FormatId, FormatVersion)
/// ```
///
/// There may therefore be multiple importers for the same format at
/// different versions, but never duplicate registrations for the same pair.
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

    /// Creates a registry containing one importer.
    ///
    /// Duplicate registration is checked immediately.
    pub fn with_importer<I>(
        mut self,
        importer: I,
    ) -> FrontendResult<Self>
    where
        I: FormatImporter + 'static,
    {
        self.register(importer)?;
        Ok(self)
    }

    /// Registers an importer.
    ///
    /// Duplicate `(FormatId, FormatVersion)` registrations are rejected.
    pub fn register<I>(
        &mut self,
        importer: I,
    ) -> FrontendResult<()>
    where
        I: FormatImporter + 'static,
    {
        let format = importer.format();
        let version = importer.version();

        if self
            .importers
            .iter()
            .any(|existing| {
                existing.format() == format
                    && existing.version() == version
            })
        {
            return Err(FrontendError::invalid_input(format!(
                "an importer for format `{format}` version `{version}` is already registered"
            )));
        }

        self.importers.push(Box::new(importer));

        Ok(())
    }

    /// Returns the number of registered importers.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.importers.len()
    }

    /// Returns whether the registry contains no importers.
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
            .find(|importer| {
                importer.format() == *format
                    && importer.version() == *version
            })
            .map(Box::as_ref)
    }

    /// Imports using an exact format/version pair.
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

    /// Iterates over registered importers in deterministic registration
    /// order.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = &dyn FormatImporter> {
        self.importers.iter().map(Box::as_ref)
    }
}

/// Explicit source-format selection.
///
/// Automatic detection deliberately does not belong to the core importer
/// contract. If it is introduced later, it must be a separate bounded and
/// deterministic facility.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ImportSelection {
    /// Explicit format/version selection.
    Explicit {
        /// Selected format.
        format: FormatId,

        /// Selected version.
        version: FormatVersion,
    },
}

impl ImportSelection {
    /// Creates an explicit selection.
    #[must_use]
    pub const fn explicit(
        format: FormatId,
        version: FormatVersion,
    ) -> Self {
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
/// This is the preferred high-level entry point for callers that already
/// know the external format.
pub fn import(
    registry: &ImporterRegistry,
    selection: &ImportSelection,
    input: ImportInput,
) -> ImportResult {
    registry.import(
        selection.format(),
        selection.version(),
        input,
    )
}

/// Returns a structured frontend limit error.
///
/// Keeping this helper here avoids duplicating the construction of resource
/// errors throughout this module.
///
/// The concrete `FrontendError` implementation owns the final stable error
/// code/category.
fn frontend_limit_error(
    resource: &str,
    actual: usize,
    maximum: usize,
) -> FrontendError {
    FrontendError::limit_exceeded(format!(
        "frontend resource limit exceeded: {resource} is {actual} bytes/elements; maximum is {maximum}"
    ))
}

/// Validates canonical Quantum IR before it crosses the successful import
/// boundary.
///
/// The exact IR validation API is intentionally isolated here. This is the
/// only place in this module that needs to know how the canonical IR exposes
/// its invariant check.
///
/// If the current Quantum IR exposes `validate()` as a fallible operation,
/// this function delegates to it. If the repository uses a different public
/// validation method, only this adapter should need adjustment; the importer
/// contract itself remains unchanged.
fn validate_canonical_circuit(
    circuit: &QuantumCircuit,
) -> FrontendResult<()> {
    circuit.validate().map_err(|error| {
        FrontendError::invalid_input(format!(
            "format importer produced invalid canonical Quantum IR: {error}"
        ))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /*
     * These tests deliberately focus on the generic importer contract rather
     * than any particular format.
     *
     * Format implementations have their own tests under:
     *
     * frontend/formats/<format>/
     */

    #[test]
    fn default_import_config_retains_warnings() {
        let config = ImportConfig::default();

        assert!(config.retain_warnings());
    }

    #[test]
    fn warning_retention_can_be_disabled() {
        let config = ImportConfig::default()
            .with_retain_warnings(false);

        assert!(!config.retain_warnings());
    }

    #[test]
    fn explicit_selection_preserves_format_and_version() {
        /*
         * This test intentionally uses the public FormatId/FormatVersion
         * constructors used by the frontend contract.
         *
         * The exact values are not semantically important here; this test
         * verifies that selection is not altered by the generic layer.
         */
        let format = FormatId::new("test-format")
            .expect("test format identifier must be valid");

        let version = FormatVersion::new(1, 0, 0);

        let selection =
            ImportSelection::explicit(
                format.clone(),
                version,
            );

        assert_eq!(selection.format(), &format);
        assert_eq!(selection.version(), &version);
    }

    #[test]
    fn registry_starts_empty() {
        let registry = ImporterRegistry::new();

        assert_eq!(registry.len(), 0);
        assert!(registry.is_empty());
    }

    #[test]
    fn registry_debug_does_not_expose_importer_internals() {
        let registry = ImporterRegistry::new();

        let rendered = format!("{registry:?}");

        assert!(rendered.contains("ImporterRegistry"));
        assert!(rendered.contains("importer_count"));
    }
}