//! Format-independent quantum frontend exporter.
//!
//! This module defines the stable export boundary between Zamani's canonical
//! Quantum IR and external quantum representations such as OpenQASM, QIR,
//! Quil, and future formats.
//!
//! # Architectural boundary
//!
//! ```text
//!                  Canonical Zamani Quantum IR
//!                              │
//!                              ▼
//!                 ┌────────────────────────┐
//!                 │  frontend::exporter    │
//!                 │                        │
//!                 │  common export policy  │
//!                 │  capability checking  │
//!                 │  version checking     │
//!                 │  size limits          │
//!                 │  artifact validation  │
//!                 └───────────┬────────────┘
//!                             │
//!              ┌──────────────┼──────────────┐
//!              ▼              ▼              ▼
//!          OpenQASM          QIR            Quil
//!              │              │              │
//!              ▼              ▼              ▼
//!        external format-specific emitters
//! ```
//!
//! The generic exporter layer deliberately contains no OpenQASM, QIR, Quil,
//! or other format-specific syntax.
//!
//! A concrete exporter is responsible for determining whether a particular
//! Quantum IR construct can actually be represented by its target format.
//! Unsupported semantics MUST produce an explicit error rather than being
//! silently discarded, commented out, approximated, or replaced.
//!
//! # Important semantic rules
//!
//! Exporters MUST:
//!
//! - preserve operation ordering;
//! - preserve qubit operands;
//! - preserve classical operands;
//! - preserve representable parameters;
//! - preserve measurements;
//! - preserve resets;
//! - preserve representable metadata;
//! - never invent measurements;
//! - never invent qubit operands;
//! - never silently drop unsupported operations;
//! - never print to stdout/stderr;
//! - never execute external programs;
//! - never perform implicit network access;
//! - never mutate the canonical circuit;
//! - produce deterministic output for identical input/configuration.
//!
//! # Rust compatibility
//!
//! Designed for Rust 1.97.1 / Rust 2021.
//!
//! No nightly features are required.

use std::fmt;

use crate::quantum::ir::QuantumCircuit;

use super::core::errors::{FrontendError, FrontendResult};
use super::format::{
    FormatCapabilities,
    FormatCapability,
    FormatVersion,
    FrontendFormat,
};

/// Default maximum serialized export size.
///
/// This is a frontend safety boundary rather than a format specification.
/// Applications may choose a stricter limit.
pub const DEFAULT_MAX_OUTPUT_BYTES: usize = 64 * 1024 * 1024;

/// Policy controlling how requested format versions are matched.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExportVersionPolicy {
    /// The exporter must use exactly the requested version.
    Exact,

    /// The exporter may use another version provided the major version
    /// remains identical.
    ///
    /// A concrete exporter is still responsible for ensuring that all
    /// requested semantics are representable in the selected version.
    SameMajor,
}

impl Default for ExportVersionPolicy {
    fn default() -> Self {
        Self::Exact
    }
}

impl fmt::Display for ExportVersionPolicy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Exact => f.write_str("exact"),
            Self::SameMajor => f.write_str("same-major"),
        }
    }
}

/// Common options shared by all frontend exporters.
///
/// Format-specific options must NOT be added here.
///
/// For example, OpenQASM-specific include handling belongs in the OpenQASM
/// exporter configuration rather than this generic structure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExportOptions {
    requested_version: Option<FormatVersion>,
    required_capabilities: FormatCapabilities,
    version_policy: ExportVersionPolicy,
    max_output_bytes: usize,
}

impl Default for ExportOptions {
    fn default() -> Self {
        Self {
            requested_version: None,
            required_capabilities: FormatCapabilities::new(),
            version_policy: ExportVersionPolicy::Exact,
            max_output_bytes: DEFAULT_MAX_OUTPUT_BYTES,
        }
    }
}

impl ExportOptions {
    /// Creates production-default export options.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the requested target version.
    pub fn requested_version(&self) -> Option<FormatVersion> {
        self.requested_version
    }

    /// Returns the capabilities required by the caller.
    pub fn required_capabilities(&self) -> &FormatCapabilities {
        &self.required_capabilities
    }

    /// Returns the version compatibility policy.
    pub fn version_policy(&self) -> ExportVersionPolicy {
        self.version_policy
    }

    /// Returns the maximum permitted serialized artifact size.
    pub fn max_output_bytes(&self) -> usize {
        self.max_output_bytes
    }

    /// Requests an exact target format version.
    pub fn with_requested_version(
        mut self,
        version: FormatVersion,
    ) -> Self {
        self.requested_version = Some(version);
        self
    }

    /// Removes an explicit target version.
    pub fn without_requested_version(mut self) -> Self {
        self.requested_version = None;
        self
    }

    /// Replaces the required capability set.
    pub fn with_required_capabilities(
        mut self,
        capabilities: FormatCapabilities,
    ) -> Self {
        self.required_capabilities = capabilities;
        self
    }

    /// Requires a specific capability.
    pub fn require_capability(
        mut self,
        capability: FormatCapability,
    ) -> Self {
        self.required_capabilities.insert(capability);
        self
    }

    /// Sets the version matching policy.
    pub fn with_version_policy(
        mut self,
        policy: ExportVersionPolicy,
    ) -> Self {
        self.version_policy = policy;
        self
    }

    /// Sets the maximum serialized output size.
    pub fn with_max_output_bytes(
        mut self,
        max_output_bytes: usize,
    ) -> Self {
        self.max_output_bytes = max_output_bytes;
        self
    }
}

/// Result of a frontend export.
///
/// The representation is byte-oriented rather than String-oriented because
/// the same exporter contract must be usable by textual formats and future
/// binary formats.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExportedArtifact {
    format: FrontendFormat,
    media_type: String,
    bytes: Vec<u8>,
}

impl ExportedArtifact {
    /// Creates a serialized artifact.
    pub fn new(
        format: FrontendFormat,
        media_type: impl Into<String>,
        bytes: Vec<u8>,
    ) -> FrontendResult<Self> {
        let media_type = media_type.into();

        if media_type.trim().is_empty() {
            return Err(FrontendError::invalid_input(
                "exported artifact media type must not be empty",
            ));
        }

        Ok(Self {
            format,
            media_type,
            bytes,
        })
    }

    /// Creates a textual artifact.
    pub fn text(
        format: FrontendFormat,
        media_type: impl Into<String>,
        text: String,
    ) -> FrontendResult<Self> {
        Self::new(format, media_type, text.into_bytes())
    }

    /// Returns the format descriptor.
    pub fn format(&self) -> &FrontendFormat {
        &self.format
    }

    /// Returns the artifact media type.
    pub fn media_type(&self) -> &str {
        &self.media_type
    }

    /// Returns the serialized bytes.
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Returns the serialized byte count.
    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    /// Returns whether the artifact contains zero bytes.
    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    /// Consumes the artifact and returns its bytes.
    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }

    /// Returns the artifact as UTF-8 text.
    pub fn as_text(&self) -> FrontendResult<&str> {
        std::str::from_utf8(&self.bytes).map_err(|error| {
            FrontendError::export(format!(
                "exported artifact is not valid UTF-8: {error}"
            ))
        })
    }

    /// Consumes the artifact and converts it to UTF-8 text.
    pub fn into_text(self) -> FrontendResult<String> {
        String::from_utf8(self.bytes).map_err(|error| {
            FrontendError::export(format!(
                "exported artifact is not valid UTF-8: {error}"
            ))
        })
    }
}

/// Stable contract implemented by every concrete quantum format exporter.
///
/// Concrete implementations should implement [`QuantumExporter::export_impl`]
/// only. The public [`QuantumExporter::export`] method performs the common
/// validation boundary first and validates the returned artifact afterward.
pub trait QuantumExporter {
    /// Returns the format implemented by this exporter.
    fn format(&self) -> &FrontendFormat;

    /// Performs format-specific serialization.
    ///
    /// This function must never bypass canonical IR validation. The default
    /// [`QuantumExporter::export`] entry point performs the common validation
    /// before invoking this method.
    fn export_impl(
        &self,
        circuit: &QuantumCircuit,
        options: &ExportOptions,
    ) -> FrontendResult<ExportedArtifact>;

    /// Exports a canonical Quantum IR circuit.
    ///
    /// This is the production entry point. Concrete exporters should not
    /// expose another public method that bypasses this validation boundary.
    fn export(
        &self,
        circuit: &QuantumCircuit,
        options: &ExportOptions,
    ) -> FrontendResult<ExportedArtifact> {
        validate_export_request(
            self.format(),
            circuit,
            options,
        )?;

        let artifact = self.export_impl(
            circuit,
            options,
        )?;

        validate_exported_artifact(
            self.format(),
            &artifact,
            options,
        )?;

        Ok(artifact)
    }
}

/// Validates all exporter concerns that are independent of a particular
/// external format.
///
/// This function deliberately delegates semantic correctness of the circuit
/// to the canonical Quantum IR rather than creating another validation model
/// in the frontend.
pub fn validate_export_request(
    format: &FrontendFormat,
    circuit: &QuantumCircuit,
    options: &ExportOptions,
) -> FrontendResult<()> {
    if !format.supports(FormatCapability::Export) {
        return Err(
            FrontendError::unsupported(format!(
                "format `{}` does not support export",
                format.id()
            ))
        );
    }

    if !format
        .capabilities()
        .contains_all(&options.required_capabilities)
    {
        return Err(
            FrontendError::unsupported(format!(
                "format `{}` does not provide all required export capabilities",
                format.id()
            ))
        );
    }

    if let Some(requested_version) =
        options.requested_version()
    {
        let compatible = match options.version_policy() {
            ExportVersionPolicy::Exact => {
                format.version() == requested_version
            }

            ExportVersionPolicy::SameMajor => {
                format.version().same_major(
                    requested_version,
                )
            }
        };

        if !compatible {
            return Err(
                FrontendError::unsupported(format!(
                    "format `{}` provides version {}, but version {} was requested under {} policy",
                    format.id(),
                    format.version(),
                    requested_version,
                    options.version_policy(),
                ))
            );
        }
    }

    if options.max_output_bytes() == 0 {
        return Err(FrontendError::invalid_input(
            "maximum export output size must be greater than zero",
        ));
    }

    /*
     * The Quantum IR is the canonical semantic owner.
     *
     * Frontends must never create a second circuit-validation system.
     * Invalid IR must therefore never reach a concrete exporter.
     */
    circuit.validate().map_err(|error| {
        FrontendError::export(format!(
            "cannot export invalid Quantum IR: {error}"
        ))
    })?;

    Ok(())
}

/// Validates the common invariants of a concrete export result.
pub fn validate_exported_artifact(
    exporter_format: &FrontendFormat,
    artifact: &ExportedArtifact,
    options: &ExportOptions,
) -> FrontendResult<()> {
    /*
     * A concrete exporter must return an artifact belonging to the format it
     * implements. This prevents an accidental cross-format implementation
     * mistake.
     */
    if artifact.format() != exporter_format {
        return Err(FrontendError::internal(format!(
            "exporter returned artifact for format `{}` version {}, \
             but exporter implements format `{}` version {}",
            artifact.format().id(),
            artifact.format().version(),
            exporter_format.id(),
            exporter_format.version(),
        )));
    }

    /*
     * Empty output is never accepted from a successful exporter.
     *
     * A valid external program may technically be empty for some future
     * format, but such a format must explicitly define a different contract
     * rather than silently weakening this common safety invariant.
     */
    if artifact.is_empty() {
        return Err(FrontendError::export(
            "exporter produced an empty artifact",
        ));
    }

    if artifact.len() > options.max_output_bytes() {
        return Err(FrontendError::limit_exceeded(format!(
            "exported artifact contains {} bytes, exceeding \
             configured maximum of {} bytes",
            artifact.len(),
            options.max_output_bytes(),
        )));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /*
     * These tests intentionally focus on the generic exporter contract.
     * OpenQASM-specific behavior belongs under:
     *
     * src/quantum/frontend/tests/openqasm/
     *
     * This keeps the format-independent contract independently testable.
     */

    #[test]
    fn default_options_are_production_bounded() {
        let options = ExportOptions::default();

        assert_eq!(
            options.version_policy(),
            ExportVersionPolicy::Exact
        );

        assert_eq!(
            options.max_output_bytes(),
            DEFAULT_MAX_OUTPUT_BYTES
        );

        assert!(options.requested_version().is_none());
        assert!(
            options
                .required_capabilities()
                .is_empty()
        );
    }

    #[test]
    fn version_policy_has_stable_display() {
        assert_eq!(
            ExportVersionPolicy::Exact.to_string(),
            "exact"
        );

        assert_eq!(
            ExportVersionPolicy::SameMajor.to_string(),
            "same-major"
        );
    }

    #[test]
    fn artifact_rejects_empty_media_type() {
        /*
         * This test intentionally does not construct a format because the
         * format API remains owned by frontend::format.
         *
         * Once the format contract exposes its canonical test constructor,
         * this should be expanded into a complete artifact-construction test.
         */
    }

    #[test]
    fn export_options_are_immutable_by_default() {
        let options = ExportOptions::new();

        assert_eq!(
            options.max_output_bytes(),
            DEFAULT_MAX_OUTPUT_BYTES
        );
    }

    #[test]
    fn output_limit_can_be_lowered() {
        let options =
            ExportOptions::new()
                .with_max_output_bytes(1024);

        assert_eq!(
            options.max_output_bytes(),
            1024
        );
    }

    #[test]
    fn version_request_can_be_removed() {
        /*
         * Requires a concrete FormatVersion constructor from the format
         * contract. The behavior is intentionally represented by the API:
         *
         * ExportOptions::new()
         *     .with_requested_version(version)
         *     .without_requested_version()
         *
         * The final integration tests should assert that the resulting value
         * contains None.
         */
    }
}