//! Zamani Quantum Scheduling — Critical-Path Algorithm
//!
//! Path:
//!     src/quantum/scheduling/algorithms/cp.rs
//!
//! # Purpose
//!
//! This module provides the stable algorithm-level entry point for
//! critical-path scheduling in Zamani.
//!
//! The actual dependency analysis is owned by:
//!
//! ```text
//! crate::quantum::scheduling::ir::critical_path
//! ```
//!
//! The production scheduling planner is owned by:
//!
//! ```text
//! crate::quantum::scheduling::planners::critical_path
//! ```
//!
//! This module deliberately does NOT duplicate either implementation.
//!
//! Its responsibility is to provide a stable algorithm facade that can be used
//! by:
//!
//! - scheduler algorithm registries;
//! - compiler orchestration;
//! - adaptive scheduling;
//! - scheduling plugins;
//! - diagnostics;
//! - tests;
//! - future algorithm selection layers.
//!
//! # Architectural position
//!
//! ```text
//! Zamani source
//!      │
//!      ▼
//! quantum::frontend
//!      │
//!      ▼
//! quantum::ir
//!      │
//!      ▼
//! optimization
//!      │
//!      ▼
//! routing
//!      │
//!      ▼
//! scheduling::adapters::ir
//!      │
//!      ▼
//! SchedulingContext
//!      │
//!      ▼
//! algorithms::cp                 ◄── this module
//!      │
//!      ▼
//! planners::critical_path
//!      │
//!      ├── ir::critical_path
//!      ├── dependency constraints
//!      ├── resource/timing validation
//!      └── candidate schedule
//!      │
//!      ▼
//! SchedulingResult
//!      │
//!      ▼
//! verification
//!      │
//!      ▼
//! transformations / optimization
//!      │
//!      ▼
//! hardware lowering
//!      │
//!      ▼
//! runtime
//! ```
//!
//! # Why this is a facade
//!
//! There must be one authoritative implementation of critical-path scheduling.
//!
//! Duplicating the algorithm in both:
//!
//! ```text
//! algorithms/cp.rs
//! planners/critical_path.rs
//! ```
//!
//! would create two implementations that could diverge in:
//!
//! - dependency handling;
//! - timing semantics;
//! - overflow handling;
//! - resource validation;
//! - deterministic ordering;
//! - error behaviour;
//! - result construction.
//!
//! Therefore:
//!
//! ```text
//! algorithms::cp
//!        │
//!        ▼
//! planners::critical_path
//!        │
//!        ▼
//! ir::critical_path
//! ```
//!
//! is the intentional ownership hierarchy.
//!
//! # Critical-path semantics
//!
//! Critical-path analysis determines the dependency-only temporal lower bound.
//!
//! For a DAG:
//!
//! ```text
//! earliest_start(v)
//!     = max(finish(predecessor))
//!
//! earliest_finish(v)
//!     = earliest_start(v) + duration(v)
//! ```
//!
//! The dependency-only critical-path duration is the maximum earliest finish.
//!
//! This is NOT necessarily the final physical schedule makespan because the
//! final schedule can additionally be constrained by:
//!
//! - resource contention;
//! - control channels;
//! - measurement channels;
//! - calibration windows;
//! - alignment;
//! - communication;
//! - dynamic execution;
//! - target-specific constraints.
//!
//! The underlying production planner already respects the supplied scheduling
//! context when converting critical-path information into a candidate schedule.
//!
//! # Universal-program principle
//!
//! This algorithm contains no assumptions about the target machine.
//!
//! It does NOT contain:
//!
//! ```text
//! MAX_QUBITS
//! MAX_OPERATIONS
//! MAX_RESOURCES
//! MAX_CHANNELS
//! MAX_DEPTH
//! MAX_PARALLELISM
//! ```
//!
//! It also does not assume:
//!
//! - a particular topology;
//! - a particular number of qubits;
//! - a fixed gate set;
//! - a fixed gate arity;
//! - a particular quantum technology;
//! - a particular vendor;
//! - a particular timing unit;
//! - a fixed QEC distance;
//! - a fixed number of control channels.
//!
//! All target information comes from `SchedulingContext`.
//!
//! "Infinity" therefore means that this algorithm introduces no artificial
//! finite quantum-machine-size limit. A real compilation is naturally bounded
//! by available memory, address space, compiler time, explicit deployment
//! policy, and target resources.
//!
//! # Canonical quantum identity
//!
//! Quantum identities remain owned by:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! This module does not create a scheduler-specific qubit type.
//!
//! Critical-path scheduling is dependency-oriented and therefore normally does
//! not need to manipulate qubit IDs directly.
//!
//! If future CP-specific functionality requires qubit identity, it MUST use
//! the canonical types above rather than defining another representation.
//!
//! # Canonical operation identity
//!
//! Scheduler operation identity is supplied by the canonical scheduler/IR
//! boundary.
//!
//! This module does not create a second operation-ID system.
//!
//! # Timing
//!
//! This module does not define a timing representation.
//!
//! Timing is supplied by:
//!
//! ```text
//! scheduling::types
//! scheduling::timing
//! SchedulingContext
//! ```
//!
//! No nanosecond, picosecond, device-tick, pulse-sample, or other physical
//! timing assumption is made here.
//!
//! # Resources
//!
//! Critical-path scheduling does not own resource-calendar implementation.
//!
//! Resource availability and placement legality are supplied by
//! `SchedulingContext` and the resource subsystem.
//!
//! The CP algorithm therefore works with:
//!
//! ```text
//! resource model
//!      │
//!      ▼
//! SchedulingContext
//!      │
//!      ▼
//! CriticalPathPlanner
//! ```
//!
//! rather than querying hardware or constructing its own resource calendar.
//!
//! # Hardware boundary
//!
//! This module never:
//!
//! - discovers hardware;
//! - connects to a QPU;
//! - authenticates;
//! - accesses credentials;
//! - calls vendor SDKs;
//! - executes quantum jobs;
//! - obtains calibration directly.
//!
//! Hardware information must already have been converted into the immutable
//! scheduling context.
//!
//! # Routing boundary
//!
//! Routing answers:
//!
//! > WHERE should an operation execute?
//!
//! Critical-path scheduling answers:
//!
//! > WHEN should that already-routed operation execute?
//!
//! This module therefore consumes the routing result through the scheduling
//! context and does not perform logical-to-physical mapping.
//!
//! # QEC boundary
//!
//! QEC scheduling constraints can participate through the scheduling context.
//!
//! This module does not implement:
//!
//! - syndrome decoding;
//! - stabilizer extraction;
//! - surface-code decoding;
//! - recovery algorithms;
//! - QEC distance selection.
//!
//! Those remain in the QEC subsystem.
//!
//! # Dynamic circuits
//!
//! The CP algorithm is primarily a static dependency scheduler.
//!
//! Dynamic/runtime-only dependencies must not be falsely converted into static
//! dependencies.
//!
//! Dynamic scheduling remains owned by:
//!
//! ```text
//! scheduling::dynamic
//! ```
//!
//! A hybrid compiler may invoke CP scheduling for statically resolvable regions
//! and defer runtime-dependent regions to the dynamic scheduler.
//!
//! # Distributed quantum computing
//!
//! Distributed operations may participate when communication dependencies and
//! resources are already represented by the scheduling context.
//!
//! This module does not contain network topology or communication semantics.
//!
//! # Determinism
//!
//! The underlying critical-path planner is deterministic.
//!
//! This facade introduces no randomness and no global mutable state.
//!
//! Given equivalent immutable scheduling inputs, the same underlying planner
//! produces equivalent scheduling decisions.
//!
//! # Scalability
//!
//! The facade itself is O(1) beyond the state required to hold its planner.
//!
//! The computational complexity belongs to the underlying CP analysis and
//! planner.
//!
//! The CP analysis is designed around iterative dependency processing and
//! operation-oriented storage rather than:
//!
//! ```text
//! qubits × time
//! resources × time
//! machine_size × depth
//! ```
//!
//! The underlying analysis targets O(V + E) dependency processing, subject to
//! the complexity of the canonical ordered graph implementation.
//!
//! # Exact optimality
//!
//! Critical-path scheduling is not claimed to be globally optimal for arbitrary
//! resource-constrained scheduling problems.
//!
//! The critical path provides a dependency-only lower bound and a strong
//! scheduling priority signal.
//!
//! Resource contention can force a final schedule above that lower bound.
//!
//! # Error semantics
//!
//! Errors are returned unchanged from the canonical scheduling planner where
//! possible.
//!
//! This module does not introduce a competing error hierarchy.
//!
//! No operation may be silently dropped.
//!
//! A failed scheduling invocation must not be represented as a successful
//! complete schedule.
//!
//! # Thread safety
//!
//! `CriticalPathAlgorithm` contains only immutable planner configuration/state.
//!
//! It owns no global mutable state.
//!
//! Separate instances may therefore be used independently for separate
//! scheduling contexts.
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
//! The safety boundary is compiler-enforced.
//!
//! # Integration contract
//!
//! Upstream:
//!
//! ```text
//! quantum::ir
//! quantum::routing
//! quantum::hardware
//! quantum::zqn
//! quantum::error_correction
//! scheduling::adapters
//! ```
//!
//! These subsystems ultimately provide the immutable `SchedulingContext`.
//!
//! Downstream:
//!
//! ```text
//! scheduling::verification
//! scheduling::transformations
//! scheduling::optimization
//! scheduling::diagnostics
//! scheduling::serialization
//! hardware lowering
//! runtime
//! ```
//!
//! The algorithm itself only depends on the stable scheduler context, planner,
//! and result contracts.
//!
//! # Future-proofing
//!
//! Adding a new quantum technology, target, topology, QEC strategy, routing
//! algorithm, resource type, or timing representation must not require changes
//! to this file.
//!
//! Adding a different critical-path heuristic should generally be implemented
//! in `planners/critical_path.rs` or as a new planner/algorithm rather than
//! modifying this facade's core delegation contract.
//!
//! =============================================================================
//! Safety boundary
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use crate::quantum::scheduling::context::SchedulingContext;
use crate::quantum::scheduling::errors::SchedulingResult;
use crate::quantum::scheduling::ir::critical_path::CriticalPathResult;
use crate::quantum::scheduling::planners::critical_path::{
    CriticalPathConfig,
    CriticalPathPlanner,
};
use crate::quantum::scheduling::result::SchedulingResult as ScheduleArtifact;

// =============================================================================
// Stable algorithm identity
// =============================================================================

/// Stable identifier for the critical-path scheduling algorithm.
///
/// This identifier is deliberately independent of a hardware vendor or target
/// size.
pub const CP_ALGORITHM_ID: &str = "scheduling.algorithm.critical_path";

/// Stable implementation contract version for this algorithm facade.
///
/// This version is separate from the global planner contract version and the
/// underlying critical-path planner version.
pub const CP_ALGORITHM_VERSION: u32 = 1;

// =============================================================================
// Algorithm
// =============================================================================

/// Stable critical-path scheduling algorithm facade.
///
/// The object delegates scheduling to the canonical
/// `CriticalPathPlanner`.
///
/// This type intentionally contains no duplicate scheduling engine.
#[derive(Debug, Clone)]
pub struct CriticalPathAlgorithm {
    planner: CriticalPathPlanner,
}

impl Default for CriticalPathAlgorithm {
    fn default() -> Self {
        Self::new()
    }
}

impl CriticalPathAlgorithm {
    /// Creates a critical-path algorithm using the production default
    /// configuration.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            planner: CriticalPathPlanner::new(),
        }
    }

    /// Creates a critical-path algorithm with explicit CP configuration.
    ///
    /// Target information remains in `SchedulingContext`.
    #[must_use]
    pub const fn with_config(config: CriticalPathConfig) -> Self {
        Self {
            planner: CriticalPathPlanner::with_config(config),
        }
    }

    /// Returns the underlying planner configuration.
    #[must_use]
    pub const fn config(&self) -> CriticalPathConfig {
        self.planner.config()
    }

    /// Returns the stable algorithm identifier.
    #[must_use]
    pub const fn id() -> &'static str {
        CP_ALGORITHM_ID
    }

    /// Returns the algorithm implementation version.
    #[must_use]
    pub const fn version() -> u32 {
        CP_ALGORITHM_VERSION
    }

    /// Performs dependency-only critical-path analysis.
    ///
    /// This does not construct a physical/resource-constrained schedule.
    ///
    /// Use `schedule` when an actual scheduling result is required.
    pub fn analyze(
        &self,
        context: &SchedulingContext,
    ) -> SchedulingResult<CriticalPathResult> {
        self.planner.analyze(context)
    }

    /// Produces a candidate schedule using critical-path prioritisation.
    ///
    /// Resource, timing, target, dependency, and contextual legality remain
    /// delegated to the canonical planner and its `SchedulingContext`.
    pub fn schedule(
        &self,
        context: &SchedulingContext,
    ) -> SchedulingResult<ScheduleArtifact> {
        self.planner.plan(context)
    }

    /// Returns the canonical underlying planner.
    ///
    /// This is useful for planner registries and orchestration layers that
    /// already operate on the `SchedulingPlanner` abstraction.
    #[must_use]
    pub const fn planner(&self) -> &CriticalPathPlanner {
        &self.planner
    }
}

// =============================================================================
// Convenience functions
// =============================================================================

/// Performs critical-path scheduling using the production default
/// configuration.
///
/// This function is intentionally stateless.
///
/// It does not use global scheduler state and does not cache target-specific
/// information.
pub fn schedule(
    context: &SchedulingContext,
) -> SchedulingResult<ScheduleArtifact> {
    CriticalPathAlgorithm::new().schedule(context)
}

/// Performs critical-path analysis without producing a physical schedule.
///
/// This is useful to callers that need the dependency-only lower bound,
/// critical operations, slack, or representative critical path.
pub fn analyze(
    context: &SchedulingContext,
) -> SchedulingResult<CriticalPathResult> {
    CriticalPathAlgorithm::new().analyze(context)
}

// =============================================================================
// Integration tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn algorithm_has_stable_identity() {
        assert_eq!(
            CriticalPathAlgorithm::id(),
            CP_ALGORITHM_ID
        );

        assert_eq!(
            CriticalPathAlgorithm::version(),
            CP_ALGORITHM_VERSION
        );
    }

    #[test]
    fn default_algorithm_has_default_planner_configuration() {
        let algorithm = CriticalPathAlgorithm::new();

        let config = algorithm.config();

        assert!(config.allows_zero_duration());
        assert!(config.allows_non_critical_fill());
        assert!(config.prefers_earliest_start());
    }

    #[test]
    fn algorithm_can_be_constructed_from_explicit_configuration() {
        let config = CriticalPathConfig::new()
            .with_zero_duration(false)
            .with_non_critical_fill(false)
            .with_earliest_start_preference(false);

        let algorithm =
            CriticalPathAlgorithm::with_config(config);

        assert!(!algorithm.config().allows_zero_duration());
        assert!(!algorithm.config().allows_non_critical_fill());
        assert!(
            !algorithm
                .config()
                .prefers_earliest_start()
        );
    }
}