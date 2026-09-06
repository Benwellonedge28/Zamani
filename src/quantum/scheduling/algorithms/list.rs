//! Zamani Quantum Scheduling — List Scheduling Algorithm
//!
//! Path:
//!     src/quantum/scheduling/algorithms/list.rs
//!
//! # Purpose
//!
//! This module is the stable algorithm-level entry point for Zamani's
//! resource-aware list scheduling strategy.
//!
//! List scheduling repeatedly selects an executable operation from a ready
//! set, places it at the earliest legal time permitted by the supplied
//! dependency, timing, resource, and constraint models, reserves the required
//! resources, and releases newly-ready successors.
//!
//! The actual scheduling mechanics are owned by:
//!
//!     crate::quantum::scheduling::planners::list
//!
//! This file therefore MUST NOT implement a second list scheduler.
//!
//! # Architectural position
//!
//! ```text
//! Zamani source
//!      |
//!      v
//! quantum::frontend
//!      |
//!      v
//! quantum::ir
//!      |
//!      v
//! optimization
//!      |
//!      v
//! routing
//!      |
//!      v
//! SchedulingContext
//!      |
//!      v
//! scheduling::algorithms::list
//!      |
//!      v
//! scheduling::planners::list
//!      |
//!      +----------------------------+
//!      |                            |
//!      v                            v
//! dependency model            resource model
//!      |                            |
//!      +-------------+--------------+
//!                    |
//!                    v
//!              ListSchedule
//!                    |
//!                    v
//!          SchedulingResult adapter
//!                    |
//!                    v
//!               verification
//!                    |
//!                    v
//!             hardware/runtime
//! ```
//!
//! # Responsibility
//!
//! This module owns:
//!
//! - stable algorithm identity;
//! - algorithm version;
//! - algorithm metadata;
//! - algorithm-level configuration;
//! - the public list-algorithm facade;
//! - capability declaration;
//! - delegation to the canonical list planner;
//! - deterministic algorithm metadata;
//! - the public algorithm integration boundary.
//!
//! This module does NOT own:
//!
//! - quantum semantics;
//! - `QubitId`;
//! - `PhysicalQubitId`;
//! - dependency graph construction;
//! - resource-calendar implementation;
//! - timing implementation;
//! - routing;
//! - hardware discovery;
//! - hardware execution;
//! - calibration acquisition;
//! - QEC decoding;
//! - noise modelling;
//! - verification;
//! - serialization;
//! - runtime execution.
//!
//! # Canonical qubit identity
//!
//! This module does not need to manipulate qubit identities directly.
//!
//! Whenever an adapter or planner requires qubit identity, the authoritative
//! types are:
//!
//!     crate::quantum::ir::qubit::QubitId
//!     crate::quantum::ir::qubit::PhysicalQubitId
//!
//! No scheduler-local qubit identity is defined here.
//!
//! # Universal-program principle
//!
//! The same Zamani quantum program must be schedulable for different targets
//! without changing the program merely because the target has a different
//! number of qubits, resources, channels, processors, modules, or QPUs.
//!
//! Consequently this module contains NO:
//!
//! - maximum qubit count;
//! - maximum operation count;
//! - maximum resource count;
//! - maximum channel count;
//! - maximum schedule depth;
//! - fixed topology;
//! - fixed gate set;
//! - fixed gate arity;
//! - fixed timing unit;
//! - fixed QEC distance;
//! - vendor-specific assumptions.
//!
//! "Infinity" means that this algorithm introduces no artificial machine-size
//! ceiling. Real compilation remains bounded by actual memory, address space,
//! execution time, explicit caller limits, and target resources.
//!
//! # Algorithmic responsibility
//!
//! List scheduling is neither merely an operation sorter nor a qubit-only
//! scheduler.
//!
//! A legal placement may depend on:
//!
//! - predecessor completion;
//! - release time;
//! - timing windows;
//! - deadlines;
//! - physical qubit resources;
//! - control channels;
//! - measurement channels;
//! - couplers;
//! - communication links;
//! - classical processors;
//! - QEC resources;
//! - synchronization;
//! - target-provided availability.
//!
//! These are supplied by the planner/resource/timing models.
//!
//! # Determinism
//!
//! The canonical list planner already provides deterministic priority ordering
//! when using its default priority model. This facade introduces no randomness.
//!
//! It must never use:
//!
//! - pointer addresses;
//! - wall-clock time;
//! - hash-map iteration order as semantic ordering;
//! - implicit random state;
//! - thread timing.
//!
//! # Dynamic scheduling
//!
//! Static list scheduling operates on dependencies known at planning time.
//!
//! Runtime-only dependencies must remain represented by the dynamic scheduling
//! subsystem. This algorithm must not pretend that unavailable runtime
//! information is known statically.
//!
//! The same facade may be invoked for a newly available scheduling region once
//! the dynamic scheduler supplies a valid scheduling model.
//!
//! # Distributed scheduling
//!
//! Distributed operations are not special cases here.
//!
//! Communication, synchronization, entanglement-generation, and classical
//! communication resources are represented by the resource/dependency models.
//!
//! Network topology belongs to routing/distributed scheduling, not this facade.
//!
//! # QEC
//!
//! QEC-generated operations can use list scheduling when the QEC adapter
//! exposes their:
//!
//! - dependencies;
//! - durations;
//! - resources;
//! - timing windows;
//! - priorities;
//! - readiness conditions.
//!
//! This module does not implement a QEC decoder or stabilizer algorithm.
//!
//! # Scalability
//!
//! The canonical planner intentionally uses:
//!
//! - dependency counts;
//! - ready-set processing;
//! - deterministic priority queues;
//! - event/resource abstractions;
//! - incremental successor processing;
//!
//! rather than a qubit-by-time matrix.
//!
//! Therefore this facade introduces no memory structure proportional to:
//!
//!     qubits × time
//!
//! or:
//!
//!     resources × maximum_time
//!
//! Dependency processing in the canonical planner targets O(V + E), excluding
//! priority and resource-model costs.
//!
//! # Ownership boundary
//!
//! The relationship between this module and the canonical planner is:
//!
//! ```text
//! algorithms::list
//!        |
//!        | "I want list scheduling"
//!        v
//! planners::list
//!        |
//!        | "Here is how ready operations,
//!        | resources, timing and dependencies work"
//!        v
//! ListSchedule
//! ```
//!
//! This separation is intentional. It allows the underlying list planner to
//! evolve without changing the algorithm-level identity used by configuration,
//! registries, diagnostics, and serialized scheduling requests.
//!
//! # Error boundary
//!
//! This module does not define a competing scheduling error hierarchy.
//!
//! Planner-level execution errors must eventually be translated through:
//!
//!     crate::quantum::scheduling::errors
//!
//! The canonical list planner currently exposes its own algorithm-local error
//! type. The context/planner adapter is responsible for translating those
//! errors into the canonical scheduling error hierarchy.
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
//! This module explicitly forbids unsafe Rust.
//!
//! # Frozen-file contract
//!
//! Once this contract is accepted, adding:
//!
//! - another hardware technology;
//! - another resource type;
//! - another topology;
//! - another QEC implementation;
//! - another timing representation;
//! - another routing implementation;
//! - another distributed transport;
//! - another optimization objective;
//!
//! must NOT require changing this file.
//!
//! Such additions belong to their respective adapters/models.
//!
//! This file should change only when the externally visible semantics of the
//! list scheduling algorithm itself change.
//!
//! ============================================================================
//! Safety boundary
//! ============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use crate::quantum::scheduling::context::SchedulingContext;
use crate::quantum::scheduling::errors::SchedulingResult;
use crate::quantum::scheduling::result::SchedulingResult as ScheduleArtifact;

// ============================================================================
// Stable algorithm identity
// ============================================================================

/// Stable identifier for the list scheduling algorithm.
///
/// This identifier is implementation-neutral and contains no hardware-size
/// information.
pub const LIST_ALGORITHM_ID: &str = "scheduling.list";

/// Stable semantic version of the list algorithm facade.
///
/// This is independent from the crate/package version and from the planner
/// contract version.
pub const LIST_ALGORITHM_VERSION: u32 = 1;

/// Stable human-readable algorithm name.
pub const LIST_ALGORITHM_NAME: &str = "resource-aware-list";

// ============================================================================
// Algorithm configuration
// ============================================================================

/// Configuration for the list scheduling algorithm facade.
///
/// Algorithm-level configuration deliberately contains no target dimensions.
/// Target-specific information is supplied by `SchedulingContext`.
///
/// The actual work limits belong to the canonical list planner configuration,
/// not to this algorithm facade.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ListAlgorithmConfig {
    /// Whether the algorithm is permitted to schedule an empty program.
    ///
    /// Empty programs are useful to compiler pipelines and therefore are
    /// accepted by default.
    pub allow_empty: bool,
}

impl Default for ListAlgorithmConfig {
    fn default() -> Self {
        Self {
            allow_empty: true,
        }
    }
}

impl ListAlgorithmConfig {
    /// Creates production-default algorithm configuration.
    #[must_use]
    pub const fn production() -> Self {
        Self {
            allow_empty: true,
        }
    }

    /// Sets whether an empty scheduling problem is accepted.
    #[must_use]
    pub const fn with_allow_empty(
        mut self,
        allow_empty: bool,
    ) -> Self {
        self.allow_empty = allow_empty;
        self
    }
}

// ============================================================================
// Algorithm facade
// ============================================================================

/// Production list scheduling algorithm facade.
///
/// This object is intentionally small and immutable.
///
/// It owns algorithm identity and configuration, but not scheduling state.
///
/// The actual ready-set/resource scheduling engine is owned by
/// `crate::quantum::scheduling::planners::list`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ListAlgorithm {
    config: ListAlgorithmConfig,
}

impl Default for ListAlgorithm {
    fn default() -> Self {
        Self::new()
    }
}

impl ListAlgorithm {
    /// Creates a production-default list scheduling algorithm.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            config: ListAlgorithmConfig::production(),
        }
    }

    /// Creates the algorithm with explicit configuration.
    #[must_use]
    pub const fn with_config(
        config: ListAlgorithmConfig,
    ) -> Self {
        Self { config }
    }

    /// Returns the algorithm configuration.
    #[must_use]
    pub const fn config(
        &self,
    ) -> ListAlgorithmConfig {
        self.config
    }

    /// Returns the stable algorithm identifier.
    #[must_use]
    pub const fn id() -> &'static str {
        LIST_ALGORITHM_ID
    }

    /// Returns the stable human-readable algorithm name.
    #[must_use]
    pub const fn name() -> &'static str {
        LIST_ALGORITHM_NAME
    }

    /// Returns the algorithm semantic version.
    #[must_use]
    pub const fn version() -> u32 {
        LIST_ALGORITHM_VERSION
    }

    /// Returns whether this algorithm is deterministic.
    ///
    /// The canonical list planner's default priority model is deterministic.
    #[must_use]
    pub const fn deterministic() -> bool {
        true
    }

    /// Returns whether this algorithm introduces an artificial machine-size
    /// limit.
    ///
    /// It does not.
    #[must_use]
    pub const fn has_machine_size_limit() -> bool {
        false
    }

    /// Returns whether this implementation requires unsafe Rust.
    ///
    /// It does not.
    #[must_use]
    pub const fn uses_unsafe() -> bool {
        false
    }

    /// Plans the supplied scheduling context using list scheduling.
    ///
    /// # Important integration boundary
    ///
    /// `SchedulingPlanner::plan` accepts an immutable `SchedulingContext`.
    ///
    /// The canonical low-level list scheduler currently accepts:
    ///
    /// ```text
    /// &dependencies
    /// &mut resources
    /// ```
    ///
    /// through its `ListDependencyModel` and `ListResourceModel` contracts.
    ///
    /// Consequently the context-to-list-model adapter must exist before this
    /// algorithm can perform an actual production invocation.
    ///
    /// That adapter belongs outside this file, normally in:
    ///
    /// ```text
    /// crate::quantum::scheduling::adapters
    /// ```
    ///
    /// This method deliberately does NOT fabricate an adapter, access hardware,
    /// mutate a `SchedulingContext`, or silently return a partial schedule.
    ///
    /// Until the canonical context/planner bridge is registered, returning a
    /// structured `Unsupported` error is safer than pretending to have
    /// scheduled the program.
    ///
    /// Once that bridge is present, this method is the stable public entry point
    /// and should delegate to the registered list planner without changing this
    /// algorithm's public identity.
    pub fn schedule(
        &self,
        _context: &SchedulingContext,
    ) -> SchedulingResult<ScheduleArtifact> {
        Err(
            crate::quantum::scheduling::errors::SchedulingError::Unsupported(
                "list scheduling requires the canonical SchedulingContext-to-list-planner adapter to be registered"
                    .to_owned(),
            ),
        )
    }

    /// Convenience entry point using production defaults.
    ///
    /// This has the same integration requirements as [`Self::schedule`].
    pub fn run(
        context: &SchedulingContext,
    ) -> SchedulingResult<ScheduleArtifact> {
        Self::new().schedule(context)
    }
}

// ============================================================================
// Algorithm capability metadata
// ============================================================================

/// Immutable description of the list algorithm's capabilities.
///
/// This is intentionally a local, lightweight capability description. The
/// authoritative planner capability contract remains in
/// `scheduling::planners::planner`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ListAlgorithmCapabilities {
    /// List scheduling understands dependency relationships.
    pub dependency_aware: bool,

    /// List scheduling understands abstract resources.
    pub resource_aware: bool,

    /// List scheduling understands temporal bounds.
    pub timing_aware: bool,

    /// Default list scheduling is deterministic.
    pub deterministic: bool,

    /// The underlying model can represent distributed resources.
    pub distributed: bool,

    /// QEC-generated operations can be represented through the common model.
    pub qec: bool,

    /// Static conditional dependencies can be represented by the common model.
    pub conditional: bool,

    /// Runtime feedback requires the dynamic scheduling integration.
    pub feedback: bool,
}

impl ListAlgorithmCapabilities {
    /// Returns production capabilities for the algorithm facade.
    #[must_use]
    pub const fn production() -> Self {
        Self {
            dependency_aware: true,
            resource_aware: true,
            timing_aware: true,
            deterministic: true,
            distributed: true,
            qec: true,
            conditional: true,
            feedback: true,
        }
    }
}

impl Default for ListAlgorithmCapabilities {
    fn default() -> Self {
        Self::production()
    }
}

// ============================================================================
// Algorithm policy marker
// ============================================================================

/// Marker trait identifying a list-scheduling strategy.
///
/// This trait intentionally contains no scheduling implementation. It exists
/// so registries and policy-selection layers can identify the algorithm without
/// coupling themselves to its internal planner representation.
pub trait ListSchedulingStrategy {
    /// Returns the stable algorithm identifier.
    fn algorithm_id(&self) -> &'static str;

    /// Returns the stable algorithm version.
    fn algorithm_version(&self) -> u32;

    /// Returns the algorithm capability declaration.
    fn capabilities(&self) -> ListAlgorithmCapabilities;
}

impl ListSchedulingStrategy for ListAlgorithm {
    fn algorithm_id(&self) -> &'static str {
        LIST_ALGORITHM_ID
    }

    fn algorithm_version(&self) -> u32 {
        LIST_ALGORITHM_VERSION
    }

    fn capabilities(&self) -> ListAlgorithmCapabilities {
        ListAlgorithmCapabilities::production()
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identity_is_stable() {
        assert_eq!(
            ListAlgorithm::id(),
            "scheduling.list"
        );

        assert_eq!(
            ListAlgorithm::name(),
            "resource-aware-list"
        );

        assert_eq!(
            ListAlgorithm::version(),
            1
        );
    }

    #[test]
    fn default_configuration_is_production_safe() {
        let algorithm = ListAlgorithm::new();

        assert!(
            algorithm.config().allow_empty
        );
    }

    #[test]
    fn algorithm_is_small_and_machine_independent() {
        assert_eq!(
            core::mem::size_of::<ListAlgorithm>(),
            core::mem::size_of::<ListAlgorithmConfig>()
        );

        assert!(
            !ListAlgorithm::has_machine_size_limit()
        );

        assert!(
            !ListAlgorithm::uses_unsafe()
        );
    }

    #[test]
    fn algorithm_is_deterministic() {
        assert!(
            ListAlgorithm::deterministic()
        );
    }

    #[test]
    fn capabilities_are_resource_and_dependency_aware() {
        let capabilities =
            ListAlgorithmCapabilities::production();

        assert!(
            capabilities.dependency_aware
        );

        assert!(
            capabilities.resource_aware
        );

        assert!(
            capabilities.timing_aware
        );

        assert!(
            capabilities.deterministic
        );
    }

    #[test]
    fn strategy_metadata_is_stable() {
        let algorithm = ListAlgorithm::new();

        assert_eq!(
            algorithm.algorithm_id(),
            LIST_ALGORITHM_ID
        );

        assert_eq!(
            algorithm.algorithm_version(),
            LIST_ALGORITHM_VERSION
        );

        assert_eq!(
            algorithm.capabilities(),
            ListAlgorithmCapabilities::production()
        );
    }
}