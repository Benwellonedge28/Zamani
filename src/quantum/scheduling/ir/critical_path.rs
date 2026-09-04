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
//! > What is the longest dependency-constrained path through the operations,
//! > what is its target-independent duration, and which operations determine
//! > that lower bound?
//!
//! This module produces an analysis artifact. It does NOT produce a physical
//! schedule and it does NOT perform resource-constrained scheduling.
//!
//! # Architectural boundary
//!
//! ```text
//! canonical quantum IR
//!          │
//!          ▼
//! scheduling::adapters::ir
//!          │
//!          ▼
//! scheduling::ir::operation
//!          │
//!          ▼
//! scheduling::ir::graph
//!          │
//!          ▼
//! scheduling::ir::critical_path       ◄── this module
//!          │
//!      ┌───┼─────────────────────────────┐
//!      ▼   ▼                             ▼
//! planners  policies                optimization
//! ```
//!
//! # Responsibilities
//!
//! This module owns:
//!
//! - critical-path lower-bound analysis;
//! - earliest-start analysis;
//! - earliest-finish analysis;
//! - latest-start analysis;
//! - latest-finish analysis;
//! - operation slack analysis;
//! - critical-operation identification;
//! - deterministic critical-path reconstruction;
//! - deterministic tie-breaking;
//! - checked temporal arithmetic;
//! - graph/duration validation required by the analysis;
//! - analysis statistics.
//!
//! It does NOT own:
//!
//! - quantum operation semantics;
//! - logical qubit identity;
//! - physical qubit identity;
//! - routing;
//! - hardware topology;
//! - resource calendars;
//! - target capability discovery;
//! - hardware timing calibration;
//! - scheduling policies;
//! - resource-constrained scheduling;
//! - QEC algorithms;
//! - noise modelling;
//! - runtime execution;
//! - serialization formats.
//!
//! # Canonical identity ownership
//!
//! Operation identity is ultimately owned by:
//!
//! ```text
//! crate::quantum::ir::core::identity::OperationId
//! ```
//!
//! and reaches this module through:
//!
//! ```text
//! crate::quantum::scheduling::types::OperationRef
//! ```
//!
//! This module deliberately does not create another operation identity.
//!
//! Likewise, this module deliberately does not import `QubitId` or
//! `PhysicalQubitId`. Critical-path analysis operates on operation dependencies.
//! If a future scheduler analysis needs qubit identity, it MUST use the
//! canonical:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! and must not introduce scheduler-local qubit identities.
//!
//! # Time semantics
//!
//! `Duration` and `TimePoint` are abstract scheduler coordinates.
//!
//! They intentionally do not assume:
//!
//! - nanoseconds;
//! - microseconds;
//! - device ticks;
//! - pulse samples;
//! - a particular clock;
//! - a particular quantum technology.
//!
//! Interpretation is supplied by the timing/target layer.
//!
//! # Critical-path definition
//!
//! For a directed acyclic graph G=(V,E), with non-negative operation weight
//! `w(v)`:
//!
//! ```text
//! earliest_start(v)
//!     = max(earliest_finish(p)) for every predecessor p of v
//!
//! earliest_finish(v)
//!     = earliest_start(v) + w(v)
//! ```
//!
//! The dependency-only lower bound is:
//!
//! ```text
//! max(earliest_finish(v))
//! ```
//!
//! Latest times are computed relative to that lower bound:
//!
//! ```text
//! latest_finish(v)
//!     = makespan                         if v is a sink
//!     = min(latest_start(s))             otherwise
//!
//! latest_start(v)
//!     = latest_finish(v) - w(v)
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
//! Critical-path duration is NOT a physical schedule makespan.
//!
//! The analysis intentionally ignores:
//!
//! - resource contention;
//! - control-channel capacity;
//! - measurement-channel capacity;
//! - hardware maintenance;
//! - calibration windows;
//! - physical routing;
//! - communication latency;
//! - alignment constraints;
//! - dynamic resource availability;
//! - target-specific execution constraints.
//!
//! Therefore the critical-path duration is a dependency-only lower bound.
//!
//! A resource-constrained scheduler may produce a schedule whose makespan is
//! greater than or equal to this value.
//!
//! # Multiple critical paths
//!
//! A graph can contain multiple distinct critical paths.
//!
//! This module therefore exposes both:
//!
//! - the complete set of zero-slack operations;
//! - one deterministic representative critical path.
//!
//! The representative path is selected using deterministic ordering.
//!
//! # Determinism
//!
//! The canonical graph uses deterministic ordered collections.
//!
//! This module preserves deterministic behavior by:
//!
//! 1. consuming the graph's deterministic topological order;
//! 2. selecting the smallest operation when equal predecessor path lengths
//!    occur;
//! 3. selecting the smallest sink when equal maximum finish times occur;
//! 4. preserving deterministic operation ordering in returned maps/sets.
//!
//! Therefore:
//!
//! ```text
//! same graph + same duration map
//!             │
//!             ▼
//!       same analysis
//! ```
//!
//! # Scalability
//!
//! There are no scheduler-defined limits for:
//!
//! - number of operations;
//! - number of dependencies;
//! - graph depth;
//! - graph width;
//! - number of qubits;
//! - number of resources;
//! - target machine size.
//!
//! The implementation uses iterative traversal and does not recurse according
//! to graph depth.
//!
//! Its dominant complexity is:
//!
//! ```text
//! O(V + E)
//! ```
//!
//! when the supplied graph traversal itself is treated as O(V + E).
//!
//! Because the canonical `DependencyGraph` uses ordered collections, its own
//! operations may introduce logarithmic factors. This module does not replace
//! that deterministic graph representation with a second graph.
//!
//! Additional analysis storage is O(V).
//!
//! # Memory behavior
//!
//! This module stores one analysis value per operation. It does not construct:
//!
//! - a timeline proportional to machine duration;
//! - a qubit × time matrix;
//! - a resource × time matrix;
//! - fixed-size arrays based on assumed machine dimensions.
//!
//! This is essential for scaling from small systems to very large systems.
//!
//! # Overflow
//!
//! All temporal arithmetic is checked.
//!
//! No wrapping arithmetic is used.
//!
//! If a path duration cannot be represented by the scheduler's `TimePoint` /
//! `Duration` representation, the analysis fails with a structured scheduling
//! error.
//!
//! # Empty graph
//!
//! An empty graph is valid.
//!
//! The result contains:
//!
//! ```text
//! operation_count = 0
//! dependency_count = 0
//! critical_path_duration = Duration::ZERO
//! critical_path = []
//! critical_operations = {}
//! ```
//!
//! # Zero-duration operations
//!
//! Zero-duration operations are valid.
//!
//! They may appear on a critical path and may have zero slack.
//!
//! Criticality is therefore determined by slack rather than requiring a
//! positive operation duration.
//!
//! # Cycle handling
//!
//! Static critical-path analysis requires an acyclic dependency graph.
//!
//! The graph's canonical `topological_order()` method is therefore used as the
//! structural source of truth.
//!
//! This module does not duplicate graph cycle-detection logic.
//!
//! # Integration
//!
//! ```text
//! DependencyGraph
//!       │
//!       │ + operation durations
//!       ▼
//! CriticalPathAnalyzer
//!       │
//!       ├── forward pass
//!       │      ├── earliest start
//!       │      └── earliest finish
//!       │
//!       ├── backward pass
//!       │      ├── latest start
//!       │      └── latest finish
//!       │
//!       ├── slack analysis
//!       │
//!       └── deterministic path reconstruction
//!              │
//!              ▼
//!       CriticalPathResult
//! ```
//!
//! Consumers include:
//!
//! - `scheduling::planners::critical_path`;
//! - `scheduling::algorithms::cp`;
//! - `scheduling::policies::priority`;
//! - `scheduling::optimization::makespan`;
//! - `scheduling::optimization::multi_objective`;
//! - `scheduling::diagnostics::explain`;
//! - `scheduling::verification`;
//! - scheduling benchmarking.
//!
//! None of those modules are dependencies of this file.
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
//! The safety boundary is compiler-enforced below.
//!
//! =============================================================================
//! Safety boundary
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::collections::{BTreeMap, BTreeSet};

use super::graph::{DependencyGraph, DependencyGraphError};
use super::super::errors::{SchedulingError, SchedulingResult};
use super::super::types::{Duration, OperationRef, TimePoint};

// =============================================================================
// Public result types
// =============================================================================

/// Per-operation critical-path timing information.
///
/// This is an analysis value, not a schedule reservation.
///
/// The values describe the dependency-only temporal envelope of an operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CriticalPathEntry {
    /// Earliest dependency-feasible start time.
    earliest_start: TimePoint,

    /// Earliest dependency-feasible finish time.
    earliest_finish: TimePoint,

    /// Latest start time that preserves the dependency-only makespan.
    latest_start: TimePoint,

    /// Latest finish time that preserves the dependency-only makespan.
    latest_finish: TimePoint,

    /// Temporal flexibility available to the operation.
    slack: Duration,

    /// Operation duration used by the analysis.
    duration: Duration,
}

impl CriticalPathEntry {
    /// Creates an analysis entry.
    #[must_use]
    const fn new(
        earliest_start: TimePoint,
        earliest_finish: TimePoint,
        latest_start: TimePoint,
        latest_finish: TimePoint,
        slack: Duration,
        duration: Duration,
    ) -> Self {
        Self {
            earliest_start,
            earliest_finish,
            latest_start,
            latest_finish,
            slack,
            duration,
        }
    }

    /// Returns the earliest start.
    #[must_use]
    pub const fn earliest_start(self) -> TimePoint {
        self.earliest_start
    }

    /// Returns the earliest finish.
    #[must_use]
    pub const fn earliest_finish(self) -> TimePoint {
        self.earliest_finish
    }

    /// Returns the latest start.
    #[must_use]
    pub const fn latest_start(self) -> TimePoint {
        self.latest_start
    }

    /// Returns the latest finish.
    #[must_use]
    pub const fn latest_finish(self) -> TimePoint {
        self.latest_finish
    }

    /// Returns the operation slack.
    #[must_use]
    pub const fn slack(self) -> Duration {
        self.slack
    }

    /// Returns the operation duration used by the analysis.
    #[must_use]
    pub const fn duration(self) -> Duration {
        self.duration
    }

    /// Returns whether the operation is critical.
    #[must_use]
    pub const fn is_critical(self) -> bool {
        self.slack.is_zero()
    }
}

/// Complete deterministic critical-path analysis.
///
/// The result is independent of hardware resource calendars and therefore can
/// safely be reused by multiple scheduling policies.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CriticalPathResult {
    /// Number of operations analyzed.
    operation_count: usize,

    /// Number of dependency edges analyzed.
    dependency_count: usize,

    /// Dependency-only makespan lower bound.
    critical_path_duration: Duration,

    /// One deterministic source-to-sink critical path.
    critical_path: Vec<OperationRef>,

    /// All zero-slack operations.
    critical_operations: BTreeSet<OperationRef>,

    /// Per-operation timing analysis.
    entries: BTreeMap<OperationRef, CriticalPathEntry>,

    /// Selected critical predecessor for each operation.
    ///
    /// This is useful to planners that want to reconstruct or explain the
    /// dependency path without rerunning the analysis.
    critical_predecessors: BTreeMap<OperationRef, Option<OperationRef>>,
}

impl CriticalPathResult {
    /// Returns the number of analyzed operations.
    #[must_use]
    pub const fn operation_count(&self) -> usize {
        self.operation_count
    }

    /// Returns the number of analyzed dependency edges.
    #[must_use]
    pub const fn dependency_count(&self) -> usize {
        self.dependency_count
    }

    /// Returns the dependency-only critical-path duration.
    #[must_use]
    pub const fn critical_path_duration(&self) -> Duration {
        self.critical_path_duration
    }

    /// Returns the selected deterministic critical path.
    ///
    /// Operations are ordered from source to sink.
    #[must_use]
    pub fn critical_path(&self) -> &[OperationRef] {
        &self.critical_path
    }

    /// Returns every zero-slack operation.
    ///
    /// This can contain more operations than the selected representative path
    /// because a DAG may have multiple critical paths.
    #[must_use]
    pub fn critical_operations(&self) -> &BTreeSet<OperationRef> {
        &self.critical_operations
    }

    /// Returns whether an operation has zero slack.
    #[must_use]
    pub fn is_critical(&self, operation: OperationRef) -> bool {
        self.critical_operations.contains(&operation)
    }

    /// Returns whether an operation is on the selected representative path.
    #[must_use]
    pub fn is_on_critical_path(&self, operation: OperationRef) -> bool {
        self.critical_path.contains(&operation)
    }

    /// Returns the complete per-operation analysis map.
    #[must_use]
    pub fn entries(&self) -> &BTreeMap<OperationRef, CriticalPathEntry> {
        &self.entries
    }

    /// Returns the analysis entry for one operation.
    #[must_use]
    pub fn entry(
        &self,
        operation: OperationRef,
    ) -> Option<CriticalPathEntry> {
        self.entries.get(&operation).copied()
    }

    /// Returns the earliest start time for an operation.
    #[must_use]
    pub fn earliest_start(
        &self,
        operation: OperationRef,
    ) -> Option<TimePoint> {
        self.entry(operation)
            .map(CriticalPathEntry::earliest_start)
    }

    /// Returns the earliest finish time for an operation.
    #[must_use]
    pub fn earliest_finish(
        &self,
        operation: OperationRef,
    ) -> Option<TimePoint> {
        self.entry(operation)
            .map(CriticalPathEntry::earliest_finish)
    }

    /// Returns the latest start time for an operation.
    #[must_use]
    pub fn latest_start(
        &self,
        operation: OperationRef,
    ) -> Option<TimePoint> {
        self.entry(operation)
            .map(CriticalPathEntry::latest_start)
    }

    /// Returns the latest finish time for an operation.
    #[must_use]
    pub fn latest_finish(
        &self,
        operation: OperationRef,
    ) -> Option<TimePoint> {
        self.entry(operation)
            .map(CriticalPathEntry::latest_finish)
    }

    /// Returns the operation slack.
    #[must_use]
    pub fn slack(
        &self,
        operation: OperationRef,
    ) -> Option<Duration> {
        self.entry(operation)
            .map(CriticalPathEntry::slack)
    }

    /// Returns the operation duration used in the analysis.
    #[must_use]
    pub fn duration(
        &self,
        operation: OperationRef,
    ) -> Option<Duration> {
        self.entry(operation)
            .map(CriticalPathEntry::duration)
    }

    /// Returns the selected critical predecessor of an operation.
    ///
    /// `Some(None)` means the operation is a source of the selected dependency
    /// path. `None` means the operation was not analyzed.
    #[must_use]
    pub fn critical_predecessor(
        &self,
        operation: OperationRef,
    ) -> Option<Option<OperationRef>> {
        self.critical_predecessors
            .get(&operation)
            .copied()
    }

    /// Returns all earliest-start values.
    #[must_use]
    pub fn earliest_starts(
        &self,
    ) -> BTreeMap<OperationRef, TimePoint> {
        self.entries
            .iter()
            .map(|(operation, entry)| {
                (*operation, entry.earliest_start())
            })
            .collect()
    }

    /// Returns all earliest-finish values.
    #[must_use]
    pub fn earliest_finishes(
        &self,
    ) -> BTreeMap<OperationRef, TimePoint> {
        self.entries
            .iter()
            .map(|(operation, entry)| {
                (*operation, entry.earliest_finish())
            })
            .collect()
    }

    /// Returns all latest-start values.
    #[must_use]
    pub fn latest_starts(
        &self,
    ) -> BTreeMap<OperationRef, TimePoint> {
        self.entries
            .iter()
            .map(|(operation, entry)| {
                (*operation, entry.latest_start())
            })
            .collect()
    }

    /// Returns all latest-finish values.
    #[must_use]
    pub fn latest_finishes(
        &self,
    ) -> BTreeMap<OperationRef, TimePoint> {
        self.entries
            .iter()
            .map(|(operation, entry)| {
                (*operation, entry.latest_finish())
            })
            .collect()
    }

    /// Returns all slack values.
    #[must_use]
    pub fn slacks(
        &self,
    ) -> BTreeMap<OperationRef, Duration> {
        self.entries
            .iter()
            .map(|(operation, entry)| {
                (*operation, entry.slack())
            })
            .collect()
    }
}

// =============================================================================
// Analyzer
// =============================================================================

/// Deterministic critical-path analyzer.
///
/// The analyzer itself contains no mutable scheduling state and can therefore
/// be reused for multiple independent graph analyses.
///
/// It does not retain a reference to the graph or duration map, which keeps the
/// result lifecycle independent from the input lifecycle.
#[derive(Debug, Clone, Copy, Default)]
pub struct CriticalPathAnalyzer;

impl CriticalPathAnalyzer {
    /// Creates a critical-path analyzer.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Performs critical-path analysis.
    ///
    /// `durations` must contain exactly one duration for every operation in the
    /// graph. Extra duration entries are rejected because silently accepting
    /// them can conceal an adapter or compiler bug.
    ///
    /// # Algorithm
    ///
    /// 1. Validate the graph.
    /// 2. Obtain deterministic topological order.
    /// 3. Perform an iterative forward pass.
    /// 4. Select the deterministic sink defining the makespan.
    /// 5. Perform an iterative backward pass.
    /// 6. Compute slack.
    /// 7. Identify zero-slack operations.
    /// 8. Reconstruct one deterministic critical path.
    ///
    /// # Complexity
    ///
    /// The analysis itself is O(V + E) over graph traversal operations and O(V)
    /// additional memory.
    pub fn analyze(
        &self,
        graph: &DependencyGraph,
        durations: &BTreeMap<OperationRef, Duration>,
    ) -> SchedulingResult<CriticalPathResult> {
        self.validate_duration_map(graph, durations)?;

        let topological_order = graph
            .topological_order()
            .map_err(map_graph_error)?;

        if topological_order.is_empty() {
            return Ok(CriticalPathResult {
                operation_count: 0,
                dependency_count: graph.dependency_count(),
                critical_path_duration: Duration::ZERO,
                critical_path: Vec::new(),
                critical_operations: BTreeSet::new(),
                entries: BTreeMap::new(),
                critical_predecessors: BTreeMap::new(),
            });
        }

        let mut earliest_start =
            BTreeMap::<OperationRef, TimePoint>::new();
        let mut earliest_finish =
            BTreeMap::<OperationRef, TimePoint>::new();

        let mut critical_predecessors =
            BTreeMap::<OperationRef, Option<OperationRef>>::new();

        // ---------------------------------------------------------------------
        // Forward pass
        // ---------------------------------------------------------------------
        //
        // Because the graph is a DAG and the topological order is deterministic,
        // every predecessor has already been processed when its successor is
        // visited.
        //
        // No recursive traversal is used.
        //

        for &operation in &topological_order {
            let duration = durations
                .get(&operation)
                .copied()
                .ok_or_else(|| SchedulingError::MissingDuration {
                    operation: operation.id(),
                })?;

            let predecessors = graph
                .predecessors(operation)
                .map_err(map_graph_error)?;

            let mut best_start = TimePoint::ZERO;
            let mut best_predecessor = None;

            for predecessor in predecessors {
                let predecessor_finish = earliest_finish
                    .get(&predecessor)
                    .copied()
                    .ok_or_else(|| {
                        SchedulingError::InvalidDependencyGraph {
                            dependency: None,
                            predecessor: Some(predecessor.id()),
                            successor: Some(operation.id()),
                            reason: String::from(
                                "predecessor was not present in the \
                                 forward critical-path state",
                            ),
                        }
                    })?;

                let replace = match best_predecessor {
                    None => true,
                    Some(current) => {
                        let current_finish = earliest_finish
                            .get(&current)
                            .copied()
                            .ok_or_else(|| {
                                SchedulingError::InvalidDependencyGraph {
                                    dependency: None,
                                    predecessor: Some(current.id()),
                                    successor: Some(operation.id()),
                                    reason: String::from(
                                        "selected predecessor was not \
                                         present in the forward critical-path \
                                         state",
                                    ),
                                }
                            })?;

                        predecessor_finish > current_finish
                            || (predecessor_finish == current_finish
                                && predecessor < current)
                    }
                };

                if replace {
                    best_start = predecessor_finish;
                    best_predecessor = Some(predecessor);
                }
            }

            let finish = best_start
                .checked_add(duration)
                .ok_or_else(|| {
                    SchedulingError::InvalidDuration {
                        operation: Some(operation.id()),
                        duration: Some(duration),
                        reason: String::from(
                            "critical-path forward accumulation \
                             overflowed the scheduler time representation",
                        ),
                    }
                })?;

            earliest_start.insert(operation, best_start);
            earliest_finish.insert(operation, finish);
            critical_predecessors.insert(operation, best_predecessor);
        }

        // ---------------------------------------------------------------------
        // Select deterministic makespan sink
        // ---------------------------------------------------------------------

        let mut selected_sink = None;
        let mut critical_path_duration = Duration::ZERO;

        for &operation in &topological_order {
            let finish = earliest_finish
                .get(&operation)
                .copied()
                .ok_or_else(|| {
                    SchedulingError::InvalidDependencyGraph {
                        dependency: None,
                        predecessor: None,
                        successor: Some(operation.id()),
                        reason: String::from(
                            "operation has no computed earliest finish",
                        ),
                    }
                })?;

            let candidate_duration =
                finish.checked_duration_until(TimePoint::ZERO);

            // `checked_duration_until` computes end - self, so using ZERO as
            // the first operand would be incorrect. The direct coordinate is
            // therefore used here through the abstract time representation.
            let candidate_duration =
                Duration::new(finish.value());

            let replace = match selected_sink {
                None => true,
                Some(current) => {
                    let current_finish = earliest_finish
                        .get(&current)
                        .copied()
                        .ok_or_else(|| {
                            SchedulingError::InvalidDependencyGraph {
                                dependency: None,
                                predecessor: None,
                                successor: Some(current.id()),
                                reason: String::from(
                                    "selected sink has no earliest finish",
                                ),
                            }
                        })?;

                    finish > current_finish
                        || (finish == current_finish
                            && operation < current)
                }
            };

            if replace {
                selected_sink = Some(operation);
                critical_path_duration = candidate_duration;
            }
        }

        let sink = selected_sink.ok_or_else(|| {
            SchedulingError::InvalidDependencyGraph {
                dependency: None,
                predecessor: None,
                successor: None,
                reason: String::from(
                    "non-empty graph produced no terminal operation",
                ),
            }
        })?;

        // ---------------------------------------------------------------------
        // Backward pass
        // ---------------------------------------------------------------------

        let mut latest_start =
            BTreeMap::<OperationRef, TimePoint>::new();

        let mut latest_finish =
            BTreeMap::<OperationRef, TimePoint>::new();

        for &operation in topological_order.iter().rev() {
            let duration = durations
                .get(&operation)
                .copied()
                .ok_or_else(|| SchedulingError::MissingDuration {
                    operation: operation.id(),
                })?;

            let successors = graph
                .successors(operation)
                .map_err(map_graph_error)?;

            let finish = if successors.is_empty() {
                TimePoint::new(critical_path_duration.value())
            } else {
                let mut best = None;

                for successor in successors {
                    let successor_start = latest_start
                        .get(&successor)
                        .copied()
                        .ok_or_else(|| {
                            SchedulingError::InvalidDependencyGraph {
                                dependency: None,
                                predecessor: Some(operation.id()),
                                successor: Some(successor.id()),
                                reason: String::from(
                                    "successor was not present in the \
                                     backward critical-path state",
                                ),
                            }
                        })?;

                    best = match best {
                        None => Some(successor_start),
                        Some(current) => {
                            Some(current.min(successor_start))
                        }
                    };
                }

                best.ok_or_else(|| {
                    SchedulingError::InvalidDependencyGraph {
                        dependency: None,
                        predecessor: Some(operation.id()),
                        successor: None,
                        reason: String::from(
                            "operation reported successors but no \
                             successor timing was available",
                        ),
                    })?
            };

            let start = finish
                .checked_sub(duration)
                .ok_or_else(|| {
                    SchedulingError::InvalidDuration {
                        operation: Some(operation.id()),
                        duration: Some(duration),
                        reason: String::from(
                            "critical-path backward accumulation \
                             underflowed the scheduler time representation",
                        ),
                    }
                })?;

            latest_finish.insert(operation, finish);
            latest_start.insert(operation, start);
        }

        // Ensure the selected sink actually defines the reported makespan.
        let sink_finish = earliest_finish
            .get(&sink)
            .copied()
            .ok_or_else(|| SchedulingError::InvalidDependencyGraph {
                dependency: None,
                predecessor: None,
                successor: Some(sink.id()),
                reason: String::from(
                    "selected critical sink has no earliest finish",
                ),
            })?;

        if sink_finish.value() != critical_path_duration.value() {
            return Err(SchedulingError::InvalidDependencyGraph {
                dependency: None,
                predecessor: None,
                successor: Some(sink.id()),
                reason: String::from(
                    "critical-path sink finish and reported critical-path \
                     duration disagree",
                ),
            });
        }

        // ---------------------------------------------------------------------
        // Slack + result entries
        // ---------------------------------------------------------------------

        let mut entries = BTreeMap::new();
        let mut critical_operations = BTreeSet::new();

        for &operation in &topological_order {
            let duration = durations
                .get(&operation)
                .copied()
                .ok_or_else(|| SchedulingError::MissingDuration {
                    operation: operation.id(),
                })?;

            let earliest = earliest_start
                .get(&operation)
                .copied()
                .ok_or_else(|| SchedulingError::InvalidDependencyGraph {
                    dependency: None,
                    predecessor: None,
                    successor: Some(operation.id()),
                    reason: String::from(
                        "operation has no earliest-start value",
                    ),
                })?;

            let earliest_end = earliest_finish
                .get(&operation)
                .copied()
                .ok_or_else(|| SchedulingError::InvalidDependencyGraph {
                    dependency: None,
                    predecessor: None,
                    successor: Some(operation.id()),
                    reason: String::from(
                        "operation has no earliest-finish value",
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
                        "operation has no latest-start value",
                    ),
                })?;

            let latest_end = latest_finish
                .get(&operation)
                .copied()
                .ok_or_else(|| SchedulingError::InvalidDependencyGraph {
                    dependency: None,
                    predecessor: None,
                    successor: Some(operation.id()),
                    reason: String::from(
                        "operation has no latest-finish value",
                    ),
                })?;

            let slack = earliest
                .checked_duration_until(latest)
                .ok_or_else(|| SchedulingError::InvalidDependencyGraph {
                    dependency: None,
                    predecessor: None,
                    successor: Some(operation.id()),
                    reason: String::from(
                        "latest start precedes earliest start; \
                         critical-path state is inconsistent",
                    ),
                })?;

            let entry = CriticalPathEntry::new(
                earliest,
                earliest_end,
                latest,
                latest_end,
                slack,
                duration,
            );

            if entry.is_critical() {
                critical_operations.insert(operation);
            }

            entries.insert(operation, entry);
        }

        // ---------------------------------------------------------------------
        // Deterministic path reconstruction
        // ---------------------------------------------------------------------

        let critical_path =
            reconstruct_path(sink, &critical_predecessors)?;

        Ok(CriticalPathResult {
            operation_count: graph.operation_count(),
            dependency_count: graph.dependency_count(),
            critical_path_duration,
            critical_path,
            critical_operations,
            entries,
            critical_predecessors,
        })
    }

    /// Convenience associated function for one-shot analysis.
    ///
    /// This is equivalent to:
    ///
    /// ```text
    /// CriticalPathAnalyzer::new().analyze(graph, durations)
    /// ```
    pub fn analyze_graph(
        graph: &DependencyGraph,
        durations: &BTreeMap<OperationRef, Duration>,
    ) -> SchedulingResult<CriticalPathResult> {
        Self::new().analyze(graph, durations)
    }

    /// Validates that the duration map exactly matches the graph's operation
    /// set.
    ///
    /// Exact matching is deliberate. Extra entries are rejected instead of
    /// silently ignored because they normally indicate a compiler adapter bug,
    /// stale scheduling state, or operation identity mismatch.
    fn validate_duration_map(
        &self,
        graph: &DependencyGraph,
        durations: &BTreeMap<OperationRef, Duration>,
    ) -> SchedulingResult<()> {
        for &operation in graph.operations() {
            let Some(duration) = durations.get(&operation).copied() else {
                return Err(SchedulingError::MissingDuration {
                    operation: operation.id(),
                });
            };

            // Duration is currently structurally non-negative by construction.
            // Keep this validation here as a semantic boundary so future
            // duration implementations can strengthen their invariants without
            // changing this analyzer's public contract.
            if duration.value() > u128::MAX {
                return Err(SchedulingError::InvalidDuration {
                    operation: Some(operation.id()),
                    duration: Some(duration),
                    reason: String::from(
                        "duration exceeds the scheduler's representable range",
                    ),
                });
            }
        }

        for &operation in durations.keys() {
            if !graph.contains_operation(operation) {
                return Err(SchedulingError::InvalidOperation {
                    operation: Some(operation.id()),
                    qubit: None,
                    physical_qubit: None,
                    reason: String::from(
                        "duration map contains an operation that is not \
                         present in the dependency graph",
                    ),
                });
            }
        }

        Ok(())
    }
}

// =============================================================================
// Internal helpers
// =============================================================================

/// Converts a local dependency-graph error into the canonical scheduling error
/// contract.
///
/// Keeping this conversion here prevents the graph module from depending on
/// higher-level scheduler errors while still giving callers one stable error
/// model.
fn map_graph_error(error: DependencyGraphError) -> SchedulingError {
    match error {
        DependencyGraphError::CycleDetected { cycle } => {
            SchedulingError::CycleDetected {
                operation: cycle.first().copied().map(OperationRef::id),
                dependency: None,
                cycle_size: u128::try_from(cycle.len()).ok(),
            }
        }

        DependencyGraphError::DuplicateOperation { operation } => {
            SchedulingError::InvalidOperation {
                operation: Some(operation.id()),
                qubit: None,
                physical_qubit: None,
                reason: String::from(
                    "dependency graph contains a duplicate operation",
                ),
            }
        }

        DependencyGraphError::UnknownOperation {
            operation,
            dependency,
        } => SchedulingError::InvalidDependencyGraph {
            dependency,
            predecessor: None,
            successor: Some(operation.id()),
            reason: String::from(
                "dependency graph references an unknown operation",
            ),
        },

        DependencyGraphError::DuplicateDependency { dependency } => {
            SchedulingError::InvalidDependencyGraph {
                dependency: Some(dependency),
                predecessor: None,
                successor: None,
                reason: String::from(
                    "dependency graph contains a duplicate dependency",
                ),
            }
        }

        DependencyGraphError::SelfDependency { operation } => {
            SchedulingError::InvalidDependencyGraph {
                dependency: None,
                predecessor: Some(operation.id()),
                successor: Some(operation.id()),
                reason: String::from(
                    "dependency graph contains a self-dependency",
                ),
            }
        }

        DependencyGraphError::InconsistentState { reason } => {
            SchedulingError::InvalidDependencyGraph {
                dependency: None,
                predecessor: None,
                successor: None,
                reason: reason.to_owned(),
            }
        }

        DependencyGraphError::OperationNotFound { operation } => {
            SchedulingError::InvalidOperation {
                operation: Some(operation.id()),
                qubit: None,
                physical_qubit: None,
                reason: String::from(
                    "dependency graph operation lookup failed",
                ),
            }
        }

        DependencyGraphError::DependencyNotFound { dependency } => {
            SchedulingError::InvalidDependencyGraph {
                dependency: Some(dependency),
                predecessor: None,
                successor: None,
                reason: String::from(
                    "dependency graph dependency lookup failed",
                ),
            }
        }
    }
}

/// Reconstructs one deterministic source-to-sink path from the selected sink.
///
/// The predecessor map was produced by the forward critical-path pass, so
/// every predecessor is guaranteed to occur earlier in topological order.
///
/// The implementation is iterative and therefore does not consume stack space
/// proportional to graph depth.
fn reconstruct_path(
    sink: OperationRef,
    predecessors: &BTreeMap<
        OperationRef,
        Option<OperationRef>,
    >,
) -> SchedulingResult<Vec<OperationRef>> {
    let mut reverse_path = Vec::new();
    let mut current = Some(sink);

    while let Some(operation) = current {
        reverse_path.push(operation);

        current = predecessors
            .get(&operation)
            .copied()
            .flatten();

        // A valid predecessor chain in a DAG cannot revisit an operation.
        // Detecting a repeated operation here protects against a corrupted
        // predecessor map even though the canonical graph itself is acyclic.
        if reverse_path
            .iter()
            .enumerate()
            .skip_while(|(_, candidate)| **candidate != operation)
            .count()
            > 1
        {
            return Err(SchedulingError::InvalidDependencyGraph {
                dependency: None,
                predecessor: Some(operation.id()),
                successor: None,
                reason: String::from(
                    "critical-path predecessor chain contains a cycle",
                ),
            });
        }
    }

    reverse_path.reverse();
    Ok(reverse_path)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::quantum::ir::core::identity::OperationId;

    fn operation(value: u64) -> OperationRef {
        OperationRef::new(OperationId::from(value))
    }

    fn duration(value: u128) -> Duration {
        Duration::new(value)
    }

    fn dependency(
        from: u64,
        to: u64,
        id: u64,
    ) -> super::super::super::types::DependencyRef {
        super::super::super::types::DependencyRef::new(
            operation(from),
            operation(to),
            super::super::super::types::DependencyKind::Explicit,
        )
        .expect("test dependency must be valid")
        .with_id(
            super::super::super::types::DependencyId::new(id),
        )
    }

    fn chain_graph() -> DependencyGraph {
        let mut graph = DependencyGraph::new();

        graph
            .add_operations([
                operation(1),
                operation(2),
                operation(3),
            ])
            .expect("operations should be accepted");

        graph
            .add_dependency(dependency(1, 2, 1))
            .expect("dependency should be accepted");

        graph
            .add_dependency(dependency(2, 3, 2))
            .expect("dependency should be accepted");

        graph
    }

    #[test]
    fn empty_graph_has_zero_critical_path() {
        let graph = DependencyGraph::new();
        let durations = BTreeMap::new();

        let result = CriticalPathAnalyzer::new()
            .analyze(&graph, &durations)
            .expect("empty graph should be valid");

        assert_eq!(result.operation_count(), 0);
        assert_eq!(
            result.critical_path_duration(),
            Duration::ZERO
        );
        assert!(result.critical_path().is_empty());
        assert!(result.critical_operations().is_empty());
    }

    #[test]
    fn linear_chain_produces_expected_path() {
        let graph = chain_graph();

        let durations = BTreeMap::from([
            (operation(1), duration(10)),
            (operation(2), duration(20)),
            (operation(3), duration(30)),
        ]);

        let result = CriticalPathAnalyzer::new()
            .analyze(&graph, &durations)
            .expect("chain should analyze");

        assert_eq!(
            result.critical_path_duration(),
            duration(60)
        );

        assert_eq!(
            result.critical_path(),
            &[
                operation(1),
                operation(2),
                operation(3)
            ]
        );

        assert_eq!(
            result.earliest_start(operation(1)),
            Some(TimePoint::new(0))
        );

        assert_eq!(
            result.earliest_start(operation(2)),
            Some(TimePoint::new(10))
        );

        assert_eq!(
            result.earliest_start(operation(3)),
            Some(TimePoint::new(30))
        );

        assert_eq!(
            result.latest_finish(operation(3)),
            Some(TimePoint::new(60))
        );

        assert_eq!(
            result.slack(operation(1)),
            Some(Duration::ZERO)
        );

        assert_eq!(
            result.slack(operation(2)),
            Some(Duration::ZERO)
        );

        assert_eq!(
            result.slack(operation(3)),
            Some(Duration::ZERO)
        );
    }

    #[test]
    fn independent_operations_select_largest_path() {
        let graph = DependencyGraph::from_operations([
            operation(1),
            operation(2),
            operation(3),
        ])
        .expect("operations should be accepted");

        let durations = BTreeMap::from([
            (operation(1), duration(10)),
            (operation(2), duration(50)),
            (operation(3), duration(20)),
        ]);

        let result = CriticalPathAnalyzer::new()
            .analyze(&graph, &durations)
            .expect("graph should analyze");

        assert_eq!(
            result.critical_path_duration(),
            duration(50)
        );

        assert_eq!(
            result.critical_path(),
            &[operation(2)]
        );

        assert!(result.is_critical(operation(2)));
        assert!(!result.is_critical(operation(1)));
        assert!(!result.is_critical(operation(3)));
    }

    #[test]
    fn equal_paths_use_deterministic_predecessor() {
        let mut graph = DependencyGraph::new();

        graph
            .add_operations([
                operation(1),
                operation(2),
                operation(3),
            ])
            .expect("operations should be accepted");

        graph
            .add_dependency(dependency(1, 3, 1))
            .expect("dependency should be accepted");

        graph
            .add_dependency(dependency(2, 3, 2))
            .expect("dependency should be accepted");

        let durations = BTreeMap::from([
            (operation(1), duration(10)),
            (operation(2), duration(10)),
            (operation(3), duration(5)),
        ]);

        let result = CriticalPathAnalyzer::new()
            .analyze(&graph, &durations)
            .expect("graph should analyze");

        assert_eq!(
            result.critical_path(),
            &[
                operation(1),
                operation(3)
            ]
        );

        assert_eq!(
            result.critical_predecessor(operation(3)),
            Some(Some(operation(1)))
        );
    }

    #[test]
    fn zero_duration_operations_are_supported() {
        let mut graph = DependencyGraph::new();

        graph
            .add_operations([
                operation(1),
                operation(2),
                operation(3),
            ])
            .expect("operations should be accepted");

        graph
            .add_dependency(dependency(1, 2, 1))
            .expect("dependency should be accepted");

        graph
            .add_dependency(dependency(2, 3, 2))
            .expect("dependency should be accepted");

        let durations = BTreeMap::from([
            (operation(1), duration(0)),
            (operation(2), duration(0)),
            (operation(3), duration(10)),
        ]);

        let result = CriticalPathAnalyzer::new()
            .analyze(&graph, &durations)
            .expect("graph should analyze");

        assert_eq!(
            result.critical_path_duration(),
            duration(10)
        );

        assert_eq!(
            result.critical_path(),
            &[
                operation(1),
                operation(2),
                operation(3)
            ]
        );

        assert_eq!(
            result.slack(operation(1)),
            Some(Duration::ZERO)
        );

        assert_eq!(
            result.slack(operation(2)),
            Some(Duration::ZERO)
        );
    }

    #[test]
    fn missing_duration_is_rejected() {
        let graph = DependencyGraph::from_operations([
            operation(1),
        ])
        .expect("operation should be accepted");

        let durations = BTreeMap::new();

        let error = CriticalPathAnalyzer::new()
            .analyze(&graph, &durations)
            .expect_err("missing duration must fail");

        assert!(matches!(
            error,
            SchedulingError::MissingDuration { .. }
        ));
    }

    #[test]
    fn extra_duration_entry_is_rejected() {
        let graph = DependencyGraph::from_operations([
            operation(1),
        ])
        .expect("operation should be accepted");

        let durations = BTreeMap::from([
            (operation(1), duration(10)),
            (operation(2), duration(20)),
        ]);

        let error = CriticalPathAnalyzer::new()
            .analyze(&graph, &durations)
            .expect_err("extra duration must fail");

        assert!(matches!(
            error,
            SchedulingError::InvalidOperation { .. }
        ));
    }

    #[test]
    fn multiple_critical_paths_are_reported_as_zero_slack_operations() {
        let mut graph = DependencyGraph::new();

        graph
            .add_operations([
                operation(1),
                operation(2),
                operation(3),
                operation(4),
            ])
            .expect("operations should be accepted");

        graph
            .add_dependency(dependency(1, 3, 1))
            .expect("dependency should be accepted");

        graph
            .add_dependency(dependency(2, 4, 2))
            .expect("dependency should be accepted");

        let durations = BTreeMap::from([
            (operation(1), duration(10)),
            (operation(2), duration(10)),
            (operation(3), duration(10)),
            (operation(4), duration(10)),
        ]);

        let result = CriticalPathAnalyzer::new()
            .analyze(&graph, &durations)
            .expect("graph should analyze");

        assert_eq!(
            result.critical_path_duration(),
            duration(20)
        );

        assert!(result.is_critical(operation(1)));
        assert!(result.is_critical(operation(2)));
        assert!(result.is_critical(operation(3)));
        assert!(result.is_critical(operation(4)));

        // Deterministic representative path.
        assert_eq!(
            result.critical_path(),
            &[
                operation(1),
                operation(3)
            ]
        );
    }
}