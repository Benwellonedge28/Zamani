
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
//! This module evaluates the canonical semantic capability requirements of
//! a program against explicitly scoped capability evidence.
//!
//! It delegates the actual semantic evaluation to:
//!
//! `crate::frontend::semantic::capabilities::evaluate_compatibility`
//!
//! It does not create a second capability model, evidence registry,
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
//! Semantic capability analysis
//!      |
//!      v
//! CapabilityRequirements
//!      |
//!      +------------------------------+
//!      |                              |
//!      v                              v
//! Compiler capability analysis   Scoped capability evidence
//!      |                              |
//!      +---------------+--------------+
//!                      |
//!                      v
//!       Canonical semantic compatibility evaluator
//!                      |
//!                      v
//!          Compiler capability-analysis report
//!                      |
//!                      v
//!          Target compatibility / feasibility
//!                      |
//!                      v
//!          Planning / lowering / backend
//! ```
//!
//! ## OWNS
//!
//! - The compiler-facing capability-analysis entry point.
//! - Compiler-facing classification of semantic compatibility results.
//! - Preservation of the canonical semantic compatibility report.
//! - The distinction between no requirements, satisfied requirements,
//!   incompatible requirements, and unresolved requirements.
//!
//! ## DOES_NOT_OWN
//!
//! - Capability syntax or grammar.
//! - AST capability declarations.
//! - Semantic capability identities.
//! - Semantic capability requirements.
//! - Capability evidence storage.
//! - Evidence authentication or trust verification.
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
//! - `frontend::semantic::capabilities::model`
//! - `frontend::semantic::capabilities::requirements`
//! - `frontend::semantic::capabilities::evidence`
//! - `frontend::semantic::capabilities::compatibility`
//! - `compiler::analysis::program`
//! - `compiler::api::compatibility`
//! - `backend::compatibility`
//!
//! Only the canonical frontend capability model and evaluator are directly
//! required by this implementation.
//!
//! Backend and compiler API compatibility remain downstream authorities.
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
//! - distributed nodes;
//! - hardware capacity.
//!
//! The canonical semantic model uses dynamically sized collections.
//!
//! This module does not impose a separate semantic limit or silently truncate
//! input. Actual processing remains bounded by available memory, address
//! space, allocator behavior, and caller-configured operational budgets.
//!
//! Literal infinity is not a realizable guarantee for a finite computer.
//!
//! ## DETERMINISM
//!
//! This module preserves the ordering of the canonical semantic report.
//! It does not introduce randomized maps, randomized identifiers, global
//! mutable registries, or order-dependent evidence selection.
//!
//! ## INTEGRATION
//!
//! Input:
//!   - `CapabilityRequirements`
//!   - `EvidenceSubject`
//!   - `CapabilityEvidenceSet`
//!
//! Output:
//!   - `CapabilityAnalysisReport`
//!
//! The report contains the original canonical
//! `CapabilityCompatibilityReport`; individual source-associated records
//! are not reconstructed or discarded.
//!
//! The compiler pipeline may consume the disposition while diagnostics,
//! tooling, and reporting consumers inspect the complete canonical report.

use std::fmt;

pub use crate::frontend::semantic::capabilities::{
    CapabilityCompatibilityError as CapabilityAnalysisError,
    CapabilityCompatibilityRecord,
    CapabilityCompatibilityReport,
    CapabilityCompatibilityStatus,
    CapabilityCompatibilitySummary,
    CapabilityEvidenceSet,
    CapabilityRequirements,
    EvidenceSubject,
};

use crate::frontend::semantic::capabilities::evaluate_compatibility;

// =============================================================================
// Module identity
// =============================================================================

/// Stable architectural identity of this compiler analysis module.
pub const CAPABILITY_ANALYSIS_MODULE_NAME: &str =
    "zamani.compiler.analysis.capabilities";

/// Version of this module's compiler-facing report contract.
///
/// This is not the Zamani language version, capability-definition version,
/// backend version, or artifact serialization version.
pub const CAPABILITY_ANALYSIS_SCHEMA_VERSION: u16 = 1;

// =============================================================================
// Analysis disposition
// =============================================================================

/// Compiler-facing classification of a canonical capability compatibility
/// report.
///
/// This is deliberately not a replacement for
/// `CapabilityCompatibilityStatus`. The canonical report retains the
/// individual outcome of every requirement.
///
/// The disposition summarizes the report for compiler orchestration.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum CapabilityAnalysisDisposition {
    /// There were no capability requirements to evaluate.
    ///
    /// This is not evidence that a target supports a program's requirements.
    NoRequirements,

    /// Every evaluated requirement has evidence establishing support.
    Satisfied,

    /// At least one requirement is explicitly incompatible.
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

    /// Whether more evidence is needed.
    #[must_use]
    pub const fn is_unresolved(self) -> bool {
        matches!(self, Self::Unresolved)
    }

    /// Whether no capability requirements were supplied.
    #[must_use]
    pub const fn has_no_requirements(self) -> bool {
        matches!(self, Self::NoRequirements)
    }

    /// Whether this report establishes successful capability compatibility.
    ///
    /// NoRequirements deliberately does not return true.
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
// Capability-analysis report
// =============================================================================

/// Compiler-facing capability-analysis result.
///
/// The canonical semantic report is retained in full. This wrapper adds only
/// the compiler-level disposition needed for pipeline orchestration.
///
/// It does not discard source AST identities, source spans, individual
/// capability requirements, evidence outcomes, or the evaluated subject.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CapabilityAnalysisReport {
    compatibility: CapabilityCompatibilityReport,
    disposition: CapabilityAnalysisDisposition,
}

impl CapabilityAnalysisReport {
    /// Construct a compiler-facing report from the canonical semantic report.
    ///
    /// This constructor is public so that other compiler components can
    /// adapt an already evaluated canonical report without re-running
    /// semantic analysis.
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

    /// Return the compiler-facing disposition.
    #[must_use]
    pub const fn disposition(&self) -> CapabilityAnalysisDisposition {
        self.disposition
    }

    /// Return the evaluated evidence subject.
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

    /// Iterate over the canonical source-associated compatibility records.
    pub fn iter(
        &self,
    ) -> impl ExactSizeIterator<Item = &CapabilityCompatibilityRecord> {
        self.compatibility.iter()
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
    /// This method intentionally does not treat an empty requirement set
    /// as evidence of target compatibility.
    #[must_use]
    pub const fn is_compatible(&self) -> bool {
        self.disposition.is_compatible()
    }

    /// Whether at least one requirement is explicitly incompatible.
    #[must_use]
    pub const fn has_incompatibilities(&self) -> bool {
        self.disposition.is_incompatible()
    }

    /// Whether compatibility remains unresolved.
    #[must_use]
    pub const fn needs_more_evidence(&self) -> bool {
        self.disposition.is_unresolved()
    }

    /// Whether the report contains no requirements.
    #[must_use]
    pub const fn has_no_requirements(&self) -> bool {
        self.disposition.has_no_requirements()
    }
}

// =============================================================================
// Capability analysis service
// =============================================================================

/// Stateless compiler-facing capability-analysis service.
///
/// All authoritative semantic data and compatibility behavior remain owned
/// by the frontend semantic capability subsystem.
#[derive(Clone, Copy, Debug, Default)]
pub struct CapabilityAnalysis;

impl CapabilityAnalysis {
    /// Analyze semantic capability requirements against evidence for one
    /// explicitly identified subject.
    ///
    /// Evidence belonging to other subjects is not combined with the
    /// requested subject.
    ///
    /// The canonical evaluator validates evidence before evaluating it.
    ///
    /// # Errors
    ///
    /// Returns `CapabilityAnalysisError` when the canonical evidence
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
        Self.analyze(&Self, requirements, subject, evidence)
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
// Public convenience function
// =============================================================================

/// Analyze semantic capability requirements for one evidence subject.
///
/// This is the preferred free-function entry point for compiler stages.
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
// Classification
// =============================================================================

/// Classify a canonical report without changing or reconstructing its
/// individual results.
fn classify(
    report: &CapabilityCompatibilityReport,
) -> CapabilityAnalysisDisposition {
    let summary = report.summary();

    if summary.total() == 0 {
        return CapabilityAnalysisDisposition::NoRequirements;
    }

    // An explicit incompatibility takes precedence in the aggregate
    // disposition. The original report still preserves every other result.
    if summary.incompatible() > 0 {
        return CapabilityAnalysisDisposition::Incompatible;
    }

    // Missing, incomplete, or contradictory evidence cannot establish
    // successful compatibility.
    if summary.no_evidence() > 0
        || summary.indeterminate() > 0
        || summary.conflicting_evidence() > 0
    {
        return CapabilityAnalysisDisposition::Unresolved;
    }

    if summary.satisfied() == summary.total() {
        return CapabilityAnalysisDisposition::Satisfied;
    }

    // Conservative defensive fallback. A future extension of the canonical
    // status model must not accidentally become a successful result here.
    CapabilityAnalysisDisposition::Unresolved
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn subject() -> EvidenceSubject {
        EvidenceSubject::new("compiler.capability-analysis.test")
            .expect("test evidence subject must be valid")
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
        assert!(report.has_no_requirements());

        assert_eq!(report.summary().total(), 0);
        assert!(!report.summary().all_satisfied());
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
    fn service_and_free_function_use_the_same_contract() {
        let requirements = CapabilityRequirements::new();
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
