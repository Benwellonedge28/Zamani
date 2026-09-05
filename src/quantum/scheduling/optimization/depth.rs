//! Zamani Quantum Scheduling — Depth Objective
//!
//! This module defines the canonical scheduling-depth objective.
//!
//! # Purpose
//!
//! The depth objective answers:
//!
//! > "What is the minimum number of dependency layers required by this
//! > schedule?"
//!
//! Depth is intentionally distinct from makespan:
//!
//! ```text
//! depth   = number of sequential dependency layers
//! makespan = elapsed target schedule time
//! ```
//!
//! Therefore:
//!
//! ```text
//! A ──┐
//!     ├──> C
//! B ──┘
//! ```
//!
//! has depth 2, even when `A` and `B` execute concurrently.
//!
//! # Architectural ownership
//!
//! This module owns:
//!
//! - schedule-depth measurement;
//! - dependency-layer analysis;
//! - depth objective scoring;
//! - deterministic comparison of depth scores;
//! - depth-specific diagnostics;
//! - depth analysis over an immutable `SchedulingResult`.
//!
//! This module does NOT own:
//!
//! - quantum operation semantics;
//! - quantum qubit identities;
//! - routing;
//! - resource allocation;
//! - timing calibration;
//! - hardware discovery;
//! - scheduling algorithms;
//! - schedule construction;
//! - QEC algorithms;
//! - runtime execution;
//! - serialization formats.
//!
//! Those responsibilities belong to their canonical subsystems.
//!
//! # Canonical identities
//!
//! Operation identity comes from:
//!
//! ```text
//! crate::quantum::ir::core::identity::OperationId
//! ```
//!
//! Logical and physical qubit identities remain owned by:
//!
//! ```text
//! crate::quantum::ir::qubit
//! ```
//!
//! This module does not define a second qubit identity.
//!
//! # Universal scalability contract
//!
//! There are deliberately no constants such as:
//!
//! ```text
//! MAX_QUBITS
//! MAX_OPERATIONS
//! MAX_DEPTH
//! MAX_LAYERS
//! ```
//!
//! The algorithm operates on the operations actually present in the supplied
//! scheduling result.
//!
//! "Infinity" means that this module imposes no artificial finite quantum
//! machine-size ceiling. A concrete compilation is naturally bounded by the
//! available address space, input size, and execution resources.
//!
//! # Algorithm
//!
//! Depth is computed as:
//!
//! ```text
//! depth(operation) =
//!     1 + max(depth(predecessor))
//! ```
//!
//! for an operation with predecessors, and:
//!
//! ```text
//! depth(operation) = 1
//! ```
//!
//! for a root operation.
//!
//! The implementation uses an iterative topological traversal rather than
//! recursion. This is important for very large dependency graphs because the
//! call stack must not become proportional to circuit depth.
//!
//! Complexity:
//!
//! ```text
//! Time:  O(V + E)
//! Space: O(V + E)
//! ```
//!
//! where:
//!
//! - `V` = scheduled operation count;
//! - `E` = predecessor-edge count.
//!
//! No sorting proportional to the number of layers is required.
//!
//! # Correctness
//!
//! A valid scheduling result already records predecessor relationships.
//! This module therefore measures the schedule's dependency depth without
//! reconstructing quantum semantics or hardware topology.
//!
//! If a dependency refers to an operation absent from the result, analysis
//! fails explicitly instead of guessing.
//!
//! If the dependency relation is cyclic, analysis fails explicitly.
//!
//! # Determinism
//!
//! Ready operations are processed in deterministic `OperationId` order.
//!
//! This means identical scheduling results always produce identical depth
//! analysis.
//!
//! # Thread safety
//!
//! The objective contains no global state and no interior mutability.
//!
//! It only reads an immutable `SchedulingResult`.
//!
//! # Rust contract
//!
//! Supported:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no `unsafe` code.
//!
//! # Integration contract
//!
//! The dependency direction is:
//!
//! ```text
//! quantum::ir
//!      │
//!      ▼
//! scheduling::result
//!      │
//!      ▼
//! scheduling::optimization::depth
//!      │
//!      ├── benchmarking
//!      ├── diagnostics
//!      ├── multi-objective optimization
//!      └── scheduling planners
//! ```
//!
//! This module does not depend on a scheduler implementation.
//!
//! Any planner capable of producing a valid `SchedulingResult` can be
//! evaluated by this objective.
//!
//! # Important distinction
//!
//! This module calculates dependency depth, not physical wall-clock depth.
//!
//! Physical wall-clock performance is represented by scheduling time and
//! `SchedulingResult::metrics().makespan()`.
//!
//! A future objective that needs hardware-weighted temporal depth should use
//! the timing subsystem rather than changing the meaning of this type.
//!
//! # No `quantum::ir::qubit` import
//!
//! This objective does not need to inspect qubit identities. It operates on
//! canonical operation dependencies already present in the scheduling result.
//!
//! Importing `QubitId` here would therefore create unnecessary coupling.
//!
//! =============================================================================
//! Safety boundary
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use crate::quantum::ir::core::identity::OperationId;

use super::super::result::{ScheduledOperation, SchedulingResult};

// =============================================================================
// Errors
// =============================================================================

/// Errors produced while calculating schedule depth.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DepthError {
    /// A scheduled operation references a predecessor that is absent from the
    /// schedule result.
    MissingPredecessor {
        /// Operation containing the invalid dependency.
        operation: OperationId,

        /// Referenced predecessor.
        predecessor: OperationId,
    },

    /// The dependency relation contains a cycle.
    ///
    /// A production scheduling result should never contain this condition.
    /// The explicit error protects the objective from silently producing an
    /// incorrect depth when it is called with an invalid or analysis-only
    /// artifact.
    DependencyCycle {
        /// Number of operations that could not be topologically processed.
        remaining_operations: usize,
    },

    /// The calculated depth cannot be represented by `u128`.
    DepthOverflow,

    /// The caller supplied an inconsistent analysis state.
    InconsistentAnalysis {
        /// Human-readable explanation.
        message: String,
    },
}

impl fmt::Display for DepthError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingPredecessor {
                operation,
                predecessor,
            } => write!(
                formatter,
                "operation `{operation}` references missing predecessor `{predecessor}`"
            ),

            Self::DependencyCycle {
                remaining_operations,
            } => write!(
                formatter,
                "schedule dependency graph contains a cycle; \
                 {remaining_operations} operation(s) could not be processed"
            ),

            Self::DepthOverflow => {
                formatter.write_str("schedule depth exceeds representable u128 range")
            }

            Self::InconsistentAnalysis { message } => {
                write!(formatter, "inconsistent depth analysis: {message}")
            }
        }
    }
}

impl Error for DepthError {}

// =============================================================================
// Depth value
// =============================================================================

/// Number of dependency layers in a schedule.
///
/// Depth zero represents an empty schedule.
///
/// A non-empty schedule has a minimum depth of one.
///
/// `u128` is deliberately used instead of `usize` because depth is a semantic
/// value rather than a collection index. This prevents the semantic range
/// from depending on host pointer width.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct ScheduleDepth(u128);

impl ScheduleDepth {
    /// Depth of an empty schedule.
    pub const ZERO: Self = Self(0);

    /// Creates a depth value.
    ///
    /// The caller is responsible for ensuring that zero is used only for an
    /// empty schedule or an explicitly defined zero-depth analysis.
    #[must_use]
    pub const fn new(value: u128) -> Self {
        Self(value)
    }

    /// Returns the numeric depth.
    #[must_use]
    pub const fn value(self) -> u128 {
        self.0
    }

    /// Returns whether the depth is zero.
    #[must_use]
    pub const fn is_zero(self) -> bool {
        self.0 == 0
    }

    /// Returns the next depth value or `None` on overflow.
    #[must_use]
    pub const fn checked_increment(self) -> Option<Self> {
        match self.0.checked_add(1) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }

    /// Returns the greater of two depth values.
    #[must_use]
    pub const fn max(self, other: Self) -> Self {
        if self.0 >= other.0 {
            self
        } else {
            other
        }
    }
}

impl fmt::Display for ScheduleDepth {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl From<u128> for ScheduleDepth {
    fn from(value: u128) -> Self {
        Self::new(value)
    }
}

impl From<ScheduleDepth> for u128 {
    fn from(value: ScheduleDepth) -> Self {
        value.value()
    }
}

// =============================================================================
// Per-operation depth
// =============================================================================

/// Depth information for one scheduled operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OperationDepth {
    operation: OperationId,
    depth: ScheduleDepth,
}

impl OperationDepth {
    /// Creates an operation-depth record.
    #[must_use]
    pub const fn new(
        operation: OperationId,
        depth: ScheduleDepth,
    ) -> Self {
        Self { operation, depth }
    }

    /// Returns the operation identity.
    #[must_use]
    pub const fn operation(self) -> OperationId {
        self.operation
    }

    /// Returns the operation's dependency depth.
    #[must_use]
    pub const fn depth(self) -> ScheduleDepth {
        self.depth
    }
}

// =============================================================================
// Analysis result
// =============================================================================

/// Complete dependency-depth analysis of a scheduling result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DepthAnalysis {
    depth: ScheduleDepth,
    operation_count: u128,
    root_operation_count: u128,
    critical_operation_count: u128,
    operation_depths: BTreeMap<OperationId, ScheduleDepth>,
}

impl DepthAnalysis {
    /// Creates a depth analysis from validated values.
    #[must_use]
    pub fn new(
        depth: ScheduleDepth,
        operation_count: u128,
        root_operation_count: u128,
        critical_operation_count: u128,
        operation_depths: BTreeMap<OperationId, ScheduleDepth>,
    ) -> Self {
        Self {
            depth,
            operation_count,
            root_operation_count,
            critical_operation_count,
            operation_depths,
        }
    }

    /// Returns the total dependency depth.
    #[must_use]
    pub const fn depth(&self) -> ScheduleDepth {
        self.depth
    }

    /// Returns the number of scheduled operations.
    #[must_use]
    pub const fn operation_count(&self) -> u128 {
        self.operation_count
    }

    /// Returns the number of root operations.
    ///
    /// A root operation has no predecessors.
    #[must_use]
    pub const fn root_operation_count(&self) -> u128 {
        self.root_operation_count
    }

    /// Returns the number of operations belonging to the deepest layer.
    #[must_use]
    pub const fn critical_operation_count(&self) -> u128 {
        self.critical_operation_count
    }

    /// Returns the depth of one operation.
    #[must_use]
    pub fn operation_depth(
        &self,
        operation: OperationId,
    ) -> Option<ScheduleDepth> {
        self.operation_depths.get(&operation).copied()
    }

    /// Returns all per-operation depths in deterministic operation-ID order.
    #[must_use]
    pub fn operation_depths(
        &self,
    ) -> &BTreeMap<OperationId, ScheduleDepth> {
        &self.operation_depths
    }

    /// Returns an iterator over per-operation depth records.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = OperationDepth> + '_ {
        self.operation_depths
            .iter()
            .map(|(&operation, &depth)| {
                OperationDepth::new(operation, depth)
            })
    }
}

// =============================================================================
// Objective score
// =============================================================================

/// Score produced by the depth objective.
///
/// Lower depth is always better.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DepthScore {
    depth: ScheduleDepth,
}

impl DepthScore {
    /// Creates a depth score.
    #[must_use]
    pub const fn new(depth: ScheduleDepth) -> Self {
        Self { depth }
    }

    /// Returns the underlying depth.
    #[must_use]
    pub const fn depth(self) -> ScheduleDepth {
        self.depth
    }

    /// Returns `true` when this score is strictly better than `other`.
    ///
    /// Smaller depth wins.
    #[must_use]
    pub const fn better_than(self, other: Self) -> bool {
        self.depth < other.depth
    }

    /// Returns the better of two scores.
    ///
    /// Lower depth wins.
    #[must_use]
    pub const fn min(self, other: Self) -> Self {
        if self.depth <= other.depth {
            self
        } else {
            other
        }
    }
}

impl Default for DepthScore {
    fn default() -> Self {
        Self::new(ScheduleDepth::ZERO)
    }
}

impl fmt::Display for DepthScore {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "depth={}", self.depth)
    }
}

// =============================================================================
// Objective
// =============================================================================

/// Canonical scheduling objective for minimizing dependency depth.
///
/// This type is stateless and therefore cheap to construct and share.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct DepthObjective;

impl DepthObjective {
    /// Creates the depth objective.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Analyzes the dependency depth of a complete scheduling result.
    ///
    /// This method does not mutate the result.
    ///
    /// # Errors
    ///
    /// Returns `DepthError::MissingPredecessor` when a dependency points to
    /// an operation absent from the result.
    ///
    /// Returns `DepthError::DependencyCycle` when the dependency relation
    /// cannot be topologically processed.
    ///
    /// Returns `DepthError::DepthOverflow` if the semantic depth exceeds the
    /// representable `u128` range.
    pub fn analyze(
        &self,
        result: &SchedulingResult,
    ) -> Result<DepthAnalysis, DepthError> {
        analyze_operations(result.operations())
    }

    /// Evaluates a complete scheduling result and returns its objective score.
    ///
    /// Lower scores are better.
    pub fn evaluate(
        &self,
        result: &SchedulingResult,
    ) -> Result<DepthScore, DepthError> {
        Ok(DepthScore::new(self.analyze(result)?.depth()))
    }

    /// Compares two scheduling results by dependency depth.
    ///
    /// Returns:
    ///
    /// - `Ordering::Less` when `left` is better;
    /// - `Ordering::Equal` when both have the same depth;
    /// - `Ordering::Greater` when `right` is better.
    pub fn compare(
        &self,
        left: &SchedulingResult,
        right: &SchedulingResult,
    ) -> Result<std::cmp::Ordering, DepthError> {
        let left_score = self.evaluate(left)?;
        let right_score = self.evaluate(right)?;

        Ok(left_score.depth().cmp(&right_score.depth()))
    }

    /// Returns whether `candidate` has strictly lower depth than `reference`.
    pub fn is_better(
        &self,
        candidate: &SchedulingResult,
        reference: &SchedulingResult,
    ) -> Result<bool, DepthError> {
        let candidate_score = self.evaluate(candidate)?;
        let reference_score = self.evaluate(reference)?;

        Ok(candidate_score.better_than(reference_score))
    }
}

// =============================================================================
// Core analysis
// =============================================================================

/// Calculates dependency depth for a collection of scheduled operations.
///
/// The input does not need to be sorted.
///
/// The implementation performs an iterative Kahn-style topological traversal.
/// A `BTreeMap` is used for deterministic operation identity ordering.
///
/// # Complexity
///
/// ```text
/// O(V + E)
/// ```
///
/// time and `O(V + E)` storage.
///
/// The operation records are not cloned.
fn analyze_operations(
    operations: &[ScheduledOperation],
) -> Result<DepthAnalysis, DepthError> {
    if operations.is_empty() {
        return Ok(DepthAnalysis::new(
            ScheduleDepth::ZERO,
            0,
            0,
            0,
            BTreeMap::new(),
        ));
    }

    // -------------------------------------------------------------------------
    // Step 1: Index operations.
    // -------------------------------------------------------------------------
    //
    // The final SchedulingResult is already deterministic, but the algorithm
    // deliberately does not rely on vector position. OperationId is the
    // semantic identity.
    //
    // BTreeMap gives deterministic iteration and lookup behavior.

    let mut operation_index =
        BTreeMap::<OperationId, &ScheduledOperation>::new();

    for operation in operations {
        let operation_id = operation.operation_id();

        if operation_index
            .insert(operation_id, operation)
            .is_some()
        {
            return Err(DepthError::InconsistentAnalysis {
                message: format!(
                    "duplicate operation `{operation_id}` encountered during depth analysis"
                ),
            });
        }
    }

    // -------------------------------------------------------------------------
    // Step 2: Validate predecessors and construct successor adjacency.
    // -------------------------------------------------------------------------
    //
    // We need successors because Kahn traversal processes an operation and
    // then updates the depth of its dependent operations.
    //
    // We intentionally construct adjacency here instead of scanning every
    // operation for every predecessor. This preserves O(V + E) behavior.

    let mut successors =
        BTreeMap::<OperationId, Vec<OperationId>>::new();

    let mut indegree =
        BTreeMap::<OperationId, u128>::new();

    let mut depths =
        BTreeMap::<OperationId, ScheduleDepth>::new();

    let mut root_count = 0u128;

    for operation_id in operation_index.keys().copied() {
        successors.insert(operation_id, Vec::new());
        indegree.insert(operation_id, 0);
        depths.insert(operation_id, ScheduleDepth::ZERO);
    }

    for operation in operations {
        let operation_id = operation.operation_id();

        if operation.predecessors().is_empty() {
            root_count = root_count
                .checked_add(1)
                .ok_or(DepthError::DepthOverflow)?;
        }

        for predecessor in operation.predecessors() {
            if !operation_index.contains_key(predecessor) {
                return Err(DepthError::MissingPredecessor {
                    operation: operation_id,
                    predecessor: *predecessor,
                });
            }

            let successor_list = successors
                .get_mut(predecessor)
                .ok_or_else(|| {
                    DepthError::InconsistentAnalysis {
                        message: format!(
                            "predecessor `{predecessor}` was indexed but \
                             has no successor entry"
                        ),
                    }
                })?;

            successor_list.push(operation_id);

            let degree = indegree
                .get_mut(&operation_id)
                .ok_or_else(|| {
                    DepthError::InconsistentAnalysis {
                        message: format!(
                            "operation `{operation_id}` was indexed but \
                             has no indegree entry"
                        ),
                    }
                })?;

            *degree = degree
                .checked_add(1)
                .ok_or(DepthError::DepthOverflow)?;
        }
    }

    // -------------------------------------------------------------------------
    // Step 3: Sort successor lists.
    // -------------------------------------------------------------------------
    //
    // Operation IDs provide deterministic arbitration.
    //
    // Sorting is performed independently for each adjacency list. This does
    // not impose a machine-size limit and is only a deterministic traversal
    // requirement.

    for list in successors.values_mut() {
        list.sort();
    }

    // -------------------------------------------------------------------------
    // Step 4: Construct deterministic ready set.
    // -------------------------------------------------------------------------
    //
    // A BTreeSet provides:
    //
    // - deterministic operation ordering;
    // - logarithmic insertion/removal;
    // - no hidden randomness;
    // - no dependency on hash iteration order.
    //
    // There is intentionally no fixed capacity.

    let mut ready = BTreeSet::<OperationId>::new();

    for (&operation_id, &degree) in &indegree {
        if degree == 0 {
            ready.insert(operation_id);
        }
    }

    // -------------------------------------------------------------------------
    // Step 5: Process the dependency DAG.
    // -------------------------------------------------------------------------

    let mut processed = 0u128;
    let mut maximum_depth = ScheduleDepth::ZERO;

    while let Some(operation_id) = ready.pop_first() {
        processed = processed
            .checked_add(1)
            .ok_or(DepthError::DepthOverflow)?;

        let current_depth = {
            let value = depths
                .get(&operation_id)
                .copied()
                .ok_or_else(|| {
                    DepthError::InconsistentAnalysis {
                        message: format!(
                            "operation `{operation_id}` has no depth entry"
                        ),
                    }
                })?;

            // Root operations begin at depth one.
            //
            // Non-root operations receive their value from predecessor
            // propagation below.
            if value.is_zero() {
                ScheduleDepth::new(1)
            } else {
                value
            }
        };

        if let Some(entry) = depths.get_mut(&operation_id) {
            *entry = current_depth;
        } else {
            return Err(DepthError::InconsistentAnalysis {
                message: format!(
                    "operation `{operation_id}` has no mutable depth entry"
                ),
            });
        }

        maximum_depth = maximum_depth.max(current_depth);

        let dependent_operations =
            successors
                .get(&operation_id)
                .ok_or_else(|| {
                    DepthError::InconsistentAnalysis {
                        message: format!(
                            "operation `{operation_id}` has no successor entry"
                        ),
                    }
                })?;

        for &successor in dependent_operations {
            // A successor's depth must be at least one layer after the
            // current operation.
            let candidate_depth = current_depth
                .checked_increment()
                .ok_or(DepthError::DepthOverflow)?;

            let successor_depth =
                depths
                    .get_mut(&successor)
                    .ok_or_else(|| {
                        DepthError::InconsistentAnalysis {
                            message: format!(
                                "successor `{successor}` has no depth entry"
                            ),
                        }
                    })?;

            *successor_depth =
                successor_depth.max(candidate_depth);

            let successor_indegree =
                indegree
                    .get_mut(&successor)
                    .ok_or_else(|| {
                        DepthError::InconsistentAnalysis {
                            message: format!(
                                "successor `{successor}` has no indegree entry"
                            ),
                        }
                    })?;

            *successor_indegree = successor_indegree
                .checked_sub(1)
                .ok_or_else(|| {
                    DepthError::InconsistentAnalysis {
                        message: format!(
                            "indegree underflow for successor `{successor}`"
                        ),
                    }
                })?;

            if *successor_indegree == 0 {
                ready.insert(successor);
            }
        }
    }

    // -------------------------------------------------------------------------
    // Step 6: Detect cycles.
    // -------------------------------------------------------------------------
    //
    // In a DAG every operation must eventually leave the ready set.
    //
    // A cycle means at least one operation retains non-zero indegree.

    let operation_count = operation_index.len() as u128;

    if processed != operation_count {
        let remaining = operation_index
            .len()
            .saturating_sub(processed as usize);

        return Err(DepthError::DependencyCycle {
            remaining_operations: remaining,
        });
    }

    // -------------------------------------------------------------------------
    // Step 7: Count deepest-layer operations.
    // -------------------------------------------------------------------------

    let mut critical_operation_count = 0u128;

    for depth in depths.values().copied() {
        if depth == maximum_depth {
            critical_operation_count = critical_operation_count
                .checked_add(1)
                .ok_or(DepthError::DepthOverflow)?;
        }
    }

    Ok(DepthAnalysis::new(
        maximum_depth,
        operation_count,
        root_count,
        critical_operation_count,
        depths,
    ))
}

// =============================================================================
// Convenience functions
// =============================================================================

/// Calculates schedule dependency depth.
///
/// This is the convenience API for callers that only need the depth value.
pub fn calculate_depth(
    result: &SchedulingResult,
) -> Result<ScheduleDepth, DepthError> {
    DepthObjective::new().evaluate(result).map(DepthScore::depth)
}

/// Performs complete depth analysis.
///
/// This is useful to diagnostics, benchmarking, and multi-objective
/// optimization consumers that need more than the scalar objective.
pub fn analyze_depth(
    result: &SchedulingResult,
) -> Result<DepthAnalysis, DepthError> {
    DepthObjective::new().analyze(result)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::quantum::ir::core::identity::OperationId;
    use crate::quantum::scheduling::result::{
        ResultProvenance,
        ScheduledOperation,
        SchedulingResultBuilder,
        VerificationSummary,
    };
    use crate::quantum::scheduling::types::{
        EpochId,
        IrVersion,
        ScheduleId,
        SchedulerSessionId,
    };

    fn operation(
        id: u64,
    ) -> ScheduledOperation {
        ScheduledOperation::new(
            OperationId::new(id),
            crate::quantum::scheduling::types::TimePoint::ZERO,
            crate::quantum::scheduling::types::TimePoint::ZERO,
        )
        .expect("test operation must be valid")
    }

    fn operation_with_predecessor(
        id: u64,
        predecessor: u64,
    ) -> ScheduledOperation {
        let mut value = operation(id);

        value.add_predecessor(OperationId::new(predecessor));

        value
    }

    fn provenance() -> ResultProvenance {
        ResultProvenance::new(
            IrVersion::new(1),
            SchedulerSessionId::new(1),
            EpochId::new(1),
            ScheduleId::new(1),
            "test",
            "test",
            "depth",
            true,
            Some(1),
        )
    }

    fn result(
        operations: Vec<ScheduledOperation>,
    ) -> SchedulingResult {
        let mut builder = SchedulingResultBuilder::new()
            .with_provenance(provenance());

        for operation in operations {
            builder
                .add_operation(operation)
                .expect("test operation must be accepted");
        }

        builder
            .set_verification(VerificationSummary::default());

        builder
            .mark_analysis_only()
            .expect("analysis-only transition must succeed");

        builder
            .build()
            .expect("test result must build")
    }

    #[test]
    fn empty_schedule_has_zero_depth() {
        let value = result(Vec::new());

        let analysis =
            analyze_depth(&value).expect("empty schedule must analyze");

        assert_eq!(analysis.depth(), ScheduleDepth::ZERO);
        assert_eq!(analysis.operation_count(), 0);
        assert_eq!(analysis.root_operation_count(), 0);
        assert_eq!(analysis.critical_operation_count(), 0);
    }

    #[test]
    fn single_operation_has_depth_one() {
        let value = result(vec![operation(1)]);

        let analysis =
            analyze_depth(&value).expect("single operation must analyze");

        assert_eq!(analysis.depth(), ScheduleDepth::new(1));
        assert_eq!(analysis.operation_count(), 1);
        assert_eq!(analysis.root_operation_count(), 1);
        assert_eq!(analysis.critical_operation_count(), 1);
    }

    #[test]
    fn independent_operations_share_one_layer() {
        let value = result(vec![
            operation(1),
            operation(2),
            operation(3),
        ]);

        let analysis =
            analyze_depth(&value)
                .expect("parallel operations must analyze");

        assert_eq!(analysis.depth(), ScheduleDepth::new(1));
        assert_eq!(analysis.root_operation_count(), 3);
        assert_eq!(analysis.critical_operation_count(), 3);
    }

    #[test]
    fn sequential_chain_has_linear_depth() {
        let value = result(vec![
            operation(1),
            operation_with_predecessor(2, 1),
            operation_with_predecessor(3, 2),
            operation_with_predecessor(4, 3),
        ]);

        let analysis =
            analyze_depth(&value)
                .expect("sequential chain must analyze");

        assert_eq!(analysis.depth(), ScheduleDepth::new(4));
        assert_eq!(
            analysis.operation_depth(OperationId::new(1)),
            Some(ScheduleDepth::new(1))
        );
        assert_eq!(
            analysis.operation_depth(OperationId::new(2)),
            Some(ScheduleDepth::new(2))
        );
        assert_eq!(
            analysis.operation_depth(OperationId::new(3)),
            Some(ScheduleDepth::new(3))
        );
        assert_eq!(
            analysis.operation_depth(OperationId::new(4)),
            Some(ScheduleDepth::new(4))
        );
    }

    #[test]
    fn diamond_graph_has_depth_three() {
        let value = result(vec![
            operation(1),
            operation_with_predecessor(2, 1),
            operation_with_predecessor(3, 1),
            operation_with_predecessor(4, 2),
            operation_with_predecessor(4, 3),
        ]);

        let analysis =
            analyze_depth(&value)
                .expect("diamond graph must analyze");

        assert_eq!(analysis.depth(), ScheduleDepth::new(3));
        assert_eq!(analysis.root_operation_count(), 1);
        assert_eq!(analysis.critical_operation_count(), 1);
    }

    #[test]
    fn missing_predecessor_is_rejected() {
        let value = result(vec![
            operation_with_predecessor(2, 99),
        ]);

        let error =
            analyze_depth(&value)
                .expect_err("missing predecessor must fail");

        assert_eq!(
            error,
            DepthError::MissingPredecessor {
                operation: OperationId::new(2),
                predecessor: OperationId::new(99),
            }
        );
    }

    #[test]
    fn operation_depths_are_deterministic() {
        let value = result(vec![
            operation_with_predecessor(4, 2),
            operation(1),
            operation_with_predecessor(3, 1),
            operation(2),
        ]);

        let first =
            analyze_depth(&value).expect("first analysis must succeed");

        let second =
            analyze_depth(&value).expect("second analysis must succeed");

        assert_eq!(first, second);
    }

    #[test]
    fn objective_prefers_lower_depth() {
        let shallow = result(vec![
            operation(1),
            operation(2),
        ]);

        let deep = result(vec![
            operation(1),
            operation_with_predecessor(2, 1),
        ]);

        let objective = DepthObjective::new();

        assert!(
            objective
                .is_better(&shallow, &deep)
                .expect("comparison must succeed")
        );

        assert!(
            !objective
                .is_better(&deep, &shallow)
                .expect("comparison must succeed")
        );
    }

    #[test]
    fn score_comparison_is_lower_is_better() {
        let shallow = DepthScore::new(ScheduleDepth::new(2));
        let deep = DepthScore::new(ScheduleDepth::new(5));

        assert!(shallow.better_than(deep));
        assert!(!deep.better_than(shallow));
        assert_eq!(shallow.min(deep), shallow);
    }

    #[test]
    fn zero_depth_is_only_empty_in_normal_analysis() {
        let value = result(Vec::new());

        let analysis =
            analyze_depth(&value).expect("empty result must analyze");

        assert!(analysis.depth().is_zero());
    }
}