
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Compiler — Heterogeneous Workload Partitioning
//!
//! Coordinates target-compatible logical workload partitioning.
//!
//! This module integrates:
//!
//! - Canonical ZUIR operation identities.
//! - Compiler target identities.
//! - Operation-level capability candidates.
//! - Global capability-matching evidence.
//! - Optional resource-feasibility evidence.
//! - Canonical distributed workload partitioning.
//!
//! It does not perform physical placement, target selection,
//! allocation, scheduling, routing, lowering, or execution.
//!
//! # Architectural authorities
//!
//! Operation identity:
//!     crate::zuir::ids::OperationId
//!
//! Target identity:
//!     crate::compiler::configuration::target::TargetId
//!
//! Capability matching:
//!     crate::compiler::planning::capability_matching
//!
//! Resource feasibility:
//!     crate::compiler::planning::resource_planning
//!
//! Generic logical partitioning:
//!     crate::distributed::planning::partition
//!
//! Source semantics:
//!     crate::frontend::semantic
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
//! # Scalability
//!
//! No arbitrary universal ceilings are imposed on:
//!
//! - Operations.
//! - Workload dependencies.
//! - Partitions.
//! - Targets.
//! - Computing domains.
//! - CPUs, GPUs, FPGAs, ASICs or QPUs.
//! - Distributed participants.
//!
//! Operational budgets are explicit caller policies.
//!
//! Actual execution remains subject to finite memory, address space,
//! available processing capacity and physical resources.
//!
//! # Partitioning invariant
//!
//! Every assigned operation appears in exactly one output segment.
//!
//! Every unassigned operation appears in exactly one unassigned record.
//!
//! Every segment contains operations with identical proven candidate
//! target sets. This conservative grouping preserves all alternatives
//! without selecting a target.
//!
//! Every input dependency is retained either as an internal dependency,
//! a cross-segment boundary, or an unresolved dependency.
//!
//! No dependency is silently discarded.

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

/// Stable architectural identity.
pub const PARTITIONING_MODULE_NAME: &str =
    "zamani.compiler.planning.partitioning";

/// Public API version.
pub const PARTITIONING_API_VERSION: u16 = 2;

// ============================================================================
// WORK ITEM
// ============================================================================

/// One indivisible compiler work item.
///
/// Operation identity belongs to ZUIR.
///
/// Candidate targets must originate from operation-level capability
/// analysis. They are not inferred from names or hardware vendors.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkItem {
    operation: OperationId,
    estimated_work: WorkEstimate,
    affinity: Option<String>,
    candidate_targets: BTreeSet<TargetId>,
}

impl WorkItem {
    /// Creates a compiler work item.
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

    /// Sets a logical affinity key.
    ///
    /// Affinity is a grouping preference. It is not a physical
    /// placement instruction.
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

    /// Returns the optional affinity.
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

/// Compiler-facing workload graph.
///
/// This is planning metadata, not a replacement for ZUIR.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PartitioningWorkload {
    work_items: Vec<WorkItem>,
    dependencies: Vec<Dependency<OperationId>>,
}

impl PartitioningWorkload {
    /// Creates a workload.
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

    /// Returns work items.
    #[must_use]
    pub fn work_items(&self) -> &[WorkItem] {
        &self.work_items
    }

    /// Returns dependencies.
    #[must_use]
    pub fn dependencies(&self) -> &[Dependency<OperationId>] {
        &self.dependencies
    }

    /// Returns whether the workload is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.work_items.is_empty()
    }
}

// ============================================================================
// OPTIONS
// ============================================================================

/// Partitioning invocation options.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PartitioningOptions {
    policy: PartitionPolicy,
    budget: PartitionBudget,
}

impl PartitioningOptions {
    /// Creates partitioning options.
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

    /// Returns the partitioning policy.
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
// PARTITION IDENTITIES
// ============================================================================

/// Identity of a compatibility-refined partition segment.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PartitionSegmentId {
    base_partition: PartitionId,
    ordinal: usize,
}

impl PartitionSegmentId {
    /// Creates an identity.
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

    /// Returns the original partition identity.
    #[must_use]
    pub const fn base_partition(self) -> PartitionId {
        self.base_partition
    }

    /// Returns the segment ordinal.
    #[must_use]
    pub const fn ordinal(self) -> usize {
        self.ordinal
    }
}

impl fmt::Display for PartitionSegmentId {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            formatter,
            "{}.{}",
            self.base_partition,
            self.ordinal
        )
    }
}

// ============================================================================
// PARTITION SEGMENTS
// ============================================================================

/// One logical target-compatible partition segment.
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

    /// Returns the base partition.
    #[must_use]
    pub const fn base_partition(&self) -> PartitionId {
        self.id.base_partition()
    }

    /// Returns member operations.
    #[must_use]
    pub fn work_items(&self) -> &[OperationId] {
        &self.work_items
    }

    /// Returns all proven candidate targets.
    #[must_use]
    pub fn candidate_targets(&self) -> &BTreeSet<TargetId> {
        &self.candidate_targets
    }

    /// Returns aggregate abstract work.
    #[must_use]
    pub const fn estimated_work(&self) -> WorkEstimate {
        self.estimated_work
    }

    /// Returns whether at least one candidate exists.
    #[must_use]
    pub fn has_candidate_target(&self) -> bool {
        !self.candidate_targets.is_empty()
    }
}

// ============================================================================
// UNASSIGNED WORK
// ============================================================================

/// Why a work item could not be assigned.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum UnassignedReason {
    /// No operation-level candidate exists.
    NoOperationCapabilityCandidate,

    /// No candidate passed global capability matching.
    NoGloballyEligibleTarget,

    /// No candidate had proven resource feasibility.
    NoResourceFeasibleTarget,
}

impl fmt::Display for UnassignedReason {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(match self {
            Self::NoOperationCapabilityCandidate =>
                "no operation-level capability candidate",

            Self::NoGloballyEligibleTarget =>
                "no globally capability-eligible target",

            Self::NoResourceFeasibleTarget =>
                "no target with proven resource feasibility",
        })
    }
}

/// An operation that could not be assigned.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnassignedWorkItem {
    operation: OperationId,
    reason: UnassignedReason,
}

impl UnassignedWorkItem {
    /// Returns its operation.
    #[must_use]
    pub const fn operation(&self) -> OperationId {
        self.operation
    }

    /// Returns its reason.
    #[must_use]
    pub const fn reason(&self) -> UnassignedReason {
        self.reason
    }
}

// ============================================================================
// DEPENDENCY BOUNDARIES
// ============================================================================

/// A dependency crossing two assigned segments.
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

    /// Returns the source segment.
    #[must_use]
    pub const fn source_segment(&self) -> PartitionSegmentId {
        self.source_segment
    }

    /// Returns the destination segment.
    #[must_use]
    pub const fn destination_segment(&self) -> PartitionSegmentId {
        self.destination_segment
    }

    /// Returns the communication estimate.
    #[must_use]
    pub const fn communication_cost(&self) -> WorkEstimate {
        self.communication_cost
    }
}

/// A dependency involving unassigned work.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnresolvedDependency {
    source: OperationId,
    destination: OperationId,
    source_assigned: bool,
    destination_assigned: bool,
    communication_cost: WorkEstimate,
}

impl UnresolvedDependency {
    /// Returns the source.
    #[must_use]
    pub const fn source(&self) -> OperationId {
        self.source
    }

    /// Returns the destination.
    #[must_use]
    pub const fn destination(&self) -> OperationId {
        self.destination
    }

    /// Returns whether the source was assigned.
    #[must_use]
    pub const fn source_assigned(&self) -> bool {
        self.source_assigned
    }

    /// Returns whether the destination was assigned.
    #[must_use]
    pub const fn destination_assigned(&self) -> bool {
        self.destination_assigned
    }

    /// Returns the communication estimate.
    #[must_use]
    pub const fn communication_cost(&self) -> WorkEstimate {
        self.communication_cost
    }
}

// ============================================================================
// DISPOSITION
// ============================================================================

/// Overall assignment disposition.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum PartitioningDisposition {
    /// All operations assigned with complete discovery.
    Complete,

    /// Some operations remain unassigned.
    Partial,

    /// No operation could be assigned.
    NoCompatibleTarget,

    /// All operations assigned, but discovery was incomplete.
    IncompleteInventory,
}

impl PartitioningDisposition {
    /// Whether all operations are represented in assigned segments.
    #[must_use]
    pub const fn all_work_assigned(self) -> bool {
        matches!(
            self,
            Self::Complete | Self::IncompleteInventory
        )
    }

    /// Conservative disposition-only completeness indication.
    ///
    /// Use PartitioningReport::inventory_is_complete for the
    /// authoritative result.
    #[must_use]
    pub const fn inventory_complete(self) -> bool {
        matches!(self, Self::Complete)
    }
}

// ============================================================================
// REPORT
// ============================================================================

/// Complete compiler partitioning report.
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
    /// Returns the generic distributed partition plan.
    #[must_use]
    pub fn base_plan(&self) -> &PartitionPlan<OperationId> {
        &self.base_plan
    }

    /// Returns compatibility-refined segments.
    #[must_use]
    pub fn segments(&self) -> &[PartitionSegment] {
        &self.segments
    }

    /// Returns unassigned work.
    #[must_use]
    pub fn unassigned(&self) -> &[UnassignedWorkItem] {
        &self.unassigned
    }

    /// Returns cross-segment boundaries.
    #[must_use]
    pub fn boundaries(&self) -> &[PartitionBoundary] {
        &self.boundaries
    }

    /// Returns unresolved dependencies.
    #[must_use]
    pub fn unresolved_dependencies(&self) -> &[UnresolvedDependency] {
        &self.unresolved_dependencies
    }

    /// Returns the assignment disposition.
    #[must_use]
    pub const fn disposition(&self) -> PartitioningDisposition {
        self.disposition
    }

    /// Returns authoritative discovery completeness.
    #[must_use]
    pub const fn inventory_is_complete(&self) -> bool {
        self.inventory_complete
    }

    /// Returns original work-item count.
    #[must_use]
    pub const fn work_item_count(&self) -> usize {
        self.work_item_count
    }

    /// Returns original dependency count.
    #[must_use]
    pub const fn dependency_count(&self) -> usize {
        self.dependency_count
    }

    /// Returns whether all work is assigned exactly once.
    #[must_use]
    pub fn is_fully_assigned(&self) -> bool {
        self.unassigned.is_empty()
            && self.assigned_count() == self.work_item_count
    }

    /// Returns the number of assigned operations.
    #[must_use]
    pub fn assigned_count(&self) -> usize {
        self.segments
            .iter()
            .map(|segment| segment.work_items.len())
            .sum()
    }

    /// Returns whether all segments have candidate targets.
    #[must_use]
    pub fn every_segment_has_candidate_targets(&self) -> bool {
        self.segments
            .iter()
            .all(PartitionSegment::has_candidate_target)
    }

    /// Returns whether the result is fully complete.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.disposition == PartitioningDisposition::Complete
            && self.is_fully_assigned()
            && self.inventory_complete
            && self.every_segment_has_candidate_targets()
            && self.unresolved_dependencies.is_empty()
    }

    /// Returns whether cross-segment dependencies exist.
    #[must_use]
    pub fn has_cross_partition_dependencies(&self) -> bool {
        !self.boundaries.is_empty()
    }

    /// Returns whether unresolved dependencies exist.
    #[must_use]
    pub fn has_unresolved_dependencies(&self) -> bool {
        !self.unresolved_dependencies.is_empty()
    }

    /// Returns whether any segment exists.
    #[must_use]
    pub fn has_segments(&self) -> bool {
        !self.segments.is_empty()
    }
}

// ============================================================================
// ERRORS
// ============================================================================

/// Compiler partitioning errors.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum PartitioningError {
    /// The canonical distributed partitioner rejected the graph.
    DistributedPartitioning(PartitionError),

    /// Duplicate work-item metadata.
    DuplicateWorkItemMetadata(OperationId),

    /// Missing work-item metadata.
    MissingWorkItemMetadata(OperationId),

    /// Checked arithmetic failed.
    ArithmeticOverflow,

    /// An invariant was violated during report construction.
    InvalidPartitionResult(String),
}

impl fmt::Display for PartitioningError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::DistributedPartitioning(error) => {
                write!(formatter, "distributed partitioning failed: {error}")
            }

            Self::DuplicateWorkItemMetadata(operation) => {
                write!(formatter, "duplicate operation metadata: {operation:?}")
            }

            Self::MissingWorkItemMetadata(operation) => {
                write!(formatter, "missing operation metadata: {operation:?}")
            }

            Self::ArithmeticOverflow => {
                formatter.write_str("partitioning arithmetic overflow")
            }

            Self::InvalidPartitionResult(reason) => {
                write!(formatter, "invalid partition result: {reason}")
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
        candidates: BTreeSet<TargetId>,
        estimate: WorkEstimate,
    ) -> Self {
        Self {
            ordinal,
            work_items: vec![operation],
            candidate_targets: candidates,
            estimated_work: estimate,
        }
    }

    fn add(
        &mut self,
        operation: OperationId,
        estimate: WorkEstimate,
    ) -> Result<(), PartitioningError> {
        self.estimated_work = self
            .estimated_work
            .checked_add(estimate)
            .ok_or(PartitioningError::ArithmeticOverflow)?;

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
#[derive(Clone, Copy, Debug, Default)]
pub struct Partitioner;

impl Partitioner {
    /// Creates a partitioner.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Partitions using canonical capability and optional resource evidence.
    ///
    /// Resource evidence is never assumed to be unlimited.
    pub fn partition(
        &self,
        workload: &PartitioningWorkload,
        options: PartitioningOptions,
        capability_report: &CapabilityMatchReport,
        resource_report: Option<&ResourcePlanningReport>,
    ) -> Result<PartitioningReport, PartitioningError> {
        let capability_targets = capability_report
            .satisfied_targets()
            .map(|candidate| candidate.target_id().clone())
            .collect::<BTreeSet<_>>();

        let resource_targets = resource_report.map(|report| {
            report
                .feasible_targets()
                .map(|target| target.target_id().clone())
                .collect::<BTreeSet<_>>()
        });

        let inventory_complete =
            capability_report.discovery_is_complete()
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

    /// Partitions using previously evaluated target evidence.
    pub fn partition_with_target_evidence(
        &self,
        workload: &PartitioningWorkload,
        options: PartitioningOptions,
        capability_eligible_targets: &BTreeSet<TargetId>,
        resource_feasible_targets: Option<&BTreeSet<TargetId>>,
        inventory_complete: bool,
    ) -> Result<PartitioningReport, PartitioningError> {
        // ------------------------------------------------------------
        // Validate metadata identity.
        // ------------------------------------------------------------

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

        // ------------------------------------------------------------
        // Construct canonical generic workload.
        // ------------------------------------------------------------

        let units = workload
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

        let graph = WorkloadGraph::new(
            units,
            workload.dependencies.clone(),
        );

        // Delegate structural validation and generic partitioning.
        let base_plan = partition_workload(
            &graph,
            options.policy(),
            options.budget(),
        )?;

        // ------------------------------------------------------------
        // Compatibility refinement.
        //
        // Exact candidate-set grouping is deliberately conservative:
        // every operation in a segment has exactly the same proven
        // alternatives.
        //
        // It avoids repeatedly scanning previously assigned operations
        // to calculate locality.
        // ------------------------------------------------------------

        let mut segments = Vec::new();
        let mut unassigned = Vec::new();

        let mut segment_by_operation =
            BTreeMap::<OperationId, PartitionSegmentId>::new();

        for base_partition in base_plan.partitions() {
            let mut groups =
                BTreeMap::<BTreeSet<TargetId>, SegmentBuilder>::new();

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

                if let Some(builder) = groups.get_mut(&eligible) {
                    builder.add(
                        *operation,
                        item.estimated_work,
                    )?;
                } else {
                    let ordinal = groups.len();

                    groups.insert(
                        eligible.clone(),
                        SegmentBuilder::new(
                            ordinal,
                            *operation,
                            eligible,
                            item.estimated_work,
                        ),
                    );
                }
            }

            for (_, builder) in groups {
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

        // ------------------------------------------------------------
        // Preserve every dependency.
        // ------------------------------------------------------------

        let mut boundaries = Vec::new();
        let mut unresolved_dependencies = Vec::new();

        for dependency in &workload.dependencies {
            let source = *dependency.source();
            let destination = *dependency.destination();

            let source_segment =
                segment_by_operation.get(&source).copied();

            let destination_segment =
                segment_by_operation.get(&destination).copied();

            match (source_segment, destination_segment) {
                (Some(left), Some(right)) if left != right => {
                    boundaries.push(PartitionBoundary {
                        source,
                        destination,
                        source_segment: left,
                        destination_segment: right,
                        communication_cost:
                            dependency.communication_cost(),
                    });
                }

                (Some(_), Some(_)) => {
                    // Dependency remains internal to its segment.
                }

                _ => {
                    unresolved_dependencies.push(
                        UnresolvedDependency {
                            source,
                            destination,
                            source_assigned: source_segment.is_some(),
                            destination_assigned:
                                destination_segment.is_some(),
                            communication_cost:
                                dependency.communication_cost(),
                        },
                    );
                }
            }
        }

        // ------------------------------------------------------------
        // Verify work conservation.
        // ------------------------------------------------------------

        let assigned = segment_by_operation.len();

        let accounted = assigned
            .checked_add(unassigned.len())
            .ok_or(PartitioningError::ArithmeticOverflow)?;

        if accounted != workload.work_items.len() {
            return Err(
                PartitioningError::InvalidPartitionResult(
                    format!(
                        "expected {} operations, accounted for {}",
                        workload.work_items.len(),
                        accounted,
                    ),
                ),
            );
        }

        // Verify unique assignment and segment membership.
        for segment in &segments {
            if segment.work_items.is_empty()
                || segment.candidate_targets.is_empty()
            {
                return Err(
                    PartitioningError::InvalidPartitionResult(
                        "empty or incompatible segment".to_owned(),
                    ),
                );
            }

            for operation in &segment.work_items {
                if segment_by_operation.get(operation)
                    != Some(&segment.id)
                {
                    return Err(
                        PartitioningError::InvalidPartitionResult(
                            format!(
                                "inconsistent membership for {operation:?}"
                            ),
                        ),
                    );
                }
            }
        }

        let disposition = determine_disposition(
            workload.work_items.len(),
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

    /// Returns the module identity.
    #[must_use]
    pub const fn module_name() -> &'static str {
        PARTITIONING_MODULE_NAME
    }

    /// Returns the API version.
    #[must_use]
    pub const fn api_version() -> u16 {
        PARTITIONING_API_VERSION
    }
}

// ============================================================================
// TARGET FILTERING
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

    if let Some(resources) = resource_targets {
        eligible.retain(|target| resources.contains(target));
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

    let has_global_candidate = item
        .candidate_targets
        .iter()
        .any(|target| capability_targets.contains(target));

    if !has_global_candidate {
        return UnassignedReason::NoGloballyEligibleTarget;
    }

    if resource_targets.is_some() {
        return UnassignedReason::NoResourceFeasibleTarget;
    }

    UnassignedReason::NoGloballyEligibleTarget
}

// ============================================================================
// DISPOSITION
// ============================================================================

fn determine_disposition(
    total: usize,
    unassigned: usize,
    inventory_complete: bool,
) -> PartitioningDisposition {
    if total > 0 && unassigned == total {
        return PartitioningDisposition::NoCompatibleTarget;
    }

    if unassigned > 0 {
        return PartitioningDisposition::Partial;
    }

    if !inventory_complete {
        return PartitioningDisposition::IncompleteInventory;
    }

    PartitioningDisposition::Complete
}
