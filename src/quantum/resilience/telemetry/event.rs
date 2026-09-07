//! Zamani Quantum Resilience — Telemetry Events
//!
//! Path:
//!     src/quantum/resilience/telemetry/event.rs
//!
//! # Purpose
//!
//! This module defines the canonical, provider-neutral event representation
//! used by `quantum::resilience::telemetry`.
//!
//! A telemetry event is an immutable observation or lifecycle fact supplied to
//! the resilience subsystem. Events are consumed by components such as:
//!
//! - telemetry collectors;
//! - anomaly detectors;
//! - statistical detectors;
//! - drift detectors;
//! - timeout detectors;
//! - execution-failure detectors;
//! - QEC-signal detectors;
//! - hardware-signal detectors;
//! - diagnosis;
//! - incident correlation;
//! - history;
//! - observability exporters;
//! - deterministic replay;
//! - resilience verification/provenance.
//!
//! This module intentionally does NOT:
//!
//! - detect faults;
//! - diagnose incidents;
//! - decide recovery;
//! - execute recovery;
//! - access hardware;
//! - access the network;
//! - access the filesystem;
//! - read the system clock;
//! - generate random identifiers;
//! - generate UUIDs;
//! - perform cryptographic hashing;
//! - serialize through a provider-specific format;
//! - define QEC semantics;
//! - define hardware capability semantics;
//! - define a second quantum-qubit identity type.
//!
//! Those responsibilities belong to their authoritative subsystems.
//!
//! # Architectural position
//!
//! ```text
//! hardware / runtime / QEC / scheduler / execution
//!                         │
//!                         ▼
//!                 telemetry producers
//!                         │
//!                         ▼
//!                 TelemetryEvent
//!                         │
//!          ┌──────────────┼──────────────┐
//!          ▼              ▼              ▼
//!       detection      history       exporter
//!          │
//!          ▼
//!       diagnosis
//!          │
//!          ▼
//!       planning
//!          │
//!          ▼
//!       recovery
//!          │
//!          ▼
//!       verification
//! ```
//!
//! `TelemetryEvent` is therefore a data contract, not a resilience algorithm.
//!
//! # Write once, scale everywhere
//!
//! This module introduces no architectural limits for:
//!
//! - qubits;
//! - physical qubits;
//! - logical qubits;
//! - events;
//! - event payload size;
//! - resources;
//! - devices;
//! - backends;
//! - execution stages;
//! - telemetry sources.
//!
//! In particular, this module MUST NOT contain constants such as:
//!
//! ```text
//! MAX_QUBITS
//! MAX_EVENTS
//! MAX_DEVICES
//! MAX_EVENT_PAYLOAD
//! ```
//!
//! Concrete limits belong to:
//!
//! - caller policy;
//! - resource policy;
//! - security policy;
//! - execution budget;
//! - storage capacity;
//! - target capability;
//! - provider capability;
//!
//! The event representation itself remains unbounded in the architectural
//! sense and naturally finite in every concrete process.
//!
//! # Canonical quantum identities
//!
//! Logical and physical qubit identities are deliberately distinct.
//!
//! The authoritative types are:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! No resilience-local qubit ID is introduced here.
//!
//! # Determinism
//!
//! Event construction is deterministic for deterministic input.
//!
//! This module does not:
//!
//! - obtain timestamps automatically;
//! - generate IDs automatically;
//! - inspect environment variables;
//! - access global mutable state;
//! - access filesystem state;
//! - access network state;
//! - use random numbers.
//!
//! Event identifiers, timestamps, sequence numbers, trace identifiers and
//! content identities are explicit producer inputs.
//!
//! This is essential for deterministic replay.
//!
//! # Ordering
//!
//! A timestamp alone is insufficient to establish event order because multiple
//! events may have equal timestamps and clocks can differ between producers.
//!
//! Therefore an event may carry an explicit producer sequence number.
//!
//! Consumers MUST NOT assume that timestamps alone provide a total ordering.
//!
//! The event model supports:
//!
//! - event timestamp;
//! - producer sequence;
//! - causal parent identifier;
//! - correlation identifier;
//! - trace identifier.
//!
//! The producer is responsible for assigning these values consistently.
//!
//! # Trust
//!
//! Telemetry is evidence, not truth.
//!
//! A malicious, faulty, stale, duplicated or delayed telemetry source must not
//! automatically become an authoritative resilience decision.
//!
//! Each event therefore records source/trust metadata separately from event
//! classification.
//!
//! Detection and diagnosis remain responsible for deciding what the evidence
//! means.
//!
//! # Event payload
//!
//! The payload is intentionally represented as a provider-neutral typed enum.
//!
//! Generic metadata is also supported for extensibility.
//!
//! Unknown future event kinds can be represented through `Custom` without
//! modifying the core event structure.
//!
//! # Security
//!
//! This module deliberately does not provide dedicated fields for:
//!
//! - credentials;
//! - API tokens;
//! - private keys;
//! - authentication headers;
//! - provider secrets.
//!
//! Free-form metadata must not be used to bypass that boundary.
//!
//! Callers remain responsible for preventing secret material from entering
//! telemetry.
//!
//! # Serialization
//!
//! Serialization is intentionally not implemented directly here.
//!
//! Canonical resilience serialization belongs under:
//!
//!     quantum::resilience::serialization
//!
//! A future serialization implementation may encode these structures through
//! an explicitly versioned schema.
//!
//! # Integration
//!
//! `telemetry/mod.rs` should expose:
//!
//! ```text
//! pub mod event;
//! ```
//!
//! Downstream components should import the event through:
//!
//! ```text
//! crate::quantum::resilience::telemetry::event::TelemetryEvent
//! ```
//!
//! Detection modules consume `TelemetryEvent`.
//!
//! `telemetry::metric` should consume or derive metrics from events rather than
//! redefining a competing event identity.
//!
//! `telemetry::trace` should use the trace/correlation fields exposed here.
//!
//! `telemetry::collector` should construct events explicitly.
//!
//! `telemetry::exporter` should consume immutable events.
//!
//! `history` may persist events according to its own retention policy.
//!
//! `diagnosis` and `detection` should never mutate an event after construction.
//!
//! # Rust contract
//!
//! - Rust 1.97
//! - Rust 1.97.1
//! - Rust 2021 edition
//! - stable Rust
//! - no nightly features
//! - no unsafe code
//!
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_debug_implementations)]
#![deny(rust_2018_idioms)]

use std::fmt;
use std::sync::Arc;

use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId};

/// Stable semantic schema identifier for resilience telemetry events.
pub const TELEMETRY_EVENT_SCHEMA_ID: &str =
    "zamani.quantum.resilience.telemetry.event";

/// Current semantic schema version.
///
/// This is a schema identity, not a machine-size or protocol limit.
pub const TELEMETRY_EVENT_SCHEMA_VERSION: u32 = 1;

/// Maximum severity ordering is intentionally represented by the enum order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum EventSeverity {
    /// Diagnostic information that does not indicate degradation.
    Informational,

    /// A potentially relevant condition has been observed.
    Notice,

    /// The system or resource is degraded.
    Warning,

    /// A significant failure or degradation has been observed.
    Error,

    /// The observation may compromise correctness or availability.
    Critical,

    /// A terminal or safety-critical condition has been observed.
    Fatal,
}

impl EventSeverity {
    /// Returns the stable machine-readable representation.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Informational => "informational",
            Self::Notice => "notice",
            Self::Warning => "warning",
            Self::Error => "error",
            Self::Critical => "critical",
            Self::Fatal => "fatal",
        }
    }

    /// Returns whether this severity requires resilience attention.
    #[must_use]
    pub const fn requires_attention(self) -> bool {
        !matches!(self, Self::Informational)
    }
}

impl fmt::Display for EventSeverity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Identifies the broad producer/source class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum EventSourceKind {
    /// Quantum hardware or hardware abstraction layer.
    Hardware,

    /// Quantum runtime/execution engine.
    Runtime,

    /// Compiler or lowering pipeline.
    Compiler,

    /// Canonical IR subsystem.
    Ir,

    /// Routing subsystem.
    Routing,

    /// Scheduling subsystem.
    Scheduling,

    /// Optimization subsystem.
    Optimization,

    /// Quantum error-correction subsystem.
    Qec,

    /// Error-mitigation subsystem.
    Mitigation,

    /// Simulator or emulator.
    Simulation,

    /// Benchmarking subsystem.
    Benchmarking,

    /// Resilience detector.
    Detection,

    /// Resilience diagnosis.
    Diagnosis,

    /// Resilience recovery.
    Recovery,

    /// Resilience verification.
    Verification,

    /// External or otherwise generic source.
    External,

    /// User/application supplied observation.
    Application,

    /// Unknown source class.
    Unknown,
}

impl EventSourceKind {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Hardware => "hardware",
            Self::Runtime => "runtime",
            Self::Compiler => "compiler",
            Self::Ir => "ir",
            Self::Routing => "routing",
            Self::Scheduling => "scheduling",
            Self::Optimization => "optimization",
            Self::Qec => "qec",
            Self::Mitigation => "mitigation",
            Self::Simulation => "simulation",
            Self::Benchmarking => "benchmarking",
            Self::Detection => "detection",
            Self::Diagnosis => "diagnosis",
            Self::Recovery => "recovery",
            Self::Verification => "verification",
            Self::External => "external",
            Self::Application => "application",
            Self::Unknown => "unknown",
        }
    }
}

impl fmt::Display for EventSourceKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Stable identifier for an event.
///
/// The value is supplied by the producer.
///
/// This module deliberately does not prescribe UUID, ULID, hash, database,
/// counter, or provider-specific identifier formats.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EventId(Arc<str>);

impl EventId {
    /// Creates an event identifier from an explicit producer value.
    pub fn new(value: impl Into<Arc<str>>) -> Result<Self, EventIdError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(EventIdError::Empty);
        }

        Ok(Self(value))
    }

    /// Returns the identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

impl fmt::Display for EventId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Errors produced while constructing an [`EventId`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EventIdError {
    /// The supplied identifier was empty or whitespace-only.
    Empty,
}

impl fmt::Display for EventIdError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("telemetry event identifier is empty"),
        }
    }
}

impl std::error::Error for EventIdError {}

/// Stable producer/source identifier.
///
/// This is intentionally opaque and provider-neutral.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SourceId(Arc<str>);

impl SourceId {
    /// Creates a source identifier.
    pub fn new(value: impl Into<Arc<str>>) -> Result<Self, SourceIdError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(SourceIdError::Empty);
        }

        Ok(Self(value))
    }

    /// Returns the identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

impl fmt::Display for SourceId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Errors produced while constructing a [`SourceId`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SourceIdError {
    /// The supplied identifier was empty or whitespace-only.
    Empty,
}

impl fmt::Display for SourceIdError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("telemetry source identifier is empty"),
        }
    }
}

impl std::error::Error for SourceIdError {}

/// Explicit event timestamp.
///
/// The resilience event layer does not read the clock itself.
///
/// `unix_seconds` is signed to allow explicit representation of timestamps
/// before the Unix epoch.
///
/// `nanos` must be in the half-open interval `[0, 1_000_000_000)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EventTimestamp {
    unix_seconds: i64,
    nanos: u32,
}

impl EventTimestamp {
    /// Number of nanoseconds in one second.
    pub const NANOS_PER_SECOND: u32 = 1_000_000_000;

    /// Creates an explicit timestamp.
    pub const fn new(unix_seconds: i64, nanos: u32) -> Result<Self, TimestampError> {
        if nanos >= Self::NANOS_PER_SECOND {
            return Err(TimestampError::InvalidNanoseconds);
        }

        Ok(Self {
            unix_seconds,
            nanos,
        })
    }

    /// Creates an integral-second timestamp.
    #[must_use]
    pub const fn from_unix_seconds(unix_seconds: i64) -> Self {
        Self {
            unix_seconds,
            nanos: 0,
        }
    }

    /// Returns Unix seconds.
    #[must_use]
    pub const fn unix_seconds(self) -> i64 {
        self.unix_seconds
    }

    /// Returns the fractional nanoseconds.
    #[must_use]
    pub const fn nanos(self) -> u32 {
        self.nanos
    }
}

impl fmt::Display for EventTimestamp {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}.{:09}",
            self.unix_seconds,
            self.nanos
        )
    }
}

/// Errors produced by [`EventTimestamp`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TimestampError {
    /// Nanosecond component was outside the canonical range.
    InvalidNanoseconds,
}

impl fmt::Display for TimestampError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidNanoseconds => {
                formatter.write_str("timestamp nanoseconds must be less than 1_000_000_000")
            }
        }
    }
}

impl std::error::Error for TimestampError {}

/// Explicit producer sequence number.
///
/// Sequence numbers are local to a producer/source scope and do not form a
/// global event count.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EventSequence(u64);

impl EventSequence {
    /// Creates a sequence number.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the numeric sequence.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }
}

/// Correlates related events.
///
/// Correlation is intentionally separate from causal parentage.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CorrelationId(Arc<str>);

impl CorrelationId {
    /// Creates a correlation identifier.
    pub fn new(value: impl Into<Arc<str>>) -> Result<Self, CorrelationIdError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(CorrelationIdError::Empty);
        }

        Ok(Self(value))
    }

    /// Returns the identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

impl fmt::Display for CorrelationId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Errors produced while constructing a [`CorrelationId`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CorrelationIdError {
    /// The supplied identifier was empty.
    Empty,
}

impl fmt::Display for CorrelationIdError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("telemetry correlation identifier is empty"),
        }
    }
}

impl std::error::Error for CorrelationIdError {}

/// Identifies a trace containing one or more correlated events.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TraceId(Arc<str>);

impl TraceId {
    /// Creates a trace identifier.
    pub fn new(value: impl Into<Arc<str>>) -> Result<Self, TraceIdError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(TraceIdError::Empty);
        }

        Ok(Self(value))
    }

    /// Returns the identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

impl fmt::Display for TraceId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Errors produced while constructing a [`TraceId`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TraceIdError {
    /// The supplied identifier was empty.
    Empty,
}

impl fmt::Display for TraceIdError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("telemetry trace identifier is empty"),
        }
    }
}

impl std::error::Error for TraceIdError {}

/// Trust classification supplied by the telemetry producer/ingress boundary.
///
/// Trust is evidence about the source, not a statement that the event itself
/// is true.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum EventTrust {
    /// Source identity is unknown.
    Unknown,

    /// Source is known but not authenticated.
    Unauthenticated,

    /// Source identity was authenticated.
    Authenticated,

    /// Source and message integrity were authenticated by the ingress layer.
    IntegrityVerified,

    /// Source is trusted according to an explicit deployment policy.
    PolicyTrusted,
}

impl EventTrust {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unknown => "unknown",
            Self::Unauthenticated => "unauthenticated",
            Self::Authenticated => "authenticated",
            Self::IntegrityVerified => "integrity_verified",
            Self::PolicyTrusted => "policy_trusted",
        }
    }
}

impl fmt::Display for EventTrust {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Broad event classification.
///
/// This enum describes the semantic role of an event, not the response to it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum EventKind {
    /// A program/execution lifecycle event.
    Execution,

    /// Backend/device state changed.
    BackendState,

    /// Quantum resource state changed.
    ResourceState,

    /// Calibration-related observation.
    Calibration,

    /// Topology/capability observation.
    Capability,

    /// Noise/error observation.
    Noise,

    /// QEC/syndrome observation.
    Qec,

    /// Measurement/readout observation.
    Measurement,

    /// Timing/latency observation.
    Timing,

    /// Compilation/lowering observation.
    Compilation,

    /// Routing observation.
    Routing,

    /// Scheduling observation.
    Scheduling,

    /// Optimization observation.
    Optimization,

    /// Detection observation.
    Detection,

    /// Diagnosis observation.
    Diagnosis,

    /// Adaptation observation.
    Adaptation,

    /// Recovery observation.
    Recovery,

    /// Verification observation.
    Verification,

    /// Mitigation observation.
    Mitigation,

    /// Health observation.
    Health,

    /// Benchmark observation.
    Benchmark,

    /// Security-related observation.
    Security,

    /// Generic metadata event.
    Metadata,

    /// Extension event.
    Custom,
}

impl EventKind {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Execution => "execution",
            Self::BackendState => "backend_state",
            Self::ResourceState => "resource_state",
            Self::Calibration => "calibration",
            Self::Capability => "capability",
            Self::Noise => "noise",
            Self::Qec => "qec",
            Self::Measurement => "measurement",
            Self::Timing => "timing",
            Self::Compilation => "compilation",
            Self::Routing => "routing",
            Self::Scheduling => "scheduling",
            Self::Optimization => "optimization",
            Self::Detection => "detection",
            Self::Diagnosis => "diagnosis",
            Self::Adaptation => "adaptation",
            Self::Recovery => "recovery",
            Self::Verification => "verification",
            Self::Mitigation => "mitigation",
            Self::Health => "health",
            Self::Benchmark => "benchmark",
            Self::Security => "security",
            Self::Metadata => "metadata",
            Self::Custom => "custom",
        }
    }
}

impl fmt::Display for EventKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Canonical logical/physical resource scope.
///
/// Logical and physical qubits are deliberately represented separately.
///
/// Other resource classes remain extensible through [`ResourceReference`].
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ResourceScope {
    logical_qubits: Arc<[QubitId]>,
    physical_qubits: Arc<[PhysicalQubitId]>,
    resources: Arc<[ResourceReference]>,
}

impl ResourceScope {
    /// Creates a resource scope.
    ///
    /// Qubit collections are sorted and deduplicated so deterministic
    /// comparisons do not depend on producer iteration order.
    #[must_use]
    pub fn new<L, P, R>(
        logical_qubits: L,
        physical_qubits: P,
        resources: R,
    ) -> Self
    where
        L: IntoIterator<Item = QubitId>,
        P: IntoIterator<Item = PhysicalQubitId>,
        R: IntoIterator<Item = ResourceReference>,
    {
        let mut logical: Vec<QubitId> = logical_qubits.into_iter().collect();
        let mut physical: Vec<PhysicalQubitId> =
            physical_qubits.into_iter().collect();
        let mut resources: Vec<ResourceReference> =
            resources.into_iter().collect();

        logical.sort_unstable();
        logical.dedup();

        physical.sort_unstable();
        physical.dedup();

        resources.sort_unstable();
        resources.dedup();

        Self {
            logical_qubits: logical.into(),
            physical_qubits: physical.into(),
            resources: resources.into(),
        }
    }

    /// Returns logical qubits.
    #[must_use]
    pub fn logical_qubits(&self) -> &[QubitId] {
        self.logical_qubits.as_ref()
    }

    /// Returns physical qubits.
    #[must_use]
    pub fn physical_qubits(&self) -> &[PhysicalQubitId] {
        self.physical_qubits.as_ref()
    }

    /// Returns generic resource references.
    #[must_use]
    pub fn resources(&self) -> &[ResourceReference] {
        self.resources.as_ref()
    }

    /// Returns the number of distinct logical qubits.
    #[must_use]
    pub fn logical_qubit_count(&self) -> usize {
        self.logical_qubits.len()
    }

    /// Returns the number of distinct physical qubits.
    #[must_use]
    pub fn physical_qubit_count(&self) -> usize {
        self.physical_qubits.len()
    }
}

/// Provider-neutral reference to a non-qubit resource.
///
/// This does not duplicate the resilience resource model. It is a lightweight
/// event-local reference for resources that may be introduced by future
/// quantum architectures.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ResourceReference {
    /// Resource category.
    kind: Arc<str>,

    /// Producer-defined stable resource identifier.
    id: Arc<str>,
}

impl ResourceReference {
    /// Creates a generic resource reference.
    pub fn new(
        kind: impl Into<Arc<str>>,
        id: impl Into<Arc<str>>,
    ) -> Result<Self, ResourceReferenceError> {
        let kind = kind.into();
        let id = id.into();

        if kind.trim().is_empty() {
            return Err(ResourceReferenceError::EmptyKind);
        }

        if id.trim().is_empty() {
            return Err(ResourceReferenceError::EmptyId);
        }

        Ok(Self { kind, id })
    }

    /// Returns the resource kind.
    #[must_use]
    pub fn kind(&self) -> &str {
        self.kind.as_ref()
    }

    /// Returns the resource identifier.
    #[must_use]
    pub fn id(&self) -> &str {
        self.id.as_ref()
    }
}

/// Errors for [`ResourceReference`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResourceReferenceError {
    /// Resource kind was empty.
    EmptyKind,

    /// Resource ID was empty.
    EmptyId,
}

impl fmt::Display for ResourceReferenceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyKind => formatter.write_str("resource kind is empty"),
            Self::EmptyId => formatter.write_str("resource identifier is empty"),
        }
    }
}

impl std::error::Error for ResourceReferenceError {}

/// Event payload describing execution state.
#[derive(Debug, Clone, PartialEq)]
pub enum ExecutionEvent {
    /// Execution was submitted.
    Submitted,

    /// Execution entered a running state.
    Started,

    /// Execution completed successfully.
    Completed,

    /// Execution was cancelled.
    Cancelled,

    /// Execution failed.
    Failed,

    /// Execution was suspended.
    Suspended,

    /// Execution resumed.
    Resumed,

    /// Execution reached a producer-defined stage.
    Stage {
        /// Stable stage identifier.
        name: Arc<str>,
    },
}

/// Event payload describing backend/device state.
#[derive(Debug, Clone, PartialEq)]
pub enum BackendStateEvent {
    /// Backend became available.
    Available,

    /// Backend became unavailable.
    Unavailable,

    /// Backend became degraded.
    Degraded,

    /// Backend entered recovery.
    Recovering,

    /// Backend was quarantined.
    Quarantined,

    /// Backend status is otherwise unknown.
    Unknown,
}

/// Event payload describing resource state.
#[derive(Debug, Clone, PartialEq)]
pub enum ResourceStateEvent {
    /// Resource is usable.
    Available,

    /// Resource is degraded.
    Degraded,

    /// Resource is unavailable.
    Unavailable,

    /// Resource is quarantined.
    Quarantined,

    /// Resource is recovering.
    Recovering,

    /// Resource is retired.
    Retired,

    /// Resource state is unknown.
    Unknown,
}

/// Event payload describing calibration state.
#[derive(Debug, Clone, PartialEq)]
pub enum CalibrationEvent {
    /// Calibration started.
    Started,

    /// Calibration completed.
    Completed,

    /// Calibration failed.
    Failed,

    /// Calibration data changed.
    Updated,

    /// Calibration data became stale.
    Stale,
}

/// Event payload describing capability observations.
#[derive(Debug, Clone, PartialEq)]
pub enum CapabilityEvent {
    /// Capabilities were discovered.
    Discovered,

    /// Capabilities changed.
    Changed,

    /// Capability information became stale.
    Stale,

    /// Capability negotiation failed.
    NegotiationFailed,
}

/// Event payload describing noise/error observations.
///
/// The event layer intentionally does not duplicate the ZQN fault ontology.
/// `kind` and `classification` are opaque producer-supplied semantic labels
/// when an external canonical fault object is unavailable in this event
/// boundary.
#[derive(Debug, Clone, PartialEq)]
pub enum NoiseEvent {
    /// Noise/error observation.
    Observed {
        /// Producer-defined canonical noise/fault classification.
        classification: Arc<str>,

        /// Optional probability supplied by the authoritative producer.
        probability: Option<f64>,
    },

    /// Noise characteristics changed.
    Drift,

    /// Noise model became unavailable or stale.
    ModelUnavailable,
}

/// Event payload for QEC observations.
#[derive(Debug, Clone, PartialEq)]
pub enum QecEvent {
    /// Syndrome information was observed.
    SyndromeObserved,

    /// Decoder produced a result.
    Decoded,

    /// Logical error was detected.
    LogicalError,

    /// QEC state/configuration changed.
    ConfigurationChanged,

    /// QEC operation failed.
    Failed,
}

/// Event payload for measurement/readout observations.
#[derive(Debug, Clone, PartialEq)]
pub enum MeasurementEvent {
    /// Measurement operation began.
    Started,

    /// Measurement operation completed.
    Completed,

    /// Readout anomaly was observed.
    ReadoutAnomaly,

    /// Readout calibration changed.
    CalibrationChanged,
}

/// Event payload for timing observations.
#[derive(Debug, Clone, PartialEq)]
pub enum TimingEvent {
    /// Operation or execution exceeded an expected timing budget.
    DeadlineExceeded,

    /// Timing drift was observed.
    Drift,

    /// Queue delay was observed.
    QueueDelay,

    /// Latency observation.
    Latency {
        /// Duration in nanoseconds.
        nanoseconds: u64,
    },
}

/// Event payload for compiler observations.
#[derive(Debug, Clone, PartialEq)]
pub enum CompilationEvent {
    /// Compilation started.
    Started,

    /// Compilation completed.
    Completed,

    /// Compilation failed.
    Failed,

    /// Compilation was invalidated by target changes.
    Invalidated,
}

/// Event payload for routing observations.
#[derive(Debug, Clone, PartialEq)]
pub enum RoutingEvent {
    /// Routing started.
    Started,

    /// Routing completed.
    Completed,

    /// Routing failed.
    Failed,

    /// Existing mapping became invalid.
    MappingInvalidated,
}

/// Event payload for scheduling observations.
#[derive(Debug, Clone, PartialEq)]
pub enum SchedulingEvent {
    /// Scheduling started.
    Started,

    /// Scheduling completed.
    Completed,

    /// Scheduling failed.
    Failed,

    /// Existing schedule became invalid.
    ScheduleInvalidated,
}

/// Event payload for optimization observations.
#[derive(Debug, Clone, PartialEq)]
pub enum OptimizationEvent {
    /// Optimization started.
    Started,

    /// Optimization completed.
    Completed,

    /// Optimization failed.
    Failed,

    /// Target changes invalidated the optimization result.
    Invalidated,
}

/// Event payload for resilience detection.
#[derive(Debug, Clone, PartialEq)]
pub enum DetectionEvent {
    /// A detector observed a candidate anomaly.
    Anomaly,

    /// A detector observed a threshold crossing.
    ThresholdCrossed,

    /// A detector observed statistical drift.
    StatisticalDrift,

    /// A detector timed out.
    Timeout,

    /// A detector received an execution failure.
    ExecutionFailure,

    /// A detector received a QEC signal.
    QecSignal,

    /// A detector received a hardware signal.
    HardwareSignal,
}

/// Event payload for diagnosis.
#[derive(Debug, Clone, PartialEq)]
pub enum DiagnosisEvent {
    /// Diagnosis started.
    Started,

    /// Diagnosis completed.
    Completed,

    /// Diagnosis was inconclusive.
    Inconclusive,

    /// Root-cause hypothesis changed.
    RootCauseUpdated,
}

/// Event payload for adaptation.
#[derive(Debug, Clone, PartialEq)]
pub enum AdaptationEvent {
    /// Logical/physical remapping occurred.
    Remapped,

    /// Routing was changed.
    Rerouted,

    /// Scheduling was changed.
    Rescheduled,

    /// Program was recompiled.
    Recompiled,

    /// Program was reoptimized.
    Reoptimized,

    /// QEC configuration changed.
    QecChanged,

    /// Backend/device target changed.
    BackendChanged,
}

/// Event payload for recovery.
#[derive(Debug, Clone, PartialEq)]
pub enum RecoveryEvent {
    /// Recovery was planned.
    Planned,

    /// Recovery started.
    Started,

    /// Recovery completed.
    Completed,

    /// Recovery failed.
    Failed,

    /// Recovery was escalated.
    Escalated,

    /// Recovery was rolled back.
    RolledBack,

    /// Recovery resumed from a valid boundary.
    Resumed,

    /// Execution migrated.
    Migrated,
}

/// Event payload for verification.
#[derive(Debug, Clone, PartialEq)]
pub enum VerificationEvent {
    /// Verification started.
    Started,

    /// Verification succeeded.
    Passed,

    /// Verification failed.
    Failed,

    /// Verification produced an inconclusive result.
    Inconclusive,

    /// Verification requires another execution/recovery cycle.
    RetryRequired,

    /// Verification accepted a degraded result.
    DegradedAccepted,
}

/// Event payload for mitigation.
#[derive(Debug, Clone, PartialEq)]
pub enum MitigationEvent {
    /// Mitigation was selected.
    Selected,

    /// Mitigation execution started.
    Started,

    /// Mitigation completed.
    Completed,

    /// Mitigation failed.
    Failed,

    /// Mitigation configuration changed.
    ConfigurationChanged,
}

/// Event payload for health observations.
#[derive(Debug, Clone, PartialEq)]
pub enum HealthEvent {
    /// Health became healthy.
    Healthy,

    /// Health became degraded.
    Degraded,

    /// Health became unstable.
    Unstable,

    /// Health became unavailable.
    Unavailable,

    /// Health state is recovering.
    Recovering,

    /// Health state is unknown.
    Unknown,
}

/// Event payload for benchmarking.
#[derive(Debug, Clone, PartialEq)]
pub enum BenchmarkEvent {
    /// Benchmark started.
    Started,

    /// Benchmark completed.
    Completed,

    /// Benchmark failed.
    Failed,

    /// Benchmark observation was produced.
    Observation,
}

/// Event payload for security observations.
#[derive(Debug, Clone, PartialEq)]
pub enum SecurityEvent {
    /// Authentication succeeded.
    AuthenticationSucceeded,

    /// Authentication failed.
    AuthenticationFailed,

    /// Integrity verification succeeded.
    IntegrityVerified,

    /// Integrity verification failed.
    IntegrityFailure,

    /// Authorization was denied.
    AuthorizationDenied,

    /// Suspicious telemetry behavior was observed.
    SuspiciousActivity,
}

/// Event payload for generic metadata.
#[derive(Debug, Clone, PartialEq)]
pub struct MetadataEvent {
    /// Metadata entries.
    entries: Arc<[MetadataEntry]>,
}

impl MetadataEvent {
    /// Creates a metadata event.
    ///
    /// Entries are sorted and duplicate keys are retained intentionally.
    /// Duplicate keys can represent ordered observations from independent
    /// producers; consumers must not assume map semantics.
    #[must_use]
    pub fn new<I>(entries: I) -> Self
    where
        I: IntoIterator<Item = MetadataEntry>,
    {
        Self {
            entries: entries.into_iter().collect::<Vec<_>>().into(),
        }
    }

    /// Returns metadata entries.
    #[must_use]
    pub fn entries(&self) -> &[MetadataEntry] {
        self.entries.as_ref()
    }
}

/// A safe free-form metadata key/value pair.
///
/// This structure is intentionally opaque to the event core.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MetadataEntry {
    key: Arc<str>,
    value: Arc<str>,
}

impl MetadataEntry {
    /// Creates a metadata entry.
    pub fn new(
        key: impl Into<Arc<str>>,
        value: impl Into<Arc<str>>,
    ) -> Result<Self, MetadataEntryError> {
        let key = key.into();
        let value = value.into();

        if key.trim().is_empty() {
            return Err(MetadataEntryError::EmptyKey);
        }

        Ok(Self { key, value })
    }

    /// Returns the key.
    #[must_use]
    pub fn key(&self) -> &str {
        self.key.as_ref()
    }

    /// Returns the value.
    #[must_use]
    pub fn value(&self) -> &str {
        self.value.as_ref()
    }
}

/// Errors for [`MetadataEntry`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MetadataEntryError {
    /// Metadata key is empty.
    EmptyKey,
}

impl fmt::Display for MetadataEntryError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyKey => formatter.write_str("telemetry metadata key is empty"),
        }
    }
}

impl std::error::Error for MetadataEntryError {}

/// Complete event payload.
///
/// Each variant remains provider-neutral.
#[derive(Debug, Clone, PartialEq)]
pub enum EventPayload {
    /// Execution lifecycle event.
    Execution(ExecutionEvent),

    /// Backend state event.
    BackendState(BackendStateEvent),

    /// Resource state event.
    ResourceState(ResourceStateEvent),

    /// Calibration event.
    Calibration(CalibrationEvent),

    /// Capability event.
    Capability(CapabilityEvent),

    /// Noise/error event.
    Noise(NoiseEvent),

    /// QEC event.
    Qec(QecEvent),

    /// Measurement event.
    Measurement(MeasurementEvent),

    /// Timing event.
    Timing(TimingEvent),

    /// Compilation event.
    Compilation(CompilationEvent),

    /// Routing event.
    Routing(RoutingEvent),

    /// Scheduling event.
    Scheduling(SchedulingEvent),

    /// Optimization event.
    Optimization(OptimizationEvent),

    /// Detection event.
    Detection(DetectionEvent),

    /// Diagnosis event.
    Diagnosis(DiagnosisEvent),

    /// Adaptation event.
    Adaptation(AdaptationEvent),

    /// Recovery event.
    Recovery(RecoveryEvent),

    /// Verification event.
    Verification(VerificationEvent),

    /// Mitigation event.
    Mitigation(MitigationEvent),

    /// Health event.
    Health(HealthEvent),

    /// Benchmark event.
    Benchmark(BenchmarkEvent),

    /// Security event.
    Security(SecurityEvent),

    /// Metadata event.
    Metadata(MetadataEvent),

    /// Future/provider/application-defined event.
    ///
    /// The namespace and name are explicit so future extensions do not require
    /// changing this core enum.
    Custom {
        /// Extension namespace.
        namespace: Arc<str>,

        /// Extension event name.
        name: Arc<str>,
    },
}

impl EventPayload {
    /// Returns the corresponding event kind.
    #[must_use]
    pub const fn kind(&self) -> EventKind {
        match self {
            Self::Execution(_) => EventKind::Execution,
            Self::BackendState(_) => EventKind::BackendState,
            Self::ResourceState(_) => EventKind::ResourceState,
            Self::Calibration(_) => EventKind::Calibration,
            Self::Capability(_) => EventKind::Capability,
            Self::Noise(_) => EventKind::Noise,
            Self::Qec(_) => EventKind::Qec,
            Self::Measurement(_) => EventKind::Measurement,
            Self::Timing(_) => EventKind::Timing,
            Self::Compilation(_) => EventKind::Compilation,
            Self::Routing(_) => EventKind::Routing,
            Self::Scheduling(_) => EventKind::Scheduling,
            Self::Optimization(_) => EventKind::Optimization,
            Self::Detection(_) => EventKind::Detection,
            Self::Diagnosis(_) => EventKind::Diagnosis,
            Self::Adaptation(_) => EventKind::Adaptation,
            Self::Recovery(_) => EventKind::Recovery,
            Self::Verification(_) => EventKind::Verification,
            Self::Mitigation(_) => EventKind::Mitigation,
            Self::Health(_) => EventKind::Health,
            Self::Benchmark(_) => EventKind::Benchmark,
            Self::Security(_) => EventKind::Security,
            Self::Metadata(_) => EventKind::Metadata,
            Self::Custom { .. } => EventKind::Custom,
        }
    }
}

/// Immutable resilience telemetry event.
///
/// The event contains all information required to transport an observation
/// without requiring hidden lookups into runtime state.
///
/// This is important for:
///
/// - deterministic replay;
/// - persistence;
/// - distributed transport;
/// - delayed processing;
/// - auditing;
/// - fault injection;
/// - testing.
#[derive(Debug, Clone, PartialEq)]
pub struct TelemetryEvent {
    /// Schema identifier.
    schema_id: Arc<str>,

    /// Schema version.
    schema_version: u32,

    /// Stable event identifier.
    id: EventId,

    /// Explicit event timestamp.
    timestamp: EventTimestamp,

    /// Optional producer-local sequence number.
    sequence: Option<EventSequence>,

    /// Source class.
    source_kind: EventSourceKind,

    /// Explicit source identifier.
    source_id: SourceId,

    /// Trust classification supplied by the ingress boundary.
    trust: EventTrust,

    /// Optional trace identifier.
    trace_id: Option<TraceId>,

    /// Optional correlation identifier.
    correlation_id: Option<CorrelationId>,

    /// Optional causal parent event.
    parent_event_id: Option<EventId>,

    /// Resource scope.
    resources: ResourceScope,

    /// Event severity.
    severity: EventSeverity,

    /// Typed event payload.
    payload: EventPayload,
}

impl TelemetryEvent {
    /// Creates a telemetry event.
    ///
    /// All identity and temporal information must be explicitly supplied.
    ///
    /// No hidden global state is consulted.
    pub fn new(
        id: EventId,
        timestamp: EventTimestamp,
        source_kind: EventSourceKind,
        source_id: SourceId,
        trust: EventTrust,
        resources: ResourceScope,
        severity: EventSeverity,
        payload: EventPayload,
    ) -> Self {
        Self {
            schema_id: Arc::from(TELEMETRY_EVENT_SCHEMA_ID),
            schema_version: TELEMETRY_EVENT_SCHEMA_VERSION,
            id,
            timestamp,
            sequence: None,
            source_kind,
            source_id,
            trust,
            trace_id: None,
            correlation_id: None,
            parent_event_id: None,
            resources,
            severity,
            payload,
        }
    }

    /// Sets an explicit producer-local sequence number.
    #[must_use]
    pub const fn with_sequence(mut self, sequence: EventSequence) -> Self {
        self.sequence = Some(sequence);
        self
    }

    /// Sets a trace identifier.
    #[must_use]
    pub fn with_trace_id(mut self, trace_id: TraceId) -> Self {
        self.trace_id = Some(trace_id);
        self
    }

    /// Sets a correlation identifier.
    #[must_use]
    pub fn with_correlation_id(mut self, correlation_id: CorrelationId) -> Self {
        self.correlation_id = Some(correlation_id);
        self
    }

    /// Sets the causal parent event.
    #[must_use]
    pub fn with_parent_event_id(mut self, parent_event_id: EventId) -> Self {
        self.parent_event_id = Some(parent_event_id);
        self
    }

    /// Returns the schema identifier.
    #[must_use]
    pub fn schema_id(&self) -> &str {
        self.schema_id.as_ref()
    }

    /// Returns the schema version.
    #[must_use]
    pub const fn schema_version(&self) -> u32 {
        self.schema_version
    }

    /// Returns the event ID.
    #[must_use]
    pub fn id(&self) -> &EventId {
        &self.id
    }

    /// Returns the timestamp.
    #[must_use]
    pub const fn timestamp(&self) -> EventTimestamp {
        self.timestamp
    }

    /// Returns the producer sequence.
    #[must_use]
    pub const fn sequence(&self) -> Option<EventSequence> {
        self.sequence
    }

    /// Returns the source kind.
    #[must_use]
    pub const fn source_kind(&self) -> EventSourceKind {
        self.source_kind
    }

    /// Returns the source ID.
    #[must_use]
    pub fn source_id(&self) -> &SourceId {
        &self.source_id
    }

    /// Returns the trust classification.
    #[must_use]
    pub const fn trust(&self) -> EventTrust {
        self.trust
    }

    /// Returns the optional trace ID.
    #[must_use]
    pub fn trace_id(&self) -> Option<&TraceId> {
        self.trace_id.as_ref()
    }

    /// Returns the optional correlation ID.
    #[must_use]
    pub fn correlation_id(&self) -> Option<&CorrelationId> {
        self.correlation_id.as_ref()
    }

    /// Returns the optional causal parent.
    #[must_use]
    pub fn parent_event_id(&self) -> Option<&EventId> {
        self.parent_event_id.as_ref()
    }

    /// Returns the event resource scope.
    #[must_use]
    pub fn resources(&self) -> &ResourceScope {
        &self.resources
    }

    /// Returns event severity.
    #[must_use]
    pub const fn severity(&self) -> EventSeverity {
        self.severity
    }

    /// Returns event payload.
    #[must_use]
    pub fn payload(&self) -> &EventPayload {
        &self.payload
    }

    /// Returns event kind.
    #[must_use]
    pub fn kind(&self) -> EventKind {
        self.payload.kind()
    }

    /// Returns whether the event represents a severity requiring attention.
    #[must_use]
    pub const fn requires_attention(&self) -> bool {
        self.severity.requires_attention()
    }
}

/// Lightweight event builder.
///
/// This is useful for producers that construct many events while keeping all
/// values explicit and avoiding hidden process-wide state.
#[derive(Debug, Clone)]
pub struct TelemetryEventBuilder {
    id: EventId,
    timestamp: EventTimestamp,
    source_kind: EventSourceKind,
    source_id: SourceId,
    trust: EventTrust,
    resources: ResourceScope,
    severity: EventSeverity,
    payload: EventPayload,
    sequence: Option<EventSequence>,
    trace_id: Option<TraceId>,
    correlation_id: Option<CorrelationId>,
    parent_event_id: Option<EventId>,
}

impl TelemetryEventBuilder {
    /// Creates a builder with the required event fields.
    #[must_use]
    pub fn new(
        id: EventId,
        timestamp: EventTimestamp,
        source_kind: EventSourceKind,
        source_id: SourceId,
        payload: EventPayload,
    ) -> Self {
        Self {
            id,
            timestamp,
            source_kind,
            source_id,
            trust: EventTrust::Unknown,
            resources: ResourceScope::default(),
            severity: EventSeverity::Informational,
            payload,
            sequence: None,
            trace_id: None,
            correlation_id: None,
            parent_event_id: None,
        }
    }

    /// Sets trust.
    #[must_use]
    pub const fn trust(mut self, trust: EventTrust) -> Self {
        self.trust = trust;
        self
    }

    /// Sets resources.
    #[must_use]
    pub fn resources(mut self, resources: ResourceScope) -> Self {
        self.resources = resources;
        self
    }

    /// Sets severity.
    #[must_use]
    pub const fn severity(mut self, severity: EventSeverity) -> Self {
        self.severity = severity;
        self
    }

    /// Sets sequence.
    #[must_use]
    pub const fn sequence(mut self, sequence: EventSequence) -> Self {
        self.sequence = Some(sequence);
        self
    }

    /// Sets trace.
    #[must_use]
    pub fn trace_id(mut self, trace_id: TraceId) -> Self {
        self.trace_id = Some(trace_id);
        self
    }

    /// Sets correlation.
    #[must_use]
    pub fn correlation_id(mut self, correlation_id: CorrelationId) -> Self {
        self.correlation_id = Some(correlation_id);
        self
    }

    /// Sets causal parent.
    #[must_use]
    pub fn parent_event_id(mut self, parent_event_id: EventId) -> Self {
        self.parent_event_id = Some(parent_event_id);
        self
    }

    /// Builds the immutable event.
    #[must_use]
    pub fn build(self) -> TelemetryEvent {
        let mut event = TelemetryEvent::new(
            self.id,
            self.timestamp,
            self.source_kind,
            self.source_id,
            self.trust,
            self.resources,
            self.severity,
            self.payload,
        );

        event.sequence = self.sequence;
        event.trace_id = self.trace_id;
        event.correlation_id = self.correlation_id;
        event.parent_event_id = self.parent_event_id;

        event
    }
}

/// Returns a deterministic comparison key for event ordering.
///
/// The key deliberately includes producer identity and sequence information.
/// Consumers should use causal relationships where available rather than
/// assuming timestamp order is authoritative.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct EventOrderingKey {
    timestamp: EventTimestamp,
    source_id: SourceId,
    sequence: Option<EventSequence>,
    event_id: EventId,
}

impl EventOrderingKey {
    /// Creates an ordering key from an event.
    #[must_use]
    pub fn from_event(event: &TelemetryEvent) -> Self {
        Self {
            timestamp: event.timestamp,
            source_id: event.source_id.clone(),
            sequence: event.sequence,
            event_id: event.id.clone(),
        }
    }

    /// Returns timestamp.
    #[must_use]
    pub const fn timestamp(&self) -> EventTimestamp {
        self.timestamp
    }

    /// Returns source.
    #[must_use]
    pub fn source_id(&self) -> &SourceId {
        &self.source_id
    }

    /// Returns sequence.
    #[must_use]
    pub const fn sequence(&self) -> Option<EventSequence> {
        self.sequence
    }

    /// Returns event ID.
    #[must_use]
    pub fn event_id(&self) -> &EventId {
        &self.event_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn event_id(value: &str) -> EventId {
        EventId::new(value).expect("test event ID must be valid")
    }

    fn source_id(value: &str) -> SourceId {
        SourceId::new(value).expect("test source ID must be valid")
    }

    fn timestamp(seconds: i64, nanos: u32) -> EventTimestamp {
        EventTimestamp::new(seconds, nanos)
            .expect("test timestamp must be valid")
    }

    #[test]
    fn timestamp_rejects_invalid_nanoseconds() {
        assert_eq!(
            EventTimestamp::new(0, EventTimestamp::NANOS_PER_SECOND),
            Err(TimestampError::InvalidNanoseconds)
        );
    }

    #[test]
    fn event_id_rejects_empty_values() {
        assert_eq!(
            EventId::new(""),
            Err(EventIdError::Empty)
        );

        assert_eq!(
            EventId::new("   "),
            Err(EventIdError::Empty)
        );
    }

    #[test]
    fn source_id_rejects_empty_values() {
        assert_eq!(
            SourceId::new(""),
            Err(SourceIdError::Empty)
        );
    }

    #[test]
    fn correlation_id_rejects_empty_values() {
        assert_eq!(
            CorrelationId::new(""),
            Err(CorrelationIdError::Empty)
        );
    }

    #[test]
    fn trace_id_rejects_empty_values() {
        assert_eq!(
            TraceId::new(""),
            Err(TraceIdError::Empty)
        );
    }

    #[test]
    fn resource_scope_deduplicates_and_sorts_qubits() {
        let scope = ResourceScope::new(
            [
                QubitId::from(4usize),
                QubitId::from(1usize),
                QubitId::from(4usize),
            ],
            [
                PhysicalQubitId::from(8usize),
                PhysicalQubitId::from(2usize),
                PhysicalQubitId::from(8usize),
            ],
            std::iter::empty(),
        );

        assert_eq!(
            scope.logical_qubits(),
            &[
                QubitId::from(1usize),
                QubitId::from(4usize)
            ]
        );

        assert_eq!(
            scope.physical_qubits(),
            &[
                PhysicalQubitId::from(2usize),
                PhysicalQubitId::from(8usize)
            ]
        );
    }

    #[test]
    fn payload_kind_is_stable() {
        let payload = EventPayload::Execution(ExecutionEvent::Started);

        assert_eq!(payload.kind(), EventKind::Execution);
    }

    #[test]
    fn custom_payload_has_custom_kind() {
        let payload = EventPayload::Custom {
            namespace: Arc::from("example"),
            name: Arc::from("future_event"),
        };

        assert_eq!(payload.kind(), EventKind::Custom);
    }

    #[test]
    fn event_is_constructed_without_hidden_state() {
        let event = TelemetryEvent::new(
            event_id("event-1"),
            timestamp(1_000, 42),
            EventSourceKind::Runtime,
            source_id("runtime-1"),
            EventTrust::Authenticated,
            ResourceScope::new(
                [QubitId::from(0usize)],
                [PhysicalQubitId::from(7usize)],
                std::iter::empty(),
            ),
            EventSeverity::Warning,
            EventPayload::Execution(ExecutionEvent::Started),
        );

        assert_eq!(
            event.schema_id(),
            TELEMETRY_EVENT_SCHEMA_ID
        );

        assert_eq!(
            event.schema_version(),
            TELEMETRY_EVENT_SCHEMA_VERSION
        );

        assert_eq!(event.kind(), EventKind::Execution);
        assert_eq!(event.severity(), EventSeverity::Warning);
        assert!(event.requires_attention());
        assert_eq!(event.resources().logical_qubit_count(), 1);
        assert_eq!(event.resources().physical_qubit_count(), 1);
    }

    #[test]
    fn builder_preserves_explicit_context() {
        let event = TelemetryEventBuilder::new(
            event_id("event-2"),
            timestamp(2_000, 10),
            EventSourceKind::Hardware,
            source_id("qpu-1"),
            EventPayload::Health(HealthEvent::Degraded),
        )
        .trust(EventTrust::IntegrityVerified)
        .severity(EventSeverity::Error)
        .sequence(EventSequence::new(17))
        .trace_id(TraceId::new("trace-1").expect("valid trace"))
        .correlation_id(
            CorrelationId::new("incident-1")
                .expect("valid correlation"),
        )
        .parent_event_id(event_id("event-1"))
        .resources(ResourceScope::new(
            [QubitId::from(3usize)],
            [PhysicalQubitId::from(9usize)],
            std::iter::empty(),
        ))
        .build();

        assert_eq!(event.sequence(), Some(EventSequence::new(17)));
        assert_eq!(
            event.trace_id().map(TraceId::as_str),
            Some("trace-1")
        );
        assert_eq!(
            event.correlation_id().map(CorrelationId::as_str),
            Some("incident-1")
        );
        assert_eq!(
            event.parent_event_id().map(EventId::as_str),
            Some("event-1")
        );
        assert_eq!(event.trust(), EventTrust::IntegrityVerified);
        assert_eq!(event.severity(), EventSeverity::Error);
    }

    #[test]
    fn ordering_key_is_deterministic() {
        let event = TelemetryEvent::new(
            event_id("event-1"),
            timestamp(1, 2),
            EventSourceKind::Runtime,
            source_id("runtime"),
            EventTrust::Authenticated,
            ResourceScope::default(),
            EventSeverity::Informational,
            EventPayload::Metadata(MetadataEvent::new(std::iter::empty())),
        )
        .with_sequence(EventSequence::new(4));

        let key_a = EventOrderingKey::from_event(&event);
        let key_b = EventOrderingKey::from_event(&event);

        assert_eq!(key_a, key_b);
    }

    #[test]
    fn severity_attention_boundary_is_correct() {
        assert!(!EventSeverity::Informational.requires_attention());
        assert!(EventSeverity::Notice.requires_attention());
        assert!(EventSeverity::Warning.requires_attention());
        assert!(EventSeverity::Error.requires_attention());
        assert!(EventSeverity::Critical.requires_attention());
        assert!(EventSeverity::Fatal.requires_attention());
    }

    #[test]
    fn schema_identity_is_stable() {
        assert_eq!(
            TELEMETRY_EVENT_SCHEMA_ID,
            "zamani.quantum.resilience.telemetry.event"
        );

        assert_eq!(
            TELEMETRY_EVENT_SCHEMA_VERSION,
            1
        );
    }

    #[test]
    fn resource_reference_rejects_empty_kind() {
        assert_eq!(
            ResourceReference::new("", "id"),
            Err(ResourceReferenceError::EmptyKind)
        );
    }

    #[test]
    fn resource_reference_rejects_empty_id() {
        assert_eq!(
            ResourceReference::new("device", ""),
            Err(ResourceReferenceError::EmptyId)
        );
    }

    #[test]
    fn metadata_rejects_empty_keys() {
        assert_eq!(
            MetadataEntry::new("", "value"),
            Err(MetadataEntryError::EmptyKey)
        );
    }
}