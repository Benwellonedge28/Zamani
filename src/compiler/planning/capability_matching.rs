
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Compiler — Capability Matching
//!
//! Matches canonical semantic capability requirements against explicitly
//! scoped capability evidence belonging to discovered targets.
//!
//! This module coordinates:
//! - target discovery;
//! - canonical semantic capability requirements;
//! - target-scoped capability evidence;
//! - canonical semantic compatibility evaluation;
//! - downstream resource and realization planning.
//!
//! It does not define another capability model, evidence registry,
//! compatibility evaluator, target descriptor, or hardware inventory.
//!
//! ## Canonical authorities
//!
//! - Target identity and descriptors:
//!   `compiler::configuration::target`
//! - Target discovery:
//!   `compiler::planning::target_discovery`
//! - Compiler capability analysis:
//!   `compiler::analysis::capabilities`
//! - Capability semantics:
//!   `frontend::semantic::capabilities`
//! - Resource feasibility:
//!   `compiler::planning::resource_planning`
//! - Overall feasibility:
//!   `compiler::planning::feasibility`
//!
//! ## Important invariants
//!
//! 1. Missing evidence never proves support.
//! 2. Unknown evidence never proves support.
//! 3. Conflicting evidence never proves support.
//! 4. Evidence for another target is never combined with this target's
//!    evidence.
//! 5. Empty requirements never prove compatibility.
//! 6. Conflicting target identities are never admitted.
//! 7. Discovery failures remain visible in the report.
//! 8. Capability satisfaction does not establish resource feasibility.
//! 9. Capability satisfaction does not establish backend availability.
//! 10. Capability satisfaction does not establish authorization.
//! 11. Capability satisfaction does not establish physical availability.
//! 12. Capability satisfaction does not establish execution success.
//!
//! ## Target roles
//!
//! The existing target-role contract identifies `execution`, `simulation`,
//! and `emulation` as computation-realization roles. These are all eligible
//! for capability evaluation.
//!
//! Other roles, including intermediate-representation and deployment-package
//! roles, are not execution candidates.
//!
//! Unknown roles fail closed. A future role that represents a computation
//! realization must be explicitly integrated into this classification.
//!
//! ## Scalability
//!
//! No universal maximum is imposed on:
//! - targets;
//! - providers;
//! - capability identities;
//! - capability requirements;
//! - evidence records;
//! - computational domains;
//! - hardware families;
//! - distributed participants.
//!
//! Evidence is indexed by subject once per matching operation. The index
//! contains references rather than duplicating the entire evidence inventory.
//! A target-specific evidence set is constructed only when that target is
//! evaluated.
//!
//! Actual operation remains bounded by available memory, address space,
//! processing time, and explicit operational budgets. Literal infinity is
//! not promised.
//!
//! ## Trust
//!
//! Capability evidence is a claim, not proof of authenticity.
//! Authentication, issuer trust, provenance verification, and authorization
//! remain the responsibility of their authoritative security subsystems.

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use crate::compiler::analysis::capabilities::{
    analyze_capabilities,
    CapabilityAnalysisDisposition,
    CapabilityAnalysisError,
    CapabilityAnalysisReport,
    CapabilityEvidence,
    CapabilityEvidenceError,
    CapabilityEvidenceSet,
    CapabilityRequirements,
    EvidenceSubject,
};

use crate::compiler::configuration::target::{
    TargetDescriptor,
    TargetId,
    TargetRole,
};

use crate::compiler::planning::target_discovery::{
    DiscoveredTarget,
    DiscoveryProviderId,
    DiscoveryReport,
    ProviderFailure,
    RejectedDescriptor,
    TargetIdentityConflict,
};

/// Stable architectural identity of this module.
pub const CAPABILITY_MATCHING_MODULE_NAME: &str =
    "zamani.compiler.planning.capability_matching";

/// Public API version of this module.
///
/// Version 2 includes the corrected computation-realization role handling
/// and the target-indexed evidence evaluation contract.
pub const CAPABILITY_MATCHING_API_VERSION: u16 = 2;

// ============================================================================
// TARGET EVIDENCE IDENTITY
// ============================================================================

/// Construct the canonical evidence subject for a target.
///
/// The target identifier is used unchanged. No vendor, architecture,
/// display-name, or operating-system inference is performed.
///
/// Evidence providers must use this same convention.
///
/// # Errors
///
/// Returns an error if the target identity cannot be represented as a
/// canonical semantic evidence subject.
pub fn target_evidence_subject(
    target_id: &TargetId,
) -> Result<EvidenceSubject, CapabilityMatchError> {
    EvidenceSubject::new(target_id.as_str()).map_err(|error| {
        CapabilityMatchError::InvalidEvidenceSubject {
            target: target_id.clone(),
            message: error.to_string(),
        }
    })
}

// ============================================================================
// TARGET ROLE CLASSIFICATION
// ============================================================================

/// Whether a target role represents a computation-realization target.
///
/// Simulation and emulation are included because they can execute a
/// computation according to their declared semantics. They must not be
/// excluded merely because they are not physical execution devices.
///
/// Unknown roles are rejected by default. This avoids assuming that an
/// arbitrary descriptor represents an executable target.
///
/// This classification is about target participation, not capability
/// support. The canonical capability evaluator still decides whether
/// the target satisfies the program's requirements.
fn is_computation_realization_role(role: &TargetRole) -> bool {
    matches!(
        role.as_str(),
        "execution" | "simulation" | "emulation"
    )
}

/// Whether the discovered target may participate in capability matching.
fn is_computation_realization_target(
    target: &DiscoveredTarget,
) -> bool {
    is_computation_realization_role(
        target.descriptor().role(),
    )
}

// ============================================================================
// MATCH DISPOSITION
// ============================================================================

/// Classification of one target's capability compatibility result.
///
/// This is not a resource-feasibility, backend-compatibility, authorization,
/// allocation, or execution result.
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
pub enum TargetMatchDisposition {
    /// Every evaluated capability requirement has supporting evidence.
    Satisfied,

    /// At least one requirement has explicitly incompatible evidence.
    Incompatible,

    /// At least one requirement lacks sufficient evidence, with no
    /// established incompatibility taking aggregate precedence.
    Unresolved,

    /// The program supplied no capability requirements.
    ///
    /// This is not proof of compatibility.
    NoRequirements,

    /// The descriptor does not represent a recognized computation-
    /// realization target.
    NotExecutionTarget,
}

impl TargetMatchDisposition {
    /// Whether all capability requirements are established as satisfied.
    #[must_use]
    pub const fn is_satisfied(self) -> bool {
        matches!(self, Self::Satisfied)
    }

    /// Whether an incompatibility has been established.
    #[must_use]
    pub const fn is_incompatible(self) -> bool {
        matches!(self, Self::Incompatible)
    }

    /// Whether the aggregate result requires additional evidence.
    #[must_use]
    pub const fn is_unresolved(self) -> bool {
        matches!(self, Self::Unresolved)
    }

    /// Whether the requirement collection was empty.
    #[must_use]
    pub const fn has_no_requirements(self) -> bool {
        matches!(self, Self::NoRequirements)
    }

    /// Whether this target is eligible for subsequent capability-based
    /// planning.
    ///
    /// Only a non-empty, fully satisfied requirement set is eligible.
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
            Self::NotExecutionTarget => "not a computation-realization target",
        })
    }
}

// ============================================================================
// TARGET MATCH
// ============================================================================

/// Capability-matching result for one discovered target.
///
/// The original discovery record and canonical semantic analysis report
/// are retained. No requirement provenance or compatibility outcome is
/// reconstructed or discarded.
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

    /// Return providers that reported this identical descriptor.
    #[must_use]
    pub fn providers(&self) -> &BTreeSet<DiscoveryProviderId> {
        self.discovered_target.providers()
    }

    /// Return the number of providers reporting this target.
    #[must_use]
    pub fn provider_count(&self) -> usize {
        self.discovered_target.provider_count()
    }

    /// Return the aggregate matching disposition.
    #[must_use]
    pub const fn disposition(&self) -> TargetMatchDisposition {
        self.disposition
    }

    /// Whether this target is eligible for subsequent capability planning.
    #[must_use]
    pub const fn is_eligible_for_planning(&self) -> bool {
        self.disposition.is_eligible_for_planning()
    }

    /// Return the complete canonical capability-analysis report.
    ///
    /// Non-computation targets have no analysis report.
    #[must_use]
    pub const fn analysis(
        &self,
    ) -> Option<&CapabilityAnalysisReport> {
        self.analysis.as_ref()
    }

    /// Return the number of evaluated requirements.
    #[must_use]
    pub fn requirement_count(&self) -> usize {
        self.analysis
            .as_ref()
            .map(CapabilityAnalysisReport::len)
            .unwrap_or(0)
    }

    /// Whether at least one requirement has explicitly incompatible
    /// evidence.
    ///
    /// This remains accurate when other requirements are unresolved.
    #[must_use]
    pub fn has_incompatibilities(&self) -> bool {
        self.analysis
            .as_ref()
            .is_some_and(CapabilityAnalysisReport::has_incompatibilities)
    }

    /// Whether at least one requirement remains unresolved.
    ///
    /// This remains accurate when the aggregate disposition is
    /// `Incompatible`.
    #[must_use]
    pub fn has_unresolved_requirements(&self) -> bool {
        self.analysis
            .as_ref()
            .is_some_and(CapabilityAnalysisReport::has_unresolved_requirements)
    }

    /// Whether all supplied requirements have definitive evidence.
    ///
    /// A definitive result can be satisfied or incompatible. An empty
    /// requirement set is not considered evidence-complete.
    #[must_use]
    pub fn has_complete_evidence(&self) -> bool {
        self.analysis
            .as_ref()
            .is_some_and(CapabilityAnalysisReport::has_complete_evidence)
    }
}

// ============================================================================
// MATCH REPORT
// ============================================================================

/// Complete capability-matching report.
///
/// Discovery failures, rejected descriptors, and target identity conflicts
/// are preserved independently from capability compatibility.
///
/// A satisfied candidate does not imply that the complete target inventory
/// was discovered successfully.
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

    /// Return all considered candidates in deterministic target-ID order.
    ///
    /// Non-computation targets are retained with the
    /// `NotExecutionTarget` disposition.
    #[must_use]
    pub fn candidates(&self) -> &[TargetCapabilityMatch] {
        &self.candidates
    }

    /// Iterate over targets with satisfied capability requirements.
    ///
    /// This does not establish resource feasibility, backend availability,
    /// evidence authenticity, authorization, or execution success.
    pub fn satisfied_targets(
        &self,
    ) -> impl Iterator<Item = &TargetCapabilityMatch> {
        self.candidates
            .iter()
            .filter(|candidate| candidate.is_eligible_for_planning())
    }

    /// Iterate over targets with established incompatible requirements.
    pub fn incompatible_targets(
        &self,
    ) -> impl Iterator<Item = &TargetCapabilityMatch> {
        self.candidates
            .iter()
            .filter(|candidate| candidate.disposition().is_incompatible())
    }

    /// Iterate over targets with unresolved capability requirements.
    ///
    /// A target whose aggregate result is `Incompatible` but which also
    /// has unresolved requirements can be inspected through
    /// `TargetCapabilityMatch::has_unresolved_requirements`.
    pub fn unresolved_targets(
        &self,
    ) -> impl Iterator<Item = &TargetCapabilityMatch> {
        self.candidates
            .iter()
            .filter(|candidate| candidate.disposition().is_unresolved())
    }

    /// Iterate over targets with empty requirement sets.
    pub fn targets_without_requirements(
        &self,
    ) -> impl Iterator<Item = &TargetCapabilityMatch> {
        self.candidates
            .iter()
            .filter(|candidate| candidate.disposition().has_no_requirements())
    }

    /// Find a candidate by stable target identity.
    ///
    /// Candidates are stored in ascending target-ID order, allowing
    /// logarithmic lookup.
    #[must_use]
    pub fn target(
        &self,
        target_id: &TargetId,
    ) -> Option<&TargetCapabilityMatch> {
        self.candidates
            .binary_search_by(|candidate| {
                candidate.target_id().cmp(target_id)
            })
            .ok()
            .map(|index| &self.candidates[index])
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

    /// Whether target discovery completed without reported failures,
    /// rejected descriptors, or identity conflicts.
    #[must_use]
    pub const fn discovery_is_complete(&self) -> bool {
        self.discovery_complete
    }

    /// Return provider failures.
    #[must_use]
    pub fn discovery_failures(&self) -> &[ProviderFailure] {
        &self.discovery_failures
    }

    /// Return rejected descriptors.
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
    /// This does not mean that discovery found every possible target.
    #[must_use]
    pub fn has_satisfied_target(&self) -> bool {
        self.satisfied_targets().next().is_some()
    }

    /// Whether any discovery provider failed.
    #[must_use]
    pub fn has_discovery_failures(&self) -> bool {
        !self.discovery_failures.is_empty()
    }

    /// Whether any target identity conflict occurred.
    #[must_use]
    pub fn has_discovery_conflicts(&self) -> bool {
        !self.discovery_conflicts.is_empty()
    }
}

// ============================================================================
// EVIDENCE INDEX
// ============================================================================

/// Index evidence by its canonical subject.
///
/// The index stores references to existing evidence records. It does not
/// clone or duplicate the entire evidence inventory.
///
/// Every record is validated before it enters the index, including records
/// for subjects that are not ultimately evaluated.
fn index_evidence<'a>(
    evidence: &'a CapabilityEvidenceSet,
) -> Result<
    BTreeMap<EvidenceSubject, Vec<&'a CapabilityEvidence>>,
    CapabilityMatchError,
> {
    let mut index: BTreeMap<
        EvidenceSubject,
        Vec<&'a CapabilityEvidence>,
    > = BTreeMap::new();

    for record in evidence.iter() {
        record
            .validate()
            .map_err(CapabilityMatchError::from)?;

        index
            .entry(record.subject().clone())
            .or_default()
            .push(record);
    }

    Ok(index)
}

/// Construct the evidence set for exactly one subject.
///
/// Evidence from other targets is never included.
fn evidence_for_subject(
    subject: &EvidenceSubject,
    index: &BTreeMap<
        EvidenceSubject,
        Vec<&CapabilityEvidence>,
    >,
) -> Result<CapabilityEvidenceSet, CapabilityMatchError> {
    let mut scoped = CapabilityEvidenceSet::new();

    if let Some(records) = index.get(subject) {
        scoped
            .extend(records.iter().map(|record| (**record).clone()))
            .map_err(CapabilityMatchError::from)?;
    }

    Ok(scoped)
}

// ============================================================================
// CANONICAL ANALYSIS ADAPTER
// ============================================================================

/// Convert the canonical analysis disposition without weakening its result.
///
/// Unknown future dispositions fail closed.
fn map_analysis_disposition(
    disposition: CapabilityAnalysisDisposition,
) -> TargetMatchDisposition {
    match disposition {
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
    }
}

/// Analyze one target using only its indexed evidence.
fn analyze_target(
    requirements: &CapabilityRequirements,
    target: &DiscoveredTarget,
    evidence_index: &BTreeMap<
        EvidenceSubject,
        Vec<&CapabilityEvidence>,
    >,
) -> Result<TargetCapabilityMatch, CapabilityMatchError> {
    if !is_computation_realization_target(target) {
        return Ok(TargetCapabilityMatch::new(
            target.clone(),
            TargetMatchDisposition::NotExecutionTarget,
            None,
        ));
    }

    let subject = target_evidence_subject(target.id())?;

    let scoped_evidence = evidence_for_subject(
        &subject,
        evidence_index,
    )?;

    let analysis = analyze_capabilities(
        requirements,
        &subject,
        &scoped_evidence,
    )
    .map_err(CapabilityMatchError::CapabilityAnalysis)?;

    let disposition = map_analysis_disposition(
        analysis.disposition(),
    );

    Ok(TargetCapabilityMatch::new(
        target.clone(),
        disposition,
        Some(analysis),
    ))
}

// ============================================================================
// MATCHING SERVICE
// ============================================================================

/// Stateless compiler capability-matching service.
///
/// All semantic compatibility decisions remain delegated to the canonical
/// frontend capability evaluator through the compiler analysis facade.
#[derive(Clone, Copy, Debug, Default)]
pub struct CapabilityMatcher;

impl CapabilityMatcher {
    /// Match canonical capability requirements against discovered targets.
    ///
    /// Evidence may contain records for many targets. It is validated and
    /// indexed once by exact evidence subject. Each target receives only
    /// its own evidence records.
    ///
    /// Discovery failures are preserved. Successfully discovered targets
    /// may still be individually assessed when discovery is incomplete.
    ///
    /// Conflicting target identities are excluded defensively.
    ///
    /// # Errors
    ///
    /// Returns an error for invalid evidence, invalid evidence-subject
    /// construction, or failure of canonical compatibility evaluation.
    pub fn match_targets(
        requirements: &CapabilityRequirements,
        discovery: &DiscoveryReport,
        evidence: &CapabilityEvidenceSet,
    ) -> Result<CapabilityMatchReport, CapabilityMatchError> {
        let evidence_index = index_evidence(evidence)?;

        let mut report = CapabilityMatchReport::new(
            discovery,
            requirements.len(),
        );

        let conflicted_ids: BTreeSet<TargetId> = discovery
            .conflicts()
            .iter()
            .map(|conflict| conflict.target.clone())
            .collect();

        for discovered in discovery.targets() {
            if conflicted_ids.contains(discovered.id()) {
                continue;
            }

            let candidate = analyze_target(
                requirements,
                discovered,
                &evidence_index,
            )?;

            report.candidates.push(candidate);
        }

        // DiscoveryReport currently supplies targets in BTreeMap order.
        // Keep this explicit invariant in case its internal representation
        // changes in the future.
        report.candidates.sort_by(|left, right| {
            left.target_id().cmp(right.target_id())
        });

        Ok(report)
    }

    /// Match requirements against one discovered target.
    ///
    /// This uses the same target identity convention, evidence validation,
    /// role classification, and canonical evaluator as `match_targets`.
    ///
    /// It is useful for incremental planning when one unambiguous target
    /// has been discovered.
    ///
    /// # Errors
    ///
    /// Returns an error for invalid evidence or compatibility evaluation.
    pub fn match_target(
        requirements: &CapabilityRequirements,
        target: &DiscoveredTarget,
        evidence: &CapabilityEvidenceSet,
    ) -> Result<TargetCapabilityMatch, CapabilityMatchError> {
        let evidence_index = index_evidence(evidence)?;

        analyze_target(
            requirements,
            target,
            &evidence_index,
        )
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
    /// Canonical evidence validation or compatibility evaluation failed.
    CapabilityAnalysis(CapabilityAnalysisError),

    /// A target identity could not be represented as a canonical evidence
    /// subject.
    InvalidEvidenceSubject {
        /// Target whose identity failed conversion.
        target: TargetId,

        /// Underlying validation explanation.
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

impl From<CapabilityEvidenceError> for CapabilityMatchError {
    fn from(error: CapabilityEvidenceError) -> Self {
        Self::CapabilityAnalysis(
            CapabilityAnalysisError::from(error),
        )
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::BTreeMap;

    use crate::compiler::configuration::target::{
        TargetDescriptor,
        TargetId,
        TargetRole,
    };

    use crate::compiler::planning::target_discovery::{
        DiscoveryPolicy,
        DiscoveryProviderId,
        DiscoveryRequest,
        ProviderFailure,
        TargetDiscoveryProvider,
        TargetDiscoveryRegistry,
    };

    use crate::frontend::ast::node::node_id::NodeId;
    use crate::frontend::ast::node::source::span::{
        SourceId,
        Span,
    };

    use crate::frontend::semantic::capabilities::evidence::{
        CapabilityEvidence,
        CapabilityEvidenceStatus,
        EvidenceOrigin,
    };

    use crate::frontend::semantic::capabilities::model::{
        CapabilityId,
        CapabilityRequirement,
    };

    use crate::frontend::semantic::capabilities::requirements::{
        CapabilityRequirementRecord,
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

    fn target(id: &str) -> TargetDescriptor {
        TargetDescriptor::builder(
            TargetId::new(id).expect("valid target ID"),
            id,
        )
        .build()
        .expect("valid target descriptor")
    }

    fn target_with_role(
        id: &str,
        role: &str,
    ) -> TargetDescriptor {
        TargetDescriptor::builder(
            TargetId::new(id).expect("valid target ID"),
            id,
        )
        .role(
            TargetRole::new(role).expect("valid target role"),
        )
        .build()
        .expect("valid target descriptor")
    }

    fn discovery_with(
        descriptors: Vec<TargetDescriptor>,
    ) -> DiscoveryReport {
        let provider = TestProvider {
            id: DiscoveryProviderId::new("test.provider")
                .expect("valid provider ID"),
            descriptors,
        };

        let mut registry = TargetDiscoveryRegistry::new();

        registry
            .register(provider)
            .expect("provider registration should succeed");

        registry
            .discover(
                &DiscoveryRequest::all_registered(),
                DiscoveryPolicy::best_effort(),
            )
            .expect("discovery should succeed")
    }

    fn capability(name: &str) -> CapabilityId {
        CapabilityId::new("test.domain", name)
            .expect("valid capability ID")
    }

    fn requirements(names: &[&str]) -> CapabilityRequirements {
        let mut requirements = CapabilityRequirements::new();

        for (index, name) in names.iter().enumerate() {
            let node_id = NodeId::new(
                u64::try_from(index + 1).expect("test index fits"),
            )
            .expect("valid node ID");

            let span = Span::from_usize(
                SourceId::from_raw(1),
                index,
                index + 1,
            )
            .expect("valid source span");

            let record = CapabilityRequirementRecord::new(
                node_id,
                span,
                CapabilityRequirement::new(capability(name)),
            );

            requirements
                .insert_record(record)
                .expect("valid requirement");
        }

        requirements
    }

    fn evidence(
        subject: &str,
        capability_name: &str,
        status: CapabilityEvidenceStatus,
    ) -> CapabilityEvidence {
        CapabilityEvidence::new(
            capability(capability_name),
            EvidenceSubject::new(subject)
                .expect("valid evidence subject"),
            status,
            None,
            BTreeMap::new(),
            EvidenceOrigin::new(
                "test",
                "test.provider",
                None,
            )
            .expect("valid evidence origin"),
        )
        .expect("valid evidence")
    }

    fn evidence_set(
        records: impl IntoIterator<Item = CapabilityEvidence>,
    ) -> CapabilityEvidenceSet {
        let mut set = CapabilityEvidenceSet::new();

        set.extend(records)
            .expect("valid evidence records");

        set
    }

    #[test]
    fn target_evidence_subject_uses_exact_target_identity() {
        let id = TargetId::new("future.photonic-device")
            .expect("valid target ID");

        let subject = target_evidence_subject(&id)
            .expect("valid evidence subject");

        assert_eq!(subject.as_str(), id.as_str());
    }

    #[test]
    fn empty_requirements_are_not_compatible() {
        let discovery = discovery_with(vec![
            target("test.execution"),
        ]);

        let report = CapabilityMatcher::match_targets(
            &CapabilityRequirements::new(),
            &discovery,
            &CapabilityEvidenceSet::new(),
        )
        .expect("matching should succeed");

        assert_eq!(report.len(), 1);

        let candidate = &report.candidates()[0];

        assert_eq!(
            candidate.disposition(),
            TargetMatchDisposition::NoRequirements,
        );

        assert!(!candidate.is_eligible_for_planning());
        assert!(!report.has_satisfied_target());
    }

    #[test]
    fn supported_target_evidence_satisfies_requirements() {
        let requirements = requirements(&["compute"]);

        let evidence = evidence_set([evidence(
            "test.execution",
            "compute",
            CapabilityEvidenceStatus::Supported,
        )]);

        let discovery = discovery_with(vec![
            target("test.execution"),
        ]);

        let report = CapabilityMatcher::match_targets(
            &requirements,
            &discovery,
            &evidence,
        )
        .expect("matching should succeed");

        let candidate = &report.candidates()[0];

        assert_eq!(
            candidate.disposition(),
            TargetMatchDisposition::Satisfied,
        );

        assert!(candidate.is_eligible_for_planning());
        assert!(candidate.has_complete_evidence());
        assert_eq!(candidate.requirement_count(), 1);
    }

    #[test]
    fn evidence_for_another_target_never_satisfies_a_requirement() {
        let requirements = requirements(&["compute"]);

        let evidence = evidence_set([evidence(
            "another.target",
            "compute",
            CapabilityEvidenceStatus::Supported,
        )]);

        let discovery = discovery_with(vec![
            target("test.execution"),
        ]);

        let report = CapabilityMatcher::match_targets(
            &requirements,
            &discovery,
            &evidence,
        )
        .expect("matching should succeed");

        let candidate = &report.candidates()[0];

        assert_eq!(
            candidate.disposition(),
            TargetMatchDisposition::Unresolved,
        );

        assert!(candidate.has_unresolved_requirements());
        assert!(!candidate.is_eligible_for_planning());
    }

    #[test]
    fn unsupported_evidence_is_incompatible() {
        let requirements = requirements(&["compute"]);

        let evidence = evidence_set([evidence(
            "test.execution",
            "compute",
            CapabilityEvidenceStatus::Unsupported,
        )]);

        let discovery = discovery_with(vec![
            target("test.execution"),
        ]);

        let report = CapabilityMatcher::match_targets(
            &requirements,
            &discovery,
            &evidence,
        )
        .expect("matching should succeed");

        let candidate = &report.candidates()[0];

        assert_eq!(
            candidate.disposition(),
            TargetMatchDisposition::Incompatible,
        );

        assert!(candidate.has_incompatibilities());
        assert!(candidate.has_complete_evidence());
    }

    #[test]
    fn mixed_incompatible_and_unresolved_results_remain_visible() {
        let requirements = requirements(&[
            "known_unsupported",
            "unknown_support",
        ]);

        let evidence = evidence_set([evidence(
            "test.execution",
            "known_unsupported",
            CapabilityEvidenceStatus::Unsupported,
        )]);

        let discovery = discovery_with(vec![
            target("test.execution"),
        ]);

        let report = CapabilityMatcher::match_targets(
            &requirements,
            &discovery,
            &evidence,
        )
        .expect("matching should succeed");

        let candidate = &report.candidates()[0];

        assert_eq!(
            candidate.disposition(),
            TargetMatchDisposition::Incompatible,
        );

        assert!(candidate.has_incompatibilities());
        assert!(candidate.has_unresolved_requirements());
        assert!(!candidate.has_complete_evidence());
        assert!(!candidate.is_eligible_for_planning());
    }

    #[test]
    fn simulation_targets_are_capability_evaluated() {
        let discovery = discovery_with(vec![
            target_with_role("test.simulator", "simulation"),
        ]);

        let report = CapabilityMatcher::match_targets(
            &CapabilityRequirements::new(),
            &discovery,
            &CapabilityEvidenceSet::new(),
        )
        .expect("matching should succeed");

        let candidate = &report.candidates()[0];

        assert_eq!(
            candidate.disposition(),
            TargetMatchDisposition::NoRequirements,
        );

        assert!(candidate.analysis().is_some());
    }

    #[test]
    fn emulation_targets_are_capability_evaluated() {
        let discovery = discovery_with(vec![
            target_with_role("test.emulator", "emulation"),
        ]);

        let report = CapabilityMatcher::match_targets(
            &CapabilityRequirements::new(),
            &discovery,
            &CapabilityEvidenceSet::new(),
        )
        .expect("matching should succeed");

        assert_eq!(
            report.candidates()[0].disposition(),
            TargetMatchDisposition::NoRequirements,
        );
    }

    #[test]
    fn intermediate_representation_is_not_an_execution_candidate() {
        let discovery = discovery_with(vec![
            target_with_role(
                "test.portable-ir",
                "intermediate-representation",
            ),
        ]);

        let report = CapabilityMatcher::match_targets(
            &CapabilityRequirements::new(),
            &discovery,
            &CapabilityEvidenceSet::new(),
        )
        .expect("matching should succeed");

        let candidate = &report.candidates()[0];

        assert_eq!(
            candidate.disposition(),
            TargetMatchDisposition::NotExecutionTarget,
        );

        assert!(candidate.analysis().is_none());
    }

    #[test]
    fn unknown_target_roles_fail_closed() {
        let discovery = discovery_with(vec![
            target_with_role(
                "test.future-role",
                "future-unknown-role",
            ),
        ]);

        let report = CapabilityMatcher::match_targets(
            &CapabilityRequirements::new(),
            &discovery,
            &CapabilityEvidenceSet::new(),
        )
        .expect("matching should succeed");

        assert_eq!(
            report.candidates()[0].disposition(),
            TargetMatchDisposition::NotExecutionTarget,
        );
    }

    #[test]
    fn target_results_are_deterministically_ordered() {
        let discovery = discovery_with(vec![
            target("zeta.target"),
            target("alpha.target"),
            target("middle.target"),
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
    fn candidate_lookup_uses_stable_target_identity() {
        let discovery = discovery_with(vec![
            target("alpha.target"),
            target("beta.target"),
        ]);

        let report = CapabilityMatcher::match_targets(
            &CapabilityRequirements::new(),
            &discovery,
            &CapabilityEvidenceSet::new(),
        )
        .expect("matching should succeed");

        let id = TargetId::new("beta.target")
            .expect("valid target ID");

        assert_eq!(
            report.target(&id).map(TargetCapabilityMatch::target_id),
            Some(&id),
        );
    }

    #[test]
    fn discovery_completeness_is_preserved() {
        let discovery = discovery_with(vec![
            target("test.execution"),
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
    fn module_identity_and_api_version_are_stable() {
        assert_eq!(
            CapabilityMatcher::module_name(),
            CAPABILITY_MATCHING_MODULE_NAME,
        );

        assert_eq!(
            CapabilityMatcher::api_version(),
            CAPABILITY_MATCHING_API_VERSION,
        );

        assert_eq!(CAPABILITY_MATCHING_API_VERSION, 2);
    }
}
