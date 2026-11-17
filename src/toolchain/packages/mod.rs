
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Toolchain — Package Management
//!
//! Path: `src/toolchain/packages/mod.rs`
//! Edition: Rust 2021.
//! Minimum Rust version: 1.97.
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! This module is the composition root and public API boundary for
//! Zamani package management.
//!
//! It coordinates the existing package model, archive, creation,
//! extraction, installation, metadata and verification subsystems.
//!
//! Package management is domain-neutral. It supports packages containing
//! classical, quantum, HDL, AI, tensor, hybrid, distributed, networking,
//! cryptographic, embedded, accelerator and future-domain content.
//!
//! ## Architectural ownership
//!
//! This module owns:
//!
//! - Package subsystem composition.
//! - Public package API boundaries.
//! - Explicit package API re-exports.
//! - Integration of canonical package-manifest decoding with the
//!   package-verification contract.
//!
//! It does not own:
//!
//! - Package identity allocation.
//! - Package naming or dependency naming.
//! - Package release-version interpretation.
//! - Package declaration parsing.
//! - Project manifest parsing.
//! - Dependency resolution.
//! - Archive serialization.
//! - Package content hashing.
//! - Package extraction mechanics.
//! - Package installation mechanics.
//! - Cryptographic algorithms.
//! - Signature verification.
//! - Publisher authentication.
//! - Trust decisions.
//! - Registry communication.
//! - Compiler execution.
//! - Backend selection.
//! - Hardware discovery.
//! - Runtime scheduling or execution.
//!
//! Those responsibilities remain with their authoritative modules.
//!
//! ## Canonical authorities
//!
//! Package identity:
//! `crate::toolchain::ids::PackageId`
//!
//! Package names:
//! `crate::toolchain::dependencies::model::DependencyName`
//!
//! Version schemes:
//! `crate::toolchain::dependencies::constraints::VersionSchemeId`
//!
//! Package declarations:
//! `grammar/modules/packages.g4`
//!
//! Dependency declarations:
//! `grammar/modules/dependencies.g4`
//!
//! Project manifest:
//! `crate::zamani_project_config`
//!
//! Package model:
//! `packages::model`
//!
//! Package manifest serialization:
//! `packages::metadata`
//!
//! Archive framing:
//! `packages::archive`
//!
//! Package integrity and authenticity:
//! `packages::verification` and the canonical security subsystem.
//!
//! ## Package lifecycle
//!
//! ```text
//! Canonical project manifest
//!           |
//!           v
//!      Manifest adapter
//!           |
//!           v
//!  Dependency resolution
//!           |
//!           v
//!    Package creation
//!           |
//!           v
//!    Package model
//!           |
//!           v
//!   Package metadata
//!           |
//!           v
//!     .zpkg archive
//!           |
//!           v
//! Package verification
//!           |
//!           v
//!   Package extraction
//!           |
//!           v
//!  Package installation
//!           |
//!           v
//!   Registry / consumers
//! ```
//!
//! This represents the logical lifecycle. Individual operations may
//! use different entry points, and package verification is mandatory
//! wherever the applicable security policy requires it.
//!
//! ## Ownership boundaries
//!
//! The package subsystem is distinct from:
//!
//! - Compiler intermediate representations.
//! - Compiler compilation graphs.
//! - Build-task graphs.
//! - Dependency-resolution graphs.
//! - Backend artifact generation.
//! - Physical hardware allocation.
//! - Runtime execution.
//!
//! Packages describe distributable content and its declared properties.
//! They do not determine where or how that content executes.
//!
//! ## POCO-REAF
//!
//! Package identity and package contents remain independent of the
//! physical machine on which a program will eventually execute.
//!
//! Package artifacts may describe different realizations, including:
//!
//! - Native executables and libraries.
//! - WebAssembly.
//! - Quantum circuits.
//! - HDL designs.
//! - AI models and datasets.
//! - Firmware.
//! - Distributed application components.
//! - Future artifact formats.
//!
//! Target-specific compatibility is represented through extensible
//! metadata and compatibility requirements.
//!
//! The package subsystem does not claim that every artifact can execute
//! on every machine. Actual compatibility and feasibility remain the
//! responsibility of the appropriate compiler, backend, capability,
//! hardware and runtime authorities.
//!
//! ## Scalability
//!
//! This composition root introduces no arbitrary maximum on:
//!
//! - Packages.
//! - Package content entries.
//! - Package artifacts.
//! - Package metadata.
//! - Dependencies.
//! - Registries.
//! - Computing domains.
//! - Target descriptions.
//! - Hardware resources.
//! - Distributed participants.
//!
//! Collections and processing grow according to available resources.
//!
//! Operational budgets are supplied by the relevant operation,
//! configuration and security policies.
//!
//! A configured budget is not a language or hardware capability limit.
//!
//! Literal infinite storage or computation is not promised.
//!
//! ## Security
//!
//! This module never treats successful parsing as proof of integrity,
//! authenticity, publisher trust, compatibility or execution feasibility.
//!
//! Cryptographic operations and trust decisions must be delegated to
//! their canonical authorities.
//!
//! Unknown or indeterminate verification results must never be
//! silently interpreted as successful verification.
//!
//! ## Integration
//!
//! Upstream:
//!
//! - `crate::zamani_project_config`
//! - `crate::toolchain::dependencies`
//! - `crate::compiler::artifacts`
//! - `crate::backend`
//! - `crate::toolchain::security`
//!
//! Downstream:
//!
//! - `crate::toolchain::registries`
//! - `crate::toolchain::build`
//! - `crate::toolchain::commands`
//! - `crate::toolchain::deployment`
//!
//! This module does not implement those integrations itself.
//!
//! ## Implementation status
//!
//! The package validation implementation is not yet present as a
//! separate `validation.rs` file in the inspected repository.
//!
//! Existing structural validation remains available through:
//!
//! - `Package::validate()`
//! - `PackageMetadata::validate()`
//! - `PackageManifest::validate()`
//!
//! Archive integrity and authenticity orchestration remain available
//! through `packages::verification`.
//!
//! A separate validation module must only be introduced after its
//! responsibility and integration contract have been defined.


// ============================================================================
// PACKAGE SUBMODULES
// ============================================================================

/// Canonical package archive format and streaming `.zpkg` reader/writer.
pub mod archive;

/// Explicit project-content selection and package creation snapshots.
pub mod creation;

/// Secure, staged and budget-aware package extraction.
pub mod extraction;

/// Verified package installation and destination publication.
pub mod installation;

/// Canonical package-manifest serialization and deserialization.
pub mod metadata;

/// Canonical package identity, metadata, content and artifact models.
pub mod model;

/// Archive integrity, authenticity and trust-verification orchestration.
pub mod verification;


// ============================================================================
// CANONICAL PACKAGE MODEL API
// ============================================================================

/// Canonical package model types.
///
/// These types describe package identity, metadata, content inventory,
/// artifacts and compatibility declarations.
///
/// They do not perform filesystem, network or cryptographic operations.
pub use self::model::{
    ArtifactProvenanceReference,
    ArtifactSignatureReference,
    ContentDigest,
    Package,
    PackageArtifact,
    PackageCompatibilityRequirement,
    PackageContent,
    PackageContentKind,
    PackageIdentity,
    PackageMetadata,
    PackageMetadataValue,
    PackageOrigin,
    PackagePath,
    PackageType,
    PackageVersion,
};


// ============================================================================
// PACKAGE CREATION API
// ============================================================================

/// Package creation errors and results.
pub use self::creation::{
    PackageCreationError,
    PackageCreationResult,
};

/// Cooperative package-creation cancellation.
pub use self::creation::PackageCancellation;

/// Content digest descriptor used during package creation.
pub use self::creation::PackageContentDigest;

/// Extensible content digest provider contract.
pub use self::creation::ContentDigestProvider;

/// Initial SHA-256 content digest provider.
pub use self::creation::Sha256DigestProvider;

/// Package source provenance input.
pub use self::creation::PackageSourceReference;

/// Package identity input supplied to package creation.
pub use self::creation::PackageIdentityInput;

/// Resolved dependency snapshot supplied to package creation.
pub use self::creation::ResolvedDependencySnapshot;

/// Artifact compatibility declaration supplied during creation.
pub use self::creation::ArtifactCompatibilityClaim;

/// Artifact descriptor supplied during package creation.
pub use self::creation::PackageArtifactDescriptor;

/// Package provenance input.
pub use self::creation::PackageProvenanceInput;

/// Explicit package-file selection.
pub use self::creation::PackageFileSpec;

/// Configurable package-creation operational policies.
pub use self::creation::PackageCreationPolicy;

/// Immutable package-creation request.
pub use self::creation::PackageCreationRequest;

/// Package-file creation record.
pub use self::creation::PackageFileRecord;

/// Package draft manifest.
pub use self::creation::PackageDraftManifest;

/// Open package payload handles.
pub use self::creation::PackagePayload;

/// Completed package-creation snapshot.
pub use self::creation::PackageDraft;

/// Stateless package creator.
pub use self::creation::PackageCreator;

/// Metadata value used specifically by the creation subsystem.
///
/// `model::PackageMetadataValue` remains the canonical package-model
/// metadata value. This alias prevents the two independently defined
/// types from colliding in the public package namespace.
pub use self::creation::PackageMetadataValue as PackageCreationMetadataValue;


// ============================================================================
// PACKAGE ARCHIVE API
// ============================================================================

/// Archive signature.
pub use self::archive::ARCHIVE_MAGIC;

/// Supported archive major version.
pub use self::archive::ARCHIVE_FORMAT_MAJOR;

/// Supported archive minor version.
pub use self::archive::ARCHIVE_FORMAT_MINOR;

/// Default archive I/O buffer size.
///
/// This controls I/O granularity, not maximum package size.
pub use self::archive::DEFAULT_IO_BUFFER_SIZE;

/// Version of the `.zpkg` archive format.
pub use self::archive::ArchiveFormatVersion;

/// Configurable archive I/O options.
pub use self::archive::ArchiveIoOptions;

/// Validated archive header.
pub use self::archive::ArchiveHeader;

/// Summary of a successfully written archive.
pub use self::archive::ArchiveWriteSummary;

/// Streaming archive reader.
pub use self::archive::ArchiveReader;

/// Streaming archive-entry reader.
pub use self::archive::ArchiveEntryReader;

/// Write a package archive using the default I/O options.
pub use self::archive::write_archive;

/// Write a package archive using explicit I/O options.
pub use self::archive::write_archive_with_options;


// ============================================================================
// PACKAGE METADATA API
// ============================================================================

/// Canonical package-manifest format identifier.
pub use self::metadata::PACKAGE_MANIFEST_FORMAT;

/// Canonical package-manifest schema version.
pub use self::metadata::PACKAGE_MANIFEST_SCHEMA_VERSION;

/// Canonical package manifest.
pub use self::metadata::PackageManifest;

/// Operational options for reading package manifests.
pub use self::metadata::ManifestReadOptions;

/// Encode a canonical package model.
pub use self::metadata::encode_package;

/// Encode a complete package manifest.
pub use self::metadata::encode_manifest;

/// Write a package manifest directly to a writer.
pub use self::metadata::write_manifest;

/// Decode a complete package manifest.
pub use self::metadata::decode_manifest;

/// Decode a package without discarding extension data.
pub use self::metadata::decode_package;

/// Read a declared manifest length from a stream.
pub use self::metadata::read_manifest;


// ============================================================================
// PACKAGE EXTRACTION API
// ============================================================================

/// Configurable extraction budgets.
pub use self::extraction::ExtractionBudgets;

/// Extraction operation options.
pub use self::extraction::ExtractionOptions;

/// Required extraction verification contract.
pub use self::extraction::ExtractionVerifier;

/// Successful extraction report.
pub use self::extraction::ExtractionReport;

/// Extract an archive using default extraction options.
pub use self::extraction::extract_archive;

/// Extract an archive using explicit extraction options.
pub use self::extraction::extract_archive_with_options;


// ============================================================================
// PACKAGE INSTALLATION API
// ============================================================================

/// Immutable package-installation request.
pub use self::installation::PackageInstallRequest;

/// Package-installation options.
pub use self::installation::PackageInstallOptions;

/// Mandatory package-installation verification contract.
pub use self::installation::PackageInstallationVerifier;

/// Successful package-installation report.
pub use self::installation::PackageInstallReport;

/// Install a verified package.
pub use self::installation::install_package;

/// Install a verified package using explicit extraction options.
pub use self::installation::install_package_with_options;


// ============================================================================
// PACKAGE VERIFICATION API
// ============================================================================

/// Package-verification operational options.
pub use self::verification::VerificationOptions;

/// Canonical package-manifest decoder contract.
pub use self::verification::PackageManifestDecoder;

/// Package authenticity and trust-verification contract.
pub use self::verification::PackageEvidenceVerifier;

/// Authenticity and trust verification outcome.
pub use self::verification::EvidenceVerification;

/// Package content-integrity status.
pub use self::verification::PackageIntegrityStatus;

/// Successful package-verification report.
pub use self::verification::PackageVerificationReport;

/// Verify a complete package archive.
pub use self::verification::verify_archive;


// ============================================================================
// CANONICAL MANIFEST DECODER ADAPTER
// ============================================================================

/// Adapter connecting the canonical package-manifest implementation
/// with the package-verification decoder contract.
///
/// The adapter delegates all parsing, schema validation, duplicate-key
/// detection and package-model construction to `packages::metadata`.
///
/// It does not implement a second manifest parser.
///
/// Extension-bearing manifests are rejected by `decode_package`
/// rather than silently discarding their extension data.
///
/// Consumers that need extension-preserving verification must implement
/// `PackageManifestDecoder` using a decoder and extension-policy contract
/// that retains and validates the complete manifest.
#[derive(Debug, Default, Clone, Copy)]
pub struct CanonicalPackageManifestDecoder;

impl PackageManifestDecoder for CanonicalPackageManifestDecoder {
    /// Decode and structurally validate a canonical package manifest.
    ///
    /// The canonical metadata subsystem remains the sole decoder.
    ///
    /// # Errors
    ///
    /// Returns an error for malformed, incompatible or extension-bearing
    /// manifests, or when the reconstructed package model is invalid.
    fn decode_and_validate(
        &mut self,
        bytes: &[u8],
    ) -> crate::toolchain::result::Result<Package> {
        self::metadata::decode_package(bytes)
    }
}


// ============================================================================
// PACKAGE API TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// Ensure the canonical decoder satisfies the verification contract.
    fn assert_manifest_decoder<T: PackageManifestDecoder>() {}

    #[test]
    fn canonical_manifest_decoder_implements_verification_contract() {
        assert_manifest_decoder::<CanonicalPackageManifestDecoder>();
    }

    #[test]
    fn package_model_is_the_canonical_metadata_authority() {
        let metadata = PackageMetadata::new();

        assert!(metadata.is_empty());
        assert!(metadata.validate().is_ok());
    }

    #[test]
    fn package_manifest_schema_is_explicit() {
        assert!(!PACKAGE_MANIFEST_FORMAT.is_empty());
        assert!(!PACKAGE_MANIFEST_SCHEMA_VERSION.is_empty());
    }

    #[test]
    fn archive_format_has_an_explicit_version() {
        let version = ArchiveFormatVersion::current();

        assert_eq!(version.major(), ARCHIVE_FORMAT_MAJOR);
        assert_eq!(version.minor(), ARCHIVE_FORMAT_MINOR);
    }

    #[test]
    fn archive_io_default_is_nonzero() {
        assert!(DEFAULT_IO_BUFFER_SIZE > 0);
    }

    #[test]
    fn extraction_defaults_do_not_impose_universal_budgets() {
        let budgets = ExtractionBudgets::default();

        assert_eq!(budgets.max_entries(), None);
        assert_eq!(budgets.max_manifest_bytes(), None);
        assert_eq!(budgets.max_entry_bytes(), None);
        assert_eq!(budgets.max_total_content_bytes(), None);
    }

    #[test]
    fn package_metadata_types_remain_distinguishable() {
        fn accepts_model_metadata(
            _: PackageMetadataValue,
        ) {
        }

        fn accepts_creation_metadata(
            _: PackageCreationMetadataValue,
        ) {
        }

        let model_value =
            PackageMetadataValue::boolean(true);

        let creation_value =
            PackageCreationMetadataValue::Boolean(true);

        accepts_model_metadata(model_value);
        accepts_creation_metadata(creation_value);
    }
}
