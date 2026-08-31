//! Zamani Quantum IR — Schedule Representation
//!
//! Production-grade, hardware-independent representation of a scheduled
//! quantum program.
//!
//! # Architectural role
//!
//! `schedule.rs` represents the temporal result of scheduling without owning
//! the scheduling algorithm itself.
//!
//! The distinction is:
//
//! ```text
//! quantum::ir
//!     │
//!     └── schedule.rs
//!           = represents WHEN operations occur
//!
//! quantum::scheduling
//!     = decides WHEN operations should occur
//!
//! quantum::hardware
//!     = describes which timing/resources a real machine supports
//!
//! backend
//!     = converts the schedule into executable machine instructions
//! ```
//!
//! This module therefore does NOT:
//!
//! - choose a routing strategy;
//! - choose a physical-qubit placement;
//! - discover hardware topology;
//! - perform calibration;
//! - generate pulses;
//! - optimize circuits;
//! - communicate with a QPU;
//! - execute a program;
//! - simulate quantum state;
//! - own hardware-specific timing constraints.
//!
//! # Universal-program principle
//!
//! A Zamani program is written once and may ultimately target systems ranging
//! from a single quantum resource to very large distributed or fault-tolerant
//! systems.
//!
//! This schedule representation therefore has no architectural fixed
//! operation-count, qubit-count, channel-count, or schedule-size ceiling.
//!
//! Concrete limits come from `QuantumIrLimits`.
//!
//! Hardware capacity comes from `quantum::hardware`.
//!
//! Consequently:
//
//! ```text
//! 1 qubit
//! 2 qubits
//! 63 qubits
//! 64 qubits
//! 128 qubits
//! 4,096 qubits
//! 1,000,000 qubits
//! N finite qubits
//! ```
//!
//! are all represented by the same semantic model.
//!
//! `usize::MAX` / `u128::MAX` are implementation-domain bounds, not promises
//! that a machine can physically contain or execute unlimited resources.
//!
//! # Time representation
//!
//! Schedule time is represented using unsigned attoseconds (`10^-18`
//! seconds) as the canonical lossless IR storage unit.
//!
//! This provides a single integer representation that is:
//!
//! - deterministic;
//! - platform-independent;
//! - lossless for common quantum-control time scales;
//! - free from floating-point rounding;
//! - safely checked for overflow;
//! - sufficiently wide for extremely long schedules.
//!
//! The future `timing.rs` module can provide richer source-level time units
//! (`fs`, `ps`, `ns`, `us`, `ms`, `s`) by converting to these canonical
//! schedule values.
//!
//! `schedule.rs` intentionally does not depend on `timing.rs`, allowing this
//! file to be completed and frozen before that module exists.
//!
//! # Qubit identity
//!
//! Logical and physical qubit identities come from the canonical IR module:
//
//! ```text
//! quantum::ir::qubit::QubitId
//! quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! The schedule does not perform logical-to-physical routing.
//!
//! A scheduled operation may optionally record the physical placement that a
//! downstream routing stage selected.
//!
//! # Channel and frame identity
//!
//! Abstract channels and frames are represented through the stable identities
//! from `identity.rs`:
//
//! ```text
//! ChannelId
//! FrameId
//! ```
//!
//! These are semantic IR identities, not physical DAC/channel numbers or
//! hardware oscillator identifiers.
//!
//! Future `channel.rs` and `frame.rs` modules can build richer types around
//! these stable identities without changing this schedule contract.
//!
//! # Operation identity
//!
//! Operations are identified using `OperationId` from `identity.rs`.
//!
//! Operation identity is independent of schedule position.
//!
//! Moving an operation in time must not inherently change its identity.
//!
//! # Scheduling semantics
//!
//! A schedule contains:
//!
//! - a stable schedule identity;
//! - the IR version;
//! - scheduled operations;
//! - optional dependencies;
//! - optional synchronization points;
//! - optional resource occupancy;
//! - total schedule duration;
//! - metadata-free deterministic structure.
//!
//! A schedule does not require operations to be physically executable.
//! Hardware compatibility is checked downstream.
//!
//! # Determinism
//!
//! Schedule construction and canonical ordering must be deterministic.
//!
//! Operations are ordered by:
//
//! 1. start time;
//! 2. end time;
//! 3. operation identity;
//! 4. deterministic insertion-independent tie breaking.
//!
//! No hash-map iteration order is used to define schedule semantics.
//!
//! # Safety
//!
//! This file contains no `unsafe` code.
//!
//! `#![forbid(unsafe_code)]` makes that requirement compiler-enforced.
//!
//! All time arithmetic uses checked operations.
//!
//! No implicit integer wrapping is permitted.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021.
//!
//! No nightly features.
//! No external dependencies.

#![forbid(unsafe_code)]

use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use super::identity::{
    ChannelId,
    FrameId,
    IrVersion,
    OperationId,
    ScheduleId,
};
use super::limits::{LimitsError, QuantumIrLimits};
use super::qubit::{PhysicalQubitId, QubitId};

// =============================================================================
// Canonical time
// =============================================================================

/// Number of attoseconds in one femtosecond.
pub const ATTOSECONDS_PER_FEMTOSECOND: u128 = 1_000;

/// Number of attoseconds in one picosecond.
pub const ATTOSECONDS_PER_PICOSECOND: u128 = 1_000_000;

/// Number of attoseconds in one nanosecond.
pub const ATTOSECONDS_PER_NANOSECOND: u128 = 1_000_000_000;

/// Number of attoseconds in one microsecond.
pub const ATTOSECONDS_PER_MICROSECOND: u128 = 1_000_000_000_000;

/// Number of attoseconds in one millisecond.
pub const ATTOSECONDS_PER_MILLISECOND: u128 = 1_000_000_000_000_000;

/// Number of attoseconds in one second.
pub const ATTOSECONDS_PER_SECOND: u128 = 1_000_000_000_000_000_000;

/// Canonical hardware-independent schedule time.
///
/// The internal representation is an integer number of attoseconds.
///
/// This is deliberately not a floating-point duration. Floating-point time
/// would make schedule equality, hashing, ordering, and reproducibility
/// unnecessarily fragile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ScheduleTime(u128);

impl ScheduleTime {
    /// The zero-time origin.
    pub const ZERO: Self = Self(0);

    /// Creates a schedule time directly from attoseconds.
    ///
    /// The value is interpreted as an absolute or relative time according to
    /// the API using it.
    #[must_use]
    pub const fn from_attoseconds(attoseconds: u128) -> Self {
        Self(attoseconds)
    }

    /// Creates a schedule time from femtoseconds.
    ///
    /// Returns `None` if conversion would overflow `u128`.
    #[must_use]
    pub const fn from_femtoseconds(femtoseconds: u128) -> Option<Self> {
        match femtoseconds.checked_mul(ATTOSECONDS_PER_FEMTOSECOND) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }

    /// Creates a schedule time from picoseconds.
    #[must_use]
    pub const fn from_picoseconds(picoseconds: u128) -> Option<Self> {
        match picoseconds.checked_mul(ATTOSECONDS_PER_PICOSECOND) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }

    /// Creates a schedule time from nanoseconds.
    #[must_use]
    pub const fn from_nanoseconds(nanoseconds: u128) -> Option<Self> {
        match nanoseconds.checked_mul(ATTOSECONDS_PER_NANOSECOND) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }

    /// Creates a schedule time from microseconds.
    #[must_use]
    pub const fn from_microseconds(microseconds: u128) -> Option<Self> {
        match microseconds.checked_mul(ATTOSECONDS_PER_MICROSECOND) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }

    /// Creates a schedule time from milliseconds.
    #[must_use]
    pub const fn from_milliseconds(milliseconds: u128) -> Option<Self> {
        match milliseconds.checked_mul(ATTOSECONDS_PER_MILLISECOND) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }

    /// Creates a schedule time from seconds.
    #[must_use]
    pub const fn from_seconds(seconds: u128) -> Option<Self> {
        match seconds.checked_mul(ATTOSECONDS_PER_SECOND) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }

    /// Returns the raw attosecond value.
    #[must_use]
    pub const fn attoseconds(self) -> u128 {
        self.0
    }

    /// Returns whether the time is zero.
    #[must_use]
    pub const fn is_zero(self) -> bool {
        self.0 == 0
    }

    /// Adds a duration represented in attoseconds.
    ///
    /// Overflow is reported instead of wrapping.
    pub const fn checked_add_attoseconds(
        self,
        duration: u128,
    ) -> Option<Self> {
        match self.0.checked_add(duration) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }

    /// Adds another canonical schedule time.
    ///
    /// This operation is primarily useful for relative time calculations.
    pub const fn checked_add(self, other: Self) -> Option<Self> {
        match self.0.checked_add(other.0) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }

    /// Subtracts another schedule time.
    #[must_use]
    pub const fn checked_sub(self, other: Self) -> Option<Self> {
        match self.0.checked_sub(other.0) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }
}

impl Default for ScheduleTime {
    fn default() -> Self {
        Self::ZERO
    }
}

impl fmt::Display for ScheduleTime {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}as", self.0)
    }
}

// =============================================================================
// Schedule duration
// =============================================================================

/// Non-negative duration represented in canonical attoseconds.
///
/// A separate type prevents accidental confusion between an absolute time
/// point and an elapsed duration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ScheduleDuration(u128);

impl ScheduleDuration {
    /// Zero duration.
    pub const ZERO: Self = Self(0);

    /// Creates a duration from attoseconds.
    #[must_use]
    pub const fn from_attoseconds(attoseconds: u128) -> Self {
        Self(attoseconds)
    }

    /// Creates a duration from femtoseconds.
    #[must_use]
    pub const fn from_femtoseconds(femtoseconds: u128) -> Option<Self> {
        match femtoseconds.checked_mul(ATTOSECONDS_PER_FEMTOSECOND) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }

    /// Creates a duration from picoseconds.
    #[must_use]
    pub const fn from_picoseconds(picoseconds: u128) -> Option<Self> {
        match picoseconds.checked_mul(ATTOSECONDS_PER_PICOSECOND) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }

    /// Creates a duration from nanoseconds.
    #[must_use]
    pub const fn from_nanoseconds(nanoseconds: u128) -> Option<Self> {
        match nanoseconds.checked_mul(ATTOSECONDS_PER_NANOSECOND) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }

    /// Creates a duration from microseconds.
    #[must_use]
    pub const fn from_microseconds(microseconds: u128) -> Option<Self> {
        match microseconds.checked_mul(ATTOSECONDS_PER_MICROSECOND) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }

    /// Creates a duration from milliseconds.
    #[must_use]
    pub const fn from_milliseconds(milliseconds: u128) -> Option<Self> {
        match milliseconds.checked_mul(ATTOSECONDS_PER_MILLISECOND) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }

    /// Creates a duration from seconds.
    #[must_use]
    pub const fn from_seconds(seconds: u128) -> Option<Self> {
        match seconds.checked_mul(ATTOSECONDS_PER_SECOND) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }

    /// Returns the duration in attoseconds.
    #[must_use]
    pub const fn attoseconds(self) -> u128 {
        self.0
    }

    /// Returns whether the duration is zero.
    #[must_use]
    pub const fn is_zero(self) -> bool {
        self.0 == 0
    }
}

impl Default for ScheduleDuration {
    fn default() -> Self {
        Self::ZERO
    }
}

impl fmt::Display for ScheduleDuration {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}as", self.0)
    }
}

// =============================================================================
// Scheduling resource references
// =============================================================================

/// Abstract resource occupied by a scheduled operation.
///
/// These references are deliberately semantic. They do not describe actual
/// hardware implementation details.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ScheduleResource {
    /// Logical qubit.
    LogicalQubit(QubitId),

    /// Physical qubit selected by a downstream routing stage.
    PhysicalQubit(PhysicalQubitId),

    /// Abstract control/acquisition channel.
    Channel(ChannelId),

    /// Abstract control frame.
    Frame(FrameId),
}

impl ScheduleResource {
    /// Returns the resource's deterministic identity class.
    #[must_use]
    pub const fn kind(self) -> ScheduleResourceKind {
        match self {
            Self::LogicalQubit(_) => ScheduleResourceKind::LogicalQubit,
            Self::PhysicalQubit(_) => ScheduleResourceKind::PhysicalQubit,
            Self::Channel(_) => ScheduleResourceKind::Channel,
            Self::Frame(_) => ScheduleResourceKind::Frame,
        }
    }
}

/// Resource category used for deterministic diagnostics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ScheduleResourceKind {
    /// Logical qubit.
    LogicalQubit,

    /// Physical qubit.
    PhysicalQubit,

    /// Abstract channel.
    Channel,

    /// Abstract frame.
    Frame,
}

impl fmt::Display for ScheduleResourceKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LogicalQubit => formatter.write_str("logical qubit"),
            Self::PhysicalQubit => formatter.write_str("physical qubit"),
            Self::Channel => formatter.write_str("channel"),
            Self::Frame => formatter.write_str("frame"),
        }
    }
}

// =============================================================================
// Scheduled operation
// =============================================================================

/// A single temporally placed IR operation.
///
/// `ScheduledOperation` does not own the actual quantum operation semantics.
/// It refers to the canonical `OperationId` so that scheduling remains a
/// transformation of existing IR operations rather than a second operation
/// representation.
///
/// The operation's actual gate/pulse/measurement/control-flow semantics remain
/// owned by their respective IR modules.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ScheduledOperation {
    operation_id: OperationId,
    start: ScheduleTime,
    duration: ScheduleDuration,
    resources: Vec<ScheduleResource>,
}

impl ScheduledOperation {
    /// Creates a scheduled operation.
    ///
    /// The caller is responsible for supplying a unique `OperationId` within
    /// the containing IR program.
    ///
    /// No hardware compatibility is inferred here.
    pub fn new(
        operation_id: OperationId,
        start: ScheduleTime,
        duration: ScheduleDuration,
    ) -> Self {
        Self {
            operation_id,
            start,
            duration,
            resources: Vec::new(),
        }
    }

    /// Returns the referenced IR operation.
    #[must_use]
    pub const fn operation_id(&self) -> OperationId {
        self.operation_id
    }

    /// Returns the operation start time.
    #[must_use]
    pub const fn start(&self) -> ScheduleTime {
        self.start
    }

    /// Returns the operation duration.
    #[must_use]
    pub const fn duration(&self) -> ScheduleDuration {
        self.duration
    }

    /// Returns the operation end time.
    ///
    /// Returns `None` on `u128` overflow.
    #[must_use]
    pub fn end(&self) -> Option<ScheduleTime> {
        self.start
            .checked_add_attoseconds(self.duration.attoseconds())
    }

    /// Adds a logical-qubit resource reference.
    ///
    /// Duplicate resource references are ignored.
    pub fn with_logical_qubit(
        mut self,
        qubit: QubitId,
    ) -> Self {
        self.add_resource(ScheduleResource::LogicalQubit(qubit));
        self
    }

    /// Adds a physical-qubit resource reference.
    ///
    /// Duplicate resource references are ignored.
    pub fn with_physical_qubit(
        mut self,
        qubit: PhysicalQubitId,
    ) -> Self {
        self.add_resource(ScheduleResource::PhysicalQubit(qubit));
        self
    }

    /// Adds an abstract channel reference.
    pub fn with_channel(
        mut self,
        channel: ChannelId,
    ) -> Self {
        self.add_resource(ScheduleResource::Channel(channel));
        self
    }

    /// Adds an abstract frame reference.
    pub fn with_frame(
        mut self,
        frame: FrameId,
    ) -> Self {
        self.add_resource(ScheduleResource::Frame(frame));
        self
    }

    /// Adds an arbitrary semantic resource.
    pub fn with_resource(
        mut self,
        resource: ScheduleResource,
    ) -> Self {
        self.add_resource(resource);
        self
    }

    /// Adds a resource reference while preserving deterministic uniqueness.
    pub fn add_resource(
        &mut self,
        resource: ScheduleResource,
    ) {
        if !self.resources.contains(&resource) {
            self.resources.push(resource);
            self.resources.sort();
        }
    }

    /// Returns all resources occupied by the operation.
    #[must_use]
    pub fn resources(&self) -> &[ScheduleResource] {
        &self.resources
    }

    /// Returns whether this operation references a resource.
    #[must_use]
    pub fn uses_resource(
        &self,
        resource: ScheduleResource,
    ) -> bool {
        self.resources.contains(&resource)
    }

    /// Returns whether this operation has a non-zero duration.
    #[must_use]
    pub const fn is_non_zero_duration(&self) -> bool {
        !self.duration.is_zero()
    }
}

// =============================================================================
// Synchronization points
// =============================================================================

/// A semantic synchronization point in a schedule.
///
/// Synchronization is represented without assuming any specific hardware
/// barrier implementation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SynchronizationPoint {
    id: u64,
    time: ScheduleTime,
}

impl SynchronizationPoint {
    /// Creates a synchronization point.
    #[must_use]
    pub const fn new(
        id: u64,
        time: ScheduleTime,
    ) -> Self {
        Self { id, time }
    }

    /// Returns the synchronization identity.
    #[must_use]
    pub const fn id(self) -> u64 {
        self.id
    }

    /// Returns the synchronization time.
    #[must_use]
    pub const fn time(self) -> ScheduleTime {
        self.time
    }
}

// =============================================================================
// Schedule errors
// =============================================================================

/// Errors produced while constructing or validating a schedule.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScheduleError {
    /// An operation ID appears more than once.
    DuplicateOperation {
        /// Duplicated operation.
        operation: OperationId,
    },

    /// Two operations overlap on a resource that must be serialized.
    ResourceOverlap {
        /// First operation.
        first: OperationId,

        /// Second operation.
        second: OperationId,

        /// Shared resource.
        resource: ScheduleResource,
    },

    /// An operation's end time overflowed.
    TimeOverflow {
        /// Operation that caused the overflow.
        operation: OperationId,
    },

    /// A schedule-level end time overflowed.
    ScheduleTimeOverflow,

    /// A requested operation count exceeds policy.
    OperationLimit(LimitsError),

    /// A requested schedule time exceeds policy.
    TimeLimit(LimitsError),

    /// A requested resource count exceeds policy.
    ResourceLimit(LimitsError),

    /// An invalid dependency references an operation that does not exist.
    UnknownDependency {
        /// Operation containing the dependency.
        operation: OperationId,

        /// Referenced dependency.
        dependency: OperationId,
    },

    /// A dependency graph contains a cycle.
    DependencyCycle,

    /// A synchronization point has an invalid time.
    InvalidSynchronizationPoint {
        /// Synchronization identity.
        id: u64,
    },

    /// A schedule has an invalid IR version.
    UnsupportedIrVersion {
        /// Version carried by the schedule.
        version: IrVersion,
    },
}

impl fmt::Display for ScheduleError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::DuplicateOperation { operation } => {
                write!(
                    formatter,
                    "schedule contains duplicate operation `{operation}`"
                )
            }

            Self::ResourceOverlap {
                first,
                second,
                resource,
            } => {
                write!(
                    formatter,
                    "scheduled operations `{first}` and `{second}` \
                     overlap on {resource:?}"
                )
            }

            Self::TimeOverflow { operation } => {
                write!(
                    formatter,
                    "scheduled operation `{operation}` has an overflowing end time"
                )
            }

            Self::ScheduleTimeOverflow => {
                formatter.write_str(
                    "schedule end time overflowed the canonical time representation",
                )
            }

            Self::OperationLimit(error) => {
                write!(formatter, "schedule operation limit: {error}")
            }

            Self::TimeLimit(error) => {
                write!(formatter, "schedule time limit: {error}")
            }

            Self::ResourceLimit(error) => {
                write!(formatter, "schedule resource limit: {error}")
            }

            Self::UnknownDependency {
                operation,
                dependency,
            } => {
                write!(
                    formatter,
                    "operation `{operation}` depends on unknown operation `{dependency}`"
                )
            }

            Self::DependencyCycle => {
                formatter.write_str("schedule dependency graph contains a cycle")
            }

            Self::InvalidSynchronizationPoint { id } => {
                write!(
                    formatter,
                    "synchronization point `{id}` has an invalid time"
                )
            }

            Self::UnsupportedIrVersion { version } => {
                write!(
                    formatter,
                    "schedule uses unsupported Quantum IR version `{version}`"
                )
            }
        }
    }
}

impl Error for ScheduleError {}

// =============================================================================
// Schedule
// =============================================================================

/// Canonical hardware-independent quantum schedule.
///
/// `Schedule` represents temporal placement of canonical IR operations.
///
/// It intentionally does not own scheduling algorithms.
///
/// # Dependency model
///
/// Dependencies are represented as:
//
//! ```text
//! operation -> dependencies
//! ```
//!
//! The schedule validates that dependencies exist and that the dependency
//! relation is acyclic.
//!
//! Dependencies are semantic ordering constraints. They do not themselves
//! determine hardware execution instructions.
//!
//! # Resource model
//!
//! Resources are semantic occupancy references. The schedule can therefore
//! validate conflicts without knowing what physical hardware implements the
//! resource.
//!
//! # Scalability
//!
//! There is no fixed architectural maximum in this structure.
//!
//! Memory usage naturally scales with the number of scheduled operations and
//! resource references actually represented.
//!
//! A `QuantumIrLimits` policy can impose a bounded deployment-specific limit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Schedule {
    id: ScheduleId,
    ir_version: IrVersion,
    operations: Vec<ScheduledOperation>,
    dependencies: BTreeMap<OperationId, BTreeSet<OperationId>>,
    synchronization_points: Vec<SynchronizationPoint>,
}

impl Schedule {
    /// Creates an empty schedule using the current IR version.
    #[must_use]
    pub fn new(id: ScheduleId) -> Self {
        Self {
            id,
            ir_version: IrVersion::CURRENT,
            operations: Vec::new(),
            dependencies: BTreeMap::new(),
            synchronization_points: Vec::new(),
        }
    }

    /// Creates an empty schedule for an explicitly supplied IR version.
    ///
    /// Version compatibility is validated when the schedule is validated.
    #[must_use]
    pub fn with_version(
        id: ScheduleId,
        ir_version: IrVersion,
    ) -> Self {
        Self {
            id,
            ir_version,
            operations: Vec::new(),
            dependencies: BTreeMap::new(),
            synchronization_points: Vec::new(),
        }
    }

    /// Returns the stable schedule identity.
    #[must_use]
    pub const fn id(&self) -> ScheduleId {
        self.id
    }

    /// Returns the IR schema version.
    #[must_use]
    pub const fn ir_version(&self) -> IrVersion {
        self.ir_version
    }

    /// Returns scheduled operations in canonical deterministic order.
    #[must_use]
    pub fn operations(&self) -> &[ScheduledOperation] {
        &self.operations
    }

    /// Returns the dependency set for an operation.
    #[must_use]
    pub fn dependencies(
        &self,
        operation: OperationId,
    ) -> Option<&BTreeSet<OperationId>> {
        self.dependencies.get(&operation)
    }

    /// Returns all synchronization points.
    #[must_use]
    pub fn synchronization_points(
        &self,
    ) -> &[SynchronizationPoint] {
        &self.synchronization_points
    }

    /// Returns the number of scheduled operations.
    #[must_use]
    pub fn operation_count(&self) -> usize {
        self.operations.len()
    }

    /// Returns the number of synchronization points.
    #[must_use]
    pub fn synchronization_point_count(&self) -> usize {
        self.synchronization_points.len()
    }

    /// Inserts a scheduled operation.
    ///
    /// The operation is inserted in deterministic order:
    ///
    /// 1. start time;
    /// 2. end time;
    /// 3. operation identity.
    ///
    /// This method does not require the schedule to be conflict-free
    /// immediately. Use `validate()` to perform complete schedule validation.
    pub fn push(
        &mut self,
        operation: ScheduledOperation,
    ) -> Result<(), ScheduleError> {
        if self
            .operations
            .iter()
            .any(|existing| existing.operation_id() == operation.operation_id())
        {
            return Err(ScheduleError::DuplicateOperation {
                operation: operation.operation_id(),
            });
        }

        if operation.end().is_none() {
            return Err(ScheduleError::TimeOverflow {
                operation: operation.operation_id(),
            });
        }

        self.operations.push(operation);
        self.sort_operations();

        Ok(())
    }

    /// Adds a dependency from `operation` to `dependency`.
    ///
    /// The dependency must exist in the schedule before final validation.
    pub fn add_dependency(
        &mut self,
        operation: OperationId,
        dependency: OperationId,
    ) -> Result<(), ScheduleError> {
        if operation == dependency {
            return Err(ScheduleError::DependencyCycle);
        }

        self.dependencies
            .entry(operation)
            .or_insert_with(BTreeSet::new)
            .insert(dependency);

        Ok(())
    }

    /// Adds a synchronization point.
    pub fn add_synchronization_point(
        &mut self,
        point: SynchronizationPoint,
    ) -> Result<(), ScheduleError> {
        self.synchronization_points.push(point);

        self.synchronization_points.sort_by(
            |left, right| {
                left.time()
                    .cmp(&right.time())
                    .then_with(|| left.id().cmp(&right.id()))
            },
        );

        Ok(())
    }

    /// Returns the schedule's total duration.
    ///
    /// The returned value is the latest operation end time or zero for an
    /// empty schedule.
    pub fn total_duration(&self) -> Result<ScheduleTime, ScheduleError> {
        let mut maximum = ScheduleTime::ZERO;

        for operation in &self.operations {
            let end = operation.end().ok_or(
                ScheduleError::TimeOverflow {
                    operation: operation.operation_id(),
                },
            )?;

            if end > maximum {
                maximum = end;
            }
        }

        Ok(maximum)
    }

    /// Returns whether the schedule is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.operations.is_empty()
    }

    /// Sorts operations into canonical deterministic order.
    fn sort_operations(&mut self) {
        self.operations.sort_by(
            |left, right| {
                let left_end = left.end();
                let right_end = right.end();

                left.start()
                    .cmp(&right.start())
                    .then_with(
                        || left_end.cmp(&right_end),
                    )
                    .then_with(
                        || left.operation_id().cmp(&right.operation_id()),
                    )
            },
        );
    }

    /// Validates the complete schedule using the supplied IR policy.
    ///
    /// Validation includes:
    ///
    /// - IR-version compatibility;
    /// - operation-count limits;
    /// - duplicate operation identities;
    /// - operation end-time overflow;
    /// - schedule duration limits;
    /// - dependency references;
    /// - dependency cycles;
    /// - synchronization-point validity;
    /// - resource overlap.
    pub fn validate(
        &self,
        limits: &QuantumIrLimits,
    ) -> Result<(), ScheduleError> {
        if !self.ir_version.is_supported_by_current() {
            return Err(
                ScheduleError::UnsupportedIrVersion {
                    version: self.ir_version,
                },
            );
        }

        limits
            .check_scheduled_operations(self.operations.len())
            .map_err(ScheduleError::OperationLimit)?;

        let mut operation_ids =
            BTreeSet::new();

        for operation in &self.operations {
            if !operation_ids.insert(operation.operation_id()) {
                return Err(
                    ScheduleError::DuplicateOperation {
                        operation: operation.operation_id(),
                    },
                );
            }

            if operation.end().is_none() {
                return Err(
                    ScheduleError::TimeOverflow {
                        operation: operation.operation_id(),
                    },
                );
            }
        }

        let total_duration =
            self.total_duration()?;

        limits
            .check_schedule_time_units(
                total_duration.attoseconds(),
            )
            .map_err(ScheduleError::TimeLimit)?;

        self.validate_dependencies(
            &operation_ids,
        )?;

        self.validate_synchronization_points(
            total_duration,
        )?;

        self.validate_resource_overlaps()?;

        Ok(())
    }

    /// Validates dependencies against the operation namespace.
    fn validate_dependencies(
        &self,
        operation_ids: &BTreeSet<OperationId>,
    ) -> Result<(), ScheduleError> {
        for (operation, dependencies) in &self.dependencies {
            if !operation_ids.contains(operation) {
                return Err(
                    ScheduleError::UnknownDependency {
                        operation: *operation,
                        dependency: *operation,
                    },
                );
            }

            for dependency in dependencies {
                if !operation_ids.contains(dependency) {
                    return Err(
                        ScheduleError::UnknownDependency {
                            operation: *operation,
                            dependency: *dependency,
                        },
                    );
                }
            }
        }

        self.detect_dependency_cycle(operation_ids)
    }

    /// Detects dependency cycles using iterative depth-first traversal.
    ///
    /// An iterative algorithm is used rather than recursive traversal so that
    /// very deep dependency graphs cannot overflow the Rust call stack.
    fn detect_dependency_cycle(
        &self,
        operation_ids: &BTreeSet<OperationId>,
    ) -> Result<(), ScheduleError> {
        #[derive(Clone, Copy)]
        enum VisitState {
            Visiting,
            Visited,
        }

        let mut states: BTreeMap<
            OperationId,
            VisitState,
        > = BTreeMap::new();

        for &root in operation_ids {
            if states.contains_key(&root) {
                continue;
            }

            let mut stack: Vec<(
                OperationId,
                bool,
            )> = Vec::new();

            stack.push((root, false));

            while let Some(
                (operation, exiting),
            ) = stack.pop()
            {
                if exiting {
                    states.insert(
                        operation,
                        VisitState::Visited,
                    );
                    continue;
                }

                if let Some(
                    VisitState::Visiting,
                ) = states.get(&operation)
                {
                    return Err(
                        ScheduleError::DependencyCycle,
                    );
                }

                if let Some(
                    VisitState::Visited,
                ) = states.get(&operation)
                {
                    continue;
                }

                states.insert(
                    operation,
                    VisitState::Visiting,
                );

                stack.push((operation, true));

                if let Some(dependencies) =
                    self.dependencies.get(&operation)
                {
                    for &dependency in dependencies
                        .iter()
                        .rev()
                    {
                        match states.get(&dependency) {
                            Some(
                                VisitState::Visiting,
                            ) => {
                                return Err(
                                    ScheduleError::DependencyCycle,
                                );
                            }

                            Some(
                                VisitState::Visited,
                            ) => {}

                            None => {
                                stack.push((
                                    dependency,
                                    false,
                                ));
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Validates synchronization points.
    fn validate_synchronization_points(
        &self,
        total_duration: ScheduleTime,
    ) -> Result<(), ScheduleError> {
        let mut previous_time =
            ScheduleTime::ZERO;

        for point in &self.synchronization_points {
            if point.time() < previous_time
                || point.time() > total_duration
            {
                return Err(
                    ScheduleError::InvalidSynchronizationPoint {
                        id: point.id(),
                    },
                );
            }

            previous_time = point.time();
        }

        Ok(())
    }

    /// Validates that operations do not overlap on the same semantic resource.
    ///
    /// Complexity is approximately O(n log n) for normal schedules because
    /// operations are already maintained in start-time order and only active
    /// resource occupancy is retained.
    fn validate_resource_overlaps(
        &self,
    ) -> Result<(), ScheduleError> {
        let mut active: BTreeMap<
            ScheduleResource,
            Vec<(
                ScheduleTime,
                OperationId,
            )>,
        > = BTreeMap::new();

        for operation in &self.operations {
            let operation_end =
                operation.end().ok_or(
                    ScheduleError::TimeOverflow {
                        operation: operation.operation_id(),
                    },
                )?;

            for &resource in operation.resources() {
                let entries =
                    active.entry(resource).or_insert_with(Vec::new);

                entries.retain(
                    |(end, _)| *end > operation.start(),
                );

                if let Some(
                    &(_, previous_operation),
                ) = entries.last()
                {
                    return Err(
                        ScheduleError::ResourceOverlap {
                            first: previous_operation,
                            second: operation.operation_id(),
                            resource,
                        },
                    );
                }

                entries.push((
                    operation_end,
                    operation.operation_id(),
                ));
            }
        }

        Ok(())
    }

    /// Validates this schedule with the default production policy.
    pub fn validate_production(
        &self,
    ) -> Result<(), ScheduleError> {
        self.validate(
            &QuantumIrLimits::production(),
        )
    }

    /// Returns a schedule containing the same operations and metadata but with
    /// operations canonically sorted.
    ///
    /// This is useful after transformations that directly manipulate the
    /// underlying operation sequence.
    #[must_use]
    pub fn canonicalized(mut self) -> Self {
        self.sort_operations();

        self.synchronization_points.sort_by(
            |left, right| {
                left.time()
                    .cmp(&right.time())
                    .then_with(|| left.id().cmp(&right.id()))
            },
        );

        self
    }
}

// =============================================================================
// Schedule builder
// =============================================================================

/// Incremental schedule builder.
///
/// The builder provides a controlled construction API while keeping the
/// resulting `Schedule` immutable-by-convention once handed to downstream
/// compiler stages.
#[derive(Debug, Clone)]
pub struct ScheduleBuilder {
    schedule: Schedule,
    limits: QuantumIrLimits,
}

impl ScheduleBuilder {
    /// Creates a builder using the production policy.
    #[must_use]
    pub fn production(
        id: ScheduleId,
    ) -> Self {
        Self {
            schedule: Schedule::new(id),
            limits: QuantumIrLimits::production(),
        }
    }

    /// Creates a builder with an explicit resource policy.
    #[must_use]
    pub fn with_limits(
        id: ScheduleId,
        limits: QuantumIrLimits,
    ) -> Self {
        Self {
            schedule: Schedule::new(id),
            limits,
        }
    }

    /// Creates a builder using an explicit IR version and resource policy.
    #[must_use]
    pub fn with_version_and_limits(
        id: ScheduleId,
        version: IrVersion,
        limits: QuantumIrLimits,
    ) -> Self {
        Self {
            schedule: Schedule::with_version(
                id,
                version,
            ),
            limits,
        }
    }

    /// Returns the configured limits.
    #[must_use]
    pub const fn limits(&self) -> &QuantumIrLimits {
        &self.limits
    }

    /// Adds a scheduled operation.
    pub fn push(
        &mut self,
        operation: ScheduledOperation,
    ) -> Result<(), ScheduleError> {
        let next_count =
            self.schedule
                .operation_count()
                .checked_add(1)
                .ok_or(
                    ScheduleError::OperationLimit(
                        LimitsError::ArithmeticOverflow {
                            resource:
                                super::limits::ResourceKind::ScheduledOperations,
                        },
                    ),
                )?;

        self.limits
            .check_scheduled_operations(next_count)
            .map_err(ScheduleError::OperationLimit)?;

        self.schedule.push(operation)
    }

    /// Adds an operation dependency.
    pub fn add_dependency(
        &mut self,
        operation: OperationId,
        dependency: OperationId,
    ) -> Result<(), ScheduleError> {
        self.schedule
            .add_dependency(
                operation,
                dependency,
            )
    }

    /// Adds a synchronization point.
    pub fn add_synchronization_point(
        &mut self,
        point: SynchronizationPoint,
    ) -> Result<(), ScheduleError> {
        self.schedule
            .add_synchronization_point(point)
    }

    /// Finalizes and validates the schedule.
    pub fn build(
        self,
    ) -> Result<Schedule, ScheduleError> {
        self.schedule
            .validate(&self.limits)?;

        Ok(self.schedule.canonicalized())
    }
}

// =============================================================================
// Schedule statistics
// =============================================================================

/// Deterministic summary of a schedule.
///
/// This type intentionally contains only structural schedule information.
/// Hardware performance, fidelity, calibration quality, and execution results
/// belong to downstream systems.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ScheduleStatistics {
    /// Number of scheduled operations.
    pub operation_count: usize,

    /// Number of distinct logical-qubit resources.
    pub logical_qubit_count: usize,

    /// Number of distinct physical-qubit resources.
    pub physical_qubit_count: usize,

    /// Number of distinct abstract channels.
    pub channel_count: usize,

    /// Number of distinct abstract frames.
    pub frame_count: usize,

    /// Number of synchronization points.
    pub synchronization_point_count: usize,

    /// Total schedule duration.
    pub total_duration: ScheduleTime,
}

impl Schedule {
    /// Computes deterministic structural schedule statistics.
    pub fn statistics(
        &self,
    ) -> Result<ScheduleStatistics, ScheduleError> {
        let mut logical_qubits =
            BTreeSet::new();

        let mut physical_qubits =
            BTreeSet::new();

        let mut channels =
            BTreeSet::new();

        let mut frames =
            BTreeSet::new();

        for operation in &self.operations {
            for &resource in operation.resources() {
                match resource {
                    ScheduleResource::LogicalQubit(
                        qubit,
                    ) => {
                        logical_qubits.insert(qubit);
                    }

                    ScheduleResource::PhysicalQubit(
                        qubit,
                    ) => {
                        physical_qubits.insert(qubit);
                    }

                    ScheduleResource::Channel(
                        channel,
                    ) => {
                        channels.insert(channel);
                    }

                    ScheduleResource::Frame(
                        frame,
                    ) => {
                        frames.insert(frame);
                    }
                }
            }
        }

        Ok(ScheduleStatistics {
            operation_count:
                self.operations.len(),
            logical_qubit_count:
                logical_qubits.len(),
            physical_qubit_count:
                physical_qubits.len(),
            channel_count:
                channels.len(),
            frame_count:
                frames.len(),
            synchronization_point_count:
                self.synchronization_points.len(),
            total_duration:
                self.total_duration()?,
        })
    }
}

// =============================================================================
// Deterministic ordering helper
// =============================================================================

/// Compares two scheduled operations using the canonical schedule ordering.
#[must_use]
pub fn compare_scheduled_operations(
    left: &ScheduledOperation,
    right: &ScheduledOperation,
) -> Ordering {
    left.start()
        .cmp(&right.start())
        .then_with(
            || {
                left.end()
                    .cmp(&right.end())
            },
        )
        .then_with(
            || {
                left.operation_id()
                    .cmp(&right.operation_id())
            },
        )
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn operation(
        id: u64,
        start_ns: u128,
        duration_ns: u128,
    ) -> ScheduledOperation {
        ScheduledOperation::new(
            OperationId::new(id),
            ScheduleTime::from_nanoseconds(
                start_ns,
            )
            .expect("test time conversion"),
            ScheduleDuration::from_nanoseconds(
                duration_ns,
            )
            .expect("test duration conversion"),
        )
    }

    #[test]
    fn nanoseconds_convert_without_floating_point() {
        let time =
            ScheduleTime::from_nanoseconds(20)
                .expect("20ns conversion");

        assert_eq!(
            time.attoseconds(),
            20_000_000_000
        );
    }

    #[test]
    fn pulse_style_twenty_nanoseconds_is_representable() {
        let duration =
            ScheduleDuration::from_nanoseconds(20)
                .expect("20ns duration");

        assert_eq!(
            duration.attoseconds(),
            20_000_000_000
        );
    }

    #[test]
    fn schedule_orders_operations_deterministically() {
        let mut schedule =
            Schedule::new(
                ScheduleId::new(1),
            );

        schedule
            .push(operation(2, 20, 10))
            .expect("insert operation");

        schedule
            .push(operation(1, 10, 5))
            .expect("insert operation");

        assert_eq!(
            schedule.operations()[0]
                .operation_id(),
            OperationId::new(1)
        );

        assert_eq!(
            schedule.operations()[1]
                .operation_id(),
            OperationId::new(2)
        );
    }

    #[test]
    fn operation_end_is_checked() {
        let operation =
            ScheduledOperation::new(
                OperationId::new(1),
                ScheduleTime::from_attoseconds(
                    u128::MAX,
                ),
                ScheduleDuration::from_attoseconds(
                    1,
                ),
            );

        assert!(operation.end().is_none());
    }

    #[test]
    fn zero_duration_operations_can_share_a_start() {
        let mut schedule =
            Schedule::new(
                ScheduleId::new(1),
            );

        let first =
            ScheduledOperation::new(
                OperationId::new(1),
                ScheduleTime::ZERO,
                ScheduleDuration::ZERO,
            )
            .with_logical_qubit(
                QubitId::new(0),
            );

        let second =
            ScheduledOperation::new(
                OperationId::new(2),
                ScheduleTime::ZERO,
                ScheduleDuration::ZERO,
            )
            .with_logical_qubit(
                QubitId::new(0),
            );

        schedule
            .push(first)
            .expect("insert first");

        schedule
            .push(second)
            .expect("insert second");

        assert!(
            schedule
                .validate_production()
                .is_ok()
        );
    }

    #[test]
    fn overlapping_operations_on_same_resource_are_rejected() {
        let mut schedule =
            Schedule::new(
                ScheduleId::new(1),
            );

        let first =
            operation(1, 0, 20)
                .with_logical_qubit(
                    QubitId::new(0),
                );

        let second =
            operation(2, 10, 20)
                .with_logical_qubit(
                    QubitId::new(0),
                );

        schedule
            .push(first)
            .expect("insert first");

        schedule
            .push(second)
            .expect("insert second");

        assert!(matches!(
            schedule.validate_production(),
            Err(
                ScheduleError::ResourceOverlap {
                    ..
                }
            )
        ));
    }

    #[test]
    fn different_resources_may_execute_in_parallel() {
        let mut schedule =
            Schedule::new(
                ScheduleId::new(1),
            );

        let first =
            operation(1, 0, 20)
                .with_logical_qubit(
                    QubitId::new(0),
                );

        let second =
            operation(2, 0, 20)
                .with_logical_qubit(
                    QubitId::new(1),
                );

        schedule
            .push(first)
            .expect("insert first");

        schedule
            .push(second)
            .expect("insert second");

        assert!(
            schedule
                .validate_production()
                .is_ok()
        );
    }

    #[test]
    fn duplicate_operation_ids_are_rejected() {
        let mut schedule =
            Schedule::new(
                ScheduleId::new(1),
            );

        schedule
            .push(operation(1, 0, 10))
            .expect("insert first");

        assert!(matches!(
            schedule.push(
                operation(1, 20, 10)
            ),
            Err(
                ScheduleError::DuplicateOperation {
                    ..
                }
            )
        ));
    }

    #[test]
    fn dependency_to_unknown_operation_is_rejected() {
        let mut schedule =
            Schedule::new(
                ScheduleId::new(1),
            );

        schedule
            .push(operation(1, 0, 10))
            .expect("insert operation");

        schedule
            .add_dependency(
                OperationId::new(1),
                OperationId::new(999),
            )
            .expect("record dependency");

        assert!(matches!(
            schedule.validate_production(),
            Err(
                ScheduleError::UnknownDependency {
                    ..
                }
            )
        ));
    }

    #[test]
    fn dependency_cycle_is_rejected() {
        let mut schedule =
            Schedule::new(
                ScheduleId::new(1),
            );

        schedule
            .push(operation(1, 0, 10))
            .expect("insert operation");

        schedule
            .push(operation(2, 10, 10))
            .expect("insert operation");

        schedule
            .add_dependency(
                OperationId::new(1),
                OperationId::new(2),
            )
            .expect("dependency");

        schedule
            .add_dependency(
                OperationId::new(2),
                OperationId::new(1),
            )
            .expect("dependency");

        assert!(matches!(
            schedule.validate_production(),
            Err(
                ScheduleError::DependencyCycle
            )
        ));
    }

    #[test]
    fn physical_qubits_are_distinct_from_logical_qubits() {
        let operation =
            operation(1, 0, 10)
                .with_logical_qubit(
                    QubitId::new(0),
                )
                .with_physical_qubit(
                    PhysicalQubitId::new(0),
                );

        assert!(
            operation.uses_resource(
                ScheduleResource::LogicalQubit(
                    QubitId::new(0),
                )
            )
        );

        assert!(
            operation.uses_resource(
                ScheduleResource::PhysicalQubit(
                    PhysicalQubitId::new(0),
                )
            )
        );
    }

    #[test]
    fn schedule_statistics_are_deterministic() {
        let mut schedule =
            Schedule::new(
                ScheduleId::new(1),
            );

        schedule
            .push(
                operation(1, 0, 20)
                    .with_logical_qubit(
                        QubitId::new(0),
                    ),
            )
            .expect("insert operation");

        schedule
            .push(
                operation(2, 20, 20)
                    .with_logical_qubit(
                        QubitId::new(1),
                    ),
            )
            .expect("insert operation");

        let statistics =
            schedule
                .statistics()
                .expect("statistics");

        assert_eq!(
            statistics.operation_count,
            2
        );

        assert_eq!(
            statistics.logical_qubit_count,
            2
        );

        assert_eq!(
            statistics.total_duration
                .attoseconds(),
            40_000_000_000
        );
    }

    #[test]
    fn builder_enforces_limits_before_build() {
        let limits =
            QuantumIrLimits::production()
                .with_max_scheduled_operations(
                    1,
                );

        let mut builder =
            ScheduleBuilder::with_limits(
                ScheduleId::new(1),
                limits,
            );

        builder
            .push(operation(1, 0, 10))
            .expect("first operation");

        assert!(matches!(
            builder.push(
                operation(2, 10, 10)
            ),
            Err(
                ScheduleError::OperationLimit {
                    ..
                }
            )
        ));
    }

    #[test]
    fn schedule_time_limit_is_enforced() {
        let limits =
            QuantumIrLimits::production()
                .with_max_schedule_time_units(
                    20_000_000_000,
                );

        let mut schedule =
            Schedule::new(
                ScheduleId::new(1),
            );

        schedule
            .push(operation(1, 0, 21))
            .expect("insert operation");

        assert!(matches!(
            schedule.validate(&limits),
            Err(
                ScheduleError::TimeLimit {
                    ..
                }
            )
        ));
    }

    #[test]
    fn huge_identifier_values_do_not_create_qubit_limits() {
        let mut schedule =
            Schedule::new(
                ScheduleId::new(u64::MAX),
            );

        schedule
            .push(
                operation(1, 0, 1)
                    .with_logical_qubit(
                        QubitId::new(
                            usize::MAX,
                        ),
                    ),
            )
            .expect("large logical identifier");

        assert_eq!(
            schedule.operation_count(),
            1
        );
    }
}