
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani Compiler — Artifact Portability
//!
//! Canonical path: `src/compiler/artifacts/portability.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: Unsafe Rust is forbidden.
//!
//! PURPOSE
//!
//! Classify and assess the portability of compiler artifacts and artifact
//! manifests without confusing target independence with missing target data.
//!
//! OWNERSHIP
//!
//! This module owns:
//! - Portability declarations.
//! - Portability evidence references.
//! - Portability classifications.
//! - Artifact portability assessments.
//! - Manifest-wide portability aggregation.
//! - Configurable acceptance policies.
//! - Structured findings and reports.
//!
//! This module does not own:
//! - Artifact or manifest storage.
//! - Metadata storage.
//! - Semantic or IR verification.
//! - Target discovery or capability registries.
//! - Backend selection or target lowering.
//! - Physical resource allocation.
//! - Runtime execution.
//! - Stable artifact serialization.
//!
//! CORE PORTABILITY RULE
//!
//! An artifact without a target reference is not automatically portable.
//!
//! Portability must be explicitly declared or established by evidence
//! supplied by the appropriate authoritative verification process.
//!
//! A portability evidence reference records the basis of an assessment.
//! This module does not independently authenticate that evidence.
//!
//! SCALABILITY
//!
//! No universal maximum is imposed on:
//! - Artifacts.
//! - Manifest entries.
//! - Target architectures.
//! - Computing domains.
//! - Hardware devices.
//! - Resource populations.
//! - Identifier lengths.
//!
//! Dynamic collections grow according to available resources.
//! Explicit policy limits may be supplied by callers.
//!
//! Literal infinite computation is not physically realizable.
//!
//! INTEGRATION
//!
//! artifact.rs:
//!     Supplies Artifact, ArtifactId, ArtifactDescriptor and
//!     ArtifactTargetRef.
//!
//! manifest.rs:
//!     Supplies ArtifactManifest and its deterministic artifact iterator.
//!
//! metadata.rs:
//!     Owns metadata conventions. Metadata values are not proof of portability.
//!
//! validation.rs:
//!     Owns artifact integrity and validation evidence.
//!
//! semantic and IR verification:
//!     Establish whether program meaning and representations are valid.
//!
//! backends:
//!     Supply target-specific realization and capability compatibility.
//!
//! runtime:
//!     Owns actual resource negotiation and execution.
//!
//! grammar/:
//!     Defines language semantics and portability requirements.
//!
//! This module is independent of downstream implementation modules.
//!
//! IMPORTANT
//!
//! A `VerifiedTargetIndependent` result is only as trustworthy as the
//! external authority that supplied its evidence. This module records
//! evidence references; it does not manufacture verification.

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;

use super::artifact::{
    Artifact,
    ArtifactId,
    ArtifactTargetRef,
};

use super::manifest::ArtifactManifest;

// ============================================================================
// Constants
// ============================================================================

/// Stable identifier for this portability assessment contract.
pub const PORTABILITY_CONTRACT_ID: &str =
    "zamani.compiler.artifact.portability";

/// Initial major version of this portability contract.
pub const PORTABILITY_CONTRACT_MAJOR: u32 = 1;

/// Initial minor version of this portability contract.
pub const PORTABILITY_CONTRACT_MINOR: u32 = 0;

/// Initial patch version of this portability contract.
pub const PORTABILITY_CONTRACT_PATCH: u32 = 0;

// ============================================================================
// Errors
// ============================================================================

/// Errors raised by portability declarations and assessments.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum PortabilityError {
    /// An identifier is empty or otherwise invalid.
    InvalidIdentifier {
        field: &'static str,
        reason: String,
    },

    /// An explicit portability declaration conflicts with the artifact's
    /// target association.
    ContradictoryDeclaration {
        artifact: ArtifactId,
        reason: String,
    },

    /// A required portability evidence reference is missing.
    MissingEvidence {
        artifact: ArtifactId,
    },

    /// A portability policy cannot accept the assessed state.
    PolicyRejected {
        artifact: Option<ArtifactId>,
        reason: String,
    },

    /// A manifest assessment could not be completed.
    ManifestAssessment {
        reason: String,
    },
}

impl fmt::Display for PortabilityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidIdentifier { field, reason } => {
                write!(f, "invalid portability {field}: {reason}")
            }

            Self::ContradictoryDeclaration {
                artifact,
                reason,
            } => {
                write!(
                    f,
                    "contradictory portability declaration for \
                     artifact '{artifact}': {reason}"
                )
            }

            Self::MissingEvidence { artifact } => {
                write!(
                    f,
                    "verified portability evidence is missing for \
                     artifact '{artifact}'"
                )
            }

            Self::PolicyRejected { artifact, reason } => {
                if let Some(artifact) = artifact {
                    write!(
                        f,
                        "portability policy rejected artifact \
                         '{artifact}': {reason}"
                    )
                } else {
                    write!(
                        f,
                        "portability policy rejected manifest: {reason}"
                    )
                }
            }

            Self::ManifestAssessment { reason } => {
                write!(
                    f,
                    "manifest portability assessment failed: {reason}"
                )
            }
        }
    }
}

impl Error for PortabilityError {}

/// Result type for portability operations.
pub type PortabilityResult<T> = Result<T, PortabilityError>;

// ============================================================================
// Identifier validation
// ============================================================================

/// Validates an externally supplied portability identifier.
///
/// No arbitrary length limit is imposed.
fn validate_identifier(
    value: &str,
    field: &'static str,
) -> PortabilityResult<()> {
    if value.is_empty() {
        return Err(PortabilityError::InvalidIdentifier {
            field,
            reason: "identifier cannot be empty".to_owned(),
        });
    }

    if value.trim() != value {
        return Err(PortabilityError::InvalidIdentifier {
            field,
            reason: "leading or trailing whitespace is forbidden"
                .to_owned(),
        });
    }

    if value.chars().any(char::is_control) {
        return Err(PortabilityError::InvalidIdentifier {
            field,
            reason: "control characters are forbidden".to_owned(),
        });
    }

    Ok(())
}

// ============================================================================
// Evidence references
// ============================================================================

/// Identifies the authority that established portability evidence.
///
/// This is an extensible textual identifier, not a closed list of
/// compilers, verifiers, vendors, or organizations.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PortabilityAuthorityId(String);

impl PortabilityAuthorityId {
    /// Creates a validated authority identity.
    pub fn new(
        value: impl Into<String>,
    ) -> PortabilityResult<Self> {
        let value = value.into();

        validate_identifier(&value, "authority identifier")?;

        Ok(Self(value))
    }

    /// Returns the authority identity.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for PortabilityAuthorityId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Identifies externally maintained portability evidence.
///
/// The reference may identify a verification record, attestation,
/// conformance report, or another authoritative evidence object.
///
/// This module does not fetch, authenticate, or interpret the referenced
/// external object.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PortabilityEvidenceRef(String);

impl PortabilityEvidenceRef {
    /// Creates a validated evidence reference.
    pub fn new(
        value: impl Into<String>,
    ) -> PortabilityResult<Self> {
        let value = value.into();

        validate_identifier(&value, "evidence reference")?;

        Ok(Self(value))
    }

    /// Returns the reference.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for PortabilityEvidenceRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// ============================================================================
// Portability evidence
// ============================================================================

/// Explicit evidence supplied by an external authoritative verifier.
///
/// Constructing this value records an assertion made by the caller.
/// It does not independently prove the assertion.
///
/// The caller must only supply this evidence after the relevant
/// verification process has completed successfully.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PortabilityEvidence {
    authority: PortabilityAuthorityId,
    reference: PortabilityEvidenceRef,
}

impl PortabilityEvidence {
    /// Creates a reference to externally established portability evidence.
    pub fn new(
        authority: PortabilityAuthorityId,
        reference: PortabilityEvidenceRef,
    ) -> Self {
        Self {
            authority,
            reference,
        }
    }

    /// Returns the evidence authority.
    pub fn authority(&self) -> &PortabilityAuthorityId {
        &self.authority
    }

    /// Returns the evidence reference.
    pub fn reference(&self) -> &PortabilityEvidenceRef {
        &self.reference
    }
}

// ============================================================================
// Portability declaration
// ============================================================================

/// A producer's explicit declaration about an artifact's target scope.
///
/// Declarations describe intent. They are not equivalent to verification.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum PortabilityDeclaration {
    /// The producer declares that the artifact is target-independent.
    ///
    /// This declaration alone does not establish verified portability.
    TargetIndependent,

    /// The producer explicitly declares that the artifact is associated
    /// with a particular target.
    TargetSpecific {
        target: ArtifactTargetRef,
    },
}

impl PortabilityDeclaration {
    /// Creates a target-independent declaration.
    pub const fn target_independent() -> Self {
        Self::TargetIndependent
    }

    /// Creates a target-specific declaration.
    pub fn target_specific(
        target: ArtifactTargetRef,
    ) -> Self {
        Self::TargetSpecific { target }
    }
}

// ============================================================================
// Classification
// ============================================================================

/// Portability classification of an artifact or manifest.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum PortabilityClass {
    /// Insufficient information exists to establish portability.
    ///
    /// This is the default when no target or declaration is supplied.
    Unknown,

    /// The producer explicitly declared target independence.
    ///
    /// No independent verification has been established by this module.
    TargetIndependentDeclared,

    /// An external authority supplied evidence that the artifact is
    /// target-independent.
    TargetIndependentVerified,

    /// The artifact is associated with a concrete target.
    TargetSpecific,

    /// The artifact contains inconsistent portability information.
    Inconsistent,

    /// A manifest contains a mixture of otherwise valid portability states.
    Mixed,
}

impl PortabilityClass {
    /// Returns whether this classification represents verified
    /// target independence.
    pub const fn is_verified_portable(self) -> bool {
        matches!(self, Self::TargetIndependentVerified)
    }

    /// Returns whether the classification explicitly describes
    /// target independence.
    pub const fn is_declared_target_independent(self) -> bool {
        matches!(
            self,
            Self::TargetIndependentDeclared
                | Self::TargetIndependentVerified
        )
    }

    /// Returns whether the classification is unresolved.
    pub const fn is_unknown(self) -> bool {
        matches!(self, Self::Unknown)
    }

    /// Returns whether the classification contains contradictory data.
    pub const fn is_inconsistent(self) -> bool {
        matches!(self, Self::Inconsistent)
    }
}

impl fmt::Display for PortabilityClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::Unknown => "unknown",
            Self::TargetIndependentDeclared => {
                "target-independent-declared"
            }
            Self::TargetIndependentVerified => {
                "target-independent-verified"
            }
            Self::TargetSpecific => "target-specific",
            Self::Inconsistent => "inconsistent",
            Self::Mixed => "mixed",
        };

        f.write_str(name)
    }
}

// ============================================================================
// Artifact portability assessment
// ============================================================================

/// Result of assessing one artifact's portability.
///
/// This report does not replace semantic verification, IR verification,
/// content integrity verification, security validation, or target
/// capability matching.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArtifactPortability {
    artifact: ArtifactId,
    class: PortabilityClass,
    target: Option<ArtifactTargetRef>,
    evidence: Option<PortabilityEvidence>,
    explanation: String,
}

impl ArtifactPortability {
    /// Returns the artifact identity.
    pub fn artifact(&self) -> &ArtifactId {
        &self.artifact
    }

    /// Returns the portability classification.
    pub const fn class(&self) -> PortabilityClass {
        self.class
    }

    /// Returns the associated target, if present.
    pub fn target(&self) -> Option<&ArtifactTargetRef> {
        self.target.as_ref()
    }

    /// Returns the external evidence reference, if supplied.
    pub fn evidence(&self) -> Option<&PortabilityEvidence> {
        self.evidence.as_ref()
    }

    /// Returns the assessment explanation.
    pub fn explanation(&self) -> &str {
        &self.explanation
    }

    /// Returns whether the artifact has verified target independence.
    pub const fn is_verified_portable(&self) -> bool {
        self.class.is_verified_portable()
    }
}

// ============================================================================
// Artifact assessment
// ============================================================================

/// Assesses the portability of an artifact.
///
/// The artifact's target association is taken from its canonical descriptor.
/// The declaration and evidence are supplied explicitly by the caller.
///
/// Important:
///
/// - Missing target + missing declaration = Unknown.
/// - Missing target + declaration = TargetIndependentDeclared.
/// - Missing target + declaration + evidence = TargetIndependentVerified.
/// - Present target = TargetSpecific, unless the supplied declaration
///   contradicts that association.
/// - Metadata values are never interpreted as portability proof.
pub fn assess_artifact(
    artifact: &Artifact,
    declaration: Option<&PortabilityDeclaration>,
    evidence: Option<&PortabilityEvidence>,
) -> PortabilityResult<ArtifactPortability> {
    let descriptor = artifact.descriptor();
    let target = descriptor.target().cloned();

    let class = match (target.as_ref(), declaration) {
        (Some(target), Some(PortabilityDeclaration::TargetIndependent)) => {
            return Err(PortabilityError::ContradictoryDeclaration {
                artifact: artifact.id().clone(),
                reason: format!(
                    "artifact is associated with target '{target}' \
                     but declared target-independent"
                ),
            });
        }

        (
            Some(target),
            Some(PortabilityDeclaration::TargetSpecific {
                target: declared_target,
            }),
        ) if target != declared_target => {
            return Err(PortabilityError::ContradictoryDeclaration {
                artifact: artifact.id().clone(),
                reason: format!(
                    "descriptor target '{target}' differs from \
                     declared target '{declared_target}'"
                ),
            });
        }

        (Some(_), _) => PortabilityClass::TargetSpecific,

        (None, Some(PortabilityDeclaration::TargetSpecific { .. })) => {
            return Err(PortabilityError::ContradictoryDeclaration {
                artifact: artifact.id().clone(),
                reason: "target-specific declaration has no matching \
                         descriptor target"
                    .to_owned(),
            });
        }

        (None, Some(PortabilityDeclaration::TargetIndependent)) => {
            if evidence.is_some() {
                PortabilityClass::TargetIndependentVerified
            } else {
                PortabilityClass::TargetIndependentDeclared
            }
        }

        (None, None) => {
            if evidence.is_some() {
                return Err(PortabilityError::MissingEvidence {
                    artifact: artifact.id().clone(),
                });
            }

            PortabilityClass::Unknown
        }
    };

    let explanation = match class {
        PortabilityClass::Unknown => {
            "No target association or explicit portability declaration \
             was supplied."
                .to_owned()
        }

        PortabilityClass::TargetIndependentDeclared => {
            "The producer declared target independence, but no external \
             verification evidence was supplied."
                .to_owned()
        }

        PortabilityClass::TargetIndependentVerified => {
            "Target independence was declared and external verification \
             evidence was supplied by the caller."
                .to_owned()
        }

        PortabilityClass::TargetSpecific => {
            "The canonical artifact descriptor associates this artifact \
             with a concrete target."
                .to_owned()
        }

        PortabilityClass::Inconsistent => {
            "The portability information is inconsistent.".to_owned()
        }

        PortabilityClass::Mixed => {
            "The classification represents multiple artifact states."
                .to_owned()
        }
    };

    Ok(ArtifactPortability {
        artifact: artifact.id().clone(),
        class,
        target,
        evidence: evidence.cloned(),
        explanation,
    })
}

// ============================================================================
// Manifest evidence inputs
// ============================================================================

/// Per-artifact portability inputs for manifest assessment.
///
/// Inputs are indexed by canonical ArtifactId. Missing entries are not
/// interpreted as portable.
#[derive(Debug, Clone, Default)]
pub struct ManifestPortabilityInputs {
    declarations: BTreeMap<ArtifactId, PortabilityDeclaration>,
    evidence: BTreeMap<ArtifactId, PortabilityEvidence>,
}

impl ManifestPortabilityInputs {
    /// Creates empty portability inputs.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds or replaces an artifact declaration.
    pub fn set_declaration(
        &mut self,
        artifact: ArtifactId,
        declaration: PortabilityDeclaration,
    ) -> Option<PortabilityDeclaration> {
        self.declarations.insert(artifact, declaration)
    }

    /// Adds or replaces external evidence for an artifact.
    pub fn set_evidence(
        &mut self,
        artifact: ArtifactId,
        evidence: PortabilityEvidence,
    ) -> Option<PortabilityEvidence> {
        self.evidence.insert(artifact, evidence)
    }

    /// Returns a declaration for an artifact.
    pub fn declaration(
        &self,
        artifact: &ArtifactId,
    ) -> Option<&PortabilityDeclaration> {
        self.declarations.get(artifact)
    }

    /// Returns evidence for an artifact.
    pub fn evidence(
        &self,
        artifact: &ArtifactId,
    ) -> Option<&PortabilityEvidence> {
        self.evidence.get(artifact)
    }

    /// Returns the number of declarations.
    pub fn declaration_count(&self) -> usize {
        self.declarations.len()
    }

    /// Returns the number of evidence records.
    pub fn evidence_count(&self) -> usize {
        self.evidence.len()
    }
}

// ============================================================================
// Manifest portability report
// ============================================================================

/// Aggregate counts for a manifest portability assessment.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct PortabilityCounts {
    unknown: usize,
    declared: usize,
    verified: usize,
    target_specific: usize,
    inconsistent: usize,
}

impl PortabilityCounts {
    /// Returns the number of unknown artifacts.
    pub const fn unknown(&self) -> usize {
        self.unknown
    }

    /// Returns the number of declared-portable artifacts.
    pub const fn declared(&self) -> usize {
        self.declared
    }

    /// Returns the number of verified-portable artifacts.
    pub const fn verified(&self) -> usize {
        self.verified
    }

    /// Returns the number of target-specific artifacts.
    pub const fn target_specific(&self) -> usize {
        self.target_specific
    }

    /// Returns the number of inconsistent artifacts.
    pub const fn inconsistent(&self) -> usize {
        self.inconsistent
    }

    /// Returns the total number of assessed artifacts.
    ///
    /// Saturating arithmetic prevents counter overflow from wrapping.
    pub const fn total(&self) -> usize {
        self.unknown
            .saturating_add(self.declared)
            .saturating_add(self.verified)
            .saturating_add(self.target_specific)
            .saturating_add(self.inconsistent)
    }
}

/// Aggregate portability report for an artifact manifest.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManifestPortabilityReport {
    manifest: String,
    class: PortabilityClass,
    assessments: Vec<ArtifactPortability>,
    counts: PortabilityCounts,
}

impl ManifestPortabilityReport {
    /// Returns the manifest identity.
    pub fn manifest(&self) -> &str {
        &self.manifest
    }

    /// Returns the aggregate classification.
    pub const fn class(&self) -> PortabilityClass {
        self.class
    }

    /// Returns all artifact assessments in manifest iteration order.
    pub fn assessments(&self) -> &[ArtifactPortability] {
        &self.assessments
    }

    /// Returns the aggregate counts.
    pub const fn counts(&self) -> PortabilityCounts {
        self.counts
    }

    /// Returns whether every artifact is verified target-independent.
    pub fn all_verified_portable(&self) -> bool {
        self.counts.verified == self.assessments.len()
    }

    /// Returns whether any artifact is unresolved.
    pub const fn contains_unknown(&self) -> bool {
        self.counts.unknown > 0
    }

    /// Returns whether any artifact is target-specific.
    pub const fn contains_target_specific(&self) -> bool {
        self.counts.target_specific > 0
    }

    /// Returns whether any artifact has contradictory information.
    pub const fn contains_inconsistent(&self) -> bool {
        self.counts.inconsistent > 0
    }
}

// ============================================================================
// Manifest assessment
// ============================================================================

/// Assesses portability for every artifact in a canonical manifest.
///
/// The manifest's artifact iterator is deterministic. Therefore, the
/// resulting assessment order is deterministic for identical manifests.
///
/// This operation does not verify manifest structural validity. Call
/// `ArtifactManifest::validate()` separately and require successful
/// structural validation before treating the manifest as deployable.
pub fn assess_manifest(
    manifest: &ArtifactManifest,
    inputs: &ManifestPortabilityInputs,
) -> PortabilityResult<ManifestPortabilityReport> {
    let mut assessments = Vec::new();
    let mut counts = PortabilityCounts::default();

    for (artifact_id, artifact) in manifest.artifacts() {
        let declaration = inputs.declaration(artifact_id);
        let evidence = inputs.evidence(artifact_id);

        let assessment = assess_artifact(
            artifact,
            declaration,
            evidence,
        )?;

        match assessment.class() {
            PortabilityClass::Unknown => {
                counts.unknown = counts.unknown.saturating_add(1);
            }

            PortabilityClass::TargetIndependentDeclared => {
                counts.declared = counts.declared.saturating_add(1);
            }

            PortabilityClass::TargetIndependentVerified => {
                counts.verified = counts.verified.saturating_add(1);
            }

            PortabilityClass::TargetSpecific => {
                counts.target_specific =
                    counts.target_specific.saturating_add(1);
            }

            PortabilityClass::Inconsistent => {
                counts.inconsistent =
                    counts.inconsistent.saturating_add(1);
            }

            PortabilityClass::Mixed => {
                counts.inconsistent =
                    counts.inconsistent.saturating_add(1);
            }
        }

        assessments.push(assessment);
    }

    let class = aggregate_classification(&counts);

    Ok(ManifestPortabilityReport {
        manifest: manifest.id().to_string(),
        class,
        assessments,
        counts,
    })
}

/// Computes the aggregate classification from artifact counts.
fn aggregate_classification(
    counts: &PortabilityCounts,
) -> PortabilityClass {
    if counts.inconsistent > 0 {
        return PortabilityClass::Inconsistent;
    }

    let total = counts.total();

    if total == 0 {
        return PortabilityClass::Unknown;
    }

    let known_states = usize::from(counts.declared > 0)
        .saturating_add(usize::from(counts.verified > 0))
        .saturating_add(usize::from(counts.target_specific > 0))
        .saturating_add(usize::from(counts.unknown > 0));

    if known_states > 1 {
        return PortabilityClass::Mixed;
    }

    if counts.unknown > 0 {
        return PortabilityClass::Unknown;
    }

    if counts.target_specific > 0 {
        return PortabilityClass::TargetSpecific;
    }

    if counts.declared > 0 {
        return PortabilityClass::TargetIndependentDeclared;
    }

    if counts.verified == total {
        return PortabilityClass::TargetIndependentVerified;
    }

    PortabilityClass::Unknown
}

// ============================================================================
// Acceptance policy
// ============================================================================

/// Policy controlling which portability states are acceptable.
///
/// All options default to conservative behavior.
///
/// These settings are operational acceptance policies. They do not change
/// the semantic meaning of an artifact.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PortabilityPolicy {
    allow_verified_portable: bool,
    allow_declared_portable: bool,
    allow_target_specific: bool,
    allow_unknown: bool,
}

impl Default for PortabilityPolicy {
    fn default() -> Self {
        Self {
            allow_verified_portable: true,
            allow_declared_portable: false,
            allow_target_specific: false,
            allow_unknown: false,
        }
    }
}

impl PortabilityPolicy {
    /// Creates the conservative default policy.
    pub const fn conservative() -> Self {
        Self {
            allow_verified_portable: true,
            allow_declared_portable: false,
            allow_target_specific: false,
            allow_unknown: false,
        }
    }

    /// Creates a policy that accepts declarations without requiring
    /// external verification.
    pub const fn allow_declared() -> Self {
        Self {
            allow_verified_portable: true,
            allow_declared_portable: true,
            allow_target_specific: false,
            allow_unknown: false,
        }
    }

    /// Creates a policy that permits target-specific artifacts.
    ///
    /// This does not establish that those artifacts are compatible with
    /// any particular target.
    pub const fn allow_target_specific(mut self, allow: bool) -> Self {
        self.allow_target_specific = allow;
        self
    }

    /// Controls whether unknown portability states are accepted.
    ///
    /// This should only be enabled for workflows that intentionally defer
    /// portability determination to a later authoritative stage.
    pub const fn allow_unknown(mut self, allow: bool) -> Self {
        self.allow_unknown = allow;
        self
    }

    /// Controls whether declared-but-unverified portable artifacts
    /// are accepted.
    pub const fn allow_declared_portable(
        mut self,
        allow: bool,
    ) -> Self {
        self.allow_declared_portable = allow;
        self
    }

    /// Controls whether verified portable artifacts are accepted.
    pub const fn allow_verified_portable(
        mut self,
        allow: bool,
    ) -> Self {
        self.allow_verified_portable = allow;
        self
    }

    /// Returns whether a classification is accepted by this policy.
    pub const fn accepts(
        &self,
        class: PortabilityClass,
    ) -> bool {
        match class {
            PortabilityClass::TargetIndependentVerified => {
                self.allow_verified_portable
            }

            PortabilityClass::TargetIndependentDeclared => {
                self.allow_declared_portable
            }

            PortabilityClass::TargetSpecific => {
                self.allow_target_specific
            }

            PortabilityClass::Unknown => self.allow_unknown,

            PortabilityClass::Inconsistent | PortabilityClass::Mixed => {
                false
            }
        }
    }

    /// Evaluates an artifact assessment.
    pub fn evaluate_artifact(
        &self,
        assessment: &ArtifactPortability,
    ) -> PortabilityResult<()> {
        if self.accepts(assessment.class()) {
            return Ok(());
        }

        Err(PortabilityError::PolicyRejected {
            artifact: Some(assessment.artifact().clone()),
            reason: format!(
                "classification '{}' is not accepted",
                assessment.class()
            ),
        })
    }

    /// Evaluates a whole-manifest assessment.
    ///
    /// Every artifact must independently satisfy the policy.
    pub fn evaluate_manifest(
        &self,
        report: &ManifestPortabilityReport,
    ) -> PortabilityResult<()> {
        for assessment in report.assessments() {
            self.evaluate_artifact(assessment)?;
        }

        Ok(())
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::compiler::artifacts::artifact::{
        ArtifactDescriptor,
        ArtifactFormatId,
        ArtifactId,
        ArtifactKindId,
        ArtifactProducerId,
        ArtifactProvenance,
    };

    use crate::compiler::artifacts::manifest::{
        ArtifactManifest,
        ManifestId,
    };

    fn make_artifact(
        id: &str,
        target: Option<&str>,
    ) -> Artifact {
        let producer = ArtifactProducerId::new("test.producer")
            .expect("valid producer");

        let provenance = ArtifactProvenance::new(producer);

        let descriptor = ArtifactDescriptor::new(
            ArtifactId::new(id).expect("valid artifact ID"),
            ArtifactKindId::new("portable-ir")
                .expect("valid artifact kind"),
            ArtifactFormatId::new("application/x-zamani-test")
                .expect("valid artifact format"),
            provenance,
        );

        let descriptor = match target {
            Some(target) => descriptor.with_target(
                crate::compiler::artifacts::artifact::ArtifactTargetRef::new(
                    target,
                )
                .expect("valid target"),
            ),
            None => descriptor,
        };

        Artifact::new_inline(descriptor, Vec::<u8>::new())
    }

    fn make_evidence() -> PortabilityEvidence {
        PortabilityEvidence::new(
            PortabilityAuthorityId::new("zamani.test.verifier")
                .expect("valid authority"),
            PortabilityEvidenceRef::new("verification-record-1")
                .expect("valid evidence reference"),
        )
    }

    #[test]
    fn missing_target_does_not_establish_portability() {
        let artifact = make_artifact("artifact.unknown", None);

        let assessment = assess_artifact(
            &artifact,
            None,
            None,
        )
        .expect("assessment succeeds");

        assert_eq!(
            assessment.class(),
            PortabilityClass::Unknown
        );

        assert!(!assessment.is_verified_portable());
    }

    #[test]
    fn explicit_declaration_is_not_verified_evidence() {
        let artifact = make_artifact("artifact.declared", None);

        let declaration =
            PortabilityDeclaration::target_independent();

        let assessment = assess_artifact(
            &artifact,
            Some(&declaration),
            None,
        )
        .expect("assessment succeeds");

        assert_eq!(
            assessment.class(),
            PortabilityClass::TargetIndependentDeclared
        );

        assert!(!assessment.is_verified_portable());
    }

    #[test]
    fn evidence_establishes_verified_classification() {
        let artifact = make_artifact("artifact.verified", None);

        let declaration =
            PortabilityDeclaration::target_independent();

        let evidence = make_evidence();

        let assessment = assess_artifact(
            &artifact,
            Some(&declaration),
            Some(&evidence),
        )
        .expect("assessment succeeds");

        assert_eq!(
            assessment.class(),
            PortabilityClass::TargetIndependentVerified
        );

        assert!(assessment.is_verified_portable());
    }

    #[test]
    fn target_association_is_target_specific() {
        let artifact = make_artifact(
            "artifact.native",
            Some("target.example.cpu"),
        );

        let assessment = assess_artifact(
            &artifact,
            None,
            None,
        )
        .expect("assessment succeeds");

        assert_eq!(
            assessment.class(),
            PortabilityClass::TargetSpecific
        );
    }

    #[test]
    fn target_and_independent_declaration_conflict() {
        let artifact = make_artifact(
            "artifact.conflict",
            Some("target.example.cpu"),
        );

        let declaration =
            PortabilityDeclaration::target_independent();

        let result = assess_artifact(
            &artifact,
            Some(&declaration),
            None,
        );

        assert!(matches!(
            result,
            Err(PortabilityError::ContradictoryDeclaration { .. })
        ));
    }

    #[test]
    fn target_specific_declaration_must_match_descriptor() {
        let artifact = make_artifact(
            "artifact.targeted",
            Some("target.example.cpu"),
        );

        let declared_target =
            ArtifactTargetRef::new("target.example.gpu")
                .expect("valid target");

        let declaration =
            PortabilityDeclaration::target_specific(declared_target);

        let result = assess_artifact(
            &artifact,
            Some(&declaration),
            None,
        );

        assert!(matches!(
            result,
            Err(PortabilityError::ContradictoryDeclaration { .. })
        ));
    }

    #[test]
    fn conservative_policy_rejects_unknown() {
        let artifact = make_artifact("artifact.unknown", None);

        let assessment = assess_artifact(
            &artifact,
            None,
            None,
        )
        .expect("assessment succeeds");

        assert!(
            PortabilityPolicy::conservative()
                .evaluate_artifact(&assessment)
                .is_err()
        );
    }

    #[test]
    fn conservative_policy_accepts_verified_portability() {
        let artifact = make_artifact("artifact.verified", None);

        let declaration =
            PortabilityDeclaration::target_independent();

        let evidence = make_evidence();

        let assessment = assess_artifact(
            &artifact,
            Some(&declaration),
            Some(&evidence),
        )
        .expect("assessment succeeds");

        assert!(
            PortabilityPolicy::conservative()
                .evaluate_artifact(&assessment)
                .is_ok()
        );
    }

    #[test]
    fn identifiers_have_no_arbitrary_length_ceiling() {
        let long_value = "x".repeat(16_384);

        let authority =
            PortabilityAuthorityId::new(long_value.clone())
                .expect("long authority identifier is accepted");

        assert_eq!(authority.as_str(), long_value);
    }

    #[test]
    fn manifest_aggregates_verified_artifacts() {
        let producer = ArtifactProducerId::new("test.producer")
            .expect("valid producer");

        let mut manifest = ArtifactManifest::new(
            ManifestId::new("manifest.test")
                .expect("valid manifest ID"),
            ArtifactProvenance::new(producer),
        );

        let artifact = make_artifact("artifact.verified", None);
        let artifact_id = artifact.id().clone();

        manifest
            .insert_artifact(artifact)
            .expect("artifact insertion succeeds");

        let mut inputs = ManifestPortabilityInputs::new();

        inputs.set_declaration(
            artifact_id.clone(),
            PortabilityDeclaration::target_independent(),
        );

        inputs.set_evidence(artifact_id, make_evidence());

        let report = assess_manifest(&manifest, &inputs)
            .expect("manifest assessment succeeds");

        assert_eq!(
            report.class(),
            PortabilityClass::TargetIndependentVerified
        );

        assert!(report.all_verified_portable());
        assert_eq!(report.counts().verified(), 1);
    }

    #[test]
    fn manifest_does_not_assume_missing_inputs_are_portable() {
        let producer = ArtifactProducerId::new("test.producer")
            .expect("valid producer");

        let mut manifest = ArtifactManifest::new(
            ManifestId::new("manifest.unknown")
                .expect("valid manifest ID"),
            ArtifactProvenance::new(producer),
        );

        manifest
            .insert_artifact(make_artifact("artifact.unknown", None))
            .expect("artifact insertion succeeds");

        let report = assess_manifest(
            &manifest,
            &ManifestPortabilityInputs::new(),
        )
        .expect("manifest assessment succeeds");

        assert_eq!(report.class(), PortabilityClass::Unknown);
        assert!(report.contains_unknown());
        assert!(!report.all_verified_portable());
    }

    #[test]
    fn empty_manifest_is_not_claimed_portable() {
        let producer = ArtifactProducerId::new("test.producer")
            .expect("valid producer");

        let manifest = ArtifactManifest::new(
            ManifestId::new("manifest.empty")
                .expect("valid manifest ID"),
            ArtifactProvenance::new(producer),
        );

        let report = assess_manifest(
            &manifest,
            &ManifestPortabilityInputs::new(),
        )
        .expect("manifest assessment succeeds");

        assert_eq!(report.class(), PortabilityClass::Unknown);
        assert!(!report.all_verified_portable());
    }
}
