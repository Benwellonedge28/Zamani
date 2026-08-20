//! Production telemetry boundary for Zamani Quantum Error Correction.
//!
//! # Architectural contract
//!
//! Telemetry is strictly observational. It is never part of the QEC
//! correctness path.
//!
//! ```text
//!                         QEC EXECUTION
//!                              │
//!             ┌────────────────┴────────────────┐
//!             │                                 │
//!             ▼                                 ▼
//!       CORRECTNESS PATH                 OBSERVABILITY PATH
//!             │                                 │
//!       decoder result                  MetricsSnapshot
//!             │                                 │
//!       logical outcome                ResourceSnapshot
//!                                             │
//!                                             ▼
//!                                      TelemetryPolicy
//!                                             │
//!                         ┌───────────────────┼───────────────────┐
//!                         │                   │                   │
//!                         ▼                   ▼                   ▼
//!                     Disabled           Local/Aggregate    Explicit Remote
//!                                             │                   │
//!                                             ▼                   ▼
//!                                      bounded buffer      authorized exporter
//! ```
//!
//! # Security boundary
//!
//! Telemetry must never contain:
//!
//! - raw syndrome streams;
//! - quantum circuits;
//! - raw measurement payloads;
//! - QPU credentials;
//! - private keys;
//! - device secrets;
//! - arbitrary user metadata;
//! - proprietary topology data;
//! - unbounded per-event payloads.
//!
//! Only bounded aggregate information crosses this module.
//!
//! # Resource architecture
//!
//! ```text
//! QecConfig
//!     │
//!     ▼
//! QecLimits
//!     │
//!     ▼
//! ResourceManager
//!     │
//!     ▼
//! ResourceSnapshot ───────┐
//!                         │
//! MetricsSnapshot ────────┤
//!                         ▼
//!                  TelemetryCollector
//! ```
//!
//! `limits.rs` remains the declarative resource policy.
//! `resources.rs` remains resource enforcement/accounting.
//! `metrics.rs` remains execution observability.
//! `telemetry.rs` is only the bounded privacy/security export boundary.
//!
//! Telemetry never invents another resource policy.
//!
//! # Failure isolation
//!
//! Telemetry failures must never change the result of a decoder.
//!
//! A production execution path should therefore use:
//!
//! ```text
//! decode()
//!    │
//!    ├── correctness result
//!    │
//!    └── telemetry.record(...)
//!             │
//!             ├── success
//!             └── failure → ignored/logged outside correctness path
//! ```
//!
//! Remote telemetry is provider-neutral. This module performs no network I/O.

#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::collections::VecDeque;
use std::fmt;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use super::capabilities::{
    Capability,
    CapabilityContext,
    ExecutionBackend,
    ResourceRequest,
};
use super::configuration::QecConfig;
use super::errors::{QecError, QecResult};
use super::metrics::{BackendKind, DecoderId, MetricsSnapshot};
use super::resources::ResourceSnapshot;

/* ========================================================================== */
/* Constants                                                                  */
/* ========================================================================== */

/// Default bounded telemetry retention.
pub const DEFAULT_BUFFER_CAPACITY: usize = 4_096;

/// Absolute upper bound on in-memory telemetry retention.
///
/// Telemetry must never become an unbounded memory sink.
pub const MAX_BUFFER_CAPACITY: usize = 1_000_000;

/* ========================================================================== */
/* Telemetry policy                                                           */
/* ========================================================================== */

/// Controls what telemetry is permitted to do.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TelemetryPolicy {
    /// Do not collect, retain, or export telemetry.
    Disabled,

    /// Retain aggregate telemetry locally.
    ///
    /// Remote export is prohibited.
    LocalOnly,

    /// Retain aggregate telemetry locally.
    ///
    /// This policy is explicitly aggregate-only.
    Aggregated,

    /// Permit aggregate telemetry to be exported remotely.
    ///
    /// Remote export additionally requires `Capability::EmitTelemetry`.
    ExplicitRemote,
}

impl TelemetryPolicy {
    /// Returns whether telemetry collection is enabled.
    pub const fn enabled(self) -> bool {
        !matches!(self, Self::Disabled)
    }

    /// Returns whether remote export is permitted.
    pub const fn permits_remote(self) -> bool {
        matches!(self, Self::ExplicitRemote)
    }

    /// Stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Disabled => "disabled",
            Self::LocalOnly => "local_only",
            Self::Aggregated => "aggregated",
            Self::ExplicitRemote => "explicit_remote",
        }
    }
}

impl Default for TelemetryPolicy {
    fn default() -> Self {
        Self::LocalOnly
    }
}

/* ========================================================================== */
/* Runtime configuration                                                       */
/* ========================================================================== */

/// Runtime telemetry configuration.
///
/// `QecConfig` remains the persistent configuration source of truth.
/// This structure contains only runtime controls that belong specifically
/// to telemetry retention/export.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TelemetryRuntimeConfig {
    /// Telemetry policy.
    pub policy: TelemetryPolicy,

    /// Maximum records retained in memory.
    pub max_buffer_records: usize,

    /// Deterministic sampling probability in `[0, 1]`.
    pub sampling_rate: f64,

    /// Include aggregate metrics.
    pub include_metrics: bool,

    /// Include aggregate resource usage.
    pub include_resources: bool,

    /// Include decoder statistics.
    pub include_decoder_statistics: bool,

    /// Include aggregate QPU statistics.
    pub include_qpu_statistics: bool,
}

impl Default for TelemetryRuntimeConfig {
    fn default() -> Self {
        Self {
            policy: TelemetryPolicy::LocalOnly,
            max_buffer_records: DEFAULT_BUFFER_CAPACITY,
            sampling_rate: 1.0,
            include_metrics: true,
            include_resources: true,
            include_decoder_statistics: true,
            include_qpu_statistics: true,
        }
    }
}

impl TelemetryRuntimeConfig {
    /// Construct runtime telemetry configuration from validated QEC config.
    pub fn from_qec_config(
        config: &QecConfig,
    ) -> Result<Self, TelemetryError> {
        config
            .telemetry
            .validate()
            .map_err(|error| TelemetryError::Configuration(error.to_string()))?;

        let policy = if !config.telemetry.enabled {
            TelemetryPolicy::Disabled
        } else if config.telemetry.export_remote {
            TelemetryPolicy::ExplicitRemote
        } else if config.telemetry.metrics
            || config.telemetry.events
            || config.telemetry.traces
        {
            TelemetryPolicy::LocalOnly
        } else {
            TelemetryPolicy::Aggregated
        };

        let runtime = Self {
            policy,
            max_buffer_records: DEFAULT_BUFFER_CAPACITY,
            sampling_rate: config.telemetry.sampling_rate,
            include_metrics: config.telemetry.metrics,
            include_resources: config.telemetry.include_resource_usage,
            include_decoder_statistics: config.telemetry.include_decoder_statistics,
            include_qpu_statistics: config.telemetry.include_qpu_statistics,
        };

        runtime.validate()?;

        Ok(runtime)
    }

    /// Validate telemetry configuration.
    pub fn validate(&self) -> Result<(), TelemetryError> {
        if self.max_buffer_records == 0
            || self.max_buffer_records > MAX_BUFFER_CAPACITY
        {
            return Err(TelemetryError::InvalidConfiguration(
                "max_buffer_records is outside the bounded telemetry range",
            ));
        }

        if !self.sampling_rate.is_finite()
            || !(0.0..=1.0).contains(&self.sampling_rate)
        {
            return Err(TelemetryError::InvalidConfiguration(
                "sampling_rate must be finite and between zero and one",
            ));
        }

        if self.policy.permits_remote()
            && !self.include_metrics
            && !self.include_resources
            && !self.include_decoder_statistics
            && !self.include_qpu_statistics
        {
            return Err(TelemetryError::InvalidConfiguration(
                "remote telemetry requires at least one aggregate payload",
            ));
        }

        Ok(())
    }
}

/* ========================================================================== */
/* Event classification                                                        */
/* ========================================================================== */

/// Aggregate telemetry event type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TelemetryEventKind {
    /// Aggregate metrics.
    MetricsSnapshot,

    /// Aggregate resources.
    ResourceSnapshot,

    /// Combined metrics/resource decoder summary.
    DecoderSummary,

    /// Aggregate QPU execution statistics.
    QpuSummary,
}

impl TelemetryEventKind {
    /// Stable machine-readable event name.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MetricsSnapshot => "metrics_snapshot",
            Self::ResourceSnapshot => "resource_snapshot",
            Self::DecoderSummary => "decoder_summary",
            Self::QpuSummary => "qpu_summary",
        }
    }
}

/* ========================================================================== */
/* Safe metrics representation                                                 */
/* ========================================================================== */

/// Aggregate metrics representation suitable for telemetry.
///
/// This intentionally contains counters and derived rates only.
///
/// It does not contain individual syndrome events, corrections, circuits,
/// measurement payloads, topology objects, or user data.
#[derive(Debug, Clone, PartialEq)]
pub struct TelemetryMetrics {
    pub decoder: DecoderId,
    pub backend: BackendKind,

    pub decode_operations: u64,
    pub decoder_success: u64,
    pub decoder_failure: u64,
    pub cancellation_count: u64,

    pub correction_count: u64,
    pub detection_event_count: u64,

    pub physical_error_count: u64,
    pub physical_error_opportunities: u64,

    pub logical_failure_count: u64,
    pub logical_error_opportunities: u64,

    pub matching_count: u64,
    pub decoder_iterations: u64,

    pub graph_nodes: u64,
    pub graph_edges: u64,

    pub qubit_count: u64,
    pub stabilizer_count: u64,
    pub measurement_rounds: u64,

    pub worker_count: u64,

    pub decoder_latency_nanos: u64,
    pub max_decoder_latency_nanos: u64,

    pub peak_memory: u64,
    pub current_memory: u64,

    pub compute_time_nanos: u64,

    pub physical_error_rate: Option<f64>,
    pub logical_error_rate: Option<f64>,

    pub decoder_success_rate: Option<f64>,
    pub decoder_failure_rate: Option<f64>,

    pub average_decoder_latency_nanos: Option<u64>,

    pub had_logical_failure: bool,
}

impl From<&MetricsSnapshot> for TelemetryMetrics {
    fn from(snapshot: &MetricsSnapshot) -> Self {
        Self {
            decoder: snapshot.decoder,
            backend: snapshot.backend,

            decode_operations: snapshot.decode_operations,
            decoder_success: snapshot.decoder_success,
            decoder_failure: snapshot.decoder_failure,
            cancellation_count: snapshot.cancellation_count,

            correction_count: snapshot.correction_count,
            detection_event_count: snapshot.detection_event_count,

            physical_error_count: snapshot.physical_error_count,
            physical_error_opportunities: snapshot.physical_error_opportunities,

            logical_failure_count: snapshot.logical_failure_count,
            logical_error_opportunities: snapshot.logical_error_opportunities,

            matching_count: snapshot.matching_count,
            decoder_iterations: snapshot.decoder_iterations,

            graph_nodes: snapshot.graph_nodes,
            graph_edges: snapshot.graph_edges,

            qubit_count: snapshot.qubit_count,
            stabilizer_count: snapshot.stabilizer_count,
            measurement_rounds: snapshot.measurement_rounds,

            worker_count: snapshot.worker_count,

            decoder_latency_nanos: duration_nanos(snapshot.decoder_latency),
            max_decoder_latency_nanos: duration_nanos(
                snapshot.max_decoder_latency,
            ),

            peak_memory: snapshot.peak_memory,
            current_memory: snapshot.current_memory,

            compute_time_nanos: duration_nanos(snapshot.compute_time),

            physical_error_rate: snapshot.physical_error_rate,
            logical_error_rate: snapshot.logical_error_rate,

            decoder_success_rate: snapshot.decoder_success_rate,
            decoder_failure_rate: snapshot.decoder_failure_rate,

            average_decoder_latency_nanos: snapshot
                .average_decoder_latency
                .map(duration_nanos),

            had_logical_failure: snapshot.had_logical_failure,
        }
    }
}

/* ========================================================================== */
/* Safe resource representation                                                */
/* ========================================================================== */

/// Aggregate resource representation suitable for telemetry.
///
/// This is observational only. It does not create or enforce resource
/// limits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TelemetryResources {
    pub allocated_bytes: u64,
    pub peak_bytes: u64,

    pub syndrome_events: u64,

    pub graph_nodes: u64,
    pub graph_edges: u64,

    pub decoder_iterations: u64,
    pub parallel_workers: usize,

    pub wall_time_nanos: u64,
    pub compute_time_nanos: u64,
}

impl From<&ResourceSnapshot> for TelemetryResources {
    fn from(snapshot: &ResourceSnapshot) -> Self {
        Self {
            allocated_bytes: snapshot.allocated_bytes,
            peak_bytes: snapshot.peak_bytes,

            syndrome_events: snapshot.syndrome_events,

            graph_nodes: snapshot.graph_nodes,
            graph_edges: snapshot.graph_edges,

            decoder_iterations: snapshot.decoder_iterations,
            parallel_workers: snapshot.parallel_workers,

            wall_time_nanos: duration_nanos(snapshot.wall_time),
            compute_time_nanos: duration_nanos(snapshot.compute_time),
        }
    }
}

/* ========================================================================== */
/* QPU telemetry                                                               */
/* ========================================================================== */

/// Aggregate QPU telemetry.
///
/// No raw hardware payloads are accepted.
#[derive(Debug, Clone, PartialEq)]
pub struct TelemetryQpuSummary {
    pub shots: u64,
    pub queue_time_nanos: u64,
    pub execution_time_nanos: u64,
    pub measurement_count: u64,

    pub readout_error_rate: Option<f64>,

    /// Optional caller-provided stable backend digest.
    ///
    /// This is deliberately a digest rather than a raw backend identifier.
    pub backend_digest: Option<u64>,

    /// Optional caller-provided stable calibration digest.
    pub calibration_digest: Option<u64>,
}

/* ========================================================================== */
/* Telemetry record                                                            */
/* ========================================================================== */

/// Bounded aggregate telemetry record.
#[derive(Debug, Clone, PartialEq)]
pub struct TelemetryRecord {
    /// Monotonic process-local sequence number.
    pub sequence: u64,

    /// Aggregate event type.
    pub kind: TelemetryEventKind,

    /// Aggregate metrics.
    pub metrics: Option<TelemetryMetrics>,

    /// Aggregate resource usage.
    pub resources: Option<TelemetryResources>,

    /// Aggregate QPU statistics.
    pub qpu: Option<TelemetryQpuSummary>,
}

/* ========================================================================== */
/* Remote exporter                                                             */
/* ========================================================================== */

/// Provider-neutral remote telemetry sink.
///
/// The exporter owns network/storage security. This module deliberately does
/// not handle URLs, sockets, TLS, credentials, tokens, or provider APIs.
pub trait TelemetryExporter: Send + Sync {
    /// Export one already-sanitized aggregate record.
    ///
    /// Export failures are observational and must never alter QEC correctness.
    fn export(&self, record: &TelemetryRecord) -> Result<(), String>;
}

/* ========================================================================== */
/* Collector statistics                                                        */
/* ========================================================================== */

/// Collector-level telemetry statistics.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TelemetrySnapshot {
    pub policy: TelemetryPolicy,

    pub retained_records: usize,

    pub emitted_records: u64,
    pub sampled_out_records: u64,
    pub dropped_records: u64,

    pub export_successes: u64,
    pub export_failures: u64,
}

/* ========================================================================== */
/* Collector                                                                    */
/* ========================================================================== */

/// Thread-safe bounded telemetry collector.
///
/// The collector is observational and non-authoritative.
///
/// ```text
/// decoder
///    │
///    ├──────────────► correctness result
///    │
///    └──────────────► MetricsSnapshot
///                         │
///                         ▼
///                   TelemetryCollector
///                         │
///                 ┌───────┴────────┐
///                 ▼                ▼
///              local            remote
///              buffer           exporter
/// ```
pub struct TelemetryCollector {
    config: TelemetryRuntimeConfig,

    records: Mutex<VecDeque<TelemetryRecord>>,

    sequence: AtomicU64,

    emitted_records: AtomicU64,
    sampled_out_records: AtomicU64,
    dropped_records: AtomicU64,

    export_successes: AtomicU64,
    export_failures: AtomicU64,

    exporter: Option<Arc<dyn TelemetryExporter>>,
}

impl fmt::Debug for TelemetryCollector {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TelemetryCollector")
            .field("config", &self.config)
            .field("snapshot", &self.snapshot())
            .finish_non_exhaustive()
    }
}

impl TelemetryCollector {
    /// Create a bounded telemetry collector.
    pub fn new(
        config: TelemetryRuntimeConfig,
    ) -> Result<Self, TelemetryError> {
        config.validate()?;

        Ok(Self {
            config,

            records: Mutex::new(VecDeque::with_capacity(
                config.max_buffer_records.min(1024),
            )),

            sequence: AtomicU64::new(0),

            emitted_records: AtomicU64::new(0),
            sampled_out_records: AtomicU64::new(0),
            dropped_records: AtomicU64::new(0),

            export_successes: AtomicU64::new(0),
            export_failures: AtomicU64::new(0),

            exporter: None,
        })
    }

    /// Create a collector with explicitly enabled remote export.
    pub fn with_exporter(
        config: TelemetryRuntimeConfig,
        exporter: Arc<dyn TelemetryExporter>,
    ) -> Result<Self, TelemetryError> {
        if !config.policy.permits_remote() {
            return Err(TelemetryError::RemoteExportNotPermitted);
        }

        let mut collector = Self::new(config)?;
        collector.exporter = Some(exporter);

        Ok(collector)
    }

    /// Construct telemetry from the central QEC configuration.
    pub fn from_qec_config(
        config: &QecConfig,
    ) -> Result<Self, TelemetryError> {
        Self::new(TelemetryRuntimeConfig::from_qec_config(config)?)
    }

    /// Return runtime configuration.
    pub const fn config(&self) -> TelemetryRuntimeConfig {
        self.config
    }

    /* ---------------------------------------------------------------------- */
    /* Recording                                                                */
    /* ---------------------------------------------------------------------- */

    /// Record aggregate metrics.
    pub fn record_metrics(
        &self,
        metrics: &MetricsSnapshot,
        authorization: Option<&CapabilityContext>,
    ) -> QecResult<()> {
        if !self.config.include_metrics {
            return Ok(());
        }

        self.record(
            TelemetryEventKind::MetricsSnapshot,
            Some(TelemetryMetrics::from(metrics)),
            None,
            None,
            authorization,
        )
    }

    /// Record aggregate resources.
    pub fn record_resources(
        &self,
        resources: &ResourceSnapshot,
        authorization: Option<&CapabilityContext>,
    ) -> QecResult<()> {
        if !self.config.include_resources {
            return Ok(());
        }

        self.record(
            TelemetryEventKind::ResourceSnapshot,
            None,
            Some(TelemetryResources::from(resources)),
            None,
            authorization,
        )
    }

    /// Record the preferred combined decoder execution summary.
    pub fn record_execution(
        &self,
        metrics: &MetricsSnapshot,
        resources: &ResourceSnapshot,
        authorization: Option<&CapabilityContext>,
    ) -> QecResult<()> {
        if !self.config.include_metrics
            && !self.config.include_resources
        {
            return Ok(());
        }

        self.record(
            TelemetryEventKind::DecoderSummary,
            if self.config.include_metrics {
                Some(TelemetryMetrics::from(metrics))
            } else {
                None
            },
            if self.config.include_resources {
                Some(TelemetryResources::from(resources))
            } else {
                None
            },
            None,
            authorization,
        )
    }

    /// Record aggregate QPU information.
    ///
    /// Backend and calibration identifiers are accepted only as already
    /// redacted stable digests.
    ///
    /// This prevents telemetry from becoming a secret-handling boundary.
    pub fn record_qpu_summary(
        &self,
        shots: u64,
        queue_time: Duration,
        execution_time: Duration,
        measurement_count: u64,
        readout_error_rate: Option<f64>,
        backend_digest: Option<u64>,
        calibration_digest: Option<u64>,
        authorization: Option<&CapabilityContext>,
    ) -> QecResult<()> {
        if !self.config.include_qpu_statistics {
            return Ok(());
        }

        if let Some(rate) = readout_error_rate {
            if !rate.is_finite()
                || !(0.0..=1.0).contains(&rate)
            {
                return Err(QecError::invalid_input(
                    "telemetry readout_error_rate must be finite and between 0 and 1",
                ));
            }
        }

        let summary = TelemetryQpuSummary {
            shots,
            queue_time_nanos: duration_nanos(queue_time),
            execution_time_nanos: duration_nanos(execution_time),
            measurement_count,
            readout_error_rate,
            backend_digest,
            calibration_digest,
        };

        self.record(
            TelemetryEventKind::QpuSummary,
            None,
            None,
            Some(summary),
            authorization,
        )
    }

    /* ---------------------------------------------------------------------- */
    /* Local state                                                              */
    /* ---------------------------------------------------------------------- */

    /// Return a bounded copy of retained telemetry.
    pub fn records(&self) -> Vec<TelemetryRecord> {
        match self.records.lock() {
            Ok(records) => records.iter().cloned().collect(),
            Err(poisoned) => poisoned.into_inner().iter().cloned().collect(),
        }
    }

    /// Return collector statistics.
    pub fn snapshot(&self) -> TelemetrySnapshot {
        let retained_records = match self.records.lock() {
            Ok(records) => records.len(),
            Err(poisoned) => poisoned.into_inner().len(),
        };

        TelemetrySnapshot {
            policy: self.config.policy,

            retained_records,

            emitted_records: self.emitted_records.load(Ordering::Relaxed),

            sampled_out_records: self
                .sampled_out_records
                .load(Ordering::Relaxed),

            dropped_records: self.dropped_records.load(Ordering::Relaxed),

            export_successes: self
                .export_successes
                .load(Ordering::Relaxed),

            export_failures: self
                .export_failures
                .load(Ordering::Relaxed),
        }
    }

    /// Clear locally retained telemetry.
    ///
    /// This cannot affect QEC correctness state.
    pub fn clear(&self) {
        match self.records.lock() {
            Ok(mut records) => records.clear(),
            Err(poisoned) => poisoned.into_inner().clear(),
        }
    }

    /* ---------------------------------------------------------------------- */
    /* Internal emission                                                       */
    /* ---------------------------------------------------------------------- */

    fn record(
        &self,
        kind: TelemetryEventKind,
        metrics: Option<TelemetryMetrics>,
        resources: Option<TelemetryResources>,
        qpu: Option<TelemetryQpuSummary>,
        authorization: Option<&CapabilityContext>,
    ) -> QecResult<()> {
        if !self.config.policy.enabled() {
            return Ok(());
        }

        /*
         * Authorization is checked before retention/export so a remote
         * telemetry record can never silently bypass the capability boundary.
         */
        self.authorize_emission(authorization)?;

        let sequence = self
            .sequence
            .fetch_add(1, Ordering::Relaxed)
            .saturating_add(1);

        if !sample_sequence(sequence, self.config.sampling_rate) {
            self.sampled_out_records
                .fetch_add(1, Ordering::Relaxed);

            return Ok(());
        }

        let record = TelemetryRecord {
            sequence,
            kind,
            metrics,
            resources,
            qpu,
        };

        self.retain(record.clone());

        self.emitted_records
            .fetch_add(1, Ordering::Relaxed);

        /*
         * Remote export occurs after bounded local retention.
         *
         * Export failure is deliberately swallowed at the telemetry boundary.
         * The caller may observe the export-failure counter, but QEC correctness
         * is never affected.
         */
        if self.config.policy.permits_remote() {
            if let Some(exporter) = &self.exporter {
                match exporter.export(&record) {
                    Ok(()) => {
                        self.export_successes
                            .fetch_add(1, Ordering::Relaxed);
                    }

                    Err(_) => {
                        self.export_failures
                            .fetch_add(1, Ordering::Relaxed);
                    }
                }
            }
        }

        Ok(())
    }

    /// Insert one record into the bounded retention buffer.
    fn retain(&self, record: TelemetryRecord) {
        match self.records.lock() {
            Ok(mut records) => {
                Self::retain_locked(
                    &mut records,
                    record,
                    self.config.max_buffer_records,
                    &self.dropped_records,
                );
            }

            Err(poisoned) => {
                let mut records = poisoned.into_inner();

                Self::retain_locked(
                    &mut records,
                    record,
                    self.config.max_buffer_records,
                    &self.dropped_records,
                );
            }
        }
    }

    fn retain_locked(
        records: &mut VecDeque<TelemetryRecord>,
        record: TelemetryRecord,
        capacity: usize,
        dropped_records: &AtomicU64,
    ) {
        if records.len() >= capacity {
            let _ = records.pop_front();

            dropped_records.fetch_add(1, Ordering::Relaxed);
        }

        records.push_back(record);
    }

    /// Enforce remote telemetry authorization.
    ///
    /// Local telemetry does not require a capability because it remains inside
    /// the QEC process.
    ///
    /// Remote telemetry requires:
    ///
    /// `Capability::EmitTelemetry`
    fn authorize_emission(
        &self,
        authorization: Option<&CapabilityContext>,
    ) -> QecResult<()> {
        if !self.config.policy.permits_remote() {
            return Ok(());
        }

        if self.exporter.is_none() {
            return Err(QecError::unsupported(
                "remote telemetry",
                "explicit remote telemetry requires a configured exporter",
            ));
        }

        let context = authorization.ok_or_else(|| {
            QecError::unsupported(
                "telemetry capability",
                "remote telemetry requires explicit capability authorization",
            )
        })?;

        /*
         * Telemetry emission itself is not a QPU operation. The exporter
         * remains an external service boundary and therefore requires the
         * dedicated EmitTelemetry capability.
         */
        let request = ResourceRequest::default();

        let now = unix_seconds();

        context
            .authorize(
                Capability::EmitTelemetry,
                ExecutionBackend::Cpu,
                &request,
                now,
            )
            .map_err(|error| {
                QecError::unsupported(
                    "telemetry capability",
                    format!("telemetry emission denied: {error}"),
                )
            })?;

        Ok(())
    }
}

/* ========================================================================== */
/* Errors                                                                     */
/* ========================================================================== */

/// Errors encountered while constructing or configuring telemetry.
///
/// Runtime emission uses the canonical `QecError` boundary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TelemetryError {
    /// Runtime configuration is invalid.
    InvalidConfiguration(&'static str),

    /// Persisted QEC configuration contains invalid telemetry settings.
    Configuration(String),

    /// Remote exporter was requested without explicit remote policy.
    RemoteExportNotPermitted,
}

impl fmt::Display for TelemetryError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidConfiguration(message) => {
                formatter.write_str(message)
            }

            Self::Configuration(message) => {
                write!(
                    formatter,
                    "invalid QEC telemetry configuration: {message}"
                )
            }

            Self::RemoteExportNotPermitted => {
                formatter.write_str(
                    "remote telemetry is not permitted by policy",
                )
            }
        }
    }
}

impl std::error::Error for TelemetryError {}

/* ========================================================================== */
/* Deterministic helpers                                                       */
/* ========================================================================== */

/// Convert a duration to a bounded nanosecond count.
fn duration_nanos(duration: Duration) -> u64 {
    duration
        .as_nanos()
        .min(u64::MAX as u128) as u64
}

/// Current Unix timestamp.
///
/// Used only for capability expiration checks.
///
/// The timestamp is never stored in telemetry.
fn unix_seconds() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or(0)
}

/// Deterministic sampling.
///
/// Sampling does not use thread-local/global RNG state. Consequently,
/// sampling decisions are stable for a given sequence number.
fn sample_sequence(sequence: u64, rate: f64) -> bool {
    if rate <= 0.0 {
        return false;
    }

    if rate >= 1.0 {
        return true;
    }

    let hash = stable_digest_u64(sequence);

    let fraction = (hash as f64) / (u64::MAX as f64);

    fraction < rate
}

/// Stable FNV-1a digest for deterministic sampling.
///
/// This is not cryptographic.
fn stable_digest_u64(value: u64) -> u64 {
    let mut hash = 0xcbf29ce484222325u64;

    for byte in value.to_le_bytes() {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }

    hash
}

/* ========================================================================== */
/* Tests                                                                      */
/* ========================================================================== */

#[cfg(test)]
mod tests {
    use super::*;

    use std::sync::atomic::{AtomicU64, Ordering};
    use std::thread;

    struct TestExporter {
        calls: AtomicU64,
        failures: AtomicU64,
    }

    impl TestExporter {
        fn successful() -> Self {
            Self {
                calls: AtomicU64::new(0),
                failures: AtomicU64::new(0),
            }
        }

        fn failing() -> Self {
            Self {
                calls: AtomicU64::new(0),
                failures: AtomicU64::new(0),
            }
        }
    }

    impl TelemetryExporter for TestExporter {
        fn export(
            &self,
            _record: &TelemetryRecord,
        ) -> Result<(), String> {
            self.calls.fetch_add(1, Ordering::Relaxed);

            if self.failures.load(Ordering::Relaxed) > 0 {
                Err("simulated exporter failure".to_owned())
            } else {
                Ok(())
            }
        }
    }

    fn configuration(
        policy: TelemetryPolicy,
    ) -> TelemetryRuntimeConfig {
        TelemetryRuntimeConfig {
            policy,
            ..TelemetryRuntimeConfig::default()
        }
    }

    fn empty_metrics() -> MetricsSnapshot {
        MetricsSnapshot::empty(
            DecoderId::UnionFind,
            BackendKind::Cpu,
        )
    }

    fn empty_resources() -> ResourceSnapshot {
        ResourceSnapshot {
            allocated_bytes: 100,
            peak_bytes: 200,
            syndrome_events: 10,
            graph_nodes: 20,
            graph_edges: 30,
            decoder_iterations: 40,
            parallel_workers: 2,
            wall_time: Duration::from_millis(5),
            compute_time: Duration::from_millis(4),
        }
    }

    #[test]
    fn telemetry_is_bounded() {
        let mut config =
            configuration(TelemetryPolicy::LocalOnly);

        config.max_buffer_records = 2;

        let collector =
            TelemetryCollector::new(config).unwrap();

        let metrics = empty_metrics();

        collector
            .record_metrics(&metrics, None)
            .unwrap();

        collector
            .record_metrics(&metrics, None)
            .unwrap();

        collector
            .record_metrics(&metrics, None)
            .unwrap();

        let snapshot = collector.snapshot();

        assert_eq!(snapshot.retained_records, 2);
        assert_eq!(snapshot.dropped_records, 1);
        assert_eq!(snapshot.emitted_records, 3);
    }

    #[test]
    fn disabled_telemetry_retains_nothing() {
        let collector =
            TelemetryCollector::new(
                configuration(TelemetryPolicy::Disabled),
            )
            .unwrap();

        let metrics = empty_metrics();

        collector
            .record_metrics(&metrics, None)
            .unwrap();

        let snapshot = collector.snapshot();

        assert_eq!(snapshot.retained_records, 0);
        assert_eq!(snapshot.emitted_records, 0);
    }

    #[test]
    fn invalid_capacity_is_rejected() {
        let mut config =
            configuration(TelemetryPolicy::LocalOnly);

        config.max_buffer_records = 0;

        assert!(
            TelemetryCollector::new(config).is_err()
        );

        config.max_buffer_records =
            MAX_BUFFER_CAPACITY + 1;

        assert!(
            TelemetryCollector::new(config).is_err()
        );
    }

    #[test]
    fn invalid_sampling_rate_is_rejected() {
        let mut config =
            configuration(TelemetryPolicy::LocalOnly);

        config.sampling_rate = f64::NAN;

        assert!(
            TelemetryCollector::new(config).is_err()
        );

        config.sampling_rate = 1.1;

        assert!(
            TelemetryCollector::new(config).is_err()
        );

        config.sampling_rate = -0.1;

        assert!(
            TelemetryCollector::new(config).is_err()
        );
    }

    #[test]
    fn zero_sampling_retains_nothing() {
        let mut config =
            configuration(TelemetryPolicy::LocalOnly);

        config.sampling_rate = 0.0;

        let collector =
            TelemetryCollector::new(config).unwrap();

        let metrics = empty_metrics();

        for _ in 0..100 {
            collector
                .record_metrics(&metrics, None)
                .unwrap();
        }

        let snapshot = collector.snapshot();

        assert_eq!(snapshot.retained_records, 0);
        assert_eq!(snapshot.emitted_records, 0);
        assert_eq!(snapshot.sampled_out_records, 100);
    }

    #[test]
    fn full_sampling_retains_every_record() {
        let mut config =
            configuration(TelemetryPolicy::LocalOnly);

        config.sampling_rate = 1.0;

        let collector =
            TelemetryCollector::new(config).unwrap();

        let metrics = empty_metrics();

        for _ in 0..100 {
            collector
                .record_metrics(&metrics, None)
                .unwrap();
        }

        let snapshot = collector.snapshot();

        assert_eq!(snapshot.retained_records, 100);
        assert_eq!(snapshot.emitted_records, 100);
        assert_eq!(snapshot.sampled_out_records, 0);
    }

    #[test]
    fn deterministic_sampling_is_stable() {
        let first: Vec<bool> = (1..10_000)
            .map(|sequence| {
                sample_sequence(sequence, 0.5)
            })
            .collect();

        let second: Vec<bool> = (1..10_000)
            .map(|sequence| {
                sample_sequence(sequence, 0.5)
            })
            .collect();

        assert_eq!(first, second);
    }

    #[test]
    fn sampling_boundaries_are_correct() {
        assert!(!sample_sequence(1, 0.0));
        assert!(!sample_sequence(2, 0.0));

        assert!(sample_sequence(1, 1.0));
        assert!(sample_sequence(2, 1.0));
    }

    #[test]
    fn resource_conversion_is_aggregate_only() {
        let resources = empty_resources();

        let telemetry =
            TelemetryResources::from(&resources);

        assert_eq!(
            telemetry.allocated_bytes,
            100
        );

        assert_eq!(
            telemetry.peak_bytes,
            200
        );

        assert_eq!(
            telemetry.syndrome_events,
            10
        );

        assert_eq!(
            telemetry.graph_nodes,
            20
        );

        assert_eq!(
            telemetry.graph_edges,
            30
        );
    }

    #[test]
    fn metrics_conversion_is_aggregate_only() {
        let metrics = empty_metrics();

        let telemetry =
            TelemetryMetrics::from(&metrics);

        assert_eq!(
            telemetry.decode_operations,
            0
        );

        assert_eq!(
            telemetry.detection_event_count,
            0
        );

        assert!(
            !telemetry.had_logical_failure
        );
    }

    #[test]
    fn execution_summary_combines_metrics_and_resources() {
        let collector =
            TelemetryCollector::new(
                configuration(
                    TelemetryPolicy::LocalOnly,
                ),
            )
            .unwrap();

        let metrics = empty_metrics();
        let resources = empty_resources();

        collector
            .record_execution(
                &metrics,
                &resources,
                None,
            )
            .unwrap();

        let records = collector.records();

        assert_eq!(records.len(), 1);
        assert_eq!(
            records[0].kind,
            TelemetryEventKind::DecoderSummary
        );

        assert!(
            records[0].metrics.is_some()
        );

        assert!(
            records[0].resources.is_some()
        );
    }

    #[test]
    fn remote_export_requires_explicit_policy() {
        let exporter = Arc::new(
            TestExporter::successful(),
        );

        assert!(
            TelemetryCollector::with_exporter(
                configuration(
                    TelemetryPolicy::LocalOnly,
                ),
                exporter,
            )
            .is_err()
        );
    }

    #[test]
    fn remote_export_requires_exporter() {
        let collector =
            TelemetryCollector::new(
                configuration(
                    TelemetryPolicy::ExplicitRemote,
                ),
            )
            .unwrap();

        let result =
            collector.record_metrics(
                &empty_metrics(),
                None,
            );

        assert!(result.is_err());
    }

    #[test]
    fn remote_export_requires_capability_context() {
        let exporter = Arc::new(
            TestExporter::successful(),
        );

        let collector =
            TelemetryCollector::with_exporter(
                configuration(
                    TelemetryPolicy::ExplicitRemote,
                ),
                exporter.clone(),
            )
            .unwrap();

        let result =
            collector.record_metrics(
                &empty_metrics(),
                None,
            );

        assert!(result.is_err());

        assert_eq!(
            exporter.calls.load(
                Ordering::Relaxed
            ),
            0
        );
    }

    #[test]
    fn remote_export_does_not_store_raw_identifiers() {
        let summary = TelemetryQpuSummary {
            shots: 10,
            queue_time_nanos: 100,
            execution_time_nanos: 200,
            measurement_count: 20,
            readout_error_rate: Some(0.01),
            backend_digest: Some(1234),
            calibration_digest: Some(5678),
        };

        let debug =
            format!("{summary:?}");

        assert!(
            !debug.contains("backend-secret")
        );

        assert!(
            !debug.contains("calibration-secret")
        );
    }

    #[test]
    fn qpu_invalid_readout_rate_is_rejected() {
        let collector =
            TelemetryCollector::new(
                configuration(
                    TelemetryPolicy::LocalOnly,
                ),
            )
            .unwrap();

        assert!(
            collector
                .record_qpu_summary(
                    10,
                    Duration::ZERO,
                    Duration::ZERO,
                    10,
                    Some(f64::NAN),
                    None,
                    None,
                    None,
                )
                .is_err()
        );

        assert!(
            collector
                .record_qpu_summary(
                    10,
                    Duration::ZERO,
                    Duration::ZERO,
                    10,
                    Some(1.1),
                    None,
                    None,
                    None,
                )
                .is_err()
        );

        assert!(
            collector
                .record_qpu_summary(
                    10,
                    Duration::ZERO,
                    Duration::ZERO,
                    10,
                    Some(-0.1),
                    None,
                    None,
                    None,
                )
                .is_err()
        );
    }

    #[test]
    fn clear_only_affects_telemetry_retention() {
        let collector =
            TelemetryCollector::new(
                configuration(
                    TelemetryPolicy::LocalOnly,
                ),
            )
            .unwrap();

        collector
            .record_metrics(
                &empty_metrics(),
                None,
            )
            .unwrap();

        assert_eq!(
            collector.snapshot().retained_records,
            1
        );

        collector.clear();

        assert_eq!(
            collector.snapshot().retained_records,
            0
        );

        assert_eq!(
            collector.snapshot().emitted_records,
            1
        );
    }

    #[test]
    fn concurrent_recording_remains_bounded() {
        let mut config =
            configuration(TelemetryPolicy::LocalOnly);

        config.max_buffer_records = 128;

        let collector = Arc::new(
            TelemetryCollector::new(config)
                .unwrap(),
        );

        let metrics = Arc::new(empty_metrics());

        let mut handles = Vec::new();

        for _ in 0..8 {
            let collector =
                Arc::clone(&collector);

            let metrics =
                Arc::clone(&metrics);

            handles.push(
                thread::spawn(move || {
                    for _ in 0..1_000 {
                        collector
                            .record_metrics(
                                &metrics,
                                None,
                            )
                            .unwrap();
                    }
                }),
            );
        }

        for handle in handles {
            handle.join().unwrap();
        }

        let snapshot =
            collector.snapshot();

        assert!(
            snapshot.retained_records
                <= 128
        );

        assert_eq!(
            snapshot.emitted_records,
            8_000
        );

        assert_eq!(
            snapshot.dropped_records,
            8_000 - 128
        );
    }

    #[test]
    fn sequence_numbers_are_monotonic() {
        let collector =
            TelemetryCollector::new(
                configuration(
                    TelemetryPolicy::LocalOnly,
                ),
            )
            .unwrap();

        let metrics = empty_metrics();

        for _ in 0..10 {
            collector
                .record_metrics(
                    &metrics,
                    None,
                )
                .unwrap();
        }

        let records =
            collector.records();

        for pair in records.windows(2) {
            assert!(
                pair[0].sequence
                    < pair[1].sequence
            );
        }
    }

    #[test]
    fn telemetry_contains_no_raw_qec_payload() {
        let metrics = empty_metrics();

        let telemetry =
            TelemetryMetrics::from(
                &metrics,
            );

        let debug =
            format!("{telemetry:?}");

        assert!(
            !debug.contains("circuit")
        );

        assert!(
            !debug.contains("credential")
        );

        assert!(
            !debug.contains("private_key")
        );

        assert!(
            !debug.contains("raw_measurement")
        );

        assert!(
            !debug.contains("syndrome_stream")
        );
    }

    #[test]
    fn duration_conversion_saturates() {
        let duration =
            Duration::from_secs(u64::MAX);

        let nanos =
            duration_nanos(duration);

        assert_eq!(
            nanos,
            u64::MAX
        );
    }

    #[test]
    fn policy_names_are_stable() {
        assert_eq!(
            TelemetryPolicy::Disabled.as_str(),
            "disabled"
        );

        assert_eq!(
            TelemetryPolicy::LocalOnly.as_str(),
            "local_only"
        );

        assert_eq!(
            TelemetryPolicy::Aggregated.as_str(),
            "aggregated"
        );

        assert_eq!(
            TelemetryPolicy::ExplicitRemote.as_str(),
            "explicit_remote"
        );
    }
}