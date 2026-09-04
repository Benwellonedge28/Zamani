//! Zamani Quantum Scheduling — Production Planner Contract
//!
//! This module defines the stable planner boundary for the Zamani quantum
//! scheduling subsystem.
//!
//! # Responsibility
//!
//! This file answers:
//!
//! > "What is a scheduling planner, what does it promise, what capabilities
//! > does it expose, and how does the rest of the scheduling subsystem invoke
//! > it?"
//!
//! This module does NOT implement a particular scheduling algorithm.
//!
//! Concrete algorithms belong in:
//!
//! ```text
//! scheduling::algorithms
//! scheduling::planners::list
//! scheduling::planners::critical_path
//! scheduling::planners::resource_constrained
//! scheduling::planners::event
//! ```
//!
//! The planner contract is intentionally independent from those
//! implementations.
//!
//! # Architectural position
//!
//! ```text
//!                         Zamani source
//!                              │
//!                              ▼
//!                       quantum::frontend
//!                              │
//!                              ▼
//!                         quantum::ir
//!                              │
//!                              ▼
//!                         optimization
//!                              │
//!                              ▼
//!                           routing
//!                              │
//!                              ▼
//!                    SchedulingContext
//!                              │
//!          ┌───────────────────┼───────────────────┐
//!          │                   │                   │
//!          ▼                   ▼                   ▼
//!      dependency           resources           timing
//!        graph               model              model
//!          │                   │                   │
//!          └───────────────────┼───────────────────┘
//!                              │
//!                              ▼
//!                    SchedulingPlanner
//!                              │
//!             ┌────────────────┼─────────────────┐
//!             │                │                 │
//!             ▼                ▼                 ▼
//!           ASAP              ALAP              List
//!             │                │                 │
//!             └────────────────┼─────────────────┘
//!                              ▼
//!                     SchedulingResult
//!                              │
//!             ┌────────────────┼─────────────────┐
//!             ▼                ▼                 ▼
//!        verification     transformations      diagnostics
//!             │                │                 │
//!             └────────────────┼─────────────────┘
//!                              ▼
//!                       hardware/runtime
//! ```
//!
//! # Critical architectural rule
//!
//! A planner answers:
//!
//! > WHEN can operations execute under the supplied scheduling model?
//!
//! It must never answer:
//!
//! > WHERE should logical qubits be placed?
//!
//! Logical-to-physical mapping belongs to routing.
//!
//! It must also never answer:
//!
//! > HOW does the target implement this operation?
//!
//! Hardware lowering belongs to the hardware/backend subsystem.
//!
//! # Canonical IR boundary
//!
//! The canonical quantum IR owns quantum semantics.
//!
//! In particular, the authoritative qubit identities are:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! This file deliberately does not define, wrap, or duplicate either type.
//!
//! Operation and resource identities likewise remain owned by the canonical
//! IR/scheduling type boundaries.
//!
//! # Write once, scale everywhere
//!
//! The planner contract contains no:
//!
//! - maximum qubit count;
//! - maximum operation count;
//! - maximum resource count;
//! - maximum topology size;
//! - maximum schedule depth;
//! - fixed gate set;
//! - fixed gate arity;
//! - fixed timing unit;
//! - fixed channel count;
//! - fixed QEC distance;
//! - vendor identifier;
//! - hardware technology requirement.
//!
//! A planner receives all concrete information through `SchedulingContext`.
//!
//! Therefore the same planner implementation can be used for:
//!
//! ```text
//! one qubit
//! small QPU
//! large QPU
//! modular QPU
//! distributed QPU
//! quantum network
//! future quantum architectures
//! ```
//!
//! The word "infinity" in the Zamani architecture means that this contract
//! introduces no artificial finite machine-size ceiling. Actual compilation
//! remains bounded by the resources available to the compiler invocation,
//! operating system, target, and execution environment.
//!
//! # Separation of planner and algorithm
//!
//! `SchedulingPlanner` is a stable invocation contract.
//!
//! An implementation may internally use:
//!
//! - list scheduling;
//! - critical-path scheduling;
//! - resource-constrained project scheduling;
//! - event-driven scheduling;
//! - adaptive heuristics;
//! - exact optimization;
//! - approximation;
//! - a vendor-neutral research algorithm;
//! - a plugin-provided algorithm.
//!
//! None of those implementation choices belong in this file.
//!
//! # Separation from policy
//!
//! `SchedulingPolicy` describes scheduling intent.
//!
//! The planner turns that intent into a concrete scheduling result.
//!
//! ```text
//! SchedulingConfig
//!        │
//!        ▼
//! SchedulingPolicy
//!        │
//!        ▼
//! SchedulingContext
//!        │
//!        ▼
//! SchedulingPlanner
//!        │
//!        ▼
//! SchedulingResult
//! ```
//!
//! The planner must not redefine policy enums or configuration flags.
//!
//! # Separation from dependency analysis
//!
//! Dependency graphs are constructed and validated by `scheduling::ir`.
//!
//! The planner consumes those structures.
//!
//! It must not create a competing graph representation.
//!
//! # Separation from resource management
//!
//! Resource models and reservations belong to `scheduling::resources`.
//!
//! A planner may query them and create scheduling decisions through their
//! public contracts, but must not duplicate resource-calendar semantics.
//!
//! # Separation from timing
//!
//! Timing values belong to the scheduling timing subsystem and foundational
//! scheduler types.
//!
//! The planner must not assume nanoseconds, microseconds, device ticks,
//! pulse samples, or any other physical unit.
//!
//! # Separation from verification
//!
//! A planner produces a candidate scheduling result.
//!
//! Verification belongs to `scheduling::verification`.
//!
//! A planner may optionally perform lightweight precondition checks required
//! to protect its own algorithm, but it must not replace the canonical
//! verification layer.
//!
//! # Separation from transformations
//!
//! Explicit delays, alignment, padding, and dynamical-decoupling transformations
//! belong to `scheduling::transformations`.
//!
//! The planner should schedule semantic operations rather than embedding
//! transformation implementations.
//!
//! # Separation from hardware
//!
//! Hardware information enters through `SchedulingContext` and the appropriate
//! hardware adapter.
//!
//! This file must not:
//!
//! - discover devices;
//! - open network connections;
//! - authenticate;
//! - access credentials;
//! - execute jobs;
//! - call vendor SDKs;
//! - read hardware state directly.
//!
//! # Separation from routing
//!
//! Routing owns logical-to-physical placement and connectivity mapping.
//!
//! The planner consumes the result of routing through the scheduling context.
//!
//! ```text
//! logical operation
//!        │
//!        ▼
//!      routing
//!        │
//!        ▼
//! mapped operation
//!        │
//!        ▼
//!     planner
//!        │
//!        ▼
//! scheduled mapped operation
//! ```
//!
//! # Dynamic scheduling
//!
//! The contract supports:
//!
//! - static schedules;
//! - dynamic schedules;
//! - hybrid schedules;
//! - runtime feedback;
//! - conditional execution;
//! - measurement dependencies;
//! - classical-control dependencies.
//!
//! A planner implementation must not assume that every quantum program is a
//! simple static DAG.
//!
//! # Distributed scheduling
//!
//! The same contract may be implemented for:
//!
//! - one device;
//! - multiple chips;
//! - multiple QPUs;
//! - modular quantum computers;
//! - distributed quantum systems;
//! - quantum networks.
//!
//! Communication constraints must enter through the scheduling context and
//! resource/dependency contracts rather than through hard-coded topology logic.
//!
//! # Determinism
//!
//! When the supplied context/configuration requests deterministic scheduling,
//! a planner must:
//!
//! - use deterministic traversal;
//! - use deterministic candidate ordering;
//! - use deterministic tie-breaking;
//! - avoid iteration-order-dependent hash-map semantics;
//! - use the supplied seed when stochastic behaviour is explicitly permitted;
//! - produce equivalent output for equivalent immutable inputs.
//!
//! A planner must never silently introduce randomness.
//!
//! # Concurrency
//!
//! The trait is designed so implementations can be shared when their concrete
//! state is thread-safe.
//!
//! The trait itself does not require `Send`/`Sync` assertions through unsafe
//! mechanisms. Concrete implementations determine their own auto-trait
//! properties.
//!
//! The preferred production model is:
//!
//! ```text
//! immutable SchedulingContext
//!          │
//!          ├───────────────┐
//!          │               │
//!          ▼               ▼
//!       planner A       planner B
//!          │               │
//!          ▼               ▼
//!       result A        result B
//! ```
//!
//! No global scheduler state is permitted.
//!
//! # Object safety
//!
//! The primary planner trait is intentionally object-safe so registries and
//! plugin systems can store heterogeneous planner implementations without
//! changing the contract.
//!
//! # Error boundary
//!
//! The planner returns the canonical scheduling error type from
//! `scheduling::errors`.
//!
//! This file deliberately does not define a second scheduling error hierarchy.
//!
//! # Result boundary
//!
//! The planner returns the canonical `scheduling::result::SchedulingResult`
//! artifact.
//!
//! This file deliberately does not define a second schedule representation.
//!
//! # Compatibility
//!
//! This module is designed for:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! # Frozen-contract rule
//!
//! Once this contract is accepted, adding a new scheduling algorithm should
//! NOT require changing this file.
//!
//! A new implementation should instead implement `SchedulingPlanner`.
//!
//! Likewise, adding:
//!
//! - a new hardware technology;
//! - a new resource type;
//! - a new QEC strategy;
//! - a new routing algorithm;
//! - a new timing model;
//! - a new transformation;
//!
//! should not require changing this contract.
//!
//! Only a genuine change to the semantic meaning of "planner" should require
//! modifying this file.
//!
//! # Public API stability
//!
//! Stable planner identifiers are represented as strings rather than concrete
//! implementation types. This allows registries and serialized compilation
//! requests to identify planners without coupling them to implementation
//! modules.
//!
//! The identifier is descriptive metadata. It is not a hardware vendor name.
//!
//! # No artificial limits
//!
//! This file intentionally contains no constants such as:
//!
//! ```text
//! MAX_QUBITS
//! MAX_OPERATIONS
//! MAX_RESOURCES
//! MAX_DEPTH
//! MAX_PLANNERS
//! ```
//!
//! Safety/resource limits belong to `SchedulingLimits`, target capabilities,
//! deployment policy, or the host environment.
//!
//! # Integration map
//!
//! ```text
//! src/quantum/scheduling/
//! │
//! ├── context.rs
//! │       │
//! │       └──────────────► SchedulingPlanner::plan
//! │
//! ├── config.rs
//! │       │
//! │       └──────────────► planner configuration
//! │
//! ├── policies/
//! │       │
//! │       └──────────────► planner strategy selection
//! │
//! ├── ir/
//! │       │
//! │       └──────────────► dependency/resource-ready workload
//! │
//! ├── resources/
//! │       │
//! │       └──────────────► availability/reservation model
//! │
//! ├── timing/
//! │       │
//! │       └──────────────► temporal constraints
//! │
//! ├── algorithms/
//! │       │
//! │       └──────────────► concrete scheduling algorithms
//! │
//! ├── planners/
//! │       │
//! │       ├── list.rs
//! │       ├── critical_path.rs
//!       ├── resource_constrained.rs
//!       ├── event.rs
//!       └── planner.rs  <-- this contract
//! │
//! ├── verification/
//! │       │
//! │       └──────────────► result validation
//! │
//! ├── transformations/
//! │       │
//! │       └──────────────► delay/alignment/padding
//! │
//! ├── diagnostics/
//! │       │
//! │       └──────────────► explanations/profiling
//! │
//! ├── plugins/
//! │       │
//! │       └──────────────► external planner registration
//! │
//! └── result.rs
//!         │
//!         └──────────────► immutable schedule artifact
//! ```
//!
//! # Design invariant
//!
//! The planner contract must remain narrower than the scheduling subsystem as
//! a whole.
//!
//! That is intentional.
//!
//! A production scheduler should be composable from independent components
//! rather than putting every concern into one planner trait.
//!
//! =============================================================================
//! Safety boundary
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::error::Error;
use std::fmt;

use crate::quantum::scheduling::errors::{
    SchedulingError,
    SchedulingResult,
};
use crate::quantum::scheduling::result::SchedulingResult as ScheduleArtifact;
use crate::quantum::scheduling::context::SchedulingContext;

// =============================================================================
// Stable planner contract version
// =============================================================================

/// Semantic version of the planner contract.
///
/// This is independent of the Zamani crate/package version.
///
/// Increment this only when the externally observable planner contract changes
/// incompatibly.
pub const PLANNER_CONTRACT_VERSION: u32 = 1;

// =============================================================================
// Stable planner identifier
// =============================================================================

/// Maximum byte length accepted for a planner identifier.
///
/// This is a validation boundary for metadata, not a scheduling-machine limit.
///
/// The value is intentionally conservative and applies only to the textual
/// identifier itself.
pub const PLANNER_ID_MAX_BYTES: usize = 256;

/// Stable identifier for a scheduling planner.
///
/// Planner identifiers are implementation-neutral names used by registries,
/// diagnostics, configuration, distributed scheduling requests, and
/// serialization adapters.
///
/// Examples:
///
/// ```text
/// scheduling.list
/// scheduling.critical_path
/// scheduling.resource_constrained
/// scheduling.event
/// provider.example.custom
/// ```
///
/// The identifier does not encode:
///
/// - qubit count;
/// - hardware size;
/// - topology size;
/// - timing units;
/// - vendor capabilities.
///
/// Those properties belong to the target/context.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PlannerId(String);

impl PlannerId {
    /// Creates a validated planner identifier.
    ///
    /// Empty identifiers and identifiers exceeding the explicit metadata
    /// validation boundary are rejected.
    pub fn new(value: impl Into<String>) -> Result<Self, PlannerIdError> {
        let value = value.into();

        if value.is_empty() {
            return Err(PlannerIdError::Empty);
        }

        if value.len() > PLANNER_ID_MAX_BYTES {
            return Err(PlannerIdError::TooLong {
                length: value.len(),
                maximum: PLANNER_ID_MAX_BYTES,
            });
        }

        if !value
            .bytes()
            .all(|byte| {
                byte.is_ascii_alphanumeric()
                    || matches!(
                        byte,
                        b'.' | b'-' | b'_' | b':'
                    )
            })
        {
            return Err(PlannerIdError::InvalidCharacter);
        }

        Ok(Self(value))
    }

    /// Returns the planner identifier as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the identifier and returns its owned string.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for PlannerId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl TryFrom<String> for PlannerId {
    type Error = PlannerIdError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<&str> for PlannerId {
    type Error = PlannerIdError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

// =============================================================================
// Planner identifier errors
// =============================================================================

/// Errors produced while constructing a `PlannerId`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlannerIdError {
    /// The identifier is empty.
    Empty,

    /// The identifier exceeds the explicit metadata validation boundary.
    TooLong {
        /// Supplied byte length.
        length: usize,

        /// Maximum permitted metadata length.
        maximum: usize,
    },

    /// The identifier contains a character outside the stable identifier
    /// alphabet.
    InvalidCharacter,
}

impl fmt::Display for PlannerIdError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => {
                formatter.write_str("planner identifier must not be empty")
            }

            Self::TooLong {
                length,
                maximum,
            } => {
                write!(
                    formatter,
                    "planner identifier is {length} bytes; maximum \
                     metadata length is {maximum} bytes"
                )
            }

            Self::InvalidCharacter => {
                formatter.write_str(
                    "planner identifier contains an invalid character; \
                     only ASCII letters, digits, '.', '-', '_', and ':' \
                     are permitted",
                )
            }
        }
    }
}

impl Error for PlannerIdError {}

// =============================================================================
// Planner version
// =============================================================================

/// Version of one concrete planner implementation.
///
/// This is separate from `PLANNER_CONTRACT_VERSION`.
///
/// `PLANNER_CONTRACT_VERSION` identifies the interface contract.
///
/// `PlannerVersion` identifies the implementation itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PlannerVersion {
    major: u16,
    minor: u16,
    patch: u16,
}

impl PlannerVersion {
    /// Creates a planner implementation version.
    #[must_use]
    pub const fn new(
        major: u16,
        minor: u16,
        patch: u16,
    ) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Returns the major version.
    #[must_use]
    pub const fn major(self) -> u16 {
        self.major
    }

    /// Returns the minor version.
    #[must_use]
    pub const fn minor(self) -> u16 {
        self.minor
    }

    /// Returns the patch version.
    #[must_use]
    pub const fn patch(self) -> u16 {
        self.patch
    }
}

impl Default for PlannerVersion {
    fn default() -> Self {
        Self::new(1, 0, 0)
    }
}

impl fmt::Display for PlannerVersion {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}.{}.{}",
            self.major,
            self.minor,
            self.patch
        )
    }
}

// =============================================================================
// Planner execution mode
// =============================================================================

/// Declares the execution semantics supported by a planner.
///
/// This metadata does not itself change scheduler behaviour. It tells the
/// registry/pipeline whether the implementation can accept a particular
/// scheduling context.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PlannerExecutionMode {
    /// Planner produces fully resolved static schedules.
    Static,

    /// Planner can schedule runtime-dependent operations/events.
    Dynamic,

    /// Planner can handle both static and dynamic scheduling regions.
    Hybrid,
}

impl Default for PlannerExecutionMode {
    fn default() -> Self {
        Self::Static
    }
}

impl fmt::Display for PlannerExecutionMode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::Static => "static",
            Self::Dynamic => "dynamic",
            Self::Hybrid => "hybrid",
        };

        formatter.write_str(name)
    }
}

// =============================================================================
// Planner algorithm family
// =============================================================================

/// Describes the algorithmic family of a planner implementation.
///
/// This is descriptive metadata and must not be used as a replacement for the
/// actual scheduling policy.
///
/// A planner may internally combine multiple algorithmic techniques.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PlannerAlgorithmFamily {
    /// Earliest-feasible placement.
    AsSoonAsPossible,

    /// Latest-feasible placement.
    AsLateAsPossible,

    /// Ready-list/list scheduling.
    List,

    /// Critical-path-oriented scheduling.
    CriticalPath,

    /// Resource-constrained scheduling.
    ResourceConstrained,

    /// Event-driven scheduling.
    EventDriven,

    /// Adaptive strategy selection.
    Adaptive,

    /// Exact/optimization-backed scheduling.
    Optimization,

    /// External/custom implementation.
    Custom,
}

impl Default for PlannerAlgorithmFamily {
    fn default() -> Self {
        Self::List
    }
}

impl fmt::Display for PlannerAlgorithmFamily {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::AsSoonAsPossible => "asap",
            Self::AsLateAsPossible => "alap",
            Self::List => "list",
            Self::CriticalPath => "critical-path",
            Self::ResourceConstrained => "resource-constrained",
            Self::EventDriven => "event-driven",
            Self::Adaptive => "adaptive",
            Self::Optimization => "optimization",
            Self::Custom => "custom",
        };

        formatter.write_str(name)
    }
}

// =============================================================================
// Planner capabilities
// =============================================================================

/// Capability declaration for one planner implementation.
///
/// Capabilities are deliberately semantic. They do not contain machine-size
/// values.
///
/// A planner may advertise support for features such as dynamic scheduling or
/// distributed resources without knowing the number of qubits in advance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlannerCapabilities {
    /// Scheduling execution mode.
    pub execution_mode: PlannerExecutionMode,

    /// Algorithmic family.
    pub algorithm_family: PlannerAlgorithmFamily,

    /// Whether resource constraints are first-class inputs.
    pub resource_aware: bool,

    /// Whether explicit timing constraints are supported.
    pub timing_aware: bool,

    /// Whether dependency graphs are required/supported.
    pub dependency_aware: bool,

    /// Whether runtime conditional dependencies are supported.
    pub conditional: bool,

    /// Whether measurement/classical feedback dependencies are supported.
    pub feedback: bool,

    /// Whether distributed communication resources are supported.
    pub distributed: bool,

    /// Whether QEC scheduling constraints are supported through the context.
    pub qec: bool,

    /// Whether the implementation guarantees deterministic output when the
    /// context requests deterministic scheduling.
    pub deterministic: bool,

    /// Whether the implementation can process symbolic timing information
    /// without first requiring every duration to be concrete.
    pub symbolic_timing: bool,
}

impl PlannerCapabilities {
    /// Creates a conservative static planner capability set.
    #[must_use]
    pub const fn static_default() -> Self {
        Self {
            execution_mode: PlannerExecutionMode::Static,
            algorithm_family: PlannerAlgorithmFamily::List,
            resource_aware: true,
            timing_aware: true,
            dependency_aware: true,
            conditional: false,
            feedback: false,
            distributed: false,
            qec: false,
            deterministic: true,
            symbolic_timing: false,
        }
    }

    /// Returns whether the planner supports the supplied scheduling mode.
    #[must_use]
    pub const fn supports_execution_mode(
        self,
        mode: PlannerExecutionMode,
    ) -> bool {
        match (self.execution_mode, mode) {
            (PlannerExecutionMode::Hybrid, _) => true,
            (PlannerExecutionMode::Static, PlannerExecutionMode::Static) => true,
            (PlannerExecutionMode::Dynamic, PlannerExecutionMode::Dynamic) => true,
            _ => false,
        }
    }
}

impl Default for PlannerCapabilities {
    fn default() -> Self {
        Self::static_default()
    }
}

// =============================================================================
// Planner metadata
// =============================================================================

/// Immutable metadata describing a concrete planner implementation.
///
/// The metadata is intentionally independent of implementation state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlannerMetadata {
    /// Stable planner identifier.
    pub id: PlannerId,

    /// Implementation version.
    pub version: PlannerVersion,

    /// Human-readable planner name.
    pub name: String,

    /// Human-readable description.
    pub description: String,

    /// Declared capabilities.
    pub capabilities: PlannerCapabilities,
}

impl PlannerMetadata {
    /// Creates planner metadata.
    ///
    /// The caller is responsible for providing stable, meaningful metadata.
    pub fn new(
        id: PlannerId,
        version: PlannerVersion,
        name: impl Into<String>,
        description: impl Into<String>,
        capabilities: PlannerCapabilities,
    ) -> Self {
        Self {
            id,
            version,
            name: name.into(),
            description: description.into(),
            capabilities,
        }
    }

    /// Returns the stable planner identifier.
    #[must_use]
    pub fn id(&self) -> &PlannerId {
        &self.id
    }

    /// Returns the implementation version.
    #[must_use]
    pub const fn version(&self) -> PlannerVersion {
        self.version
    }

    /// Returns the implementation capabilities.
    #[must_use]
    pub const fn capabilities(&self) -> PlannerCapabilities {
        self.capabilities
    }
}

// =============================================================================
// Planner execution outcome
// =============================================================================

/// Metadata describing one planner invocation.
///
/// This is intentionally lightweight and does not contain the schedule itself.
/// The immutable schedule remains owned by `scheduling::result`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlannerExecution {
    /// Planner contract version used for the invocation.
    pub contract_version: u32,

    /// Planner implementation version.
    pub implementation_version: PlannerVersion,
}

impl PlannerExecution {
    /// Creates invocation metadata.
    #[must_use]
    pub const fn new(
        implementation_version: PlannerVersion,
    ) -> Self {
        Self {
            contract_version: PLANNER_CONTRACT_VERSION,
            implementation_version,
        }
    }
}

// =============================================================================
// Planner error context
// =============================================================================

/// Structured planner precondition error.
///
/// This type is deliberately not the canonical scheduling error hierarchy.
/// It exists only for planner-specific capability/precondition reporting.
///
/// Implementations may map this information into `SchedulingError` at their
/// integration boundary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlannerPreconditionError {
    /// The planner cannot execute the requested scheduling mode.
    UnsupportedExecutionMode {
        /// Planner identifier.
        planner: PlannerId,

        /// Requested execution mode.
        requested: PlannerExecutionMode,
    },

    /// The planner cannot provide deterministic results under the requested
    /// context.
    DeterminismUnsupported {
        /// Planner identifier.
        planner: PlannerId,
    },

    /// The planner does not support symbolic timing required by the context.
    SymbolicTimingUnsupported {
        /// Planner identifier.
        planner: PlannerId,
    },

    /// The planner cannot process distributed scheduling requirements.
    DistributedSchedulingUnsupported {
        /// Planner identifier.
        planner: PlannerId,
    },

    /// The planner cannot process dynamic conditional scheduling requirements.
    DynamicSchedulingUnsupported {
        /// Planner identifier.
        planner: PlannerId,
    },

    /// The planner cannot process feedback-dependent scheduling requirements.
    FeedbackSchedulingUnsupported {
        /// Planner identifier.
        planner: PlannerId,
    },

    /// The planner cannot process QEC scheduling requirements.
    QecSchedulingUnsupported {
        /// Planner identifier.
        planner: PlannerId,
    },
}

impl fmt::Display for PlannerPreconditionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedExecutionMode {
                planner,
                requested,
            } => {
                write!(
                    formatter,
                    "planner `{planner}` does not support \
                     `{requested}` scheduling"
                )
            }

            Self::DeterminismUnsupported { planner } => {
                write!(
                    formatter,
                    "planner `{planner}` cannot guarantee \
                     deterministic scheduling for this request"
                )
            }

            Self::SymbolicTimingUnsupported { planner } => {
                write!(
                    formatter,
                    "planner `{planner}` does not support symbolic timing"
                )
            }

            Self::DistributedSchedulingUnsupported { planner } => {
                write!(
                    formatter,
                    "planner `{planner}` does not support distributed \
                     scheduling requirements"
                )
            }

            Self::DynamicSchedulingUnsupported { planner } => {
                write!(
                    formatter,
                    "planner `{planner}` does not support dynamic \
                     scheduling requirements"
                )
            }

            Self::FeedbackSchedulingUnsupported { planner } => {
                write!(
                    formatter,
                    "planner `{planner}` does not support feedback-dependent \
                     scheduling requirements"
                )
            }

            Self::QecSchedulingUnsupported { planner } => {
                write!(
                    formatter,
                    "planner `{planner}` does not support QEC scheduling \
                     requirements"
                )
            }
        }
    }
}

impl Error for PlannerPreconditionError {}

// =============================================================================
// Planner trait
// =============================================================================

/// Production scheduling planner contract.
///
/// A concrete planner implements this trait and owns the actual scheduling
/// algorithm.
///
/// The trait deliberately receives an immutable `SchedulingContext`.
///
/// This provides the central architectural invariant:
///
/// ```text
/// immutable scheduling input
///           │
///           ▼
///       planner
///           │
///           ▼
/// immutable scheduling result
/// ```
///
/// The planner must not mutate canonical Quantum IR, routing state, hardware
/// state, global state, or another scheduler instance.
///
/// # Required implementation properties
///
/// A production implementation must:
///
/// 1. preserve quantum semantics;
/// 2. respect dependency constraints;
/// 3. respect resource constraints;
/// 4. respect timing constraints;
/// 5. respect target capabilities;
/// 6. respect explicit scheduling limits;
/// 7. avoid machine-size assumptions;
/// 8. avoid unsafe Rust;
/// 9. avoid hidden global state;
/// 10. honour deterministic scheduling when requested;
/// 11. use checked temporal arithmetic;
/// 12. return structured scheduling errors;
/// 13. produce a canonical `SchedulingResult`;
/// 14. remain independent of hardware-provider I/O;
/// 15. remain independent of routing implementation details;
/// 16. remain independent of frontend syntax.
///
/// # Algorithm independence
///
/// This trait does not prescribe whether an implementation uses:
///
/// - greedy heuristics;
/// - list scheduling;
/// - critical-path scheduling;
/// - RCPSP;
/// - event-driven scheduling;
/// - exact optimization;
/// - approximation;
/// - adaptive heuristics;
/// - plugin-provided algorithms.
///
/// Those are implementation details.
///
/// # Scalability
///
/// There is no qubit-count argument.
///
/// There is no operation-count argument.
///
/// There is no topology-size argument.
///
/// There is no channel-count argument.
///
/// All such information is obtained from the immutable context.
///
/// This is what allows one planner implementation to operate across different
/// target sizes without recompilation around a fixed machine size.
pub trait SchedulingPlanner {
    /// Returns immutable metadata for this planner.
    fn metadata(&self) -> &PlannerMetadata;

    /// Executes the planner against one immutable scheduling context.
    ///
    /// The planner must not mutate `context`.
    ///
    /// A successful result represents a candidate schedule artifact. Canonical
    /// verification remains the responsibility of the verification pipeline.
    ///
    /// Implementations must never return a successful result containing known
    /// dependency/resource/timing violations.
    fn plan(
        &self,
        context: &SchedulingContext,
    ) -> SchedulingResult<ScheduleArtifact>;

    /// Performs planner-specific capability validation before planning.
    ///
    /// The default implementation validates only capabilities represented by
    /// the planner's metadata.
    ///
    /// This method deliberately does not inspect implementation-specific
    /// hardware details.
    fn validate_context(
        &self,
        context: &SchedulingContext,
    ) -> Result<(), PlannerPreconditionError> {
        let metadata = self.metadata();
        let capabilities = metadata.capabilities;

        let requested_mode = planner_execution_mode(context);

        if !capabilities.supports_execution_mode(requested_mode) {
            return Err(
                PlannerPreconditionError::UnsupportedExecutionMode {
                    planner: metadata.id.clone(),
                    requested: requested_mode,
                },
            );
        }

        if context_requests_determinism(context)
            && !capabilities.deterministic
        {
            return Err(
                PlannerPreconditionError::DeterminismUnsupported {
                    planner: metadata.id.clone(),
                },
            );
        }

        if context_requires_symbolic_timing(context)
            && !capabilities.symbolic_timing
        {
            return Err(
                PlannerPreconditionError::SymbolicTimingUnsupported {
                    planner: metadata.id.clone(),
                },
            );
        }

        if context_requires_distributed_scheduling(context)
            && !capabilities.distributed
        {
            return Err(
                PlannerPreconditionError::DistributedSchedulingUnsupported {
                    planner: metadata.id.clone(),
                },
            );
        }

        if context_requires_dynamic_scheduling(context)
            && !capabilities.conditional
        {
            return Err(
                PlannerPreconditionError::DynamicSchedulingUnsupported {
                    planner: metadata.id.clone(),
                },
            );
        }

        if context_requires_feedback_scheduling(context)
            && !capabilities.feedback
        {
            return Err(
                PlannerPreconditionError::FeedbackSchedulingUnsupported {
                    planner: metadata.id.clone(),
                },
            );
        }

        if context_requires_qec_scheduling(context)
            && !capabilities.qec
        {
            return Err(
                PlannerPreconditionError::QecSchedulingUnsupported {
                    planner: metadata.id.clone(),
                },
            );
        }

        Ok(())
    }

    /// Plans after performing the planner's capability precondition checks.
    ///
    /// Implementations normally should not override this method.
    ///
    /// The separation between `validate_context` and `plan` allows registries
    /// and orchestration pipelines to perform capability checks independently
    /// before committing to execution.
    fn plan_checked(
        &self,
        context: &SchedulingContext,
    ) -> SchedulingResult<ScheduleArtifact> {
        if let Err(error) = self.validate_context(context) {
            return Err(map_planner_precondition_error(error));
        }

        self.plan(context)
    }
}

// =============================================================================
// Optional planner lifecycle hook
// =============================================================================

/// Extended planner lifecycle contract.
///
/// This trait is intentionally separate from `SchedulingPlanner`.
///
/// A planner does not need lifecycle hooks to be a valid planner.
///
/// Registries or execution pipelines may use this trait when they need an
/// explicit preparation/finalization phase.
///
/// No mutable global state is implied.
///
/// A lifecycle implementation may use these hooks to prepare local immutable
/// caches or release resources owned by the planner instance.
pub trait SchedulingPlannerLifecycle: SchedulingPlanner {
    /// Called before planning begins.
    ///
    /// The default implementation performs no work.
    fn prepare(
        &self,
        _context: &SchedulingContext,
    ) -> SchedulingResult<()> {
        Ok(())
    }

    /// Called after a planning attempt completes.
    ///
    /// The default implementation performs no work.
    ///
    /// The result is supplied as an optional reference so implementations can
    /// record local diagnostics without mutating the result artifact.
    fn finalize(
        &self,
        _context: &SchedulingContext,
        _result: Option<&ScheduleArtifact>,
    ) -> SchedulingResult<()> {
        Ok(())
    }

    /// Performs a complete lifecycle-managed planning operation.
    ///
    /// The planner remains responsible for producing the schedule itself.
    fn plan_with_lifecycle(
        &self,
        context: &SchedulingContext,
    ) -> SchedulingResult<ScheduleArtifact> {
        self.prepare(context)?;

        let result = self.plan_checked(context);

        match &result {
            Ok(schedule) => {
                self.finalize(context, Some(schedule))?;
            }

            Err(_) => {
                self.finalize(context, None)?;
            }
        }

        result
    }
}

// =============================================================================
// Planner registry compatibility contract
// =============================================================================

/// Minimal catalog interface required by planner selection.
///
/// This trait intentionally does not own planner implementations.
///
/// A registry module can implement this contract later without modifying
/// `SchedulingPlanner`.
///
/// The catalog returns immutable references, allowing the scheduling pipeline
/// to inspect planner metadata without taking ownership.
pub trait PlannerCatalog {
    /// Returns the planner registered under `id`, if present.
    fn planner(
        &self,
        id: &PlannerId,
    ) -> Option<&dyn SchedulingPlanner>;

    /// Returns whether a planner is registered under `id`.
    fn contains(
        &self,
        id: &PlannerId,
    ) -> bool {
        self.planner(id).is_some()
    }
}

// =============================================================================
// Planner selection request
// =============================================================================

/// Stable planner-selection request.
///
/// This is deliberately smaller than `SchedulingContext`.
///
/// The request identifies an explicit planner when one has been selected.
///
/// It does not contain:
///
/// - qubit counts;
/// - operation counts;
/// - resource counts;
/// - hardware identifiers;
/// - topology definitions.
///
/// Those remain in the context.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlannerSelection {
    /// Let the scheduling orchestration layer choose a suitable planner.
    Automatic,

    /// Request a specific registered planner.
    Explicit(PlannerId),
}

impl Default for PlannerSelection {
    fn default() -> Self {
        Self::Automatic
    }
}

impl fmt::Display for PlannerSelection {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Automatic => {
                formatter.write_str("automatic")
            }

            Self::Explicit(id) => {
                write!(formatter, "explicit:{id}")
            }
        }
    }
}

// =============================================================================
// Planner compatibility report
// =============================================================================

/// Structured capability compatibility report.
///
/// This report is useful to registries and adaptive planner selection.
///
/// It does not execute a planner.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlannerCompatibility {
    /// Planner metadata.
    pub planner: PlannerId,

    /// Whether the planner is compatible with the context.
    pub compatible: bool,

    /// Reasons preventing compatibility.
    pub reasons: Vec<PlannerPreconditionError>,
}

impl PlannerCompatibility {
    /// Creates a successful compatibility report.
    #[must_use]
    pub fn compatible(
        planner: PlannerId,
    ) -> Self {
        Self {
            planner,
            compatible: true,
            reasons: Vec::new(),
        }
    }

    /// Creates an incompatible compatibility report.
    #[must_use]
    pub fn incompatible(
        planner: PlannerId,
        reasons: Vec<PlannerPreconditionError>,
    ) -> Self {
        Self {
            planner,
            compatible: false,
            reasons,
        }
    }

    /// Returns whether the planner is compatible.
    #[must_use]
    pub const fn is_compatible(&self) -> bool {
        self.compatible
    }

    /// Returns compatibility reasons.
    #[must_use]
    pub fn reasons(&self) -> &[PlannerPreconditionError] {
        &self.reasons
    }
}

// =============================================================================
// Planner inspection helpers
// =============================================================================

/// Inspects planner compatibility against a context.
///
/// This function performs no scheduling.
///
/// It is intentionally independent from a registry so a caller can inspect an
/// individual planner before registration or execution.
#[must_use]
pub fn inspect_planner(
    planner: &dyn SchedulingPlanner,
    context: &SchedulingContext,
) -> PlannerCompatibility {
    let metadata = planner.metadata();

    match planner.validate_context(context) {
        Ok(()) => PlannerCompatibility::compatible(
            metadata.id.clone(),
        ),

        Err(reason) => PlannerCompatibility::incompatible(
            metadata.id.clone(),
            vec![reason],
        ),
    }
}

/// Returns planner metadata without exposing implementation state.
///
/// This helper is useful for registries, diagnostics, and API layers.
#[must_use]
pub fn planner_metadata(
    planner: &dyn SchedulingPlanner,
) -> &PlannerMetadata {
    planner.metadata()
}

// =============================================================================
// Context capability adapters
// =============================================================================
//
// IMPORTANT:
//
// These functions intentionally form a narrow compatibility layer around the
// current SchedulingContext contract.
//
// They are kept private because planner implementations should consume the
// public context contract rather than depending on these helper names.
//
// If SchedulingContext later gains richer capability-query methods, these
// helpers can be changed in one place without changing SchedulingPlanner.
//
// No qubit/resource counts are embedded here.
//
// =============================================================================

/// Determines the execution mode requested by the context.
///
/// The context is the authoritative source. This function deliberately does
/// not inspect implementation details of a planner.
fn planner_execution_mode(
    _context: &SchedulingContext,
) -> PlannerExecutionMode {
    // The current SchedulingContext contract exposes scheduling-mode semantics,
    // while the exact public accessor may evolve as the context contract is
    // finalized.
    //
    // The conservative default is Static because static scheduling is the
    // least permissive interpretation and cannot silently claim dynamic
    // capability.
    //
    // Dynamic-aware concrete planners may override `validate_context` and
    // obtain the richer context information through the finalized context API.
    PlannerExecutionMode::Static
}

/// Returns whether deterministic scheduling is requested.
///
/// The immutable SchedulingContext is the source of truth.
///
/// This conservative implementation returns true because the repository's
/// scheduling architecture defaults to deterministic compilation.
fn context_requests_determinism(
    _context: &SchedulingContext,
) -> bool {
    true
}

/// Returns whether symbolic timing is required by the context.
///
/// This conservative default avoids silently permitting unresolved timing in
/// planners that have not explicitly declared support.
fn context_requires_symbolic_timing(
    _context: &SchedulingContext,
) -> bool {
    false
}

/// Returns whether distributed scheduling is required by the context.
///
/// The final context capability API may provide a richer query. Until that
/// query exists, the planner contract does not infer distributed execution
/// from arbitrary target strings or vendor metadata.
fn context_requires_distributed_scheduling(
    _context: &SchedulingContext,
) -> bool {
    false
}

/// Returns whether dynamic scheduling is required by the context.
///
/// This remains conservative until the dynamic-control requirement is exposed
/// through the finalized SchedulingContext API.
fn context_requires_dynamic_scheduling(
    _context: &SchedulingContext,
) -> bool {
    false
}

/// Returns whether classical feedback scheduling is required by the context.
fn context_requires_feedback_scheduling(
    _context: &SchedulingContext,
) -> bool {
    false
}

/// Returns whether QEC scheduling is required by the context.
fn context_requires_qec_scheduling(
    _context: &SchedulingContext,
) -> bool {
    false
}

// =============================================================================
// Error mapping
// =============================================================================
//
// Planner-specific capability failures must cross into the canonical
// SchedulingError boundary.
//
// The exact canonical error representation belongs to errors.rs.
//
// Keeping this conversion here means concrete planners do not need to know
// about the representation of the top-level scheduler error hierarchy.
// =============================================================================

fn map_planner_precondition_error(
    error: PlannerPreconditionError,
) -> SchedulingError {
    SchedulingError::InvalidInput {
        reason: error.to_string(),
    }
}

// =============================================================================
// Blanket utility implementation
// =============================================================================

/// Allows immutable references to planners to be used as planners.
///
/// This makes registry/pipeline composition ergonomic without requiring
/// wrappers or unsafe delegation.
impl<T> SchedulingPlanner for &T
where
    T: SchedulingPlanner + ?Sized,
{
    fn metadata(&self) -> &PlannerMetadata {
        (**self).metadata()
    }

    fn plan(
        &self,
        context: &SchedulingContext,
    ) -> SchedulingResult<ScheduleArtifact> {
        (**self).plan(context)
    }

    fn validate_context(
        &self,
        context: &SchedulingContext,
    ) -> Result<(), PlannerPreconditionError> {
        (**self).validate_context(context)
    }

    fn plan_checked(
        &self,
        context: &SchedulingContext,
    ) -> SchedulingResult<ScheduleArtifact> {
        (**self).plan_checked(context)
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    struct TestPlanner {
        metadata: PlannerMetadata,
    }

    impl TestPlanner {
        fn new() -> Self {
            let id = PlannerId::new("test.planner")
                .expect("test planner identifier must be valid");

            Self {
                metadata: PlannerMetadata::new(
                    id,
                    PlannerVersion::new(1, 0, 0),
                    "Test Planner",
                    "Planner used only for contract tests.",
                    PlannerCapabilities {
                        execution_mode: PlannerExecutionMode::Static,
                        algorithm_family:
                            PlannerAlgorithmFamily::List,
                        resource_aware: true,
                        timing_aware: true,
                        dependency_aware: true,
                        conditional: false,
                        feedback: false,
                        distributed: false,
                        qec: false,
                        deterministic: true,
                        symbolic_timing: false,
                    },
                ),
            }
        }
    }

    impl SchedulingPlanner for TestPlanner {
        fn metadata(&self) -> &PlannerMetadata {
            &self.metadata
        }

        fn plan(
            &self,
            _context: &SchedulingContext,
        ) -> SchedulingResult<ScheduleArtifact> {
            Err(SchedulingError::InvalidInput {
                reason: "contract test planner does not execute schedules"
                    .to_string(),
            })
        }
    }

    #[test]
    fn planner_identifier_rejects_empty_values() {
        let result = PlannerId::new("");

        assert!(matches!(
            result,
            Err(PlannerIdError::Empty)
        ));
    }

    #[test]
    fn planner_identifier_rejects_invalid_characters() {
        let result = PlannerId::new("planner with spaces");

        assert!(matches!(
            result,
            Err(PlannerIdError::InvalidCharacter)
        ));
    }

    #[test]
    fn planner_identifier_accepts_stable_identifier_syntax() {
        let id = PlannerId::new(
            "zamani.quantum.scheduling.resource_constrained",
        )
        .expect("identifier should be valid");

        assert_eq!(
            id.as_str(),
            "zamani.quantum.scheduling.resource_constrained"
        );
    }

    #[test]
    fn planner_version_is_deterministic() {
        let version = PlannerVersion::new(2, 7, 11);

        assert_eq!(version.major(), 2);
        assert_eq!(version.minor(), 7);
        assert_eq!(version.patch(), 11);
        assert_eq!(version.to_string(), "2.7.11");
    }

    #[test]
    fn static_capabilities_support_static_mode() {
        let capabilities =
            PlannerCapabilities::static_default();

        assert!(
            capabilities.supports_execution_mode(
                PlannerExecutionMode::Static
            )
        );

        assert!(
            !capabilities.supports_execution_mode(
                PlannerExecutionMode::Dynamic
            )
        );
    }

    #[test]
    fn hybrid_capabilities_support_all_execution_modes() {
        let capabilities = PlannerCapabilities {
            execution_mode: PlannerExecutionMode::Hybrid,
            ..PlannerCapabilities::static_default()
        };

        assert!(
            capabilities.supports_execution_mode(
                PlannerExecutionMode::Static
            )
        );

        assert!(
            capabilities.supports_execution_mode(
                PlannerExecutionMode::Dynamic
            )
        );

        assert!(
            capabilities.supports_execution_mode(
                PlannerExecutionMode::Hybrid
            )
        );
    }

    #[test]
    fn planner_metadata_is_stable() {
        let planner = TestPlanner::new();

        assert_eq!(
            planner.metadata().id.as_str(),
            "test.planner"
        );

        assert_eq!(
            planner.metadata().version,
            PlannerVersion::new(1, 0, 0)
        );
    }

    #[test]
    fn planner_reference_delegates_to_concrete_planner() {
        let planner = TestPlanner::new();
        let reference = &planner;

        assert_eq!(
            reference.metadata().id.as_str(),
            planner.metadata().id.as_str()
        );
    }
}