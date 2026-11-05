
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Compiler — Feasibility Planning
//!
//! Path: src/compiler/planning/feasibility.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! ## PURPOSE
//!
//! Determine whether a proposed computation realization is supported by
//! explicit semantic, capability, resource, and realization evidence.
//!
//! This is a compiler-side planning authority. It does not perform physical
//! resource allocation, placement, scheduling, routing, compilation,
//! lowering, or execution.
//!
//! ## AUTHORITATIVE OWNERS
//!
//! Semantic validity:
//!     crate::frontend::semantic
//!
//! Capability analysis:
//!     crate::compiler::analysis::capabilities
//!
//! Capability matching:
//!     crate::compiler::planning::capability_matching
//!
//! Resource planning:
//!     crate::compiler::planning::resource_planning
//!
//! Dependency analysis:
//!     crate::compiler::analysis::dependency
//!
//! Canonical computation representations:
//!     crate::zuir
//!     crate::classical
//!     crate::quantum::ir
//!     crate::hdl::hdlir
//!     crate::ai
//!     crate::hybrid
//!     crate::distributed
//!
//! Downstream planning:
//!     partitioning
//!     placement
//!     scheduling
//!     routing
//!     data_movement
//!
//! Physical resource management and execution:
//!     crate::runtime
//!     hardware/provider authorities
//!
//! ## OWNS
//!
//! - Composition of independently established feasibility evidence.
//! - Structured overall feasibility outcomes.
//! - Per-target feasibility outcomes.
//! - Explicit uncertainty and infeasibility findings.
//! - Caller-configurable feasibility-analysis budgets.
//! - Deterministic feasibility reports.
//!
//! ## DOES NOT OWN
//!
//! - Source-language semantics.
//! - Capability definitions.
//! - Resource definitions.
//! - Hardware inventories.
//! - Evidence authentication.
//! - Backend implementation.
//! - Target discovery.
//! - Capability matching algorithms.
//! - Resource assessment algorithms.
//! - Dependency graph construction.
//! - Data-transfer algorithms.
//! - Physical placement.
//! - Scheduling.
//! - Routing.
//! - Device allocation.
//! - Runtime execution.
//! - Quantum routing or QEC.
//! - Any canonical IR.
//!
//! ## IMPORTANT SEMANTIC RULES
//!
//! 1. Unknown evidence is never proof of feasibility.
//! 2. Missing resource evidence is not unlimited capacity.
//! 3. Missing capability evidence is not capability support.
//! 4. An empty capability requirement set is not proof of compatibility.
//! 5. An unmet preference is not automatically a semantic failure.
//! 6. A semantic error is distinct from target infeasibility.
//! 7. An unrepresentable operation must not be silently approximated.
//! 8. Incomplete target discovery must remain visible.
//! 9. A planning result is not a resource reservation.
//! 10. Runtime must revalidate resource availability before execution.
//!
//! ## SCALABILITY
//!
//! No universal maximum is imposed on:
//!
//! - targets;
//! - capabilities;
//! - requirements;
//! - resource dimensions;
//! - computing domains;
//! - operations;
//! - dependencies;
//! - devices;
//! - processors;
//! - accelerators;
//! - qubits;
//! - distributed participants;
//! - evidence records.
//!
//! Optional budgets are caller-supplied operational policies.
//!
//! Literal infinite computation is not promised. Actual processing is
//! constrained by available memory, address space, processing capacity,
//! allocator behavior, and explicit operational budgets.
//!
//! ## DETERMINISM
//!
//! Target results follow canonical capability-report ordering.
//! Evidence records are sorted deterministically.
//! No random identifiers, global mutable state, or implicit hardware
//! discovery are used.
//!
//! ## INTEGRATION
//!
//! Input:
//!     CapabilityMatchReport
//!     ResourcePlanningReport
//!     SemanticValidity
//!     FeasibilityEvidenceSet
//!
//! Output:
//!     FeasibilityReport
//!
//! Downstream consumers:
//!     partitioning.rs
//!     placement.rs
//!     scheduling.rs
//!     routing.rs
//!     data_movement.rs
//!     hybrid planning
//!     backend realization planning
//!
//! A downstream consumer must not reinterpret an Indeterminate result
//! as a successful feasibility decision.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use crate::compiler::configuration::target::TargetId;

use crate::compiler::planning::capability_matching::{
    CapabilityMatchReport,
    TargetMatchDisposition,
};

use crate::compiler::planning::resource_planning::{
    ResourcePlanningReport,
    TargetResourceDisposition,
};

// ============================================================================
// MODULE IDENTITY
// ============================================================================

/// Stable architectural identity of this module.
pub const FEASIBILITY_MODULE_NAME: &str =
    "zamani.compiler.planning.feasibility";

/// Public API version.
pub const FEASIBILITY_API_VERSION: u16 = 1;

// ============================================================================
// STANDARD EVIDENCE DOMAINS
// ============================================================================

/// Evidence domain identifying semantic validity.
pub const DOMAIN_SEMANTIC_VALIDITY: &str = "semantic.validity";

/// Evidence domain identifying representability.
pub const DOMAIN_REPRESENTABILITY: &str = "realization.representability";

/// Evidence domain identifying dependency feasibility.
pub const DOMAIN_DEPENDENCY: &str = "planning.dependency";

/// Evidence domain identifying data-movement feasibility.
pub const DOMAIN_DATA_MOVEMENT: &str = "planning.data_movement";

/// Evidence domain identifying synchronization feasibility.
pub const DOMAIN_SYNCHRONIZATION: &str = "planning.synchronization";

/// Evidence domain identifying security-policy feasibility.
pub const DOMAIN_SECURITY: &str = "planning.security";

/// Evidence domain identifying authorization feasibility.
pub const DOMAIN_AUTHORIZATION: &str = "planning.authorization";

/// Evidence domain identifying backend compatibility.
pub const DOMAIN_BACKEND_COMPATIBILITY: &str =
    "planning.backend_compatibility";

// ============================================================================
// EVIDENCE DOMAIN
// ============================================================================

/// Open-world identity of an evidence-producing subsystem.
///
/// This is deliberately a string-based identity rather than a closed enum.
/// Future computing domains can introduce new evidence domains without
/// requiring this module to be redesigned.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct EvidenceDomain(String);

impl EvidenceDomain {
    /// Creates a validated evidence-domain identifier.
    ///
    /// No arbitrary length limit is imposed.
    pub fn new(
        value: impl Into<String>,
    ) -> Result<Self, FeasibilityError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(FeasibilityError::InvalidEvidenceDomain(
                "domain identity cannot be empty".to_owned(),
            ));
        }

        if value.trim() != value {
            return Err(FeasibilityError::InvalidEvidenceDomain(
                "domain identity cannot contain surrounding whitespace"
                    .to_owned(),
            ));
        }

        if value.chars().any(char::is_control) {
            return Err(FeasibilityError::InvalidEvidenceDomain(
                "domain identity cannot contain control characters"
                    .to_owned(),
            ));
        }

        Ok(Self(value))
    }

    /// Returns the domain identity.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Returns whether this is the semantic-validity domain.
    #[must_use]
    pub fn is_semantic_validity(&self) -> bool {
        self.0 == DOMAIN_SEMANTIC_VALIDITY
    }

    /// Returns whether this is the representability domain.
    #[must_use]
    pub fn is_representability(&self) -> bool {
        self.0 == DOMAIN_REPRESENTABILITY
    }
}

impl fmt::Display for EvidenceDomain {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// ============================================================================
// SEMANTIC VALIDITY
// ============================================================================

/// Semantic validity established by the authoritative semantic subsystem.
///
/// The compiler must obtain this status from semantic validation. It must
/// not infer semantic validity merely because parsing or capability
/// matching succeeded.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum SemanticValidity {
    /// The semantic authority has established validity.
    Valid,

    /// The semantic authority has established a semantic error.
    Invalid {
        /// Explanation supplied by the semantic authority.
        explanation: String,
    },

    /// Semantic validity has not been established.
    Unknown {
        /// Explanation for the missing or incomplete validation.
        explanation: String,
    },
}

impl SemanticValidity {
    /// Constructs a confirmed valid result.
    #[must_use]
    pub const fn valid() -> Self {
        Self::Valid
    }

    /// Constructs a confirmed semantic-invalid result.
    #[must_use]
    pub fn invalid(
        explanation: impl Into<String>,
    ) -> Self {
        Self::Invalid {
            explanation: explanation.into(),
        }
    }

    /// Constructs an unresolved semantic result.
    #[must_use]
    pub fn unknown(
        explanation: impl Into<String>,
    ) -> Self {
        Self::Unknown {
            explanation: explanation.into(),
        }
    }

    /// Returns whether semantic validity has been established.
    #[must_use]
    pub const fn is_valid(&self) -> bool {
        matches!(self, Self::Valid)
    }
}

// ============================================================================
// EVIDENCE STATUS
// ============================================================================

/// Outcome supplied by an authoritative evidence-producing subsystem.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum EvidenceStatus {
    /// The evidence establishes that the assessed condition is satisfied.
    Satisfied,

    /// The evidence establishes that the assessed condition is not satisfied.
    Unsatisfied,

    /// The available evidence is insufficient to decide.
    Unknown,

    /// The condition does not apply to this realization.
    NotApplicable,
}

// ============================================================================
// EVIDENCE SCOPE
// ============================================================================

/// Scope to which an evidence record applies.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum EvidenceScope {
    /// Applies to the computation independently of target.
    Global,

    /// Applies only to the specified target.
    Target(TargetId),
}

impl EvidenceScope {
    /// Returns the target identity if this is target-scoped evidence.
    #[must_use]
    pub fn target(&self) -> Option<&TargetId> {
        match self {
            Self::Global => None,
            Self::Target(target) => Some(target),
        }
    }

    /// Returns whether the evidence applies to a particular target.
    #[must_use]
    pub fn applies_to(&self, target: &TargetId) -> bool {
        match self {
            Self::Global => true,
            Self::Target(evidence_target) => evidence_target == target,
        }
    }
}

// ============================================================================
// FEASIBILITY EVIDENCE
// ============================================================================

/// One independently established feasibility observation.
///
/// The evidence-producing subsystem remains authoritative for the meaning
/// of its domain. This structure only carries the outcome and provenance
/// needed to compose planning decisions.
///
/// `evidence_id` may refer to an immutable report, snapshot, or source
/// record maintained by the originating subsystem.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FeasibilityEvidence {
    scope: EvidenceScope,
    domain: EvidenceDomain,
    status: EvidenceStatus,
    explanation: String,
    evidence_id: Option<String>,
}

impl FeasibilityEvidence {
    /// Creates one evidence record.
    pub fn new(
        scope: EvidenceScope,
        domain: EvidenceDomain,
        status: EvidenceStatus,
        explanation: impl Into<String>,
        evidence_id: Option<String>,
    ) -> Self {
        Self {
            scope,
            domain,
            status,
            explanation: explanation.into(),
            evidence_id,
        }
    }

    /// Creates global evidence.
    pub fn global(
        domain: EvidenceDomain,
        status: EvidenceStatus,
        explanation: impl Into<String>,
    ) -> Self {
        Self::new(
            EvidenceScope::Global,
            domain,
            status,
            explanation,
            None,
        )
    }

    /// Creates target-scoped evidence.
    pub fn for_target(
        target: TargetId,
        domain: EvidenceDomain,
        status: EvidenceStatus,
        explanation: impl Into<String>,
    ) -> Self {
        Self::new(
            EvidenceScope::Target(target),
            domain,
            status,
            explanation,
            None,
        )
    }

    /// Attaches an originating evidence identifier.
    #[must_use]
    pub fn with_evidence_id(
        mut self,
        evidence_id: impl Into<String>,
    ) -> Self {
        self.evidence_id = Some(evidence_id.into());
        self
    }

    /// Returns the evidence scope.
    #[must_use]
    pub fn scope(&self) -> &EvidenceScope {
        &self.scope
    }

    /// Returns the evidence domain.
    #[must_use]
    pub fn domain(&self) -> &EvidenceDomain {
        &self.domain
    }

    /// Returns the evidence status.
    #[must_use]
    pub const fn status(&self) -> EvidenceStatus {
        self.status
    }

    /// Returns the explanation.
    #[must_use]
    pub fn explanation(&self) -> &str {
        &self.explanation
    }

    /// Returns the originating evidence identifier.
    #[must_use]
    pub fn evidence_id(&self) -> Option<&str> {
        self.evidence_id.as_deref()
    }
}

// ============================================================================
// EVIDENCE SET
// ============================================================================

/// Collection of independently established feasibility evidence.
///
/// `complete` means that all applicable additional evidence producers
/// required by the current planning request have reported their results.
///
/// It does not mean that every conceivable future domain has been evaluated.
/// It must be set by the orchestration layer, not guessed by this module.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FeasibilityEvidenceSet {
    records: Vec<FeasibilityEvidence>,
    complete: bool,
}

impl FeasibilityEvidenceSet {
    /// Creates an evidence set.
    ///
    /// Records are sorted deterministically. Duplicate records are retained
    /// because repeated evidence may represent distinct observations.
    #[must_use]
    pub fn new(
        mut records: Vec<FeasibilityEvidence>,
        complete: bool,
    ) -> Self {
        records.sort_by(|left, right| {
            (
                &left.scope,
                &left.domain,
                left.status,
                &left.explanation,
                &left.evidence_id,
            )
                .cmp(&(
                    &right.scope,
                    &right.domain,
                    right.status,
                    &right.explanation,
                    &right.evidence_id,
                ))
        });

        Self { records, complete }
    }

    /// Creates an empty evidence set that is explicitly complete.
    ///
    /// Use this only when the orchestration layer has established that
    /// no additional evidence checks apply.
    #[must_use]
    pub fn complete_without_additional_checks() -> Self {
        Self::new(Vec::new(), true)
    }

    /// Returns all evidence records.
    #[must_use]
    pub fn records(&self) -> &[FeasibilityEvidence] {
        &self.records
    }

    /// Returns whether all applicable additional evidence was supplied.
    #[must_use]
    pub const fn is_complete(&self) -> bool {
        self.complete
    }

    /// Returns the number of evidence records.
    #[must_use]
    pub fn len(&self) -> usize {
        self.records.len()
    }

    /// Returns whether the evidence set contains no records.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    /// Returns records that apply to a target.
    pub fn for_target<'a>(
        &'a self,
        target: &'a TargetId,
    ) -> impl Iterator<Item = &'a FeasibilityEvidence> + 'a {
        self.records
            .iter()
            .filter(move |record| record.scope.applies_to(target))
    }

    /// Returns global records.
    pub fn global_records(
        &self,
    ) -> impl Iterator<Item = &FeasibilityEvidence> {
        self.records.iter().filter(|record| {
            matches!(record.scope, EvidenceScope::Global)
        })
    }
}

// ============================================================================
// OPERATIONAL BUDGET
// ============================================================================

/// Caller-configurable operational budgets for feasibility analysis.
///
/// These are implementation workload budgets, not semantic or hardware
/// limits. An absent limit means that this module imposes no additional
/// limit for that dimension.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FeasibilityBudget {
    max_targets: Option<usize>,
    max_evidence_records: Option<usize>,
}

impl FeasibilityBudget {
    /// Creates a budget with no additional limits.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self {
            max_targets: None,
            max_evidence_records: None,
        }
    }

    /// Sets an optional target-analysis budget.
    #[must_use]
    pub const fn with_max_targets(
        mut self,
        maximum: Option<usize>,
    ) -> Self {
        self.max_targets = maximum;
        self
    }

    /// Sets an optional evidence-record budget.
    #[must_use]
    pub const fn with_max_evidence_records(
        mut self,
        maximum: Option<usize>,
    ) -> Self {
        self.max_evidence_records = maximum;
        self
    }

    /// Returns the configured target-analysis budget.
    #[must_use]
    pub const fn max_targets(self) -> Option<usize> {
        self.max_targets
    }

    /// Returns the configured evidence-record budget.
    #[must_use]
    pub const fn max_evidence_records(self) -> Option<usize> {
        self.max_evidence_records
    }
}

impl Default for FeasibilityBudget {
    fn default() -> Self {
        Self::unlimited()
    }
}

// ============================================================================
// REQUEST
// ============================================================================

/// Immutable input to one feasibility-analysis invocation.
pub struct FeasibilityRequest<'a> {
    capabilities: &'a CapabilityMatchReport,
    resources: &'a ResourcePlanningReport,
    semantic_validity: SemanticValidity,
    evidence: &'a FeasibilityEvidenceSet,
    budget: FeasibilityBudget,
}

impl<'a> FeasibilityRequest<'a> {
    /// Creates a feasibility request from already evaluated evidence.
    ///
    /// Capability matching and resource planning must be performed by
    /// their respective authoritative modules before this call.
    #[must_use]
    pub fn new(
        capabilities: &'a CapabilityMatchReport,
        resources: &'a ResourcePlanningReport,
        semantic_validity: SemanticValidity,
        evidence: &'a FeasibilityEvidenceSet,
    ) -> Self {
        Self {
            capabilities,
            resources,
            semantic_validity,
            evidence,
            budget: FeasibilityBudget::unlimited(),
        }
    }

    /// Sets caller-controlled operational budgets.
    #[must_use]
    pub const fn with_budget(
        mut self,
        budget: FeasibilityBudget,
    ) -> Self {
        self.budget = budget;
        self
    }
}

// ============================================================================
// FEASIBILITY DISPOSITION
// ============================================================================

/// Overall or per-target feasibility classification.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum FeasibilityDisposition {
    /// At least one evaluated realization has proven feasibility.
    Feasible,

    /// At least one realization is feasible but has unmet non-mandatory
    /// preferences or other explicitly reported trade-offs.
    FeasibleWithTradeoffs,

    /// Required capabilities cannot be satisfied by the evaluated inventory.
    CapabilityUnsatisfiable,

    /// Required resource demands cannot be satisfied.
    ResourceUnsatisfiable,

    /// An applicable non-semantic realization constraint is unsatisfied.
    ConstraintUnsatisfiable,

    /// The authoritative semantic subsystem has rejected the program.
    SemanticInvalid,

    /// The requested computation cannot be represented by the assessed
    /// realization without changing its required meaning.
    Unrepresentable,

    /// Available evidence is incomplete, missing, contradictory, or unknown.
    Indeterminate,
}

impl FeasibilityDisposition {
    /// Returns whether the result establishes feasibility.
    #[must_use]
    pub const fn is_feasible(self) -> bool {
        matches!(
            self,
            Self::Feasible | Self::FeasibleWithTradeoffs
        )
    }

    /// Returns whether the result is definitely infeasible.
    #[must_use]
    pub const fn is_definitively_infeasible(self) -> bool {
        matches!(
            self,
            Self::CapabilityUnsatisfiable
                | Self::ResourceUnsatisfiable
                | Self::ConstraintUnsatisfiable
                | Self::SemanticInvalid
                | Self::Unrepresentable
        )
    }

    /// Returns whether further evidence is required.
    #[must_use]
    pub const fn is_indeterminate(self) -> bool {
        matches!(self, Self::Indeterminate)
    }
}

impl fmt::Display for FeasibilityDisposition {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(match self {
            Self::Feasible => "feasible",
            Self::FeasibleWithTradeoffs => "feasible with trade-offs",
            Self::CapabilityUnsatisfiable => "capability-unsatisfiable",
            Self::ResourceUnsatisfiable => "resource-unsatisfiable",
            Self::ConstraintUnsatisfiable => "constraint-unsatisfiable",
            Self::SemanticInvalid => "semantically invalid",
            Self::Unrepresentable => "unrepresentable",
            Self::Indeterminate => "indeterminate",
        })
    }
}

// ============================================================================
// FINDING
// ============================================================================

/// Stable machine-readable feasibility finding code.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum FeasibilityFindingCode {
    /// Semantic validation rejected the program.
    SemanticInvalid,

    /// The requested computation cannot be represented.
    Unrepresentable,

    /// Required capability evidence is incompatible.
    CapabilityUnsupported,

    /// Capability evidence is missing or unresolved.
    CapabilityUnknown,

    /// Required resource capacity is insufficient.
    ResourceInsufficient,

    /// Resource evidence is missing or unresolved.
    ResourceUnknown,

    /// Another realization constraint is unsatisfied.
    ConstraintUnsatisfied,

    /// An additional evidence producer has not completed its assessment.
    EvidenceIncomplete,

    /// A feasibility preference is unmet.
    Tradeoff,

    /// Target discovery is incomplete.
    InventoryIncomplete,
}

/// One structured explanation of a feasibility result.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FeasibilityFinding {
    code: FeasibilityFindingCode,
    domain: Option<EvidenceDomain>,
    target: Option<TargetId>,
    message: String,
    evidence_id: Option<String>,
}

impl FeasibilityFinding {
    fn new(
        code: FeasibilityFindingCode,
        domain: Option<EvidenceDomain>,
        target: Option<TargetId>,
        message: impl Into<String>,
        evidence_id: Option<String>,
    ) -> Self {
        Self {
            code,
            domain,
            target,
            message: message.into(),
            evidence_id,
        }
    }

    /// Returns the stable finding code.
    #[must_use]
    pub const fn code(&self) -> FeasibilityFindingCode {
        self.code
    }

    /// Returns the evidence domain, if applicable.
    #[must_use]
    pub fn domain(&self) -> Option<&EvidenceDomain> {
        self.domain.as_ref()
    }

    /// Returns the target identity, if target-scoped.
    #[must_use]
    pub fn target(&self) -> Option<&TargetId> {
        self.target.as_ref()
    }

    /// Returns the explanation.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Returns the originating evidence identifier.
    #[must_use]
    pub fn evidence_id(&self) -> Option<&str> {
        self.evidence_id.as_deref()
    }
}

// ============================================================================
// PER-TARGET RESULT
// ============================================================================

/// Feasibility assessment for one discovered target.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TargetFeasibility {
    target_id: TargetId,
    disposition: FeasibilityDisposition,
    capability_disposition: TargetMatchDisposition,
    resource_disposition: Option<TargetResourceDisposition>,
    findings: Vec<FeasibilityFinding>,
}

impl TargetFeasibility {
    /// Returns the target identity.
    #[must_use]
    pub fn target_id(&self) -> &TargetId {
        &self.target_id
    }

    /// Returns the final target feasibility classification.
    #[must_use]
    pub const fn disposition(&self) -> FeasibilityDisposition {
        self.disposition
    }

    /// Returns the original capability-matching disposition.
    #[must_use]
    pub const fn capability_disposition(&self) -> TargetMatchDisposition {
        self.capability_disposition
    }

    /// Returns the resource-planning disposition, if available.
    #[must_use]
    pub const fn resource_disposition(
        &self,
    ) -> Option<TargetResourceDisposition> {
        self.resource_disposition
    }

    /// Returns all findings for this target.
    #[must_use]
    pub fn findings(&self) -> &[FeasibilityFinding] {
        &self.findings
    }

    /// Returns whether this target is proven feasible.
    #[must_use]
    pub const fn is_feasible(&self) -> bool {
        self.disposition.is_feasible()
    }
}

// ============================================================================
// COMPLETE REPORT
// ============================================================================

/// Complete compiler-side feasibility report.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FeasibilityReport {
    disposition: FeasibilityDisposition,
    targets: Vec<TargetFeasibility>,
    findings: Vec<FeasibilityFinding>,
    discovery_complete: bool,
    evidence_complete: bool,
    semantic_validity: SemanticValidity,
    capability_requirement_count: usize,
}

impl FeasibilityReport {
    /// Returns the overall feasibility disposition.
    #[must_use]
    pub const fn disposition(&self) -> FeasibilityDisposition {
        self.disposition
    }

    /// Returns every evaluated target result.
    #[must_use]
    pub fn targets(&self) -> &[TargetFeasibility] {
        &self.targets
    }

    /// Returns all report-level findings.
    #[must_use]
    pub fn findings(&self) -> &[FeasibilityFinding] {
        &self.findings
    }

    /// Returns the result for a target.
    #[must_use]
    pub fn target(
        &self,
        id: &TargetId,
    ) -> Option<&TargetFeasibility> {
        self.targets
            .iter()
            .find(|target| target.target_id() == id)
    }

    /// Iterates over proven feasible targets.
    pub fn feasible_targets(
        &self,
    ) -> impl Iterator<Item = &TargetFeasibility> {
        self.targets
            .iter()
            .filter(|target| target.is_feasible())
    }

    /// Returns whether at least one assessed target is feasible.
    #[must_use]
    pub fn has_feasible_target(&self) -> bool {
        self.feasible_targets().next().is_some()
    }

    /// Returns whether target discovery was complete.
    #[must_use]
    pub const fn discovery_is_complete(&self) -> bool {
        self.discovery_complete
    }

    /// Returns whether all applicable additional evidence was supplied.
    #[must_use]
    pub const fn evidence_is_complete(&self) -> bool {
        self.evidence_complete
    }

    /// Returns the semantic validation result.
    #[must_use]
    pub fn semantic_validity(&self) -> &SemanticValidity {
        &self.semantic_validity
    }

    /// Returns the number of capability requirements assessed.
    #[must_use]
    pub const fn capability_requirement_count(&self) -> usize {
        self.capability_requirement_count
    }

    /// Returns whether the report is complete.
    ///
    /// A feasible target may exist even when discovery is incomplete.
    /// In that situation the report is useful but not exhaustive.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.discovery_complete
            && self.evidence_complete
            && !matches!(
                self.semantic_validity,
                SemanticValidity::Unknown { .. }
            )
            && self.targets.iter().all(|target| {
                !target.disposition().is_indeterminate()
            })
    }

    /// Returns the stable number of assessed targets.
    #[must_use]
    pub fn len(&self) -> usize {
        self.targets.len()
    }

    /// Returns whether no targets were assessed.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.targets.is_empty()
    }
}

// ============================================================================
// ERRORS
// ============================================================================

/// Errors produced by feasibility analysis.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum FeasibilityError {
    /// An evidence-domain identity is invalid.
    InvalidEvidenceDomain(String),

    /// A caller-configured operational budget was exceeded.
    BudgetExceeded {
        /// Budget dimension.
        dimension: &'static str,

        /// Actual number of items.
        actual: usize,

        /// Configured maximum.
        maximum: usize,
    },
}

impl fmt::Display for FeasibilityError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidEvidenceDomain(message) => {
                write!(formatter, "invalid feasibility evidence domain: {message}")
            }

            Self::BudgetExceeded {
                dimension,
                actual,
                maximum,
            } => {
                write!(
                    formatter,
                    "feasibility {dimension} budget exceeded: \
                     {actual} items supplied, maximum {maximum}"
                )
            }
        }
    }
}

impl Error for FeasibilityError {}

// ============================================================================
// ANALYZER
// ============================================================================

/// Stateless feasibility-analysis service.
#[derive(Clone, Copy, Debug, Default)]
pub struct FeasibilityAnalyzer;

impl FeasibilityAnalyzer {
    /// Creates a feasibility analyzer.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Evaluates the supplied evidence.
    ///
    /// This method does not invoke capability matching or resource planning.
    /// It consumes their completed reports.
    ///
    /// # Errors
    ///
    /// Returns an error when a caller-supplied operational budget is exceeded.
    pub fn evaluate(
        &self,
        request: FeasibilityRequest<'_>,
    ) -> Result<FeasibilityReport, FeasibilityError> {
        check_budget(
            "targets",
            request.capabilities.len(),
            request.budget.max_targets,
        )?;

        check_budget(
            "evidence records",
            request.evidence.len(),
            request.budget.max_evidence_records,
        )?;

        let discovery_complete =
            request.capabilities.discovery_is_complete()
                && request.resources.discovery_is_complete();

        let evidence_complete = request.evidence.is_complete();

        let mut targets = Vec::new();
        let mut findings = Vec::new();

        // Global semantic invalidity cannot be repaired by choosing
        // a different target.
        if let SemanticValidity::Invalid { explanation } =
            &request.semantic_validity
        {
            findings.push(FeasibilityFinding::new(
                FeasibilityFindingCode::SemanticInvalid,
                Some(EvidenceDomain::new(DOMAIN_SEMANTIC_VALIDITY)?),
                None,
                explanation.clone(),
                None,
            ));
        }

        if let SemanticValidity::Unknown { explanation } =
            &request.semantic_validity
        {
            findings.push(FeasibilityFinding::new(
                FeasibilityFindingCode::EvidenceIncomplete,
                Some(EvidenceDomain::new(DOMAIN_SEMANTIC_VALIDITY)?),
                None,
                explanation.clone(),
                None,
            ));
        }

        // Global evidence applies to every candidate.
        for evidence in request.evidence.global_records() {
            if let Some(finding) = finding_from_evidence(
                evidence,
                None,
            ) {
                findings.push(finding);
            }
        }

        // Evaluate each discovered target independently.
        for candidate in request.capabilities.candidates() {
            let target_id = candidate.target_id().clone();

            let resource = request.resources.target(&target_id);

            let mut target_findings = Vec::new();

            for evidence in request.evidence.for_target(&target_id) {
                if let Some(finding) = finding_from_evidence(
                    evidence,
                    Some(target_id.clone()),
                ) {
                    target_findings.push(finding);
                }
            }

            if !evidence_complete {
                target_findings.push(FeasibilityFinding::new(
                    FeasibilityFindingCode::EvidenceIncomplete,
                    None,
                    Some(target_id.clone()),
                    "one or more applicable feasibility evidence producers \
                     have not completed their assessment",
                    None,
                ));
            }

            if !discovery_complete {
                target_findings.push(FeasibilityFinding::new(
                    FeasibilityFindingCode::InventoryIncomplete,
                    None,
                    Some(target_id.clone()),
                    "the target inventory is incomplete",
                    None,
                ));
            }

            let resource_disposition =
                resource.map(|plan| plan.disposition());

            if resource.is_none()
                && candidate.is_eligible_for_planning()
            {
                target_findings.push(FeasibilityFinding::new(
                    FeasibilityFindingCode::ResourceUnknown,
                    None,
                    Some(target_id.clone()),
                    "no resource-planning result exists for this target",
                    None,
                ));
            }

            let disposition = classify_target(
                &request.semantic_validity,
                candidate.disposition(),
                resource_disposition,
                &target_findings,
                evidence_complete,
            );

            findings.extend(target_findings.iter().cloned());

            targets.push(TargetFeasibility {
                target_id,
                disposition,
                capability_disposition: candidate.disposition(),
                resource_disposition,
                findings: target_findings,
            });
        }

        let disposition = classify_overall(
            &request.semantic_validity,
            request.capabilities,
            request.resources,
            &targets,
            request.evidence,
        );

        if !discovery_complete {
            findings.push(FeasibilityFinding::new(
                FeasibilityFindingCode::InventoryIncomplete,
                None,
                None,
                "target discovery or resource-planning inventory is incomplete",
                None,
            ));
        }

        Ok(FeasibilityReport {
            disposition,
            targets,
            findings,
            discovery_complete,
            evidence_complete,
            semantic_validity: request.semantic_validity,
            capability_requirement_count:
                request.capabilities.requirement_count(),
        })
    }

    /// Returns the stable module identity.
    #[must_use]
    pub const fn module_name() -> &'static str {
        FEASIBILITY_MODULE_NAME
    }

    /// Returns the public API version.
    #[must_use]
    pub const fn api_version() -> u16 {
        FEASIBILITY_API_VERSION
    }
}

// ============================================================================
// INTERNAL BUDGET VALIDATION
// ============================================================================

fn check_budget(
    dimension: &'static str,
    actual: usize,
    maximum: Option<usize>,
) -> Result<(), FeasibilityError> {
    if let Some(maximum) = maximum {
        if actual > maximum {
            return Err(FeasibilityError::BudgetExceeded {
                dimension,
                actual,
                maximum,
            });
        }
    }

    Ok(())
}

// ============================================================================
// EVIDENCE CLASSIFICATION
// ============================================================================

fn finding_from_evidence(
    evidence: &FeasibilityEvidence,
    target: Option<TargetId>,
) -> Option<FeasibilityFinding> {
    let code = match evidence.status {
        EvidenceStatus::Satisfied | EvidenceStatus::NotApplicable => {
            return None;
        }

        EvidenceStatus::Unknown => {
            FeasibilityFindingCode::EvidenceIncomplete
        }

        EvidenceStatus::Unsatisfied
            if evidence.domain.is_semantic_validity() =>
        {
            FeasibilityFindingCode::SemanticInvalid
        }

        EvidenceStatus::Unsatisfied
            if evidence.domain.is_representability() =>
        {
            FeasibilityFindingCode::Unrepresentable
        }

        EvidenceStatus::Unsatisfied => {
            FeasibilityFindingCode::ConstraintUnsatisfied
        }
    };

    Some(FeasibilityFinding::new(
        code,
        Some(evidence.domain.clone()),
        target.or_else(|| evidence.scope.target().cloned()),
        evidence.explanation.clone(),
        evidence.evidence_id.clone(),
    ))
}

// ============================================================================
// TARGET CLASSIFICATION
// ============================================================================

fn classify_target(
    semantic: &SemanticValidity,
    capability: TargetMatchDisposition,
    resource: Option<TargetResourceDisposition>,
    findings: &[FeasibilityFinding],
    evidence_complete: bool,
) -> FeasibilityDisposition {
    if matches!(semantic, SemanticValidity::Invalid { .. })
        || findings.iter().any(|finding| {
            finding.code == FeasibilityFindingCode::SemanticInvalid
        })
    {
        return FeasibilityDisposition::SemanticInvalid;
    }

    if findings.iter().any(|finding| {
        finding.code == FeasibilityFindingCode::Unrepresentable
    }) {
        return FeasibilityDisposition::Unrepresentable;
    }

    if findings.iter().any(|finding| {
        finding.code == FeasibilityFindingCode::ConstraintUnsatisfied
    }) {
        return FeasibilityDisposition::ConstraintUnsatisfiable;
    }

    if !matches!(semantic, SemanticValidity::Valid) {
        return FeasibilityDisposition::Indeterminate;
    }

    if !evidence_complete
        || findings.iter().any(|finding| {
            finding.code == FeasibilityFindingCode::EvidenceIncomplete
        })
    {
        return FeasibilityDisposition::Indeterminate;
    }

    match capability {
        TargetMatchDisposition::Incompatible
        | TargetMatchDisposition::NotExecutionTarget => {
            return FeasibilityDisposition::CapabilityUnsatisfiable;
        }

        TargetMatchDisposition::Unresolved
        | TargetMatchDisposition::NoRequirements => {
            return FeasibilityDisposition::Indeterminate;
        }

        TargetMatchDisposition::Satisfied => {}
    }

    match resource {
        Some(TargetResourceDisposition::Feasible) 
        | Some(TargetResourceDisposition::NoResourceRequirements) => {
            FeasibilityDisposition::Feasible
        }

        Some(TargetResourceDisposition::FeasibleWithUnmetPreferences) => {
            FeasibilityDisposition::FeasibleWithTradeoffs
        }

        Some(TargetResourceDisposition::Infeasible) => {
            FeasibilityDisposition::ResourceUnsatisfiable
        }

        Some(
            TargetResourceDisposition::Unresolved
            | TargetResourceDisposition::EvidenceUnavailable
            | TargetResourceDisposition::EvidenceFailure
            | TargetResourceDisposition::InvalidEvidence
            | TargetResourceDisposition::SkippedCapabilityIneligible,
        )
        | None => FeasibilityDisposition::Indeterminate,
    }
}

// ============================================================================
// OVERALL CLASSIFICATION
// ============================================================================

fn classify_overall(
    semantic: &SemanticValidity,
    capabilities: &CapabilityMatchReport,
    resources: &ResourcePlanningReport,
    targets: &[TargetFeasibility],
    evidence: &FeasibilityEvidenceSet,
) -> FeasibilityDisposition {
    if matches!(semantic, SemanticValidity::Invalid { .. })
        || evidence.global_records().any(|record| {
            record.domain.is_semantic_validity()
                && record.status == EvidenceStatus::Unsatisfied
        })
    {
        return FeasibilityDisposition::SemanticInvalid;
    }

    if !matches!(semantic, SemanticValidity::Valid)
        || !evidence.is_complete()
    {
        return FeasibilityDisposition::Indeterminate;
    }

    // A global representability or constraint failure applies to all targets.
    if evidence.global_records().any(|record| {
        record.domain.is_representability()
            && record.status == EvidenceStatus::Unsatisfied
    }) {
        return FeasibilityDisposition::Unrepresentable;
    }

    if evidence.global_records().any(|record| {
        record.status == EvidenceStatus::Unsatisfied
            && !record.domain.is_semantic_validity()
            && !record.domain.is_representability()
    }) {
        return FeasibilityDisposition::ConstraintUnsatisfiable;
    }

    // An explicit successful target is sufficient to establish that at
    // least one assessed realization is feasible.
    if targets.iter().any(|target| {
        target.disposition == FeasibilityDisposition::Feasible
    }) {
        return FeasibilityDisposition::Feasible;
    }

    if targets.iter().any(|target| {
        target.disposition
            == FeasibilityDisposition::FeasibleWithTradeoffs
    }) {
        return FeasibilityDisposition::FeasibleWithTradeoffs;
    }

    let capability_candidates = capabilities
        .candidates()
        .iter()
        .filter(|candidate| candidate.is_eligible_for_planning())
        .count();

    // No requirements is not proof of compatibility.
    if capabilities.requirement_count() == 0 {
        return FeasibilityDisposition::Indeterminate;
    }

    if capability_candidates == 0
        && capabilities.discovery_is_complete()
    {
        return FeasibilityDisposition::CapabilityUnsatisfiable;
    }

    // Resource unsatisfiability requires all capability-eligible targets
    // to have proven resource shortfalls and a complete resource inventory.
    let resource_failures = targets
        .iter()
        .filter(|target| {
            target.capability_disposition.is_eligible_for_planning()
                && target.disposition
                    == FeasibilityDisposition::ResourceUnsatisfiable
        })
        .count();

    if capability_candidates > 0
        && resource_failures == capability_candidates
        && resources.discovery_is_complete()
    {
        return FeasibilityDisposition::ResourceUnsatisfiable;
    }

    let unrepresentable = targets
        .iter()
        .filter(|target| {
            target.capability_disposition.is_eligible_for_planning()
                && target.disposition == FeasibilityDisposition::Unrepresentable
        })
        .count();

    if capability_candidates > 0
        && unrepresentable == capability_candidates
    {
        return FeasibilityDisposition::Unrepresentable;
    }

    let constraint_failures = targets
        .iter()
        .filter(|target| {
            target.capability_disposition.is_eligible_for_planning()
                && target.disposition
                    == FeasibilityDisposition::ConstraintUnsatisfiable
        })
        .count();

    if capability_candidates > 0
        && constraint_failures == capability_candidates
    {
        return FeasibilityDisposition::ConstraintUnsatisfiable;
    }

    // Incomplete inventories or mixed unresolved outcomes must not be
    // upgraded into a global infeasibility or success claim.
    FeasibilityDisposition::Indeterminate
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn module_identity_is_stable() {
        assert_eq!(
            FeasibilityAnalyzer::module_name(),
            FEASIBILITY_MODULE_NAME,
        );

        assert_eq!(
            FeasibilityAnalyzer::api_version(),
            FEASIBILITY_API_VERSION,
        );
    }

    #[test]
    fn evidence_domains_are_open_ended() {
        let domain = EvidenceDomain::new(
            "future.photonic.execution",
        )
        .expect("valid open-world domain");

        assert_eq!(
            domain.as_str(),
            "future.photonic.execution",
        );
    }

    #[test]
    fn empty_evidence_domain_is_rejected() {
        assert!(
            EvidenceDomain::new("").is_err()
        );

        assert!(
            EvidenceDomain::new("   ").is_err()
        );
    }

    #[test]
    fn evidence_domain_rejects_control_characters() {
        assert!(
            EvidenceDomain::new("planning\ninvalid").is_err()
        );
    }

    #[test]
    fn unknown_evidence_is_not_success() {
        let domain = EvidenceDomain::new(
            DOMAIN_DEPENDENCY,
        )
        .expect("valid domain");

        let evidence = FeasibilityEvidence::global(
            domain,
            EvidenceStatus::Unknown,
            "dependency validation has not completed",
        );

        let finding = finding_from_evidence(
            &evidence,
            None,
        )
        .expect("unknown evidence must create a finding");

        assert_eq!(
            finding.code(),
            FeasibilityFindingCode::EvidenceIncomplete,
        );
    }

    #[test]
    fn semantic_invalidity_is_distinct_from_resource_failure() {
        let semantic = SemanticValidity::invalid(
            "invalid semantic operation",
        );

        let disposition = classify_target(
            &semantic,
            TargetMatchDisposition::Satisfied,
            Some(TargetResourceDisposition::Feasible),
            &[],
            true,
        );

        assert_eq!(
            disposition,
            FeasibilityDisposition::SemanticInvalid,
        );
    }

    #[test]
    fn unknown_semantics_are_indeterminate() {
        let semantic = SemanticValidity::unknown(
            "semantic validation is incomplete",
        );

        let disposition = classify_target(
            &semantic,
            TargetMatchDisposition::Satisfied,
            Some(TargetResourceDisposition::Feasible),
            &[],
            true,
        );

        assert_eq!(
            disposition,
            FeasibilityDisposition::Indeterminate,
        );
    }

    #[test]
    fn unmet_preferences_allow_tradeoffs() {
        let disposition = classify_target(
            &SemanticValidity::Valid,
            TargetMatchDisposition::Satisfied,
            Some(
                TargetResourceDisposition::FeasibleWithUnmetPreferences,
            ),
            &[],
            true,
        );

        assert_eq!(
            disposition,
            FeasibilityDisposition::FeasibleWithTradeoffs,
        );
    }

    #[test]
    fn insufficient_resources_are_not_indeterminate() {
        let disposition = classify_target(
            &SemanticValidity::Valid,
            TargetMatchDisposition::Satisfied,
            Some(TargetResourceDisposition::Infeasible),
            &[],
            true,
        );

        assert_eq!(
            disposition,
            FeasibilityDisposition::ResourceUnsatisfiable,
        );
    }

    #[test]
    fn absent_resource_evidence_is_indeterminate() {
        let disposition = classify_target(
            &SemanticValidity::Valid,
            TargetMatchDisposition::Satisfied,
            None,
            &[],
            true,
        );

        assert_eq!(
            disposition,
            FeasibilityDisposition::Indeterminate,
        );
    }

    #[test]
    fn empty_capability_requirements_are_not_compatible() {
        assert_eq!(
            classify_target(
                &SemanticValidity::Valid,
                TargetMatchDisposition::NoRequirements,
                Some(TargetResourceDisposition::NoResourceRequirements),
                &[],
                true,
            ),
            FeasibilityDisposition::Indeterminate,
        );
    }

    #[test]
    fn explicit_capability_incompatibility_is_preserved() {
        assert_eq!(
            classify_target(
                &SemanticValidity::Valid,
                TargetMatchDisposition::Incompatible,
                None,
                &[],
                true,
            ),
            FeasibilityDisposition::CapabilityUnsatisfiable,
        );
    }

    #[test]
    fn unknown_resource_evidence_is_indeterminate() {
        assert_eq!(
            classify_target(
                &SemanticValidity::Valid,
                TargetMatchDisposition::Satisfied,
                Some(TargetResourceDisposition::EvidenceUnavailable),
                &[],
                true,
            ),
            FeasibilityDisposition::Indeterminate,
        );
    }

    #[test]
    fn unrepresentable_evidence_is_not_a_successful_fallback() {
        let evidence = FeasibilityEvidence::global(
            EvidenceDomain::new(DOMAIN_REPRESENTABILITY)
                .expect("valid domain"),
            EvidenceStatus::Unsatisfied,
            "required operation cannot be represented",
        );

        let finding = finding_from_evidence(
            &evidence,
            None,
        )
        .expect("unrepresentability must be reported");

        assert_eq!(
            finding.code(),
            FeasibilityFindingCode::Unrepresentable,
        );
    }

    #[test]
    fn evidence_order_is_deterministic() {
        let first = FeasibilityEvidence::global(
            EvidenceDomain::new("z.domain").unwrap(),
            EvidenceStatus::Satisfied,
            "z",
        );

        let second = FeasibilityEvidence::global(
            EvidenceDomain::new("a.domain").unwrap(),
            EvidenceStatus::Unknown,
            "a",
        );

        let left = FeasibilityEvidenceSet::new(
            vec![first.clone(), second.clone()],
            true,
        );

        let right = FeasibilityEvidenceSet::new(
            vec![second, first],
            true,
        );

        assert_eq!(left, right);
    }

    #[test]
    fn unlimited_budget_has_no_target_ceiling() {
        let budget = FeasibilityBudget::unlimited();

        assert_eq!(budget.max_targets(), None);
        assert_eq!(budget.max_evidence_records(), None);
    }

    #[test]
    fn operational_budgets_are_caller_configurable() {
        let budget = FeasibilityBudget::unlimited()
            .with_max_targets(Some(12))
            .with_max_evidence_records(Some(400));

        assert_eq!(budget.max_targets(), Some(12));
        assert_eq!(budget.max_evidence_records(), Some(400));
    }

    #[test]
    fn zero_budget_is_an_explicit_caller_policy() {
        let budget = FeasibilityBudget::unlimited()
            .with_max_targets(Some(0));

        assert_eq!(budget.max_targets(), Some(0));
    }

    #[test]
    fn not_applicable_evidence_does_not_create_a_failure() {
        let evidence = FeasibilityEvidence::global(
            EvidenceDomain::new(DOMAIN_SYNCHRONIZATION)
                .unwrap(),
            EvidenceStatus::NotApplicable,
            "no synchronization boundary exists",
        );

        assert!(
            finding_from_evidence(&evidence, None).is_none()
        );
    }
}
