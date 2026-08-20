//! Production telemetry boundary for Zamani Quantum Error Correction.
//!
//! Telemetry is observational only. It must never become part of the QEC
//! correctness path and must never receive:
//!
//! - raw syndrome streams;
//! - quantum circuits;
//! - raw measurement data;
//! - QPU credentials;
//! - private keys;
//! - device secrets;
//! - proprietary topology;
//! - unbounded per-event payloads.
//!
//! The telemetry boundary is:
//!
//! - disabled/local/aggregated/explicit-remote;
//! - bounded in memory;
//! - deterministic when sampling is enabled;
//! - capability-gated for remote emission;
//! - aggregate-only;
//! - resource-aware through `ResourceSnapshot`;
//! - metrics-aware through `MetricsSnapshot`;
//! - fail-closed for remote export;
//! - independent of decoder correctness.
//!
//! ```text
//! QEC execution
//!      |
//!      +----------------------> correctness path
//!      |
//!      +----------------------> metrics
//!                                  |
//!                                  v
//!                            telemetry policy
//!                                  |
//!                 +----------------+----------------+
//!                 |                |                |
//!              disabled         local/agg       explicit remote
//!                                  |                |
//!                                  v                v
//!                             bounded buffer   authorized exporter
//! ```
//!
//! Remote telemetry is deliberately provider-neutral. This module does not
//! perform network I/O or handle credentials.

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

/// Default bounded telemetry buffer capacity.
pub const DEFAULT_BUFFER_CAPACITY: usize = 4_096;

/// Absolute safety ceiling for telemetry retention.
///
/// Telemetry must never become an unbounded-memory vector.
pub const MAX_BUFFER_CAPACITY: usize = 1_000_000;

/* -------------------------------------------------------------------------- */
/* Telemetry policy                                                           */
/* -------------------------------------------------------------------------- */

/// Controls what the telemetry subsystem is allowed to do.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TelemetryPolicy {
    /// Do not retain or export telemetry.
    Disabled,

    /// Retain aggregate telemetry locally.
    ///
    /// Remote export is prohibited.
    LocalOnly,

    /// Retain aggregate telemetry locally.
    ///
    /// The representation is restricted to aggregate information and cannot
    /// contain arbitrary user-supplied attributes.
    Aggregated,

    /// Permit remote aggregate telemetry.
    ///
    /// Remote emission additionally requires `Capability::EmitTelemetry`.
    ExplicitRemote,
}

impl TelemetryPolicy {
    /// Returns whether telemetry collection is enabled.
    pub const fn enabled(self) -> bool {
        !matches!(self, Self::Disabled)
    }

    /// Returns whether remote export is allowed by policy.
    pub const fn permits_remote(self) -> bool {
        matches!(self, Self::ExplicitRemote)
    }

    /// Stable machine-readable policy identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Disabled => "disabled",
            Self::LocalOnly => "local_only",
            Self::Aggregated => "aggregated",
            Self::ExplicitRemote => "explicit_remote",
        }
    }
}

/* -------------------------------------------------------------------------- */
/* Runtime configuration                                                       */
/* -------------------------------------------------------------------------- */

/// Runtime telemetry configuration.
///
/// This is deliberately separate from `configuration::TelemetryConfig`.
///
/// `QecConfig` remains the persistent source of truth while this type adds
/// runtime-only controls such as bounded local retention.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TelemetryRuntimeConfig {
    /// Telemetry policy.
    pub policy: TelemetryPolicy,

    /// Maximum number of records retained in memory.
    pub max_buffer_records: usize,

    /// Deterministic sampling probability.
    pub sampling_rate: f64,

    /// Include aggregate decoder metrics.
    pub include_metrics: bool,

    /// Include aggregate resource information.
    pub include_resources: bool,

    /// Include decoder-specific aggregate information.
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
    /// Construct runtime telemetry policy from validated `QecConfig`.
    pub fn from_qec_config(
        config: &QecConfig,
    ) -> Result<Self, TelemetryError> {
        config
            .telemetry
            .validate()
            .map_err(|error| {
                TelemetryError::Configuration(error.to_string())
            })?;

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
            include_decoder_statistics:
                config.telemetry.include_decoder_statistics,
            include_qpu_statistics:
                config.telemetry.include_qpu_statistics,
        };

        runtime.validate()?;

        Ok(runtime)
    }

    /// Validate runtime telemetry policy.
    pub fn validate(&self) -> Result<(), TelemetryError> {
        if self.max_buffer_records == 0
            || self.max_buffer_records > MAX_BUFFER_CAPACITY
        {
            return Err(
                TelemetryError::InvalidConfiguration(
                    "max_buffer_records is outside the bounded telemetry range",
                ),
            );
        }

        if !self.sampling_rate.is_finite()
            || !(0.0..=1.0).contains(&self.sampling_rate)
        {
            return Err(
                TelemetryError::InvalidConfiguration(
                    "sampling_rate must be finite and between zero and one",
                ),
            );
        }

        if self.policy.permits_remote()
            && !self.include_metrics
            && !self.include_resources
            && !self.include_decoder_statistics
            && !self.include_qpu_statistics
        {
            return Err(
                TelemetryError::InvalidConfiguration(
                    "remote telemetry requires at least one aggregate payload",
                ),
            );
        }

        Ok(())
    }
}

/* -------------------------------------------------------------------------- */
/* Telemetry event classification                                              */
/* -------------------------------------------------------------------------- */

/// Aggregate telemetry event type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TelemetryEventKind {
    /// Aggregate metrics snapshot.
    MetricsSnapshot,

    /// Aggregate resource snapshot.
    ResourceSnapshot,

    /// Combined decoder execution summary.
    DecoderSummary,

    /// Aggregate QPU execution summary.
    QpuSummary,

    /// Non-sensitive lifecycle event.
    Lifecycle,
}

impl TelemetryEventKind {
    /// Stable machine-readable event name.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MetricsSnapshot => "metrics_snapshot",
            Self::ResourceSnapshot => "resource_snapshot",
            Self::DecoderSummary => "decoder_summary",
            Self::QpuSummary => "qpu_summary",
            Self::Lifecycle => "lifecycle",
        }
    }
}

/* -------------------------------------------------------------------------- */
/* Safe metrics representation                                                 */
/* -------------------------------------------------------------------------- */

/// Aggregate metrics representation suitable for telemetry.
///
/// This deliberately contains counters and derived rates only.
///
/// It does not contain individual syndrome events or decoder operations.
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
            physical_error_opportunities:
                snapshot.physical_error_opportunities,

            logical_failure_count: snapshot.logical_failure_count,
            logical_error_opportunities:
                snapshot.logical_error_opportunities,

            matching_count: snapshot.matching_count,
            decoder_iterations: snapshot.decoder_iterations,

            graph_nodes: snapshot.graph_nodes,
            graph_edges: snapshot.graph_edges,

            qubit_count: snapshot.qubit_count,
            stabilizer_count: snapshot.stabilizer_count,
            measurement_rounds: snapshot.measurement_rounds,

            worker_count: snapshot.worker_count,

            decoder_latency_nanos:
                duration_nanos(snapshot.decoder_latency),

            max_decoder_latency_nanos:
                duration_nanos(snapshot.max_decoder_latency),

            peak_memory: snapshot.peak_memory,
            current_memory: snapshot.current_memory,

            compute_time_nanos:
                duration_nanos(snapshot.compute_time),

            physical_error_rate:
                snapshot.physical_error_rate,

            logical_error_rate:
                snapshot.logical_error_rate,

            decoder_success_rate:
                snapshot.decoder_success_rate,

            decoder_failure_rate:
                snapshot.decoder_failure_rate,

            average_decoder_latency_nanos:
                snapshot
                    .average_decoder_latency
                    .map(duration_nanos),

            had_logical_failure:
                snapshot.had_logical_failure,
        }
    }
}

/* -------------------------------------------------------------------------- */
/* Safe resource representation                                                */
/* -------------------------------------------------------------------------- */

/// Aggregate resource representation suitable for telemetry.
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

            decoder_iterations:
                snapshot.decoder_iterations,

            parallel_workers:
                snapshot.parallel_workers,

            wall_time_nanos:
                duration_nanos(snapshot.wall_time),

            compute_time_nanos:
                duration_nanos(snapshot.compute_time),
        }
    }
}

/* -------------------------------------------------------------------------- */
/* Safe QPU representation                                                      */
/* -------------------------------------------------------------------------- */

/// Aggregate QPU telemetry.
///
/// The type intentionally has no field for:
///
/// - circuits;
/// - raw measurements;
/// - topology;
/// - credentials;
/// - API tokens;
/// - private keys;
/// - device secrets.
///
/// Backend and calibration identifiers are reduced to stable digests.
#[derive(Debug, Clone, PartialEq)]
pub struct TelemetryQpuSummary {
    pub shots: u64,
    pub queue_time_nanos: u64,
    pub execution_time_nanos: u64,
    pub measurement_count: u64,

    pub readout_error_rate: Option<f64>,

    pub backend_digest: Option<u64>,
    pub calibration_digest: Option<u64>,
}

/* -------------------------------------------------------------------------- */
/* Telemetry record                                                            */
/* -------------------------------------------------------------------------- */

/// Bounded aggregate telemetry record.
#[derive(Debug, Clone, PartialEq)]
pub struct TelemetryRecord {
    /// Monotonic process-local sequence number.
    pub sequence: u64,

    /// Aggregate event kind.
    pub kind: TelemetryEventKind,

    /// Aggregate metrics.
    pub metrics: Option<TelemetryMetrics>,

    /// Aggregate resource usage.
    pub resources: Option<TelemetryResources>,

    /// Aggregate QPU statistics.
    pub qpu: Option<TelemetryQpuSummary>,
}

/* -------------------------------------------------------------------------- */
/* Remote exporter                                                             */
/* -------------------------------------------------------------------------- */

/// Remote telemetry sink.
///
/// The implementation is responsible for network/storage I/O.
///
/// The QEC telemetry module deliberately does not handle:
///
/// - URLs;
/// - sockets;
/// - authentication tokens;
/// - TLS state;
/// - provider-specific APIs.
pub trait TelemetryExporter: Send + Sync {
    /// Export one aggregate telemetry record.
    ///
    /// Export failures never alter QEC correctness.
    fn export(
        &self,
        record: &TelemetryRecord,
    ) -> Result<(), String>;
}

/* -------------------------------------------------------------------------- */
/* Collector snapshot                                                          */
/* -------------------------------------------------------------------------- */

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

/* -------------------------------------------------------------------------- */
/* Collector                                                                   */
/* -------------------------------------------------------------------------- */

/// Thread-safe bounded telemetry collector.
///
/// Telemetry is non-authoritative:
///
/// ```text
/// decoder result
///      |
//!      +----> correctness
//!      |
//!      +----> metrics
//!               |
//!               +----> telemetry
//! ```
///
/// A telemetry failure must never alter a decoder result.
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
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
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

            records: Mutex::new(
                VecDeque::with_capacity(
                    config.max_buffer_records.min(1024),
                ),
            ),

            sequence: AtomicU64::new(0),

            emitted_records: AtomicU64::new(0),
            sampled_out_records: AtomicU64::new(0),
            dropped_records: AtomicU64::new(0),

            export_successes: AtomicU64::new(0),
            export_failures: AtomicU64::new(0),

            exporter: None,
        })
    }

    /// Create a collector with an explicitly authorized remote-export
    /// capability boundary.
    ///
    /// The exporter itself remains responsible for transport security.
    pub fn with_exporter(
        config: TelemetryRuntimeConfig,
        exporter: Arc<dyn TelemetryExporter>,
    ) -> Result<Self, TelemetryError> {
        if !config.policy.permits_remote() {
            return Err(
                TelemetryError::RemoteExportNotPermitted,
            );
        }

        let mut collector = Self::new(config)?;
        collector.exporter = Some(exporter);

        Ok(collector)
    }

    /// Create a collector from the repository's validated QEC configuration.
    pub fn from_qec_config(
        config: &QecConfig,
    ) -> Result<Self, TelemetryError> {
        Self::new(
            TelemetryRuntimeConfig::from_qec_config(config)?,
        )
    }

    /// Return the runtime configuration.
    pub const fn config(&self) -> TelemetryRuntimeConfig {
        self.config
    }

    /* ---------------------------------------------------------------------- */
    /* Recording APIs                                                          */
    /* ---------------------------------------------------------------------- */

    /// Record an aggregate metrics snapshot.
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

    /// Record an aggregate resource snapshot.
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

    /// Record a combined decoder execution summary.
    ///
    /// This is the preferred API for the actual QEC execution path because it
    /// joins metrics and resource accounting without storing individual
    /// operations.
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

    /// Record aggregate QPU execution information.
    ///
    /// Only aggregate values are accepted.
    pub fn record_qpu_summary(
        &self,
        shots: u64,
        queue_time: Duration,
        execution_time: Duration,
        measurement_count: u64,
        readout_error_rate: Option<f64>,
        backend_id: Option<&str>,
        calibration_id: Option<&str>,
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
                    "telemetry readout_error_rate must be finite \
                     and between 0 and 1",
                ));
            }
        }

        let summary = TelemetryQpuSummary {
            shots,

            queue_time_nanos:
                duration_nanos(queue_time),

            execution_time_nanos:
                duration_nanos(execution_time),

            measurement_count,

            readout_error_rate,

            backend_digest:
                backend_id.map(stable_digest),

            calibration_digest:
                calibration_id.map(stable_digest),
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
    /* Local state                                                             */
    /* ---------------------------------------------------------------------- */

    /// Return a bounded copy of locally retained records.
    pub fn records(&self) -> Vec<TelemetryRecord> {
        match self.records.lock() {
            Ok(records) => records.iter().cloned().collect(),

            Err(poisoned) => {
                poisoned
                    .into_inner()
                    .iter()
                    .cloned()
                    .collect()
            }
        }
    }

    /// Return collector statistics.
    pub fn snapshot(&self) -> TelemetrySnapshot {
        let retained_records =
            match self.records.lock() {
                Ok(records) => records.len(),

                Err(poisoned) => {
                    poisoned.into_inner().len()
                }
            };

        TelemetrySnapshot {
            policy: self.config.policy,

            retained_records,

            emitted_records:
                self.emitted_records
                    .load(Ordering::Relaxed),

            sampled_out_records:
                self.sampled_out_records
                    .load(Ordering::Relaxed),

            dropped_records:
                self.dropped_records
                    .load(Ordering::Relaxed),

            export_successes:
                self.export_successes
                    .load(Ordering::Relaxed),

            export_failures:
                self.export_failures
                    .load(Ordering::Relaxed),
        }
    }

    /// Clear local telemetry records.
    ///
    /// This operation cannot affect QEC state.
    pub fn clear(&self) {
        match self.records.lock() {
            Ok(mut records) => records.clear(),

            Err(poisoned) => {
                poisoned.into_inner().clear();
            }
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

        self.authorize_emission(authorization)?;

        let sequence = self
            .sequence
            .fetch_add(1, Ordering::Relaxed)
            .saturating_add(1);

        if !sample_sequence(
            sequence,
            self.config.sampling_rate,
        ) {
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

        match self.records.lock() {
            Ok(mut records) => {
                if records.len()
                    >= self.config.max_buffer_records
                {
                    records.pop_front();

                    self.dropped_records
                        .fetch_add(1, Ordering::Relaxed);
                }

                records.push_back(record.clone());
            }

            Err(poisoned) => {
                let mut records = poisoned.into_inner();

                if records.len()
                    >= self.config.max_buffer_records
                {
                    records.pop_front();

                    self.dropped_records
                        .fetch_add(1, Ordering::Relaxed);
                }

                records.push_back(record.clone());
            }
        }

        self.emitted_records
            .fetch_add(1, Ordering::Relaxed);

        /*
         * Remote export is deliberately after local bounded retention.
         *
         * Export failure cannot change the QEC result.
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

    /// Enforce the telemetry capability boundary.
    ///
    /// Local telemetry may be used without a capability context because it
    /// never leaves the local QEC process.
    ///
    /// Remote telemetry always requires:
    ///
    /// `Capability::EmitTelemetry`
    ///
    /// and a valid capability grant.
    fn authorize_emission(
        &self,
        authorization: Option<&CapabilityContext>,
    ) -> QecResult<()> {
        if self.config.policy.permits_remote()
            && self.exporter.is_none()
        {
            return Err(QecError::unsupported(
                "remote telemetry",
                "explicit remote telemetry requires a configured exporter",
            ));
        }

        if !self.config.policy.permits_remote() {
            return Ok(());
        }

        let context = authorization.ok_or_else(|| {
            QecError::unsupported(
                "telemetry capability",
                "remote telemetry requires explicit capability authorization",
            )
        })?;

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
                    format!(
                        "telemetry emission denied: {error}"
                    ),
                )
            })?;

        Ok(())
    }
}

/* -------------------------------------------------------------------------- */
/* Errors                                                                      */
/* -------------------------------------------------------------------------- */

/// Errors encountered while constructing/configuring telemetry.
///
/// Runtime telemetry emission uses the canonical `QecError` boundary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TelemetryError {
    /// Runtime telemetry configuration is invalid.
    InvalidConfiguration(&'static str),

    /// Persisted `QecConfig` contains invalid telemetry configuration.
    Configuration(String),

    /// Remote export was requested without an explicit remote policy.
    RemoteExportNotPermitted,
}

impl fmt::Display for TelemetryError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
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

/* -------------------------------------------------------------------------- */
/* Deterministic helpers                                                       */
/* -------------------------------------------------------------------------- */

/// Convert a duration to a bounded nanosecond counter.
///
/// Saturation prevents telemetry conversion itself from becoming an overflow
/// source.
fn duration_nanos(duration: Duration) -> u64 {
    duration
        .as_nanos()
        .min(u64::MAX as u128) as u64
}

/// Current Unix timestamp used only for capability expiry checks.
///
/// The timestamp is not stored in telemetry records. This keeps telemetry
/// deterministic and avoids unnecessary wall-clock metadata leakage.
fn unix_seconds() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or(0)
}

/// Deterministic sampling.
///
/// No RNG state is used. Therefore worker scheduling cannot change sampling
/// decisions for a given sequence number.
fn sample_sequence(
    sequence: u64,
    rate: f64,
) -> bool {
    if rate <= 0.0 {
        return false;
    }

    if rate >= 1.0 {
        return true;
    }

    let hash = stable_digest_u64(sequence);

    let fraction =
        (hash as f64) / (u64::MAX as f64);

    fraction < rate
}

/// Stable FNV-1a digest for a numeric sequence.
fn stable_digest_u64(value: u64) -> u64 {
    let mut hash = 0xcbf29ce484222325u64;

    for byte in value.to_le_bytes() {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }

    hash
}

/// Stable FNV-1a digest for redacting identifiers.
///
/// This is NOT a cryptographic hash.
///
/// It is used only to prevent raw backend/calibration identifiers from being
/// copied into telemetry records while retaining stable cardinality grouping.
fn stable_digest(value: &str) -> u64 {
    let mut hash = 0xcbf29ce484222325u64;

    for byte in value.as_bytes() {
        hash ^= *byte as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }

    hash
}

/* -------------------------------------------------------------------------- */
/* Tests                                                                       */
/* -------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use super::*;

    use std::sync::atomic::{
        AtomicU64,
        Ordering,
    };

    struct TestExporter {
        calls: AtomicU64,
    }

    impl TelemetryExporter for TestExporter {
        fn export(
            &self,
            _record: &TelemetryRecord,
        ) -> Result<(), String> {
            self.calls.fetch_add(
                1,
                Ordering::Relaxed,
            );

            Ok(())
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

    #[test]
    fn telemetry_is_bounded() {
        let mut config =
            configuration(TelemetryPolicy::LocalOnly);

        config.max_buffer_records = 2;

        let collector =
            TelemetryCollector::new(config).unwrap();

        let metrics =
            MetricsSnapshot::empty(
                DecoderId::UnionFind,
                BackendKind::Cpu,
            );

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

        assert_eq!(
            snapshot.retained_records,
            2
        );

        assert_eq!(
            snapshot.dropped_records,
            1
        );
    }

    #[test]
    fn disabled_telemetry_retains_nothing() {
        let collector =
            TelemetryCollector::new(
                configuration(
                    TelemetryPolicy::Disabled,
                ),
            )
            .unwrap();

        let metrics =
            MetricsSnapshot::empty(
                DecoderId::UnionFind,
                BackendKind::Cpu,
            );

        collector
            .record_metrics(&metrics, None)
            .unwrap();

        assert_eq!(
            collector.snapshot().retained_records,
            0
        );
    }

    #[test]
    fn remote_export_requires_explicit_policy() {
        let exporter = Arc::new(
            TestExporter {
                calls: AtomicU64::new(0),
            },
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
    fn remote_export_requires_capability_context() {
        let exporter = Arc::new(
            TestExporter {
                calls: AtomicU64::new(0),
            },
        );

        let collector =
            TelemetryCollector::with_exporter(
                configuration(
                    TelemetryPolicy::ExplicitRemote,
                ),
                exporter.clone(),
            )
            .unwrap();

        let metrics =
            MetricsSnapshot::empty(
                DecoderId::UnionFind,
                BackendKind::Cpu,
            );

        let result =
            collector.record_metrics(
                &metrics,
                None,
            );

        assert!(result.is_err());

        assert_eq!(
            exporter.calls.load(
                Ordering::Relaxed,
            ),
            0
        );
    }

    #[test]
    fn deterministic_sampling_is_stable() {
        let first: Vec<bool> =
            (1..1_000)
                .map(|sequence| {
                    sample_sequence(
                        sequence,
                        0.5,
                    )
                })
                .collect();

        let second: Vec<bool> =
            (1..1_000)
                .map(|sequence| {
                    sample_sequence(
                        sequence,
                        0.5,
                    )
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
    fn resource_snapshot_is_aggregate_only() {
        let resources =
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
            };

        let telemetry =
            TelemetryResources::from(
                &resources,
            );

        assert_eq!(
            telemetry.allocated_bytes,
            100
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
    fn qpu_identifiers_are_not_stored_raw() {
        let backend =
            stable_digest("example-qpu");

        let calibration =
            stable_digest("calibration-secret-name");

        assert_ne!(
            backend,
            0
        );

        assert_ne!(
            calibration,
            0
        );
    }

    #[test]
    fn telemetry_record_has_no_raw_qec_payload() {
        let metrics =
            MetricsSnapshot::empty(
                DecoderId::UnionFind,
                BackendKind::Cpu,
            );

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
    }
}