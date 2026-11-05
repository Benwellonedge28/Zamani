
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Compiler — Capability Matching
//!
//! Path: src/compiler/planning/capability_matching.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! ## PURPOSE
//!
//! Match canonical semantic capability requirements against explicitly
//! scoped capability evidence for discovered compilation targets.
//!
//! This module connects:
//!
//! - compiler target discovery;
//! - canonical frontend capability requirements;
//! - canonical capability evidence;
//! - canonical semantic compatibility evaluation;
//! - downstream resource and realization planning.
//!
//! It does not define another capability model or compatibility engine.
//!
//! ## AUTHORITATIVE OWNERS
//!
//! Target identity and descriptors:
//!     crate::compiler::configuration::target
//!
//! Target discovery:
//!     crate::compiler::planning::target_discovery
//!
//! Compiler-facing capability analysis:
//!     crate::compiler::analysis::capabilities
//!
//! Canonical capability requirements:
//!     crate::frontend::semantic::capabilities::requirements
//!
//! Canonical capability evidence:
//!     crate::frontend::semantic::capabilities::evidence
//!
//! Canonical compatibility evaluation:
//!     crate::frontend::semantic::capabilities::compatibility
//!
//! Backend compatibility:
//!     crate::backend::compatibility
//!
//! ## OWNERSHIP
//!
//! This module owns:
//!
//! - Matching discovered targets against semantic capability evidence.
//! - The target-evidence subject identity convention.
//! - Per-target matching results.
//! - Deterministic candidate ordering.
//! - Preservation of discovery diagnostics.
//! - Classification of satisfied, incompatible, unresolved, and
//!   requirement-free target results.
//!
//! This module does not own:
//!
//! - Capability definitions.
//! - Capability requirements.
//! - Capability evidence storage.
//! - Evidence authentication or trust.
//! - Hardware discovery.
//! - Target descriptors.
//! - Backend registration.
//! - Backend implementation.
//! - Resource sufficiency.
//! - Resource allocation.
//! - Scheduling.
//! - Partitioning.
//! - Routing.
//! - Physical placement.
//! - Runtime execution.
//! - Semantic analysis.
//! - IR definitions.
//!
//! ## EVIDENCE SUBJECT CONVENTION
//!
//! A target's evidence subject is exactly its canonical TargetId string.
//!
//! For example:
//!
//!     TargetId("x86_64-linux")
//!
//! uses:
//!
//!     EvidenceSubject("x86_64-linux")
//!
//! Evidence providers must use this convention when creating
//! CapabilityEvidence records.
//!
//! No vendor or architecture inference is performed.
//!
//! ## SEMANTIC INVARIANTS
//!
//! 1. Missing evidence is not proof of support.
//! 2. Unknown evidence is not proof of support.
//! 3. Conflicting evidence is not proof of support.
//! 4. Explicitly unsupported capabilities remain incompatible.
//! 5. Empty requirements are not proof of compatibility.
//! 6. Evidence belonging to another target is never combined.
//! 7. Discovery conflicts never produce an eligible target.
//! 8. Discovery failures remain visible in the result.
//! 9. Capability compatibility does not establish resource sufficiency.
//! 10. Capability compatibility does not establish authorization.
//! 11. Capability compatibility does not authenticate evidence.
//! 12. Capability compatibility does not prove backend code generation.
//! 13. A semantically valid program is not made semantically invalid
//!     merely because a discovered target cannot realize it.
//!
//! ## SCALABILITY
//!
//! No fixed maximum is imposed on:
//!
//! - targets;
//! - capabilities;
//! - capability requirements;
//! - evidence records;
//! - computing domains;
//! - providers;
//! - processors;
//! - accelerators;
//! - devices;
//! - distributed nodes;
//! - resource quantities.
//!
//! Collections grow according to available memory.
//!
//! This module does not promise literal infinite computation.
//! Resource exhaustion must remain an explicit implementation failure.
//!
//! ## DETERMINISM
//!
//! Target discovery supplies targets in stable TargetId order.
//! Capability requirements preserve canonical semantic ordering.
//! The canonical evaluator determines requirement outcomes.
//! This module introduces no random ordering or global mutable state.
//!
//! ## TRUST
//!
//! CapabilityEvidence is a claim, not proof of authenticity.
//!
//! Evidence authentication, issuer trust, signatures, and authorization
//! must be completed by their authoritative security and provenance
//! subsystems before the result is used as a trusted deployment decision.
//!
//! ## DOWNSTREAM INTEGRATION
//!
//! The result may be consumed by:
//!
//! - resource_planning.rs;
//! - partitioning.rs;
//! - feasibility.rs;
//! - backend selection;
//! - deployment planning.
//!
//! This module never allocates or executes a target.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use crate::compiler::analysis::capabilities::{
    analyze_capabilities,
    CapabilityAnalysisDisposition,
    CapabilityAnalysisError,
    CapabilityAnalysisReport,
    CapabilityEvidenceSet,
    CapabilityRequirements,
    EvidenceSubject,
};

use crate::compiler::planning::target_discovery::{
    DiscoveryReport,
    DiscoveryProviderId,
    DiscoveredTarget,
    ProviderFailure,
    RejectedDescriptor,
    TargetIdentityConflict,
};

use crate::compiler::configuration::target::{
    TargetDescriptor,
    TargetId,
};

// ============================================================================
// MODULE IDENTITY
// ============================================================================

/// Stable architectural identity of this module.
pub const CAPABILITY_MATCHING_MODULE_NAME: &str =
    "zamani.compiler.planning.capability_matching";

/// Public API version of this module.
pub const CAPABILITY_MATCHING_API_VERSION: u16 = 1;

// ============================================================================
// TARGET EVIDENCE SUBJECT
// ============================================================================

/// Construct the canonical evidence subject for a discovered target.
///
/// The target identifier is used unchanged. This ensures that capability
/// evidence cannot accidentally be associated with another target through
/// architecture aliases, vendor names, or display names.
///
/// Evidence providers must use the same convention.
///
/// # Errors
///
/// Returns the canonical evidence-model validation error if construction
/// of the subject fails. Validated TargetId values should satisfy the
/// evidence subject's textual validation requirements.
pub fn target_evidence_subject(
    target_id: &TargetId,
) -> Result<EvidenceSubject, CapabilityMatchError> {
    EvidenceSubject::new(target_id.as_str())
        .map_err(|error| {
            CapabilityMatchError::InvalidEvidenceSubject {
                target: target_id.clone(),
                message: error.to_string(),
            }
        })
}

// ============================================================================
// MATCH DISPOSITION
// ============================================================================

/// Classification of one discovered target's capability result.
///
/// This classification is intentionally separate from target discovery,
/// resource feasibility, backend compatibility, and execution availability.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum TargetMatchDisposition {
    /// All evaluated capability requirements have supporting evidence.
    ///
    /// This does not prove resource sufficiency, evidence authenticity,
    /// authorization, or executable backend availability.
    Satisfied,

    /// At least one requirement has explicit incompatible evidence.
    Incompatible,

    /// At least one requirement lacks sufficient evidence.
    Unresolved,

    /// The program supplied no capability requirements.
    ///
    /// This is not proof that the target is compatible.
    NoRequirements,

    /// The discovered descriptor does not represent an execution target.
    ///
    /// Such a descriptor may describe an IR format, deployment package,
    /// or other non-execution artifact.
    NotExecutionTarget,
}

impl TargetMatchDisposition {
    /// Whether the canonical evidence establishes satisfaction.
    #[must_use]
    pub const fn is_satisfied(self) -> bool {
        matches!(self, Self::Satisfied)
    }

    /// Whether an explicit incompatibility was established.
    #[must_use]
    pub const fn is_incompatible(self) -> bool {
        matches!(self, Self::Incompatible)
    }

    /// Whether additional evidence is required.
    #[must_use]
    pub const fn is_unresolved(self) -> bool {
        matches!(self, Self::Unresolved)
    }

    /// Whether the requirement set was empty.
    #[must_use]
    pub const fn has_no_requirements(self) -> bool {
        matches!(self, Self::NoRequirements)
    }

    /// Whether this target is eligible for subsequent capability-based
    /// planning.
    ///
    /// Only Satisfied returns true.
    #[must_use]
    pub const fn is_eligible_for_planning(self) -> bool {
        self.is_satisfied()
    }
}

impl fmt::Display for TargetMatchDisposition {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(match self {
            Self::Satisfied => "capability requirements satisfied",
            Self::Incompatible => "capability requirements incompatible",
            Self::Unresolved => "capability evidence unresolved",
            Self::NoRequirements => "no capability requirements",
            Self::NotExecutionTarget => "not an execution target",
        })
    }
}

// ============================================================================
// ONE TARGET MATCH
// ============================================================================

/// Capability-matching result for one discovered target.
///
/// The original discovery record is retained, including the providers
/// that reported the identical target descriptor.
///
/// The canonical capability analysis report is retained without
/// reconstructing or discarding source-associated requirement records.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TargetCapabilityMatch {
    discovered_target: DiscoveredTarget,
    disposition: TargetMatchDisposition,
    analysis: Option<CapabilityAnalysisReport>,
}

impl TargetCapabilityMatch {
    fn new(
        discovered_target: DiscoveredTarget,
        disposition: TargetMatchDisposition,
        analysis: Option<CapabilityAnalysisReport>,
    ) -> Self {
        Self {
            discovered_target,
            disposition,
            analysis,
        }
    }

    /// Return the original discovered target.
    #[must_use]
    pub fn discovered_target(&self) -> &DiscoveredTarget {
        &self.discovered_target
    }

    /// Return the canonical target descriptor.
    #[must_use]
    pub fn descriptor(&self) -> &TargetDescriptor {
        self.discovered_target.descriptor()
    }

    /// Return the stable target identity.
    #[must_use]
    pub fn target_id(&self) -> &TargetId {
        self.discovered_target.id()
    }

    /// Return providers that reported this target.
    #[must_use]
    pub fn providers(&self) -> &BTreeSet<DiscoveryProviderId> {
        self.discovered_target.providers()
    }

    /// Return the number of providers reporting this target.
    #[must_use]
    pub fn provider_count(&self) -> usize {
        self.discovered_target.provider_count()
    }

    /// Return the matching disposition.
    #[must_use]
    pub const fn disposition(&self) -> TargetMatchDisposition {
        self.disposition
    }

    /// Return whether the target is eligible for subsequent
    /// capability-based planning.
    #[must_use]
    pub const fn is_eligible_for_planning(&self) -> bool {
        self.disposition.is_eligible_for_planning()
    }

    /// Return the complete canonical capability-analysis report.
    ///
    /// Non-execution targets have no capability analysis report.
    #[must_use]
    pub const fn analysis(
        &self,
    ) -> Option<&CapabilityAnalysisReport> {
        self.analysis.as_ref()
    }

    /// Return the number of evaluated semantic capability requirements.
    ///
    /// Non-execution targets return zero because no analysis was performed.
    #[must_use]
    pub fn requirement_count(&self) -> usize {
        self.analysis
            .as_ref()
            .map(CapabilityAnalysisReport::len)
            .unwrap_or(0)
    }
}

// ============================================================================
// MATCHING REPORT
// ============================================================================

/// Complete capability-matching report.
///
/// The report retains:
///
/// - every non-conflicting discovered target considered;
/// - each target's canonical capability-analysis result;
/// - discovery provider failures;
/// - rejected discovery descriptors;
/// - target identity conflicts;
/// - whether the original discovery inventory was complete.
///
/// A report with one or more satisfied targets does not imply that the
/// complete target inventory was discovered successfully.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CapabilityMatchReport {
    candidates: Vec<TargetCapabilityMatch>,
    discovery_failures: Vec<ProviderFailure>,
    discovery_rejections: Vec<RejectedDescriptor>,
    discovery_conflicts: Vec<TargetIdentityConflict>,
    discovery_complete: bool,
    requirement_count: usize,
}

impl CapabilityMatchReport {
    fn new(
        discovery: &DiscoveryReport,
        requirement_count: usize,
    ) -> Self {
        Self {
            candidates: Vec::new(),
            discovery_failures: discovery.failures().to_vec(),
            discovery_rejections: discovery
                .rejected_descriptors()
                .to_vec(),
            discovery_conflicts: discovery.conflicts().to_vec(),
            discovery_complete: discovery.is_complete(),
            requirement_count,
        }
    }

    /// Return all evaluated target results in deterministic target-ID order.
    ///
    /// Non-execution targets are included with the
    /// `NotExecutionTarget` disposition.
    #[must_use]
    pub fn candidates(&self) -> &[TargetCapabilityMatch] {
        &self.candidates
    }

    /// Iterate over targets whose capability requirements are satisfied.
    ///
    /// This iterator does not establish resource feasibility, backend
    /// availability, evidence authenticity, or authorization.
    pub fn satisfied_targets(
        &self,
    ) -> impl Iterator<Item = &TargetCapabilityMatch> {
        self.candidates
            .iter()
            .filter(|candidate| candidate.is_eligible_for_planning())
    }

    /// Iterate over targets with explicitly incompatible evidence.
    pub fn incompatible_targets(
        &self,
    ) -> impl Iterator<Item = &TargetCapabilityMatch> {
        self.candidates.iter().filter(|candidate| {
            candidate.disposition().is_incompatible()
        })
    }

    /// Iterate over targets for which capability evidence is incomplete.
    pub fn unresolved_targets(
        &self,
    ) -> impl Iterator<Item = &TargetCapabilityMatch> {
        self.candidates.iter().filter(|candidate| {
            candidate.disposition().is_unresolved()
        })
    }

    /// Iterate over targets analyzed with an empty requirement set.
    pub fn targets_without_requirements(
        &self,
    ) -> impl Iterator<Item = &TargetCapabilityMatch> {
        self.candidates.iter().filter(|candidate| {
            candidate.disposition().has_no_requirements()
        })
    }

    /// Return a candidate by target identity.
    #[must_use]
    pub fn target(
        &self,
        target_id: &TargetId,
    ) -> Option<&TargetCapabilityMatch> {
        self.candidates
            .iter()
            .find(|candidate| candidate.target_id() == target_id)
    }

    /// Return the number of considered target descriptors.
    #[must_use]
    pub fn len(&self) -> usize {
        self.candidates.len()
    }

    /// Whether no target descriptors were considered.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.candidates.is_empty()
    }

    /// Return the number of semantic capability requirements.
    #[must_use]
    pub const fn requirement_count(&self) -> usize {
        self.requirement_count
    }

    /// Whether discovery completed without provider failures, rejected
    /// descriptors, or target identity conflicts.
    #[must_use]
    pub const fn discovery_is_complete(&self) -> bool {
        self.discovery_complete
    }

    /// Return discovery provider failures.
    #[must_use]
    pub fn discovery_failures(&self) -> &[ProviderFailure] {
        &self.discovery_failures
    }

    /// Return rejected discovery descriptors.
    #[must_use]
    pub fn discovery_rejections(&self) -> &[RejectedDescriptor] {
        &self.discovery_rejections
    }

    /// Return target identity conflicts.
    #[must_use]
    pub fn discovery_conflicts(&self) -> &[TargetIdentityConflict] {
        &self.discovery_conflicts
    }

    /// Whether at least one candidate has satisfied capability evidence.
    ///
    /// This is not a global inventory-completeness claim.
    #[must_use]
    pub fn has_satisfied_target(&self) -> bool {
        self.satisfied_targets().next().is_some()
    }

    /// Whether any provider failed during target discovery.
    #[must_use]
    pub fn has_discovery_failures(&self) -> bool {
        !self.discovery_failures.is_empty()
    }

    /// Whether any discovered target identity was ambiguous.
    #[must_use]
    pub fn has_discovery_conflicts(&self) -> bool {
        !self.discovery_conflicts.is_empty()
    }
}

// ============================================================================
// MATCHING SERVICE
// ============================================================================

/// Stateless compiler capability-matching service.
///
/// The service delegates compatibility evaluation to the canonical
/// frontend semantic capability implementation through the existing
/// compiler-facing analysis facade.
#[derive(Clone, Copy, Debug, Default)]
pub struct CapabilityMatcher;

impl CapabilityMatcher {
    /// Match semantic capability requirements against a discovered
    /// target inventory and scoped evidence.
    ///
    /// The supplied evidence set may contain evidence for multiple targets.
    /// The canonical evaluator filters it by the exact target evidence
    /// subject for each target.
    ///
    /// Discovery failures are preserved in the report. Successfully
    /// discovered targets can still be evaluated, but the report explicitly
    /// records that the inventory may be incomplete.
    ///
    /// Target identity conflicts are excluded defensively, even if a
    /// malformed or older discovery report happens to expose a target
    /// whose identity is also listed as conflicted.
    ///
    /// Non-execution target descriptors are retained as
    /// `NotExecutionTarget` results and are not evaluated as execution
    /// candidates.
    ///
    /// # Errors
    ///
    /// Returns an error if canonical capability evidence validation or
    /// compatibility evaluation fails.
    ///
    /// Invalid evidence is not silently converted into an unresolved
    /// or successful match.
    pub fn match_targets(
        requirements: &CapabilityRequirements,
        discovery: &DiscoveryReport,
        evidence: &CapabilityEvidenceSet,
    ) -> Result<CapabilityMatchReport, CapabilityMatchError> {
        let mut report = CapabilityMatchReport::new(
            discovery,
            requirements.len(),
        );

        // Discovery reports are ordered by TargetId. The explicit conflict
        // set also protects against the current discovery aggregation edge
        // case where a later observation could reintroduce a conflicted ID.
        let conflicted_ids: BTreeSet<TargetId> = discovery
            .conflicts()
            .iter()
            .map(|conflict| conflict.target.clone())
            .collect();

        for discovered in discovery.targets() {
            if conflicted_ids.contains(discovered.id()) {
                continue;
            }

            let descriptor = discovered.descriptor();

            // Only execution targets are candidates for execution planning.
            // Target roles are open strings; this comparison uses the
            // established role identifier rather than introducing an enum.
            if descriptor.role().as_str() != "execution" {
                report.candidates.push(TargetCapabilityMatch::new(
                    discovered.clone(),
                    TargetMatchDisposition::NotExecutionTarget,
                    None,
                ));

                continue;
            }

            let subject = target_evidence_subject(discovered.id())?;

            let analysis = analyze_capabilities(
                requirements,
                &subject,
                evidence,
            )
            .map_err(CapabilityMatchError::CapabilityAnalysis)?;

            let disposition = match analysis.disposition() {
                CapabilityAnalysisDisposition::Satisfied => {
                    TargetMatchDisposition::Satisfied
                }

                CapabilityAnalysisDisposition::Incompatible => {
                    TargetMatchDisposition::Incompatible
                }

                CapabilityAnalysisDisposition::Unresolved => {
                    TargetMatchDisposition::Unresolved
                }

                CapabilityAnalysisDisposition::NoRequirements => {
                    TargetMatchDisposition::NoRequirements
                }

                // CapabilityAnalysisDisposition is non-exhaustive.
                // A future disposition must not become a successful match.
                _ => TargetMatchDisposition::Unresolved,
            };

            report.candidates.push(TargetCapabilityMatch::new(
                discovered.clone(),
                disposition,
                Some(analysis),
            ));
        }

        Ok(report)
    }

    /// Match requirements against a single discovered target.
    ///
    /// This method uses the same canonical target-evidence identity
    /// convention as `match_targets`.
    ///
    /// It is useful for incremental planning when one target is added
    /// to an existing inventory.
    ///
    /// # Errors
    ///
    /// Returns an error for invalid evidence or compatibility evaluation.
    pub fn match_target(
        requirements: &CapabilityRequirements,
        target: &DiscoveredTarget,
        evidence: &CapabilityEvidenceSet,
    ) -> Result<TargetCapabilityMatch, CapabilityMatchError> {
        let descriptor = target.descriptor();

        if descriptor.role().as_str() != "execution" {
            return Ok(TargetCapabilityMatch::new(
                target.clone(),
                TargetMatchDisposition::NotExecutionTarget,
                None,
            ));
        }

        let subject = target_evidence_subject(target.id())?;

        let analysis = analyze_capabilities(
            requirements,
            &subject,
            evidence,
        )
        .map_err(CapabilityMatchError::CapabilityAnalysis)?;

        let disposition = match analysis.disposition() {
            CapabilityAnalysisDisposition::Satisfied => {
                TargetMatchDisposition::Satisfied
            }

            CapabilityAnalysisDisposition::Incompatible => {
                TargetMatchDisposition::Incompatible
            }

            CapabilityAnalysisDisposition::Unresolved => {
                TargetMatchDisposition::Unresolved
            }

            CapabilityAnalysisDisposition::NoRequirements => {
                TargetMatchDisposition::NoRequirements
            }

            _ => TargetMatchDisposition::Unresolved,
        };

        Ok(TargetCapabilityMatch::new(
            target.clone(),
            disposition,
            Some(analysis),
        ))
    }

    /// Return the stable architectural module name.
    #[must_use]
    pub const fn module_name() -> &'static str {
        CAPABILITY_MATCHING_MODULE_NAME
    }

    /// Return the public API version.
    #[must_use]
    pub const fn api_version() -> u16 {
        CAPABILITY_MATCHING_API_VERSION
    }
}

// ============================================================================
// ERRORS
// ============================================================================

/// Errors produced by capability matching.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CapabilityMatchError {
    /// Canonical capability evidence or compatibility evaluation failed.
    CapabilityAnalysis(CapabilityAnalysisError),

    /// A target identity could not be converted into a canonical evidence
    /// subject.
    InvalidEvidenceSubject {
        /// Target whose identity failed conversion.
        target: TargetId,

        /// Underlying validation message.
        message: String,
    },
}

impl fmt::Display for CapabilityMatchError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::CapabilityAnalysis(error) => {
                write!(
                    formatter,
                    "capability matching failed during canonical \
                     semantic compatibility analysis: {error}"
                )
            }

            Self::InvalidEvidenceSubject { target, message } => {
                write!(
                    formatter,
                    "cannot construct capability evidence subject \
                     for target `{target}`: {message}"
                )
            }
        }
    }
}

impl Error for CapabilityMatchError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::CapabilityAnalysis(error) => Some(error),
            Self::InvalidEvidenceSubject { .. } => None,
        }
    }
}

impl From<CapabilityAnalysisError> for CapabilityMatchError {
    fn from(error: CapabilityAnalysisError) -> Self {
        Self::CapabilityAnalysis(error)
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::compiler::configuration::target::{
        TargetDescriptor,
        TargetId,
    };

    use crate::compiler::planning::target_discovery::{
        DiscoveryProviderId,
        DiscoveryRequest,
        TargetDiscoveryProvider,
        TargetDiscoveryRegistry,
        ProviderFailure,
    };

    use crate::frontend::semantic::capabilities::{
        CapabilityEvidenceSet,
    };

    use crate::frontend::semantic::capabilities::requirements::{
        CapabilityRequirements,
    };

    struct TestProvider {
        id: DiscoveryProviderId,
        descriptors: Vec<TargetDescriptor>,
    }

    impl TargetDiscoveryProvider for TestProvider {
        fn id(&self) -> &DiscoveryProviderId {
            &self.id
        }

        fn discover(
            &self,
            _request: &DiscoveryRequest,
        ) -> Result<Vec<TargetDescriptor>, ProviderFailure> {
            Ok(self.descriptors.clone())
        }
    }

    fn execution_target(id: &str) -> TargetDescriptor {
        TargetDescriptor::builder(
            TargetId::new(id).expect("valid target identifier"),
            id,
        )
        .build()
        .expect("valid target descriptor")
    }

    fn discovery_with(
        descriptors: Vec<TargetDescriptor>,
    ) -> DiscoveryReport {
        let provider_id = DiscoveryProviderId::new("test.provider")
            .expect("valid provider identity");

        let provider = TestProvider {
            id: provider_id,
            descriptors,
        };

        let mut registry = TargetDiscoveryRegistry::new();

        registry
            .register(provider)
            .expect("provider registration should succeed");

        registry
            .discover(
                &DiscoveryRequest::all_registered(),
                crate::compiler::planning::target_discovery::DiscoveryPolicy::best_effort(),
            )
            .expect("discovery should succeed")
    }

    #[test]
    fn target_evidence_subject_uses_exact_target_identity() {
        let id = TargetId::new("future.photonic-device")
            .expect("valid target ID");

        let subject = target_evidence_subject(&id)
            .expect("target evidence subject should be valid");

        assert_eq!(subject.as_str(), id.as_str());
    }

    #[test]
    fn empty_requirements_do_not_create_a_compatible_target() {
        let discovery = discovery_with(vec![
            execution_target("test.execution"),
        ]);

        let requirements = CapabilityRequirements::new();
        let evidence = CapabilityEvidenceSet::new();

        let report = CapabilityMatcher::match_targets(
            &requirements,
            &discovery,
            &evidence,
        )
        .expect("empty requirements are valid");

        assert_eq!(report.len(), 1);
        assert_eq!(
            report.candidates()[0].disposition(),
            TargetMatchDisposition::NoRequirements,
        );

        assert!(!report.has_satisfied_target());
        assert!(
            !report.candidates()[0].is_eligible_for_planning()
        );
    }

    #[test]
    fn non_execution_descriptors_are_not_execution_candidates() {
        let descriptor = TargetDescriptor::builder(
            TargetId::new("test.portable-ir")
                .expect("valid target ID"),
            "Portable IR",
        )
        .role(
            crate::compiler::configuration::target::TargetRole::new(
                "intermediate-representation",
            )
            .expect("valid target role"),
        )
        .build()
        .expect("valid descriptor");

        let discovery = discovery_with(vec![descriptor]);

        let report = CapabilityMatcher::match_targets(
            &CapabilityRequirements::new(),
            &discovery,
            &CapabilityEvidenceSet::new(),
        )
        .expect("matching should succeed");

        assert_eq!(report.len(), 1);
        assert_eq!(
            report.candidates()[0].disposition(),
            TargetMatchDisposition::NotExecutionTarget,
        );
    }

    #[test]
    fn discovery_completeness_is_preserved() {
        let discovery = discovery_with(vec![
            execution_target("test.execution"),
        ]);

        let report = CapabilityMatcher::match_targets(
            &CapabilityRequirements::new(),
            &discovery,
            &CapabilityEvidenceSet::new(),
        )
        .expect("matching should succeed");

        assert!(report.discovery_is_complete());
        assert!(!report.has_discovery_failures());
        assert!(!report.has_discovery_conflicts());
    }

    #[test]
    fn target_results_are_deterministically_ordered() {
        let discovery = discovery_with(vec![
            execution_target("zeta.target"),
            execution_target("alpha.target"),
            execution_target("middle.target"),
        ]);

        let report = CapabilityMatcher::match_targets(
            &CapabilityRequirements::new(),
            &discovery,
            &CapabilityEvidenceSet::new(),
        )
        .expect("matching should succeed");

        let ids: Vec<&str> = report
            .candidates()
            .iter()
            .map(|candidate| candidate.target_id().as_str())
            .collect();

        assert_eq!(
            ids,
            vec![
                "alpha.target",
                "middle.target",
                "zeta.target",
            ],
        );
    }

    #[test]
    fn module_identity_is_stable() {
        assert_eq!(
            CapabilityMatcher::module_name(),
            CAPABILITY_MATCHING_MODULE_NAME,
        );

        assert_eq!(
            CapabilityMatcher::api_version(),
            CAPABILITY_MATCHING_API_VERSION,
        );
    }
}
