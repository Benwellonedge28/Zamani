
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Distributed Observability
//!
//! Path: src/distributed/observability/mod.rs
//!
//! Language: Rust
//! Edition: 2021
//! Minimum supported Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## 1. PURPOSE
//!
//! This module is the canonical composition root for distributed
//! observability within the Zamani programming language.
//!
//! It organizes the existing event, metrics, and tracing
//! implementations into one public distributed-observability API.
//!
//! Its portability objective is:
//!
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
//!
//! Observability must work across:
//!
//! - one execution context;
//! - multiple threads;
//! - multiple processes;
//! - multiple workers;
//! - multiple logical participants;
//! - multiple physical machines;
//! - embedded and edge systems;
//! - cloud and HPC systems;
//! - heterogeneous accelerators;
//! - classical and quantum execution;
//! - distributed AI and tensor execution;
//! - future computational domains.
//!
//! The physical scale of an execution must not determine the
//! meaning of the program or require changes to its source.
//!
//! ## 2. ARCHITECTURAL OWNERSHIP
//!
//! This composition root owns:
//!
//! - observability module composition;
//! - public observability module boundaries;
//! - deliberate public API re-exports;
//! - observability integration documentation;
//! - module-level safety requirements;
//! - architectural dependency documentation.
//!
//! It does not own:
//!
//! - source-language observability syntax;
//! - semantic effect definitions;
//! - semantic validation;
//! - execution lifecycle state;
//! - task scheduling;
//! - feasibility analysis;
//! - task placement;
//! - partitioning;
//! - communication routing;
//! - distributed membership;
//! - participant health;
//! - retry and recovery decisions;
//! - checkpoint implementation;
//! - physical transport;
//! - authentication or authorization;
//! - backend compilation;
//! - hardware allocation;
//! - quantum error correction;
//! - ZUIR, ZCIR, quantum::ir, or HDLIR;
//! - another metrics registry;
//! - another tracing implementation;
//! - another distributed event journal.
//!
//! Those responsibilities remain with their existing repository
//! authorities.
//!
//! ## 3. CHILD MODULE RESPONSIBILITIES
//!
//! ### events
//!
//! `events.rs` is the sole owner of structured distributed
//! observations.
//!
//! It provides:
//!
//! - DistributedEvent;
//! - EventCategory;
//! - EventName;
//! - EventSubject;
//! - EventOutcome;
//! - EventTimestamp;
//! - EventContext;
//! - EventAttributes;
//! - EventSink;
//! - CompositeEventSink;
//! - EventError.
//!
//! Events describe facts already established by their owning
//! subsystems. They must never initiate or change execution.
//!
//! ### metrics
//!
//! `metrics.rs` is the sole owner of generic distributed metrics.
//!
//! It provides:
//!
//! - counters;
//! - gauges;
//! - histograms;
//! - metric descriptors;
//! - metric names and units;
//! - validated metric labels;
//! - configurable series budgets;
//! - thread-safe metric registration and updates;
//! - deterministic metric snapshots.
//!
//! It does not make scheduling, admission, or resource-allocation
//! decisions based on collected measurements.
//!
//! ### tracing
//!
//! `tracing.rs` is the sole owner of distributed trace and span
//! correlation.
//!
//! It provides:
//!
//! - trace identities;
//! - span identities;
//! - parent-child span relationships;
//! - trace context propagation;
//! - transport-extension integration;
//! - sampling policies;
//! - span exporters;
//! - span completion and duration measurements.
//!
//! Trace context is correlation metadata, not authentication,
//! authorization, or proof of execution.
//!
//! ## 4. DEPENDENCY CONTRACT
//!
//! The dependency direction is:
//!
//! distributed::ids
//!       |
//!       +--------------------------+
//!       |                          |
//!       v                          v
//! observability::events       observability::tracing
//!       |                          |
//!       |                          v
//!       |                   transport::messages
//!       |
//!       +----------+---------------+
//!                  |
//!                  v
//!          Observability consumers
//!                  |
//!        +---------+---------+
//!        |         |         |
//!        v         v         v
//!    execution  membership resilience
//!    lifecycle    health    recovery
//!
//! The above consumers supply authoritative observations.
//! They do not transfer ownership of their state to this module.
//!
//! Metrics can consume appropriately classified observations,
//! but must not duplicate the authoritative lifecycle, membership,
//! or resilience state.
//!
//! ## 5. INTEGRATION CONTRACTS
//!
//! ### Distributed identities
//!
//! `crate::distributed::ids` owns logical distributed identifiers.
//!
//! Events use its existing:
//!
//! - CorrelationId;
//! - TaskId;
//! - ParticipantId;
//! - PlanId;
//! - OperationId;
//! - MessageId;
//! - ExecutionContextId.
//!
//! Tracing uses the existing CorrelationId and SpanId allocation
//! mechanisms.
//!
//! This module must not create a competing identity allocator.
//!
//! ### Execution
//!
//! `crate::distributed::execution` remains authoritative for
//! execution and task lifecycle.
//!
//! Instrumentation consumers may observe:
//!
//! - task acceptance;
//! - task execution;
//! - task completion;
//! - task failure;
//! - cancellation;
//! - execution recovery;
//! - result publication.
//!
//! Observability must not independently transition task states.
//!
//! ### Planning
//!
//! `crate::distributed::planning` remains authoritative for:
//!
//! - feasibility;
//! - partitioning;
//! - placement;
//! - scheduling;
//! - routing.
//!
//! Observability may record:
//!
//! - feasibility outcomes;
//! - rejection reasons;
//! - selected placement decisions;
//! - scheduling delays;
//! - routing outcomes;
//! - documented planning trade-offs.
//!
//! It must not select or modify the plan.
//!
//! ### Membership
//!
//! `crate::distributed::membership` remains authoritative for
//! participant discovery, membership and health.
//!
//! Observability records established membership and health
//! transitions without maintaining a second registry.
//!
//! ### Resilience
//!
//! `crate::distributed::resilience` remains authoritative for:
//!
//! - retry decisions;
//! - idempotency;
//! - checkpoints;
//! - recovery;
//! - reconciliation.
//!
//! Observability records the decisions and their outcomes.
//!
//! It must never automatically retry or recover work.
//!
//! ### Transport
//!
//! `crate::distributed::transport::messages` owns the message
//! extension mechanism.
//!
//! `tracing.rs` propagates its existing trace context through
//! MessageExtensions.
//!
//! This composition root must not introduce another message
//! envelope, network protocol, or serialization implementation.
//!
//! ### Semantic and IR integration
//!
//! Source-level observability semantics belong to the existing
//! effect system and semantic authorities.
//!
//! The existing computational representations remain authoritative:
//!
//! - ZUIR;
//! - ZCIR and existing classical IR;
//! - quantum::ir;
//! - HDLIR;
//! - existing domain-specific IRs.
//!
//! Observability must not modify their meaning or introduce
//! another computational representation.
//!
//! ## 6. OPTIONAL INSTRUMENTATION
//!
//! Observability is optional infrastructure.
//!
//! A program must remain semantically correct when:
//!
//! - no event observer is registered;
//! - no metrics registry is configured;
//! - tracing is disabled;
//! - a sampling policy declines to record a span;
//! - an exporter is unavailable;
//! - an external telemetry service is unreachable.
//!
//! Instrumentation failures must not be interpreted as failures
//! of the computation being observed.
//!
//! An application may choose to enforce a separate operational
//! policy requiring telemetry delivery. Such a policy belongs
//! outside this composition root and must not silently change
//! language-level computational semantics.
//!
//! No process-global registry, hidden initialization, background
//! worker, or implicit network connection is created here.
//!
//! ## 7. SCALABILITY CONTRACT
//!
//! This module introduces no arbitrary universal maximum for:
//!
//! - events;
//! - metrics;
//! - metric series;
//! - labels;
//! - traces;
//! - spans;
//! - participants;
//! - tasks;
//! - operations;
//! - execution contexts;
//! - machines;
//! - hardware devices;
//! - exporters;
//! - observers.
//!
//! Actual consumption depends on available memory, processing
//! capacity, configured operational budgets, and provider policies.
//!
//! A configured budget is an operational constraint, not a
//! semantic restriction on Zamani programs.
//!
//! If a budget is exhausted, the responsible implementation
//! must return an explicit error rather than silently corrupt,
//! truncate, or misrepresent collected information.
//!
//! This module does not promise mathematically infinite storage,
//! memory, identifier space, or processing capacity.
//!
//! It preserves an open architecture that can scale as resources
//! and compatible implementations become available.
//!
//! ## 8. SECURITY AND PRIVACY
//!
//! Observability must never be treated as a trusted execution
//! boundary.
//!
//! The following information must not be recorded as ordinary
//! event attributes, metric labels, or trace attributes:
//!
//! - passwords;
//! - authentication tokens;
//! - private keys;
//! - encryption secrets;
//! - private computational state;
//! - checkpoint contents;
//! - unrestricted execution payloads;
//! - sensitive personal information.
//!
//! Metric labels must not use unbounded identities such as
//! individual task IDs, participant IDs, or trace IDs.
//!
//! Trace identifiers must never be used for authorization.
//!
//! Event producers and telemetry providers remain responsible
//! for data classification, redaction, retention, access control,
//! integrity, and encryption.
//!
//! ## 9. OBSERVABILITY DELIVERY
//!
//! This composition root does not guarantee durable event,
//! metric, or span delivery.
//!
//! Delivery guarantees are established by the selected sinks,
//! exporters, and storage providers.
//!
//! Implementations must explicitly document:
//!
//! - buffering;
//! - backpressure;
//! - retention;
//! - persistence;
//! - ordering;
//! - delivery acknowledgement;
//! - failure handling;
//! - shutdown behavior.
//!
//! No unbounded internal queue is created by this module.
//!
//! ## 10. COMPATIBILITY
//!
//! Existing child-module APIs remain authoritative.
//!
//! Public re-exports in this composition root are convenience
//! access paths and must not create competing definitions.
//!
//! Adding future observability providers must not require
//! modifying distributed semantic definitions or existing IRs.
//!
//! New observability domains should be introduced through
//! independent implementations and explicit provider interfaces.
//!
//! ## 11. COMPLETION CRITERIA
//!
//! This composition root is complete when:
//!
//! - all existing observability modules are declared;
//! - their public APIs are accessible through stable paths;
//! - no duplicate type definitions exist;
//! - no circular dependency is introduced;
//! - no unsafe Rust is used;
//! - no process-global mutable state is introduced;
//! - no arbitrary scalability ceiling is introduced;
//! - existing module tests remain available;
//! - the composition tests pass;
//! - repository-wide integration checks pass.
//!
//! This module intentionally does not claim that the complete
//! distributed runtime is production-ready merely because its
//! observability composition root exists.

// =============================================================================
// CHILD MODULES
// =============================================================================

/// Structured distributed events and event sinks.
pub mod events;

/// Generic distributed metrics and metric snapshots.
pub mod metrics;

/// Distributed trace context, spans, and exporters.
pub mod tracing;

// =============================================================================
// PUBLIC API
// =============================================================================

// Event API.

pub use self::events::{
    CompositeEventSink,
    DistributedEvent,
    EventAttributes,
    EventCategory,
    EventContext,
    EventError,
    EventName,
    EventOutcome,
    EventResult,
    EventSink,
    EventSubject,
    EventTimestamp,
    EventValue,
};

// Metrics API.

pub use self::metrics::{
    LabelName,
    LabelValue,
    MetricDescriptor,
    MetricKind,
    MetricLabels,
    MetricName,
    MetricSeriesSnapshot,
    MetricSnapshot,
    MetricSnapshotValue,
    MetricUnit,
    MetricsConfig,
    MetricsError,
    MetricsRegistry,
    MetricsResult,
};

// Tracing API.

pub use self::tracing::{
    extract_context,
    inject_context,
    insert_attribute,
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
    TraceFlags,
    TraceResult,
    TraceValue,
    Tracer,
    TRACE_CONTEXT_EXTENSION,
};

// =============================================================================
// COMPOSITION TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use std::sync::Arc;

    /// Verify that the event module is publicly composed.
    #[test]
    fn event_api_is_available() {
        let name = EventName::new("planning.feasibility_checked")
            .expect("valid event name");

        let timestamp = EventTimestamp::new(1, 0)
            .expect("valid timestamp");

        let event = DistributedEvent::new(
            EventCategory::Planning,
            name,
            timestamp,
            EventSubject::Runtime,
            EventOutcome::Succeeded,
        );

        assert_eq!(event.category(), EventCategory::Planning);
        assert_eq!(event.outcome(), EventOutcome::Succeeded);
    }

    /// Verify that the metrics module is publicly composed.
    #[test]
    fn metrics_api_is_available() {
        let registry = MetricsRegistry::new(
            MetricsConfig::unlimited(),
        );

        assert_eq!(
            registry.series_count().expect("valid registry"),
            0
        );
    }

    /// Verify that the tracing module is publicly composed.
    #[test]
    fn tracing_api_is_available() {
        let tracer = Tracer::with_fresh_namespace(
            Arc::new(PropagateOnly),
        );

        let span = tracer
            .start_root("zamani.distributed.root")
            .expect("valid root span");

        assert_eq!(
            span.sampling_decision(),
            SamplingDecision::PropagateOnly
        );

        assert!(span.end_ok().expect("span completion").is_none());
    }

    /// Verify that public modules remain accessible independently.
    #[test]
    fn child_module_namespaces_are_available() {
        let _event_module =
            events::EventCategory::Execution;

        let _metric_module =
            metrics::MetricKind::Counter;

        let _trace_module =
            tracing::SpanStatus::Ok;
    }
}
