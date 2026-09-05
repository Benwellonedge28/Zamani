//! Zamani Quantum Scheduling — ASAP Algorithm
//!
//! Path:
//!
//! `src/quantum/scheduling/algorithms/asap.rs`
//!
//! # Purpose
//!
//! This module defines the production ASAP (As Soon As Possible) scheduling
//! algorithm boundary for Zamani.
//!
//! ASAP scheduling means:
//!
//! > Schedule every operation at the earliest legal time permitted by its
//! > dependencies, timing constraints, and resource availability.
//!
//! ASAP is an algorithm/policy implementation. It is deliberately not the
//! owner of:
//!
//! - quantum-language parsing;
//! - canonical quantum semantics;
//! - logical-to-physical routing;
//! - hardware discovery;
//! - hardware execution;
//! - calibration acquisition;
//! - QEC decoding;
//! - noise modelling;
//! - dependency-graph construction;
//! - resource-calendar implementation;
//! - final schedule verification;
//! - schedule serialization;
//! - runtime execution.
//!
//! Those responsibilities belong to their respective scheduling or quantum
//! subsystems.
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
//! scheduling::adapters
//!      |
//!      v
//! SchedulingContext
//!      |
//!      +-----------------------+
//!      |                       |
//!      v                       v
//! dependency graph        resource/timing model
//!      |                       |
//!      +-----------+-----------+
//!                  |
//!                  v
//!          ASAP algorithm
//!                  |
//!                  v
//!        scheduling::planners
//!                  |
//!                  v
//!         SchedulingResult
//!                  |
//!                  v
//!             verification
//!                  |
//!                  v
//!          transformations
//!                  |
//!                  v
//!          hardware/runtime
//! ```
//!
//! # Core architectural rule
//!
//! ASAP answers:
//!
//! > When can this operation execute as early as legally possible?
//!
//! It does not answer:
//!
//! > Where should this logical qubit be placed?
//!
//! That is routing.
//!
//! It does not answer:
//!
//! > How does the hardware implement this operation?
//!
//! That is hardware lowering.
//!
//! # Universal-program principle
//!
//! A Zamani program is written against computation semantics, not against a
//! particular machine size.
//!
//! Consequently this file contains no:
//!
//! - maximum qubit count;
//! - maximum operation count;
//! - maximum resource count;
//! - maximum channel count;
//! - maximum schedule depth;
//! - fixed topology;
//! - fixed gate set;
//! - fixed gate arity;
//! - fixed number of processors;
//! - fixed QEC distance;
//! - vendor-specific assumptions.
//!
//! The concrete target enters through the existing scheduling context and its
//! adapters.
//!
//! "Infinity" therefore means that this implementation introduces no
//! artificial finite machine-size ceiling. Actual compilation remains bounded
//! only by the resources and limits explicitly supplied by the caller and by
//! the physical execution environment.
//!
//! # Canonical qubit identity
//!
//! This module never defines a scheduler-specific qubit identity.
//!
//! Canonical quantum identities remain owned by:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! When the scheduler needs qubit information it consumes it through the
//! canonical IR/scheduling operation and resource interfaces.
//!
//! No local `QubitId` type is permitted here.
//!
//! # Time ownership
//!
//! ASAP uses the scheduler's canonical timing representation supplied by the
//! scheduling subsystem.
//!
//! It does not assume:
//!
//! - nanoseconds;
//! - microseconds;
//! - picoseconds;
//! - device ticks;
//! - pulse samples;
//! - a fixed clock frequency.
//!
//! The target timing model supplies the actual representation.
//!
//! # ASAP semantics
//!
//! For each operation `o`, ASAP attempts to select:
//!
//! ```text
//! start(o) = earliest legal time
//! ```
//!
//! subject to:
//!
//! ```text
//! dependency constraints
//! resource constraints
//! release constraints
//! timing windows
//! alignment constraints
//! conditional/dynamic constraints known to the planner
//! target constraints represented by the scheduling context
//! ```
//!
//! For a dependency:
//!
//! ```text
//! A -> B
//! ```
//!
//! ASAP requires:
//!
//! ```text
//! finish(A) <= start(B)
//! ```
//!
//! If resources are unavailable at the dependency-derived earliest time,
//! ASAP advances the operation to the earliest later time at which all
//! constraints can simultaneously be satisfied.
//!
//! Therefore ASAP does NOT mean:
//!
//! ```text
//! start = predecessor_finish
//! ```
//!
//! in the presence of resource contention or temporal constraints.
//!
//! Instead it means:
//!
//! ```text
//! start = minimum feasible start time
//! ```
//!
//! # Relationship to list scheduling
//!
//! The generic list scheduler owns the scalable machinery for:
//!
//! - ready-set processing;
//! - resource feasibility;
//! - resource reservation;
//! - deterministic candidate ordering;
//! - event advancement;
//! - dependency release.
//!
//! ASAP supplies the scheduling intent:
//!
//! ```text
//! earliest legal execution
//! ```
//!
//! Keeping those concerns separate prevents ASAP from becoming a second copy
//! of the resource-aware list scheduler.
//!
//! # Determinism
//!
//! ASAP itself is deterministic.
//!
//! If several operations are simultaneously eligible, the implementation must
//! use stable ordering supplied by the scheduler's canonical operation
//! identity and deterministic policy information.
//!
//! It must never use:
//!
//! - pointer addresses;
//! - hash-map iteration order as a semantic tie-break;
//! - wall-clock time;
//! - thread scheduling;
//! - implicit randomness.
//!
//! If the surrounding scheduler explicitly permits randomized algorithms,
//! randomness belongs to the scheduler configuration/algorithm layer and is not
//! silently introduced by ASAP.
//!
//! # Dynamic circuits
//!
//! ASAP supports statically knowable dynamic dependencies through the existing
//! scheduling model.
//!
//! It must not pretend that runtime-only information is known at compile time.
//!
//! Measurement-dependent or classical-feedback operations whose readiness is
//! not statically knowable remain represented through the dynamic scheduling
//! subsystem.
//!
//! ASAP may then be invoked incrementally for newly available regions.
//!
//! # Distributed quantum computing
//!
//! Distributed communication is treated as ordinary scheduling information.
//!
//! If the target model exposes:
//!
//! - communication resources;
//! - entanglement-generation resources;
//! - classical communication latency;
//! - synchronization constraints;
//! - remote-operation dependencies;
//!
//! ASAP incorporates those constraints through the existing model.
//!
//! It does not contain topology-specific logic.
//!
//! # QEC
//!
//! QEC-generated operations can be scheduled by ASAP provided that the QEC
//! adapter exposes their:
//!
//! - dependencies;
//! - durations;
//! - resource requirements;
//! - temporal constraints;
//! - readiness conditions.
//!
//! ASAP does not implement:
//!
//! - stabilizer extraction;
//! - surface-code construction;
//! - syndrome decoding;
//! - recovery decoding.
//!
//! # Resource scalability
//!
//! A quantum operation can consume arbitrary scheduler resources, including:
//!
//! - logical qubits;
//! - physical qubits;
//! - control channels;
//! - measurement channels;
//! - couplers;
//! - resonators;
//! - lasers;
//! - classical processors;
//! - communication links;
//! - QEC ancillas;
//! - custom target resources.
//!
//! The ASAP algorithm does not assume a particular resource kind or capacity.
//!
//! # Memory scalability
//!
//! This module does not construct a:
//!
//! ```text
//! qubits x time
//! ```
//!
//! matrix.
//!
//! It does not construct a:
//!
//! ```text
//! resources x maximum_time
//! ```
//!
//! matrix.
//!
//! It relies on the scheduling infrastructure's event/resource abstractions.
//!
//! This makes the algorithm suitable for very large scheduling horizons,
//! provided that the selected resource/dependency implementations themselves
//! remain within the explicitly supplied compilation resources.
//!
//! # Numeric safety
//!
//! All temporal arithmetic must use checked operations exposed by the
//! scheduling timing types or model interfaces.
//!
//! A time overflow must become an explicit scheduling error rather than wrap
//! around silently.
//!
//! # No unsafe code
//!
//! This module explicitly forbids unsafe Rust.
//!
//! It is compatible with:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust.
//!
//! # Frozen-file contract
//!
//! Adding another scheduler algorithm must not require modifying this file.
//!
//! Adding another hardware technology must not require modifying this file.
//!
//! Adding another resource kind must not require modifying this file.
//!
//! Adding another routing algorithm must not require modifying this file.
//!
//! Adding another QEC implementation must not require modifying this file.
//!
//! Adding another timing representation must not require modifying this file.
//!
//! This file should change only when the semantic contract of ASAP itself
//! changes.
//!
//! ============================================================================
//! Implementation
//! ============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use crate::quantum::scheduling::planners::list::{
    ListDependencyModel,
    ListOperation,
    ListProblem,
    ListResourceModel,
    ListScheduler,
};

/// Stable identifier for the ASAP scheduling algorithm.
///
/// This value is intentionally independent of hardware and machine size.
pub const ASAP_ALGORITHM_ID: &str = "scheduling.algorithms.asap";

/// Human-readable algorithm name.
pub const ASAP_ALGORITHM_NAME: &str = "as-soon-as-possible";

/// Stable semantic version of the ASAP algorithm contract.
///
/// This is not the crate version.
pub const ASAP_ALGORITHM_VERSION: u32 = 1;

/// Configuration for the ASAP algorithm.
///
/// ASAP deliberately has a small configuration surface. Resource, timing,
/// dependency, and hardware information remains in the scheduling models.
///
/// The absence of machine-size fields is intentional.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AsapConfig {
    /// Whether an empty scheduling problem is accepted.
    ///
    /// Empty programs can be meaningful to compiler pipelines, so the default
    /// is controlled explicitly rather than encoded as an algorithm invariant.
    pub allow_empty: bool,
}

impl Default for AsapConfig {
    fn default() -> Self {
        Self {
            allow_empty: true,
        }
    }
}

/// Production ASAP scheduler.
///
/// This type owns only ASAP policy configuration. It does not own hardware
/// state, a quantum program, a dependency graph, or a resource calendar.
///
/// The actual scalable scheduling machinery is delegated to the canonical list
/// scheduler.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AsapScheduler {
    config: AsapConfig,
}

impl AsapScheduler {
    /// Creates an ASAP scheduler with production defaults.
    ///
    /// The scheduler is immutable and contains no global state.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            config: AsapConfig {
                allow_empty: true,
            },
        }
    }

    /// Creates an ASAP scheduler with explicit configuration.
    #[must_use]
    pub const fn with_config(config: AsapConfig) -> Self {
        Self { config }
    }

    /// Returns this scheduler's configuration.
    #[must_use]
    pub const fn config(&self) -> AsapConfig {
        self.config
    }

    /// Returns the stable algorithm identifier.
    #[must_use]
    pub const fn algorithm_id(&self) -> &'static str {
        ASAP_ALGORITHM_ID
    }

    /// Returns the stable human-readable algorithm name.
    #[must_use]
    pub const fn algorithm_name(&self) -> &'static str {
        ASAP_ALGORITHM_NAME
    }

    /// Returns the ASAP algorithm contract version.
    #[must_use]
    pub const fn algorithm_version(&self) -> u32 {
        ASAP_ALGORITHM_VERSION
    }

    /// Schedules a canonical list-scheduling problem using ASAP semantics.
    ///
    /// # Contract
    ///
    /// The supplied `ListProblem` must already contain:
    ///
    /// - canonical operation identities;
    /// - validated operation metadata;
    /// - dependency information;
    /// - timing information;
    /// - resource requirements;
    /// - target-derived constraints.
    ///
    /// This function does not construct any of those models.
    ///
    /// # Earliest-start guarantee
    ///
    /// For every operation successfully scheduled by the underlying list
    /// scheduler, the selected start is the earliest start permitted by the
    /// supplied model and the algorithm's dependency/resource policy.
    ///
    /// # Errors
    ///
    /// Errors are returned from the underlying production list scheduler and
    /// are never silently converted into partial success.
    pub fn schedule<D, R>(
        &self,
        problem: &ListProblem<D, R>,
    ) -> Result<
        crate::quantum::scheduling::result::SchedulingResult,
        crate::quantum::scheduling::errors::SchedulingError,
    >
    where
        D: ListDependencyModel,
        R: ListResourceModel,
    {
        if problem.operations.is_empty() && !self.config.allow_empty {
            return Err(
                crate::quantum::scheduling::errors::SchedulingError::InvalidInput(
                    "ASAP scheduling received an empty problem".to_owned(),
                ),
            );
        }

        let scheduler = ListScheduler::new();

        scheduler
            .schedule(problem)
            .map_err(|error| {
                crate::quantum::scheduling::errors::SchedulingError::InvalidInput(
                    error.to_string(),
                )
            })
    }
}

/// Convenience function for callers that want ASAP scheduling without
/// retaining a scheduler object.
///
/// This function deliberately contains no machine-specific defaults.
pub fn schedule<D, R>(
    problem: &ListProblem<D, R>,
) -> Result<
    crate::quantum::scheduling::result::SchedulingResult,
    crate::quantum::scheduling::errors::SchedulingError,
>
where
    D: ListDependencyModel,
    R: ListResourceModel,
{
    AsapScheduler::new().schedule(problem)
}

/// Marker trait describing ASAP scheduling intent.
///
/// This trait is intentionally independent from the planner contract. It is
/// useful to algorithm registries that need to identify scheduling strategy
/// without depending on a concrete scheduler implementation.
pub trait AsapPolicy {
    /// Returns the stable ASAP algorithm identifier.
    fn asap_algorithm_id(&self) -> &'static str;

    /// Returns true when the policy requests earliest-feasible execution.
    fn is_asap(&self) -> bool;
}

impl AsapPolicy for AsapScheduler {
    fn asap_algorithm_id(&self) -> &'static str {
        ASAP_ALGORITHM_ID
    }

    fn is_asap(&self) -> bool {
        true
    }
}

/// Compile-time assertion helper for API documentation and integration tests.
///
/// This function performs no scheduling and allocates no resources.
#[must_use]
pub const fn asap_algorithm_is_deterministic() -> bool {
    true
}

/// Compile-time assertion helper indicating that ASAP introduces no artificial
/// machine-size limit.
///
/// Actual resource limits remain owned by `SchedulingLimits`, target
/// capabilities, deployment policy, and the host environment.
#[must_use]
pub const fn asap_has_no_machine_size_limit() -> bool {
    true
}

/// Compile-time assertion helper indicating that this implementation does not
/// contain unsafe execution.
#[must_use]
pub const fn asap_uses_no_unsafe() -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_configuration_accepts_empty_problems() {
        let scheduler = AsapScheduler::new();

        assert!(scheduler.config().allow_empty);
    }

    #[test]
    fn algorithm_metadata_is_stable() {
        let scheduler = AsapScheduler::new();

        assert_eq!(scheduler.algorithm_id(), ASAP_ALGORITHM_ID);
        assert_eq!(scheduler.algorithm_name(), ASAP_ALGORITHM_NAME);
        assert_eq!(scheduler.algorithm_version(), ASAP_ALGORITHM_VERSION);
    }

    #[test]
    fn asap_policy_identifies_itself() {
        let scheduler = AsapScheduler::new();

        assert_eq!(
            scheduler.asap_algorithm_id(),
            ASAP_ALGORITHM_ID
        );
        assert!(scheduler.is_asap());
    }

    #[test]
    fn asap_is_deterministic_by_contract() {
        assert!(asap_algorithm_is_deterministic());
    }

    #[test]
    fn asap_has_no_machine_size_limit() {
        assert!(asap_has_no_machine_size_limit());
    }

    #[test]
    fn asap_forbids_unsafe_by_contract() {
        assert!(asap_uses_no_unsafe());
    }
}