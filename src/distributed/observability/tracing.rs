
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! Distributed tracing for Zamani.
//!
//! Path: src/distributed/observability/tracing.rs
//! Edition: 2021
//! Minimum Rust version: 1.97
//!
//! # Purpose
//!
//! Provides portable distributed trace and span context, context
//! propagation through the existing transport extension mechanism,
//! injectable span exporters, and explicit span lifecycle management.
//!
//! # Ownership
//!
//! This module owns:
//! - Trace context and parent/child span relationships.
//! - Span creation and completion.
//! - Trace-specific attributes and status.
//! - Sampling policy interfaces.
//! - Span exporter interfaces.
//! - Transport-extension injection and extraction.
//! - Local monotonic duration measurement.
//!
//! It does not own:
//! - Distributed logical identity allocation.
//! - Execution lifecycle state.
//! - Transport protocols or network I/O.
//! - Authentication or authorization.
//! - Task scheduling, placement, routing, or recovery.
//! - Metrics aggregation.
//! - Durable event storage.
//! - Source semantics, AST, IR, or backend execution.
//!
//! # Integration
//!
//! - `distributed::ids` supplies CorrelationId and SpanId.
//! - `distributed::observability::events` supplies observation timestamps.
//! - `distributed::transport::messages` supplies message extensions.
//! - `distributed::execution::lifecycle` supplies authoritative lifecycle
//!   observations to runtime instrumentation adapters.
//! - `distributed::error` remains the canonical distributed error owner.
//!
//! # Security
//!
//! Trace context is correlation metadata, not authenticated identity.
//! A received trace context MUST NOT be used for authentication,
//! authorization, tenant isolation, or access-control decisions.
//!
//! Attributes must contain operational metadata only. Credentials,
//! private keys, tokens, private state, and execution payloads must not
//! be recorded.
//!
//! # Scalability
//!
//! No fixed number of traces, spans, participants, tasks, exporters,
//! or attributes is imposed by this module.
//!
//! Actual memory, export throughput, retention, and propagation size
//! are constrained by available resources and optional caller-supplied
//! operational budgets.
//!
//! The ID representation is finite and its allocator reports exhaustion.
//!
//! # Delivery semantics
//!
//! Export is best-effort unless the configured exporter provides stronger
//! guarantees. A successful span completion does not prove durable export.
//! Export failures do not change the computation being observed.

use core::fmt;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};
use std::time::Instant;

use crate::distributed::ids::{
    CorrelationId,
    IdAllocator,
    IdError,
    SpanId,
};

use crate::distributed::observability::events::{
    EventTimestamp,
};

use crate::distributed::transport::messages::{
    MessageExtensions,
};

/// Namespaced transport extension key used to propagate trace context.
pub const TRACE_CONTEXT_EXTENSION: &str = "zamani.tracing.context";

/// Result type for tracing operations.
pub type TraceResult<T> = Result<T, TraceError>;

/// Machine-readable tracing error category.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum TraceErrorKind {
    /// A supplied trace or span context is invalid.
    InvalidContext,

    /// A supplied span name or attribute is invalid.
    InvalidMetadata,

    /// The canonical identifier allocator failed.
    IdentityAllocation,

    /// The system wall clock could not provide a timestamp.
    ClockUnavailable,

    /// The configured propagation budget was exceeded.
    PropagationBudgetExceeded,

    /// Trace-context serialization or deserialization failed.
    Serialization,

    /// The exporter registry could not be accessed.
    ExporterRegistryUnavailable,

    /// One or more exporters failed.
    ExportFailure,

    /// The span has already been completed.
    SpanAlreadyEnded,

    /// The sampling policy failed.
    SamplingFailure,
}

impl TraceErrorKind {
    /// Return a stable machine-readable error code.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InvalidContext => "tracing.invalid_context",
            Self::InvalidMetadata => "tracing.invalid_metadata",
            Self::IdentityAllocation => "tracing.identity_allocation",
            Self::ClockUnavailable => "tracing.clock_unavailable",
            Self::PropagationBudgetExceeded => {
                "tracing.propagation_budget_exceeded"
            }
            Self::Serialization => "tracing.serialization",
            Self::ExporterRegistryUnavailable => {
                "tracing.exporter_registry_unavailable"
            }
            Self::ExportFailure => "tracing.export_failure",
            Self::SpanAlreadyEnded => "tracing.span_already_ended",
            Self::SamplingFailure => "tracing.sampling_failure",
        }
    }
}

/// Error returned by distributed tracing operations.
///
/// External exporter error text is deliberately not retained. Exporters
/// should return stable error codes rather than exposing credentials,
/// private endpoints, or sensitive implementation details.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TraceError {
    kind: TraceErrorKind,
    code: Option<String>,
}

impl TraceError {
    /// Construct an error without additional provider-specific information.
    #[must_use]
    pub const fn new(kind: TraceErrorKind) -> Self {
        Self {
            kind,
            code: None,
        }
    }

    /// Construct an error with a non-sensitive machine-readable code.
    pub fn with_code(
        kind: TraceErrorKind,
        code: impl Into<String>,
    ) -> TraceResult<Self> {
        let code = code.into();

        if !valid_identifier(&code) {
            return Err(Self::new(TraceErrorKind::InvalidMetadata));
        }

        Ok(Self {
            kind,
            code: Some(code),
        })
    }

    /// Return the error category.
    #[must_use]
    pub const fn kind(&self) -> TraceErrorKind {
        self.kind
    }

    /// Return the optional stable provider error code.
    #[must_use]
    pub fn code(&self) -> Option<&str> {
        self.code.as_deref()
    }
}

impl fmt::Display for TraceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.kind.as_str())?;

        if let Some(code) = &self.code {
            write!(formatter, ": {code}")?;
        }

        Ok(())
    }
}

impl std::error::Error for TraceError {}

impl From<IdError> for TraceError {
    fn from(_: IdError) -> Self {
        Self::new(TraceErrorKind::IdentityAllocation)
    }
}

impl From<serde_json::Error> for TraceError {
    fn from(_: serde_json::Error) -> Self {
        Self::new(TraceErrorKind::Serialization)
    }
}

fn valid_identifier(value: &str) -> bool {
    !value.trim().is_empty()
        && !value.chars().any(char::is_control)
}

// =============================================================================
// TRACE CONTEXT
// =============================================================================

/// Flags describing trace propagation behavior.
///
/// Sampling is an instrumentation decision. It must never determine
/// whether a distributed computation is executed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraceFlags {
    sampled: bool,
}

impl TraceFlags {
    /// Construct trace flags.
    #[must_use]
    pub const fn new(sampled: bool) -> Self {
        Self { sampled }
    }

    /// Return whether this trace was sampled.
    #[must_use]
    pub const fn sampled(self) -> bool {
        self.sampled
    }
}

/// Portable identity and parent relationship for one span.
///
/// The trace identity reuses the canonical distributed CorrelationId.
/// Span identity belongs to the tracing subsystem and uses the canonical
/// SpanId type declared in distributed::ids.
///
/// A context is correlation metadata only. It is not proof of trust.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraceContext {
    trace_id: CorrelationId,
    span_id: SpanId,
    parent_span_id: Option<SpanId>,
    flags: TraceFlags,
}

impl TraceContext {
    /// Construct a trace context.
    #[must_use]
    pub const fn new(
        trace_id: CorrelationId,
        span_id: SpanId,
        parent_span_id: Option<SpanId>,
        flags: TraceFlags,
    ) -> Self {
        Self {
            trace_id,
            span_id,
            parent_span_id,
            flags,
        }
    }

    /// Return the canonical trace identity.
    #[must_use]
    pub const fn trace_id(&self) -> CorrelationId {
        self.trace_id
    }

    /// Return this span's identity.
    #[must_use]
    pub const fn span_id(&self) -> SpanId {
        self.span_id
    }

    /// Return the parent span identity, if present.
    #[must_use]
    pub const fn parent_span_id(&self) -> Option<SpanId> {
        self.parent_span_id
    }

    /// Return the trace flags.
    #[must_use]
    pub const fn flags(&self) -> TraceFlags {
        self.flags
    }

    /// Validate the structural context.
    ///
    /// This checks representation consistency only. It does not prove
    /// that the trace or parent span exists.
    pub fn validate(&self) -> TraceResult<()> {
        if self.parent_span_id == Some(self.span_id) {
            return Err(TraceError::new(TraceErrorKind::InvalidContext));
        }

        Ok(())
    }

    /// Encode the context for a transport extension.
    ///
    /// The caller must enforce its configured transport/message byte budget.
    pub fn encode(&self) -> TraceResult<Vec<u8>> {
        self.validate()?;
        Ok(serde_json::to_vec(self)?)
    }

    /// Decode a context from untrusted transport metadata.
    ///
    /// `maximum_bytes` is supplied by the caller's operational policy.
    /// It is not a universal language or protocol ceiling.
    pub fn decode(
        bytes: &[u8],
        maximum_bytes: Option<usize>,
    ) -> TraceResult<Self> {
        if maximum_bytes.is_some_and(|limit| bytes.len() > limit) {
            return Err(TraceError::new(
                TraceErrorKind::PropagationBudgetExceeded,
            ));
        }

        let context: Self = serde_json::from_slice(bytes)?;
        context.validate()?;
        Ok(context)
    }
}

// =============================================================================
// TRANSPORT PROPAGATION
// =============================================================================

/// Insert a trace context into existing transport extensions.
///
/// This function does not create a message, send a message, or authenticate
/// a remote participant. The caller supplies the existing message-extension
/// collection before constructing its MessageEnvelope.
pub fn inject_context(
    extensions: &mut MessageExtensions,
    context: &TraceContext,
) -> TraceResult<()> {
    context.validate()?;

    extensions.insert(
        TRACE_CONTEXT_EXTENSION.to_owned(),
        context.encode()?,
    );

    Ok(())
}

/// Extract trace context from an existing validated message's extensions.
///
/// The decoded context remains untrusted correlation metadata. The caller
/// must independently authenticate the peer and authorize the operation.
pub fn extract_context(
    extensions: &MessageExtensions,
    maximum_bytes: Option<usize>,
) -> TraceResult<Option<TraceContext>> {
    match extensions.get(TRACE_CONTEXT_EXTENSION) {
        Some(bytes) => Ok(Some(TraceContext::decode(
            bytes,
            maximum_bytes,
        )?)),
        None => Ok(None),
    }
}

// =============================================================================
// SPAN METADATA
// =============================================================================

/// Classification of a span's operational status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum SpanStatus {
    /// The operation is still in progress.
    Unset,

    /// The operation completed successfully.
    Ok,

    /// The operation failed.
    Error,
}

impl SpanStatus {
    /// Return a stable status identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unset => "unset",
            Self::Ok => "ok",
            Self::Error => "error",
        }
    }
}

/// Safe, structured tracing attribute values.
///
/// These values are intended for operational metadata. Do not insert
/// credentials, private state, source secrets, or execution payloads.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum TraceValue {
    /// Boolean observation.
    Boolean(bool),

    /// Signed integer observation.
    Signed(i128),

    /// Unsigned integer observation.
    Unsigned(u128),

    /// Public, non-sensitive descriptive value.
    Text(String),

    /// Stable machine-readable code.
    Code(String),
}

/// Deterministically ordered span attributes.
pub type TraceAttributes = BTreeMap<String, TraceValue>;

/// Immutable exported representation of a completed span.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompletedSpan {
    context: TraceContext,
    name: String,
    started_at: EventTimestamp,
    ended_at: EventTimestamp,
    duration_nanoseconds: u128,
    status: SpanStatus,
    attributes: TraceAttributes,
}

impl CompletedSpan {
    /// Return the trace and span identity.
    #[must_use]
    pub fn context(&self) -> &TraceContext {
        &self.context
    }

    /// Return the span name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return the wall-clock start timestamp.
    #[must_use]
    pub const fn started_at(&self) -> EventTimestamp {
        self.started_at
    }

    /// Return the wall-clock end timestamp.
    #[must_use]
    pub const fn ended_at(&self) -> EventTimestamp {
        self.ended_at
    }

    /// Return the monotonic elapsed duration in nanoseconds.
    ///
    /// This duration is measured locally and must not be compared as a
    /// synchronized clock value across different machines.
    #[must_use]
    pub const fn duration_nanoseconds(&self) -> u128 {
        self.duration_nanoseconds
    }

    /// Return the span status.
    #[must_use]
    pub const fn status(&self) -> SpanStatus {
        self.status
    }

    /// Return the structured attributes.
    #[must_use]
    pub fn attributes(&self) -> &TraceAttributes {
        &self.attributes
    }
}

// =============================================================================
// SAMPLING
// =============================================================================

/// Decision returned by an injected sampling policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SamplingDecision {
    /// Record and export the span.
    Record,

    /// Propagate context without recording this span.
    PropagateOnly,

    /// Do not record this span.
    Drop,
}

/// Injectable sampling policy.
///
/// Implementations must not change program execution, task acceptance,
/// placement, retries, or distributed correctness.
pub trait SpanSampler: Send + Sync + fmt::Debug {
    /// Decide how a new span should be instrumented.
    fn decide(
        &self,
        name: &str,
        parent: Option<&TraceContext>,
    ) -> TraceResult<SamplingDecision>;
}

/// Sampling policy that records every span.
#[derive(Debug, Default)]
pub struct AlwaysRecord;

impl SpanSampler for AlwaysRecord {
    fn decide(
        &self,
        _name: &str,
        _parent: Option<&TraceContext>,
    ) -> TraceResult<SamplingDecision> {
        Ok(SamplingDecision::Record)
    }
}

/// Sampling policy that propagates context without recording spans.
#[derive(Debug, Default)]
pub struct PropagateOnly;

impl SpanSampler for PropagateOnly {
    fn decide(
        &self,
        _name: &str,
        _parent: Option<&TraceContext>,
    ) -> TraceResult<SamplingDecision> {
        Ok(SamplingDecision::PropagateOnly)
    }
}

// =============================================================================
// EXPORT
// =============================================================================

/// Exporter failure with a stable, non-sensitive code.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExportFailure {
    code: String,
}

impl ExportFailure {
    /// Construct an exporter failure.
    pub fn new(code: impl Into<String>) -> TraceResult<Self> {
        let code = code.into();

        if !valid_identifier(&code) {
            return Err(TraceError::new(TraceErrorKind::InvalidMetadata));
        }

        Ok(Self { code })
    }

    /// Return the stable exporter failure code.
    #[must_use]
    pub fn code(&self) -> &str {
        &self.code
    }
}

/// Result returned by a span exporter.
pub type ExportResult = Result<(), ExportFailure>;

/// Injectable destination for completed spans.
///
/// Implementations may export to a local collector, persistent store,
/// telemetry service, or another observability provider.
///
/// Exporters must not block indefinitely. If buffering is required,
/// the exporter owns its queue, backpressure, shutdown, and persistence
/// policy.
pub trait SpanExporter: Send + Sync + fmt::Debug {
    /// Export one completed span.
    fn export(&self, span: &CompletedSpan) -> ExportResult;
}

// =============================================================================
// TRACER
// =============================================================================

/// Shared tracer implementation.
#[derive(Debug)]
struct TracerInner {
    ids: IdAllocator,
    sampler: Arc<dyn SpanSampler>,
    exporters: RwLock<Vec<Arc<dyn SpanExporter>>>,
}

/// Cloneable handle to an explicitly owned tracer.
///
/// Clones share the same ID allocator, sampling policy, and exporter
/// registry. There is no process-global tracer or hidden initialization.
#[derive(Clone, Debug)]
pub struct Tracer {
    inner: Arc<TracerInner>,
}

impl Tracer {
    /// Construct a tracer with an explicit ID allocator and sampling policy.
    ///
    /// An explicit allocator allows the application to control namespace
    /// assignment and coordinate identity policy across its processes.
    #[must_use]
    pub fn new(
        ids: IdAllocator,
        sampler: Arc<dyn SpanSampler>,
    ) -> Self {
        Self {
            inner: Arc::new(TracerInner {
                ids,
                sampler,
                exporters: RwLock::new(Vec::new()),
            }),
        }
    }

    /// Construct a tracer using a fresh allocator namespace.
    ///
    /// The allocator namespace is probabilistically unique. Applications
    /// requiring coordinated identity allocation should use `new`.
    #[must_use]
    pub fn with_fresh_namespace(
        sampler: Arc<dyn SpanSampler>,
    ) -> Self {
        Self::new(IdAllocator::new(), sampler)
    }

    /// Register an exporter.
    ///
    /// Exporters are called outside the registry lock. An exporter may
    /// therefore interact with the registry without holding its lock.
    pub fn register_exporter(
        &self,
        exporter: Arc<dyn SpanExporter>,
    ) -> TraceResult<()> {
        let mut exporters = self.inner.exporters.write().map_err(|_| {
            TraceError::new(TraceErrorKind::ExporterRegistryUnavailable)
        })?;

        exporters.push(exporter);
        Ok(())
    }

    /// Return the current number of registered exporters.
    pub fn exporter_count(&self) -> TraceResult<usize> {
        let exporters = self.inner.exporters.read().map_err(|_| {
            TraceError::new(TraceErrorKind::ExporterRegistryUnavailable)
        })?;

        Ok(exporters.len())
    }

    /// Start a root span with a newly allocated trace identity.
    pub fn start_root(
        &self,
        name: impl Into<String>,
    ) -> TraceResult<SpanStart> {
        self.start_span(name, None)
    }

    /// Start a child span using an existing parent context.
    ///
    /// The parent context is correlation metadata. It is not treated as
    /// an authenticated identity or an authorization grant.
    pub fn start_child(
        &self,
        name: impl Into<String>,
        parent: &TraceContext,
    ) -> TraceResult<SpanStart> {
        parent.validate()?;
        self.start_span(name, Some(parent))
    }

    fn start_span(
        &self,
        name: impl Into<String>,
        parent: Option<&TraceContext>,
    ) -> TraceResult<SpanStart> {
        let name = name.into();

        if !valid_identifier(&name) {
            return Err(TraceError::new(TraceErrorKind::InvalidMetadata));
        }

        let decision = self.inner.sampler.decide(&name, parent)?;

        let trace_id = match parent {
            Some(context) => context.trace_id(),
            None => self.inner.ids.correlation_id()?,
        };

        let span_id = self.inner.ids.span_id()?;

        let flags = TraceFlags::new(matches!(
            decision,
            SamplingDecision::Record
        ));

        let context = TraceContext::new(
            trace_id,
            span_id,
            parent.map(TraceContext::span_id),
            flags,
        );

        let started_at = EventTimestamp::now().map_err(|_| {
            TraceError::new(TraceErrorKind::ClockUnavailable)
        })?;

        let start = Instant::now();

        Ok(SpanStart {
            tracer: self.clone(),
            context,
            name,
            started_at,
            monotonic_start: start,
            decision,
        })
    }

    fn export(&self, span: &CompletedSpan) -> TraceResult<()> {
        let exporters = self.inner.exporters.read().map_err(|_| {
            TraceError::new(TraceErrorKind::ExporterRegistryUnavailable)
        })?;

        // Clone handles while locked, then release the lock before invoking
        // provider-controlled code.
        let snapshot = exporters.clone();
        drop(exporters);

        let mut failures = Vec::new();

        for exporter in snapshot {
            if let Err(error) = exporter.export(span) {
                failures.push(error.code().to_owned());
            }
        }

        if failures.is_empty() {
            Ok(())
        } else {
            Err(TraceError::with_code(
                TraceErrorKind::ExportFailure,
                failures.join(","),
            )?)
        }
    }
}

// =============================================================================
// SPAN LIFECYCLE
// =============================================================================

/// A newly started span.
///
/// A span is completed explicitly through `end`. Dropping an unfinished
/// span does not synchronously call external exporters.
#[derive(Debug)]
pub struct SpanStart {
    tracer: Tracer,
    context: TraceContext,
    name: String,
    started_at: EventTimestamp,
    monotonic_start: Instant,
    decision: SamplingDecision,
}

impl SpanStart {
    /// Return the span context for propagation to child operations.
    #[must_use]
    pub fn context(&self) -> &TraceContext {
        &self.context
    }

    /// Return the span's sampling decision.
    #[must_use]
    pub const fn sampling_decision(&self) -> SamplingDecision {
        self.decision
    }

    /// Complete the span with status and attributes.
    ///
    /// Export failures are returned to the caller. They must not be
    /// interpreted as failure of the computation being observed.
    pub fn end(
        self,
        status: SpanStatus,
        attributes: TraceAttributes,
    ) -> TraceResult<Option<CompletedSpan>> {
        let ended_at = EventTimestamp::now().map_err(|_| {
            TraceError::new(TraceErrorKind::ClockUnavailable)
        })?;

        let duration_nanoseconds =
            self.monotonic_start.elapsed().as_nanos();

        if self.decision != SamplingDecision::Record {
            return Ok(None);
        }

        let completed = CompletedSpan {
            context: self.context,
            name: self.name,
            started_at: self.started_at,
            ended_at,
            duration_nanoseconds,
            status,
            attributes,
        };

        self.tracer.export(&completed)?;
        Ok(Some(completed))
    }

    /// Complete a span successfully without additional attributes.
    pub fn end_ok(self) -> TraceResult<Option<CompletedSpan>> {
        self.end(SpanStatus::Ok, TraceAttributes::new())
    }

    /// Complete a span as failed without recording arbitrary error text.
    ///
    /// Use a stable, non-sensitive error code as an attribute if needed.
    pub fn end_error(self) -> TraceResult<Option<CompletedSpan>> {
        self.end(SpanStatus::Error, TraceAttributes::new())
    }
}

// =============================================================================
// ATTRIBUTE HELPERS
// =============================================================================

/// Insert a validated attribute into a trace attribute collection.
///
/// Attribute names must be qualified, for example
/// `zamani.execution.stage` or `zamani.planning.outcome`.
pub fn insert_attribute(
    attributes: &mut TraceAttributes,
    key: impl Into<String>,
    value: TraceValue,
) -> TraceResult<()> {
    let key = key.into();

    if !valid_identifier(&key)
        || !key.contains('.')
        || key.chars().any(char::is_whitespace)
    {
        return Err(TraceError::new(TraceErrorKind::InvalidMetadata));
    }

    attributes.insert(key, value);
    Ok(())
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[derive(Debug, Default)]
    struct CountingExporter {
        count: AtomicUsize,
    }

    impl SpanExporter for CountingExporter {
        fn export(&self, _span: &CompletedSpan) -> ExportResult {
            self.count.fetch_add(1, Ordering::Relaxed);
            Ok(())
        }
    }

    #[derive(Debug)]
    struct FailingExporter;

    impl SpanExporter for FailingExporter {
        fn export(&self, _span: &CompletedSpan) -> ExportResult {
            Err(ExportFailure::new("collector_unavailable").unwrap())
        }
    }

    fn tracer() -> Tracer {
        Tracer::new(
            IdAllocator::with_namespace(100).unwrap(),
            Arc::new(AlwaysRecord),
        )
    }

    #[test]
    fn creates_and_completes_root_span() {
        let tracer = tracer();
        let exporter = Arc::new(CountingExporter::default());

        tracer
            .register_exporter(exporter.clone())
            .unwrap();

        let span = tracer.start_root("zamani.execution.root").unwrap();

        assert!(span.context().parent_span_id().is_none());

        let completed = span.end_ok().unwrap().unwrap();

        assert_eq!(completed.status(), SpanStatus::Ok);
        assert_eq!(exporter.count.load(Ordering::Relaxed), 1);
    }

    #[test]
    fn child_span_inherits_trace_identity() {
        let tracer = tracer();

        let parent = tracer.start_root("zamani.execution.root").unwrap();
        let parent_context = parent.context().clone();

        let child = tracer
            .start_child("zamani.execution.remote", &parent_context)
            .unwrap();

        assert_eq!(
            child.context().trace_id(),
            parent_context.trace_id()
        );

        assert_eq!(
            child.context().parent_span_id(),
            Some(parent_context.span_id())
        );
    }

    #[test]
    fn context_round_trip() {
        let tracer = tracer();
        let span = tracer.start_root("zamani.root").unwrap();

        let encoded = span.context().encode().unwrap();

        let decoded = TraceContext::decode(&encoded, Some(encoded.len()))
            .unwrap();

        assert_eq!(&decoded, span.context());
    }

    #[test]
    fn propagation_uses_existing_message_extensions() {
        let tracer = tracer();
        let span = tracer.start_root("zamani.transport.request").unwrap();

        let mut extensions = MessageExtensions::new();

        inject_context(&mut extensions, span.context()).unwrap();

        let extracted = extract_context(&extensions, None)
            .unwrap()
            .unwrap();

        assert_eq!(&extracted, span.context());
    }

    #[test]
    fn propagation_budget_is_enforced() {
        let tracer = tracer();
        let span = tracer.start_root("zamani.transport.request").unwrap();

        let mut extensions = MessageExtensions::new();

        inject_context(&mut extensions, span.context()).unwrap();

        let error = extract_context(&extensions, Some(1)).unwrap_err();

        assert_eq!(
            error.kind(),
            TraceErrorKind::PropagationBudgetExceeded
        );
    }

    #[test]
    fn missing_propagation_context_is_not_an_error() {
        let extensions = MessageExtensions::new();

        assert!(
            extract_context(&extensions, None)
                .unwrap()
                .is_none()
        );
    }

    #[test]
    fn exporter_failure_is_reported() {
        let tracer = tracer();

        tracer
            .register_exporter(Arc::new(FailingExporter))
            .unwrap();

        let span = tracer.start_root("zamani.execution.root").unwrap();

        let error = span.end_ok().unwrap_err();

        assert_eq!(error.kind(), TraceErrorKind::ExportFailure);
    }

    #[test]
    fn propagate_only_does_not_export() {
        let tracer = Tracer::new(
            IdAllocator::with_namespace(101).unwrap(),
            Arc::new(PropagateOnly),
        );

        let exporter = Arc::new(CountingExporter::default());

        tracer
            .register_exporter(exporter.clone())
            .unwrap();

        let span = tracer.start_root("zamani.execution.root").unwrap();

        assert_eq!(
            span.sampling_decision(),
            SamplingDecision::PropagateOnly
        );

        assert!(span.end_ok().unwrap().is_none());

        assert_eq!(exporter.count.load(Ordering::Relaxed), 0);
    }

    #[test]
    fn qualified_attributes_are_accepted() {
        let mut attributes = TraceAttributes::new();

        insert_attribute(
            &mut attributes,
            "zamani.execution.stage",
            TraceValue::Code("running".to_owned()),
        )
        .unwrap();

        assert!(attributes.contains_key("zamani.execution.stage"));
    }

    #[test]
    fn unqualified_attributes_are_rejected() {
        let mut attributes = TraceAttributes::new();

        assert!(
            insert_attribute(
                &mut attributes,
                "password",
                TraceValue::Text("not-recorded".to_owned()),
            )
            .is_err()
        );
    }

    #[test]
    fn parent_cannot_be_its_own_span() {
        let id = SpanId::new(1, 1).unwrap();
        let context = TraceContext::new(
            CorrelationId::new(1, 2).unwrap(),
            id,
            Some(id),
            TraceFlags::new(true),
        );

        assert_eq!(
            context.validate().unwrap_err().kind(),
            TraceErrorKind::InvalidContext
        );
    }

    #[test]
    fn exporter_registry_is_shared_between_tracer_clones() {
        let tracer = tracer();
        let clone = tracer.clone();

        tracer
            .register_exporter(Arc::new(CountingExporter::default()))
            .unwrap();

        assert_eq!(clone.exporter_count().unwrap(), 1);
    }
}
