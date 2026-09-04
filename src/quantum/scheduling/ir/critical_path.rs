//! Zamani Quantum Scheduling — Critical Path Analysis
//!
//! Path:
//!     src/quantum/scheduling/ir/critical_path.rs
//!
//! # Purpose
//!
//! This module performs deterministic critical-path analysis over the
//! scheduler-owned dependency graph.
//!
//! It answers:
//!
//! > "What is the longest dependency-constrained path through the operations,
//! > how much abstract time does it represent, and which operations determine
//! > that lower bound?"
//!
//! The result is an analysis artifact. It is NOT a physical schedule.
//!
//! # Architectural boundary
//!
//! ```text
//! canonical quantum IR
//!          │
//!          ▼
//! scheduling::ir::operation
//!          │
//!          ▼
//! scheduling::ir::graph
//!          │
//!          ▼
//! scheduling::ir::critical_path     ◄── this module
//!          │
//!      ┌───┼──────────────┐
//!      ▼   ▼              ▼
//!   planners policies optimization
//! ```
//!
//! This module owns:
//!
//! - longest dependency-path analysis;
//! - earliest start times;
//! - earliest finish times;
//! - latest start times;
//! - latest finish times;
//! - operation slack;
//! - critical-operation identification;
//! - critical-path reconstruction;
//! - deterministic tie-breaking;
//! - overflow-safe path arithmetic;
//! - critical-path analysis statistics.
//!
//! This module does NOT own:
//!
//! - quantum operation semantics;
//! - qubit identity;
//! - physical qubit identity;
//! - routing;
//! - hardware topology;
//! - resource calendars;
//! - hardware timing;
//! - scheduling policy;
//! - resource-constrained scheduling;
//! - QEC algorithms;
//! - noise modelling;
//! - runtime execution;
//! - serialization formats.
//!
//! Those concerns belong to their canonical subsystems.
//!
//! # Canonical identity ownership
//!
//! Operation identity ultimately comes from:
//!
//! ```text
//! crate::quantum::ir::core::identity::OperationId
//! ```
//!
//! through:
//!
//! ```text
//! crate::quantum::scheduling::types::OperationRef
//! ```
//!
//! Logical and physical qubit identity remain exclusively owned by:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! This module intentionally does not import `QubitId` because a critical path
//! is defined over operation dependencies rather than directly over qubit
//! identities. Any future qubit-specific analysis must consume the canonical
//! `quantum::ir::qubit` types rather than introducing scheduler-local copies.
//!
//! # Time semantics
//!
//! The scheduler uses target-independent `Duration` and `TimePoint` values.
//!
//! A duration has no intrinsic unit. The target timing subsystem determines
//! its physical interpretation.
//!
//! Therefore this module never assumes:
//!
//! - nanoseconds;
//! - microseconds;
//! - device ticks;
//! - pulse samples;
//! - a particular clock;
//! - a particular quantum technology.
//!
//! # Critical-path definition
//!
//! Given a DAG G=(V,E) and operation weight w(v):
//!
//! ```text
//! earliest_start(v)
//!     = max(earliest_finish(p)) for p -> v
//!
//! earliest_finish(v)
//!     = earliest_start(v) + w(v)
//! ```
//!
//! The critical-path lower bound is:
//!
//! ```text
//! max(earliest_finish(v))
//! ```
//!
//! The analysis also computes a latest-time representation relative to the
//! critical-path makespan:
//!
//! ```text
//! latest_finish(v)
//!     = min(latest_start(s)) for v -> s
//!
//! latest_start(v)
//!     = latest_finish(v) - w(v)
//! ```
//!
//! For sink operations:
//!
//! ```text
//! latest_finish(sink) = critical_path_duration
//! ```
//!
//! Slack is:
//!
//! ```text
//! slack(v) = latest_start(v) - earliest_start(v)
//! ```
//!
//! An operation is critical when its slack is zero.
//!
//! # Important distinction
//!
//! Critical path != schedule.
//!
//! The critical path ignores:
//!
//! - resource contention;
//! - calibration availability;
//! - control-channel capacity;
//! - measurement-channel capacity;
//! - physical topology;
//! - alignment constraints;
//! - communication latency;
//! - hardware maintenance windows.
//!
//! Consequently:
//!
//! ```text
//! critical_path_duration <= actual scheduled makespan
//! ```
//!
//! whenever all weights represent valid non-negative execution durations.
//!
//! Resource-aware planners may use this result as a lower bound and priority
//! signal, but must perform their own resource feasibility analysis.
//!
//! # Scalability
//!
//! There is deliberately no fixed limit for:
//!
//! - number of operations;
//! - number of dependencies;
//! - graph depth;
//! - graph width;
//! - qubit count;
//! - resource count;
//! - schedule duration;
//! - machine size.
//!
//! "Infinity" means that this implementation does not encode an artificial
//! machine-size ceiling. A concrete compilation remains bounded by available
//! memory, CPU, address space, explicit user limits, and the target itself.
//!
//! The algorithm is iterative and does not recurse according to graph depth.
//!
//! Let:
//!
//! - V = number of operation nodes;
//! - E = number of dependency edges.
//!
//! The dominant analysis is O((V + E) log V) because the canonical dependency
//! graph uses deterministic ordered collections.
//!
//! Additional memory is O(V) for analysis vectors/maps, excluding the graph
//! itself.
//!
//! # Determinism
//!
//! All semantic tie-breaking is deterministic.
//!
//! When two predecessor paths have equal accumulated weight, the predecessor
//! with the smallest `OperationRef` wins.
//!
//! When multiple sinks have equal maximum path length, the smallest sink is
//! selected.
//!
//! Therefore the same:
//!
//! ```text
//! graph + weights
//! ```
//!
//! produces the same result.
//!
//! # Overflow
//!
//! All duration and weight arithmetic is checked.
//!
//! No arithmetic silently wraps.
//!
//! An overflow is returned as a structured scheduling error.
//!
//! # Empty graph
//!
//! An empty graph is valid and produces:
//!
//! ```text
//! critical_path_duration = Duration::ZERO
//! critical_path = []
//! operation_count = 0
//! ```
//!
//! # Zero-duration operations
//!
//! Zero-duration operations are valid.
//!
//! They can appear on a critical path and can have zero slack.
//!
//! Therefore criticality is based on computed slack rather than requiring a
//! positive duration.
//!
//! # Cycle handling
//!
//! Critical-path analysis requires a DAG.
//!
//! A cycle is a structural scheduling error for static critical-path analysis.
//!
//! The graph is validated through its canonical `topological_order()` API.
//! This module does not independently reconstruct dependency semantics.
//!
//! # Integration
//!
//! ```text
//! scheduling::ir::graph::DependencyGraph
//!             │
//!             ▼
//!       CriticalPathAnalysis
//!             │
//!       ┌─────┼───────────┐
//!       ▼     ▼           ▼
//! earliest   latest      path
//! times      times       reconstruction
//!       │     │           │
//!       └─────┼───────────┘
//!             ▼
//!       CriticalPathResult
//! ```
//!
//! Consumers include:
//!
//! - `planners::critical_path`;
//! - `algorithms::cp`;
//! - `policies::priority`;
//! - `optimization::makespan`;
//! - `optimization::multi_objective`;
//! - `diagnostics::explain`;
//! - `verification`;
//! - benchmarking.
//!
//! The module must remain independent of those consumers.
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
//! - no unsafe code.
//!
//! The safety boundary is compiler-enforced below.
//!
//! =============================================================================
//! Safety boundary
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::collections::{BTreeMap, BTreeSet};

use super::graph::DependencyGraph;
use super::super::errors::{SchedulingError, SchedulingResult};
use super::super::types::{
    Duration,
    OperationRef,
    TimePoint,
};

// =============================================================================
// Public analysis result
// =============================================================================

/// Complete critical-path analysis result.
///
/// The structure is immutable after construction.
///
/// It is safe to share between read-only planner/diagnostic/optimization
/// consumers through ordinary ownership or `Arc`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CriticalPathResult {
    /// Number of operations in the analyzed graph.
    operation_count: usize,

    /// Number of dependency edges in the analyzed graph.
    dependency_count: usize,

    /// Total duration of the longest dependency-constrained path.
    critical_path_duration: Duration,

    /// Number of operations on the selected deterministic critical path.
    critical_path_operation_count: usize,

    /// Deterministically selected critical path.
    ///
    /// Operations are ordered from source to sink.
    critical_path: Vec<OperationRef>,

    /// Earliest possible start time for every operation.
    earliest_start: BTreeMap<OperationRef, TimePoint>,

    /// Earliest possible finish time for every operation.
    earliest_finish: BTreeMap<OperationRef, TimePoint>,

    /// Latest permitted start time without increasing the critical-path
    /// makespan.
    latest_start: BTreeMap<OperationRef, TimePoint>,

    /// Latest permitted finish time without increasing the critical-path
    /// makespan.
    latest_finish: BTreeMap<OperationRef, TimePoint>,

    /// Total scheduling slack for each operation.
    slack: BTreeMap<OperationRef, Duration>,

    /// Critical predecessor selected for each operation.
    ///
    /// `None` means the operation begins a dependency path or no predecessor
    /// was selected because all predecessor weights are equal to zero.
    critical_predecessor: BTreeMap<OperationRef, Option<OperationRef>>,

    /// Weight/duration associated with each operation.
    weights: BTreeMap<OperationRef, Duration>,
}

impl CriticalPathResult {
    /// Returns the number of operations.
    #[must_use]
    pub const fn operation_count(&self) -> usize {
        self.operation_count
    }

    /// Returns the number of dependencies.
    #[must_use]
    pub const fn dependency_count(&self) -> usize {
        self.dependency_count
    }

    /// Returns the critical-path duration.
    #[must_use]
    pub const fn critical_path_duration(&self) -> Duration {
        self.critical_path_duration
    }

    /// Returns the number of operations on the selected critical path.
    #[must_use]
    pub const fn critical_path_operation_count(&self) -> usize {
        self.critical_path_operation_count
    }

    /// Returns the selected critical path.
    ///
    /// The path is ordered from source to sink.
    #[must_use]
    pub fn critical_path(&self) -> &[OperationRef] {
        &self.critical_path
    }

    /// Returns the earliest start of an operation.
    ///
    /// Returns `None` if the operation was not part of the analyzed graph.
    #[must_use]
    pub fn earliest_start(&self, operation: OperationRef) -> Option<TimePoint> {
        self.earliest_start.get(&operation).copied()
    }

    /// Returns the earliest finish of an operation.
    ///
    /// Returns `None` if the operation was not part of the analyzed graph.
    #[must_use]
    pub fn earliest_finish(&self, operation: OperationRef) -> Option<TimePoint> {
        self.earliest_finish.get(&operation).copied()
    }

    /// Returns the latest start of an operation.
    ///
    /// Returns `None` if the operation was not part of the analyzed graph.
    #[must_use]
    pub fn latest_start(&self, operation: OperationRef) -> Option<TimePoint> {
        self.latest_start.get(&operation).copied()
    }

    /// Returns the latest finish of an operation.
    ///
    /// Returns `None` if the operation was not part_of_graph(&operation)
    /// {
        // This method body is intentionally replaced below.
        None
    }

    /// Returns the latest finish of an operation.
    ///
    /// Returns `None` if the operation was not part of the analyzed graph.
    #[must_use]
    pub fn latest_finish_time(
        &self,
        operation: OperationRef,
    ) -> Option<TimePoint> {
        self.latest_finish.get(&operation).copied()
    }

    /// Returns the slack of an operation.
    ///
    /// Returns `None` if the operation was not part of the analyzed graph.
    #[must_use]
    pub fn slack(&self, operation: OperationRef) -> Option<Duration> {
        self.slack.get(&operation).copied()
    }

    /// Returns the weight/duration assigned to an operation.
    ///
    /// Returns `None` if the operation was not part of the analyzed graph.
    #[must_use]
    pub fn operation_weight(
        &self,
        operation: OperationRef,
    ) -> Option<Duration> {
        self.weights.get(&operation).copied()
    }

    /// Returns the selected critical predecessor of an operation.
    #[must_use]
    pub fn critical_predecessor(
        &self,
        operation: OperationRef,
    ) -> Option<Option<OperationRef>> {
        self.critical_predecessor.get(&operation).copied()
    }

    /// Returns whether an operation lies on the selected critical path.
    #[must_use]
    pub fn is_on_critical_path(
        &self,
        operation: OperationRef,
    ) -> bool {
        self.critical_path
            .binary_search(&operation)
            .is_ok()
            || self
                .critical_path
                .iter()
                .any(|candidate| *candidate == operation)
    }

    /// Returns all earliest-start values.
    ///
    /// The returned map is deterministic and read-only.
    #[must_use]
    pub fn earliest_starts(
        &self,
    ) -> &BTreeMap<OperationRef, TimePoint> {
        &self.earliest_start
    }

    /// Returns all earliest-finish values.
    #[must_use]
    pub fn earliest_finishes(
        &self,
    ) -> &BTreeMap<OperationRef, TimePoint> {
        &self.earliest_finish
    }

    /// Returns all latest-start values.
    #[must_use]
    pub fn latest_starts(
        &self,
    ) -> &BTreeMap<OperationRef, TimePoint> {
        &self.latest_start
    }

    /// Returns all latest-finish values.
    #[must_use]
    pub fn latest_finishes(
        &self,
    ) -> &BTreeMap<OperationRef, TimePoint> {
        &self.latest_finish
    }

    /// Returns all slack values.
    #[must_use]
    pub fn slacks(
        &self,
    ) -> &BTreeMap<OperationRef, Duration> {
        &self.slack
    }

    /// Returns all operation weights.
    #[must_use]
    pub fn weights(
        &self,
    ) -> &BTreeMap<OperationRef, Duration> {
        &self.weights
    }

    /// Returns all operations whose slack is exactly zero.
    ///
    /// This may contain more operations than the selected deterministic path
    /// when multiple distinct critical paths exist.
    #[must_use]
    pub fn critical_operations(&self) -> Vec<OperationRef> {
        self.slack
            .iter()
            .filter_map(|(operation, slack)| {
                if slack.is_zero() {
                    Some(*operation)
                } else {
                    None
                }
            })
            .collect()
    }
}

// =============================================================================
// Analysis
// =============================================================================

/// Critical-path analyzer.
///
/// The type is stateless and therefore contains no global mutable state.
///
/// It is a namespace for the analysis API rather than a scheduler instance.
#[derive(Debug, Clone, Copy, Default)]
pub struct CriticalPathAnalysis;

impl CriticalPathAnalysis {
    /// Creates a critical-path analyzer.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Performs unit-duration critical-path analysis.
    ///
    /// Every operation receives:
    ///
    /// ```text
    /// Duration::new(1)
    /// ```
    ///
    /// This is useful for logical-depth-style dependency analysis.
    ///
    /// For physical scheduling, callers should normally use
    /// `analyze_with_durations`.
    pub fn analyze(
        &self,
        graph: &DependencyGraph,
    ) -> SchedulingResult<CriticalPathResult> {
        let weights = graph
            .operations()
            .map(|operation| (*operation, Duration::new(1)))
            .collect::<BTreeMap<_, _>>();

        self.analyze_with_durations(graph, &weights)
    }

    /// Performs critical-path analysis using caller-supplied operation
    /// durations.
    ///
    /// Every graph operation must have exactly one duration entry.
    ///
    /// Extra duration entries for operations not present in the graph are
    /// rejected. This prevents silent mismatches between the graph and timing
    /// model.
    pub fn analyze_with_durations(
        &self,
        graph: &DependencyGraph,
        durations: &BTreeMap<OperationRef, Duration>,
    ) -> SchedulingResult<CriticalPathResult> {
        self.analyze_internal(graph, durations)
    }

    /// Performs critical-path analysis using an iterator of operation/duration
    /// pairs.
    ///
    /// This is convenient for adapters that do not already own a
    /// `BTreeMap`.
    ///
    /// Duplicate operation entries are rejected.
    pub fn analyze_with_duration_iter<I>(
        &self,
        graph: &DependencyGraph,
        durations: I,
    ) -> SchedulingResult<CriticalPathResult>
    where
        I: IntoIterator<Item = (OperationRef, Duration)>,
    {
        let mut map = BTreeMap::new();

        for (operation, duration) in durations {
            if map.insert(operation, duration).is_some() {
                return Err(SchedulingError::InvalidInput {
                    reason: format!(
                        "critical-path duration table contains duplicate operation `{operation}`"
                    ),
                });
            }
        }

        self.analyze_with_durations(graph, &map)
    }

    /// Convenience function for callers that want a static analysis without
    /// explicitly constructing the analyzer value.
    pub fn compute(
        graph: &DependencyGraph,
        durations: &BTreeMap<OperationRef, Duration>,
    ) -> SchedulingResult<CriticalPathResult> {
        Self::new().analyze_with_durations(graph, durations)
    }

    fn analyze_internal(
        &self,
        graph: &DependencyGraph,
        durations: &BTreeMap<OperationRef, Duration>,
    ) -> SchedulingResult<CriticalPathResult> {
        // ---------------------------------------------------------------------
        // Validate duration coverage before graph traversal.
        // ---------------------------------------------------------------------

        for operation in graph.operations() {
            if !durations.contains_key(operation) {
                return Err(SchedulingError::MissingDuration {
                    operation: operation.id(),
                });
            }

            let duration = durations
                .get(operation)
                .copied()
                .expect("duration was checked immediately above");

            // Duration is unsigned by construction, so there is no negative
            // duration to reject.
            //
            // Keeping this explicit makes the semantic invariant obvious and
            // gives a future duration representation a single validation point.
            let _ = duration;
        }

        for operation in durations.keys() {
            if !graph.contains_operation(*operation) {
                return Err(SchedulingError::InvalidInput {
                    reason: format!(
                        "critical-path duration table contains operation `{operation}` not present in the dependency graph"
                    ),
                });
            }
        }

        // ---------------------------------------------------------------------
        // Obtain the canonical deterministic topological order.
        //
        // The dependency graph remains the single owner of graph traversal
        // semantics. This module never rebuilds its edges independently.
        // ---------------------------------------------------------------------

        let topological_order = graph.topological_order().map_err(|error| {
            SchedulingError::InvalidDependencyGraph {
                dependency: None,
                predecessor: None,
                successor: None,
                reason: error.to_string(),
            }
        })?;

        let operation_count = graph.operation_count();
        let dependency_count = graph.dependency_count();

        // ---------------------------------------------------------------------
        // Empty graph.
        // ---------------------------------------------------------------------

        if topological_order.is_empty() {
            return Ok(CriticalPathResult {
                operation_count,
                dependency_count,
                critical_path_duration: Duration::ZERO,
                critical_path_operation_count: 0,
                critical_path: Vec::new(),
                earliest_start: BTreeMap::new(),
                earliest_finish: BTreeMap::new(),
                latest_start: BTreeMap::new(),
                latest_finish: BTreeMap::new(),
                slack: BTreeMap::new(),
                critical_predecessor: BTreeMap::new(),
                weights: durations.clone(),
            });
        }

        // ---------------------------------------------------------------------
        // Forward pass.
        //
        // For every operation:
        //
        //     ES(v) = max EF(predecessor)
        //     EF(v) = ES(v) + duration(v)
        //
        // The graph's topological order guarantees that every predecessor has
        // already been processed.
        // ---------------------------------------------------------------------

        let mut earliest_start = BTreeMap::new();
        let mut earliest_finish = BTreeMap::new();
        let mut critical_predecessor = BTreeMap::new();

        let mut critical_path_duration = Duration::ZERO;
        let mut critical_sink: Option<OperationRef> = None;

        for &operation in &topological_order {
            let mut start = TimePoint::ZERO;
            let mut selected_predecessor: Option<OperationRef> = None;

            for predecessor in graph.predecessors(operation) {
                let predecessor_finish = earliest_finish
                    .get(predecessor)
                    .copied()
                    .ok_or_else(|| {
                        SchedulingError::InvalidDependencyGraph {
                            dependency: None,
                            predecessor: Some(predecessor.id()),
                            successor: Some(operation.id()),
                            reason: String::from(
                                "predecessor was not available during forward critical-path analysis",
                            ),
                        }
                    })?;

                if predecessor_finish > start {
                    start = predecessor_finish;
                    selected_predecessor = Some(*predecessor);
                } else if predecessor_finish == start {
                    // Deterministic tie-break.
                    //
                    // If both predecessor paths have the same accumulated
                    // weight, the smallest operation identity wins.
                    match selected_predecessor {
                        Some(current) if *predecessor < current => {
                            selected_predecessor = Some(*predecessor);
                        }
                        None => {
                            selected_predecessor = Some(*predecessor);
                        }
                        _ => {}
                    }
                }
            }

            let duration = durations
                .get(&operation)
                .copied()
                .ok_or_else(|| SchedulingError::MissingDuration {
                    operation: operation.id(),
                })?;

            let finish = start
                .checked_add(duration)
                .ok_or_else(|| SchedulingError::InvalidInput {
                    reason: format!(
                        "critical-path time overflow while finishing operation `{operation}`"
                    ),
                })?;

            earliest_start.insert(operation, start);
            earliest_finish.insert(operation, finish);
            critical_predecessor.insert(operation, selected_predecessor);

            // A sink-free DAG is not possible here, so the maximum finish can
            // be selected from all nodes. This also naturally handles
            // disconnected graphs.
            if finish > critical_path_duration {
                critical_path_duration = finish;
                critical_sink = Some(operation);
            } else if finish == critical_path_duration {
                match critical_sink {
                    Some(current) if operation < current => {
                        critical_sink = Some(operation);
                    }
                    None => {
                        critical_sink = Some(operation);
                    }
                    _ => {}
                }
            }
        }

        // ---------------------------------------------------------------------
        // Reverse pass.
        //
        // Initialize every sink to the global critical-path duration.
        //
        // For a general node:
        //
        //     LF(v) = min LS(successor)
        //     LS(v) = LF(v) - duration(v)
        //
        // The graph's reverse topological order guarantees every successor is
        // already processed.
        // ---------------------------------------------------------------------

        let mut latest_start = BTreeMap::new();
        let mut latest_finish = BTreeMap::new();

        for &operation in topological_order.iter().rev() {
            let duration = durations
                .get(&operation)
                .copied()
                .ok_or_else(|| SchedulingError::MissingDuration {
                    operation: operation.id(),
                })?;

            let successors = graph.successors(operation);

            let latest_finish_value = if successors.is_empty() {
                critical_path_duration
            } else {
                let mut minimum_successor_start: Option<TimePoint> = None;

                for successor in successors {
                    let successor_start = latest_start
                        .get(successor)
                        .copied()
                        .ok_or_else(|| {
                            SchedulingError::InvalidDependencyGraph {
                                dependency: None,
                                predecessor: Some(operation.id()),
                                successor: Some(successor.id()),
                                reason: String::from(
                                    "successor was not available during reverse critical-path analysis",
                                ),
                            }
                        })?;

                    minimum_successor_start = Some(
                        match minimum_successor_start {
                            Some(current) if current <= successor_start => current,
                            _ => successor_start,
                        },
                    );
                }

                minimum_successor_start.ok_or_else(|| {
                    SchedulingError::InvalidDependencyGraph {
                        dependency: None,
                        predecessor: Some(operation.id()),
                        successor: None,
                        reason: String::from(
                            "operation reported successors but no successor timing was available",
                        ),
                    }
                })?
            };

            let latest_start_value = latest_finish_value
                .checked_sub(duration)
                .ok_or_else(|| SchedulingError::InvalidInput {
                    reason: format!(
                        "critical-path latest-time underflow for operation `{operation}`"
                    ),
                })?;

            latest_finish.insert(operation, latest_finish_value);
            latest_start.insert(operation, latest_start_value);
        }

        // ---------------------------------------------------------------------
        // Slack.
        // ---------------------------------------------------------------------

        let mut slack = BTreeMap::new();

        for &operation in &topological_order {
            let earliest = earliest_start
                .get(&operation)
                .copied()
                .ok_or_else(|| SchedulingError::InvalidDependencyGraph {
                    dependency: None,
                    predecessor: None,
                    successor: Some(operation.id()),
                    reason: String::from(
                        "earliest start missing during slack calculation",
                    ),
                })?;

            let latest = latest_start
                .get(&operation)
                .copied()
                .ok_or_else(|| SchedulingError::InvalidDependencyGraph {
                    dependency: None,
                    predecessor: None,
                    successor: Some(operation.id()),
                    reason: String::from(
                        "latest start missing during slack calculation",
                    ),
                })?;

            let operation_slack = earliest
                .checked_duration_until(latest)
                .ok_or_else(|| SchedulingError::InvalidInput {
                    reason: format!(
                        "negative scheduling slack detected for operation `{operation}`"
                    ),
                })?;

            slack.insert(operation, operation_slack);
        }

        // ---------------------------------------------------------------------
        // Reconstruct the selected critical path.
        //
        // We follow the predecessor relation chosen during the forward pass.
        // This is iterative and therefore safe for extremely deep dependency
        // chains.
        // ---------------------------------------------------------------------

        let mut critical_path_reverse = Vec::new();

        if let Some(mut current) = critical_sink {
            loop {
                critical_path_reverse.push(current);

                match critical_predecessor
                    .get(&current)
                    .copied()
                    .flatten()
                {
                    Some(predecessor) => {
                        current = predecessor;
                    }
                    None => break,
                }
            }
        }

        critical_path_reverse.reverse();

        // ---------------------------------------------------------------------
        // Internal result validation.
        //
        // This catches implementation errors before the result is exposed to
        // planners or optimizers.
        // ---------------------------------------------------------------------

        Self::validate_result(
            graph,
            durations,
            &topological_order,
            &earliest_start,
            &earliest_finish,
            &latest_start,
            &latest_finish,
            &slack,
            &critical_predecessor,
            &critical_path_reverse,
            critical_path_duration,
        )?;

        Ok(CriticalPathResult {
            operation_count,
            dependency_count,
            critical_path_duration,
            critical_path_operation_count: critical_path_reverse.len(),
            critical_path: critical_path_reverse,
            earliest_start,
            earliest_finish,
            latest_start,
            latest_finish,
            slack,
            critical_predecessor,
            weights: durations.clone(),
        })
    }

    fn validate_result(
        graph: &DependencyGraph,
        durations: &BTreeMap<OperationRef, Duration>,
        topological_order: &[OperationRef],
        earliest_start: &BTreeMap<OperationRef, TimePoint>,
        earliest_finish: &BTreeMap<OperationRef, TimePoint>,
        latest_start: &BTreeMap<OperationRef, TimePoint>,
        latest_finish: &BTreeMap<OperationRef, TimePoint>,
        slack: &BTreeMap<OperationRef, Duration>,
        critical_predecessor: &BTreeMap<
            OperationRef,
            Option<OperationRef>,
        >,
        critical_path: &[OperationRef],
        critical_path_duration: Duration,
    ) -> SchedulingResult<()> {
        // Every graph operation must have exactly one timing record.
        if earliest_start.len() != graph.operation_count()
            || earliest_finish.len() != graph.operation_count()
            || latest_start.len() != graph.operation_count()
            || latest_finish.len() != graph.operation_count()
            || slack.len() != graph.operation_count()
            || critical_predecessor.len() != graph.operation_count()
        {
            return Err(SchedulingError::InvalidDependencyGraph {
                dependency: None,
                predecessor: None,
                successor: None,
                reason: String::from(
                    "critical-path analysis produced incomplete per-operation timing maps",
                ),
            });
        }

        // Forward timing invariant:
        //
        //     EF = ES + duration
        //
        for &operation in topological_order {
            let start = earliest_start
                .get(&operation)
                .copied()
                .ok_or_else(|| SchedulingError::InvalidDependencyGraph {
                    dependency: None,
                    predecessor: None,
                    successor: Some(operation.id()),
                    reason: String::from(
                        "missing earliest-start value during validation",
                    ),
                })?;

            let finish = earliest_finish
                .get(&operation)
                .copied()
                .ok_or_else(|| SchedulingError::InvalidDependencyGraph {
                    dependency: None,
                    predecessor: None,
                    successor: Some(operation.id()),
                    reason: String::from(
                        "missing earliest-finish value during validation",
                    ),
                })?;

            let duration = durations
                .get(&operation)
                .copied()
                .ok_or_else(|| SchedulingError::MissingDuration {
                    operation: operation.id(),
                })?;

            let expected_finish = start
                .checked_add(duration)
                .ok_or_else(|| SchedulingError::InvalidInput {
                    reason: format!(
                        "overflow validating earliest finish for `{operation}`"
                    ),
                })?;

            if expected_finish != finish {
                return Err(SchedulingError::InvalidDependencyGraph {
                    dependency: None,
                    predecessor: None,
                    successor: Some(operation.id()),
                    reason: format!(
                        "earliest finish invariant violated for operation `{operation}`"
                    ),
                });
            }

            // Every predecessor must finish no later than this operation
            // starts.
            for predecessor in graph.predecessors(operation) {
                let predecessor_finish = earliest_finish
                    .get(predecessor)
                    .copied()
                    .ok_or_else(|| {
                        SchedulingError::InvalidDependencyGraph {
                            dependency: None,
                            predecessor: Some(predecessor.id()),
                            successor: Some(operation.id()),
                            reason: String::from(
                                "missing predecessor finish during validation",
                            ),
                        }
                    })?;

                if predecessor_finish > start {
                    return Err(SchedulingError::InvalidDependencyGraph {
                        dependency: None,
                        predecessor: Some(predecessor.id()),
                        successor: Some(operation.id()),
                        reason: format!(
                            "critical-path earliest-start dependency invariant violated: `{predecessor}` finishes after `{operation}` starts"
                        ),
                    });
                }
            }
        }

        // Reverse timing invariant:
        //
        //     LS = LF - duration
        //
        for &operation in topological_order {
            let latest_finish_value = latest_finish
                .get(&operation)
                .copied()
                .ok_or_else(|| SchedulingError::InvalidDependencyGraph {
                    dependency: None,
                    predecessor: None,
                    successor: Some(operation.id()),
                    reason: String::from(
                        "missing latest-finish value during validation",
                    ),
                })?;

            let latest_start_value = latest_start
                .get(&operation)
                .copied()
                .ok_or_else(|| SchedulingError::InvalidDependencyGraph {
                    dependency: None,
                    predecessor: None,
                    successor: Some(operation.id()),
                    reason: String::from(
                        "missing latest-start value during validation",
                    ),
                })?;

            let duration = durations
                .get(&operation)
                .copied()
                .ok_or_else(|| SchedulingError::MissingDuration {
                    operation: operation.id(),
                })?;

            let expected_start = latest_finish_value
                .checked_sub(duration)
                .ok_or_else(|| SchedulingError::InvalidInput {
                    reason: format!(
                        "underflow validating latest start for `{operation}`"
                    ),
                })?;

            if expected_start != latest_start_value {
                return Err(SchedulingError::InvalidDependencyGraph {
                    dependency: None,
                    predecessor: None,
                    successor: Some(operation.id()),
                    reason: format!(
                        "latest start invariant violated for operation `{operation}`"
                    ),
                });
            }

            // Every successor must be schedulable after this operation.
            for successor in graph.successors(operation) {
                let successor_start = latest_start
                    .get(successor)
                    .copied()
                    .ok_or_else(|| {
                        SchedulingError::InvalidDependencyGraph {
                            dependency: None,
                            predecessor: Some(operation.id()),
                            successor: Some(successor.id()),
                            reason: String::from(
                                "missing successor latest-start during validation",
                            ),
                        }
                    })?;

                if latest_finish_value > successor_start {
                    return Err(SchedulingError::InvalidDependencyGraph {
                        dependency: None,
                        predecessor: Some(operation.id()),
                        successor: Some(successor.id()),
                        reason: format!(
                            "latest-time dependency invariant violated between `{operation}` and `{successor}`"
                        ),
                    });
                }
            }

            let earliest = earliest_start
                .get(&operation)
                .copied()
                .ok_or_else(|| SchedulingError::InvalidDependencyGraph {
                    dependency: None,
                    predecessor: None,
                    successor: Some(operation.id()),
                    reason: String::from(
                        "missing earliest start during slack validation",
                    ),
                })?;

            let calculated_slack = earliest
                .checked_duration_until(latest_start_value)
                .ok_or_else(|| SchedulingError::InvalidInput {
                    reason: format!(
                        "negative slack detected for `{operation}`"
                    ),
                })?;

            let stored_slack = slack
                .get(&operation)
                .copied()
                .ok_or_else(|| SchedulingError::InvalidDependencyGraph {
                    dependency: None,
                    predecessor: None,
                    successor: Some(operation.id()),
                    reason: String::from(
                        "missing slack during validation",
                    ),
                })?;

            if calculated_slack != stored_slack {
                return Err(SchedulingError::InvalidDependencyGraph {
                    dependency: None,
                    predecessor: None,
                    successor: Some(operation.id()),
                    reason: format!(
                        "slack invariant violated for operation `{operation}`"
                    ),
                });
            }
        }

        // Every selected critical predecessor must actually be an incoming
        // dependency.
        for (&operation, predecessor) in critical_predecessor {
            if let Some(predecessor) = predecessor {
                if !graph
                    .predecessors(operation)
                    .iter()
                    .any(|candidate| *candidate == *predecessor)
                {
                    return Err(SchedulingError::InvalidDependencyGraph {
                        dependency: None,
                        predecessor: Some(predecessor.id()),
                        successor: Some(operation.id()),
                        reason: format!(
                            "selected critical predecessor `{predecessor}` is not an actual predecessor of `{operation}`"
                        ),
                    });
                }
            }
        }

        // Validate the reconstructed path.
        if !critical_path.is_empty() {
            if critical_path_duration
                != earliest_finish
                    .get(
                        critical_path
                            .last()
                            .ok_or_else(|| SchedulingError::InvalidDependencyGraph {
                                dependency: None,
                                predecessor: None,
                                successor: None,
                                reason: String::from(
                                    "critical path unexpectedly has no last operation",
                                ),
                            })?,
                    )
                    .copied()
                    .ok_or_else(|| SchedulingError::InvalidDependencyGraph {
                        dependency: None,
                        predecessor: None,
                        successor: None,
                        reason: String::from(
                            "critical-path sink has no earliest-finish value",
                        ),
                    })?
            {
                return Err(SchedulingError::InvalidDependencyGraph {
                    dependency: None,
                    predecessor: None,
                    successor: None,
                    reason: String::from(
                        "critical path duration does not match its selected sink finish",
                    ),
                });
            }

            for window in critical_path.windows(2) {
                let predecessor = window[0];
                let successor = window[1];

                if !graph
                    .successors(predecessor)
                    .iter()
                    .any(|candidate| *candidate == successor)
                {
                    return Err(SchedulingError::InvalidDependencyGraph {
                        dependency: None,
                        predecessor: Some(predecessor.id()),
                        successor: Some(successor.id()),
                        reason: String::from(
                            "reconstructed critical path contains a non-edge",
                        ),
                    });
                }
            }
        } else if critical_path_duration != Duration::ZERO {
            return Err(SchedulingError::InvalidDependencyGraph {
                dependency: None,
                predecessor: None,
                successor: None,
                reason: String::from(
                    "non-zero critical-path duration was produced without a critical path",
                ),
            });
        }

        Ok(())
    }
}

// =============================================================================
// Convenience functions
// =============================================================================

/// Computes a unit-weight critical path.
pub fn critical_path(
    graph: &DependencyGraph,
) -> SchedulingResult<CriticalPathResult> {
    CriticalPathAnalysis::new().analyze(graph)
}

/// Computes a duration-weighted critical path.
pub fn critical_path_with_durations(
    graph: &DependencyGraph,
    durations: &BTreeMap<OperationRef, Duration>,
) -> SchedulingResult<CriticalPathResult> {
    CriticalPathAnalysis::new()
        .analyze_with_durations(graph, durations)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::quantum::ir::core::identity::OperationId;
    use super::super::super::types::{
        DependencyKind,
        DependencyRef,
    };

    fn operation(value: u64) -> OperationRef {
        OperationRef::new(OperationId::from(value))
    }

    fn dependency(
        value: u64,
        from: OperationRef,
        to: OperationRef,
    ) -> DependencyRef {
        DependencyRef::new(
            from,
            to,
            DependencyKind::Data,
        )
        .expect("test dependency must be valid")
    }

    fn chain_graph() -> DependencyGraph {
        let a = operation(1);
        let b = operation(2);
        let c = operation(3);

        let mut graph = DependencyGraph::new();

        graph.add_operation(a).expect("operation");
        graph.add_operation(b).expect("operation");
        graph.add_operation(c).expect("operation");

        graph
            .add_dependency(dependency(1, a, b))
            .expect("dependency");
        graph
            .add_dependency(dependency(2, b, c))
            .expect("dependency");

        graph
    }

    #[test]
    fn empty_graph_has_zero_critical_path() {
        let graph = DependencyGraph::new();

        let result = critical_path(&graph)
            .expect("empty graph should be valid");

        assert_eq!(result.operation_count(), 0);
        assert_eq!(result.dependency_count(), 0);
        assert_eq!(
            result.critical_path_duration(),
            Duration::ZERO
        );
        assert!(result.critical_path().is_empty());
    }

    #[test]
    fn unit_weight_chain_has_expected_path() {
        let graph = chain_graph();

        let result = critical_path(&graph)
            .expect("chain should be analyzable");

        assert_eq!(
            result.critical_path_duration(),
            Duration::new(3)
        );

        assert_eq!(
            result.critical_path_operation_count(),
            3
        );

        assert_eq!(
            result.critical_path(),
            &[
                operation(1),
                operation(2),
                operation(3),
            ]
        );
    }

    #[test]
    fn weighted_chain_accumulates_duration() {
        let graph = chain_graph();

        let mut durations = BTreeMap::new();

        durations.insert(operation(1), Duration::new(5));
        durations.insert(operation(2), Duration::new(7));
        durations.insert(operation(3), Duration::new(11));

        let result = critical_path_with_durations(
            &graph,
            &durations,
        )
        .expect("weighted chain should be analyzable");

        assert_eq!(
            result.critical_path_duration(),
            Duration::new(23)
        );

        assert_eq!(
            result.earliest_start(operation(1)),
            Some(TimePoint::ZERO)
        );

        assert_eq!(
            result.earliest_finish(operation(1)),
            Some(TimePoint::new(5))
        );

        assert_eq!(
            result.earliest_start(operation(2)),
            Some(TimePoint::new(5))
        );

        assert_eq!(
            result.earliest_finish(operation(2)),
            Some(TimePoint::new(12))
        );

        assert_eq!(
            result.earliest_start(operation(3)),
            Some(TimePoint::new(12))
        );

        assert_eq!(
            result.earliest_finish(operation(3)),
            Some(TimePoint::new(23))
        );
    }

    #[test]
    fn parallel_branches_select_longest_branch() {
        let a = operation(1);
        let b = operation(2);
        let c = operation(3);
        let d = operation(4);

        let mut graph = DependencyGraph::new();

        for operation in [a, b, c, d] {
            graph
                .add_operation(operation)
                .expect("operation");
        }

        graph
            .add_dependency(dependency(1, a, b))
            .expect("dependency");

        graph
            .add_dependency(dependency(2, a, c))
            .expect("dependency");

        graph
            .add_dependency(dependency(3, b, d))
            .expect("dependency");

        graph
            .add_dependency(dependency(4, c, d))
            .expect("dependency");

        let mut durations = BTreeMap::new();

        durations.insert(a, Duration::new(1));
        durations.insert(b, Duration::new(10));
        durations.insert(c, Duration::new(3));
        durations.insert(d, Duration::new(1));

        let result = critical_path_with_durations(
            &graph,
            &durations,
        )
        .expect("parallel graph should be analyzable");

        assert_eq!(
            result.critical_path_duration(),
            Duration::new(12)
        );

        assert_eq!(
            result.critical_path(),
            &[a, b, d]
        );

        assert_eq!(
            result.critical_predecessor(d),
            Some(Some(b))
        );
    }

    #[test]
    fn equal_weight_predecessors_are_deterministic() {
        let a = operation(1);
        let b = operation(2);
        let c = operation(3);

        let mut graph = DependencyGraph::new();

        for operation in [a, b, c] {
            graph
                .add_operation(operation)
                .expect("operation");
        }

        graph
            .add_dependency(dependency(1, a, c))
            .expect("dependency");

        graph
            .add_dependency(dependency(2, b, c))
            .expect("dependency");

        let mut durations = BTreeMap::new();

        durations.insert(a, Duration::new(4));
        durations.insert(b, Duration::new(4));
        durations.insert(c, Duration::new(1));

        let result = critical_path_with_durations(
            &graph,
            &durations,
        )
        .expect("graph should be analyzable");

        assert_eq!(
            result.critical_predecessor(c),
            Some(Some(a))
        );

        assert_eq!(
            result.critical_path(),
            &[a, c]
        );
    }

    #[test]
    fn disconnected_graph_uses_longest_component() {
        let a = operation(1);
        let b = operation(2);
        let c = operation(3);

        let mut graph = DependencyGraph::new();

        for operation in [a, b, c] {
            graph
                .add_operation(operation)
                .expect("operation");
        }

        graph
            .add_dependency(dependency(1, a, b))
            .expect("dependency");

        let mut durations = BTreeMap::new();

        durations.insert(a, Duration::new(2));
        durations.insert(b, Duration::new(8));
        durations.insert(c, Duration::new(100));

        let result = critical_path_with_durations(
            &graph,
            &durations,
        )
        .expect("disconnected graph should be valid");

        assert_eq!(
            result.critical_path_duration(),
            Duration::new(100)
        );

        assert_eq!(
            result.critical_path(),
            &[c]
        );
    }

    #[test]
    fn zero_duration_operations_are_supported() {
        let a = operation(1);
        let b = operation(2);

        let mut graph = DependencyGraph::new();

        graph
            .add_operation(a)
            .expect("operation");

        graph
            .add_operation(b)
            .expect("operation");

        graph
            .add_dependency(dependency(1, a, b))
            .expect("dependency");

        let mut durations = BTreeMap::new();

        durations.insert(a, Duration::ZERO);
        durations.insert(b, Duration::ZERO);

        let result = critical_path_with_durations(
            &graph,
            &durations,
        )
        .expect("zero-duration graph should be valid");

        assert_eq!(
            result.critical_path_duration(),
            Duration::ZERO
        );

        assert_eq!(
            result.critical_path(),
            &[a, b]
        );

        assert_eq!(
            result.slack(a),
            Some(Duration::ZERO)
        );

        assert_eq!(
            result.slack(b),
            Some(Duration::ZERO)
        );
    }

    #[test]
    fn slack_identifies_non_critical_branch() {
        let a = operation(1);
        let b = operation(2);
        let c = operation(3);
        let d = operation(4);

        let mut graph = DependencyGraph::new();

        for operation in [a, b, c, d] {
            graph
                .add_operation(operation)
                .expect("operation");
        }

        graph
            .add_dependency(dependency(1, a, b))
            .expect("dependency");

        graph
            .add_dependency(dependency(2, a, c))
            .expect("dependency");

        graph
            .add_dependency(dependency(3, b, d))
            .expect("dependency");

        graph
            .add_dependency(dependency(4, c, d))
            .expect("dependency");

        let mut durations = BTreeMap::new();

        durations.insert(a, Duration::new(1));
        durations.insert(b, Duration::new(10));
        durations.insert(c, Duration::new(3));
        durations.insert(d, Duration::new(1));

        let result = critical_path_with_durations(
            &graph,
            &durations,
        )
        .expect("graph should be analyzable");

        assert_eq!(
            result.slack(a),
            Some(Duration::ZERO)
        );

        assert_eq!(
            result.slack(b),
            Some(Duration::ZERO)
        );

        assert_eq!(
            result.slack(c),
            Some(Duration::new(7))
        );

        assert_eq!(
            result.slack(d),
            Some(Duration::ZERO)
        );
    }
}