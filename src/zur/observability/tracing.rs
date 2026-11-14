
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Runtime — Tracing Integration
//!
//! Path: `src/zur/observability/tracing.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! ## Purpose
//!
//! Provides the ZUR-facing tracing API for runtime operations, execution
//! sessions, services and provider integrations.
//!
//! This module adapts the canonical distributed tracing implementation.
//! It does not implement a second tracing engine.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - ZUR tracing facade and public runtime-facing API.
//! - Association between RuntimeOperationId and distributed spans.
//! - ZUR-specific span metadata.
//! - Adaptation of tracing errors to ZurError.
//!
//! It does not own:
//!
//! - Distributed trace or span identity allocation.
//! - General-purpose trace and span storage.
//! - Span sampling implementation.
//! - Span exporter implementation.
//! - Transport protocols or network I/O.
//! - Execution lifecycle state.
//! - Task scheduling, placement or recovery.
//! - Metrics aggregation.
//! - Structured event storage.
//! - Compiler profiling or JIT hotness decisions.
//! - Authentication or authorization.
//! - Source-language observability semantics.
//!
//! ## Canonical authorities
//!
//! Distributed tracing:
//!     crate::distributed::observability::tracing
//!
//! Distributed identities:
//!     crate::distributed::ids
//!
//! ZUR operation identities:
//!     crate::zur::ids
//!
//! ZUR operational errors:
//!     crate::zur::error
//!
//! ZUR metrics:
//!     crate::zur::observability::metrics
//!
//! ## Architectural invariants
//!
//! 1. RuntimeOperationId is supplied by the owning runtime operation.
//! 2. Trace and span identities are allocated by distributed tracing.
//! 3. A trace context is correlation metadata, not authenticated identity.
//! 4. A span records observations; it does not execute or control work.
//! 5. Sampling never determines whether computation is executed.
//! 6. Export failure does not retroactively change computation outcome.
//! 7. Metrics and tracing remain separate observability responsibilities.
//! 8. Domain-specific metadata remains open-ended.
//! 9. No global mutable tracer is created.
//! 10. All external exporter calls remain owned by the canonical tracer.
//!
//! ## Security
//!
//! Trace attributes must contain operational metadata only.
//!
//! Do not record credentials, authentication tokens, private keys,
//! secrets, private execution payloads or sensitive user information.
//!
//! Trace context is not proof of identity, authorization, execution,
//! resource ownership or provider trust.
//!
//! ## Scalability
//!
//! This adapter introduces no fixed maximum for:
//!
//! - operations;
//! - traces;
//! - spans;
//! - providers;
//! - services;
//! - execution domains;
//! - attributes;
//! - exporters.
//!
//! Actual memory, processing, retention and export limits are determined
//! by available resources and explicitly configured underlying providers.
//!
//! No finite machine promises literal infinite computation.
//!
//! ## Integration
//!
//! - Runtime execution supplies RuntimeOperationId.
//! - Runtime sessions supply parent trace context where appropriate.
//! - Runtime services create child spans.
//! - Domain providers create spans around actual provider operations.
//! - Distributed execution propagates the canonical distributed context.
//! - Metrics consume aggregate measurements separately.
//! - Structured events retain their existing event ownership.

use core::fmt;
use std::collections::BTreeMap;
use std::sync::Arc;

use crate::distributed::observability::tracing::{
    self as distributed_tracing,
    AlwaysRecord,
    CompletedSpan,
    ExportFailure,
    ExportResult,
    PropagateOnly,
    SamplingDecision,
    SpanExporter,
    SpanSampler,
    SpanStart,
    SpanStatus,
    TraceAttributes,
    TraceContext,
    TraceError,
    TraceErrorKind,
    TraceValue,
    Tracer,
};

use crate::zur::error::{
    ErrorKind,
    ZurError,
    ZurResult,
};

use crate::zur::ids::RuntimeOperationId;

// ============================================================================
// PUBLIC RE-EXPORTS
// ============================================================================

/// Canonical distributed trace context.
///
/// ZUR deliberately reuses the distributed tracing context rather than
/// defining a competing trace-context representation.
pub use crate::distributed::observability::tracing::{
    TraceFlags,
};

/// Canonical completed distributed span.
pub use crate::distributed::observability::tracing::{
    CompletedSpan as DistributedCompletedSpan,
};

/// Canonical distributed trace context.
pub use crate::distributed::observability::tracing::{
    TraceContext as DistributedTraceContext,
};

/// Canonical distributed tracing attribute collection.
pub use crate::distributed::observability::tracing::{
    TraceAttributes as DistributedTraceAttributes,
};

/// Canonical distributed tracing attribute value.
pub use crate::distributed::observability::tracing::{
    TraceValue as DistributedTraceValue,
};

/// Canonical span status.
pub use crate::distributed::observability::tracing::{
    SpanStatus as DistributedSpanStatus,
};

/// Stable attribute name containing the ZUR runtime operation identity.
pub const RUNTIME_OPERATION_ATTRIBUTE: &str =
    "zamani.runtime.operation.id";

/// ZUR tracing result type.
pub type ZurTracingResult<T> = ZurResult<T>;

// ============================================================================
// ERROR ADAPTATION
// ============================================================================

/// Convert a canonical distributed tracing error into the ZUR error model.
///
/// Exporter error details are represented only through the canonical
/// tracing error's stable, non-sensitive description.
fn tracing_error(error: TraceError) -> ZurError {
    let kind = match error.kind() {
        TraceErrorKind::InvalidContext
        | TraceErrorKind::InvalidMetadata => ErrorKind::Validation,

        TraceErrorKind::IdentityAllocation
        | TraceErrorKind::ClockUnavailable => {
            ErrorKind::ResourceUnavailable
        }

        TraceErrorKind::PropagationBudgetExceeded => {
            ErrorKind::BudgetExhausted
        }

        TraceErrorKind::Serialization => ErrorKind::Serialization,

        TraceErrorKind::ExporterRegistryUnavailable => {
            ErrorKind::Concurrency
        }

        TraceErrorKind::ExportFailure
        | TraceErrorKind::SamplingFailure => ErrorKind::Provider,

        TraceErrorKind::SpanAlreadyEnded => ErrorKind::Lifecycle,
    };

    ZurError::with_code(
        kind,
        "zur.observability.tracing.failure",
        error.to_string(),
    )
    .expect("the static ZUR tracing error code is valid")
    .with_component("zur.observability.tracing")
}

// ============================================================================
// TRACER
// ============================================================================

/// ZUR-facing adapter over the canonical distributed tracer.
///
/// Clones share the underlying distributed tracer, identity allocator,
/// sampler and exporter registry.
///
/// Construct this adapter by injecting an already configured canonical
/// distributed tracer. This prevents ZUR from creating a second tracing
/// infrastructure or silently choosing an identity-allocation policy.
#[derive(Clone)]
pub struct ZurTracer {
    inner: Tracer,
}

impl fmt::Debug for ZurTracer {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ZurTracer")
            .field("exporter_count", &self.inner.exporter_count())
            .finish()
    }
}

impl ZurTracer {
    /// Wrap an explicitly configured canonical distributed tracer.
    ///
    /// The supplied tracer determines identity allocation, sampling and
    /// exporter behavior.
    #[must_use]
    pub fn from_distributed(tracer: Tracer) -> Self {
        Self { inner: tracer }
    }

    /// Return a reference to the canonical distributed tracer.
    ///
    /// This is useful for integrations that need to use existing
    /// distributed tracing and transport-propagation APIs.
    #[must_use]
    pub fn as_distributed(&self) -> &Tracer {
        &self.inner
    }

    /// Register an exporter through the canonical distributed tracer.
    ///
    /// Exporter implementations remain responsible for their own
    /// buffering, backpressure, persistence and shutdown behavior.
    pub fn register_exporter(
        &self,
        exporter: Arc<dyn SpanExporter>,
    ) -> ZurTracingResult<()> {
        self.inner
            .register_exporter(exporter)
            .map_err(tracing_error)
    }

    /// Return the number of registered exporters.
    pub fn exporter_count(&self) -> ZurTracingResult<usize> {
        self.inner
            .exporter_count()
            .map_err(tracing_error)
    }

    /// Start a root span associated with a ZUR runtime operation.
    ///
    /// The operation identity is supplied by the runtime operation owner.
    /// This method does not allocate or invent an operation identity.
    pub fn start_root(
        &self,
        operation_id: RuntimeOperationId,
        name: impl Into<String>,
    ) -> ZurTracingResult<ZurSpan> {
        let span = self
            .inner
            .start_root(name)
            .map_err(tracing_error)?;

        Ok(ZurSpan::new(span, operation_id))
    }

    /// Start a child span associated with a supplied runtime operation.
    ///
    /// The child inherits the distributed trace identity and parent-span
    /// relationship from the supplied context.
    ///
    /// The caller supplies the operation identity because the child may
    /// represent a different runtime operation.
    pub fn start_child(
        &self,
        operation_id: RuntimeOperationId,
        name: impl Into<String>,
        parent: &ZurTraceContext,
    ) -> ZurTracingResult<ZurSpan> {
        let span = self
            .inner
            .start_child(name, parent.distributed_context())
            .map_err(tracing_error)?;

        Ok(ZurSpan::new(span, operation_id))
    }

    /// Start a child span while retaining its parent's runtime operation.
    ///
    /// This is useful when a runtime operation creates nested service or
    /// provider spans without changing the operation being observed.
    pub fn start_child_in_operation(
        &self,
        name: impl Into<String>,
        parent: &ZurTraceContext,
    ) -> ZurTracingResult<ZurSpan> {
        self.start_child(
            parent.operation_id().clone(),
            name,
            parent,
        )
    }
}

// ============================================================================
// ZUR TRACE CONTEXT
// ============================================================================

/// ZUR correlation context.
///
/// This combines the canonical distributed trace context with the
/// ZUR-owned runtime operation identity.
///
/// It does not claim that the operation still exists, is running,
/// succeeded, or is authorized.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ZurTraceContext {
    distributed: TraceContext,
    operation_id: RuntimeOperationId,
}

impl ZurTraceContext {
    /// Construct a ZUR trace context from an existing distributed context
    /// and a canonical ZUR operation identity.
    ///
    /// The distributed context is structurally validated before use.
    pub fn new(
        distributed: TraceContext,
        operation_id: RuntimeOperationId,
    ) -> ZurTracingResult<Self> {
        distributed
            .validate()
            .map_err(tracing_error)?;

        Ok(Self {
            distributed,
            operation_id,
        })
    }

    /// Return the canonical distributed trace context.
    #[must_use]
    pub fn distributed_context(&self) -> &TraceContext {
        &self.distributed
    }

    /// Return the ZUR runtime operation identity.
    #[must_use]
    pub fn operation_id(&self) -> &RuntimeOperationId {
        &self.operation_id
    }
}

// ============================================================================
// SPAN
// ============================================================================

/// A ZUR span wrapping a canonical distributed span.
///
/// The span records runtime operation correlation metadata and delegates
/// span lifecycle, timing, sampling and export to the distributed tracer.
///
/// Completion consumes the span. This prevents a caller from successfully
/// completing the same span more than once through this API.
#[derive(Debug)]
pub struct ZurSpan {
    inner: SpanStart,
    operation_id: RuntimeOperationId,
}

impl ZurSpan {
    fn new(
        inner: SpanStart,
        operation_id: RuntimeOperationId,
    ) -> Self {
        Self {
            inner,
            operation_id,
        }
    }

    /// Return the ZUR and distributed correlation context.
    #[must_use]
    pub fn context(&self) -> ZurTraceContext {
        ZurTraceContext {
            distributed: self.inner.context().clone(),
            operation_id: self.operation_id.clone(),
        }
    }

    /// Return the canonical distributed trace context.
    #[must_use]
    pub fn distributed_context(&self) -> &TraceContext {
        self.inner.context()
    }

    /// Return the ZUR runtime operation identity.
    #[must_use]
    pub fn operation_id(&self) -> &RuntimeOperationId {
        &self.operation_id
    }

    /// Return the sampling decision made by the canonical tracer.
    ///
    /// Sampling is an observability decision. It does not determine
    /// whether the underlying computation is executed.
    #[must_use]
    pub fn sampling_decision(&self) -> SamplingDecision {
        self.inner.sampling_decision()
    }

    /// Complete the span with a status and structured attributes.
    ///
    /// The runtime-operation attribute is inserted automatically.
    /// Callers cannot replace that reserved attribute.
    ///
    /// A successful return means the canonical tracer completed its
    /// configured span operation. It does not prove that the observed
    /// computation succeeded or that telemetry was durably persisted.
    pub fn end(
        self,
        status: SpanStatus,
        mut attributes: TraceAttributes,
    ) -> ZurTracingResult<Option<CompletedSpan>> {
        if attributes.contains_key(RUNTIME_OPERATION_ATTRIBUTE) {
            return Err(
                ZurError::invalid_input(
                    "the runtime operation tracing attribute is reserved",
                )
                .with_component("zur.observability.tracing"),
            );
        }

        distributed_tracing::insert_attribute(
            &mut attributes,
            RUNTIME_OPERATION_ATTRIBUTE,
            TraceValue::Text(self.operation_id.to_string()),
        )
        .map_err(tracing_error)?;

        self.inner
            .end(status, attributes)
            .map_err(tracing_error)
    }

    /// Complete the span with successful operation status.
    ///
    /// This describes the span's supplied status. It does not independently
    /// inspect or certify the runtime operation's actual result.
    pub fn end_ok(
        self,
    ) -> ZurTracingResult<Option<CompletedSpan>> {
        self.end(SpanStatus::Ok, BTreeMap::new())
    }

    /// Complete the span with error status.
    ///
    /// Arbitrary error text is deliberately not captured automatically.
    /// Use a stable, non-sensitive error code as an explicit attribute
    /// when the caller's privacy policy permits it.
    pub fn end_error(
        self,
    ) -> ZurTracingResult<Option<CompletedSpan>> {
        self.end(SpanStatus::Error, BTreeMap::new())
    }
}

// ============================================================================
// CONTEXT VALIDATION AND ATTRIBUTE HELPERS
// ============================================================================

/// Insert a validated tracing attribute.
///
/// Attribute names use the canonical distributed tracing validation.
/// The runtime-operation attribute is reserved and cannot be overwritten
/// by callers.
pub fn insert_attribute(
    attributes: &mut TraceAttributes,
    name: impl Into<String>,
    value: TraceValue,
) -> ZurTracingResult<()> {
    let name = name.into();

    if name == RUNTIME_OPERATION_ATTRIBUTE {
        return Err(
            ZurError::invalid_input(
                "the runtime operation tracing attribute is reserved",
            )
            .with_component("zur.observability.tracing"),
        );
    }

    distributed_tracing::insert_attribute(
        attributes,
        name,
        value,
    )
    .map_err(tracing_error)
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::distributed::ids::IdAllocator;
    use crate::zur::ids::IdNamespace;

    use std::sync::Arc;

    fn operation_id(sequence: u8) -> RuntimeOperationId {
        RuntimeOperationId::new(
            IdNamespace::new("zamani.runtime.operation").unwrap(),
            vec![sequence],
        )
        .unwrap()
    }

    fn tracer() -> ZurTracer {
        let allocator = IdAllocator::with_namespace(0x5a_55).unwrap();

        let distributed = Tracer::new(
            allocator,
            Arc::new(AlwaysRecord),
        );

        ZurTracer::from_distributed(distributed)
    }

    #[test]
    fn creates_root_span_with_runtime_operation_identity() {
        let tracer = tracer();
        let operation = operation_id(1);

        let span = tracer
            .start_root(
                operation.clone(),
                "zamani.runtime.execution",
            )
            .unwrap();

        assert_eq!(span.operation_id(), &operation);
        assert!(span.distributed_context().parent_span_id().is_none());

        let completed = span.end_ok().unwrap().unwrap();

        assert_eq!(completed.status(), SpanStatus::Ok);

        assert_eq!(
            completed
                .attributes()
                .get(RUNTIME_OPERATION_ATTRIBUTE),
            Some(&TraceValue::Text(operation.to_string())),
        );
    }

    #[test]
    fn child_span_inherits_distributed_trace_identity() {
        let tracer = tracer();

        let parent = tracer
            .start_root(
                operation_id(1),
                "zamani.runtime.execution",
            )
            .unwrap();

        let parent_context = parent.context();

        let child = tracer
            .start_child_in_operation(
                "zamani.runtime.provider",
                &parent_context,
            )
            .unwrap();

        assert_eq!(
            child.distributed_context().trace_id(),
            parent_context.distributed_context().trace_id(),
        );

        assert_eq!(
            child.distributed_context().parent_span_id(),
            Some(parent_context.distributed_context().span_id()),
        );

        assert_eq!(
            child.operation_id(),
            parent_context.operation_id(),
        );
    }

    #[test]
    fn child_can_be_associated_with_another_runtime_operation() {
        let tracer = tracer();

        let parent = tracer
            .start_root(
                operation_id(1),
                "zamani.runtime.execution",
            )
            .unwrap();

        let parent_context = parent.context();
        let child_operation = operation_id(2);

        let child = tracer
            .start_child(
                child_operation.clone(),
                "zamani.runtime.suboperation",
                &parent_context,
            )
            .unwrap();

        assert_eq!(child.operation_id(), &child_operation);
        assert_ne!(
            child.operation_id(),
            parent_context.operation_id(),
        );
    }

    #[test]
    fn reserved_operation_attribute_cannot_be_overwritten() {
        let tracer = tracer();

        let span = tracer
            .start_root(
                operation_id(1),
                "zamani.runtime.execution",
            )
            .unwrap();

        let mut attributes = TraceAttributes::new();

        attributes.insert(
            RUNTIME_OPERATION_ATTRIBUTE.to_owned(),
            TraceValue::Text("forged".to_owned()),
        );

        let error = span
            .end(SpanStatus::Ok, attributes)
            .unwrap_err();

        assert_eq!(error.kind(), ErrorKind::InvalidInput);
    }

    #[test]
    fn caller_attributes_are_preserved() {
        let tracer = tracer();

        let span = tracer
            .start_root(
                operation_id(1),
                "zamani.runtime.execution",
            )
            .unwrap();

        let mut attributes = TraceAttributes::new();

        insert_attribute(
            &mut attributes,
            "zamani.runtime.service",
            TraceValue::Text("execution".to_owned()),
        )
        .unwrap();

        let completed = span
            .end(SpanStatus::Ok, attributes)
            .unwrap()
            .unwrap();

        assert_eq!(
            completed.attributes().get("zamani.runtime.service"),
            Some(&TraceValue::Text("execution".to_owned())),
        );
    }

    #[test]
    fn caller_cannot_insert_reserved_operation_attribute() {
        let mut attributes = TraceAttributes::new();

        let result = insert_attribute(
            &mut attributes,
            RUNTIME_OPERATION_ATTRIBUTE,
            TraceValue::Text("forged".to_owned()),
        );

        assert!(result.is_err());
        assert!(attributes.is_empty());
    }

    #[test]
    fn failed_span_is_not_reported_as_successful() {
        let tracer = tracer();

        let span = tracer
            .start_root(
                operation_id(1),
                "zamani.runtime.execution",
            )
            .unwrap();

        let completed = span.end_error().unwrap().unwrap();

        assert_eq!(completed.status(), SpanStatus::Error);
    }

    #[test]
    fn cloned_tracer_shares_the_underlying_tracer() {
        let tracer = tracer();
        let cloned = tracer.clone();

        assert_eq!(
            tracer.exporter_count().unwrap(),
            cloned.exporter_count().unwrap(),
        );
    }
}
