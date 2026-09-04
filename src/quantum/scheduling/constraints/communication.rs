//! Zamani Quantum Scheduling — Communication Constraints
//!
//! Path:
//!     src/quantum/scheduling/constraints/communication.rs
//!
//! # Purpose
//!
//! This module provides production-grade, target-independent communication
//! constraints for the Zamani quantum scheduler.
//!
//! It answers:
//!
//! > "Can this operation be placed at this time with respect to the
//! > communication resources, communication readiness requirements, and
//! > explicitly declared communication dependencies supplied by the target
//! > and scheduler context?"
//!
//! Communication is treated as a schedulable resource domain.
//!
//! The module supports:
//!
//! - communication links;
//! - quantum communication resources;
//! - classical communication resources;
//! - synchronization resources;
//! - entanglement-generation resources;
//! - teleportation resources;
//! - remote-operation resources;
//! - shared or exclusive communication resources;
//! - capacity-limited communication resources;
//! - operation-specific communication requirements;
//! - earliest legal start times;
//! - explicit communication dependency completion;
//! - deterministic evaluation;
//! - structured diagnostics;
//! - scalable sparse resource descriptions;
//! - arbitrary communication-resource counts;
//! - arbitrary operation counts;
//! - arbitrary communication topology sizes;
//! - dynamic and distributed scheduling integration.
//!
//! # Architectural responsibility
//!
//! This module OWNS:
//!
//! - communication-specific scheduling constraints;
//! - communication requirement descriptions;
//! - communication resource policies;
//! - communication dependency requirements;
//! - communication readiness requirements;
//! - communication conflict classification;
//! - deterministic communication constraint evaluation;
//! - communication-specific diagnostics.
//!
//! This module DOES NOT own:
//!
//! - quantum semantics;
//! - quantum gate definitions;
//! - logical-to-physical routing;
//! - network topology discovery;
//! - hardware discovery;
//! - hardware calibration;
//! - QPU execution;
//! - network execution;
//! - entanglement generation;
//! - teleportation protocol implementation;
//! - QEC decoding;
//! - scheduling algorithms;
//! - resource inventory;
//! - resource calendars;
//! - global scheduler state.
//!
//! Those responsibilities belong to the corresponding subsystems.
//!
//! # Architectural position
//!
//! ```text
//! Zamani source
//!       |
//!       v
//! quantum::ir
//!       |
//!       v
//! optimization
//!       |
//!       v
//! routing
//!       |
//!       v
//! scheduling
//!       |
//!       +------------------------------+
//!       |                              |
//!       v                              v
//! dependency analysis          communication constraints
//!                                      |
//!                                      v
//!                              communication resources
//!                                      |
//!                                      v
//!                              resource reservations
//!                                      |
//!                                      v
//!                                schedule result
//! ```
//!
//! Communication constraints therefore answer **WHEN** communication-related
//! resource usage is legal. They do not answer **WHERE** communication should
//! occur. Routing and distributed-network planning determine that.
//!
//! # Canonical identity boundary
//!
//! This module MUST NOT define:
//!
//! - `QubitId`;
//! - `PhysicalQubitId`;
//! - `OperationId`;
//! - `ResourceId`;
//! - `ReservationId`.
//!
//! Canonical operation and resource identities are imported from the existing
//! Zamani IR/scheduling identity model.
//!
//! Canonical qubit identities, when required by higher-level communication
//! adapters, remain owned by:
//!
//! ```text
//! crate::quantum::ir::qubit
//! ```
//!
//! This file does not need to duplicate or reinterpret qubit identity.
//!
//! # Universal-program principle
//!
//! A Zamani program must not contain assumptions such as:
//!
//! ```text
//! 2 communication links
//! 8 network nodes
//! 100 qubits
//! 4 classical channels
//! ```
//!
//! None of those values belong in this file.
//!
//! Communication resources are supplied by the target/context.
//!
//! The same program may therefore be scheduled for:
//!
//! - a single QPU;
//! - a multi-chip QPU;
//! - a modular quantum computer;
//! - a distributed quantum computer;
//! - a quantum data center;
//! - a quantum network;
//! - a heterogeneous quantum/classical system;
//! - a future communication architecture.
//!
//! "Infinity" means that this module introduces no artificial machine-size
//! ceiling. Concrete executions remain bounded by available memory, CPU time,
//! target resources, and explicit compiler limits.
//!
//! # Capacity semantics
//!
//! This module distinguishes between:
//!
//! 1. exclusive communication resources;
//! 2. capacity-limited communication resources;
//! 3. shared communication resources;
//! 4. explicitly serialized communication resources.
//!
//! The constraint does not own the global resource inventory.
//!
//! The target/resource subsystem remains authoritative for actual capacity.
//!
//! This module only applies communication-specific policy supplied to it.
//!
//! # Timing semantics
//!
//! All timing uses the canonical scheduling types:
//!
//! ```text
//! TimePoint
//! Duration
//! ```
//!
//! No nanosecond, picosecond, clock-cycle, or device-specific duration is
//! hard-coded here.
//!
//! Hardware/timing adapters translate target timing into the scheduler's
//! canonical timing representation before evaluation.
//!
//! # Reservation semantics
//!
//! Existing reservations are supplied through `ConstraintState`.
//!
//! The constraint performs interval overlap checks using checked arithmetic.
//!
//! Half-open intervals are assumed:
//!
//! ```text
//! [start, end)
//! ```
//!
//! Therefore:
//!
//! ```text
//! [0, 10)
//! [10, 20)
//! ```
//!
//! do not overlap.
//!
//! # Static and dynamic scheduling
//!
//! The same constraint supports:
//!
//! ```text
//! static compilation
//! dynamic scheduling
//! runtime scheduling
//! distributed scheduling
//! ```
//!
//! For static scheduling, `minimum_start` can be computed by upstream
//! dependency/communication analysis.
//!
//! For dynamic scheduling, the runtime can construct a fresh `ConstraintState`
//! containing operations and resources that have actually completed or become
//! available.
//!
//! # Finish-once rule
//!
//! This file intentionally depends only on:
//!
//! ```text
//! scheduling::constraints::constraint
//! scheduling::types / canonical identity types
//! ```
//!
//! It does not depend on:
//!
//! - planners;
//! - routing algorithms;
//! - hardware providers;
//! - QEC implementations;
//! - runtime implementations;
//! - network implementations.
//!
//! Adding those systems later therefore does not require this file to be
//! rewritten merely to integrate them.
//!
//! # Safety
//!
//! This module uses safe Rust only.
//!
//! Supported:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no `unsafe`.
//!
//! ```text
//! #![forbid(unsafe_code)]
//! ```
//!
//! # Determinism
//!
//! Evaluation is deterministic.
//!
//! This module does not use:
//!
//! - wall-clock time;
//! - global mutable state;
//! - process identifiers;
//! - pointer addresses;
//! - implicit randomness;
//! - hash-map iteration order.
//!
//! Requirements are stored in ordered collections.
//!
//! ============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::collections::BTreeMap;
use std::fmt;

use crate::quantum::ir::core::identity::{OperationId, ResourceId};

use super::constraint::{
    Constraint,
    ConstraintApplicability,
    ConstraintContext,
    ConstraintId,
    ConstraintKind,
    ConstraintSeverity,
    ConstraintViolation,
};

use super::super::types::TimePoint;

// ============================================================================
// Result
// ============================================================================

/// Result type for communication-specific constraint construction.
pub type CommunicationConstraintResult<T> =
    Result<T, CommunicationConstraintError>;

// ============================================================================
// Communication kind
// ============================================================================

/// Semantic category of communication required by an operation.
///
/// This enum describes scheduling semantics, not a concrete implementation.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CommunicationKind {
    /// Quantum information transported between locations.
    Quantum,

    /// Classical information transported between locations.
    Classical,

    /// Entanglement-generation resource usage.
    Entanglement,

    /// Teleportation-related communication.
    Teleportation,

    /// Synchronization between distributed scheduling domains.
    Synchronization,

    /// Remote operation requiring communication support.
    RemoteOperation,

    /// User- or target-defined communication semantic.
    Custom(String),
}

impl CommunicationKind {
    /// Returns a stable machine-readable name.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Quantum => "quantum",
            Self::Classical => "classical",
            Self::Entanglement => "entanglement",
            Self::Teleportation => "teleportation",
            Self::Synchronization => "synchronization",
            Self::RemoteOperation => "remote-operation",
            Self::Custom(name) => name.as_str(),
        }
    }
}

impl fmt::Display for CommunicationKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Custom(name) => write!(formatter, "custom:{name}"),
            _ => formatter.write_str(self.as_str()),
        }
    }
}

// ============================================================================
// Communication resource policy
// ============================================================================

/// Policy describing how a communication resource may be occupied.
///
/// This is a scheduling policy, not a declaration of hardware inventory.
///
/// The hardware/resource adapter remains responsible for supplying the
/// authoritative resource model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CommunicationResourcePolicy {
    /// Maximum simultaneous quantity accepted by this communication policy.
    ///
    /// `None` means that this constraint does not impose an explicit capacity
    /// ceiling and therefore delegates capacity semantics to the resource
    /// subsystem.
    capacity: Option<u128>,

    /// Whether communication usage is serialized regardless of quantity.
    exclusive: bool,
}

impl CommunicationResourcePolicy {
    /// Creates a policy that delegates capacity semantics to the resource
    /// subsystem.
    #[must_use]
    pub const fn delegated() -> Self {
        Self {
            capacity: None,
            exclusive: false,
        }
    }

    /// Creates an exclusive communication-resource policy.
    #[must_use]
    pub const fn exclusive() -> Self {
        Self {
            capacity: Some(1),
            exclusive: true,
        }
    }

    /// Creates a capacity-limited communication-resource policy.
    ///
    /// A zero capacity is rejected by the public constructor returning an
    /// error. Use `unavailable()` when a resource is intentionally disabled.
    pub const fn capacity(
        capacity: u128,
    ) -> CommunicationConstraintResult<Self> {
        if capacity == 0 {
            return Err(CommunicationConstraintError::ZeroCapacity);
        }

        Ok(Self {
            capacity: Some(capacity),
            exclusive: false,
        })
    }

    /// Creates an explicitly unavailable communication resource.
    #[must_use]
    pub const fn unavailable() -> Self {
        Self {
            capacity: Some(0),
            exclusive: false,
        }
    }

    /// Returns the configured capacity.
    #[must_use]
    pub const fn capacity(self) -> Option<u128> {
        self.capacity
    }

    /// Returns whether usage is exclusive.
    #[must_use]
    pub const fn is_exclusive(self) -> bool {
        self.exclusive
    }

    /// Returns whether the resource is explicitly unavailable.
    #[must_use]
    pub const fn is_unavailable(self) -> bool {
        matches!(self.capacity, Some(0))
    }
}

impl Default for CommunicationResourcePolicy {
    fn default() -> Self {
        Self::delegated()
    }
}

// ============================================================================
// Communication requirement
// ============================================================================

/// A communication-resource requirement belonging to one operation.
///
/// A requirement does not contain timing. Timing comes from the scheduling
/// candidate.
///
/// Resource identity is canonical `ResourceId`; this type does not create a
/// communication-specific resource identity.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CommunicationRequirement {
    resource: ResourceId,
    kind: CommunicationKind,
    quantity: u128,
    policy: CommunicationResourcePolicy,
}

impl CommunicationRequirement {
    /// Creates a communication requirement.
    pub fn new(
        resource: ResourceId,
        kind: CommunicationKind,
        quantity: u128,
        policy: CommunicationResourcePolicy,
    ) -> CommunicationConstraintResult<Self> {
        if quantity == 0 {
            return Err(CommunicationConstraintError::ZeroQuantity {
                resource,
            });
        }

        if policy.is_unavailable() {
            return Err(CommunicationConstraintError::UnavailableResource {
                resource,
            });
        }

        if let Some(capacity) = policy.capacity() {
            if quantity > capacity {
                return Err(CommunicationConstraintError::QuantityExceedsCapacity {
                    resource,
                    requested: quantity,
                    capacity,
                });
            }
        }

        Ok(Self {
            resource,
            kind,
            quantity,
            policy,
        })
    }

    /// Creates a one-unit delegated communication requirement.
    pub fn delegated(
        resource: ResourceId,
        kind: CommunicationKind,
    ) -> CommunicationConstraintResult<Self> {
        Self::new(
            resource,
            kind,
            1,
            CommunicationResourcePolicy::delegated(),
        )
    }

    /// Creates an exclusive one-unit communication requirement.
    pub fn exclusive(
        resource: ResourceId,
        kind: CommunicationKind,
    ) -> CommunicationConstraintResult<Self> {
        Self::new(
            resource,
            kind,
            1,
            CommunicationResourcePolicy::exclusive(),
        )
    }

    /// Returns the canonical resource identity.
    #[must_use]
    pub const fn resource(&self) -> ResourceId {
        self.resource
    }

    /// Returns the communication semantic kind.
    #[must_use]
    pub fn kind(&self) -> &CommunicationKind {
        &self.kind
    }

    /// Returns requested capacity.
    #[must_use]
    pub const fn quantity(&self) -> u128 {
        self.quantity
    }

    /// Returns the resource policy.
    #[must_use]
    pub const fn policy(&self) -> CommunicationResourcePolicy {
        self.policy
    }
}

// ============================================================================
// Communication dependency
// ============================================================================

/// An operation that must complete before communication-dependent execution
/// may begin.
///
/// This is deliberately separate from the global dependency graph.
///
/// The graph remains owned by the scheduling IR/planner layer. This structure
/// is only the communication constraint's explicit requirement view.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CommunicationDependency {
    operation: OperationId,
}

impl CommunicationDependency {
    /// Creates a communication dependency.
    #[must_use]
    pub const fn new(operation: OperationId) -> Self {
        Self { operation }
    }

    /// Returns the prerequisite operation.
    #[must_use]
    pub const fn operation(self) -> OperationId {
        self.operation
    }
}

// ============================================================================
// Communication readiness
// ============================================================================

/// Minimum legal start time for an operation caused by communication
/// availability/readiness.
///
/// This is target/context data. It is not a hard-coded scheduler constant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CommunicationReadiness {
    operation: OperationId,
    earliest_start: TimePoint,
}

impl CommunicationReadiness {
    /// Creates a communication readiness requirement.
    #[must_use]
    pub const fn new(
        operation: OperationId,
        earliest_start: TimePoint,
    ) -> Self {
        Self {
            operation,
            earliest_start,
        }
    }

    /// Returns the operation to which the readiness requirement applies.
    #[must_use]
    pub const fn operation(self) -> OperationId {
        self.operation
    }

    /// Returns the earliest permitted start.
    #[must_use]
    pub const fn earliest_start(self) -> TimePoint {
        self.earliest_start
    }
}

// ============================================================================
// Communication requirement set
// ============================================================================

/// Complete communication requirements supplied to one constraint instance.
///
/// All collections are sparse:
///
/// only resources/operations with explicit communication requirements are
/// represented.
///
/// Therefore a target containing millions or billions of resources does not
/// require this object to allocate entries for unused resources.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommunicationRequirements {
    operation_resources:
        BTreeMap<OperationId, Vec<CommunicationRequirement>>,

    resource_policies:
        BTreeMap<ResourceId, CommunicationResourcePolicy>,

    dependencies:
        BTreeMap<OperationId, Vec<CommunicationDependency>>,

    readiness:
        BTreeMap<OperationId, CommunicationReadiness>,
}

impl CommunicationRequirements {
    /// Creates an empty communication requirement set.
    #[must_use]
    pub fn new() -> Self {
        Self {
            operation_resources: BTreeMap::new(),
            resource_policies: BTreeMap::new(),
            dependencies: BTreeMap::new(),
            readiness: BTreeMap::new(),
        }
    }

    /// Registers the policy for a communication resource.
    ///
    /// Re-registering the same resource replaces the previous policy.
    pub fn set_resource_policy(
        &mut self,
        resource: ResourceId,
        policy: CommunicationResourcePolicy,
    ) -> CommunicationConstraintResult<()> {
        if policy.is_unavailable() {
            self.resource_policies.insert(resource, policy);
            return Ok(());
        }

        self.resource_policies.insert(resource, policy);
        Ok(())
    }

    /// Registers one operation/resource communication requirement.
    pub fn add_requirement(
        &mut self,
        operation: OperationId,
        requirement: CommunicationRequirement,
    ) {
        self.operation_resources
            .entry(operation)
            .or_default()
            .push(requirement);
    }

    /// Registers a communication dependency.
    pub fn add_dependency(
        &mut self,
        operation: OperationId,
        dependency: CommunicationDependency,
    ) {
        self.dependencies
            .entry(operation)
            .or_default()
            .push(dependency);
    }

    /// Registers an earliest legal start time.
    pub fn set_readiness(
        &mut self,
        readiness: CommunicationReadiness,
    ) {
        self.readiness
            .insert(readiness.operation(), readiness);
    }

    /// Returns requirements for an operation.
    #[must_use]
    pub fn requirements_for(
        &self,
        operation: OperationId,
    ) -> &[CommunicationRequirement] {
        self.operation_resources
            .get(&operation)
            .map(Vec::as_slice)
            .unwrap_or(&[])
    }

    /// Returns communication dependencies for an operation.
    #[must_use]
    pub fn dependencies_for(
        &self,
        operation: OperationId,
    ) -> &[CommunicationDependency] {
        self.dependencies
            .get(&operation)
            .map(Vec::as_slice)
            .unwrap_or(&[])
    }

    /// Returns readiness information for an operation.
    #[must_use]
    pub fn readiness_for(
        &self,
        operation: OperationId,
    ) -> Option<CommunicationReadiness> {
        self.readiness.get(&operation).copied()
    }

    /// Returns the configured policy for a resource.
    #[must_use]
    pub fn resource_policy(
        &self,
        resource: ResourceId,
    ) -> Option<CommunicationResourcePolicy> {
        self.resource_policies.get(&resource).copied()
    }

    /// Returns whether the operation has an explicit communication
    /// requirement.
    #[must_use]
    pub fn contains_operation(
        &self,
        operation: OperationId,
    ) -> bool {
        self.operation_resources.contains_key(&operation)
            || self.dependencies.contains_key(&operation)
            || self.readiness.contains_key(&operation)
    }

    /// Returns whether a resource is registered as a communication resource.
    #[must_use]
    pub fn contains_resource(
        &self,
        resource: ResourceId,
    ) -> bool {
        self.resource_policies.contains_key(&resource)
    }

    /// Returns whether no communication requirements have been configured.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.operation_resources.is_empty()
            && self.resource_policies.is_empty()
            && self.dependencies.is_empty()
            && self.readiness.is_empty()
    }
}

impl Default for CommunicationRequirements {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Communication conflict
// ============================================================================

/// Classification of a communication scheduling failure.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CommunicationConflictKind {
    /// A communication resource is unavailable.
    ResourceUnavailable,

    /// A communication resource is already occupied.
    ResourceOverlap,

    /// Combined resource quantity exceeds configured capacity.
    CapacityExceeded,

    /// An explicit communication prerequisite has not completed.
    DependencyIncomplete,

    /// Communication readiness has not been reached.
    NotReady,

    /// The candidate did not claim a required communication resource.
    MissingResourceClaim,

    /// Candidate claims a resource that is configured as unavailable.
    UnavailableResourceClaim,

    /// A requirement exceeds the configured resource capacity.
    RequirementExceedsCapacity,

    /// Arithmetic overflow prevented safe interval evaluation.
    TimeOverflow,
}

impl fmt::Display for CommunicationConflictKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ResourceUnavailable => {
                formatter.write_str("resource-unavailable")
            }
            Self::ResourceOverlap => {
                formatter.write_str("resource-overlap")
            }
            Self::CapacityExceeded => {
                formatter.write_str("capacity-exceeded")
            }
            Self::DependencyIncomplete => {
                formatter.write_str("dependency-incomplete")
            }
            Self::NotReady => formatter.write_str("not-ready"),
            Self::MissingResourceClaim => {
                formatter.write_str("missing-resource-claim")
            }
            Self::UnavailableResourceClaim => {
                formatter.write_str("unavailable-resource-claim")
            }
            Self::RequirementExceedsCapacity => {
                formatter.write_str("requirement-exceeds-capacity")
            }
            Self::TimeOverflow => formatter.write_str("time-overflow"),
        }
    }
}

// ============================================================================
// Construction errors
// ============================================================================

/// Errors produced while constructing communication constraints.
///
/// These are configuration errors, not scheduling violations.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CommunicationConstraintError {
    /// A requirement requested zero capacity.
    ZeroQuantity {
        /// Resource involved.
        resource: ResourceId,
    },

    /// A configured capacity is zero.
    ZeroCapacity,

    /// A resource is explicitly unavailable.
    UnavailableResource {
        /// Resource involved.
        resource: ResourceId,
    },

    /// Requirement exceeds configured capacity.
    QuantityExceedsCapacity {
        /// Resource involved.
        resource: ResourceId,

        /// Requested quantity.
        requested: u128,

        /// Configured capacity.
        capacity: u128,
    },

    /// A readiness time was invalid in a context where it cannot be used.
    InvalidReadiness {
        /// Operation involved.
        operation: OperationId,
    },
}

impl fmt::Display for CommunicationConstraintError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ZeroQuantity { resource } => {
                write!(
                    formatter,
                    "communication requirement for {resource:?} \
                     has zero quantity"
                )
            }

            Self::ZeroCapacity => {
                formatter.write_str(
                    "communication resource capacity cannot be zero; \
                     use unavailable() for an explicitly unavailable resource",
                )
            }

            Self::UnavailableResource { resource } => {
                write!(
                    formatter,
                    "communication resource {resource:?} is unavailable"
                )
            }

            Self::QuantityExceedsCapacity {
                resource,
                requested,
                capacity,
            } => {
                write!(
                    formatter,
                    "communication requirement for {resource:?} \
                     requests {requested} units but capacity is {capacity}"
                )
            }

            Self::InvalidReadiness { operation } => {
                write!(
                    formatter,
                    "invalid communication readiness for operation {operation:?}"
                )
            }
        }
    }
}

impl std::error::Error for CommunicationConstraintError {}

// ============================================================================
// Communication constraint
// ============================================================================

/// Production communication scheduling constraint.
///
/// The constraint is immutable after construction from the scheduler's point
/// of view. Build the requirement set first, then construct this object.
///
/// It is safe to share between scheduler threads because it owns no mutable
/// state and performs no external I/O.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommunicationConstraint {
    id: ConstraintId,
    name: String,
    requirements: CommunicationRequirements,
    severity: ConstraintSeverity,

    /// Whether an operation with an explicit communication requirement must
    /// claim every configured communication resource associated with it.
    require_explicit_resource_claims: bool,

    /// Whether communication dependencies must be recorded as completed in
    /// `ConstraintState`.
    require_completed_dependencies: bool,
}

impl CommunicationConstraint {
    /// Creates a communication constraint with strict resource-claim
    /// validation.
    #[must_use]
    pub fn new(
        id: ConstraintId,
        requirements: CommunicationRequirements,
    ) -> Self {
        Self {
            id,
            name: String::from("communication"),
            requirements,
            severity: ConstraintSeverity::Error,
            require_explicit_resource_claims: true,
            require_completed_dependencies: true,
        }
    }

    /// Creates a permissive communication constraint.
    ///
    /// Permissive mode is useful for analysis where communication requirements
    /// are supplied by another constraint/resource layer.
    #[must_use]
    pub fn permissive(
        id: ConstraintId,
        requirements: CommunicationRequirements,
    ) -> Self {
        Self {
            id,
            name: String::from("communication"),
            requirements,
            severity: ConstraintSeverity::Error,
            require_explicit_resource_claims: false,
            require_completed_dependencies: true,
        }
    }

    /// Changes the diagnostic name.
    #[must_use]
    pub fn with_name(
        mut self,
        name: impl Into<String>,
    ) -> Self {
        self.name = name.into();
        self
    }

    /// Changes violation severity.
    #[must_use]
    pub const fn with_severity(
        mut self,
        severity: ConstraintSeverity,
    ) -> Self {
        self.severity = severity;
        self
    }

    /// Enables or disables strict communication resource-claim checking.
    #[must_use]
    pub const fn require_explicit_resource_claims(
        mut self,
        required: bool,
    ) -> Self {
        self.require_explicit_resource_claims = required;
        self
    }

    /// Enables or disables explicit dependency-completion checking.
    #[must_use]
    pub const fn require_completed_dependencies(
        mut self,
        required: bool,
    ) -> Self {
        self.require_completed_dependencies = required;
        self
    }

    /// Returns the configured communication requirements.
    #[must_use]
    pub const fn requirements(
        &self,
    ) -> &CommunicationRequirements {
        &self.requirements
    }

    /// Determines whether two intervals overlap.
    ///
    /// Half-open interval semantics are used:
    ///
    /// `[start, end)`
    ///
    /// A zero-duration candidate therefore has no positive-duration overlap.
    #[must_use]
    fn intervals_overlap(
        left_start: TimePoint,
        left_end: TimePoint,
        right_start: TimePoint,
        right_end: TimePoint,
    ) -> bool {
        left_start < right_end && right_start < left_end
    }

    /// Safely calculates the candidate end time.
    fn candidate_end(
        context: &ConstraintContext<'_>,
    ) -> Result<TimePoint, CommunicationViolationData> {
        context
            .candidate()
            .checked_end()
            .ok_or(CommunicationViolationData::TimeOverflow)
    }

    /// Returns whether the candidate claims a resource.
    #[must_use]
    fn claims_resource(
        context: &ConstraintContext<'_>,
        resource: ResourceId,
    ) -> bool {
        context
            .candidate()
            .resource_claims()
            .iter()
            .any(|claim| claim.resource() == resource)
    }

    /// Returns the quantity claimed by the candidate for a resource.
    #[must_use]
    fn claimed_quantity(
        context: &ConstraintContext<'_>,
        resource: ResourceId,
    ) -> u128 {
        context
            .candidate()
            .resource_claims()
            .iter()
            .filter(|claim| claim.resource() == resource)
            .fold(0_u128, |total, claim| {
                total.saturating_add(claim.quantity())
            })
    }

    /// Returns the total quantity already reserved for a resource over the
    /// candidate interval.
    ///
    /// Only overlapping reservations contribute.
    fn reserved_quantity(
        context: &ConstraintContext<'_>,
        resource: ResourceId,
        candidate_start: TimePoint,
        candidate_end: TimePoint,
    ) -> Result<u128, CommunicationViolationData> {
        let mut total = 0_u128;

        for reservation in context.state().reservations() {
            if reservation.resource() != resource {
                continue;
            }

            let reservation_end = reservation
                .checked_end()
                .ok_or(CommunicationViolationData::TimeOverflow)?;

            if !Self::intervals_overlap(
                candidate_start,
                candidate_end,
                reservation.start(),
                reservation_end,
            ) {
                continue;
            }

            total = total
                .checked_add(reservation.quantity())
                .ok_or(CommunicationViolationData::TimeOverflow)?;
        }

        Ok(total)
    }

    /// Converts an internal communication violation into the canonical
    /// scheduler constraint violation.
    fn violation(
        &self,
        context: &ConstraintContext<'_>,
        kind: CommunicationConflictKind,
        reason: impl Into<String>,
        resource: Option<ResourceId>,
    ) -> ConstraintViolation {
        let mut violation = ConstraintViolation::new(
            self.id,
            ConstraintKind::Communication,
            self.severity,
            format!("{}: {}", kind, reason.into()),
        )
        .with_operation(context.candidate().operation())
        .with_timing(
            context.candidate().start(),
            context.candidate().duration(),
        );

        if let Some(resource) = resource {
            violation = violation.with_resource(resource);
        }

        violation
    }

    /// Checks communication readiness.
    fn check_readiness(
        &self,
        context: &ConstraintContext<'_>,
    ) -> Result<(), ConstraintViolation> {
        let operation = context.candidate().operation();

        let Some(readiness) =
            self.requirements.readiness_for(operation)
        else {
            return Ok(());
        };

        if context.candidate().start() < readiness.earliest_start() {
            return Err(self.violation(
                context,
                CommunicationConflictKind::NotReady,
                format!(
                    "operation cannot start at {}; communication readiness \
                     begins at {}",
                    context.candidate().start(),
                    readiness.earliest_start(),
                ),
                None,
            ));
        }

        Ok(())
    }

    /// Checks explicit communication dependencies.
    fn check_dependencies(
        &self,
        context: &ConstraintContext<'_>,
    ) -> Result<(), ConstraintViolation> {
        if !self.require_completed_dependencies {
            return Ok(());
        }

        let operation = context.candidate().operation();

        for dependency in self.requirements.dependencies_for(operation) {
            if !context
                .state()
                .is_operation_completed(dependency.operation())
            {
                return Err(self.violation(
                    context,
                    CommunicationConflictKind::DependencyIncomplete,
                    format!(
                        "communication prerequisite operation {:?} has not \
                         completed",
                        dependency.operation(),
                    ),
                    None,
                ));
            }
        }

        Ok(())
    }

    /// Checks one configured communication requirement.
    fn check_requirement(
        &self,
        context: &ConstraintContext<'_>,
        requirement: &CommunicationRequirement,
        candidate_end: TimePoint,
    ) -> Result<(), ConstraintViolation> {
        let resource = requirement.resource();

        if let Some(policy) =
            self.requirements.resource_policy(resource)
        {
            if policy.is_unavailable() {
                return Err(self.violation(
                    context,
                    CommunicationConflictKind::ResourceUnavailable,
                    format!(
                        "communication resource {resource:?} is explicitly \
                         unavailable"
                    ),
                    Some(resource),
                ));
            }

            if let Some(capacity) = policy.capacity() {
                if requirement.quantity() > capacity {
                    return Err(self.violation(
                        context,
                        CommunicationConflictKind::RequirementExceedsCapacity,
                        format!(
                            "communication requirement requests {} units \
                             from resource {resource:?}, whose configured \
                             capacity is {capacity}",
                            requirement.quantity(),
                        ),
                        Some(resource),
                    ));
                }
            }
        }

        if !Self::claims_resource(context, resource) {
            if self.require_explicit_resource_claims {
                return Err(self.violation(
                    context,
                    CommunicationConflictKind::MissingResourceClaim,
                    format!(
                        "operation declares communication use of resource \
                         {resource:?} but the scheduling candidate does not \
                         claim that resource"
                    ),
                    Some(resource),
                ));
            }

            return Ok(());
        }

        let candidate_quantity =
            Self::claimed_quantity(context, resource);

        if candidate_quantity < requirement.quantity() {
            return Err(self.violation(
                context,
                CommunicationConflictKind::MissingResourceClaim,
                format!(
                    "operation claims {candidate_quantity} units of \
                     communication resource {resource:?}, but \
                     {} units are required",
                    requirement.quantity(),
                ),
                Some(resource),
            ));
        }

        let policy = self
            .requirements
            .resource_policy(resource)
            .unwrap_or(requirement.policy());

        if policy.is_exclusive() {
            let reserved = Self::reserved_quantity(
                context,
                resource,
                context.candidate().start(),
                candidate_end,
            )
            .map_err(|data| {
                self.violation(
                    context,
                    data.kind(),
                    data.reason(),
                    Some(resource),
                )
            })?;

            if reserved > 0 {
                return Err(self.violation(
                    context,
                    CommunicationConflictKind::ResourceOverlap,
                    format!(
                        "exclusive communication resource {resource:?} \
                         is already reserved during the candidate interval"
                    ),
                    Some(resource),
                ));
            }

            return Ok(());
        }

        if let Some(capacity) = policy.capacity() {
            let reserved = Self::reserved_quantity(
                context,
                resource,
                context.candidate().start(),
                candidate_end,
            )
            .map_err(|data| {
                self.violation(
                    context,
                    data.kind(),
                    data.reason(),
                    Some(resource),
                )
            })?;

            let total = reserved
                .checked_add(candidate_quantity)
                .ok_or_else(|| {
                    self.violation(
                        context,
                        CommunicationConflictKind::TimeOverflow,
                        "communication resource quantity overflow",
                        Some(resource),
                    )
                })?;

            if total > capacity {
                return Err(self.violation(
                    context,
                    CommunicationConflictKind::CapacityExceeded,
                    format!(
                        "communication resource {resource:?} would require \
                         {total} units during the candidate interval, \
                         exceeding capacity {capacity}"
                    ),
                    Some(resource),
                ));
            }
        }

        Ok(())
    }

    /// Performs the complete communication evaluation.
    fn evaluate_inner(
        &self,
        context: &ConstraintContext<'_>,
    ) -> Result<(), ConstraintViolation> {
        let candidate_end =
            Self::candidate_end(context).map_err(|data| {
                self.violation(
                    context,
                    data.kind(),
                    data.reason(),
                    None,
                )
            })?;

        self.check_readiness(context)?;
        self.check_dependencies(context)?;

        let operation = context.candidate().operation();

        for requirement in self.requirements.requirements_for(operation) {
            self.check_requirement(
                context,
                requirement,
                candidate_end,
            )?;
        }

        Ok(())
    }
}

impl Constraint for CommunicationConstraint {
    fn id(&self) -> ConstraintId {
        self.id
    }

    fn kind(&self) -> ConstraintKind {
        ConstraintKind::Communication
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn severity(&self) -> ConstraintSeverity {
        self.severity
    }

    fn applies(
        &self,
        context: &ConstraintContext<'_>,
    ) -> ConstraintApplicability {
        let operation = context.candidate().operation();

        if self.requirements.contains_operation(operation) {
            return ConstraintApplicability::Applicable;
        }

        for claim in context.candidate().resource_claims() {
            if self
                .requirements
                .contains_resource(claim.resource())
            {
                return ConstraintApplicability::Applicable;
            }
        }

        ConstraintApplicability::NotApplicable
    }

    fn evaluate(
        &self,
        context: &ConstraintContext<'_>,
    ) -> Result<(), ConstraintViolation> {
        self.evaluate_inner(context)
    }
}

// ============================================================================
// Internal violation data
// ============================================================================

/// Internal representation of a communication constraint failure.
#[derive(Debug, Clone, PartialEq, Eq)]
enum CommunicationViolationData {
    TimeOverflow,
}

impl CommunicationViolationData {
    /// Converts the internal category to the public conflict kind.
    #[must_use]
    const fn kind(&self) -> CommunicationConflictKind {
        match self {
            Self::TimeOverflow => CommunicationConflictKind::TimeOverflow,
        }
    }

    /// Returns a stable explanation.
    #[must_use]
    const fn reason(&self) -> &'static str {
        match self {
            Self::TimeOverflow => {
                "checked time arithmetic overflowed while evaluating \
                 communication occupancy"
            }
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::quantum::ir::core::identity::{
        OperationId,
        ResourceId,
    };

    use super::super::constraint::{
        ConstraintResourceClaim,
        ConstraintState,
        SchedulingCandidate,
    };

    use super::super::super::types::{
        Duration,
    };

    fn operation(value: u64) -> OperationId {
        OperationId::new(value)
    }

    fn resource(value: u64) -> ResourceId {
        ResourceId::new(value)
    }

    fn candidate<'a>(
        operation_id: OperationId,
        claims: &'a [ConstraintResourceClaim],
        start: TimePoint,
        duration: Duration,
    ) -> SchedulingCandidate<'a> {
        SchedulingCandidate::new(
            operation_id,
            &[],
            &[],
            claims,
            start,
            duration,
        )
    }

    fn state<'a>(
        reservations: &'a [ConstraintReservationView],
    ) -> ConstraintState<'a> {
        ConstraintState::new(reservations, &[], &[])
    }

    #[test]
    fn delegated_requirement_can_be_constructed() {
        let requirement = CommunicationRequirement::delegated(
            resource(1),
            CommunicationKind::Quantum,
        )
        .expect("delegated requirement should be valid");

        assert_eq!(requirement.resource(), resource(1));
        assert_eq!(requirement.quantity(), 1);
    }

    #[test]
    fn exclusive_requirement_is_exclusive() {
        let requirement = CommunicationRequirement::exclusive(
            resource(2),
            CommunicationKind::Classical,
        )
        .expect("exclusive requirement should be valid");

        assert!(requirement.policy().is_exclusive());
    }

    #[test]
    fn readiness_blocks_early_operation() {
        let mut requirements = CommunicationRequirements::new();

        requirements.set_readiness(
            CommunicationReadiness::new(operation(2), TimePoint::new(10)),
        );

        let constraint =
            CommunicationConstraint::new(ConstraintId::new(1), requirements);

        let claims = [ConstraintResourceClaim::new(resource(1), 1)];

        let candidate = candidate(
            operation(2),
            &claims,
            TimePoint::new(5),
            Duration::new(1),
        );

        let state = state(&[]);

        let context = ConstraintContext::new(
            &candidate,
            &state,
            ConstraintPhase::Planning,
        );

        assert!(constraint.evaluate(&context).is_err());
    }

    #[test]
    fn completed_dependency_allows_operation() {
        let mut requirements = CommunicationRequirements::new();

        requirements.add_dependency(
            operation(2),
            CommunicationDependency::new(operation(1)),
        );

        let constraint =
            CommunicationConstraint::new(ConstraintId::new(2), requirements);

        let claims = [ConstraintResourceClaim::new(resource(1), 1)];

        let candidate = candidate(
            operation(2),
            &claims,
            TimePoint::new(10),
            Duration::new(1),
        );

        let completed = [operation(1)];

        let reservations: [ConstraintReservationView; 0] = [];

        let state = ConstraintState::new(
            &reservations,
            &completed,
            &[],
        );

        let context = ConstraintContext::new(
            &candidate,
            &state,
            ConstraintPhase::Planning,
        );

        assert!(constraint.evaluate(&context).is_ok());
    }

    #[test]
    fn incomplete_dependency_blocks_operation() {
        let mut requirements = CommunicationRequirements::new();

        requirements.add_dependency(
            operation(2),
            CommunicationDependency::new(operation(1)),
        );

        let constraint =
            CommunicationConstraint::new(ConstraintId::new(3), requirements);

        let claims = [ConstraintResourceClaim::new(resource(1), 1)];

        let candidate = candidate(
            operation(2),
            &claims,
            TimePoint::new(10),
            Duration::new(1),
        );

        let state = state(&[]);

        let context = ConstraintContext::new(
            &candidate,
            &state,
            ConstraintPhase::Planning,
        );

        assert!(constraint.evaluate(&context).is_err());
    }
}