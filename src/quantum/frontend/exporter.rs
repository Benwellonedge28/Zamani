//! Format-independent quantum frontend exporter.
//!
//! This module is the stable export boundary between Zamani's canonical
//! Quantum IR and external quantum representations such as OpenQASM, QIR,
//! Quil, and future formats.
//!
//! # Architectural boundary
//!
//! ```text
//! Canonical Quantum IR
//!        │
//!        ▼
//! frontend::exporter
//!        │
//!        ├── request validation
//!        ├── capability/version policy
//!        ├── canonical IR validation
//!        ├── format-specific serialization
//!        └── artifact validation / output bounds
//!        │
//!        ▼
//! concrete format exporter
//! ```
//!
//! The generic layer deliberately contains no OpenQASM, QIR, Quil, hardware,
//! filesystem, network, process, or execution logic.
//!
//! A concrete exporter is responsible for deciding whether every canonical IR
//! construct it receives can be represented by its target format. Unsupported
//! semantics MUST result in an explicit error. They MUST NOT be silently
//! discarded, approximated, reordered, or replaced.
//!
//! # Production invariants
//!
//! The public [`QuantumExporter::export`] boundary guarantees that:
//!
//! - the target format advertises export capability;
//! - required capabilities are present;
//! - requested-version policy is satisfied;
//! - the caller supplied a positive output limit;
//! - canonical Quantum IR validation succeeds before serialization;
//! - the returned artifact belongs to the exporter that produced it;
//! - successful exports are non-empty;
//! - serialized output does not exceed the configured bound;
//! - no generic exporter operation mutates the canonical circuit.
//!
//! Concrete exporters remain responsible for deterministic serialization and
//! for format-specific representability checks.
//!
//! # Security boundary
//!
//! Exporting is compiler work on canonical IR, but the IR may originate from
//! untrusted source input or deserialization. This module therefore treats the
//! circuit and exporter options as untrusted data at the API boundary.
//!
//! The generic exporter MUST NOT:
//!
//! - perform filesystem I/O;
//! - perform network I/O;
//! - spawn processes;
//! - access hardware/QPUs;
//! - execute source-level directives;
//! - mutate the supplied circuit;
//! - depend on hash-map iteration for observable output;
//! - expose internal panic paths for malformed inputs.
//!
//! # Version policy
//!
//! `Exact` requires the exporter descriptor to match the requested revision.
//! `SameMajor` accepts a configured exporter whose revision has the requested
//! major version. `SameMajor` is intentionally a compatibility gate, not a
//! request to silently rewrite the concrete exporter's configured version.
//! The concrete exporter remains responsible for selecting and emitting its
//! actual revision, which is returned in [`ExportedArtifact::format`].
//!
//! # Integration contract
//!
//! `exporter.rs` is intentionally downstream of `format.rs` and the canonical
//! Quantum IR and upstream of every concrete exporter:
//!
//! ```text
//! frontend::format ─────────────┐
//!                               ▼
//!                         ExportOptions
//!                               │
//! QuantumCircuit ───────────────┼──► QuantumExporter
//!                               │          │
//!                               │          ▼
//!                               │   ExportedArtifact
//!                               │
//!                               └──► validation / bounds
//! ```
//!
//! Concrete exporters such as OpenQASM implement only
//! [`QuantumExporter::export_impl`]. Their existing public convenience methods
//! should call the trait's [`QuantumExporter::export`] method rather than
//! creating a second generic validation path.
//!
//! # Rust compatibility
//!
//! Rust 2021 / Rust 1.97.1.
//! No nightly features are required.

use std::fmt;

use crate::quantum::ir::QuantumCircuit;

use super::core::errors::{
    FrontendError,
    FrontendErrorCode,
    FrontendErrorKind,
    FrontendResult,
};
use super::core::limits::{
    FrontendLimitKind,
    FrontendLimitViolation,
};
use super::format::{
    FormatCapabilities,
    FormatCapability,
    FormatVersion,
    FrontendFormat,
};

/// Default maximum serialized export size.
///
/// This mirrors the frontend's production output boundary. Applications may
/// choose a stricter value for a particular compilation request.
pub const DEFAULT_MAX_OUTPUT_BYTES: usize = 64 * 1024 * 1024;

/// Maximum media-type metadata retained by one exported artifact.
pub const MAX_MEDIA_TYPE_BYTES: usize = 256;

/// Policy controlling how a requested format version is matched against the
/// concrete exporter's configured revision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExportVersionPolicy {
    /// The exporter must use exactly the requested revision.
    Exact,

    /// The exporter revision must have the same major version as the request.
    ///
    /// This is a compatibility check only. It does not cause a concrete
    /// exporter to switch versions. The emitted revision is always reported by
    /// the returned artifact's [`FrontendFormat`].
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
/// Format-specific options MUST remain in the concrete format implementation.
/// For example, OpenQASM include resolution belongs to the OpenQASM layer,
/// not this generic contract.
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
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the requested target revision, if one was supplied.
    #[must_use]
    pub fn requested_version(&self) -> Option<FormatVersion> {
        self.requested_version
    }

    /// Returns the capabilities required by the caller.
    #[must_use]
    pub fn required_capabilities(&self) -> &FormatCapabilities {
        &self.required_capabilities
    }

    /// Returns the version compatibility policy.
    #[must_use]
    pub fn version_policy(&self) -> ExportVersionPolicy {
        self.version_policy
    }

    /// Returns the maximum permitted serialized artifact size in bytes.
    #[must_use]
    pub fn max_output_bytes(&self) -> usize {
        self.max_output_bytes
    }

    /// Requests an exact or compatible target revision according to
    /// [`ExportVersionPolicy`].
    #[must_use]
    pub fn with_requested_version(
        mut self,
        version: FormatVersion,
    ) -> Self {
        self.requested_version = Some(version);
        self
    }

    /// Removes an explicit target revision.
    #[must_use]
    pub fn without_requested_version(mut self) -> Self {
        self.requested_version = None;
        self
    }

    /// Replaces the required capability set.
    #[must_use]
    pub fn with_required_capabilities(
        mut self,
        capabilities: FormatCapabilities,
    ) -> Self {
        self.required_capabilities = capabilities;
        self
    }

    /// Requires one capability.
    ///
    /// The frontend capability vocabulary is bounded by
    /// `MAX_FORMAT_CAPABILITIES`, and the currently defined enum contains far
    /// fewer entries. Consequently insertion cannot fail for a
    /// `FormatCapability` value under the current contract. The result is
    /// nevertheless checked so this method does not discard a `Result`.
    #[must_use]
    pub fn require_capability(
        mut self,
        capability: FormatCapability,
    ) -> Self {
        if let Err(error) =
            self.required_capabilities.insert(capability)
        {
            debug_assert!(
                false,
                "FormatCapability insertion unexpectedly failed: {error}"
            );
        }

        self
    }

    /// Sets the version matching policy.
    #[must_use]
    pub fn with_version_policy(
        mut self,
        policy: ExportVersionPolicy,
    ) -> Self {
        self.version_policy = policy;
        self
    }

    /// Sets the maximum serialized output size.
    #[must_use]
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
/// The representation is byte-oriented so the generic contract can support
/// textual and future binary formats without introducing a second API.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExportedArtifact {
    format: FrontendFormat,
    media_type: String,
    bytes: Vec<u8>,
}

impl ExportedArtifact {
    /// Creates a serialized artifact after validating its generic metadata.
    pub fn new(
        format: FrontendFormat,
        media_type: impl Into<String>,
        bytes: Vec<u8>,
    ) -> FrontendResult<Self> {
        let media_type = media_type.into();

        validate_media_type(&media_type)?;

        Ok(Self {
            format,
            media_type,
            bytes,
        })
    }

    /// Creates a textual UTF-8 artifact.
    pub fn text(
        format: FrontendFormat,
        media_type: impl Into<String>,
        text: String,
    ) -> FrontendResult<Self> {
        Self::new(
            format,
            media_type,
            text.into_bytes(),
        )
    }

    /// Returns the format descriptor carried by this artifact.
    #[must_use]
    pub fn format(&self) -> &FrontendFormat {
        &self.format
    }

    /// Returns the artifact media type.
    #[must_use]
    pub fn media_type(&self) -> &str {
        &self.media_type
    }

    /// Returns the serialized bytes.
    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Returns the serialized byte count.
    #[must_use]
    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    /// Returns whether the artifact contains no bytes.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    /// Consumes the artifact and returns its bytes.
    #[must_use]
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
pub trait QuantumExporter {
    /// Returns the format implemented by this exporter.
    fn format(&self) -> &FrontendFormat;

    /// Performs format-specific serialization.
    ///
    /// Implementations MUST NOT mutate `circuit`, perform I/O, execute source
    /// constructs, or silently drop unsupported semantics.
    fn export_impl(
        &self,
        circuit: &QuantumCircuit,
        options: &ExportOptions,
    ) -> FrontendResult<ExportedArtifact>;

    /// Exports a canonical Quantum IR circuit through the production boundary.
    ///
    /// This method is deliberately provided as the single generic entry point:
    /// request validation happens before `export_impl`, and returned-artifact
    /// validation happens afterward.
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

        let artifact =
            self.export_impl(circuit, options)?;

        validate_exported_artifact(
            self.format(),
            &artifact,
            options,
        )?;

        Ok(artifact)
    }
}

/// Validates format-independent exporter request invariants.
///
/// Canonical Quantum IR validation is intentionally delegated to
/// `QuantumCircuit::validate()`. The frontend must not maintain a second
/// semantic model for the IR.
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
            .context(
                "format",
                format.id().as_str(),
            )
            .context(
                "stage",
                "export-request",
            ),
        );
    }

    let missing = format
        .capabilities()
        .missing_from(
            &options.required_capabilities,
        );

    if !missing.is_empty() {
        let names = missing
            .iter()
            .map(|capability| capability.as_str())
            .collect::<Vec<_>>()
            .join(", ");

        return Err(
            FrontendError::unsupported(format!(
                "format `{}` does not provide required export capabilities: {}",
                format.id(),
                names,
            ))
            .context(
                "format",
                format.id().as_str(),
            )
            .context(
                "stage",
                "export-request",
            ),
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
                format
                    .version()
                    .same_major(requested_version)
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
                .context(
                    "format",
                    format.id().as_str(),
                )
                .context(
                    "stage",
                    "export-version",
                ),
            );
        }
    }

    if options.max_output_bytes() == 0 {
        return Err(
            FrontendError::invalid_input(
                "maximum export output size must be greater than zero",
            )
            .context(
                "stage",
                "export-request",
            ),
        );
    }

    circuit.validate().map_err(|error| {
        FrontendError::with_code(
            FrontendErrorKind::Export,
            FrontendErrorCode::EXPORT,
            format!(
                "cannot export invalid Quantum IR: {error}"
            ),
        )
        .context(
            "format",
            format.id().as_str(),
        )
        .context(
            "stage",
            "ir-validation",
        )
    })?;

    Ok(())
}

/// Validates generic invariants of a concrete export result.
pub fn validate_exported_artifact(
    exporter_format: &FrontendFormat,
    artifact: &ExportedArtifact,
    options: &ExportOptions,
) -> FrontendResult<()> {
    if artifact.format() != exporter_format {
        return Err(
            FrontendError::internal(format!(
                "exporter returned artifact for format `{}` version {}, \
                 but exporter implements format `{}` version {}",
                artifact.format().id(),
                artifact.format().version(),
                exporter_format.id(),
                exporter_format.version(),
            ))
            .context(
                "stage",
                "export-artifact-validation",
            ),
        );
    }

    if artifact.is_empty() {
        return Err(
            FrontendError::export(
                "exporter produced an empty artifact",
            )
            .context(
                "format",
                exporter_format.id().as_str(),
            )
            .context(
                "stage",
                "export-artifact-validation",
            ),
        );
    }

    if artifact.len()
        > options.max_output_bytes()
    {
        let actual =
            u64::try_from(artifact.len())
                .unwrap_or(u64::MAX);

        let maximum =
            u64::try_from(
                options.max_output_bytes(),
            )
            .unwrap_or(u64::MAX);

        return Err(
            FrontendError::limit_exceeded(
                FrontendLimitViolation::new(
                    FrontendLimitKind::OutputBytes,
                    actual,
                    maximum,
                ),
            )
            .context(
                "format",
                exporter_format.id().as_str(),
            )
            .context(
                "stage",
                "export-artifact-validation",
            ),
        );
    }

    Ok(())
}

/// Validates the generic media-type metadata attached to an artifact.
fn validate_media_type(
    media_type: &str,
) -> FrontendResult<()> {
    if media_type.trim().is_empty() {
        return Err(
            FrontendError::invalid_input(
                "exported artifact media type must not be empty",
            )
        );
    }

    if media_type.len()
        > MAX_MEDIA_TYPE_BYTES
    {
        return Err(
            FrontendError::invalid_input(
                format!(
                    "exported artifact media type exceeds {} bytes",
                    MAX_MEDIA_TYPE_BYTES
                ),
            )
        );
    }

    if media_type
        .chars()
        .any(char::is_control)
    {
        return Err(
            FrontendError::invalid_input(
                "exported artifact media type must not contain control characters",
            )
        );
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::quantum::ir::QuantumCircuit;

    fn test_format(
        id: &str,
        version: FormatVersion,
        capabilities: &[FormatCapability],
    ) -> FrontendFormat {
        let id =
            super::super::format::FormatId::new(id)
                .expect(
                    "test format id must be valid",
                );

        let capabilities =
            FormatCapabilities::from_iter(
                capabilities.iter().copied(),
            )
            .expect(
                "test capabilities must be valid",
            );

        FrontendFormat::new(
            id,
            version,
            capabilities,
        )
    }

    #[test]
    fn default_options_are_bounded_and_exact() {
        let options =
            ExportOptions::default();

        assert_eq!(
            options.version_policy(),
            ExportVersionPolicy::Exact
        );

        assert_eq!(
            options.max_output_bytes(),
            DEFAULT_MAX_OUTPUT_BYTES
        );

        assert!(
            options
                .requested_version()
                .is_none()
        );

        assert!(
            options
                .required_capabilities()
                .is_empty()
        );
    }

    #[test]
    fn options_builder_preserves_requested_contract() {
        let version =
            FormatVersion::major_minor(3, 1);

        let options =
            ExportOptions::new()
                .with_requested_version(
                    version,
                )
                .with_version_policy(
                    ExportVersionPolicy::Exact,
                )
                .require_capability(
                    FormatCapability::Export,
                )
                .with_max_output_bytes(4096);

        assert_eq!(
            options.requested_version(),
            Some(version)
        );

        assert_eq!(
            options.version_policy(),
            ExportVersionPolicy::Exact
        );

        assert!(
            options
                .required_capabilities()
                .supports(
                    FormatCapability::Export
                )
        );

        assert_eq!(
            options.max_output_bytes(),
            4096
        );
    }

    #[test]
    fn artifact_rejects_invalid_media_type() {
        let format = test_format(
            "test",
            FormatVersion::major_minor(1, 0),
            &[FormatCapability::Export],
        );

        assert!(
            ExportedArtifact::new(
                format.clone(),
                "",
                vec![1],
            )
            .is_err()
        );

        assert!(
            ExportedArtifact::new(
                format,
                "text/x-test\ninvalid",
                vec![1],
            )
            .is_err()
        );
    }

    #[test]
    fn artifact_supports_binary_and_text_access() {
        let format = test_format(
            "test",
            FormatVersion::major_minor(1, 0),
            &[FormatCapability::Export],
        );

        let artifact =
            ExportedArtifact::new(
                format,
                "application/octet-stream",
                vec![0, 1, 2],
            )
            .expect(
                "valid artifact should construct",
            );

        assert_eq!(
            artifact.len(),
            3
        );

        assert_eq!(
            artifact.bytes(),
            &[0, 1, 2]
        );

        assert!(
            artifact.as_text().is_err()
        );
    }

    #[test]
    fn text_artifact_round_trips_utf8() {
        let format = test_format(
            "test",
            FormatVersion::major_minor(1, 0),
            &[FormatCapability::Export],
        );

        let artifact =
            ExportedArtifact::text(
                format,
                "text/x-test",
                "OPENQASM-like text"
                    .to_owned(),
            )
            .expect(
                "valid text artifact should construct",
            );

        assert_eq!(
            artifact
                .as_text()
                .expect("UTF-8 must decode"),
            "OPENQASM-like text"
        );

        assert_eq!(
            artifact
                .clone()
                .into_text()
                .expect("UTF-8 must decode"),
            "OPENQASM-like text"
        );
    }

    #[test]
    fn request_rejects_missing_export_capability() {
        let format = test_format(
            "test",
            FormatVersion::major_minor(1, 0),
            &[],
        );

        let circuit =
            QuantumCircuit::new(1, 0)
                .expect(
                    "test circuit should construct",
                );

        let error =
            validate_export_request(
                &format,
                &circuit,
                &ExportOptions::default(),
            )
            .expect_err(
                "format without export must fail",
            );

        assert_eq!(
            error.kind(),
            FrontendErrorKind::Unsupported
        );
    }

    #[test]
    fn request_rejects_zero_output_limit() {
        let format = test_format(
            "test",
            FormatVersion::major_minor(1, 0),
            &[FormatCapability::Export],
        );

        let circuit =
            QuantumCircuit::new(1, 0)
                .expect(
                    "test circuit should construct",
                );

        let options =
            ExportOptions::default()
                .with_max_output_bytes(0);

        let error =
            validate_export_request(
                &format,
                &circuit,
                &options,
            )
            .expect_err(
                "zero output limit must fail",
            );

        assert_eq!(
            error.kind(),
            FrontendErrorKind::InvalidInput
        );
    }

    #[test]
    fn exact_version_policy_rejects_mismatch() {
        let format = test_format(
            "test",
            FormatVersion::major_minor(1, 1),
            &[FormatCapability::Export],
        );

        let circuit =
            QuantumCircuit::new(1, 0)
                .expect(
                    "test circuit should construct",
                );

        let options =
            ExportOptions::default()
                .with_requested_version(
                    FormatVersion::major_minor(
                        1,
                        0,
                    ),
                );

        let error =
            validate_export_request(
                &format,
                &circuit,
                &options,
            )
            .expect_err(
                "exact mismatch must fail",
            );

        assert_eq!(
            error.kind(),
            FrontendErrorKind::Unsupported
        );
    }

    #[test]
    fn same_major_policy_accepts_same_major_revision() {
        let format = test_format(
            "test",
            FormatVersion::major_minor(1, 1),
            &[FormatCapability::Export],
        );

        let circuit =
            QuantumCircuit::new(1, 0)
                .expect(
                    "test circuit should construct",
                );

        let options =
            ExportOptions::default()
                .with_requested_version(
                    FormatVersion::major_minor(
                        1,
                        0,
                    ),
                )
                .with_version_policy(
                    ExportVersionPolicy::SameMajor,
                );

        validate_export_request(
            &format,
            &circuit,
            &options,
        )
        .expect(
            "same-major request should pass",
        );
    }

    #[test]
    fn artifact_limit_uses_canonical_frontend_limit_error() {
        let format = test_format(
            "test",
            FormatVersion::major_minor(1, 0),
            &[FormatCapability::Export],
        );

        let artifact =
            ExportedArtifact::new(
                format.clone(),
                "application/octet-stream",
                vec![0; 8],
            )
            .expect(
                "artifact should construct",
            );

        let options =
            ExportOptions::default()
                .with_max_output_bytes(4);

        let error =
            validate_exported_artifact(
                &format,
                &artifact,
                &options,
            )
            .expect_err(
                "oversized artifact must fail",
            );

        assert!(
            error.is_limit_exceeded()
        );

        assert_eq!(
            error
                .limit_violation()
                .expect(
                    "limit information must exist",
                )
                .kind(),
            FrontendLimitKind::OutputBytes
        );
    }

    #[test]
    fn valid_ir_passes_generic_export_boundary() {
        let format = test_format(
            "test",
            FormatVersion::major_minor(1, 0),
            &[FormatCapability::Export],
        );

        let circuit =
            QuantumCircuit::new(1, 0)
                .expect(
                    "test circuit should construct",
                );

        validate_export_request(
            &format,
            &circuit,
            &ExportOptions::default(),
        )
        .expect(
            "valid IR must pass the generic boundary",
        );
    }
}