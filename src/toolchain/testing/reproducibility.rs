// src/toolchain/testing/reproducibility.rs

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Toolchain — Reproducibility Verification
//!
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Verify reproducibility evidence for actual Zamani build operations.
//!
//! This module verifies whether independent build observations agree
//! under the same declared logical and target-realization inputs.
//!
//! It supports:
//!
//! - Deterministic build-input identity.
//! - Logical build identity.
//! - Target-specific realization identity.
//! - Canonical evidence collection.
//! - Compiler and backend identity evidence.
//! - Grammar, semantic and IR schema evidence.
//! - Dependency and lockfile evidence.
//! - Configuration and environment evidence.
//! - External provider and toolchain evidence.
//! - Artifact content identity.
//! - Artifact validation status.
//! - Provenance evidence.
//! - Multi-target reproducibility.
//! - Open-ended computing domains.
//! - Structured reproducibility reports.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Reproducibility evidence requirements.
//! - Immutable reproducibility snapshots.
//! - Canonical evidence ordering.
//! - Reproducibility snapshot fingerprints.
//! - Comparison of independent build observations.
//! - Reproducibility classifications.
//! - Structured discrepancy reporting.
//!
//! It does not own:
//!
//! - Source parsing or grammar interpretation.
//! - Semantic analysis.
//! - Compilation-input discovery.
//! - Compiler fingerprint generation.
//! - Backend fingerprint generation.
//! - External tool discovery.
//! - Dependency resolution.
//! - Lockfile generation.
//! - Environment discovery.
//! - Artifact production.
//! - Artifact storage.
//! - Artifact digest generation.
//! - Artifact payload verification.
//! - Digital signatures.
//! - Provenance authentication.
//! - Semantic equivalence verification.
//! - Target selection or feasibility.
//! - Physical resource allocation.
//! - Runtime execution.
//!
//! Those responsibilities remain with their canonical authorities.
//!
//! ## Canonical integrations
//!
//! Fingerprinting:
//!     crate::zuir::fingerprint::stable_hash
//!
//! Build-input identity:
//!     crate::toolchain::build::inputs
//!
//! Build outputs:
//!     crate::toolchain::build::outputs
//!
//! Canonical artifact model:
//!     crate::compiler::artifacts::artifact
//!
//! Artifact verification:
//!     crate::compiler::artifacts::validation
//!
//! Compiler reproducibility:
//!     crate::compiler::reproducibility
//!
//! Backend reproducibility:
//!     crate::backend::reproducibility
//!
//! External toolchain identity:
//!     crate::backend::toolchain
//!
//! Provenance:
//!     crate::toolchain::security
//!
//! Canonical errors:
//!     crate::toolchain::errors
//!
//! ## Important distinction
//!
//! Matching build fingerprints do not establish that two builds
//! actually produced identical artifacts.
//!
//! Matching artifact fingerprints do not establish semantic
//! equivalence, authenticity or execution safety.
//!
//! Every claim must be supported by the appropriate authority.
//!
//! ## POCO-REAF
//!
//! One logical project can produce multiple target-specific artifacts.
//!
//! The logical project identity remains distinct from target-specific
//! realization identities.
//!
//! Different targets may legitimately produce different artifact bytes.
//!
//! This module compares realizations only when their declared
//! realization identities are compatible.
//!
//! It never requires different hardware architectures to produce
//! byte-identical artifacts.
//!
//! ## Scalability
//!
//! This module imposes no arbitrary universal ceilings on:
//!
//! - Projects.
//! - Build inputs.
//! - Evidence records.
//! - Evidence categories.
//! - Computing domains.
//! - Providers.
//! - Targets.
//! - Artifacts.
//! - Distributed participants.
//! - Hardware resources.
//! - Quantum resources.
//! - Tensor dimensions.
//!
//! Collections grow according to available resources and Rust's
//! representational limits.
//!
//! Allocation failures are reported rather than silently ignored.
//!
//! Literal infinite memory, storage or computation is not promised.
//!
//! ## Determinism
//!
//! Canonical ordering is established before snapshot fingerprinting.
//!
//! No timestamps, random numbers, hostnames, process IDs, memory
//! addresses or unordered hash-map iteration are included.
//!
//! The same canonical snapshot produces the same fingerprint.
//!
//! ## Security
//!
//! Fingerprints are identifiers, not signatures.
//!
//! Evidence must originate from the relevant authoritative subsystem.
//!
//! Untrusted evidence must not be promoted to verified evidence.
//!
//! Credentials, private keys, passwords, tokens and secret values
//! must never be included in fingerprint records.
//!
//! ## Completion
//!
//! A reproducibility comparison is successful only when:
//!
//! - Both snapshots are structurally valid.
//! - Logical build identities match.
//! - Target-realization identities match.
//! - All required evidence is present.
//! - Required evidence is verified.
//! - Artifact identities match.
//! - Artifact formats and target associations match.
//! - Artifact content fingerprints match.
//! - Artifact sizes match.
//! - No authoritative validation has rejected an artifact.
//!
//! Missing evidence produces an incomplete result.
//!
//! Known differences produce a non-reproducible result.
//!
//! Rejected evidence produces an invalid result.

use std::cmp::Ordering;
use std::fmt;

use crate::toolchain::errors::ToolchainError;
use crate::toolchain::result::ToolchainResult;

use crate::zuir::fingerprint::stable_hash::{
    Fingerprint,
    FingerprintError,
    StableHashBuilder,
};

// ============================================================================
// CONTRACT VERSION
// ============================================================================

/// Public API version of the reproducibility verification contract.
pub const REPRODUCIBILITY_API_VERSION: u16 = 1;

/// Version of the reproducibility snapshot encoding schema.
///
/// Increment when the canonical field selection, ordering or interpretation
/// changes incompatibly.
pub const REPRODUCIBILITY_SCHEMA_VERSION: u32 = 1;

/// Canonical fingerprint domain for a reproducibility snapshot.
pub const REPRODUCIBILITY_DOMAIN: &str =
    "zamani.toolchain.testing.reproducibility.v1";

/// Stable operation identifier for reproducibility verification.
pub const REPRODUCIBILITY_OPERATION: &str =
    "toolchain.testing.reproducibility";

// ============================================================================
// ERROR CONSTRUCTION
// ============================================================================

fn invalid_input(message: impl Into<String>) -> ToolchainError {
    ToolchainError::validation_failed(message)
        .with_operation(REPRODUCIBILITY_OPERATION)
}

fn resource_error(
    description: &str,
    error: impl fmt::Display,
) -> ToolchainError {
    ToolchainError::resource_exhausted(format!(
        "unable to allocate memory for {description}: {error}"
    ))
    .with_operation(REPRODUCIBILITY_OPERATION)
}

fn fingerprint_error(error: FingerprintError) -> ToolchainError {
    ToolchainError::validation_failed(format!(
        "canonical reproducibility fingerprint construction failed: {error}"
    ))
    .with_operation(REPRODUCIBILITY_OPERATION)
}

fn validate_identifier(
    value: &str,
    description: &'static str,
) -> ToolchainResult<()> {
    if value.trim().is_empty() {
        return Err(invalid_input(format!(
            "{description} must not be empty"
        )));
    }

    if value.chars().any(char::is_control) {
        return Err(invalid_input(format!(
            "{description} must not contain control characters"
        )));
    }

    Ok(())
}

// ============================================================================
// EVIDENCE STATUS
// ============================================================================

/// Verification state of evidence supplied by an authoritative subsystem.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[non_exhaustive]
pub enum EvidenceStatus {
    /// The owning authority completed its required verification.
    Verified,

    /// Evidence exists, but at least one required check remains unresolved.
    Incomplete,

    /// The owning authority identified an invalid condition.
    Rejected,
}

impl EvidenceStatus {
    /// Returns true only when evidence has been verified.
    #[must_use]
    pub const fn is_verified(self) -> bool {
        matches!(self, Self::Verified)
    }
}

// ============================================================================
// EVIDENCE REQUIREMENTS
// ============================================================================

/// Identifies one required category of reproducibility evidence.
///
/// Category and authority identifiers are open-ended strings.
///
/// This permits new compiler domains, providers, hardware systems,
/// grammars and future computational technologies to participate
/// without changing this module.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EvidenceRequirement {
    category: String,
    authority: String,
}

impl EvidenceRequirement {
    /// Construct a required evidence category.
    pub fn new(
        category: impl Into<String>,
        authority: impl Into<String>,
    ) -> ToolchainResult<Self> {
        let category = category.into();
        let authority = authority.into();

        validate_identifier(&category, "evidence category")?;
        validate_identifier(&authority, "evidence authority")?;

        Ok(Self {
            category,
            authority,
        })
    }

    /// Return the evidence category.
    #[must_use]
    pub fn category(&self) -> &str {
        &self.category
    }

    /// Return the authoritative subsystem identifier.
    #[must_use]
    pub fn authority(&self) -> &str {
        &self.authority
    }
}

fn compare_requirement(
    left: &EvidenceRequirement,
    right: &EvidenceRequirement,
) -> Ordering {
    left.category
        .cmp(&right.category)
        .then_with(|| left.authority.cmp(&right.authority))
}

// ============================================================================
// EVIDENCE RECORDS
// ============================================================================

/// One independently supplied reproducibility evidence record.
///
/// The fingerprint must originate from the named authority.
///
/// This type does not calculate or authenticate that fingerprint.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvidenceRecord {
    category: String,
    authority: String,
    fingerprint: Fingerprint,
    status: EvidenceStatus,
}

impl EvidenceRecord {
    /// Construct an evidence record.
    ///
    /// The caller must obtain the fingerprint from the declared authority.
    pub fn new(
        category: impl Into<String>,
        authority: impl Into<String>,
        fingerprint: Fingerprint,
        status: EvidenceStatus,
    ) -> ToolchainResult<Self> {
        let category = category.into();
        let authority = authority.into();

        validate_identifier(&category, "evidence category")?;
        validate_identifier(&authority, "evidence authority")?;

        Ok(Self {
            category,
            authority,
            fingerprint,
            status,
        })
    }

    /// Return the evidence category.
    #[must_use]
    pub fn category(&self) -> &str {
        &self.category
    }

    /// Return the authority that supplied the evidence.
    #[must_use]
    pub fn authority(&self) -> &str {
        &self.authority
    }

    /// Return the evidence fingerprint.
    #[must_use]
    pub const fn fingerprint(&self) -> Fingerprint {
        self.fingerprint
    }

    /// Return the evidence verification state.
    #[must_use]
    pub const fn status(&self) -> EvidenceStatus {
        self.status
    }
}

fn compare_evidence(
    left: &EvidenceRecord,
    right: &EvidenceRecord,
) -> Ordering {
    left.category
        .cmp(&right.category)
        .then_with(|| left.authority.cmp(&right.authority))
}

// ============================================================================
// ARTIFACT EVIDENCE
// ============================================================================

/// Independently verified information about one generated artifact.
///
/// The artifact fingerprint and size must originate from the canonical
/// artifact and validation authorities.
///
/// One reproducibility snapshot describes one target realization.
/// A multi-target build therefore has multiple target-specific snapshots
/// associated with the same logical build identity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArtifactEvidence {
    artifact_id: String,
    artifact_kind: String,
    artifact_format: String,
    target_id: String,
    content_fingerprint: Fingerprint,
    content_size_bytes: u64,
    validation_status: EvidenceStatus,
}

impl ArtifactEvidence {
    /// Construct an artifact evidence record.
    ///
    /// `content_fingerprint` must identify the exact artifact bytes.
    ///
    /// `validation_status` must reflect the actual canonical artifact
    /// validation outcome, not the caller's assumption.
    pub fn new(
        artifact_id: impl Into<String>,
        artifact_kind: impl Into<String>,
        artifact_format: impl Into<String>,
        target_id: impl Into<String>,
        content_fingerprint: Fingerprint,
        content_size_bytes: u64,
        validation_status: EvidenceStatus,
    ) -> ToolchainResult<Self> {
        let artifact_id = artifact_id.into();
        let artifact_kind = artifact_kind.into();
        let artifact_format = artifact_format.into();
        let target_id = target_id.into();

        validate_identifier(&artifact_id, "artifact identity")?;
        validate_identifier(&artifact_kind, "artifact kind")?;
        validate_identifier(&artifact_format, "artifact format")?;
        validate_identifier(&target_id, "artifact target identity")?;

        Ok(Self {
            artifact_id,
            artifact_kind,
            artifact_format,
            target_id,
            content_fingerprint,
            content_size_bytes,
            validation_status,
        })
    }

    /// Return the canonical artifact identity.
    #[must_use]
    pub fn artifact_id(&self) -> &str {
        &self.artifact_id
    }

    /// Return the artifact kind.
    #[must_use]
    pub fn artifact_kind(&self) -> &str {
        &self.artifact_kind
    }

    /// Return the artifact format.
    #[must_use]
    pub fn artifact_format(&self) -> &str {
        &self.artifact_format
    }

    /// Return the target identity.
    #[must_use]
    pub fn target_id(&self) -> &str {
        &self.target_id
    }

    /// Return the exact artifact content fingerprint.
    #[must_use]
    pub const fn content_fingerprint(&self) -> Fingerprint {
        self.content_fingerprint
    }

    /// Return the artifact size in bytes.
    #[must_use]
    pub const fn content_size_bytes(&self) -> u64 {
        self.content_size_bytes
    }

    /// Return the canonical validation state.
    #[must_use]
    pub const fn validation_status(&self) -> EvidenceStatus {
        self.validation_status
    }
}

fn compare_artifacts(
    left: &ArtifactEvidence,
    right: &ArtifactEvidence,
) -> Ordering {
    left.target_id
        .cmp(&right.target_id)
        .then_with(|| left.artifact_id.cmp(&right.artifact_id))
}

// ============================================================================
// REPRODUCIBILITY SNAPSHOT
// ============================================================================

/// Immutable evidence snapshot for one actual build realization.
///
/// A snapshot separates:
///
/// - Target-independent logical build identity.
/// - Target-specific realization identity.
/// - Required evidence.
/// - Supplied evidence.
/// - Produced artifacts.
///
/// All collections are stored in canonical ordering.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReproducibilitySnapshot {
    logical_build_fingerprint: Fingerprint,
    realization_id: String,
    target_id: String,
    requirements: Vec<EvidenceRequirement>,
    evidence: Vec<EvidenceRecord>,
    artifacts: Vec<ArtifactEvidence>,
}

impl ReproducibilitySnapshot {
    /// Construct and validate a reproducibility snapshot.
    ///
    /// The logical build fingerprint must be obtained from the canonical
    /// build-input/build-identity authority.
    ///
    /// The realization identifier must identify the complete target-specific
    /// realization context.
    pub fn new(
        logical_build_fingerprint: Fingerprint,
        realization_id: impl Into<String>,
        target_id: impl Into<String>,
        mut requirements: Vec<EvidenceRequirement>,
        mut evidence: Vec<EvidenceRecord>,
        mut artifacts: Vec<ArtifactEvidence>,
    ) -> ToolchainResult<Self> {
        let realization_id = realization_id.into();
        let target_id = target_id.into();

        validate_identifier(&realization_id, "realization identity")?;
        validate_identifier(&target_id, "target identity")?;

        requirements.sort_unstable_by(compare_requirement);
        evidence.sort_unstable_by(compare_evidence);
        artifacts.sort_unstable_by(compare_artifacts);

        validate_unique_requirements(&requirements)?;
        validate_unique_evidence(&evidence)?;
        validate_unique_artifacts(&artifacts)?;

        for artifact in &artifacts {
            if artifact.target_id != target_id {
                return Err(invalid_input(format!(
                    "artifact '{}' belongs to target '{}', but the snapshot \
                     describes target '{}'",
                    artifact.artifact_id,
                    artifact.target_id,
                    target_id
                )));
            }
        }

        Ok(Self {
            logical_build_fingerprint,
            realization_id,
            target_id,
            requirements,
            evidence,
            artifacts,
        })
    }

    /// Return the target-independent logical build fingerprint.
    #[must_use]
    pub const fn logical_build_fingerprint(&self) -> Fingerprint {
        self.logical_build_fingerprint
    }

    /// Return the target-specific realization identity.
    #[must_use]
    pub fn realization_id(&self) -> &str {
        &self.realization_id
    }

    /// Return the target identity.
    #[must_use]
    pub fn target_id(&self) -> &str {
        &self.target_id
    }

    /// Return required evidence.
    #[must_use]
    pub fn requirements(&self) -> &[EvidenceRequirement] {
        &self.requirements
    }

    /// Return supplied evidence.
    #[must_use]
    pub fn evidence(&self) -> &[EvidenceRecord] {
        &self.evidence
    }

    /// Return artifact evidence.
    #[must_use]
    pub fn artifacts(&self) -> &[ArtifactEvidence] {
        &self.artifacts
    }

    /// Calculate the deterministic fingerprint of the complete snapshot.
    ///
    /// This uses the repository's canonical ZUIR stable-hash implementation.
    ///
    /// The fingerprint is an identity, not proof of authenticity.
    pub fn fingerprint(&self) -> ToolchainResult<Fingerprint> {
        let mut builder =
            StableHashBuilder::new(REPRODUCIBILITY_DOMAIN)
                .map_err(fingerprint_error)?;

        builder
            .field_u32(
                "reproducibility-schema",
                REPRODUCIBILITY_SCHEMA_VERSION,
            )
            .map_err(fingerprint_error)?;

        builder
            .field_fingerprint(
                "logical-build",
                self.logical_build_fingerprint,
            )
            .map_err(fingerprint_error)?;

        builder
            .field_str("realization-id", &self.realization_id)
            .map_err(fingerprint_error)?;

        builder
            .field_str("target-id", &self.target_id)
            .map_err(fingerprint_error)?;

        let requirement_count =
            u64::try_from(self.requirements.len()).map_err(|_| {
                invalid_input("evidence requirement count is not representable")
            })?;

        builder
            .field_u64("requirement-count", requirement_count)
            .map_err(fingerprint_error)?;

        for requirement in &self.requirements {
            builder
                .field_str(
                    "requirement-category",
                    &requirement.category,
                )
                .map_err(fingerprint_error)?;

            builder
                .field_str(
                    "requirement-authority",
                    &requirement.authority,
                )
                .map_err(fingerprint_error)?;
        }

        let evidence_count =
            u64::try_from(self.evidence.len()).map_err(|_| {
                invalid_input("evidence count is not representable")
            })?;

        builder
            .field_u64("evidence-count", evidence_count)
            .map_err(fingerprint_error)?;

        for record in &self.evidence {
            builder
                .field_str("evidence-category", &record.category)
                .map_err(fingerprint_error)?;

            builder
                .field_str("evidence-authority", &record.authority)
                .map_err(fingerprint_error)?;

            builder
                .field_fingerprint(
                    "evidence-fingerprint",
                    record.fingerprint,
                )
                .map_err(fingerprint_error)?;

            builder
                .field_u8(
                    "evidence-status",
                    evidence_status_code(record.status),
                )
                .map_err(fingerprint_error)?;
        }

        let artifact_count =
            u64::try_from(self.artifacts.len()).map_err(|_| {
                invalid_input("artifact count is not representable")
            })?;

        builder
            .field_u64("artifact-count", artifact_count)
            .map_err(fingerprint_error)?;

        for artifact in &self.artifacts {
            builder
                .field_str("artifact-id", &artifact.artifact_id)
                .map_err(fingerprint_error)?;

            builder
                .field_str("artifact-kind", &artifact.artifact_kind)
                .map_err(fingerprint_error)?;

            builder
                .field_str("artifact-format", &artifact.artifact_format)
                .map_err(fingerprint_error)?;

            builder
                .field_str("artifact-target", &artifact.target_id)
                .map_err(fingerprint_error)?;

            builder
                .field_fingerprint(
                    "artifact-content",
                    artifact.content_fingerprint,
                )
                .map_err(fingerprint_error)?;

            builder
                .field_u64(
                    "artifact-size",
                    artifact.content_size_bytes,
                )
                .map_err(fingerprint_error)?;

            builder
                .field_u8(
                    "artifact-validation",
                    evidence_status_code(artifact.validation_status),
                )
                .map_err(fingerprint_error)?;
        }

        Ok(builder.finish())
    }

    /// Determine whether all required evidence is present and verified.
    ///
    /// This checks evidence completeness, not authenticity.
    #[must_use]
    pub fn evidence_is_complete(&self) -> bool {
        self.requirements.iter().all(|requirement| {
            self.evidence.iter().any(|record| {
                record.category == requirement.category
                    && record.authority == requirement.authority
                    && record.status.is_verified()
            })
        })
    }

    /// Determine whether any supplied evidence has been rejected.
    #[must_use]
    pub fn has_rejected_evidence(&self) -> bool {
        self.evidence
            .iter()
            .any(|record| record.status == EvidenceStatus::Rejected)
            || self.artifacts.iter().any(|artifact| {
                artifact.validation_status == EvidenceStatus::Rejected
            })
    }

    /// Determine whether any evidence or artifact remains incomplete.
    #[must_use]
    pub fn has_incomplete_evidence(&self) -> bool {
        !self.evidence_is_complete()
            || self.evidence.iter().any(|record| {
                record.status == EvidenceStatus::Incomplete
            })
            || self.artifacts.iter().any(|artifact| {
                artifact.validation_status == EvidenceStatus::Incomplete
            })
    }
}

fn evidence_status_code(status: EvidenceStatus) -> u8 {
    match status {
        EvidenceStatus::Verified => 1,
        EvidenceStatus::Incomplete => 2,
        EvidenceStatus::Rejected => 3,
    }
}

fn validate_unique_requirements(
    requirements: &[EvidenceRequirement],
) -> ToolchainResult<()> {
    for pair in requirements.windows(2) {
        if compare_requirement(&pair[0], &pair[1]) == Ordering::Equal {
            return Err(invalid_input(format!(
                "duplicate evidence requirement: {}/{}",
                pair[0].category,
                pair[0].authority
            )));
        }
    }

    Ok(())
}

fn validate_unique_evidence(
    evidence: &[EvidenceRecord],
) -> ToolchainResult<()> {
    for pair in evidence.windows(2) {
        if compare_evidence(&pair[0], &pair[1]) == Ordering::Equal {
            return Err(invalid_input(format!(
                "duplicate evidence record: {}/{}",
                pair[0].category,
                pair[0].authority
            )));
        }
    }

    Ok(())
}

fn validate_unique_artifacts(
    artifacts: &[ArtifactEvidence],
) -> ToolchainResult<()> {
    for pair in artifacts.windows(2) {
        if compare_artifacts(&pair[0], &pair[1]) == Ordering::Equal {
            return Err(invalid_input(format!(
                "duplicate artifact identity: {}/{}",
                pair[0].target_id,
                pair[0].artifact_id
            )));
        }
    }

    Ok(())
}

// ============================================================================
// COMPARISON CLASSIFICATION
// ============================================================================

/// Overall outcome of an independent reproducibility comparison.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ReproducibilityStatus {
    /// Both snapshots have complete verified evidence and identical
    /// declared identities and artifact content.
    Reproducible,

    /// A known difference exists between the two build observations.
    NotReproducible,

    /// Required evidence is absent or a required check is unresolved.
    Incomplete,

    /// An authoritative source rejected supplied evidence.
    Invalid,
}

impl ReproducibilityStatus {
    /// Returns true only for a complete reproducibility result.
    #[must_use]
    pub const fn is_reproducible(self) -> bool {
        matches!(self, Self::Reproducible)
    }
}

/// Machine-readable category of a reproducibility discrepancy.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum DifferenceKind {
    /// Logical project/build inputs differ.
    LogicalBuildIdentity,

    /// Target realization identity differs.
    RealizationIdentity,

    /// Target identity differs.
    TargetIdentity,

    /// Required evidence is absent.
    MissingEvidence,

    /// Evidence was not verified.
    UnverifiedEvidence,

    /// Evidence fingerprints differ.
    EvidenceFingerprint,

    /// Artifact sets differ.
    ArtifactSet,

    /// Artifact metadata differs.
    ArtifactMetadata,

    /// Artifact content fingerprints differ.
    ArtifactContent,

    /// Artifact sizes differ.
    ArtifactSize,

    /// Artifact validation has not completed.
    ArtifactValidation,

    /// Artifact validation rejected the artifact.
    RejectedArtifact,
}

impl fmt::Display for DifferenceKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let description = match self {
            Self::LogicalBuildIdentity => "logical build identity",
            Self::RealizationIdentity => "realization identity",
            Self::TargetIdentity => "target identity",
            Self::MissingEvidence => "missing required evidence",
            Self::UnverifiedEvidence => "unverified evidence",
            Self::EvidenceFingerprint => "evidence fingerprint",
            Self::ArtifactSet => "artifact set",
            Self::ArtifactMetadata => "artifact metadata",
            Self::ArtifactContent => "artifact content",
            Self::ArtifactSize => "artifact size",
            Self::ArtifactValidation => "artifact validation",
            Self::RejectedArtifact => "rejected artifact",
        };

        formatter.write_str(description)
    }
}

/// One machine-readable reproducibility discrepancy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReproducibilityDifference {
    kind: DifferenceKind,
    subject: String,
    detail: String,
}

impl ReproducibilityDifference {
    fn new(
        kind: DifferenceKind,
        subject: impl Into<String>,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            subject: subject.into(),
            detail: detail.into(),
        }
    }

    /// Return the discrepancy classification.
    #[must_use]
    pub fn kind(&self) -> &DifferenceKind {
        &self.kind
    }

    /// Return the affected subject.
    #[must_use]
    pub fn subject(&self) -> &str {
        &self.subject
    }

    /// Return the human-readable explanation.
    #[must_use]
    pub fn detail(&self) -> &str {
        &self.detail
    }
}

/// Complete result of comparing two independent build observations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReproducibilityReport {
    status: ReproducibilityStatus,
    first_snapshot: Fingerprint,
    second_snapshot: Fingerprint,
    differences: Vec<ReproducibilityDifference>,
}

impl ReproducibilityReport {
    /// Return the final comparison classification.
    #[must_use]
    pub const fn status(&self) -> ReproducibilityStatus {
        self.status
    }

    /// Return the first snapshot fingerprint.
    #[must_use]
    pub const fn first_snapshot(&self) -> Fingerprint {
        self.first_snapshot
    }

    /// Return the second snapshot fingerprint.
    #[must_use]
    pub const fn second_snapshot(&self) -> Fingerprint {
        self.second_snapshot
    }

    /// Return all identified discrepancies.
    #[must_use]
    pub fn differences(&self) -> &[ReproducibilityDifference] {
        &self.differences
    }

    /// Returns true only when the complete comparison succeeded.
    #[must_use]
    pub const fn is_reproducible(&self) -> bool {
        self.status.is_reproducible()
    }
}

// ============================================================================
// REPRODUCIBILITY VERIFIER
// ============================================================================

/// Stateless reproducibility comparison service.
///
/// No global mutable state, implicit filesystem access, hardware discovery,
/// process execution or hidden caching is performed.
#[derive(Debug, Default, Clone, Copy)]
pub struct ReproducibilityVerifier;

impl ReproducibilityVerifier {
    /// Construct the stateless verifier.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Compare two independently collected build snapshots.
    ///
    /// The snapshots must describe the same logical build and the same
    /// target realization.
    ///
    /// Different target realizations must be compared separately.
    ///
    /// This method never upgrades incomplete evidence to verified evidence.
    pub fn compare(
        &self,
        first: &ReproducibilitySnapshot,
        second: &ReproducibilitySnapshot,
    ) -> ToolchainResult<ReproducibilityReport> {
        let first_fingerprint = first.fingerprint()?;
        let second_fingerprint = second.fingerprint()?;

        let mut differences = Vec::new();

        if first.logical_build_fingerprint
            != second.logical_build_fingerprint
        {
            push_difference(
                &mut differences,
                DifferenceKind::LogicalBuildIdentity,
                "logical-build",
                "The logical build fingerprints differ.",
            )?;
        }

        if first.realization_id != second.realization_id {
            push_difference(
                &mut differences,
                DifferenceKind::RealizationIdentity,
                "realization",
                "The target realization identities differ.",
            )?;
        }

        if first.target_id != second.target_id {
            push_difference(
                &mut differences,
                DifferenceKind::TargetIdentity,
                "target",
                "The target identities differ. Cross-target artifact equality \
                 is not a valid reproducibility comparison.",
            )?;
        }

        if first.has_rejected_evidence()
            || second.has_rejected_evidence()
        {
            push_difference(
                &mut differences,
                DifferenceKind::RejectedArtifact,
                "authoritative-validation",
                "At least one authoritative evidence or artifact validation \
                 result rejected the supplied data.",
            )?;
        }

        compare_required_evidence(
            first,
            second,
            &mut differences,
        )?;

        compare_evidence_records(
            first,
            second,
            &mut differences,
        )?;

        compare_artifact_sets(
            first,
            second,
            &mut differences,
        )?;

        let status = classify_comparison(
            first,
            second,
            &differences,
        );

        Ok(ReproducibilityReport {
            status,
            first_snapshot: first_fingerprint,
            second_snapshot: second_fingerprint,
            differences,
        })
    }
}

fn push_difference(
    differences: &mut Vec<ReproducibilityDifference>,
    kind: DifferenceKind,
    subject: impl Into<String>,
    detail: impl Into<String>,
) -> ToolchainResult<()> {
    differences
        .try_reserve(1)
        .map_err(|error| {
            resource_error("reproducibility discrepancies", error)
        })?;

    differences.push(ReproducibilityDifference::new(
        kind,
        subject,
        detail,
    ));

    Ok(())
}

fn compare_required_evidence(
    first: &ReproducibilitySnapshot,
    second: &ReproducibilitySnapshot,
    differences: &mut Vec<ReproducibilityDifference>,
) -> ToolchainResult<()> {
    for requirement in first.requirements.iter() {
        let first_record = first.evidence.iter().find(|record| {
            record.category == requirement.category
                && record.authority == requirement.authority
        });

        let second_record = second.evidence.iter().find(|record| {
            record.category == requirement.category
                && record.authority == requirement.authority
        });

        match (first_record, second_record) {
            (Some(left), Some(right)) => {
                if !left.status.is_verified()
                    || !right.status.is_verified()
                {
                    push_difference(
                        differences,
                        DifferenceKind::UnverifiedEvidence,
                        format!(
                            "{}/{}",
                            requirement.category,
                            requirement.authority
                        ),
                        "Required evidence has not been verified in both runs.",
                    )?;
                }
            }

            _ => {
                push_difference(
                    differences,
                    DifferenceKind::MissingEvidence,
                    format!(
                        "{}/{}",
                        requirement.category,
                        requirement.authority
                    ),
                    "Required evidence is absent from at least one build.",
                )?;
            }
        }
    }

    for requirement in second.requirements.iter() {
        if !first.requirements.iter().any(|existing| {
            compare_requirement(existing, requirement) == Ordering::Equal
        }) {
            push_difference(
                differences,
                DifferenceKind::MissingEvidence,
                format!(
                    "{}/{}",
                    requirement.category,
                    requirement.authority
                ),
                "The second build declares an additional evidence requirement.",
            )?;
        }
    }

    Ok(())
}

fn compare_evidence_records(
    first: &ReproducibilitySnapshot,
    second: &ReproducibilitySnapshot,
    differences: &mut Vec<ReproducibilityDifference>,
) -> ToolchainResult<()> {
    if first.evidence.len() != second.evidence.len() {
        push_difference(
            differences,
            DifferenceKind::EvidenceFingerprint,
            "evidence-set",
            "The evidence record counts differ.",
        )?;
    }

    for left in &first.evidence {
        let right = second.evidence.iter().find(|record| {
            record.category == left.category
                && record.authority == left.authority
        });

        match right {
            Some(right) => {
                if left.fingerprint != right.fingerprint {
                    push_difference(
                        differences,
                        DifferenceKind::EvidenceFingerprint,
                        format!("{}/{}", left.category, left.authority),
                        "The authoritative evidence fingerprints differ.",
                    )?;
                }

                if left.status != right.status {
                    push_difference(
                        differences,
                        DifferenceKind::UnverifiedEvidence,
                        format!("{}/{}", left.category, left.authority),
                        "The evidence verification states differ.",
                    )?;
                }
            }

            None => {
                push_difference(
                    differences,
                    DifferenceKind::MissingEvidence,
                    format!("{}/{}", left.category, left.authority),
                    "The evidence record is missing from the second build.",
                )?;
            }
        }
    }

    for right in &second.evidence {
        if !first.evidence.iter().any(|left| {
            compare_evidence(left, right) == Ordering::Equal
        }) {
            push_difference(
                differences,
                DifferenceKind::MissingEvidence,
                format!("{}/{}", right.category, right.authority),
                "The second build contains an additional evidence record.",
            )?;
        }
    }

    Ok(())
}

fn compare_artifact_sets(
    first: &ReproducibilitySnapshot,
    second: &ReproducibilitySnapshot,
    differences: &mut Vec<ReproducibilityDifference>,
) -> ToolchainResult<()> {
    if first.artifacts.len() != second.artifacts.len() {
        push_difference(
            differences,
            DifferenceKind::ArtifactSet,
            "artifact-set",
            "The generated artifact counts differ.",
        )?;
    }

    for left in &first.artifacts {
        let right = second.artifacts.iter().find(|artifact| {
            artifact.target_id == left.target_id
                && artifact.artifact_id == left.artifact_id
        });

        match right {
            Some(right) => {
                compare_individual_artifact(
                    left,
                    right,
                    differences,
                )?;
            }

            None => {
                push_difference(
                    differences,
                    DifferenceKind::ArtifactSet,
                    &left.artifact_id,
                    "The artifact is missing from the second build.",
                )?;
            }
        }
    }

    for right in &second.artifacts {
        if !first.artifacts.iter().any(|left| {
            compare_artifacts(left, right) == Ordering::Equal
        }) {
            push_difference(
                differences,
                DifferenceKind::ArtifactSet,
                &right.artifact_id,
                "The second build contains an additional artifact.",
            )?;
        }
    }

    Ok(())
}

fn compare_individual_artifact(
    first: &ArtifactEvidence,
    second: &ArtifactEvidence,
    differences: &mut Vec<ReproducibilityDifference>,
) -> ToolchainResult<()> {
    let subject = first.artifact_id.as_str();

    if first.artifact_kind != second.artifact_kind
        || first.artifact_format != second.artifact_format
        || first.target_id != second.target_id
    {
        push_difference(
            differences,
            DifferenceKind::ArtifactMetadata,
            subject,
            "Artifact kind, format or target metadata differs.",
        )?;
    }

    if first.content_fingerprint != second.content_fingerprint {
        push_difference(
            differences,
            DifferenceKind::ArtifactContent,
            subject,
            "The exact artifact content fingerprints differ.",
        )?;
    }

    if first.content_size_bytes != second.content_size_bytes {
        push_difference(
            differences,
            DifferenceKind::ArtifactSize,
            subject,
            "The artifact byte sizes differ.",
        )?;
    }

    if first.validation_status == EvidenceStatus::Rejected
        || second.validation_status == EvidenceStatus::Rejected
    {
        push_difference(
            differences,
            DifferenceKind::RejectedArtifact,
            subject,
            "Canonical artifact validation rejected this artifact.",
        )?;
    } else if first.validation_status != EvidenceStatus::Verified
        || second.validation_status != EvidenceStatus::Verified
    {
        push_difference(
            differences,
            DifferenceKind::ArtifactValidation,
            subject,
            "Artifact validation is incomplete.",
        )?;
    }

    Ok(())
}

fn classify_comparison(
    first: &ReproducibilitySnapshot,
    second: &ReproducibilitySnapshot,
    differences: &[ReproducibilityDifference],
) -> ReproducibilityStatus {
    if first.has_rejected_evidence()
        || second.has_rejected_evidence()
    {
        return ReproducibilityStatus::Invalid;
    }

    if differences.iter().any(|difference| {
        matches!(
            difference.kind,
            DifferenceKind::LogicalBuildIdentity
                | DifferenceKind::RealizationIdentity
                | DifferenceKind::TargetIdentity
                | DifferenceKind::EvidenceFingerprint
                | DifferenceKind::ArtifactSet
                | DifferenceKind::ArtifactMetadata
                | DifferenceKind::ArtifactContent
                | DifferenceKind::ArtifactSize
        )
    }) {
        return ReproducibilityStatus::NotReproducible;
    }

    if first.has_incomplete_evidence()
        || second.has_incomplete_evidence()
        || !differences.is_empty()
    {
        return ReproducibilityStatus::Incomplete;
    }

    ReproducibilityStatus::Reproducible
}

// ============================================================================
// TEST UTILITIES
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn test_fingerprint(value: u8) -> Fingerprint {
        Fingerprint::from_bytes([value; 32])
    }

    fn test_requirement() -> EvidenceRequirement {
        EvidenceRequirement::new(
            "compiler",
            "zamani.compiler.reproducibility",
        )
        .expect("valid requirement")
    }

    fn test_evidence(value: u8) -> EvidenceRecord {
        EvidenceRecord::new(
            "compiler",
            "zamani.compiler.reproducibility",
            test_fingerprint(value),
            EvidenceStatus::Verified,
        )
        .expect("valid evidence")
    }

    fn test_artifact(value: u8) -> ArtifactEvidence {
        ArtifactEvidence::new(
            "artifact:primary",
            "executable",
            "native",
            "target:example",
            test_fingerprint(value),
            1024,
            EvidenceStatus::Verified,
        )
        .expect("valid artifact")
    }

    fn test_snapshot(
        evidence_value: u8,
        artifact_value: u8,
    ) -> ReproducibilitySnapshot {
        ReproducibilitySnapshot::new(
            test_fingerprint(10),
            "realization:example",
            "target:example",
            vec![test_requirement()],
            vec![test_evidence(evidence_value)],
            vec![test_artifact(artifact_value)],
        )
        .expect("valid snapshot")
    }

    #[test]
    fn identical_snapshots_have_identical_fingerprints() {
        let first = test_snapshot(20, 30);
        let second = test_snapshot(20, 30);

        assert_eq!(
            first.fingerprint().expect("fingerprint"),
            second.fingerprint().expect("fingerprint")
        );
    }

    #[test]
    fn canonical_order_does_not_depend_on_input_order() {
        let first_requirement = EvidenceRequirement::new(
            "compiler",
            "compiler.authority",
        )
        .expect("requirement");

        let second_requirement = EvidenceRequirement::new(
            "grammar",
            "grammar.authority",
        )
        .expect("requirement");

        let first_evidence = EvidenceRecord::new(
            "compiler",
            "compiler.authority",
            test_fingerprint(1),
            EvidenceStatus::Verified,
        )
        .expect("evidence");

        let second_evidence = EvidenceRecord::new(
            "grammar",
            "grammar.authority",
            test_fingerprint(2),
            EvidenceStatus::Verified,
        )
        .expect("evidence");

        let first = ReproducibilitySnapshot::new(
            test_fingerprint(10),
            "realization",
            "target",
            vec![
                first_requirement.clone(),
                second_requirement.clone(),
            ],
            vec![
                first_evidence.clone(),
                second_evidence.clone(),
            ],
            Vec::new(),
        )
        .expect("snapshot");

        let second = ReproducibilitySnapshot::new(
            test_fingerprint(10),
            "realization",
            "target",
            vec![
                second_requirement,
                first_requirement,
            ],
            vec![
                second_evidence,
                first_evidence,
            ],
            Vec::new(),
        )
        .expect("snapshot");

        assert_eq!(
            first.fingerprint().expect("fingerprint"),
            second.fingerprint().expect("fingerprint")
        );
    }

    #[test]
    fn identical_verified_builds_are_reproducible() {
        let first = test_snapshot(20, 30);
        let second = test_snapshot(20, 30);

        let report = ReproducibilityVerifier::new()
            .compare(&first, &second)
            .expect("comparison");

        assert_eq!(
            report.status(),
            ReproducibilityStatus::Reproducible
        );

        assert!(report.is_reproducible());
        assert!(report.differences().is_empty());
    }

    #[test]
    fn different_artifact_bytes_are_not_reproducible() {
        let first = test_snapshot(20, 30);
        let second = test_snapshot(20, 31);

        let report = ReproducibilityVerifier::new()
            .compare(&first, &second)
            .expect("comparison");

        assert_eq!(
            report.status(),
            ReproducibilityStatus::NotReproducible
        );

        assert!(report.differences().iter().any(|difference| {
            difference.kind() == &DifferenceKind::ArtifactContent
        }));
    }

    #[test]
    fn different_compiler_evidence_is_not_reproducible() {
        let first = test_snapshot(20, 30);
        let second = test_snapshot(21, 30);

        let report = ReproducibilityVerifier::new()
            .compare(&first, &second)
            .expect("comparison");

        assert_eq!(
            report.status(),
            ReproducibilityStatus::NotReproducible
        );
    }

    #[test]
    fn missing_required_evidence_is_incomplete() {
        let first = test_snapshot(20, 30);

        let second = ReproducibilitySnapshot::new(
            test_fingerprint(10),
            "realization:example",
            "target:example",
            vec![test_requirement()],
            Vec::new(),
            vec![test_artifact(30)],
        )
        .expect("snapshot");

        let report = ReproducibilityVerifier::new()
            .compare(&first, &second)
            .expect("comparison");

        assert_eq!(
            report.status(),
            ReproducibilityStatus::Incomplete
        );
    }

    #[test]
    fn incomplete_artifact_validation_is_not_success() {
        let first = test_snapshot(20, 30);

        let incomplete_artifact = ArtifactEvidence::new(
            "artifact:primary",
            "executable",
            "native",
            "target:example",
            test_fingerprint(30),
            1024,
            EvidenceStatus::Incomplete,
        )
        .expect("artifact");

        let second = ReproducibilitySnapshot::new(
            test_fingerprint(10),
            "realization:example",
            "target:example",
            vec![test_requirement()],
            vec![test_evidence(20)],
            vec![incomplete_artifact],
        )
        .expect("snapshot");

        let report = ReproducibilityVerifier::new()
            .compare(&first, &second)
            .expect("comparison");

        assert_eq!(
            report.status(),
            ReproducibilityStatus::Incomplete
        );
    }

    #[test]
    fn rejected_artifact_validation_is_invalid() {
        let first = test_snapshot(20, 30);

        let rejected_artifact = ArtifactEvidence::new(
            "artifact:primary",
            "executable",
            "native",
            "target:example",
            test_fingerprint(30),
            1024,
            EvidenceStatus::Rejected,
        )
        .expect("artifact");

        let second = ReproducibilitySnapshot::new(
            test_fingerprint(10),
            "realization:example",
            "target:example",
            vec![test_requirement()],
            vec![test_evidence(20)],
            vec![rejected_artifact],
        )
        .expect("snapshot");

        let report = ReproducibilityVerifier::new()
            .compare(&first, &second)
            .expect("comparison");

        assert_eq!(
            report.status(),
            ReproducibilityStatus::Invalid
        );
    }

    #[test]
    fn different_targets_are_not_compared_as_identical_realizations() {
        let first = test_snapshot(20, 30);

        let second = ReproducibilitySnapshot::new(
            test_fingerprint(10),
            "realization:other",
            "target:other",
            vec![test_requirement()],
            vec![test_evidence(20)],
            Vec::new(),
        )
        .expect("snapshot");

        let report = ReproducibilityVerifier::new()
            .compare(&first, &second)
            .expect("comparison");

        assert_eq!(
            report.status(),
            ReproducibilityStatus::NotReproducible
        );

        assert!(report.differences().iter().any(|difference| {
            difference.kind() == &DifferenceKind::TargetIdentity
        }));
    }

    #[test]
    fn duplicate_evidence_requirements_are_rejected() {
        let requirement = test_requirement();

        let result = ReproducibilitySnapshot::new(
            test_fingerprint(10),
            "realization",
            "target",
            vec![
                requirement.clone(),
                requirement,
            ],
            Vec::new(),
            Vec::new(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn duplicate_artifact_identities_are_rejected() {
        let artifact = test_artifact(30);

        let result = ReproducibilitySnapshot::new(
            test_fingerprint(10),
            "realization",
            "target:example",
            Vec::new(),
            Vec::new(),
            vec![
                artifact.clone(),
                artifact,
            ],
        );

        assert!(result.is_err());
    }

    #[test]
    fn artifact_for_another_target_is_rejected() {
        let artifact = ArtifactEvidence::new(
            "artifact",
            "executable",
            "native",
            "different-target",
            test_fingerprint(1),
            100,
            EvidenceStatus::Verified,
        )
        .expect("artifact");

        let result = ReproducibilitySnapshot::new(
            test_fingerprint(10),
            "realization",
            "target:example",
            Vec::new(),
            Vec::new(),
            vec![artifact],
        );

        assert!(result.is_err());
    }

    #[test]
    fn absent_artifacts_are_not_automatically_reproducible() {
        let first = test_snapshot(20, 30);

        let second = ReproducibilitySnapshot::new(
            test_fingerprint(10),
            "realization:example",
            "target:example",
            vec![test_requirement()],
            vec![test_evidence(20)],
            Vec::new(),
        )
        .expect("snapshot");

        let report = ReproducibilityVerifier::new()
            .compare(&first, &second)
            .expect("comparison");

        assert_eq!(
            report.status(),
            ReproducibilityStatus::NotReproducible
        );
    }

    #[test]
    fn snapshot_fingerprint_changes_when_logical_identity_changes() {
        let first = test_snapshot(20, 30);

        let second = ReproducibilitySnapshot::new(
            test_fingerprint(11),
            "realization:example",
            "target:example",
            vec![test_requirement()],
            vec![test_evidence(20)],
            vec![test_artifact(30)],
        )
        .expect("snapshot");

        assert_ne!(
            first.fingerprint().expect("fingerprint"),
            second.fingerprint().expect("fingerprint")
        );
    }

    #[test]
    fn snapshot_fingerprint_changes_when_artifact_size_changes() {
        let first = test_snapshot(20, 30);

        let artifact = ArtifactEvidence::new(
            "artifact:primary",
            "executable",
            "native",
            "target:example",
            test_fingerprint(30),
            2048,
            EvidenceStatus::Verified,
        )
        .expect("artifact");

        let second = ReproducibilitySnapshot::new(
            test_fingerprint(10),
            "realization:example",
            "target:example",
            vec![test_requirement()],
            vec![test_evidence(20)],
            vec![artifact],
        )
        .expect("snapshot");

        assert_ne!(
            first.fingerprint().expect("fingerprint"),
            second.fingerprint().expect("fingerprint")
        );
    }

    #[test]
    fn empty_identifiers_are_rejected() {
        assert!(
            EvidenceRequirement::new("", "authority").is_err()
        );

        assert!(
            EvidenceRequirement::new("category", "").is_err()
        );

        assert!(
            ReproducibilitySnapshot::new(
                test_fingerprint(1),
                "",
                "target",
                Vec::new(),
                Vec::new(),
                Vec::new(),
            )
            .is_err()
        );
    }

    #[test]
    fn evidence_status_is_part_of_snapshot_identity() {
        let verified = test_snapshot(20, 30);

        let incomplete_record = EvidenceRecord::new(
            "compiler",
            "zamani.compiler.reproducibility",
            test_fingerprint(20),
            EvidenceStatus::Incomplete,
        )
        .expect("evidence");

        let incomplete = ReproducibilitySnapshot::new(
            test_fingerprint(10),
            "realization:example",
            "target:example",
            vec![test_requirement()],
            vec![incomplete_record],
            vec![test_artifact(30)],
        )
        .expect("snapshot");

        assert_ne!(
            verified.fingerprint().expect("fingerprint"),
            incomplete.fingerprint().expect("fingerprint")
        );
    }
}