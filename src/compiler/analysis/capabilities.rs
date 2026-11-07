
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Zamani Compiler — Capability Analysis
//!
//! Path: `src/compiler/analysis/capabilities.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97+
//! Safety: Unsafe Rust is forbidden.
//!
//! ## PURPOSE
//!
//! Provide the compiler-facing capability-analysis boundary.
//!
//! This module evaluates canonical frontend semantic capability
//! requirements against explicitly scoped capability evidence.
//!
//! The authoritative evaluation is delegated to:
//!
//! `crate::frontend::semantic::capabilities::evaluate_compatibility`
//!
//! This module adds compiler-facing reporting and aggregate classification.
//! It does not create a competing capability model, evidence registry,
//! compatibility algorithm, target model, or resource model.
//!
//! ## ARCHITECTURAL POSITION
//!
//! ```text
//! Zamani source
//!      |
//!      v
//! Canonical grammar
//!      |
//!      v
//! Frontend AST
//!      |
//!      v
//! Frontend semantic analysis
//!      |
//!      v
//! CapabilityRequirements
//!      |
//!      +-------------------------------+
//!      |                               |
//!      v                               v
//! Compiler capability analysis    Scoped capability evidence
//!      |                               |
//!      +---------------+---------------+
//!                      |
//!                      v
//!       Canonical semantic compatibility evaluator
//!                      |
//!                      v
//!          Compiler capability-analysis report
//!                      |
//!                      v
//!          Target matching / feasibility
//!                      |
//!                      v
//!          Planning / lowering / backend
//! ```
//!
//! ## OWNS
//!
//! - The compiler-facing capability-analysis entry point.
//! - Compiler-facing aggregate classification.
//! - Preservation of the canonical semantic compatibility report.
//! - Convenient inspection of individual compatibility outcomes.
//! - Explicit distinction between empty, satisfied, incompatible,
//!   and unresolved requirement sets.
//! - Accurate reporting when incompatible and unresolved requirements
//!   occur in the same analysis.
//!
//! ## DOES_NOT_OWN
//!
//! - Capability syntax or grammar.
//! - AST capability declarations.
//! - Semantic capability identities.
//! - Semantic capability requirements.
//! - Capability evidence storage.
//! - Evidence authentication or trust verification.
//! - Capability-definition registries.
//! - Backend capability inventories.
//! - Target discovery or target selection.
//! - Resource requirements or resource feasibility.
//! - Authorization or security policy.
//! - Physical resource allocation.
//! - Scheduling, routing, placement, or execution.
//! - ZUIR or any domain-specific IR.
//!
//! ## AUTHORITATIVE DEPENDENCIES
//!
//! Canonical capability model:
//! `frontend::semantic::capabilities::model`
//!
//! Canonical requirement collection:
//! `frontend::semantic::capabilities::requirements`
//!
//! Canonical evidence:
//! `frontend::semantic::capabilities::evidence`
//!
//! Canonical compatibility evaluation:
//! `frontend::semantic::capabilities::compatibility`
//!
//! Target discovery and target-specific evidence scoping are downstream
//! responsibilities of:
//! `compiler::planning::target_discovery`
//! `compiler::planning::capability_matching`
//!
//! Backend capability declarations and backend compatibility remain owned
//! by the backend subsystem.
//!
//! ## SEMANTIC INVARIANTS
//!
//! 1. An empty requirement set is not proof of target compatibility.
//! 2. Missing evidence is not proof of capability support.
//! 3. Incomplete evidence is not proof of capability support.
//! 4. Contradictory evidence must remain visible.
//! 5. Evidence from another subject must not be combined with the requested
//!    subject's evidence.
//! 6. An incompatible target does not automatically make the source program
//!    semantically invalid.
//! 7. Capability compatibility does not establish resource sufficiency.
//! 8. Capability compatibility does not establish authorization.
//! 9. Capability compatibility does not authenticate evidence.
//! 10. A capability name does not imply that a target implements it.
//! 11. A satisfied capability report does not prove backend code generation.
//! 12. A satisfied capability report does not guarantee later execution.
//! 13. An incompatible result must not hide unresolved results in the same
//!     report.
//! 14. Unknown future capability identifiers remain the responsibility of
//!     the canonical semantic model and its extension contracts.
//!
//! ## SCALABILITY
//!
//! This module introduces no fixed maximum for:
//!
//! - capability requirements;
//! - capability evidence records;
//! - capability namespaces;
//! - capability domains;
//! - targets;
//! - devices;
//! - processors;
//! - accelerators;
//! - quantum resources;
//! - distributed participants;
//! - hardware capacity.
//!
//! It introduces no universal `MAX_*` constants, closed hardware inventory,
//! fixed device count, or fixed capability count.
//!
//! The canonical semantic model uses dynamically sized collections.
//! This module neither truncates input nor imposes another semantic limit.
//!
//! Processing is necessarily subject to available memory, address space,
//! allocator behavior, processing capacity, and explicitly configured
//! operational budgets.
//!
//! Literal infinity is not a realizable guarantee for a finite computer.
//!
//! Operational budget exhaustion must be reported by the owning compilation
//! session or pipeline. It must never be converted into successful or
//! complete capability analysis.
//!
//! ## DETERMINISM
//!
//! This module preserves the canonical semantic report and its ordering.
//!
//! The canonical requirement collection orders records by source AST node
//! identity. The canonical compatibility evaluator preserves that order.
//!
//! This module introduces no randomized maps, random identifiers, timestamps,
//! global mutable registries, or order-dependent evidence selection.
//!
//! ## TRUST
//!
//! Capability evidence is a claim, not proof of authenticity.
//!
//! Evidence authentication, issuer trust, signature verification,
//! authorization, and provenance verification remain responsibilities of
//! their authoritative security and provenance subsystems.
//!
//! Consumers must not use an unauthenticated capability report as a
//! deployment authorization decision.
//!
//! ## INTEGRATION
//!
//! Inputs:
//! - `CapabilityRequirements`
//! - `EvidenceSubject`
//! - `CapabilityEvidenceSet`
//!
//! Output:
//! - `CapabilityAnalysisReport`
//!
//! The report retains the original canonical
//! `CapabilityCompatibilityReport`.
//!
//! Target-specific matching remains in:
//! `compiler::planning::capability_matching`
//!
//! Resource feasibility remains in:
//! `compiler::planning::resource_planning`
//! and `compiler::planning::feasibility`.
//!
//! Backend compatibility remains in:
//! `compiler::backends::compatibility`
//! and the authoritative backend compatibility subsystem.
//!
//! The report does not claim that any target has been selected, allocated,
//! reserved, authorized, or executed.

use std::fmt;

pub use crate::frontend::semantic::capabilities::{
    CapabilityCompatibilityError as CapabilityAnalysisError,
    CapabilityCompatibilityRecord,
    CapabilityCompatibilityReport,
    CapabilityCompatibilityStatus,
    CapabilityCompatibilitySummary,
    CapabilityEvidence,
    CapabilityEvidenceError,
    CapabilityEvidenceResult,
    CapabilityEvidenceSet,
    CapabilityEvidenceStatus,
    CapabilityRequirement,
    CapabilityRequirementError,
    CapabilityRequirementRecord,
    CapabilityRequirements,
    CapabilityVersion,
    CapabilityVersionConstraint,
    EvidenceEvaluation,
    EvidenceOrigin,
    EvidenceSubject,
};

use crate::frontend::semantic::capabilities::evaluate_compatibility;

// =============================================================================
// MODULE IDENTITY
// =============================================================================

/// Stable architectural identity of this compiler analysis module.
pub const CAPABILITY_ANALYSIS_MODULE_NAME: &str =
    "zamani.compiler.analysis.capabilities";

/// Version of this module's compiler-facing report contract.
///
/// This is not the Zamani language version, capability-definition version,
/// backend version, target-profile version, or artifact serialization
/// version.
pub const CAPABILITY_ANALYSIS_SCHEMA_VERSION: u16 = 2;

// =============================================================================
// ANALYSIS DISPOSITION
// =============================================================================

/// Aggregate classification of a canonical capability compatibility report.
///
/// This is deliberately not a replacement for
/// [`CapabilityCompatibilityStatus`].
///
/// The canonical report retains the individual outcome of every requirement.
/// This disposition provides a concise aggregate classification for
/// compiler orchestration.
///
/// When a report contains both incompatible and unresolved requirements,
/// `Incompatible` is the primary disposition. The report's summary and
/// inspection methods continue to expose the unresolved requirements.
///
/// Therefore, consumers must not use this enum alone when they need to know
/// whether further evidence is also required.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
#[non_exhaustive]
pub enum CapabilityAnalysisDisposition {
    /// There were no capability requirements to evaluate.
    ///
    /// This is not evidence that a target supports a program's requirements.
    NoRequirements,

    /// Every evaluated requirement has evidence establishing support.
    Satisfied,

    /// At least one requirement has explicit incompatible evidence.
    ///
    /// Other requirements may still be unresolved.
    Incompatible,

    /// No explicit incompatibility was established, but one or more
    /// requirements lack sufficient evidence.
    Unresolved,
}

impl CapabilityAnalysisDisposition {
    /// Whether the evidence establishes that all evaluated requirements
    /// are satisfied.
    ///
    /// Empty requirement sets return false.
    #[must_use]
    pub const fn is_satisfied(self) -> bool {
        matches!(self, Self::Satisfied)
    }

    /// Whether an explicit incompatibility was established.
    #[must_use]
    pub const fn is_incompatible(self) -> bool {
        matches!(self, Self::Incompatible)
    }

    /// Whether the aggregate classification is unresolved.
    ///
    /// Use [`CapabilityAnalysisReport::has_unresolved_requirements`] to
    /// inspect the report itself. A report may be classified as incompatible
    /// and still contain unresolved requirements.
    #[must_use]
    pub const fn is_unresolved(self) -> bool {
        matches!(self, Self::Unresolved)
    }

    /// Whether no capability requirements were supplied.
    #[must_use]
    pub const fn has_no_requirements(self) -> bool {
        matches!(self, Self::NoRequirements)
    }

    /// Whether this disposition establishes successful capability
    /// compatibility.
    ///
    /// `NoRequirements` deliberately does not return true.
    #[must_use]
    pub const fn is_compatible(self) -> bool {
        self.is_satisfied()
    }
}

impl fmt::Display for CapabilityAnalysisDisposition {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::NoRequirements => "no capability requirements",
            Self::Satisfied => "all capability requirements satisfied",
            Self::Incompatible => "capability requirements incompatible",
            Self::Unresolved => "capability compatibility unresolved",
        })
    }
}

// =============================================================================
// CAPABILITY-ANALYSIS REPORT
// =============================================================================

/// Compiler-facing capability-analysis result.
///
/// The complete canonical semantic report is retained. This wrapper adds
/// aggregate classification and convenient inspection methods without
/// reconstructing, merging, or discarding source-associated records.
///
/// The report is an evidence-based compatibility result. It is not a
/// resource-feasibility, authorization, backend-realization, or execution
/// certificate.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CapabilityAnalysisReport {
    compatibility: CapabilityCompatibilityReport,
    disposition: CapabilityAnalysisDisposition,
}

impl CapabilityAnalysisReport {
    /// Construct a compiler-facing report from a canonical semantic report.
    ///
    /// This constructor allows another compiler component to adapt an
    /// already evaluated canonical report without repeating semantic
    /// capability evaluation.
    #[must_use]
    pub fn from_compatibility_report(
        compatibility: CapabilityCompatibilityReport,
    ) -> Self {
        let disposition = classify(&compatibility);

        Self {
            compatibility,
            disposition,
        }
    }

    /// Return the complete canonical semantic compatibility report.
    #[must_use]
    pub const fn compatibility(&self) -> &CapabilityCompatibilityReport {
        &self.compatibility
    }

    /// Consume this wrapper and return the canonical semantic report.
    #[must_use]
    pub fn into_compatibility(self) -> CapabilityCompatibilityReport {
        self.compatibility
    }

    /// Return the compiler-facing aggregate disposition.
    #[must_use]
    pub const fn disposition(&self) -> CapabilityAnalysisDisposition {
        self.disposition
    }

    /// Return the exact evidence subject that was evaluated.
    #[must_use]
    pub fn subject(&self) -> &str {
        self.compatibility.subject()
    }

    /// Return the number of evaluated requirements.
    #[must_use]
    pub fn len(&self) -> usize {
        self.compatibility.len()
    }

    /// Whether no requirements were evaluated.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.compatibility.is_empty()
    }

    /// Return the canonical aggregate summary.
    #[must_use]
    pub const fn summary(&self) -> &CapabilityCompatibilitySummary {
        self.compatibility.summary()
    }

    /// Iterate over all canonical source-associated compatibility records.
    ///
    /// Records retain the ordering established by the canonical semantic
    /// requirement collection.
    pub fn iter(
        &self,
    ) -> impl ExactSizeIterator<Item = &CapabilityCompatibilityRecord> {
        self.compatibility.iter()
    }

    /// Iterate over requirements whose evidence establishes support.
    pub fn satisfied(
        &self,
    ) -> impl Iterator<Item = &CapabilityCompatibilityRecord> {
        self.iter().filter(|record| {
            record.status() == CapabilityCompatibilityStatus::Satisfied
        })
    }

    /// Iterate over requirements with explicit incompatible evidence.
    pub fn incompatible(
        &self,
    ) -> impl Iterator<Item = &CapabilityCompatibilityRecord> {
        self.iter().filter(|record| {
            record.status() == CapabilityCompatibilityStatus::Incompatible
        })
    }

    /// Iterate over requirements with missing, indeterminate, or conflicting
    /// evidence.
    pub fn unresolved(
        &self,
    ) -> impl Iterator<Item = &CapabilityCompatibilityRecord> {
        self.iter().filter(|record| record.status().is_unresolved())
    }

    /// Find the compatibility result for one source AST node.
    #[must_use]
    pub fn get(
        &self,
        source_node: crate::frontend::ast::node::node_id::NodeId,
    ) -> Option<&CapabilityCompatibilityRecord> {
        self.compatibility.get(source_node)
    }

    /// Whether every evaluated requirement is established as satisfied.
    ///
    /// This is false for an empty report.
    #[must_use]
    pub const fn all_satisfied(&self) -> bool {
        self.compatibility.all_satisfied()
    }

    /// Whether this report establishes capability compatibility.
    ///
    /// An empty requirement set does not establish compatibility.
    #[must_use]
    pub const fn is_compatible(&self) -> bool {
        self.disposition.is_compatible()
    }

    /// Whether at least one requirement is explicitly incompatible.
    #[must_use]
    pub const fn has_incompatibilities(&self) -> bool {
        self.summary().has_incompatibilities()
    }

    /// Whether at least one requirement remains unresolved.
    ///
    /// Unlike the aggregate disposition, this method remains accurate when
    /// incompatible and unresolved requirements occur together.
    #[must_use]
    pub const fn has_unresolved_requirements(&self) -> bool {
        self.summary().has_unresolved_requirements()
    }

    /// Whether further capability evidence or conflict resolution is needed.
    ///
    /// This method deliberately examines the canonical summary rather than
    /// the aggregate disposition. An incompatible report can still require
    /// further evidence for other requirements.
    #[must_use]
    pub const fn needs_more_evidence(&self) -> bool {
        self.has_unresolved_requirements()
    }

    /// Whether the report contains no requirements.
    #[must_use]
    pub const fn has_no_requirements(&self) -> bool {
        self.disposition.has_no_requirements()
    }

    /// Number of requirements whose evidence establishes support.
    #[must_use]
    pub const fn satisfied_count(&self) -> usize {
        self.summary().satisfied()
    }

    /// Number of requirements with explicit incompatible evidence.
    #[must_use]
    pub const fn incompatible_count(&self) -> usize {
        self.summary().incompatible()
    }

    /// Number of requirements for which no evidence exists.
    #[must_use]
    pub const fn no_evidence_count(&self) -> usize {
        self.summary().no_evidence()
    }

    /// Number of requirements with indeterminate evidence.
    #[must_use]
    pub const fn indeterminate_count(&self) -> usize {
        self.summary().indeterminate()
    }

    /// Number of requirements with conflicting evidence.
    #[must_use]
    pub const fn conflicting_evidence_count(&self) -> usize {
        self.summary().conflicting_evidence()
    }

    /// Whether the report has at least one evaluated requirement and every
    /// requirement has a definitive evidence outcome.
    ///
    /// A definitive outcome may be either satisfied or incompatible.
    /// An empty report is not considered evidence-complete.
    #[must_use]
    pub const fn has_complete_evidence(&self) -> bool {
        self.len() > 0 && !self.has_unresolved_requirements()
    }
}

// =============================================================================
// CAPABILITY ANALYSIS SERVICE
// =============================================================================

/// Stateless compiler-facing capability-analysis service.
///
/// All authoritative semantic data and compatibility behavior remain owned
/// by the frontend semantic capability subsystem.
///
/// This service is safe to share between compiler tasks because it contains
/// no mutable state or global registry.
#[derive(Clone, Copy, Debug, Default)]
pub struct CapabilityAnalysis;

impl CapabilityAnalysis {
    /// Analyze semantic capability requirements against evidence for one
    /// explicitly identified subject.
    ///
    /// Evidence belonging to other subjects is not combined with the
    /// requested subject's evidence.
    ///
    /// The canonical evaluator validates the complete evidence set before
    /// evaluating requirements.
    ///
    /// # Errors
    ///
    /// Returns [`CapabilityAnalysisError`] when canonical evidence
    /// validation or compatibility evaluation fails.
    ///
    /// Such an error indicates malformed or invalid evidence data. It is
    /// not itself proof that the target is incompatible.
    pub fn analyze(
        &self,
        requirements: &CapabilityRequirements,
        subject: &EvidenceSubject,
        evidence: &CapabilityEvidenceSet,
    ) -> Result<CapabilityAnalysisReport, CapabilityAnalysisError> {
        let compatibility =
            evaluate_compatibility(requirements, subject, evidence)?;

        Ok(CapabilityAnalysisReport::from_compatibility_report(
            compatibility,
        ))
    }

    /// Analyze requirements using the canonical semantic evaluator.
    ///
    /// This associated function is provided for callers that prefer not
    /// to construct a service value.
    pub fn analyze_requirements(
        requirements: &CapabilityRequirements,
        subject: &EvidenceSubject,
        evidence: &CapabilityEvidenceSet,
    ) -> Result<CapabilityAnalysisReport, CapabilityAnalysisError> {
        let compatibility =
            evaluate_compatibility(requirements, subject, evidence)?;

        Ok(CapabilityAnalysisReport::from_compatibility_report(
            compatibility,
        ))
    }

    /// Return this module's stable contract version.
    #[must_use]
    pub const fn schema_version() -> u16 {
        CAPABILITY_ANALYSIS_SCHEMA_VERSION
    }

    /// Return this module's stable architectural identity.
    #[must_use]
    pub const fn module_name() -> &'static str {
        CAPABILITY_ANALYSIS_MODULE_NAME
    }
}

// =============================================================================
// PUBLIC CONVENIENCE FUNCTION
// =============================================================================

/// Analyze semantic capability requirements for one evidence subject.
///
/// This is the preferred free-function entry point for compiler stages.
///
/// The subject must identify the scope to which the supplied evidence
/// applies. Target-specific subject construction belongs to the target
/// matching layer.
#[must_use]
pub fn analyze_capabilities(
    requirements: &CapabilityRequirements,
    subject: &EvidenceSubject,
    evidence: &CapabilityEvidenceSet,
) -> Result<CapabilityAnalysisReport, CapabilityAnalysisError> {
    CapabilityAnalysis::analyze_requirements(
        requirements,
        subject,
        evidence,
    )
}

// =============================================================================
// CLASSIFICATION
// =============================================================================

/// Classify a canonical report without changing or reconstructing its
/// individual results.
///
/// Precedence:
///
/// 1. No requirements.
/// 2. At least one explicit incompatibility.
/// 3. At least one unresolved requirement.
/// 4. Every requirement satisfied.
///
/// The report summary independently preserves both incompatible and
/// unresolved counts. Consumers must inspect those methods when they need
/// the full state rather than a single aggregate label.
fn classify(
    report: &CapabilityCompatibilityReport,
) -> CapabilityAnalysisDisposition {
    let summary = report.summary();

    if summary.total() == 0 {
        return CapabilityAnalysisDisposition::NoRequirements;
    }

    if summary.has_incompatibilities() {
        return CapabilityAnalysisDisposition::Incompatible;
    }

    if summary.has_unresolved_requirements() {
        return CapabilityAnalysisDisposition::Unresolved;
    }

    if summary.all_satisfied() {
        return CapabilityAnalysisDisposition::Satisfied;
    }

    // Defensive fallback. An unexpected future status must never become
    // successful merely because it was not explicitly recognized here.
    CapabilityAnalysisDisposition::Unresolved
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::BTreeMap;

    use crate::frontend::ast::node::node_id::NodeId;
    use crate::frontend::ast::node::source::span::{
        SourceId,
        Span,
    };
    use crate::frontend::semantic::capabilities::model::CapabilityId;

    fn subject() -> EvidenceSubject {
        EvidenceSubject::new("compiler.capability-analysis.test")
            .expect("test evidence subject must be valid")
    }

    fn capability_id() -> CapabilityId {
        CapabilityId::new(
            "zamani.quantum",
            "dynamic_control",
        )
        .expect("test capability identity must be valid")
    }

    fn requirements() -> CapabilityRequirements {
        let node_id = NodeId::new(1)
            .expect("test node identity must be valid");

        let span = Span::from_usize(
            SourceId::from_raw(1),
            0,
            1,
        )
        .expect("test source span must be valid");

        let requirement =
            CapabilityRequirement::new(capability_id());

        let record = CapabilityRequirementRecord::new(
            node_id,
            span,
            requirement,
        );

        let mut requirements = CapabilityRequirements::new();

        requirements
            .insert_record(record)
            .expect("test requirement must be valid");

        requirements
    }

    fn evidence(
        requested_subject: &EvidenceSubject,
        status: CapabilityEvidenceStatus,
        origin: &str,
    ) -> CapabilityEvidence {
        CapabilityEvidence::new(
            capability_id(),
            requested_subject.clone(),
            status,
            None,
            BTreeMap::new(),
            EvidenceOrigin::new(
                "test",
                origin,
                None,
            )
            .expect("test evidence origin must be valid"),
        )
        .expect("test capability evidence must be valid")
    }

    fn evidence_set(
        records: impl IntoIterator<Item = CapabilityEvidence>,
    ) -> CapabilityEvidenceSet {
        let mut set = CapabilityEvidenceSet::new();

        set.extend(records)
            .expect("test evidence records must be valid");

        set
    }

    #[test]
    fn empty_requirements_are_not_reported_as_compatible() {
        let requirements = CapabilityRequirements::new();
        let evidence = CapabilityEvidenceSet::new();

        let report = analyze_capabilities(
            &requirements,
            &subject(),
            &evidence,
        )
        .expect("empty capability analysis should be valid");

        assert_eq!(
            report.disposition(),
            CapabilityAnalysisDisposition::NoRequirements
        );

        assert!(report.is_empty());
        assert_eq!(report.len(), 0);
        assert!(!report.is_compatible());
        assert!(!report.all_satisfied());
        assert!(!report.has_incompatibilities());
        assert!(!report.needs_more_evidence());
        assert!(!report.has_complete_evidence());
        assert!(report.has_no_requirements());

        assert_eq!(report.summary().total(), 0);
        assert!(!report.summary().all_satisfied());
    }

    #[test]
    fn missing_evidence_is_unresolved_not_satisfied() {
        let requirements = requirements();
        let evidence = CapabilityEvidenceSet::new();

        let report = analyze_capabilities(
            &requirements,
            &subject(),
            &evidence,
        )
        .expect("capability analysis should succeed");

        assert_eq!(
            report.disposition(),
            CapabilityAnalysisDisposition::Unresolved
        );

        assert!(!report.is_compatible());
        assert!(!report.all_satisfied());
        assert!(report.has_unresolved_requirements());
        assert!(report.needs_more_evidence());
        assert!(!report.has_incompatibilities());
        assert_eq!(report.no_evidence_count(), 1);
        assert_eq!(report.unresolved().count(), 1);
    }

    #[test]
    fn supported_evidence_satisfies_the_requirement() {
        let requirements = requirements();
        let requested_subject = subject();

        let evidence = evidence_set([evidence(
            &requested_subject,
            CapabilityEvidenceStatus::Supported,
            "test-provider",
        )]);

        let report = analyze_capabilities(
            &requirements,
            &requested_subject,
            &evidence,
        )
        .expect("supported capability evidence should be valid");

        assert_eq!(
            report.disposition(),
            CapabilityAnalysisDisposition::Satisfied
        );

        assert!(report.is_compatible());
        assert!(report.all_satisfied());
        assert!(!report.has_unresolved_requirements());
        assert!(!report.has_incompatibilities());
        assert!(report.has_complete_evidence());
        assert_eq!(report.satisfied_count(), 1);
        assert_eq!(report.satisfied().count(), 1);
    }

    #[test]
    fn unsupported_evidence_is_incompatible() {
        let requirements = requirements();
        let requested_subject = subject();

        let evidence = evidence_set([evidence(
            &requested_subject,
            CapabilityEvidenceStatus::Unsupported,
            "test-provider",
        )]);

        let report = analyze_capabilities(
            &requirements,
            &requested_subject,
            &evidence,
        )
        .expect("unsupported capability evidence should be valid");

        assert_eq!(
            report.disposition(),
            CapabilityAnalysisDisposition::Incompatible
        );

        assert!(!report.is_compatible());
        assert!(report.has_incompatibilities());
        assert!(!report.has_unresolved_requirements());
        assert_eq!(report.incompatible_count(), 1);
        assert_eq!(report.incompatible().count(), 1);
        assert!(report.has_complete_evidence());
    }

    #[test]
    fn unknown_evidence_is_unresolved() {
        let requirements = requirements();
        let requested_subject = subject();

        let evidence = evidence_set([evidence(
            &requested_subject,
            CapabilityEvidenceStatus::Unknown,
            "test-provider",
        )]);

        let report = analyze_capabilities(
            &requirements,
            &requested_subject,
            &evidence,
        )
        .expect("unknown capability evidence should be valid");

        assert_eq!(
            report.disposition(),
            CapabilityAnalysisDisposition::Unresolved
        );

        assert!(!report.is_compatible());
        assert!(report.has_unresolved_requirements());
        assert!(report.needs_more_evidence());
        assert_eq!(report.indeterminate_count(), 1);
    }

    #[test]
    fn evidence_from_another_subject_is_not_combined() {
        let requirements = requirements();
        let requested_subject = subject();

        let other_subject = EvidenceSubject::new(
            "another.execution.environment",
        )
        .expect("other subject must be valid");

        let evidence = evidence_set([evidence(
            &other_subject,
            CapabilityEvidenceStatus::Supported,
            "other-provider",
        )]);

        let report = analyze_capabilities(
            &requirements,
            &requested_subject,
            &evidence,
        )
        .expect("scoped capability analysis should succeed");

        assert_eq!(report.subject(), requested_subject.as_str());
        assert_eq!(
            report.disposition(),
            CapabilityAnalysisDisposition::Unresolved
        );
        assert_eq!(report.no_evidence_count(), 1);
        assert!(!report.is_compatible());
    }

    #[test]
    fn contradictory_evidence_remains_visible() {
        let requirements = requirements();
        let requested_subject = subject();

        let evidence = evidence_set([
            evidence(
                &requested_subject,
                CapabilityEvidenceStatus::Supported,
                "provider-a",
            ),
            evidence(
                &requested_subject,
                CapabilityEvidenceStatus::Unsupported,
                "provider-b",
            ),
        ]);

        let report = analyze_capabilities(
            &requirements,
            &requested_subject,
            &evidence,
        )
        .expect("conflicting evidence should produce a report");

        assert_eq!(
            report.disposition(),
            CapabilityAnalysisDisposition::Unresolved
        );

        assert!(report.has_unresolved_requirements());
        assert!(report.needs_more_evidence());
        assert_eq!(report.conflicting_evidence_count(), 1);
        assert_eq!(report.unresolved().count(), 1);
    }

    #[test]
    fn mixed_incompatible_and_unresolved_results_preserve_both_states() {
        let first_node = NodeId::new(1)
            .expect("first node identity must be valid");

        let second_node = NodeId::new(2)
            .expect("second node identity must be valid");

        let first_span = Span::from_usize(
            SourceId::from_raw(1),
            0,
            1,
        )
        .expect("first source span must be valid");

        let second_span = Span::from_usize(
            SourceId::from_raw(1),
            2,
            3,
        )
        .expect("second source span must be valid");

        let mut requirements = CapabilityRequirements::new();

        requirements
            .insert_record(CapabilityRequirementRecord::new(
                first_node,
                first_span,
                CapabilityRequirement::new(capability_id()),
            ))
            .expect("first requirement must be valid");

        let second_capability = CapabilityId::new(
            "zamani.classical",
            "parallel_execution",
        )
        .expect("second capability identity must be valid");

        requirements
            .insert_record(CapabilityRequirementRecord::new(
                second_node,
                second_span,
                CapabilityRequirement::new(second_capability),
            ))
            .expect("second requirement must be valid");

        let requested_subject = subject();

        let evidence = evidence_set([evidence(
            &requested_subject,
            CapabilityEvidenceStatus::Unsupported,
            "test-provider",
        )]);

        let report = analyze_capabilities(
            &requirements,
            &requested_subject,
            &evidence,
        )
        .expect("mixed capability analysis should succeed");

        assert_eq!(
            report.disposition(),
            CapabilityAnalysisDisposition::Incompatible
        );

        // This is the critical regression test: an incompatible aggregate
        // disposition must not hide unresolved requirements.
        assert!(report.has_incompatibilities());
        assert!(report.has_unresolved_requirements());
        assert!(report.needs_more_evidence());

        assert_eq!(report.incompatible_count(), 1);
        assert_eq!(report.no_evidence_count(), 1);
        assert_eq!(report.incompatible().count(), 1);
        assert_eq!(report.unresolved().count(), 1);
        assert!(!report.is_compatible());
        assert!(!report.has_complete_evidence());
    }

    #[test]
    fn analysis_preserves_the_requested_evidence_subject() {
        let requirements = CapabilityRequirements::new();
        let evidence = CapabilityEvidenceSet::new();
        let requested_subject = subject();

        let report = analyze_capabilities(
            &requirements,
            &requested_subject,
            &evidence,
        )
        .expect("capability analysis should succeed");

        assert_eq!(report.subject(), requested_subject.as_str());
        assert_eq!(
            report.compatibility().subject(),
            requested_subject.as_str()
        );
    }

    #[test]
    fn report_preserves_source_associated_records() {
        let requirements = requirements();
        let requested_subject = subject();

        let evidence = evidence_set([evidence(
            &requested_subject,
            CapabilityEvidenceStatus::Supported,
            "test-provider",
        )]);

        let report = analyze_capabilities(
            &requirements,
            &requested_subject,
            &evidence,
        )
        .expect("capability analysis should succeed");

        let source_node = NodeId::new(1)
            .expect("test node identity must be valid");

        let record = report
            .get(source_node)
            .expect("source-associated result must be retained");

        assert_eq!(record.source_node(), source_node);
        assert_eq!(
            record.status(),
            CapabilityCompatibilityStatus::Satisfied
        );
    }

    #[test]
    fn service_and_free_function_use_the_same_contract() {
        let requirements = requirements();
        let evidence = CapabilityEvidenceSet::new();
        let requested_subject = subject();

        let service_report = CapabilityAnalysis
            .analyze(
                &requirements,
                &requested_subject,
                &evidence,
            )
            .expect("service analysis should succeed");

        let function_report = analyze_capabilities(
            &requirements,
            &requested_subject,
            &evidence,
        )
        .expect("free-function analysis should succeed");

        assert_eq!(service_report, function_report);
    }

    #[test]
    fn report_conversion_preserves_the_canonical_report() {
        let requirements = requirements();
        let requested_subject = subject();

        let evidence = evidence_set([evidence(
            &requested_subject,
            CapabilityEvidenceStatus::Supported,
            "test-provider",
        )]);

        let canonical = evaluate_compatibility(
            &requirements,
            &requested_subject,
            &evidence,
        )
        .expect("canonical evaluation should succeed");

        let expected = canonical.clone();

        let report =
            CapabilityAnalysisReport::from_compatibility_report(canonical);

        assert_eq!(report.compatibility(), &expected);
        assert_eq!(report.into_compatibility(), expected);
    }

    #[test]
    fn module_identity_is_stable() {
        assert_eq!(
            CapabilityAnalysis::module_name(),
            CAPABILITY_ANALYSIS_MODULE_NAME
        );

        assert_eq!(
            CapabilityAnalysis::schema_version(),
            CAPABILITY_ANALYSIS_SCHEMA_VERSION
        );
    }

    #[test]
    fn disposition_display_is_stable() {
        assert_eq!(
            CapabilityAnalysisDisposition::NoRequirements.to_string(),
            "no capability requirements"
        );

        assert_eq!(
            CapabilityAnalysisDisposition::Satisfied.to_string(),
            "all capability requirements satisfied"
        );

        assert_eq!(
            CapabilityAnalysisDisposition::Incompatible.to_string(),
            "capability requirements incompatible"
        );

        assert_eq!(
            CapabilityAnalysisDisposition::Unresolved.to_string(),
            "capability compatibility unresolved"
        );
    }
}
