#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Backend Output Contracts
//!
//! Path: `src/backend/output.rs`
//! Edition: Rust 2021.
//! Minimum Rust version: 1.97.
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Represent the results of backend compilation without coupling code
//! generation to filesystem writes, console output, a particular target,
//! or a particular computational domain.
//!
//! ## Architectural ownership
//!
//! This module owns:
//!
//! - The collection of outputs produced by a backend compilation.
//! - Typed artifact payloads and artifact metadata.
//! - Artifact validation state and output disposition.
//! - Structured backend diagnostics attached to output.
//! - Consistency checks for output collections.
//! - A compatibility adapter for the existing compiler output structure.
//!
//! It does not own:
//!
//! - Source-language meaning, grammar, or semantic validation.
//! - IR generation, optimization, or IR verification.
//! - Target selection, capability negotiation, or hardware allocation.
//! - LLVM, WebAssembly, quantum, HDL, native, or bytecode emission.
//! - Filesystem writes, process execution, linking, or deployment.
//! - Artifact serialization formats or artifact hashing algorithms.
//! - Runtime execution or claims that an artifact works on every target.
//!
//! ## Architectural authority
//!
//! This module follows the contracts in:
//!
//! - `grammar/DESIGN.md`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/validation/semantic-boundaries.md`
//! - `src/compiler_types.rs`
//! - `src/compiler/mod.rs`
//! - `src/compiler/diagnostics.rs`
//! - `src/backend/errors.rs`
//! - `src/backend/diagnostics.rs`
//! - `src/backend/ids.rs`
//! - `src/backend.rs`
//!
//! ## Output invariants
//!
//! - Generation and persistence are separate operations.
//! - Artifact bytes are never assumed to be UTF-8.
//! - Text artifacts are not automatically assumed to be valid source,
//!   LLVM IR, assembly, QASM, HDL, or any other particular format.
//! - Artifact format, target identity, and validation state are explicit.
//! - An unverified artifact is never represented as verified.
//! - A target-specific artifact does not imply universal target support.
//! - Unsupported requirements must be represented by structured errors or
//!   diagnostics, not silently removed from the output contract.
//! - Output collections preserve insertion order.
//! - Duplicate artifact identities within one collection are rejected.
//! - No universal hardware capacity or artifact-count limit is imposed.
//! - Allocation failures and process resource exhaustion remain possible;
//!   this module does not claim that available memory is unlimited.
//! - This module performs no filesystem, network, or console operations.
//!
//! ## Integration
//!
//! The canonical `src/backend/mod.rs` must declare:
//!
//!     pub mod output;
//!
//! The backend pipeline should construct `BackendOutput` after generation
//! and artifact checks. A later persistence stage may consume the completed
//! value and write artifacts using an explicitly configured destination.
//!
//! `src/compiler/mod.rs` currently defines `CompilationOutput` with `bytes`,
//! `target`, and `size_bytes`. `to_legacy_compilation_output` provides an
//! explicit adapter for that existing API. It accepts exactly one artifact
//! and rejects ambiguous multi-artifact results rather than discarding data.
//!
//! `src/backend/artifact.rs` does not currently exist on the inspected
//! default branch. This file therefore defines the output-facing artifact
//! record without importing a speculative sibling module. When the artifact
//! contract is implemented, ownership must be reconciled deliberately rather
//! than creating two independent authoritative artifact models.
//!
//! ## Completion contract
//!
//! This file is complete against its own API when its unit tests pass and
//! the canonical module root exposes it. Repository-wide completion also
//! requires resolving the existing `src/backend.rs` / `src/backend/mod.rs`
//! collision, integrating the pipeline, and running the full workspace gates.

use std::collections::BTreeSet;
use std::fmt;

use super::diagnostics::BackendDiagnostic;
use super::errors::{BackendError, BackendErrorKind, BackendResult};
use super::ids::ArtifactId;

/// The representation of an artifact's payload.
///
/// Payload bytes are retained exactly as produced by the emitter. Text
/// encoding is metadata; it does not change or normalize the payload.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArtifactEncoding {
    /// An artifact whose format is textual.
    ///
    /// This label does not prove that the bytes are valid UTF-8. Consumers
    /// requiring UTF-8 must call [`ArtifactPayload::as_text`].
    Text,

    /// An artifact whose format is binary or otherwise byte-oriented.
    Binary,
}

/// An artifact payload owned by a backend output.
///
/// The payload is not interpreted by this type. Format-specific parsers and
/// validators belong to the relevant target or artifact-format subsystem.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArtifactPayload {
    encoding: ArtifactEncoding,
    bytes: Vec<u8>,
}

impl ArtifactPayload {
    /// Construct a textual payload from UTF-8 text.
    pub fn text(value: impl Into<String>) -> Self {
        Self {
            encoding: ArtifactEncoding::Text,
            bytes: value.into().into_bytes(),
        }
    }

    /// Construct a binary payload without changing its bytes.
    pub fn binary(bytes: impl Into<Vec<u8>>) -> Self {
        Self {
            encoding: ArtifactEncoding::Binary,
            bytes: bytes.into(),
        }
    }

    /// Construct a payload from bytes and an explicit encoding label.
    ///
    /// Text encoding is not validated here. Use [`Self::as_text`] to check
    /// whether a text-labelled payload is valid UTF-8.
    pub fn from_bytes(encoding: ArtifactEncoding, bytes: Vec<u8>) -> Self {
        Self { encoding, bytes }
    }

    /// Return the payload encoding label.
    pub const fn encoding(&self) -> ArtifactEncoding {
        self.encoding
    }

    /// Return the payload bytes.
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Return the payload length in bytes.
    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    /// Whether the payload is empty.
    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    /// Interpret a text-labelled payload as UTF-8.
    ///
    /// Returns an error if the encoding label is `Binary` or if the bytes
    /// are not valid UTF-8. No lossy conversion is performed.
    pub fn as_text(&self) -> BackendResult<&str> {
        if self.encoding != ArtifactEncoding::Text {
            return Err(BackendError::new(
                BackendErrorKind::InvalidArtifact,
                "binary artifact cannot be interpreted as a text artifact",
            ));
        }

        std::str::from_utf8(&self.bytes).map_err(|error| {
            BackendError::new(
                BackendErrorKind::InvalidArtifact,
                "text-labelled artifact contains invalid UTF-8",
            )
            .with_cause(error.to_string())
        })
    }

    /// Consume the payload and return its bytes.
    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }
}

/// Validation evidence associated with a generated artifact.
///
/// `Verified` means that a named verifier completed successfully for the
/// artifact represented by this record. It does not establish that the
/// artifact is semantically equivalent to every possible target or that it
/// will execute successfully in every runtime.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArtifactValidation {
    /// The artifact has not been verified.
    Unverified,

    /// A named verifier successfully checked the artifact.
    Verified {
        /// Stable name of the verifier or validation procedure.
        verifier: String,

        /// Optional version or identity of the verifier used.
        verifier_version: Option<String>,
    },

    /// Verification was explicitly not required by the applicable policy.
    ///
    /// This is not equivalent to successful verification.
    NotRequired {
        /// Explanation of why verification was not required.
        reason: String,
    },
}

impl ArtifactValidation {
    /// Whether a verifier has explicitly validated this artifact.
    pub fn is_verified(&self) -> bool {
        matches!(self, Self::Verified { .. })
    }
}

/// Metadata and payload for one generated artifact.
///
/// The `ArtifactId` must be allocated by the caller through the repository's
/// backend identifier contract. This type does not create global IDs or
/// assume that an ID is globally unique across unrelated builds.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutputArtifact {
    /// Stable identity assigned to this artifact.
    pub id: ArtifactId,

    /// Target identity or target-family name that produced the artifact.
    ///
    /// Examples include a target triple or a registered target identifier.
    pub target: String,

    /// Extensible artifact-format identifier.
    ///
    /// Examples include `llvm-ir`, `wasm`, `wasm-text`, `qasm`, `verilog`,
    /// `systemverilog`, `native-assembly`, and a registered custom format.
    pub format: String,

    /// Optional suggested filename extension, without path components.
    ///
    /// This is metadata only. It is not a destination path.
    pub extension: Option<String>,

    /// Optional media type or format-specific content type.
    pub media_type: Option<String>,

    /// Exact artifact content.
    pub payload: ArtifactPayload,

    /// Verification evidence for this artifact.
    pub validation: ArtifactValidation,

    /// Optional content digest supplied by an authoritative hashing stage.
    ///
    /// This field is descriptive metadata. It is not computed or trusted by
    /// this module, and must not be used as a security proof by itself.
    pub content_digest: Option<String>,
}

impl OutputArtifact {
    /// Construct an artifact with the minimum required metadata.
    pub fn new(
        id: ArtifactId,
        target: impl Into<String>,
        format: impl Into<String>,
        payload: ArtifactPayload,
    ) -> BackendResult<Self> {
        let artifact = Self {
            id,
            target: target.into(),
            format: format.into(),
            extension: None,
            media_type: None,
            payload,
            validation: ArtifactValidation::Unverified,
            content_digest: None,
        };

        artifact.validate_metadata()?;
        Ok(artifact)
    }

    /// Validate metadata owned by this module.
    ///
    /// This does not parse or verify the artifact's actual format.
    pub fn validate_metadata(&self) -> BackendResult<()> {
        if self.target.trim().is_empty() {
            return Err(BackendError::new(
                BackendErrorKind::InvalidArtifact,
                "artifact target must not be empty",
            ));
        }

        if self.format.trim().is_empty() {
            return Err(BackendError::new(
                BackendErrorKind::InvalidArtifact,
                "artifact format must not be empty",
            )
            .with_target(self.target.clone()));
        }

        if let Some(extension) = &self.extension {
            if extension.is_empty()
                || extension.contains('/')
                || extension.contains('\\')
                || extension.contains('\0')
            {
                return Err(BackendError::new(
                    BackendErrorKind::InvalidArtifact,
                    "artifact extension must be a non-empty filename extension without path separators",
                )
                .with_target(self.target.clone()));
            }
        }

        if let Some(media_type) = &self.media_type {
            if media_type.trim().is_empty() {
                return Err(BackendError::new(
                    BackendErrorKind::InvalidArtifact,
                    "artifact media type must not be empty when supplied",
                )
                .with_target(self.target.clone()));
            }
        }

        if let Some(digest) = &self.content_digest {
            if digest.trim().is_empty() {
                return Err(BackendError::new(
                    BackendErrorKind::InvalidArtifact,
                    "artifact content digest must not be empty when supplied",
                )
                .with_target(self.target.clone()));
            }
        }

        if let ArtifactValidation::Verified { verifier, .. } = &self.validation {
            if verifier.trim().is_empty() {
                return Err(BackendError::new(
                    BackendErrorKind::InvalidArtifact,
                    "verified artifact must identify the verifier used",
                )
                .with_target(self.target.clone()));
            }
        }

        if let ArtifactValidation::NotRequired { reason } = &self.validation {
            if reason.trim().is_empty() {
                return Err(BackendError::new(
                    BackendErrorKind::InvalidArtifact,
                    "an explicit verification exemption must include a reason",
                )
                .with_target(self.target.clone()));
            }
        }

        Ok(())
    }

    /// Return the exact payload length in bytes.
    pub fn size_bytes(&self) -> usize {
        self.payload.len()
    }
}

/// Describes what the caller is expected to do with completed output.
///
/// This is a disposition contract, not a filesystem operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum OutputDisposition {
    /// Keep the generated artifacts in memory.
    ///
    /// No file is created by this module.
    #[default]
    InMemory,

    /// The caller is responsible for persisting artifacts.
    ///
    /// The output collection contains no implied destination path.
    CallerManagedPersistence,
}

/// The outcome category of a backend output collection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum OutputStatus {
    /// Output generation completed according to the producing stage.
    ///
    /// This does not automatically mean that every artifact was verified.
    #[default]
    Complete,

    /// The collection represents incomplete work and must not be mistaken
    /// for a successful complete compilation.
    Incomplete,
}

/// A complete collection of artifacts and diagnostics from a backend stage.
///
/// Artifacts preserve insertion order. IDs must be unique within the
/// collection. Diagnostics preserve emission order; deterministic sorting,
/// deduplication, and rendering belong to the diagnostic subsystem.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct BackendOutput {
    artifacts: Vec<OutputArtifact>,
    diagnostics: Vec<BackendDiagnostic>,
    disposition: OutputDisposition,
    status: OutputStatus,
}

impl BackendOutput {
    /// Create an empty output collection with an explicit disposition.
    pub fn new(disposition: OutputDisposition) -> Self {
        Self {
            artifacts: Vec::new(),
            diagnostics: Vec::new(),
            disposition,
            status: OutputStatus::Complete,
        }
    }

    /// Create an empty output collection using the in-memory disposition.
    pub fn in_memory() -> Self {
        Self::new(OutputDisposition::InMemory)
    }

    /// Return the artifact collection in insertion order.
    pub fn artifacts(&self) -> &[OutputArtifact] {
        &self.artifacts
    }

    /// Return the diagnostics in emission order.
    pub fn diagnostics(&self) -> &[BackendDiagnostic] {
        &self.diagnostics
    }

    /// Return the output disposition.
    pub const fn disposition(&self) -> OutputDisposition {
        self.disposition
    }

    /// Return the output status.
    pub const fn status(&self) -> OutputStatus {
        self.status
    }

    /// Whether the output collection has no artifacts.
    pub fn is_empty(&self) -> bool {
        self.artifacts.is_empty()
    }

    /// Return the number of artifacts.
    pub fn len(&self) -> usize {
        self.artifacts.len()
    }

    /// Whether this output collection contains an error-severity diagnostic.
    pub fn has_errors(&self) -> bool {
        self.diagnostics.iter().any(BackendDiagnostic::is_error)
    }

    /// Set the disposition without performing persistence.
    pub fn set_disposition(&mut self, disposition: OutputDisposition) {
        self.disposition = disposition;
    }

    /// Mark the collection incomplete.
    ///
    /// Incomplete output may be useful for diagnostics or tooling, but must
    /// not be reported as a successful complete compilation.
    pub fn mark_incomplete(&mut self) {
        self.status = OutputStatus::Incomplete;
    }

    /// Append an artifact after validating its metadata and identity.
    ///
    /// The same artifact ID cannot be inserted twice into this collection.
    /// No artifact-count limit is imposed by this API.
    pub fn push_artifact(&mut self, artifact: OutputArtifact) -> BackendResult<()> {
        artifact.validate_metadata()?;

        let candidate_id = artifact.id.to_string();

        if self
            .artifacts
            .iter()
            .any(|existing| existing.id.to_string() == candidate_id)
        {
            return Err(BackendError::new(
                BackendErrorKind::InvalidArtifact,
                format!("duplicate artifact identity `{candidate_id}` in output collection"),
            )
            .with_target(artifact.target.clone()));
        }

        self.artifacts.push(artifact);
        Ok(())
    }

    /// Append a structured diagnostic.
    pub fn push_diagnostic(&mut self, diagnostic: BackendDiagnostic) {
        self.diagnostics.push(diagnostic);
    }

    /// Validate the collection-level invariants.
    ///
    /// Format-specific validation and required-verifier policy are enforced
    /// by the relevant pipeline stages, not inferred here.
    pub fn validate(&self) -> BackendResult<()> {
        let mut identities = BTreeSet::new();

        for artifact in &self.artifacts {
            artifact.validate_metadata()?;

            let identity = artifact.id.to_string();
            if !identities.insert(identity.clone()) {
                return Err(BackendError::new(
                    BackendErrorKind::InvalidArtifact,
                    format!("duplicate artifact identity `{identity}` in output collection"),
                )
                .with_target(artifact.target.clone()));
            }
        }

        if self.status == OutputStatus::Complete && self.has_errors() {
            return Err(BackendError::new(
                BackendErrorKind::InvalidArtifact,
                "output is marked complete but contains error-severity diagnostics",
            ));
        }

        Ok(())
    }

    /// Return the sum of payload lengths when representable by `usize`.
    ///
    /// This is a calculated statistic, not an allocation request. Overflow
    /// returns `None` rather than wrapping. The sum can be smaller than the
    /// actual memory retained because vector and metadata allocations are
    /// not included.
    pub fn total_payload_bytes(&self) -> Option<usize> {
        self.artifacts
            .iter()
            .try_fold(0usize, |total, artifact| {
                total.checked_add(artifact.size_bytes())
            })
    }

    /// Find an artifact by its stable ID.
    pub fn artifact(&self, id: &ArtifactId) -> Option<&OutputArtifact> {
        self.artifacts.iter().find(|artifact| &artifact.id == id)
    }

    /// Consume the collection and return its artifacts.
    pub fn into_artifacts(self) -> Vec<OutputArtifact> {
        self.artifacts
    }

    /// Convert a single-artifact result to the legacy compiler output type.
    ///
    /// The legacy API stores only bytes, target, and size. It cannot represent
    /// multiple artifacts, validation evidence, or structured diagnostics.
    /// Therefore this adapter rejects ambiguous or unsuccessful output
    /// instead of dropping metadata or selecting an arbitrary artifact.
    pub fn into_legacy_compilation_output(
        self,
    ) -> BackendResult<crate::compiler::CompilationOutput> {
        self.validate()?;

        if self.status != OutputStatus::Complete {
            return Err(BackendError::new(
                BackendErrorKind::InvalidArtifact,
                "incomplete backend output cannot be converted to a successful compiler result",
            ));
        }

        if self.has_errors() {
            return Err(BackendError::new(
                BackendErrorKind::InvalidArtifact,
                "backend output containing errors cannot be converted to a successful compiler result",
            ));
        }

        if self.artifacts.len() != 1 {
            return Err(BackendError::new(
                BackendErrorKind::InvalidInput,
                format!(
                    "legacy CompilationOutput requires exactly one artifact; received {}",
                    self.artifacts.len()
                ),
            ));
        }

        let artifact = self.artifacts.into_iter().next().ok_or_else(|| {
            BackendError::new(
                BackendErrorKind::Internal,
                "artifact count changed while converting backend output",
            )
        })?;

        let size_bytes = artifact.size_bytes();
        let bytes = artifact.payload.into_bytes();

        Ok(crate::compiler::CompilationOutput {
            bytes,
            target: artifact.target,
            size_bytes,
        })
    }
}

impl fmt::Display for ArtifactEncoding {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Text => formatter.write_str("text"),
            Self::Binary => formatter.write_str("binary"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::backend::ids::{ArtifactId, IdNamespace};

    fn artifact_id(value: u128) -> ArtifactId {
        let namespace = IdNamespace::new("zamani.artifact")
            .expect("test namespace is valid");

        ArtifactId::new(namespace, value)
            .expect("test artifact ID is valid")
    }

    fn text_artifact(value: u128, target: &str, content: &str) -> OutputArtifact {
        OutputArtifact::new(
            artifact_id(value),
            target,
            "test-text",
            ArtifactPayload::text(content),
        )
        .expect("test artifact metadata is valid")
    }

    #[test]
    fn output_starts_empty_and_in_memory() {
        let output = BackendOutput::in_memory();

        assert!(output.is_empty());
        assert_eq!(output.len(), 0);
        assert_eq!(output.disposition(), OutputDisposition::InMemory);
        assert_eq!(output.status(), OutputStatus::Complete);
        assert_eq!(output.total_payload_bytes(), Some(0));
    }

    #[test]
    fn preserves_artifact_bytes_and_metadata() {
        let artifact = text_artifact(1, "test-target", "hello");
        assert_eq!(artifact.size_bytes(), 5);
        assert_eq!(artifact.payload.as_text().expect("valid UTF-8"), "hello");

        let mut output = BackendOutput::in_memory();
        output.push_artifact(artifact).expect("unique artifact");

        assert_eq!(output.len(), 1);
        assert_eq!(output.total_payload_bytes(), Some(5));
        output.validate().expect("collection is valid");
    }

    #[test]
    fn binary_payload_is_not_mistaken_for_text() {
        let payload = ArtifactPayload::binary(vec![0xff, 0x00, 0x80]);

        assert_eq!(payload.as_bytes(), &[0xff, 0x00, 0x80]);
        assert_eq!(payload.len(), 3);
        assert!(payload.as_text().is_err());
    }

    #[test]
    fn invalid_utf8_is_not_lossily_converted() {
        let payload =
            ArtifactPayload::from_bytes(ArtifactEncoding::Text, vec![0xff]);

        assert!(payload.as_text().is_err());
    }

    #[test]
    fn duplicate_artifact_ids_are_rejected() {
        let mut output = BackendOutput::in_memory();

        output
            .push_artifact(text_artifact(1, "target-a", "one"))
            .expect("first artifact");

        let result = output.push_artifact(text_artifact(1, "target-a", "two"));

        assert!(result.is_err());
        assert_eq!(output.len(), 1);
    }

    #[test]
    fn artifacts_keep_insertion_order() {
        let mut output = BackendOutput::in_memory();

        output
            .push_artifact(text_artifact(2, "target", "second"))
            .expect("first insertion");

        output
            .push_artifact(text_artifact(1, "target", "first"))
            .expect("second insertion");

        assert_eq!(
            output.artifacts()[0].payload.as_text().expect("text"),
            "second"
        );
        assert_eq!(
            output.artifacts()[1].payload.as_text().expect("text"),
            "first"
        );
    }

    #[test]
    fn empty_target_is_rejected() {
        let result = OutputArtifact::new(
            artifact_id(1),
            " ",
            "test-text",
            ArtifactPayload::text("content"),
        );

        assert!(result.is_err());
    }

    #[test]
    fn empty_format_is_rejected() {
        let result = OutputArtifact::new(
            artifact_id(1),
            "test-target",
            " ",
            ArtifactPayload::text("content"),
        );

        assert!(result.is_err());
    }

    #[test]
    fn path_separators_are_rejected_in_extension() {
        let mut artifact = text_artifact(1, "target", "content");
        artifact.extension = Some("../output.ll".into());

        assert!(artifact.validate_metadata().is_err());
    }

    #[test]
    fn empty_verifier_is_rejected() {
        let mut artifact = text_artifact(1, "target", "content");
        artifact.validation = ArtifactValidation::Verified {
            verifier: " ".into(),
            verifier_version: None,
        };

        assert!(artifact.validate_metadata().is_err());
    }

    #[test]
    fn incomplete_output_is_not_successful_legacy_output() {
        let mut output = BackendOutput::in_memory();
        output
            .push_artifact(text_artifact(1, "target", "content"))
            .expect("valid artifact");
        output.mark_incomplete();

        assert!(output.into_legacy_compilation_output().is_err());
    }

    #[test]
    fn legacy_adapter_requires_exactly_one_artifact() {
        let mut output = BackendOutput::in_memory();
        output
            .push_artifact(text_artifact(1, "target", "one"))
            .expect("first artifact");
        output
            .push_artifact(text_artifact(2, "target", "two"))
            .expect("second artifact");

        assert!(output.into_legacy_compilation_output().is_err());
    }

    #[test]
    fn legacy_adapter_preserves_bytes_target_and_size() {
        let mut output = BackendOutput::in_memory();
        output
            .push_artifact(text_artifact(1, "test-target", "hello"))
            .expect("valid artifact");

        let legacy = output
            .into_legacy_compilation_output()
            .expect("single artifact converts");

        assert_eq!(legacy.bytes, b"hello");
        assert_eq!(legacy.target, "test-target");
        assert_eq!(legacy.size_bytes, 5);
    }

    #[test]
    fn total_size_is_computed_from_payloads() {
        let mut output = BackendOutput::in_memory();
        output
            .push_artifact(text_artifact(1, "target", "abc"))
            .expect("first artifact");
        output
            .push_artifact(text_artifact(2, "target", "defgh"))
            .expect("second artifact");

        assert_eq!(output.total_payload_bytes(), Some(8));
    }

    #[test]
    fn caller_managed_persistence_does_not_write_files() {
        let mut output =
            BackendOutput::new(OutputDisposition::CallerManagedPersistence);

        output
            .push_artifact(text_artifact(1, "target", "content"))
            .expect("valid artifact");

        assert_eq!(
            output.disposition(),
            OutputDisposition::CallerManagedPersistence
        );
        assert_eq!(output.len(), 1);
    }

    #[test]
    fn incomplete_output_can_be_inspected_but_not_claimed_complete() {
        let mut output = BackendOutput::in_memory();
        output.mark_incomplete();

        assert_eq!(output.status(), OutputStatus::Incomplete);
        assert!(output.validate().is_ok());
    }
}