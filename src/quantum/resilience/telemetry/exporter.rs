//! Zamani Quantum Resilience — Telemetry Export Boundary
//!
//! Path:
//!     src/quantum/resilience/telemetry/exporter.rs
//!
//! # Purpose
//!
//! This module defines the provider-neutral export contract for resilience
//! telemetry.
//!
//! The exporter boundary is intentionally separate from:
//!
//! - telemetry collection;
//! - event/metric semantics;
//! - serialization;
//! - transport;
//! - persistence;
//! - detection;
//! - diagnosis;
//! - recovery;
//! - hardware;
//! - backend/provider integrations.
//!
//! The exporter receives already-validated immutable telemetry and hands it to
//! a concrete deployment-specific exporter implementation.
//!
//! The concrete implementation may export to:
//!
//! - an in-process observer;
//! - a file or local store;
//! - an OpenTelemetry collector;
//! - a metrics system;
//! - a message bus;
//! - a remote telemetry service;
//! - a database;
//! - a user-defined telemetry sink.
//!
//! None of those technologies is required by this module.
//!
//! # Architectural position
//!
//! ```text
//! hardware / runtime / QEC / compiler / scheduler / execution
//!                              │
//!                              ▼
//!                         collector
//!                              │
//!                    immutable telemetry
//!                              │
//!             ┌────────────────┴────────────────┐
//!             │                                 │
//!             ▼                                 ▼
//!          detection                         exporter
//!                                                 │
//!                         ┌───────────────────────┼────────────────────┐
//!                         ▼                       ▼                    ▼
//!                    local sink              remote sink          observability
//! ```
//!
//! Exporting is deliberately downstream of telemetry semantics.
//!
//! A failed exporter MUST NOT alter the canonical telemetry object and MUST NOT
//! become a resilience decision by itself.
//!
//! # Critical resilience invariant
//!
//! ```text
//! telemetry producer
//!       │
//!       ▼
//! canonical telemetry
//!       │
//!       ├──────────► resilience detection
//!       │
//!       └──────────► exporter
//! ```
//!
//! The exporter is therefore observational. It must never be a required
//! dependency for correctness, fault detection, diagnosis, planning,
//! recovery, or verification unless a higher-level policy explicitly makes
//! external telemetry delivery a deployment requirement.
//!
//! # Write once, scale everywhere
//!
//! This file contains no hard-coded:
//!
//! - qubit count;
//! - physical qubit count;
//! - logical qubit count;
//! - device count;
//! - backend count;
//! - event count;
//! - metric count;
//! - batch size;
//! - retry count;
//! - timeout;
//! - memory size;
//! - payload size;
//! - provider;
//! - protocol;
//! - network endpoint.
//!
//! Those values belong to exporter configuration, deployment policy, target
//! capability, or the concrete exporter implementation.
//!
//! The exporter contract consequently scales from a single observation to a
//! distributed quantum fleet without changing the API.
//!
//! # Determinism
//!
//! This module does not:
//!
//! - read the system clock;
//! - generate identifiers;
//! - generate random numbers;
//! - access environment variables;
//! - access the filesystem;
//! - access the network;
//! - mutate telemetry;
//! - serialize telemetry;
//! - select a provider.
//!
//! Deterministic callers therefore receive deterministic lifecycle behavior.
//!
//! A concrete exporter may use nondeterministic external infrastructure, but
//! that nondeterminism must remain outside the canonical telemetry model.
//!
//! # Ownership
//!
//! Export methods receive borrowed telemetry.
//!
//! Exporters MUST NOT retain references after the export operation returns
//! unless their implementation explicitly owns a copied representation.
//!
//! This prevents exporter lifetime requirements from leaking into the
//! resilience telemetry model.
//!
//! # Retry
//!
//! Retry belongs to the concrete exporter because retry semantics depend on
//! the transport and destination.
//!
//! The core resilience telemetry layer MUST NOT blindly retry exports.
//!
//! A concrete exporter may implement:
//!
//! - exponential backoff;
//! - jitter;
//! - destination-provided retry delays;
//! - connection recovery;
//! - bounded retry budgets;
//! - circuit breaking.
//!
//! Retry MUST remain independent of quantum execution recovery.
//!
//! A telemetry export failure must never cause a quantum program to be retried
//! unless an explicitly higher-level policy decides that telemetry delivery is
//! itself a correctness requirement.
//!
//! Current OpenTelemetry guidance likewise places concurrent-request and retry
//! behavior at the exporter/protocol layer rather than in the generic SDK
//! processor layer. 
//!
//! # Backpressure
//!
//! Exporters must not require an unbounded in-memory queue.
//!
//! Queueing, batching, dropping, persistence, backpressure and spill-to-disk
//! are deployment/exporter responsibilities.
//!
//! The core interface therefore uses borrowed batches and reports the outcome
//! explicitly.
//!
//! # Partial success
//!
//! An exporter may accept some records while rejecting others.
//!
//! This is represented explicitly by [`ExportOutcome`] rather than treating a
//! partial result as full success.
//!
//! A concrete protocol such as OTLP may have protocol-specific rules for
//! partial success and retry. Those rules belong in that concrete exporter.
//! OTLP, for example, specifies that populated partial-success responses are
//! not retried. 
//!
//! # Lifecycle
//!
//! Exporters have four conceptual states:
//!
//! ```text
//! Created → Running → ShuttingDown → ShutDown
//! ```
//!
//! `export()` is only valid while running.
//!
//! `force_flush()` requests completion of already accepted/buffered data.
//!
//! `shutdown()` transitions the exporter into its terminal state.
//!
//! Shutdown is idempotent at the contract level.
//!
//! An implementation MUST NOT block indefinitely during shutdown.
//! OpenTelemetry's SDK specifications use the same lifecycle expectation for
//! exporters. 
//!
//! # Thread safety
//!
//! Exporter implementations are required to be `Send + Sync`.
//!
//! This does not require every exporter to perform concurrent exports.
//! Implementations may internally serialize calls.
//!
//! The concrete exporter is responsible for defining whether concurrent calls
//! are supported. The core contract never assumes a global lock.
//!
//! # Serialization boundary
//!
//! Serialization belongs to:
//!
//!     crate::quantum::resilience::serialization
//!
//! An exporter may call the canonical serialization subsystem internally, but
//! this module does not depend on a particular wire representation.
//!
//! # Security
//!
//! Exporters operate at an external-observability boundary.
//!
//! Implementations MUST treat telemetry as potentially sensitive.
//!
//! Exporters must not:
//!
//! - manufacture credentials;
//! - expose private keys;
//! - export authentication headers as telemetry;
//! - infer secrets from identifiers;
//! - modify classification metadata to bypass policy;
//! - downgrade restricted telemetry without authorization;
//! - export raw quantum state unless an explicit security policy permits it.
//!
//! Data classification is already part of the metric model and should be
//! honored by concrete exporters.
//!
//! # Error contract
//!
//! All fallible exporter operations return:
//!
//!     Result<T, ResilienceError>
//!
//! The shared resilience error model already provides stable error codes,
//! categories, retryability and recoverability. Exporter failures therefore
//! use that contract rather than creating a competing exporter-specific error
//! hierarchy. 
//!
//! # Integration
//!
//! `telemetry/mod.rs` should expose:
//!
//! ```text
//! pub mod exporter;
//! ```
//!
//! The exporter imports canonical telemetry types:
//!
//! ```text
//! crate::quantum::resilience::telemetry::event::TelemetryEvent
//! crate::quantum::resilience::telemetry::metric::Metric
//! ```
//!
//! No resilience-local qubit identity is introduced here.
//!
//! Event and metric definitions already use:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! The exporter deliberately does not duplicate those types.
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
#![deny(missing_docs)]

// =============================================================================
// Imports
// =============================================================================

use std::fmt;
use std::sync::atomic::{AtomicU8, Ordering};

use crate::quantum::resilience::errors::error::{
    ResilienceError,
    ResilienceErrorCode,
};
use crate::quantum::resilience::telemetry::event::TelemetryEvent;
use crate::quantum::resilience::telemetry::metric::Metric;

// =============================================================================
// Schema
// =============================================================================

/// Stable semantic schema identifier for the exporter contract.
pub const TELEMETRY_EXPORTER_SCHEMA_ID: &str =
    "zamani.quantum.resilience.telemetry.exporter";

/// Semantic version of the exporter contract.
pub const TELEMETRY_EXPORTER_SCHEMA_VERSION: u32 = 1;

// =============================================================================
// Exporter lifecycle
// =============================================================================

/// Internal exporter lifecycle state.
///
/// Numeric values are private implementation details. They must never become
/// an external protocol representation.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ExporterState {
    Created = 0,
    Running = 1,
    ShuttingDown = 2,
    ShutDown = 3,
}

impl ExporterState {
    const fn from_u8(value: u8) -> Self {
        match value {
            0 => Self::Created,
            1 => Self::Running,
            2 => Self::ShuttingDown,
            _ => Self::ShutDown,
        }
    }
}

// =============================================================================
// Public lifecycle state
// =============================================================================

/// Observable exporter lifecycle state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ExporterLifecycle {
    /// The exporter has been constructed but not started.
    Created,

    /// The exporter accepts exports.
    Running,

    /// Shutdown has begun and new exports are rejected.
    ShuttingDown,

    /// The exporter is permanently closed.
    ShutDown,
}

impl ExporterLifecycle {
    /// Returns a stable machine-readable representation.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Created => "created",
            Self::Running => "running",
            Self::ShuttingDown => "shutting_down",
            Self::ShutDown => "shut_down",
        }
    }
}

impl fmt::Display for ExporterLifecycle {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Export request
// =============================================================================

/// Immutable batch of telemetry presented to an exporter.
///
/// The batch owns no telemetry. All records are borrowed for the duration of
/// the export call.
///
/// This is intentionally not a `Vec` and has no implicit capacity limit.
/// Batching policy belongs to the collector or concrete exporter.
#[derive(Debug, Clone, Copy)]
pub struct TelemetryBatch<'a> {
    events: &'a [TelemetryEvent],
    metrics: &'a [Metric],
}

impl<'a> TelemetryBatch<'a> {
    /// Creates a telemetry batch.
    ///
    /// No machine-size assumption is made.
    #[must_use]
    pub const fn new(
        events: &'a [TelemetryEvent],
        metrics: &'a [Metric],
    ) -> Self {
        Self { events, metrics }
    }

    /// Returns the event records.
    #[must_use]
    pub const fn events(&self) -> &'a [TelemetryEvent] {
        self.events
    }

    /// Returns metric records.
    #[must_use]
    pub const fn metrics(&self) -> &'a [Metric] {
        self.metrics
    }

    /// Returns the number of events.
    #[must_use]
    pub const fn event_count(&self) -> usize {
        self.events.len()
    }

    /// Returns the number of metrics.
    #[must_use]
    pub const fn metric_count(&self) -> usize {
        self.metrics.len()
    }

    /// Returns the total number of records.
    #[must_use]
    pub const fn record_count(&self) -> usize {
        self.events.len() + self.metrics.len()
    }

    /// Returns whether the batch contains no telemetry.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.events.is_empty() && self.metrics.is_empty()
    }
}

// =============================================================================
// Export result
// =============================================================================

/// Outcome of an export operation.
///
/// The exporter must distinguish complete success, partial success and failure
/// so higher-level observability can accurately account for delivered data.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExportOutcome {
    /// Every supplied record was accepted by the destination.
    Accepted {
        /// Number of accepted event records.
        events: usize,

        /// Number of accepted metric records.
        metrics: usize,
    },

    /// Some records were accepted and some were rejected.
    ///
    /// Rejected records must not be silently represented as accepted.
    PartiallyAccepted {
        /// Number of accepted event records.
        accepted_events: usize,

        /// Number of rejected event records.
        rejected_events: usize,

        /// Number of accepted metric records.
        accepted_metrics: usize,

        /// Number of rejected metric records.
        rejected_metrics: usize,
    },
}

impl ExportOutcome {
    /// Creates a full-success result for a batch.
    #[must_use]
    pub const fn accepted(batch: TelemetryBatch<'_>) -> Self {
        Self::Accepted {
            events: batch.event_count(),
            metrics: batch.metric_count(),
        }
    }

    /// Returns whether every supplied record was accepted.
    #[must_use]
    pub const fn is_complete(self) -> bool {
        matches!(self, Self::Accepted { .. })
    }

    /// Returns the number of accepted records.
    #[must_use]
    pub const fn accepted_count(self) -> usize {
        match self {
            Self::Accepted { events, metrics } => events + metrics,

            Self::PartiallyAccepted {
                accepted_events,
                accepted_metrics,
                ..
            } => accepted_events + accepted_metrics,
        }
    }

    /// Returns the number of rejected records.
    #[must_use]
    pub const fn rejected_count(self) -> usize {
        match self {
            Self::Accepted { .. } => 0,

            Self::PartiallyAccepted {
                rejected_events,
                rejected_metrics,
                ..
            } => rejected_events + rejected_metrics,
        }
    }

    /// Validates that the outcome cannot claim more records than were supplied.
    pub fn validate_against(
        self,
        batch: TelemetryBatch<'_>,
    ) -> Result<(), ResilienceError> {
        let event_count = batch.event_count();
        let metric_count = batch.metric_count();

        let valid = match self {
            Self::Accepted { events, metrics } => {
                events == event_count && metrics == metric_count
            }

            Self::PartiallyAccepted {
                accepted_events,
                rejected_events,
                accepted_metrics,
                rejected_metrics,
            } => {
                accepted_events
                    .checked_add(rejected_events)
                    == Some(event_count)
                    && accepted_metrics
                        .checked_add(rejected_metrics)
                        == Some(metric_count)
            }
        };

        if valid {
            Ok(())
        } else {
            Err(exporter_error(
                ResilienceErrorCode::InvariantViolation,
                "telemetry exporter returned an outcome inconsistent with the submitted batch",
            ))
        }
    }
}

// =============================================================================
// Exporter configuration
// =============================================================================

/// Configuration contract for an exporter.
///
/// The core exporter does not supply defaults for deployment-specific values.
/// Every field is optional so a concrete exporter can define its own required
/// configuration without forcing a universal policy onto Zamani.
///
/// This structure deliberately contains no endpoint, timeout, retry count or
/// batch-size constant.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ExporterConfig {
    /// Optional deployment-defined exporter identity.
    identity: Option<String>,

    /// Whether export failures are considered observable by the caller.
    ///
    /// This does not make failures affect quantum execution automatically.
    report_failures: bool,
}

impl ExporterConfig {
    /// Creates a configuration with no deployment identity.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            identity: None,
            report_failures: true,
        }
    }

    /// Sets the deployment-defined exporter identity.
    #[must_use]
    pub fn with_identity(mut self, identity: impl Into<String>) -> Self {
        self.identity = Some(identity.into());
        self
    }

    /// Sets whether callers should receive exporter failures.
    ///
    /// This setting controls API reporting only. It does not suppress internal
    /// exporter errors or turn an exporter failure into a quantum execution
    /// failure.
    #[must_use]
    pub const fn report_failures(mut self, value: bool) -> Self {
        self.report_failures = value;
        self
    }

    /// Returns the configured exporter identity.
    #[must_use]
    pub fn identity(&self) -> Option<&str> {
        self.identity.as_deref()
    }

    /// Returns whether export failures are reported.
    #[must_use]
    pub const fn reports_failures(&self) -> bool {
        self.report_failures
    }

    /// Validates the configuration.
    pub fn validate(&self) -> Result<(), ResilienceError> {
        if let Some(identity) = &self.identity {
            if identity.trim().is_empty() {
                return Err(exporter_error(
                    ResilienceErrorCode::InvalidConfiguration,
                    "telemetry exporter identity must not be empty",
                ));
            }

            if identity.chars().any(char::is_control) {
                return Err(exporter_error(
                    ResilienceErrorCode::InvalidConfiguration,
                    "telemetry exporter identity must not contain control characters",
                ));
            }
        }

        Ok(())
    }
}

// =============================================================================
// Exporter statistics
// =============================================================================

/// Monotonic exporter statistics.
///
/// Statistics are exporter-local operational observations. They are not part
/// of quantum program semantics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ExporterStatistics {
    /// Number of export operations attempted.
    pub export_operations: u64,

    /// Number of export operations that completed successfully.
    pub successful_operations: u64,

    /// Number of export operations that returned an error.
    pub failed_operations: u64,

    /// Number of records accepted.
    pub accepted_records: u64,

    /// Number of records rejected.
    pub rejected_records: u64,

    /// Number of flush operations.
    pub flush_operations: u64,

    /// Number of shutdown operations.
    pub shutdown_operations: u64,
}

impl ExporterStatistics {
    /// Returns the number of records that were attempted.
    #[must_use]
    pub const fn attempted_records(self) -> u64 {
        self.accepted_records
            .saturating_add(self.rejected_records)
    }
}

// =============================================================================
// Exporter trait
// =============================================================================

/// Provider-neutral telemetry exporter contract.
///
/// Concrete exporters implement this trait.
///
/// The trait deliberately contains no serialization or transport methods.
/// Those belong to concrete implementations and the serialization subsystem.
///
/// # Concurrency
///
/// The trait is `Send + Sync`. An implementation may serialize export calls
/// internally if its transport requires it.
///
/// # Ownership
///
/// Telemetry is borrowed. Implementations must not retain references to the
/// supplied batch after the call returns.
///
/// # Failure semantics
///
/// Export failures are represented using [`ResilienceError`].
///
/// A concrete exporter should classify transient failures as retryable where
/// appropriate. The core resilience system must not infer quantum recovery
/// behavior from exporter retryability.
pub trait TelemetryExporter: Send + Sync + fmt::Debug {
    /// Returns the exporter identity, if configured.
    fn identity(&self) -> Option<&str> {
        None
    }

    /// Exports a telemetry batch.
    ///
    /// The operation must eventually return either an outcome or an error.
    /// Concrete exporters must enforce their own configured deadline or
    /// cancellation mechanism.
    fn export(
        &self,
        batch: TelemetryBatch<'_>,
    ) -> Result<ExportOutcome, ResilienceError>;

    /// Flushes accepted/buffered telemetry.
    ///
    /// A stateless exporter may simply return `Ok(())`.
    fn force_flush(&self) -> Result<(), ResilienceError> {
        Ok(())
    }

    /// Performs exporter shutdown.
    ///
    /// Shutdown should be idempotent for concrete implementations.
    fn shutdown(&self) -> Result<(), ResilienceError> {
        Ok(())
    }
}

// =============================================================================
// Exporter lifecycle wrapper
// =============================================================================

/// Lifecycle-managed exporter handle.
///
/// `ManagedExporter` provides the common lifecycle contract while leaving
/// transport, serialization, buffering, batching and retry behavior to the
/// concrete exporter.
///
/// This prevents every exporter implementation from independently reinventing
/// lifecycle state transitions.
#[derive(Debug)]
pub struct ManagedExporter<E> {
    exporter: E,
    state: AtomicU8,
}

impl<E> ManagedExporter<E>
where
    E: TelemetryExporter,
{
    /// Creates a managed exporter in the `Created` state.
    ///
    /// The exporter is not usable until [`Self::start`] is called.
    pub fn new(exporter: E) -> Result<Self, ResilienceError> {
        if let Some(identity) = exporter.identity() {
            validate_identity(identity)?;
        }

        Ok(Self {
            exporter,
            state: AtomicU8::new(ExporterState::Created as u8),
        })
    }

    /// Starts the exporter.
    ///
    /// Starting is idempotent if the exporter is already running.
    pub fn start(&self) -> Result<(), ResilienceError> {
        let current = ExporterState::from_u8(
            self.state.load(Ordering::Acquire),
        );

        match current {
            ExporterState::Created => {
                self.state.store(
                    ExporterState::Running as u8,
                    Ordering::Release,
                );
                Ok(())
            }

            ExporterState::Running => Ok(()),

            ExporterState::ShuttingDown => Err(lifecycle_error(
                "telemetry exporter is shutting down",
            )),

            ExporterState::ShutDown => Err(lifecycle_error(
                "telemetry exporter has already shut down",
            )),
        }
    }

    /// Returns the current lifecycle state.
    #[must_use]
    pub fn lifecycle(&self) -> ExporterLifecycle {
        match ExporterState::from_u8(
            self.state.load(Ordering::Acquire),
        ) {
            ExporterState::Created => ExporterLifecycle::Created,
            ExporterState::Running => ExporterLifecycle::Running,
            ExporterState::ShuttingDown => ExporterLifecycle::ShuttingDown,
            ExporterState::ShutDown => ExporterLifecycle::ShutDown,
        }
    }

    /// Returns a shared reference to the concrete exporter.
    #[must_use]
    pub const fn exporter(&self) -> &E {
        &self.exporter
    }

    /// Exports a batch through the managed lifecycle.
    ///
    /// The telemetry is validated before being handed to the concrete
    /// exporter. This prevents malformed canonical telemetry from crossing
    /// the external-observability boundary.
    pub fn export(
        &self,
        batch: TelemetryBatch<'_>,
    ) -> Result<ExportOutcome, ResilienceError> {
        self.ensure_running()?;

        validate_batch(batch)?;

        let outcome = self.exporter.export(batch)?;

        outcome.validate_against(batch)?;

        Ok(outcome)
    }

    /// Flushes the concrete exporter.
    pub fn force_flush(&self) -> Result<(), ResilienceError> {
        self.ensure_running()?;
        self.exporter.force_flush()
    }

    /// Shuts down the exporter.
    ///
    /// Shutdown is idempotent.
    pub fn shutdown(&self) -> Result<(), ResilienceError> {
        loop {
            let current = ExporterState::from_u8(
                self.state.load(Ordering::Acquire),
            );

            match current {
                ExporterState::Created => {
                    match self.state.compare_exchange(
                        ExporterState::Created as u8,
                        ExporterState::ShuttingDown as u8,
                        Ordering::AcqRel,
                        Ordering::Acquire,
                    ) {
                        Ok(_) => {
                            let result = self.exporter.shutdown();

                            self.state.store(
                                ExporterState::ShutDown as u8,
                                Ordering::Release,
                            );

                            return result;
                        }

                        Err(_) => continue,
                    }
                }

                ExporterState::Running => {
                    match self.state.compare_exchange(
                        ExporterState::Running as u8,
                        ExporterState::ShuttingDown as u8,
                        Ordering::AcqRel,
                        Ordering::Acquire,
                    ) {
                        Ok(_) => {
                            let result = self.exporter.shutdown();

                            self.state.store(
                                ExporterState::ShutDown as u8,
                                Ordering::Release,
                            );

                            return result;
                        }

                        Err(_) => continue,
                    }
                }

                ExporterState::ShuttingDown => {
                    return Ok(());
                }

                ExporterState::ShutDown => {
                    return Ok(());
                }
            }
        }
    }

    fn ensure_running(&self) -> Result<(), ResilienceError> {
        match self.lifecycle() {
            ExporterLifecycle::Running => Ok(()),

            ExporterLifecycle::Created => Err(lifecycle_error(
                "telemetry exporter has not been started",
            )),

            ExporterLifecycle::ShuttingDown => Err(lifecycle_error(
                "telemetry exporter is shutting down",
            )),

            ExporterLifecycle::ShutDown => Err(lifecycle_error(
                "telemetry exporter has already shut down",
            )),
        }
    }
}

// =============================================================================
// Convenience stateless exporter
// =============================================================================

/// A no-op exporter.
///
/// This is useful for deployments where telemetry collection remains enabled
/// but external export is intentionally disabled.
///
/// It does not discard telemetry silently from the caller's perspective:
/// every submitted record is explicitly reported as accepted by this local
/// sink. No external delivery is claimed.
#[derive(Debug, Default, Clone, Copy)]
pub struct NullExporter;

impl NullExporter {
    /// Creates a null exporter.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl TelemetryExporter for NullExporter {
    fn export(
        &self,
        batch: TelemetryBatch<'_>,
    ) -> Result<ExportOutcome, ResilienceError> {
        Ok(ExportOutcome::accepted(batch))
    }

    fn force_flush(&self) -> Result<(), ResilienceError> {
        Ok(())
    }

    fn shutdown(&self) -> Result<(), ResilienceError> {
        Ok(())
    }
}

// =============================================================================
// Validation
// =============================================================================

/// Validates an export batch without modifying it.
///
/// Event and metric constructors already provide their own structural
/// validation. The exporter nevertheless validates them again at the
/// observability boundary so malformed data cannot be introduced through
/// future deserialization or alternate construction paths.
fn validate_batch(
    batch: TelemetryBatch<'_>,
) -> Result<(), ResilienceError> {
    for event in batch.events() {
        event.validate().map_err(|_| {
            exporter_error(
                ResilienceErrorCode::InvalidArgument,
                "telemetry event failed validation before export",
            )
        })?;
    }

    for metric in batch.metrics() {
        metric.validate().map_err(|_| {
            exporter_error(
                ResilienceErrorCode::InvalidArgument,
                "telemetry metric failed validation before export",
            )
        })?;
    }

    Ok(())
}

fn validate_identity(identity: &str) -> Result<(), ResilienceError> {
    if identity.trim().is_empty() {
        return Err(exporter_error(
            ResilienceErrorCode::InvalidConfiguration,
            "telemetry exporter identity must not be empty",
        ));
    }

    if identity.chars().any(char::is_control) {
        return Err(exporter_error(
            ResilienceErrorCode::InvalidConfiguration,
            "telemetry exporter identity must not contain control characters",
        ));
    }

    Ok(())
}

// =============================================================================
// Error helpers
// =============================================================================

fn exporter_error(
    code: ResilienceErrorCode,
    message: &'static str,
) -> ResilienceError {
    ResilienceError::new(code, message)
        .with_operation("telemetry_export")
}

fn lifecycle_error(message: &'static str) -> ResilienceError {
    ResilienceError::new(
        ResilienceErrorCode::InvalidState,
        message,
    )
    .with_operation("telemetry_exporter_lifecycle")
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::quantum::resilience::telemetry::event::{
        EventId,
        EventSeverity,
        EventSourceKind,
        EventTimestamp,
        SourceId,
        TelemetryEvent,
    };

    use crate::quantum::resilience::telemetry::metric::{
        Metric,
        MetricId,
        MetricKind,
        MetricName,
        MetricTimestamp,
        MetricUnit,
        MetricValue,
        Temporality,
    };

    #[derive(Debug, Default)]
    struct RecordingExporter {
        export_calls: std::sync::Mutex<usize>,
        flush_calls: std::sync::Mutex<usize>,
        shutdown_calls: std::sync::Mutex<usize>,
    }

    impl TelemetryExporter for RecordingExporter {
        fn export(
            &self,
            batch: TelemetryBatch<'_>,
        ) -> Result<ExportOutcome, ResilienceError> {
            let mut calls = self
                .export_calls
                .lock()
                .expect("recording exporter lock must not be poisoned");

            *calls = calls.saturating_add(1);

            Ok(ExportOutcome::accepted(batch))
        }

        fn force_flush(&self) -> Result<(), ResilienceError> {
            let mut calls = self
                .flush_calls
                .lock()
                .expect("recording exporter lock must not be poisoned");

            *calls = calls.saturating_add(1);

            Ok(())
        }

        fn shutdown(&self) -> Result<(), ResilienceError> {
            let mut calls = self
                .shutdown_calls
                .lock()
                .expect("recording exporter lock must not be poisoned");

            *calls = calls.saturating_add(1);

            Ok(())
        }
    }

    fn sample_event() -> TelemetryEvent {
        /*
         * The event model is intentionally constructed through its public
         * builder/constructor contract. This test section should be adjusted
         * only if event.rs changes its public construction API.
         */
        TelemetryEvent::builder()
            .id(EventId::new("event-1").expect("valid event id"))
            .severity(EventSeverity::Informational)
            .source_kind(EventSourceKind::Application)
            .source(SourceId::new("test").expect("valid source"))
            .timestamp(
                EventTimestamp::from_unix_seconds(1_700_000_000),
            )
            .build()
            .expect("valid telemetry event")
    }

    fn sample_metric() -> Metric {
        Metric::builder()
            .id(MetricId::new("metric-1").expect("valid metric id"))
            .name(
                MetricName::new("export_test")
                    .expect("valid metric name"),
            )
            .kind(MetricKind::Gauge)
            .temporality(Temporality::Instant)
            .unit(
                MetricUnit::new("dimensionless")
                    .expect("valid metric unit"),
            )
            .value(MetricValue::Gauge(1.0))
            .timestamp(
                MetricTimestamp::from_unix_seconds(1_700_000_000),
            )
            .build()
            .expect("valid telemetry metric")
    }

    #[test]
    fn empty_batch_is_valid() {
        let batch = TelemetryBatch::new(&[], &[]);

        assert!(batch.is_empty());
        assert_eq!(batch.record_count(), 0);
    }

    #[test]
    fn accepted_outcome_matches_batch() {
        let batch = TelemetryBatch::new(&[], &[]);

        let outcome = ExportOutcome::accepted(batch);

        assert!(outcome.is_complete());
        assert_eq!(outcome.accepted_count(), 0);
        assert_eq!(outcome.rejected_count(), 0);
        assert!(outcome.validate_against(batch).is_ok());
    }

    #[test]
    fn managed_exporter_requires_start() {
        let exporter = ManagedExporter::new(NullExporter::new())
            .expect("managed exporter should construct");

        let batch = TelemetryBatch::new(&[], &[]);

        let result = exporter.export(batch);

        assert!(result.is_err());
        assert_eq!(
            exporter.lifecycle(),
            ExporterLifecycle::Created
        );
    }

    #[test]
    fn managed_exporter_can_start_and_export_empty_batch() {
        let exporter = ManagedExporter::new(NullExporter::new())
            .expect("managed exporter should construct");

        exporter.start().expect("start should succeed");

        let batch = TelemetryBatch::new(&[], &[]);

        let outcome = exporter
            .export(batch)
            .expect("export should succeed");

        assert!(outcome.is_complete());
        assert_eq!(
            exporter.lifecycle(),
            ExporterLifecycle::Running
        );
    }

    #[test]
    fn shutdown_is_idempotent() {
        let exporter = ManagedExporter::new(NullExporter::new())
            .expect("managed exporter should construct");

        exporter.start().expect("start should succeed");

        exporter.shutdown().expect("shutdown should succeed");
        exporter
            .shutdown()
            .expect("second shutdown should be harmless");

        assert_eq!(
            exporter.lifecycle(),
            ExporterLifecycle::ShutDown
        );
    }

    #[test]
    fn export_after_shutdown_is_rejected() {
        let exporter = ManagedExporter::new(NullExporter::new())
            .expect("managed exporter should construct");

        exporter.start().expect("start should succeed");
        exporter.shutdown().expect("shutdown should succeed");

        let batch = TelemetryBatch::new(&[], &[]);

        let result = exporter.export(batch);

        assert!(result.is_err());
    }

    #[test]
    fn flush_requires_running_state() {
        let exporter = ManagedExporter::new(NullExporter::new())
            .expect("managed exporter should construct");

        assert!(exporter.force_flush().is_err());

        exporter.start().expect("start should succeed");

        assert!(exporter.force_flush().is_ok());
    }

    #[test]
    fn outcome_rejects_inconsistent_counts() {
        let batch = TelemetryBatch::new(&[], &[]);

        let invalid = ExportOutcome::PartiallyAccepted {
            accepted_events: 1,
            rejected_events: 0,
            accepted_metrics: 0,
            rejected_metrics: 0,
        };

        assert!(invalid.validate_against(batch).is_err());
    }

    #[test]
    fn configuration_rejects_empty_identity() {
        let config = ExporterConfig::new().with_identity("   ");

        assert!(config.validate().is_err());
    }

    #[test]
    fn configuration_rejects_control_characters() {
        let config = ExporterConfig::new().with_identity("exporter\nname");

        assert!(config.validate().is_err());
    }

    #[test]
    fn configuration_accepts_valid_identity() {
        let config = ExporterConfig::new()
            .with_identity("local-observability");

        assert!(config.validate().is_ok());
        assert_eq!(
            config.identity(),
            Some("local-observability")
        );
    }

    #[test]
    fn null_exporter_accepts_batch_without_claiming_external_delivery() {
        let exporter = NullExporter::new();

        let batch = TelemetryBatch::new(&[], &[]);

        let outcome = exporter
            .export(batch)
            .expect("null exporter should succeed");

        assert_eq!(outcome.accepted_count(), 0);
        assert_eq!(outcome.rejected_count(), 0);
    }

    #[test]
    fn recording_exporter_lifecycle_is_supported() {
        let exporter = ManagedExporter::new(
            RecordingExporter::default(),
        )
        .expect("managed exporter should construct");

        exporter.start().expect("start should succeed");

        let batch = TelemetryBatch::new(&[], &[]);

        exporter
            .export(batch)
            .expect("export should succeed");

        exporter
            .force_flush()
            .expect("flush should succeed");

        exporter
            .shutdown()
            .expect("shutdown should succeed");

        assert_eq!(
            exporter.lifecycle(),
            ExporterLifecycle::ShutDown
        );
    }

    /*
     * Keep constructors referenced in this test module so changes to the
     * telemetry contracts are caught by compilation rather than silently
     * allowing exporter tests to drift.
     */
    #[test]
    fn telemetry_models_remain_exportable_contracts() {
        let _event = sample_event();
        let _metric = sample_metric();
    }
}