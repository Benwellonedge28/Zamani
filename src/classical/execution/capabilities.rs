#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Classical Execution — Capability Assessment
//!
//! Path: `src/classical/execution/capabilities.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Provide the classical execution boundary for assessing canonical
//! semantic capability requirements against externally supplied,
//! scope-specific capability evidence.
//!
//! This module adapts existing frontend capability infrastructure. It does
//! not define a second capability identity, requirement, evidence, or
//! compatibility model.
//!
//! ## Authority
//!
//! - `grammar/Zamani.g4`: root grammar composition.
//! - `grammar/antlr/ZamaniLexer.g4`: canonical lexer boundary.
//! - `grammar/antlr/ZamaniParser.g4`: canonical parser composition.
//! - `grammar/core/capabilities.g4`: capability identity syntax.
//! - `grammar/resources/capabilities.g4`: resource-scoped capability intent.
//! - `grammar/specification/semantic-model.md`: semantic authority.
//! - `grammar/specification/scalability-model.md`: scalability contract.
//! - `grammar/spec/resources.md`: resource and capability intent.
//! - `src/frontend/semantic/capabilities/model.rs`: canonical requirements.
//! - `src/frontend/semantic/capabilities/requirements.rs`: source provenance.
//! - `src/frontend/semantic/capabilities/evidence.rs`: scoped evidence.
//! - `src/frontend/semantic/capabilities/compatibility.rs`:
//!   canonical compatibility evaluation.
//! - `src/classical/execution/requirements.rs`: classical requirement
//!   aggregation.
//! - `src/classical/execution/plan.rs`: target-independent execution plans.
//!
//! The grammar defines syntax; the specifications define meaning; the
//! frontend semantic subsystem owns canonical capability values and
//! compatibility evaluation.
//!
//! ## Owns
//!
//! - The execution-facing wrapper around a capability evidence scope.
//! - A classical execution assessment of canonical capability requirements.
//! - A stable, aggregate assessment status.
//! - Structured errors for assessment preparation.
//!
//! ## Does not own
//!
//! - Capability identities, versions, or requirement properties.
//! - Source parsing, AST construction, name resolution, or type checking.
//! - Hardware discovery, provider APIs, or evidence authentication.
//! - Physical resource capacity or resource allocation.
//! - Target selection, placement, partitioning, routing, or scheduling.
//! - Classical IR, ZUIR, quantum IR, or runtime execution.
//! - Fallback policy or permission to change program semantics.
//!
//! ## Integration contract
//!
//! The frontend supplies a `CapabilityRequirements` collection whose
//! records retain canonical AST provenance. A target adapter supplies an
//! `EvidenceSubject` and a `CapabilityEvidenceSet` describing claims for
//! that scope.
//!
//! This module delegates evaluation to `evaluate_compatibility`. It never
//! treats a missing claim as proof of support, never combines evidence
//! from different scopes, and never authenticates evidence.
//!
//! Assessment status is an aggregate summary:
//!
//! - `NoRequirements`: the supplied requirement collection is empty.
//! - `Satisfied`: every requirement has evidence establishing satisfaction.
//! - `Incompatible`: evidence establishes at least one incompatibility.
//! - `Unresolved`: requirements remain without sufficient evidence.
//! - `ConflictingEvidence`: evidence contains contradictory claims.
//!
//! Conflicting evidence takes precedence over other aggregate outcomes,
//! followed by known incompatibility, unresolved outcomes, and satisfaction.
//! This ordering ensures that uncertainty and contradictions are not
//! reported as success.
//!
//! An empty requirement collection is deliberately not reported as
//! `Satisfied`: absence of requirements is not proof of target support.
//!
//! ## Scalability and POCO-REAF
//!
//! Capability identities are open-world and remain owned by the semantic
//! model. This module imposes no universal ceiling on capability counts,
//! domains, devices, workers, or machine sizes.
//!
//! Evidence collections and assessments are limited by actual memory,
//! processing time, representable values, and any explicit external
//! operational budgets. Literal infinite capacity is not promised.
//!
//! Target limitations must be represented as target evidence or explicit
//! downstream feasibility results, not as universal language restrictions.
//!
//! ## Determinism
//!
//! Requirement ordering and per-requirement compatibility results follow
//! the canonical frontend subsystem's ordering contract. Aggregate status
//! does not depend on evidence insertion order.
//!
//! ## Safety
//!
//! This module uses safe Rust only and performs no I/O, hardware discovery,
//! global mutation, allocation, or program execution.
//!
//! ## Completion status
//!
//! Proposed implementation. Repository compilation and integration tests
//! must pass before this file can be marked verified.

use core::fmt;
use std::error::Error;

use crate::frontend::semantic::capabilities::{
    evaluate_compatibility,
    CapabilityCompatibilityError,
    CapabilityCompatibilityReport,
    CapabilityCompatibilityStatus,
    CapabilityEvidenceSet,
    CapabilityRequirements,
    EvidenceSubject,
};

/// Result type for classical execution capability assessments.
pub type ExecutionCapabilityResult<T> =
    Result<T, ExecutionCapabilityError>;

/// Aggregate outcome of assessing capability requirements.
///
/// This describes the evidence available for the selected scope. It does
/// not establish that evidence is authentic, that resources are sufficient,
/// or that a program can be executed successfully.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum ExecutionCapabilityStatus {
    /// There were no capability requirements to assess.
    ///
    /// This is not equivalent to proof that a target is suitable.
    NoRequirements,

    /// Evidence establishes that every supplied requirement is satisfied.
    Satisfied,

    /// Evidence establishes that one or more requirements are incompatible.
    Incompatible,

    /// At least one requirement lacks sufficient evidence for a conclusion.
    Unresolved,

    /// Contradictory capability evidence was encountered.
    ///
    /// The contradiction remains visible even if other requirements are
    /// satisfied or known to be incompatible.
    ConflictingEvidence,
}

impl ExecutionCapabilityStatus {
    /// Return whether the assessment establishes that all requirements
    /// are satisfied.
    ///
    /// An empty requirement collection is not considered satisfied.
    #[must_use]
    pub const fn is_satisfied(self) -> bool {
        matches!(self, Self::Satisfied)
    }

    /// Return whether the assessment found a known incompatibility.
    #[must_use]
    pub const fn has_incompatibility(self) -> bool {
        matches!(self, Self::Incompatible)
    }

    /// Return whether more evidence or conflict resolution is needed.
    #[must_use]
    pub const fn is_unresolved(self) -> bool {
        matches!(
            self,
            Self::Unresolved | Self::ConflictingEvidence
        )
    }
}

impl fmt::Display for ExecutionCapabilityStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::NoRequirements => "no capability requirements",
            Self::Satisfied => "all capability requirements satisfied",
            Self::Incompatible => "capability requirements incompatible",
            Self::Unresolved => "capability assessment unresolved",
            Self::ConflictingEvidence => "conflicting capability evidence",
        })
    }
}

/// Structured failure while preparing a capability assessment.
///
/// These errors indicate that compatibility evaluation could not be
/// completed successfully. They must not be confused with an assessment
/// that completed and found a target incompatibility.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ExecutionCapabilityError {
    /// The canonical frontend compatibility evaluator rejected its input.
    Compatibility(CapabilityCompatibilityError),
}

impl fmt::Display for ExecutionCapabilityError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Compatibility(error) => {
                write!(formatter, "capability compatibility evaluation failed: {error}")
            }
        }
    }
}

impl Error for ExecutionCapabilityError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Compatibility(error) => Some(error),
        }
    }
}

impl From<CapabilityCompatibilityError> for ExecutionCapabilityError {
    fn from(error: CapabilityCompatibilityError) -> Self {
        Self::Compatibility(error)
    }
}

/// Capability evidence supplied for one explicitly identified scope.
///
/// The subject may identify a target profile, execution environment, or
/// another scope understood by the upstream capability subsystem. Its
/// interpretation belongs to the component that creates the evidence.
///
/// This type stores evidence supplied by a caller; it does not discover
/// hardware, verify evidence authenticity, or reserve resources.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExecutionCapabilities {
    subject: EvidenceSubject,
    evidence: CapabilityEvidenceSet,
}

impl ExecutionCapabilities {
    /// Create an execution capability context for one evidence scope.
    ///
    /// Evidence is validated by the canonical compatibility evaluator
    /// during assessment. This constructor does not imply that the
    /// evidence is trusted or that the scope is executable.
    #[must_use]
    pub const fn new(
        subject: EvidenceSubject,
        evidence: CapabilityEvidenceSet,
    ) -> Self {
        Self { subject, evidence }
    }

    /// Return the scope against which requirements will be assessed.
    #[must_use]
    pub const fn subject(&self) -> &EvidenceSubject {
        &self.subject
    }

    /// Return the supplied canonical evidence set.
    #[must_use]
    pub const fn evidence(&self) -> &CapabilityEvidenceSet {
        &self.evidence
    }

    /// Assess canonical semantic requirements against this scope.
    ///
    /// Evaluation is delegated to the frontend compatibility subsystem.
    /// Requirements retain their canonical identities and provenance;
    /// this method neither rewrites nor deduplicates them.
    ///
    /// # Errors
    ///
    /// Returns an error if the canonical compatibility evaluator rejects
    /// the evidence or cannot prepare the report.
    pub fn assess(
        &self,
        requirements: &CapabilityRequirements,
    ) -> ExecutionCapabilityResult<ExecutionCapabilityAssessment> {
        let report = evaluate_compatibility(
            requirements,
            &self.subject,
            &self.evidence,
        )?;

        Ok(ExecutionCapabilityAssessment { report })
    }
}

/// Compatibility report with a classical execution-facing summary.
///
/// The canonical frontend report remains the authoritative per-requirement
/// result, including source provenance. This wrapper adds no second
/// representation of capability identities or evidence.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExecutionCapabilityAssessment {
    report: CapabilityCompatibilityReport,
}

impl ExecutionCapabilityAssessment {
    /// Return the canonical per-requirement compatibility report.
    #[must_use]
    pub const fn report(&self) -> &CapabilityCompatibilityReport {
        &self.report
    }

    /// Return the number of capability requirements assessed.
    #[must_use]
    pub fn requirement_count(&self) -> usize {
        self.report.len()
    }

    /// Return whether the report contains no requirements.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.report.is_empty()
    }

    /// Return the aggregate execution-facing status.
    ///
    /// Contradictory evidence takes precedence over known incompatibility,
    /// which takes precedence over unresolved outcomes. Only a non-empty
    /// report whose individual requirements are all satisfied returns
    /// `Satisfied`.
    #[must_use]
    pub fn status(&self) -> ExecutionCapabilityStatus {
        aggregate_status(self.report.iter().map(|record| record.status()))
    }

    /// Return whether all assessed requirements are established as satisfied.
    ///
    /// Empty reports return `false`.
    #[must_use]
    pub fn all_requirements_satisfied(&self) -> bool {
        self.status().is_satisfied()
    }

    /// Return whether at least one requirement is known to be incompatible.
    #[must_use]
    pub fn has_incompatibilities(&self) -> bool {
        self.report.iter().any(|record| {
            record.status() == CapabilityCompatibilityStatus::Incompatible
        })
    }

    /// Return whether any requirement remains unresolved or has conflicting
    /// evidence.
    #[must_use]
    pub fn has_unresolved_requirements(&self) -> bool {
        self.report.iter().any(|record| {
            matches!(
                record.status(),
                CapabilityCompatibilityStatus::NoEvidence
                    | CapabilityCompatibilityStatus::Indeterminate
                    | CapabilityCompatibilityStatus::ConflictingEvidence
            )
        })
    }
}

/// Combine per-requirement outcomes without hiding conflicts or uncertainty.
///
/// The function is independent of the order in which outcomes are supplied.
fn aggregate_status(
    statuses: impl IntoIterator<Item = CapabilityCompatibilityStatus>,
) -> ExecutionCapabilityStatus {
    let mut saw_any = false;
    let mut saw_incompatible = false;
    let mut saw_unresolved = false;
    let mut saw_conflict = false;

    for status in statuses {
        saw_any = true;

        match status {
            CapabilityCompatibilityStatus::Satisfied => {}
            CapabilityCompatibilityStatus::Incompatible => {
                saw_incompatible = true;
            }
            CapabilityCompatibilityStatus::NoEvidence
            | CapabilityCompatibilityStatus::Indeterminate => {
                saw_unresolved = true;
            }
            CapabilityCompatibilityStatus::ConflictingEvidence => {
                saw_conflict = true;
            }
        }
    }

    if !saw_any {
        ExecutionCapabilityStatus::NoRequirements
    } else if saw_conflict {
        ExecutionCapabilityStatus::ConflictingEvidence
    } else if saw_incompatible {
        ExecutionCapabilityStatus::Incompatible
    } else if saw_unresolved {
        ExecutionCapabilityStatus::Unresolved
    } else {
        ExecutionCapabilityStatus::Satisfied
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn aggregate(
        statuses: &[CapabilityCompatibilityStatus],
    ) -> ExecutionCapabilityStatus {
        aggregate_status(statuses.iter().copied())
    }

    #[test]
    fn empty_requirements_are_not_reported_as_satisfied() {
        assert_eq!(
            aggregate(&[]),
            ExecutionCapabilityStatus::NoRequirements
        );
        assert!(!ExecutionCapabilityStatus::NoRequirements.is_satisfied());
    }

    #[test]
    fn all_satisfied_requirements_produce_satisfied_status() {
        assert_eq!(
            aggregate(&[
                CapabilityCompatibilityStatus::Satisfied,
                CapabilityCompatibilityStatus::Satisfied,
            ]),
            ExecutionCapabilityStatus::Satisfied
        );
    }

    #[test]
    fn missing_evidence_is_not_treated_as_support() {
        assert_eq!(
            aggregate(&[CapabilityCompatibilityStatus::NoEvidence]),
            ExecutionCapabilityStatus::Unresolved
        );
    }

    #[test]
    fn indeterminate_evidence_is_not_treated_as_support() {
        assert_eq!(
            aggregate(&[CapabilityCompatibilityStatus::Indeterminate]),
            ExecutionCapabilityStatus::Unresolved
        );
    }

    #[test]
    fn known_incompatibility_is_reported() {
        assert_eq!(
            aggregate(&[
                CapabilityCompatibilityStatus::Satisfied,
                CapabilityCompatibilityStatus::Incompatible,
            ]),
            ExecutionCapabilityStatus::Incompatible
        );
    }

    #[test]
    fn conflicting_evidence_is_never_hidden() {
        assert_eq!(
            aggregate(&[
                CapabilityCompatibilityStatus::Satisfied,
                CapabilityCompatibilityStatus::ConflictingEvidence,
            ]),
            ExecutionCapabilityStatus::ConflictingEvidence
        );
    }

    #[test]
    fn conflict_takes_precedence_over_incompatibility() {
        assert_eq!(
            aggregate(&[
                CapabilityCompatibilityStatus::Incompatible,
                CapabilityCompatibilityStatus::ConflictingEvidence,
            ]),
            ExecutionCapabilityStatus::ConflictingEvidence
        );
    }

    #[test]
    fn aggregate_status_is_independent_of_input_order() {
        let first = aggregate(&[
            CapabilityCompatibilityStatus::NoEvidence,
            CapabilityCompatibilityStatus::Incompatible,
            CapabilityCompatibilityStatus::ConflictingEvidence,
        ]);

        let second = aggregate(&[
            CapabilityCompatibilityStatus::ConflictingEvidence,
            CapabilityCompatibilityStatus::NoEvidence,
            CapabilityCompatibilityStatus::Incompatible,
        ]);

        assert_eq!(first, second);
    }

    #[test]
    fn unresolved_predicate_includes_conflicting_evidence() {
        assert!(
            ExecutionCapabilityStatus::ConflictingEvidence.is_unresolved()
        );
        assert!(ExecutionCapabilityStatus::Unresolved.is_unresolved());
        assert!(
            !ExecutionCapabilityStatus::Satisfied.is_unresolved()
        );
    }
}