
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Compiler — Independent Reproducibility Verification.
//!
//! Path:
//!     src/compiler/reproducibility/verification.rs
//!
//! Rust:
//!     Edition: 2021
//!     Minimum supported version: Rust 1.97
//!
//! # Purpose
//!
//! Independently verify that two compilation executions, using equivalent
//! declared compilation inputs and environments, produce equivalent
//! validated compilation artifacts.
//!
//! This module coordinates existing reproducibility authorities.
//!
//! It does not implement another hashing algorithm, artifact format,
//! provenance ledger, determinism policy, semantic model or IR.
//!
//! # Canonical authorities
//!
//! Compilation inputs:
//!     crate::compiler::reproducibility::inputs
//!
//! Compilation environment:
//!     crate::compiler::reproducibility::environment
//!
//! Determinism assessment:
//!     crate::compiler::reproducibility::determinism
//!
//! Provenance fingerprint:
//!     crate::compiler::reproducibility::provenance
//!
//! Canonical fingerprint:
//!     crate::zuir::fingerprint::stable_hash
//!
//! Artifact validation:
//!     crate::compiler::artifacts::validation
//!
//! Artifact identity:
//!     crate::compiler::artifacts
//!
//! Provenance:
//!     crate::compiler::context::provenance_context
//!
//! Security:
//!     crate::compiler::security
//!
//! # Ownership
//!
//! This module owns:
//!
//! - Independent build identity.
//! - Reproducibility evidence snapshots.
//! - Artifact-output comparison.
//! - Independent build comparison.
//! - Verification findings.
//! - Verification decisions.
//! - Verification reports.
//! - Verification policies.
//!
//! This module does not own:
//!
//! - Source parsing.
//! - Semantic interpretation.
//! - Dependency resolution.
//! - Input fingerprint construction.
//! - Environment discovery.
//! - Cryptographic hashing.
//! - Digital signatures.
//! - Artifact serialization.
//! - Artifact payload hashing.
//! - Provenance ledger construction.
//! - Determinism policy.
//! - Numerical reproducibility.
//! - IR verification.
//! - Backend implementation.
//! - Physical resource allocation.
//! - Runtime execution.
//!
//! # Reproducibility
//!
//! Deterministic ordering is not sufficient to establish reproducibility.
//!
//! Identical source text alone is not sufficient.
//!
//! Identical configuration alone is not sufficient.
//!
//! A successful verification requires equivalent declared inputs,
//! compatible compilation environments, valid determinism evidence,
//! valid artifact validation and identical corresponding artifact
//! fingerprints.
//!
//! # POCO-REAF
//!
//! No arbitrary universal hardware ceilings are introduced.
//!
//! Portable compilation is independent of incidental physical machine
//! identity.
//!
//! Target-specific compilation remains explicitly target-specific.
//!
//! Actual physical resource limitations remain the responsibility of
//! the relevant target and runtime authorities.
//!
//! Literal infinite physical resources are not promised.
//!
//! # Security
//!
//! Fingerprint equality establishes content identity, not authenticity.
//!
//! This module does not authenticate signing keys, providers or artifacts.
//!
//! Those operations remain the responsibility of compiler security and
//! supply-chain verification.
//!
//! Callers must provide artifact fingerprints produced by the canonical
//! artifact subsystem, not fingerprints of untrusted display metadata.

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;

use crate::compiler::artifacts::validation::{
    ValidationReport,
    ValidationStatus,
};

use crate::compiler::reproducibility::determinism::{
    DeterminismAssessment,
};

use crate::compiler::reproducibility::inputs::{
    CompilationInputs,
};

use crate::compiler::reproducibility::provenance::{
    ProvenanceFingerprint,
};

use crate::zuir::fingerprint::stable_hash::Fingerprint;

// ============================================================================
// API VERSION
// ============================================================================

/// Public API version of reproducibility verification.
pub const REPRODUCIBILITY_VERIFICATION_API_VERSION: u16 = 1;

/// Version of the reproducibility verification contract.
///
/// Increment when the meaning of verification evidence, comparison
/// requirements or acceptance decisions changes incompatibly.
pub const REPRODUCIBILITY_VERIFICATION_SCHEMA_VERSION: u32 = 1;

/// Stable identity of this verification contract.
pub const REPRODUCIBILITY_VERIFICATION_CONTRACT_ID: &str =
    "zamani.compiler.reproducibility.verification.v1";

// ============================================================================
// BUILD IDENTITY
// ============================================================================

/// Stable identity of one independent compilation execution.
///
/// This is an opaque logical identifier, not a process ID, timestamp,
/// memory address or physical machine identifier.
///
/// Independent builds must have different identifiers.
///
/// Identifier uniqueness is established by the caller's build orchestration
/// authority.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BuildInstanceId(String);

impl BuildInstanceId {
    /// Creates a validated build identity.
    pub fn new(
        value: impl Into<String>,
    ) -> Result<Self, ReproducibilityVerificationError> {
        let value = value.into();

        validate_identifier(
            &value,
            "build instance identifier",
        )?;

        Ok(Self(value))
    }

    /// Returns the exact identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for BuildInstanceId {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// ============================================================================
// ARTIFACT OUTPUT IDENTITY
// ============================================================================

/// Stable logical identifier for an output artifact.
///
/// The identifier must originate from the canonical artifact subsystem.
///
/// This module does not establish artifact identity or artifact semantics.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ArtifactOutputId(String);

impl ArtifactOutputId {
    /// Creates an artifact output reference.
    pub fn new(
        value: impl Into<String>,
    ) -> Result<Self, ReproducibilityVerificationError> {
        let value = value.into();

        validate_identifier(
            &value,
            "artifact output identifier",
        )?;

        Ok(Self(value))
    }

    /// Returns the exact artifact identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ArtifactOutputId {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// ============================================================================
// ARTIFACT OUTPUT
// ============================================================================

/// Fingerprint of one independently validated artifact output.
///
/// The fingerprint must be supplied by the authoritative artifact
/// fingerprinting implementation.
///
/// This structure does not hash artifact payloads itself.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArtifactOutput {
    identifier: ArtifactOutputId,
    fingerprint: Fingerprint,
}

impl ArtifactOutput {
    /// Creates an artifact output evidence record.
    #[must_use]
    pub const fn new(
        identifier: ArtifactOutputId,
        fingerprint: Fingerprint,
    ) -> Self {
        Self {
            identifier,
            fingerprint,
        }
    }

    /// Returns the artifact identifier.
    #[must_use]
    pub fn identifier(&self) -> &ArtifactOutputId {
        &self.identifier
    }

    /// Returns the canonical artifact fingerprint.
    #[must_use]
    pub const fn fingerprint(&self) -> Fingerprint {
        self.fingerprint
    }
}

// ============================================================================
// ARTIFACT COLLECTION
// ============================================================================

/// Deterministically ordered collection of compilation outputs.
///
/// Artifact identity and artifact content fingerprints originate from
/// the canonical artifact subsystem.
///
/// Duplicate artifact identifiers are rejected.
///
/// No arbitrary artifact-count ceiling is introduced.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ArtifactOutputs {
    outputs: BTreeMap<ArtifactOutputId, Fingerprint>,
}

impl ArtifactOutputs {
    /// Creates an empty artifact collection.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts an artifact fingerprint.
    ///
    /// Duplicate identifiers are rejected even when their fingerprints
    /// happen to be identical.
    pub fn insert(
        &mut self,
        output: ArtifactOutput,
    ) -> Result<(), ReproducibilityVerificationError> {
        let identifier = output.identifier;

        if self.outputs.contains_key(&identifier) {
            return Err(
                ReproducibilityVerificationError::DuplicateArtifact {
                    identifier: identifier.to_string(),
                },
            );
        }

        self.outputs.insert(
            identifier,
            output.fingerprint,
        );

        Ok(())
    }

    /// Returns the number of artifacts.
    #[must_use]
    pub fn len(&self) -> usize {
        self.outputs.len()
    }

    /// Returns whether the collection is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.outputs.is_empty()
    }

    /// Returns the fingerprint of an artifact.
    #[must_use]
    pub fn get(
        &self,
        identifier: &ArtifactOutputId,
    ) -> Option<Fingerprint> {
        self.outputs.get(identifier).copied()
    }

    /// Iterates artifacts in deterministic identifier order.
    pub fn iter(
        &self,
    ) -> impl ExactSizeIterator<Item = (&ArtifactOutputId, &Fingerprint)> {
        self.outputs.iter()
    }
}

// ============================================================================
// BUILD EVIDENCE
// ============================================================================

/// Immutable evidence snapshot from one compilation execution.
///
/// All component identities are obtained from their existing authoritative
/// compiler subsystems.
///
/// A snapshot is independent of subsequent modifications to the original
/// compilation context.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuildEvidence {
    build_id: BuildInstanceId,

    input_fingerprint: Fingerprint,

    environment_fingerprint: Fingerprint,

    provenance: ProvenanceFingerprint,

    determinism_satisfied: bool,

    artifact_validation: ValidationStatus,

    artifacts: ArtifactOutputs,
}

impl BuildEvidence {
    /// Constructs evidence for one compilation execution.
    ///
    /// The input fingerprint and environment fingerprint are extracted
    /// from the canonical CompilationInputs object.
    ///
    /// The determinism assessment is consumed from the existing
    /// compiler-wide determinism authority.
    ///
    /// The provenance fingerprint is consumed from the existing
    /// provenance fingerprinting subsystem.
    ///
    /// Artifact validation status originates from the canonical
    /// artifact validation subsystem.
    pub fn new(
        build_id: BuildInstanceId,
        inputs: &CompilationInputs,
        provenance: ProvenanceFingerprint,
        determinism: &DeterminismAssessment,
        artifact_validation: &ValidationReport,
        artifacts: ArtifactOutputs,
    ) -> Self {
        Self {
            build_id,

            input_fingerprint: inputs.fingerprint(),

            environment_fingerprint: inputs
                .environment()
                .fingerprint(),

            provenance,

            determinism_satisfied: determinism
                .contract_satisfied(),

            artifact_validation: artifact_validation.status(),

            artifacts,
        }
    }

    /// Returns the independent build identity.
    #[must_use]
    pub fn build_id(&self) -> &BuildInstanceId {
        &self.build_id
    }

    /// Returns the canonical compilation-input fingerprint.
    #[must_use]
    pub const fn input_fingerprint(&self) -> Fingerprint {
        self.input_fingerprint
    }

    /// Returns the compilation-environment fingerprint.
    #[must_use]
    pub const fn environment_fingerprint(&self) -> Fingerprint {
        self.environment_fingerprint
    }

    /// Returns the provenance evidence.
    #[must_use]
    pub const fn provenance(&self) -> &ProvenanceFingerprint {
        &self.provenance
    }

    /// Returns whether the determinism contract was satisfied.
    #[must_use]
    pub const fn determinism_satisfied(&self) -> bool {
        self.determinism_satisfied
    }

    /// Returns the artifact validation status.
    #[must_use]
    pub const fn artifact_validation_status(&self) -> ValidationStatus {
        self.artifact_validation
    }

    /// Returns the artifact collection.
    #[must_use]
    pub const fn artifacts(&self) -> &ArtifactOutputs {
        &self.artifacts
    }
}

// ============================================================================
// VERIFICATION POLICY
// ============================================================================

/// Controls the requirements for accepting a reproducibility comparison.
///
/// These settings do not change language semantics.
///
/// They control the evidence required to claim successful independent
/// compilation reproducibility.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReproducibilityVerificationPolicy {
    require_determinism: bool,

    require_valid_artifacts: bool,

    require_provenance: bool,

    require_nonempty_outputs: bool,
}

impl Default for ReproducibilityVerificationPolicy {
    fn default() -> Self {
        Self::strict()
    }
}

impl ReproducibilityVerificationPolicy {
    /// Creates the strict production verification policy.
    ///
    /// Every mandatory reproducibility condition is enabled.
    #[must_use]
    pub const fn strict() -> Self {
        Self {
            require_determinism: true,
            require_valid_artifacts: true,
            require_provenance: true,
            require_nonempty_outputs: true,
        }
    }

    /// Creates a configurable verification policy.
    ///
    /// Relaxing a policy does not establish the guarantees removed
    /// by that relaxation.
    #[must_use]
    pub const fn new() -> Self {
        Self::strict()
    }

    /// Configures determinism evidence requirements.
    #[must_use]
    pub const fn with_determinism(
        mut self,
        required: bool,
    ) -> Self {
        self.require_determinism = required;
        self
    }

    /// Configures artifact validation requirements.
    #[must_use]
    pub const fn with_artifact_validation(
        mut self,
        required: bool,
    ) -> Self {
        self.require_valid_artifacts = required;
        self
    }

    /// Configures provenance requirements.
    #[must_use]
    pub const fn with_provenance(
        mut self,
        required: bool,
    ) -> Self {
        self.require_provenance = required;
        self
    }

    /// Configures whether at least one output artifact is required.
    #[must_use]
    pub const fn with_nonempty_outputs(
        mut self,
        required: bool,
    ) -> Self {
        self.require_nonempty_outputs = required;
        self
    }

    /// Returns the determinism requirement.
    #[must_use]
    pub const fn requires_determinism(&self) -> bool {
        self.require_determinism
    }

    /// Returns the artifact validation requirement.
    #[must_use]
    pub const fn requires_valid_artifacts(&self) -> bool {
        self.require_valid_artifacts
    }

    /// Returns the provenance requirement.
    #[must_use]
    pub const fn requires_provenance(&self) -> bool {
        self.require_provenance
    }

    /// Returns the nonempty-output requirement.
    #[must_use]
    pub const fn requires_nonempty_outputs(&self) -> bool {
        self.require_nonempty_outputs
    }
}

// ============================================================================
// VERIFICATION FINDINGS
// ============================================================================

/// Classification of a reproducibility verification finding.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ReproducibilityFindingKind {
    /// Both builds have the same build identity.
    ///
    /// The two executions therefore cannot be established as independent
    /// using their supplied identities.
    NonIndependentBuilds,

    /// Compilation input fingerprints differ.
    InputMismatch,

    /// Compilation environment fingerprints differ.
    EnvironmentMismatch,

    /// A required determinism assessment was not satisfied.
    DeterminismNotEstablished,

    /// Artifact validation rejected one or both builds.
    ArtifactValidationFailed,

    /// Artifact validation did not complete.
    ArtifactValidationIncomplete,

    /// Provenance evidence is missing or incomplete.
    ProvenanceUnavailable,

    /// One build produced no output artifacts.
    EmptyArtifactSet,

    /// The two builds contain different artifact identifiers.
    ArtifactSetMismatch {
        identifier: String,
    },

    /// Corresponding artifact fingerprints differ.
    ArtifactMismatch {
        identifier: String,
    },
}

/// Severity of a reproducibility finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ReproducibilityFindingSeverity {
    /// Verification cannot establish the required guarantee.
    Indeterminate,

    /// An explicit contradiction was discovered.
    Rejected,
}

/// One independently reported verification finding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReproducibilityFinding {
    severity: ReproducibilityFindingSeverity,
    kind: ReproducibilityFindingKind,
}

impl ReproducibilityFinding {
    fn new(
        severity: ReproducibilityFindingSeverity,
        kind: ReproducibilityFindingKind,
    ) -> Self {
        Self {
            severity,
            kind,
        }
    }

    /// Returns the finding severity.
    #[must_use]
    pub const fn severity(&self) -> ReproducibilityFindingSeverity {
        self.severity
    }

    /// Returns the finding classification.
    #[must_use]
    pub fn kind(&self) -> &ReproducibilityFindingKind {
        &self.kind
    }
}

// ============================================================================
// VERIFICATION DECISION
// ============================================================================

/// Overall reproducibility verification decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReproducibilityDecision {
    /// All mandatory checks succeeded.
    Verified,

    /// An explicit reproducibility contradiction was discovered.
    Rejected,

    /// Available evidence is insufficient to establish reproducibility.
    Indeterminate,
}

impl ReproducibilityDecision {
    /// Returns whether reproducibility was established.
    #[must_use]
    pub const fn is_verified(self) -> bool {
        matches!(self, Self::Verified)
    }

    /// Returns whether verification explicitly rejected the comparison.
    #[must_use]
    pub const fn is_rejected(self) -> bool {
        matches!(self, Self::Rejected)
    }

    /// Returns whether additional evidence is necessary.
    #[must_use]
    pub const fn is_indeterminate(self) -> bool {
        matches!(self, Self::Indeterminate)
    }
}

// ============================================================================
// VERIFICATION REPORT
// ============================================================================

/// Immutable result of an independent reproducibility comparison.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReproducibilityVerificationReport {
    first_build: BuildInstanceId,

    second_build: BuildInstanceId,

    decision: ReproducibilityDecision,

    findings: Vec<ReproducibilityFinding>,

    artifacts_compared: usize,
}

impl ReproducibilityVerificationReport {
    fn new(
        first_build: BuildInstanceId,
        second_build: BuildInstanceId,
        decision: ReproducibilityDecision,
        findings: Vec<ReproducibilityFinding>,
        artifacts_compared: usize,
    ) -> Self {
        Self {
            first_build,
            second_build,
            decision,
            findings,
            artifacts_compared,
        }
    }

    /// Returns the first build identity.
    #[must_use]
    pub fn first_build(&self) -> &BuildInstanceId {
        &self.first_build
    }

    /// Returns the second build identity.
    #[must_use]
    pub fn second_build(&self) -> &BuildInstanceId {
        &self.second_build
    }

    /// Returns the final decision.
    #[must_use]
    pub const fn decision(&self) -> ReproducibilityDecision {
        self.decision
    }

    /// Returns whether reproducibility was established.
    #[must_use]
    pub const fn is_verified(&self) -> bool {
        self.decision.is_verified()
    }

    /// Returns whether verification rejected the comparison.
    #[must_use]
    pub const fn is_rejected(&self) -> bool {
        self.decision.is_rejected()
    }

    /// Returns whether the result is indeterminate.
    #[must_use]
    pub const fn is_indeterminate(&self) -> bool {
        self.decision.is_indeterminate()
    }

    /// Returns every finding.
    #[must_use]
    pub fn findings(&self) -> &[ReproducibilityFinding] {
        &self.findings
    }

    /// Returns the number of compared artifact outputs.
    #[must_use]
    pub const fn artifacts_compared(&self) -> usize {
        self.artifacts_compared
    }

    /// Returns the number of rejected findings.
    #[must_use]
    pub fn rejection_count(&self) -> usize {
        self.findings
            .iter()
            .filter(|finding| {
                finding.severity
                    == ReproducibilityFindingSeverity::Rejected
            })
            .count()
    }

    /// Returns the number of indeterminate findings.
    #[must_use]
    pub fn indeterminate_count(&self) -> usize {
        self.findings
            .iter()
            .filter(|finding| {
                finding.severity
                    == ReproducibilityFindingSeverity::Indeterminate
            })
            .count()
    }

    /// Returns whether any findings exist.
    #[must_use]
    pub fn has_findings(&self) -> bool {
        !self.findings.is_empty()
    }
}

// ============================================================================
// ERRORS
// ============================================================================

/// Errors raised while constructing verification evidence.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ReproducibilityVerificationError {
    /// An identifier is empty or contains forbidden control characters.
    InvalidIdentifier {
        description: &'static str,
    },

    /// An artifact identifier was supplied more than once.
    DuplicateArtifact {
        identifier: String,
    },
}

impl fmt::Display for ReproducibilityVerificationError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidIdentifier { description } => {
                write!(
                    formatter,
                    "invalid reproducibility verification {description}"
                )
            }

            Self::DuplicateArtifact { identifier } => {
                write!(
                    formatter,
                    "duplicate reproducibility artifact: {identifier}"
                )
            }
        }
    }
}

impl Error for ReproducibilityVerificationError {}

// ============================================================================
// IDENTIFIER VALIDATION
// ============================================================================

fn validate_identifier(
    value: &str,
    description: &'static str,
) -> Result<(), ReproducibilityVerificationError> {
    if value.trim().is_empty()
        || value.chars().any(char::is_control)
    {
        return Err(
            ReproducibilityVerificationError::InvalidIdentifier {
                description,
            },
        );
    }

    Ok(())
}

// ============================================================================
// REPRODUCIBILITY VERIFICATION
// ============================================================================

/// Independently compares two compilation executions.
///
/// # Verification procedure
///
/// 1. Verify independent build identities.
/// 2. Compare complete compilation-input fingerprints.
/// 3. Compare compilation-environment fingerprints.
/// 4. Evaluate determinism evidence.
/// 5. Evaluate artifact validation.
/// 6. Evaluate provenance completeness.
/// 7. Compare artifact inventories.
/// 8. Compare corresponding artifact fingerprints.
/// 9. Produce an explicit final decision.
///
/// # Important
///
/// The input and environment fingerprints must originate from their
/// canonical reproducibility subsystems.
///
/// Artifact fingerprints must originate from the canonical artifact
/// subsystem.
///
/// This function does not independently hash source or artifact payloads.
///
/// # Determinism
///
/// Artifact iteration uses ordered maps.
///
/// Findings are generated in deterministic order.
///
/// # Scalability
///
/// The comparison uses dynamically sized collections.
///
/// No arbitrary artifact-count ceiling is introduced.
///
/// Memory consumption grows with the supplied artifact inventories.
pub fn verify_reproducibility(
    first: &BuildEvidence,
    second: &BuildEvidence,
    policy: &ReproducibilityVerificationPolicy,
) -> ReproducibilityVerificationReport {
    let mut findings = Vec::new();

    let mut artifacts_compared = 0usize;

    // ------------------------------------------------------------------------
    // 1. Independent execution identity
    // ------------------------------------------------------------------------

    if first.build_id == second.build_id {
        findings.push(ReproducibilityFinding::new(
            ReproducibilityFindingSeverity::Rejected,
            ReproducibilityFindingKind::NonIndependentBuilds,
        ));
    }

    // ------------------------------------------------------------------------
    // 2. Compilation input identity
    // ------------------------------------------------------------------------

    if first.input_fingerprint != second.input_fingerprint {
        findings.push(ReproducibilityFinding::new(
            ReproducibilityFindingSeverity::Rejected,
            ReproducibilityFindingKind::InputMismatch,
        ));
    }

    // ------------------------------------------------------------------------
    // 3. Compilation environment identity
    // ------------------------------------------------------------------------

    if first.environment_fingerprint
        != second.environment_fingerprint
    {
        findings.push(ReproducibilityFinding::new(
            ReproducibilityFindingSeverity::Rejected,
            ReproducibilityFindingKind::EnvironmentMismatch,
        ));
    }

    // ------------------------------------------------------------------------
    // 4. Determinism evidence
    // ------------------------------------------------------------------------

    if policy.require_determinism
        && (!first.determinism_satisfied
            || !second.determinism_satisfied)
    {
        findings.push(ReproducibilityFinding::new(
            ReproducibilityFindingSeverity::Indeterminate,
            ReproducibilityFindingKind::DeterminismNotEstablished,
        ));
    }

    // ------------------------------------------------------------------------
    // 5. Artifact validation
    // ------------------------------------------------------------------------

    if policy.require_valid_artifacts {
        let statuses = [
            first.artifact_validation,
            second.artifact_validation,
        ];

        for status in statuses {
            match status {
                ValidationStatus::Valid => {}

                ValidationStatus::Invalid => {
                    findings.push(ReproducibilityFinding::new(
                        ReproducibilityFindingSeverity::Rejected,
                        ReproducibilityFindingKind::ArtifactValidationFailed,
                    ));
                }

                ValidationStatus::Incomplete => {
                    findings.push(ReproducibilityFinding::new(
                        ReproducibilityFindingSeverity::Indeterminate,
                        ReproducibilityFindingKind::ArtifactValidationIncomplete,
                    ));
                }
            }
        }
    }

    // ------------------------------------------------------------------------
    // 6. Provenance evidence
    // ------------------------------------------------------------------------

    if policy.require_provenance
        && (first.provenance.event_count() == 0
            || second.provenance.event_count() == 0)
    {
        findings.push(ReproducibilityFinding::new(
            ReproducibilityFindingSeverity::Indeterminate,
            ReproducibilityFindingKind::ProvenanceUnavailable,
        ));
    }

    // ------------------------------------------------------------------------
    // 7. Artifact inventory
    // ------------------------------------------------------------------------

    if policy.require_nonempty_outputs
        && (first.artifacts.is_empty()
            || second.artifacts.is_empty())
    {
        findings.push(ReproducibilityFinding::new(
            ReproducibilityFindingSeverity::Indeterminate,
            ReproducibilityFindingKind::EmptyArtifactSet,
        ));
    }

    // ------------------------------------------------------------------------
    // 8. Compare artifact identifiers and fingerprints
    // ------------------------------------------------------------------------

    for (identifier, first_fingerprint) in first.artifacts.iter() {
        match second.artifacts.get(identifier) {
            None => {
                findings.push(ReproducibilityFinding::new(
                    ReproducibilityFindingSeverity::Rejected,
                    ReproducibilityFindingKind::ArtifactSetMismatch {
                        identifier: identifier.to_string(),
                    },
                ));
            }

            Some(second_fingerprint) => {
                artifacts_compared += 1;

                if first_fingerprint != &second_fingerprint {
                    findings.push(ReproducibilityFinding::new(
                        ReproducibilityFindingSeverity::Rejected,
                        ReproducibilityFindingKind::ArtifactMismatch {
                            identifier: identifier.to_string(),
                        },
                    ));
                }
            }
        }
    }

    // Detect artifacts that exist only in the second build.

    for identifier in second.artifacts.outputs.keys() {
        if !first.artifacts.outputs.contains_key(identifier) {
            findings.push(ReproducibilityFinding::new(
                ReproducibilityFindingSeverity::Rejected,
                ReproducibilityFindingKind::ArtifactSetMismatch {
                    identifier: identifier.to_string(),
                },
            ));
        }
    }

    // ------------------------------------------------------------------------
    // 9. Determine the final verification decision
    // ------------------------------------------------------------------------

    let decision = if findings.iter().any(|finding| {
        finding.severity == ReproducibilityFindingSeverity::Rejected
    }) {
        ReproducibilityDecision::Rejected
    } else if !findings.is_empty() {
        ReproducibilityDecision::Indeterminate
    } else {
        ReproducibilityDecision::Verified
    };

    ReproducibilityVerificationReport::new(
        first.build_id.clone(),
        second.build_id.clone(),
        decision,
        findings,
        artifacts_compared,
    )
}

// ============================================================================
// CONVENIENCE VERIFICATION
// ============================================================================

/// Performs independent verification using the strict production policy.
///
/// This is the recommended entry point for release qualification.
pub fn verify_reproducibility_strict(
    first: &BuildEvidence,
    second: &BuildEvidence,
) -> ReproducibilityVerificationReport {
    verify_reproducibility(
        first,
        second,
        &ReproducibilityVerificationPolicy::strict(),
    )
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn fingerprint(value: u8) -> Fingerprint {
        Fingerprint::from_bytes([value; 32])
    }

    fn output(
        name: &str,
        value: u8,
    ) -> ArtifactOutput {
        ArtifactOutput::new(
            ArtifactOutputId::new(name).expect("valid identifier"),
            fingerprint(value),
        )
    }

    #[test]
    fn build_identifier_rejects_empty_values() {
        assert!(
            BuildInstanceId::new("").is_err()
        );

        assert!(
            BuildInstanceId::new("   ").is_err()
        );
    }

    #[test]
    fn artifact_identifier_rejects_invalid_values() {
        assert!(
            ArtifactOutputId::new("").is_err()
        );

        assert!(
            ArtifactOutputId::new("artifact\ninvalid").is_err()
        );
    }

    #[test]
    fn artifact_collection_rejects_duplicates() {
        let mut outputs = ArtifactOutputs::new();

        outputs.insert(output("application", 1))
            .expect("first artifact");

        let result = outputs.insert(output("application", 1));

        assert!(result.is_err());

        assert_eq!(outputs.len(), 1);
    }

    #[test]
    fn artifact_collection_uses_deterministic_order() {
        let mut outputs = ArtifactOutputs::new();

        outputs.insert(output("zeta", 1))
            .expect("insert zeta");

        outputs.insert(output("alpha", 2))
            .expect("insert alpha");

        let identifiers: Vec<&str> = outputs
            .iter()
            .map(|(identifier, _)| identifier.as_str())
            .collect();

        assert_eq!(identifiers, ["alpha", "zeta"]);
    }

    #[test]
    fn strict_policy_requires_all_evidence() {
        let policy = ReproducibilityVerificationPolicy::strict();

        assert!(policy.requires_determinism());
        assert!(policy.requires_valid_artifacts());
        assert!(policy.requires_provenance());
        assert!(policy.requires_nonempty_outputs());
    }

    #[test]
    fn decision_classification_is_correct() {
        assert!(
            ReproducibilityDecision::Verified.is_verified()
        );

        assert!(
            ReproducibilityDecision::Rejected.is_rejected()
        );

        assert!(
            ReproducibilityDecision::Indeterminate.is_indeterminate()
        );
    }

    #[test]
    fn finding_severities_are_distinct() {
        let rejected = ReproducibilityFinding::new(
            ReproducibilityFindingSeverity::Rejected,
            ReproducibilityFindingKind::InputMismatch,
        );

        let incomplete = ReproducibilityFinding::new(
            ReproducibilityFindingSeverity::Indeterminate,
            ReproducibilityFindingKind::ProvenanceUnavailable,
        );

        assert_eq!(
            rejected.severity(),
            ReproducibilityFindingSeverity::Rejected
        );

        assert_eq!(
            incomplete.severity(),
            ReproducibilityFindingSeverity::Indeterminate
        );
    }
}
