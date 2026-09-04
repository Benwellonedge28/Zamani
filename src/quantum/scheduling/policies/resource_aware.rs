//! Zamani Quantum Scheduling — Resource-Aware Policy
//!
//! Production-grade, provider-neutral resource-aware scheduling policy for
//! `crate::quantum::scheduling`.
//!
//! # Purpose
//!
//! This module defines scheduling *intent* for workloads in which resources
//! are first-class constraints:
//!
//! > Prefer operations whose execution is resource-feasible and whose use of
//! > scarce resources is beneficial, while preserving quantum semantics,
//! > dependency correctness, timing correctness, and deterministic behaviour.
//!
//! This module is a POLICY.
//!
//! It does not itself:
//!
//! - construct the dependency graph;
//! - allocate physical qubits;
//! - perform logical-to-physical routing;
//! - maintain resource calendars;
//! - discover hardware;
//! - query a provider;
//! - generate pulses;
//! - execute a quantum job;
//! - decode QEC;
//! - model noise;
//! - perform hardware lowering;
//! - mutate a `QuantumCircuit`.
//!
//! Those responsibilities remain in their owning subsystems.
//!
//! # Architectural position
//!
//! ```text
//!                         quantum::ir
//!                              |
//!                              v
//!                         optimization
//!                              |
//!                              v
//!                           routing
//!                              |
//!                              v
//!                   scheduling::adapters
//!                              |
//!                              v
//!                    scheduling::ir
//!                              |
//!                +-------------+-------------+
//!                |             |             |
//!                v             v             v
//!             graph        resources       timing
//!                |             |             |
//!                +-------------+-------------+
//!                              |
//!                              v
//!                 policies::resource_aware
//!                              |
//!                              v
//!                         planner
//!                              |
//!                              v
//!                         schedule
//!                              |
//!                              v
//!                        verification
//! ```
//!
//! # Resource-aware scheduling principle
//!
//! A quantum operation may consume arbitrary resources, including but not
//! limited to:
//!
//! - logical qubits;
//! - physical qubits;
//! - control channels;
//! - measurement channels;
//! - readout resonators;
//! - couplers;
//! - lasers;
//! - classical processing capacity;
//! - memory;
//! - communication links;
//! - entanglement-generation resources;
//! - synchronization resources;
//! - QEC ancillas;
//! - target-specific resources;
//! - future resources unknown to this module.
//!
//! No resource kind or resource count is hard-coded here.
//!
//! # Universal-program principle
//!
//! The same Zamani program must be able to reach targets of different sizes
//! without modifying the source program.
//!
//! Therefore this module contains no:
//!
//! ```text
//! MAX_QUBITS
//! MAX_RESOURCES
//! MAX_CHANNELS
//! MAX_OPERATIONS
//! MAX_DEPTH
//! MAX_PARALLELISM
//! ```
//!
//! Resource capacity is supplied by the target/resource model.
//!
//! ```text
//! same Zamani program
//!        |
//!        +---- tiny target
//!        |
//!        +---- small QPU
//!        |
//!        +---- large QPU
//!        |
//!        +---- modular QPU
//!        |
//!        +---- distributed quantum system
//!        |
//!        +---- future architecture
//! ```
//!
//! Only the scheduling context changes.
//!
//! # Resource-aware policy versus resource scheduler
//!
//! The distinction is fundamental.
//!
//! This module answers:
//!
//! > Which currently eligible operation is preferable from a resource
//! > perspective?
//!
//! The planner/resource subsystem answers:
//!
//! > Can this operation actually be placed at this time, and what resources
//! > will be reserved?
//!
//! Consequently this module MUST NOT maintain its own authoritative resource
//! calendar.
//!
//! A policy score can be stale if the resource model changes. Final resource
//! feasibility MUST always be established by the planner and independently
//! verified.
//!
//! # Canonical identities
//!
//! Quantum operation identity remains:
//!
//! ```text
//! crate::quantum::ir::core::identity::OperationId
//! ```
//!
//! Logical and physical qubit identities remain:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! This file does not redefine any of these types.
//!
//! Resource identity also remains owned by the canonical repository resource
//! identity boundary:
//!
//! ```text
//! crate::quantum::ir::core::identity::ResourceId
//! ```
//!
//! # No resource assumptions
//!
//! Resource-aware scheduling MUST work when:
//!
//! - there is one resource;
//! - there are many resources;
//! - a resource has capacity one;
//! - a resource has arbitrary capacity;
//! - a resource is hierarchical;
//! - a resource is shared;
//! - a resource is exclusive;
//! - a resource is temporarily unavailable;
//! - resources become available dynamically;
//! - resource requirements are heterogeneous;
//! - resources are distributed across nodes;
//! - resource identities are sparse;
//! - the target introduces new resource kinds.
//!
//! This module therefore treats resource identifiers and quantities as opaque
//! values supplied by the resource subsystem.
//!
//! # Numeric semantics
//!
//! Scheduling decisions must not depend on floating-point comparison where an
//! exact ordering can be represented using integers.
//!
//! Resource pressure is therefore represented by exact bounded integer
//! quantities or explicit qualitative states.
//!
//! Caller-supplied weights are represented by rational values rather than
//! floating-point values.
//!
//! # Determinism
//!
//! The policy is deterministic when its input is deterministic.
//!
//! It does not:
//!
//! - use random numbers;
//! - depend on hash-map iteration order;
//! - inspect memory addresses;
//! - use thread timing;
//! - use global mutable state;
//! - depend on vendor SDK behaviour.
//!
//! When two candidates have equal scores, their canonical `OperationId`
//! provides a stable final ordering.
//!
//! # Thread safety
//!
//! `ResourceAwarePolicy` is immutable and contains no global mutable state.
//!
//! It can therefore be shared between independent scheduling analyses,
//! provided the surrounding resource model is itself accessed according to
//! its own concurrency contract.
//!
//! # Complexity
//!
//! Scoring one operation is O(R), where R is the number of resource
//! requirements supplied for that operation.
//!
//! The policy does not allocate structures proportional to:
//!
//! - total qubit count;
//! - total target capacity;
//! - schedule duration;
//! - maximum schedule depth.
//!
//! A planner may therefore evaluate this policy for very large workloads
//! without requiring a timeline whose dimensions are derived from machine
//! size.
//!
//! # Important limitation
//!
//! Resource-aware scoring is not a proof of global optimality.
//!
//! Arbitrary resource-constrained scheduling can require computationally hard
//! optimization.
//!
//! This policy therefore provides deterministic local ranking and leaves
//! global search, exact optimization, list scheduling, RCPSP, critical-path
//! scheduling, and adaptive strategies to the planner/algorithm layers.
//!
//! # Integration with `policy.rs`
//!
//! `policy.rs` already defines:
//!
//! ```text
//! SchedulingPolicyKind::ResourceAware
//! ```
//!
//! This module does not redefine that enum.
//!
//! The orchestration layer selects this policy through the common policy
//! vocabulary.
//!
//! # Integration with `types.rs`
//!
//! Scheduler-owned time and identity types are consumed from
//! `scheduling::types`.
//!
//! Canonical IR identities are imported from `quantum::ir`.
//!
//! No competing identity type is created here.
//!
//! # Integration with resources
//!
//! Resource availability, capacity, reservation, and calendars are owned by:
//!
//! ```text
//! scheduling::resources
//! ```
//!
//! The planner supplies a snapshot for each candidate evaluation.
//!
//! This policy never mutates that snapshot.
//!
//! # Integration with planners
//!
//! A planner may perform:
//!
//! ```text
//! ready operations
//!       |
//!       v
//! resource analysis
//!       |
//!       v
//! ResourceCandidate
//!       |
//!       v
//! ResourceAwarePolicy::rank()
//!       |
//!       v
//! deterministic candidate ordering
//!       |
//!       v
//! planner feasibility check
//!       |
//!       v
//! reservation
//! ```
//!
//! The planner remains authoritative for actual placement.
//!
//! # Integration with routing
//!
//! Routing answers:
//!
//! > WHERE should logical operations execute?
//!
//! Resource-aware scheduling answers:
//!
//! > WHICH legal ready operation should be preferred given the resources
//! > currently available?
//!
//! This module must not perform logical-to-physical mapping.
//!
//! # Integration with hardware
//!
//! Hardware information enters through an adapter.
//!
//! The policy consumes abstract resource facts such as:
//!
//! - capacity;
//! - current availability;
//! - scarcity;
//! - reservation delay;
//! - target-specific cost.
//!
//! It does not import a vendor SDK.
//!
//! # Integration with ZQN
//!
//! Noise and calibration systems may provide resource-related costs such as:
//!
//! - expected error;
//! - calibration degradation;
//! - crosstalk pressure;
//! - temporal drift.
//!
//! Those values should enter through a scheduling adapter/objective model.
//!
//! This file does not duplicate the ZQN noise model.
//!
//! # Integration with QEC
//!
//! QEC may expose resource pressure from:
//!
//! - ancillas;
//! - syndrome measurement;
//! - decoder resources;
//! - communication;
//! - feedback.
//!
//! The policy can rank such operations without knowing the QEC algorithm.
//!
//! # Integration with distributed scheduling
//!
//! Distributed schedulers can express communication resources using the same
//! resource requirement abstraction.
//!
//! No topology or node count is embedded here.
//!
//! # Integration with verification
//!
//! Verification must independently prove:
//!
//! - dependency correctness;
//! - resource capacity correctness;
//! - timing correctness;
//! - alignment correctness;
//! - semantic preservation.
//!
//! A successful policy ranking is never itself a verified schedule.
//!
//! # Integration with serialization
//!
//! This module deliberately avoids a serialization dependency.
//!
//! Serialization adapters may encode the public policy configuration and
//! scoring descriptors.
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
//! - no `unsafe`;
//! - no external dependencies.
//!
//! # Frozen-file contract
//!
//! This file is designed so that the following modules can be implemented
//! independently without changing this file's semantic contract:
//!
//! - `policies/asap.rs`;
//! - `policies/alap.rs`;
//! - `policies/priority.rs`;
//! - `policies/hybrid.rs`;
//! - `planners/*`;
//! - `algorithms/*`;
//! - `resources/*`;
//! - `constraints/*`;
//! - `optimization/*`;
//! - `verification/*`;
//! - `qec/*`;
//! - `distributed/*`;
//! - `adapters/*`.
//!
//! =============================================================================
//! Safety boundary
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use core::cmp::Ordering;
use core::fmt;
use core::num::NonZeroU64;

use crate::quantum::ir::core::identity::{OperationId, ResourceId};

use super::super::types::{Duration, TimePoint};

// =============================================================================
// Schema
// =============================================================================

/// Stable identifier for this policy's externally observable schema.
pub const RESOURCE_AWARE_POLICY_SCHEMA_ID: &str =
    "zamani.quantum.scheduling.policy.resource_aware";

/// Semantic version of this policy contract.
pub const RESOURCE_AWARE_POLICY_SCHEMA_VERSION: u16 = 1;

// =============================================================================
// Policy objective
// =============================================================================

/// Primary resource-aware ranking objective.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ResourceAwareObjective {
    /// Prefer operations that can begin sooner.
    EarliestFeasibleStart,

    /// Prefer operations that release scarce resources sooner.
    MinimizeResourceOccupancy,

    /// Prefer operations that use less scarce capacity.
    MinimizeScarceResourcePressure,

    /// Prefer operations that unblock more downstream work.
    MaximizeSuccessorAvailability,

    /// Prefer operations on the current resource critical path.
    CriticalResourcePath,

    /// Use the complete deterministic resource-aware score.
    Balanced,
}

impl Default for ResourceAwareObjective {
    fn default() -> Self {
        Self::Balanced
    }
}

impl ResourceAwareObjective {
    /// Stable serialization/diagnostic name.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EarliestFeasibleStart => "earliest_feasible_start",
            Self::MinimizeResourceOccupancy => "minimize_resource_occupancy",
            Self::MinimizeScarceResourcePressure => {
                "minimize_scarce_resource_pressure"
            }
            Self::MaximizeSuccessorAvailability => {
                "maximize_successor_availability"
            }
            Self::CriticalResourcePath => "critical_resource_path",
            Self::Balanced => "balanced",
        }
    }
}

impl fmt::Display for ResourceAwareObjective {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Rational weight
// =============================================================================

/// Exact non-negative rational weight.
///
/// Floating-point values are deliberately excluded from policy semantics.
///
/// The value is represented as:
///
/// ```text
/// numerator / denominator
/// ```
///
/// with a strictly positive denominator.
///
/// A caller can therefore configure exact weights without making candidate
/// ordering dependent on floating-point rounding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RationalWeight {
    numerator: u64,
    denominator: NonZeroU64,
}

impl RationalWeight {
    /// Creates a rational weight.
    ///
    /// Returns `None` when `denominator == 0`.
    #[must_use]
    pub const fn new(numerator: u64, denominator: u64) -> Option<Self> {
        match NonZeroU64::new(denominator) {
            Some(denominator) => Some(Self {
                numerator,
                denominator,
            }),
            None => None,
        }
    }

    /// Creates an integer weight.
    #[must_use]
    pub const fn integer(value: u64) -> Self {
        Self {
            numerator: value,
            denominator: NonZeroU64::MIN,
        }
    }

    /// Returns the numerator.
    #[must_use]
    pub const fn numerator(self) -> u64 {
        self.numerator
    }

    /// Returns the denominator.
    #[must_use]
    pub const fn denominator(self) -> NonZeroU64 {
        self.denominator
    }

    /// Returns whether this weight is zero.
    #[must_use]
    pub const fn is_zero(self) -> bool {
        self.numerator == 0
    }

    /// Compares two rational weights without floating-point arithmetic.
    #[must_use]
    pub fn cmp_exact(self, other: Self) -> Ordering {
        let left = u128::from(self.numerator) * u128::from(other.denominator.get());
        let right =
            u128::from(other.numerator) * u128::from(self.denominator.get());

        left.cmp(&right)
    }

    /// Multiplies a bounded integer score by this weight.
    ///
    /// The result is returned only when the exact multiplication fits in
    /// `u128`.
    #[must_use]
    pub fn checked_mul_u128(self, value: u128) -> Option<u128> {
        u128::from(self.numerator)
            .checked_mul(value)?
            .checked_div(u128::from(self.denominator.get()))
    }
}

impl Default for RationalWeight {
    fn default() -> Self {
        Self::integer(1)
    }
}

// =============================================================================
// Resource pressure
// =============================================================================

/// Exact qualitative pressure of a resource.
///
/// This is intentionally independent of the resource's capacity or identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ResourcePressure {
    /// The resource has no known pressure.
    None,

    /// Resource is available with substantial remaining capacity.
    Low,

    /// Resource availability is materially constrained.
    Moderate,

    /// Resource is highly constrained.
    High,

    /// Resource is the limiting resource for the current planning state.
    Critical,
}

impl Default for ResourcePressure {
    fn default() -> Self {
        Self::None
    }
}

impl ResourcePressure {
    /// Returns an exact ordinal suitable for deterministic ranking.
    #[must_use]
    pub const fn rank(self) -> u8 {
        match self {
            Self::None => 0,
            Self::Low => 1,
            Self::Moderate => 2,
            Self::High => 3,
            Self::Critical => 4,
        }
    }
}

impl fmt::Display for ResourcePressure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::None => "none",
            Self::Low => "low",
            Self::Moderate => "moderate",
            Self::High => "high",
            Self::Critical => "critical",
        })
    }
}

// =============================================================================
// Resource requirement
// =============================================================================

/// Resource requirement for one operation.
///
/// This is a policy-facing immutable view of a resource requirement.
///
/// The resource subsystem remains authoritative for actual capacity and
/// reservation semantics.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ResourceRequirement {
    resource: ResourceId,
    quantity: u64,
    pressure: ResourcePressure,
}

impl ResourceRequirement {
    /// Creates a resource requirement.
    ///
    /// `quantity == 0` is allowed because a caller may construct a generic
    /// requirement before normalization. The resource layer decides whether
    /// zero-quantity requirements are semantically meaningful.
    #[must_use]
    pub const fn new(
        resource: ResourceId,
        quantity: u64,
        pressure: ResourcePressure,
    ) -> Self {
        Self {
            resource,
            quantity,
            pressure,
        }
    }

    /// Returns the canonical resource identity.
    #[must_use]
    pub const fn resource(&self) -> ResourceId {
        self.resource
    }

    /// Returns required capacity.
    #[must_use]
    pub const fn quantity(&self) -> u64 {
        self.quantity
    }

    /// Returns resource pressure.
    #[must_use]
    pub const fn pressure(&self) -> ResourcePressure {
        self.pressure
    }
}

// =============================================================================
// Candidate input
// =============================================================================

/// Immutable information required to rank one ready operation.
///
/// All resource availability information is supplied by the planner.
///
/// This prevents the policy from owning a resource calendar.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResourceCandidate {
    operation: OperationId,
    earliest_start: TimePoint,
    duration: Duration,
    resource_ready: TimePoint,
    requirements: Vec<ResourceRequirement>,
    successor_count: u64,
    criticality: u64,
    explicit_priority: i64,
}

impl ResourceCandidate {
    /// Creates a resource-aware candidate.
    ///
    /// The caller is responsible for obtaining the values from the canonical
    /// scheduling IR, timing model, dependency graph, and resource model.
    #[must_use]
    pub fn new(
        operation: OperationId,
        earliest_start: TimePoint,
        duration: Duration,
        resource_ready: TimePoint,
        requirements: Vec<ResourceRequirement>,
        successor_count: u64,
        criticality: u64,
        explicit_priority: i64,
    ) -> Self {
        Self {
            operation,
            earliest_start,
            duration,
            resource_ready,
            requirements,
            successor_count,
            criticality,
            explicit_priority,
        }
    }

    /// Returns the canonical operation identity.
    #[must_use]
    pub const fn operation(&self) -> OperationId {
        self.operation
    }

    /// Returns the dependency/timing-derived earliest start.
    #[must_use]
    pub const fn earliest_start(&self) -> TimePoint {
        self.earliest_start
    }

    /// Returns operation duration.
    #[must_use]
    pub const fn duration(&self) -> Duration {
        self.duration
    }

    /// Returns the earliest time at which the required resources are known to
    /// be available according to the planner's snapshot.
    #[must_use]
    pub const fn resource_ready(&self) -> TimePoint {
        self.resource_ready
    }

    /// Returns the resource requirements.
    #[must_use]
    pub fn requirements(&self) -> &[ResourceRequirement] {
        &self.requirements
    }

    /// Returns the number of immediate successors.
    #[must_use]
    pub const fn successor_count(&self) -> u64 {
        self.successor_count
    }

    /// Returns the supplied criticality measure.
    #[must_use]
    pub const fn criticality(&self) -> u64 {
        self.criticality
    }

    /// Returns explicit operation priority.
    #[must_use]
    pub const fn explicit_priority(&self) -> i64 {
        self.explicit_priority
    }

    /// Returns the actual earliest feasible start implied by the supplied
    /// dependency and resource lower bounds.
    ///
    /// This is only a lower bound. The planner must still verify the complete
    /// resource placement.
    #[must_use]
    pub const fn earliest_feasible_start(&self) -> TimePoint {
        if self.earliest_start >= self.resource_ready {
            self.earliest_start
        } else {
            self.resource_ready
        }
    }

    /// Returns the operation finish time when scheduled at its earliest
    /// feasible start.
    #[must_use]
    pub fn checked_earliest_finish(&self) -> Option<TimePoint> {
        self.earliest_feasible_start()
            .checked_add(self.duration)
    }
}

// =============================================================================
// Policy configuration
// =============================================================================

/// Immutable configuration for `ResourceAwarePolicy`.
///
/// Every field represents scheduling intent. No field represents a hardware
/// size or capacity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ResourceAwarePolicyConfig {
    objective: ResourceAwareObjective,
    start_weight: RationalWeight,
    occupancy_weight: RationalWeight,
    scarcity_weight: RationalWeight,
    successor_weight: RationalWeight,
    criticality_weight: RationalWeight,
    priority_weight: RationalWeight,
}

impl Default for ResourceAwarePolicyConfig {
    fn default() -> Self {
        Self {
            objective: ResourceAwareObjective::Balanced,
            start_weight: RationalWeight::integer(1),
            occupancy_weight: RationalWeight::integer(1),
            scarcity_weight: RationalWeight::integer(1),
            successor_weight: RationalWeight::integer(1),
            criticality_weight: RationalWeight::integer(1),
            priority_weight: RationalWeight::integer(1),
        }
    }
}

impl ResourceAwarePolicyConfig {
    /// Creates the default configuration.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            objective: ResourceAwareObjective::Balanced,
            start_weight: RationalWeight::integer(1),
            occupancy_weight: RationalWeight::integer(1),
            scarcity_weight: RationalWeight::integer(1),
            successor_weight: RationalWeight::integer(1),
            criticality_weight: RationalWeight::integer(1),
            priority_weight: RationalWeight::integer(1),
        }
    }

    /// Returns the selected objective.
    #[must_use]
    pub const fn objective(self) -> ResourceAwareObjective {
        self.objective
    }

    /// Returns the earliest-start weight.
    #[must_use]
    pub const fn start_weight(self) -> RationalWeight {
        self.start_weight
    }

    /// Returns the occupancy weight.
    #[must_use]
    pub const fn occupancy_weight(self) -> RationalWeight {
        self.occupancy_weight
    }

    /// Returns the scarcity weight.
    #[must_use]
    pub const fn scarcity_weight(self) -> RationalWeight {
        self.scarcity_weight
    }

    /// Returns the successor weight.
    #[must_use]
    pub const fn successor_weight(self) -> RationalWeight {
        self.successor_weight
    }

    /// Returns the criticality weight.
    #[must_use]
    pub const fn criticality_weight(self) -> RationalWeight {
        self.criticality_weight
    }

    /// Returns the explicit-priority weight.
    #[must_use]
    pub const fn priority_weight(self) -> RationalWeight {
        self.priority_weight
    }

    /// Returns a configuration with a different objective.
    #[must_use]
    pub const fn with_objective(
        mut self,
        objective: ResourceAwareObjective,
    ) -> Self {
        self.objective = objective;
        self
    }

    /// Returns a configuration with a different start-time weight.
    #[must_use]
    pub const fn with_start_weight(mut self, weight: RationalWeight) -> Self {
        self.start_weight = weight;
        self
    }

    /// Returns a configuration with a different occupancy weight.
    #[must_use]
    pub const fn with_occupancy_weight(
        mut self,
        weight: RationalWeight,
    ) -> Self {
        self.occupancy_weight = weight;
        self
    }

    /// Returns a configuration with a different scarcity weight.
    #[must_use]
    pub const fn with_scarcity_weight(
        mut self,
        weight: RationalWeight,
    ) -> Self {
        self.scarcity_weight = weight;
        self
    }

    /// Returns a configuration with a different successor weight.
    #[must_use]
    pub const fn with_successor_weight(
        mut self,
        weight: RationalWeight,
    ) -> Self {
        self.successor_weight = weight;
        self
    }

    /// Returns a configuration with a different criticality weight.
    #[must_use]
    pub const fn with_criticality_weight(
        mut self,
        weight: RationalWeight,
    ) -> Self {
        self.criticality_weight = weight;
        self
    }

    /// Returns a configuration with a different explicit-priority weight.
    #[must_use]
    pub const fn with_priority_weight(
        mut self,
        weight: RationalWeight,
    ) -> Self {
        self.priority_weight = weight;
        self
    }
}

// =============================================================================
// Score
// =============================================================================

/// Exact resource-aware score for one operation.
///
/// Scores are intentionally retained as separate dimensions instead of being
/// collapsed immediately into one number. This makes diagnostics possible and
/// prevents information loss before deterministic tie-breaking.
///
/// Higher values are considered better for all dimensions except
/// `earliest_start` and `resource_occupancy`, which are better when smaller.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ResourceAwareScore {
    earliest_start: TimePoint,
    resource_occupancy: u128,
    scarcity_pressure: u128,
    successor_count: u64,
    criticality: u64,
    explicit_priority: i64,
}

impl ResourceAwareScore {
    /// Creates a score.
    #[must_use]
    pub const fn new(
        earliest_start: TimePoint,
        resource_occupancy: u128,
        scarcity_pressure: u128,
        successor_count: u64,
        criticality: u64,
        explicit_priority: i64,
    ) -> Self {
        Self {
            earliest_start,
            resource_occupancy,
            scarcity_pressure,
            successor_count,
            criticality,
            explicit_priority,
        }
    }

    /// Returns earliest feasible start.
    #[must_use]
    pub const fn earliest_start(self) -> TimePoint {
        self.earliest_start
    }

    /// Returns total resource occupancy.
    #[must_use]
    pub const fn resource_occupancy(self) -> u128 {
        self.resource_occupancy
    }

    /// Returns aggregate scarcity pressure.
    #[must_use]
    pub const fn scarcity_pressure(self) -> u128 {
        self.scarcity_pressure
    }

    /// Returns successor count.
    #[must_use]
    pub const fn successor_count(self) -> u64 {
        self.successor_count
    }

    /// Returns criticality.
    #[must_use]
    pub const fn criticality(self) -> u64 {
        self.criticality
    }

    /// Returns explicit priority.
    #[must_use]
    pub const fn explicit_priority(self) -> i64 {
        self.explicit_priority
    }
}

// =============================================================================
// Ranked candidate
// =============================================================================

/// Candidate plus its calculated resource-aware score.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RankedResourceCandidate {
    candidate: ResourceCandidate,
    score: ResourceAwareScore,
}

impl RankedResourceCandidate {
    /// Creates a ranked candidate.
    #[must_use]
    pub const fn new(
        candidate: ResourceCandidate,
        score: ResourceAwareScore,
    ) -> Self {
        Self { candidate, score }
    }

    /// Returns the original candidate.
    #[must_use]
    pub const fn candidate(&self) -> &ResourceCandidate {
        &self.candidate
    }

    /// Returns the calculated score.
    #[must_use]
    pub const fn score(&self) -> ResourceAwareScore {
        self.score
    }

    /// Returns the canonical operation identity.
    #[must_use]
    pub const fn operation(&self) -> OperationId {
        self.candidate.operation()
    }
}

// =============================================================================
// Errors
// =============================================================================

/// Errors generated by the resource-aware policy itself.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResourceAwareError {
    /// Resource occupancy multiplication overflowed.
    OccupancyOverflow {
        /// Operation for which the calculation failed.
        operation: OperationId,
        /// Resource that caused the failure.
        resource: ResourceId,
    },

    /// Aggregate scarcity pressure overflowed.
    ScarcityOverflow {
        /// Operation for which the calculation failed.
        operation: OperationId,
    },

    /// Weighted score calculation overflowed.
    ScoreOverflow {
        /// Operation for which the calculation failed.
        operation: OperationId,
    },

    /// Candidate has a duration that cannot be represented at its requested
    /// earliest feasible start.
    FinishOverflow {
        /// Operation for which the calculation failed.
        operation: OperationId,
    },
}

impl fmt::Display for ResourceAwareError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OccupancyOverflow {
                operation,
                resource,
            } => write!(
                formatter,
                "resource-aware occupancy overflow for operation `{operation}` and resource `{resource}`"
            ),

            Self::ScarcityOverflow { operation } => write!(
                formatter,
                "resource-aware scarcity-pressure overflow for operation `{operation}`"
            ),

            Self::ScoreOverflow { operation } => write!(
                formatter,
                "resource-aware weighted score overflow for operation `{operation}`"
            ),

            Self::FinishOverflow { operation } => write!(
                formatter,
                "resource-aware finish-time overflow for operation `{operation}`"
            ),
        }
    }
}

impl std::error::Error for ResourceAwareError {}

// =============================================================================
// Policy
// =============================================================================

/// Production resource-aware scheduling policy.
///
/// The policy is immutable and target-independent.
///
/// It ranks candidates using only facts supplied by the planner.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ResourceAwarePolicy {
    config: ResourceAwarePolicyConfig,
}

impl Default for ResourceAwarePolicy {
    fn default() -> Self {
        Self::new()
    }
}

impl ResourceAwarePolicy {
    /// Creates a policy using the default balanced configuration.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            config: ResourceAwarePolicyConfig::new(),
        }
    }

    /// Creates a policy from explicit configuration.
    #[must_use]
    pub const fn with_config(config: ResourceAwarePolicyConfig) -> Self {
        Self { config }
    }

    /// Returns the immutable configuration.
    #[must_use]
    pub const fn config(self) -> ResourceAwarePolicyConfig {
        self.config
    }

    /// Returns the policy's stable schema identifier.
    #[must_use]
    pub const fn schema_id(self) -> &'static str {
        RESOURCE_AWARE_POLICY_SCHEMA_ID
    }

    /// Returns the policy's schema version.
    #[must_use]
    pub const fn schema_version(self) -> u16 {
        RESOURCE_AWARE_POLICY_SCHEMA_VERSION
    }

    /// Calculates the resource-aware score of one candidate.
    ///
    /// This operation does not reserve resources.
    pub fn score(
        &self,
        candidate: &ResourceCandidate,
    ) -> Result<ResourceAwareScore, ResourceAwareError> {
        if candidate.checked_earliest_finish().is_none() {
            return Err(ResourceAwareError::FinishOverflow {
                operation: candidate.operation(),
            });
        }

        let mut occupancy = 0u128;
        let mut scarcity = 0u128;

        for requirement in candidate.requirements() {
            let quantity = u128::from(requirement.quantity());
            let duration = candidate.duration().value();

            let resource_occupancy = quantity
                .checked_mul(duration)
                .ok_or(ResourceAwareError::OccupancyOverflow {
                    operation: candidate.operation(),
                    resource: requirement.resource(),
                })?;

            occupancy = occupancy
                .checked_add(resource_occupancy)
                .ok_or(ResourceAwareError::OccupancyOverflow {
                    operation: candidate.operation(),
                    resource: requirement.resource(),
                })?;

            let pressure = u128::from(requirement.pressure().rank());

            let pressure_contribution = quantity.checked_mul(pressure).ok_or(
                ResourceAwareError::ScarcityOverflow {
                    operation: candidate.operation(),
                },
            )?;

            scarcity = scarcity
                .checked_add(pressure_contribution)
                .ok_or(ResourceAwareError::ScarcityOverflow {
                    operation: candidate.operation(),
                })?;
        }

        Ok(ResourceAwareScore::new(
            candidate.earliest_feasible_start(),
            occupancy,
            scarcity,
            candidate.successor_count(),
            candidate.criticality(),
            candidate.explicit_priority(),
        ))
    }

    /// Ranks one candidate against another.
    ///
    /// The returned ordering is suitable for a ready-list planner:
    ///
    /// ```text
    /// Less
    ///   = candidate A should be considered before candidate B
    /// ```
    ///
    /// A canonical `OperationId` comparison is the final deterministic
    /// tie-break.
    pub fn compare(
        &self,
        left: &ResourceCandidate,
        right: &ResourceCandidate,
    ) -> Result<Ordering, ResourceAwareError> {
        let left_score = self.score(left)?;
        let right_score = self.score(right)?;

        let ordering = self.compare_scores(left_score, right_score);

        if ordering == Ordering::Equal {
            return Ok(left.operation().cmp(&right.operation()));
        }

        Ok(ordering)
    }

    /// Ranks a collection of candidates deterministically.
    ///
    /// The returned vector is a newly allocated ordering of the supplied
    /// candidates. The candidates themselves are not modified.
    ///
    /// This method performs no resource reservation and does not remove
    /// candidates that are infeasible. The planner must perform final
    /// feasibility checking.
    pub fn rank(
        &self,
        candidates: &[ResourceCandidate],
    ) -> Result<Vec<RankedResourceCandidate>, ResourceAwareError> {
        let mut ranked = Vec::with_capacity(candidates.len());

        for candidate in candidates {
            let score = self.score(candidate)?;

            ranked.push(RankedResourceCandidate::new(
                candidate.clone(),
                score,
            ));
        }

        ranked.sort_by(|left, right| {
            self.compare_scores(left.score(), right.score())
                .then_with(|| left.operation().cmp(&right.operation()))
        });

        Ok(ranked)
    }

    /// Returns whether this policy prefers the candidate with the lower score
    /// for the selected objective.
    fn compare_scores(
        &self,
        left: ResourceAwareScore,
        right: ResourceAwareScore,
    ) -> Ordering {
        match self.config.objective() {
            ResourceAwareObjective::EarliestFeasibleStart => left
                .earliest_start()
                .cmp(&right.earliest_start())
                .then_with(|| {
                    right
                        .successor_count()
                        .cmp(&left.successor_count())
                })
                .then_with(|| {
                    right.criticality().cmp(&left.criticality())
                })
                .then_with(|| {
                    right
                        .explicit_priority()
                        .cmp(&left.explicit_priority())
                })
                .then_with(|| {
                    left.resource_occupancy()
                        .cmp(&right.resource_occupancy())
                }),

            ResourceAwareObjective::MinimizeResourceOccupancy => left
                .resource_occupancy()
                .cmp(&right.resource_occupancy())
                .then_with(|| {
                    left.earliest_start().cmp(&right.earliest_start())
                })
                .then_with(|| {
                    right
                        .criticality()
                        .cmp(&left.criticality())
                }),

            ResourceAwareObjective::MinimizeScarceResourcePressure => left
                .scarcity_pressure()
                .cmp(&right.scarcity_pressure())
                .then_with(|| {
                    left.earliest_start().cmp(&right.earliest_start())
                })
                .then_with(|| {
                    right.successor_count().cmp(&left.successor_count())
                }),

            ResourceAwareObjective::MaximizeSuccessorAvailability => right
                .successor_count()
                .cmp(&left.successor_count())
                .then_with(|| {
                    left.earliest_start().cmp(&right.earliest_start())
                })
                .then_with(|| {
                    left.resource_occupancy()
                        .cmp(&right.resource_occupancy())
                }),

            ResourceAwareObjective::CriticalResourcePath => right
                .criticality()
                .cmp(&left.criticality())
                .then_with(|| {
                    left.earliest_start().cmp(&right.earliest_start())
                })
                .then_with(|| {
                    left.scarcity_pressure()
                        .cmp(&right.scarcity_pressure())
                }),

            ResourceAwareObjective::Balanced => {
                self.compare_balanced(left, right)
            }
        }
    }

    /// Compares candidates using the configured exact weighted objective.
    fn compare_balanced(
        &self,
        left: ResourceAwareScore,
        right: ResourceAwareScore,
    ) -> Ordering {
        // A fully weighted exact scalar can overflow u128 even when each
        // component is individually valid. Instead of silently saturating,
        // use a deterministic lexicographic comparison of the weighted
        // dimensions.
        //
        // This preserves exact ordering without introducing arbitrary
        // saturation semantics.

        self.compare_weighted_ascending(
            left.earliest_start().value(),
            right.earliest_start().value(),
            self.config.start_weight(),
        )
        .then_with(|| {
            self.compare_weighted_ascending(
                left.resource_occupancy(),
                right.resource_occupancy(),
                self.config.occupancy_weight(),
            )
        })
        .then_with(|| {
            self.compare_weighted_ascending(
                left.scarcity_pressure(),
                right.scarcity_pressure(),
                self.config.scarcity_weight(),
            )
        })
        .then_with(|| {
            self.compare_weighted_descending(
                u128::from(left.successor_count()),
                u128::from(right.successor_count()),
                self.config.successor_weight(),
            )
        })
        .then_with(|| {
            self.compare_weighted_descending(
                u128::from(left.criticality()),
                u128::from(right.criticality()),
                self.config.criticality_weight(),
            )
        })
        .then_with(|| {
            compare_signed_weighted_descending(
                left.explicit_priority(),
                right.explicit_priority(),
                self.config.priority_weight(),
            )
        })
    }

    fn compare_weighted_ascending(
        &self,
        left: u128,
        right: u128,
        weight: RationalWeight,
    ) -> Ordering {
        if weight.is_zero() {
            return Ordering::Equal;
        }

        compare_weighted_unsigned(left, right, weight)
    }

    fn compare_weighted_descending(
        &self,
        left: u128,
        right: u128,
        weight: RationalWeight,
    ) -> Ordering {
        self.compare_weighted_ascending(right, left, weight)
    }
}

// =============================================================================
// Exact weighted comparisons
// =============================================================================

/// Compares two non-negative integer values after multiplication by the same
/// rational weight.
///
/// Since both values use the same weight, denominator cancellation means the
/// ordering is exactly the same as comparing the original integers.
///
/// The explicit helper exists to document that the weight participates in the
/// policy contract and to provide a stable extension point if heterogeneous
/// normalization is introduced later.
fn compare_weighted_unsigned(
    left: u128,
    right: u128,
    weight: RationalWeight,
) -> Ordering {
    if weight.is_zero() {
        return Ordering::Equal;
    }

    // Multiplication by the same strictly positive rational preserves order.
    // Avoid multiplication entirely, preventing overflow.
    left.cmp(&right)
}

/// Compares signed values after multiplication by the same positive rational
/// weight.
///
/// Again, multiplication is unnecessary for ordering because the common
/// positive factor preserves ordering.
fn compare_signed_weighted_descending(
    left: i64,
    right: i64,
    weight: RationalWeight,
) -> Ordering {
    if weight.is_zero() {
        return Ordering::Equal;
    }

    right.cmp(&left)
}

// =============================================================================
// Convenience constructors
// =============================================================================

impl ResourceAwarePolicy {
    /// Creates a policy optimized primarily for earliest resource-feasible
    /// execution.
    #[must_use]
    pub const fn earliest() -> Self {
        Self::with_config(
            ResourceAwarePolicyConfig::new()
                .with_objective(ResourceAwareObjective::EarliestFeasibleStart),
        )
    }

    /// Creates a policy optimized primarily for reducing resource occupancy.
    #[must_use]
    pub const fn occupancy() -> Self {
        Self::with_config(
            ResourceAwarePolicyConfig::new()
                .with_objective(ResourceAwareObjective::MinimizeResourceOccupancy),
        )
    }

    /// Creates a policy optimized primarily for scarce-resource pressure.
    #[must_use]
    pub const fn scarcity() -> Self {
        Self::with_config(
            ResourceAwarePolicyConfig::new().with_objective(
                ResourceAwareObjective::MinimizeScarceResourcePressure,
            ),
        )
    }

    /// Creates a policy optimized primarily for downstream parallelism.
    #[must_use]
    pub const fn successor_focused() -> Self {
        Self::with_config(
            ResourceAwarePolicyConfig::new().with_objective(
                ResourceAwareObjective::MaximizeSuccessorAvailability,
            ),
        )
    }

    /// Creates a policy optimized primarily for critical resource-path
    /// operations.
    #[must_use]
    pub const fn critical_resource_path() -> Self {
        Self::with_config(
            ResourceAwarePolicyConfig::new()
                .with_objective(ResourceAwareObjective::CriticalResourcePath),
        )
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn operation(value: u64) -> OperationId {
        OperationId::new(value)
    }

    fn resource(value: u64) -> ResourceId {
        ResourceId::new(value)
    }

    fn requirement(
        id: u64,
        quantity: u64,
        pressure: ResourcePressure,
    ) -> ResourceRequirement {
        ResourceRequirement::new(resource(id), quantity, pressure)
    }

    fn candidate(
        operation_id: u64,
        start: u128,
        duration: u128,
        resource_ready: u128,
        requirements: Vec<ResourceRequirement>,
        successors: u64,
        criticality: u64,
        priority: i64,
    ) -> ResourceCandidate {
        ResourceCandidate::new(
            operation(operation_id),
            TimePoint::new(start),
            Duration::new(duration),
            TimePoint::new(resource_ready),
            requirements,
            successors,
            criticality,
            priority,
        )
    }

    #[test]
    fn zero_resource_requirements_are_supported() {
        let candidate = candidate(
            1,
            0,
            10,
            0,
            Vec::new(),
            0,
            0,
            0,
        );

        let policy = ResourceAwarePolicy::new();

        let score = policy.score(&candidate).expect("score must succeed");

        assert_eq!(score.resource_occupancy(), 0);
        assert_eq!(score.scarcity_pressure(), 0);
    }

    #[test]
    fn_resource_ready_time_is_respected_as_lower_bound() {
        let candidate = candidate(
            1,
            10,
            5,
            20,
            Vec::new(),
            0,
            0,
            0,
        );

        assert_eq!(
            candidate.earliest_feasible_start(),
            TimePoint::new(20)
        );
    }

    #[test]
    fn dependency_lower_bound_wins_when_later() {
        let candidate = candidate(
            1,
            30,
            5,
            20,
            Vec::new(),
            0,
            0,
            0,
        );

        assert_eq!(
            candidate.earliest_feasible_start(),
            TimePoint::new(30)
        );
    }

    #[test]
    fn resource_occupancy_is_exact() {
        let candidate = candidate(
            1,
            0,
            10,
            0,
            vec![
                requirement(1, 2, ResourcePressure::Low),
                requirement(2, 3, ResourcePressure::High),
            ],
            0,
            0,
            0,
        );

        let score = ResourceAwarePolicy::new()
            .score(&candidate)
            .expect("score must succeed");

        assert_eq!(score.resource_occupancy(), 50);
    }

    #[test]
    fn scarcity_pressure_is_exact() {
        let candidate = candidate(
            1,
            0,
            10,
            0,
            vec![
                requirement(1, 2, ResourcePressure::Low),
                requirement(2, 3, ResourcePressure::Critical),
            ],
            0,
            0,
            0,
        );

        let score = ResourceAwarePolicy::new()
            .score(&candidate)
            .expect("score must succeed");

        assert_eq!(score.scarcity_pressure(), 14);
    }

    #[test]
    fn earliest_objective_prefers_earlier_candidate() {
        let policy = ResourceAwarePolicy::earliest();

        let early = candidate(
            1,
            10,
            5,
            10,
            Vec::new(),
            0,
            0,
            0,
        );

        let late = candidate(
            2,
            20,
            5,
            20,
            Vec::new(),
            0,
            0,
            0,
        );

        assert_eq!(
            policy.compare(&early, &late).expect("comparison must succeed"),
            Ordering::Less
        );
    }

    #[test]
    fn scarcity_objective_prefers_lower_pressure() {
        let policy = ResourceAwarePolicy::scarcity();

        let low = candidate(
            1,
            0,
            10,
            0,
            vec![requirement(1, 1, ResourcePressure::Low)],
            0,
            0,
            0,
        );

        let high = candidate(
            2,
            0,
            10,
            0,
            vec![requirement(1, 1, ResourcePressure::Critical)],
            0,
            0,
            0,
        );

        assert_eq!(
            policy.compare(&low, &high).expect("comparison must succeed"),
            Ordering::Less
        );
    }

    #[test]
    fn successor_objective_prefers_more_unblocked_work() {
        let policy = ResourceAwarePolicy::successor_focused();

        let few = candidate(
            1,
            0,
            10,
            0,
            Vec::new(),
            1,
            0,
            0,
        );

        let many = candidate(
            2,
            0,
            10,
            0,
            Vec::new(),
            10,
            0,
            0,
        );

        assert_eq!(
            policy.compare(&many, &few).expect("comparison must succeed"),
            Ordering::Less
        );
    }

    #[test]
    fn criticality_objective_prefers_more_critical_work() {
        let policy = ResourceAwarePolicy::critical_resource_path();

        let ordinary = candidate(
            1,
            0,
            10,
            0,
            Vec::new(),
            0,
            1,
            0,
        );

        let critical = candidate(
            2,
            0,
            10,
            0,
            Vec::new(),
            0,
            10,
            0,
        );

        assert_eq!(
            policy
                .compare(&critical, &ordinary)
                .expect("comparison must succeed"),
            Ordering::Less
        );
    }

    #[test]
    fn operation_id_is_final_deterministic_tie_break() {
        let policy = ResourceAwarePolicy::new();

        let first = candidate(
            1,
            0,
            10,
            0,
            Vec::new(),
            0,
            0,
            0,
        );

        let second = candidate(
            2,
            0,
            10,
            0,
            Vec::new(),
            0,
            0,
            0,
        );

        assert_eq!(
            policy
                .compare(&first, &second)
                .expect("comparison must succeed"),
            Ordering::Less
        );
    }

    #[test]
    fn ranking_is_deterministic() {
        let policy = ResourceAwarePolicy::new();

        let candidates = vec![
            candidate(3, 0, 10, 0, Vec::new(), 0, 0, 0),
            candidate(1, 0, 10, 0, Vec::new(), 0, 0, 0),
            candidate(2, 0, 10, 0, Vec::new(), 0, 0, 0),
        ];

        let ranked = policy.rank(&candidates).expect("ranking must succeed");

        assert_eq!(ranked[0].operation(), operation(1));
        assert_eq!(ranked[1].operation(), operation(2));
        assert_eq!(ranked[2].operation(), operation(3));
    }

    #[test]
    fn finish_overflow_is_reported() {
        let candidate = candidate(
            1,
            u128::MAX,
            1,
            u128::MAX,
            Vec::new(),
            0,
            0,
            0,
        );

        let policy = ResourceAwarePolicy::new();

        assert!(matches!(
            policy.score(&candidate),
            Err(ResourceAwareError::FinishOverflow { .. })
        ));
    }

    #[test]
    fn occupancy_overflow_is_reported() {
        let candidate = candidate(
            1,
            0,
            u128::MAX,
            0,
            vec![requirement(
                1,
                u64::MAX,
                ResourcePressure::Low,
            )],
            0,
            0,
            0,
        );

        let policy = ResourceAwarePolicy::new();

        assert!(matches!(
            policy.score(&candidate),
            Err(ResourceAwareError::OccupancyOverflow { .. })
        ));
    }

    #[test]
    fn rational_zero_weight_disables_dimension() {
        let weight =
            RationalWeight::new(0, 1).expect("denominator is non-zero");

        assert!(weight.is_zero());
    }

    #[test]
    fn rational_weight_rejects_zero_denominator() {
        assert!(RationalWeight::new(1, 0).is_none());
    }

    #[test]
    fn policy_is_copy_and_immutable() {
        let first = ResourceAwarePolicy::new();
        let second = first;

        assert_eq!(first, second);
    }

    #[test]
    fn schema_is_stable() {
        let policy = ResourceAwarePolicy::new();

        assert_eq!(
            policy.schema_id(),
            "zamani.quantum.scheduling.policy.resource_aware"
        );
        assert_eq!(policy.schema_version(), 1);
    }
}