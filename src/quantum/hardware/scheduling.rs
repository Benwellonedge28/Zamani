//! Zamani Quantum — Hardware-Constrained Scheduling
//!
//! Production-grade, provider-neutral scheduling primitives for quantum
//! hardware execution.
//!
//! # Responsibility
//!
//! This module answers:
//!
//! > "Given a hardware execution workload and its physical/resource
//! > constraints, when can each operation execute safely and deterministically?"
//!
//! It owns:
//!
//! - hardware scheduling constraints;
//! - operation timing requirements;
//! - resource occupancy;
//! - qubit/resource conflicts;
//! - dependency ordering;
//! - coupling/resource conflicts;
//! - crosstalk exclusion groups;
//! - measurement/reset/feed-forward latency;
//! - alignment constraints;
//! - scheduling granularity;
//! - parallel-operation validation;
//! - deterministic list scheduling;
//! - schedule validation;
//! - schedule statistics;
//! - makespan calculation;
//! - resource-occupancy calculation;
//! - scheduling diagnostics;
//! - provider-neutral scheduling policies;
//! - immutable schedule results;
//! - stable serialization-oriented representations;
//! - integration contracts for routing, backend, calibration, execution,
//!   benchmarking and Danga.
//!
//! It deliberately does NOT own:
//!
//! - quantum IR semantics;
//! - source-language parsing;
//! - circuit optimization;
//! - logical-to-physical routing;
//! - provider API communication;
//! - authentication;
//! - credentials;
//! - calibration acquisition;
//! - benchmark mathematics;
//! - QEC algorithms;
//! - pulse waveform generation;
//! - provider-specific scheduling APIs.
//!
//! Those responsibilities belong to their owning subsystems.
//!
//! # Architectural position
//!
//! ```text
//! Zamani source
//!       |
//!       v
//! Quantum Frontend
//!       |
//!       v
//! Zamani Quantum IR
//!       |
//!       v
//! optimization
//!       |
//!       v
//! routing
//!       |
//!       v
//! physical workload
//!       |
//!       +------------------------+
//!       |                        |
//!       v                        v
//! hardware topology       hardware calibration
//!       |                        |
//!       +------------+-----------+
//!                    |
//!                    v
//!          hardware scheduling
//!                    |
//!                    v
//!             ScheduledWorkload
//!                    |
//!                    v
//!             hardware execution
//! ```
//!
//! Scheduling is therefore a consumer of hardware constraints. It does not
//! mutate the authoritative topology or calibration state.
//!
//! # Relationship with `quantum::scheduling`
//!
//! This module is intentionally separate from:
//!
//! `quantum::scheduling::stabilizer_scheduler`
//!
//! The latter owns QEC/stabilizer-specific scheduling.
//!
//! This module owns generic physical hardware scheduling constraints.
//!
//! A future QEC scheduler may construct `SchedulingOperation`s and submit
//! them to this module for physical timing validation, but this module must
//! never depend on QEC.
//!
//! # Relationship with routing
//!
//! Routing answers:
//!
//! > "Which physical resources should each logical operation use?"
//!
//! Scheduling answers:
//!
//! > "When can those already-mapped physical operations execute?"
//!
//! Therefore this module accepts physical resource identifiers but does not
//! calculate logical-to-physical mappings.
//!
//! # Relationship with `hardware::backend`
//!
//! The existing backend abstraction already establishes the correct
//! separation between backend capabilities, topology and execution.
//! Scheduling consumes those concepts but does not own them.
//!
//! In particular, this module must not duplicate:
//!
//! - `BackendCapabilities`;
//! - `BackendLimits`;
//! - `BackendStatus`;
//! - `HardwareTopology`.
//!
//! A future integration layer should translate authoritative backend,
//! topology, timing and calibration information into `SchedulingConstraints`.
//!
//! # Relationship with `hardware::validation`
//!
//! Validation determines whether a workload/backend pair is acceptable.
//!
//! Scheduling determines a legal execution timeline once the workload has
//! passed the appropriate structural/capability validation.
//!
//! Scheduling MUST NOT be used as a substitute for backend validation.
//!
//! # Relationship with execution
//!
//! The intended flow is:
//!
//! ```text
//! Workload
//!    |
//!    v
//! validation
//!    |
//!    v
//! routing
//!    |
//!    v
//! hardware scheduling
//!    |
//!    v
//! ScheduledWorkload
//!    |
//!    v
//! execution adapter
//! ```
//!
//! # Design goals
//!
//! This module is designed for:
//!
//! - deterministic output;
//! - explicit units;
//! - checked arithmetic;
//! - no floating-point timing;
//! - no hidden wall-clock access;
//! - no randomness;
//! - no provider assumptions;
//! - no global state;
//! - no unsafe Rust;
//! - reproducible scheduling;
//! - explicit failure;
//! - immutable schedule results;
//! - bounded resource usage;
//! - stable machine-readable diagnostics.
//!
//! # Time representation
//!
//! Scheduling uses integer attoseconds (`10^-18` seconds) internally.
//!
//! This avoids floating-point rounding and permits exact composition of
//! nanosecond, picosecond and finer-grained hardware timing constraints.
//!
//! Callers may construct durations from:
//!
//! - seconds;
//! - milliseconds;
//! - microseconds;
//! - nanoseconds;
//! - picoseconds;
//! - femtoseconds;
//! - attoseconds.
//!
//! No floating-point value is accepted by the core scheduling API.
//!
//! # Determinism
//!
//! Given identical:
//!
//! - operations;
//! - dependencies;
//! - resource identifiers;
//! - constraints;
//! - policy;
//!
//! the scheduler MUST produce identical output.
//!
//! Tie-breaking is deterministic and based on:
//!
//! 1. earliest legal start;
//! 2. operation priority;
//! 3. criticality;
//! 4. operation identifier.
//!
//! # Safety
//!
//! This module uses no unsafe code.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021.
//!
//! No nightly features are required.
//!
//! # Integration contract
//!
//! This file is independently complete.
//!
//! Future modules may integrate with it without modifying this file.
//!
//! The authoritative integration points are:
//!
//! - `hardware::backend` -> translate backend limits/capabilities;
//! - `hardware::topology` -> translate connectivity/resource constraints;
//! - `hardware::calibration` -> translate calibrated durations/latencies;
//! - `hardware::timing` -> translate canonical timing quantities;
//! - `hardware::instruction_set` -> translate operation timing/requirements;
//! - `hardware::routing` -> provide physical mappings;
//! - `hardware::validation` -> validate before scheduling;
//! - `hardware::execution` -> consume `Schedule`;
//! - `benchmarking` -> record schedule provenance/statistics;
//! - `Danga` -> expose scheduling diagnostics and estimates.
//!
//! No future module is permitted to change the semantics of this file merely
//! to accommodate its own representation.
//!
//! If another subsystem uses a different timing/resource representation, it
//! must adapt that representation at the integration boundary.
//!
//! # Versioning
//!
//! `SCHEDULING_SCHEMA_VERSION` is part of the serialized/provenance contract.
//! It must only be incremented when the meaning of the scheduling model
//! changes incompatibly.
//!

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

// =============================================================================
// Schema
// =============================================================================

/// Stable scheduling schema identifier.
pub const SCHEDULING_SCHEMA_ID: &str = "zamani.quantum.hardware.scheduling";

/// Stable scheduling schema version.
pub const SCHEDULING_SCHEMA_VERSION: u16 = 1;

/// Maximum number of operations accepted by one schedule.
///
/// This prevents accidental unbounded memory consumption from malformed
/// workloads. Applications requiring larger workloads should partition them
/// into explicit execution units.
pub const MAX_SCHEDULE_OPERATIONS: usize = 10_000_000;

/// Maximum number of resources accepted by one schedule.
pub const MAX_SCHEDULE_RESOURCES: usize = 10_000_000;

/// Maximum number of dependency edges.
pub const MAX_DEPENDENCY_EDGES: usize = 50_000_000;

/// Maximum number of crosstalk groups.
pub const MAX_CROSSTALK_GROUPS: usize = 1_000_000;

/// Maximum number of resources in one operation.
pub const MAX_OPERATION_RESOURCES: usize = 1024;

/// Maximum supported operation duration in attoseconds.
///
/// One day expressed in attoseconds.
///
/// This is intentionally bounded so malformed durations cannot silently
/// overflow scheduling arithmetic.
pub const MAX_DURATION_ATTOSECONDS: u128 = 86_400_u128 * 1_000_000_000_000_000_000_u128;

// =============================================================================
// Scheduling time
// =============================================================================

/// Exact non-negative hardware duration.
///
/// Internally represented in attoseconds.
///
/// `1 second = 10^18 attoseconds`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Duration {
    attoseconds: u128,
}

impl Duration {
    /// Zero duration.
    pub const ZERO: Self = Self { attoseconds: 0 };

    /// Creates a duration from attoseconds.
    ///
    /// Returns `None` if the value exceeds the production safety bound.
    pub const fn from_attoseconds(attoseconds: u128) -> Option<Self> {
        if attoseconds <= MAX_DURATION_ATTOSECONDS {
            Some(Self { attoseconds })
        } else {
            None
        }
    }

    /// Creates a duration from femtoseconds.
    pub const fn from_femtoseconds(value: u64) -> Option<Self> {
        Self::from_attoseconds((value as u128).checked_mul(1_000_000_000_u128)?)
    }

    /// Creates a duration from picoseconds.
    pub const fn from_picoseconds(value: u64) -> Option<Self> {
        Self::from_attoseconds((value as u128).checked_mul(1_000_000_000_000_u128)?)
    }

    /// Creates a duration from nanoseconds.
    pub const fn from_nanoseconds(value: u64) -> Option<Self> {
        Self::from_attoseconds((value as u128).checked_mul(1_000_000_000_000_000_u128)?)
    }

    /// Creates a duration from microseconds.
    pub const fn from_microseconds(value: u64) -> Option<Self> {
        Self::from_attoseconds((value as u128).checked_mul(1_000_000_000_000_000_000_u128)? / 1_000_u128)
    }

    /// Creates a duration from milliseconds.
    pub const fn from_milliseconds(value: u64) -> Option<Self> {
        Self::from_attoseconds((value as u128).checked_mul(1_000_000_000_000_000_000_u128)? / 1_000_u128 / 1_000_u128)
    }

    /// Creates a duration from seconds.
    pub const fn from_seconds(value: u64) -> Option<Self> {
        Self::from_attoseconds(
            (value as u128).checked_mul(1_000_000_000_000_000_000_u128)?,
        )
    }

    /// Returns attoseconds.
    pub const fn as_attoseconds(self) -> u128 {
        self.attoseconds
    }

    /// Returns whether this is zero.
    pub const fn is_zero(self) -> bool {
        self.attoseconds == 0
    }

    /// Checked addition.
    pub const fn checked_add(self, other: Self) -> Option<Self> {
        match self.attoseconds.checked_add(other.attoseconds) {
            Some(value) => Self::from_attoseconds(value),
            None => None,
        }
    }

    /// Checked subtraction.
    pub const fn checked_sub(self, other: Self) -> Option<Self> {
        match self.attoseconds.checked_sub(other.attoseconds) {
            Some(value) => Self::from_attoseconds(value),
            None => None,
        }
    }
}

impl Default for Duration {
    fn default() -> Self {
        Self::ZERO
    }
}

impl fmt::Display for Duration {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} as", self.attoseconds)
    }
}

// =============================================================================
// Time point
// =============================================================================

/// Absolute schedule time represented in attoseconds from schedule origin.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct TimePoint {
    attoseconds: u128,
}

impl TimePoint {
    /// Schedule origin.
    pub const ZERO: Self = Self { attoseconds: 0 };

    /// Creates a time point.
    pub const fn from_attoseconds(attoseconds: u128) -> Option<Self> {
        if attoseconds <= MAX_DURATION_ATTOSECONDS {
            Some(Self { attoseconds })
        } else {
            None
        }
    }

    /// Returns attoseconds from schedule origin.
    pub const fn as_attoseconds(self) -> u128 {
        self.attoseconds
    }

    /// Returns the time point reached after adding a duration.
    pub const fn checked_add(self, duration: Duration) -> Option<Self> {
        match self.attoseconds.checked_add(duration.as_attoseconds()) {
            Some(value) => Self::from_attoseconds(value),
            None => None,
        }
    }

    /// Returns the elapsed duration between two ordered points.
    pub const fn checked_duration_since(self, earlier: Self) -> Option<Duration> {
        match self.attoseconds.checked_sub(earlier.attoseconds) {
            Some(value) => Duration::from_attoseconds(value),
            None => None,
        }
    }
}

impl fmt::Display for TimePoint {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} as", self.attoseconds)
    }
}

// =============================================================================
// Resource identifiers
// =============================================================================

/// Physical hardware resource identifier.
///
/// A resource may represent a physical qubit, control channel, measurement
/// channel, coupler, resonator, bus, or any other mutually-exclusive
/// scheduling resource.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ResourceId(pub u32);

impl ResourceId {
    /// Creates a resource identifier.
    pub const fn new(id: u32) -> Self {
        Self(id)
    }

    /// Returns the numeric identifier.
    pub const fn get(self) -> u32 {
        self.0
    }
}

impl fmt::Display for ResourceId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "r{}", self.0)
    }
}

// =============================================================================
// Operation identifiers
// =============================================================================

/// Stable operation identifier within one workload.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct OperationId(pub u64);

impl OperationId {
    /// Creates an operation identifier.
    pub const fn new(id: u64) -> Self {
        Self(id)
    }

    /// Returns the numeric identifier.
    pub const fn get(self) -> u64 {
        self.0
    }
}

impl fmt::Display for OperationId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "op{}", self.0)
    }
}

// =============================================================================
// Scheduling priority
// =============================================================================

/// Scheduling priority.
///
/// Higher values have precedence when two ready operations have the same
/// earliest legal start time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Priority(u16);

impl Priority {
    /// Lowest priority.
    pub const MIN: Self = Self(0);

    /// Highest priority.
    pub const MAX: Self = Self(u16::MAX);

    /// Creates a priority.
    pub const fn new(value: u16) -> Self {
        Self(value)
    }

    /// Returns the numeric priority.
    pub const fn get(self) -> u16 {
        self.0
    }
}

impl Default for Priority {
    fn default() -> Self {
        Self::MIN
    }
}

// =============================================================================
// Operation class
// =============================================================================

/// Semantic scheduling class of an operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum OperationClass {
    /// Ordinary quantum gate.
    Gate,

    /// Measurement.
    Measurement,

    /// Reset.
    Reset,

    /// Delay/idle interval.
    Delay,

    /// Classical feed-forward/control operation.
    ClassicalControl,

    /// Synchronization barrier.
    Barrier,

    /// Pulse/control operation.
    Pulse,

    /// Analog-control operation.
    Analog,

    /// Annealing operation.
    Annealing,

    /// Error-correction/syndrome operation.
    Syndrome,

    /// Provider-defined operation.
    Custom,
}

impl OperationClass {
    /// Stable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Gate => "gate",
            Self::Measurement => "measurement",
            Self::Reset => "reset",
            Self::Delay => "delay",
            Self::ClassicalControl => "classical_control",
            Self::Barrier => "barrier",
            Self::Pulse => "pulse",
            Self::Analog => "analog",
            Self::Annealing => "annealing",
            Self::Syndrome => "syndrome",
            Self::Custom => "custom",
        }
    }
}

impl fmt::Display for OperationClass {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Scheduling operation
// =============================================================================

/// One physical operation presented to the scheduler.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SchedulingOperation {
    /// Stable operation identifier.
    pub id: OperationId,

    /// Scheduling class.
    pub class: OperationClass,

    /// Stable operation name.
    pub name: String,

    /// Physical resources occupied by the operation.
    pub resources: Vec<ResourceId>,

    /// Execution duration.
    pub duration: Duration,

    /// Minimum priority.
    pub priority: Priority,

    /// Optional earliest legal start.
    pub earliest_start: Option<TimePoint>,

    /// Optional latest legal start.
    ///
    /// This is a hard constraint.
    pub latest_start: Option<TimePoint>,

    /// Operations that must complete before this operation starts.
    pub dependencies: Vec<OperationId>,

    /// Whether this operation requires exclusive occupancy of all resources.
    pub exclusive: bool,

    /// Optional crosstalk group.
    ///
    /// Operations belonging to the same non-empty group cannot overlap when
    /// that group is configured as exclusive.
    pub crosstalk_group: Option<u32>,

    /// Whether the operation is latency-sensitive.
    pub latency_sensitive: bool,
}

impl SchedulingOperation {
    /// Creates an operation with conservative defaults.
    pub fn new(
        id: OperationId,
        class: OperationClass,
        name: impl Into<String>,
        resources: Vec<ResourceId>,
        duration: Duration,
    ) -> Result<Self, SchedulingError> {
        let name = name.into();

        validate_operation_name(&name)?;

        if resources.is_empty() {
            return Err(SchedulingError::OperationHasNoResources { operation: id });
        }

        if resources.len() > MAX_OPERATION_RESOURCES {
            return Err(SchedulingError::TooManyResources {
                operation: id,
                count: resources.len(),
                maximum: MAX_OPERATION_RESOURCES,
            });
        }

        let mut sorted = resources;
        sorted.sort_unstable();

        if sorted.windows(2).any(|pair| pair[0] == pair[1]) {
            return Err(SchedulingError::DuplicateOperationResource { operation: id });
        }

        Ok(Self {
            id,
            class,
            name,
            resources: sorted,
            duration,
            priority: Priority::MIN,
            earliest_start: None,
            latest_start: None,
            dependencies: Vec::new(),
            exclusive: true,
            crosstalk_group: None,
            latency_sensitive: false,
        })
    }

    /// Sets priority.
    pub const fn with_priority(mut self, priority: Priority) -> Self {
        self.priority = priority;
        self
    }

    /// Sets earliest start.
    pub const fn with_earliest_start(mut self, start: TimePoint) -> Self {
        self.earliest_start = Some(start);
        self
    }

    /// Sets latest start.
    pub const fn with_latest_start(mut self, start: TimePoint) -> Self {
        self.latest_start = Some(start);
        self
    }

    /// Adds one dependency.
    pub fn with_dependency(mut self, dependency: OperationId) -> Self {
        self.dependencies.push(dependency);
        self
    }

    /// Adds multiple dependencies.
    pub fn with_dependencies<I>(mut self, dependencies: I) -> Self
    where
        I: IntoIterator<Item = OperationId>,
    {
        self.dependencies.extend(dependencies);
        self
    }

    /// Changes resource occupancy semantics.
    pub const fn non_exclusive(mut self) -> Self {
        self.exclusive = false;
        self
    }

    /// Assigns a crosstalk group.
    pub const fn with_crosstalk_group(mut self, group: u32) -> Self {
        self.crosstalk_group = Some(group);
        self
    }

    /// Marks the operation latency-sensitive.
    pub const fn latency_sensitive(mut self) -> Self {
        self.latency_sensitive = true;
        self
    }
}

// =============================================================================
// Resource constraints
// =============================================================================

/// Constraint applied to one physical resource.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResourceConstraint {
    /// Resource identifier.
    pub resource: ResourceId,

    /// Whether only one operation may occupy the resource at a time.
    pub exclusive: bool,

    /// Additional guard time required after an operation.
    pub guard_time: Duration,

    /// Alignment required for operation starts.
    pub start_alignment: Option<Duration>,

    /// Optional maximum simultaneous occupancy.
    pub concurrency_limit: Option<u32>,
}

impl ResourceConstraint {
    /// Creates a conservative exclusive resource constraint.
    pub fn new(resource: ResourceId) -> Self {
        Self {
            resource,
            exclusive: true,
            guard_time: Duration::ZERO,
            start_alignment: None,
            concurrency_limit: None,
        }
    }

    /// Sets exclusivity.
    pub const fn with_exclusive(mut self, exclusive: bool) -> Self {
        self.exclusive = exclusive;
        self
    }

    /// Sets guard time.
    pub const fn with_guard_time(mut self, guard_time: Duration) -> Self {
        self.guard_time = guard_time;
        self
    }

    /// Sets start alignment.
    pub const fn with_alignment(mut self, alignment: Duration) -> Self {
        self.start_alignment = Some(alignment);
        self
    }

    /// Sets concurrency limit.
    pub const fn with_concurrency_limit(mut self, limit: u32) -> Self {
        self.concurrency_limit = Some(limit);
        self
    }
}

// =============================================================================
// Crosstalk constraints
// =============================================================================

/// A set of operations/resources that cannot safely overlap.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrosstalkGroup {
    /// Stable group identifier.
    pub id: u32,

    /// Resources participating in the group.
    pub resources: BTreeSet<ResourceId>,

    /// Whether operations in this group are mutually exclusive.
    pub exclusive: bool,

    /// Additional guard time after occupancy.
    pub guard_time: Duration,
}

impl CrosstalkGroup {
    /// Creates an empty exclusive group.
    pub fn new(id: u32) -> Self {
        Self {
            id,
            resources: BTreeSet::new(),
            exclusive: true,
            guard_time: Duration::ZERO,
        }
    }

    /// Adds a resource.
    pub fn add_resource(mut self, resource: ResourceId) -> Self {
        self.resources.insert(resource);
        self
    }

    /// Sets exclusivity.
    pub const fn with_exclusive(mut self, exclusive: bool) -> Self {
        self.exclusive = exclusive;
        self
    }

    /// Sets guard time.
    pub const fn with_guard_time(mut self, guard_time: Duration) -> Self {
        self.guard_time = guard_time;
        self
    }
}

// =============================================================================
// Global scheduling constraints
// =============================================================================

/// Hardware-wide scheduling constraints.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SchedulingConstraints {
    /// Resource constraints.
    pub resources: BTreeMap<ResourceId, ResourceConstraint>,

    /// Crosstalk groups.
    pub crosstalk_groups: BTreeMap<u32, CrosstalkGroup>,

    /// Global start alignment.
    pub global_alignment: Option<Duration>,

    /// Minimum delay after measurement before dependent classical control.
    pub measurement_to_control_latency: Duration,

    /// Minimum delay after reset before a subsequent operation on that
    /// resource.
    pub reset_latency: Duration,

    /// Minimum delay after a measurement before the same resource can be
    /// reused.
    pub measurement_latency: Duration,

    /// Maximum allowed makespan.
    pub maximum_makespan: Option<Duration>,

    /// Whether operations may execute in parallel when their resources permit.
    pub allow_parallelism: bool,

    /// Whether all resources must be known before scheduling.
    pub require_declared_resources: bool,
}

impl Default for SchedulingConstraints {
    fn default() -> Self {
        Self {
            resources: BTreeMap::new(),
            crosstalk_groups: BTreeMap::new(),
            global_alignment: None,
            measurement_to_control_latency: Duration::ZERO,
            reset_latency: Duration::ZERO,
            measurement_latency: Duration::ZERO,
            maximum_makespan: None,
            allow_parallelism: true,
            require_declared_resources: true,
        }
    }
}

impl SchedulingConstraints {
    /// Creates empty constraints.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a resource constraint.
    pub fn with_resource(
        mut self,
        constraint: ResourceConstraint,
    ) -> Result<Self, SchedulingError> {
        if self.resources.len() >= MAX_SCHEDULE_RESOURCES
            && !self.resources.contains_key(&constraint.resource)
        {
            return Err(SchedulingError::TooManyResources {
                operation: OperationId::new(0),
                count: self.resources.len() + 1,
                maximum: MAX_SCHEDULE_RESOURCES,
            });
        }

        self.resources.insert(constraint.resource, constraint);
        Ok(self)
    }

    /// Adds a crosstalk group.
    pub fn with_crosstalk_group(
        mut self,
        group: CrosstalkGroup,
    ) -> Result<Self, SchedulingError> {
        if self.crosstalk_groups.len() >= MAX_CROSSTALK_GROUPS
            && !self.crosstalk_groups.contains_key(&group.id)
        {
            return Err(SchedulingError::TooManyCrosstalkGroups);
        }

        self.crosstalk_groups.insert(group.id, group);
        Ok(self)
    }

    /// Sets global alignment.
    pub const fn with_global_alignment(mut self, alignment: Duration) -> Self {
        self.global_alignment = Some(alignment);
        self
    }

    /// Sets measurement-to-control latency.
    pub const fn with_measurement_to_control_latency(
        mut self,
        latency: Duration,
    ) -> Self {
        self.measurement_to_control_latency = latency;
        self
    }

    /// Sets reset latency.
    pub const fn with_reset_latency(mut self, latency: Duration) -> Self {
        self.reset_latency = latency;
        self
    }

    /// Sets measurement reuse latency.
    pub const fn with_measurement_latency(mut self, latency: Duration) -> Self {
        self.measurement_latency = latency;
        self
    }

    /// Sets maximum makespan.
    pub const fn with_maximum_makespan(mut self, duration: Duration) -> Self {
        self.maximum_makespan = Some(duration);
        self
    }

    /// Sets whether parallelism is permitted.
    pub const fn with_parallelism(mut self, allowed: bool) -> Self {
        self.allow_parallelism = allowed;
        self
    }

    /// Sets whether resources must be declared.
    pub const fn with_declared_resources(mut self, required: bool) -> Self {
        self.require_declared_resources = required;
        self
    }
}

// =============================================================================
// Scheduling policy
// =============================================================================

/// Scheduling strategy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SchedulingPolicy {
    /// Earliest legal execution with deterministic tie-breaking.
    EarliestStart,

    /// Prioritize operations with the longest dependency tail.
    CriticalPath,

    /// Prioritize latency-sensitive operations.
    LatencyAware,

    /// Deterministic hybrid policy.
    Hybrid,
}

impl Default for SchedulingPolicy {
    fn default() -> Self {
        Self::Hybrid
    }
}

// =============================================================================
// Scheduler configuration
// =============================================================================

/// Immutable scheduler configuration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SchedulerConfig {
    /// Scheduling policy.
    pub policy: SchedulingPolicy,

    /// Whether to validate the produced schedule before returning it.
    pub validate_result: bool,

    /// Whether a deterministic topological order must be used.
    pub deterministic: bool,

    /// Maximum number of scheduling iterations.
    ///
    /// Prevents pathological malformed dependency graphs from causing
    /// unbounded work.
    pub maximum_iterations: usize,
}

impl Default for SchedulerConfig {
    fn default() -> Self {
        Self {
            policy: SchedulingPolicy::Hybrid,
            validate_result: true,
            deterministic: true,
            maximum_iterations: MAX_SCHEDULE_OPERATIONS.saturating_mul(4),
        }
    }
}

impl SchedulerConfig {
    /// Creates production defaults.
    pub fn production() -> Self {
        Self::default()
    }

    /// Sets the policy.
    pub const fn with_policy(mut self, policy: SchedulingPolicy) -> Self {
        self.policy = policy;
        self
    }

    /// Enables/disables result validation.
    pub const fn with_result_validation(mut self, enabled: bool) -> Self {
        self.validate_result = enabled;
        self
    }

    /// Sets maximum iterations.
    pub const fn with_maximum_iterations(mut self, maximum: usize) -> Self {
        self.maximum_iterations = maximum;
        self
    }
}

// =============================================================================
// Scheduled operation
// =============================================================================

/// One operation after physical scheduling.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScheduledOperation {
    /// Operation identity.
    pub operation: OperationId,

    /// Scheduled start.
    pub start: TimePoint,

    /// Scheduled end.
    pub end: TimePoint,

    /// Occupied resources.
    pub resources: Vec<ResourceId>,

    /// Scheduling class.
    pub class: OperationClass,
}

impl ScheduledOperation {
    /// Returns execution duration.
    pub fn duration(&self) -> Option<Duration> {
        self.end.checked_duration_since(self.start)
    }
}

// =============================================================================
// Schedule
// =============================================================================

/// Immutable physical execution schedule.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Schedule {
    /// Scheduling schema version.
    pub schema_version: u16,

    /// Stable schema identifier.
    pub schema_id: &'static str,

    /// Scheduled operations in deterministic start/order sequence.
    pub operations: Vec<ScheduledOperation>,

    /// Total schedule duration.
    pub makespan: Duration,

    /// Maximum simultaneous operations.
    pub peak_parallelism: usize,

    /// Number of resources used.
    pub resource_count: usize,
}

impl Schedule {
    /// Returns the number of scheduled operations.
    pub fn operation_count(&self) -> usize {
        self.operations.len()
    }

    /// Returns whether no operations were scheduled.
    pub fn is_empty(&self) -> bool {
        self.operations.is_empty()
    }

    /// Finds a scheduled operation by ID.
    pub fn get(&self, id: OperationId) -> Option<&ScheduledOperation> {
        self.operations
            .iter()
            .find(|operation| operation.operation == id)
    }

    /// Returns operations occupying a resource.
    pub fn operations_on_resource(
        &self,
        resource: ResourceId,
    ) -> Vec<&ScheduledOperation> {
        self.operations
            .iter()
            .filter(|operation| operation.resources.binary_search(&resource).is_ok())
            .collect()
    }
}

// =============================================================================
// Schedule statistics
// =============================================================================

/// Deterministic scheduling statistics.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScheduleStatistics {
    /// Number of operations.
    pub operation_count: usize,

    /// Number of resources.
    pub resource_count: usize,

    /// Total execution duration.
    pub makespan: Duration,

    /// Maximum parallelism.
    pub peak_parallelism: usize,

    /// Number of operations that started later than their requested earliest
    /// start due to resource/dependency constraints.
    pub delayed_operations: usize,

    /// Number of operations with a non-zero idle gap before execution.
    pub operations_with_idle_gap: usize,
}

// =============================================================================
// Validation
// =============================================================================

/// Validation severity for schedule diagnostics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SchedulingSeverity {
    /// Informational diagnostic.
    Info,

    /// Non-blocking warning.
    Warning,

    /// Blocking error.
    Error,

    /// Fundamental invariant failure.
    Fatal,
}

impl SchedulingSeverity {
    /// Returns whether execution must be blocked.
    pub const fn blocks_execution(self) -> bool {
        matches!(self, Self::Error | Self::Fatal)
    }

    /// Stable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Warning => "warning",
            Self::Error => "error",
            Self::Fatal => "fatal",
        }
    }
}

/// Stable scheduling diagnostic code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SchedulingCode {
    EmptyOperationName,
    OperationWithoutResources,
    DuplicateOperation,
    DuplicateDependency,
    UnknownDependency,
    DependencyCycle,
    ResourceUndeclared,
    ResourceConflict,
    CrosstalkConflict,
    AlignmentViolation,
    EarliestStartViolation,
    LatestStartViolation,
    DependencyTimingViolation,
    NegativeInterval,
    DurationMismatch,
    MakespanExceeded,
    IterationLimitExceeded,
    EmptySchedule,
    ParallelismDisabled,
    InvalidConstraint,
}

impl SchedulingCode {
    /// Stable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptyOperationName => "operation.name.empty",
            Self::OperationWithoutResources => "operation.resources.empty",
            Self::DuplicateOperation => "operation.duplicate",
            Self::DuplicateDependency => "dependency.duplicate",
            Self::UnknownDependency => "dependency.unknown",
            Self::DependencyCycle => "dependency.cycle",
            Self::ResourceUndeclared => "resource.undeclared",
            Self::ResourceConflict => "resource.conflict",
            Self::CrosstalkConflict => "crosstalk.conflict",
            Self::AlignmentViolation => "timing.alignment",
            Self::EarliestStartViolation => "timing.earliest_start",
            Self::LatestStartViolation => "timing.latest_start",
            Self::DependencyTimingViolation => "dependency.timing",
            Self::NegativeInterval => "timing.negative_interval",
            Self::DurationMismatch => "timing.duration_mismatch",
            Self::MakespanExceeded => "schedule.makespan_exceeded",
            Self::IterationLimitExceeded => "scheduler.iteration_limit",
            Self::EmptySchedule => "schedule.empty",
            Self::ParallelismDisabled => "scheduler.parallelism_disabled",
            Self::InvalidConstraint => "constraint.invalid",
        }
    }
}

/// One scheduling diagnostic.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SchedulingDiagnostic {
    /// Stable code.
    pub code: SchedulingCode,

    /// Severity.
    pub severity: SchedulingSeverity,

    /// Human-readable explanation.
    pub message: String,

    /// Optional operation.
    pub operation: Option<OperationId>,

    /// Optional resource.
    pub resource: Option<ResourceId>,

    /// Optional conflicting operation.
    pub conflicting_operation: Option<OperationId>,
}

impl SchedulingDiagnostic {
    fn new(
        code: SchedulingCode,
        severity: SchedulingSeverity,
        message: impl Into<String>,
    ) -> Self {
        Self {
            code,
            severity,
            message: message.into(),
            operation: None,
            resource: None,
            conflicting_operation: None,
        }
    }

    fn with_operation(mut self, operation: OperationId) -> Self {
        self.operation = Some(operation);
        self
    }

    fn with_resource(mut self, resource: ResourceId) -> Self {
        self.resource = Some(resource);
        self
    }

    fn with_conflict(mut self, operation: OperationId) -> Self {
        self.conflicting_operation = Some(operation);
        self
    }
}

/// Complete schedule validation report.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SchedulingReport {
    /// Schema version.
    pub schema_version: u16,

    /// Schema identifier.
    pub schema_id: &'static str,

    /// Diagnostics.
    pub diagnostics: Vec<SchedulingDiagnostic>,
}

impl SchedulingReport {
    /// Creates an empty report.
    pub fn new() -> Self {
        Self {
            schema_version: SCHEDULING_SCHEMA_VERSION,
            schema_id: SCHEDULING_SCHEMA_ID,
            diagnostics: Vec::new(),
        }
    }

    /// Adds a diagnostic.
    pub fn push(&mut self, diagnostic: SchedulingDiagnostic) {
        self.diagnostics.push(diagnostic);
    }

    /// Returns whether there are blocking diagnostics.
    pub fn has_errors(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|diagnostic| diagnostic.severity.blocks_execution())
    }

    /// Returns whether validation passed.
    pub fn is_valid(&self) -> bool {
        !self.has_errors()
    }

    /// Returns warning count.
    pub fn warning_count(&self) -> usize {
        self.diagnostics
            .iter()
            .filter(|diagnostic| diagnostic.severity == SchedulingSeverity::Warning)
            .count()
    }
}

impl Default for SchedulingReport {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Errors
// =============================================================================

/// Production scheduling error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SchedulingError {
    /// Operation name is empty.
    EmptyOperationName {
        /// Operation ID.
        operation: OperationId,
    },

    /// Operation has no physical resources.
    OperationHasNoResources {
        /// Operation ID.
        operation: OperationId,
    },

    /// Operation uses too many resources.
    TooManyResources {
        /// Operation ID.
        operation: OperationId,

        /// Actual count.
        count: usize,

        /// Maximum count.
        maximum: usize,
    },

    /// Duplicate resource inside one operation.
    DuplicateOperationResource {
        /// Operation ID.
        operation: OperationId,
    },

    /// Too many operations.
    TooManyOperations {
        /// Actual count.
        count: usize,

        /// Maximum count.
        maximum: usize,
    },

    /// Too many resources.
    TooManyResourcesInWorkload {
        /// Actual count.
        count: usize,

        /// Maximum count.
        maximum: usize,
    },

    /// Too many dependency edges.
    TooManyDependencies {
        /// Actual count.
        count: usize,

        /// Maximum count.
        maximum: usize,
    },

    /// Too many crosstalk groups.
    TooManyCrosstalkGroups,

    /// Duplicate operation ID.
    DuplicateOperation {
        /// Operation ID.
        operation: OperationId,
    },

    /// Unknown dependency.
    UnknownDependency {
        /// Operation containing the dependency.
        operation: OperationId,

        /// Missing dependency.
        dependency: OperationId,
    },

    /// Dependency cycle.
    DependencyCycle,

    /// Required resource has no constraint declaration.
    UndeclaredResource {
        /// Resource.
        resource: ResourceId,

        /// Operation.
        operation: OperationId,
    },

    /// Latest start cannot satisfy constraints.
    LatestStartExceeded {
        /// Operation.
        operation: OperationId,
    },

    /// Makespan exceeds configured maximum.
    MakespanExceeded {
        /// Actual duration.
        actual: Duration,

        /// Maximum allowed.
        maximum: Duration,
    },

    /// Arithmetic overflow.
    ArithmeticOverflow,

    /// Scheduler exceeded its bounded work budget.
    IterationLimitExceeded,

    /// Invalid alignment.
    InvalidAlignment,

    /// Invalid concurrency limit.
    InvalidConcurrencyLimit,

    /// Produced schedule failed validation.
    InvalidSchedule {
        /// Complete validation report.
        report: SchedulingReport,
    },
}

impl fmt::Display for SchedulingError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyOperationName { operation } => {
                write!(formatter, "operation {operation} has an empty name")
            }
            Self::OperationHasNoResources { operation } => {
                write!(formatter, "operation {operation} has no resources")
            }
            Self::TooManyResources {
                operation,
                count,
                maximum,
            } => write!(
                formatter,
                "operation {operation} uses {count} resources; maximum is {maximum}"
            ),
            Self::DuplicateOperationResource { operation } => {
                write!(formatter, "operation {operation} contains duplicate resources")
            }
            Self::TooManyOperations { count, maximum } => write!(
                formatter,
                "workload contains {count} operations; maximum is {maximum}"
            ),
            Self::TooManyResourcesInWorkload {
                count,
                maximum,
            } => write!(
                formatter,
                "workload contains {count} resources; maximum is {maximum}"
            ),
            Self::TooManyDependencies { count, maximum } => write!(
                formatter,
                "workload contains {count} dependency edges; maximum is {maximum}"
            ),
            Self::TooManyCrosstalkGroups => {
                write!(formatter, "workload contains too many crosstalk groups")
            }
            Self::DuplicateOperation { operation } => {
                write!(formatter, "duplicate operation {operation}")
            }
            Self::UnknownDependency {
                operation,
                dependency,
            } => write!(
                formatter,
                "operation {operation} depends on unknown operation {dependency}"
            ),
            Self::DependencyCycle => write!(formatter, "operation dependency graph contains a cycle"),
            Self::UndeclaredResource { resource, operation } => write!(
                formatter,
                "operation {operation} uses undeclared resource {resource}"
            ),
            Self::LatestStartExceeded { operation } => {
                write!(formatter, "operation {operation} cannot satisfy latest-start constraint")
            }
            Self::MakespanExceeded { actual, maximum } => write!(
                formatter,
                "schedule makespan {actual} exceeds maximum {maximum}"
            ),
            Self::ArithmeticOverflow => {
                write!(formatter, "scheduling time arithmetic overflowed")
            }
            Self::IterationLimitExceeded => {
                write!(formatter, "scheduler exceeded its bounded iteration budget")
            }
            Self::InvalidAlignment => {
                write!(formatter, "scheduling alignment must be non-zero")
            }
            Self::InvalidConcurrencyLimit => {
                write!(formatter, "concurrency limit must be greater than zero")
            }
            Self::InvalidSchedule { report } => {
                write!(
                    formatter,
                    "generated schedule failed validation with {} diagnostic(s)",
                    report.diagnostics.len()
                )
            }
        }
    }
}

impl std::error::Error for SchedulingError {}

// =============================================================================
// Scheduler
// =============================================================================

/// Production hardware scheduler.
///
/// The scheduler is stateless. All input required for scheduling is supplied
/// through the workload, constraints and configuration.
///
/// This makes it safe to instantiate independently and share by immutable
/// reference between execution pipelines.
#[derive(Debug, Clone)]
pub struct HardwareScheduler {
    config: SchedulerConfig,
}

impl HardwareScheduler {
    /// Creates a production scheduler.
    pub fn new(config: SchedulerConfig) -> Self {
        Self { config }
    }

    /// Returns scheduler configuration.
    pub fn config(&self) -> &SchedulerConfig {
        &self.config
    }

    /// Schedules a workload.
    pub fn schedule(
        &self,
        operations: &[SchedulingOperation],
        constraints: &SchedulingConstraints,
    ) -> Result<Schedule, SchedulingError> {
        validate_constraints(constraints)?;
        validate_operations(operations, constraints)?;

        if operations.len() > MAX_SCHEDULE_OPERATIONS {
            return Err(SchedulingError::TooManyOperations {
                count: operations.len(),
                maximum: MAX_SCHEDULE_OPERATIONS,
            });
        }

        if operations.is_empty() {
            return Ok(Schedule {
                schema_version: SCHEDULING_SCHEMA_VERSION,
                schema_id: SCHEDULING_SCHEMA_ID,
                operations: Vec::new(),
                makespan: Duration::ZERO,
                peak_parallelism: 0,
                resource_count: 0,
            });
        }

        let operation_map = build_operation_map(operations)?;
        let dependency_map = build_dependency_map(operations)?;

        let mut scheduled: BTreeMap<OperationId, ScheduledOperation> = BTreeMap::new();
        let mut remaining: BTreeSet<OperationId> =
            operations.iter().map(|operation| operation.id).collect();

        let mut iterations = 0usize;

        while !remaining.is_empty() {
            iterations = iterations
                .checked_add(1)
                .ok_or(SchedulingError::IterationLimitExceeded)?;

            if iterations > self.config.maximum_iterations {
                return Err(SchedulingError::IterationLimitExceeded);
            }

            let ready = ready_operations(
                &remaining,
                &dependency_map,
                &scheduled,
            );

            if ready.is_empty() {
                return Err(SchedulingError::DependencyCycle);
            }

            let mut candidates: Vec<&SchedulingOperation> = ready
                .iter()
                .filter_map(|id| operation_map.get(id).copied())
                .collect();

            order_candidates(&mut candidates, self.config.policy, &dependency_map);

            let mut progressed = false;

            for operation in candidates {
                let start = calculate_earliest_start(
                    operation,
                    &scheduled,
                    constraints,
                    self.config.deterministic,
                )?;

                let end = start
                    .checked_add(operation.duration)
                    .ok_or(SchedulingError::ArithmeticOverflow)?;

                if let Some(latest) = operation.latest_start {
                    if start > latest {
                        return Err(SchedulingError::LatestStartExceeded {
                            operation: operation.id,
                        });
                    }
                }

                let scheduled_operation = ScheduledOperation {
                    operation: operation.id,
                    start,
                    end,
                    resources: operation.resources.clone(),
                    class: operation.class,
                };

                scheduled.insert(operation.id, scheduled_operation);
                remaining.remove(&operation.id);
                progressed = true;

                if !constraints.allow_parallelism {
                    break;
                }
            }

            if !progressed {
                return Err(SchedulingError::IterationLimitExceeded);
            }
        }

        let schedule = build_schedule(&scheduled)?;

        if self.config.validate_result {
            let report = validate_schedule(&schedule, operations, constraints);

            if !report.is_valid() {
                return Err(SchedulingError::InvalidSchedule { report });
            }
        }

        if let Some(maximum) = constraints.maximum_makespan {
            if schedule.makespan > maximum {
                return Err(SchedulingError::MakespanExceeded {
                    actual: schedule.makespan,
                    maximum,
                });
            }
        }

        Ok(schedule)
    }

    /// Validates an already produced schedule.
    pub fn validate(
        &self,
        schedule: &Schedule,
        operations: &[SchedulingOperation],
        constraints: &SchedulingConstraints,
    ) -> SchedulingReport {
        validate_schedule(schedule, operations, constraints)
    }

    /// Computes deterministic schedule statistics.
    pub fn statistics(
        &self,
        schedule: &Schedule,
        operations: &[SchedulingOperation],
    ) -> ScheduleStatistics {
        let earliest_by_operation: BTreeMap<OperationId, TimePoint> = operations
            .iter()
            .filter_map(|operation| {
                operation
                    .earliest_start
                    .map(|start| (operation.id, start))
            })
            .collect();

        let mut delayed_operations = 0usize;
        let mut operations_with_idle_gap = 0usize;

        for operation in &schedule.operations {
            if let Some(earliest) = earliest_by_operation.get(&operation.operation) {
                if operation.start > *earliest {
                    delayed_operations += 1;
                }
            }

            if !operation.start == TimePoint::ZERO {
                operations_with_idle_gap += 1;
            }
        }

        ScheduleStatistics {
            operation_count: schedule.operation_count(),
            resource_count: schedule.resource_count,
            makespan: schedule.makespan,
            peak_parallelism: schedule.peak_parallelism,
            delayed_operations,
            operations_with_idle_gap,
        }
    }
}

// =============================================================================
// Workload validation
// =============================================================================

fn validate_operation_name(name: &str) -> Result<(), SchedulingError> {
    if name.trim().is_empty() {
        return Err(SchedulingError::EmptyOperationName {
            operation: OperationId::new(0),
        });
    }

    Ok(())
}

fn validate_operations(
    operations: &[SchedulingOperation],
    constraints: &SchedulingConstraints,
) -> Result<(), SchedulingError> {
    if operations.len() > MAX_SCHEDULE_OPERATIONS {
        return Err(SchedulingError::TooManyOperations {
            count: operations.len(),
            maximum: MAX_SCHEDULE_OPERATIONS,
        });
    }

    let mut ids = BTreeSet::new();
    let mut resources = BTreeSet::new();
    let mut dependency_count = 0usize;

    for operation in operations {
        if !ids.insert(operation.id) {
            return Err(SchedulingError::DuplicateOperation {
                operation: operation.id,
            });
        }

        if operation.name.trim().is_empty() {
            return Err(SchedulingError::EmptyOperationName {
                operation: operation.id,
            });
        }

        if operation.resources.is_empty() {
            return Err(SchedulingError::OperationHasNoResources {
                operation: operation.id,
            });
        }

        if operation.resources.len() > MAX_OPERATION_RESOURCES {
            return Err(SchedulingError::TooManyResources {
                operation: operation.id,
                count: operation.resources.len(),
                maximum: MAX_OPERATION_RESOURCES,
            });
        }

        for pair in operation.resources.windows(2) {
            if pair[0] >= pair[1] {
                return Err(SchedulingError::DuplicateOperationResource {
                    operation: operation.id,
                });
            }
        }

        resources.extend(operation.resources.iter().copied());

        dependency_count = dependency_count
            .checked_add(operation.dependencies.len())
            .ok_or(SchedulingError::TooManyDependencies {
                count: usize::MAX,
                maximum: MAX_DEPENDENCY_EDGES,
            })?;

        if dependency_count > MAX_DEPENDENCY_EDGES {
            return Err(SchedulingError::TooManyDependencies {
                count: dependency_count,
                maximum: MAX_DEPENDENCY_EDGES,
            });
        }

        let mut dependency_set = BTreeSet::new();

        for dependency in &operation.dependencies {
            if *dependency == operation.id {
                return Err(SchedulingError::DependencyCycle);
            }

            if !dependency_set.insert(*dependency) {
                return Err(SchedulingError::UnknownDependency {
                    operation: operation.id,
                    dependency: *dependency,
                });
            }

            if !ids.contains(dependency)
                && !operations.iter().any(|candidate| candidate.id == *dependency)
            {
                return Err(SchedulingError::UnknownDependency {
                    operation: operation.id,
                    dependency: *dependency,
                });
            }
        }
    }

    if resources.len() > MAX_SCHEDULE_RESOURCES {
        return Err(SchedulingError::TooManyResourcesInWorkload {
            count: resources.len(),
            maximum: MAX_SCHEDULE_RESOURCES,
        });
    }

    if constraints.require_declared_resources {
        for operation in operations {
            for resource in &operation.resources {
                if !constraints.resources.contains_key(resource) {
                    return Err(SchedulingError::UndeclaredResource {
                        resource: *resource,
                        operation: operation.id,
                    });
                }
            }
        }
    }

    Ok(())
}

fn validate_constraints(
    constraints: &SchedulingConstraints,
) -> Result<(), SchedulingError> {
    if let Some(alignment) = constraints.global_alignment {
        if alignment.is_zero() {
            return Err(SchedulingError::InvalidAlignment);
        }
    }

    for resource in constraints.resources.values() {
        if let Some(alignment) = resource.start_alignment {
            if alignment.is_zero() {
                return Err(SchedulingError::InvalidAlignment);
            }
        }

        if let Some(limit) = resource.concurrency_limit {
            if limit == 0 {
                return Err(SchedulingError::InvalidConcurrencyLimit);
            }
        }
    }

    Ok(())
}

// =============================================================================
// Dependency graph
// =============================================================================

fn build_operation_map<'a>(
    operations: &'a [SchedulingOperation],
) -> Result<BTreeMap<OperationId, &'a SchedulingOperation>, SchedulingError> {
    let mut map = BTreeMap::new();

    for operation in operations {
        if map.insert(operation.id, operation).is_some() {
            return Err(SchedulingError::DuplicateOperation {
                operation: operation.id,
            });
        }
    }

    Ok(map)
}

fn build_dependency_map(
    operations: &[SchedulingOperation],
) -> Result<BTreeMap<OperationId, Vec<OperationId>>, SchedulingError> {
    let known: BTreeSet<OperationId> =
        operations.iter().map(|operation| operation.id).collect();

    let mut map = BTreeMap::new();

    for operation in operations {
        let mut dependencies = operation.dependencies.clone();
        dependencies.sort_unstable();

        for dependency in &dependencies {
            if !known.contains(dependency) {
                return Err(SchedulingError::UnknownDependency {
                    operation: operation.id,
                    dependency: *dependency,
                });
            }
        }

        dependencies.dedup();

        map.insert(operation.id, dependencies);
    }

    Ok(map)
}

fn ready_operations(
    remaining: &BTreeSet<OperationId>,
    dependencies: &BTreeMap<OperationId, Vec<OperationId>>,
    scheduled: &BTreeMap<OperationId, ScheduledOperation>,
) -> Vec<OperationId> {
    remaining
        .iter()
        .copied()
        .filter(|id| {
            dependencies
                .get(id)
                .map(|deps| deps.iter().all(|dependency| scheduled.contains_key(dependency)))
                .unwrap_or(true)
        })
        .collect()
}

// =============================================================================
// Candidate ordering
// =============================================================================

fn order_candidates(
    candidates: &mut Vec<&SchedulingOperation>,
    policy: SchedulingPolicy,
    dependencies: &BTreeMap<OperationId, Vec<OperationId>>,
) {
    candidates.sort_by(|left, right| {
        let left_criticality = criticality(left.id, dependencies, 0);
        let right_criticality = criticality(right.id, dependencies, 0);

        match policy {
            SchedulingPolicy::EarliestStart => left
                .priority
                .cmp(&right.priority)
                .reverse()
                .then_with(|| left.id.cmp(&right.id)),

            SchedulingPolicy::CriticalPath => right_criticality
                .cmp(&left_criticality)
                .then_with(|| right.priority.cmp(&left.priority))
                .then_with(|| left.id.cmp(&right.id)),

            SchedulingPolicy::LatencyAware => right
                .latency_sensitive
                .cmp(&left.latency_sensitive)
                .then_with(|| right.priority.cmp(&left.priority))
                .then_with(|| left.id.cmp(&right.id)),

            SchedulingPolicy::Hybrid => right
                .latency_sensitive
                .cmp(&left.latency_sensitive)
                .then_with(|| right_criticality.cmp(&left_criticality))
                .then_with(|| right.priority.cmp(&left.priority))
                .then_with(|| left.id.cmp(&right.id)),
        }
    });
}

fn criticality(
    operation: OperationId,
    dependencies: &BTreeMap<OperationId, Vec<OperationId>>,
    depth: usize,
) -> usize {
    if depth > 1024 {
        return 1024;
    }

    dependencies
        .get(&operation)
        .map(|deps| {
            deps.iter()
                .map(|dependency| {
                    1usize.saturating_add(criticality(*dependency, dependencies, depth + 1))
                })
                .max()
                .unwrap_or(0)
        })
        .unwrap_or(0)
}

// =============================================================================
// Start-time calculation
// =============================================================================

fn calculate_earliest_start(
    operation: &SchedulingOperation,
    scheduled: &BTreeMap<OperationId, ScheduledOperation>,
    constraints: &SchedulingConstraints,
    _deterministic: bool,
) -> Result<TimePoint, SchedulingError> {
    let mut start = operation.earliest_start.unwrap_or(TimePoint::ZERO);

    for dependency in &operation.dependencies {
        let dependency_operation = scheduled
            .get(dependency)
            .ok_or(SchedulingError::UnknownDependency {
                operation: operation.id,
                dependency: *dependency,
            })?;

        start = max_time(start, dependency_operation.end)?;
    }

    for resource in &operation.resources {
        if let Some(resource_constraint) = constraints.resources.get(resource) {
            if resource_constraint.exclusive {
                for existing in scheduled.values() {
                    if existing.resources.binary_search(resource).is_ok() {
                        let guard_end = existing
                            .end
                            .checked_add(resource_constraint.guard_time)
                            .ok_or(SchedulingError::ArithmeticOverflow)?;

                        start = max_time(start, guard_end)?;
                    }
                }
            }
        }
    }

    if let Some(group_id) = operation.crosstalk_group {
        if let Some(group) = constraints.crosstalk_groups.get(&group_id) {
            if group.exclusive {
                for existing in scheduled.values() {
                    if existing
                        .resources
                        .iter()
                        .any(|resource| group.resources.contains(resource))
                    {
                        let guard_end = existing
                            .end
                            .checked_add(group.guard_time)
                            .ok_or(SchedulingError::ArithmeticOverflow)?;

                        start = max_time(start, guard_end)?;
                    }
                }
            }
        }
    }

    if operation.class == OperationClass::Measurement {
        start = align_time(
            start,
            constraints.global_alignment,
        )?;
    }

    if operation.class == OperationClass::Reset {
        start = align_time(
            start,
            constraints.global_alignment,
        )?;
    }

    if operation.class == OperationClass::ClassicalControl {
        let mut latest_measurement_end = start;

        for existing in scheduled.values() {
            if existing.class == OperationClass::Measurement
                && existing.end > latest_measurement_end
            {
                latest_measurement_end = existing.end;
            }
        }

        latest_measurement_end = latest_measurement_end
            .checked_add(constraints.measurement_to_control_latency)
            .ok_or(SchedulingError::ArithmeticOverflow)?;

        start = max_time(start, latest_measurement_end)?;
    }

    for resource in &operation.resources {
        if let Some(resource_constraint) = constraints.resources.get(resource) {
            start = align_time(start, resource_constraint.start_alignment)?;
        }
    }

    start = align_time(start, constraints.global_alignment)?;

    if !constraints.allow_parallelism {
        for existing in scheduled.values() {
            start = max_time(start, existing.end)?;
        }
    }

    Ok(start)
}

fn max_time(left: TimePoint, right: TimePoint) -> Result<TimePoint, SchedulingError> {
    Ok(if left >= right { left } else { right })
}

fn align_time(
    time: TimePoint,
    alignment: Option<Duration>,
) -> Result<TimePoint, SchedulingError> {
    let Some(alignment) = alignment else {
        return Ok(time);
    };

    let unit = alignment.as_attoseconds();

    if unit == 0 {
        return Err(SchedulingError::InvalidAlignment);
    }

    let value = time.as_attoseconds();

    let remainder = value % unit;

    if remainder == 0 {
        return Ok(time);
    }

    let increment = unit
        .checked_sub(remainder)
        .ok_or(SchedulingError::ArithmeticOverflow)?;

    let aligned = value
        .checked_add(increment)
        .ok_or(SchedulingError::ArithmeticOverflow)?;

    TimePoint::from_attoseconds(aligned)
        .ok_or(SchedulingError::ArithmeticOverflow)
}

// =============================================================================
// Schedule construction
// =============================================================================

fn build_schedule(
    scheduled: &BTreeMap<OperationId, ScheduledOperation>,
) -> Result<Schedule, SchedulingError> {
    let mut operations: Vec<ScheduledOperation> =
        scheduled.values().cloned().collect();

    operations.sort_by(|left, right| {
        left.start
            .cmp(&right.start)
            .then_with(|| left.end.cmp(&right.end))
            .then_with(|| left.operation.cmp(&right.operation))
    });

    let makespan = operations
        .iter()
        .map(|operation| operation.end)
        .max()
        .unwrap_or(TimePoint::ZERO)
        .checked_duration_since(TimePoint::ZERO)
        .ok_or(SchedulingError::ArithmeticOverflow)?;

    let resource_count = operations
        .iter()
        .flat_map(|operation| operation.resources.iter().copied())
        .collect::<BTreeSet<_>>()
        .len();

    let peak_parallelism = calculate_peak_parallelism(&operations);

    Ok(Schedule {
        schema_version: SCHEDULING_SCHEMA_VERSION,
        schema_id: SCHEDULING_SCHEMA_ID,
        operations,
        makespan,
        peak_parallelism,
        resource_count,
    })
}

fn calculate_peak_parallelism(operations: &[ScheduledOperation]) -> usize {
    let mut events: Vec<(TimePoint, bool)> = Vec::with_capacity(operations.len() * 2);

    for operation in operations {
        events.push((operation.start, true));
        events.push((operation.end, false));
    }

    events.sort_by(|left, right| {
        left.0
            .cmp(&right.0)
            // End events occur before start events at the same point.
            .then_with(|| left.1.cmp(&right.1))
    });

    let mut current = 0usize;
    let mut peak = 0usize;

    for (_, start) in events {
        if start {
            current = current.saturating_add(1);
            peak = peak.max(current);
        } else {
            current = current.saturating_sub(1);
        }
    }

    peak
}

// =============================================================================
// Schedule validation
// =============================================================================

/// Validates a schedule independently from the scheduler implementation.
pub fn validate_schedule(
    schedule: &Schedule,
    operations: &[SchedulingOperation],
    constraints: &SchedulingConstraints,
) -> SchedulingReport {
    let mut report = SchedulingReport::new();

    let operation_map: BTreeMap<OperationId, &SchedulingOperation> =
        operations.iter().map(|operation| (operation.id, operation)).collect();

    let mut seen = BTreeSet::new();

    for scheduled in &schedule.operations {
        if !seen.insert(scheduled.operation) {
            report.push(
                SchedulingDiagnostic::new(
                    SchedulingCode::DuplicateOperation,
                    SchedulingSeverity::Fatal,
                    "operation appears more than once in schedule",
                )
                .with_operation(scheduled.operation),
            );
            continue;
        }

        let Some(operation) = operation_map.get(&scheduled.operation) else {
            report.push(
                SchedulingDiagnostic::new(
                    SchedulingCode::UnknownDependency,
                    SchedulingSeverity::Fatal,
                    "schedule references an operation absent from workload",
                )
                .with_operation(scheduled.operation),
            );
            continue;
        };

        if scheduled.end < scheduled.start {
            report.push(
                SchedulingDiagnostic::new(
                    SchedulingCode::NegativeInterval,
                    SchedulingSeverity::Fatal,
                    "scheduled operation has an end before its start",
                )
                .with_operation(scheduled.operation),
            );
            continue;
        }

        let expected_end = scheduled.start.checked_add(operation.duration);

        match expected_end {
            Some(expected_end) if expected_end == scheduled.end => {}
            _ => report.push(
                SchedulingDiagnostic::new(
                    SchedulingCode::DurationMismatch,
                    SchedulingSeverity::Fatal,
                    "scheduled interval does not equal operation duration",
                )
                .with_operation(scheduled.operation),
            ),
        }

        if let Some(earliest) = operation.earliest_start {
            if scheduled.start < earliest {
                report.push(
                    SchedulingDiagnostic::new(
                        SchedulingCode::EarliestStartViolation,
                        SchedulingSeverity::Error,
                        "operation starts before its earliest legal start",
                    )
                    .with_operation(scheduled.operation),
                );
            }
        }

        if let Some(latest) = operation.latest_start {
            if scheduled.start > latest {
                report.push(
                    SchedulingDiagnostic::new(
                        SchedulingCode::LatestStartViolation,
                        SchedulingSeverity::Error,
                        "operation starts after its latest legal start",
                    )
                    .with_operation(scheduled.operation),
                );
            }
        }

        for resource in &scheduled.resources {
            if constraints.require_declared_resources
                && !constraints.resources.contains_key(resource)
            {
                report.push(
                    SchedulingDiagnostic::new(
                        SchedulingCode::ResourceUndeclared,
                        SchedulingSeverity::Error,
                        "scheduled operation uses an undeclared resource",
                    )
                    .with_operation(scheduled.operation)
                    .with_resource(*resource),
                );
            }
        }
    }

    // Dependency timing.
    for operation in operations {
        let Some(current) = schedule.get(operation.id) else {
            continue;
        };

        for dependency in &operation.dependencies {
            let Some(previous) = schedule.get(*dependency) else {
                report.push(
                    SchedulingDiagnostic::new(
                        SchedulingCode::UnknownDependency,
                        SchedulingSeverity::Error,
                        "dependency is absent from schedule",
                    )
                    .with_operation(operation.id),
                );
                continue;
            };

            if previous.end > current.start {
                report.push(
                    SchedulingDiagnostic::new(
                        SchedulingCode::DependencyTimingViolation,
                        SchedulingSeverity::Error,
                        "operation begins before a dependency completes",
                    )
                    .with_operation(operation.id)
                    .with_conflict(*dependency),
                );
            }
        }
    }

    // Pairwise resource conflict validation.
    for left_index in 0..schedule.operations.len() {
        for right_index in (left_index + 1)..schedule.operations.len() {
            let left = &schedule.operations[left_index];
            let right = &schedule.operations[right_index];

            if !intervals_overlap(left.start, left.end, right.start, right.end) {
                continue;
            }

            let shared_resource = left
                .resources
                .iter()
                .find(|resource| right.resources.binary_search(resource).is_ok())
                .copied();

            if let Some(resource) = shared_resource {
                let exclusive = constraints
                    .resources
                    .get(&resource)
                    .map(|constraint| constraint.exclusive)
                    .unwrap_or(constraints.require_declared_resources);

                if exclusive {
                    report.push(
                        SchedulingDiagnostic::new(
                            SchedulingCode::ResourceConflict,
                            SchedulingSeverity::Error,
                            "two exclusive operations overlap on one resource",
                        )
                        .with_operation(left.operation)
                        .with_resource(resource)
                        .with_conflict(right.operation),
                    );
                }
            }

            for group in constraints.crosstalk_groups.values() {
                if !group.exclusive {
                    continue;
                }

                let left_in_group = left
                    .resources
                    .iter()
                    .any(|resource| group.resources.contains(resource));

                let right_in_group = right
                    .resources
                    .iter()
                    .any(|resource| group.resources.contains(resource));

                if left_in_group && right_in_group {
                    report.push(
                        SchedulingDiagnostic::new(
                            SchedulingCode::CrosstalkConflict,
                            SchedulingSeverity::Error,
                            "operations overlap inside an exclusive crosstalk group",
                        )
                        .with_operation(left.operation)
                        .with_conflict(right.operation),
                    );
                }
            }
        }
    }

    report
}

fn intervals_overlap(
    left_start: TimePoint,
    left_end: TimePoint,
    right_start: TimePoint,
    right_end: TimePoint,
) -> bool {
    left_start < right_end && right_start < left_end
}

// =============================================================================
// Public convenience API
// =============================================================================

/// Schedules a workload using production defaults.
pub fn schedule(
    operations: &[SchedulingOperation],
    constraints: &SchedulingConstraints,
) -> Result<Schedule, SchedulingError> {
    HardwareScheduler::new(SchedulerConfig::production())
        .schedule(operations, constraints)
}

/// Validates a schedule using the provider-neutral validation contract.
pub fn validate(
    schedule: &Schedule,
    operations: &[SchedulingOperation],
    constraints: &SchedulingConstraints,
) -> SchedulingReport {
    validate_schedule(schedule, operations, constraints)
}

/// Computes schedule statistics.
pub fn statistics(
    schedule: &Schedule,
    operations: &[SchedulingOperation],
) -> ScheduleStatistics {
    HardwareScheduler::new(SchedulerConfig::production()).statistics(schedule, operations)
}

// =============================================================================
// Integration adapter contracts
// =============================================================================

/// Trait for converting authoritative hardware timing information into
/// scheduler constraints.
///
/// This trait deliberately lives here as an integration contract rather than
/// importing a future `timing.rs` module. That keeps this file independently
/// compilable and frozen.
///
/// `hardware::timing` can implement this trait later.
pub trait SchedulingTimingProvider {
    /// Returns the global hardware scheduling alignment.
    fn global_alignment(&self) -> Option<Duration>;

    /// Returns resource-specific timing constraints.
    fn resource_constraint(
        &self,
        resource: ResourceId,
    ) -> Option<ResourceConstraint>;

    /// Returns measurement-to-classical-control latency.
    fn measurement_to_control_latency(&self) -> Duration;

    /// Returns reset latency.
    fn reset_latency(&self) -> Duration;

    /// Returns measurement reuse latency.
    fn measurement_latency(&self) -> Duration;
}

/// Builds scheduler constraints from a timing provider.
pub fn constraints_from_timing_provider<T>(
    provider: &T,
    resources: &[ResourceId],
) -> Result<SchedulingConstraints, SchedulingError>
where
    T: SchedulingTimingProvider,
{
    let mut constraints = SchedulingConstraints::new()
        .with_global_alignment(
            provider
                .global_alignment()
                .unwrap_or(Duration::ZERO),
        )
        .with_measurement_to_control_latency(
            provider.measurement_to_control_latency(),
        )
        .with_reset_latency(provider.reset_latency())
        .with_measurement_latency(provider.measurement_latency());

    for resource in resources {
        if let Some(constraint) = provider.resource_constraint(*resource) {
            constraints = constraints.with_resource(constraint)?;
        } else {
            constraints = constraints.with_resource(ResourceConstraint::new(*resource))?;
        }
    }

    Ok(constraints)
}

/// Trait for supplying scheduling resources from a hardware topology.
///
/// `hardware::topology` can implement this contract later without modifying
/// this scheduler.
pub trait SchedulingTopologyProvider {
    /// Returns physical resources available for scheduling.
    fn resources(&self) -> Vec<ResourceId>;

    /// Returns whether two resources may participate in the same operation.
    ///
    /// This is informational for scheduling. The actual routing decision
    /// remains outside this module.
    fn supports_operation(
        &self,
        resources: &[ResourceId],
    ) -> bool;
}

/// Validates that all operations can use the supplied topology.
pub fn validate_against_topology<T>(
    topology: &T,
    operations: &[SchedulingOperation],
) -> SchedulingReport
where
    T: SchedulingTopologyProvider,
{
    let available: BTreeSet<ResourceId> =
        topology.resources().into_iter().collect();

    let mut report = SchedulingReport::new();

    for operation in operations {
        for resource in &operation.resources {
            if !available.contains(resource) {
                report.push(
                    SchedulingDiagnostic::new(
                        SchedulingCode::ResourceUndeclared,
                        SchedulingSeverity::Error,
                        "operation references a resource unavailable in topology",
                    )
                    .with_operation(operation.id)
                    .with_resource(*resource),
                );
            }
        }

        if !topology.supports_operation(&operation.resources) {
            report.push(
                SchedulingDiagnostic::new(
                    SchedulingCode::InvalidConstraint,
                    SchedulingSeverity::Error,
                    "topology does not permit the operation resource combination",
                )
                .with_operation(operation.id),
            );
        }
    }

    report
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn ns(value: u64) -> Duration {
        Duration::from_nanoseconds(value).expect("valid duration")
    }

    fn resource(value: u32) -> ResourceId {
        ResourceId::new(value)
    }

    fn op(
        id: u64,
        name: &str,
        resource_id: u32,
        duration_ns: u64,
    ) -> SchedulingOperation {
        SchedulingOperation::new(
            OperationId::new(id),
            OperationClass::Gate,
            name,
            vec![resource(resource_id)],
            ns(duration_ns),
        )
        .expect("valid operation")
    }

    #[test]
    fn duration_units_are_exact() {
        assert_eq!(
            Duration::from_nanoseconds(1)
                .expect("valid")
                .as_attoseconds(),
            1_000_000_000_000_000
        );

        assert_eq!(
            Duration::from_picoseconds(1)
                .expect("valid")
                .as_attoseconds(),
            1_000_000_000_000
        );
    }

    #[test]
    fn duration_addition_is_checked() {
        let left = ns(10);
        let right = ns(20);

        assert_eq!(
            left.checked_add(right)
                .expect("addition")
                .as_attoseconds(),
            ns(30).as_attoseconds()
        );
    }

    #[test]
    fn independent_operations_can_run_in_parallel() {
        let operations = vec![
            op(0, "h", 0, 10),
            op(1, "x", 1, 20),
        ];

        let constraints = SchedulingConstraints::new()
            .with_resource(ResourceConstraint::new(resource(0)))
            .expect("resource")
            .with_resource(ResourceConstraint::new(resource(1)))
            .expect("resource");

        let result = schedule(&operations, &constraints).expect("schedule");

        assert_eq!(result.operation_count(), 2);
        assert_eq!(result.makespan, ns(20));
        assert_eq!(result.peak_parallelism, 2);
    }

    #[test]
    fn same_resource_operations_are_serialized() {
        let operations = vec![
            op(0, "h", 0, 10),
            op(1, "x", 0, 20),
        ];

        let constraints = SchedulingConstraints::new()
            .with_resource(ResourceConstraint::new(resource(0)))
            .expect("resource");

        let result = schedule(&operations, &constraints).expect("schedule");

        let first = result.get(OperationId::new(0)).expect("first");
        let second = result.get(OperationId::new(1)).expect("second");

        assert_eq!(first.start, TimePoint::ZERO);
        assert_eq!(first.end, TimePoint::ZERO.checked_add(ns(10)).expect("time"));
        assert_eq!(second.start, first.end);
        assert_eq!(result.makespan, ns(30));
    }

    #[test]
    fn dependency_is_respected() {
        let operations = vec![
            op(0, "h", 0, 10),
            op(1, "x", 1, 20)
                .with_dependency(OperationId::new(0)),
        ];

        let constraints = SchedulingConstraints::new()
            .with_resource(ResourceConstraint::new(resource(0)))
            .expect("resource")
            .with_resource(ResourceConstraint::new(resource(1)))
            .expect("resource");

        let result = schedule(&operations, &constraints).expect("schedule");

        let first = result.get(OperationId::new(0)).expect("first");
        let second = result.get(OperationId::new(1)).expect("second");

        assert!(second.start >= first.end);
        assert_eq!(result.makespan, ns(30));
    }

    #[test]
    fn global_alignment_is_respected() {
        let operations = vec![
            op(0, "h", 0, 7),
            op(1, "x", 0, 3),
        ];

        let constraints = SchedulingConstraints::new()
            .with_global_alignment(ns(5))
            .with_resource(ResourceConstraint::new(resource(0)))
            .expect("resource");

        let result = schedule(&operations, &constraints).expect("schedule");

        let second = result.get(OperationId::new(1)).expect("second");

        assert_eq!(second.start.as_attoseconds() % ns(5).as_attoseconds(), 0);
    }

    #[test]
    fn resource_guard_time_is_respected() {
        let operations = vec![
            op(0, "h", 0, 10),
            op(1, "x", 0, 10),
        ];

        let constraints = SchedulingConstraints::new()
            .with_resource(
                ResourceConstraint::new(resource(0))
                    .with_guard_time(ns(5)),
            )
            .expect("resource");

        let result = schedule(&operations, &constraints).expect("schedule");

        let first = result.get(OperationId::new(0)).expect("first");
        let second = result.get(OperationId::new(1)).expect("second");

        assert_eq!(second.start, first.end.checked_add(ns(5)).expect("time"));
    }

    #[test]
    fn latest_start_is_enforced() {
        let operations = vec![
            op(0, "h", 0, 10),
            op(1, "x", 0, 10)
                .with_latest_start(TimePoint::ZERO),
        ];

        let constraints = SchedulingConstraints::new()
            .with_resource(ResourceConstraint::new(resource(0)))
            .expect("resource");

        let result = schedule(&operations, &constraints);

        assert!(matches!(
            result,
            Err(SchedulingError::LatestStartExceeded {
                operation: OperationId(1)
            })
        ));
    }

    #[test]
    fn dependency_cycle_is_rejected() {
        let operations = vec![
            op(0, "h", 0, 10)
                .with_dependency(OperationId::new(1)),
            op(1, "x", 0, 10)
                .with_dependency(OperationId::new(0)),
        ];

        let constraints = SchedulingConstraints::new()
            .with_resource(ResourceConstraint::new(resource(0)))
            .expect("resource");

        let result = schedule(&operations, &constraints);

        assert!(matches!(
            result,
            Err(SchedulingError::DependencyCycle)
        ));
    }

    #[test]
    fn unknown_dependency_is_rejected() {
        let operations = vec![
            op(0, "h", 0, 10)
                .with_dependency(OperationId::new(99)),
        ];

        let constraints = SchedulingConstraints::new()
            .with_resource(ResourceConstraint::new(resource(0)))
            .expect("resource");

        let result = schedule(&operations, &constraints);

        assert!(matches!(
            result,
            Err(SchedulingError::UnknownDependency {
                operation: OperationId(0),
                dependency: OperationId(99)
            })
        ));
    }

    #[test]
    fn crosstalk_group_is_enforced() {
        let operations = vec![
            op(0, "cx0", 0, 10)
                .with_crosstalk_group(1),
            op(1, "cx1", 1, 10)
                .with_crosstalk_group(1),
        ];

        let constraints = SchedulingConstraints::new()
            .with_resource(ResourceConstraint::new(resource(0)))
            .expect("resource")
            .with_resource(ResourceConstraint::new(resource(1)))
            .expect("resource")
            .with_crosstalk_group(
                CrosstalkGroup::new(1)
                    .add_resource(resource(0))
                    .add_resource(resource(1)),
            )
            .expect("group");

        let result = schedule(&operations, &constraints).expect("schedule");

        assert_eq!(result.makespan, ns(20));
    }

    #[test]
    fn parallelism_can_be_disabled() {
        let operations = vec![
            op(0, "h", 0, 10),
            op(1, "x", 1, 20),
        ];

        let constraints = SchedulingConstraints::new()
            .with_parallelism(false)
            .with_resource(ResourceConstraint::new(resource(0)))
            .expect("resource")
            .with_resource(ResourceConstraint::new(resource(1)))
            .expect("resource");

        let result = schedule(&operations, &constraints).expect("schedule");

        assert_eq!(result.makespan, ns(30));
        assert_eq!(result.peak_parallelism, 1);
    }

    #[test]
    fn schedule_validation_accepts_valid_schedule() {
        let operations = vec![
            op(0, "h", 0, 10),
            op(1, "x", 0, 20),
        ];

        let constraints = SchedulingConstraints::new()
            .with_resource(ResourceConstraint::new(resource(0)))
            .expect("resource");

        let schedule_result = schedule(&operations, &constraints).expect("schedule");

        let report = validate(&schedule_result, &operations, &constraints);

        assert!(report.is_valid());
        assert!(report.diagnostics.is_empty());
    }

    #[test]
    fn deterministic_schedule_is_reproducible() {
        let operations = vec![
            op(2, "x", 2, 10),
            op(0, "h", 0, 10),
            op(1, "x", 1, 10),
        ];

        let constraints = SchedulingConstraints::new()
            .with_resource(ResourceConstraint::new(resource(0)))
            .expect("resource")
            .with_resource(ResourceConstraint::new(resource(1)))
            .expect("resource")
            .with_resource(ResourceConstraint::new(resource(2)))
            .expect("resource");

        let first = schedule(&operations, &constraints).expect("first");
        let second = schedule(&operations, &constraints).expect("second");

        assert_eq!(first, second);
    }

    #[test]
    fn empty_workload_is_valid() {
        let constraints = SchedulingConstraints::new();

        let result = schedule(&[], &constraints).expect("empty schedule");

        assert!(result.is_empty());
        assert_eq!(result.makespan, Duration::ZERO);
    }

    #[test]
    fn resource_alignment_is_respected() {
        let operations = vec![
            op(0, "h", 0, 7),
            op(1, "x", 0, 3),
        ];

        let constraints = SchedulingConstraints::new()
            .with_resource(
                ResourceConstraint::new(resource(0))
                    .with_alignment(ns(10)),
            )
            .expect("resource");

        let result = schedule(&operations, &constraints).expect("schedule");

        let second = result.get(OperationId::new(1)).expect("second");

        assert_eq!(
            second.start.as_attoseconds() % ns(10).as_attoseconds(),
            0
        );
    }

    #[test]
    fn statistics_are_deterministic() {
        let operations = vec![
            op(0, "h", 0, 10),
            op(1, "x", 1, 20),
        ];

        let constraints = SchedulingConstraints::new()
            .with_resource(ResourceConstraint::new(resource(0)))
            .expect("resource")
            .with_resource(ResourceConstraint::new(resource(1)))
            .expect("resource");

        let scheduler = HardwareScheduler::new(SchedulerConfig::production());
        let result = scheduler
            .schedule(&operations, &constraints)
            .expect("schedule");

        let statistics = scheduler.statistics(&result, &operations);

        assert_eq!(statistics.operation_count, 2);
        assert_eq!(statistics.resource_count, 2);
        assert_eq!(statistics.makespan, ns(20));
        assert_eq!(statistics.peak_parallelism, 2);
    }
}