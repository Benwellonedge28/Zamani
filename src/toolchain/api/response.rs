
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Toolchain — API Response Contracts
//!
//! Path: src/toolchain/api/response.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Defines the canonical, serializable response contract for the Zamani
//! Toolchain API.
//!
//! Every toolchain client uses this response boundary:
//!
//! - Command-line applications.
//! - IDEs and editors.
//! - Human developers.
//! - Automation services.
//! - AI agents.
//! - Enterprise integrations.
//! - Third-party developer applications.
//! - Local and remote toolchain clients.
//!
//! ## OWNERSHIP
//!
//! This module owns:
//!
//! - Response envelopes.
//! - Request-response correlation.
//! - Operation lifecycle status.
//! - Structured response errors.
//! - Response diagnostics.
//! - Source-location snapshots.
//! - Compiler artifact references.
//! - Dependency-change summaries.
//! - Execution summaries.
//! - Extensible machine-readable response data.
//! - Response structural validation.
//!
//! This module does not own:
//!
//! - Request definitions.
//! - Compiler diagnostics or semantic diagnostics.
//! - Compiler artifact construction or storage.
//! - Artifact integrity verification.
//! - Dependency resolution.
//! - Target feasibility.
//! - Hardware discovery or allocation.
//! - Runtime execution.
//! - Deployment.
//! - Diagnostic rendering.
//! - Logging, retry, or recovery policy.
//!
//! ## AUTHORITATIVE INTEGRATIONS
//!
//! Request contracts:
//!     crate::toolchain::api::request
//!
//! Error contracts:
//!     crate::toolchain::errors
//!
//! API version contracts:
//!     crate::toolchain::version
//!
//! Artifact contracts:
//!     crate::compiler::artifacts::artifact
//!
//! Source semantics:
//!     grammar/specification/semantic-model.md
//!
//! Compilation:
//!     grammar/specification/compilation-model.md
//!
//! Execution:
//!     grammar/specification/execution-model.md
//!
//! Portability:
//!     grammar/specification/poco-reaf.md
//!
//! Scalability:
//!     grammar/specification/scalability-model.md
//!
//! ## RESPONSE SEMANTICS
//!
//! Request acceptance does not mean that an operation completed.
//!
//! A successful operation must be explicitly reported as Succeeded.
//!
//! Partial completion, cancellation, rejection and failure are distinct.
//!
//! An operation that failed must not be represented as successful.
//!
//! A response must not claim that an artifact is verified, portable,
//! executable or deployable merely because it has an artifact reference.
//!
//! ## POCO-REAF
//!
//! One logical request may return multiple artifacts and multiple
//! target-specific realizations.
//!
//! Different realizations retain their individual artifact identities,
//! formats and target references.
//!
//! The response contract does not define a closed list of computational
//! domains, architectures, providers or hardware types.
//!
//! ## SCALABILITY
//!
//! No arbitrary maximum is imposed on:
//!
//! - Artifacts per response.
//! - Diagnostics per response.
//! - Dependency changes.
//! - Computing domains.
//! - Targets.
//! - Providers.
//! - Hardware resources.
//! - Distributed participants.
//! - Tensor dimensions.
//!
//! Collections grow subject to available memory and address space.
//!
//! Large binary artifacts must be represented by references to the
//! canonical artifact storage system, not copied into response JSON.
//!
//! Operational response-size limits, if needed, belong to configurable
//! transport and service policies.
//!
//! Literal infinite storage or execution is not promised.
//!
//! ## SECURITY
//!
//! Response objects must not expose:
//!
//! - Passwords.
//! - Access tokens.
//! - Private keys.
//! - Raw authentication credentials.
//! - Unredacted secret configuration.
//!
//! Error causes are summarized without serializing arbitrary underlying
//! error objects.
//!
//! Deserialized responses must pass validate() before being trusted.
//!
//! ## COMPATIBILITY
//!
//! The response API version is inherited from the originating request.
//!
//! This module does not create a second API version authority.
//!
//! Unknown extension fields are tolerated during deserialization.
//! Unknown enum values are represented by explicit Unknown variants
//! where the relevant enum supports them.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use std::collections::BTreeMap;
use std::fmt;
use std::time::Duration;

use super::request::{
    RequestEnvelope,
    RequestValidationError,
};

use crate::compiler::artifacts::artifact::Artifact;

use crate::toolchain::errors::{
    ToolchainError,
    ToolchainErrorKind,
    ToolchainResult,
};

use crate::toolchain::version::ToolchainApiVersion;

// ============================================================================
// RESPONSE VALIDATION ERRORS
// ============================================================================

/// Structural response-validation failure.
///
/// This error describes an invalid response representation.
/// It does not replace the canonical ToolchainError classification.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct ResponseValidationError {
    field: String,
    message: String,
}

impl ResponseValidationError {
    fn new(
        field: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            field: field.into(),
            message: message.into(),
        }
    }

    /// Return the invalid field path.
    #[must_use]
    pub fn field(&self) -> &str {
        &self.field
    }

    /// Return the validation explanation.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for ResponseValidationError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            formatter,
            "invalid response field '{}': {}",
            self.field,
            self.message
        )
    }
}

impl std::error::Error for ResponseValidationError {}

/// Result of response structural validation.
pub type ResponseValidationResult<T> =
    std::result::Result<T, ResponseValidationError>;

fn validate_identifier(
    field: &str,
    value: &str,
) -> ResponseValidationResult<()> {
    if value.is_empty() || value.trim().is_empty() {
        return Err(ResponseValidationError::new(
            field,
            "identifier must not be empty",
        ));
    }

    if value != value.trim() {
        return Err(ResponseValidationError::new(
            field,
            "identifier must not contain surrounding whitespace",
        ));
    }

    if value.chars().any(char::is_control) {
        return Err(ResponseValidationError::new(
            field,
            "identifier must not contain control characters",
        ));
    }

    Ok(())
}

// ============================================================================
// OPERATION LIFECYCLE
// ============================================================================

/// Lifecycle state of a toolchain operation.
///
/// These states describe the operation, not a computational domain.
///
/// An Accepted or Running operation has not necessarily completed.
///
/// Unknown supports forward-compatible interpretation of future wire values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum ResponseStatus {
    /// The request was accepted for processing.
    Accepted,

    /// The operation is currently executing.
    Running,

    /// The operation completed successfully.
    Succeeded,

    /// The operation completed with some successful and some unsuccessful
    /// results.
    PartiallySucceeded,

    /// The operation failed.
    Failed,

    /// The operation was cancelled.
    Cancelled,

    /// The request was rejected before operation execution.
    Rejected,

    /// An unrecognized future lifecycle state.
    #[serde(other)]
    Unknown,
}

impl ResponseStatus {
    /// Whether the operation has reached a terminal state.
    #[must_use]
    pub const fn is_terminal(self) -> bool {
        matches!(
            self,
            Self::Succeeded
                | Self::PartiallySucceeded
                | Self::Failed
                | Self::Cancelled
                | Self::Rejected
        )
    }

    /// Whether the operation completed fully successfully.
    #[must_use]
    pub const fn is_success(self) -> bool {
        matches!(self, Self::Succeeded)
    }

    /// Whether the operation completed without full success.
    #[must_use]
    pub const fn is_unsuccessful(self) -> bool {
        matches!(
            self,
            Self::Failed | Self::Cancelled | Self::Rejected
        )
    }

    /// Return the canonical status identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Accepted => "accepted",
            Self::Running => "running",
            Self::Succeeded => "succeeded",
            Self::PartiallySucceeded => "partially_succeeded",
            Self::Failed => "failed",
            Self::Cancelled => "cancelled",
            Self::Rejected => "rejected",
            Self::Unknown => "unknown",
        }
    }
}

impl fmt::Display for ResponseStatus {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// STRUCTURED ERROR SNAPSHOT
// ============================================================================

/// Serializable snapshot of a canonical toolchain error.
///
/// This is a transport representation, not a second error authority.
///
/// The original causal error remains owned by ToolchainError and is
/// deliberately not serialized as an arbitrary object.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResponseError {
    /// Stable toolchain error code.
    pub code: String,

    /// Stable category identifier.
    pub category: String,

    /// Human-readable explanation.
    pub message: String,

    /// Operation associated with the failure.
    pub operation: Option<String>,

    /// Subject associated with the failure.
    pub subject: Option<String>,

    /// Ordered contextual explanations.
    #[serde(default)]
    pub context: Vec<String>,
}

impl ResponseError {
    /// Convert the canonical ToolchainError into a transport snapshot.
    ///
    /// Credentials and arbitrary causal error objects are never serialized.
    #[must_use]
    pub fn from_toolchain_error(error: &ToolchainError) -> Self {
        Self {
            code: error.code().to_owned(),
            category: error.category().to_owned(),
            message: error.message().to_owned(),
            operation: error.operation().map(str::to_owned),
            subject: error.subject().map(str::to_owned),
            context: error.context().to_vec(),
        }
    }

    /// Validate the serialized error representation.
    pub fn validate(&self) -> ResponseValidationResult<()> {
        validate_identifier("error.code", &self.code)?;
        validate_identifier("error.category", &self.category)?;

        if self.message.trim().is_empty() {
            return Err(ResponseValidationError::new(
                "error.message",
                "error message must not be empty",
            ));
        }

        if let Some(operation) = &self.operation {
            validate_identifier("error.operation", operation)?;
        }

        if let Some(subject) = &self.subject {
            validate_identifier("error.subject", subject)?;
        }

        for (index, context) in self.context.iter().enumerate() {
            if context.trim().is_empty() {
                return Err(ResponseValidationError::new(
                    format!("error.context[{index}]"),
                    "context entry must not be empty",
                ));
            }
        }

        Ok(())
    }
}

// ============================================================================
// SOURCE LOCATION
// ============================================================================

/// Source location associated with a diagnostic.
///
/// Byte offsets use a half-open interval [start, end).
///
/// Line and column numbers are one-based.
///
/// These are diagnostic snapshots. The canonical source map and AST
/// location authorities remain in the frontend.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticLocation {
    /// Source identity or source URI.
    pub source: String,

    /// Inclusive starting byte offset.
    pub start_byte: u64,

    /// Exclusive ending byte offset.
    pub end_byte: u64,

    /// One-based starting line.
    pub start_line: u64,

    /// One-based starting column.
    pub start_column: u64,

    /// One-based ending line.
    pub end_line: u64,

    /// One-based ending column.
    pub end_column: u64,
}

impl DiagnosticLocation {
    /// Construct a validated source location.
    pub fn new(
        source: impl Into<String>,
        start_byte: u64,
        end_byte: u64,
        start_line: u64,
        start_column: u64,
        end_line: u64,
        end_column: u64,
    ) -> ResponseValidationResult<Self> {
        let location = Self {
            source: source.into(),
            start_byte,
            end_byte,
            start_line,
            start_column,
            end_line,
            end_column,
        };

        location.validate()?;

        Ok(location)
    }

    /// Validate the source location.
    pub fn validate(&self) -> ResponseValidationResult<()> {
        validate_identifier("location.source", &self.source)?;

        if self.end_byte < self.start_byte {
            return Err(ResponseValidationError::new(
                "location.end_byte",
                "end byte must not precede start byte",
            ));
        }

        if self.start_line == 0 || self.end_line == 0 {
            return Err(ResponseValidationError::new(
                "location.line",
                "line numbers must be one-based",
            ));
        }

        if self.start_column == 0 || self.end_column == 0 {
            return Err(ResponseValidationError::new(
                "location.column",
                "column numbers must be one-based",
            ));
        }

        if self.end_line < self.start_line {
            return Err(ResponseValidationError::new(
                "location.end_line",
                "end line must not precede start line",
            ));
        }

        if self.end_line == self.start_line
            && self.end_column < self.start_column
        {
            return Err(ResponseValidationError::new(
                "location.end_column",
                "end column must not precede start column",
            ));
        }

        Ok(())
    }
}

// ============================================================================
// RESPONSE DIAGNOSTICS
// ============================================================================

/// Structured diagnostic transported through the Toolchain API.
///
/// Severity is an extensible identifier rather than a closed domain enum.
///
/// Common values are:
/// - error
/// - warning
/// - note
/// - help
/// - info
///
/// Diagnostic creation and semantic classification remain with the
/// frontend, compiler, backend and other authoritative subsystems.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResponseDiagnostic {
    /// Stable diagnostic code.
    pub code: String,

    /// Extensible severity identifier.
    pub severity: String,

    /// Human-readable diagnostic message.
    pub message: String,

    /// Optional source location.
    pub location: Option<DiagnosticLocation>,

    /// Additional diagnostic notes.
    #[serde(default)]
    pub notes: Vec<String>,

    /// Optional remediation guidance.
    pub help: Option<String>,

    /// Optional originating subsystem or provider.
    pub origin: Option<String>,

    /// Namespaced extension data.
    #[serde(default)]
    pub extensions: BTreeMap<String, Value>,
}

impl ResponseDiagnostic {
    /// Construct a structured diagnostic.
    pub fn new(
        code: impl Into<String>,
        severity: impl Into<String>,
        message: impl Into<String>,
    ) -> ResponseValidationResult<Self> {
        let diagnostic = Self {
            code: code.into(),
            severity: severity.into(),
            message: message.into(),
            location: None,
            notes: Vec::new(),
            help: None,
            origin: None,
            extensions: BTreeMap::new(),
        };

        diagnostic.validate()?;

        Ok(diagnostic)
    }

    /// Attach a source location.
    #[must_use]
    pub fn with_location(
        mut self,
        location: DiagnosticLocation,
    ) -> Self {
        self.location = Some(location);
        self
    }

    /// Append a diagnostic note.
    #[must_use]
    pub fn with_note(
        mut self,
        note: impl Into<String>,
    ) -> Self {
        self.notes.push(note.into());
        self
    }

    /// Attach remediation guidance.
    #[must_use]
    pub fn with_help(
        mut self,
        help: impl Into<String>,
    ) -> Self {
        self.help = Some(help.into());
        self
    }

    /// Identify the originating subsystem or provider.
    #[must_use]
    pub fn with_origin(
        mut self,
        origin: impl Into<String>,
    ) -> Self {
        self.origin = Some(origin.into());
        self
    }

    /// Attach namespaced extension data.
    ///
    /// Extensions must not redefine the meaning of canonical fields.
    #[must_use]
    pub fn with_extension(
        mut self,
        key: impl Into<String>,
        value: Value,
    ) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }

    /// Validate the diagnostic.
    pub fn validate(&self) -> ResponseValidationResult<()> {
        validate_identifier("diagnostic.code", &self.code)?;
        validate_identifier("diagnostic.severity", &self.severity)?;

        if self.message.trim().is_empty() {
            return Err(ResponseValidationError::new(
                "diagnostic.message",
                "diagnostic message must not be empty",
            ));
        }

        if let Some(location) = &self.location {
            location.validate()?;
        }

        if let Some(origin) = &self.origin {
            validate_identifier("diagnostic.origin", origin)?;
        }

        if let Some(help) = &self.help {
            if help.trim().is_empty() {
                return Err(ResponseValidationError::new(
                    "diagnostic.help",
                    "help text must not be empty when supplied",
                ));
            }
        }

        for (index, note) in self.notes.iter().enumerate() {
            if note.trim().is_empty() {
                return Err(ResponseValidationError::new(
                    format!("diagnostic.notes[{index}]"),
                    "diagnostic note must not be empty",
                ));
            }
        }

        for key in self.extensions.keys() {
            validate_identifier("diagnostic.extensions.key", key)?;
        }

        Ok(())
    }
}

// ============================================================================
// CANONICAL ARTIFACT REFERENCES
// ============================================================================

/// Storage representation of an artifact payload.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum ArtifactPayloadKind {
    /// Artifact bytes are available in the canonical artifact object.
    Inline,

    /// Artifact bytes are stored by an external content provider.
    External,

    /// A future or unrecognized storage representation.
    #[serde(other)]
    Unknown,
}

/// Integrity information available for the referenced artifact.
///
/// This deliberately distinguishes a locally computed digest from an
/// expected digest attached to an external payload.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum ArtifactDigestState {
    /// Digest was computed from the inline artifact payload.
    ComputedForInlinePayload,

    /// Digest is declared for externally stored content.
    ExpectedForExternalPayload,

    /// Integrity state is not established.
    Unknown,
}

/// Lightweight transport reference to a canonical compiler artifact.
///
/// This type does not own the artifact, its bytes, storage, or verification.
///
/// The canonical source remains:
/// crate::compiler::artifacts::artifact::Artifact
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactReference {
    /// Canonical artifact identity.
    pub id: String,

    /// Extensible computational artifact category.
    pub kind: String,

    /// Artifact representation format.
    pub format: String,

    /// Explicit target identity, if supplied by the artifact producer.
    pub target: Option<String>,

    /// Digest algorithm identifier.
    pub digest_algorithm: String,

    /// Lowercase hexadecimal content digest.
    pub digest_hex: String,

    /// Known payload size in bytes.
    pub size_bytes: Option<u64>,

    /// Payload storage representation.
    pub payload_kind: ArtifactPayloadKind,

    /// External storage provider identity, when applicable.
    pub external_store: Option<String>,

    /// External content identity, when applicable.
    pub external_content: Option<String>,

    /// Integrity information supplied by the canonical artifact model.
    pub digest_state: ArtifactDigestState,

    /// Artifact producer identity.
    pub producer: String,

    /// Optional source fingerprint.
    pub source_fingerprint: Option<String>,

    /// Transformation fingerprints in their recorded order.
    #[serde(default)]
    pub transformation_fingerprints: Vec<String>,
}

impl From<&Artifact> for ArtifactReference {
    fn from(artifact: &Artifact) -> Self {
        let descriptor = artifact.descriptor();
        let provenance = descriptor.provenance();
        let digest = artifact.digest();

        let (payload_kind, external_store, external_content, digest_state) =
            match artifact.payload().external_reference() {
                Some(reference) => (
                    ArtifactPayloadKind::External,
                    Some(reference.store().to_string()),
                    Some(reference.content().to_string()),
                    ArtifactDigestState::ExpectedForExternalPayload,
                ),

                None => (
                    ArtifactPayloadKind::Inline,
                    None,
                    None,
                    ArtifactDigestState::ComputedForInlinePayload,
                ),
            };

        Self {
            id: artifact.id().as_str().to_owned(),
            kind: artifact.kind().as_str().to_owned(),
            format: artifact.format().as_str().to_owned(),

            target: descriptor
                .target()
                .map(|target| target.as_str().to_owned()),

            digest_algorithm: digest.algorithm().as_str().to_owned(),
            digest_hex: digest.to_hex(),

            size_bytes: artifact.size_bytes(),

            payload_kind,
            external_store,
            external_content,
            digest_state,

            producer: provenance.producer().as_str().to_owned(),

            source_fingerprint: provenance
                .source_fingerprint()
                .map(ToString::to_string),

            transformation_fingerprints: provenance
                .transformations()
                .iter()
                .map(ToString::to_string)
                .collect(),
        }
    }
}

impl ArtifactReference {
    /// Validate the artifact reference representation.
    pub fn validate(&self) -> ResponseValidationResult<()> {
        validate_identifier("artifact.id", &self.id)?;
        validate_identifier("artifact.kind", &self.kind)?;
        validate_identifier("artifact.format", &self.format)?;
        validate_identifier(
            "artifact.digest_algorithm",
            &self.digest_algorithm,
        )?;
        validate_identifier("artifact.digest_hex", &self.digest_hex)?;
        validate_identifier("artifact.producer", &self.producer)?;

        if let Some(target) = &self.target {
            validate_identifier("artifact.target", target)?;
        }

        match self.payload_kind {
            ArtifactPayloadKind::Inline => {
                if self.external_store.is_some()
                    || self.external_content.is_some()
                {
                    return Err(ResponseValidationError::new(
                        "artifact.external_reference",
                        "inline payload must not contain external storage references",
                    ));
                }

                if self.digest_state
                    != ArtifactDigestState::ComputedForInlinePayload
                {
                    return Err(ResponseValidationError::new(
                        "artifact.digest_state",
                        "inline payload must identify its computed digest state",
                    ));
                }
            }

            ArtifactPayloadKind::External => {
                let store = self.external_store.as_deref().ok_or_else(|| {
                    ResponseValidationError::new(
                        "artifact.external_store",
                        "external payload requires a storage provider identity",
                    )
                })?;

                let content = self.external_content.as_deref().ok_or_else(|| {
                    ResponseValidationError::new(
                        "artifact.external_content",
                        "external payload requires a content identity",
                    )
                })?;

                validate_identifier("artifact.external_store", store)?;
                validate_identifier("artifact.external_content", content)?;

                if self.digest_state
                    != ArtifactDigestState::ExpectedForExternalPayload
                {
                    return Err(ResponseValidationError::new(
                        "artifact.digest_state",
                        "external payload must distinguish expected from verified content",
                    ));
                }
            }

            ArtifactPayloadKind::Unknown => {}
        }

        if let Some(fingerprint) = &self.source_fingerprint {
            validate_identifier(
                "artifact.source_fingerprint",
                fingerprint,
            )?;
        }

        for (index, fingerprint) in
            self.transformation_fingerprints.iter().enumerate()
        {
            validate_identifier(
                &format!("artifact.transformation_fingerprints[{index}]"),
                fingerprint,
            )?;
        }

        Ok(())
    }
}

// ============================================================================
// DEPENDENCY CHANGE REPORTING
// ============================================================================

/// Type of dependency change reported by an operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum DependencyChangeKind {
    /// A dependency was added.
    Added,

    /// A dependency was updated.
    Updated,

    /// A dependency was removed.
    Removed,

    /// A dependency remained unchanged.
    Unchanged,

    /// An unrecognized future change type.
    #[serde(other)]
    Unknown,
}

/// Summary of one resolved dependency change.
///
/// This does not replace the canonical dependency resolver or lockfile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DependencyChange {
    /// Canonical package identity represented as an opaque string.
    pub package_id: String,

    /// Nature of the change.
    pub kind: DependencyChangeKind,

    /// Previous package version, when applicable.
    pub previous_version: Option<String>,

    /// Resulting package version, when applicable.
    pub resulting_version: Option<String>,

    /// Optional source identity.
    pub source: Option<String>,
}

impl DependencyChange {
    /// Construct and validate a dependency-change record.
    pub fn new(
        package_id: impl Into<String>,
        kind: DependencyChangeKind,
    ) -> ResponseValidationResult<Self> {
        let change = Self {
            package_id: package_id.into(),
            kind,
            previous_version: None,
            resulting_version: None,
            source: None,
        };

        change.validate()?;

        Ok(change)
    }

    /// Attach the previous package version.
    #[must_use]
    pub fn with_previous_version(
        mut self,
        version: impl Into<String>,
    ) -> Self {
        self.previous_version = Some(version.into());
        self
    }

    /// Attach the resulting package version.
    #[must_use]
    pub fn with_resulting_version(
        mut self,
        version: impl Into<String>,
    ) -> Self {
        self.resulting_version = Some(version.into());
        self
    }

    /// Attach the dependency source identity.
    #[must_use]
    pub fn with_source(
        mut self,
        source: impl Into<String>,
    ) -> Self {
        self.source = Some(source.into());
        self
    }

    /// Validate the dependency-change record.
    pub fn validate(&self) -> ResponseValidationResult<()> {
        validate_identifier(
            "dependency_change.package_id",
            &self.package_id,
        )?;

        if let Some(version) = &self.previous_version {
            validate_identifier(
                "dependency_change.previous_version",
                version,
            )?;
        }

        if let Some(version) = &self.resulting_version {
            validate_identifier(
                "dependency_change.resulting_version",
                version,
            )?;
        }

        if let Some(source) = &self.source {
            validate_identifier(
                "dependency_change.source",
                source,
            )?;
        }

        if self.kind == DependencyChangeKind::Added
            && self.resulting_version.is_none()
        {
            return Err(ResponseValidationError::new(
                "dependency_change.resulting_version",
                "added dependency must identify its resulting version",
            ));
        }

        if self.kind == DependencyChangeKind::Removed
            && self.previous_version.is_none()
        {
            return Err(ResponseValidationError::new(
                "dependency_change.previous_version",
                "removed dependency must identify its previous version",
            ));
        }

        Ok(())
    }
}

// ============================================================================
// EXECUTION SUMMARY
// ============================================================================

/// Execution lifecycle state reported by an operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum ExecutionState {
    /// Execution has not started.
    NotStarted,

    /// Execution is in progress.
    Running,

    /// Execution completed successfully.
    Succeeded,

    /// Execution failed.
    Failed,

    /// Execution was cancelled.
    Cancelled,

    /// The execution state is not established.
    Unknown,
}

/// Exact elapsed time representation.
///
/// Nanoseconds are normalized to the range 0..1_000_000_000.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ElapsedTime {
    /// Whole elapsed seconds.
    pub seconds: u64,

    /// Remaining nanoseconds.
    pub nanoseconds: u32,
}

impl ElapsedTime {
    /// Construct a normalized elapsed-time value.
    pub fn new(
        seconds: u64,
        nanoseconds: u32,
    ) -> ResponseValidationResult<Self> {
        if nanoseconds >= 1_000_000_000 {
            return Err(ResponseValidationError::new(
                "elapsed_time.nanoseconds",
                "nanoseconds must be less than one billion",
            ));
        }

        Ok(Self {
            seconds,
            nanoseconds,
        })
    }

    /// Convert a standard Rust Duration without losing its precision.
    #[must_use]
    pub fn from_duration(duration: Duration) -> Self {
        Self {
            seconds: duration.as_secs(),
            nanoseconds: duration.subsec_nanos(),
        }
    }

    /// Convert to standard Rust Duration.
    #[must_use]
    pub fn to_duration(self) -> Duration {
        Duration::new(self.seconds, self.nanoseconds)
    }

    /// Validate the elapsed-time representation.
    pub fn validate(&self) -> ResponseValidationResult<()> {
        if self.nanoseconds >= 1_000_000_000 {
            return Err(ResponseValidationError::new(
                "elapsed_time.nanoseconds",
                "nanoseconds must be less than one billion",
            ));
        }

        Ok(())
    }
}

/// Execution summary supplied by the authoritative runtime.
///
/// This is descriptive transport information. It does not perform
/// execution, resource allocation, scheduling or runtime verification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionSummary {
    /// Execution lifecycle state.
    pub state: ExecutionState,

    /// Optional runtime environment identity.
    pub environment_id: Option<String>,

    /// Optional process or provider exit code.
    pub exit_code: Option<i64>,

    /// Optional elapsed execution time.
    pub elapsed: Option<ElapsedTime>,

    /// Additional namespaced execution observations.
    #[serde(default)]
    pub observations: BTreeMap<String, Value>,
}

impl ExecutionSummary {
    /// Construct an execution summary.
    #[must_use]
    pub fn new(state: ExecutionState) -> Self {
        Self {
            state,
            environment_id: None,
            exit_code: None,
            elapsed: None,
            observations: BTreeMap::new(),
        }
    }

    /// Attach an execution environment identity.
    #[must_use]
    pub fn with_environment(
        mut self,
        environment_id: impl Into<String>,
    ) -> Self {
        self.environment_id = Some(environment_id.into());
        self
    }

    /// Attach an exit code.
    #[must_use]
    pub fn with_exit_code(mut self, exit_code: i64) -> Self {
        self.exit_code = Some(exit_code);
        self
    }

    /// Attach elapsed execution time.
    #[must_use]
    pub fn with_elapsed(mut self, elapsed: Duration) -> Self {
        self.elapsed = Some(ElapsedTime::from_duration(elapsed));
        self
    }

    /// Attach an execution observation.
    #[must_use]
    pub fn with_observation(
        mut self,
        key: impl Into<String>,
        value: Value,
    ) -> Self {
        self.observations.insert(key.into(), value);
        self
    }

    /// Validate the execution summary.
    pub fn validate(&self) -> ResponseValidationResult<()> {
        if let Some(environment_id) = &self.environment_id {
            validate_identifier(
                "execution.environment_id",
                environment_id,
            )?;
        }

        if let Some(elapsed) = &self.elapsed {
            elapsed.validate()?;
        }

        for key in self.observations.keys() {
            validate_identifier(
                "execution.observations.key",
                key,
            )?;
        }

        if self.state == ExecutionState::Succeeded
            && self.exit_code.is_some_and(|code| code != 0)
        {
            return Err(ResponseValidationError::new(
                "execution.exit_code",
                "successful execution cannot report a nonzero exit code",
            ));
        }

        if self.state == ExecutionState::Failed
            && self.exit_code == Some(0)
        {
            return Err(ResponseValidationError::new(
                "execution.exit_code",
                "failed execution cannot report a successful zero exit code",
            ));
        }

        Ok(())
    }
}

// ============================================================================
// RESPONSE DATA
// ============================================================================

/// Structured data associated with a toolchain operation.
///
/// Canonical artifacts, diagnostics and dependency changes have dedicated
/// representations. Domain-specific and future operation data may be
/// transported through the extensible JSON value.
///
/// The extension map must not redefine canonical response fields.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ResponseData {
    /// References to canonical compiler artifacts.
    #[serde(default)]
    pub artifacts: Vec<ArtifactReference>,

    /// Structured diagnostics.
    #[serde(default)]
    pub diagnostics: Vec<ResponseDiagnostic>,

    /// Dependency changes reported by the dependency service.
    #[serde(default)]
    pub dependency_changes: Vec<DependencyChange>,

    /// Optional execution summary.
    pub execution: Option<ExecutionSummary>,

    /// Optional operation-specific structured result.
    ///
    /// This may contain workspace information, package metadata,
    /// deployment plans, test summaries or other API-owned data.
    pub value: Option<Value>,

    /// Namespaced extension data.
    #[serde(default)]
    pub extensions: BTreeMap<String, Value>,
}

impl ResponseData {
    /// Whether this response data contains no reported information.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.artifacts.is_empty()
            && self.diagnostics.is_empty()
            && self.dependency_changes.is_empty()
            && self.execution.is_none()
            && self.value.is_none()
            && self.extensions.is_empty()
    }

    /// Validate all structured response data.
    pub fn validate(&self) -> ResponseValidationResult<()> {
        for (index, artifact) in self.artifacts.iter().enumerate() {
            artifact.validate().map_err(|error| {
                ResponseValidationError::new(
                    format!("data.artifacts[{index}].{}", error.field()),
                    error.message(),
                )
            })?;
        }

        for (index, diagnostic) in self.diagnostics.iter().enumerate() {
            diagnostic.validate().map_err(|error| {
                ResponseValidationError::new(
                    format!("data.diagnostics[{index}].{}", error.field()),
                    error.message(),
                )
            })?;
        }

        for (index, change) in self.dependency_changes.iter().enumerate() {
            change.validate().map_err(|error| {
                ResponseValidationError::new(
                    format!("data.dependency_changes[{index}].{}", error.field()),
                    error.message(),
                )
            })?;
        }

        if let Some(execution) = &self.execution {
            execution.validate()?;
        }

        for key in self.extensions.keys() {
            validate_identifier("data.extensions.key", key)?;
        }

        Ok(())
    }
}

// ============================================================================
// RESPONSE ENVELOPE
// ============================================================================

/// Canonical response envelope for every Zamani Toolchain API operation.
///
/// A response is correlated with the originating request through:
///
/// - API version.
/// - Request identifier.
/// - Optional correlation identifier.
/// - Stable operation identifier.
///
/// `operation_id` is assigned by the service when an operation receives
/// a persistent execution identity. It is not the request identifier.
///
/// A response may be serialized and transported to local or remote clients.
///
/// Deserialized responses MUST pass validate() before being trusted.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResponseEnvelope {
    /// Toolchain API version used by the originating request.
    pub api_version: ToolchainApiVersion,

    /// Request identifier copied from the originating request.
    pub request_id: String,

    /// Optional correlation identifier copied from the request.
    pub correlation_id: Option<String>,

    /// Stable operation identifier.
    pub operation: String,

    /// Optional service-assigned persistent operation identity.
    pub operation_id: Option<String>,

    /// Current operation lifecycle state.
    pub status: ResponseStatus,

    /// Structured operation results.
    #[serde(default)]
    pub data: ResponseData,

    /// Structured error, when the operation encountered a failure.
    pub error: Option<ResponseError>,
}

impl ResponseEnvelope {
    /// Construct a response correlated with a validated request.
    ///
    /// The initial status is Accepted.
    ///
    /// This does not execute the requested operation.
    pub fn new(
        request: &RequestEnvelope,
    ) -> ToolchainResult<Self> {
        request.validate().map_err(|source: RequestValidationError| {
            ToolchainError::invalid_input(
                source.to_string(),
            )
            .with_operation("api.response.create")
            .with_source(source)
        })?;

        Ok(Self {
            api_version: request.context.api_version.clone(),
            request_id: request.context.request_id.clone(),
            correlation_id: request.context.correlation_id.clone(),
            operation: request.operation_name().to_owned(),
            operation_id: None,
            status: ResponseStatus::Accepted,
            data: ResponseData::default(),
            error: None,
        })
    }

    /// Construct a response representing a failed toolchain operation.
    #[must_use]
    pub fn from_error(
        request: &RequestEnvelope,
        error: &ToolchainError,
    ) -> Self {
        let status = if error.kind() == ToolchainErrorKind::Cancelled {
            ResponseStatus::Cancelled
        } else {
            ResponseStatus::Failed
        };

        Self {
            api_version: request.context.api_version.clone(),
            request_id: request.context.request_id.clone(),
            correlation_id: request.context.correlation_id.clone(),
            operation: request.operation_name().to_owned(),
            operation_id: None,
            status,
            data: ResponseData::default(),
            error: Some(ResponseError::from_toolchain_error(error)),
        }
    }

    /// Assign a persistent service-side operation identity.
    ///
    /// The identity may reference a build, task, deployment or other
    /// long-running operation.
    #[must_use]
    pub fn with_operation_id(
        mut self,
        operation_id: impl Into<String>,
    ) -> Self {
        self.operation_id = Some(operation_id.into());
        self
    }

    /// Set the operation lifecycle status.
    ///
    /// The completed response must still satisfy the envelope invariants.
    pub fn with_status(
        mut self,
        status: ResponseStatus,
    ) -> ResponseValidationResult<Self> {
        self.status = status;
        self.validate()?;
        Ok(self)
    }

    /// Attach structured response data.
    #[must_use]
    pub fn with_data(mut self, data: ResponseData) -> Self {
        self.data = data;
        self
    }

    /// Attach one canonical compiler artifact.
    #[must_use]
    pub fn with_artifact(mut self, artifact: &Artifact) -> Self {
        self.data.artifacts.push(ArtifactReference::from(artifact));
        self
    }

    /// Attach one structured diagnostic.
    #[must_use]
    pub fn with_diagnostic(
        mut self,
        diagnostic: ResponseDiagnostic,
    ) -> Self {
        self.data.diagnostics.push(diagnostic);
        self
    }

    /// Attach one dependency change.
    #[must_use]
    pub fn with_dependency_change(
        mut self,
        change: DependencyChange,
    ) -> Self {
        self.data.dependency_changes.push(change);
        self
    }

    /// Attach an execution summary.
    #[must_use]
    pub fn with_execution(
        mut self,
        execution: ExecutionSummary,
    ) -> Self {
        self.data.execution = Some(execution);
        self
    }

    /// Attach an operation-specific JSON result.
    #[must_use]
    pub fn with_value(mut self, value: Value) -> Self {
        self.data.value = Some(value);
        self
    }

    /// Attach a namespaced response extension.
    #[must_use]
    pub fn with_extension(
        mut self,
        key: impl Into<String>,
        value: Value,
    ) -> Self {
        self.data.extensions.insert(key.into(), value);
        self
    }

    /// Attach a canonical toolchain error.
    ///
    /// Cancellation remains distinguishable from ordinary failure.
    #[must_use]
    pub fn with_error(mut self, error: &ToolchainError) -> Self {
        self.status = if error.kind() == ToolchainErrorKind::Cancelled {
            ResponseStatus::Cancelled
        } else {
            ResponseStatus::Failed
        };

        self.error = Some(ResponseError::from_toolchain_error(error));
        self
    }

    /// Return whether the operation reached a terminal state.
    #[must_use]
    pub const fn is_terminal(&self) -> bool {
        self.status.is_terminal()
    }

    /// Return whether the operation fully succeeded.
    #[must_use]
    pub const fn is_success(&self) -> bool {
        self.status.is_success()
    }

    /// Validate the complete response envelope.
    ///
    /// This method must be called after deserialization and before
    /// accepting a response from an untrusted transport.
    pub fn validate(&self) -> ResponseValidationResult<()> {
        validate_identifier("request_id", &self.request_id)?;
        validate_identifier("operation", &self.operation)?;

        if let Some(correlation_id) = &self.correlation_id {
            validate_identifier("correlation_id", correlation_id)?;
        }

        if let Some(operation_id) = &self.operation_id {
            validate_identifier("operation_id", operation_id)?;
        }

        self.data.validate()?;

        if let Some(error) = &self.error {
            error.validate()?;
        }

        match self.status {
            ResponseStatus::Accepted | ResponseStatus::Running => {
                if self.error.is_some() {
                    return Err(ResponseValidationError::new(
                        "error",
                        "accepted or running operations must not contain a terminal error",
                    ));
                }
            }

            ResponseStatus::Succeeded => {
                if self.error.is_some() {
                    return Err(ResponseValidationError::new(
                        "error",
                        "successful operations must not contain an operation error",
                    ));
                }
            }

            ResponseStatus::Failed | ResponseStatus::Rejected => {
                if self.error.is_none() {
                    return Err(ResponseValidationError::new(
                        "error",
                        "failed or rejected operations must contain a structured error",
                    ));
                }
            }

            ResponseStatus::Cancelled => {
                if self.error.is_none() {
                    return Err(ResponseValidationError::new(
                        "error",
                        "cancelled operations must contain a cancellation explanation",
                    ));
                }
            }

            ResponseStatus::PartiallySucceeded
            | ResponseStatus::Unknown => {}
        }

        Ok(())
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::toolchain::ids::IdNamespace;
    use crate::toolchain::request_placeholder::Unused;

    use super::super::request::{
        ClientIdentity,
        RequestContext,
        ToolchainOperation,
    };

    fn request() -> RequestEnvelope {
        let context = RequestContext::new(
            "request-1",
            ClientIdentity::new("cli", "zamani-pkg")
                .expect("valid client"),
        )
        .expect("valid request context");

        // A workspace-discovery request supplies a simple, genuine
        // request variant without requiring a compiler or backend.
        let operation = ToolchainOperation::DiscoverWorkspace(
            super::super::request::WorkspaceDiscoveryRequest::new(
                std::path::PathBuf::from("."),
                true,
            )
            .expect("valid workspace request"),
        );

        RequestEnvelope::new(context, operation)
            .expect("valid request")
    }

    #[test]
    fn response_correlates_with_originating_request() {
        let request = request();
        let response = ResponseEnvelope::new(&request)
            .expect("valid response");

        assert_eq!(
            response.request_id,
            request.context.request_id
        );

        assert_eq!(
            response.operation,
            request.operation_name()
        );

        assert_eq!(
            response.status,
            ResponseStatus::Accepted
        );

        assert!(response.validate().is_ok());
    }

    #[test]
    fn request_and_response_api_versions_match() {
        let request = request();
        let response = ResponseEnvelope::new(&request)
            .expect("valid response");

        assert_eq!(
            response.api_version,
            request.context.api_version
        );
    }

    #[test]
    fn successful_response_requires_no_error() {
        let response = ResponseEnvelope::new(&request())
            .expect("valid response")
            .with_status(ResponseStatus::Succeeded)
            .expect("valid success");

        assert!(response.is_success());
        assert!(response.is_terminal());
        assert!(response.error.is_none());
    }

    #[test]
    fn failed_response_requires_structured_error() {
        let mut response = ResponseEnvelope::new(&request())
            .expect("valid response");

        response.status = ResponseStatus::Failed;

        assert!(response.validate().is_err());
    }

    #[test]
    fn cancellation_remains_distinct_from_failure() {
        let request = request();

        let cancellation =
            ToolchainError::cancelled("operation cancelled");

        let response =
            ResponseEnvelope::from_error(&request, &cancellation);

        assert_eq!(
            response.status,
            ResponseStatus::Cancelled
        );

        assert!(response.error.is_some());
        assert!(response.validate().is_ok());
    }

    #[test]
    fn canonical_errors_are_preserved() {
        let error =
            ToolchainError::build_failed("compilation failed")
                .with_operation("build.execute")
                .with_subject("project:sample")
                .with_context("target realization");

        let response =
            ResponseEnvelope::from_error(&request(), &error);

        let transported =
            response.error.expect("structured error");

        assert_eq!(transported.code, error.code());
        assert_eq!(transported.category, error.category());
        assert_eq!(transported.message, error.message());
        assert_eq!(transported.operation.as_deref(), Some("build.execute"));
        assert_eq!(transported.subject.as_deref(), Some("project:sample"));
        assert_eq!(transported.context.len(), 1);
    }

    #[test]
    fn diagnostic_supports_open_severity_identifiers() {
        let diagnostic = ResponseDiagnostic::new(
            "quantum.provider.unavailable",
            "warning",
            "Optional quantum provider is unavailable",
        )
        .expect("valid diagnostic")
        .with_origin("quantum.provider");

        assert_eq!(diagnostic.severity, "warning");
        assert_eq!(
            diagnostic.origin.as_deref(),
            Some("quantum.provider")
        );

        assert!(diagnostic.validate().is_ok());
    }

    #[test]
    fn diagnostic_location_rejects_invalid_ranges() {
        let location = DiagnosticLocation::new(
            "src/main.zm",
            20,
            10,
            1,
            1,
            1,
            5,
        );

        assert!(location.is_err());
    }

    #[test]
    fn diagnostic_location_accepts_valid_ranges() {
        let location = DiagnosticLocation::new(
            "src/main.zm",
            0,
            12,
            1,
            1,
            1,
            13,
        )
        .expect("valid source location");

        assert!(location.validate().is_ok());
    }

    #[test]
    fn elapsed_time_preserves_nanosecond_precision() {
        let duration = Duration::new(42, 123_456_789);

        let elapsed = ElapsedTime::from_duration(duration);

        assert_eq!(elapsed.seconds, 42);
        assert_eq!(elapsed.nanoseconds, 123_456_789);
        assert_eq!(elapsed.to_duration(), duration);
    }

    #[test]
    fn elapsed_time_rejects_non_normalized_nanoseconds() {
        assert!(ElapsedTime::new(1, 1_000_000_000).is_err());
    }

    #[test]
    fn dependency_addition_requires_resulting_version() {
        let change = DependencyChange::new(
            "example.package",
            DependencyChangeKind::Added,
        );

        assert!(change.is_err());
    }

    #[test]
    fn dependency_change_preserves_version_information() {
        let change = DependencyChange::new(
            "example.package",
            DependencyChangeKind::Updated,
        )
        .expect("valid dependency change")
        .with_previous_version("1.0.0")
        .with_resulting_version("2.0.0");

        assert!(change.validate().is_ok());
        assert_eq!(
            change.previous_version.as_deref(),
            Some("1.0.0")
        );
        assert_eq!(
            change.resulting_version.as_deref(),
            Some("2.0.0")
        );
    }

    #[test]
    fn response_data_supports_future_extensions() {
        let response = ResponseEnvelope::new(&request())
            .expect("valid response")
            .with_extension(
                "third_party.example",
                serde_json::json!({
                    "provider": "example",
                    "realization": "future-target"
                }),
            );

        assert!(
            response.data.extensions.contains_key(
                "third_party.example"
            )
        );
    }

    #[test]
    fn response_round_trips_through_json() {
        let response = ResponseEnvelope::new(&request())
            .expect("valid response")
            .with_operation_id("build:example:1")
            .with_diagnostic(
                ResponseDiagnostic::new(
                    "compiler.note",
                    "note",
                    "Compilation completed",
                )
                .expect("valid diagnostic"),
            )
            .with_value(serde_json::json!({
                "completed": true
            }))
            .with_status(ResponseStatus::Succeeded)
            .expect("valid success");

        let encoded = serde_json::to_string(&response)
            .expect("serialization");

        let decoded: ResponseEnvelope =
            serde_json::from_str(&encoded)
                .expect("deserialization");

        assert_eq!(decoded, response);
        assert!(decoded.validate().is_ok());
    }

    #[test]
    fn deserialized_invalid_response_is_rejected() {
        let mut response = ResponseEnvelope::new(&request())
            .expect("valid response");

        response.request_id.clear();

        let encoded = serde_json::to_string(&response)
            .expect("serialization");

        let decoded: ResponseEnvelope =
            serde_json::from_str(&encoded)
                .expect("wire decoding");

        assert!(decoded.validate().is_err());
    }

    #[test]
    fn running_operation_is_not_terminal() {
        let response = ResponseEnvelope::new(&request())
            .expect("valid response")
            .with_status(ResponseStatus::Running)
            .expect("valid running response");

        assert!(!response.is_terminal());
        assert!(!response.is_success());
    }

    #[test]
    fn partial_success_is_distinct_from_success() {
        let response = ResponseEnvelope::new(&request())
            .expect("valid response")
            .with_status(ResponseStatus::PartiallySucceeded)
            .expect("valid partial response");

        assert!(response.is_terminal());
        assert!(!response.is_success());
    }

    #[test]
    fn response_extensions_do_not_change_canonical_status() {
        let response = ResponseEnvelope::new(&request())
            .expect("valid response")
            .with_extension(
                "provider.example",
                Value::String("available".to_owned()),
            );

        assert_eq!(response.status, ResponseStatus::Accepted);
    }

    #[test]
    fn response_error_does_not_expose_causal_error_objects() {
        let error = ToolchainError::provider_failed(
            "provider operation failed",
        )
        .with_context("provider invocation");

        let response =
            ResponseEnvelope::from_error(&request(), &error);

        let encoded = serde_json::to_string(&response)
            .expect("serialization");

        assert!(encoded.contains("provider operation failed"));
        assert!(!encoded.contains("source"));
        assert!(response.validate().is_ok());
    }
}
