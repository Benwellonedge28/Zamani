//! Zamani Quantum Resilience — Telemetry Traces
//!
//! Path:
//!     src/quantum/resilience/telemetry/trace.rs
//!
//! # Purpose
//!
//! This module defines the canonical, provider-neutral trace and span model
//! used by `quantum::resilience::telemetry`.
//!
//! A trace represents the causal execution history of a quantum workload or
//! resilience operation. A trace is composed of spans. Spans represent
//! logically meaningful operations such as:
//!
//! - compilation;
//! - lowering;
//! - optimization;
//! - routing;
//! - scheduling;
//! - hardware submission;
//! - execution;
//! - measurement;
//! - QEC processing;
//! - fault detection;
//! - diagnosis;
//! - resilience planning;
//! - adaptation;
//! - recovery;
//! - mitigation;
//! - verification;
//! - migration;
//! - checkpointing.
//!
//! This module is a DATA CONTRACT.
//!
//! It does not:
//!
//! - read the system clock;
//! - generate random identifiers;
//! - generate UUIDs;
//! - communicate with tracing providers;
//! - export telemetry;
//! - write files;
//! - access networks;
//! - access quantum hardware;
//! - execute quantum programs;
//! - perform fault detection;
//! - diagnose failures;
//! - perform recovery;
//! - perform QEC;
//! - perform mitigation;
//! - serialize into a provider-specific wire format.
//!
//! Those responsibilities belong to other layers.
//!
//! # Architectural position
//!
//! ```text
//! quantum execution / compiler / hardware / QEC / resilience
//!                         │
//!                         ▼
//!                   trace producers
//!                         │
//!                         ▼
//!                       Trace
//!                         │
//!                         ▼
//!                       Span
//!                         │
//!             ┌───────────┼───────────┐
//!             ▼           ▼           ▼
//!          events       metrics     history
//!             │
//!             ▼
//!          diagnosis
//!             │
//!             ▼
//!          resilience
//! ```
//!
//! # Relationship with `event.rs`
//!
//! `event.rs` defines individual telemetry observations:
//!
//! ```text
//! TelemetryEvent
//! ```
//!
//! This module defines causal execution structure:
//!
//! ```text
//! Trace
//! └── Span
//!     ├── child Span
//!     ├── child Span
//!     └── EventId references
//! ```
//!
//! Event identity and span identity are deliberately different.
//!
//! A single event can be associated with a span, while a span may contain
//! multiple events or no events.
//!
//! # Write once, scale everywhere
//!
//! No machine-size assumptions exist in this module.
//!
//! There is no:
//!
//! ```text
//! MAX_QUBITS
//! MAX_SPANS
//! MAX_EVENTS
//! MAX_TRACE_DEPTH
//! MAX_ATTRIBUTES
//! ```
//!
//! Trace cardinality is controlled by the telemetry collector, retention
//! policy, resource policy, storage policy, or execution policy rather than
//! by this data structure.
//!
//! The representation naturally scales from one operation to arbitrarily
//! large finite execution graphs subject only to available resources.
//!
//! # Canonical quantum identities
//!
//! This module never defines another qubit identity type.
//!
//! Canonical identities come from:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! This is important because routing, scheduling, ZQN, hardware and
//! resilience must agree on resource identity.
//!
//! # Determinism
//!
//! This module does not generate timestamps or identifiers.
//!
//! Producers explicitly supply:
//!
//! - trace IDs;
//! - span IDs;
//! - timestamps;
//! - sequence numbers;
//! - parent relationships;
//! - event IDs.
//!
//! `BTreeMap` is used for attributes so iteration order is deterministic.
//!
//! Deterministic replay therefore does not depend on hash-map iteration order.
//!
//! # Ordering
//!
//! Timestamps do not establish a total order by themselves.
//!
//! Events/spans may have equal timestamps and distributed clocks may differ.
//!
//! Producer sequence numbers and explicit parent relationships therefore
//! provide additional ordering information.
//!
//! Consumers MUST NOT infer causality from timestamps alone.
//!
//! # Distributed execution
//!
//! A trace may span:
//!
//! - multiple processes;
//! - multiple runtime instances;
//! - multiple QPUs;
//! - multiple backends;
//! - simulators;
//! - emulators;
//! - classical orchestration services.
//!
//! Parent/child relationships and trace identifiers therefore use opaque
//! producer-defined identifiers rather than process-local counters.
//!
//! # Security
//!
//! Trace data is telemetry and therefore untrusted evidence.
//!
//! A trace MUST NOT be treated as proof that an operation actually succeeded.
//! Verification remains authoritative for semantic correctness.
//!
//! Trace attributes MUST NOT contain:
//!
//! - credentials;
//! - private keys;
//! - authentication tokens;
//! - session secrets;
//! - raw provider secrets.
//!
//! Trace data can contain sensitive execution information and must therefore
//! be protected according to the surrounding telemetry/security policy.
//!
//! # Serialization
//!
//! Serialization is intentionally not implemented here.
//!
//! Canonical resilience serialization belongs under:
//!
//! ```text
//! quantum::resilience::serialization
//! ```
//!
//! This keeps the trace model independent of JSON, CBOR, protobuf, OpenTelemetry
//! or any other transport format.
//!
//! # Integration
//!
//! `telemetry/mod.rs` should expose:
//!
//! ```text
//! pub mod trace;
//! ```
//!
//! Downstream consumers should use:
//!
//! ```text
//! crate::quantum::resilience::telemetry::trace::Trace
//! crate::quantum::resilience::telemetry::trace::Span
//! ```
//!
//! `event.rs` supplies event identity and timestamps.
//!
//! `collector.rs` constructs traces and spans.
//!
//! `exporter.rs` consumes immutable traces/spans.
//!
//! `history` may persist traces subject to retention policy.
//!
//! `diagnosis` may consume trace information as evidence.
//!
//! `verification` remains responsible for determining whether execution was
//! semantically correct.
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

use std::collections::BTreeMap;
use std::fmt;
use std::sync::Arc;

use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId};

use super::event::{EventId, EventTimestamp};

/// Stable semantic schema identifier for resilience traces.
pub const TELEMETRY_TRACE_SCHEMA_ID: &str =
    "zamani.quantum.resilience.telemetry.trace";

/// Current semantic schema version.
///
/// This identifies the trace schema. It is not a hardware-size limit.
pub const TELEMETRY_TRACE_SCHEMA_VERSION: u32 = 1;

// =============================================================================
// Trace identifiers
// =============================================================================

/// Opaque identifier for a complete distributed trace.
///
/// The producer chooses the identifier format. This module does not require
/// UUID, ULID, hashes, database IDs, or counters.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TraceId(Arc<str>);

impl TraceId {
    /// Creates an explicit trace identifier.
    ///
    /// Empty and whitespace-only identifiers are rejected because they cannot
    /// reliably identify a trace.
    pub fn new(value: impl Into<Arc<str>>) -> Result<Self, TraceIdError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(TraceIdError::Empty);
        }

        Ok(Self(value))
    }

    /// Returns the identifier as a string slice.
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

/// Errors produced while constructing [`TraceId`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TraceIdError {
    /// The identifier was empty or whitespace-only.
    Empty,
}

impl fmt::Display for TraceIdError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("trace identifier is empty"),
        }
    }
}

impl std::error::Error for TraceIdError {}

/// Opaque identifier for an individual span.
///
/// Span identifiers are distinct from [`EventId`].
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SpanId(Arc<str>);

impl SpanId {
    /// Creates an explicit span identifier.
    pub fn new(value: impl Into<Arc<str>>) -> Result<Self, SpanIdError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(SpanIdError::Empty);
        }

        Ok(Self(value))
    }

    /// Returns the identifier as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

impl fmt::Display for SpanId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Errors produced while constructing [`SpanId`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpanIdError {
    /// The identifier was empty or whitespace-only.
    Empty,
}

impl fmt::Display for SpanIdError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("span identifier is empty"),
        }
    }
}

impl std::error::Error for SpanIdError {}

// =============================================================================
// Span kind
// =============================================================================

/// Semantic class of a trace span.
///
/// This classification is intentionally provider-neutral.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SpanKind {
    /// Top-level quantum workload.
    QuantumProgram,

    /// Compilation activity.
    Compilation,

    /// IR transformation/lowering.
    Lowering,

    /// Optimization activity.
    Optimization,

    /// Logical-to-physical routing.
    Routing,

    /// Schedule generation or execution scheduling.
    Scheduling,

    /// Hardware submission.
    Submission,

    /// Quantum execution.
    Execution,

    /// Measurement activity.
    Measurement,

    /// Quantum error correction.
    Qec,

    /// Error mitigation.
    Mitigation,

    /// Fault detection.
    Detection,

    /// Fault diagnosis.
    Diagnosis,

    /// Recovery planning.
    Planning,

    /// Adaptation of a workload to changed resources.
    Adaptation,

    /// Recovery operation.
    Recovery,

    /// Verification.
    Verification,

    /// Checkpoint creation or restoration.
    Checkpoint,

    /// Migration between execution resources.
    Migration,

    /// Backend discovery or selection.
    BackendSelection,

    /// Generic internal resilience operation.
    Internal,

    /// External operation.
    External,

    /// Unknown or future span category.
    Unknown,
}

impl SpanKind {
    /// Returns a stable machine-readable representation.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::QuantumProgram => "quantum_program",
            Self::Compilation => "compilation",
            Self::Lowering => "lowering",
            Self::Optimization => "optimization",
            Self::Routing => "routing",
            Self::Scheduling => "scheduling",
            Self::Submission => "submission",
            Self::Execution => "execution",
            Self::Measurement => "measurement",
            Self::Qec => "qec",
            Self::Mitigation => "mitigation",
            Self::Detection => "detection",
            Self::Diagnosis => "diagnosis",
            Self::Planning => "planning",
            Self::Adaptation => "adaptation",
            Self::Recovery => "recovery",
            Self::Verification => "verification",
            Self::Checkpoint => "checkpoint",
            Self::Migration => "migration",
            Self::BackendSelection => "backend_selection",
            Self::Internal => "internal",
            Self::External => "external",
            Self::Unknown => "unknown",
        }
    }
}

impl fmt::Display for SpanKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Span status
// =============================================================================

/// Final semantic status of a span.
///
/// A span status is an observation about the span lifecycle. It is NOT proof
/// that the quantum computation produced a correct result.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SpanStatus {
    /// Span is still active.
    Running,

    /// Span completed normally.
    Ok,

    /// Span completed with a failure.
    Error,

    /// Span was intentionally cancelled.
    Cancelled,

    /// Span was intentionally skipped.
    Skipped,

    /// Span ended without a definitive status.
    Unknown,
}

impl SpanStatus {
    /// Returns a stable machine-readable representation.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Running => "running",
            Self::Ok => "ok",
            Self::Error => "error",
            Self::Cancelled => "cancelled",
            Self::Skipped => "skipped",
            Self::Unknown => "unknown",
        }
    }

    /// Returns whether the span represents successful completion.
    ///
    /// This means only that the span reported success. It does not imply
    /// semantic verification of the quantum computation.
    #[must_use]
    pub const fn is_success(self) -> bool {
        matches!(self, Self::Ok)
    }

    /// Returns whether the span represents a failure.
    #[must_use]
    pub const fn is_failure(self) -> bool {
        matches!(self, Self::Error)
    }

    /// Returns whether the span is still active.
    #[must_use]
    pub const fn is_running(self) -> bool {
        matches!(self, Self::Running)
    }
}

impl fmt::Display for SpanStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Attribute values
// =============================================================================

/// Provider-neutral typed telemetry attribute value.
///
/// This avoids making the core trace model depend on JSON or another
/// serialization technology.
///
/// Nested structures are intentionally not required here. If a caller needs
/// complex structured data, it should either emit multiple attributes/events
/// or use a versioned serialization-layer representation.
#[derive(Debug, Clone, PartialEq)]
pub enum TraceAttributeValue {
    /// UTF-8 text.
    String(String),

    /// Boolean value.
    Boolean(bool),

    /// Signed integer.
    Integer(i64),

    /// Unsigned integer.
    Unsigned(u64),

    /// Floating-point value.
    ///
    /// Consumers must define the handling of NaN and infinity according to
    /// their serialization/telemetry policy.
    Float(f64),

    /// Opaque bytes.
    ///
    /// This variant MUST NOT be used for secrets.
    Bytes(Vec<u8>),
}

impl TraceAttributeValue {
    /// Creates a string attribute.
    #[must_use]
    pub fn string(value: impl Into<String>) -> Self {
        Self::String(value.into())
    }

    /// Creates a boolean attribute.
    #[must_use]
    pub const fn boolean(value: bool) -> Self {
        Self::Boolean(value)
    }

    /// Creates a signed integer attribute.
    #[must_use]
    pub const fn integer(value: i64) -> Self {
        Self::Integer(value)
    }

    /// Creates an unsigned integer attribute.
    #[must_use]
    pub const fn unsigned(value: u64) -> Self {
        Self::Unsigned(value)
    }

    /// Creates a floating-point attribute.
    #[must_use]
    pub const fn float(value: f64) -> Self {
        Self::Float(value)
    }

    /// Creates an opaque byte attribute.
    #[must_use]
    pub fn bytes(value: impl Into<Vec<u8>>) -> Self {
        Self::Bytes(value.into())
    }
}

impl From<String> for TraceAttributeValue {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for TraceAttributeValue {
    fn from(value: &str) -> Self {
        Self::String(value.to_owned())
    }
}

impl From<bool> for TraceAttributeValue {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}

impl From<i64> for TraceAttributeValue {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}

impl From<u64> for TraceAttributeValue {
    fn from(value: u64) -> Self {
        Self::Unsigned(value)
    }
}

impl From<f64> for TraceAttributeValue {
    fn from(value: f64) -> Self {
        Self::Float(value)
    }
}

// =============================================================================
// Resource attribution
// =============================================================================

/// Resource explicitly associated with a trace/span.
///
/// Canonical qubit identity types are reused from the IR.
///
/// Generic resource identifiers remain strings so this module does not need
/// to know every future Zamani resource kind.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TraceResource {
    /// Logical qubit.
    LogicalQubit(QubitId),

    /// Physical qubit.
    PhysicalQubit(PhysicalQubitId),

    /// Generic resource identifier.
    Resource(Arc<str>),
}

impl TraceResource {
    /// Creates a generic resource reference.
    pub fn resource(value: impl Into<Arc<str>>) -> Result<Self, TraceResourceError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(TraceResourceError::EmptyResource);
        }

        Ok(Self::Resource(value))
    }
}

/// Resource-reference construction errors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TraceResourceError {
    /// Generic resource ID was empty.
    EmptyResource,
}

impl fmt::Display for TraceResourceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyResource => {
                formatter.write_str("trace resource identifier is empty")
            }
        }
    }
}

impl std::error::Error for TraceResourceError {}

// =============================================================================
// Span links
// =============================================================================

/// A causal or contextual relationship between spans.
///
/// Links allow distributed execution to represent relationships that are not
/// a simple parent/child tree.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SpanLink {
    /// Trace containing the linked span.
    trace_id: TraceId,

    /// Linked span.
    span_id: SpanId,

    /// Relationship classification.
    relationship: SpanLinkRelationship,
}

impl SpanLink {
    /// Creates a span link.
    #[must_use]
    pub fn new(
        trace_id: TraceId,
        span_id: SpanId,
        relationship: SpanLinkRelationship,
    ) -> Self {
        Self {
            trace_id,
            span_id,
            relationship,
        }
    }

    /// Returns the linked trace.
    #[must_use]
    pub fn trace_id(&self) -> &TraceId {
        &self.trace_id
    }

    /// Returns the linked span.
    #[must_use]
    pub fn span_id(&self) -> &SpanId {
        &self.span_id
    }

    /// Returns the relationship.
    #[must_use]
    pub const fn relationship(&self) -> SpanLinkRelationship {
        self.relationship
    }
}

/// Semantic relationship between spans.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SpanLinkRelationship {
    /// Linked span represents causal predecessor.
    FollowsFrom,

    /// Linked span is a concurrent/related operation.
    Related,

    /// Linked span represents a retry or continuation.
    RetryOf,

    /// Linked span represents migrated work.
    MigratedFrom,

    /// Relationship is intentionally unspecified.
    Unknown,
}

// =============================================================================
// Span
// =============================================================================

/// Immutable trace span.
///
/// A span represents one logical unit of execution.
///
/// Spans are constructed through [`SpanBuilder`] and should then be treated as
/// immutable telemetry facts.
#[derive(Debug, Clone, PartialEq)]
pub struct Span {
    trace_id: TraceId,
    span_id: SpanId,
    parent_span_id: Option<SpanId>,
    name: Arc<str>,
    kind: SpanKind,
    status: SpanStatus,
    started_at: EventTimestamp,
    ended_at: Option<EventTimestamp>,
    sequence: Option<u64>,
    resources: Vec<TraceResource>,
    links: Vec<SpanLink>,
    event_ids: Vec<EventId>,
    attributes: BTreeMap<String, TraceAttributeValue>,
}

impl Span {
    /// Creates a span builder.
    #[must_use]
    pub fn builder(
        trace_id: TraceId,
        span_id: SpanId,
        name: impl Into<Arc<str>>,
        kind: SpanKind,
        started_at: EventTimestamp,
    ) -> SpanBuilder {
        SpanBuilder::new(trace_id, span_id, name, kind, started_at)
    }

    /// Returns the trace identifier.
    #[must_use]
    pub fn trace_id(&self) -> &TraceId {
        &self.trace_id
    }

    /// Returns the span identifier.
    #[must_use]
    pub fn span_id(&self) -> &SpanId {
        &self.span_id
    }

    /// Returns the optional parent span.
    #[must_use]
    pub fn parent_span_id(&self) -> Option<&SpanId> {
        self.parent_span_id.as_ref()
    }

    /// Returns the span name.
    #[must_use]
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    /// Returns the semantic span kind.
    #[must_use]
    pub const fn kind(&self) -> SpanKind {
        self.kind
    }

    /// Returns the current/final span status.
    #[must_use]
    pub const fn status(&self) -> SpanStatus {
        self.status
    }

    /// Returns the start timestamp.
    #[must_use]
    pub const fn started_at(&self) -> EventTimestamp {
        self.started_at
    }

    /// Returns the end timestamp, if available.
    #[must_use]
    pub const fn ended_at(&self) -> Option<EventTimestamp> {
        self.ended_at
    }

    /// Returns the producer sequence number.
    #[must_use]
    pub const fn sequence(&self) -> Option<u64> {
        self.sequence
    }

    /// Returns all attributed resources.
    #[must_use]
    pub fn resources(&self) -> &[TraceResource] {
        &self.resources
    }

    /// Returns all span links.
    #[must_use]
    pub fn links(&self) -> &[SpanLink] {
        &self.links
    }

    /// Returns associated event identifiers.
    #[must_use]
    pub fn event_ids(&self) -> &[EventId] {
        &self.event_ids
    }

    /// Returns deterministic attributes.
    #[must_use]
    pub fn attributes(&self) -> &BTreeMap<String, TraceAttributeValue> {
        &self.attributes
    }

    /// Returns whether this span has ended.
    #[must_use]
    pub const fn is_ended(&self) -> bool {
        !self.status.is_running()
    }

    /// Returns the duration when both timestamps are available and ordered.
    ///
    /// The duration is expressed in nanoseconds when representable.
    ///
    /// `None` is returned when the span is still running or the timestamp
    /// interval cannot be represented safely as a `u128` nanosecond value.
    #[must_use]
    pub fn duration_nanos(&self) -> Option<u128> {
        let end = self.ended_at?;

        let start_seconds = i128::from(self.started_at.unix_seconds());
        let end_seconds = i128::from(end.unix_seconds());

        let start_nanos = i128::from(self.started_at.nanos());
        let end_nanos = i128::from(end.nanos());

        let total_start = start_seconds
            .checked_mul(i128::from(EventTimestamp::NANOS_PER_SECOND))?
            .checked_add(start_nanos)?;

        let total_end = end_seconds
            .checked_mul(i128::from(EventTimestamp::NANOS_PER_SECOND))?
            .checked_add(end_nanos)?;

        let duration = total_end.checked_sub(total_start)?;

        if duration < 0 {
            return None;
        }

        u128::try_from(duration).ok()
    }
}

// =============================================================================
// Span builder
// =============================================================================

/// Builder for immutable [`Span`] values.
///
/// Required values are validated at `build()` time so partially initialized
/// spans cannot escape through the public API.
#[derive(Debug)]
pub struct SpanBuilder {
    trace_id: TraceId,
    span_id: SpanId,
    parent_span_id: Option<SpanId>,
    name: Arc<str>,
    kind: SpanKind,
    status: SpanStatus,
    started_at: EventTimestamp,
    ended_at: Option<EventTimestamp>,
    sequence: Option<u64>,
    resources: Vec<TraceResource>,
    links: Vec<SpanLink>,
    event_ids: Vec<EventId>,
    attributes: BTreeMap<String, TraceAttributeValue>,
}

impl SpanBuilder {
    /// Creates a span builder.
    #[must_use]
    pub fn new(
        trace_id: TraceId,
        span_id: SpanId,
        name: impl Into<Arc<str>>,
        kind: SpanKind,
        started_at: EventTimestamp,
    ) -> Self {
        Self {
            trace_id,
            span_id,
            parent_span_id: None,
            name: name.into(),
            kind,
            status: SpanStatus::Running,
            started_at,
            ended_at: None,
            sequence: None,
            resources: Vec::new(),
            links: Vec::new(),
            event_ids: Vec::new(),
            attributes: BTreeMap::new(),
        }
    }

    /// Sets the parent span.
    #[must_use]
    pub fn parent_span(mut self, parent: SpanId) -> Self {
        self.parent_span_id = Some(parent);
        self
    }

    /// Sets the final span status.
    #[must_use]
    pub fn status(mut self, status: SpanStatus) -> Self {
        self.status = status;
        self
    }

    /// Sets the explicit producer sequence.
    #[must_use]
    pub fn sequence(mut self, sequence: u64) -> Self {
        self.sequence = Some(sequence);
        self
    }

    /// Sets the end timestamp.
    #[must_use]
    pub fn ended_at(mut self, timestamp: EventTimestamp) -> Self {
        self.ended_at = Some(timestamp);
        self
    }

    /// Adds a logical qubit resource.
    #[must_use]
    pub fn logical_qubit(mut self, qubit: QubitId) -> Self {
        self.resources.push(TraceResource::LogicalQubit(qubit));
        self
    }

    /// Adds a physical qubit resource.
    #[must_use]
    pub fn physical_qubit(mut self, qubit: PhysicalQubitId) -> Self {
        self.resources
            .push(TraceResource::PhysicalQubit(qubit));
        self
    }

    /// Adds a generic resource.
    pub fn resource(
        mut self,
        resource: impl Into<Arc<str>>,
    ) -> Result<Self, TraceResourceError> {
        self.resources
            .push(TraceResource::resource(resource)?);
        Ok(self)
    }

    /// Adds a causal/contextual span link.
    #[must_use]
    pub fn link(mut self, link: SpanLink) -> Self {
        self.links.push(link);
        self
    }

    /// Associates an existing telemetry event with this span.
    #[must_use]
    pub fn event(mut self, event_id: EventId) -> Self {
        self.event_ids.push(event_id);
        self
    }

    /// Adds or replaces an attribute.
    ///
    /// `BTreeMap` guarantees deterministic key ordering.
    pub fn attribute(
        mut self,
        key: impl Into<String>,
        value: impl Into<TraceAttributeValue>,
    ) -> Result<Self, TraceAttributeError> {
        let key = validate_attribute_key(key.into())?;

        self.attributes.insert(key, value.into());
        Ok(self)
    }

    /// Adds a string attribute.
    pub fn string_attribute(
        self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Self, TraceAttributeError> {
        self.attribute(key, TraceAttributeValue::String(value.into()))
    }

    /// Builds the immutable span.
    pub fn build(self) -> Result<Span, SpanError> {
        validate_span_name(&self.name)?;

        if self.status.is_running() && self.ended_at.is_some() {
            return Err(SpanError::RunningSpanHasEndTimestamp);
        }

        if !self.status.is_running() && self.ended_at.is_none() {
            return Err(SpanError::CompletedSpanMissingEndTimestamp);
        }

        if let Some(end) = self.ended_at {
            if compare_timestamps(end, self.started_at).is_lt() {
                return Err(SpanError::EndBeforeStart);
            }
        }

        if let Some(parent) = &self.parent_span_id {
            if parent == &self.span_id {
                return Err(SpanError::SelfParent);
            }
        }

        Ok(Span {
            trace_id: self.trace_id,
            span_id: self.span_id,
            parent_span_id: self.parent_span_id,
            name: self.name,
            kind: self.kind,
            status: self.status,
            started_at: self.started_at,
            ended_at: self.ended_at,
            sequence: self.sequence,
            resources: self.resources,
            links: self.links,
            event_ids: self.event_ids,
            attributes: self.attributes,
        })
    }
}

// =============================================================================
// Trace
// =============================================================================

/// Immutable distributed execution trace.
///
/// A trace is a collection of causally related spans.
///
/// The trace does not require a tree: distributed systems can contain
/// cross-links between spans.
#[derive(Debug, Clone, PartialEq)]
pub struct Trace {
    trace_id: TraceId,
    root_span_id: Option<SpanId>,
    name: Arc<str>,
    started_at: EventTimestamp,
    ended_at: Option<EventTimestamp>,
    status: TraceStatus,
    spans: Vec<Span>,
    attributes: BTreeMap<String, TraceAttributeValue>,
}

impl Trace {
    /// Creates a trace builder.
    #[must_use]
    pub fn builder(
        trace_id: TraceId,
        name: impl Into<Arc<str>>,
        started_at: EventTimestamp,
    ) -> TraceBuilder {
        TraceBuilder::new(trace_id, name, started_at)
    }

    /// Returns the trace identifier.
    #[must_use]
    pub fn trace_id(&self) -> &TraceId {
        &self.trace_id
    }

    /// Returns the root span identifier.
    #[must_use]
    pub fn root_span_id(&self) -> Option<&SpanId> {
        self.root_span_id.as_ref()
    }

    /// Returns the trace name.
    #[must_use]
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    /// Returns the start timestamp.
    #[must_use]
    pub const fn started_at(&self) -> EventTimestamp {
        self.started_at
    }

    /// Returns the end timestamp.
    #[must_use]
    pub const fn ended_at(&self) -> Option<EventTimestamp> {
        self.ended_at
    }

    /// Returns the final trace status.
    #[must_use]
    pub const fn status(&self) -> TraceStatus {
        self.status
    }

    /// Returns all spans in producer-supplied order.
    ///
    /// Consumers must use explicit sequence/parent information when they need
    /// causal ordering.
    #[must_use]
    pub fn spans(&self) -> &[Span] {
        &self.spans
    }

    /// Returns deterministic trace attributes.
    #[must_use]
    pub fn attributes(&self) -> &BTreeMap<String, TraceAttributeValue> {
        &self.attributes
    }

    /// Returns the span with the requested identifier.
    #[must_use]
    pub fn span(&self, span_id: &SpanId) -> Option<&Span> {
        self.spans.iter().find(|span| span.span_id() == span_id)
    }

    /// Returns whether the trace has completed.
    #[must_use]
    pub const fn is_ended(&self) -> bool {
        !matches!(self.status, TraceStatus::Running)
    }

    /// Returns the total trace duration when the trace has an end timestamp.
    #[must_use]
    pub fn duration_nanos(&self) -> Option<u128> {
        let end = self.ended_at?;

        let start_seconds = i128::from(self.started_at.unix_seconds());
        let end_seconds = i128::from(end.unix_seconds());

        let start_nanos = i128::from(self.started_at.nanos());
        let end_nanos = i128::from(end.nanos());

        let total_start = start_seconds
            .checked_mul(i128::from(EventTimestamp::NANOS_PER_SECOND))?
            .checked_add(start_nanos)?;

        let total_end = end_seconds
            .checked_mul(i128::from(EventTimestamp::NANOS_PER_SECOND))?
            .checked_add(end_nanos)?;

        let duration = total_end.checked_sub(total_start)?;

        if duration < 0 {
            return None;
        }

        u128::try_from(duration).ok()
    }
}

// =============================================================================
// Trace builder
// =============================================================================

/// Builder for immutable [`Trace`] values.
#[derive(Debug)]
pub struct TraceBuilder {
    trace_id: TraceId,
    root_span_id: Option<SpanId>,
    name: Arc<str>,
    started_at: EventTimestamp,
    ended_at: Option<EventTimestamp>,
    status: TraceStatus,
    spans: Vec<Span>,
    attributes: BTreeMap<String, TraceAttributeValue>,
}

impl TraceBuilder {
    /// Creates a trace builder.
    #[must_use]
    pub fn new(
        trace_id: TraceId,
        name: impl Into<Arc<str>>,
        started_at: EventTimestamp,
    ) -> Self {
        Self {
            trace_id,
            root_span_id: None,
            name: name.into(),
            started_at,
            ended_at: None,
            status: TraceStatus::Running,
            spans: Vec::new(),
            attributes: BTreeMap::new(),
        }
    }

    /// Sets the root span.
    #[must_use]
    pub fn root_span(mut self, span_id: SpanId) -> Self {
        self.root_span_id = Some(span_id);
        self
    }

    /// Sets the final trace status.
    #[must_use]
    pub fn status(mut self, status: TraceStatus) -> Self {
        self.status = status;
        self
    }

    /// Sets the trace end timestamp.
    #[must_use]
    pub fn ended_at(mut self, timestamp: EventTimestamp) -> Self {
        self.ended_at = Some(timestamp);
        self
    }

    /// Adds a completed/immutable span to the trace.
    ///
    /// The span must belong to this trace.
    pub fn span(mut self, span: Span) -> Result<Self, TraceError> {
        if span.trace_id() != &self.trace_id {
            return Err(TraceError::SpanBelongsToDifferentTrace);
        }

        if let Some(existing) = self
            .spans
            .iter()
            .find(|existing| existing.span_id() == span.span_id())
        {
            if existing != &span {
                return Err(TraceError::DuplicateSpanId);
            }

            return Ok(self);
        }

        self.spans.push(span);
        Ok(self)
    }

    /// Adds or replaces a trace attribute.
    pub fn attribute(
        mut self,
        key: impl Into<String>,
        value: impl Into<TraceAttributeValue>,
    ) -> Result<Self, TraceAttributeError> {
        let key = validate_attribute_key(key.into())?;

        self.attributes.insert(key, value.into());
        Ok(self)
    }

    /// Builds the immutable trace.
    pub fn build(self) -> Result<Trace, TraceError> {
        validate_trace_name(&self.name)?;

        if self.status == TraceStatus::Running && self.ended_at.is_some() {
            return Err(TraceError::RunningTraceHasEndTimestamp);
        }

        if self.status != TraceStatus::Running && self.ended_at.is_none() {
            return Err(TraceError::CompletedTraceMissingEndTimestamp);
        }

        if let Some(end) = self.ended_at {
            if compare_timestamps(end, self.started_at).is_lt() {
                return Err(TraceError::EndBeforeStart);
            }
        }

        if let Some(root) = &self.root_span_id {
            let root_span = self
                .spans
                .iter()
                .find(|span| span.span_id() == root);

            let Some(root_span) = root_span else {
                return Err(TraceError::RootSpanNotPresent);
            };

            if root_span.parent_span_id().is_some() {
                return Err(TraceError::RootSpanHasParent);
            }
        }

        for span in &self.spans {
            if span.trace_id() != &self.trace_id {
                return Err(TraceError::SpanBelongsToDifferentTrace);
            }

            if let Some(parent) = span.parent_span_id() {
                if parent == span.span_id() {
                    return Err(TraceError::SpanSelfParent);
                }
            }
        }

        Ok(Trace {
            trace_id: self.trace_id,
            root_span_id: self.root_span_id,
            name: self.name,
            started_at: self.started_at,
            ended_at: self.ended_at,
            status: self.status,
            spans: self.spans,
            attributes: self.attributes,
        })
    }
}

// =============================================================================
// Trace status
// =============================================================================

/// Overall lifecycle status of a trace.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TraceStatus {
    /// Trace is still active.
    Running,

    /// Trace completed successfully from the perspective of its lifecycle.
    ///
    /// This does NOT mean semantic verification succeeded.
    Ok,

    /// Trace completed with an execution/resilience error.
    Error,

    /// Trace was intentionally cancelled.
    Cancelled,

    /// Trace ended in a degraded but usable condition.
    Degraded,

    /// Trace ended without a definitive status.
    Unknown,
}

impl TraceStatus {
    /// Returns a stable machine-readable representation.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Running => "running",
            Self::Ok => "ok",
            Self::Error => "error",
            Self::Cancelled => "cancelled",
            Self::Degraded => "degraded",
            Self::Unknown => "unknown",
        }
    }

    /// Returns whether the trace is still active.
    #[must_use]
    pub const fn is_running(self) -> bool {
        matches!(self, Self::Running)
    }
}

impl fmt::Display for TraceStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Errors
// =============================================================================

/// Errors produced while constructing spans.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpanError {
    /// Span name is empty or whitespace-only.
    EmptyName,

    /// Span is marked running but has an end timestamp.
    RunningSpanHasEndTimestamp,

    /// Span is completed but has no end timestamp.
    CompletedSpanMissingEndTimestamp,

    /// End timestamp precedes start timestamp.
    EndBeforeStart,

    /// A span cannot be its own parent.
    SelfParent,
}

impl fmt::Display for SpanError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => formatter.write_str("span name is empty"),
            Self::RunningSpanHasEndTimestamp => {
                formatter.write_str("running span cannot have an end timestamp")
            }
            Self::CompletedSpanMissingEndTimestamp => {
                formatter.write_str("completed span requires an end timestamp")
            }
            Self::EndBeforeStart => {
                formatter.write_str("span end timestamp precedes start timestamp")
            }
            Self::SelfParent => {
                formatter.write_str("span cannot be its own parent")
            }
        }
    }
}

impl std::error::Error for SpanError {}

/// Errors produced while constructing traces.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TraceError {
    /// Trace name is empty or whitespace-only.
    EmptyName,

    /// Running trace has an end timestamp.
    RunningTraceHasEndTimestamp,

    /// Completed trace has no end timestamp.
    CompletedTraceMissingEndTimestamp,

    /// Trace end precedes start.
    EndBeforeStart,

    /// Span belongs to another trace.
    SpanBelongsToDifferentTrace,

    /// Two different spans use the same identifier.
    DuplicateSpanId,

    /// Declared root span was not added to the trace.
    RootSpanNotPresent,

    /// Root span cannot itself have a parent.
    RootSpanHasParent,

    /// A span references itself as parent.
    SpanSelfParent,
}

impl fmt::Display for TraceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => formatter.write_str("trace name is empty"),
            Self::RunningTraceHasEndTimestamp => {
                formatter.write_str("running trace cannot have an end timestamp")
            }
            Self::CompletedTraceMissingEndTimestamp => {
                formatter.write_str("completed trace requires an end timestamp")
            }
            Self::EndBeforeStart => {
                formatter.write_str("trace end timestamp precedes start timestamp")
            }
            Self::SpanBelongsToDifferentTrace => {
                formatter.write_str("span belongs to a different trace")
            }
            Self::DuplicateSpanId => {
                formatter.write_str("trace contains conflicting duplicate span ID")
            }
            Self::RootSpanNotPresent => {
                formatter.write_str("declared root span is not present in trace")
            }
            Self::RootSpanHasParent => {
                formatter.write_str("root span cannot have a parent")
            }
            Self::SpanSelfParent => {
                formatter.write_str("span cannot be its own parent")
            }
        }
    }
}

impl std::error::Error for TraceError {}

/// Errors produced while adding trace attributes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TraceAttributeError {
    /// Attribute key is empty.
    EmptyKey,

    /// Attribute key contains a control character.
    ControlCharacter,
}

impl fmt::Display for TraceAttributeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyKey => formatter.write_str("trace attribute key is empty"),
            Self::ControlCharacter => {
                formatter.write_str("trace attribute key contains a control character")
            }
        }
    }
}

impl std::error::Error for TraceAttributeError {}

// =============================================================================
// Validation helpers
// =============================================================================

fn validate_trace_name(name: &str) -> Result<(), TraceError> {
    if name.trim().is_empty() {
        return Err(TraceError::EmptyName);
    }

    Ok(())
}

fn validate_span_name(name: &str) -> Result<(), SpanError> {
    if name.trim().is_empty() {
        return Err(SpanError::EmptyName);
    }

    Ok(())
}

fn validate_attribute_key(
    key: String,
) -> Result<String, TraceAttributeError> {
    if key.trim().is_empty() {
        return Err(TraceAttributeError::EmptyKey);
    }

    if key.chars().any(char::is_control) {
        return Err(TraceAttributeError::ControlCharacter);
    }

    Ok(key)
}

fn compare_timestamps(
    left: EventTimestamp,
    right: EventTimestamp,
) -> std::cmp::Ordering {
    left.cmp(&right)
}

// =============================================================================
// Public integration helpers
// =============================================================================

/// Returns the canonical trace schema identifier.
///
/// This is useful to serialization/export layers without making this module
/// responsible for serialization.
#[must_use]
pub const fn schema_id() -> &'static str {
    TELEMETRY_TRACE_SCHEMA_ID
}

/// Returns the current trace schema version.
#[must_use]
pub const fn schema_version() -> u32 {
    TELEMETRY_TRACE_SCHEMA_VERSION
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn timestamp(seconds: i64, nanos: u32) -> EventTimestamp {
        EventTimestamp::new(seconds, nanos).expect("valid timestamp")
    }

    fn trace_id() -> TraceId {
        TraceId::new("trace-1").expect("valid trace ID")
    }

    fn span_id(value: &str) -> SpanId {
        SpanId::new(value).expect("valid span ID")
    }

    #[test]
    fn trace_id_rejects_empty_values() {
        assert_eq!(
            TraceId::new(""),
            Err(TraceIdError::Empty)
        );

        assert_eq!(
            TraceId::new("   "),
            Err(TraceIdError::Empty)
        );
    }

    #[test]
    fn span_id_rejects_empty_values() {
        assert_eq!(
            SpanId::new(""),
            Err(SpanIdError::Empty)
        );
    }

    #[test]
    fn running_span_can_be_built_without_end_timestamp() {
        let span = Span::builder(
            trace_id(),
            span_id("span-1"),
            "execution",
            SpanKind::Execution,
            timestamp(100, 0),
        )
        .build()
        .expect("running span should build");

        assert_eq!(span.status(), SpanStatus::Running);
        assert!(span.ended_at().is_none());
        assert!(span.duration_nanos().is_none());
    }

    #[test]
    fn completed_span_requires_end_timestamp() {
        let result = Span::builder(
            trace_id(),
            span_id("span-1"),
            "execution",
            SpanKind::Execution,
            timestamp(100, 0),
        )
        .status(SpanStatus::Ok)
        .build();

        assert_eq!(
            result,
            Err(SpanError::CompletedSpanMissingEndTimestamp)
        );
    }

    #[test]
    fn span_rejects_end_before_start() {
        let result = Span::builder(
            trace_id(),
            span_id("span-1"),
            "execution",
            SpanKind::Execution,
            timestamp(100, 0),
        )
        .status(SpanStatus::Ok)
        .ended_at(timestamp(99, 0))
        .build();

        assert_eq!(result, Err(SpanError::EndBeforeStart));
    }

    #[test]
    fn span_duration_is_computed_without_overflow_for_normal_values() {
        let span = Span::builder(
            trace_id(),
            span_id("span-1"),
            "execution",
            SpanKind::Execution,
            timestamp(100, 100),
        )
        .status(SpanStatus::Ok)
        .ended_at(timestamp(102, 200))
        .build()
        .expect("valid span");

        assert_eq!(span.duration_nanos(), Some(2_000_000_100));
    }

    #[test]
    fn span_cannot_parent_itself() {
        let id = span_id("span-1");

        let result = Span::builder(
            trace_id(),
            id.clone(),
            "execution",
            SpanKind::Execution,
            timestamp(100, 0),
        )
        .parent_span(id)
        .build();

        assert_eq!(result, Err(SpanError::SelfParent));
    }

    #[test]
    fn trace_requires_completed_end_timestamp() {
        let result = Trace::builder(
            trace_id(),
            "program",
            timestamp(100, 0),
        )
        .status(TraceStatus::Ok)
        .build();

        assert_eq!(
            result,
            Err(TraceError::CompletedTraceMissingEndTimestamp)
        );
    }

    #[test]
    fn trace_rejects_span_from_another_trace() {
        let foreign_trace =
            TraceId::new("foreign").expect("valid trace ID");

        let span = Span::builder(
            foreign_trace,
            span_id("span-1"),
            "execution",
            SpanKind::Execution,
            timestamp(100, 0),
        )
        .build()
        .expect("valid span");

        let result = Trace::builder(
            trace_id(),
            "program",
            timestamp(100, 0),
        )
        .span(span);

        assert_eq!(
            result,
            Err(TraceError::SpanBelongsToDifferentTrace)
        );
    }

    #[test]
    fn trace_accepts_matching_span() {
        let id = trace_id();

        let span = Span::builder(
            id.clone(),
            span_id("root"),
            "program",
            SpanKind::QuantumProgram,
            timestamp(100, 0),
        )
        .status(SpanStatus::Ok)
        .ended_at(timestamp(101, 0))
        .build()
        .expect("valid span");

        let trace = Trace::builder(
            id,
            "program",
            timestamp(100, 0),
        )
        .root_span(span.span_id().clone())
        .status(TraceStatus::Ok)
        .ended_at(timestamp(101, 0))
        .span(span)
        .expect("span belongs to trace")
        .build()
        .expect("valid trace");

        assert_eq!(trace.spans().len(), 1);
        assert!(trace.root_span_id().is_some());
    }

    #[test]
    fn root_span_must_not_have_parent() {
        let id = trace_id();

        let span = Span::builder(
            id.clone(),
            span_id("root"),
            "program",
            SpanKind::QuantumProgram,
            timestamp(100, 0),
        )
        .parent_span(span_id("parent"))
        .status(SpanStatus::Ok)
        .ended_at(timestamp(101, 0))
        .build()
        .expect("span itself is structurally valid");

        let result = Trace::builder(
            id,
            "program",
            timestamp(100, 0),
        )
        .root_span(span.span_id().clone())
        .status(TraceStatus::Ok)
        .ended_at(timestamp(101, 0))
        .span(span)
        .expect("span belongs to trace")
        .build();

        assert_eq!(result, Err(TraceError::RootSpanHasParent));
    }

    #[test]
    fn duplicate_identical_span_is_idempotent() {
        let id = trace_id();

        let span = Span::builder(
            id.clone(),
            span_id("span-1"),
            "execution",
            SpanKind::Execution,
            timestamp(100, 0),
        )
        .build()
        .expect("valid span");

        let trace = Trace::builder(
            id,
            "program",
            timestamp(100, 0),
        )
        .span(span.clone())
        .expect("first span")
        .span(span)
        .expect("identical duplicate")
        .build()
        .expect("valid trace");

        assert_eq!(trace.spans().len(), 1);
    }

    #[test]
    fn conflicting_duplicate_span_is_rejected() {
        let id = trace_id();

        let first = Span::builder(
            id.clone(),
            span_id("span-1"),
            "execution",
            SpanKind::Execution,
            timestamp(100, 0),
        )
        .build()
        .expect("valid span");

        let second = Span::builder(
            id.clone(),
            span_id("span-1"),
            "different",
            SpanKind::Recovery,
            timestamp(100, 0),
        )
        .build()
        .expect("valid span");

        let result = Trace::builder(
            id,
            "program",
            timestamp(100, 0),
        )
        .span(first)
        .expect("first span")
        .span(second);

        assert_eq!(result, Err(TraceError::DuplicateSpanId));
    }

    #[test]
    fn attributes_are_deterministically_ordered() {
        let span = Span::builder(
            trace_id(),
            span_id("span-1"),
            "execution",
            SpanKind::Execution,
            timestamp(100, 0),
        )
        .attribute("z", 1_u64)
        .expect("valid attribute")
        .attribute("a", 2_u64)
        .expect("valid attribute")
        .build()
        .expect("valid span");

        let keys = span.attributes().keys().collect::<Vec<_>>();

        assert_eq!(keys, vec![&"a".to_owned(), &"z".to_owned()]);
    }

    #[test]
    fn attribute_keys_reject_control_characters() {
        let result = Span::builder(
            trace_id(),
            span_id("span-1"),
            "execution",
            SpanKind::Execution,
            timestamp(100, 0),
        )
        .attribute("bad\nkey", true);

        assert_eq!(
            result,
            Err(TraceAttributeError::ControlCharacter)
        );
    }

    #[test]
    fn logical_and_physical_qubits_use_canonical_ir_types() {
        let logical = QubitId::new(1);
        let physical = PhysicalQubitId::new(2);

        let span = Span::builder(
            trace_id(),
            span_id("span-1"),
            "execution",
            SpanKind::Execution,
            timestamp(100, 0),
        )
        .logical_qubit(logical)
        .physical_qubit(physical)
        .build()
        .expect("valid span");

        assert_eq!(span.resources().len(), 2);

        assert!(matches!(
            span.resources()[0],
            TraceResource::LogicalQubit(_)
        ));

        assert!(matches!(
            span.resources()[1],
            TraceResource::PhysicalQubit(_)
        ));
    }

    #[test]
    fn generic_resource_rejects_empty_identifier() {
        let result = Span::builder(
            trace_id(),
            span_id("span-1"),
            "execution",
            SpanKind::Execution,
            timestamp(100, 0),
        )
        .resource("");

        assert_eq!(
            result,
            Err(TraceResourceError::EmptyResource)
        );
    }

    #[test]
    fn linked_span_can_reference_another_trace() {
        let source_trace =
            TraceId::new("source").expect("valid trace ID");

        let source_span =
            SpanId::new("source-span").expect("valid span ID");

        let link = SpanLink::new(
            source_trace.clone(),
            source_span.clone(),
            SpanLinkRelationship::FollowsFrom,
        );

        assert_eq!(link.trace_id(), &source_trace);
        assert_eq!(link.span_id(), &source_span);
        assert_eq!(
            link.relationship(),
            SpanLinkRelationship::FollowsFrom
        );
    }

    #[test]
    fn event_identity_is_separate_from_span_identity() {
        let event =
            EventId::new("event-1").expect("valid event ID");

        let span = Span::builder(
            trace_id(),
            span_id("span-1"),
            "execution",
            SpanKind::Execution,
            timestamp(100, 0),
        )
        .event(event.clone())
        .build()
        .expect("valid span");

        assert_eq!(span.event_ids(), &[event]);
        assert_ne!(
            span.span_id().as_str(),
            span.event_ids()[0].as_str()
        );
    }

    #[test]
    fn schema_metadata_is_stable() {
        assert_eq!(
            schema_id(),
            "zamani.quantum.resilience.telemetry.trace"
        );

        assert_eq!(schema_version(), 1);
    }

    #[test]
    fn running_trace_does_not_have_end_timestamp() {
        let trace = Trace::builder(
            trace_id(),
            "program",
            timestamp(100, 0),
        )
        .build()
        .expect("valid running trace");

        assert!(trace.is_ended() == false);
        assert!(trace.ended_at().is_none());
    }

    #[test]
    fn completed_trace_has_duration() {
        let trace = Trace::builder(
            trace_id(),
            "program",
            timestamp(100, 0),
        )
        .status(TraceStatus::Ok)
        .ended_at(timestamp(105, 0))
        .build()
        .expect("valid trace");

        assert_eq!(trace.duration_nanos(), Some(5_000_000_000));
    }
}