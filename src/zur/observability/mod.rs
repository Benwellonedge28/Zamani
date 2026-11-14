
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Runtime — Observability
//!
//! Path: `src/zur/observability/mod.rs`
//!
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! # Purpose
//!
//! This module is the composition root for operational observability
//! in the Zamani Universal Runtime (ZUR).
//!
//! It exposes the existing logging, metrics, tracing and profiling
//! implementations through one consistent public module.
//!
//! It does not implement another logging engine, metric registry,
//! tracing engine, profiler, telemetry exporter or event journal.
//!
//! # Primary objective
//!
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever.
//!
//! Observability must remain portable across:
//!
//! - Classical computing.
//! - Quantum computing.
//! - HDL and hardware execution.
//! - AI and tensor computation.
//! - Hybrid computing.
//! - Distributed computing.
//! - Accelerators.
//! - Embedded systems.
//! - Photonic computing.
//! - Future computational domains.
//!
//! Observability must describe actual execution without determining
//! how computation is compiled, scheduled, routed or physically
//! realized.
//!
//! # Architectural ownership
//!
//! This module owns:
//!
//! - Observability module composition.
//! - Public observability module boundaries.
//! - Deliberate public API re-exports.
//! - Observability integration documentation.
//! - Module-level safety requirements.
//! - Architectural dependency documentation.
//!
//! It does not own:
//!
//! - Source-language observability semantics.
//! - Grammar or AST representations.
//! - Semantic validation.
//! - Runtime lifecycle.
//! - Execution lifecycle.
//! - Task scheduling.
//! - Resource allocation.
//! - Hardware discovery.
//! - Provider selection.
//! - Distributed membership.
//! - Distributed execution.
//! - Event persistence.
//! - Network transport.
//! - Security authorization.
//! - Compiler profiling.
//! - JIT hotness decisions.
//! - Backend telemetry.
//! - Quantum error correction.
//! - Hardware performance counter discovery.
//! - A second telemetry implementation.
//!
//! Those responsibilities remain with their existing authorities.
//!
//! # Child module responsibilities
//!
//! ## logging
//!
//! `logging.rs` owns structured runtime logging.
//!
//! It provides:
//!
//! - Structured log events.
//! - Log levels and classifications.
//! - Structured log fields.
//! - Sensitive-value redaction.
//! - Logging configuration.
//! - Logging policies.
//! - Injectable logging sinks.
//! - JSON Lines output.
//! - Explicit logger construction.
//!
//! Logging failures remain distinct from computation failures.
//!
//! ## metrics
//!
//! `metrics.rs` owns ZUR-specific metric interfaces.
//!
//! It provides:
//!
//! - Runtime metric descriptors.
//! - Metric names and units.
//! - Counters.
//! - Gauges.
//! - Histograms.
//! - Metric labels.
//! - Metric registration.
//! - Metric snapshots.
//! - Optional series budgets.
//!
//! Generic metric collection and storage remain owned by:
//!
//! `crate::distributed::observability::metrics`
//!
//! ZUR does not create another generic metric registry.
//!
//! ## tracing
//!
//! `tracing.rs` owns the ZUR-facing tracing adapter.
//!
//! It associates canonical RuntimeOperationId values with
//! distributed trace and span contexts.
//!
//! Generic trace identities, span identities, sampling,
//! propagation and export remain owned by:
//!
//! `crate::distributed::observability::tracing`
//!
//! ZUR must not introduce another trace identity allocator,
//! exporter registry or distributed tracing engine.
//!
//! Trace context is correlation metadata, not authentication.
//!
//! ## profiling
//!
//! `profiling.rs` owns runtime execution profiling.
//!
//! It provides:
//!
//! - Profiling scopes.
//! - Profiling records.
//! - Measurements.
//! - Profiling attributes.
//! - Profiling outcomes.
//! - Sampling configuration.
//! - Profiling sinks.
//! - Explicit profiling budgets.
//!
//! Compiler profiling remains owned by:
//!
//! `crate::compiler::profiling`
//!
//! JIT profiling remains owned by:
//!
//! `crate::zur::jit::profiling`
//!
//! AI profiling remains owned by:
//!
//! `crate::ai::observability::profiling`
//!
//! Runtime profiling must not duplicate those implementations.
//!
//! # Dependency direction
//!
//! ```text
//!                    ZUR
//!                     |
//!                     v
//!             observability/mod.rs
//!                     |
//!          +----------+----------+
//!          |          |          |
//!          v          v          v
//!       logging    metrics    tracing
//!          |          |          |
//!          |          v          v
//!          |    distributed  distributed
//!          |      metrics      tracing
//!          |
//!          v
//!      Log sinks
//!
//!                     |
//!                     v
//!                 profiling
//!                     |
//!                     v
//!              Profiling sinks
//! ```
//!
//! The composition root must not initialize any of these services.
//!
//! Construction, configuration, activation and shutdown remain
//! explicit responsibilities of their owning components.
//!
//! # Canonical integration authorities
//!
//! Runtime identity:
//! `crate::zur::ids`
//!
//! Runtime errors:
//! `crate::zur::error`
//!
//! Runtime policy:
//! `crate::zur::policy`
//!
//! Runtime context:
//! `crate::zur::core::context`
//!
//! Runtime execution:
//! `crate::zur::execution`
//!
//! Runtime JIT:
//! `crate::zur::jit`
//!
//! Runtime resources:
//! `crate::zur::resources`
//!
//! Generic distributed events:
//! `crate::distributed::observability::events`
//!
//! Generic distributed metrics:
//! `crate::distributed::observability::metrics`
//!
//! Generic distributed tracing:
//! `crate::distributed::observability::tracing`
//!
//! Compiler profiling:
//! `crate::compiler::profiling`
//!
//! AI observability:
//! `crate::ai::observability`
//!
//! Source-level observability intent:
//! `grammar/execution/observability.g4`
//!
//! # Observability invariants
//!
//! 1. Observation is not execution.
//! 2. Measurement is not proof of successful computation.
//! 3. Trace correlation is not authorization.
//! 4. Metric collection does not allocate resources.
//! 5. Profiling does not make scheduling decisions.
//! 6. Logging does not replace program output.
//! 7. Export success does not imply durable persistence.
//! 8. Sampling must not determine whether computation executes.
//! 9. Telemetry failures must not fabricate computational results.
//! 10. Missing optional observability must not silently change
//!     program semantics.
//! 11. Operational telemetry requirements must be enforced
//!     through explicit policies.
//! 12. Cancellation and failure must not be reported as success.
//! 13. Domain extensions must not require modifying this module.
//!
//! # POCO-REAF and scalability
//!
//! This module introduces no arbitrary universal maximum for:
//!
//! - Log events.
//! - Metric definitions.
//! - Metric observations.
//! - Metric series.
//! - Trace contexts.
//! - Spans.
//! - Profiling scopes.
//! - Profiling records.
//! - Runtime operations.
//! - Providers.
//! - Devices.
//! - Computational domains.
//! - Distributed participants.
//! - Machines.
//! - Hardware resources.
//!
//! The module does not impose:
//!
//! - MAX_EVENTS.
//! - MAX_METRICS.
//! - MAX_SPANS.
//! - MAX_TRACE_DEPTH.
//! - MAX_SAMPLES.
//! - MAX_PROVIDERS.
//! - MAX_DEVICES.
//! - MAX_DOMAINS.
//!
//! Explicit operational budgets remain permissible.
//!
//! Such budgets are resource policies, not language-level
//! computational ceilings.
//!
//! Actual throughput and storage depend on available memory,
//! storage, processing capacity and configured providers.
//!
//! No finite machine promises literal infinite computation.
//!
//! # Safety
//!
//! This module enforces safe Rust.
//!
//! It introduces:
//!
//! - No unsafe blocks.
//! - No unsafe functions.
//! - No unsafe implementations.
//! - No raw-pointer ownership.
//! - No process-global mutable state.
//! - No implicit logger installation.
//! - No implicit tracer installation.
//! - No implicit profiler installation.
//! - No hidden background workers.
//! - No automatic network connections.
//! - No fabricated telemetry.
//!
//! # Security
//!
//! Observability must not become an unrestricted payload storage
//! mechanism.
//!
//! Credentials, authentication tokens, private keys, secrets and
//! private execution payloads must not be recorded as ordinary
//! operational attributes.
//!
//! Logging redaction is provided by the logging implementation.
//!
//! Trace attributes and metric labels must follow their respective
//! canonical validation rules.
//!
//! Observability does not establish authorization to access,
//! export, retain or disclose information.
//!
//! # Grammar integration
//!
//! The canonical grammar:
//!
//! `grammar/execution/observability.g4`
//!
//! owns source-level observability syntax and intent.
//!
//! It describes:
//!
//! - Events.
//! - Metrics.
//! - Logs.
//! - Traces.
//! - Spans.
//! - Profiles.
//! - Sampling.
//! - Filtering.
//! - Attributes.
//! - Correlation.
//! - Collection.
//! - Retention.
//! - Export.
//! - Privacy.
//! - Classification.
//! - Integrity.
//! - Provenance.
//! - Requirements.
//! - Capabilities.
//! - Preferences.
//! - Constraints.
//! - Hints.
//! - Extensions.
//!
//! The grammar does not implement telemetry.
//!
//! The frontend and semantic layer validate source declarations.
//!
//! ZUR consumes validated runtime configuration and operational
//! contracts; it must not parse the grammar or infer authorization
//! from source-level declarations.
//!
//! # Integration lifecycle
//!
//! ```text
//! Validated observability intent
//!              |
//!              v
//!       Runtime configuration
//!              |
//!              v
//!       Explicit service setup
//!              |
//!              v
//!       Observability component
//!              |
//!              v
//!       Actual observation
//!              |
//!              v
//!       Configured sink/provider
//! ```
//!
//! This composition root performs none of the initialization steps.
//!
//! # Integration status
//!
//! This file composes the existing four ZUR observability modules.
//!
//! It does not claim that every exporter, storage provider,
//! observability grammar construct or cross-domain instrumentation
//! adapter is implemented or production-certified.

// ============================================================================
// OBSERVABILITY SUBMODULES
// ============================================================================

/// Structured ZUR runtime logging.
///
/// Owns log construction, classification, redaction, filtering
/// and explicitly injected logging sinks.
pub mod logging;

/// ZUR-specific runtime metric interfaces.
///
/// Generic metric storage and collection are delegated to
/// distributed observability.
pub mod metrics;

/// ZUR-facing distributed tracing adapter.
///
/// Trace and span identities, propagation and export are delegated
/// to canonical distributed tracing.
pub mod tracing;

/// Domain-neutral runtime execution profiling.
///
/// Compiler, JIT and AI profiling remain independently owned.
pub mod profiling;

// ============================================================================
// PUBLIC LOGGING API
// ============================================================================

/// Structured runtime log event.
pub use self::logging::LogEvent;

/// Structured runtime log field.
pub use self::logging::LogField;

/// Runtime logging severity.
pub use self::logging::LogLevel;

/// Runtime logging classification.
pub use self::logging::LogClass;

/// Logging field visibility.
pub use self::logging::LogVisibility;

/// Runtime logging configuration.
pub use self::logging::LogConfig;

/// Result of a logging operation.
pub use self::logging::LogDisposition;

/// Runtime logging sink contract.
pub use self::logging::LogSink;

/// JSON Lines logging sink.
pub use self::logging::JsonLineSink;

/// Explicitly constructed runtime logger.
pub use self::logging::Logger;

// ============================================================================
// PUBLIC METRICS API
// ============================================================================

/// ZUR-specific metric name.
pub use self::metrics::ZurMetricName;

/// ZUR-specific metric unit.
pub use self::metrics::ZurMetricUnit;

/// ZUR metric labels.
pub use self::metrics::ZurMetricLabels;

/// ZUR metric descriptor.
pub use self::metrics::ZurMetricDescriptor;

/// ZUR metrics configuration.
pub use self::metrics::ZurMetricsConfig;

/// ZUR counter handle.
pub use self::metrics::ZurCounter;

/// ZUR gauge handle.
pub use self::metrics::ZurGauge;

/// ZUR histogram handle.
pub use self::metrics::ZurHistogram;

/// ZUR metrics registry adapter.
pub use self::metrics::ZurMetricsRegistry;

/// ZUR metric series snapshot.
pub use self::metrics::ZurMetricSeriesSnapshot;

/// ZUR metric snapshot value.
pub use self::metrics::ZurMetricSnapshotValue;

/// ZUR metric snapshot.
pub use self::metrics::ZurMetricSnapshot;

/// Result of a ZUR metrics operation.
pub use self::metrics::ZurMetricsResult;

// ============================================================================
// PUBLIC TRACING API
// ============================================================================

/// ZUR runtime tracing adapter.
pub use self::tracing::ZurTracer;

/// ZUR runtime trace context.
pub use self::tracing::ZurTraceContext;

/// ZUR runtime span.
pub use self::tracing::ZurSpan;

/// Result of a ZUR tracing operation.
pub use self::tracing::ZurTracingResult;

/// Canonical distributed trace context.
pub use self::tracing::DistributedTraceContext;

/// Canonical distributed trace attributes.
pub use self::tracing::DistributedTraceAttributes;

/// Canonical distributed trace attribute value.
pub use self::tracing::DistributedTraceValue;

/// Canonical distributed span status.
pub use self::tracing::DistributedSpanStatus;

/// Canonical distributed completed span.
pub use self::tracing::DistributedCompletedSpan;

/// Canonical distributed trace flags.
pub use self::tracing::TraceFlags;

/// Reserved ZUR runtime-operation trace attribute.
pub use self::tracing::RUNTIME_OPERATION_ATTRIBUTE;

// ============================================================================
// PUBLIC PROFILING API
// ============================================================================

/// Runtime profiling configuration.
pub use self::profiling::ProfilingConfig;

/// Runtime profiling budget.
pub use self::profiling::ProfilingBudget;

/// Runtime profiling sampling policy.
pub use self::profiling::ProfilingSampling;

/// Runtime profiling outcome.
pub use self::profiling::ProfilingOutcome;

/// Runtime profiling measurement value.
pub use self::profiling::ProfilingValue;

/// Runtime profiling attributes.
pub use self::profiling::ProfilingAttributes;

/// Runtime profiling measurements.
pub use self::profiling::ProfilingMeasurements;

/// Runtime profiling record.
pub use self::profiling::ProfilingRecord;

/// Runtime profiling sink.
pub use self::profiling::ProfilingSink;

/// Runtime profiling sink error.
pub use self::profiling::ProfilingSinkError;

/// Runtime profiling sink result.
pub use self::profiling::ProfilingSinkResult;

/// Runtime profiler.
pub use self::profiling::Profiler;

/// Runtime profiling scope.
pub use self::profiling::ProfilingScope;

/// Runtime profiling error.
pub use self::profiling::ProfilingError;

/// Runtime profiling error classification.
pub use self::profiling::ProfilingErrorKind;

/// Result of a runtime profiling operation.
pub use self::profiling::ProfilingResult;

/// Runtime profiling API version.
pub use self::profiling::RUNTIME_PROFILING_API_VERSION;

// ============================================================================
// COMPOSITION CONTRACT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// Confirms that all four independently owned components
    /// are publicly composed.
    #[test]
    fn all_observability_components_are_available() {
        let _logging: fn() -> LogLevel = || LogLevel::Info;
        let _metrics: fn() -> ZurMetricsConfig =
            ZurMetricsConfig::unlimited;
        let _tracing_version: fn() -> &'static str = || {
            RUNTIME_OPERATION_ATTRIBUTE
        };
        let _profiling_version: fn() -> u16 = || {
            RUNTIME_PROFILING_API_VERSION
        };

        assert_eq!(RUNTIME_PROFILING_API_VERSION, 1);
        assert_eq!(
            RUNTIME_OPERATION_ATTRIBUTE,
            "zamani.runtime.operation.id"
        );
    }

    /// Confirms that the canonical submodule paths remain
    /// independently accessible.
    #[test]
    fn canonical_child_module_paths_are_available() {
        let _ = logging::LogLevel::Debug;
        let _ = metrics::ZurMetricsConfig::unlimited();
        let _ = tracing::RUNTIME_OPERATION_ATTRIBUTE;
        let _ = profiling::RUNTIME_PROFILING_API_VERSION;
    }

    /// Confirms that the composition root does not impose a
    /// default metric-series budget.
    #[test]
    fn default_metrics_configuration_has_no_extra_series_budget() {
        let configuration = ZurMetricsConfig::unlimited();

        assert!(configuration.max_series().is_none());
    }

    /// Confirms that the composition root does not impose a
    /// profiling-field budget.
    #[test]
    fn profiling_supports_resource_policy_configuration() {
        let budget = ProfilingBudget::unlimited();

        assert!(budget.max_fields_per_scope().is_none());
    }
}
