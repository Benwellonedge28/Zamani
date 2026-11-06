#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani Compiler — Artifact Subsystem
//!
//! Canonical path:
//! `src/compiler/artifacts/mod.rs`
//!
//! Repository:
//! `github.com/Benwellonedge28/Zamani`
//!
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//!
//! Safety:
//! Production Rust code in this module tree must not use unsafe Rust.
//!
//! ============================================================================
//! PURPOSE
//! ============================================================================
//!
//! Establish the canonical public module boundary for Zamani compiler
//! artifacts.
//!
//! This module integrates the existing artifact implementations without
//! introducing duplicate artifact, manifest, metadata, portability,
//! serialization, or validation authorities.
//!
//! The subsystem supports compilation products from all registered
//! computational domains, including:
//!
//! - Portable intermediate representations.
//! - Classical computation.
//! - Quantum computation.
//! - Classical-quantum hybrid computation.
//! - HDL and hardware computation.
//! - AI and machine learning.
//! - Tensor computation.
//! - Distributed computation.
//! - Networking.
//! - Cryptographic computation.
//! - WebAssembly.
//! - Native executables and libraries.
//! - Accelerators.
//! - Future computational domains.
//!
//! These are examples, not a closed inventory.
//!
//! ============================================================================
//! ARCHITECTURAL AUTHORITY
//! ============================================================================
//!
//! This module must conform to:
//!
//! - grammar/DESIGN.md
//! - grammar/OWNERSHIP.yaml
//! - grammar/DEPENDENCIES.yaml
//! - grammar/MANIFEST.yaml
//! - grammar/specification/semantic-model.md
//! - grammar/specification/poco-reaf.md
//! - grammar/specification/scalability-model.md
//! - grammar/validation/semantic-boundaries.md
//! - grammar/Zamani.g4
//! - grammar/antlr/ZamaniLexer.g4
//! - grammar/antlr/ZamaniParser.g4
//! - src/zuir/
//! - src/backend/
//! - src/compiler/
//!
//! The grammar defines source syntax.
//! The semantic model defines source meaning.
//! Canonical IR implementations define computational representations.
//! Backends produce target-specific artifacts.
//! This subsystem describes, organizes, validates and serializes artifacts.
//!
//! ============================================================================
//! OWNERSHIP
//! ============================================================================
//!
//! This module owns:
//!
//! - Artifact subsystem module composition.
//! - Public module visibility.
//! - Deliberate public API re-exports.
//! - Artifact subsystem integration tests.
//!
//! It does not own:
//!
//! - Artifact construction algorithms.
//! - Artifact payload storage implementations.
//! - Artifact digest algorithms.
//! - Metadata storage.
//! - Manifest dependency algorithms.
//! - Portability classification algorithms.
//! - Wire serialization algorithms.
//! - Semantic verification.
//! - IR verification.
//! - Backend selection.
//! - Physical hardware allocation.
//! - Linking and relocation.
//! - Runtime execution.
//!
//! These responsibilities remain with their authoritative modules.
//!
//! ============================================================================
//! DEPENDENCY DIRECTION
//! ============================================================================
//!
//! ```text
//!
//!                  compiler/artifacts/mod.rs
//!                            |
//!             +--------------+--------------+
//!             |              |              |
//!             v              v              v
//!         artifact.rs    manifest.rs    metadata.rs
//!             |              |              |
//!             +--------------+--------------+
//!                            |
//!              +-------------+-------------+
//!              |                           |
//!              v                           v
//!        portability.rs              validation.rs
//!              |                           |
//!              +-------------+-------------+
//!                            |
//!                            v
//!                     serialization.rs
//!
//! ```
//!
//! The diagram describes subsystem relationships, not permission for
//! child modules to depend on this facade's re-export aliases.
//!
//! Child implementations must use their explicit sibling module paths.
//!
//! ============================================================================
//! SCALABILITY
//! ============================================================================
//!
//! This module introduces no fixed maximum for:
//!
//! - Artifact count.
//! - Manifest count.
//! - Dependency count.
//! - Artifact payload size.
//! - Computing domains.
//! - Hardware architectures.
//! - Processors.
//! - Accelerators.
//! - Quantum processors.
//! - Qubits.
//! - Distributed nodes.
//! - Tensor rank.
//! - Register width.
//! - Metadata entries.
//! - Identifier lengths.
//!
//! Collections and payloads grow according to available resources.
//!
//! Optional operational budgets belong to the relevant operations,
//! particularly serialization, validation, storage and compilation.
//!
//! No finite implementation can guarantee literal infinity.
//! Actual operation remains subject to address space, memory, storage,
//! processing time, operating-system limits and physical resources.
//!
//! The language and artifact model must not introduce arbitrary
//! universal hardware-capacity ceilings.
//!
//! ============================================================================
//! POCO-REAF
//! ============================================================================
//!
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever.
//!
//! A portable artifact and a target-specific realization are distinct
//! compilation products.
//!
//! An artifact without a target identifier is not automatically portable.
//!
//! A portable declaration is not equivalent to verified portability.
//!
//! A valid digest is not proof of semantic correctness.
//!
//! A successfully decoded artifact is not automatically safe to execute.
//!
//! The appropriate verification, capability, portability, security,
//! backend and runtime authorities must establish their respective
//! guarantees.
//!
//! ============================================================================
//! INTEGRATION CONTRACT
//! ============================================================================
//!
//! artifact.rs:
//!     Canonical artifact identity, payload, descriptor and provenance.
//!
//! manifest.rs:
//!     Canonical artifact collection and dependency relationships.
//!
//! metadata.rs:
//!     Typed metadata operations and configurable metadata policies.
//!
//! portability.rs:
//!     Artifact and manifest portability assessment.
//!
//! validation.rs:
//!     Artifact and manifest validation orchestration.
//!
//! serialization.rs:
//!     Stable versioned artifact and manifest wire representation.
//!
//! src/backend/:
//!     Produces target-specific artifact payloads.
//!
//! src/zuir/:
//!     Supplies canonical computational representations.
//!
//! src/compiler/linker.rs:
//!     Consumes artifact and dependency information for linking.
//!
//! src/compiler/incremental.rs:
//!     Uses stable artifact identities and compilation dependencies
//!     where appropriate.
//!
//! src/runtime/:
//!     Resolves external payloads and handles execution.
//!
//! ============================================================================
//! COMPATIBILITY
//! ============================================================================
//!
//! Existing child module paths remain public.
//!
//! Root-level exports are deliberate and explicit.
//!
//! No wildcard re-exports are used.
//!
//! Existing artifact structures and their serialization contracts
//! remain authoritative.
//!
//! Adding a future artifact format must not require modifying this
//! facade unless a new public API export is intentionally introduced.
//!
//! ============================================================================
//! FREEZE CONDITIONS
//! ============================================================================
//!
//! - Every child module resolves.
//! - Every public re-export resolves.
//! - No duplicate artifact authority is introduced.
//! - No unsafe code is introduced.
//! - No arbitrary hardware ceiling is introduced.
//! - Existing artifact tests pass.
//! - Artifact serialization tests pass.
//! - Manifest validation tests pass.
//! - Portability assessment tests pass.
//! - Integration tests compile.
//! - Rust 1.97 or later is supported.


// ============================================================================
// 1. Canonical artifact model
// ============================================================================

/// Canonical artifact identity, descriptors, payloads, digests,
/// metadata storage and provenance.
pub mod artifact;


// ============================================================================
// 2. Artifact manifests
// ============================================================================

/// Canonical artifact collections, dependencies, roots and
/// manifest structural validation.
pub mod manifest;


// ============================================================================
// 3. Artifact metadata
// ============================================================================

/// Namespaced metadata access, typed values, validation policies
/// and conflict-aware metadata merging.
pub mod metadata;


// ============================================================================
// 4. Artifact portability
// ============================================================================

/// Artifact portability declarations, evidence, classifications
/// and manifest portability assessment.
pub mod portability;


// ============================================================================
// 5. Artifact validation
// ============================================================================

/// Artifact and manifest validation policies, findings,
/// reports and validation orchestration.
pub mod validation;


// ============================================================================
// 6. Artifact serialization
// ============================================================================

/// Versioned artifact and manifest serialization, decoding,
/// configurable input/output budgets and reader/writer interfaces.
pub mod serialization;


// ============================================================================
// PUBLIC API — ARTIFACT MODEL
// ============================================================================

pub use self::artifact::{
    Artifact,
    ArtifactContentId,
    ArtifactDescriptor,
    ArtifactDigest,
    ArtifactError,
    ArtifactFormatId,
    ArtifactIdentifier,
    ArtifactId,
    ArtifactKindId,
    ArtifactMetadata,
    ArtifactPayload,
    ArtifactPayloadRef,
    ArtifactProducerId,
    ArtifactProvenance,
    ArtifactResult,
    ArtifactStoreId,
    ArtifactTargetRef,
    DigestAlgorithm,
    ARTIFACT_MODEL_API_VERSION,
    SHA256_ALGORITHM,
};


// ============================================================================
// PUBLIC API — MANIFESTS
// ============================================================================

pub use self::manifest::{
    ArtifactDependency,
    ArtifactManifest,
    DependencyKind,
    ManifestError,
    ManifestId,
    ManifestResult,
    ManifestVersion,
    ARTIFACT_MANIFEST_SCHEMA_ID,
    ARTIFACT_MANIFEST_SCHEMA_MAJOR,
    ARTIFACT_MANIFEST_SCHEMA_MINOR,
    ARTIFACT_MANIFEST_SCHEMA_PATCH,
    CURRENT_ARTIFACT_MANIFEST_VERSION,
};


// ============================================================================
// PUBLIC API — METADATA
// ============================================================================

pub use self::metadata::{
    merge_metadata,
    validate_metadata,
    MetadataAccess,
    MetadataError,
    MetadataIssue,
    MetadataIssueKind,
    MetadataIssueSeverity,
    MetadataKey,
    MetadataMergeReport,
    MetadataMergeStrategy,
    MetadataNamespace,
    MetadataResult,
    MetadataValidationPolicy,
    MetadataValidationReport,
    ARTIFACT_METADATA_NAMESPACE,
    COMPILER_METADATA_NAMESPACE,
    METADATA_CONTRACT_VERSION,
    METADATA_SCHEMA_NAMESPACE,
};


// ============================================================================
// PUBLIC API — PORTABILITY
// ============================================================================

pub use self::portability::{
    assess_artifact,
    assess_manifest,
    ArtifactPortability,
    ManifestPortabilityInputs,
    ManifestPortabilityReport,
    PortabilityAuthorityId,
    PortabilityClass,
    PortabilityCounts,
    PortabilityDeclaration,
    PortabilityError,
    PortabilityEvidence,
    PortabilityEvidenceRef,
    PortabilityPolicy,
    PortabilityResult,
    PORTABILITY_CONTRACT_ID,
    PORTABILITY_CONTRACT_MAJOR,
    PORTABILITY_CONTRACT_MINOR,
    PORTABILITY_CONTRACT_PATCH,
};


// ============================================================================
// PUBLIC API — VALIDATION
// ============================================================================

pub use self::validation::{
    validate_artifact,
    validate_artifact_with_portability,
    validate_manifest,
    validate_manifest_with_portability,
    ArtifactValidationPolicy,
    ManifestValidationPolicy,
    ValidationError,
    ValidationIssue,
    ValidationIssueKind,
    ValidationReport,
    ValidationResult,
    ValidationSeverity,
    ValidationStatus,
    ValidationSubject,
    ARTIFACT_VALIDATION_CONTRACT_ID,
    ARTIFACT_VALIDATION_CONTRACT_MAJOR,
    ARTIFACT_VALIDATION_CONTRACT_MINOR,
    ARTIFACT_VALIDATION_CONTRACT_PATCH,
};


// ============================================================================
// PUBLIC API — SERIALIZATION
// ============================================================================

pub use self::serialization::{
    decode_artifact,
    decode_artifact_from_reader,
    decode_artifact_with_options,
    decode_manifest,
    decode_manifest_from_reader,
    decode_manifest_with_options,
    encode_artifact,
    encode_artifact_to_writer,
    encode_artifact_with_options,
    encode_manifest,
    encode_manifest_to_writer,
    encode_manifest_with_options,
    DecodeOptions,
    EncodeOptions,
    EncodedDocument,
    SerializationError,
    SerializationResult,
    SerializationVersion,
    ARTIFACT_DOCUMENT_SCHEMA_ID,
    CURRENT_SERIALIZATION_FORMAT_VERSION,
    MANIFEST_DOCUMENT_SCHEMA_ID,
    SERIALIZATION_FORMAT_MAJOR,
    SERIALIZATION_FORMAT_MINOR,
    SERIALIZATION_FORMAT_PATCH,
};


// ============================================================================
// FACADE TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn artifact_facade_exposes_canonical_artifact_identity() {
        let id = ArtifactId::new("artifact.example")
            .expect("valid artifact identifier");

        assert_eq!(id.as_str(), "artifact.example");
    }

    #[test]
    fn artifact_facade_exposes_content_integrity() {
        let payload = b"zamani-artifact";

        let digest = ArtifactDigest::sha256(payload);

        assert!(digest.verify(payload).is_ok());

        assert!(digest.verify(b"different-content").is_err());
    }

    #[test]
    fn artifact_facade_exposes_manifest_versioning() {
        let version = ManifestVersion::current();

        assert_eq!(
            version,
            CURRENT_ARTIFACT_MANIFEST_VERSION
        );

        assert!(version.supports(version));
    }

    #[test]
    fn artifact_facade_exposes_serialization_options() {
        let options = EncodeOptions::new()
            .with_pretty_output(true);

        assert!(options.is_pretty());
        assert_eq!(options.max_output_bytes(), None);

        let decode_options = DecodeOptions::new();

        assert_eq!(decode_options.max_input_bytes(), None);
    }

    #[test]
    fn artifact_facade_exposes_portability_classification() {
        let class = PortabilityClass::Unknown;

        assert!(class.is_unknown());
        assert!(!class.is_verified_portable());
    }

    #[test]
    fn artifact_facade_exposes_validation_states() {
        assert!(ValidationStatus::Valid.is_valid());

        assert!(ValidationStatus::Invalid.is_invalid());

        assert!(ValidationStatus::Incomplete.is_incomplete());
    }

    #[test]
    fn artifact_facade_preserves_open_ended_artifact_identifiers() {
        let kind = ArtifactKindId::new(
            "future-computational-domain"
        )
        .expect("future artifact kinds must be representable");

        assert_eq!(
            kind.as_str(),
            "future-computational-domain"
        );
    }

    #[test]
    fn artifact_facade_exposes_canonical_metadata_storage() {
        let mut metadata = ArtifactMetadata::new();

        metadata
            .insert(
                "zamani.compiler.example",
                "portable"
            )
            .expect("valid metadata");

        assert_eq!(
            metadata.get("zamani.compiler.example"),
            Some("portable")
        );
    }

    #[test]
    fn artifact_facade_has_no_implicit_serialization_budget() {
        let encode = EncodeOptions::default();
        let decode = DecodeOptions::default();

        assert_eq!(encode.max_output_bytes(), None);
        assert_eq!(decode.max_input_bytes(), None);
    }
}