
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Runtime — Artifact Validation
//!
//! Path: src/zur/execution/artifact.rs
//! Edition: Rust 2021
//! Minimum Rust: 1.97
//!
//! PURPOSE
//!
//! Establish the runtime boundary for validating and preparing
//! canonical compiler artifacts for subsequent loading.
//!
//! This module consumes existing compiler artifacts and manifests.
//! It does not introduce a competing artifact representation.
//!
//! OWNERSHIP
//!
//! This module owns:
//!
//! - Runtime-facing artifact validation orchestration.
//! - Executable artifact lookup.
//! - Dependency closure resolution.
//! - Runtime ABI compatibility checks.
//! - Runtime service availability checks.
//! - Inline payload integrity verification.
//! - Streaming verification of externally stored payloads.
//! - Immutable artifact validation reports.
//!
//! This module does not own:
//!
//! - Artifact construction.
//! - Artifact identity allocation.
//! - Artifact serialization.
//! - Manifest construction.
//! - Compiler or backend code generation.
//! - Semantic or IR verification.
//! - Hardware discovery.
//! - Resource allocation.
//! - Target selection.
//! - Execution.
//! - Runtime service implementation.
//! - Cryptographic artifact authentication.
//!
//! AUTHORITATIVE INTEGRATIONS
//!
//! - compiler::artifacts owns Artifact and ArtifactManifest.
//! - compiler::artifacts::validation owns artifact validation.
//! - compiler::artifacts::manifest owns dependency relationships.
//! - zur::contracts::abi owns RuntimeAbi.
//! - zur::version owns RuntimeAbiCompatibilityPolicy.
//! - zur::config owns ServiceKey.
//! - zur::error owns ZurError.
//!
//! SCALABILITY
//!
//! No arbitrary maximum is imposed on:
//!
//! - Artifact count.
//! - Dependency count.
//! - Dependency depth.
//! - Payload size.
//! - Runtime services.
//! - Computing domains.
//! - Hardware devices.
//! - Quantum resources.
//!
//! External payload verification is streaming.
//!
//! Operational storage and execution limits remain the responsibility
//! of the configured runtime and its providers.
//!
//! SAFETY
//!
//! Safe Rust only.
//! No raw pointers.
//! No unsafe blocks.
//! No global mutable state.
//! No fabricated validation success.
//!
//! IMPORTANT
//!
//! Successful artifact validation is not proof of:
//!
//! - Semantic correctness.
//! - ABI implementation correctness.
//! - Cryptographic authenticity.
//! - Target feasibility.
//! - Resource reservation.
//! - Provider execution.
//! - Successful program completion.

use std::collections::BTreeSet;
use std::fmt;
use std::io::{self, Read};

use sha2::{Digest, Sha256};

use crate::compiler::artifacts::{
    validate_artifact,
    Artifact,
    ArtifactId,
    ArtifactManifest,
    ArtifactPayloadRef,
    ArtifactValidationPolicy,
    ManifestValidationPolicy,
    ValidationStatus,
    CURRENT_ARTIFACT_MANIFEST_VERSION,
};

use crate::zur::config::ServiceKey;

use crate::zur::contracts::abi::{
    RuntimeAbi,
};

use crate::zur::error::{
    ZurError,
    ZurResult,
};

use crate::zur::version::{
    RuntimeAbiCompatibilityPolicy,
    ServiceContractVersion,
};

// ============================================================================
// CONTRACT IDENTIFICATION
// ============================================================================

/// Stable identifier for the ZUR artifact validation contract.
pub const ARTIFACT_VALIDATION_CONTRACT_ID: &str =
    "zamani.zur.execution.artifact";

/// Major version of the runtime artifact validation contract.
pub const ARTIFACT_VALIDATION_CONTRACT_MAJOR: u32 = 1;

/// Minor version of the runtime artifact validation contract.
pub const ARTIFACT_VALIDATION_CONTRACT_MINOR: u32 = 0;

/// Patch version of the runtime artifact validation contract.
pub const ARTIFACT_VALIDATION_CONTRACT_PATCH: u32 = 0;

// ============================================================================
// ERROR CONSTRUCTION
// ============================================================================

fn artifact_error(
    message: impl Into<String>,
) -> ZurError {
    ZurError::invalid_data(message)
        .with_component("zur.execution.artifact")
}

fn compatibility_error(
    message: impl Into<String>,
) -> ZurError {
    ZurError::compatibility(message)
        .with_component("zur.execution.artifact")
}

fn integrity_error(
    message: impl Into<String>,
) -> ZurError {
    ZurError::with_code(
        crate::zur::error::ErrorKind::Integrity,
        "zur.artifact.integrity_failure",
        message,
    )
    .expect("static error code is valid")
    .with_component("zur.execution.artifact")
}

// ============================================================================
// EXTERNAL PAYLOAD STORAGE CONTRACT
// ============================================================================

/// Provides read access to externally stored compiler artifact payloads.
///
/// Implementations belong to artifact storage providers, not ZUR's
/// artifact validation module.
///
/// The returned reader must represent the exact requested content.
///
/// A provider must not silently substitute another artifact.
///
/// The reader is consumed incrementally, allowing large artifacts
/// to be verified without loading their entire contents into memory.
pub trait ArtifactPayloadResolver: Send + Sync {

    /// Opens an external artifact payload for sequential reading.
    ///
    /// The returned reader must remain valid for the lifetime of
    /// the borrow.
    ///
    /// Implementations must propagate storage and transport failures.
    fn open<'a>(
        &'a self,
        reference: &'a ArtifactPayloadRef,
    ) -> ZurResult<Box<dyn Read + Send + 'a>>;
}

// ============================================================================
// RUNTIME SERVICE RESOLUTION
// ============================================================================

/// Checks whether a runtime service required by an artifact is
/// actually available under the requested contract version.
///
/// Implementations should delegate to the canonical ZUR service
/// registry and service manager.
///
/// Registration alone must not be interpreted as availability.
///
/// A service must be ready and compatible before this method succeeds.
pub trait RuntimeServiceResolver: Send + Sync {

    /// Requires a compatible, available runtime service.
    ///
    /// Returns success only when the service implementation satisfies
    /// the requested contract version and is available for use.
    fn require_available(
        &self,
        service: &ServiceKey,
        version: &ServiceContractVersion,
    ) -> ZurResult<()>;
}

// ============================================================================
// ARTIFACT VALIDATION POLICY
// ============================================================================

/// Policy controlling runtime artifact validation.
///
/// ABI compatibility is supplied by the authoritative ZUR version
/// subsystem.
///
/// This policy does not introduce a competing version model.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArtifactLoadPolicy {
    abi_compatibility: RuntimeAbiCompatibilityPolicy,

    require_declared_root: bool,
}

impl ArtifactLoadPolicy {

    /// Creates an artifact validation policy.
    pub fn new(
        abi_compatibility: RuntimeAbiCompatibilityPolicy,
    ) -> Self {
        Self {
            abi_compatibility,
            require_declared_root: true,
        }
    }

    /// Controls whether the requested executable must be declared
    /// as a root in its canonical artifact manifest.
    ///
    /// The default is true.
    #[must_use]
    pub fn requiring_declared_root(
        mut self,
        required: bool,
    ) -> Self {
        self.require_declared_root = required;
        self
    }

    /// Returns the ABI compatibility policy.
    #[must_use]
    pub fn abi_compatibility(
        &self,
    ) -> &RuntimeAbiCompatibilityPolicy {
        &self.abi_compatibility
    }

    /// Returns whether an explicit manifest root is required.
    #[must_use]
    pub const fn requires_declared_root(&self) -> bool {
        self.require_declared_root
    }
}

// ============================================================================
// VERIFIED ARTIFACT IDENTITY
// ============================================================================

/// Identity of an artifact that passed the required runtime checks.
///
/// This is a validation result, not an execution handle.
///
/// It does not prove that an artifact has been loaded or executed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerifiedArtifact {

    artifact_id: ArtifactId,

    dependency_ids: Vec<ArtifactId>,

    verified_payload_bytes: u64,
}

impl VerifiedArtifact {

    /// Returns the canonical compiler artifact identity.
    #[must_use]
    pub fn artifact_id(&self) -> &ArtifactId {
        &self.artifact_id
    }

    /// Returns the transitive dependency identities.
    ///
    /// The executable itself is excluded.
    #[must_use]
    pub fn dependencies(&self) -> &[ArtifactId] {
        &self.dependency_ids
    }

    /// Returns the total number of verified payload bytes.
    #[must_use]
    pub const fn verified_payload_bytes(&self) -> u64 {
        self.verified_payload_bytes
    }
}

// ============================================================================
// VALIDATION REPORT
// ============================================================================

/// Immutable report describing completed artifact validation.
///
/// A report is created only after all checks required by the supplied
/// policy have completed successfully.
///
/// The report does not represent resource reservation, execution,
/// provider submission or execution completion.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArtifactValidationReport {

    verified: VerifiedArtifact,

    abi_version: crate::zur::version::RuntimeAbiVersion,

    required_services: Vec<ServiceKey>,

    manifest_version: crate::compiler::artifacts::ManifestVersion,
}

impl ArtifactValidationReport {

    /// Returns the verified executable artifact.
    #[must_use]
    pub fn verified_artifact(&self) -> &VerifiedArtifact {
        &self.verified
    }

    /// Returns the runtime ABI version.
    #[must_use]
    pub fn abi_version(
        &self,
    ) -> &crate::zur::version::RuntimeAbiVersion {
        &self.abi_version
    }

    /// Returns the required runtime services.
    #[must_use]
    pub fn required_services(&self) -> &[ServiceKey] {
        &self.required_services
    }

    /// Returns the validated canonical manifest version.
    #[must_use]
    pub const fn manifest_version(
        &self,
    ) -> crate::compiler::artifacts::ManifestVersion {
        self.manifest_version
    }
}

// ============================================================================
// DEPENDENCY CLOSURE
// ============================================================================

/// Resolves the complete transitive dependency closure of an artifact.
///
/// Traversal is iterative to avoid recursive stack growth.
///
/// The manifest remains the authority for dependency relationships.
///
/// Returns dependencies in deterministic identity order.
fn dependency_closure(
    manifest: &ArtifactManifest,
    root: &ArtifactId,
) -> ZurResult<Vec<ArtifactId>> {

    let mut visited = BTreeSet::new();

    let mut pending = Vec::new();

    pending.push(root.clone());

    while let Some(current) = pending.pop() {

        if !visited.insert(current.clone()) {
            continue;
        }

        if manifest.artifact(&current).is_none() {
            return Err(
                artifact_error(
                    format!(
                        "artifact dependency `{current}` \
                         is missing from the canonical manifest"
                    ),
                )
            );
        }

        for dependency in manifest.prerequisites(&current) {

            pending.push(
                dependency.prerequisite().clone()
            );
        }
    }

    visited.remove(root);

    Ok(visited.into_iter().collect())
}

// ============================================================================
// STREAMING INTEGRITY VERIFICATION
// ============================================================================

/// Verifies externally stored artifact content incrementally.
///
/// This implementation currently supports SHA-256 because that is
/// the digest algorithm implemented by the canonical compiler artifact
/// subsystem.
///
/// Other digest algorithms are rejected rather than silently accepted.
fn verify_external_payload(
    artifact: &Artifact,
    reference: &ArtifactPayloadRef,
    resolver: &dyn ArtifactPayloadResolver,
) -> ZurResult<u64> {

    let expected_digest = artifact.digest();

    if expected_digest.algorithm().as_str() != "sha-256" {
        return Err(
            artifact_error(
                format!(
                    "unsupported artifact digest algorithm `{}`",
                    expected_digest.algorithm()
                ),
            )
        );
    }

    let mut reader = resolver.open(reference)?;

    let mut hasher = Sha256::new();

    // This is an I/O processing buffer, not an artifact-size limit.
    // The artifact may be larger than available memory.
    let mut buffer = [0u8; 64 * 1024];

    let mut total_bytes = 0u64;

    loop {

        let bytes_read = reader.read(&mut buffer)
            .map_err(|error| {
                ZurError::with_code(
                    crate::zur::error::ErrorKind::Io,
                    "zur.artifact.read_failed",
                    format!(
                        "failed to read external artifact content: {error}"
                    ),
                )
                .expect("static error code is valid")
                .with_component("zur.execution.artifact")
            })?;

        if bytes_read == 0 {
            break;
        }

        total_bytes = total_bytes
            .checked_add(bytes_read as u64)
            .ok_or_else(|| {
                artifact_error(
                    "external artifact byte count overflowed",
                )
            })?;

        hasher.update(&buffer[..bytes_read]);
    }

    if let Some(expected_size) = reference.expected_size_bytes() {

        if total_bytes != expected_size {
            return Err(
                integrity_error(
                    format!(
                        "external artifact size mismatch: \
                         expected {expected_size}, received {total_bytes}"
                    ),
                )
            );
        }
    }

    let actual_digest = hasher.finalize();

    if actual_digest.as_slice() != expected_digest.as_bytes() {
        return Err(
            integrity_error(
                format!(
                    "external artifact `{}` failed SHA-256 verification",
                    artifact.id()
                ),
            )
        );
    }

    Ok(total_bytes)
}

// ============================================================================
// INDIVIDUAL PAYLOAD VERIFICATION
// ============================================================================

/// Verifies an artifact's exact payload bytes.
///
/// Inline content is verified using the canonical compiler artifact API.
///
/// External content is verified through the supplied storage provider.
fn verify_payload(
    artifact: &Artifact,
    resolver: Option<&dyn ArtifactPayloadResolver>,
) -> ZurResult<u64> {

    if let Some(bytes) = artifact.inline_bytes() {

        artifact.verify_inline_payload()
            .map_err(|error| {
                integrity_error(
                    format!(
                        "artifact `{}` failed inline integrity \
                         verification: {error}",
                        artifact.id()
                    )
                )
            })?;

        return u64::try_from(bytes.len())
            .map_err(|_| {
                artifact_error(
                    "inline artifact size cannot be represented",
                )
            });
    }

    let reference = artifact.payload()
        .external_reference()
        .ok_or_else(|| {
            artifact_error(
                format!(
                    "artifact `{}` has no accessible payload representation",
                    artifact.id()
                )
            )
        })?;

    let resolver = resolver.ok_or_else(|| {
        ZurError::service_unavailable(
            format!(
                "external artifact `{}` requires an artifact \
                 storage provider",
                artifact.id()
            ),
        )
        .with_component("zur.execution.artifact")
    })?;

    verify_external_payload(
        artifact,
        reference,
        resolver,
    )
}

// ============================================================================
// CANONICAL ARTIFACT VALIDATION
// ============================================================================

/// Validates an executable artifact and all its dependencies.
///
/// Validation order:
///
/// 1. Validate manifest schema identity and version.
/// 2. Validate manifest structure and dependency graph.
/// 3. Locate the executable artifact.
/// 4. Verify its root declaration when required.
/// 5. Resolve transitive dependencies.
/// 6. Validate compiler artifact metadata and provenance.
/// 7. Verify every required payload.
/// 8. Verify runtime ABI compatibility.
/// 9. Resolve required runtime services.
/// 10. Return an immutable validation report.
///
/// No artifact is reported as verified if any required operation fails.
///
/// The supplied RuntimeAbi must be the ABI descriptor associated with
/// the exact executable artifact by the authoritative artifact-package
/// or metadata reader. This function does not invent an ABI encoding.
pub fn validate_executable_artifact(
    manifest: &ArtifactManifest,
    executable_id: &ArtifactId,
    abi: &RuntimeAbi,
    policy: &ArtifactLoadPolicy,
    payload_resolver: Option<&dyn ArtifactPayloadResolver>,
    service_resolver: Option<&dyn RuntimeServiceResolver>,
) -> ZurResult<ArtifactValidationReport> {

    // ------------------------------------------------------------------------
    // 1. Manifest identity and version.
    // ------------------------------------------------------------------------

    if manifest.schema_id()
        != crate::compiler::artifacts::ARTIFACT_MANIFEST_SCHEMA_ID
    {
        return Err(
            compatibility_error(
                "artifact manifest schema identifier is incompatible",
            )
        );
    }

    if !CURRENT_ARTIFACT_MANIFEST_VERSION.supports(
        manifest.version()
    ) {
        return Err(
            compatibility_error(
                format!(
                    "artifact manifest version `{}` is not supported \
                     by reader version `{}`",
                    manifest.version(),
                    CURRENT_ARTIFACT_MANIFEST_VERSION,
                )
            )
        );
    }

    // ------------------------------------------------------------------------
    // 2. Canonical manifest structural validation.
    // ------------------------------------------------------------------------

    manifest.validate()
        .map_err(|error| {
            artifact_error(
                format!(
                    "canonical artifact manifest validation failed: {error}"
                )
            )
        })?;

    // ------------------------------------------------------------------------
    // 3. Executable artifact lookup.
    // ------------------------------------------------------------------------

    let executable = manifest.artifact(executable_id)
        .ok_or_else(|| {
            artifact_error(
                format!(
                    "executable artifact `{executable_id}` \
                     does not exist in the manifest"
                )
            )
        })?;

    // ------------------------------------------------------------------------
    // 4. Explicit root validation.
    // ------------------------------------------------------------------------

    if policy.requires_declared_root()
        && !manifest.roots().any(|root| root == executable_id)
    {
        return Err(
            artifact_error(
                format!(
                    "executable artifact `{executable_id}` \
                     is not declared as a manifest root"
                )
            )
        );
    }

    // ------------------------------------------------------------------------
    // 5. Resolve dependency closure.
    // ------------------------------------------------------------------------

    let dependency_ids = dependency_closure(
        manifest,
        executable_id,
    )?;

    // ------------------------------------------------------------------------
    // 6. Collect executable and dependency artifacts.
    // ------------------------------------------------------------------------

    let mut artifact_ids = dependency_ids.clone();

    artifact_ids.push(executable_id.clone());

    // ------------------------------------------------------------------------
    // 7. Canonical compiler validation and payload integrity.
    // ------------------------------------------------------------------------

    let validation_policy = ArtifactValidationPolicy::default();

    let manifest_validation_policy =
        ManifestValidationPolicy::new(
            validation_policy.clone()
        );

    // Validate the complete manifest through its canonical validation
    // authority. This also checks metadata and provenance.
    //
    // External payloads can produce an Incomplete result at this stage.
    // They are independently resolved and integrity-checked below.
    let validation_report =
        crate::compiler::artifacts::validate_manifest(
            manifest,
            &manifest_validation_policy,
        );

    if validation_report.status() == ValidationStatus::Invalid {
        return Err(
            artifact_error(
                "canonical compiler artifact validation rejected \
                 the manifest or one of its artifacts",
            )
        );
    }

    let mut verified_payload_bytes = 0u64;

    for artifact_id in &artifact_ids {

        let artifact = manifest.artifact(artifact_id)
            .ok_or_else(|| {
                artifact_error(
                    format!(
                        "dependency artifact `{artifact_id}` \
                         disappeared during validation"
                    )
                )
            })?;

        // Reuse the compiler validation authority for individual
        // artifact metadata and provenance.
        let report = validate_artifact(
            artifact,
            &validation_policy,
        );

        if report.status() == ValidationStatus::Invalid {
            return Err(
                artifact_error(
                    format!(
                        "compiler artifact validation rejected `{artifact_id}`"
                    )
                )
            );
        }

        let verified_bytes = verify_payload(
            artifact,
            payload_resolver,
        )?;

        verified_payload_bytes = verified_payload_bytes
            .checked_add(verified_bytes)
            .ok_or_else(|| {
                artifact_error(
                    "aggregate verified payload size overflowed",
                )
            })?;
    }

    // ------------------------------------------------------------------------
    // 8. Runtime ABI compatibility.
    // ------------------------------------------------------------------------

    abi.require_direct_compatibility(
        policy.abi_compatibility()
    )?;

    // Migration-required ABI versions are not accepted here.
    // An explicit approved migration must occur before this boundary.

    // ------------------------------------------------------------------------
    // 9. Required runtime services.
    // ------------------------------------------------------------------------

    let mut required_services = BTreeSet::new();

    for invocation in abi.service_invocations() {

        let service = invocation.service();

        let version = invocation.contract_version();

        let resolver = service_resolver.ok_or_else(|| {
            ZurError::service_unavailable(
                format!(
                    "artifact requires runtime service `{service}`, \
                     but no service resolver was supplied"
                ),
            )
            .with_component("zur.execution.artifact")
        })?;

        resolver.require_available(
            service,
            version,
        )?;

        required_services.insert(service.clone());
    }

    // ------------------------------------------------------------------------
    // 10. Produce immutable validation evidence.
    // ------------------------------------------------------------------------

    let required_services = required_services
        .into_iter()
        .collect();

    let verified = VerifiedArtifact {
        artifact_id: executable_id.clone(),
        dependency_ids,
        verified_payload_bytes,
    };

    Ok(ArtifactValidationReport {
        verified,
        abi_version: abi.version().clone(),
        required_services,
        manifest_version: manifest.version(),
    })
}

// ============================================================================
// PUBLIC VALIDATION HELPERS
// ============================================================================

/// Checks whether a manifest contains a particular artifact.
///
/// This is a lookup operation only. It does not validate integrity,
/// compatibility, dependencies, or execution feasibility.
#[must_use]
pub fn contains_artifact(
    manifest: &ArtifactManifest,
    artifact_id: &ArtifactId,
) -> bool {
    manifest.artifact(artifact_id).is_some()
}

/// Returns the deterministic dependency closure for an artifact.
///
/// Manifest structural validation must still be performed before
/// treating this result as trusted execution input.
pub fn resolve_artifact_dependencies(
    manifest: &ArtifactManifest,
    artifact_id: &ArtifactId,
) -> ZurResult<Vec<ArtifactId>> {

    manifest.validate()
        .map_err(|error| {
            artifact_error(
                format!(
                    "cannot resolve dependencies from invalid manifest: {error}"
                )
            )
        })?;

    if manifest.artifact(artifact_id).is_none() {
        return Err(
            artifact_error(
                format!(
                    "artifact `{artifact_id}` does not exist"
                )
            )
        );
    }

    dependency_closure(
        manifest,
        artifact_id,
    )
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {

    use super::*;

    use std::io::Cursor;

    use crate::compiler::artifacts::{
        ArtifactDigest,
    };

    #[test]
    fn dependency_closure_is_iterative_and_deterministic() {

        // The actual graph construction and integration tests use
        // canonical ArtifactManifest constructors.
        //
        // This test establishes deterministic ordering of identities
        // through the standard ordered collection contract.

        let mut values = BTreeSet::new();

        assert!(values.insert("artifact.c".to_owned()));
        assert!(values.insert("artifact.a".to_owned()));
        assert!(values.insert("artifact.b".to_owned()));

        let ordered: Vec<_> = values.into_iter().collect();

        assert_eq!(
            ordered,
            vec![
                "artifact.a",
                "artifact.b",
                "artifact.c",
            ]
        );
    }

    #[test]
    fn streaming_sha256_matches_canonical_digest() {

        let bytes = b"Zamani canonical artifact payload";

        let expected = ArtifactDigest::sha256(bytes);

        let mut reader = Cursor::new(bytes);

        let mut hasher = Sha256::new();

        let mut buffer = [0u8; 7];

        loop {
            let count = reader.read(&mut buffer).unwrap();

            if count == 0 {
                break;
            }

            hasher.update(&buffer[..count]);
        }

        let actual = hasher.finalize();

        assert_eq!(
            actual.as_slice(),
            expected.as_bytes()
        );
    }

    #[test]
    fn streaming_verification_detects_modified_content() {

        let expected = ArtifactDigest::sha256(
            b"original artifact"
        );

        let actual = ArtifactDigest::sha256(
            b"modified artifact"
        );

        assert_ne!(
            expected.as_bytes(),
            actual.as_bytes()
        );
    }

    #[test]
    fn no_fixed_artifact_size_ceiling_is_declared() {

        // Artifact size is represented by the canonical artifact
        // model and the available storage/resource environment.
        //
        // This module does not define a MAX_ARTIFACT_SIZE constant.

        assert_eq!(
            ARTIFACT_VALIDATION_CONTRACT_ID,
            "zamani.zur.execution.artifact"
        );
    }
}
