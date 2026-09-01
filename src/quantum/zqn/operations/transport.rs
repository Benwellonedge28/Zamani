//! Zamani Quantum Noise (ZQN)
//! Transport-operation noise semantics.
//!
//! `src/quantum/zqn/operations/transport.rs`
//!
//! # Purpose
//!
//! This module defines the ZQN-side semantic description of quantum-resource
//! transport and the noise context associated with that transport.
//!
//! Transport is deliberately treated as an operation boundary rather than as
//! a routing algorithm or hardware API.
//!
//! The central question answered here is:
//!
//! > "What quantum resources are being transported, from where to where, for
//! > how long, by which abstract transport semantics, and which ZQN noise
//! > model is associated with that transport?"
//!
//! This module does NOT answer:
//!
//! - which route a router should choose;
//! - whether a physical path actually exists;
//! - how a device moves a quantum state;
//! - how a vendor API performs transport;
//! - how a simulator evolves a state;
//! - how a fault-tolerant code corrects transport errors.
//!
//! Those responsibilities belong to routing, target/hardware integration,
//! simulation/runtime, and QEC respectively.
//!
//! # Critical ownership boundary
//!
//! Canonical quantum-resource identities remain owned by the canonical
//! quantum IR.
//!
//! In particular:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! crate::quantum::ir::identity::OperationId
//! ```
//!
//! are reused directly here.
//!
//! ZQN MUST NOT define another `QubitId`, `PhysicalQubitId`, or `OperationId`.
//!
//! This is consistent with the repository's ZQN operation contract, which
//! explicitly establishes canonical IR identity ownership rather than
//! duplicating quantum identities inside ZQN.
//!
//! # Architectural position
//!
//! ```text
//!                         Zamani source
//!                              |
//!                              v
//!                         quantum::ir
//!                              |
//!                              v
//!                    transport semantic intent
//!                              |
//!                              v
//!              zqn::operations::transport
//!                              |
//!              +---------------+----------------+
//!              |               |                |
//!              v               v                v
//!            noise         calibration       provenance
//!              |               |                |
//!              +---------------+----------------+
//!                              |
//!                              v
//!                       integration layer
//!                              |
//!               +--------------+---------------+
//!               |              |               |
//!               v              v               v
//!            routing       scheduling       hardware
//!               |              |               |
//!               +--------------+---------------+
//!                              |
//!                              v
//!                            runtime
//!                              |
//!                              v
//!                           simulator
//!                              |
//!                              v
//!                              QEC
//! ```
//!
//! # Ownership
//!
//! This file owns:
//!
//! - transport-operation semantic data;
//! - transport-resource references;
//! - transport endpoints;
//! - transport route/segment descriptions;
//! - transport duration;
//! - transport classification;
//! - optional noise-model association;
//! - structural validation local to transport semantics;
//! - deterministic transport metadata;
//! - immutable accessors;
//! - transport-specific error reporting;
//! - conversion-free integration data that downstream ZQN modules can consume.
//!
//! # Does not own
//!
//! This file does NOT own:
//!
//! - canonical quantum IR;
//! - qubit identity;
//! - physical topology;
//! - routing algorithms;
//! - shortest-path algorithms;
//! - scheduling;
//! - hardware drivers;
//! - vendor APIs;
//! - network transports;
//! - quantum state evolution;
//! - channel mathematics;
//! - probability mathematics;
//! - RNG;
//! - calibration values;
//! - calibration storage;
//! - QEC decoding;
//! - syndrome generation;
//! - benchmarking methodology;
//! - serialization wire formats;
//! - global registries;
//! - global mutable state.
//!
//! # Why transport belongs in `operations`
//!
//! Transport can create noise even when no gate is executed.
//!
//! Examples include:
//!
//! - ion shuttling;
//! - movement between trapping zones;
//! - photonic transmission;
//! - quantum-memory transfer;
//! - resonator transfer;
//! - quantum-network links;
//! - mode transfer;
//! - state movement between physical resources;
//! - future quantum-resource movement mechanisms.
//!
//! Therefore transport cannot be represented safely as merely another gate.
//!
//! A transport interval may introduce:
//!
//! - loss;
//! - dephasing;
//! - leakage;
//! - thermal noise;
//! - timing-dependent noise;
//! - correlated environmental noise;
//! - distance-dependent noise;
//! - path-dependent noise;
//! - transport-induced crosstalk;
//! - calibration-dependent error.
//!
//! The operation model must therefore preserve transport semantics without
//! prematurely deciding how a particular target realizes them.
//!
//! # Write-once / scale-everywhere principle
//!
//! The same Zamani program must be able to describe transport independently
//! of whether the eventual target contains:
//!
//! - one resource;
//! - thousands of resources;
//! - millions of logical resources;
//! - distributed quantum resources;
//! - multiple quantum technologies;
//! - heterogeneous devices;
//! - future quantum architectures.
//!
//! Consequently this file contains no semantic constants such as:
//!
//! ```text
//! MAX_QUBITS
//! MAX_TRANSPORTS
//! MAX_ROUTE_LENGTH
//! MAX_PATH_LENGTH
//! MAX_RESOURCES
//! ```
//!
//! No finite machine-size ceiling is encoded here.
//!
//! Any resource ceiling belongs to an explicit execution/resource policy.
//!
//! "Infinity" therefore means:
//!
//! ```text
//! no artificial semantic upper bound
//! ```
//!
//! rather than a claim that a concrete machine or process can materialize an
//! infinite computation.
//!
//! # Ordering
//!
//! Transport ordering is semantic.
//!
//! A transport path is therefore stored in the exact order supplied by its
//! producer.
//!
//! This module MUST NOT sort route segments.
//!
//! This matters because:
//!
//! ```text
//! A -> B -> C
//! ```
//!
//! is not equivalent to:
//!
//! ```text
//! A -> C -> B
//! ```
//!
//! even when the same resources occur in both paths.
//!
//! # Identity-domain policy
//!
//! A transport resource may be:
//!
//! - a canonical logical qubit;
//! - a canonical physical qubit;
//! - an explicitly named non-qubit quantum resource.
//!
//! Logical and physical identity are deliberately not conflated.
//!
//! Mapping a logical resource to a physical resource belongs to the canonical
//! IR mapping/routing boundary.
//!
//! This module may represent either side because transport can exist at
//! different abstraction levels, but it does not perform the mapping.
//!
//! # Determinism
//!
//! This module contains no randomness.
//!
//! It does not:
//!
//! - create an RNG;
//! - consume an RNG;
//! - derive values from wall-clock time;
//! - use thread-local randomness;
//! - depend on thread count;
//! - use process-global state.
//!
//! A transport operation therefore has deterministic value semantics.
//!
//! Stochastic transport noise is selected or sampled later by the ZQN noise
//! and simulation layers using the explicit deterministic execution context.
//!
//! # Resource safety
//!
//! This module does not impose architectural resource limits.
//!
//! Collections are dynamically sized and therefore scale with available
//! resources.
//!
//! Constructors validate structural correctness but do not reject a transport
//! merely because it contains a large number of resources.
//!
//! A caller that accepts untrusted data MUST enforce explicit resource limits
//! before allocating arbitrarily large collections.
//!
//! This is intentionally separated from semantic validity:
//!
//! ```text
//! semantic validity
//!         !=
//! deployment resource policy
//! ```
//!
//! # Security
//!
//! Transport descriptions are data, not executable instructions.
//!
//! A transport operation MUST NOT:
//!
//! - open a network connection;
//! - access a device;
//! - execute a vendor command;
//! - access credentials;
//! - mutate hardware;
//! - invoke a process;
//! - allocate a quantum state.
//!
//! Downstream consumers must treat all textual resource names and metadata as
//! untrusted data where applicable.
//!
//! # Numerical safety
//!
//! Duration values are represented by the existing ZQN
//! `operations::operation::OperationDuration` type.
//!
//! That type already rejects non-finite and negative duration values.
//!
//! This module therefore does not silently coerce:
//!
//! ```text
//! NaN       -> 0
//! Infinity  -> maximum
//! negative  -> absolute value
//! ```
//!
//! # Calibration
//!
//! Calibration data is not owned here.
//!
//! A transport operation can carry an optional calibration identity so that a
//! downstream calibration layer can resolve the appropriate parameters for
//! the operation and time interval.
//!
//! The calibration identifier is a reference, not the calibration itself.
//!
//! # Noise integration
//!
//! A transport operation can optionally identify a reusable ZQN noise model.
//!
//! The noise model is NOT stored as a concrete executable object here.
//!
//! This keeps the transport value:
//!
//! - serializable by the ZQN I/O layer;
//! - deterministic;
//! - independent of registry implementation;
//! - independent of simulator implementation;
//! - independent of backend implementation.
//!
//! The intended flow is:
//!
//! ```text
//! TransportOperation
//!        |
//!        +---- NoiseModelId
//!        |
//!        v
//! ZQN noise registry/model resolution
//!        |
//!        v
//! NoiseApplication
//!        |
//!        v
//! channel/fault realization
//!        |
//!        v
//! runtime/simulator/QEC
//! ```
//!
//! # Routing integration
//!
//! Routing owns path selection.
//!
//! This module only represents a path once one is supplied.
//!
//! Therefore a router can:
//!
//! 1. construct candidate transport descriptions;
//! 2. ask ZQN for transport-related costs/noise;
//! 3. compare candidates;
//! 4. select a route;
//! 5. pass the selected transport operation downstream.
//!
//! `TransportOperation` does not calculate shortest paths.
//!
//! # Scheduling integration
//!
//! Scheduling owns placement in time.
//!
//! `TransportOperation` owns only the semantic transport duration supplied by
//! the producer.
//!
//! A scheduler may use this duration when determining:
//!
//! - resource occupation;
//! - idle exposure;
//! - crosstalk windows;
//! - calibration validity;
//! - temporal noise;
//! - transport overlap.
//!
//! This file does not assign a start time.
//!
//! # Hardware integration
//!
//! Hardware adapters consume transport semantics and determine whether the
//! target can realize them.
//!
//! Hardware-specific lowering belongs outside this file.
//!
//! For example:
//!
//! ```text
//! abstract transport
//!        |
//!        v
//! target capability validation
//!        |
//!        v
//! target lowering
//!        |
//!        v
//! vendor/device transport
//! ```
//!
//! # Simulation integration
//!
//! Simulation consumes this semantic operation and its associated noise model.
//!
//! This file does not mutate a quantum state.
//!
//! # QEC integration
//!
//! Transport-induced noise can eventually be converted into physical faults
//! through `zqn::integration::qec`.
//!
//! QEC remains responsible for:
//!
//! - code definitions;
//! - syndrome extraction;
//! - decoding;
//! - correction;
//! - logical-error analysis.
//!
//! # Benchmarking integration
//!
//! Benchmarking can identify transport operations using:
//!
//! - operation identity;
//! - transport kind;
//! - resource identity;
//! - route length;
//! - duration;
//! - model identity.
//!
//! Benchmarking owns benchmark methodology, not this semantic structure.
//!
//! # Serialization
//!
//! This module intentionally does not define a wire format.
//!
//! `zqn::io` owns serialization and schema compatibility.
//!
//! A serialized transport operation must preserve, at minimum:
//!
//! - operation identity;
//! - transport kind;
//! - source/destination resources;
//! - route ordering;
//! - duration;
//! - optional noise-model identity;
//! - optional calibration identity;
//! - schema/version information supplied by the ZQN I/O layer.
//!
//! Rust field layout MUST NOT be treated as the external serialization
//! contract.
//!
//! # Thread safety
//!
//! All owned state is immutable after construction.
//!
//! The types in this module contain no interior mutability or global mutable
//! state and are therefore naturally safe to share when their contained
//! standard-library types are shared.
//!
//! # Rust compatibility
//!
//! Required:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! `#![forbid(unsafe_code)]` makes the no-unsafe requirement compiler-enforced.
//!
//! # Implementation
//!
//! The implementation below intentionally depends only on:
//!
//! - the Rust standard library;
//! - canonical Zamani IR identity types;
//! - existing ZQN operation duration;
//! - existing ZQN noise-model identity;
//! - existing ZQN calibration identity.
//!
//! It does not depend on concrete channel, simulator, router, scheduler or
//! hardware implementations.
//!
//! This keeps the file independently completable and prevents downstream
//! additions from requiring semantic rewrites here.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::fmt;
use std::sync::Arc;

use crate::quantum::ir::identity::OperationId;
use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId};
use crate::quantum::zqn::core::ids::{CalibrationId, NoiseModelId};
use crate::quantum::zqn::operations::operation::OperationDuration;

// ============================================================================
// Version
// ============================================================================

/// Semantic representation version for transport operations.
///
/// This is a representation marker only.
///
/// It is NOT a machine-size limit, route-length limit, resource limit, or
/// execution limit.
///
/// Global ZQN schema compatibility remains owned by `zqn::core::version` and
/// `zqn::io`.
pub const TRANSPORT_OPERATION_MODEL_VERSION: u16 = 1;

// ============================================================================
// Result / errors
// ============================================================================

/// Result type for transport-operation construction and validation.
pub type TransportResult<T> = Result<T, TransportError>;

/// Errors produced by the transport semantic boundary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransportError {
    /// A required source resource was not supplied.
    MissingSource,

    /// A required destination resource was not supplied.
    MissingDestination,

    /// A transport path contains a malformed segment.
    InvalidPath {
        /// Stable reason for the local structural failure.
        reason: &'static str,
    },

    /// A route contains a segment whose endpoints do not connect.
    DisconnectedPath {
        /// Zero-based segment index at which the discontinuity begins.
        segment: usize,
    },

    /// The supplied operation identity is inconsistent with the caller's
    /// transport contract.
    InvalidOperation {
        /// Stable reason for the failure.
        reason: &'static str,
    },

    /// An empty textual resource namespace was supplied.
    EmptyNamespace,

    /// An empty textual resource identifier was supplied.
    EmptyResourceId,

    /// A textual identifier contains unsupported characters.
    InvalidIdentifier {
        /// Name of the field being validated.
        field: &'static str,
    },

    /// The operation has a semantically invalid duration.
    InvalidDuration {
        /// Stable reason for the failure.
        reason: &'static str,
    },

    /// A transport kind requires a resource relationship that was not
    /// supplied.
    InvalidTransportKind {
        /// Stable reason for the failure.
        reason: &'static str,
    },

    /// The transport operation violates a structural invariant.
    InvalidStructure {
        /// Stable reason for the failure.
        reason: &'static str,
    },
}

impl fmt::Display for TransportError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingSource => {
                formatter.write_str("transport operation requires a source resource")
            }
            Self::MissingDestination => {
                formatter.write_str("transport operation requires a destination resource")
            }
            Self::InvalidPath { reason } => {
                write!(formatter, "invalid transport path: {reason}")
            }
            Self::DisconnectedPath { segment } => {
                write!(
                    formatter,
                    "transport path is disconnected before segment {segment}"
                )
            }
            Self::InvalidOperation { reason } => {
                write!(formatter, "invalid transport operation: {reason}")
            }
            Self::EmptyNamespace => {
                formatter.write_str("transport resource namespace must not be empty")
            }
            Self::EmptyResourceId => {
                formatter.write_str("transport resource identifier must not be empty")
            }
            Self::InvalidIdentifier { field } => {
                write!(formatter, "transport {field} contains invalid characters")
            }
            Self::InvalidDuration { reason } => {
                write!(formatter, "invalid transport duration: {reason}")
            }
            Self::InvalidTransportKind { reason } => {
                write!(formatter, "invalid transport kind: {reason}")
            }
            Self::InvalidStructure { reason } => {
                write!(formatter, "invalid transport structure: {reason}")
            }
        }
    }
}

impl std::error::Error for TransportError {}

// ============================================================================
// Transport kind
// ============================================================================

/// Abstract semantic classification of transport.
///
/// These values describe what the operation means, not how a hardware target
/// implements it.
///
/// New hardware technologies do not require adding a new enum variant merely
/// because they use a new physical mechanism. `Custom` exists for genuinely
/// new semantic classes.
///
/// Vendor names MUST NOT be embedded here.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TransportKind {
    /// Physical movement of a quantum resource.
    Move,

    /// Transfer of quantum information/resource state between resources.
    Transfer,

    /// Transport associated with a quantum communication link.
    Communication,

    /// Movement through a sequence of physical or logical locations.
    Shuttle,

    /// Exchange/repositioning operation involving quantum resources.
    Exchange,

    /// A target-independent custom transport semantic.
    Custom(String),
}

impl TransportKind {
    /// Creates a custom transport kind.
    pub fn custom(value: impl Into<String>) -> TransportResult<Self> {
        let value = validate_identifier(value.into(), "transport kind")?;
        Ok(Self::Custom(value))
    }

    /// Returns a stable semantic name.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Move => "move",
            Self::Transfer => "transfer",
            Self::Communication => "communication",
            Self::Shuttle => "shuttle",
            Self::Exchange => "exchange",
            Self::Custom(value) => value.as_str(),
        }
    }

    /// Returns whether this is a communication transport.
    #[must_use]
    pub const fn is_communication(&self) -> bool {
        matches!(self, Self::Communication)
    }

    /// Returns whether this is a movement/shuttling transport.
    #[must_use]
    pub const fn is_movement(&self) -> bool {
        matches!(self, Self::Move | Self::Shuttle)
    }
}

impl fmt::Display for TransportKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// Transport resource
// ============================================================================

/// Quantum resource participating in transport.
///
/// Canonical logical and physical qubit identities are used directly from
/// `quantum::ir::qubit`.
///
/// `Named` exists so transport is not permanently restricted to qubits. It
/// can represent future resource types such as modes, resonators, memory
/// locations, network endpoints, traps, zones, links, or other semantic
/// quantum resources without changing the canonical qubit identity model.
///
/// `Named` is an opaque semantic reference. It does NOT assert that the
/// referenced resource physically exists.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TransportResource {
    /// Canonical logical qubit.
    LogicalQubit(QubitId),

    /// Canonical physical qubit.
    PhysicalQubit(PhysicalQubitId),

    /// Generic named quantum resource.
    Named {
        /// Namespace owned by the producer/target abstraction.
        namespace: String,

        /// Resource identifier within that namespace.
        id: String,
    },
}

impl TransportResource {
    /// Creates a logical-qubit transport resource.
    #[must_use]
    pub const fn logical_qubit(qubit: QubitId) -> Self {
        Self::LogicalQubit(qubit)
    }

    /// Creates a physical-qubit transport resource.
    #[must_use]
    pub const fn physical_qubit(qubit: PhysicalQubitId) -> Self {
        Self::PhysicalQubit(qubit)
    }

    /// Creates an opaque named transport resource.
    pub fn named(
        namespace: impl Into<String>,
        id: impl Into<String>,
    ) -> TransportResult<Self> {
        let namespace = validate_identifier(namespace.into(), "resource namespace")?;
        let id = validate_identifier(id.into(), "resource id")?;

        Ok(Self::Named { namespace, id })
    }

    /// Returns true when this resource is a logical qubit.
    #[must_use]
    pub const fn is_logical_qubit(&self) -> bool {
        matches!(self, Self::LogicalQubit(_))
    }

    /// Returns true when this resource is a physical qubit.
    #[must_use]
    pub const fn is_physical_qubit(&self) -> bool {
        matches!(self, Self::PhysicalQubit(_))
    }

    /// Returns true when this resource is an opaque named resource.
    #[must_use]
    pub const fn is_named(&self) -> bool {
        matches!(self, Self::Named { .. })
    }

    /// Returns the logical qubit when this resource represents one.
    #[must_use]
    pub const fn logical_qubit_id(&self) -> Option<QubitId> {
        match self {
            Self::LogicalQubit(id) => Some(*id),
            _ => None,
        }
    }

    /// Returns the physical qubit when this resource represents one.
    #[must_use]
    pub const fn physical_qubit_id(&self) -> Option<PhysicalQubitId> {
        match self {
            Self::PhysicalQubit(id) => Some(*id),
            _ => None,
        }
    }
}

impl fmt::Display for TransportResource {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LogicalQubit(id) => write!(formatter, "logical-qubit:{id}"),
            Self::PhysicalQubit(id) => write!(formatter, "physical-qubit:{id}"),
            Self::Named { namespace, id } => {
                write!(formatter, "{namespace}:{id}")
            }
        }
    }
}

// ============================================================================
// Transport segment
// ============================================================================

/// One ordered segment of a transport path.
///
/// A segment is a semantic relationship only.
///
/// It does not assert that the target topology actually permits the movement.
///
/// Topology validation belongs to routing/target integration.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TransportSegment {
    source: TransportResource,
    destination: TransportResource,
}

impl TransportSegment {
    /// Creates one transport segment.
    ///
    /// Source and destination may be equal because identity transport can be
    /// meaningful for calibration/characterization or an explicitly modelled
    /// zero-distance operation.
    pub fn new(
        source: TransportResource,
        destination: TransportResource,
    ) -> TransportResult<Self> {
        Ok(Self {
            source,
            destination,
        })
    }

    /// Returns the source resource.
    #[must_use]
    pub const fn source(&self) -> &TransportResource {
        &self.source
    }

    /// Returns the destination resource.
    #[must_use]
    pub const fn destination(&self) -> &TransportResource {
        &self.destination
    }

    /// Returns whether source and destination identify the same semantic
    /// resource.
    #[must_use]
    pub fn is_identity(&self) -> bool {
        self.source == self.destination
    }
}

// ============================================================================
// Transport path
// ============================================================================

/// Ordered transport path.
///
/// A path is represented as a sequence of connected transport segments.
///
/// The sequence is deliberately retained in producer order.
///
/// No route-finding or topology validation is performed here.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TransportPath {
    segments: Arc<[TransportSegment]>,
}

impl TransportPath {
    /// Creates a path from an ordered collection of segments.
    ///
    /// The collection may be empty.
    ///
    /// An empty path is useful for a direct source/destination transport,
    /// because the source and destination are separately represented by the
    /// enclosing operation.
    ///
    /// When segments are supplied, they must form a connected ordered chain.
    pub fn new(segments: impl Into<Vec<TransportSegment>>) -> TransportResult<Self> {
        let segments = segments.into();

        validate_path_connectivity(&segments)?;

        Ok(Self {
            segments: Arc::from(segments),
        })
    }

    /// Creates an empty/direct path.
    #[must_use]
    pub fn direct() -> Self {
        Self {
            segments: Arc::from([]),
        }
    }

    /// Returns the ordered path segments.
    #[must_use]
    pub fn segments(&self) -> &[TransportSegment] {
        &self.segments
    }

    /// Returns the number of path segments.
    #[must_use]
    pub fn len(&self) -> usize {
        self.segments.len()
    }

    /// Returns whether this path contains no intermediate segments.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.segments.is_empty()
    }

    /// Returns the first resource in the path when present.
    #[must_use]
    pub fn first_source(&self) -> Option<&TransportResource> {
        self.segments.first().map(TransportSegment::source)
    }

    /// Returns the final resource in the path when present.
    #[must_use]
    pub fn final_destination(&self) -> Option<&TransportResource> {
        self.segments.last().map(TransportSegment::destination)
    }
}

// ============================================================================
// Transport operation
// ============================================================================

/// Immutable semantic transport operation.
///
/// This is the principal public type of this module.
///
/// It describes:
///
/// - canonical operation identity;
/// - abstract transport kind;
/// - source;
/// - destination;
/// - optional intermediate route;
/// - duration;
/// - optional noise model;
/// - optional calibration identity.
///
/// It does not execute anything.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TransportOperation {
    operation_id: OperationId,
    kind: TransportKind,
    source: TransportResource,
    destination: TransportResource,
    path: TransportPath,
    duration: OperationDuration,
    noise_model_id: Option<NoiseModelId>,
    calibration_id: Option<CalibrationId>,
}

impl TransportOperation {
    /// Creates a transport operation.
    ///
    /// # Validation
    ///
    /// The constructor validates only invariants owned by this module:
    ///
    /// - source is present;
    /// - destination is present;
    /// - path is structurally connected;
    /// - path endpoints, when present, agree with source/destination;
    /// - duration is already valid through `OperationDuration`;
    /// - transport kind is structurally valid.
    ///
    /// It does NOT validate:
    ///
    /// - target topology;
    /// - hardware availability;
    /// - calibration validity;
    /// - noise-model compatibility;
    /// - routing optimality.
    ///
    /// Those belong to their respective subsystems.
    pub fn new(
        operation_id: OperationId,
        kind: TransportKind,
        source: TransportResource,
        destination: TransportResource,
        path: TransportPath,
        duration: OperationDuration,
    ) -> TransportResult<Self> {
        validate_transport_kind(&kind)?;

        if let Some(path_source) = path.first_source() {
            if path_source != &source {
                return Err(TransportError::InvalidStructure {
                    reason: "path source does not match operation source",
                });
            }
        }

        if let Some(path_destination) = path.final_destination() {
            if path_destination != &destination {
                return Err(TransportError::InvalidStructure {
                    reason: "path destination does not match operation destination",
                });
            }
        }

        Ok(Self {
            operation_id,
            kind,
            source,
            destination,
            path,
            duration,
            noise_model_id: None,
            calibration_id: None,
        })
    }

    /// Creates a direct transport operation without intermediate path
    /// segments.
    pub fn direct(
        operation_id: OperationId,
        kind: TransportKind,
        source: TransportResource,
        destination: TransportResource,
        duration: OperationDuration,
    ) -> TransportResult<Self> {
        Self::new(
            operation_id,
            kind,
            source,
            destination,
            TransportPath::direct(),
            duration,
        )
    }

    /// Returns the canonical operation identity.
    #[must_use]
    pub const fn operation_id(&self) -> OperationId {
        self.operation_id
    }

    /// Returns the abstract transport kind.
    #[must_use]
    pub const fn kind(&self) -> &TransportKind {
        &self.kind
    }

    /// Returns the source resource.
    #[must_use]
    pub const fn source(&self) -> &TransportResource {
        &self.source
    }

    /// Returns the destination resource.
    #[must_use]
    pub const fn destination(&self) -> &TransportResource {
        &self.destination
    }

    /// Returns the ordered intermediate path.
    #[must_use]
    pub const fn path(&self) -> &TransportPath {
        &self.path
    }

    /// Returns the semantic duration.
    #[must_use]
    pub const fn duration(&self) -> OperationDuration {
        self.duration
    }

    /// Returns the optional noise-model identity.
    #[must_use]
    pub const fn noise_model_id(&self) -> Option<NoiseModelId> {
        self.noise_model_id
    }

    /// Returns the optional calibration identity.
    #[must_use]
    pub const fn calibration_id(&self) -> Option<CalibrationId> {
        self.calibration_id
    }

    /// Returns the number of explicitly represented path segments.
    #[must_use]
    pub fn segment_count(&self) -> usize {
        self.path.len()
    }

    /// Returns whether the operation is a direct source/destination
    /// transport.
    #[must_use]
    pub fn is_direct(&self) -> bool {
        self.path.is_empty()
    }

    /// Returns whether source and destination are the same semantic resource.
    #[must_use]
    pub fn is_identity(&self) -> bool {
        self.source == self.destination
    }

    /// Associates a ZQN noise model identity with the operation.
    ///
    /// This does not resolve, validate, or execute the model.
    #[must_use]
    pub const fn with_noise_model(mut self, noise_model_id: NoiseModelId) -> Self {
        self.noise_model_id = Some(noise_model_id);
        self
    }

    /// Removes the optional noise-model association.
    #[must_use]
    pub const fn without_noise_model(mut self) -> Self {
        self.noise_model_id = None;
        self
    }

    /// Associates a calibration identity with the operation.
    ///
    /// This is only a reference. Calibration resolution belongs to the
    /// calibration subsystem.
    #[must_use]
    pub const fn with_calibration(mut self, calibration_id: CalibrationId) -> Self {
        self.calibration_id = Some(calibration_id);
        self
    }

    /// Removes the optional calibration association.
    #[must_use]
    pub const fn without_calibration(mut self) -> Self {
        self.calibration_id = None;
        self
    }

    /// Returns the canonical operation model version.
    #[must_use]
    pub const fn model_version(&self) -> u16 {
        TRANSPORT_OPERATION_MODEL_VERSION
    }

    /// Validates the complete operation structure.
    ///
    /// This method is intentionally cheap: it validates semantic structure
    /// only and does not inspect hardware or resolve external registries.
    pub fn validate(&self) -> TransportResult<()> {
        validate_transport_kind(&self.kind)?;

        if let Some(path_source) = self.path.first_source() {
            if path_source != &self.source {
                return Err(TransportError::InvalidStructure {
                    reason: "path source does not match operation source",
                });
            }
        }

        if let Some(path_destination) = self.path.final_destination() {
            if path_destination != &self.destination {
                return Err(TransportError::InvalidStructure {
                    reason: "path destination does not match operation destination",
                });
            }
        }

        Ok(())
    }
}

// ============================================================================
// Builder
// ============================================================================

/// Builder for transport operations.
///
/// The builder is useful for integrations that discover transport properties
/// incrementally while lowering or compiling.
///
/// It remains allocation-local and contains no global state.
#[derive(Debug, Clone)]
pub struct TransportOperationBuilder {
    operation_id: OperationId,
    kind: Option<TransportKind>,
    source: Option<TransportResource>,
    destination: Option<TransportResource>,
    path: Option<TransportPath>,
    duration: Option<OperationDuration>,
    noise_model_id: Option<NoiseModelId>,
    calibration_id: Option<CalibrationId>,
}

impl TransportOperationBuilder {
    /// Creates an empty builder associated with the canonical operation ID.
    #[must_use]
    pub const fn new(operation_id: OperationId) -> Self {
        Self {
            operation_id,
            kind: None,
            source: None,
            destination: None,
            path: None,
            duration: None,
            noise_model_id: None,
            calibration_id: None,
        }
    }

    /// Sets the transport kind.
    #[must_use]
    pub fn kind(mut self, kind: TransportKind) -> Self {
        self.kind = Some(kind);
        self
    }

    /// Sets the source.
    #[must_use]
    pub fn source(mut self, source: TransportResource) -> Self {
        self.source = Some(source);
        self
    }

    /// Sets the destination.
    #[must_use]
    pub fn destination(mut self, destination: TransportResource) -> Self {
        self.destination = Some(destination);
        self
    }

    /// Sets the path.
    #[must_use]
    pub fn path(mut self, path: TransportPath) -> Self {
        self.path = Some(path);
        self
    }

    /// Sets the duration.
    #[must_use]
    pub fn duration(mut self, duration: OperationDuration) -> Self {
        self.duration = Some(duration);
        self
    }

    /// Associates a noise model.
    #[must_use]
    pub const fn noise_model(mut self, id: NoiseModelId) -> Self {
        self.noise_model_id = Some(id);
        self
    }

    /// Associates calibration.
    #[must_use]
    pub const fn calibration(mut self, id: CalibrationId) -> Self {
        self.calibration_id = Some(id);
        self
    }

    /// Builds and validates the transport operation.
    pub fn build(self) -> TransportResult<TransportOperation> {
        let kind = self.kind.ok_or(TransportError::InvalidTransportKind {
            reason: "transport kind was not supplied",
        })?;

        let source = self.source.ok_or(TransportError::MissingSource)?;

        let destination = self
            .destination
            .ok_or(TransportError::MissingDestination)?;

        let path = self.path.unwrap_or_else(TransportPath::direct);

        let duration = self.duration.ok_or(TransportError::InvalidDuration {
            reason: "transport duration was not supplied",
        })?;

        let mut operation = TransportOperation::new(
            self.operation_id,
            kind,
            source,
            destination,
            path,
            duration,
        )?;

        operation.noise_model_id = self.noise_model_id;
        operation.calibration_id = self.calibration_id;

        Ok(operation)
    }
}

// ============================================================================
// Helpers
// ============================================================================

fn validate_path_connectivity(segments: &[TransportSegment]) -> TransportResult<()> {
    for (index, pair) in segments.windows(2).enumerate() {
        if pair[0].destination() != pair[1].source() {
            return Err(TransportError::DisconnectedPath { segment: index + 1 });
        }
    }

    Ok(())
}

fn validate_transport_kind(kind: &TransportKind) -> TransportResult<()> {
    if let TransportKind::Custom(value) = kind {
        if value.trim().is_empty() {
            return Err(TransportError::InvalidTransportKind {
                reason: "custom transport kind must not be empty",
            });
        }
    }

    Ok(())
}

fn validate_identifier(value: String, field: &'static str) -> TransportResult<String> {
    if value.trim().is_empty() {
        return Err(match field {
            "resource namespace" => TransportError::EmptyNamespace,
            "resource id" => TransportError::EmptyResourceId,
            _ => TransportError::InvalidIdentifier { field },
        });
    }

    let valid = value.chars().all(|character| {
        character.is_ascii_alphanumeric()
            || matches!(
                character,
                '_' | '-' | '.' | ':' | '/' | '$'
            )
    });

    if !valid {
        return Err(TransportError::InvalidIdentifier { field });
    }

    Ok(value)
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::quantum::ir::identity::OperationId;
    use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId};

    fn operation_id() -> OperationId {
        // OperationId construction is intentionally kept in the canonical IR.
        //
        // The transport module only consumes the canonical identity.
        OperationId::new(1)
    }

    #[test]
    fn logical_resource_uses_canonical_qubit_identity() {
        let qubit = QubitId::new(7);
        let resource = TransportResource::logical_qubit(qubit);

        assert_eq!(resource.logical_qubit_id(), Some(qubit));
        assert!(resource.is_logical_qubit());
        assert!(!resource.is_physical_qubit());
    }

    #[test]
    fn physical_resource_uses_canonical_physical_qubit_identity() {
        let qubit = PhysicalQubitId::new(11);
        let resource = TransportResource::physical_qubit(qubit);

        assert_eq!(resource.physical_qubit_id(), Some(qubit));
        assert!(resource.is_physical_qubit());
        assert!(!resource.is_logical_qubit());
    }

    #[test]
    fn named_resource_is_extensible() {
        let resource = TransportResource::named("mode", "resonator_0")
            .expect("valid named resource");

        assert!(resource.is_named());
    }

    #[test]
    fn rejects_empty_named_namespace() {
        let result = TransportResource::named("", "resource");

        assert!(matches!(result, Err(TransportError::EmptyNamespace)));
    }

    #[test]
    fn rejects_empty_named_resource_id() {
        let result = TransportResource::named("mode", "");

        assert!(matches!(result, Err(TransportError::EmptyResourceId)));
    }

    #[test]
    fn rejects_invalid_named_resource_identifier() {
        let result = TransportResource::named("mode", "resource with spaces");

        assert!(matches!(
            result,
            Err(TransportError::InvalidIdentifier {
                field: "resource id"
            })
        ));
    }

    #[test]
    fn direct_path_is_empty() {
        let path = TransportPath::direct();

        assert!(path.is_empty());
        assert_eq!(path.len(), 0);
    }

    #[test]
    fn connected_path_is_accepted() {
        let a = TransportResource::physical_qubit(PhysicalQubitId::new(0));
        let b = TransportResource::physical_qubit(PhysicalQubitId::new(1));
        let c = TransportResource::physical_qubit(PhysicalQubitId::new(2));

        let path = TransportPath::new(vec![
            TransportSegment::new(a.clone(), b.clone()).expect("valid segment"),
            TransportSegment::new(b, c.clone()).expect("valid segment"),
        ])
        .expect("connected path");

        assert_eq!(path.len(), 2);
        assert_eq!(path.final_destination(), Some(&c));
    }

    #[test]
    fn disconnected_path_is_rejected() {
        let a = TransportResource::physical_qubit(PhysicalQubitId::new(0));
        let b = TransportResource::physical_qubit(PhysicalQubitId::new(1));
        let c = TransportResource::physical_qubit(PhysicalQubitId::new(2));
        let d = TransportResource::physical_qubit(PhysicalQubitId::new(3));

        let result = TransportPath::new(vec![
            TransportSegment::new(a, b).expect("valid segment"),
            TransportSegment::new(c, d).expect("valid segment"),
        ]);

        assert!(matches!(
            result,
            Err(TransportError::DisconnectedPath { segment: 1 })
        ));
    }

    #[test]
    fn direct_transport_operation_is_valid() {
        let source = TransportResource::physical_qubit(PhysicalQubitId::new(0));
        let destination = TransportResource::physical_qubit(PhysicalQubitId::new(4));

        let duration =
            OperationDuration::from_seconds(1.0e-6).expect("finite non-negative duration");

        let operation = TransportOperation::direct(
            operation_id(),
            TransportKind::Move,
            source.clone(),
            destination.clone(),
            duration,
        )
        .expect("valid transport operation");

        assert_eq!(operation.source(), &source);
        assert_eq!(operation.destination(), &destination);
        assert!(operation.is_direct());
        assert!(!operation.is_identity());
        assert_eq!(operation.duration(), duration);
    }

    #[test]
    fn routed_transport_operation_preserves_path_order() {
        let a = TransportResource::physical_qubit(PhysicalQubitId::new(0));
        let b = TransportResource::physical_qubit(PhysicalQubitId::new(1));
        let c = TransportResource::physical_qubit(PhysicalQubitId::new(2));

        let path = TransportPath::new(vec![
            TransportSegment::new(a.clone(), b.clone()).expect("valid segment"),
            TransportSegment::new(b, c.clone()).expect("valid segment"),
        ])
        .expect("valid path");

        let duration =
            OperationDuration::from_seconds(2.0e-6).expect("finite non-negative duration");

        let operation = TransportOperation::new(
            operation_id(),
            TransportKind::Shuttle,
            a,
            c,
            path,
            duration,
        )
        .expect("valid transport operation");

        assert_eq!(operation.segment_count(), 2);
        assert_eq!(
            operation.path().segments()[0].source(),
            operation.source()
        );
        assert_eq!(
            operation.path().segments()[1].destination(),
            operation.destination()
        );
    }

    #[test]
    fn path_endpoints_must_match_operation() {
        let a = TransportResource::physical_qubit(PhysicalQubitId::new(0));
        let b = TransportResource::physical_qubit(PhysicalQubitId::new(1));
        let c = TransportResource::physical_qubit(PhysicalQubitId::new(2));

        let path = TransportPath::new(vec![
            TransportSegment::new(a.clone(), b.clone()).expect("valid segment"),
        ])
        .expect("valid path");

        let duration =
            OperationDuration::from_seconds(1.0e-6).expect("finite non-negative duration");

        let result = TransportOperation::new(
            operation_id(),
            TransportKind::Move,
            a,
            c,
            path,
            duration,
        );

        assert!(matches!(
            result,
            Err(TransportError::InvalidStructure {
                reason: "path destination does not match operation destination"
            })
        ));
    }

    #[test]
    fn noise_model_reference_is_metadata_only() {
        let source = TransportResource::physical_qubit(PhysicalQubitId::new(0));
        let destination = TransportResource::physical_qubit(PhysicalQubitId::new(1));

        let duration =
            OperationDuration::from_seconds(1.0e-6).expect("valid duration");

        let noise_model = NoiseModelId::new(42);

        let operation = TransportOperation::direct(
            operation_id(),
            TransportKind::Transfer,
            source,
            destination,
            duration,
        )
        .expect("valid operation")
        .with_noise_model(noise_model);

        assert_eq!(operation.noise_model_id(), Some(noise_model));
    }

    #[test]
    fn calibration_reference_is_metadata_only() {
        let source = TransportResource::physical_qubit(PhysicalQubitId::new(0));
        let destination = TransportResource::physical_qubit(PhysicalQubitId::new(1));

        let duration =
            OperationDuration::from_seconds(1.0e-6).expect("valid duration");

        let calibration = CalibrationId::new(99);

        let operation = TransportOperation::direct(
            operation_id(),
            TransportKind::Move,
            source,
            destination,
            duration,
        )
        .expect("valid operation")
        .with_calibration(calibration);

        assert_eq!(operation.calibration_id(), Some(calibration));
    }

    #[test]
    fn builder_constructs_equivalent_operation() {
        let source = TransportResource::logical_qubit(QubitId::new(0));
        let destination = TransportResource::logical_qubit(QubitId::new(1));

        let duration =
            OperationDuration::from_seconds(1.0e-6).expect("valid duration");

        let operation = TransportOperationBuilder::new(operation_id())
            .kind(TransportKind::Transfer)
            .source(source.clone())
            .destination(destination.clone())
            .duration(duration)
            .build()
            .expect("valid operation");

        assert_eq!(operation.source(), &source);
        assert_eq!(operation.destination(), &destination);
        assert_eq!(operation.kind(), &TransportKind::Transfer);
    }

    #[test]
    fn identity_transport_is_allowed() {
        let resource = TransportResource::physical_qubit(PhysicalQubitId::new(5));

        let duration =
            OperationDuration::from_seconds(0.0).expect("zero duration is valid");

        let operation = TransportOperation::direct(
            operation_id(),
            TransportKind::Transfer,
            resource.clone(),
            resource,
            duration,
        )
        .expect("identity transport is structurally valid");

        assert!(operation.is_identity());
    }

    #[test]
    fn custom_transport_kind_is_extensible() {
        let kind = TransportKind::custom("future_transport").expect("valid custom kind");

        assert_eq!(kind.as_str(), "future_transport");
    }

    #[test]
    fn transport_operation_is_deterministic_value_data() {
        let source = TransportResource::logical_qubit(QubitId::new(1));
        let destination = TransportResource::logical_qubit(QubitId::new(2));

        let duration =
            OperationDuration::from_seconds(5.0e-7).expect("valid duration");

        let first = TransportOperation::direct(
            operation_id(),
            TransportKind::Move,
            source.clone(),
            destination.clone(),
            duration,
        )
        .expect("valid operation");

        let second = TransportOperation::direct(
            operation_id(),
            TransportKind::Move,
            source,
            destination,
            duration,
        )
        .expect("valid operation");

        assert_eq!(first, second);
    }

    #[test]
    fn operation_model_version_is_stable() {
        assert_eq!(TRANSPORT_OPERATION_MODEL_VERSION, 1);
    }
}