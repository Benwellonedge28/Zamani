//! Zamani Quantum Scheduling — Immutable Scheduling Context
//!
//! This module defines the immutable, target-independent input boundary for
//! the production quantum scheduler.
//!
//! # Architectural role
//!
//! `SchedulingContext` answers:
//!
//! > What information is the scheduler allowed to use when deciding WHEN
//! > already-defined quantum operations should execute?
//!
//! It does NOT:
//!
//! - define quantum operation semantics;
//! - define a second `QubitId`;
//! - perform logical-to-physical routing;
//! - discover hardware;
//! - contact a QPU;
//! - execute a circuit;
//! - implement a scheduling algorithm;
//! - mutate canonical IR;
//! - own calibration acquisition;
//! - own QEC decoding;
//! - own noise modelling.
//!
//! The intended dependency direction is:
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
//!      ├──────────────► optimization
//!      │
//!      ▼
//! quantum::routing
//!      │
//!      ▼
//! SchedulingContext
//!      │
//!      ├── executable scheduling tasks
//!      ├── target snapshot
//!      ├── timing model
//!      ├── resource model
//!      ├── constraints
//!      ├── policy
//!      ├── objective
//!      ├── limits
//!      ├── QEC requirements
//!      ├── dynamic-control requirements
//!      └── reproducibility information
//!      │
//!      ▼
//! scheduling planner / algorithm
//!      │
//!      ▼
//! ScheduleResult
//!      │
//!      ├── verification
//!      ├── hardware lowering
//!      └── execution
//! ```
//!
//! # Write once, scale everywhere
//!
//! A Zamani program must not encode:
//!
//! - a fixed number of qubits;
//! - a fixed number of operations;
//! - a fixed number of control channels;
//! - a fixed topology;
//! - a fixed timing grid;
//! - a fixed QEC distance;
//! - a fixed machine size;
//! - a fixed vendor;
//! - a fixed quantum technology.
//!
//! `SchedulingContext` therefore describes the execution target through
//! capabilities and resources rather than compile-time machine constants.
//!
//! "Infinity" means that no artificial finite machine-size ceiling is encoded
//! by this module. Actual executions remain finite because available memory,
//! address space, execution time and physical resources are finite.
//!
//! # Canonical qubit identity
//!
//! This file intentionally uses:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! No scheduler-specific qubit identity is introduced.
//!
//! # Immutability
//!
//! A context is a snapshot.
//!
//! Once constructed, its scheduling inputs cannot be changed through ordinary
//! methods. This gives schedulers deterministic inputs and makes concurrent
//! read-only use possible without synchronization.
//!
//! If hardware calibration or availability changes, construct a new context.
//! Do not mutate an existing context underneath a running scheduler.
//!
//! # Determinism
//!
//! Determinism is controlled by the context itself:
//!
//! - deterministic mode is explicit;
//! - the random seed is explicit when randomized strategies are allowed;
//! - ordered collections are used where ordering affects semantics;
//! - target snapshots have stable identities;
//! - context fingerprints can be used for reproducibility.
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
//! The safety boundary is compiler-enforced with `forbid(unsafe_code)`.
//!
//! # Integration contract
//!
//! Upstream providers:
//!
//! - `quantum::ir`
//! - `quantum::routing`
//! - `quantum::hardware`
//! - `quantum::zqn`
//! - `quantum::error_correction`
//!
//! Downstream consumers:
//!
//! - `scheduling::planners`
//! - `scheduling::algorithms`
//! - `scheduling::verification`
//! - `scheduling::optimization`
//! - `scheduling::diagnostics`
//! - `scheduling::serialization`
//!
//! This file contains no dependency on a particular scheduling algorithm.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::num::NonZeroU64;
use std::sync::Arc;

use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId};

// =============================================================================
// Public context error
// =============================================================================

/// Errors produced while constructing or validating a scheduling context.
///
/// Context errors are deliberately separate from scheduler execution errors.
/// A scheduler should never receive a context that has failed validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SchedulingContextError {
    /// A required identifier was empty.
    EmptyIdentifier {
        /// Logical name of the invalid field.
        field: &'static str,
    },

    /// A target capability was declared inconsistently.
    InvalidTarget {
        /// Stable diagnostic.
        message: String,
    },

    /// A resource capacity is invalid.
    InvalidResourceCapacity {
        /// Resource identifier.
        resource: String,
    },

    /// A timing resolution is invalid.
    InvalidTimingResolution {
        /// Stable diagnostic.
        message: String,
    },

    /// An operation references a physical qubit that is not present in the
    /// supplied target snapshot.
    UnknownPhysicalQubit {
        /// Referenced qubit.
        qubit: PhysicalQubitId,
    },

    /// An operation references a logical qubit that is not represented by the
    /// supplied logical program snapshot.
    UnknownLogicalQubit {
        /// Referenced qubit.
        qubit: QubitId,
    },

    /// A deadline is earlier than the context origin.
    InvalidDeadline,

    /// A supplied seed is invalid.
    InvalidSeed,

    /// Context metadata violates an invariant.
    InvalidMetadata {
        /// Metadata key.
        key: String,

        /// Stable diagnostic.
        message: String,
    },
}

impl fmt::Display for SchedulingContextError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::EmptyIdentifier { field } => {
                write!(formatter, "{field} must not be empty")
            }

            Self::InvalidTarget { message } => {
                write!(formatter, "invalid scheduling target: {message}")
            }

            Self::InvalidResourceCapacity { resource } => {
                write!(
                    formatter,
                    "invalid capacity for scheduling resource `{resource}`"
                )
            }

            Self::InvalidTimingResolution { message } => {
                write!(
                    formatter,
                    "invalid scheduling timing resolution: {message}"
                )
            }

            Self::UnknownPhysicalQubit { qubit } => {
                write!(
                    formatter,
                    "scheduling context references unknown physical qubit `{qubit}`"
                )
            }

            Self::UnknownLogicalQubit { qubit } => {
                write!(
                    formatter,
                    "scheduling context references unknown logical qubit `{qubit}`"
                )
            }

            Self::InvalidDeadline => {
                formatter.write_str(
                    "scheduling deadline precedes the context origin",
                )
            }

            Self::InvalidSeed => {
                formatter.write_str("invalid scheduling random seed")
            }

            Self::InvalidMetadata { key, message } => {
                write!(
                    formatter,
                    "invalid scheduling metadata `{key}`: {message}"
                )
            }
        }
    }
}

impl std::error::Error for SchedulingContextError {}

/// Result type for scheduling-context construction.
pub type SchedulingContextResult<T> =
    Result<T, SchedulingContextError>;

// =============================================================================
// Stable identifiers
// =============================================================================

/// Stable identifier for an execution target snapshot.
///
/// A target ID identifies a target description, not a live network connection.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TargetId(Arc<str>);

impl TargetId {
    /// Creates a target identifier.
    pub fn new(
        value: impl Into<Arc<str>>,
    ) -> SchedulingContextResult<Self> {
        let value = value.into();

        if value.is_empty() {
            return Err(SchedulingContextError::EmptyIdentifier {
                field: "target_id",
            });
        }

        Ok(Self(value))
    }

    /// Returns the identifier as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for TargetId {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// Stable identifier for a target snapshot/version.
///
/// A new calibration or availability snapshot should normally receive a new
/// revision rather than mutating a context that is already in use.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TargetRevision(NonZeroU64);

impl TargetRevision {
    /// Creates a target revision.
    #[must_use]
    pub const fn new(value: NonZeroU64) -> Self {
        Self(value)
    }

    /// Returns the numeric revision.
    #[must_use]
    pub const fn get(self) -> u64 {
        self.0.get()
    }
}

/// Stable identifier for a scheduling-context snapshot.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ContextId(Arc<str>);

impl ContextId {
    /// Creates a context identifier.
    pub fn new(
        value: impl Into<Arc<str>>,
    ) -> SchedulingContextResult<Self> {
        let value = value.into();

        if value.is_empty() {
            return Err(SchedulingContextError::EmptyIdentifier {
                field: "context_id",
            });
        }

        Ok(Self(value))
    }

    /// Returns the identifier as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ContextId {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// =============================================================================
// Scheduling origin/deadline
// =============================================================================

/// Abstract scheduling clock origin.
///
/// This is deliberately not tied to a hardware wall clock. Hardware-specific
/// timestamps belong to the execution layer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SchedulingEpoch(u64);

impl SchedulingEpoch {
    /// Creates an epoch value.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the raw epoch value.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }
}

impl Default for SchedulingEpoch {
    fn default() -> Self {
        Self(0)
    }
}

/// Optional scheduling deadline.
///
/// The unit is intentionally abstract. A concrete timing domain is supplied
/// by the target timing model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SchedulingDeadline(u64);

impl SchedulingDeadline {
    /// Creates a deadline.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the raw deadline.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }
}

// =============================================================================
// Scheduling mode
// =============================================================================

/// Determines whether the schedule is resolved entirely before execution or
/// may contain runtime-resolved decisions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SchedulingMode {
    /// All schedulable decisions are resolved before execution.
    Static,

    /// Runtime conditions may determine subsequent scheduling decisions.
    Dynamic,

    /// The schedule contains both static and runtime-resolved regions.
    Hybrid,
}

impl Default for SchedulingMode {
    fn default() -> Self {
        Self::Static
    }
}

// =============================================================================
// Determinism
// =============================================================================

/// Reproducibility configuration for scheduling.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Determinism {
    /// Whether scheduler decisions must be deterministic.
    deterministic: bool,

    /// Optional explicit seed for algorithms that use randomness.
    seed: Option<u64>,
}

impl Determinism {
    /// Creates deterministic scheduling with no random seed.
    #[must_use]
    pub const fn deterministic() -> Self {
        Self {
            deterministic: true,
            seed: None,
        }
    }

    /// Creates non-deterministic scheduling.
    #[must_use]
    pub const fn nondeterministic() -> Self {
        Self {
            deterministic: false,
            seed: None,
        }
    }

    /// Creates deterministic scheduling with an explicit algorithm seed.
    #[must_use]
    pub const fn with_seed(seed: u64) -> Self {
        Self {
            deterministic: true,
            seed: Some(seed),
        }
    }

    /// Returns whether deterministic decisions are required.
    #[must_use]
    pub const fn is_deterministic(self) -> bool {
        self.deterministic
    }

    /// Returns the optional seed.
    #[must_use]
    pub const fn seed(self) -> Option<u64> {
        self.seed
    }
}

impl Default for Determinism {
    fn default() -> Self {
        Self::deterministic()
    }
}

// =============================================================================
// Target technology
// =============================================================================

/// Hardware-neutral quantum technology classification.
///
/// This is descriptive metadata only. Scheduling algorithms must not branch
/// on vendor names or technology strings unless the target capability model
/// explicitly requires it.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum QuantumTechnology {
    /// Superconducting technology.
    Superconducting,

    /// Trapped-ion technology.
    TrappedIon,

    /// Neutral-atom technology.
    NeutralAtom,

    /// Photonic technology.
    Photonic,

    /// Spin-based technology.
    Spin,

    /// Topological technology.
    Topological,

    /// Quantum annealing technology.
    Annealing,

    /// Measurement-based quantum computing.
    MeasurementBased,

    /// Continuous-variable quantum computing.
    ContinuousVariable,

    /// Hybrid or heterogeneous target.
    Hybrid,

    /// Target technology not represented by a built-in enum variant.
    Other(Arc<str>),
}

// =============================================================================
// Target capabilities
// =============================================================================

/// Immutable snapshot of the capabilities relevant to scheduling.
///
/// This structure describes what the target *can* expose to the scheduler.
/// It does not represent a connection to the target.
///
/// The fields are deliberately capability-oriented rather than machine-sized.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SchedulingTarget {
    id: TargetId,
    revision: TargetRevision,
    technology: QuantumTechnology,
    logical_qubits: BTreeSet<QubitId>,
    physical_qubits: BTreeSet<PhysicalQubitId>,
    resources: BTreeMap<Arc<str>, ResourceCapability>,
    operations: BTreeMap<Arc<str>, OperationCapability>,
    timing: TimingCapability,
    dynamic_control: DynamicControlCapability,
    distributed: DistributedCapability,
}

impl SchedulingTarget {
    /// Creates a new target capability snapshot.
    #[must_use]
    pub fn new(
        id: TargetId,
        revision: TargetRevision,
        technology: QuantumTechnology,
        timing: TimingCapability,
    ) -> Self {
        Self {
            id,
            revision,
            technology,
            logical_qubits: BTreeSet::new(),
            physical_qubits: BTreeSet::new(),
            resources: BTreeMap::new(),
            operations: BTreeMap::new(),
            timing,
            dynamic_control: DynamicControlCapability::default(),
            distributed: DistributedCapability::default(),
        }
    }

    /// Returns the target identity.
    #[must_use]
    pub fn id(&self) -> &TargetId {
        &self.id
    }

    /// Returns the target snapshot revision.
    #[must_use]
    pub const fn revision(&self) -> TargetRevision {
        self.revision
    }

    /// Returns the technology classification.
    #[must_use]
    pub fn technology(&self) -> &QuantumTechnology {
        &self.technology
    }

    /// Returns all known logical qubits.
    #[must_use]
    pub fn logical_qubits(&self) -> &BTreeSet<QubitId> {
        &self.logical_qubits
    }

    /// Returns all known physical qubits.
    #[must_use]
    pub fn physical_qubits(&self) -> &BTreeSet<PhysicalQubitId> {
        &self.physical_qubits
    }

    /// Returns target resources.
    #[must_use]
    pub fn resources(&self) -> &BTreeMap<Arc<str>, ResourceCapability> {
        &self.resources
    }

    /// Returns operation capabilities.
    #[must_use]
    pub fn operations(&self) -> &BTreeMap<Arc<str>, OperationCapability> {
        &self.operations
    }

    /// Returns timing capabilities.
    #[must_use]
    pub const fn timing(&self) -> &TimingCapability {
        &self.timing
    }

    /// Returns dynamic-control capabilities.
    #[must_use]
    pub const fn dynamic_control(&self) -> &DynamicControlCapability {
        &self.dynamic_control
    }

    /// Returns distributed-computation capabilities.
    #[must_use]
    pub const fn distributed(&self) -> &DistributedCapability {
        &self.distributed
    }

    /// Adds a logical qubit to the immutable builder state.
    #[must_use]
    pub fn with_logical_qubit(
        mut self,
        qubit: QubitId,
    ) -> Self {
        self.logical_qubits.insert(qubit);
        self
    }

    /// Adds a physical qubit to the immutable builder state.
    #[must_use]
    pub fn with_physical_qubit(
        mut self,
        qubit: PhysicalQubitId,
    ) -> Self {
        self.physical_qubits.insert(qubit);
        self
    }

    /// Adds a resource capability.
    pub fn with_resource(
        mut self,
        resource: ResourceCapability,
    ) -> SchedulingContextResult<Self> {
        if resource.id.is_empty() {
            return Err(
                SchedulingContextError::InvalidResourceCapacity {
                    resource: String::new(),
                },
            );
        }

        self.resources.insert(resource.id.clone(), resource);
        Ok(self)
    }

    /// Adds an operation capability.
    #[must_use]
    pub fn with_operation(
        mut self,
        capability: OperationCapability,
    ) -> Self {
        self.operations
            .insert(capability.name.clone(), capability);
        self
    }

    /// Sets dynamic-control capability.
    #[must_use]
    pub fn with_dynamic_control(
        mut self,
        capability: DynamicControlCapability,
    ) -> Self {
        self.dynamic_control = capability;
        self
    }

    /// Sets distributed capability.
    #[must_use]
    pub fn with_distributed(
        mut self,
        capability: DistributedCapability,
    ) -> Self {
        self.distributed = capability;
        self
    }
}

// =============================================================================
// Resource capability
// =============================================================================

/// Capability of a schedulable target resource.
///
/// Capacity is not interpreted as a machine-wide maximum. It describes the
/// capacity exposed by this particular target snapshot.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResourceCapability {
    id: Arc<str>,
    capacity: u64,
    shareable: bool,
    hierarchical_parent: Option<Arc<str>>,
}

impl ResourceCapability {
    /// Creates a resource capability.
    pub fn new(
        id: impl Into<Arc<str>>,
        capacity: u64,
    ) -> SchedulingContextResult<Self> {
        let id = id.into();

        if id.is_empty() || capacity == 0 {
            return Err(
                SchedulingContextError::InvalidResourceCapacity {
                    resource: id.to_string(),
                },
            );
        }

        Ok(Self {
            id,
            capacity,
            shareable: false,
            hierarchical_parent: None,
        })
    }

    /// Marks this resource as shareable.
    #[must_use]
    pub fn shareable(
        mut self,
        value: bool,
    ) -> Self {
        self.shareable = value;
        self
    }

    /// Associates this resource with a parent resource.
    #[must_use]
    pub fn with_parent(
        mut self,
        parent: impl Into<Arc<str>>,
    ) -> Self {
        self.hierarchical_parent = Some(parent.into());
        self
    }

    /// Returns the resource identifier.
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns exposed capacity.
    #[must_use]
    pub const fn capacity(&self) -> u64 {
        self.capacity
    }

    /// Returns whether concurrent sharing is allowed.
    #[must_use]
    pub const fn is_shareable(&self) -> bool {
        self.shareable
    }

    /// Returns the optional parent resource.
    #[must_use]
    pub fn parent(&self) -> Option<&str> {
        self.hierarchical_parent.as_deref()
    }
}

// =============================================================================
// Operation capability
// =============================================================================

/// Target capability describing one schedulable operation class.
///
/// Operation semantics remain owned by the canonical quantum IR.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperationCapability {
    name: Arc<str>,
    min_arity: usize,
    max_arity: usize,
    duration: Option<DurationSpec>,
    required_resources: BTreeSet<Arc<str>>,
    supports_dynamic_control: bool,
}

impl OperationCapability {
    /// Creates an operation capability.
    #[must_use]
    pub fn new(
        name: impl Into<Arc<str>>,
        arity: usize,
    ) -> Self {
        let name = name.into();

        Self {
            name,
            min_arity: arity,
            max_arity: arity,
            duration: None,
            required_resources: BTreeSet::new(),
            supports_dynamic_control: false,
        }
    }

    /// Creates an operation capability supporting an arity range.
    #[must_use]
    pub fn with_arity_range(
        mut self,
        min_arity: usize,
        max_arity: usize,
    ) -> Self {
        self.min_arity = min_arity;
        self.max_arity = max_arity;
        self
    }

    /// Associates an operation duration specification.
    #[must_use]
    pub fn with_duration(
        mut self,
        duration: DurationSpec,
    ) -> Self {
        self.duration = Some(duration);
        self
    }

    /// Adds a required resource.
    #[must_use]
    pub fn with_required_resource(
        mut self,
        resource: impl Into<Arc<str>>,
    ) -> Self {
        self.required_resources.insert(resource.into());
        self
    }

    /// Enables dynamic-control execution for this operation.
    #[must_use]
    pub fn supports_dynamic_control(
        mut self,
        value: bool,
    ) -> Self {
        self.supports_dynamic_control = value;
        self
    }

    /// Returns operation name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns minimum arity.
    #[must_use]
    pub const fn min_arity(&self) -> usize {
        self.min_arity
    }

    /// Returns maximum arity.
    #[must_use]
    pub const fn max_arity(&self) -> usize {
        self.max_arity
    }

    /// Returns duration information.
    #[must_use]
    pub const fn duration(&self) -> Option<&DurationSpec> {
        self.duration.as_ref()
    }

    /// Returns required resources.
    #[must_use]
    pub fn required_resources(&self) -> &BTreeSet<Arc<str>> {
        &self.required_resources
    }

    /// Returns whether dynamic control is supported.
    #[must_use]
    pub const fn supports_dynamic_control(&self) -> bool {
        self.supports_dynamic_control
    }
}

// =============================================================================
// Duration specification
// =============================================================================

/// Hardware-neutral duration description.
///
/// Scheduling algorithms may resolve this to concrete target time units.
/// A duration may be exact, bounded, or target-provided.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DurationSpec {
    /// Exact duration.
    Exact {
        /// Duration in target timing units.
        units: u64,
    },

    /// Minimum and maximum possible duration.
    Range {
        /// Minimum duration.
        min: u64,

        /// Maximum duration.
        max: u64,
    },

    /// Duration must be supplied by the target/calibration snapshot.
    TargetProvided,
}

// =============================================================================
// Timing capability
// =============================================================================

/// Target timing capabilities.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TimingCapability {
    resolution: TimingResolution,
    supports_absolute_windows: bool,
    supports_overlapping_operations: bool,
}

impl TimingCapability {
    /// Creates timing capabilities.
    #[must_use]
    pub const fn new(
        resolution: TimingResolution,
    ) -> Self {
        Self {
            resolution,
            supports_absolute_windows: true,
            supports_overlapping_operations: true,
        }
    }

    /// Enables/disables absolute timing windows.
    #[must_use]
    pub const fn with_absolute_windows(
        mut self,
        enabled: bool,
    ) -> Self {
        self.supports_absolute_windows = enabled;
        self
    }

    /// Enables/disables overlapping operations.
    #[must_use]
    pub const fn with_overlapping_operations(
        mut self,
        enabled: bool,
    ) -> Self {
        self.supports_overlapping_operations = enabled;
        self
    }

    /// Returns timing resolution.
    #[must_use]
    pub const fn resolution(&self) -> TimingResolution {
        self.resolution
    }

    /// Returns whether absolute windows are supported.
    #[must_use]
    pub const fn supports_absolute_windows(&self) -> bool {
        self.supports_absolute_windows
    }

    /// Returns whether overlapping operations can be represented.
    #[must_use]
    pub const fn supports_overlapping_operations(&self) -> bool {
        self.supports_overlapping_operations
    }
}

/// Timing resolution expressed as a rational unit.
///
/// A rational representation avoids embedding assumptions such as
/// "one hardware tick equals one nanosecond".
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TimingResolution {
    numerator: u64,
    denominator: NonZeroU64,
}

impl TimingResolution {
    /// Creates a timing resolution.
    pub const fn new(
        numerator: u64,
        denominator: NonZeroU64,
    ) -> SchedulingContextResult<Self> {
        if numerator == 0 {
            return Err(
                SchedulingContextError::InvalidTimingResolution {
                    message: "numerator must be non-zero".to_owned(),
                },
            );
        }

        Ok(Self {
            numerator,
            denominator,
        })
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
}

// =============================================================================
// Dynamic control
// =============================================================================

/// Dynamic-control capability snapshot.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DynamicControlCapability {
    /// Measurements may feed classical control.
    measurement_feedback: bool,

    /// Conditional operations are supported.
    conditionals: bool,

    /// Runtime loop/branch regions are supported.
    runtime_control_flow: bool,

    /// Classical processing latency can be represented.
    classical_latency: bool,
}

impl DynamicControlCapability {
    /// Creates a fully disabled dynamic-control capability.
    #[must_use]
    pub const fn disabled() -> Self {
        Self {
            measurement_feedback: false,
            conditionals: false,
            runtime_control_flow: false,
            classical_latency: false,
        }
    }

    /// Enables measurement feedback.
    #[must_use]
    pub const fn with_measurement_feedback(
        mut self,
        enabled: bool,
    ) -> Self {
        self.measurement_feedback = enabled;
        self
    }

    /// Enables conditional operations.
    #[must_use]
    pub const fn with_conditionals(
        mut self,
        enabled: bool,
    ) -> Self {
        self.conditionals = enabled;
        self
    }

    /// Enables runtime control flow.
    #[must_use]
    pub const fn with_runtime_control_flow(
        mut self,
        enabled: bool,
    ) -> Self {
        self.runtime_control_flow = enabled;
        self
    }

    /// Enables classical-latency modelling.
    #[must_use]
    pub const fn with_classical_latency(
        mut self,
        enabled: bool,
    ) -> Self {
        self.classical_latency = enabled;
        self
    }

    /// Returns whether measurement feedback is supported.
    #[must_use]
    pub const fn measurement_feedback(self) -> bool {
        self.measurement_feedback
    }

    /// Returns whether conditional operations are supported.
    #[must_use]
    pub const fn conditionals(self) -> bool {
        self.conditionals
    }

    /// Returns whether runtime control flow is supported.
    #[must_use]
    pub const fn runtime_control_flow(self) -> bool {
        self.runtime_control_flow
    }

    /// Returns whether classical latency can be modelled.
    #[must_use]
    pub const fn classical_latency(self) -> bool {
        self.classical_latency
    }
}

impl Default for DynamicControlCapability {
    fn default() -> Self {
        Self::disabled()
    }
}

// =============================================================================
// Distributed capability
// =============================================================================

/// Distributed-quantum scheduling capability.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DistributedCapability {
    /// Whether the target can span multiple quantum processing domains.
    multi_node: bool,

    /// Whether communication resources have explicit timing.
    timed_communication: bool,

    /// Whether remote operations can be represented.
    remote_operations: bool,
}

impl DistributedCapability {
    /// Creates a non-distributed capability.
    #[must_use]
    pub const fn disabled() -> Self {
        Self {
            multi_node: false,
            timed_communication: false,
            remote_operations: false,
        }
    }

    /// Configures distributed capabilities.
    #[must_use]
    pub const fn new(
        multi_node: bool,
        timed_communication: bool,
        remote_operations: bool,
    ) -> Self {
        Self {
            multi_node,
            timed_communication,
            remote_operations,
        }
    }

    /// Returns whether multiple quantum nodes are supported.
    #[must_use]
    pub const fn multi_node(self) -> bool {
        self.multi_node
    }

    /// Returns whether communication timing is explicit.
    #[must_use]
    pub const fn timed_communication(self) -> bool {
        self.timed_communication
    }

    /// Returns whether remote operations are representable.
    #[must_use]
    pub const fn remote_operations(self) -> bool {
        self.remote_operations
    }
}

impl Default for DistributedCapability {
    fn default() -> Self {
        Self::disabled()
    }
}

// =============================================================================
// Scheduling objective
// =============================================================================

/// Primary scheduling objective.
///
/// This describes intent only. Algorithms remain responsible for actually
/// implementing the objective.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SchedulingObjective {
    /// Minimize total execution time.
    MinimizeMakespan,

    /// Minimize logical/physical scheduled depth.
    MinimizeDepth,

    /// Minimize resource idle time.
    MinimizeIdleTime,

    /// Prefer schedules with better estimated physical quality.
    MaximizeEstimatedFidelity,

    /// Minimize estimated execution energy/cost.
    MinimizeEnergy,

    /// Use an explicitly weighted multi-objective strategy.
    MultiObjective,
}

impl Default for SchedulingObjective {
    fn default() -> Self {
        Self::MinimizeMakespan
    }
}

// =============================================================================
// Scheduling limits
// =============================================================================

/// Explicit invocation/deployment limits.
///
/// These are *policies*, not architectural machine-size limits.
///
/// `None` means that this particular dimension has no scheduler-imposed limit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SchedulingLimits {
    max_operations: Option<u64>,
    max_dependency_edges: Option<u64>,
    max_resources: Option<u64>,
    max_schedule_time: Option<u64>,
    max_memory_bytes: Option<u64>,
}

impl SchedulingLimits {
    /// Creates unrestricted scheduling limits.
    #[must_use]
    pub const fn unrestricted() -> Self {
        Self {
            max_operations: None,
            max_dependency_edges: None,
            max_resources: None,
            max_schedule_time: None,
            max_memory_bytes: None,
        }
    }

    /// Sets an operation-count limit.
    #[must_use]
    pub const fn with_max_operations(
        mut self,
        value: Option<u64>,
    ) -> Self {
        self.max_operations = value;
        self
    }

    /// Sets a dependency-edge limit.
    #[must_use]
    pub const fn with_max_dependency_edges(
        mut self,
        value: Option<u64>,
    ) -> Self {
        self.max_dependency_edges = value;
        self
    }

    /// Sets a resource-count limit.
    #[must_use]
    pub const fn with_max_resources(
        mut self,
        value: Option<u64>,
    ) -> Self {
        self.max_resources = value;
        self
    }

    /// Sets a schedule-duration limit.
    #[must_use]
    pub const fn with_max_schedule_time(
        mut self,
        value: Option<u64>,
    ) -> Self {
        self.max_schedule_time = value;
        self
    }

    /// Sets a memory limit.
    #[must_use]
    pub const fn with_max_memory_bytes(
        mut self,
        value: Option<u64>,
    ) -> Self {
        self.max_memory_bytes = value;
        self
    }

    /// Returns the operation limit.
    #[must_use]
    pub const fn max_operations(self) -> Option<u64> {
        self.max_operations
    }

    /// Returns the dependency-edge limit.
    #[must_use]
    pub const fn max_dependency_edges(self) -> Option<u64> {
        self.max_dependency_edges
    }

    /// Returns the resource limit.
    #[must_use]
    pub const fn max_resources(self) -> Option<u64> {
        self.max_resources
    }

    /// Returns the schedule-time limit.
    #[must_use]
    pub const fn max_schedule_time(self) -> Option<u64> {
        self.max_schedule_time
    }

    /// Returns the memory limit.
    #[must_use]
    pub const fn max_memory_bytes(self) -> Option<u64> {
        self.max_memory_bytes
    }
}

impl Default for SchedulingLimits {
    fn default() -> Self {
        Self::unrestricted()
    }
}

// =============================================================================
// Scheduling metadata
// =============================================================================

/// Immutable metadata attached to a scheduling invocation.
///
/// Metadata is diagnostic/provenance information only. It must never become a
/// hidden scheduling input.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SchedulingMetadata {
    values: BTreeMap<Arc<str>, Arc<str>>,
}

impl SchedulingMetadata {
    /// Creates empty metadata.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds metadata.
    ///
    /// Keys and values are immutable after insertion into the resulting
    /// metadata value.
    pub fn with(
        mut self,
        key: impl Into<Arc<str>>,
        value: impl Into<Arc<str>>,
    ) -> SchedulingContextResult<Self> {
        let key = key.into();
        let value = value.into();

        if key.is_empty() {
            return Err(
                SchedulingContextError::InvalidMetadata {
                    key: String::new(),
                    message: "metadata key must not be empty".to_owned(),
                },
            );
        }

        self.values.insert(key, value);
        Ok(self)
    }

    /// Returns all metadata.
    #[must_use]
    pub fn values(&self) -> &BTreeMap<Arc<str>, Arc<str>> {
        &self.values
    }

    /// Returns one metadata value.
    #[must_use]
    pub fn get(
        &self,
        key: &str,
    ) -> Option<&str> {
        self.values.get(key).map(Arc::as_ref)
    }
}

// =============================================================================
// SchedulingContext
// =============================================================================

/// Immutable, validated input snapshot for the scheduling subsystem.
///
/// This is the central integration boundary for production scheduling.
///
/// The context deliberately stores *descriptions and snapshots*, not live
/// services. A scheduler therefore cannot accidentally acquire hidden mutable
/// state from hardware, network clients, credentials, calibration services or
/// global registries.
#[derive(Debug, Clone)]
pub struct SchedulingContext {
    id: ContextId,
    target: SchedulingTarget,
    mode: SchedulingMode,
    epoch: SchedulingEpoch,
    deadline: Option<SchedulingDeadline>,
    objective: SchedulingObjective,
    determinism: Determinism,
    limits: SchedulingLimits,
    metadata: SchedulingMetadata,

    /// Logical-to-physical mapping snapshot.
    ///
    /// Routing owns creation of this mapping. Scheduling merely consumes it.
    mapping: BTreeMap<QubitId, PhysicalQubitId>,

    /// Optional dependency identifiers supplied by upstream analysis.
    ///
    /// The scheduler's graph subsystem is responsible for turning these
    /// references into the complete dependency graph.
    dependencies: BTreeMap<u64, BTreeSet<u64>>,

    /// Optional abstract resource requirements keyed by operation identity.
    ///
    /// Operation IDs are represented as raw stable numeric values at this
    /// boundary so this file does not redefine the canonical IR operation
    /// identity type.
    resource_requirements: BTreeMap<u64, BTreeSet<Arc<str>>>,
}

impl SchedulingContext {
    /// Creates a new scheduling context.
    ///
    /// The returned context is not accepted by a scheduler until
    /// `validate()` succeeds.
    #[must_use]
    pub fn new(
        id: ContextId,
        target: SchedulingTarget,
    ) -> Self {
        Self {
            id,
            target,
            mode: SchedulingMode::Static,
            epoch: SchedulingEpoch::default(),
            deadline: None,
            objective: SchedulingObjective::default(),
            determinism: Determinism::default(),
            limits: SchedulingLimits::default(),
            metadata: SchedulingMetadata::default(),
            mapping: BTreeMap::new(),
            dependencies: BTreeMap::new(),
            resource_requirements: BTreeMap::new(),
        }
    }

    /// Returns the context identity.
    #[must_use]
    pub fn id(&self) -> &ContextId {
        &self.id
    }

    /// Returns the target capability snapshot.
    #[must_use]
    pub const fn target(&self) -> &SchedulingTarget {
        &self.target
    }

    /// Returns the scheduling mode.
    #[must_use]
    pub const fn mode(&self) -> SchedulingMode {
        self.mode
    }

    /// Returns the scheduling epoch.
    #[must_use]
    pub const fn epoch(&self) -> SchedulingEpoch {
        self.epoch
    }

    /// Returns the optional scheduling deadline.
    #[must_use]
    pub const fn deadline(&self) -> Option<SchedulingDeadline> {
        self.deadline
    }

    /// Returns the scheduling objective.
    #[must_use]
    pub const fn objective(&self) -> SchedulingObjective {
        self.objective
    }

    /// Returns determinism configuration.
    #[must_use]
    pub const fn determinism(&self) -> Determinism {
        self.determinism
    }

    /// Returns explicit scheduler limits.
    #[must_use]
    pub const fn limits(&self) -> SchedulingLimits {
        self.limits
    }

    /// Returns immutable metadata.
    #[must_use]
    pub fn metadata(&self) -> &SchedulingMetadata {
        &self.metadata
    }

    /// Returns the routing-produced logical-to-physical mapping.
    #[must_use]
    pub fn mapping(&self) -> &BTreeMap<QubitId, PhysicalQubitId> {
        &self.mapping
    }

    /// Returns scheduling dependencies.
    #[must_use]
    pub fn dependencies(&self) -> &BTreeMap<u64, BTreeSet<u64>> {
        &self.dependencies
    }

    /// Returns abstract resource requirements.
    #[must_use]
    pub fn resource_requirements(
        &self,
    ) -> &BTreeMap<u64, BTreeSet<Arc<str>>> {
        &self.resource_requirements
    }

    /// Sets the scheduling mode.
    #[must_use]
    pub fn with_mode(
        mut self,
        mode: SchedulingMode,
    ) -> Self {
        self.mode = mode;
        self
    }

    /// Sets the scheduling epoch.
    #[must_use]
    pub fn with_epoch(
        mut self,
        epoch: SchedulingEpoch,
    ) -> Self {
        self.epoch = epoch;
        self
    }

    /// Sets a scheduling deadline.
    #[must_use]
    pub fn with_deadline(
        mut self,
        deadline: SchedulingDeadline,
    ) -> Self {
        self.deadline = Some(deadline);
        self
    }

    /// Sets the scheduling objective.
    #[must_use]
    pub fn with_objective(
        mut self,
        objective: SchedulingObjective,
    ) -> Self {
        self.objective = objective;
        self
    }

    /// Sets determinism configuration.
    #[must_use]
    pub fn with_determinism(
        mut self,
        determinism: Determinism,
    ) -> Self {
        self.determinism = determinism;
        self
    }

    /// Sets explicit invocation limits.
    #[must_use]
    pub fn with_limits(
        mut self,
        limits: SchedulingLimits,
    ) -> Self {
        self.limits = limits;
        self
    }

    /// Sets immutable diagnostic metadata.
    #[must_use]
    pub fn with_metadata(
        mut self,
        metadata: SchedulingMetadata,
    ) -> Self {
        self.metadata = metadata;
        self
    }

    /// Adds a routing-produced logical-to-physical mapping.
    ///
    /// The scheduler does not perform routing. It merely records the mapping
    /// supplied by the routing subsystem.
    #[must_use]
    pub fn with_mapping(
        mut self,
        logical: QubitId,
        physical: PhysicalQubitId,
    ) -> Self {
        self.mapping.insert(logical, physical);
        self
    }

    /// Adds a dependency relation.
    ///
    /// `operation` and `predecessor` are stable operation identities supplied
    /// by the upstream IR analysis layer.
    #[must_use]
    pub fn with_dependency(
        mut self,
        operation: u64,
        predecessor: u64,
    ) -> Self {
        self.dependencies
            .entry(operation)
            .or_default()
            .insert(predecessor);

        self
    }

    /// Adds an abstract resource requirement to an operation.
    #[must_use]
    pub fn with_resource_requirement(
        mut self,
        operation: u64,
        resource: impl Into<Arc<str>>,
    ) -> Self {
        self.resource_requirements
            .entry(operation)
            .or_default()
            .insert(resource.into());

        self
    }

    /// Validates all context invariants.
    ///
    /// This method performs only context validation. It does not validate the
    /// complete quantum program or dependency graph; those remain owned by
    /// their respective subsystems.
    pub fn validate(&self) -> SchedulingContextResult<()> {
        if self.id.as_str().is_empty() {
            return Err(SchedulingContextError::EmptyIdentifier {
                field: "context_id",
            });
        }

        if self.target.id().as_str().is_empty() {
            return Err(SchedulingContextError::EmptyIdentifier {
                field: "target_id",
            });
        }

        if let Some(deadline) = self.deadline {
            if deadline.value() < self.epoch.value() {
                return Err(SchedulingContextError::InvalidDeadline);
            }
        }

        for resource in self.target.resources().values() {
            if resource.id().is_empty() || resource.capacity() == 0 {
                return Err(
                    SchedulingContextError::InvalidResourceCapacity {
                        resource: resource.id().to_owned(),
                    },
                );
            }
        }

        for (logical, physical) in &self.mapping {
            if !self.target.logical_qubits().is_empty()
                && !self.target.logical_qubits().contains(logical)
            {
                return Err(
                    SchedulingContextError::UnknownLogicalQubit {
                        qubit: *logical,
                    },
                );
            }

            if !self.target.physical_qubits().is_empty()
                && !self.target.physical_qubits().contains(physical)
            {
                return Err(
                    SchedulingContextError::UnknownPhysicalQubit {
                        qubit: *physical,
                    },
                );
            }
        }

        for (operation, predecessors) in &self.dependencies {
            if predecessors.contains(operation) {
                return Err(
                    SchedulingContextError::InvalidMetadata {
                        key: operation.to_string(),
                        message:
                            "an operation cannot depend on itself"
                                .to_owned(),
                    },
                );
            }
        }

        Ok(())
    }

    /// Returns whether this context is compatible with static scheduling.
    #[must_use]
    pub fn supports_static_scheduling(&self) -> bool {
        matches!(
            self.mode,
            SchedulingMode::Static | SchedulingMode::Hybrid
        )
    }

    /// Returns whether this context requires runtime scheduling support.
    #[must_use]
    pub fn requires_dynamic_scheduling(&self) -> bool {
        matches!(
            self.mode,
            SchedulingMode::Dynamic | SchedulingMode::Hybrid
        )
    }

    /// Returns whether deterministic scheduling is required.
    #[must_use]
    pub const fn requires_determinism(&self) -> bool {
        self.determinism.is_deterministic()
    }

    /// Returns whether the target supports distributed scheduling.
    #[must_use]
    pub const fn supports_distributed_scheduling(&self) -> bool {
        self.target.distributed().multi_node()
    }

    /// Returns whether dynamic classical feedback can be represented.
    #[must_use]
    pub const fn supports_dynamic_control(&self) -> bool {
        self.target.dynamic_control().conditionals()
            && self.target.dynamic_control().measurement_feedback()
    }
}

// =============================================================================
// Context builder
// =============================================================================

/// Builder for constructing a validated `SchedulingContext`.
///
/// The builder exists so callers do not need mutable access to a partially
/// constructed context. `build()` validates the resulting snapshot.
#[derive(Debug, Clone)]
pub struct SchedulingContextBuilder {
    context: SchedulingContext,
}

impl SchedulingContextBuilder {
    /// Creates a builder.
    #[must_use]
    pub fn new(
        id: ContextId,
        target: SchedulingTarget,
    ) -> Self {
        Self {
            context: SchedulingContext::new(id, target),
        }
    }

    /// Sets scheduling mode.
    #[must_use]
    pub fn mode(
        mut self,
        mode: SchedulingMode,
    ) -> Self {
        self.context = self.context.with_mode(mode);
        self
    }

    /// Sets epoch.
    #[must_use]
    pub fn epoch(
        mut self,
        epoch: SchedulingEpoch,
    ) -> Self {
        self.context = self.context.with_epoch(epoch);
        self
    }

    /// Sets deadline.
    #[must_use]
    pub fn deadline(
        mut self,
        deadline: SchedulingDeadline,
    ) -> Self {
        self.context = self.context.with_deadline(deadline);
        self
    }

    /// Sets objective.
    #[must_use]
    pub fn objective(
        mut self,
        objective: SchedulingObjective,
    ) -> Self {
        self.context = self.context.with_objective(objective);
        self
    }

    /// Sets determinism.
    #[must_use]
    pub fn determinism(
        mut self,
        determinism: Determinism,
    ) -> Self {
        self.context = self.context.with_determinism(determinism);
        self
    }

    /// Sets limits.
    #[must_use]
    pub fn limits(
        mut self,
        limits: SchedulingLimits,
    ) -> Self {
        self.context = self.context.with_limits(limits);
        self
    }

    /// Sets metadata.
    #[must_use]
    pub fn metadata(
        mut self,
        metadata: SchedulingMetadata,
    ) -> Self {
        self.context = self.context.with_metadata(metadata);
        self
    }

    /// Adds a logical-to-physical mapping.
    #[must_use]
    pub fn mapping(
        mut self,
        logical: QubitId,
        physical: PhysicalQubitId,
    ) -> Self {
        self.context =
            self.context.with_mapping(logical, physical);
        self
    }

    /// Adds a dependency.
    #[must_use]
    pub fn dependency(
        mut self,
        operation: u64,
        predecessor: u64,
    ) -> Self {
        self.context =
            self.context.with_dependency(operation, predecessor);
        self
    }

    /// Adds a resource requirement.
    #[must_use]
    pub fn resource_requirement(
        mut self,
        operation: u64,
        resource: impl Into<Arc<str>>,
    ) -> Self {
        self.context = self
            .context
            .with_resource_requirement(operation, resource);
        self
    }

    /// Validates and returns the immutable context.
    pub fn build(self) -> SchedulingContextResult<SchedulingContext> {
        self.context.validate()?;
        Ok(self.context)
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn revision() -> TargetRevision {
        TargetRevision::new(
            NonZeroU64::new(1).expect("one is non-zero"),
        )
    }

    fn target() -> SchedulingTarget {
        let resolution = TimingResolution::new(
            1,
            NonZeroU64::new(1).expect("one is non-zero"),
        )
        .expect("valid timing resolution");

        SchedulingTarget::new(
            TargetId::new("test-target").expect("valid target"),
            revision(),
            QuantumTechnology::Hybrid,
            TimingCapability::new(resolution),
        )
    }

    #[test]
    fn context_is_valid_by_default() {
        let context = SchedulingContext::new(
            ContextId::new("context").expect("valid context"),
            target(),
        );

        assert!(context.validate().is_ok());
        assert!(context.requires_determinism());
        assert!(context.supports_static_scheduling());
    }

    #[test]
    fn logical_and_physical_mapping_uses_canonical_ids() {
        let logical = QubitId::new(0);
        let physical = PhysicalQubitId::new(0);

        let target = target()
            .with_logical_qubit(logical)
            .with_physical_qubit(physical);

        let context = SchedulingContext::new(
            ContextId::new("mapping-test").expect("valid context"),
            target,
        )
        .with_mapping(logical, physical);

        assert!(context.validate().is_ok());
        assert_eq!(
            context.mapping().get(&logical),
            Some(&physical)
        );
    }

    #[test]
    fn invalid_mapping_is_rejected() {
        let logical = QubitId::new(0);
        let physical = PhysicalQubitId::new(1);

        let target = target()
            .with_logical_qubit(logical)
            .with_physical_qubit(
                PhysicalQubitId::new(0),
            );

        let context = SchedulingContext::new(
            ContextId::new("invalid-mapping")
                .expect("valid context"),
            target,
        )
        .with_mapping(logical, physical);

        assert!(matches!(
            context.validate(),
            Err(
                SchedulingContextError::UnknownPhysicalQubit {
                    ..
                }
            )
        ));
    }

    #[test]
    fn deadline_before_epoch_is_rejected() {
        let context = SchedulingContext::new(
            ContextId::new("deadline")
                .expect("valid context"),
            target(),
        )
        .with_epoch(SchedulingEpoch::new(100))
        .with_deadline(SchedulingDeadline::new(99));

        assert!(matches!(
            context.validate(),
            Err(SchedulingContextError::InvalidDeadline)
        ));
    }

    #[test]
    fn self_dependency_is_rejected() {
        let context = SchedulingContext::new(
            ContextId::new("dependency")
                .expect("valid context"),
            target(),
        )
        .with_dependency(42, 42);

        assert!(context.validate().is_err());
    }

    #[test]
    fn deterministic_configuration_is_stable() {
        let deterministic = Determinism::deterministic();
        assert!(deterministic.is_deterministic());
        assert_eq!(deterministic.seed(), None);

        let seeded = Determinism::with_seed(1234);
        assert!(seeded.is_deterministic());
        assert_eq!(seeded.seed(), Some(1234));
    }

    #[test]
    fn metadata_is_ordered_and_immutable() {
        let metadata = SchedulingMetadata::new()
            .with("target", "test")
            .expect("valid metadata")
            .with("compiler", "zamani")
            .expect("valid metadata");

        assert_eq!(metadata.get("target"), Some("test"));
        assert_eq!(metadata.get("compiler"), Some("zamani"));
    }

    #[test]
    fn explicit_limits_do_not_define_architectural_limits() {
        let unlimited = SchedulingLimits::unrestricted();

        assert_eq!(unlimited.max_operations(), None);
        assert_eq!(unlimited.max_dependency_edges(), None);
        assert_eq!(unlimited.max_resources(), None);
        assert_eq!(unlimited.max_schedule_time(), None);
        assert_eq!(unlimited.max_memory_bytes(), None);
    }

    #[test]
    fn dynamic_context_is_reported_correctly() {
        let context = SchedulingContext::new(
            ContextId::new("dynamic")
                .expect("valid context"),
            target(),
        )
        .with_mode(SchedulingMode::Dynamic);

        assert!(!context.supports_static_scheduling());
        assert!(context.requires_dynamic_scheduling());
    }
}