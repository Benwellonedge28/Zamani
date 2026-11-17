
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Toolchain — Build Outputs
//!
//! Path: src/toolchain/build/outputs.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Collect, validate, associate and report canonical artifacts produced
//! by one logical Zamani build.
//!
//! The input is the actual BuildExecutionResult produced by the
//! canonical toolchain build executor.
//!
//! VerifiedBuildArtifact values are converted through an injected
//! canonical artifact adapter. This module does not implement a
//! competing artifact representation.
//!
//! ## OWNERSHIP
//!
//! This module owns:
//!
//! - Build-output collection.
//! - Canonical artifact adaptation orchestration.
//! - Artifact-to-target association.
//! - Target-output coverage.
//! - Output completeness classification.
//! - Aggregate output statistics.
//! - Public API artifact references.
//! - Build-output reporting data.
//!
//! It does not own:
//!
//! - Source compilation.
//! - Artifact generation.
//! - Artifact ID allocation.
//! - Artifact format registration.
//! - Artifact payload storage.
//! - Artifact digest algorithms.
//! - Artifact verification algorithms.
//! - Portability classification.
//! - Semantic or IR verification.
//! - Target discovery.
//! - Feasibility analysis.
//! - Physical resource allocation.
//! - Runtime execution.
//! - Deployment.
//!
//! ## AUTHORITATIVE INTEGRATIONS
//!
//! Build feasibility:
//!     crate::toolchain::build::plan
//!
//! Actual build execution:
//!     crate::toolchain::build::executor
//!
//! Verified execution outputs:
//!     crate::toolchain::build::executor::BuildExecutionResult
//!
//! Canonical artifact model:
//!     crate::compiler::artifacts::artifact
//!
//! Canonical artifact manifest:
//!     crate::compiler::artifacts::manifest
//!
//! Artifact validation:
//!     crate::compiler::artifacts::validation
//!
//! Public artifact references:
//!     crate::toolchain::api::response
//!
//! ## POCO-REAF
//!
//! One logical build can produce multiple artifacts for multiple
//! compatible targets.
//!
//! Each artifact retains its own:
//!
//! - Canonical identity.
//! - Representation kind.
//! - Format.
//! - Target identity.
//! - Content digest.
//! - Payload reference.
//! - Source provenance.
//! - Transformation provenance.
//!
//! No fixed computational-domain or target enumeration exists.
//!
//! Different targets may produce different artifacts while preserving
//! the applicable source-program semantic contracts.
//!
//! ## COMPLETION
//!
//! A complete output collection requires:
//!
//! - Successful actual build execution.
//! - Consistent build identities.
//! - Consistent skipped-target information.
//! - Canonical artifact adaptation.
//! - Target identity consistency.
//! - Correct artifact content digest.
//! - Correct artifact payload size.
//! - Canonical artifact validation.
//! - Required source provenance.
//! - Complete target-output coverage.
//! - Complete feasibility assessment.
//!
//! Missing evidence produces an incomplete result.
//!
//! ## SCALABILITY
//!
//! No arbitrary maximum is imposed on:
//!
//! - Artifact count.
//! - Target count.
//! - Domain count.
//! - Payload size.
//! - Distributed participants.
//! - Hardware architectures.
//! - Processors.
//! - Accelerators.
//! - Quantum resources.
//!
//! Canonical artifacts may use inline or external payload storage.
//!
//! External payloads are never falsely represented as verified.
//!
//! Operational memory and storage limitations remain real.
//!
//! ## DETERMINISM
//!
//! Artifacts follow canonical manifest identity ordering.
//!
//! Targets follow canonical compiler target ordering.
//!
//! No implicit target selection, timestamps or random identities
//! are introduced.

use std::fmt;

use crate::compiler::artifacts::artifact::{
    Artifact,
    ArtifactDigest,
    ArtifactId,
    ArtifactKindId,
    ArtifactFormatId,
    ArtifactProducerId,
    ArtifactProvenance,
    ArtifactTargetRef,
};

use crate::compiler::artifacts::manifest::{
    ArtifactManifest,
    ManifestId,
};

use crate::compiler::artifacts::validation::{
    validate_manifest,
    ManifestValidationPolicy,
    ValidationReport,
    ValidationStatus,
};

use crate::toolchain::api::response::ArtifactReference;

use crate::toolchain::build::executor::{
    BuildExecutionResult,
    VerifiedBuildArtifact,
};

use crate::toolchain::build::plan::AssessedBuildPlan;

use crate::toolchain::errors::ToolchainError;

use crate::toolchain::ids::BuildId;

use crate::toolchain::result::ToolchainResult;

// ============================================================================
// ERROR HELPERS
// ============================================================================

fn invalid_outputs(message: impl Into<String>) -> ToolchainError {
    ToolchainError::validation_failed(message)
        .with_operation("build.outputs")
}

fn allocation_failure(
    subject: &'static str,
    error: std::collections::TryReserveError,
) -> ToolchainError {
    ToolchainError::resource_exhausted(format!(
        "unable to reserve memory for {subject}: {error}"
    ))
    .with_operation("build.outputs")
}

// ============================================================================
// CANONICAL ARTIFACT ADAPTER
// ============================================================================

/// Integration contract between verified build execution outputs and
/// the canonical compiler artifact model.
///
/// Implementations belong to the compiler/backend integration layer.
///
/// The adapter is responsible for:
///
/// - Selecting the registered artifact kind.
/// - Selecting the actual output representation format.
/// - Obtaining or allocating canonical artifact identity.
/// - Preserving target identity.
/// - Preserving source and transformation provenance.
/// - Selecting inline or external payload storage.
/// - Persisting external payloads when used.
/// - Establishing that the adapted payload represents the exact
///   verified compiler output.
///
/// This trait does not implement any of those algorithms itself.
///
/// No vendor, computing domain, artifact format or target is hard-coded.
///
/// Implementations must not manufacture source provenance,
/// content digests or verification evidence.
///
/// An external payload adapter must establish that the stored
/// content corresponds exactly to the supplied compiler output.
///
/// # Thread safety
///
/// Implementations must be Send + Sync so the integration can be
/// reused by concurrent toolchain operations.
///
/// # Errors
///
/// Return an error whenever adaptation or its required verification
/// cannot be completed.
pub trait CanonicalArtifactAdapter: Send + Sync {
    /// Convert one actually verified compiler output into the
    /// canonical immutable compiler artifact.
    fn adapt(
        &self,
        output: &VerifiedBuildArtifact,
    ) -> ToolchainResult<Artifact>;

    /// Verify the relationship between the verified compiler output
    /// and its canonical artifact representation.
    ///
    /// The implementation must establish exact content correspondence,
    /// including external storage when used.
    ///
    /// This does not replace canonical artifact validation.
    fn verify_adaptation(
        &self,
        output: &VerifiedBuildArtifact,
        artifact: &Artifact,
    ) -> ToolchainResult<()>;
}

// ============================================================================
// OUTPUT STATUS
// ============================================================================

/// Overall completeness of the collected build outputs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BuildOutputStatus {
    /// Every required output and validation condition passed.
    Complete,

    /// At least one permitted target realization succeeded,
    /// while other requested targets were skipped.
    Partial,

    /// Required output or validation evidence is missing.
    Incomplete,
}

impl BuildOutputStatus {
    /// Returns whether the output collection is complete.
    #[must_use]
    pub const fn is_complete(self) -> bool {
        matches!(self, Self::Complete)
    }

    /// Returns whether the output collection is partial.
    #[must_use]
    pub const fn is_partial(self) -> bool {
        matches!(self, Self::Partial)
    }

    /// Returns whether the output collection is incomplete.
    #[must_use]
    pub const fn is_incomplete(self) -> bool {
        matches!(self, Self::Incomplete)
    }

    /// Returns whether at least one successful realization exists.
    ///
    /// This does not authorize execution or deployment.
    #[must_use]
    pub const fn has_successful_realization(self) -> bool {
        matches!(self, Self::Complete | Self::Partial)
    }
}

// ============================================================================
// TARGET OUTPUT COVERAGE
// ============================================================================

/// Output coverage for one compiler-assessed build target.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TargetOutputCoverage {
    target_id: String,
    artifact_ids: Vec<ArtifactId>,
}

impl TargetOutputCoverage {
    /// Returns the canonical compiler target identity.
    #[must_use]
    pub fn target_id(&self) -> &str {
        &self.target_id
    }

    /// Returns canonical artifact identities associated with this target.
    #[must_use]
    pub fn artifact_ids(&self) -> &[ArtifactId] {
        &self.artifact_ids
    }

    /// Returns whether the target has at least one collected artifact.
    #[must_use]
    pub fn is_covered(&self) -> bool {
        !self.artifact_ids.is_empty()
    }

    /// Returns the number of artifacts associated with this target.
    #[must_use]
    pub fn artifact_count(&self) -> usize {
        self.artifact_ids.len()
    }
}

// ============================================================================
// OUTPUT SUMMARY
// ============================================================================

/// Aggregate statistics for one logical build's outputs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BuildOutputSummary {
    artifact_count: usize,
    targeted_artifact_count: usize,
    shared_artifact_count: usize,
    target_count: usize,
    covered_target_count: usize,
    missing_target_count: usize,
    total_payload_bytes: Option<u64>,
}

impl BuildOutputSummary {
    fn calculate(
        manifest: &ArtifactManifest,
        coverage: &[TargetOutputCoverage],
    ) -> Self {
        let artifact_count = manifest.artifact_count();

        let targeted_artifact_count = manifest
            .artifacts()
            .filter(|(_, artifact)| {
                artifact.descriptor().target().is_some()
            })
            .count();

        let shared_artifact_count =
            artifact_count - targeted_artifact_count;

        let mut total_payload_bytes = Some(0u64);

        for (_, artifact) in manifest.artifacts() {
            total_payload_bytes = match (
                total_payload_bytes,
                artifact.size_bytes(),
            ) {
                (Some(total), Some(size)) => {
                    total.checked_add(size)
                }

                _ => None,
            };
        }

        let covered_target_count = coverage
            .iter()
            .filter(|entry| entry.is_covered())
            .count();

        let missing_target_count =
            coverage.len() - covered_target_count;

        Self {
            artifact_count,
            targeted_artifact_count,
            shared_artifact_count,
            target_count: coverage.len(),
            covered_target_count,
            missing_target_count,
            total_payload_bytes,
        }
    }

    /// Returns the number of canonical artifacts.
    #[must_use]
    pub const fn artifact_count(&self) -> usize {
        self.artifact_count
    }

    /// Returns the number of target-specific artifacts.
    #[must_use]
    pub const fn targeted_artifact_count(&self) -> usize {
        self.targeted_artifact_count
    }

    /// Returns the number of artifacts without an explicit target.
    ///
    /// An absent target does not prove portability.
    #[must_use]
    pub const fn shared_artifact_count(&self) -> usize {
        self.shared_artifact_count
    }

    /// Returns the number of buildable targets.
    #[must_use]
    pub const fn target_count(&self) -> usize {
        self.target_count
    }

    /// Returns the number of targets with output artifacts.
    #[must_use]
    pub const fn covered_target_count(&self) -> usize {
        self.covered_target_count
    }

    /// Returns the number of targets without output artifacts.
    #[must_use]
    pub const fn missing_target_count(&self) -> usize {
        self.missing_target_count
    }

    /// Returns the total known payload size.
    ///
    /// None means at least one payload size is unknown or the
    /// total cannot be represented by u64.
    #[must_use]
    pub const fn total_payload_bytes(&self) -> Option<u64> {
        self.total_payload_bytes
    }
}

// ============================================================================
// BUILD OUTPUT COLLECTION
// ============================================================================

/// Canonical outputs collected from one actual logical build execution.
///
/// The compiler ArtifactManifest remains the canonical owner of
/// artifact representations, dependencies, roots and provenance.
///
/// ArtifactReference values are lightweight API representations.
/// They do not duplicate binary payload storage.
#[derive(Debug, Clone)]
#[must_use = "build outputs must be inspected or reported"]
pub struct BuildOutputs {
    build_id: BuildId,
    manifest: ArtifactManifest,
    artifact_references: Vec<ArtifactReference>,
    target_coverage: Vec<TargetOutputCoverage>,
    validation: ValidationReport,
    summary: BuildOutputSummary,
    status: BuildOutputStatus,
}

impl BuildOutputs {
    /// Collect outputs from a real, successfully returned build execution.
    ///
    /// The build execution result must originate from the canonical
    /// BuildExecutor.
    ///
    /// The adapter must produce canonical artifacts and preserve their
    /// content, target identity and provenance.
    ///
    /// The supplied validation policy determines which artifact
    /// properties are required.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    ///
    /// - The execution result belongs to another build.
    /// - Skipped targets disagree with the assessed plan.
    /// - No verified artifacts were produced.
    /// - An artifact cannot be adapted.
    /// - An artifact has an incorrect target.
    /// - An artifact's digest or payload size is inconsistent.
    /// - Artifact adaptation verification fails.
    /// - Canonical artifact validation finds a known error.
    /// - An artifact cannot be associated with a buildable target.
    /// - Required memory reservation fails.
    pub fn collect<A: CanonicalArtifactAdapter + ?Sized>(
        plan: &AssessedBuildPlan,
        execution: &BuildExecutionResult,
        adapter: &A,
        validation_policy: &ManifestValidationPolicy,
    ) -> ToolchainResult<Self> {
        // --------------------------------------------------------------------
        // 1. Verify build identity.
        // --------------------------------------------------------------------

        let build_id = plan.plan().id();

        if execution.build_id() != build_id {
            return Err(invalid_outputs(
                "execution result belongs to a different logical build",
            ));
        }

        // --------------------------------------------------------------------
        // 2. Verify target-skipping information.
        // --------------------------------------------------------------------

        if !plan.can_attempt_build() {
            return Err(invalid_outputs(
                "assessed build plan does not authorize compilation",
            ));
        }

        let expected_skipped: Vec<_> = plan
            .non_buildable_targets()
            .map(|target| target.target_id().clone())
            .collect();

        if expected_skipped.as_slice() != execution.skipped_targets() {
            return Err(invalid_outputs(
                "execution skipped-target information contradicts \
                 the authoritative assessed build plan",
            ));
        }

        // --------------------------------------------------------------------
        // 3. Require actual verified compiler output.
        // --------------------------------------------------------------------

        if !execution.has_artifacts() {
            return Err(invalid_outputs(
                "build execution contains no verified artifacts",
            ));
        }

        // --------------------------------------------------------------------
        // 4. Construct the canonical manifest.
        // --------------------------------------------------------------------

        let manifest_id = ManifestId::new(
            format!("build-output:{build_id}"),
        )
        .map_err(|error| {
            invalid_outputs(format!(
                "cannot construct canonical artifact manifest identity: \
                 {error}"
            ))
        })?;

        let producer = ArtifactProducerId::new(
            "zamani.toolchain.build.outputs",
        )
        .map_err(|error| {
            invalid_outputs(format!(
                "cannot construct artifact manifest producer identity: \
                 {error}"
            ))
        })?;

        let manifest_provenance = ArtifactProvenance::new(producer);

        let mut manifest = ArtifactManifest::new(
            manifest_id,
            manifest_provenance,
        );

        // --------------------------------------------------------------------
        // 5. Construct deterministic target coverage.
        // --------------------------------------------------------------------

        let mut target_coverage = Vec::new();

        let buildable_targets: Vec<String> = plan
            .buildable_targets()
            .map(|target| target.target_id().to_string())
            .collect();

        target_coverage
            .try_reserve(buildable_targets.len())
            .map_err(|error| {
                allocation_failure("target output coverage", error)
            })?;

        for target_id in buildable_targets {
            target_coverage.push(TargetOutputCoverage {
                target_id,
                artifact_ids: Vec::new(),
            });
        }

        // --------------------------------------------------------------------
        // 6. Adapt actual verified compiler outputs.
        // --------------------------------------------------------------------

        for verified in execution.artifacts() {
            if verified.build_id() != build_id {
                return Err(invalid_outputs(
                    "verified artifact has a different build identity",
                ));
            }

            let target_id = verified.target_id().as_str();

            let coverage = target_coverage
                .iter_mut()
                .find(|entry| entry.target_id == target_id)
                .ok_or_else(|| {
                    invalid_outputs(format!(
                        "verified artifact references target '{target_id}' \
                         which is not buildable in this plan"
                    ))
                })?;

            let artifact = adapter.adapt(verified)?;

            // The canonical artifact must preserve the actual target.
            let artifact_target = artifact
                .descriptor()
                .target()
                .ok_or_else(|| {
                    invalid_outputs(format!(
                        "adapted artifact {} has no explicit target identity",
                        artifact.id()
                    ))
                })?;

            if artifact_target.as_str() != target_id {
                return Err(invalid_outputs(format!(
                    "adapted artifact {} targets '{}' but its verified \
                     compiler output belongs to '{}'",
                    artifact.id(),
                    artifact_target.as_str(),
                    target_id,
                )));
            }

            // The canonical digest must identify the exact compiler bytes.
            let expected_digest = ArtifactDigest::sha256(
                &verified.output().bytes,
            );

            if artifact.digest() != &expected_digest {
                return Err(invalid_outputs(format!(
                    "adapted artifact {} does not preserve the verified \
                     compiler output digest",
                    artifact.id()
                )));
            }

            let expected_size = u64::try_from(
                verified.output().bytes.len(),
            )
            .map_err(|_| {
                invalid_outputs(
                    "compiler output size cannot be represented as u64",
                )
            })?;

            if artifact.size_bytes() != Some(expected_size) {
                return Err(invalid_outputs(format!(
                    "adapted artifact {} has an incorrect or unknown \
                     payload size",
                    artifact.id()
                )));
            }

            // Independently verify inline payloads.
            //
            // External payloads require their storage provider to
            // establish exact content correspondence.
            if artifact.has_inline_payload() {
                if artifact.inline_bytes()
                    != Some(verified.output().bytes.as_slice())
                {
                    return Err(invalid_outputs(format!(
                        "inline artifact {} does not contain the exact \
                         verified compiler output",
                        artifact.id()
                    )));
                }

                artifact.verify_inline_payload().map_err(|error| {
                    invalid_outputs(format!(
                        "canonical inline artifact {} failed integrity \
                         verification: {error}",
                        artifact.id()
                    ))
                })?;
            }

            // This adapter verification is mandatory for both storage
            // representations, particularly external payload storage.
            adapter.verify_adaptation(verified, &artifact)?;

            // Preserve the canonical identity before transferring ownership.
            let artifact_id = artifact.id().clone();

            manifest
                .insert_artifact(artifact)
                .map_err(|error| {
                    invalid_outputs(format!(
                        "cannot insert canonical build artifact: {error}"
                    ))
                })?;

            manifest.add_root(artifact_id.clone());

            coverage
                .artifact_ids
                .try_reserve(1)
                .map_err(|error| {
                    allocation_failure(
                        "target artifact associations",
                        error,
                    )
                })?;

            coverage.artifact_ids.push(artifact_id);
        }

        // --------------------------------------------------------------------
        // 7. Validate the canonical manifest and its artifacts.
        // --------------------------------------------------------------------

        let validation = validate_manifest(
            &manifest,
            validation_policy,
        );

        if validation.status() == ValidationStatus::Invalid {
            return Err(invalid_outputs(format!(
                "canonical artifact validation found {} error(s)",
                validation.error_count()
            )));
        }

        // --------------------------------------------------------------------
        // 8. Create canonical API references.
        // --------------------------------------------------------------------

        let mut artifact_references = Vec::new();

        artifact_references
            .try_reserve(manifest.artifact_count())
            .map_err(|error| {
                allocation_failure("API artifact references", error)
            })?;

        for (_, artifact) in manifest.artifacts() {
            let reference = ArtifactReference::from(artifact);

            reference.validate().map_err(|error| {
                invalid_outputs(format!(
                    "canonical artifact cannot be represented by \
                     the toolchain API: {error}"
                ))
            })?;

            artifact_references.push(reference);
        }

        // --------------------------------------------------------------------
        // 9. Calculate output completeness.
        // --------------------------------------------------------------------

        let summary = BuildOutputSummary::calculate(
            &manifest,
            &target_coverage,
        );

        let status = Self::determine_status(
            plan,
            execution,
            &validation,
            &summary,
            &manifest,
        );

        Ok(Self {
            build_id: build_id.clone(),
            manifest,
            artifact_references,
            target_coverage,
            validation,
            summary,
            status,
        })
    }

    /// Determine output completeness from authoritative evidence.
    fn determine_status(
        plan: &AssessedBuildPlan,
        execution: &BuildExecutionResult,
        validation: &ValidationReport,
        summary: &BuildOutputSummary,
        manifest: &ArtifactManifest,
    ) -> BuildOutputStatus {
        if manifest.is_empty()
            || summary.missing_target_count() != 0
            || !plan.assessment_is_complete()
            || validation.status() != ValidationStatus::Valid
        {
            return BuildOutputStatus::Incomplete;
        }

        // Every adapted artifact must preserve source provenance.
        //
        // A missing source fingerprint is not silently interpreted
        // as reproducibility evidence.
        for (_, artifact) in manifest.artifacts() {
            if artifact
                .descriptor()
                .provenance()
                .source_fingerprint()
                .is_none()
            {
                return BuildOutputStatus::Incomplete;
            }
        }

        // A successful partial build is possible when the original
        // request explicitly permitted skipping infeasible targets.
        if execution.has_skipped_targets()
            || !plan.is_fully_feasible()
        {
            return BuildOutputStatus::Partial;
        }

        BuildOutputStatus::Complete
    }

    // ------------------------------------------------------------------------
    // Identity
    // ------------------------------------------------------------------------

    /// Returns the logical build identity.
    #[must_use]
    pub fn build_id(&self) -> &BuildId {
        &self.build_id
    }

    // ------------------------------------------------------------------------
    // Canonical artifacts
    // ------------------------------------------------------------------------

    /// Borrows the canonical compiler artifact manifest.
    #[must_use]
    pub fn manifest(&self) -> &ArtifactManifest {
        &self.manifest
    }

    /// Borrows a canonical artifact by identity.
    #[must_use]
    pub fn artifact(
        &self,
        id: &ArtifactId,
    ) -> Option<&Artifact> {
        self.manifest.artifact(id)
    }

    /// Iterates over canonical artifacts in deterministic identity order.
    pub fn artifacts(
        &self,
    ) -> impl Iterator<Item = (&ArtifactId, &Artifact)> {
        self.manifest.artifacts()
    }

    /// Returns the number of canonical artifacts.
    #[must_use]
    pub fn artifact_count(&self) -> usize {
        self.manifest.artifact_count()
    }

    /// Returns whether the collection is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.manifest.is_empty()
    }

    // ------------------------------------------------------------------------
    // API references
    // ------------------------------------------------------------------------

    /// Borrows lightweight API artifact references.
    #[must_use]
    pub fn artifact_references(&self) -> &[ArtifactReference] {
        &self.artifact_references
    }

    /// Returns the API reference for a canonical artifact identity.
    ///
    /// References follow the canonical manifest's artifact-ID ordering.
    #[must_use]
    pub fn artifact_reference(
        &self,
        id: &ArtifactId,
    ) -> Option<&ArtifactReference> {
        self.artifact_references
            .binary_search_by(|reference| {
                reference.id.as_str().cmp(id.as_str())
            })
            .ok()
            .and_then(|index| self.artifact_references.get(index))
    }

    // ------------------------------------------------------------------------
    // Target coverage
    // ------------------------------------------------------------------------

    /// Borrows target-output coverage.
    #[must_use]
    pub fn target_coverage(&self) -> &[TargetOutputCoverage] {
        &self.target_coverage
    }

    /// Returns coverage for one canonical target identity.
    #[must_use]
    pub fn coverage_for_target(
        &self,
        target_id: &str,
    ) -> Option<&TargetOutputCoverage> {
        self.target_coverage
            .iter()
            .find(|entry| entry.target_id == target_id)
    }

    /// Returns whether every buildable target has at least one artifact.
    #[must_use]
    pub fn covers_all_buildable_targets(&self) -> bool {
        self.summary.missing_target_count() == 0
    }

    // ------------------------------------------------------------------------
    // Validation
    // ------------------------------------------------------------------------

    /// Borrows the canonical artifact validation report.
    #[must_use]
    pub fn validation(&self) -> &ValidationReport {
        &self.validation
    }

    /// Returns the validation status.
    #[must_use]
    pub fn validation_status(&self) -> ValidationStatus {
        self.validation.status()
    }

    // ------------------------------------------------------------------------
    // Summary and status
    // ------------------------------------------------------------------------

    /// Borrows the aggregate output summary.
    #[must_use]
    pub const fn summary(&self) -> &BuildOutputSummary {
        &self.summary
    }

    /// Returns the output completeness status.
    #[must_use]
    pub const fn status(&self) -> BuildOutputStatus {
        self.status
    }

    /// Returns whether output collection is complete.
    #[must_use]
    pub const fn is_complete(&self) -> bool {
        self.status.is_complete()
    }

    /// Returns whether output collection is partial.
    #[must_use]
    pub const fn is_partial(&self) -> bool {
        self.status.is_partial()
    }

    /// Returns whether output collection is incomplete.
    #[must_use]
    pub const fn is_incomplete(&self) -> bool {
        self.status.is_incomplete()
    }

    /// Returns whether at least one successful target realization exists.
    ///
    /// This does not establish runtime compatibility.
    #[must_use]
    pub const fn has_successful_realization(&self) -> bool {
        self.status.has_successful_realization()
    }
}

// ============================================================================
// DISPLAY
// ============================================================================

impl fmt::Display for BuildOutputStatus {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::Complete => formatter.write_str("complete"),
            Self::Partial => formatter.write_str("partial"),
            Self::Incomplete => formatter.write_str("incomplete"),
        }
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_status_distinguishes_completion_states() {
        assert!(BuildOutputStatus::Complete.is_complete());
        assert!(BuildOutputStatus::Partial.is_partial());
        assert!(BuildOutputStatus::Incomplete.is_incomplete());

        assert!(
            BuildOutputStatus::Complete.has_successful_realization()
        );

        assert!(
            BuildOutputStatus::Partial.has_successful_realization()
        );

        assert!(
            !BuildOutputStatus::Incomplete.has_successful_realization()
        );
    }

    #[test]
    fn target_coverage_identifies_missing_artifacts() {
        let covered = TargetOutputCoverage {
            target_id: "target.example".to_owned(),
            artifact_ids: vec![
                ArtifactId::new("artifact.example")
                    .expect("valid artifact identity"),
            ],
        };

        let missing = TargetOutputCoverage {
            target_id: "target.other".to_owned(),
            artifact_ids: Vec::new(),
        };

        assert!(covered.is_covered());
        assert_eq!(covered.artifact_count(), 1);

        assert!(!missing.is_covered());
        assert_eq!(missing.artifact_count(), 0);
    }

    #[test]
    fn output_status_display_is_stable() {
        assert_eq!(
            BuildOutputStatus::Complete.to_string(),
            "complete"
        );

        assert_eq!(
            BuildOutputStatus::Partial.to_string(),
            "partial"
        );

        assert_eq!(
            BuildOutputStatus::Incomplete.to_string(),
            "incomplete"
        );
    }

    #[test]
    fn payload_size_overflow_is_not_wrapped() {
        assert_eq!(u64::MAX.checked_add(1), None);
    }

    #[test]
    fn unknown_payload_size_is_not_zero() {
        let size: Option<u64> = None;

        assert_eq!(size, None);
        assert_ne!(size, Some(0));
    }

    #[test]
    fn artifact_kind_and_format_are_extensible() {
        let kind = ArtifactKindId::new(
            "future.registered.computational.domain"
        )
        .expect("valid extensible artifact kind");

        let format = ArtifactFormatId::new(
            "application/x-future-computational-format"
        )
        .expect("valid extensible artifact format");

        assert_eq!(
            kind.as_str(),
            "future.registered.computational.domain"
        );

        assert_eq!(
            format.as_str(),
            "application/x-future-computational-format"
        );
    }

    #[test]
    fn canonical_producer_identity_is_valid() {
        let producer = ArtifactProducerId::new(
            "zamani.toolchain.build.outputs"
        )
        .expect("valid producer identity");

        assert_eq!(
            producer.as_str(),
            "zamani.toolchain.build.outputs"
        );
    }
}
