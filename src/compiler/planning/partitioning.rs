#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Compiler — Heterogeneous Workload Partitioning
//!
//! Path: `src/compiler/planning/partitioning.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! ## PURPOSE
//!
//! Coordinate compiler-level heterogeneous workload partitioning by
//! integrating:
//!
//! - Canonical ZUIR operation identities.
//! - Operation-level capability candidate information.
//! - Compiler target discovery and capability matching.
//! - Target-scoped resource feasibility results.
//! - The existing distributed logical workload partitioner.
//!
//! The result is a collection of logical partitions. Each partition contains
//! work units that share at least one eligible target according to the
//! supplied evidence.
//!
//! This module does not select a particular target for a partition.
//!
//! ## AUTHORITATIVE OWNERS
//!
//! Canonical operation identities:
//!     crate::zuir::ids::OperationId
//!
//! Target identity:
//!     crate::compiler::configuration::target::TargetId
//!
//! Target discovery:
//!     crate::compiler::planning::target_discovery
//!
//! Capability matching:
//!     crate::compiler::planning::capability_matching
//!
//! Resource feasibility:
//!     crate::compiler::planning::resource_planning
//!
//! Generic workload partitioning:
//!     crate::distributed::planning::partition
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
//! Physical placement and execution:
//!     crate::runtime
//!     crate::distributed
//!     hardware-specific authorities
//!
//! ## OWNS
//!
//! - Compiler-facing partitioning input.
//! - Per-work-unit target eligibility filtering.
//! - Integration of capability and resource planning evidence.
//! - Compatibility-aware refinement of logical partitions.
//! - Per-partition common-target assessment.
//! - Cross-partition dependency reporting.
//! - Explicit reporting of unassignable work.
//! - Deterministic compiler partitioning results.
//!
//! ## DOES NOT OWN
//!
//! - Source-language semantics.
//! - AST or semantic resource models.
//! - ZUIR operation definitions.
//! - Capability definitions.
//! - Resource definitions.
//! - Hardware inventories.
//! - Physical resource allocation.
//! - Device reservation.
//! - Backend selection.
//! - Backend code generation.
//! - Runtime scheduling.
//! - Network routing.
//! - Quantum routing.
//! - Quantum error correction.
//! - Domain-specific operation decomposition.
//! - Execution.
//!
//! ## PARTITIONING MODEL
//!
//! The existing distributed partitioner first creates generic logical
//! partitions while preserving workload dependencies.
//!
//! This module then refines those partitions where necessary so that every
//! resulting partition has a non-empty intersection of eligible targets.
//!
//! A partition's candidate-target set is not a device assignment.
//!
//! Physical placement remains a separate downstream decision.
//!
//! ## POCO-REAF
//!
//! No universal maximum is imposed here on:
//!
//! - Work units.
//! - Dependencies.
//! - Logical partitions.
//! - Candidate targets.
//! - Computing domains.
//! - CPUs, GPUs, FPGAs, ASICs or QPUs.
//! - Distributed participants.
//! - Hardware architectures.
//! - Future computational technologies.
//!
//! The module uses the existing distributed partitioner's WorkEstimate
//! representation and policy contracts. Any arithmetic overflow reported
//! by that existing implementation remains an explicit error.
//!
//! Operational budgets are caller-supplied policies, not language limits.
//!
//! No finite process can promise literal infinity. The objective is to avoid
//! arbitrary universal ceilings and allow workloads to grow with available
//! implementation resources.
//!
//! ## DETERMINISM
//!
//! - Canonical OperationId ordering is used by the existing partitioner.
//! - Target candidates use ordered sets.
//! - Refinement follows deterministic partition and operation ordering.
//! - Locality ties are resolved deterministically.
//! - No random selection or global mutable state is used.
//!
//! ## EVIDENCE RULES
//!
//! - Missing capability evidence is not compatibility.
//! - Unknown mandatory resource evidence is not feasibility.
//! - A target absent from the capability report is not eligible.
//! - A target without a proven resource-feasible result is not admitted
//!   when a resource report is supplied.
//! - An incomplete discovery inventory remains incomplete.
//! - A partition without a common eligible target is not reported as
//!   successfully realizable.
//!
//! ## INTEGRATION
//!
//! The caller constructs PartitioningWorkload from canonical IR and
//! domain-specific analysis results.
//!
//! Each WorkItem must receive its candidate targets from the appropriate
//! operation-level capability analysis. The compiler-wide capability report
//! provides the additional global target-eligibility boundary.
//!
//! The caller may supply a ResourcePlanningReport to restrict candidates
//! to targets with proven resource feasibility.
//!
//! The result can be consumed by:
//!
//! - compiler feasibility analysis;
//! - compiler scheduling planning;
//! - data-movement planning;
//! - distributed placement;
//! - backend realization planning.
//!
//! It does not allocate, reserve, schedule, route, or execute resources.

use core::fmt;
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;

use crate::compiler::configuration::target::TargetId;

use crate::compiler::planning::capability_matching::CapabilityMatchReport;

use crate::compiler::planning::resource_planning::ResourcePlanningReport;

use crate::distributed::planning::partition::{
    partition_workload,
    Dependency,
    PartitionBudget,
    PartitionError,
    PartitionId,
    PartitionPlan,
    PartitionPolicy,
    WorkEstimate,
    WorkUnit,
    WorkloadGraph,
};

use crate::zuir::ids::OperationId;

// ============================================================================
// MODULE IDENTITY
// ============================================================================

/// Stable architectural identity of this module.
pub const PARTITIONING_MODULE_NAME: &str =
    "zamani.compiler.planning.partitioning";

/// Public API version of this module.
pub const PARTITIONING_API_VERSION: u16 = 1;

// ============================================================================
// WORK ITEM
// ============================================================================

/// One indivisible compiler work item associated with a canonical ZUIR
/// operation identity.
///
/// The operation identity is borrowed from the canonical IR authority.
/// This type does not define another operation representation.
///
/// `candidate_targets` must be populated from operation-level capability
/// analysis. It must not be inferred from an operation's display name,
/// dialect name, vendor name, or target architecture.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkItem {
    operation: OperationId,
    estimated_work: WorkEstimate,
    affinity: Option<String>,
    candidate_targets: BTreeSet<TargetId>,
}

impl WorkItem {
    /// Creates a compiler work item.
    ///
    /// `estimated_work` is an abstract, non-negative estimate compatible
    /// with the existing distributed partitioner's WorkEstimate contract.
    ///
    /// `candidate_targets` contains targets admitted by the appropriate
    /// operation-level capability analysis.
    #[must_use]
    pub fn new(
        operation: OperationId,
        estimated_work: WorkEstimate,
        candidate_targets: impl IntoIterator<Item = TargetId>,
    ) -> Self {
        Self {
            operation,
            estimated_work,
            affinity: None,
            candidate_targets: candidate_targets.into_iter().collect(),
        }
    }

    /// Sets an optional logical affinity key.
    ///
    /// Affinity is a grouping preference, not a mandatory placement
    /// constraint.
    #[must_use]
    pub fn with_affinity(
        mut self,
        affinity: impl Into<String>,
    ) -> Self {
        self.affinity = Some(affinity.into());
        self
    }

    /// Returns the canonical operation identity.
    #[must_use]
    pub const fn operation(&self) -> OperationId {
        self.operation
    }

    /// Returns the abstract work estimate.
    #[must_use]
    pub const fn estimated_work(&self) -> WorkEstimate {
        self.estimated_work
    }

    /// Returns the optional grouping affinity.
    #[must_use]
    pub fn affinity(&self) -> Option<&str> {
        self.affinity.as_deref()
    }

    /// Returns operation-level candidate targets.
    #[must_use]
    pub fn candidate_targets(&self) -> &BTreeSet<TargetId> {
        &self.candidate_targets
    }
}

// ============================================================================
// WORKLOAD
// ============================================================================

/// Compiler-facing heterogeneous workload graph.
///
/// Work items refer to canonical ZUIR operation identities. Dependencies
/// refer to those same identities.
///
/// The graph does not own or redefine operation semantics.
///
/// The constructor preserves the supplied collections. Structural graph
/// validation is delegated to the existing distributed partitioner.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PartitioningWorkload {
    work_items: Vec<WorkItem>,
    dependencies: Vec<Dependency<OperationId>>,
}

impl PartitioningWorkload {
    /// Creates a workload graph.
    #[must_use]
    pub fn new(
        work_items: Vec<WorkItem>,
        dependencies: Vec<Dependency<OperationId>>,
    ) -> Self {
        Self {
            work_items,
            dependencies,
        }
    }

    /// Returns all compiler work items.
    #[must_use]
    pub fn work_items(&self) -> &[WorkItem] {
        &self.work_items
    }

    /// Returns all workload dependencies.
    #[must_use]
    pub fn dependencies(&self) -> &[Dependency<OperationId>] {
        &self.dependencies
    }

    /// Returns whether the workload contains no work items.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.work_items.is_empty()
    }
}

// ============================================================================
// OPTIONS
// ============================================================================

/// Options for one compiler partitioning invocation.
///
/// The actual logical partitioning strategy and operational budgets are
/// delegated to the existing distributed partitioning subsystem.
///
/// No default hardware-count or partition-count ceiling is introduced.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PartitioningOptions {
    policy: PartitionPolicy,
    budget: PartitionBudget,
}

impl PartitioningOptions {
    /// Creates options using the supplied logical partitioning policy
    /// and operational budget.
    #[must_use]
    pub const fn new(
        policy: PartitionPolicy,
        budget: PartitionBudget,
    ) -> Self {
        Self { policy, budget }
    }

    /// Creates options without additional operational budgets.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self {
            policy: PartitionPolicy::new(),
            budget: PartitionBudget::unlimited(),
        }
    }

    /// Returns the logical partitioning policy.
    #[must_use]
    pub const fn policy(self) -> PartitionPolicy {
        self.policy
    }

    /// Returns the operational budget.
    #[must_use]
    pub const fn budget(self) -> PartitionBudget {
        self.budget
    }
}

impl Default for PartitioningOptions {
    fn default() -> Self {
        Self::unlimited()
    }
}

// ============================================================================
// PARTITION SEGMENT IDENTITY
// ============================================================================

/// Identity of a compiler-refined logical partition.
///
/// A base partition is produced by the existing distributed partitioner.
/// The ordinal identifies a compatibility-aware segment within that base
/// partition.
///
/// This identity is local to one planning result. It is not a persistent
/// device identity or runtime allocation identifier.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PartitionSegmentId {
    base_partition: PartitionId,
    ordinal: usize,
}

impl PartitionSegmentId {
    /// Creates a partition-segment identity.
    #[must_use]
    pub const fn new(
        base_partition: PartitionId,
        ordinal: usize,
    ) -> Self {
        Self {
            base_partition,
            ordinal,
        }
    }

    /// Returns the generic distributed partition identity.
    #[must_use]
    pub const fn base_partition(self) -> PartitionId {
        self.base_partition
    }

    /// Returns the segment ordinal within the base partition.
    #[must_use]
    pub const fn ordinal(self) -> usize {
        self.ordinal
    }
}

impl fmt::Display for PartitionSegmentId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}.{}",
            self.base_partition,
            self.ordinal
        )
    }
}

// ============================================================================
// PARTITION SEGMENT
// ============================================================================

/// One logical partition after target-compatibility refinement.
///
/// Every segment has a non-empty candidate-target set.
///
/// The set represents possible realization targets, not a selected target.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PartitionSegment {
    id: PartitionSegmentId,
    work_items: Vec<OperationId>,
    candidate_targets: BTreeSet<TargetId>,
    estimated_work: WorkEstimate,
}

impl PartitionSegment {
    /// Returns the segment identity.
    #[must_use]
    pub const fn id(&self) -> PartitionSegmentId {
        self.id
    }

    /// Returns the original generic partition identity.
    #[must_use]
    pub const fn base_partition(&self) -> PartitionId {
        self.id.base_partition()
    }

    /// Returns all member operation identities.
    #[must_use]
    pub fn work_items(&self) -> &[OperationId] {
        &self.work_items
    }

    /// Returns targets that can satisfy the intersection of member
    /// operation capability requirements and supplied resource evidence.
    #[must_use]
    pub fn candidate_targets(&self) -> &BTreeSet<TargetId> {
        &self.candidate_targets
    }

    /// Returns the aggregate abstract work estimate.
    #[must_use]
    pub const fn estimated_work(&self) -> WorkEstimate {
        self.estimated_work
    }

    /// Returns whether the segment has at least one possible target.
    #[must_use]
    pub fn has_candidate_target(&self) -> bool {
        !self.candidate_targets.is_empty()
    }
}

// ============================================================================
// UNASSIGNED WORK
// ============================================================================

/// Why a work item could not be included in a target-compatible segment.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum UnassignedReason {
    /// The operation has no operation-level capability candidate.
    NoOperationCapabilityCandidate,

    /// None of the operation's candidates passed global capability matching.
    NoGloballyEligibleTarget,

    /// Candidate targets did not have proven resource feasibility.
    NoResourceFeasibleTarget,
}

impl fmt::Display for UnassignedReason {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::NoOperationCapabilityCandidate => {
                "operation has no capability-compatible target"
            }
            Self::NoGloballyEligibleTarget => {
                "no operation candidate passed global capability matching"
            }
            Self::NoResourceFeasibleTarget => {
                "no operation candidate has proven resource feasibility"
            }
        })
    }
}

/// One work item that could not be assigned to a target-compatible
/// logical partition.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnassignedWorkItem {
    operation: OperationId,
    reason: UnassignedReason,
}

impl UnassignedWorkItem {
    /// Returns the operation identity.
    #[must_use]
    pub const fn operation(&self) -> OperationId {
        self.operation
    }

    /// Returns the reason the work item was not assigned.
    #[must_use]
    pub const fn reason(&self) -> UnassignedReason {
        self.reason
    }
}

// ============================================================================
// DEPENDENCY BOUNDARIES
// ============================================================================

/// One dependency crossing two compiler partition segments.
///
/// The dependency is preserved. A downstream data-movement, scheduling,
/// routing, or placement stage determines how it is realized.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PartitionBoundary {
    source: OperationId,
    destination: OperationId,
    source_segment: PartitionSegmentId,
    destination_segment: PartitionSegmentId,
    communication_cost: WorkEstimate,
}

impl PartitionBoundary {
    /// Returns the source operation.
    #[must_use]
    pub const fn source(&self) -> OperationId {
        self.source
    }

    /// Returns the destination operation.
    #[must_use]
    pub const fn destination(&self) -> OperationId {
        self.destination
    }

    /// Returns the source partition segment.
    #[must_use]
    pub const fn source_segment(&self) -> PartitionSegmentId {
        self.source_segment
    }

    /// Returns the destination partition segment.
    #[must_use]
    pub const fn destination_segment(&self) -> PartitionSegmentId {
        self.destination_segment
    }

    /// Returns the supplied abstract communication cost.
    #[must_use]
    pub const fn communication_cost(&self) -> WorkEstimate {
        self.communication_cost
    }
}

/// One dependency involving work that could not be assigned.
///
/// The dependency is retained rather than silently discarded.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnresolvedDependency {
    source: OperationId,
    destination: OperationId,
    source_assigned: bool,
    destination_assigned: bool,
    communication_cost: WorkEstimate,
}

impl UnresolvedDependency {
    /// Returns the source operation.
    #[must_use]
    pub const fn source(&self) -> OperationId {
        self.source
    }

    /// Returns the destination operation.
    #[must_use]
    pub const fn destination(&self) -> OperationId {
        self.destination
    }

    /// Returns whether the source operation was assigned.
    #[must_use]
    pub const fn source_assigned(&self) -> bool {
        self.source_assigned
    }

    /// Returns whether the destination operation was assigned.
    #[must_use]
    pub const fn destination_assigned(&self) -> bool {
        self.destination_assigned
    }

    /// Returns the supplied abstract communication cost.
    #[must_use]
    pub const fn communication_cost(&self) -> WorkEstimate {
        self.communication_cost
    }
}

// ============================================================================
// REPORT DISPOSITION
// ============================================================================

/// Overall compiler partitioning disposition.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum PartitioningDisposition {
    /// Every work item has been assigned and the supplied discovery
    /// inventory is complete.
    Complete,

    /// Some work items were assigned, but others could not be assigned.
    Partial,

    /// No work item could be assigned to a target-compatible segment.
    NoCompatibleTarget,

    /// Every work item was assigned, but the target inventory was incomplete.
    IncompleteInventory,
}

impl PartitioningDisposition {
    /// Returns whether every work item has been assigned.
    ///
    /// This does not establish backend compatibility, scheduling feasibility,
    /// authorization, physical availability, or successful execution.
    #[must_use]
    pub const fn all_work_assigned(self) -> bool {
        matches!(
            self,
            Self::Complete | Self::IncompleteInventory
        )
    }

    /// Returns whether the result has a complete target inventory.
    #[must_use]
    pub const fn inventory_complete(self) -> bool {
        matches!(self, Self::Complete | Self::Partial | Self::NoCompatibleTarget)
    }
}

// ============================================================================
// COMPLETE REPORT
// ============================================================================

/// Complete compiler-side partitioning result.
///
/// `base_plan` is the structural workload partition plan produced by the
/// existing distributed partitioner.
///
/// `segments` are the compatibility-aware refinements of those partitions.
///
/// The report never claims that a particular physical target has been
/// selected or reserved.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PartitioningReport {
    base_plan: PartitionPlan<OperationId>,
    segments: Vec<PartitionSegment>,
    unassigned: Vec<UnassignedWorkItem>,
    boundaries: Vec<PartitionBoundary>,
    unresolved_dependencies: Vec<UnresolvedDependency>,
    disposition: PartitioningDisposition,
    inventory_complete: bool,
    work_item_count: usize,
    dependency_count: usize,
}

impl PartitioningReport {
    /// Returns the original generic logical partition plan.
    #[must_use]
    pub fn base_plan(&self) -> &PartitionPlan<OperationId> {
        &self.base_plan
    }

    /// Returns all target-compatible partition segments.
    #[must_use]
    pub fn segments(&self) -> &[PartitionSegment] {
        &self.segments
    }

    /// Returns all work items that could not be assigned.
    #[must_use]
    pub fn unassigned(&self) -> &[UnassignedWorkItem] {
        &self.unassigned
    }

    /// Returns dependencies crossing assigned partition segments.
    #[must_use]
    pub fn boundaries(&self) -> &[PartitionBoundary] {
        &self.boundaries
    }

    /// Returns dependencies involving at least one unassigned work item.
    #[must_use]
    pub fn unresolved_dependencies(&self) -> &[UnresolvedDependency] {
        &self.unresolved_dependencies
    }

    /// Returns the overall planning disposition.
    #[must_use]
    pub const fn disposition(&self) -> PartitioningDisposition {
        self.disposition
    }

    /// Returns whether target discovery was complete.
    #[must_use]
    pub const fn inventory_is_complete(&self) -> bool {
        self.inventory_complete
    }

    /// Returns the original number of work items.
    #[must_use]
    pub const fn work_item_count(&self) -> usize {
        self.work_item_count
    }

    /// Returns the original number of dependencies.
    #[must_use]
    pub const fn dependency_count(&self) -> usize {
        self.dependency_count
    }

    /// Returns whether all work items were assigned.
    #[must_use]
    pub fn is_fully_assigned(&self) -> bool {
        self.unassigned.is_empty()
            && self.segments.iter().map(|segment| segment.work_items.len()).sum::<usize>()
                == self.work_item_count
    }

    /// Returns whether every segment has at least one candidate target.
    #[must_use]
    pub fn every_segment_has_candidate_targets(&self) -> bool {
        self.segments
            .iter()
            .all(PartitionSegment::has_candidate_target)
    }

    /// Returns whether the report is complete.
    ///
    /// A complete partitioning report is not a complete deployment plan.
    /// Scheduling, routing, backend realization, authorization, and runtime
    /// resource revalidation remain separate.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.disposition == PartitioningDisposition::Complete
            && self.is_fully_assigned()
            && self.inventory_complete
            && self.every_segment_has_candidate_targets()
    }

    /// Returns whether any dependency crosses a partition boundary.
    #[must_use]
    pub fn has_cross_partition_dependencies(&self) -> bool {
        !self.boundaries.is_empty()
    }

    /// Returns whether any dependency could not be fully assigned.
    #[must_use]
    pub fn has_unresolved_dependencies(&self) -> bool {
        !self.unresolved_dependencies.is_empty()
    }

    /// Returns whether there is at least one assigned segment.
    #[must_use]
    pub fn has_segments(&self) -> bool {
        !self.segments.is_empty()
    }
}

// ============================================================================
// ERRORS
// ============================================================================

/// Errors produced by compiler partitioning.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum PartitioningError {
    /// The existing distributed workload partitioner rejected the graph
    /// or its configured policy.
    DistributedPartitioning(PartitionError),

    /// Multiple compiler work items used the same canonical operation ID.
    DuplicateWorkItemMetadata(OperationId),

    /// An operation from a base partition was absent from the compiler
    /// workload metadata.
    MissingWorkItemMetadata(OperationId),

    /// A checked aggregate estimate could not be represented.
    ArithmeticOverflow,
}

impl fmt::Display for PartitioningError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DistributedPartitioning(error) => {
                write!(formatter, "distributed logical partitioning failed: {error}")
            }

            Self::DuplicateWorkItemMetadata(operation) => {
                write!(
                    formatter,
                    "duplicate compiler work-item metadata for operation {operation:?}"
                )
            }

            Self::MissingWorkItemMetadata(operation) => {
                write!(
                    formatter,
                    "missing compiler work-item metadata for operation {operation:?}"
                )
            }

            Self::ArithmeticOverflow => {
                formatter.write_str(
                    "compiler partitioning estimate arithmetic overflowed",
                )
            }
        }
    }
}

impl Error for PartitioningError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::DistributedPartitioning(error) => Some(error),
            _ => None,
        }
    }
}

impl From<PartitionError> for PartitioningError {
    fn from(error: PartitionError) -> Self {
        Self::DistributedPartitioning(error)
    }
}

// ============================================================================
// INTERNAL SEGMENT BUILDER
// ============================================================================

struct SegmentBuilder {
    ordinal: usize,
    work_items: Vec<OperationId>,
    candidate_targets: BTreeSet<TargetId>,
    estimated_work: WorkEstimate,
}

impl SegmentBuilder {
    fn new(
        ordinal: usize,
        operation: OperationId,
        candidate_targets: BTreeSet<TargetId>,
        estimated_work: WorkEstimate,
    ) -> Self {
        Self {
            ordinal,
            work_items: vec![operation],
            candidate_targets,
            estimated_work,
        }
    }

    fn can_accept(
        &self,
        candidates: &BTreeSet<TargetId>,
    ) -> bool {
        self.candidate_targets
            .iter()
            .any(|target| candidates.contains(target))
    }

    fn common_targets(
        &self,
        candidates: &BTreeSet<TargetId>,
    ) -> BTreeSet<TargetId> {
        self.candidate_targets
            .intersection(candidates)
            .cloned()
            .collect()
    }

    fn add(
        &mut self,
        operation: OperationId,
        candidates: &BTreeSet<TargetId>,
        estimated_work: WorkEstimate,
    ) -> Result<(), PartitioningError> {
        let common_targets = self.common_targets(candidates);

        if common_targets.is_empty() {
            return Err(PartitioningError::ArithmeticOverflow);
        }

        self.estimated_work = self
            .estimated_work
            .checked_add(estimated_work)
            .ok_or(PartitioningError::ArithmeticOverflow)?;

        self.candidate_targets = common_targets;
        self.work_items.push(operation);

        Ok(())
    }

    fn finish(
        self,
        base_partition: PartitionId,
    ) -> PartitionSegment {
        PartitionSegment {
            id: PartitionSegmentId::new(
                base_partition,
                self.ordinal,
            ),
            work_items: self.work_items,
            candidate_targets: self.candidate_targets,
            estimated_work: self.estimated_work,
        }
    }
}

// ============================================================================
// PARTITIONER
// ============================================================================

/// Stateless compiler partitioning coordinator.
///
/// It integrates canonical capability and resource results with the
/// repository's existing distributed workload partitioner.
#[derive(Clone, Copy, Debug, Default)]
pub struct Partitioner;

impl Partitioner {
    /// Creates a compiler partitioner.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Partitions a workload using canonical capability matching and
    /// optional resource feasibility results.
    ///
    /// Every work item's candidate set is first intersected with the
    /// globally capability-eligible targets.
    ///
    /// If a resource report is supplied, candidates are further restricted
    /// to targets whose mandatory resource requirements are proven feasible.
    ///
    /// A missing resource report means that this invocation is not applying
    /// a resource-evidence filter. It does not mean that resource capacity
    /// has been proven unlimited.
    ///
    /// # Errors
    ///
    /// Returns an error if the generic workload graph is invalid, an
    /// operational budget is exceeded, or checked arithmetic fails.
    pub fn partition(
        &self,
        workload: &PartitioningWorkload,
        options: PartitioningOptions,
        capability_report: &CapabilityMatchReport,
        resource_report: Option<&ResourcePlanningReport>,
    ) -> Result<PartitioningReport, PartitioningError> {
        let capability_targets: BTreeSet<TargetId> =
            capability_report
                .satisfied_targets()
                .map(|candidate| candidate.target_id().clone())
                .collect();

        let resource_targets = resource_report.map(|report| {
            report
                .feasible_targets()
                .map(|target| target.target_id().clone())
                .collect::<BTreeSet<_>>()
        });

        let inventory_complete = capability_report.discovery_is_complete()
            && resource_report
                .map(ResourcePlanningReport::discovery_is_complete)
                .unwrap_or(true);

        self.partition_with_target_evidence(
            workload,
            options,
            &capability_targets,
            resource_targets.as_ref(),
            inventory_complete,
        )
    }

    /// Partitions a workload using already-evaluated target evidence.
    ///
    /// This lower-level entry point is useful when a higher-level compiler
    /// coordinator has already combined capability and resource reports.
    ///
    /// `capability_eligible_targets` must come from canonical capability
    /// matching.
    ///
    /// If `resource_feasible_targets` is supplied, it must contain only
    /// targets with proven resource feasibility.
    ///
    /// The caller must not include targets with unresolved mandatory
    /// capability or resource evidence.
    ///
    /// `inventory_complete` must accurately describe the completeness
    /// of the supplied target evidence.
    ///
    /// # Errors
    ///
    /// Returns an error if the workload graph or configured partitioning
    /// policy is invalid.
    pub fn partition_with_target_evidence(
        &self,
        workload: &PartitioningWorkload,
        options: PartitioningOptions,
        capability_eligible_targets: &BTreeSet<TargetId>,
        resource_feasible_targets: Option<&BTreeSet<TargetId>>,
        inventory_complete: bool,
    ) -> Result<PartitioningReport, PartitioningError> {
        let mut metadata = BTreeMap::new();

        for item in &workload.work_items {
            if metadata
                .insert(item.operation, item)
                .is_some()
            {
                return Err(
                    PartitioningError::DuplicateWorkItemMetadata(
                        item.operation,
                    ),
                );
            }
        }

        let generic_units = workload
            .work_items
            .iter()
            .map(|item| {
                WorkUnit::new(
                    item.operation,
                    item.estimated_work,
                    item.affinity.clone(),
                )
            })
            .collect();

        let generic_graph = WorkloadGraph::new(
            generic_units,
            workload.dependencies.clone(),
        );

        // Reuse the repository's canonical generic graph partitioner.
        // This preserves its validation, budgets, dependency accounting,
        // deterministic ordering, and structural output validation.
        let base_plan = partition_workload(
            &generic_graph,
            options.policy(),
            options.budget(),
        )?;

        let adjacency = build_adjacency(&workload.dependencies);

        let mut segments = Vec::new();
        let mut unassigned = Vec::new();

        let mut segment_by_operation =
            BTreeMap::<OperationId, PartitionSegmentId>::new();

        for base_partition in base_plan.partitions() {
            let mut builders: Vec<SegmentBuilder> = Vec::new();

            for operation in base_partition.units() {
                let item = metadata
                    .get(operation)
                    .copied()
                    .ok_or(
                        PartitioningError::MissingWorkItemMetadata(
                            *operation,
                        ),
                    )?;

                let eligible = eligible_targets_for_item(
                    item,
                    capability_eligible_targets,
                    resource_feasible_targets,
                );

                if eligible.is_empty() {
                    unassigned.push(UnassignedWorkItem {
                        operation: *operation,
                        reason: unassigned_reason(
                            item,
                            capability_eligible_targets,
                            resource_feasible_targets,
                        ),
                    });

                    continue;
                }

                let mut best: Option<(usize, WorkEstimate)> = None;

                for (index, builder) in builders.iter().enumerate() {
                    if !builder.can_accept(&eligible) {
                        continue;
                    }

                    let locality = locality_score(
                        *operation,
                        &builder.work_items,
                        &adjacency,
                    )?;

                    let replace = match best {
                        None => true,

                        Some((best_index, best_locality)) => {
                            locality > best_locality
                                || (
                                    locality == best_locality
                                        && index < best_index
                                )
                        }
                    };

                    if replace {
                        best = Some((index, locality));
                    }
                }

                match best {
                    Some((index, _)) => {
                        builders[index].add(
                            *operation,
                            &eligible,
                            item.estimated_work,
                        )?;
                    }

                    None => {
                        let ordinal = builders.len();

                        builders.push(SegmentBuilder::new(
                            ordinal,
                            *operation,
                            eligible,
                            item.estimated_work,
                        ));
                    }
                }
            }

            for builder in builders {
                let segment = builder.finish(base_partition.id());

                for operation in &segment.work_items {
                    if segment_by_operation
                        .insert(*operation, segment.id)
                        .is_some()
                    {
                        return Err(
                            PartitioningError::DuplicateWorkItemMetadata(
                                *operation,
                            ),
                        );
                    }
                }

                segments.push(segment);
            }
        }

        let mut boundaries = Vec::new();
        let mut unresolved_dependencies = Vec::new();

        for dependency in &workload.dependencies {
            let source = *dependency.source();
            let destination = *dependency.destination();

            let source_segment = segment_by_operation.get(&source).copied();
            let destination_segment =
                segment_by_operation.get(&destination).copied();

            match (source_segment, destination_segment) {
                (Some(source_id), Some(destination_id))
                    if source_id != destination_id =>
                {
                    boundaries.push(PartitionBoundary {
                        source,
                        destination,
                        source_segment: source_id,
                        destination_segment: destination_id,
                        communication_cost: dependency.communication_cost(),
                    });
                }

                (Some(_), Some(_)) => {
                    // The dependency remains internal to one segment.
                }

                _ => {
                    unresolved_dependencies.push(
                        UnresolvedDependency {
                            source,
                            destination,
                            source_assigned: source_segment.is_some(),
                            destination_assigned: destination_segment.is_some(),
                            communication_cost: dependency.communication_cost(),
                        },
                    );
                }
            }
        }

        let assigned_count = segment_by_operation.len();

        if assigned_count
            .checked_add(unassigned.len())
            .ok_or(PartitioningError::ArithmeticOverflow)?
            != workload.work_items.len()
        {
            return Err(
                PartitioningError::MissingWorkItemMetadata(
                    workload
                        .work_items
                        .iter()
                        .find(|item| {
                            !segment_by_operation.contains_key(&item.operation)
                                && !unassigned.iter().any(|entry| {
                                    entry.operation == item.operation
                                })
                        })
                        .map(|item| item.operation)
                        .unwrap_or_else(|| {
                            // This branch is only reachable if the report
                            // violates the workload accounting invariant.
                            workload.work_items[0].operation
                        }),
                ),
            );
        }

        let disposition = determine_disposition(
            workload.work_items.len(),
            assigned_count,
            unassigned.len(),
            inventory_complete,
        );

        Ok(PartitioningReport {
            base_plan,
            segments,
            unassigned,
            boundaries,
            unresolved_dependencies,
            disposition,
            inventory_complete,
            work_item_count: workload.work_items.len(),
            dependency_count: workload.dependencies.len(),
        })
    }

    /// Returns the stable module identity.
    #[must_use]
    pub const fn module_name() -> &'static str {
        PARTITIONING_MODULE_NAME
    }

    /// Returns the public API version.
    #[must_use]
    pub const fn api_version() -> u16 {
        PARTITIONING_API_VERSION
    }
}

// ============================================================================
// INTERNAL TARGET FILTERING
// ============================================================================

fn eligible_targets_for_item(
    item: &WorkItem,
    capability_targets: &BTreeSet<TargetId>,
    resource_targets: Option<&BTreeSet<TargetId>>,
) -> BTreeSet<TargetId> {
    let mut eligible = item
        .candidate_targets
        .intersection(capability_targets)
        .cloned()
        .collect::<BTreeSet<_>>();

    if let Some(resource_targets) = resource_targets {
        eligible.retain(|target| resource_targets.contains(target));
    }

    eligible
}

fn unassigned_reason(
    item: &WorkItem,
    capability_targets: &BTreeSet<TargetId>,
    resource_targets: Option<&BTreeSet<TargetId>>,
) -> UnassignedReason {
    if item.candidate_targets.is_empty() {
        return UnassignedReason::NoOperationCapabilityCandidate;
    }

    let globally_eligible = item
        .candidate_targets
        .intersection(capability_targets)
        .next()
        .is_some();

    if !globally_eligible {
        return UnassignedReason::NoGloballyEligibleTarget;
    }

    if resource_targets.is_some() {
        return UnassignedReason::NoResourceFeasibleTarget;
    }

    UnassignedReason::NoGloballyEligibleTarget
}

// ============================================================================
// INTERNAL GRAPH HELPERS
// ============================================================================

fn build_adjacency(
    dependencies: &[Dependency<OperationId>],
) -> BTreeMap<OperationId, Vec<(OperationId, WorkEstimate)>> {
    let mut adjacency =
        BTreeMap::<OperationId, Vec<(OperationId, WorkEstimate)>>::new();

    for dependency in dependencies {
        let source = *dependency.source();
        let destination = *dependency.destination();
        let cost = dependency.communication_cost();

        adjacency
            .entry(source)
            .or_default()
            .push((destination, cost));

        adjacency
            .entry(destination)
            .or_default()
            .push((source, cost));
    }

    for neighbors in adjacency.values_mut() {
        neighbors.sort_by_key(|(operation, cost)| (*operation, *cost));
    }

    adjacency
}

fn locality_score(
    operation: OperationId,
    segment_members: &[OperationId],
    adjacency: &BTreeMap<
        OperationId,
        Vec<(OperationId, WorkEstimate)>,
    >,
) -> Result<WorkEstimate, PartitioningError> {
    let Some(neighbors) = adjacency.get(&operation) else {
        return Ok(0);
    };

    let members = segment_members
        .iter()
        .copied()
        .collect::<BTreeSet<_>>();

    neighbors
        .iter()
        .filter(|(neighbor, _)| members.contains(neighbor))
        .try_fold(0_u128, |total, (_, cost)| {
            total
                .checked_add(*cost)
                .ok_or(PartitioningError::ArithmeticOverflow)
        })
}

// ============================================================================
// INTERNAL REPORT CLASSIFICATION
// ============================================================================

fn determine_disposition(
    total: usize,
    assigned: usize,
    unassigned: usize,
    inventory_complete: bool,
) -> PartitioningDisposition {
    if unassigned == total && total > 0 {
        return PartitioningDisposition::NoCompatibleTarget;
    }

    if unassigned > 0 {
        return PartitioningDisposition::Partial;
    }

    if !inventory_complete {
        return PartitioningDisposition::IncompleteInventory;
    }

    // An empty workload is valid and requires no target assignment.
    let _ = assigned;

    PartitioningDisposition::Complete
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn operation(value: u128) -> OperationId {
        OperationId::new(value).expect("valid operation identity")
    }

    fn target(value: &str) -> TargetId {
        TargetId::new(value).expect("valid target identity")
    }

    fn work_item(
        id: u128,
        candidates: &[&str],
    ) -> WorkItem {
        WorkItem::new(
            operation(id),
            1,
            candidates.iter().map(|value| target(value)),
        )
    }

    fn workload(
        items: Vec<WorkItem>,
        dependencies: Vec<Dependency<OperationId>>,
    ) -> PartitioningWorkload {
        PartitioningWorkload::new(items, dependencies)
    }

    fn partition_with(
        graph: &PartitioningWorkload,
        eligible: &[&str],
        complete: bool,
    ) -> PartitioningReport {
        let targets = eligible
            .iter()
            .map(|value| target(value))
            .collect::<BTreeSet<_>>();

        Partitioner::new()
            .partition_with_target_evidence(
                graph,
                PartitioningOptions::default(),
                &targets,
                None,
                complete,
            )
            .expect("partitioning should succeed")
    }

    #[test]
    fn module_identity_is_stable() {
        assert_eq!(
            Partitioner::module_name(),
            PARTITIONING_MODULE_NAME,
        );

        assert_eq!(
            Partitioner::api_version(),
            PARTITIONING_API_VERSION,
        );
    }

    #[test]
    fn empty_workload_produces_complete_empty_report() {
        let graph = workload(Vec::new(), Vec::new());

        let report = partition_with(&graph, &[], true);

        assert!(report.is_complete());
        assert!(report.segments().is_empty());
        assert!(report.unassigned().is_empty());
        assert!(report.boundaries().is_empty());
    }

    #[test]
    fn one_compatible_work_item_is_assigned() {
        let graph = workload(
            vec![work_item(1, &["cpu"])],
            Vec::new(),
        );

        let report = partition_with(&graph, &["cpu"], true);

        assert!(report.is_complete());
        assert_eq!(report.segments().len(), 1);

        assert_eq!(
            report.segments()[0].candidate_targets(),
            &BTreeSet::from([target("cpu")]),
        );
    }

    #[test]
    fn incompatible_operations_are_refined_into_separate_segments() {
        let graph = workload(
            vec![
                work_item(1, &["cpu"]),
                work_item(2, &["gpu"]),
            ],
            vec![
                Dependency::new(operation(1), operation(2), 10),
            ],
        );

        let report = partition_with(
            &graph,
            &["cpu", "gpu"],
            true,
        );

        assert!(report.is_complete());
        assert_eq!(report.segments().len(), 2);
        assert_eq!(report.boundaries().len(), 1);

        assert_eq!(
            report.boundaries()[0].source(),
            operation(1),
        );

        assert_eq!(
            report.boundaries()[0].destination(),
            operation(2),
        );
    }

    #[test]
    fn operations_with_common_targets_can_share_a_segment() {
        let graph = workload(
            vec![
                work_item(1, &["cpu", "gpu"]),
                work_item(2, &["cpu"]),
            ],
            vec![
                Dependency::new(operation(1), operation(2), 5),
            ],
        );

        let report = partition_with(
            &graph,
            &["cpu", "gpu"],
            true,
        );

        assert!(report.is_complete());
        assert_eq!(report.segments().len(), 1);

        assert_eq!(
            report.segments()[0].candidate_targets(),
            &BTreeSet::from([target("cpu")]),
        );
    }

    #[test]
    fn work_without_candidate_targets_is_not_silently_assigned() {
        let graph = workload(
            vec![work_item(1, &[])],
            Vec::new(),
        );

        let report = partition_with(&graph, &["cpu"], true);

        assert_eq!(
            report.disposition(),
            PartitioningDisposition::NoCompatibleTarget,
        );

        assert_eq!(report.unassigned().len(), 1);

        assert_eq!(
            report.unassigned()[0].reason(),
            UnassignedReason::NoOperationCapabilityCandidate,
        );
    }

    #[test]
    fn candidates_not_in_global_capability_report_are_rejected() {
        let graph = workload(
            vec![work_item(1, &["cpu"])],
            Vec::new(),
        );

        let report = partition_with(&graph, &["gpu"], true);

        assert_eq!(report.unassigned().len(), 1);

        assert_eq!(
            report.unassigned()[0].reason(),
            UnassignedReason::NoGloballyEligibleTarget,
        );
    }

    #[test]
    fn resource_evidence_filters_candidate_targets() {
        let graph = workload(
            vec![
                work_item(1, &["cpu", "gpu"]),
            ],
            Vec::new(),
        );

        let capability_targets =
            BTreeSet::from([target("cpu"), target("gpu")]);

        let resource_targets =
            BTreeSet::from([target("gpu")]);

        let report = Partitioner::new()
            .partition_with_target_evidence(
                &graph,
                PartitioningOptions::default(),
                &capability_targets,
                Some(&resource_targets),
                true,
            )
            .expect("partitioning should succeed");

        assert!(report.is_complete());

        assert_eq!(
            report.segments()[0].candidate_targets(),
            &BTreeSet::from([target("gpu")]),
        );
    }

    #[test]
    fn missing_resource_feasibility_is_not_assumed() {
        let graph = workload(
            vec![work_item(1, &["cpu"])],
            Vec::new(),
        );

        let capability_targets =
            BTreeSet::from([target("cpu")]);

        let resource_targets = BTreeSet::new();

        let report = Partitioner::new()
            .partition_with_target_evidence(
                &graph,
                PartitioningOptions::default(),
                &capability_targets,
                Some(&resource_targets),
                true,
            )
            .expect("partitioning should succeed");

        assert_eq!(report.unassigned().len(), 1);

        assert_eq!(
            report.unassigned()[0].reason(),
            UnassignedReason::NoResourceFeasibleTarget,
        );
    }

    #[test]
    fn incomplete_inventory_is_not_reported_as_complete() {
        let graph = workload(
            vec![work_item(1, &["cpu"])],
            Vec::new(),
        );

        let report = partition_with(&graph, &["cpu"], false);

        assert_eq!(
            report.disposition(),
            PartitioningDisposition::IncompleteInventory,
        );

        assert!(!report.is_complete());
    }

    #[test]
    fn dependency_cycles_are_preserved_by_the_existing_partitioner() {
        let graph = workload(
            vec![
                work_item(1, &["cpu"]),
                work_item(2, &["cpu"]),
            ],
            vec![
                Dependency::new(operation(1), operation(2), 1),
                Dependency::new(operation(2), operation(1), 1),
            ],
        );

        let report = partition_with(&graph, &["cpu"], true);

        assert!(report.is_complete());
        assert_eq!(report.dependency_count(), 2);
        assert!(report.boundaries().is_empty());
    }

    #[test]
    fn dependencies_involving_unassigned_work_are_preserved() {
        let graph = workload(
            vec![
                work_item(1, &["cpu"]),
                work_item(2, &[]),
            ],
            vec![
                Dependency::new(operation(1), operation(2), 7),
            ],
        );

        let report = partition_with(&graph, &["cpu"], true);

        assert_eq!(report.unresolved_dependencies().len(), 1);

        assert!(report.unresolved_dependencies()[0].source_assigned());
        assert!(!report.unresolved_dependencies()[0].destination_assigned());
    }

    #[test]
    fn duplicate_operation_metadata_is_rejected() {
        let graph = workload(
            vec![
                work_item(1, &["cpu"]),
                work_item(1, &["gpu"]),
            ],
            Vec::new(),
        );

        let targets = BTreeSet::from([
            target("cpu"),
            target("gpu"),
        ]);

        let result = Partitioner::new()
            .partition_with_target_evidence(
                &graph,
                PartitioningOptions::default(),
                &targets,
                None,
                true,
            );

        assert!(matches!(
            result,
            Err(PartitioningError::DuplicateWorkItemMetadata(_)),
        ));
    }

    #[test]
    fn cross_partition_dependency_is_not_lost() {
        let graph = workload(
            vec![
                work_item(1, &["cpu"]),
                work_item(2, &["gpu"]),
            ],
            vec![
                Dependency::new(operation(1), operation(2), 99),
            ],
        );

        let report = partition_with(
            &graph,
            &["cpu", "gpu"],
            true,
        );

        assert_eq!(report.dependency_count(), 1);
        assert_eq!(report.boundaries().len(), 1);
        assert_eq!(
            report.boundaries()[0].communication_cost(),
            99,
        );
    }
}