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
//! PURPOSE
//!
//! Produce target-scoped resource feasibility assessments from:
//!
//! - Canonical semantic ResourceRequirement values.
//! - Capability-matched target candidates.
//! - Explicit target-scoped resource evidence.
//! - Canonical semantic quantity resolution.
//! - Canonical unit comparison and conversion.
//!
//! OWNERSHIP
//!
//! This module owns abstract resource feasibility planning.
//!
//! It does not own semantic resource definitions, hardware discovery,
//! physical inventory, allocation, reservation, scheduling, routing,
//! lowering, or execution.
//!
//! AUTHORITATIVE DEPENDENCIES
//!
//! Target identity:
//!     crate::compiler::configuration::target
//!
//! Capability matching:
//!     crate::compiler::planning::capability_matching
//!
//! Resource semantics:
//!     crate::frontend::semantic::resources::model
//!
//! Resource resolution:
//!     crate::frontend::semantic::resources::constraints
//!
//! Operational budgets:
//!     crate::compiler::configuration::budget
//!
//! Physical resource management:
//!     crate::runtime and hardware/provider authorities
//!
//! SCALABILITY
//!
//! No arbitrary universal limits are imposed on resource quantities,
//! resource kinds, targets, devices, processors, or accelerators.
//!
//! Exact quantities remain arbitrary-length canonical decimal strings.
//!
//! Actual processing remains subject to available memory, execution time,
//! address-space limits, physical capacity and explicit operational budgets.
//!
//! Literal infinite computation is not promised.
//!
//! SEMANTIC INVARIANTS
//!
//! 1. Requirements remain mandatory.
//! 2. Preferences remain advisory.
//! 3. Hints remain advisory.
//! 4. Budgets remain policy obligations.
//! 5. Prohibitions remain policy obligations.
//! 6. Missing evidence is never unlimited capacity.
//! 7. Symbolic quantities are resolved through canonical semantic services.
//! 8. Incompatible units are never compared numerically without validation.
//! 9. Different targets never share resource evidence implicitly.
//! 10. Feasibility never implies allocation or execution.
//!
//! INTEGRATION
//!
//! Input:
//!     CapabilityMatchReport
//!     &[ResourceRequirement]
//!     ResourceEvidenceProvider
//!
//! Output:
//!     ResourcePlanningReport
//!
//! Downstream:
//!     planning/partitioning.rs
//!     planning/feasibility.rs
//!     planning/scheduling.rs
//!     deployment planning
//!     backend realization planning

use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;

use crate::compiler::configuration::target::TargetId;

use crate::compiler::planning::capability_matching::{
    CapabilityMatchReport,
    TargetCapabilityMatch,
    TargetMatchDisposition,
};

use crate::frontend::semantic::resources::constraints::{
    ExpressionValueResolution,
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

/// Stable architectural identity.
pub const RESOURCE_PLANNING_MODULE_NAME: &str =
    "zamani.compiler.planning.resource_planning";

/// Resource-planning API version.
///
/// Increment when making an incompatible public API change.
pub const RESOURCE_PLANNING_API_VERSION: u16 = 2;

// ============================================================================
// RESOURCE EVIDENCE
// ============================================================================

/// Immutable resource evidence for exactly one target.
///
/// A snapshot is associated with a target identity and a stable external
/// snapshot revision.
///
/// The resolver supplies observations, not reservations or allocations.
///
/// The snapshot must represent a consistent observation of the target's
/// resource state.
pub struct ResourceEvidenceSnapshot {
    target_id: TargetId,
    snapshot_id: String,
    resolver: Box<dyn ResourceConstraintResolver>,
}

impl ResourceEvidenceSnapshot {
    /// Creates a target-bound resource evidence snapshot.
    ///
    /// # Errors
    ///
    /// Returns an error when the snapshot identity is invalid.
    pub fn new<R>(
        target_id: TargetId,
        snapshot_id: impl Into<String>,
        resolver: R,
    ) -> Result<Self, ResourcePlanningError>
    where
        R: ResourceConstraintResolver + 'static,
    {
        Self::from_boxed(
            target_id,
            snapshot_id,
            Box::new(resolver),
        )
    }

    /// Creates a target-bound snapshot from an existing resolver.
    pub fn from_boxed(
        target_id: TargetId,
        snapshot_id: impl Into<String>,
        resolver: Box<dyn ResourceConstraintResolver>,
    ) -> Result<Self, ResourcePlanningError> {
        let snapshot_id = snapshot_id.into();

        if snapshot_id.trim().is_empty() {
            return Err(
                ResourcePlanningError::InvalidSnapshotId(
                    "snapshot identity cannot be empty".to_owned(),
                ),
            );
        }

        if snapshot_id.chars().any(char::is_control) {
            return Err(
                ResourcePlanningError::InvalidSnapshotId(
                    "snapshot identity contains control characters".to_owned(),
                ),
            );
        }

        Ok(Self {
            target_id,
            snapshot_id,
            resolver,
        })
    }

    /// Returns the target identity.
    #[must_use]
    pub fn target_id(&self) -> &TargetId {
        &self.target_id
    }

    /// Returns the snapshot revision.
    #[must_use]
    pub fn snapshot_id(&self) -> &str {
        &self.snapshot_id
    }

    /// Returns the canonical semantic resolver.
    #[must_use]
    pub fn resolver(&self) -> &dyn ResourceConstraintResolver {
        self.resolver.as_ref()
    }
}

// ============================================================================
// EVIDENCE PROVIDER
// ============================================================================

/// Supplies immutable resource evidence for a target.
///
/// Implementations may adapt:
///
/// - Hardware inventory providers.
/// - Runtime inventory observations.
/// - Quantum processor inventories.
/// - FPGA and ASIC resource descriptions.
/// - Accelerator inventories.
/// - Distributed resource inventories.
/// - Simulator capabilities and resource descriptions.
///
/// The provider must not allocate or reserve resources.
///
/// An absent snapshot means unavailable evidence, not zero capacity.
///
/// Providers must return a snapshot whose target identity matches the
/// requested target.
pub trait ResourceEvidenceProvider {
    /// Obtains a target-specific evidence snapshot.
    ///
    /// `Ok(None)` means that evidence is unavailable.
    ///
    /// `Err` means that trustworthy evidence could not be established.
    fn snapshot_for(
        &self,
        target: &TargetId,
    ) -> Result<Option<ResourceEvidenceSnapshot>, ResourceEvidenceError>;
}

/// Failure reported by a resource evidence provider.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResourceEvidenceError {
    /// Target associated with the failure.
    pub target: TargetId,

    /// Provider explanation.
    pub message: String,
}

impl ResourceEvidenceError {
    /// Constructs a provider error.
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
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            f,
            "resource evidence failed for target `{}`: {}",
            self.target,
            self.message
        )
    }
}

impl Error for ResourceEvidenceError {}

// ============================================================================
// REQUIREMENT CLASSIFICATION
// ============================================================================

/// Classification of semantic resource intent.
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
pub enum ResourceDemandClass {
    /// Mandatory quantitative demand.
    Mandatory,

    /// Advisory quantitative demand.
    Advisory,

    /// Policy obligation.
    Policy,

    /// Unknown future intent.
    Unknown,
}

impl ResourceDemandClass {
    /// Classifies canonical resource intent.
    #[must_use]
    pub const fn from_intent(intent: ResourceIntent) -> Self {
        match intent {
            ResourceIntent::Required => Self::Mandatory,

            ResourceIntent::Preferred
            | ResourceIntent::Hint => Self::Advisory,

            ResourceIntent::Allowed
            | ResourceIntent::Forbidden
            | ResourceIntent::Budget => Self::Policy,

            _ => Self::Unknown,
        }
    }
}

// ============================================================================
// DEMAND DISPOSITION
// ============================================================================

/// Result of assessing one quantitative demand.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    PartialEq,
)]
#[non_exhaustive]
pub enum DemandDisposition {
    /// Evidence proves sufficient capacity.
    Satisfied,

    /// Evidence proves insufficient capacity.
    Insufficient,

    /// Evidence or the quantity remains unresolved.
    Unresolved,

    /// Invalid input or evidence.
    Invalid,

    /// No demand of this class exists.
    NotRequested,
}

impl DemandDisposition {
    /// Whether the demand is proven satisfied.
    #[must_use]
    pub const fn is_satisfied(self) -> bool {
        matches!(self, Self::Satisfied)
    }

    /// Whether insufficient capacity is proven.
    #[must_use]
    pub const fn is_insufficient(self) -> bool {
        matches!(self, Self::Insufficient)
    }

    /// Whether more information is needed.
    #[must_use]
    pub const fn is_unresolved(self) -> bool {
        matches!(self, Self::Unresolved)
    }

    /// Whether the assessment is invalid.
    #[must_use]
    pub const fn is_invalid(self) -> bool {
        matches!(self, Self::Invalid)
    }
}

// ============================================================================
// TARGET DISPOSITION
// ============================================================================

/// Overall resource assessment for a target.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    PartialEq,
)]
#[non_exhaustive]
pub enum TargetResourceDisposition {
    /// Capability matching did not admit the target.
    SkippedCapabilityIneligible,

    /// No quantitative resource requirements exist.
    NoResourceRequirements,

    /// Mandatory demands are satisfied.
    Feasible,

    /// Mandatory demands are satisfied, but preferences are unmet.
    FeasibleWithUnmetPreferences,

    /// A mandatory resource demand is proven infeasible.
    Infeasible,

    /// Evidence is insufficient to decide.
    Unresolved,

    /// No resource evidence snapshot exists.
    EvidenceUnavailable,

    /// Evidence provider failed.
    EvidenceFailure,

    /// Invalid evidence or arithmetic.
    InvalidEvidence,
}

impl TargetResourceDisposition {
    /// Whether mandatory resource demands are proven satisfied.
    #[must_use]
    pub const fn mandatory_demands_satisfied(self) -> bool {
        matches!(
            self,
            Self::Feasible
                | Self::FeasibleWithUnmetPreferences
        )
    }

    /// Whether mandatory resource infeasibility is proven.
    #[must_use]
    pub const fn is_infeasible(self) -> bool {
        matches!(self, Self::Infeasible)
    }

    /// Whether the assessment remains incomplete.
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

    /// Whether resource feasibility has been established.
    ///
    /// This is not backend, scheduling, authorization, allocation,
    /// or execution feasibility.
    #[must_use]
    pub const fn is_resource_feasible(self) -> bool {
        self.mandatory_demands_satisfied()
    }
}

// ============================================================================
// RESOURCE DIMENSION
// ============================================================================

/// Assessment of one resource kind and unit combination.
///
/// Different unit identities are assessed independently. The canonical
/// resolver decides whether quantities are comparable or convertible.
///
/// Mandatory and advisory demands are kept separate.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResourceDimensionAssessment {
    resource_kind: String,

    unit: Option<ResourceUnit>,

    mandatory_demand: Option<ExactQuantity>,

    advisory_demand: Option<ExactQuantity>,

    available: Option<ExactQuantity>,

    available_unit: Option<ResourceUnit>,

    mandatory_disposition: DemandDisposition,

    advisory_disposition: DemandDisposition,

    mandatory_requirement_indices: Vec<usize>,

    advisory_requirement_indices: Vec<usize>,

    unresolved_requirement_indices: Vec<usize>,

    explanation: String,
}

impl ResourceDimensionAssessment {
    /// Returns resource-kind identity.
    #[must_use]
    pub fn resource_kind(&self) -> &str {
        &self.resource_kind
    }

    /// Returns the requested unit.
    #[must_use]
    pub fn unit(&self) -> Option<&ResourceUnit> {
        self.unit.as_ref()
    }

    /// Returns mandatory demand.
    #[must_use]
    pub fn mandatory_demand(&self) -> Option<&ExactQuantity> {
        self.mandatory_demand.as_ref()
    }

    /// Returns advisory demand.
    #[must_use]
    pub fn advisory_demand(&self) -> Option<&ExactQuantity> {
        self.advisory_demand.as_ref()
    }

    /// Returns observed capacity.
    #[must_use]
    pub fn available(&self) -> Option<&ExactQuantity> {
        self.available.as_ref()
    }

    /// Returns the observed capacity unit.
    #[must_use]
    pub fn available_unit(&self) -> Option<&ResourceUnit> {
        self.available_unit.as_ref()
    }

    /// Returns mandatory disposition.
    #[must_use]
    pub const fn mandatory_disposition(&self) -> DemandDisposition {
        self.mandatory_disposition
    }

    /// Returns advisory disposition.
    #[must_use]
    pub const fn advisory_disposition(&self) -> DemandDisposition {
        self.advisory_disposition
    }

    /// Returns mandatory requirement indices.
    #[must_use]
    pub fn mandatory_requirement_indices(&self) -> &[usize] {
        &self.mandatory_requirement_indices
    }

    /// Returns advisory requirement indices.
    #[must_use]
    pub fn advisory_requirement_indices(&self) -> &[usize] {
        &self.advisory_requirement_indices
    }

    /// Returns unresolved requirement indices.
    #[must_use]
    pub fn unresolved_requirement_indices(&self) -> &[usize] {
        &self.unresolved_requirement_indices
    }

    /// Returns the explanation.
    #[must_use]
    pub fn explanation(&self) -> &str {
        &self.explanation
    }
}

// ============================================================================
// POLICY OBLIGATIONS
// ============================================================================

/// Preserved semantic policy obligation.
///
/// This planner does not reinterpret budgets, prohibitions or permissions
/// as ordinary minimum resource demands.
///
/// The complete original information is retained for downstream evaluation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResourcePolicyObligation {
    requirement_index: usize,

    resource_kind: String,

    intent: ResourceIntent,

    quantity: ResourceQuantity,

    unit: Option<ResourceUnit>,

    source_node: Option<
        crate::frontend::ast::node::node_id::NodeId
    >,

    requires_downstream_evaluation: bool,
}

impl ResourcePolicyObligation {
    /// Returns original requirement index.
    #[must_use]
    pub const fn requirement_index(&self) -> usize {
        self.requirement_index
    }

    /// Returns resource kind.
    #[must_use]
    pub fn resource_kind(&self) -> &str {
        &self.resource_kind
    }

    /// Returns original intent.
    #[must_use]
    pub const fn intent(&self) -> ResourceIntent {
        self.intent
    }

    /// Returns original quantity.
    #[must_use]
    pub fn quantity(&self) -> &ResourceQuantity {
        &self.quantity
    }

    /// Returns original unit.
    #[must_use]
    pub fn unit(&self) -> Option<&ResourceUnit> {
        self.unit.as_ref()
    }

    /// Returns original source node.
    #[must_use]
    pub const fn source_node(
        &self,
    ) -> Option<crate::frontend::ast::node::node_id::NodeId> {
        self.source_node
    }

    /// Whether another authority must evaluate this obligation.
    #[must_use]
    pub const fn requires_downstream_evaluation(&self) -> bool {
        self.requires_downstream_evaluation
    }
}

// ============================================================================
// TARGET PLAN
// ============================================================================

/// Complete resource assessment for one target.
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
    /// Returns target identity.
    #[must_use]
    pub fn target_id(&self) -> &TargetId {
        &self.target_id
    }

    /// Returns capability disposition.
    #[must_use]
    pub const fn capability_disposition(&self) -> TargetMatchDisposition {
        self.capability_disposition
    }

    /// Returns resource disposition.
    #[must_use]
    pub const fn disposition(&self) -> TargetResourceDisposition {
        self.disposition
    }

    /// Returns evidence snapshot identity.
    #[must_use]
    pub fn snapshot_id(&self) -> Option<&str> {
        self.snapshot_id.as_deref()
    }

    /// Returns dimension assessments.
    #[must_use]
    pub fn dimensions(&self) -> &[ResourceDimensionAssessment] {
        &self.dimensions
    }

    /// Returns downstream policy obligations.
    #[must_use]
    pub fn policy_obligations(&self) -> &[ResourcePolicyObligation] {
        &self.policy_obligations
    }

    /// Returns provider failure information.
    #[must_use]
    pub fn evidence_error(&self) -> Option<&str> {
        self.evidence_error.as_deref()
    }

    /// Whether mandatory resource demands are satisfied.
    #[must_use]
    pub const fn is_resource_feasible(&self) -> bool {
        self.disposition.is_resource_feasible()
    }

    /// Whether mandatory demands are proven infeasible.
    #[must_use]
    pub const fn is_infeasible(&self) -> bool {
        self.disposition.is_infeasible()
    }

    /// Whether the assessment is unresolved.
    #[must_use]
    pub const fn is_unresolved(&self) -> bool {
        self.disposition.is_unresolved()
    }
}

// ============================================================================
// COMPLETE REPORT
// ============================================================================

/// Complete resource-planning report.
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

    /// Returns target plans in capability-report order.
    #[must_use]
    pub fn targets(&self) -> &[TargetResourcePlan] {
        &self.targets
    }

    /// Returns proven feasible targets.
    pub fn feasible_targets(
        &self,
    ) -> impl Iterator<Item = &TargetResourcePlan> {
        self.targets
            .iter()
            .filter(|target| target.is_resource_feasible())
    }

    /// Returns proven infeasible targets.
    pub fn infeasible_targets(
        &self,
    ) -> impl Iterator<Item = &TargetResourcePlan> {
        self.targets
            .iter()
            .filter(|target| target.is_infeasible())
    }

    /// Returns unresolved targets.
    pub fn unresolved_targets(
        &self,
    ) -> impl Iterator<Item = &TargetResourcePlan> {
        self.targets
            .iter()
            .filter(|target| target.is_unresolved())
    }

    /// Finds a target by identity.
    #[must_use]
    pub fn target(
        &self,
        id: &TargetId,
    ) -> Option<&TargetResourcePlan> {
        self.targets
            .iter()
            .find(|target| target.target_id() == id)
    }

    /// Returns target count.
    #[must_use]
    pub fn len(&self) -> usize {
        self.targets.len()
    }

    /// Returns whether the report contains no targets.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.targets.is_empty()
    }

    /// Returns original requirement count.
    #[must_use]
    pub const fn requirement_count(&self) -> usize {
        self.requirement_count
    }

    /// Returns discovery completeness.
    #[must_use]
    pub const fn discovery_is_complete(&self) -> bool {
        self.discovery_complete
    }

    /// Whether every eligible target has a decisive assessment and
    /// discovery is complete.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.discovery_complete
            && self.targets.iter().all(|target| {
                !target.is_unresolved()
            })
    }

    /// Whether at least one target is proven resource-feasible.
    #[must_use]
    pub fn has_feasible_target(&self) -> bool {
        self.feasible_targets().next().is_some()
    }
}

// ============================================================================
// PLANNER
// ============================================================================

/// Stateless resource-planning service.
#[derive(Clone, Copy, Debug, Default)]
pub struct ResourcePlanner;

impl ResourcePlanner {
    /// Creates a planner.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Plans all capability-matched targets.
    ///
    /// Provider failures are retained per target. They do not erase
    /// successful assessments for other targets.
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
            let result = if !candidate.is_eligible_for_planning() {
                skipped_target(candidate)
            } else {
                match evidence_provider.snapshot_for(
                    candidate.target_id(),
                ) {
                    Ok(Some(snapshot)) => {
                        plan_target_with_snapshot(
                            candidate,
                            requirements,
                            snapshot,
                        )
                    }

                    Ok(None) => unavailable_target(candidate),

                    Err(error) => {
                        failed_target(candidate, error)
                    }
                }
            };

            report.targets.push(result);
        }

        report
    }

    /// Plans one capability-matched target.
    ///
    /// The supplied evidence snapshot must belong to that target.
    pub fn plan_target(
        &self,
        candidate: &TargetCapabilityMatch,
        requirements: &[ResourceRequirement],
        snapshot: ResourceEvidenceSnapshot,
    ) -> TargetResourcePlan {
        if !candidate.is_eligible_for_planning() {
            return skipped_target(candidate);
        }

        plan_target_with_snapshot(
            candidate,
            requirements,
            snapshot,
        )
    }

    /// Returns module identity.
    #[must_use]
    pub const fn module_name() -> &'static str {
        RESOURCE_PLANNING_MODULE_NAME
    }

    /// Returns API version.
    #[must_use]
    pub const fn api_version() -> u16 {
        RESOURCE_PLANNING_API_VERSION
    }
}

// ============================================================================
// INTERNAL QUANTITY RESOLUTION
// ============================================================================

#[derive(Clone, Debug)]
enum QuantityResolution {
    Known(ExactQuantity),

    Unresolved,

    Invalid(String),
}

fn resolve_quantity(
    quantity: &ResourceQuantity,
    resolver: &dyn ResourceConstraintResolver,
) -> QuantityResolution {
    match quantity {
        ResourceQuantity::Known(value) => {
            QuantityResolution::Known(value.clone())
        }

        ResourceQuantity::Symbolic(expression) => {
            match resolver.expression_value(*expression) {
                Ok(ExpressionValueResolution::Known(value)) => {
                    QuantityResolution::Known(value)
                }

                Ok(ExpressionValueResolution::Symbolic)
                | Ok(ExpressionValueResolution::Runtime) => {
                    QuantityResolution::Unresolved
                }

                Ok(ExpressionValueResolution::Invalid(message)) => {
                    QuantityResolution::Invalid(message)
                }

                Err(message) => {
                    QuantityResolution::Invalid(message)
                }

                _ => QuantityResolution::Unresolved,
            }
        }

        ResourceQuantity::Runtime(_) => {
            QuantityResolution::Unresolved
        }

        ResourceQuantity::Unspecified => {
            QuantityResolution::Unresolved
        }

        _ => QuantityResolution::Unresolved,
    }
}

// ============================================================================
// DEMAND ACCUMULATION
// ============================================================================

#[derive(Clone)]
struct DemandAccumulator {
    kind: ResourceKind,

    unit: Option<ResourceUnit>,

    mandatory: Option<ExactQuantity>,

    advisory: Option<ExactQuantity>,

    mandatory_unknown: bool,

    advisory_unknown: bool,

    mandatory_invalid: bool,

    advisory_invalid: bool,

    mandatory_indices: Vec<usize>,

    advisory_indices: Vec<usize>,

    unresolved_indices: Vec<usize>,

    invalid_messages: Vec<String>,
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
            mandatory_invalid: false,
            advisory_invalid: false,
            mandatory_indices: Vec::new(),
            advisory_indices: Vec::new(),
            unresolved_indices: Vec::new(),
            invalid_messages: Vec::new(),
        }
    }

    fn add(
        &mut self,
        class: ResourceDemandClass,
        quantity: &ResourceQuantity,
        index: usize,
        resolver: &dyn ResourceConstraintResolver,
    ) {
        let destination = match class {
            ResourceDemandClass::Mandatory => &mut self.mandatory,

            ResourceDemandClass::Advisory => &mut self.advisory,

            ResourceDemandClass::Policy
            | ResourceDemandClass::Unknown => return,
        };

        let resolution = resolve_quantity(quantity, resolver);

        match class {
            ResourceDemandClass::Mandatory => {
                self.mandatory_indices.push(index);
            }

            ResourceDemandClass::Advisory => {
                self.advisory_indices.push(index);
            }

            _ => return,
        }

        match resolution {
            QuantityResolution::Known(value) => {
                match destination.take() {
                    Some(current) => {
                        match add_exact_quantities(&current, &value) {
                            Ok(total) => {
                                *destination = Some(total);
                            }

                            Err(error) => {
                                match class {
                                    ResourceDemandClass::Mandatory => {
                                        self.mandatory_invalid = true;
                                    }

                                    ResourceDemandClass::Advisory => {
                                        self.advisory_invalid = true;
                                    }

                                    _ => {}
                                }

                                self.invalid_messages.push(
                                    error.to_string(),
                                );
                            }
                        }
                    }

                    None => {
                        *destination = Some(value);
                    }
                }
            }

            QuantityResolution::Unresolved => {
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
            }

            QuantityResolution::Invalid(message) => {
                match class {
                    ResourceDemandClass::Mandatory => {
                        self.mandatory_invalid = true;
                    }

                    ResourceDemandClass::Advisory => {
                        self.advisory_invalid = true;
                    }

                    _ => {}
                }

                self.invalid_messages.push(message);
                self.unresolved_indices.push(index);
            }
        }
    }
}

// ============================================================================
// TARGET PLANNING
// ============================================================================

fn plan_target_with_snapshot(
    candidate: &TargetCapabilityMatch,
    requirements: &[ResourceRequirement],
    snapshot: ResourceEvidenceSnapshot,
) -> TargetResourcePlan {
    if snapshot.target_id() != candidate.target_id() {
        return invalid_target(
            candidate,
            snapshot.snapshot_id.clone(),
            "resource evidence belongs to a different target",
        );
    }

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

    let mut policy_obligations = Vec::new();

    for (index, requirement) in requirements.iter().enumerate() {
        let kind = requirement.kind().clone();

        let kind_name = kind.as_str().to_owned();

        let intent = requirement.intent();

        let class = ResourceDemandClass::from_intent(intent);

        if matches!(
            class,
            ResourceDemandClass::Policy
                | ResourceDemandClass::Unknown
        ) {
            policy_obligations.push(
                ResourcePolicyObligation {
                    requirement_index: index,
                    resource_kind: kind_name,
                    intent,
                    quantity: requirement.quantity().clone(),
                    unit: requirement.unit().cloned(),
                    source_node: requirement.source_node(),

                    requires_downstream_evaluation: !matches!(
                        intent,
                        ResourceIntent::Allowed
                    ),
                },
            );

            continue;
        }

        let unit = requirement.unit().cloned();

        let key = (kind_name, unit.clone());

        let accumulator = groups
            .entry(key)
            .or_insert_with(|| {
                DemandAccumulator::new(kind, unit)
            });

        accumulator.add(
            class,
            requirement.quantity(),
            index,
            snapshot.resolver(),
        );
    }

    let mut dimensions = Vec::new();

    let mut mandatory_insufficient = false;

    let mut mandatory_unresolved = false;

    let mut mandatory_invalid = false;

    let mut advisory_unmet = false;

    let mut advisory_unresolved = false;

    for accumulator in groups.into_values() {
        let assessment = assess_dimension(
            &accumulator,
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

        dimensions.push(assessment);
    }

    let requires_policy_evaluation = policy_obligations
        .iter()
        .any(
            ResourcePolicyObligation::requires_downstream_evaluation,
        );

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
    } else if dimensions.is_empty() && !requires_policy_evaluation {
        TargetResourceDisposition::Feasible
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

// ============================================================================
// DIMENSION ASSESSMENT
// ============================================================================

fn assess_dimension(
    accumulator: &DemandAccumulator,
    resolver: &dyn ResourceConstraintResolver,
) -> ResourceDimensionAssessment {
    let mut assessment = ResourceDimensionAssessment {
        resource_kind: accumulator.kind.as_str().to_owned(),

        unit: accumulator.unit.clone(),

        mandatory_demand: accumulator.mandatory.clone(),

        advisory_demand: accumulator.advisory.clone(),

        available: None,

        available_unit: None,

        mandatory_disposition: DemandDisposition::NotRequested,

        advisory_disposition: DemandDisposition::NotRequested,

        mandatory_requirement_indices:
            accumulator.mandatory_indices.clone(),

        advisory_requirement_indices:
            accumulator.advisory_indices.clone(),

        unresolved_requirement_indices:
            accumulator.unresolved_indices.clone(),

        explanation: String::new(),
    };

    if accumulator.mandatory_invalid {
        assessment.mandatory_disposition =
            DemandDisposition::Invalid;
    }

    if accumulator.advisory_invalid {
        assessment.advisory_disposition =
            DemandDisposition::Invalid;
    }

    if accumulator.mandatory_unknown {
        assessment.mandatory_disposition =
            DemandDisposition::Unresolved;
    }

    if accumulator.advisory_unknown {
        assessment.advisory_disposition =
            DemandDisposition::Unresolved;
    }

    let needs_capacity = accumulator.mandatory.is_some()
        || accumulator.advisory.is_some()
        || accumulator.mandatory_unknown
        || accumulator.advisory_unknown;

    if !needs_capacity {
        assessment.explanation =
            "no quantitative capacity demand exists".to_owned();

        return assessment;
    }

    let available = match resolver.resource_value(&accumulator.kind) {
        Ok(ResourceValueResolution::Known(value)) => {
            assessment.available = Some(value.quantity().clone());

            assessment.available_unit = value.unit().cloned();

            value
        }

        Ok(ResourceValueResolution::Symbolic { .. })
        | Ok(ResourceValueResolution::Unavailable) => {
            if accumulator.mandatory.is_some()
                || accumulator.mandatory_unknown
            {
                assessment.mandatory_disposition =
                    DemandDisposition::Unresolved;
            }

            if accumulator.advisory.is_some()
                || accumulator.advisory_unknown
            {
                assessment.advisory_disposition =
                    DemandDisposition::Unresolved;
            }

            assessment.explanation =
                "resource capacity is unavailable or symbolic; \
                 missing evidence is not unlimited capacity"
                    .to_owned();

            return assessment;
        }

        Err(message) => {
            if accumulator.mandatory.is_some()
                || accumulator.mandatory_unknown
            {
                assessment.mandatory_disposition =
                    DemandDisposition::Invalid;
            }

            if accumulator.advisory.is_some()
                || accumulator.advisory_unknown
            {
                assessment.advisory_disposition =
                    DemandDisposition::Invalid;
            }

            assessment.explanation =
                format!("resource evidence resolution failed: {message}");

            return assessment;
        }

        _ => {
            assessment.mandatory_disposition =
                DemandDisposition::Invalid;

            assessment.advisory_disposition =
                DemandDisposition::Invalid;

            assessment.explanation =
                "unsupported resource evidence state".to_owned();

            return assessment;
        }
    };

    // Mandatory demands and advisory preferences are evaluated separately.
    //
    // A preference must never increase or modify mandatory demand.

    if !accumulator.mandatory_invalid
        && !accumulator.mandatory_unknown
    {
        if let Some(required) = &accumulator.mandatory {
            assessment.mandatory_disposition = compare_demand(
                resolver,
                required,
                accumulator.unit.as_ref(),
                available.quantity(),
                available.unit(),
            );
        }
    }

    if !accumulator.advisory_invalid
        && !accumulator.advisory_unknown
    {
        if let Some(preferred) = &accumulator.advisory {
            assessment.advisory_disposition = compare_demand(
                resolver,
                preferred,
                accumulator.unit.as_ref(),
                available.quantity(),
                available.unit(),
            );
        }
    }

    assessment.explanation = match (
        assessment.mandatory_disposition,
        assessment.advisory_disposition,
    ) {
        (DemandDisposition::Invalid, _) => {
            "mandatory demand could not be evaluated".to_owned()
        }

        (DemandDisposition::Insufficient, _) => {
            "available capacity is below mandatory demand".to_owned()
        }

        (DemandDisposition::Unresolved, _) => {
            "mandatory demand remains unresolved".to_owned()
        }

        (_, DemandDisposition::Invalid) => {
            "advisory preference could not be evaluated".to_owned()
        }

        (_, DemandDisposition::Insufficient) => {
            "mandatory demand is satisfied or unrequested, \
             but the advisory preference is unmet"
                .to_owned()
        }

        (_, DemandDisposition::Unresolved) => {
            "advisory preference remains unresolved".to_owned()
        }

        _ => {
            "resource evidence was evaluated".to_owned()
        }
    };

    assessment
}

// ============================================================================
// CANONICAL QUANTITY COMPARISON
// ============================================================================

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

        Ok(None) => {
            DemandDisposition::Unresolved
        }

        Err(_) => {
            DemandDisposition::Invalid
        }
    }
}

// ============================================================================
// EXACT DECIMAL ARITHMETIC
// ============================================================================

/// Adds two canonical non-negative decimal quantities.
///
/// No floating-point or machine-sized integer conversion occurs.
fn add_exact_quantities(
    left: &ExactQuantity,
    right: &ExactQuantity,
) -> Result<ExactQuantity, ResourcePlanningError> {
    let (left_integer, left_fraction) =
        split_decimal(left.as_str());

    let (right_integer, right_fraction) =
        split_decimal(right.as_str());

    let scale = left_fraction.len().max(right_fraction.len());

    let mut left_scaled = left_integer.to_owned();

    left_scaled.push_str(left_fraction);

    left_scaled.extend(
        std::iter::repeat('0')
            .take(scale.saturating_sub(left_fraction.len())),
    );

    let mut right_scaled = right_integer.to_owned();

    right_scaled.push_str(right_fraction);

    right_scaled.extend(
        std::iter::repeat('0')
            .take(scale.saturating_sub(right_fraction.len())),
    );

    let integer_sum = add_unsigned_decimal_strings(
        &left_scaled,
        &right_scaled,
    )?;

    let combined = if scale == 0 {
        integer_sum
    } else if integer_sum.len() <= scale {
        let mut padded = String::with_capacity(
            scale.saturating_add(1),
        );

        padded.extend(
            std::iter::repeat('0')
                .take(scale + 1 - integer_sum.len()),
        );

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

/// Adds unsigned decimal integer strings.
fn add_unsigned_decimal_strings(
    left: &str,
    right: &str,
) -> Result<String, ResourcePlanningError> {
    let mut left_digits = left.bytes().rev();

    let mut right_digits = right.bytes().rev();

    let mut result = Vec::new();

    let mut carry = 0_u8;

    loop {
        let left_digit = left_digits.next();

        let right_digit = right_digits.next();

        if left_digit.is_none()
            && right_digit.is_none()
            && carry == 0
        {
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

    String::from_utf8(result).map_err(|error| {
        ResourcePlanningError::Arithmetic(error.to_string())
    })
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

        disposition:
            TargetResourceDisposition::SkippedCapabilityIneligible,

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

        disposition:
            TargetResourceDisposition::EvidenceUnavailable,

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
    let message = if error.target != *candidate.target_id() {
        format!(
            "provider reported an error for a different target: {error}"
        )
    } else {
        error.to_string()
    };

    TargetResourcePlan {
        target_id: candidate.target_id().clone(),

        capability_disposition: candidate.disposition(),

        disposition:
            TargetResourceDisposition::EvidenceFailure,

        snapshot_id: None,

        dimensions: Vec::new(),

        policy_obligations: Vec::new(),

        evidence_error: Some(message),
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

        disposition:
            TargetResourceDisposition::InvalidEvidence,

        snapshot_id: Some(snapshot_id),

        dimensions: Vec::new(),

        policy_obligations: Vec::new(),

        evidence_error: Some(message.to_owned()),
    }
}

// ============================================================================
// ERRORS
// ============================================================================

/// Errors produced while constructing planning contracts.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ResourcePlanningError {
    /// Invalid evidence snapshot identity.
    InvalidSnapshotId(String),

    /// Exact arithmetic failed.
    Arithmetic(String),
}

impl fmt::Display for ResourcePlanningError {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidSnapshotId(message) => {
                write!(
                    f,
                    "invalid resource snapshot identity: {message}"
                )
            }

            Self::Arithmetic(message) => {
                write!(
                    f,
                    "resource arithmetic failed: {message}"
                )
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
        ExpressionValueResolution,
        ResolvedResourceValue,
    };

    struct FixedResolver {
        kind: ResourceKind,
        available: ResourceValueResolution,
        expression: ExpressionValueResolution,
    }

    impl ResourceConstraintResolver for FixedResolver {
        fn resource_value(
            &self,
            kind: &ResourceKind,
        ) -> Result<ResourceValueResolution, String> {
            if kind == &self.kind {
                Ok(self.available.clone())
            } else {
                Ok(ResourceValueResolution::Unavailable)
            }
        }

        fn expression_value(
            &self,
            _expression: crate::frontend::ast::node::node_id::NodeId,
        ) -> Result<ExpressionValueResolution, String> {
            Ok(self.expression.clone())
        }
    }

    fn kind() -> ResourceKind {
        ResourceKind::new("compute", "worker-slots")
            .expect("valid resource kind")
    }

    fn quantity(value: &str) -> ExactQuantity {
        ExactQuantity::new(value)
            .expect("valid exact quantity")
    }

    fn fixed_resolver(
        available: &str,
    ) -> FixedResolver {
        FixedResolver {
            kind: kind(),

            available: ResourceValueResolution::Known(
                ResolvedResourceValue::new(
                    quantity(available),
                    None,
                ),
            ),

            expression: ExpressionValueResolution::Symbolic,
        }
    }

    #[test]
    fn exact_decimal_addition_handles_large_values() {
        let left = quantity(
            "999999999999999999999999999999999999999999999999.75",
        );

        let right = quantity("0.50");

        let result = add_exact_quantities(&left, &right)
            .expect("exact addition");

        assert_eq!(
            result.as_str(),
            "1000000000000000000000000000000000000000000000000.25"
        );
    }

    #[test]
    fn exact_decimal_addition_handles_fractional_carry() {
        let result = add_exact_quantities(
            &quantity("1.95"),
            &quantity("0.15"),
        )
        .expect("exact addition");

        assert_eq!(result.as_str(), "2.1");
    }

    #[test]
    fn exact_decimal_addition_handles_integer_carry() {
        let result = add_exact_quantities(
            &quantity("999"),
            &quantity("1"),
        )
        .expect("exact addition");

        assert_eq!(result.as_str(), "1000");
    }

    #[test]
    fn requirements_remain_distinct_from_preferences() {
        assert_eq!(
            ResourceDemandClass::from_intent(
                ResourceIntent::Required
            ),
            ResourceDemandClass::Mandatory,
        );

        assert_eq!(
            ResourceDemandClass::from_intent(
                ResourceIntent::Preferred
            ),
            ResourceDemandClass::Advisory,
        );

        assert_eq!(
            ResourceDemandClass::from_intent(
                ResourceIntent::Hint
            ),
            ResourceDemandClass::Advisory,
        );
    }

    #[test]
    fn budgets_remain_policy_obligations() {
        assert_eq!(
            ResourceDemandClass::from_intent(
                ResourceIntent::Budget
            ),
            ResourceDemandClass::Policy,
        );

        assert_eq!(
            ResourceDemandClass::from_intent(
                ResourceIntent::Forbidden
            ),
            ResourceDemandClass::Policy,
        );
    }

    #[test]
    fn symbolic_quantities_use_canonical_expression_resolution() {
        let resolver = FixedResolver {
            kind: kind(),

            available: ResourceValueResolution::Unavailable,

            expression: ExpressionValueResolution::Known(
                quantity("128"),
            ),
        };

        let result = resolve_quantity(
            &ResourceQuantity::Symbolic(
                crate::frontend::ast::node::node_id::NodeId::new(1),
            ),
            &resolver,
        );

        assert!(matches!(
            result,
            QuantityResolution::Known(_)
        ));
    }

    #[test]
    fn unresolved_symbolic_quantities_are_not_zero() {
        let resolver = fixed_resolver("100");

        let result = resolve_quantity(
            &ResourceQuantity::Symbolic(
                crate::frontend::ast::node::node_id::NodeId::new(1),
            ),
            &resolver,
        );

        assert!(matches!(
            result,
            QuantityResolution::Unresolved
        ));
    }

    #[test]
    fn mandatory_capacity_comparison_is_exact() {
        let resolver = fixed_resolver("100");

        let result = compare_demand(
            &resolver,
            &quantity("99.99"),
            None,
            &quantity("100"),
            None,
        );

        assert_eq!(
            result,
            DemandDisposition::Satisfied
        );
    }

    #[test]
    fn insufficient_capacity_is_explicit() {
        let resolver = fixed_resolver("5");

        let result = compare_demand(
            &resolver,
            &quantity("8"),
            None,
            &quantity("5"),
            None,
        );

        assert_eq!(
            result,
            DemandDisposition::Insufficient
        );
    }

    #[test]
    fn unknown_unit_conversion_is_not_assumed() {
        let resolver = fixed_resolver("100");

        let demand_unit = ResourceUnit::new(
            "memory",
            "kilobytes",
        )
        .expect("valid unit");

        let available_unit = ResourceUnit::new(
            "memory",
            "bytes",
        )
        .expect("valid unit");

        let result = compare_demand(
            &resolver,
            &quantity("1"),
            Some(&demand_unit),
            &quantity("100"),
            Some(&available_unit),
        );

        assert_eq!(
            result,
            DemandDisposition::Unresolved
        );
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