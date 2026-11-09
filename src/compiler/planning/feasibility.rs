
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Compiler — Feasibility Planning
//!
//! Path: src/compiler/planning/feasibility.rs
//! Edition: 2021
//! Minimum Rust version: 1.97
//!
//! Determines whether a proposed computation realization is supported by
//! authoritative semantic, capability, resource, and realization evidence.
//!
//! This module composes evidence. It does not redefine the semantic model,
//! capability model, resource model, target model, or canonical IRs.
//!
//! It does not allocate or reserve physical resources, place operations,
//! schedule physical devices, route computation, lower IR, invoke backends,
//! or execute programs.
//!
//! # Canonical authorities
//!
//! - Semantic validity: crate::frontend::semantic
//! - Target identity: crate::compiler::configuration::target
//! - Capability matching: crate::compiler::planning::capability_matching
//! - Resource planning: crate::compiler::planning::resource_planning
//! - Canonical computation representations: crate::zuir and domain IRs
//! - Physical resource acquisition and execution: crate::runtime
//!
//! # Important invariants
//!
//! 1. Unknown evidence never proves feasibility.
//! 2. Invalid evidence never proves feasibility.
//! 3. Missing resource evidence never means unlimited capacity.
//! 4. Empty capability requirements never prove compatibility.
//! 5. Preferences and hints are not promoted to mandatory requirements.
//! 6. Global evidence applies to every applicable target.
//! 7. Target-scoped evidence is never transferred to another target.
//! 8. Proven incompatibility is not downgraded by unrelated missing evidence.
//! 9. A non-execution target is not classified as an incompatible execution
//!    target.
//! 10. A planning result is not a reservation or execution guarantee.
//! 11. Runtime must revalidate availability before physical execution.
//!
//! # Scalability
//!
//! No universal maximum is imposed on targets, capabilities, requirements,
//! resource dimensions, domains, evidence records, processors, accelerators,
//! qubits, devices, or distributed participants.
//!
//! Optional operational budgets are caller-supplied. They are not semantic
//! or hardware limits.
//!
//! Literal infinite computation is not promised. Actual processing is
//! bounded by available memory, address space, processing capacity, and
//! explicitly configured operational budgets.
//!
//! # Determinism
//!
//! Reports preserve canonical target ordering and sort findings by stable
//! fields. No random identifiers, global mutable state, or implicit hardware
//! discovery are used.

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

/// Stable architectural identity.
pub const FEASIBILITY_MODULE_NAME: &str =
    "zamani.compiler.planning.feasibility";

/// Public API version.
///
/// Version 2 adds explicit invalid-evidence and not-applicable outcomes.
pub const FEASIBILITY_API_VERSION: u16 = 2;

// ============================================================================
// EVIDENCE DOMAINS
// ============================================================================

/// Semantic validity evidence domain.
pub const DOMAIN_SEMANTIC_VALIDITY: &str =
    "semantic.validity";

/// Representability evidence domain.
pub const DOMAIN_REPRESENTABILITY: &str =
    "realization.representability";

/// Dependency feasibility evidence domain.
pub const DOMAIN_DEPENDENCY: &str =
    "planning.dependency";

/// Data-movement feasibility evidence domain.
pub const DOMAIN_DATA_MOVEMENT: &str =
    "planning.data_movement";

/// Synchronization feasibility evidence domain.
pub const DOMAIN_SYNCHRONIZATION: &str =
    "planning.synchronization";

/// Security feasibility evidence domain.
pub const DOMAIN_SECURITY: &str =
    "planning.security";

/// Authorization feasibility evidence domain.
pub const DOMAIN_AUTHORIZATION: &str =
    "planning.authorization";

/// Backend compatibility evidence domain.
pub const DOMAIN_BACKEND_COMPATIBILITY: &str =
    "planning.backend_compatibility";

// ============================================================================
// EVIDENCE DOMAIN
// ============================================================================

/// Open-world identity of an evidence-producing subsystem.
///
/// Future domains can introduce new evidence identities without modifying
/// this module.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct EvidenceDomain(String);

impl EvidenceDomain {
    /// Creates a validated evidence-domain identity.
    ///
    /// No arbitrary identifier-length limit is imposed.
    pub fn new(value: impl Into<String>) -> Result<Self, FeasibilityError> {
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

/// Semantic validity supplied by the authoritative frontend semantic layer.
///
/// This type carries the result. It does not implement semantic analysis.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum SemanticValidity {
    /// Semantic validation has established validity.
    Valid,

    /// Semantic validation has rejected the program.
    Invalid {
        /// Explanation supplied by the semantic authority.
        explanation: String,
    },

    /// Semantic validation has not established validity.
    Unknown {
        /// Explanation for incomplete validation.
        explanation: String,
    },
}

impl SemanticValidity {
    /// Creates a confirmed valid result.
    #[must_use]
    pub const fn valid() -> Self {
        Self::Valid
    }

    /// Creates a semantic-invalid result.
    #[must_use]
    pub fn invalid(
        explanation: impl Into<String>,
    ) -> Self {
        Self::Invalid {
            explanation: explanation.into(),
        }
    }

    /// Creates an unresolved semantic result.
    #[must_use]
    pub fn unknown(
        explanation: impl Into<String>,
    ) -> Self {
        Self::Unknown {
            explanation: explanation.into(),
        }
    }

    /// Returns whether validity has been established.
    #[must_use]
    pub const fn is_valid(&self) -> bool {
        matches!(self, Self::Valid)
    }

    /// Returns whether semantic validation rejected the program.
    #[must_use]
    pub const fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid { .. })
    }

    /// Returns whether semantic validity remains unknown.
    #[must_use]
    pub const fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown { .. })
    }
}

// ============================================================================
// EVIDENCE STATUS
// ============================================================================

/// Outcome supplied by an authoritative evidence producer.
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
pub enum EvidenceStatus {
    /// Evidence establishes satisfaction.
    Satisfied,

    /// Evidence establishes non-satisfaction.
    Unsatisfied,

    /// Evidence is missing, incomplete, or unresolved.
    Unknown,

    /// The assessed condition does not apply.
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

    /// Applies only to one target.
    Target(TargetId),
}

impl EvidenceScope {
    /// Returns the target identity for target-scoped evidence.
    #[must_use]
    pub fn target(&self) -> Option<&TargetId> {
        match self {
            Self::Global => None,
            Self::Target(target) => Some(target),
        }
    }

    /// Returns whether the evidence applies to the specified target.
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
/// The originating subsystem remains authoritative for the meaning and
/// authenticity of its evidence.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FeasibilityEvidence {
    scope: EvidenceScope,
    domain: EvidenceDomain,
    status: EvidenceStatus,
    explanation: String,
    evidence_id: Option<String>,
}

impl FeasibilityEvidence {
    /// Creates an evidence record.
    #[must_use]
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
    #[must_use]
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
    #[must_use]
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

    /// Attaches the originating evidence identifier.
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

    /// Returns the evidence explanation.
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
/// `complete` must be supplied by orchestration. It means that all
/// applicable evidence producers required by the current request have
/// completed their assessments.
///
/// It does not mean that every possible future computing domain has been
/// evaluated.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FeasibilityEvidenceSet {
    records: Vec<FeasibilityEvidence>,
    complete: bool,
}

impl FeasibilityEvidenceSet {
    /// Creates an evidence set with deterministic record ordering.
    #[must_use]
    pub fn new(
        mut records: Vec<FeasibilityEvidence>,
        complete: bool,
    ) -> Self {
        records.sort_unstable_by(|left, right| {
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

    /// Creates an explicitly complete empty evidence set.
    ///
    /// Use only when orchestration has established that no additional
    /// evidence checks apply.
    #[must_use]
    pub fn complete_without_additional_checks() -> Self {
        Self::new(Vec::new(), true)
    }

    /// Returns all records.
    #[must_use]
    pub fn records(&self) -> &[FeasibilityEvidence] {
        &self.records
    }

    /// Returns whether all required evidence producers have completed.
    #[must_use]
    pub const fn is_complete(&self) -> bool {
        self.complete
    }

    /// Returns the record count.
    #[must_use]
    pub fn len(&self) -> usize {
        self.records.len()
    }

    /// Returns whether the set is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    /// Iterates over records applicable to a target.
    pub fn for_target<'a>(
        &'a self,
        target: &'a TargetId,
    ) -> impl Iterator<Item = &'a FeasibilityEvidence> + 'a {
        self.records
            .iter()
            .filter(move |record| record.scope.applies_to(target))
    }

    /// Iterates over global records.
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

/// Caller-configurable operational budgets for this analysis.
///
/// These limits control the amount of compiler work requested by the caller.
/// They do not represent universal hardware limits or semantic restrictions.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FeasibilityBudget {
    max_targets: Option<usize>,
    max_evidence_records: Option<usize>,
}

impl FeasibilityBudget {
    /// Creates a budget without additional limits.
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
    /// Creates a request from completed upstream reports.
    ///
    /// Capability matching and resource planning must be performed by their
    /// respective authoritative modules before this request is evaluated.
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

    /// Applies caller-controlled operational budgets.
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
pub enum FeasibilityDisposition {
    /// A realization is proven feasible.
    Feasible,

    /// A realization is feasible with unmet non-mandatory preferences.
    FeasibleWithTradeoffs,

    /// Required capabilities are proven unsatisfied.
    CapabilityUnsatisfiable,

    /// Required resource demands are proven unsatisfied.
    ResourceUnsatisfiable,

    /// An applicable realization constraint is proven unsatisfied.
    ConstraintUnsatisfiable,

    /// The authoritative semantic subsystem rejected the program.
    SemanticInvalid,

    /// The computation cannot be represented without changing its meaning.
    Unrepresentable,

    /// The target is not a computation-realization target.
    NotApplicable,

    /// No applicable realization target was discovered.
    NoRealizationTarget,

    /// The evidence is invalid and cannot establish a conclusion.
    InvalidEvidence,

    /// The available evidence is insufficient to decide.
    Indeterminate,
}

impl FeasibilityDisposition {
    /// Returns whether feasibility is established.
    #[must_use]
    pub const fn is_feasible(self) -> bool {
        matches!(
            self,
            Self::Feasible | Self::FeasibleWithTradeoffs
        )
    }

    /// Returns whether the outcome is definitively infeasible.
    #[must_use]
    pub const fn is_definitively_infeasible(self) -> bool {
        matches!(
            self,
            Self::CapabilityUnsatisfiable
                | Self::ResourceUnsatisfiable
                | Self::ConstraintUnsatisfiable
                | Self::SemanticInvalid
                | Self::Unrepresentable
                | Self::NoRealizationTarget
        )
    }

    /// Returns whether the assessment is unresolved.
    #[must_use]
    pub const fn is_indeterminate(self) -> bool {
        matches!(self, Self::Indeterminate)
    }

    /// Returns whether the evidence itself is invalid.
    #[must_use]
    pub const fn is_invalid(self) -> bool {
        matches!(self, Self::InvalidEvidence)
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
            Self::NotApplicable => "not applicable",
            Self::NoRealizationTarget => "no realization target",
            Self::InvalidEvidence => "invalid evidence",
            Self::Indeterminate => "indeterminate",
        })
    }
}

// ============================================================================
// FINDINGS
// ============================================================================

/// Stable machine-readable feasibility finding code.
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

    /// Resource evidence is invalid.
    ResourceInvalid,

    /// Another realization constraint is unsatisfied.
    ConstraintUnsatisfied,

    /// An evidence producer has not completed its assessment.
    EvidenceIncomplete,

    /// A non-mandatory preference is unmet.
    Tradeoff,

    /// Target discovery is incomplete.
    InventoryIncomplete,

    /// A discovered target is not an execution target.
    TargetNotApplicable,

    /// No applicable realization target was discovered.
    NoRealizationTarget,
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

    /// Returns the evidence domain, if available.
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

    /// Returns all target-specific and applicable global findings.
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

    /// Returns all evaluated target results.
    #[must_use]
    pub fn targets(&self) -> &[TargetFeasibility] {
        &self.targets
    }

    /// Returns all report findings.
    #[must_use]
    pub fn findings(&self) -> &[FeasibilityFinding] {
        &self.findings
    }

    /// Finds the result for a target.
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

    /// Returns whether at least one target is proven feasible.
    #[must_use]
    pub fn has_feasible_target(&self) -> bool {
        self.feasible_targets().next().is_some()
    }

    /// Returns whether target and resource discovery completed.
    #[must_use]
    pub const fn discovery_is_complete(&self) -> bool {
        self.discovery_complete
    }

    /// Returns whether all applicable additional evidence producers completed.
    #[must_use]
    pub const fn evidence_is_complete(&self) -> bool {
        self.evidence_complete
    }

    /// Returns the authoritative semantic validation result.
    #[must_use]
    pub fn semantic_validity(&self) -> &SemanticValidity {
        &self.semantic_validity
    }

    /// Returns the number of capability requirements assessed.
    #[must_use]
    pub const fn capability_requirement_count(&self) -> usize {
        self.capability_requirement_count
    }

    /// Returns whether the complete assessment is exhaustive and decisive.
    ///
    /// A proven feasible target can exist while discovery is incomplete.
    /// In that case feasibility is established existentially, but the
    /// inventory report is not exhaustive.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.discovery_complete
            && self.evidence_complete
            && !self.semantic_validity.is_unknown()
            && self.targets.iter().all(|target| {
                !target.disposition().is_indeterminate()
                    && !target.disposition().is_invalid()
            })
    }

    /// Returns the number of evaluated targets.
    #[must_use]
    pub fn len(&self) -> usize {
        self.targets.len()
    }

    /// Returns whether no targets were evaluated.
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

        /// Actual number of supplied items.
        actual: usize,

        /// Configured maximum.
        maximum: usize,
    },

    /// The host could not reserve memory for the requested report.
    AllocationFailure,
}

impl fmt::Display for FeasibilityError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidEvidenceDomain(message) => {
                write!(
                    formatter,
                    "invalid feasibility evidence domain: {message}"
                )
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

            Self::AllocationFailure => {
                formatter.write_str(
                    "unable to reserve memory for feasibility analysis",
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
    /// Creates an analyzer.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Evaluates the supplied canonical reports and additional evidence.
    ///
    /// This method does not invoke capability matching or resource planning.
    /// It consumes their completed reports.
    ///
    /// # Errors
    ///
    /// Returns an error if an operational budget is exceeded or the host
    /// cannot reserve memory for the report.
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
        targets
            .try_reserve(request.capabilities.len())
            .map_err(|_| FeasibilityError::AllocationFailure)?;

        let mut findings = Vec::new();

        let estimated_findings = request
            .evidence
            .len()
            .saturating_add(request.capabilities.len())
            .saturating_add(2);

        findings
            .try_reserve(estimated_findings)
            .map_err(|_| FeasibilityError::AllocationFailure)?;

        // Semantic validity is global: changing targets cannot repair
        // a semantically invalid source program.
        match &request.semantic_validity {
            SemanticValidity::Valid => {}

            SemanticValidity::Invalid { explanation } => {
                findings.push(FeasibilityFinding::new(
                    FeasibilityFindingCode::SemanticInvalid,
                    Some(EvidenceDomain::new(
                        DOMAIN_SEMANTIC_VALIDITY,
                    )?),
                    None,
                    explanation.clone(),
                    None,
                ));
            }

            SemanticValidity::Unknown { explanation } => {
                findings.push(FeasibilityFinding::new(
                    FeasibilityFindingCode::EvidenceIncomplete,
                    Some(EvidenceDomain::new(
                        DOMAIN_SEMANTIC_VALIDITY,
                    )?),
                    None,
                    explanation.clone(),
                    None,
                ));
            }
        }

        // Preserve global findings even when no candidate target exists.
        if request.capabilities.candidates().is_empty() {
            for evidence in request.evidence.global_records() {
                if let Some(finding) =
                    finding_from_evidence(evidence, None)
                {
                    findings.push(finding);
                }
            }
        }

        // Evaluate every discovered candidate independently.
        for candidate in request.capabilities.candidates() {
            let target_id = candidate.target_id().clone();

            let resource = request.resources.target(&target_id);

            let resource_disposition =
                resource.map(|plan| plan.disposition());

            let mut target_findings = Vec::new();

            let applicable_evidence_count =
                request.evidence.for_target(&target_id).count();

            target_findings
                .try_reserve(
                    applicable_evidence_count.saturating_add(3),
                )
                .map_err(|_| FeasibilityError::AllocationFailure)?;

            // Global evidence applies to every candidate.
            // Target-scoped evidence applies only to its own target.
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
                    "one or more applicable evidence producers \
                     have not completed their assessment",
                    None,
                ));
            }

            if !discovery_complete {
                target_findings.push(FeasibilityFinding::new(
                    FeasibilityFindingCode::InventoryIncomplete,
                    None,
                    Some(target_id.clone()),
                    "target or resource discovery is incomplete",
                    None,
                ));
            }

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

            if candidate.disposition()
                == TargetMatchDisposition::NotExecutionTarget
            {
                target_findings.push(FeasibilityFinding::new(
                    FeasibilityFindingCode::TargetNotApplicable,
                    None,
                    Some(target_id.clone()),
                    "the discovered target does not represent a \
                     computation-realization target",
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

        if !discovery_complete {
            findings.push(FeasibilityFinding::new(
                FeasibilityFindingCode::InventoryIncomplete,
                None,
                None,
                "target discovery or resource discovery is incomplete",
                None,
            ));
        }

        // Stable total ordering makes report comparisons reproducible.
        findings.sort_unstable_by(compare_findings);

        let disposition = classify_overall(
            &request.semantic_validity,
            request.capabilities,
            request.resources,
            &targets,
            request.evidence,
        );

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
// INTERNAL HELPERS
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

fn compare_findings(
    left: &FeasibilityFinding,
    right: &FeasibilityFinding,
) -> std::cmp::Ordering {
    left.code
        .cmp(&right.code)
        .then_with(|| left.domain.cmp(&right.domain))
        .then_with(|| left.target.cmp(&right.target))
        .then_with(|| left.message.cmp(&right.message))
        .then_with(|| left.evidence_id.cmp(&right.evidence_id))
}

// ============================================================================
// EVIDENCE CLASSIFICATION
// ============================================================================

fn finding_from_evidence(
    evidence: &FeasibilityEvidence,
    target: Option<TargetId>,
) -> Option<FeasibilityFinding> {
    let code = match evidence.status {
        EvidenceStatus::Satisfied
        | EvidenceStatus::NotApplicable => {
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
    // Global semantic invalidity cannot be repaired by target selection.
    if semantic.is_invalid()
        || findings.iter().any(|finding| {
            finding.code == FeasibilityFindingCode::SemanticInvalid
        })
    {
        return FeasibilityDisposition::SemanticInvalid;
    }

    // Proven representability failure is definitive.
    if findings.iter().any(|finding| {
        finding.code == FeasibilityFindingCode::Unrepresentable
    }) {
        return FeasibilityDisposition::Unrepresentable;
    }

    // Proven mandatory constraint failure is definitive.
    if findings.iter().any(|finding| {
        finding.code == FeasibilityFindingCode::ConstraintUnsatisfied
    }) {
        return FeasibilityDisposition::ConstraintUnsatisfiable;
    }

    // Proven capability incompatibility is not made uncertain by unrelated
    // missing evidence.
    match capability {
        TargetMatchDisposition::Incompatible => {
            return FeasibilityDisposition::CapabilityUnsatisfiable;
        }

        TargetMatchDisposition::NotExecutionTarget => {
            return FeasibilityDisposition::NotApplicable;
        }

        TargetMatchDisposition::Satisfied
        | TargetMatchDisposition::Unresolved
        | TargetMatchDisposition::NoRequirements => {}
    }

    // Proven resource shortfall is also definitive for this target.
    if resource == Some(TargetResourceDisposition::Infeasible) {
        return FeasibilityDisposition::ResourceUnsatisfiable;
    }

    if resource == Some(TargetResourceDisposition::InvalidEvidence) {
        return FeasibilityDisposition::InvalidEvidence;
    }

    // Unknown semantic validity cannot establish a realization.
    if semantic.is_unknown() {
        return FeasibilityDisposition::Indeterminate;
    }

    // A missing or unresolved evidence record blocks a success conclusion.
    if !evidence_complete
        || findings.iter().any(|finding| {
            finding.code == FeasibilityFindingCode::EvidenceIncomplete
                || finding.code == FeasibilityFindingCode::ResourceUnknown
        })
    {
        return FeasibilityDisposition::Indeterminate;
    }

    match capability {
        TargetMatchDisposition::Unresolved
        | TargetMatchDisposition::NoRequirements => {
            return FeasibilityDisposition::Indeterminate;
        }

        TargetMatchDisposition::Satisfied => {}

        TargetMatchDisposition::Incompatible
        | TargetMatchDisposition::NotExecutionTarget => {
            return FeasibilityDisposition::NotApplicable;
        }
    }

    match resource {
        Some(TargetResourceDisposition::Feasible) => {
            FeasibilityDisposition::Feasible
        }

        Some(TargetResourceDisposition::NoResourceRequirements) => {
            FeasibilityDisposition::Feasible
        }

        Some(
            TargetResourceDisposition::FeasibleWithUnmetPreferences,
        ) => {
            FeasibilityDisposition::FeasibleWithTradeoffs
        }

        Some(TargetResourceDisposition::Infeasible) => {
            FeasibilityDisposition::ResourceUnsatisfiable
        }

        Some(TargetResourceDisposition::InvalidEvidence) => {
            FeasibilityDisposition::InvalidEvidence
        }

        Some(
            TargetResourceDisposition::Unresolved
            | TargetResourceDisposition::EvidenceUnavailable
            | TargetResourceDisposition::EvidenceFailure
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
    if semantic.is_invalid()
        || evidence.global_records().any(|record| {
            record.domain.is_semantic_validity()
                && record.status == EvidenceStatus::Unsatisfied
        })
    {
        return FeasibilityDisposition::SemanticInvalid;
    }

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

    if targets.iter().any(|target| {
        target.disposition == FeasibilityDisposition::InvalidEvidence
    }) {
        // A separate proven feasible target may still establish an
        // existentially feasible realization.
        if !targets.iter().any(|target| target.is_feasible()) {
            return FeasibilityDisposition::InvalidEvidence;
        }
    }

    // A proven feasible candidate establishes existential feasibility.
    // Incomplete discovery can still mean that the inventory is not
    // exhaustive; that is reported separately by is_complete().
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

    if semantic.is_unknown() || !evidence.is_complete() {
        return FeasibilityDisposition::Indeterminate;
    }

    // Explicit unknown global evidence blocks an overall conclusion.
    if evidence.global_records().any(|record| {
        record.status == EvidenceStatus::Unknown
    }) {
        return FeasibilityDisposition::Indeterminate;
    }

    // No capability requirements is not proof of compatibility.
    if capabilities.requirement_count() == 0 {
        return FeasibilityDisposition::Indeterminate;
    }

    let realization_targets = capabilities
        .candidates()
        .iter()
        .filter(|candidate| {
            candidate.disposition()
                != TargetMatchDisposition::NotExecutionTarget
        })
        .count();

    if realization_targets == 0 {
        if capabilities.discovery_is_complete() {
            return FeasibilityDisposition::NoRealizationTarget;
        }

        return FeasibilityDisposition::Indeterminate;
    }

    let eligible_count = capabilities
        .candidates()
        .iter()
        .filter(|candidate| candidate.is_eligible_for_planning())
        .count();

    let incompatible_count = capabilities
        .candidates()
        .iter()
        .filter(|candidate| {
            candidate.disposition()
                == TargetMatchDisposition::Incompatible
        })
        .count();

    // All applicable computation targets explicitly reject the required
    // capabilities.
    if incompatible_count == realization_targets
        && capabilities.discovery_is_complete()
    {
        return FeasibilityDisposition::CapabilityUnsatisfiable;
    }

    // All capability-eligible candidates have proven resource shortfalls.
    let resource_failures = targets
        .iter()
        .filter(|target| {
            target.capability_disposition
                == TargetMatchDisposition::Satisfied
                && target.disposition
                    == FeasibilityDisposition::ResourceUnsatisfiable
        })
        .count();

    if eligible_count > 0
        && resource_failures == eligible_count
        && resources.discovery_is_complete()
    {
        return FeasibilityDisposition::ResourceUnsatisfiable;
    }

    let unrepresentable = targets
        .iter()
        .filter(|target| {
            target.capability_disposition
                == TargetMatchDisposition::Satisfied
                && target.disposition
                    == FeasibilityDisposition::Unrepresentable
        })
        .count();

    if eligible_count > 0 && unrepresentable == eligible_count {
        return FeasibilityDisposition::Unrepresentable;
    }

    let constraint_failures = targets
        .iter()
        .filter(|target| {
            target.capability_disposition
                == TargetMatchDisposition::Satisfied
                && target.disposition
                    == FeasibilityDisposition::ConstraintUnsatisfiable
        })
        .count();

    if eligible_count > 0
        && constraint_failures == eligible_count
    {
        return FeasibilityDisposition::ConstraintUnsatisfiable;
    }

    // Mixed, incomplete, or unresolved candidate outcomes must not be
    // upgraded into a success or a universal infeasibility claim.
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
    fn api_version_is_current() {
        assert_eq!(FEASIBILITY_API_VERSION, 2);
    }

    #[test]
    fn evidence_domains_are_open_world() {
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
        assert!(EvidenceDomain::new("").is_err());
        assert!(EvidenceDomain::new("   ").is_err());
    }

    #[test]
    fn evidence_domain_rejects_control_characters() {
        assert!(
            EvidenceDomain::new("planning\ninvalid").is_err()
        );
    }

    #[test]
    fn unknown_evidence_is_not_success() {
        let evidence = FeasibilityEvidence::global(
            EvidenceDomain::new(DOMAIN_DEPENDENCY)
                .expect("valid domain"),
            EvidenceStatus::Unknown,
            "dependency validation has not completed",
        );

        let finding = finding_from_evidence(&evidence, None)
            .expect("unknown evidence must produce a finding");

        assert_eq!(
            finding.code(),
            FeasibilityFindingCode::EvidenceIncomplete,
        );
    }

    #[test]
    fn global_constraint_failure_is_reported() {
        let evidence = FeasibilityEvidence::global(
            EvidenceDomain::new(DOMAIN_SECURITY)
                .expect("valid domain"),
            EvidenceStatus::Unsatisfied,
            "security constraint is not satisfied",
        );

        let finding = finding_from_evidence(&evidence, None)
            .expect("failed global evidence must be reported");

        assert_eq!(
            finding.code(),
            FeasibilityFindingCode::ConstraintUnsatisfied,
        );
    }

    #[test]
    fn semantic_invalidity_is_distinct_from_resource_failure() {
        let disposition = classify_target(
            &SemanticValidity::invalid(
                "invalid semantic operation",
            ),
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
        let disposition = classify_target(
            &SemanticValidity::unknown(
                "semantic validation is incomplete",
            ),
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
            &SemanticValidity::valid(),
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
    fn insufficient_resources_are_definitive() {
        let disposition = classify_target(
            &SemanticValidity::valid(),
            TargetMatchDisposition::Satisfied,
            Some(TargetResourceDisposition::Infeasible),
            &[
                FeasibilityFinding::new(
                    FeasibilityFindingCode::EvidenceIncomplete,
                    None,
                    None,
                    "another evidence producer is incomplete",
                    None,
                ),
            ],
            false,
        );

        assert_eq!(
            disposition,
            FeasibilityDisposition::ResourceUnsatisfiable,
        );
    }

    #[test]
    fn absent_resource_evidence_is_indeterminate() {
        let disposition = classify_target(
            &SemanticValidity::valid(),
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
        let disposition = classify_target(
            &SemanticValidity::valid(),
            TargetMatchDisposition::NoRequirements,
            Some(
                TargetResourceDisposition::NoResourceRequirements,
            ),
            &[],
            true,
        );

        assert_eq!(
            disposition,
            FeasibilityDisposition::Indeterminate,
        );
    }

    #[test]
    fn explicit_capability_incompatibility_is_preserved() {
        let disposition = classify_target(
            &SemanticValidity::valid(),
            TargetMatchDisposition::Incompatible,
            None,
            &[],
            false,
        );

        assert_eq!(
            disposition,
            FeasibilityDisposition::CapabilityUnsatisfiable,
        );
    }

    #[test]
    fn non_execution_targets_are_not_capability_failures() {
        let disposition = classify_target(
            &SemanticValidity::valid(),
            TargetMatchDisposition::NotExecutionTarget,
            None,
            &[],
            true,
        );

        assert_eq!(
            disposition,
            FeasibilityDisposition::NotApplicable,
        );
    }

    #[test]
    fn invalid_resource_evidence_remains_distinct() {
        let disposition = classify_target(
            &SemanticValidity::valid(),
            TargetMatchDisposition::Satisfied,
            Some(TargetResourceDisposition::InvalidEvidence),
            &[],
            true,
        );

        assert_eq!(
            disposition,
            FeasibilityDisposition::InvalidEvidence,
        );
    }

    #[test]
    fn unrepresentable_evidence_is_not_a_fallback() {
        let evidence = FeasibilityEvidence::global(
            EvidenceDomain::new(DOMAIN_REPRESENTABILITY)
                .expect("valid domain"),
            EvidenceStatus::Unsatisfied,
            "required operation cannot be represented",
        );

        let finding = finding_from_evidence(&evidence, None)
            .expect("unrepresentability must be reported");

        assert_eq!(
            finding.code(),
            FeasibilityFindingCode::Unrepresentable,
        );
    }

    #[test]
    fn evidence_order_is_deterministic() {
        let first = FeasibilityEvidence::global(
            EvidenceDomain::new("z.domain")
                .expect("valid domain"),
            EvidenceStatus::Satisfied,
            "z",
        );

        let second = FeasibilityEvidence::global(
            EvidenceDomain::new("a.domain")
                .expect("valid domain"),
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
                .expect("valid domain"),
            EvidenceStatus::NotApplicable,
            "no synchronization boundary exists",
        );

        assert!(
            finding_from_evidence(&evidence, None).is_none()
        );
    }
}
