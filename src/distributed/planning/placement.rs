
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Distributed placement planning
//!
//! Path: `src/distributed/planning/placement.rs`
//!
//! Selects logical execution targets for the partitions produced by
//! `planning::partition`. Placement decisions are based on target snapshots
//! and an injected evaluator that owns authoritative resource, capability,
//! policy, security, and domain-specific compatibility checks.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Deterministic assignment of logical partitions to supplied targets.
//! - A pluggable target-assessment contract.
//! - Projected placement load accounting.
//! - Explicit per-target partition-count budgets.
//! - Placement explanations and plan validation.
//!
//! This module does not own:
//!
//! - Source syntax or semantic analysis.
//! - ZUIR, ZCIR, quantum::ir, HDLIR, or another domain IR.
//! - Resource discovery or capability discovery.
//! - The canonical resource/capability requirement models.
//! - Network routing, execution scheduling, deployment, or device allocation.
//! - Authentication, authorization, or security policy implementation.
//!
//! The evaluator must consult the owning repository subsystems for those
//! decisions. Unknown mandatory capability or resource evidence must not be
//! reported as eligible.
//!
//! ## Scalability
//!
//! There is no universal maximum target or partition count. The caller may
//! configure an operational partition-count budget. Actual memory, numeric
//! representation, evaluator cost, and available targets remain finite.
//!
//! The default algorithm is deterministic and greedy, not globally optimal.
//! A different algorithm can implement the same evaluator and plan contracts
//! without changing language semantics or the canonical IRs.

use core::fmt;
use std::collections::{BTreeMap, BTreeSet};

use super::partition::{PartitionId, PartitionPlan, WorkEstimate, WorkPartition};

/// A candidate execution target and its caller-owned snapshot.
///
/// `Snapshot` is deliberately generic. It can wrap a discovered resource
/// record, a membership-registry snapshot, a remote provider handle, or a
/// composite of those values without this module defining another resource
/// inventory.
///
/// The snapshot must be valid for the planning operation. If it becomes stale,
/// the caller must refresh it or revalidate the placement before execution.
#[derive(Clone, Debug)]
pub struct PlacementTarget<Id, Snapshot> {
    id: Id,
    snapshot: Snapshot,
}

impl<Id, Snapshot> PlacementTarget<Id, Snapshot> {
    /// Creates a target from its stable logical identifier and snapshot.
    #[must_use]
    pub const fn new(id: Id, snapshot: Snapshot) -> Self {
        Self { id, snapshot }
    }

    /// Returns the target identifier.
    #[must_use]
    pub const fn id(&self) -> &Id {
        &self.id
    }

    /// Returns the caller-owned target snapshot.
    #[must_use]
    pub const fn snapshot(&self) -> &Snapshot {
        &self.snapshot
    }
}

/// Load already assigned to a target during this planning operation.
///
/// `assigned_work` is the checked sum of the estimated work of partitions
/// already assigned to the target. It is an abstract estimate, not a claim
/// that heterogeneous resources share identical performance.
///
/// An evaluator that needs hardware-specific capacity checks must interpret
/// estimates through its own documented model rather than assuming that
/// this value represents bytes, seconds, instructions, or FLOPs.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ProjectedLoad {
    assigned_work: WorkEstimate,
    assigned_partitions: usize,
}

impl ProjectedLoad {
    /// Creates an empty projected load.
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            assigned_work: 0,
            assigned_partitions: 0,
        }
    }

    /// Returns accumulated estimated work.
    #[must_use]
    pub const fn assigned_work(self) -> WorkEstimate {
        self.assigned_work
    }

    /// Returns the number of partitions already assigned.
    #[must_use]
    pub const fn assigned_partitions(self) -> usize {
        self.assigned_partitions
    }

    fn with_partition(
        self,
        work: WorkEstimate,
    ) -> Result<Self, PlacementError> {
        let assigned_work = self
            .assigned_work
            .checked_add(work)
            .ok_or(PlacementError::ArithmeticOverflow)?;

        let assigned_partitions = self
            .assigned_partitions
            .checked_add(1)
            .ok_or(PlacementError::ArithmeticOverflow)?;

        Ok(Self {
            assigned_work,
            assigned_partitions,
        })
    }
}

/// Result of assessing a partition against one target.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum TargetAssessment {
    /// The evaluator establishes that this target can accept the partition.
    ///
    /// `score` is an evaluator-defined, non-negative cost. Lower values are
    /// preferred. Its units and meaning belong to the evaluator contract.
    Eligible {
        /// Placement cost; lower values are preferred.
        score: u128,
        /// Human-readable explanation for diagnostics.
        explanation: String,
    },

    /// The evaluator establishes that this target cannot accept the partition.
    Ineligible {
        /// Why the target is not a valid candidate.
        reason: String,
    },

    /// Available evidence cannot establish that placement is valid.
    ///
    /// This result is never treated as eligible. A caller must not convert
    /// unknown mandatory resource or capability evidence into permission.
    Indeterminate {
        /// Why eligibility could not be established.
        reason: String,
    },
}

impl TargetAssessment {
    /// Constructs an eligible assessment.
    #[must_use]
    pub fn eligible(score: u128, explanation: impl Into<String>) -> Self {
        Self::Eligible {
            score,
            explanation: explanation.into(),
        }
    }

    /// Constructs an ineligible assessment.
    #[must_use]
    pub fn ineligible(reason: impl Into<String>) -> Self {
        Self::Ineligible {
            reason: reason.into(),
        }
    }

    /// Constructs an indeterminate assessment.
    #[must_use]
    pub fn indeterminate(reason: impl Into<String>) -> Self {
        Self::Indeterminate {
            reason: reason.into(),
        }
    }
}

/// Error returned when a placement evaluator itself fails.
///
/// This is distinct from an ineligible target or an indeterminate decision:
/// an evaluator failure means the placement algorithm could not complete the
/// requested assessment reliably.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlacementEvaluationError {
    evaluator: String,
    message: String,
}

impl PlacementEvaluationError {
    /// Constructs an evaluator error.
    #[must_use]
    pub fn new(
        evaluator: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            evaluator: evaluator.into(),
            message: message.into(),
        }
    }

    /// Returns the evaluator identifier.
    #[must_use]
    pub fn evaluator(&self) -> &str {
        &self.evaluator
    }

    /// Returns the error explanation.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for PlacementEvaluationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "placement evaluator {:?} failed: {}",
            self.evaluator, self.message
        )
    }
}

impl std::error::Error for PlacementEvaluationError {}

/// Authoritative placement-assessment interface.
///
/// Implementations are responsible for consulting the repository's canonical
/// requirements, capability evidence, resource observations, security
/// decisions, target status, and any domain-specific placement constraints.
///
/// `projected_load` is the load assigned by this invocation so far. The
/// evaluator may use it to enforce resource capacity or return a placement
/// cost that accounts for current load.
///
/// Implementations must not treat missing mandatory evidence as satisfaction.
/// A fixed snapshot and policy configuration should produce deterministic
/// assessments.
///
/// # Errors
///
/// Return an error if assessment itself fails. Represent a known rejection
/// with `TargetAssessment::Ineligible`; represent insufficient evidence with
/// `TargetAssessment::Indeterminate`.
pub trait PlacementEvaluator<WorkId, TargetId, Snapshot> {
    /// Assesses whether one partition may be placed on one target.
    fn assess(
        &self,
        partition: &WorkPartition<WorkId>,
        target: &PlacementTarget<TargetId, Snapshot>,
        projected_load: ProjectedLoad,
    ) -> Result<TargetAssessment, PlacementEvaluationError>;
}

/// Configurable operational policy for placement.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PlacementPolicy {
    max_partitions_per_target: Option<usize>,
}

impl PlacementPolicy {
    /// Creates a policy without an additional per-target partition-count cap.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            max_partitions_per_target: None,
        }
    }

    /// Sets an optional maximum number of partitions assigned to each target.
    ///
    /// This is an operational policy, not a semantic limit on the program.
    /// A value of zero is invalid for a non-empty placement request.
    #[must_use]
    pub const fn with_max_partitions_per_target(
        mut self,
        limit: Option<usize>,
    ) -> Self {
        self.max_partitions_per_target = limit;
        self
    }
}

/// One successful partition-to-target assignment.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlacementAssignment<TargetId> {
    partition: PartitionId,
    target: TargetId,
    score: u128,
    projected_load_after: ProjectedLoad,
    explanation: String,
}

impl<TargetId> PlacementAssignment<TargetId> {
    /// Returns the logical partition assigned.
    #[must_use]
    pub const fn partition(&self) -> PartitionId {
        self.partition
    }

    /// Returns the selected target identifier.
    #[must_use]
    pub fn target(&self) -> &TargetId {
        &self.target
    }

    /// Returns the evaluator's placement cost.
    #[must_use]
    pub const fn score(&self) -> u128 {
        self.score
    }

    /// Returns the projected target load after this assignment.
    #[must_use]
    pub const fn projected_load_after(&self) -> ProjectedLoad {
        self.projected_load_after
    }

    /// Returns the evaluator's explanation for the selection.
    #[must_use]
    pub fn explanation(&self) -> &str {
        &self.explanation
    }
}

/// A rejected or unresolved target considered during placement.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TargetRejection {
    target: String,
    reason: String,
    indeterminate: bool,
}

impl TargetRejection {
    /// Returns the target's diagnostic identifier.
    #[must_use]
    pub fn target(&self) -> &str {
        &self.target
    }

    /// Returns the rejection or uncertainty explanation.
    #[must_use]
    pub fn reason(&self) -> &str {
        &self.reason
    }

    /// Returns whether the evaluator could not decide eligibility.
    #[must_use]
    pub const fn is_indeterminate(&self) -> bool {
        self.indeterminate
    }
}

/// A complete, validated placement result for one partition plan.
///
/// This result is a logical assignment proposal. It does not itself allocate
/// hardware, reserve resources, deploy processes, establish network routes,
/// or guarantee that a target remains available until execution.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlacementPlan<TargetId> {
    assignments: Vec<PlacementAssignment<TargetId>>,
    unselected_target_count: usize,
}

impl<TargetId> PlacementPlan<TargetId> {
    /// Returns assignments in stable partition-identifier order.
    #[must_use]
    pub fn assignments(&self) -> &[PlacementAssignment<TargetId>] {
        &self.assignments
    }

    /// Returns the number of supplied targets that received no assignment.
    #[must_use]
    pub const fn unselected_target_count(&self) -> usize {
        self.unselected_target_count
    }

    /// Returns whether the placement result contains no assignments.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.assignments.is_empty()
    }

    /// Finds the target assigned to a partition.
    #[must_use]
    pub fn target_for(
        &self,
        partition: PartitionId,
    ) -> Option<&TargetId> {
        self.assignments
            .iter()
            .find(|assignment| assignment.partition == partition)
            .map(|assignment| &assignment.target)
    }

    /// Validates that every partition in `partitions` has exactly one
    /// assignment and that no assignment refers to an unknown partition.
    ///
    /// This structural validation does not re-evaluate resource feasibility.
    ///
    /// # Errors
    ///
    /// Returns an error if a partition is missing, assigned more than once,
    /// or unknown to the supplied partition plan.
    pub fn validate_against<WorkId>(
        &self,
        partitions: &PartitionPlan<WorkId>,
    ) -> Result<(), PlacementError>
    where
        TargetId: Ord,
    {
        let expected: BTreeSet<PartitionId> = partitions
            .partitions()
            .iter()
            .map(WorkPartition::id)
            .collect();

        let mut actual = BTreeSet::new();

        for assignment in &self.assignments {
            if !expected.contains(&assignment.partition) {
                return Err(PlacementError::UnknownPartition {
                    partition: assignment.partition,
                });
            }

            if !actual.insert(assignment.partition) {
                return Err(PlacementError::DuplicatePartitionAssignment {
                    partition: assignment.partition,
                });
            }
        }

        for partition in expected {
            if !actual.contains(&partition) {
                return Err(PlacementError::MissingPartitionAssignment {
                    partition,
                });
            }
        }

        Ok(())
    }
}

/// Failure that prevents a complete placement plan from being produced.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum PlacementError {
    /// A non-empty plan was supplied without any candidate targets.
    NoTargets,

    /// Two candidate targets have the same identifier.
    DuplicateTarget,

    /// A partition has no target established as eligible.
    NoEligibleTarget {
        /// Partition that could not be placed.
        partition: PartitionId,
        /// Per-target rejection and uncertainty details.
        reasons: Vec<TargetRejection>,
    },

    /// The configured per-target partition-count limit is zero.
    ZeroTargetCapacity,

    /// The evaluator failed before a complete plan could be established.
    EvaluatorFailed(PlacementEvaluationError),

    /// Checked load or count arithmetic overflowed.
    ArithmeticOverflow,

    /// The placement result contains an assignment to an unknown partition.
    UnknownPartition {
        /// Unknown partition identifier.
        partition: PartitionId,
    },

    /// A partition has more than one placement assignment.
    DuplicatePartitionAssignment {
        /// Repeated partition identifier.
        partition: PartitionId,
    },

    /// A partition has no placement assignment.
    MissingPartitionAssignment {
        /// Unassigned partition identifier.
        partition: PartitionId,
    },
}

impl fmt::Display for PlacementError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoTargets => {
                f.write_str("no placement targets were supplied")
            }
            Self::DuplicateTarget => {
                f.write_str("placement targets contain duplicate identifiers")
            }
            Self::NoEligibleTarget { partition, reasons } => write!(
                f,
                "no eligible target for {partition}; {} candidate assessment(s) rejected or unresolved",
                reasons.len()
            ),
            Self::ZeroTargetCapacity => {
                f.write_str("per-target partition capacity cannot be zero")
            }
            Self::EvaluatorFailed(error) => {
                write!(f, "placement assessment failed: {error}")
            }
            Self::ArithmeticOverflow => {
                f.write_str("placement load arithmetic overflowed")
            }
            Self::UnknownPartition { partition } => {
                write!(f, "placement references unknown {partition}")
            }
            Self::DuplicatePartitionAssignment { partition } => {
                write!(f, "{partition} has multiple placement assignments")
            }
            Self::MissingPartitionAssignment { partition } => {
                write!(f, "{partition} has no placement assignment")
            }
        }
    }
}

impl std::error::Error for PlacementError {}

/// Assigns every partition to an eligible target.
///
/// # Selection behavior
///
/// Partitions are considered in descending estimated-work order, with
/// partition identifier as the deterministic tie-breaker. This places larger
/// units first so they are not needlessly left until after smaller assignments.
///
/// For each partition, every candidate target is assessed by `evaluator`.
/// Ineligible and indeterminate targets are excluded. Among eligible targets,
/// the algorithm chooses the minimum tuple of:
///
/// 1. Evaluator-supplied placement score.
/// 2. Projected accumulated work on the target.
/// 3. Projected number of assigned partitions.
/// 4. Stable target identifier.
///
/// The evaluator score is the primary policy signal. The remaining fields
/// provide deterministic tie-breaking and a basic load-balancing preference.
/// The evaluator remains responsible for hard capacity and compatibility
/// checks; this function never treats the score as proof of eligibility.
///
/// # Errors
///
/// Returns an error if a target identifier is duplicated, an evaluator fails,
/// a partition has no eligible target, or checked arithmetic overflows.
/// No partial placement plan is returned.
///
/// # Limitations
///
/// This is a deterministic greedy algorithm, not a global optimizer. It does
/// not reserve resources. A runtime must revalidate or atomically reserve the
/// selected resources before execution to address changing availability.
pub fn place_partitions<WorkId, TargetId, Snapshot, Evaluator>(
    partitions: &PartitionPlan<WorkId>,
    targets: &[PlacementTarget<TargetId, Snapshot>],
    policy: PlacementPolicy,
    evaluator: &Evaluator,
) -> Result<PlacementPlan<TargetId>, PlacementError>
where
    WorkId: Clone + Ord + fmt::Debug,
    TargetId: Clone + Ord + fmt::Debug,
    Evaluator: PlacementEvaluator<WorkId, TargetId, Snapshot>,
{
    if partitions.is_empty() {
        return Ok(PlacementPlan {
            assignments: Vec::new(),
            unselected_target_count: targets.len(),
        });
    }

    if targets.is_empty() {
        return Err(PlacementError::NoTargets);
    }

    if policy.max_partitions_per_target == Some(0) {
        return Err(PlacementError::ZeroTargetCapacity);
    }

    let mut target_indices: Vec<usize> = (0..targets.len()).collect();
    target_indices.sort_by(|left, right| {
        targets[*left].id.cmp(&targets[*right].id)
    });

    for pair in target_indices.windows(2) {
        if targets[pair[0]].id == targets[pair[1]].id {
            return Err(PlacementError::DuplicateTarget);
        }
    }

    let mut ordered_partitions: Vec<&WorkPartition<WorkId>> =
        partitions.partitions().iter().collect();

    ordered_partitions.sort_by(|left, right| {
        right
            .estimated_work()
            .cmp(&left.estimated_work())
            .then_with(|| left.id().cmp(&right.id()))
    });

    let mut loads: BTreeMap<TargetId, ProjectedLoad> = BTreeMap::new();
    let mut assignments = Vec::with_capacity(ordered_partitions.len());

    for partition in ordered_partitions {
        let mut best: Option<(usize, u128, ProjectedLoad, String)> = None;
        let mut reasons = Vec::new();

        for index in target_indices.iter().copied() {
            let target = &targets[index];
            let current_load = loads
                .get(&target.id)
                .copied()
                .unwrap_or_else(ProjectedLoad::empty);

            if let Some(maximum) = policy.max_partitions_per_target {
                if current_load.assigned_partitions() >= maximum {
                    reasons.push(TargetRejection {
                        target: format!("{:?}", target.id),
                        reason: format!(
                            "configured per-target partition limit ({maximum}) reached"
                        ),
                        indeterminate: false,
                    });
                    continue;
                }
            }

            let assessment = evaluator
                .assess(partition, target, current_load)
                .map_err(PlacementError::EvaluatorFailed)?;

            match assessment {
                TargetAssessment::Eligible {
                    score,
                    explanation,
                } => {
                    let projected = current_load
                        .with_partition(partition.estimated_work())?;

                    // The target ID is already unique and sorted. The
                    // index is retained to access the exact supplied snapshot.
                    let candidate = (index, score, projected, explanation);

                    let should_replace = match &best {
                        None => true,
                        Some((best_index, best_score, best_load, _)) => {
                            (
                                candidate.1,
                                candidate.2.assigned_work(),
                                candidate.2.assigned_partitions(),
                                &targets[candidate.0].id,
                            ) < (
                                *best_score,
                                best_load.assigned_work(),
                                best_load.assigned_partitions(),
                                &targets[*best_index].id,
                            )
                        }
                    };

                    if should_replace {
                        best = Some(candidate);
                    }
                }
                TargetAssessment::Ineligible { reason } => {
                    reasons.push(TargetRejection {
                        target: format!("{:?}", target.id),
                        reason,
                        indeterminate: false,
                    });
                }
                TargetAssessment::Indeterminate { reason } => {
                    reasons.push(TargetRejection {
                        target: format!("{:?}", target.id),
                        reason,
                        indeterminate: true,
                    });
                }
            }
        }

        let Some((target_index, score, projected_load_after, explanation)) = best
        else {
            return Err(PlacementError::NoEligibleTarget {
                partition: partition.id(),
                reasons,
            });
        };

        let target_id = targets[target_index].id.clone();
        loads.insert(target_id.clone(), projected_load_after);

        assignments.push(PlacementAssignment {
            partition: partition.id(),
            target: target_id,
            score,
            projected_load_after,
            explanation,
        });
    }

    assignments.sort_by_key(PlacementAssignment::partition);

    let selected_targets: BTreeSet<TargetId> = assignments
        .iter()
        .map(|assignment| assignment.target.clone())
        .collect();

    let unselected_target_count =
        targets.iter().filter(|target| !selected_targets.contains(&target.id)).count();

    let result = PlacementPlan {
        assignments,
        unselected_target_count,
    };

    result.validate_against(partitions)?;

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::distributed::planning::partition::{
        partition_workload, Dependency, PartitionBudget, PartitionPolicy, WorkUnit,
        WorkloadGraph,
    };

    #[derive(Clone, Debug)]
    struct TestSnapshot {
        accepts: bool,
        score: u128,
    }

    struct TestEvaluator;

    impl PlacementEvaluator<&'static str, &'static str, TestSnapshot>
        for TestEvaluator
    {
        fn assess(
            &self,
            _partition: &WorkPartition<&'static str>,
            target: &PlacementTarget<&'static str, TestSnapshot>,
            _load: ProjectedLoad,
        ) -> Result<TargetAssessment, PlacementEvaluationError> {
            if target.snapshot.accepts {
                Ok(TargetAssessment::eligible(
                    target.snapshot.score,
                    "test target accepts this partition",
                ))
            } else {
                Ok(TargetAssessment::ineligible(
                    "test target does not support this partition",
                ))
            }
        }
    }

    fn test_partitions() -> PartitionPlan<&'static str> {
        let graph = WorkloadGraph::new(
            vec![
                WorkUnit::new("a", 9, None),
                WorkUnit::new("b", 5, None),
                WorkUnit::new("c", 2, None),
            ],
            vec![
                Dependency::new("a", "b", 1),
                Dependency::new("b", "c", 1),
            ],
        );

        partition_workload(
            &graph,
            PartitionPolicy::new().with_target_partition_count(Some(3)),
            PartitionBudget::unlimited(),
        )
        .expect("test workload should partition")
    }

    #[test]
    fn places_every_partition_exactly_once() {
        let partitions = test_partitions();
        let targets = vec![
            PlacementTarget::new(
                "target-a",
                TestSnapshot {
                    accepts: true,
                    score: 10,
                },
            ),
            PlacementTarget::new(
                "target-b",
                TestSnapshot {
                    accepts: true,
                    score: 1,
                },
            ),
        ];

        let result = place_partitions(
            &partitions,
            &targets,
            PlacementPolicy::new(),
            &TestEvaluator,
        )
        .expect("placement should succeed");

        assert_eq!(
            result.assignments().len(),
            partitions.partitions().len()
        );
        result
            .validate_against(&partitions)
            .expect("placement should be structurally valid");
    }

    #[test]
    fn rejects_targets_that_do_not_meet_requirements() {
        let partitions = test_partitions();
        let targets = vec![PlacementTarget::new(
            "unsupported",
            TestSnapshot {
                accepts: false,
                score: 0,
            },
        )];

        let error = place_partitions(
            &partitions,
            &targets,
            PlacementPolicy::new(),
            &TestEvaluator,
        )
        .expect_err("no target should be eligible");

        assert!(matches!(
            error,
            PlacementError::NoEligibleTarget { .. }
        ));
    }

    #[test]
    fn duplicate_target_ids_are_rejected() {
        let partitions = test_partitions();
        let targets = vec![
            PlacementTarget::new(
                "duplicate",
                TestSnapshot {
                    accepts: true,
                    score: 1,
                },
            ),
            PlacementTarget::new(
                "duplicate",
                TestSnapshot {
                    accepts: true,
                    score: 2,
                },
            ),
        ];

        let error = place_partitions(
            &partitions,
            &targets,
            PlacementPolicy::new(),
            &TestEvaluator,
        )
        .expect_err("duplicate target IDs must be rejected");

        assert_eq!(error, PlacementError::DuplicateTarget);
    }

    #[test]
    fn unknown_eligibility_is_not_accepted() {
        struct UnknownEvaluator;

        impl PlacementEvaluator<&'static str, &'static str, TestSnapshot>
            for UnknownEvaluator
        {
            fn assess(
                &self,
                _partition: &WorkPartition<&'static str>,
                _target: &PlacementTarget<&'static str, TestSnapshot>,
                _load: ProjectedLoad,
            ) -> Result<TargetAssessment, PlacementEvaluationError> {
                Ok(TargetAssessment::indeterminate(
                    "resource evidence is missing",
                ))
            }
        }

        let partitions = test_partitions();
        let targets = vec![PlacementTarget::new(
            "unknown",
            TestSnapshot {
                accepts: true,
                score: 0,
            },
        )];

        let error = place_partitions(
            &partitions,
            &targets,
            PlacementPolicy::new(),
            &UnknownEvaluator,
        )
        .expect_err("indeterminate targets must not be selected");

        assert!(matches!(
            error,
            PlacementError::NoEligibleTarget { .. }
        ));
    }

    #[test]
    fn empty_partition_plan_needs_no_targets() {
        let graph: WorkloadGraph<&str> = WorkloadGraph::new(Vec::new(), Vec::new());
        let partitions = partition_workload(
            &graph,
            PartitionPolicy::new(),
            PartitionBudget::unlimited(),
        )
        .expect("empty workload should partition");

        let result: PlacementPlan<&str> = place_partitions(
            &partitions,
            &[],
            PlacementPolicy::new(),
            &TestEvaluator,
        )
        .expect("empty placement should succeed");

        assert!(result.is_empty());
    }

    #[test]
    fn per_target_partition_limit_is_enforced() {
        let partitions = test_partitions();
        let targets = vec![PlacementTarget::new(
            "only-target",
            TestSnapshot {
                accepts: true,
                score: 0,
            },
        )];

        let error = place_partitions(
            &partitions,
            &targets,
            PlacementPolicy::new().with_max_partitions_per_target(Some(1)),
            &TestEvaluator,
        )
        .expect_err("target limit should prevent full placement");

        assert!(matches!(
            error,
            PlacementError::NoEligibleTarget { .. }
        ));
    }
}
