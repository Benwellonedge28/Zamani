//! Production metrics and observability for Zamani Quantum Error Correction.
//!
//! # Ownership
//!
//! This module owns:
//!
//! - aggregate execution metrics;
//! - decoder lifecycle counters;
//! - logical/physical error statistics;
//! - latency statistics;
//! - graph/work counters;
//! - resource observations;
//! - QPU aggregate metrics;
//! - distributed snapshot aggregation;
//! - threshold-experiment summaries;
//! - bounded, thread-safe metric collection.
//!
//! This module does NOT own:
//!
//! - resource limits;
//! - resource admission;
//! - memory allocation;
//! - decoder correctness;
//! - authorization;
//! - telemetry transport;
//! - QPU credentials;
//! - raw syndrome data;
//! - raw measurement data;
//! - quantum circuits;
//! - checkpoint serialization.
//!
//! # Integration contract
//!
//! ```text
//!                         QecConfig
//!                             │
//!                             ▼
//!                       execution policy
//!                             │
//!                ┌────────────┴────────────┐
//!                │                         │
//!                ▼                         ▼
//!        ResourceManager              Decoder/Backend
//!                │                         │
//!                ▼                         ▼
//!        ResourceSnapshot          execution events
//!                │                         │
//!                └────────────┬────────────┘
//!                             ▼
//!                    MetricsCollector
//!                             │
//!                             ▼
//!                     MetricsSnapshot
//!                    /       |        \
//!                   /        |         \
//!                  ▼         ▼          ▼
//!             telemetry  threshold   checkpoint
//! ```
//!
//! `limits.rs` remains the policy authority.
//! `resources.rs` remains the runtime accounting authority.
//! `metrics.rs` only observes.
//!
//! # Security contract
//!
//! Metrics must never contain:
//!
//! - passwords;
//! - API tokens;
//! - private keys;
//! - QPU credentials;
//! - raw quantum circuits;
//! - raw syndrome streams;
//! - raw measurement payloads;
//! - proprietary device secrets;
//! - user data.
//!
//! # Determinism contract
//!
//! Correctness-affecting execution must never depend on metrics.
//!
//! Metric counters use integer aggregation and saturating arithmetic.
//! Floating-point rates are derived only when a snapshot is requested.
//!
//! Distributed aggregation therefore uses:
//!
//! ```text
//! integer counters
//!      ↓
//! deterministic addition/max
//!      ↓
//! derived rates
//! ```
//!
//! Wall-clock measurements are observational and must not be used to decide
//! whether two deterministic QEC executions are equivalent.
//!
//! # Overflow contract
//!
//! Metric overflow must never fail QEC execution.
//!
//! Counters saturate at their representable maximum.
//!
//! Resource enforcement remains the responsibility of `resources.rs`.
//!
//! # Rust compatibility
//!
//! Target: Rust 1.97.1.
//!
//! No unstable features are required.

#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;
use std::sync::atomic::{
    AtomicBool,
    AtomicU64,
    Ordering,
};
use std::sync::Arc;
use std::time::{Duration, Instant};

use super::backend::BackendKind;
use super::resources::ResourceSnapshot;

/* ========================================================================== */
/* Errors                                                                     */
/* ========================================================================== */

/// Errors produced while configuring or merging metric snapshots.
///
/// Recording an individual metric is intentionally infallible.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetricsError {
    /// A metric configuration contains an invalid value.
    InvalidConfiguration {
        field: &'static str,
    },

    /// Two snapshots cannot be safely merged.
    IncompatibleSnapshots {
        reason: &'static str,
    },
}

impl fmt::Display for MetricsError {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidConfiguration { field } => {
                write!(
                    f,
                    "invalid metrics configuration field: {field}"
                )
            }

            Self::IncompatibleSnapshots { reason } => {
                write!(
                    f,
                    "incompatible metric snapshots: {reason}"
                )
            }
        }
    }
}

impl std::error::Error for MetricsError {}

/* ========================================================================== */
/* Decoder metric identity                                                    */
/* ========================================================================== */

/// Stable metric identity for a decoder.
///
/// This deliberately does not reuse the decoder registry's internal numeric
/// identifier. Decoder registry identity may change independently from the
/// externally observable metrics identity.
///
/// This prevents `metrics.rs` from becoming coupled to decoder registration.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DecoderId {
    SurfaceCode,
    Mwpm,
    UnionFind,
    BeliefPropagation,
    TensorNetwork,
    LookupTable,
    Streaming,
    Distributed,

    /// User-defined decoder identity.
    Custom(String),
}

impl DecoderId {
    pub fn custom(
        name: impl Into<String>,
    ) -> Result<Self, MetricsError> {
        let name = name.into();

        if name.trim().is_empty() {
            return Err(
                MetricsError::InvalidConfiguration {
                    field: "decoder_name",
                },
            );
        }

        Ok(Self::Custom(name))
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::SurfaceCode => "surface_code",
            Self::Mwpm => "mwpm",
            Self::UnionFind => "union_find",
            Self::BeliefPropagation => "belief_propagation",
            Self::TensorNetwork => "tensor_network",
            Self::LookupTable => "lookup_table",
            Self::Streaming => "streaming",
            Self::Distributed => "distributed",
            Self::Custom(name) => name.as_str(),
        }
    }
}

impl Default for DecoderId {
    fn default() -> Self {
        Self::Custom("unknown".to_owned())
    }
}

impl fmt::Display for DecoderId {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/* ========================================================================== */
/* QPU metrics                                                                */
/* ========================================================================== */

/// Aggregate QPU metrics.
///
/// This type deliberately contains no credentials, raw results, circuits or
/// device secrets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct QpuMetrics {
    pub shots: u64,
    pub circuits: u64,
    pub measurement_count: u64,

    pub queue_time: Duration,
    pub execution_time: Duration,

    pub submission_failures: u64,
    pub invalid_measurements: u64,

    pub completed_jobs: u64,
    pub cancelled_jobs: u64,

    pub readout_error_count: u64,
    pub readout_error_opportunities: u64,
}

impl QpuMetrics {
    #[must_use]
    pub fn readout_error_rate(&self) -> Option<f64> {
        ratio(
            self.readout_error_count,
            self.readout_error_opportunities,
        )
    }

    fn merge_from(
        &mut self,
        other: &Self,
    ) {
        self.shots = self.shots.saturating_add(other.shots);
        self.circuits =
            self.circuits.saturating_add(other.circuits);

        self.measurement_count = self
            .measurement_count
            .saturating_add(other.measurement_count);

        self.queue_time = saturating_duration_add(
            self.queue_time,
            other.queue_time,
        );

        self.execution_time = saturating_duration_add(
            self.execution_time,
            other.execution_time,
        );

        self.submission_failures = self
            .submission_failures
            .saturating_add(other.submission_failures);

        self.invalid_measurements = self
            .invalid_measurements
            .saturating_add(other.invalid_measurements);

        self.completed_jobs = self
            .completed_jobs
            .saturating_add(other.completed_jobs);

        self.cancelled_jobs = self
            .cancelled_jobs
            .saturating_add(other.cancelled_jobs);

        self.readout_error_count = self
            .readout_error_count
            .saturating_add(other.readout_error_count);

        self.readout_error_opportunities = self
            .readout_error_opportunities
            .saturating_add(
                other.readout_error_opportunities,
            );
    }
}

/* ========================================================================== */
/* Metrics configuration                                                       */
/* ========================================================================== */

/// Metrics configuration.
///
/// Metrics are deliberately bounded and aggregate-only.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MetricsConfig {
    /// Whether worker high-water marks are collected.
    pub track_worker_count: bool,

    /// Whether QPU metrics are collected.
    pub track_qpu: bool,

    /// Whether resource observations are collected.
    pub track_resources: bool,
}

impl Default for MetricsConfig {
    fn default() -> Self {
        Self {
            track_worker_count: true,
            track_qpu: true,
            track_resources: true,
        }
    }
}

impl MetricsConfig {
    pub fn validate(
        &self,
    ) -> Result<(), MetricsError> {
        Ok(())
    }
}

/* ========================================================================== */
/* Metrics snapshot                                                           */
/* ========================================================================== */

/// Immutable aggregate metrics snapshot.
///
/// A snapshot is suitable for:
///
/// - telemetry;
/// - threshold experiments;
/// - checkpoint metadata;
/// - distributed aggregation;
/// - deterministic regression testing.
///
/// It contains no raw execution payloads.
#[derive(Debug, Clone, PartialEq)]
pub struct MetricsSnapshot {
    /* ---------------------------------------------------------------------- */
    /* Identity                                                               */
    /* ---------------------------------------------------------------------- */

    pub decoder: DecoderId,
    pub backend: BackendKind,

    /* ---------------------------------------------------------------------- */
    /* Decoder lifecycle                                                      */
    /* ---------------------------------------------------------------------- */

    pub decode_operations: u64,
    pub decoder_success: u64,
    pub decoder_failure: u64,
    pub cancellation_count: u64,

    /* ---------------------------------------------------------------------- */
    /* QEC activity                                                            */
    /* ---------------------------------------------------------------------- */

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

    /* ---------------------------------------------------------------------- */
    /* Resource observations                                                   */
    /* ---------------------------------------------------------------------- */

    pub checkpoint_bytes: u64,
    pub partitions: u64,
    pub stream_buffer_events: u64,
    pub verification_operations: u64,

    pub peak_memory: u64,
    pub current_memory: u64,

    /* ---------------------------------------------------------------------- */
    /* Parallel execution                                                     */
    /* ---------------------------------------------------------------------- */

    pub worker_count: u64,

    /* ---------------------------------------------------------------------- */
    /* Timing                                                                  */
    /* ---------------------------------------------------------------------- */

    pub decoder_latency: Duration,
    pub max_decoder_latency: Duration,
    pub min_decoder_latency: Option<Duration>,
    pub average_decoder_latency: Option<Duration>,

    pub compute_time: Duration,

    /// Observational wall-clock time.
    ///
    /// This value must never participate in deterministic correctness.
    pub wall_time: Duration,

    /* ---------------------------------------------------------------------- */
    /* Derived rates                                                           */
    /* ---------------------------------------------------------------------- */

    pub physical_error_rate: Option<f64>,
    pub logical_error_rate: Option<f64>,

    pub decoder_success_rate: Option<f64>,
    pub decoder_failure_rate: Option<f64>,

    /* ---------------------------------------------------------------------- */
    /* State                                                                    */
    /* ---------------------------------------------------------------------- */

    pub had_logical_failure: bool,

    pub operations_balanced: bool,

    /* ---------------------------------------------------------------------- */
    /* QPU                                                                      */
    /* ---------------------------------------------------------------------- */

    pub qpu: QpuMetrics,
}

impl MetricsSnapshot {
    #[must_use]
    pub fn empty(
        decoder: DecoderId,
        backend: BackendKind,
    ) -> Self {
        Self {
            decoder,
            backend,

            decode_operations: 0,
            decoder_success: 0,
            decoder_failure: 0,
            cancellation_count: 0,

            correction_count: 0,
            detection_event_count: 0,

            physical_error_count: 0,
            physical_error_opportunities: 0,

            logical_failure_count: 0,
            logical_error_opportunities: 0,

            matching_count: 0,
            decoder_iterations: 0,

            graph_nodes: 0,
            graph_edges: 0,

            qubit_count: 0,
            stabilizer_count: 0,
            measurement_rounds: 0,

            checkpoint_bytes: 0,
            partitions: 0,
            stream_buffer_events: 0,
            verification_operations: 0,

            peak_memory: 0,
            current_memory: 0,

            worker_count: 0,

            decoder_latency: Duration::ZERO,
            max_decoder_latency: Duration::ZERO,
            min_decoder_latency: None,
            average_decoder_latency: None,

            compute_time: Duration::ZERO,
            wall_time: Duration::ZERO,

            physical_error_rate: None,
            logical_error_rate: None,
            decoder_success_rate: None,
            decoder_failure_rate: None,

            had_logical_failure: false,
            operations_balanced: true,

            qpu: QpuMetrics::default(),
        }
    }

    #[must_use]
    pub fn terminal_operations(&self) -> u64 {
        self.decoder_success
            .saturating_add(self.decoder_failure)
            .saturating_add(self.cancellation_count)
    }

    #[must_use]
    pub fn all_decodes_successful(&self) -> bool {
        self.decode_operations > 0
            && self.decoder_failure == 0
            && self.cancellation_count == 0
            && self.operations_balanced
    }

    #[must_use]
    pub fn physical_error_rate_or_zero(&self) -> f64 {
        self.physical_error_rate.unwrap_or(0.0)
    }

    #[must_use]
    pub fn logical_error_rate_or_zero(&self) -> f64 {
        self.logical_error_rate.unwrap_or(0.0)
    }

    /// Merge another snapshot into this snapshot.
    ///
    /// Additive quantities are summed.
    /// High-water marks use `max`.
    /// Boolean failure state uses logical OR.
    ///
    /// Identity must match.
    pub fn merge(
        &mut self,
        other: &Self,
    ) -> Result<(), MetricsError> {
        if self.decoder != other.decoder {
            return Err(
                MetricsError::IncompatibleSnapshots {
                    reason: "decoder identities differ",
                },
            );
        }

        if self.backend != other.backend {
            return Err(
                MetricsError::IncompatibleSnapshots {
                    reason: "backend identities differ",
                },
            );
        }

        self.decode_operations =
            self.decode_operations.saturating_add(
                other.decode_operations,
            );

        self.decoder_success =
            self.decoder_success.saturating_add(
                other.decoder_success,
            );

        self.decoder_failure =
            self.decoder_failure.saturating_add(
                other.decoder_failure,
            );

        self.cancellation_count =
            self.cancellation_count.saturating_add(
                other.cancellation_count,
            );

        self.correction_count =
            self.correction_count.saturating_add(
                other.correction_count,
            );

        self.detection_event_count =
            self.detection_event_count.saturating_add(
                other.detection_event_count,
            );

        self.physical_error_count =
            self.physical_error_count.saturating_add(
                other.physical_error_count,
            );

        self.physical_error_opportunities =
            self
                .physical_error_opportunities
                .saturating_add(
                    other.physical_error_opportunities,
                );

        self.logical_failure_count =
            self.logical_failure_count.saturating_add(
                other.logical_failure_count,
            );

        self.logical_error_opportunities =
            self
                .logical_error_opportunities
                .saturating_add(
                    other.logical_error_opportunities,
                );

        self.matching_count =
            self.matching_count.saturating_add(
                other.matching_count,
            );

        self.decoder_iterations =
            self.decoder_iterations.saturating_add(
                other.decoder_iterations,
            );

        self.graph_nodes =
            self.graph_nodes.saturating_add(
                other.graph_nodes,
            );

        self.graph_edges =
            self.graph_edges.saturating_add(
                other.graph_edges,
            );

        self.qubit_count =
            self.qubit_count.max(other.qubit_count);

        self.stabilizer_count =
            self.stabilizer_count.max(
                other.stabilizer_count,
            );

        self.measurement_rounds =
            self.measurement_rounds.max(
                other.measurement_rounds,
            );

        self.checkpoint_bytes =
            self.checkpoint_bytes.max(
                other.checkpoint_bytes,
            );

        self.partitions =
            self.partitions.max(other.partitions);

        self.stream_buffer_events =
            self.stream_buffer_events.max(
                other.stream_buffer_events,
            );

        self.verification_operations =
            self.verification_operations.saturating_add(
                other.verification_operations,
            );

        self.peak_memory =
            self.peak_memory.max(other.peak_memory);

        self.current_memory = other.current_memory;

        self.worker_count =
            self.worker_count.max(other.worker_count);

        self.decoder_latency =
            saturating_duration_add(
                self.decoder_latency,
                other.decoder_latency,
            );

        self.max_decoder_latency =
            self.max_decoder_latency.max(
                other.max_decoder_latency,
            );

        self.min_decoder_latency =
            match (
                self.min_decoder_latency,
                other.min_decoder_latency,
            ) {
                (Some(a), Some(b)) => Some(a.min(b)),
                (Some(a), None) => Some(a),
                (None, Some(b)) => Some(b),
                (None, None) => None,
            };

        self.compute_time =
            saturating_duration_add(
                self.compute_time,
                other.compute_time,
            );

        self.wall_time =
            saturating_duration_add(
                self.wall_time,
                other.wall_time,
            );

        self.had_logical_failure |=
            other.had_logical_failure;

        self.qpu.merge_from(&other.qpu);

        self.recalculate_rates();

        self.operations_balanced =
            self.decode_operations
                == self.terminal_operations();

        Ok(())
    }

    fn recalculate_rates(&mut self) {
        self.physical_error_rate = ratio(
            self.physical_error_count,
            self.physical_error_opportunities,
        );

        self.logical_error_rate = ratio(
            self.logical_failure_count,
            self.logical_error_opportunities,
        );

        self.decoder_success_rate = ratio(
            self.decoder_success,
            self.decode_operations,
        );

        self.decoder_failure_rate = ratio(
            self.decoder_failure,
            self.decode_operations,
        );

        self.average_decoder_latency =
            if self.decode_operations > 0 {
                Some(duration_divide(
                    self.decoder_latency,
                    self.decode_operations,
                ))
            } else {
                None
            };
    }
}

/* ========================================================================== */
/* Threshold metrics                                                          */
/* ========================================================================== */

/// Aggregate metrics for a threshold experiment.
///
/// Statistical confidence intervals intentionally belong to `statistical.rs`.
/// This type stores only the raw aggregate quantities required by that layer.
#[derive(Debug, Clone, PartialEq)]
pub struct ThresholdMetrics {
    pub code_distance: u64,
    pub physical_error_probability: f64,

    pub trials: u64,
    pub logical_failures: u64,

    pub logical_error_rate: Option<f64>,

    pub decoder_failures: u64,
    pub cancelled_trials: u64,

    pub total_latency: Duration,
    pub average_latency: Option<Duration>,

    pub peak_memory: u64,
}

impl ThresholdMetrics {
    #[must_use]
    pub fn from_snapshot(
        snapshot: &MetricsSnapshot,
        code_distance: u64,
        physical_error_probability: f64,
    ) -> Self {
        Self {
            code_distance,
            physical_error_probability,

            trials: snapshot.logical_error_opportunities,
            logical_failures: snapshot.logical_failure_count,

            logical_error_rate:
                snapshot.logical_error_rate,

            decoder_failures:
                snapshot.decoder_failure,

            cancelled_trials:
                snapshot.cancellation_count,

            total_latency:
                snapshot.decoder_latency,

            average_latency:
                snapshot.average_decoder_latency,

            peak_memory:
                snapshot.peak_memory,
        }
    }
}

/* ========================================================================== */
/* Atomic storage                                                             */
/* ========================================================================== */

#[derive(Debug)]
struct MetricsCounters {
    decode_operations: AtomicU64,
    decoder_success: AtomicU64,
    decoder_failure: AtomicU64,
    cancellation_count: AtomicU64,

    correction_count: AtomicU64,
    detection_event_count: AtomicU64,

    physical_error_count: AtomicU64,
    physical_error_opportunities: AtomicU64,

    logical_failure_count: AtomicU64,
    logical_error_opportunities: AtomicU64,

    matching_count: AtomicU64,
    decoder_iterations: AtomicU64,

    graph_nodes: AtomicU64,
    graph_edges: AtomicU64,

    qubit_count: AtomicU64,
    stabilizer_count: AtomicU64,
    measurement_rounds: AtomicU64,

    checkpoint_bytes: AtomicU64,
    partitions: AtomicU64,
    stream_buffer_events: AtomicU64,
    verification_operations: AtomicU64,

    worker_count: AtomicU64,

    decoder_latency_nanos: AtomicU64,
    max_decoder_latency_nanos: AtomicU64,
    min_decoder_latency_nanos: AtomicU64,

    compute_time_nanos: AtomicU64,

    peak_memory: AtomicU64,
    current_memory: AtomicU64,

    qpu_shots: AtomicU64,
    qpu_circuits: AtomicU64,
    qpu_measurement_count: AtomicU64,

    qpu_queue_time_nanos: AtomicU64,
    qpu_execution_time_nanos: AtomicU64,

    qpu_submission_failures: AtomicU64,
    qpu_invalid_measurements: AtomicU64,

    qpu_completed_jobs: AtomicU64,
    qpu_cancelled_jobs: AtomicU64,

    qpu_readout_error_count: AtomicU64,
    qpu_readout_error_opportunities: AtomicU64,

    had_logical_failure: AtomicBool,
}

impl Default for MetricsCounters {
    fn default() -> Self {
        Self {
            decode_operations:
                AtomicU64::new(0),
            decoder_success:
                AtomicU64::new(0),
            decoder_failure:
                AtomicU64::new(0),
            cancellation_count:
                AtomicU64::new(0),

            correction_count:
                AtomicU64::new(0),
            detection_event_count:
                AtomicU64::new(0),

            physical_error_count:
                AtomicU64::new(0),
            physical_error_opportunities:
                AtomicU64::new(0),

            logical_failure_count:
                AtomicU64::new(0),
            logical_error_opportunities:
                AtomicU64::new(0),

            matching_count:
                AtomicU64::new(0),
            decoder_iterations:
                AtomicU64::new(0),

            graph_nodes:
                AtomicU64::new(0),
            graph_edges:
                AtomicU64::new(0),

            qubit_count:
                AtomicU64::new(0),
            stabilizer_count:
                AtomicU64::new(0),
            measurement_rounds:
                AtomicU64::new(0),

            checkpoint_bytes:
                AtomicU64::new(0),
            partitions:
                AtomicU64::new(0),
            stream_buffer_events:
                AtomicU64::new(0),
            verification_operations:
                AtomicU64::new(0),

            worker_count:
                AtomicU64::new(0),

            decoder_latency_nanos:
                AtomicU64::new(0),
            max_decoder_latency_nanos:
                AtomicU64::new(0),
            min_decoder_latency_nanos:
                AtomicU64::new(u64::MAX),

            compute_time_nanos:
                AtomicU64::new(0),

            peak_memory:
                AtomicU64::new(0),
            current_memory:
                AtomicU64::new(0),

            qpu_shots:
                AtomicU64::new(0),
            qpu_circuits:
                AtomicU64::new(0),
            qpu_measurement_count:
                AtomicU64::new(0),

            qpu_queue_time_nanos:
                AtomicU64::new(0),
            qpu_execution_time_nanos:
                AtomicU64::new(0),

            qpu_submission_failures:
                AtomicU64::new(0),
            qpu_invalid_measurements:
                AtomicU64::new(0),

            qpu_completed_jobs:
                AtomicU64::new(0),
            qpu_cancelled_jobs:
                AtomicU64::new(0),

            qpu_readout_error_count:
                AtomicU64::new(0),
            qpu_readout_error_opportunities:
                AtomicU64::new(0),

            had_logical_failure:
                AtomicBool::new(false),
        }
    }
}

/* ========================================================================== */
/* Collector                                                                  */
/* ========================================================================== */

/// Thread-safe aggregate metrics collector.
///
/// The collector is safe to share across decoder workers.
///
/// Metrics recording is intentionally infallible.
#[derive(Debug)]
pub struct MetricsCollector {
    decoder: DecoderId,
    backend: BackendKind,
    config: MetricsConfig,

    counters: MetricsCounters,

    started: Instant,
}

impl MetricsCollector {
    pub fn new(
        decoder: DecoderId,
        backend: BackendKind,
        config: MetricsConfig,
    ) -> Result<Self, MetricsError> {
        config.validate()?;

        Ok(Self {
            decoder,
            backend,
            config,
            counters: MetricsCounters::default(),
            started: Instant::now(),
        })
    }

    #[must_use]
    pub fn standard(
        decoder: DecoderId,
        backend: BackendKind,
    ) -> Self {
        Self {
            decoder,
            backend,
            config: MetricsConfig::default(),
            counters: MetricsCounters::default(),
            started: Instant::now(),
        }
    }

    #[must_use]
    pub fn decoder(&self) -> &DecoderId {
        &self.decoder
    }

    #[must_use]
    pub const fn backend(&self) -> BackendKind {
        self.backend
    }

    #[must_use]
    pub const fn config(&self) -> MetricsConfig {
        self.config
    }

    /* ---------------------------------------------------------------------- */
    /* Decoder lifecycle                                                      */
    /* ---------------------------------------------------------------------- */

    /// Starts a decoder operation.
    ///
    /// The caller must subsequently record exactly one terminal state:
    ///
    /// - `record_success`;
    /// - `record_failure`;
    /// - `record_cancellation`.
    pub fn begin_decode(&self) {
        saturating_increment(
            &self.counters.decode_operations,
        );
    }

    pub fn record_success(
        &self,
        latency: Duration,
    ) {
        saturating_increment(
            &self.counters.decoder_success,
        );

        self.record_latency(latency);
    }

    pub fn record_failure(
        &self,
        latency: Duration,
    ) {
        saturating_increment(
            &self.counters.decoder_failure,
        );

        self.record_latency(latency);
    }

    pub fn record_cancellation(
        &self,
        latency: Duration,
    ) {
        saturating_increment(
            &self.counters.cancellation_count,
        );

        self.record_latency(latency);
    }

    /// Records a complete operation in one call.
    pub fn record_outcome(
        &self,
        success: bool,
        cancelled: bool,
        latency: Duration,
    ) {
        self.begin_decode();

        if cancelled {
            self.record_cancellation(latency);
        } else if success {
            self.record_success(latency);
        } else {
            self.record_failure(latency);
        }
    }

    /* ---------------------------------------------------------------------- */
    /* QEC activity                                                            */
    /* ---------------------------------------------------------------------- */

    pub fn record_corrections(
        &self,
        count: u64,
    ) {
        saturating_add(
            &self.counters.correction_count,
            count,
        );
    }

    pub fn record_detection_events(
        &self,
        count: u64,
    ) {
        saturating_add(
            &self.counters.detection_event_count,
            count,
        );
    }

    pub fn record_physical_errors(
        &self,
        count: u64,
    ) {
        saturating_add(
            &self.counters.physical_error_count,
            count,
        );
    }

    pub fn record_physical_error_opportunities(
        &self,
        count: u64,
    ) {
        saturating_add(
            &self.counters.physical_error_opportunities,
            count,
        );
    }

    pub fn record_physical_error_trial(
        &self,
        error_occurred: bool,
    ) {
        saturating_increment(
            &self.counters.physical_error_opportunities,
        );

        if error_occurred {
            saturating_increment(
                &self.counters.physical_error_count,
            );
        }
    }

    pub fn record_logical_failures(
        &self,
        count: u64,
    ) {
        if count > 0 {
            self.counters
                .had_logical_failure
                .store(
                    true,
                    Ordering::Release,
                );
        }

        saturating_add(
            &self.counters.logical_failure_count,
            count,
        );
    }

    pub fn record_logical_error_opportunities(
        &self,
        count: u64,
    ) {
        saturating_add(
            &self.counters.logical_error_opportunities,
            count,
        );
    }

    pub fn record_logical_trial(
        &self,
        logical_failure: bool,
    ) {
        saturating_increment(
            &self.counters.logical_error_opportunities,
        );

        if logical_failure {
            self.record_logical_failures(1);
        }
    }

    /* ---------------------------------------------------------------------- */
    /* Decoder internals                                                       */
    /* ---------------------------------------------------------------------- */

    pub fn record_matching(
        &self,
        count: u64,
    ) {
        saturating_add(
            &self.counters.matching_count,
            count,
        );
    }

    pub fn record_decoder_iterations(
        &self,
        count: u64,
    ) {
        saturating_add(
            &self.counters.decoder_iterations,
            count,
        );
    }

    pub fn record_graph_nodes(
        &self,
        count: u64,
    ) {
        saturating_add(
            &self.counters.graph_nodes,
            count,
        );
    }

    pub fn record_graph_edges(
        &self,
        count: u64,
    ) {
        saturating_add(
            &self.counters.graph_edges,
            count,
        );
    }

    /// Records the largest observed qubit count.
    pub fn record_qubits(
        &self,
        count: u64,
    ) {
        saturating_max(
            &self.counters.qubit_count,
            count,
        );
    }

    /// Records the largest observed stabilizer count.
    pub fn record_stabilizers(
        &self,
        count: u64,
    ) {
        saturating_max(
            &self.counters.stabilizer_count,
            count,
        );
    }

    /// Records the largest observed measurement-round count.
    pub fn record_measurement_rounds(
        &self,
        count: u64,
    ) {
        saturating_max(
            &self.counters.measurement_rounds,
            count,
        );
    }

    /* ---------------------------------------------------------------------- */
    /* Resource observations                                                   */
    /* ---------------------------------------------------------------------- */

    pub fn record_checkpoint_bytes(
        &self,
        count: u64,
    ) {
        saturating_max(
            &self.counters.checkpoint_bytes,
            count,
        );
    }

    pub fn record_partitions(
        &self,
        count: u64,
    ) {
        saturating_max(
            &self.counters.partitions,
            count,
        );
    }

    pub fn record_stream_buffer_events(
        &self,
        count: u64,
    ) {
        saturating_max(
            &self.counters.stream_buffer_events,
            count,
        );
    }

    pub fn record_verification_operations(
        &self,
        count: u64,
    ) {
        saturating_add(
            &self.counters.verification_operations,
            count,
        );
    }

    pub fn record_workers(
        &self,
        workers: usize,
    ) {
        if !self.config.track_worker_count {
            return;
        }

        saturating_max(
            &self.counters.worker_count,
            usize_to_u64_saturating(workers),
        );
    }

    /// Observes the runtime resource manager.
    ///
    /// This function never enforces limits.
    pub fn record_resource_snapshot(
        &self,
        snapshot: ResourceSnapshot,
    ) {
        if !self.config.track_resources {
            return;
        }

        self.record_memory(
            snapshot.allocated_bytes,
        );

        saturating_max(
            &self.counters.peak_memory,
            snapshot.peak_bytes,
        );

        saturating_max(
            &self.counters.detection_event_count,
            snapshot.syndrome_events,
        );

        saturating_max(
            &self.counters.graph_nodes,
            snapshot.graph_nodes,
        );

        saturating_max(
            &self.counters.graph_edges,
            snapshot.graph_edges,
        );

        saturating_max(
            &self.counters.decoder_iterations,
            snapshot.decoder_iterations,
        );

        self.record_workers(
            snapshot.parallel_workers,
        );

        self.record_qubits(
            usize_to_u64_saturating(
                snapshot.qubits,
            ),
        );

        self.record_stabilizers(
            usize_to_u64_saturating(
                snapshot.stabilizers,
            ),
        );

        self.record_measurement_rounds(
            usize_to_u64_saturating(
                snapshot.measurement_rounds,
            ),
        );

        self.record_checkpoint_bytes(
            snapshot.checkpoint_bytes,
        );

        self.record_partitions(
            usize_to_u64_saturating(
                snapshot.partitions,
            ),
        );

        self.record_stream_buffer_events(
            usize_to_u64_saturating(
                snapshot.stream_buffer_events,
            ),
        );

        self.record_verification_operations(
            snapshot.verification_operations,
        );

        if self.config.track_qpu {
            self.record_qpu_shots(
                snapshot.qpu_shots,
            );

            self.record_qpu_circuits(
                snapshot.qpu_circuits,
            );
        }

        self.record_compute_time(
            snapshot.compute_time,
        );
    }

    pub fn record_memory(
        &self,
        current_memory: u64,
    ) {
        self.counters
            .current_memory
            .store(
                current_memory,
                Ordering::Release,
            );

        saturating_max(
            &self.counters.peak_memory,
            current_memory,
        );
    }

    /* ---------------------------------------------------------------------- */
    /* Timing                                                                  */
    /* ---------------------------------------------------------------------- */

    pub fn record_latency(
        &self,
        latency: Duration,
    ) {
        let nanos =
            duration_to_nanos_saturating(latency);

        saturating_add(
            &self.counters.decoder_latency_nanos,
            nanos,
        );

        saturating_max(
            &self.counters.max_decoder_latency_nanos,
            nanos,
        );

        update_min(
            &self.counters.min_decoder_latency_nanos,
            nanos,
        );
    }

    pub fn record_compute_time(
        &self,
        duration: Duration,
    ) {
        saturating_add(
            &self.counters.compute_time_nanos,
            duration_to_nanos_saturating(
                duration,
            ),
        );
    }

    #[must_use]
    pub fn start_timer(
        &self,
    ) -> MetricsTimer<'_> {
        MetricsTimer {
            collector: self,
            started: Instant::now(),
            stopped: false,
        }
    }

    /* ---------------------------------------------------------------------- */
    /* QPU                                                                      */
    /* ---------------------------------------------------------------------- */

    pub fn record_qpu_shots(
        &self,
        count: u64,
    ) {
        if !self.config.track_qpu {
            return;
        }

        saturating_add(
            &self.counters.qpu_shots,
            count,
        );
    }

    pub fn record_qpu_circuits(
        &self,
        count: u64,
    ) {
        if !self.config.track_qpu {
            return;
        }

        saturating_add(
            &self.counters.qpu_circuits,
            count,
        );
    }

    pub fn record_qpu_measurements(
        &self,
        count: u64,
    ) {
        if !self.config.track_qpu {
            return;
        }

        saturating_add(
            &self.counters.qpu_measurement_count,
            count,
        );
    }

    pub fn record_qpu_queue_time(
        &self,
        duration: Duration,
    ) {
        if !self.config.track_qpu {
            return;
        }

        saturating_add(
            &self.counters.qpu_queue_time_nanos,
            duration_to_nanos_saturating(
                duration,
            ),
        );
    }

    pub fn record_qpu_execution_time(
        &self,
        duration: Duration,
    ) {
        if !self.config.track_qpu {
            return;
        }

        saturating_add(
            &self.counters.qpu_execution_time_nanos,
            duration_to_nanos_saturating(
                duration,
            ),
        );
    }

    pub fn record_qpu_submission_failure(
        &self,
    ) {
        if !self.config.track_qpu {
            return;
        }

        saturating_increment(
            &self.counters.qpu_submission_failures,
        );
    }

    pub fn record_qpu_invalid_measurements(
        &self,
        count: u64,
    ) {
        if !self.config.track_qpu {
            return;
        }

        saturating_add(
            &self.counters.qpu_invalid_measurements,
            count,
        );
    }

    pub fn record_qpu_job_completed(
        &self,
    ) {
        if !self.config.track_qpu {
            return;
        }

        saturating_increment(
            &self.counters.qpu_completed_jobs,
        );
    }

    pub fn record_qpu_job_cancelled(
        &self,
    ) {
        if !self.config.track_qpu {
            return;
        }

        saturating_increment(
            &self.counters.qpu_cancelled_jobs,
        );
    }

    pub fn record_qpu_readout_trial(
        &self,
        error_occurred: bool,
    ) {
        if !self.config.track_qpu {
            return;
        }

        saturating_increment(
            &self.counters
                .qpu_readout_error_opportunities,
        );

        if error_occurred {
            saturating_increment(
                &self.counters
                    .qpu_readout_error_count,
            );
        }
    }

    pub fn record_qpu_metrics(
        &self,
        metrics: QpuMetrics,
    ) {
        if !self.config.track_qpu {
            return;
        }

        saturating_add(
            &self.counters.qpu_shots,
            metrics.shots,
        );

        saturating_add(
            &self.counters.qpu_circuits,
            metrics.circuits,
        );

        saturating_add(
            &self.counters.qpu_measurement_count,
            metrics.measurement_count,
        );

        saturating_add(
            &self.counters.qpu_queue_time_nanos,
            duration_to_nanos_saturating(
                metrics.queue_time,
            ),
        );

        saturating_add(
            &self.counters.qpu_execution_time_nanos,
            duration_to_nanos_saturating(
                metrics.execution_time,
            ),
        );

        saturating_add(
            &self.counters
                .qpu_submission_failures,
            metrics.submission_failures,
        );

        saturating_add(
            &self.counters
                .qpu_invalid_measurements,
            metrics.invalid_measurements,
        );

        saturating_add(
            &self.counters.qpu_completed_jobs,
            metrics.completed_jobs,
        );

        saturating_add(
            &self.counters.qpu_cancelled_jobs,
            metrics.cancelled_jobs,
        );

        saturating_add(
            &self.counters
                .qpu_readout_error_count,
            metrics.readout_error_count,
        );

        saturating_add(
            &self.counters
                .qpu_readout_error_opportunities,
            metrics
                .readout_error_opportunities,
        );
    }

    /* ---------------------------------------------------------------------- */
    /* Snapshot                                                                 */
    /* ---------------------------------------------------------------------- */

    #[must_use]
    pub fn snapshot(
        &self,
    ) -> MetricsSnapshot {
        let decode_operations =
            load(&self.counters.decode_operations);

        let decoder_success =
            load(&self.counters.decoder_success);

        let decoder_failure =
            load(&self.counters.decoder_failure);

        let cancellation_count =
            load(&self.counters.cancellation_count);

        let decoder_latency =
            Duration::from_nanos(
                load(
                    &self.counters
                        .decoder_latency_nanos,
                ),
            );

        let min_latency =
            load(
                &self.counters
                    .min_decoder_latency_nanos,
            );

        let physical_errors =
            load(
                &self.counters
                    .physical_error_count,
            );

        let physical_opportunities =
            load(
                &self.counters
                    .physical_error_opportunities,
            );

        let logical_failures =
            load(
                &self.counters
                    .logical_failure_count,
            );

        let logical_opportunities =
            load(
                &self.counters
                    .logical_error_opportunities,
            );

        let qpu = if self.config.track_qpu {
            QpuMetrics {
                shots: load(
                    &self.counters.qpu_shots,
                ),
                circuits: load(
                    &self.counters.qpu_circuits,
                ),
                measurement_count: load(
                    &self.counters
                        .qpu_measurement_count,
                ),
                queue_time:
                    Duration::from_nanos(
                        load(
                            &self.counters
                                .qpu_queue_time_nanos,
                        ),
                    ),
                execution_time:
                    Duration::from_nanos(
                        load(
                            &self.counters
                                .qpu_execution_time_nanos,
                        ),
                    ),
                submission_failures: load(
                    &self.counters
                        .qpu_submission_failures,
                ),
                invalid_measurements: load(
                    &self.counters
                        .qpu_invalid_measurements,
                ),
                completed_jobs: load(
                    &self.counters
                        .qpu_completed_jobs,
                ),
                cancelled_jobs: load(
                    &self.counters
                        .qpu_cancelled_jobs,
                ),
                readout_error_count: load(
                    &self.counters
                        .qpu_readout_error_count,
                ),
                readout_error_opportunities:
                    load(
                        &self.counters
                            .qpu_readout_error_opportunities,
                    ),
            }
        } else {
            QpuMetrics::default()
        };

        let terminal_operations =
            decoder_success
                .saturating_add(
                    decoder_failure,
                )
                .saturating_add(
                    cancellation_count,
                );

        MetricsSnapshot {
            decoder: self.decoder.clone(),
            backend: self.backend,

            decode_operations,

            decoder_success,
            decoder_failure,
            cancellation_count,

            correction_count: load(
                &self.counters.correction_count,
            ),

            detection_event_count: load(
                &self.counters
                    .detection_event_count,
            ),

            physical_error_count:
                physical_errors,

            physical_error_opportunities:
                physical_opportunities,

            logical_failure_count:
                logical_failures,

            logical_error_opportunities:
                logical_opportunities,

            matching_count: load(
                &self.counters.matching_count,
            ),

            decoder_iterations: load(
                &self.counters
                    .decoder_iterations,
            ),

            graph_nodes: load(
                &self.counters.graph_nodes,
            ),

            graph_edges: load(
                &self.counters.graph_edges,
            ),

            qubit_count: load(
                &self.counters.qubit_count,
            ),

            stabilizer_count: load(
                &self.counters.stabilizer_count,
            ),

            measurement_rounds: load(
                &self.counters
                    .measurement_rounds,
            ),

            checkpoint_bytes: load(
                &self.counters
                    .checkpoint_bytes,
            ),

            partitions: load(
                &self.counters.partitions,
            ),

            stream_buffer_events: load(
                &self.counters
                    .stream_buffer_events,
            ),

            verification_operations: load(
                &self.counters
                    .verification_operations,
            ),

            peak_memory: load(
                &self.counters.peak_memory,
            ),

            current_memory: load(
                &self.counters
                    .current_memory,
            ),

            worker_count: load(
                &self.counters.worker_count,
            ),

            decoder_latency,

            max_decoder_latency:
                Duration::from_nanos(
                    load(
                        &self.counters
                            .max_decoder_latency_nanos,
                    ),
                ),

            min_decoder_latency:
                if min_latency == u64::MAX {
                    None
                } else {
                    Some(
                        Duration::from_nanos(
                            min_latency,
                        ),
                    )
                },

            average_decoder_latency:
                if decode_operations > 0 {
                    Some(duration_divide(
                        decoder_latency,
                        decode_operations,
                    ))
                } else {
                    None
                },

            compute_time:
                Duration::from_nanos(
                    load(
                        &self.counters
                            .compute_time_nanos,
                    ),
                ),

            wall_time:
                self.started.elapsed(),

            physical_error_rate:
                ratio(
                    physical_errors,
                    physical_opportunities,
                ),

            logical_error_rate:
                ratio(
                    logical_failures,
                    logical_opportunities,
                ),

            decoder_success_rate:
                ratio(
                    decoder_success,
                    decode_operations,
                ),

            decoder_failure_rate:
                ratio(
                    decoder_failure,
                    decode_operations,
                ),

            had_logical_failure:
                self.counters
                    .had_logical_failure
                    .load(Ordering::Acquire),

            operations_balanced:
                decode_operations
                    == terminal_operations,

            qpu,
        }
    }

    /* ---------------------------------------------------------------------- */
    /* Reset                                                                    */
    /* ---------------------------------------------------------------------- */

    /// Resets the current measurement window.
    ///
    /// Decoder identity, backend identity and configuration remain unchanged.
    pub fn reset(&self) {
        self.counters
            .decode_operations
            .store(0, Ordering::Release);

        self.counters
            .decoder_success
            .store(0, Ordering::Release);

        self.counters
            .decoder_failure
            .store(0, Ordering::Release);

        self.counters
            .cancellation_count
            .store(0, Ordering::Release);

        self.counters
            .correction_count
            .store(0, Ordering::Release);

        self.counters
            .detection_event_count
            .store(0, Ordering::Release);

        self.counters
            .physical_error_count
            .store(0, Ordering::Release);

        self.counters
            .physical_error_opportunities
            .store(0, Ordering::Release);

        self.counters
            .logical_failure_count
            .store(0, Ordering::Release);

        self.counters
            .logical_error_opportunities
            .store(0, Ordering::Release);

        self.counters
            .matching_count
            .store(0, Ordering::Release);

        self.counters
            .decoder_iterations
            .store(0, Ordering::Release);

        self.counters
            .graph_nodes
            .store(0, Ordering::Release);

        self.counters
            .graph_edges
            .store(0, Ordering::Release);

        self.counters
            .qubit_count
            .store(0, Ordering::Release);

        self.counters
            .stabilizer_count
            .store(0, Ordering::Release);

        self.counters
            .measurement_rounds
            .store(0, Ordering::Release);

        self.counters
            .checkpoint_bytes
            .store(0, Ordering::Release);

        self.counters
            .partitions
            .store(0, Ordering::Release);

        self.counters
            .stream_buffer_events
            .store(0, Ordering::Release);

        self.counters
            .verification_operations
            .store(0, Ordering::Release);

        self.counters
            .worker_count
            .store(0, Ordering::Release);

        self.counters
            .decoder_latency_nanos
            .store(0, Ordering::Release);

        self.counters
            .max_decoder_latency_nanos
            .store(0, Ordering::Release);

        self.counters
            .min_decoder_latency_nanos
            .store(u64::MAX, Ordering::Release);

        self.counters
            .compute_time_nanos
            .store(0, Ordering::Release);

        self.counters
            .peak_memory
            .store(0, Ordering::Release);

        self.counters
            .current_memory
            .store(0, Ordering::Release);

        self.counters
            .qpu_shots
            .store(0, Ordering::Release);

        self.counters
            .qpu_circuits
            .store(0, Ordering::Release);

        self.counters
            .qpu_measurement_count
            .store(0, Ordering::Release);

        self.counters
            .qpu_queue_time_nanos
            .store(0, Ordering::Release);

        self.counters
            .qpu_execution_time_nanos
            .store(0, Ordering::Release);

        self.counters
            .qpu_submission_failures
            .store(0, Ordering::Release);

        self.counters
            .qpu_invalid_measurements
            .store(0, Ordering::Release);

        self.counters
            .qpu_completed_jobs
            .store(0, Ordering::Release);

        self.counters
            .qpu_cancelled_jobs
            .store(0, Ordering::Release);

        self.counters
            .qpu_readout_error_count
            .store(0, Ordering::Release);

        self.counters
            .qpu_readout_error_opportunities
            .store(0, Ordering::Release);

        self.counters
            .had_logical_failure
            .store(false, Ordering::Release);
    }
}

/* ========================================================================== */
/* Timer                                                                      */
/* ========================================================================== */

/// RAII latency timer.
///
/// Exactly one latency measurement is recorded.
#[derive(Debug)]
pub struct MetricsTimer<'a> {
    collector: &'a MetricsCollector,
    started: Instant,
    stopped: bool,
}

impl MetricsTimer<'_> {
    #[must_use]
    pub fn stop(
        mut self,
    ) -> Duration {
        if self.stopped {
            return self.started.elapsed();
        }

        let elapsed =
            self.started.elapsed();

        self.collector
            .record_latency(elapsed);

        self.stopped = true;

        elapsed
    }

    #[must_use]
    pub fn elapsed(&self) -> Duration {
        self.started.elapsed()
    }
}

impl Drop for MetricsTimer<'_> {
    fn drop(&mut self) {
        if !self.stopped {
            self.collector
                .record_latency(
                    self.started.elapsed(),
                );

            self.stopped = true;
        }
    }
}

/* ========================================================================== */
/* Shared metrics                                                             */
/* ========================================================================== */

/// Thread-safe shared metrics collector.
pub type SharedMetrics =
    Arc<MetricsCollector>;

#[must_use]
pub fn shared_metrics(
    decoder: DecoderId,
    backend: BackendKind,
) -> SharedMetrics {
    Arc::new(
        MetricsCollector::standard(
            decoder,
            backend,
        ),
    )
}

/* ========================================================================== */
/* Helpers                                                                    */
/* ========================================================================== */

fn load(
    value: &AtomicU64,
) -> u64 {
    value.load(Ordering::Acquire)
}

fn saturating_increment(
    value: &AtomicU64,
) {
    saturating_add(value, 1);
}

fn saturating_add(
    value: &AtomicU64,
    amount: u64,
) {
    let _ = value.fetch_update(
        Ordering::AcqRel,
        Ordering::Acquire,
        |current| {
            Some(
                current.saturating_add(amount),
            )
        },
    );
}

fn saturating_max(
    value: &AtomicU64,
    candidate: u64,
) {
    let _ = value.fetch_update(
        Ordering::AcqRel,
        Ordering::Acquire,
        |current| {
            Some(current.max(candidate))
        },
    );
}

fn update_min(
    value: &AtomicU64,
    candidate: u64,
) {
    let _ = value.fetch_update(
        Ordering::AcqRel,
        Ordering::Acquire,
        |current| {
            Some(current.min(candidate))
        },
    );
}

fn ratio(
    numerator: u64,
    denominator: u64,
) -> Option<f64> {
    if denominator == 0 {
        None
    } else {
        Some(
            numerator as f64
                / denominator as f64,
        )
    }
}

fn duration_to_nanos_saturating(
    duration: Duration,
) -> u64 {
    duration
        .as_nanos()
        .min(u64::MAX as u128)
        as u64
}

fn saturating_duration_add(
    a: Duration,
    b: Duration,
) -> Duration {
    let nanos = a
        .as_nanos()
        .saturating_add(b.as_nanos())
        .min(u64::MAX as u128);

    Duration::from_nanos(
        nanos as u64,
    )
}

fn duration_divide(
    duration: Duration,
    divisor: u64,
) -> Duration {
    if divisor == 0 {
        return Duration::ZERO;
    }

    let nanos =
        duration.as_nanos()
            / divisor as u128;

    Duration::from_nanos(
        nanos.min(u64::MAX as u128)
            as u64,
    )
}

fn usize_to_u64_saturating(
    value: usize,
) -> u64 {
    if value > u64::MAX as usize {
        u64::MAX
    } else {
        value as u64
    }
}

/* ========================================================================== */
/* Tests                                                                      */
/* ========================================================================== */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_snapshot_is_stable() {
        let snapshot =
            MetricsSnapshot::empty(
                DecoderId::Mwpm,
                BackendKind::Cpu,
            );

        assert_eq!(
            snapshot.decode_operations,
            0
        );

        assert_eq!(
            snapshot.logical_error_rate,
            None
        );

        assert_eq!(
            snapshot.qpu.shots,
            0
        );

        assert!(
            snapshot.operations_balanced
        );
    }

    #[test]
    fn physical_error_rate_is_correct() {
        let metrics =
            MetricsCollector::standard(
                DecoderId::Mwpm,
                BackendKind::Cpu,
            );

        metrics
            .record_physical_error_opportunities(
                100,
            );

        metrics
            .record_physical_errors(10);

        assert_eq!(
            metrics.snapshot()
                .physical_error_rate,
            Some(0.10)
        );
    }

    #[test]
    fn logical_trial_keeps_denominator_correct() {
        let metrics =
            MetricsCollector::standard(
                DecoderId::Mwpm,
                BackendKind::Cpu,
            );

        for _ in 0..95 {
            metrics
                .record_logical_trial(false);
        }

        for _ in 0..5 {
            metrics
                .record_logical_trial(true);
        }

        let snapshot =
            metrics.snapshot();

        assert_eq!(
            snapshot.logical_error_opportunities,
            100
        );

        assert_eq!(
            snapshot.logical_failure_count,
            5
        );

        assert_eq!(
            snapshot.logical_error_rate,
            Some(0.05)
        );
    }

    #[test]
    fn decoder_operations_balance() {
        let metrics =
            MetricsCollector::standard(
                DecoderId::UnionFind,
                BackendKind::Cpu,
            );

        metrics.record_outcome(
            true,
            false,
            Duration::from_millis(1),
        );

        metrics.record_outcome(
            false,
            false,
            Duration::from_millis(2),
        );

        metrics.record_outcome(
            false,
            true,
            Duration::from_millis(3),
        );

        let snapshot =
            metrics.snapshot();

        assert_eq!(
            snapshot.decode_operations,
            3
        );

        assert_eq!(
            snapshot.decoder_success,
            1
        );

        assert_eq!(
            snapshot.decoder_failure,
            1
        );

        assert_eq!(
            snapshot.cancellation_count,
            1
        );

        assert!(
            snapshot.operations_balanced
        );
    }

    #[test]
    fn latency_statistics_are_correct() {
        let metrics =
            MetricsCollector::standard(
                DecoderId::Mwpm,
                BackendKind::Cpu,
            );

        metrics.record_success(
            Duration::from_nanos(100),
        );

        metrics.record_success(
            Duration::from_nanos(300),
        );

        let snapshot =
            metrics.snapshot();

        assert_eq!(
            snapshot.decoder_latency,
            Duration::from_nanos(400)
        );

        assert_eq!(
            snapshot.max_decoder_latency,
            Duration::from_nanos(300)
        );

        assert_eq!(
            snapshot.min_decoder_latency,
            Some(Duration::from_nanos(100))
        );

        assert_eq!(
            snapshot.average_decoder_latency,
            Some(Duration::from_nanos(200))
        );
    }

    #[test]
    fn resource_dimensions_are_observed() {
        let metrics =
            MetricsCollector::standard(
                DecoderId::Mwpm,
                BackendKind::ParallelCpu,
            );

        let snapshot =
            ResourceSnapshot {
                allocated_bytes: 10_000,
                peak_bytes: 20_000,

                syndrome_events: 100,
                graph_nodes: 200,
                graph_edges: 400,
                decoder_iterations: 50,

                parallel_workers: 8,

                code_distance: 25,
                qubits: 625,
                stabilizers: 624,
                measurement_rounds: 100,

                checkpoint_bytes: 4096,
                partitions: 4,
                stream_buffer_events: 512,

                qpu_shots: 1000,
                qpu_circuits: 20,

                verification_operations: 7,

                wall_time:
                    Duration::from_secs(1),

                compute_time:
                    Duration::from_millis(500),
            };

        metrics.record_resource_snapshot(
            snapshot,
        );

        let result =
            metrics.snapshot();

        assert_eq!(
            result.current_memory,
            10_000
        );

        assert_eq!(
            result.peak_memory,
            20_000
        );

        assert_eq!(
            result.detection_event_count,
            100
        );

        assert_eq!(
            result.graph_nodes,
            200
        );

        assert_eq!(
            result.graph_edges,
            400
        );

        assert_eq!(
            result.decoder_iterations,
            50
        );

        assert_eq!(
            result.worker_count,
            8
        );

        assert_eq!(
            result.qubit_count,
            625
        );

        assert_eq!(
            result.stabilizer_count,
            624
        );

        assert_eq!(
            result.checkpoint_bytes,
            4096
        );

        assert_eq!(
            result.partitions,
            4
        );

        assert_eq!(
            result.stream_buffer_events,
            512
        );

        assert_eq!(
            result.verification_operations,
            7
        );

        assert_eq!(
            result.qpu.shots,
            1000
        );

        assert_eq!(
            result.qpu.circuits,
            20
        );
    }

    #[test]
    fn qpu_metrics_are_aggregate_only() {
        let metrics =
            MetricsCollector::standard(
                DecoderId::SurfaceCode,
                BackendKind::Qpu,
            );

        metrics.record_qpu_shots(1000);
        metrics.record_qpu_circuits(20);
        metrics.record_qpu_measurements(4000);

        metrics.record_qpu_readout_trial(true);
        metrics.record_qpu_readout_trial(false);

        let snapshot =
            metrics.snapshot();

        assert_eq!(
            snapshot.qpu.shots,
            1000
        );

        assert_eq!(
            snapshot.qpu.circuits,
            20
        );

        assert_eq!(
            snapshot.qpu.measurement_count,
            4000
        );

        assert_eq!(
            snapshot.qpu.readout_error_count,
            1
        );

        assert_eq!(
            snapshot.qpu.readout_error_opportunities,
            2
        );

        assert_eq!(
            snapshot.qpu.readout_error_rate(),
            Some(0.5)
        );
    }

    #[test]
    fn snapshots_merge_deterministically() {
        let first =
            MetricsCollector::standard(
                DecoderId::Mwpm,
                BackendKind::Distributed,
            );

        first.record_logical_trial(true);
        first.record_matching(10);

        let second =
            MetricsCollector::standard(
                DecoderId::Mwpm,
                BackendKind::Distributed,
            );

        second.record_logical_trial(false);
        second.record_matching(20);

        let mut merged =
            first.snapshot();

        merged
            .merge(&second.snapshot())
            .unwrap();

        assert_eq!(
            merged.logical_error_opportunities,
            2
        );

        assert_eq!(
            merged.logical_failure_count,
            1
        );

        assert_eq!(
            merged.matching_count,
            30
        );

        assert_eq!(
            merged.logical_error_rate,
            Some(0.5)
        );
    }

    #[test]
    fn incompatible_backend_snapshots_are_rejected() {
        let cpu =
            MetricsCollector::standard(
                DecoderId::Mwpm,
                BackendKind::Cpu,
            );

        let gpu =
            MetricsCollector::standard(
                DecoderId::Mwpm,
                BackendKind::Gpu,
            );

        let mut snapshot =
            cpu.snapshot();

        assert!(
            snapshot
                .merge(&gpu.snapshot())
                .is_err()
        );
    }

    #[test]
    fn timer_records_latency() {
        let metrics =
            MetricsCollector::standard(
                DecoderId::Mwpm,
                BackendKind::Cpu,
            );

        let timer =
            metrics.start_timer();

        let _ = timer.stop();

        assert!(
            metrics.snapshot()
                .decoder_latency
                > Duration::ZERO
        );
    }

    #[test]
    fn reset_clears_measurement_window() {
        let metrics =
            MetricsCollector::standard(
                DecoderId::Mwpm,
                BackendKind::Cpu,
            );

        metrics.begin_decode();

        metrics.record_success(
            Duration::from_millis(1),
        );

        metrics.record_logical_trial(true);
        metrics.record_memory(4096);
        metrics.record_qpu_shots(100);

        metrics.reset();

        let snapshot =
            metrics.snapshot();

        assert_eq!(
            snapshot.decode_operations,
            0
        );

        assert_eq!(
            snapshot.decoder_success,
            0
        );

        assert_eq!(
            snapshot.logical_failure_count,
            0
        );

        assert_eq!(
            snapshot.peak_memory,
            0
        );

        assert_eq!(
            snapshot.qpu.shots,
            0
        );
    }

    #[test]
    fn threshold_metrics_use_logical_trials() {
        let metrics =
            MetricsCollector::standard(
                DecoderId::Mwpm,
                BackendKind::Cpu,
            );

        for _ in 0..99 {
            metrics
                .record_logical_trial(false);
        }

        metrics
            .record_logical_trial(true);

        let threshold =
            ThresholdMetrics::from_snapshot(
                &metrics.snapshot(),
                25,
                0.01,
            );

        assert_eq!(
            threshold.code_distance,
            25
        );

        assert_eq!(
            threshold.trials,
            100
        );

        assert_eq!(
            threshold.logical_failures,
            1
        );

        assert_eq!(
            threshold.logical_error_rate,
            Some(0.01)
        );
    }

    #[test]
    fn custom_decoder_identity_is_supported() {
        let decoder =
            DecoderId::custom("my_decoder")
                .unwrap();

        assert_eq!(
            decoder.as_str(),
            "my_decoder"
        );
    }
}