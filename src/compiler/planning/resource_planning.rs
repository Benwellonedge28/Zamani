
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Compiler — Resource Planning
//!
//! Path: src/compiler/planning/resource_planning.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! ## PURPOSE
//!
//! Produce abstract, target-scoped resource feasibility assessments from:
//!
//! - canonical semantic ResourceRequirement values;
//! - targets admitted by capability matching;
//! - explicitly supplied, target-scoped resource evidence.
//!
//! The result is a planning assessment. It is not an allocation, reservation,
//! schedule, physical placement, or guarantee of future availability.
//!
//! ## AUTHORITATIVE OWNERS
//!
//! Target identity:
//!     crate::compiler::configuration::target
//!
//! Target discovery:
//!     crate::compiler::planning::target_discovery
//!
//! Capability matching:
//!     crate::compiler::planning::capability_matching
//!
//! Canonical semantic resource model:
//!     crate::frontend::semantic::resources::model
//!
//! Canonical resource evidence interface:
//!     crate::frontend::semantic::resources::constraints
//!
//! Physical resource acquisition:
//!     crate::runtime and the relevant hardware/provider subsystems
//!
//! ## OWNS
//!
//! - Target-scoped resource evidence acquisition interface.
//! - Aggregation of compatible quantitative resource demands.
//! - Exact, arbitrary-decimal demand addition.
//! - Comparison of demands against supplied capacity evidence.
//! - Per-target resource assessment.
//! - Explicit unresolved and invalid assessment states.
//! - Deterministic resource planning reports.
//! - Snapshot provenance in planning results.
//!
//! ## DOES NOT OWN
//!
//! - Resource requirement semantics or resource-kind identity.
//! - Capability definitions or capability compatibility.
//! - Target discovery or target descriptors.
//! - Physical resource inventories.
//! - Hardware probing or provider authentication.
//! - Unit conversion registries.
//! - Resource allocation or reservation.
//! - Scheduling, partitioning, routing, or physical placement.
//! - Runtime execution.
//! - IR definitions or source grammar.
//!
//! ## SCALABILITY
//!
//! There are no fixed limits on:
//!
//! - targets;
//! - resource kinds;
//! - resource requirements;
//! - resource quantities;
//! - resource providers;
//! - processors;
//! - accelerators;
//! - devices;
//! - distributed nodes;
//! - memory capacity;
//! - future resource dimensions.
//!
//! Exact quantities remain decimal strings through the canonical semantic
//! model. Aggregation does not convert quantities to u64, u128, usize, or
//! floating-point values.
//!
//! Actual execution remains bounded by available memory, address space,
//! processing time, provider capacity, and explicitly configured budgets.
//!
//! Literal infinity is not promised.
//!
//! ## DETERMINISM
//!
//! Ordered maps group resource demands.
//! Target results preserve capability-matching order.
//! No clock, random identifier, environment lookup, or global mutable state
//! influences a result.
//!
//! ## SAFETY
//!
//! Safe Rust only. No hardware operation is performed by this module.
//!
//! ## INTEGRATION
//!
//! The public entry point is ResourcePlanner::plan.
//!
//! It consumes CapabilityMatchReport and canonical semantic resource
//! requirements, then obtains immutable target-specific evidence through
//! ResourceEvidenceProvider.
//!
//! The returned ResourcePlanningReport can be consumed by:
//!
//! - partitioning.rs;
//! - feasibility.rs;
//! - scheduling.rs;
//! - deployment planning;
//! - backend realization planning.
//!
//! Those consumers must still perform their own responsibilities.

use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use crate::compiler::configuration::target::TargetId;

use crate::compiler::planning::capability_matching::{
    CapabilityMatchReport,
    TargetCapabilityMatch,
    TargetMatchDisposition,
};

use crate::frontend::semantic::resources::constraints::{
    ResourceConstraintResolver,
    ResourceValueResolution,
};

use crate::frontend::semantic::resources::model::{
    ExactQuantity,
    ResourceIntent,
    ResourceKind,
    ResourceQuantity,
    ResourceRequirement,
    ResourceUnit,
};

// ============================================================================
// MODULE IDENTITY
// ============================================================================

/// Stable architectural identity of this module.
pub const RESOURCE_PLANNING_MODULE_NAME: &str =
    "zamani.compiler.planning.resource_planning";

/// Public API version of this module.
pub const RESOURCE_PLANNING_API_VERSION: u16 = 1;

// ============================================================================
// RESOURCE EVIDENCE SNAPSHOT
// ============================================================================

/// One immutable, target-scoped resource evidence snapshot.
///
/// The resolver must describe the target identified by the corresponding
/// target ID. It must not combine evidence from unrelated targets.
///
/// `snapshot_id` is an externally supplied stable revision or identity.
/// It allows downstream systems to detect that a plan was made against an
/// older inventory observation.
///
/// The resolver supplies observations only. It must not allocate or reserve
/// resources as a side effect of being queried.
pub struct ResourceEvidenceSnapshot {
    snapshot_id: String,
    resolver: Box<dyn ResourceConstraintResolver>,
}

impl ResourceEvidenceSnapshot {
    /// Creates a resource evidence snapshot.
    ///
    /// # Errors
    ///
    /// Rejects empty snapshot identities and identities containing control
    /// characters.
    pub fn new<R>(
        snapshot_id: impl Into<String>,
        resolver: R,
    ) -> Result<Self, ResourcePlanningError>
    where
        R: ResourceConstraintResolver + 'static,
    {
        Self::from_boxed(snapshot_id, Box::new(resolver))
    }

    /// Creates a snapshot from an existing resolver trait object.
    pub fn from_boxed(
        snapshot_id: impl Into<String>,
        resolver: Box<dyn ResourceConstraintResolver>,
    ) -> Result<Self, ResourcePlanningError> {
        let snapshot_id = snapshot_id.into();

        if snapshot_id.trim().is_empty() {
            return Err(ResourcePlanningError::InvalidSnapshotId(
                "snapshot identity cannot be empty".to_owned(),
            ));
        }

        if snapshot_id.chars().any(char::is_control) {
            return Err(ResourcePlanningError::InvalidSnapshotId(
                "snapshot identity cannot contain control characters"
                    .to_owned(),
            ));
        }

        Ok(Self {
            snapshot_id,
            resolver,
        })
    }

    /// Returns the immutable snapshot identity.
    #[must_use]
    pub fn snapshot_id(&self) -> &str {
        &self.snapshot_id
    }

    /// Returns the target-scoped semantic resource resolver.
    ///
    /// This is primarily useful to adapters and integration layers.
    #[must_use]
    pub fn resolver(&self) -> &dyn ResourceConstraintResolver {
        self.resolver.as_ref()
    }
}

// ============================================================================
// RESOURCE EVIDENCE PROVIDER
// ============================================================================

/// Supplies a target-specific immutable resource evidence snapshot.
///
/// Implementations may adapt an existing hardware inventory, runtime
/// inventory observation, provider inventory, simulator inventory, or
/// distributed resource inventory.
///
/// The provider must not reserve or allocate resources.
///
/// An absent snapshot means that resource evidence is unavailable. It does
/// not mean that the target has zero resources.
///
/// The returned snapshot must be internally consistent. If its source
/// changes, the provider must use a different snapshot identity.
pub trait ResourceEvidenceProvider {
    /// Obtains the current planning snapshot for one target.
    ///
    /// `Ok(None)` means that no usable snapshot is currently available.
    ///
    /// `Err` means that the provider could not establish a trustworthy
    /// snapshot. These outcomes are deliberately distinct.
    fn snapshot_for(
        &self,
        target: &TargetId,
    ) -> Result<Option<ResourceEvidenceSnapshot>, ResourceEvidenceError>;
}

/// Failure reported by a resource evidence provider.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResourceEvidenceError {
    /// Target whose resource evidence could not be obtained.
    pub target: TargetId,

    /// Provider's explanation.
    pub message: String,
}

impl ResourceEvidenceError {
    /// Creates a resource evidence provider error.
    #[must_use]
    pub fn new(
        target: TargetId,
        message: impl Into<String>,
    ) -> Self {
        Self {
            target,
            message: message.into(),
        }
    }
}

impl fmt::Display for ResourceEvidenceError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            formatter,
            "resource evidence provider failed for target `{}`: {}",
            self.target,
            self.message
        )
    }
}

impl Error for ResourceEvidenceError {}

// ============================================================================
// REQUIREMENT INTENT CLASSIFICATION
// ============================================================================

/// Classification of a canonical semantic resource requirement.
///
/// The categories preserve semantic intent rather than treating every
/// resource declaration as a mandatory capacity demand.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum ResourceDemandClass {
    /// A mandatory quantitative demand.
    Mandatory,

    /// A non-binding quantitative preference or hint.
    Advisory,

    /// An intent that requires a different constraint or policy evaluation.
    Policy,

    /// An intent not recognized by this implementation version.
    Unknown,
}

impl ResourceDemandClass {
    /// Classifies a canonical semantic resource intent.
    #[must_use]
    pub const fn from_intent(intent: ResourceIntent) -> Self {
        match intent {
            ResourceIntent::Required => Self::Mandatory,
            ResourceIntent::Preferred | ResourceIntent::Hint => Self::Advisory,
            ResourceIntent::Budget
            | ResourceIntent::Allowed
            | ResourceIntent::Forbidden => Self::Policy,
            _ => Self::Unknown,
        }
    }
}

// ============================================================================
// ASSESSMENT DISPOSITIONS
// ============================================================================

/// Result of assessing one quantitative resource demand.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum DemandDisposition {
    /// The available evidence establishes sufficient capacity.
    Satisfied,

    /// The known available capacity is smaller than the demand.
    Insufficient,

    /// Required evidence or a symbolic quantity remains unresolved.
    Unresolved,

    /// The evidence or quantity was invalid.
    Invalid,

    /// No demand of this class was supplied.
    NotRequested,
}

impl DemandDisposition {
    /// Whether the demand is proven satisfied.
    #[must_use]
    pub const fn is_satisfied(self) -> bool {
        matches!(self, Self::Satisfied)
    }

    /// Whether evidence proves insufficient capacity.
    #[must_use]
    pub const fn is_insufficient(self) -> bool {
        matches!(self, Self::Insufficient)
    }

    /// Whether more evidence is needed.
    #[must_use]
    pub const fn is_unresolved(self) -> bool {
        matches!(self, Self::Unresolved)
    }

    /// Whether assessment encountered invalid input or evidence.
    #[must_use]
    pub const fn is_invalid(self) -> bool {
        matches!(self, Self::Invalid)
    }
}

/// Overall resource-planning outcome for one target.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum TargetResourceDisposition {
    /// Capability matching did not admit this target for resource planning.
    SkippedCapabilityIneligible,

    /// The target has no resource requirements to quantify.
    ///
    /// This is not a claim that every other aspect of target realization
    /// is feasible.
    NoResourceRequirements,

    /// All mandatory resource demands are satisfied.
    ///
    /// Advisory preferences may still be unmet.
    Feasible,

    /// Mandatory resource demands are satisfied, but one or more advisory
    /// preferences could not be met.
    FeasibleWithUnmetPreferences,

    /// Available evidence proves that at least one mandatory demand cannot
    /// be satisfied.
    Infeasible,

    /// The assessment lacks enough evidence to decide.
    Unresolved,

    /// The resource evidence provider returned no snapshot.
    EvidenceUnavailable,

    /// The resource evidence provider failed.
    EvidenceFailure,

    /// Invalid resource evidence or arithmetic was encountered.
    InvalidEvidence,
}

impl TargetResourceDisposition {
    /// Whether mandatory quantitative demands are proven feasible.
    #[must_use]
    pub const fn mandatory_demands_satisfied(self) -> bool {
        matches!(
            self,
            Self::Feasible | Self::FeasibleWithUnmetPreferences
        )
    }

    /// Whether the target has a proven mandatory resource shortfall.
    #[must_use]
    pub const fn is_infeasible(self) -> bool {
        matches!(self, Self::Infeasible)
    }

    /// Whether the result is incomplete or unresolved.
    #[must_use]
    pub const fn is_unresolved(self) -> bool {
        matches!(
            self,
            Self::Unresolved
                | Self::EvidenceUnavailable
                | Self::EvidenceFailure
                | Self::InvalidEvidence
        )
    }

    /// Whether this result establishes mandatory resource feasibility.
    ///
    /// This does not establish backend compatibility, authorization,
    /// scheduling feasibility, or runtime availability.
    #[must_use]
    pub const fn is_resource_feasible(self) -> bool {
        self.mandatory_demands_satisfied()
    }
}

// ============================================================================
// RESOURCE GROUP ASSESSMENT
// ============================================================================

/// Assessment for one resource kind and unit combination.
///
/// Multiple requirements with the same kind and unit are aggregated before
/// comparing their combined demand with available capacity.
///
/// Mandatory and advisory demands are retained separately.
///
/// The `available` value is the observed capacity supplied by the resolver.
/// It is not a reservation or an exclusive allocation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResourceDimensionAssessment {
    resource_kind: String,
    unit: Option<ResourceUnit>,
    mandatory_demand: Option<ExactQuantity>,
    advisory_demand: Option<ExactQuantity>,
    available: Option<ExactQuantity>,
    mandatory_disposition: DemandDisposition,
    advisory_disposition: DemandDisposition,
    mandatory_requirement_indices: Vec<usize>,
    advisory_requirement_indices: Vec<usize>,
    unresolved_requirement_indices: Vec<usize>,
    explanation: String,
}

impl ResourceDimensionAssessment {
    /// Returns the canonical resource-kind identity.
    #[must_use]
    pub fn resource_kind(&self) -> &str {
        &self.resource_kind
    }

    /// Returns the unit identity, if supplied.
    #[must_use]
    pub fn unit(&self) -> Option<&ResourceUnit> {
        self.unit.as_ref()
    }

    /// Returns the aggregated mandatory demand.
    #[must_use]
    pub fn mandatory_demand(&self) -> Option<&ExactQuantity> {
        self.mandatory_demand.as_ref()
    }

    /// Returns the aggregated advisory demand.
    #[must_use]
    pub fn advisory_demand(&self) -> Option<&ExactQuantity> {
        self.advisory_demand.as_ref()
    }

    /// Returns the observed capacity, if known.
    #[must_use]
    pub fn available(&self) -> Option<&ExactQuantity> {
        self.available.as_ref()
    }

    /// Returns the mandatory-demand assessment.
    #[must_use]
    pub const fn mandatory_disposition(&self) -> DemandDisposition {
        self.mandatory_disposition
    }

    /// Returns the advisory-demand assessment.
    #[must_use]
    pub const fn advisory_disposition(&self) -> DemandDisposition {
        self.advisory_disposition
    }

    /// Returns source requirement indices contributing to mandatory demand.
    #[must_use]
    pub fn mandatory_requirement_indices(&self) -> &[usize] {
        &self.mandatory_requirement_indices
    }

    /// Returns source requirement indices contributing to advisory demand.
    #[must_use]
    pub fn advisory_requirement_indices(&self) -> &[usize] {
        &self.advisory_requirement_indices
    }

    /// Returns requirement indices whose quantities remain unresolved.
    #[must_use]
    pub fn unresolved_requirement_indices(&self) -> &[usize] {
        &self.unresolved_requirement_indices
    }

    /// Returns a human-readable explanation.
    ///
    /// Programmatic consumers must use the typed dispositions.
    #[must_use]
    pub fn explanation(&self) -> &str {
        &self.explanation
    }
}

// ============================================================================
// POLICY OBLIGATIONS
// ============================================================================

/// A semantic resource intent that this quantitative planner must not
/// reinterpret as an ordinary minimum capacity demand.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResourcePolicyObligation {
    requirement_index: usize,
    resource_kind: String,
    intent: ResourceIntent,
    requires_downstream_evaluation: bool,
}

impl ResourcePolicyObligation {
    /// Returns the original requirement index.
    #[must_use]
    pub const fn requirement_index(&self) -> usize {
        self.requirement_index
    }

    /// Returns the canonical resource-kind identity.
    #[must_use]
    pub fn resource_kind(&self) -> &str {
        &self.resource_kind
    }

    /// Returns the original semantic intent.
    #[must_use]
    pub const fn intent(&self) -> ResourceIntent {
        self.intent
    }

    /// Whether a downstream constraint or usage analysis must evaluate it.
    #[must_use]
    pub const fn requires_downstream_evaluation(&self) -> bool {
        self.requires_downstream_evaluation
    }
}

// ============================================================================
// ONE TARGET RESOURCE PLAN
// ============================================================================

/// Resource-planning result for one target.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TargetResourcePlan {
    target_id: TargetId,
    capability_disposition: TargetMatchDisposition,
    disposition: TargetResourceDisposition,
    snapshot_id: Option<String>,
    dimensions: Vec<ResourceDimensionAssessment>,
    policy_obligations: Vec<ResourcePolicyObligation>,
    evidence_error: Option<String>,
}

impl TargetResourcePlan {
    /// Returns the target identity.
    #[must_use]
    pub fn target_id(&self) -> &TargetId {
        &self.target_id
    }

    /// Returns the capability-matching disposition.
    #[must_use]
    pub const fn capability_disposition(&self) -> TargetMatchDisposition {
        self.capability_disposition
    }

    /// Returns the resource-planning disposition.
    #[must_use]
    pub const fn disposition(&self) -> TargetResourceDisposition {
        self.disposition
    }

    /// Returns the resource evidence snapshot identity.
    #[must_use]
    pub fn snapshot_id(&self) -> Option<&str> {
        self.snapshot_id.as_deref()
    }

    /// Returns all resource dimension assessments.
    #[must_use]
    pub fn dimensions(&self) -> &[ResourceDimensionAssessment] {
        &self.dimensions
    }

    /// Returns semantic policy obligations not treated as capacity demands.
    #[must_use]
    pub fn policy_obligations(&self) -> &[ResourcePolicyObligation] {
        &self.policy_obligations
    }

    /// Returns the provider error, if any.
    #[must_use]
    pub fn evidence_error(&self) -> Option<&str> {
        self.evidence_error.as_deref()
    }

    /// Whether all mandatory quantitative demands are proven satisfied.
    #[must_use]
    pub const fn is_resource_feasible(&self) -> bool {
        self.disposition.is_resource_feasible()
    }

    /// Whether at least one mandatory resource demand is insufficient.
    #[must_use]
    pub const fn is_infeasible(&self) -> bool {
        self.disposition.is_infeasible()
    }

    /// Whether the result is unresolved.
    #[must_use]
    pub const fn is_unresolved(&self) -> bool {
        self.disposition.is_unresolved()
    }
}

// ============================================================================
// COMPLETE REPORT
// ============================================================================

/// Complete resource-planning report across capability-matched targets.
///
/// Discovery completeness is preserved from capability matching. A report
/// may contain individually feasible targets while the global inventory
/// remains incomplete.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResourcePlanningReport {
    targets: Vec<TargetResourcePlan>,
    discovery_complete: bool,
    requirement_count: usize,
}

impl ResourcePlanningReport {
    fn new(
        discovery_complete: bool,
        requirement_count: usize,
    ) -> Self {
        Self {
            targets: Vec::new(),
            discovery_complete,
            requirement_count,
        }
    }

    /// Returns all target plans in capability-matching order.
    #[must_use]
    pub fn targets(&self) -> &[TargetResourcePlan] {
        &self.targets
    }

    /// Returns targets whose mandatory resource demands are proven satisfied.
    pub fn feasible_targets(
        &self,
    ) -> impl Iterator<Item = &TargetResourcePlan> {
        self.targets
            .iter()
            .filter(|target| target.is_resource_feasible())
    }

    /// Returns targets with proven mandatory resource shortfalls.
    pub fn infeasible_targets(
        &self,
    ) -> impl Iterator<Item = &TargetResourcePlan> {
        self.targets
            .iter()
            .filter(|target| target.is_infeasible())
    }

    /// Returns targets whose resource assessments remain unresolved.
    pub fn unresolved_targets(
        &self,
    ) -> impl Iterator<Item = &TargetResourcePlan> {
        self.targets
            .iter()
            .filter(|target| target.is_unresolved())
    }

    /// Finds a plan by target identity.
    #[must_use]
    pub fn target(
        &self,
        id: &TargetId,
    ) -> Option<&TargetResourcePlan> {
        self.targets
            .iter()
            .find(|target| target.target_id() == id)
    }

    /// Returns the number of considered targets.
    #[must_use]
    pub fn len(&self) -> usize {
        self.targets.len()
    }

    /// Whether no capability-matched targets were supplied.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.targets.is_empty()
    }

    /// Returns the number of semantic resource requirements supplied.
    #[must_use]
    pub const fn requirement_count(&self) -> usize {
        self.requirement_count
    }

    /// Whether target discovery was complete before resource planning.
    #[must_use]
    pub const fn discovery_is_complete(&self) -> bool {
        self.discovery_complete
    }

    /// Whether every eligible target has a decisive resource assessment
    /// and the target inventory is complete.
    ///
    /// This does not establish scheduling, backend, authorization, or
    /// execution feasibility.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.discovery_complete
            && self.targets.iter().all(|target| {
                !target.is_unresolved()
            })
    }

    /// Whether at least one target has proven mandatory resource feasibility.
    #[must_use]
    pub fn has_feasible_target(&self) -> bool {
        self.feasible_targets().next().is_some()
    }
}

// ============================================================================
// PLANNER
// ============================================================================

/// Stateless compiler resource-planning service.
///
/// It consumes canonical semantic requirements and capability-matched
/// targets. It does not discover targets or allocate resources.
#[derive(Clone, Copy, Debug, Default)]
pub struct ResourcePlanner;

impl ResourcePlanner {
    /// Creates a resource planner.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Plans resource feasibility for capability-matched targets.
    ///
    /// Targets that did not satisfy capability matching are retained as
    /// skipped results. They are never accidentally promoted into resource
    /// candidates.
    ///
    /// Resource evidence failures are preserved per target. A failed
    /// provider does not erase successful assessments for other targets.
    ///
    /// # Important
    ///
    /// An empty capability requirement set is not admitted as a satisfied
    /// target by the canonical capability matcher. This method respects that
    /// decision rather than attempting to bypass it.
    pub fn plan(
        &self,
        capability_report: &CapabilityMatchReport,
        requirements: &[ResourceRequirement],
        evidence_provider: &dyn ResourceEvidenceProvider,
    ) -> ResourcePlanningReport {
        let mut report = ResourcePlanningReport::new(
            capability_report.discovery_is_complete(),
            requirements.len(),
        );

        for candidate in capability_report.candidates() {
            let target_plan = if !candidate.is_eligible_for_planning() {
                skipped_target(candidate)
            } else {
                match evidence_provider.snapshot_for(candidate.target_id()) {
                    Ok(Some(snapshot)) => {
                        plan_target_with_snapshot(
                            candidate,
                            requirements,
                            snapshot,
                        )
                    }

                    Ok(None) => unavailable_target(candidate),

                    Err(error) => failed_target(candidate, error),
                }
            };

            report.targets.push(target_plan);
        }

        report
    }

    /// Plans one already capability-matched target directly.
    ///
    /// This method is useful for incremental planning when one target's
    /// resource evidence changes.
    ///
    /// It does not repeat capability matching.
    pub fn plan_target(
        &self,
        candidate: &TargetCapabilityMatch,
        requirements: &[ResourceRequirement],
        snapshot: ResourceEvidenceSnapshot,
    ) -> TargetResourcePlan {
        if !candidate.is_eligible_for_planning() {
            return skipped_target(candidate);
        }

        plan_target_with_snapshot(candidate, requirements, snapshot)
    }

    /// Returns the stable module identity.
    #[must_use]
    pub const fn module_name() -> &'static str {
        RESOURCE_PLANNING_MODULE_NAME
    }

    /// Returns the public API version.
    #[must_use]
    pub const fn api_version() -> u16 {
        RESOURCE_PLANNING_API_VERSION
    }
}

// ============================================================================
// INTERNAL AGGREGATION
// ============================================================================

#[derive(Clone)]
struct DemandAccumulator {
    kind: ResourceKind,
    unit: Option<ResourceUnit>,
    mandatory: Option<ExactQuantity>,
    advisory: Option<ExactQuantity>,
    mandatory_unknown: bool,
    advisory_unknown: bool,
    mandatory_indices: Vec<usize>,
    advisory_indices: Vec<usize>,
    unresolved_indices: Vec<usize>,
}

impl DemandAccumulator {
    fn new(
        kind: ResourceKind,
        unit: Option<ResourceUnit>,
    ) -> Self {
        Self {
            kind,
            unit,
            mandatory: None,
            advisory: None,
            mandatory_unknown: false,
            advisory_unknown: false,
            mandatory_indices: Vec::new(),
            advisory_indices: Vec::new(),
            unresolved_indices: Vec::new(),
        }
    }

    fn add(
        &mut self,
        class: ResourceDemandClass,
        quantity: &ResourceQuantity,
        index: usize,
    ) -> Result<(), ResourcePlanningError> {
        let destination = match class {
            ResourceDemandClass::Mandatory => &mut self.mandatory,
            ResourceDemandClass::Advisory => &mut self.advisory,
            ResourceDemandClass::Policy | ResourceDemandClass::Unknown => {
                return Ok(());
            }
        };

        let indices = match class {
            ResourceDemandClass::Mandatory => &mut self.mandatory_indices,
            ResourceDemandClass::Advisory => &mut self.advisory_indices,
            _ => return Ok(()),
        };

        match quantity {
            ResourceQuantity::Known(value) => {
                *destination = Some(match destination.take() {
                    Some(current) => add_exact_quantities(&current, value)?,
                    None => value.clone(),
                });
                indices.push(index);
            }

            ResourceQuantity::Unspecified
            | ResourceQuantity::Symbolic(_)
            | ResourceQuantity::Runtime(_) => {
                match class {
                    ResourceDemandClass::Mandatory => {
                        self.mandatory_unknown = true;
                    }
                    ResourceDemandClass::Advisory => {
                        self.advisory_unknown = true;
                    }
                    _ => {}
                }

                self.unresolved_indices.push(index);
                indices.push(index);
            }

            _ => {
                self.unresolved_indices.push(index);
                indices.push(index);

                match class {
                    ResourceDemandClass::Mandatory => {
                        self.mandatory_unknown = true;
                    }
                    ResourceDemandClass::Advisory => {
                        self.advisory_unknown = true;
                    }
                    _ => {}
                }
            }
        }

        Ok(())
    }
}

fn plan_target_with_snapshot(
    candidate: &TargetCapabilityMatch,
    requirements: &[ResourceRequirement],
    snapshot: ResourceEvidenceSnapshot,
) -> TargetResourcePlan {
    if requirements.is_empty() {
        return TargetResourcePlan {
            target_id: candidate.target_id().clone(),
            capability_disposition: candidate.disposition(),
            disposition: TargetResourceDisposition::NoResourceRequirements,
            snapshot_id: Some(snapshot.snapshot_id),
            dimensions: Vec::new(),
            policy_obligations: Vec::new(),
            evidence_error: None,
        };
    }

    let mut groups: BTreeMap<
        (String, Option<ResourceUnit>),
        DemandAccumulator,
    > = BTreeMap::new();

    let mut units_by_kind: BTreeMap<
        String,
        BTreeSet<Option<ResourceUnit>>,
    > = BTreeMap::new();

    let mut policy_obligations = Vec::new();

    for (index, requirement) in requirements.iter().enumerate() {
        let kind_name = requirement.kind().as_str().to_owned();
        let intent = requirement.intent();
        let class = ResourceDemandClass::from_intent(intent);

        match class {
            ResourceDemandClass::Policy | ResourceDemandClass::Unknown => {
                policy_obligations.push(ResourcePolicyObligation {
                    requirement_index: index,
                    resource_kind: kind_name,
                    intent,
                    requires_downstream_evaluation: !matches!(
                        intent,
                        ResourceIntent::Allowed
                    ),
                });

                continue;
            }

            ResourceDemandClass::Mandatory
            | ResourceDemandClass::Advisory => {}
        }

        let unit = requirement.unit().cloned();

        units_by_kind
            .entry(kind_name.clone())
            .or_default()
            .insert(unit.clone());

        let key = (kind_name, unit.clone());

        let accumulator = groups.entry(key).or_insert_with(|| {
            DemandAccumulator::new(
                requirement.kind().clone(),
                unit,
            )
        });

        if accumulator
            .add(class, requirement.quantity(), index)
            .is_err()
        {
            return invalid_target(
                candidate,
                snapshot.snapshot_id,
                "exact resource-demand aggregation failed",
            );
        }
    }

    let mut dimensions = Vec::new();

    let mut mandatory_insufficient = false;
    let mut mandatory_unresolved = false;
    let mut mandatory_invalid = false;
    let mut advisory_unmet = false;
    let mut advisory_unresolved = false;

    for ((kind_name, unit), accumulator) in groups {
        let multiple_units = units_by_kind
            .get(&kind_name)
            .is_some_and(|units| units.len() > 1);

        let assessment = assess_dimension(
            &accumulator,
            multiple_units,
            snapshot.resolver(),
        );

        mandatory_insufficient |= assessment
            .mandatory_disposition()
            .is_insufficient();

        mandatory_unresolved |= assessment
            .mandatory_disposition()
            .is_unresolved();

        mandatory_invalid |= assessment
            .mandatory_disposition()
            .is_invalid();

        advisory_unmet |= assessment
            .advisory_disposition()
            .is_insufficient();

        advisory_unresolved |= assessment
            .advisory_disposition()
            .is_unresolved();

        let _ = unit;
        dimensions.push(assessment);
    }

    let requires_policy_evaluation = policy_obligations
        .iter()
        .any(ResourcePolicyObligation::requires_downstream_evaluation);

    let disposition = if mandatory_invalid {
        TargetResourceDisposition::InvalidEvidence
    } else if mandatory_insufficient {
        TargetResourceDisposition::Infeasible
    } else if mandatory_unresolved
        || advisory_unresolved
        || requires_policy_evaluation
    {
        TargetResourceDisposition::Unresolved
    } else if advisory_unmet {
        TargetResourceDisposition::FeasibleWithUnmetPreferences
    } else {
        TargetResourceDisposition::Feasible
    };

    TargetResourcePlan {
        target_id: candidate.target_id().clone(),
        capability_disposition: candidate.disposition(),
        disposition,
        snapshot_id: Some(snapshot.snapshot_id),
        dimensions,
        policy_obligations,
        evidence_error: None,
    }
}

fn assess_dimension(
    accumulator: &DemandAccumulator,
    multiple_units: bool,
    resolver: &dyn ResourceConstraintResolver,
) -> ResourceDimensionAssessment {
    let mut assessment = ResourceDimensionAssessment {
        resource_kind: accumulator.kind.as_str().to_owned(),
        unit: accumulator.unit.clone(),
        mandatory_demand: accumulator.mandatory.clone(),
        advisory_demand: accumulator.advisory.clone(),
        available: None,
        mandatory_disposition: DemandDisposition::NotRequested,
        advisory_disposition: DemandDisposition::NotRequested,
        mandatory_requirement_indices: accumulator.mandatory_indices.clone(),
        advisory_requirement_indices: accumulator.advisory_indices.clone(),
        unresolved_requirement_indices: accumulator.unresolved_indices.clone(),
        explanation: String::new(),
    };

    if multiple_units {
        if accumulator.mandatory.is_some() || accumulator.mandatory_unknown {
            assessment.mandatory_disposition = DemandDisposition::Unresolved;
        }

        if accumulator.advisory.is_some() || accumulator.advisory_unknown {
            assessment.advisory_disposition = DemandDisposition::Unresolved;
        }

        assessment.explanation =
            "multiple unit identities exist for this resource kind; \
             exact cross-unit aggregation requires the canonical unit \
             conversion service"
                .to_owned();

        return assessment;
    }

    let available = match resolver.resource_value(&accumulator.kind) {
        Ok(ResourceValueResolution::Known(value)) => {
            assessment.available = Some(value.quantity().clone());
            value
        }

        Ok(ResourceValueResolution::Symbolic { .. })
        | Ok(ResourceValueResolution::Unavailable) => {
            if accumulator.mandatory.is_some() || accumulator.mandatory_unknown {
                assessment.mandatory_disposition = DemandDisposition::Unresolved;
            }

            if accumulator.advisory.is_some() || accumulator.advisory_unknown {
                assessment.advisory_disposition = DemandDisposition::Unresolved;
            }

            assessment.explanation =
                "resource capacity is symbolic or unavailable; \
                 absence of evidence is not zero capacity"
                    .to_owned();

            return assessment;
        }

        Err(message) => {
            if accumulator.mandatory.is_some() || accumulator.mandatory_unknown {
                assessment.mandatory_disposition = DemandDisposition::Invalid;
            }

            if accumulator.advisory.is_some() || accumulator.advisory_unknown {
                assessment.advisory_disposition = DemandDisposition::Invalid;
            }

            assessment.explanation =
                format!("resource evidence resolver failed: {message}");

            return assessment;
        }

        _ => {
            assessment.mandatory_disposition = DemandDisposition::Invalid;
            assessment.advisory_disposition = DemandDisposition::Invalid;
            assessment.explanation =
                "resource evidence returned an unsupported resolution state"
                    .to_owned();

            return assessment;
        }
    };

    if accumulator.mandatory_unknown {
        assessment.mandatory_disposition = DemandDisposition::Unresolved;
    } else if let Some(required) = &accumulator.mandatory {
        assessment.mandatory_disposition = compare_demand(
            resolver,
            required,
            accumulator.unit.as_ref(),
            available.quantity(),
            available.unit(),
        );
    }

    if accumulator.advisory_unknown {
        assessment.advisory_disposition = DemandDisposition::Unresolved;
    } else if let Some(preferred) = &accumulator.advisory {
        let total = match &accumulator.mandatory {
            Some(required) => add_exact_quantities(required, preferred),
            None => Ok(preferred.clone()),
        };

        match total {
            Ok(total) => {
                assessment.advisory_disposition = compare_demand(
                    resolver,
                    &total,
                    accumulator.unit.as_ref(),
                    available.quantity(),
                    available.unit(),
                );
            }

            Err(_) => {
                assessment.advisory_disposition = DemandDisposition::Invalid;
            }
        }
    }

    assessment.explanation = match (
        assessment.mandatory_disposition,
        assessment.advisory_disposition,
    ) {
        (DemandDisposition::Insufficient, _) => {
            "available capacity is below the mandatory demand".to_owned()
        }

        (DemandDisposition::Unresolved, _) => {
            "mandatory resource demand remains unresolved".to_owned()
        }

        (_, DemandDisposition::Insufficient) => {
            "mandatory demand fits, but the combined advisory demand does not"
                .to_owned()
        }

        (_, DemandDisposition::Unresolved) => {
            "advisory resource demand remains unresolved".to_owned()
        }

        _ => "resource evidence was evaluated".to_owned(),
    };

    assessment
}

fn compare_demand(
    resolver: &dyn ResourceConstraintResolver,
    demand: &ExactQuantity,
    demand_unit: Option<&ResourceUnit>,
    available: &ExactQuantity,
    available_unit: Option<&ResourceUnit>,
) -> DemandDisposition {
    match resolver.compare_quantities(
        demand,
        demand_unit,
        available,
        available_unit,
    ) {
        Ok(Some(Ordering::Less | Ordering::Equal)) => {
            DemandDisposition::Satisfied
        }

        Ok(Some(Ordering::Greater)) => {
            DemandDisposition::Insufficient
        }

        Ok(None) => DemandDisposition::Unresolved,

        Err(_) => DemandDisposition::Invalid,
    }
}

// ============================================================================
// EXACT DECIMAL ADDITION
// ============================================================================

/// Adds two canonical exact non-negative decimal quantities.
///
/// No machine-sized integer or floating-point conversion is performed.
fn add_exact_quantities(
    left: &ExactQuantity,
    right: &ExactQuantity,
) -> Result<ExactQuantity, ResourcePlanningError> {
    let (left_integer, left_fraction) = split_decimal(left.as_str());
    let (right_integer, right_fraction) = split_decimal(right.as_str());

    let scale = left_fraction.len().max(right_fraction.len());

    let mut left_scaled = left_integer.to_owned();
    left_scaled.push_str(left_fraction);
    left_scaled.extend(std::iter::repeat_n(
        '0',
        scale.saturating_sub(left_fraction.len()),
    ));

    let mut right_scaled = right_integer.to_owned();
    right_scaled.push_str(right_fraction);
    right_scaled.extend(std::iter::repeat_n(
        '0',
        scale.saturating_sub(right_fraction.len()),
    ));

    let integer_sum = add_unsigned_decimal_strings(
        &left_scaled,
        &right_scaled,
    );

    let combined = if scale == 0 {
        integer_sum
    } else if integer_sum.len() <= scale {
        let mut padded = String::with_capacity(scale + 1);
        padded.extend(std::iter::repeat_n(
            '0',
            scale + 1 - integer_sum.len(),
        ));
        padded.push_str(&integer_sum);
        padded
    } else {
        integer_sum
    };

    let canonical = if scale == 0 {
        combined
    } else {
        let split = combined.len() - scale;
        let integer = &combined[..split];
        let fraction = &combined[split..];

        let fraction = fraction.trim_end_matches('0');

        if fraction.is_empty() {
            integer.to_owned()
        } else {
            format!("{integer}.{fraction}")
        }
    };

    ExactQuantity::new(canonical).map_err(|error| {
        ResourcePlanningError::Arithmetic(error.to_string())
    })
}

fn split_decimal(value: &str) -> (&str, &str) {
    match value.split_once('.') {
        Some((integer, fraction)) => (integer, fraction),
        None => (value, ""),
    }
}

/// Adds two unsigned integer strings using decimal digit arithmetic.
fn add_unsigned_decimal_strings(
    left: &str,
    right: &str,
) -> String {
    let mut left_digits = left.bytes().rev();
    let mut right_digits = right.bytes().rev();

    let mut result = Vec::new();
    let mut carry = 0_u8;

    loop {
        let left_digit = left_digits.next();
        let right_digit = right_digits.next();

        if left_digit.is_none() && right_digit.is_none() && carry == 0 {
            break;
        }

        let left_value = left_digit
            .map(|digit| digit - b'0')
            .unwrap_or(0);

        let right_value = right_digit
            .map(|digit| digit - b'0')
            .unwrap_or(0);

        let sum = left_value + right_value + carry;

        result.push(b'0' + (sum % 10));
        carry = sum / 10;
    }

    result.reverse();

    String::from_utf8(result)
        .expect("decimal digit arithmetic produces valid ASCII")
}

// ============================================================================
// TARGET RESULT CONSTRUCTORS
// ============================================================================

fn skipped_target(
    candidate: &TargetCapabilityMatch,
) -> TargetResourcePlan {
    TargetResourcePlan {
        target_id: candidate.target_id().clone(),
        capability_disposition: candidate.disposition(),
        disposition: TargetResourceDisposition::SkippedCapabilityIneligible,
        snapshot_id: None,
        dimensions: Vec::new(),
        policy_obligations: Vec::new(),
        evidence_error: None,
    }
}

fn unavailable_target(
    candidate: &TargetCapabilityMatch,
) -> TargetResourcePlan {
    TargetResourcePlan {
        target_id: candidate.target_id().clone(),
        capability_disposition: candidate.disposition(),
        disposition: TargetResourceDisposition::EvidenceUnavailable,
        snapshot_id: None,
        dimensions: Vec::new(),
        policy_obligations: Vec::new(),
        evidence_error: None,
    }
}

fn failed_target(
    candidate: &TargetCapabilityMatch,
    error: ResourceEvidenceError,
) -> TargetResourcePlan {
    TargetResourcePlan {
        target_id: candidate.target_id().clone(),
        capability_disposition: candidate.disposition(),
        disposition: TargetResourceDisposition::EvidenceFailure,
        snapshot_id: None,
        dimensions: Vec::new(),
        policy_obligations: Vec::new(),
        evidence_error: Some(error.to_string()),
    }
}

fn invalid_target(
    candidate: &TargetCapabilityMatch,
    snapshot_id: String,
    message: &str,
) -> TargetResourcePlan {
    TargetResourcePlan {
        target_id: candidate.target_id().clone(),
        capability_disposition: candidate.disposition(),
        disposition: TargetResourceDisposition::InvalidEvidence,
        snapshot_id: Some(snapshot_id),
        dimensions: Vec::new(),
        policy_obligations: Vec::new(),
        evidence_error: Some(message.to_owned()),
    }
}

// ============================================================================
// ERRORS
// ============================================================================

/// Errors produced while constructing resource-planning contracts.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ResourcePlanningError {
    /// Snapshot identity was invalid.
    InvalidSnapshotId(String),

    /// Exact resource arithmetic failed.
    Arithmetic(String),
}

impl fmt::Display for ResourcePlanningError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidSnapshotId(message) => {
                write!(formatter, "invalid resource snapshot identity: {message}")
            }

            Self::Arithmetic(message) => {
                write!(formatter, "resource quantity arithmetic failed: {message}")
            }
        }
    }
}

impl Error for ResourcePlanningError {}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::frontend::ast::node::resources::resource_kind::ResourceKind;

    use crate::frontend::semantic::resources::constraints::{
        ResolvedResourceValue,
    };

    use crate::frontend::semantic::resources::model::{
        ResourceQuantity,
        ResourceUnit,
    };

    struct FixedResolver {
        kind: ResourceKind,
        value: ResourceValueResolution,
    }

    impl ResourceConstraintResolver for FixedResolver {
        fn resource_value(
            &self,
            kind: &ResourceKind,
        ) -> Result<ResourceValueResolution, String> {
            if kind == &self.kind {
                Ok(self.value.clone())
            } else {
                Ok(ResourceValueResolution::Unavailable)
            }
        }

        fn expression_value(
            &self,
            _expression: crate::frontend::ast::node::node_id::NodeId,
        ) -> Result<
            crate::frontend::semantic::resources::constraints::ExpressionValueResolution,
            String,
        > {
            Ok(
                crate::frontend::semantic::resources::constraints::ExpressionValueResolution::Symbolic,
            )
        }
    }

    fn kind() -> ResourceKind {
        ResourceKind::new("compute", "worker-slots")
            .expect("valid resource kind")
    }

    fn quantity(value: &str) -> ExactQuantity {
        ExactQuantity::new(value).expect("valid exact quantity")
    }

    fn requirement(
        amount: &str,
        intent: ResourceIntent,
    ) -> ResourceRequirement {
        ResourceRequirement::new(
            kind(),
            ResourceQuantity::Known(quantity(amount)),
            None,
            intent,
        )
        .expect("valid resource requirement")
    }

    fn resolver(
        available: &str,
    ) -> FixedResolver {
        FixedResolver {
            kind: kind(),
            value: ResourceValueResolution::Known(
                ResolvedResourceValue::new(
                    quantity(available),
                    None,
                ),
            ),
        }
    }

    #[test]
    fn exact_decimal_addition_does_not_use_bounded_integers() {
        let left = quantity(
            "999999999999999999999999999999999999999999999999.75",
        );

        let right = quantity("0.50");

        let sum = add_exact_quantities(&left, &right)
            .expect("exact addition should succeed");

        assert_eq!(
            sum.as_str(),
            "1000000000000000000000000000000000000000000000000.25"
        );
    }

    #[test]
    fn exact_addition_handles_fractional_carry() {
        let sum = add_exact_quantities(
            &quantity("1.95"),
            &quantity("0.15"),
        )
        .expect("exact addition should succeed");

        assert_eq!(sum.as_str(), "2.1");
    }

    #[test]
    fn exact_addition_handles_integer_carry() {
        let sum = add_exact_quantities(
            &quantity("999"),
            &quantity("1"),
        )
        .expect("exact addition should succeed");

        assert_eq!(sum.as_str(), "1000");
    }

    #[test]
    fn mandatory_demands_are_aggregated() {
        let mut accumulator = DemandAccumulator::new(
            kind(),
            None,
        );

        accumulator
            .add(
                ResourceDemandClass::Mandatory,
                &ResourceQuantity::Known(quantity("4")),
                0,
            )
            .expect("first demand");

        accumulator
            .add(
                ResourceDemandClass::Mandatory,
                &ResourceQuantity::Known(quantity("7")),
                1,
            )
            .expect("second demand");

        assert_eq!(
            accumulator.mandatory.expect("aggregate").as_str(),
            "11"
        );

        assert_eq!(accumulator.mandatory_indices, vec![0, 1]);
    }

    #[test]
    fn unknown_quantities_are_not_treated_as_zero() {
        let mut accumulator = DemandAccumulator::new(
            kind(),
            None,
        );

        accumulator
            .add(
                ResourceDemandClass::Mandatory,
                &ResourceQuantity::Symbolic(
                    crate::frontend::ast::node::node_id::NodeId::new(1),
                ),
                0,
            )
            .expect("symbolic demand should be retained");

        assert!(accumulator.mandatory_unknown);
        assert_eq!(accumulator.mandatory, None);
        assert_eq!(accumulator.unresolved_indices, vec![0]);
    }

    #[test]
    fn mandatory_and_advisory_intents_remain_distinct() {
        assert_eq!(
            ResourceDemandClass::from_intent(ResourceIntent::Required),
            ResourceDemandClass::Mandatory,
        );

        assert_eq!(
            ResourceDemandClass::from_intent(ResourceIntent::Preferred),
            ResourceDemandClass::Advisory,
        );

        assert_eq!(
            ResourceDemandClass::from_intent(ResourceIntent::Hint),
            ResourceDemandClass::Advisory,
        );

        assert_eq!(
            ResourceDemandClass::from_intent(ResourceIntent::Budget),
            ResourceDemandClass::Policy,
        );

        assert_eq!(
            ResourceDemandClass::from_intent(ResourceIntent::Forbidden),
            ResourceDemandClass::Policy,
        );
    }

    #[test]
    fn known_capacity_is_compared_without_integer_conversion() {
        let available = resolver("10");
        let required = quantity("9");

        let result = compare_demand(
            &available,
            &required,
            None,
            &quantity("10"),
            None,
        );

        assert_eq!(result, DemandDisposition::Satisfied);
    }

    #[test]
    fn insufficient_capacity_is_explicit() {
        let available = resolver("5");

        let result = compare_demand(
            &available,
            &quantity("8"),
            None,
            &quantity("5"),
            None,
        );

        assert_eq!(result, DemandDisposition::Insufficient);
    }

    #[test]
    fn different_units_are_not_assumed_compatible() {
        let available = resolver("100");

        let first = ResourceUnit::new("memory", "bytes")
            .expect("valid unit");

        let second = ResourceUnit::new("memory", "kilobytes")
            .expect("valid unit");

        let result = compare_demand(
            &available,
            Some(&first),
            &quantity("1"),
            Some(&second),
            &quantity("100"),
        );

        assert_eq!(result, DemandDisposition::Unresolved);
    }

    #[test]
    fn module_identity_is_stable() {
        assert_eq!(
            ResourcePlanner::module_name(),
            RESOURCE_PLANNING_MODULE_NAME,
        );

        assert_eq!(
            ResourcePlanner::api_version(),
            RESOURCE_PLANNING_API_VERSION,
        );
    }
}
