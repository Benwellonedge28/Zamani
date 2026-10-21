#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Backend Artifacts
//!
//! Path: `src/backend/artifact.rs`
//! Edition: Rust 2021.
//! Minimum Rust version: 1.97.
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Define the canonical representation of artifacts produced by Zamani
//! backends. Artifacts can represent textual IR, assembly, object files,
//! executables, bytecode, WebAssembly, quantum programs, HDL, manifests,
//! debug information, and registered future formats.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Artifact format identity and encoding.
//! - Artifact payload ownership.
//! - Artifact metadata and target identity.
//! - Artifact validation evidence.
//! - Artifact provenance records.
//! - Content digest computation and integrity checking.
//! - Validation of artifact-local invariants.
//!
//! This module does not own:
//!
//! - Source-language semantics or grammar.
//! - Semantic analysis, IR construction, or IR verification.
//! - Target selection or target capability negotiation.
//! - LLVM, WebAssembly, quantum, HDL, or native code generation.
//! - Filesystem persistence, process execution, linking, or deployment.
//! - The policy deciding which verifier is required for a target.
//! - Proof that a generated artifact preserves source-program semantics.
//!
//! ## Architectural contract
//!
//! The artifact model follows the repository's grammar and semantic
//! boundaries. Artifacts are outputs of compilation stages, not substitutes
//! for the semantic model or canonical intermediate representations.
//!
//! An artifact's declared format is metadata, not proof that its payload
//! conforms to that format. Format-specific validation must be performed by
//! the appropriate parser, verifier, assembler, simulator, or toolchain.
//!
//! A verified artifact records evidence of a completed validation procedure.
//! It does not claim universal execution compatibility.
//!
//! ## Scalability
//!
//! No universal maximum artifact size, artifact count, device count, qubit
//! count, tensor rank, node count, or hardware capacity is imposed here.
//! Rust allocations, address-space limits, process limits, and available
//! resources remain finite. Allocation failure must not be confused with
//! successful artifact creation.
//!
//! ## Integration
//!
//! - `src/backend/ids.rs` owns `ArtifactId`.
//! - `src/backend/errors.rs` owns `BackendError` and `BackendResult`.
//! - `src/backend/output.rs` owns output collections and must re-export the
//!   artifact types from this module instead of defining duplicate types.
//! - `src/backend/manifest.rs` should own persisted artifact manifests.
//! - `src/backend/reproducibility.rs` should own build fingerprints.
//! - Target-specific modules own format parsing and verification.
//!
//! The canonical backend module root must declare `pub mod artifact;`.
//!
//! No filesystem writes, console output, network access, or external process
//! execution occurs in this module.

use std::fmt;
use std::str::FromStr;

use sha2::{Digest, Sha256};

use super::errors::{BackendError, BackendResult};
use super::ids::ArtifactId;

/// The encoding of an artifact's payload.
///
/// Encoding is distinct from artifact format. For example, LLVM IR and QASM
/// are generally text-encoded artifacts, whereas object files and WebAssembly
/// modules are generally binary-encoded artifacts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArtifactEncoding {
    /// Text-encoded content.
    Text,

    /// Binary or otherwise opaque byte content.
    Binary,
}

/// A validated, extensible artifact-format identifier.
///
/// Format names are identifiers, not filenames or paths. This type does not
/// maintain a closed list of formats, allowing future domains to register
/// formats without changing a universal enum.
///
/// Examples include `llvm-ir`, `wasm`, `wasm-text`, `qasm`, `verilog`,
/// `systemverilog`, `native-assembly`, and vendor-specific formats.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ArtifactFormat(String);

impl ArtifactFormat {
    /// Construct a format identifier.
    ///
    /// Identifiers must begin with an ASCII letter or digit. Subsequent
    /// characters may be ASCII letters, digits, `.`, `_`, `-`, or `+`.
    /// Identifiers are preserved as supplied; callers should use a
    /// consistent canonical spelling for reproducible metadata.
    pub fn new(value: impl Into<String>) -> BackendResult<Self> {
        let value = value.into();

        let mut characters = value.chars();

        let Some(first) = characters.next() else {
            return Err(BackendError::invalid_artifact(
                "artifact format identifier must not be empty",
            ));
        };

        if !first.is_ascii_alphanumeric()
            || !characters.all(|character| {
                character.is_ascii_alphanumeric()
                    || matches!(character, '.' | '_' | '-' | '+')
            })
        {
            return Err(BackendError::invalid_artifact(
                "artifact format identifier contains unsupported characters",
            ));
        }

        Ok(Self(value))
    }

    /// Return the canonical identifier string as supplied by the caller.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl FromStr for ArtifactFormat {
    type Err = BackendError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

impl fmt::Display for ArtifactFormat {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// An owned artifact payload.
///
/// The payload is stored as bytes and is never normalized or silently
/// converted. Format-specific interpretation belongs to the corresponding
/// format implementation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArtifactPayload {
    encoding: ArtifactEncoding,
    bytes: Vec<u8>,
}

impl ArtifactPayload {
    /// Construct a text payload from a Rust string.
    pub fn text(value: impl Into<String>) -> Self {
        Self {
            encoding: ArtifactEncoding::Text,
            bytes: value.into().into_bytes(),
        }
    }

    /// Construct a binary payload from owned bytes.
    pub fn binary(bytes: impl Into<Vec<u8>>) -> Self {
        Self {
            encoding: ArtifactEncoding::Binary,
            bytes: bytes.into(),
        }
    }

    /// Construct a payload from bytes with an explicit encoding label.
    ///
    /// A `Text` label does not establish that the bytes are valid UTF-8.
    /// Call [`Self::as_text`] when a consumer requires UTF-8.
    pub fn from_bytes(encoding: ArtifactEncoding, bytes: Vec<u8>) -> Self {
        Self { encoding, bytes }
    }

    /// Return the encoding label.
    pub const fn encoding(&self) -> ArtifactEncoding {
        self.encoding
    }

    /// Return the payload as an immutable byte slice.
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Return the payload size in bytes.
    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    /// Whether the payload contains no bytes.
    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    /// Interpret a text-labelled payload as UTF-8 without lossy conversion.
    pub fn as_text(&self) -> BackendResult<&str> {
        if self.encoding != ArtifactEncoding::Text {
            return Err(BackendError::invalid_artifact(
                "binary artifact cannot be interpreted as text",
            ));
        }

        std::str::from_utf8(&self.bytes).map_err(|error| {
            BackendError::invalid_artifact(
                "text-labelled artifact contains invalid UTF-8",
            )
            .with_cause(error.to_string())
        })
    }

    /// Consume the payload and return its bytes.
    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }

    /// Compute a SHA-256 digest of the exact payload bytes.
    ///
    /// The digest is formatted as `sha256:` followed by lowercase hexadecimal
    /// digits. It is an integrity fingerprint, not a digital signature or
    /// proof of artifact authenticity.
    pub fn sha256_digest(&self) -> String {
        let digest = Sha256::digest(&self.bytes);
        format!("sha256:{}", hex::encode(digest))
    }
}

/// Validation evidence attached to an artifact.
///
/// Verification is explicit. Neither constructing an artifact nor computing
/// its digest proves that its format or semantics are correct.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArtifactValidation {
    /// No format or target validation has been recorded.
    Unverified,

    /// A named verifier successfully validated the artifact.
    Verified {
        /// Name of the validation tool or procedure.
        verifier: String,

        /// Optional verifier version or identity.
        verifier_version: Option<String>,
    },

    /// The applicable validation policy explicitly waived verification.
    ///
    /// This state is not equivalent to successful verification.
    NotRequired {
        /// Reason the applicable policy permits the waiver.
        reason: String,
    },
}

impl ArtifactValidation {
    /// Whether successful verification evidence is recorded.
    pub fn is_verified(&self) -> bool {
        matches!(self, Self::Verified { .. })
    }

    /// Validate the metadata associated with this state.
    pub fn validate(&self) -> BackendResult<()> {
        match self {
            Self::Unverified => Ok(()),

            Self::Verified {
                verifier,
                verifier_version,
            } => {
                if verifier.trim().is_empty() {
                    return Err(BackendError::invalid_artifact(
                        "verified artifact must identify its verifier",
                    ));
                }

                if verifier_version
                    .as_ref()
                    .is_some_and(|version| version.trim().is_empty())
                {
                    return Err(BackendError::invalid_artifact(
                        "verifier version must not be empty when supplied",
                    ));
                }

                Ok(())
            }

            Self::NotRequired { reason } => {
                if reason.trim().is_empty() {
                    return Err(BackendError::invalid_artifact(
                        "verification waiver must include a reason",
                    ));
                }

                Ok(())
            }
        }
    }
}

/// The origin and transformation history of an artifact.
///
/// Provenance is descriptive metadata. It is not itself a cryptographic
/// signature, trust decision, or proof that a recorded operation occurred.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ArtifactProvenance {
    /// Optional identifier of the producing backend implementation.
    pub backend: Option<String>,

    /// Optional version of the producing backend.
    pub backend_version: Option<String>,

    /// Optional identifier of the source or IR input.
    pub input_identity: Option<String>,

    /// Optional compilation or transformation stage.
    pub stage: Option<String>,

    /// Ordered descriptions of transformations known to have occurred.
    pub transformations: Vec<String>,
}

impl ArtifactProvenance {
    /// Validate supplied provenance fields.
    pub fn validate(&self) -> BackendResult<()> {
        for (field, value) in [
            ("backend", self.backend.as_deref()),
            ("backend version", self.backend_version.as_deref()),
            ("input identity", self.input_identity.as_deref()),
            ("stage", self.stage.as_deref()),
        ] {
            if value.is_some_and(|value| value.trim().is_empty()) {
                return Err(BackendError::invalid_artifact(format!(
                    "artifact provenance {field} must not be empty when supplied"
                )));
            }
        }

        if self
            .transformations
            .iter()
            .any(|transformation| transformation.trim().is_empty())
        {
            return Err(BackendError::invalid_artifact(
                "artifact provenance transformations must not contain empty entries",
            ));
        }

        Ok(())
    }
}

/// Complete metadata and content for one generated artifact.
///
/// Public metadata retains the established output-facing field names. The
/// format string remains extensible for compatibility with existing callers;
/// [`Self::format_id`] provides validated typed access to it.
///
/// Artifact identity is allocated by the caller using `ArtifactId`. This
/// type does not use a global mutable counter or infer identity from content.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutputArtifact {
    /// Stable artifact identifier in its owning namespace.
    pub id: ArtifactId,

    /// Identity of the target that produced the artifact.
    ///
    /// This is a target identifier or target triple, not a display label
    /// that may silently change between builds.
    pub target: String,

    /// Extensible format identifier, such as `llvm-ir` or `wasm`.
    pub format: String,

    /// Optional suggested filename extension without a leading dot or path.
    pub extension: Option<String>,

    /// Optional media type.
    pub media_type: Option<String>,

    /// Exact artifact payload.
    pub payload: ArtifactPayload,

    /// Recorded verification state.
    pub validation: ArtifactValidation,

    /// Optional SHA-256 digest in `sha256:<lowercase-hex>` format.
    ///
    /// Use [`Self::refresh_content_digest`] to calculate it and
    /// [`Self::verify_content_digest`] to verify it against the current bytes.
    pub content_digest: Option<String>,

    /// Optional provenance describing the artifact's origin.
    pub provenance: Option<ArtifactProvenance>,
}

impl OutputArtifact {
    /// Construct an artifact with required identity, target, format, and
    /// payload metadata.
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
            provenance: None,
        };

        artifact.validate_metadata()?;
        Ok(artifact)
    }

    /// Return the validated artifact format identifier.
    pub fn format_id(&self) -> BackendResult<ArtifactFormat> {
        ArtifactFormat::new(self.format.clone())
    }

    /// Return the exact payload length in bytes.
    pub fn size_bytes(&self) -> usize {
        self.payload.len()
    }

    /// Compute and store the payload's SHA-256 digest.
    ///
    /// This changes integrity metadata only; it does not modify the payload.
    pub fn refresh_content_digest(&mut self) {
        self.content_digest = Some(self.payload.sha256_digest());
    }

    /// Verify the stored digest against the current payload.
    ///
    /// Returns an error if no digest is present, if its syntax is invalid,
    /// or if it does not match the current content.
    pub fn verify_content_digest(&self) -> BackendResult<()> {
        let Some(expected) = self.content_digest.as_deref() else {
            return Err(BackendError::invalid_artifact(
                "artifact has no content digest to verify",
            ));
        };

        let Some(hex_digest) = expected.strip_prefix("sha256:") else {
            return Err(BackendError::invalid_artifact(
                "unsupported content digest algorithm; expected sha256",
            ));
        };

        if hex_digest.len() != 64
            || !hex_digest
                .bytes()
                .all(|byte| byte.is_ascii_hexdigit())
        {
            return Err(BackendError::invalid_artifact(
                "SHA-256 digest must contain exactly 64 hexadecimal digits",
            ));
        }

        let expected_normalized = hex_digest.to_ascii_lowercase();
        let actual = self.payload.sha256_digest();

        if actual != format!("sha256:{expected_normalized}") {
            return Err(BackendError::invalid_artifact(
                "artifact content does not match its recorded SHA-256 digest",
            ));
        }

        Ok(())
    }

    /// Validate artifact-local metadata and invariants.
    ///
    /// This method does not parse the payload according to its declared
    /// format, invoke external tools, or establish semantic equivalence.
    pub fn validate_metadata(&self) -> BackendResult<()> {
        if self.target.trim().is_empty() {
            return Err(BackendError::invalid_artifact(
                "artifact target must not be empty",
            ));
        }

        self.format_id()?;

        if let Some(extension) = &self.extension {
            if extension.trim().is_empty()
                || extension.starts_with('.')
                || extension.contains('/')
                || extension.contains('\\')
                || extension.contains('\0')
                || extension.contains("..")
            {
                return Err(BackendError::invalid_artifact(
                    "artifact extension must be a filename extension without a leading dot or path components",
                ));
            }
        }

        if let Some(media_type) = &self.media_type {
            if media_type.trim().is_empty()
                || media_type.contains('\n')
                || media_type.contains('\r')
            {
                return Err(BackendError::invalid_artifact(
                    "artifact media type is empty or contains invalid line breaks",
                ));
            }
        }

        self.validation.validate()?;

        if let Some(provenance) = &self.provenance {
            provenance.validate()?;
        }

        if self.content_digest.is_some() {
            self.verify_content_digest()?;
        }

        Ok(())
    }

    /// Replace the recorded validation state after an external validation
    /// stage has completed.
    ///
    /// Callers are responsible for ensuring that `Verified` is only assigned
    /// after the named verifier has actually succeeded.
    pub fn set_validation(
        &mut self,
        validation: ArtifactValidation,
    ) -> BackendResult<()> {
        validation.validate()?;
        self.validation = validation;
        Ok(())
    }

    /// Return whether the artifact has recorded successful verification.
    pub fn is_verified(&self) -> bool {
        self.validation.is_verified()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::backend::ids::{ArtifactId, IdNamespace};

    fn artifact_id(value: u128) -> ArtifactId {
        let namespace =
            IdNamespace::new("zamani.artifact").expect("valid namespace");

        ArtifactId::new(namespace, value).expect("valid artifact ID")
    }

    fn sample_artifact() -> OutputArtifact {
        OutputArtifact::new(
            artifact_id(1),
            "wasm32-unknown-unknown",
            "wasm",
            ArtifactPayload::binary(vec![0x00, 0x61, 0x73, 0x6d]),
        )
        .expect("valid artifact")
    }

    #[test]
    fn accepts_extensible_format_identifiers() {
        assert_eq!(
            ArtifactFormat::new("systemverilog").expect("valid format").as_str(),
            "systemverilog"
        );

        assert_eq!(
            ArtifactFormat::new("vendor.quantum+v2")
                .expect("valid custom format")
                .to_string(),
            "vendor.quantum+v2"
        );
    }

    #[test]
    fn rejects_invalid_format_identifiers() {
        for invalid in ["", " ", "../wasm", "wasm/text", "llvm ir", "-format"] {
            assert!(
                ArtifactFormat::new(invalid).is_err(),
                "should reject {invalid:?}"
            );
        }
    }

    #[test]
    fn text_payload_preserves_utf8() {
        let payload = ArtifactPayload::text("Zamani λ");

        assert_eq!(payload.as_text().expect("valid UTF-8"), "Zamani λ");
        assert_eq!(payload.encoding(), ArtifactEncoding::Text);
    }

    #[test]
    fn binary_payload_is_not_silently_decoded() {
        let payload = ArtifactPayload::binary(vec![0xff, 0x00, 0x80]);

        assert!(payload.as_text().is_err());
        assert_eq!(payload.as_bytes(), &[0xff, 0x00, 0x80]);
    }

    #[test]
    fn invalid_utf8_is_rejected_for_text_access() {
        let payload =
            ArtifactPayload::from_bytes(ArtifactEncoding::Text, vec![0xff]);

        assert!(payload.as_text().is_err());
    }

    #[test]
    fn digest_is_stable_for_identical_bytes() {
        let left = ArtifactPayload::binary(vec![1, 2, 3]);
        let right = ArtifactPayload::binary(vec![1, 2, 3]);

        assert_eq!(left.sha256_digest(), right.sha256_digest());
    }

    #[test]
    fn digest_changes_when_content_changes() {
        let left = ArtifactPayload::binary(vec![1, 2, 3]);
        let right = ArtifactPayload::binary(vec![1, 2, 4]);

        assert_ne!(left.sha256_digest(), right.sha256_digest());
    }

    #[test]
    fn stored_digest_verifies() {
        let mut artifact = sample_artifact();
        artifact.refresh_content_digest();

        assert!(artifact.verify_content_digest().is_ok());
        assert!(artifact.validate_metadata().is_ok());
    }

    #[test]
    fn tampered_payload_fails_digest_validation() {
        let mut artifact = sample_artifact();
        artifact.refresh_content_digest();

        artifact.payload = ArtifactPayload::binary(vec![1, 2, 3]);

        assert!(artifact.verify_content_digest().is_err());
        assert!(artifact.validate_metadata().is_err());
    }

    #[test]
    fn malformed_digest_is_rejected() {
        let mut artifact = sample_artifact();
        artifact.content_digest = Some("sha256:xyz".into());

        assert!(artifact.validate_metadata().is_err());
    }

    #[test]
    fn missing_digest_cannot_be_verified() {
        let artifact = sample_artifact();

        assert!(artifact.verify_content_digest().is_err());
    }

    #[test]
    fn artifact_requires_target_identity() {
        let result = OutputArtifact::new(
            artifact_id(1),
            " ",
            "wasm",
            ArtifactPayload::binary(vec![]),
        );

        assert!(result.is_err());
    }

    #[test]
    fn artifact_requires_a_valid_format() {
        let result = OutputArtifact::new(
            artifact_id(1),
            "wasm32-unknown-unknown",
            "../wasm",
            ArtifactPayload::binary(vec![]),
        );

        assert!(result.is_err());
    }

    #[test]
    fn extensions_cannot_contain_paths() {
        let mut artifact = sample_artifact();
        artifact.extension = Some("../artifact.wasm".into());

        assert!(artifact.validate_metadata().is_err());
    }

    #[test]
    fn verification_requires_a_verifier_identity() {
        let mut artifact = sample_artifact();

        assert!(
            artifact
                .set_validation(ArtifactValidation::Verified {
                    verifier: " ".into(),
                    verifier_version: None,
                })
                .is_err()
        );
    }

    #[test]
    fn verification_waiver_requires_a_reason() {
        let mut artifact = sample_artifact();

        assert!(
            artifact
                .set_validation(ArtifactValidation::NotRequired {
                    reason: String::new(),
                })
                .is_err()
        );
    }

    #[test]
    fn provenance_rejects_empty_supplied_fields() {
        let mut artifact = sample_artifact();

        artifact.provenance = Some(ArtifactProvenance {
            backend: Some(" ".into()),
            ..ArtifactProvenance::default()
        });

        assert!(artifact.validate_metadata().is_err());
    }

    #[test]
    fn artifact_is_unverified_by_default() {
        let artifact = sample_artifact();

        assert!(!artifact.is_verified());
        assert_eq!(artifact.validation, ArtifactValidation::Unverified);
    }

    #[test]
    fn artifact_id_is_independent_of_content_digest() {
        let mut first = sample_artifact();
        let mut second = sample_artifact();

        first.refresh_content_digest();
        second.payload = ArtifactPayload::binary(vec![9, 8, 7]);
        second.refresh_content_digest();

        assert_eq!(first.id, second.id);
        assert_ne!(first.content_digest, second.content_digest);
    }
}