
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Toolchain — Target Build Matrix
//!
//! Path: src/toolchain/targets/matrix.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Coordinate one logical Zamani build across multiple requested targets.
//!
//! A target matrix preserves:
//!
//! - One logical build identity.
//! - One source identity.
//! - One dependency identity.
//! - Multiple canonical target identities.
//! - Target-specific feasibility assessments.
//! - Target-specific evidence and unresolved conditions.
//! - Deterministic result ordering.
//!
//! # Architectural authorities
//!
//! Target identities:
//!     crate::backend::target::TargetId
//!
//! Target descriptions:
//!     crate::backend::target::TargetDescriptor
//!
//! Capabilities:
//!     crate::backend::capabilities
//!
//! Workload requirements:
//!     crate::backend::requirements
//!
//! Target discovery and feasibility:
//!     crate::compiler::planning
//!
//! Logical build identity:
//!     crate::toolchain::ids::BuildId
//!
//! Structured errors:
//!     crate::toolchain::errors
//!
//! Target profiles:
//!     crate::toolchain::targets::profiles
//!
//! # Ownership
//!
//! This module owns:
//!
//! - TargetMatrix.
//! - TargetMatrixRequest.
//! - TargetMatrixEntry.
//! - TargetMatrixAssessment.
//! - TargetMatrixDisposition.
//! - Assessment provenance references.
//! - Deterministic matrix aggregation.
//! - Matrix completeness inspection.
//! - Matrix identity consistency.
//! - Transactional assessment recording.
//!
//! # Non-ownership
//!
//! This module does not own:
//!
//! - Source-language target declarations.
//! - Semantic analysis.
//! - Target registration.
//! - Target discovery.
//! - Capability definitions.
//! - Requirement definitions.
//! - Capability matching.
//! - Feasibility calculation.
//! - Resource planning.
//! - Physical resource allocation.
//! - Target selection policy.
//! - Backend selection.
//! - Compilation.
//! - Code generation.
//! - Physical placement or scheduling.
//! - Runtime execution.
//! - Deployment.
//!
//! # POCO-REAF
//!
//! A logical build may produce multiple target-specific realizations.
//!
//! Target identities and target counts are open-world.
//!
//! No universal maximum is imposed on:
//!
//! - Targets.
//! - Computing domains.
//! - Processors.
//! - Accelerators.
//! - Quantum resources.
//! - Distributed nodes.
//! - Memory.
//! - Tensor dimensions.
//! - Hardware resources.
//!
//! Actual collection sizes remain subject to available implementation
//! resources and explicitly configured operational budgets.
//!
//! Infinite physical resources and guaranteed execution on every
//! conceivable machine are not promised.
//!
//! # Critical distinction
//!
//! A matrix entry being present does not establish feasibility.
//!
//! A target descriptor does not establish feasibility.
//!
//! A capability declaration does not establish current availability.
//!
//! Only an authoritative assessment supplied by the compiler planning
//! and realization authorities may establish a feasibility disposition.
//!
//! This module preserves that assessment rather than recreating it.
//!
//! # Determinism
//!
//! Entries are ordered by canonical TargetId.
//!
//! Duplicate targets are rejected.
//!
//! Assessment updates do not depend on registration order.
//!
//! The matrix does not use timestamps, randomness or implicit host discovery.
//!
//! # Security
//!
//! Assessment references are provenance identifiers, not cryptographic
//! verification.
//!
//! The owning compiler/provider/security subsystem remains responsible
//! for authenticating and validating assessment evidence.
//!
//! No filesystem, network, process, hardware or runtime operations occur here.

use std::collections::BTreeSet;
use std::fmt;

use crate::backend::target::TargetId;

use crate::toolchain::errors::{
    ToolchainError,
    ToolchainErrorKind,
    ToolchainResult,
};

use crate::toolchain::ids::BuildId;

/// Stable architectural identity of this module.
pub const TARGET_MATRIX_MODULE_NAME: &str =
    "zamani.toolchain.targets.matrix";

/// Public API version of target-matrix coordination.
pub const TARGET_MATRIX_API_VERSION: u16 = 1;

// ============================================================================
// TARGET MATRIX REQUEST
// ============================================================================

/// Immutable logical build request used to construct a target matrix.
///
/// This request represents one logical build, not independent unrelated
/// builds.
///
/// The source and dependency identities are shared across every target
/// entry in the matrix.
///
/// Their actual hashing, canonicalization and provenance are owned by
/// the source and dependency subsystems.
///
/// Target references use the canonical backend TargetId.
///
/// An empty target collection is rejected because an empty collection
/// does not represent an explicitly requested target matrix.
///
/// An unspecified target selection must be resolved by the upstream
/// target-selection authority before matrix construction.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TargetMatrixRequest {
    build_id: BuildId,
    source_identity: String,
    dependency_identity: String,
    targets: Vec<TargetId>,
}

impl TargetMatrixRequest {
    /// Construct a logical multi-target build request.
    ///
    /// Target identities are sorted deterministically.
    ///
    /// Duplicate target identities are rejected.
    ///
    /// No target is inferred or added implicitly.
    pub fn new(
        build_id: BuildId,
        source_identity: impl Into<String>,
        dependency_identity: impl Into<String>,
        targets: Vec<TargetId>,
    ) -> ToolchainResult<Self> {
        let source_identity = source_identity.into();
        let dependency_identity = dependency_identity.into();

        validate_identity(
            "source identity",
            &source_identity,
        )?;

        validate_identity(
            "dependency identity",
            &dependency_identity,
        )?;

        if targets.is_empty() {
            return Err(ToolchainError::invalid_input(
                "target matrix requires at least one explicitly resolved target",
            ));
        }

        let mut targets = targets;

        targets.sort();

        for pair in targets.windows(2) {
            if pair[0] == pair[1] {
                return Err(ToolchainError::invalid_input(
                    format!(
                        "duplicate target `{}` in target matrix request",
                        pair[0]
                    ),
                ));
            }
        }

        Ok(Self {
            build_id,
            source_identity,
            dependency_identity,
            targets,
        })
    }

    /// Return the logical build identity.
    #[must_use]
    pub fn build_id(&self) -> &BuildId {
        &self.build_id
    }

    /// Return the shared source identity.
    #[must_use]
    pub fn source_identity(&self) -> &str {
        &self.source_identity
    }

    /// Return the shared dependency identity.
    #[must_use]
    pub fn dependency_identity(&self) -> &str {
        &self.dependency_identity
    }

    /// Return explicitly requested targets in deterministic order.
    #[must_use]
    pub fn targets(&self) -> &[TargetId] {
        &self.targets
    }

    /// Return the number of requested target realizations.
    #[must_use]
    pub fn target_count(&self) -> usize {
        self.targets.len()
    }
}

// ============================================================================
// ASSESSMENT IDENTIFIERS
// ============================================================================

/// Reference to an authoritative compiler or provider assessment.
///
/// This is an opaque stable reference.
///
/// It does not replace the original assessment report.
///
/// The referenced authority must retain the actual assessment and its
/// evidence so that consumers can inspect and verify the decision.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AssessmentReference(String);

impl AssessmentReference {
    /// Construct a validated assessment reference.
    pub fn new(value: impl Into<String>) -> ToolchainResult<Self> {
        let value = value.into();

        validate_identity(
            "assessment reference",
            &value,
        )?;

        Ok(Self(value))
    }

    /// Return the reference without allocating.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for AssessmentReference {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// Identifies the authority that produced an assessment.
///
/// The provider or compiler planning registry owns the actual authority
/// identity and authentication policy.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AssessmentAuthority(String);

impl AssessmentAuthority {
    /// Construct an assessment-authority reference.
    pub fn new(value: impl Into<String>) -> ToolchainResult<Self> {
        let value = value.into();

        validate_identity(
            "assessment authority",
            &value,
        )?;

        Ok(Self(value))
    }

    /// Return the authority identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

// ============================================================================
// ASSESSMENT EVIDENCE
// ============================================================================

/// Reference to evidence supporting an assessment.
///
/// The actual evidence remains owned by its originating subsystem.
///
/// Evidence references must not contain credentials, private keys,
/// access tokens or other secrets.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AssessmentEvidence {
    reference: String,
    description: Option<String>,
}

impl AssessmentEvidence {
    /// Construct an evidence reference.
    pub fn new(
        reference: impl Into<String>,
        description: Option<String>,
    ) -> ToolchainResult<Self> {
        let reference = reference.into();

        validate_identity(
            "assessment evidence reference",
            &reference,
        )?;

        if description
            .as_deref()
            .is_some_and(|value| value.trim().is_empty())
        {
            return Err(ToolchainError::invalid_input(
                "assessment evidence description must not be empty when supplied",
            ));
        }

        Ok(Self {
            reference,
            description,
        })
    }

    /// Return the evidence reference.
    #[must_use]
    pub fn reference(&self) -> &str {
        &self.reference
    }

    /// Return the optional evidence description.
    #[must_use]
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }
}

// ============================================================================
// ASSESSMENT REASONS
// ============================================================================

/// An explicit explanation attached to an assessment.
///
/// Reasons are explanatory data, not an alternative feasibility solver.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct AssessmentReason {
    code: String,
    message: String,
}

impl AssessmentReason {
    /// Construct an assessment explanation.
    pub fn new(
        code: impl Into<String>,
        message: impl Into<String>,
    ) -> ToolchainResult<Self> {
        let code = code.into();
        let message = message.into();

        validate_identity(
            "assessment reason code",
            &code,
        )?;

        if message.trim().is_empty() {
            return Err(ToolchainError::invalid_input(
                "assessment reason message must not be empty",
            ));
        }

        Ok(Self {
            code,
            message,
        })
    }

    /// Return the machine-readable reason code.
    #[must_use]
    pub fn code(&self) -> &str {
        &self.code
    }

    /// Return the human-readable explanation.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

// ============================================================================
// TARGET MATRIX DISPOSITION
// ============================================================================

/// Evidence-based disposition of one requested target realization.
///
/// `Pending` means no authoritative assessment has been recorded.
///
/// `Feasible` means the authoritative assessment established feasibility
/// for the assessment's stated scope and evidence snapshot.
///
/// `Infeasible` means the authoritative assessment established that the
/// requested realization cannot satisfy its mandatory conditions.
///
/// `Unresolved` means the available evidence cannot establish feasibility
/// or infeasibility.
///
/// These states must never be conflated.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum TargetMatrixDisposition {
    /// No authoritative assessment has been recorded.
    Pending,

    /// Feasibility has been established by the assessment authority.
    Feasible,

    /// Mandatory conditions have been established as unsatisfied.
    Infeasible,

    /// Available evidence is insufficient for a conclusive decision.
    Unresolved,
}

impl TargetMatrixDisposition {
    /// Return whether an authoritative assessment has been recorded.
    #[must_use]
    pub const fn is_assessed(self) -> bool {
        !matches!(self, Self::Pending)
    }

    /// Return whether feasibility has been established.
    #[must_use]
    pub const fn is_feasible(self) -> bool {
        matches!(self, Self::Feasible)
    }

    /// Return whether the result remains inconclusive.
    #[must_use]
    pub const fn is_unresolved(self) -> bool {
        matches!(
            self,
            Self::Pending | Self::Unresolved
        )
    }

    /// Return a stable textual representation.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Feasible => "feasible",
            Self::Infeasible => "infeasible",
            Self::Unresolved => "unresolved",
        }
    }
}

impl fmt::Display for TargetMatrixDisposition {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// AUTHORITATIVE TARGET ASSESSMENT
// ============================================================================

/// Completed assessment supplied by an authoritative downstream subsystem.
///
/// This type is an integration envelope.
///
/// It does not calculate feasibility.
///
/// The caller must obtain the disposition, authority, reference and evidence
/// from the actual compiler planning or realization assessment.
///
/// A feasible assessment requires at least one evidence reference.
///
/// An infeasible or unresolved assessment requires at least one explanation.
///
/// A pending assessment cannot be submitted as a completed assessment.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TargetMatrixAssessment {
    target: TargetId,
    disposition: TargetMatrixDisposition,
    authority: AssessmentAuthority,
    assessment_reference: AssessmentReference,
    evidence: Vec<AssessmentEvidence>,
    reasons: Vec<AssessmentReason>,
}

impl TargetMatrixAssessment {
    /// Construct a completed assessment.
    ///
    /// Validation is performed before the assessment can be recorded.
    pub fn new(
        target: TargetId,
        disposition: TargetMatrixDisposition,
        authority: AssessmentAuthority,
        assessment_reference: AssessmentReference,
        evidence: Vec<AssessmentEvidence>,
        reasons: Vec<AssessmentReason>,
    ) -> ToolchainResult<Self> {
        if disposition == TargetMatrixDisposition::Pending {
            return Err(ToolchainError::invalid_input(
                "pending is not a completed target assessment",
            ));
        }

        match disposition {
            TargetMatrixDisposition::Feasible => {
                if evidence.is_empty() {
                    return Err(ToolchainError::validation_failed(
                        "feasible assessment requires supporting evidence references",
                    ));
                }
            }

            TargetMatrixDisposition::Infeasible
            | TargetMatrixDisposition::Unresolved => {
                if reasons.is_empty() {
                    return Err(ToolchainError::validation_failed(
                        "infeasible and unresolved assessments require explanatory reasons",
                    ));
                }
            }

            TargetMatrixDisposition::Pending => {
                return Err(ToolchainError::invalid_input(
                    "pending is not a completed target assessment",
                ));
            }
        }

        Ok(Self {
            target,
            disposition,
            authority,
            assessment_reference,
            evidence,
            reasons,
        })
    }

    /// Return the assessed target.
    #[must_use]
    pub fn target(&self) -> &TargetId {
        &self.target
    }

    /// Return the authoritative disposition.
    #[must_use]
    pub const fn disposition(&self) -> TargetMatrixDisposition {
        self.disposition
    }

    /// Return the assessment authority.
    #[must_use]
    pub fn authority(&self) -> &AssessmentAuthority {
        &self.authority
    }

    /// Return the authoritative assessment reference.
    #[must_use]
    pub fn assessment_reference(&self) -> &AssessmentReference {
        &self.assessment_reference
    }

    /// Return the supporting evidence references.
    #[must_use]
    pub fn evidence(&self) -> &[AssessmentEvidence] {
        &self.evidence
    }

    /// Return the assessment explanations.
    #[must_use]
    pub fn reasons(&self) -> &[AssessmentReason] {
        &self.reasons
    }
}

// ============================================================================
// MATRIX ENTRY
// ============================================================================

/// One target-specific realization entry.
///
/// The entry retains its canonical target identity and the authoritative
/// assessment, when one has been recorded.
///
/// A feasible entry is not a compiled artifact.
///
/// Compilation and artifact verification remain separate build stages.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TargetMatrixEntry {
    target: TargetId,
    assessment: Option<TargetMatrixAssessment>,
}

impl TargetMatrixEntry {
    fn pending(target: TargetId) -> Self {
        Self {
            target,
            assessment: None,
        }
    }

    /// Return the canonical target identity.
    #[must_use]
    pub fn target(&self) -> &TargetId {
        &self.target
    }

    /// Return the current disposition.
    #[must_use]
    pub fn disposition(&self) -> TargetMatrixDisposition {
        self.assessment
            .as_ref()
            .map(TargetMatrixAssessment::disposition)
            .unwrap_or(TargetMatrixDisposition::Pending)
    }

    /// Return the authoritative assessment, if available.
    #[must_use]
    pub fn assessment(&self) -> Option<&TargetMatrixAssessment> {
        self.assessment.as_ref()
    }

    /// Return whether this target has been assessed.
    #[must_use]
    pub fn is_assessed(&self) -> bool {
        self.assessment.is_some()
    }
}

// ============================================================================
// MATRIX AGGREGATE STATE
// ============================================================================

/// Aggregate state of the complete requested target matrix.
///
/// A partially assessed matrix is never reported as complete.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum TargetMatrixStatus {
    /// No requested target has been assessed.
    Pending,

    /// Some targets are assessed and others remain pending.
    InProgress,

    /// Every requested target is assessed and feasible.
    AllFeasible,

    /// Every requested target is assessed, but at least one is infeasible.
    CompletedWithInfeasibleTargets,

    /// Every requested target is assessed, but at least one is unresolved.
    CompletedWithUnresolvedTargets,

    /// Every requested target is assessed, with a mixture of infeasible
    /// and unresolved outcomes.
    CompletedWithUnresolvedAndInfeasibleTargets,
}

impl TargetMatrixStatus {
    /// Whether every requested target has an assessment.
    #[must_use]
    pub const fn is_complete(self) -> bool {
        matches!(
            self,
            Self::AllFeasible
                | Self::CompletedWithInfeasibleTargets
                | Self::CompletedWithUnresolvedTargets
                | Self::CompletedWithUnresolvedAndInfeasibleTargets
        )
    }

    /// Whether every requested target has been established as feasible.
    #[must_use]
    pub const fn is_fully_feasible(self) -> bool {
        matches!(self, Self::AllFeasible)
    }
}

// ============================================================================
// TARGET MATRIX
// ============================================================================

/// Deterministic collection of target-specific build realizations.
///
/// All entries belong to the same logical build and share the same source
/// and dependency identities.
///
/// The matrix records target feasibility assessments. It does not execute
/// the requested builds.
///
/// Matrix entries are sorted by canonical TargetId.
///
/// The requested target set is immutable after construction. A different
/// target set requires a different matrix request and a separately
/// identified build operation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TargetMatrix {
    build_id: BuildId,
    source_identity: String,
    dependency_identity: String,
    entries: Vec<TargetMatrixEntry>,
}

impl TargetMatrix {
    /// Construct a matrix from a validated logical build request.
    ///
    /// Every requested target starts in the Pending state.
    pub fn new(request: TargetMatrixRequest) -> ToolchainResult<Self> {
        let mut entries = Vec::new();

        entries
            .try_reserve(request.targets.len())
            .map_err(|error| {
                ToolchainError::resource_exhausted(
                    format!(
                        "unable to reserve target matrix entries: {error}"
                    ),
                )
            })?;

        for target in request.targets {
            entries.push(TargetMatrixEntry::pending(target));
        }

        Ok(Self {
            build_id: request.build_id,
            source_identity: request.source_identity,
            dependency_identity: request.dependency_identity,
            entries,
        })
    }

    /// Return the logical build identity.
    #[must_use]
    pub fn build_id(&self) -> &BuildId {
        &self.build_id
    }

    /// Return the shared source identity.
    #[must_use]
    pub fn source_identity(&self) -> &str {
        &self.source_identity
    }

    /// Return the shared dependency identity.
    #[must_use]
    pub fn dependency_identity(&self) -> &str {
        &self.dependency_identity
    }

    /// Return all entries in deterministic target-identity order.
    #[must_use]
    pub fn entries(&self) -> &[TargetMatrixEntry] {
        &self.entries
    }

    /// Return the number of requested targets.
    #[must_use]
    pub fn target_count(&self) -> usize {
        self.entries.len()
    }

    /// Return the number of targets with completed assessments.
    #[must_use]
    pub fn assessed_count(&self) -> usize {
        self.entries
            .iter()
            .filter(|entry| entry.is_assessed())
            .count()
    }

    /// Return the number of feasible targets.
    #[must_use]
    pub fn feasible_count(&self) -> usize {
        self.count_disposition(TargetMatrixDisposition::Feasible)
    }

    /// Return the number of infeasible targets.
    #[must_use]
    pub fn infeasible_count(&self) -> usize {
        self.count_disposition(TargetMatrixDisposition::Infeasible)
    }

    /// Return the number of unresolved targets.
    #[must_use]
    pub fn unresolved_count(&self) -> usize {
        self.count_disposition(TargetMatrixDisposition::Unresolved)
    }

    /// Return the number of pending targets.
    #[must_use]
    pub fn pending_count(&self) -> usize {
        self.count_disposition(TargetMatrixDisposition::Pending)
    }

    /// Return the aggregate matrix status.
    #[must_use]
    pub fn status(&self) -> TargetMatrixStatus {
        let pending = self.pending_count();

        if self.assessed_count() == 0 {
            return TargetMatrixStatus::Pending;
        }

        if pending > 0 {
            return TargetMatrixStatus::InProgress;
        }

        let infeasible = self.infeasible_count() > 0;
        let unresolved = self.unresolved_count() > 0;

        match (infeasible, unresolved) {
            (false, false) => TargetMatrixStatus::AllFeasible,

            (true, false) => {
                TargetMatrixStatus::CompletedWithInfeasibleTargets
            }

            (false, true) => {
                TargetMatrixStatus::CompletedWithUnresolvedTargets
            }

            (true, true) => {
                TargetMatrixStatus::
                    CompletedWithUnresolvedAndInfeasibleTargets
            }
        }
    }

    /// Look up a target entry by canonical identity.
    ///
    /// Entries are sorted, so lookup uses binary search.
    #[must_use]
    pub fn get(
        &self,
        target: &TargetId,
    ) -> Option<&TargetMatrixEntry> {
        self.entries
            .binary_search_by(|entry| entry.target.cmp(target))
            .ok()
            .map(|index| &self.entries[index])
    }

    /// Record an authoritative assessment for a requested target.
    ///
    /// The operation is transactional:
    ///
    /// - The target must belong to this matrix.
    /// - The target must not already have an assessment.
    /// - The assessment must be complete and valid.
    /// - The existing matrix remains unchanged on failure.
    ///
    /// This method does not validate the truth of external evidence.
    /// The assessment authority is responsible for that validation.
    pub fn record_assessment(
        &mut self,
        assessment: TargetMatrixAssessment,
    ) -> ToolchainResult<()> {
        let index = self
            .entries
            .binary_search_by(|entry| {
                entry.target.cmp(&assessment.target)
            })
            .map_err(|_| {
                ToolchainError::validation_failed(
                    format!(
                        "assessment target `{}` was not requested by this matrix",
                        assessment.target
                    ),
                )
            })?;

        let entry = self.entries.get_mut(index).ok_or_else(|| {
            ToolchainError::internal(
                "target matrix index became invalid during assessment recording",
            )
        })?;

        if entry.assessment.is_some() {
            return Err(ToolchainError::new(
                ToolchainErrorKind::AlreadyExists,
                format!(
                    "target `{}` already has an assessment",
                    entry.target
                ),
            ));
        }

        entry.assessment = Some(assessment);

        Ok(())
    }

    /// Return whether all requested targets have assessments.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.pending_count() == 0
    }

    /// Return whether every requested target is feasible.
    ///
    /// An empty matrix cannot be considered feasible.
    #[must_use]
    pub fn is_fully_feasible(&self) -> bool {
        !self.entries.is_empty()
            && self.feasible_count() == self.entries.len()
    }

    /// Return whether any requested target remains unresolved or pending.
    #[must_use]
    pub fn has_unresolved_targets(&self) -> bool {
        self.unresolved_count() > 0 || self.pending_count() > 0
    }

    /// Validate matrix structural invariants.
    ///
    /// This does not recalculate feasibility or verify external evidence.
    pub fn validate(&self) -> ToolchainResult<()> {
        validate_identity(
            "source identity",
            &self.source_identity,
        )?;

        validate_identity(
            "dependency identity",
            &self.dependency_identity,
        )?;

        if self.entries.is_empty() {
            return Err(ToolchainError::validation_failed(
                "target matrix contains no requested targets",
            ));
        }

        for pair in self.entries.windows(2) {
            if pair[0].target >= pair[1].target {
                return Err(ToolchainError::internal(
                    "target matrix entries are not strictly ordered or contain duplicates",
                ));
            }
        }

        for entry in &self.entries {
            if let Some(assessment) = &entry.assessment {
                if assessment.target != entry.target {
                    return Err(ToolchainError::internal(
                        "assessment target identity does not match its matrix entry",
                    ));
                }

                if assessment.disposition
                    == TargetMatrixDisposition::Pending
                {
                    return Err(ToolchainError::internal(
                        "completed assessment contains pending disposition",
                    ));
                }

                match assessment.disposition {
                    TargetMatrixDisposition::Feasible
                        if assessment.evidence.is_empty() =>
                    {
                        return Err(ToolchainError::validation_failed(
                            "feasible assessment has no evidence references",
                        ));
                    }

                    TargetMatrixDisposition::Infeasible
                    | TargetMatrixDisposition::Unresolved
                        if assessment.reasons.is_empty() =>
                    {
                        return Err(ToolchainError::validation_failed(
                            "infeasible or unresolved assessment has no reasons",
                        ));
                    }

                    _ => {}
                }
            }
        }

        Ok(())
    }

    fn count_disposition(
        &self,
        disposition: TargetMatrixDisposition,
    ) -> usize {
        self.entries
            .iter()
            .filter(|entry| entry.disposition() == disposition)
            .count()
    }
}

// ============================================================================
// IDENTITY VALIDATION
// ============================================================================

fn validate_identity(
    name: &str,
    value: &str,
) -> ToolchainResult<()> {
    if value.trim().is_empty() {
        return Err(ToolchainError::invalid_input(
            format!("{name} must not be empty"),
        ));
    }

    if value.trim() != value {
        return Err(ToolchainError::invalid_input(
            format!(
                "{name} must not contain leading or trailing whitespace"
            ),
        ));
    }

    if value.chars().any(char::is_control) {
        return Err(ToolchainError::invalid_input(
            format!("{name} must not contain control characters"),
        ));
    }

    Ok(())
}

// ============================================================================
// MODULE TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn target(name: &str) -> TargetId {
        TargetId::new(name.to_owned())
            .expect("valid target identity")
    }

    fn authority() -> AssessmentAuthority {
        AssessmentAuthority::new("compiler.planning")
            .expect("valid assessment authority")
    }

    fn reference(name: &str) -> AssessmentReference {
        AssessmentReference::new(name)
            .expect("valid assessment reference")
    }

    fn evidence(name: &str) -> AssessmentEvidence {
        AssessmentEvidence::new(
            name,
            Some("verified planning evidence".to_owned()),
        )
        .expect("valid evidence")
    }

    fn reason() -> AssessmentReason {
        AssessmentReason::new(
            "capability.unavailable",
            "A mandatory capability could not be established",
        )
        .expect("valid reason")
    }

    #[test]
    fn module_identity_is_stable() {
        assert_eq!(
            TARGET_MATRIX_MODULE_NAME,
            "zamani.toolchain.targets.matrix"
        );

        assert!(TARGET_MATRIX_API_VERSION > 0);
    }

    #[test]
    fn assessment_reference_rejects_empty_values() {
        assert!(AssessmentReference::new("").is_err());
        assert!(AssessmentAuthority::new(" ").is_err());
    }

    #[test]
    fn evidence_rejects_empty_references() {
        assert!(
            AssessmentEvidence::new(" ", None).is_err()
        );

        assert!(
            AssessmentEvidence::new(
                "evidence.1",
                Some(String::new())
            )
            .is_err()
        );
    }

    #[test]
    fn assessment_rejects_pending_disposition() {
        let result = TargetMatrixAssessment::new(
            target("target.alpha"),
            TargetMatrixDisposition::Pending,
            authority(),
            reference("assessment.1"),
            Vec::new(),
            Vec::new(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn feasible_assessment_requires_evidence() {
        let result = TargetMatrixAssessment::new(
            target("target.alpha"),
            TargetMatrixDisposition::Feasible,
            authority(),
            reference("assessment.1"),
            Vec::new(),
            Vec::new(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn unresolved_assessment_requires_explanation() {
        let result = TargetMatrixAssessment::new(
            target("target.alpha"),
            TargetMatrixDisposition::Unresolved,
            authority(),
            reference("assessment.1"),
            Vec::new(),
            Vec::new(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn infeasible_assessment_requires_explanation() {
        let result = TargetMatrixAssessment::new(
            target("target.alpha"),
            TargetMatrixDisposition::Infeasible,
            authority(),
            reference("assessment.1"),
            Vec::new(),
            Vec::new(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn assessment_preserves_authoritative_evidence() {
        let assessment = TargetMatrixAssessment::new(
            target("target.alpha"),
            TargetMatrixDisposition::Feasible,
            authority(),
            reference("assessment.1"),
            vec![evidence("evidence.1")],
            Vec::new(),
        )
        .expect("valid assessment");

        assert_eq!(
            assessment.disposition(),
            TargetMatrixDisposition::Feasible
        );

        assert_eq!(assessment.evidence().len(), 1);

        assert_eq!(
            assessment.authority().as_str(),
            "compiler.planning"
        );
    }

    #[test]
    fn disposition_states_are_distinct() {
        assert!(!TargetMatrixDisposition::Pending.is_assessed());

        assert!(
            TargetMatrixDisposition::Feasible.is_feasible()
        );

        assert!(
            TargetMatrixDisposition::Unresolved.is_unresolved()
        );

        assert!(
            TargetMatrixDisposition::Pending.is_unresolved()
        );

        assert!(
            !TargetMatrixDisposition::Infeasible.is_feasible()
        );
    }

    #[test]
    fn assessment_reason_preserves_code_and_message() {
        let reason = reason();

        assert_eq!(
            reason.code(),
            "capability.unavailable"
        );

        assert!(
            reason.message().contains("mandatory capability")
        );
    }

    #[test]
    fn aggregate_status_types_are_distinct() {
        assert!(
            !TargetMatrixStatus::Pending.is_complete()
        );

        assert!(
            !TargetMatrixStatus::InProgress.is_complete()
        );

        assert!(
            TargetMatrixStatus::AllFeasible.is_fully_feasible()
        );

        assert!(
            !TargetMatrixStatus::
                CompletedWithInfeasibleTargets
                .is_fully_feasible()
        );
    }

    #[test]
    fn identity_validation_rejects_control_characters() {
        assert!(
            validate_identity(
                "source identity",
                "source\nidentity"
            )
            .is_err()
        );

        assert!(
            validate_identity(
                "source identity",
                "source.identity"
            )
            .is_ok()
        );
    }

    #[test]
    fn assessment_reference_is_orderable() {
        let first = reference("assessment.alpha");
        let second = reference("assessment.beta");

        assert!(first < second);
    }

    #[test]
    fn target_identity_is_preserved() {
        let target = target("quantum.qasm");

        assert_eq!(target.as_str(), "quantum.qasm");
    }

    #[test]
    fn reason_collections_are_deterministic() {
        let first = reason();

        let second = AssessmentReason::new(
            "resource.unavailable",
            "Required resources are not established",
        )
        .expect("valid reason");

        let mut reasons = BTreeSet::new();

        reasons.insert(first);
        reasons.insert(second);

        assert_eq!(reasons.len(), 2);
    }
}
